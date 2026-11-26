use std::path::Path;
use std::process::{Command, Output};

use finch::catalog::Scanner;
use finch::model::{Component, MappingsFile};
use finch::{ops, store};

#[test]
fn loads_checked_in_mappings() {
    let root: &Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    for path in [
        "mappings.yaml",
        "docs/demo/mappings.yaml",
        "examples/platform-a-mappings.yaml",
        "examples/platform-b-mappings.yaml",
        "examples/platform-c-mappings.yaml",
        "examples/platform-u-mappings.yaml",
    ] {
        let document: MappingsFile = store::load(&root.join(path)).unwrap();
        assert!(!document.components.is_empty(), "{path}");
        for component in &document.components {
            for group in &component.rules {
                let scanner: Scanner = Scanner::find(&group.tool).unwrap();
                for rule in &group.rules {
                    assert_eq!(scanner.normalize_rule(&rule.id).unwrap(), rule.id);
                    assert!(component.controls.contains(&rule.control));
                    assert!(!rule.justification.trim().is_empty());
                    ops::validate_expiry(&rule.expiry).unwrap();
                }
            }
        }
    }
}

#[test]
fn cli_creates_entries_the_library_can_read() {
    let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
    let path: &Path = &directory.path().join("test.yaml");
    store::save(path, &MappingsFile::default()).unwrap();

    for args in [
        &[
            "component",
            "--type",
            "service",
            "--description",
            "Gateway",
            "--purpose",
            "Filter requests",
        ][..],
        &["control", "--control", "sc-7"],
        &[
            "rule",
            "--control",
            "sc-7",
            "--scanner",
            "ZAP",
            "--rule",
            "010038",
            "--justification",
            "Example justification",
            "--expiry",
            "2028-02-29",
        ],
    ] {
        let output: Output = Command::new(env!("CARGO_BIN_EXE_finch"))
            .args(["create", "--mappings-file"])
            .arg(path)
            .args(args)
            .args(["--component-name", "Gateway"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let document: MappingsFile = store::load(path).unwrap();
    let component: &Component = &document.components[0];
    assert_eq!(component.description, "Gateway");
    assert_eq!(component.rules[0].rules[0].id, "10038");
}
