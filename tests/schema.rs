use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use finch::ops::{CreateComponent, MapControl, MapRule};
use finch::schema;
use schemars::schema_for;
use serde_json::Value;

#[test]
fn committed_schema_matches_cli_output() {
    let output: Output = Command::new(env!("CARGO_BIN_EXE_finch"))
        .arg("schema")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        std::str::from_utf8(&output.stdout).unwrap(),
        include_str!("../schema/mappings.schema.json"),
        "Update the committed schema with: cargo run --quiet -- schema > schema/mappings.schema.json"
    );
}

#[test]
fn derived_schemas_reject_unknown_keys_and_require_rule_fields() {
    let schema: Value = serde_json::to_value(schema::mappings()).unwrap();
    assert_eq!(schema["additionalProperties"], false);
    for name in [
        "Component",
        "ScannerRules",
        "RuleMapping",
        "MappingsMetadata",
    ] {
        assert_eq!(
            schema["$defs"][name]["additionalProperties"], false,
            "{name}"
        );
    }
    let rule: &Value = &schema["$defs"]["RuleMapping"];
    assert_eq!(rule["type"], "object");
    let mut required: Vec<&str> = rule["required"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect();
    required.sort_unstable();
    assert_eq!(required, ["control", "expiry", "id", "justification"]);

    for schema in [
        schema_for!(CreateComponent),
        schema_for!(MapControl),
        schema_for!(MapRule),
    ] {
        let schema: Value = serde_json::to_value(schema).unwrap();
        assert_eq!(schema["additionalProperties"], false);
    }
}

#[test]
fn checked_in_yaml_conforms_to_json_schema() {
    let root: &Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut paths: Vec<PathBuf> = vec![
        root.join("mappings.yaml"),
        root.join("docs/demo/mappings.yaml"),
    ];
    paths.extend(
        fs::read_dir(root.join("examples"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension == "yaml")
            }),
    );
    let schema: Value =
        serde_json::from_str(include_str!("../schema/mappings.schema.json")).unwrap();
    let validator: jsonschema::Validator = jsonschema::validator_for(&schema).unwrap();
    for path in paths {
        let yaml: String = fs::read_to_string(&path).unwrap();
        let instance: Value = serde_yaml_ng::from_str(&yaml)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        if let Err(error) = validator.validate(&instance) {
            panic!("{} at {}: {error}", path.display(), error.instance_path());
        }
    }
}
