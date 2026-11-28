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
        "docs/src/demos/generate/mappings.yaml",
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
fn cli_adds_and_gets_entries() {
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
            .args(["add", "--mappings-file"])
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

    let get = |entry: &str, name: &str, args: &[&str]| {
        return Command::new(env!("CARGO_BIN_EXE_finch"))
            .args(["get", entry, "--mappings-file"])
            .arg(path)
            .args(["--component-name", name])
            .args(args)
            .output()
            .unwrap();
    };
    for args in [&[][..], &["--output-format", "json"]] {
        let output: Output = get("component", "Gateway", args);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let reported: Component = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(&reported, component);
    }

    let output: Output = get("component", "Gateway", &["-o", "yaml"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let reported: Component = serde_yaml_ng::from_slice(&output.stdout).unwrap();
    assert_eq!(&reported, component);

    let output: Output = get("component", "Missing", &[]);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("expected an existing component named \"Missing\"")
    );

    let output: Output = get("control", "Gateway", &["--control", "sc-7"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let control: String = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(control, "sc-7");

    let lookup: &[&str] = &["--scanner", "zap", "--rule", "010038", "--control", "sc-7"];
    let output: Output = get("rule", "Gateway", lookup);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let rule: finch::model::RuleMapping = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(rule, component.rules[0].rules[0]);

    let args: Vec<&str> = [lookup, &["-o", "yaml"]].concat();
    let output: Output = get("rule", "Gateway", &args);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let reported: finch::model::RuleMapping = serde_yaml_ng::from_slice(&output.stdout).unwrap();
    assert_eq!(reported, rule);

    for (entry, args) in [
        ("control", &["--control", "missing"][..]),
        (
            "rule",
            &[
                "--scanner",
                "zap",
                "--rule",
                "10038",
                "--control",
                "missing",
            ][..],
        ),
    ] {
        let output: Output = get(entry, "Gateway", args);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("missing"));
    }
}

#[test]
fn cli_delete_checks_dependencies_and_force_limits_the_cascade() {
    let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
    let path: &Path = &directory.path().join("delete.yaml");
    let rule: finch::model::RuleMapping = finch::model::RuleMapping {
        id: "10038".to_owned(),
        control: "sc-7".to_owned(),
        justification: "Example".to_owned(),
        expiry: "2028-02-29".to_owned(),
    };
    let document: MappingsFile = MappingsFile {
        components: vec![
            Component {
                title: "Gateway".to_owned(),
                controls: vec!["sc-7".to_owned(), "si-10".to_owned()],
                rules: vec![finch::model::ScannerRules {
                    tool: "ZAP".to_owned(),
                    rules: vec![
                        rule.clone(),
                        finch::model::RuleMapping {
                            control: "si-10".to_owned(),
                            ..rule.clone()
                        },
                        finch::model::RuleMapping {
                            id: "10020".to_owned(),
                            ..rule
                        },
                    ],
                }],
                ..Component::default()
            },
            Component {
                title: "Other".to_owned(),
                ..Component::default()
            },
        ],
        ..MappingsFile::default()
    };
    store::save(path, &document).unwrap();
    let original: Vec<u8> = std::fs::read(path).unwrap();
    let delete = |entry: &[&str]| {
        return Command::new(env!("CARGO_BIN_EXE_finch"))
            .args(["delete", "--mappings-file"])
            .arg(path)
            .args(entry)
            .args(["--component-name", "Gateway"])
            .output()
            .unwrap();
    };

    for (entry, message) in [
        (&["component"][..], "controls depend on it (sc-7, si-10)"),
        (
            &["control", "--control", "sc-7"][..],
            "rules depend on it (ZAP 10038, ZAP 10020)",
        ),
    ] {
        let output: Output = delete(entry);
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains(message));
        assert_eq!(std::fs::read(path).unwrap(), original);
    }

    let output: Output = delete(&[
        "rule",
        "--control",
        "sc-7",
        "--scanner",
        "zap",
        "--rule",
        "010038",
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let updated: MappingsFile = store::load(path).unwrap();
    assert_eq!(
        updated.components[0].rules[0].rules,
        document.components[0].rules[0].rules[1..]
    );
    assert_eq!(
        updated.components[0].controls,
        document.components[0].controls
    );

    let output: Output = delete(&["control", "--control", "sc-7", "-f"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let updated: MappingsFile = store::load(path).unwrap();
    assert_eq!(updated.components[0].controls, ["si-10"]);
    assert_eq!(
        updated.components[0].rules[0].rules,
        document.components[0].rules[0].rules[1..2]
    );

    let output: Output = delete(&["component", "-f"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        store::load(path).unwrap().components,
        document.components[1..]
    );
}
