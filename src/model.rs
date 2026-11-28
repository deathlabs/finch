use chrono::{SecondsFormat, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ssp::Plan;

/// An OSCAL-based representation of a [hosting platform](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#hosting-platform) element and the implemented [controls](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#control) it supports.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Component {
    /// Unique ID for the [component](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#component).
    pub uuid: String,
    /// Name of the [component](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#component).
    pub title: String,
    /// Type of the [component](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#component).
    #[serde(rename = "type")]
    pub component_type: String,
    /// Description of the [component](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#component).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// Purpose of the [component](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#component).
    pub purpose: String,
    /// IDs of [controls](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#control) implemented by the [component](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#component).
    #[serde(rename = "controls_implemented_by_this_component")]
    pub controls: Vec<String>,
    /// Groups of [mappings](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#mapping) by [scanner](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#scanner).
    #[serde(rename = "security_rules_mitigated_by_this_component")]
    pub rules: Vec<ScannerRules>,
}

/// [Mappings](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#mapping) for rules from one [scanner](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#scanner).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScannerRules {
    /// Name of the [scanner](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#scanner).
    pub tool: String,
    /// [Mappings](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#mapping) for this [scanner](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#scanner).
    pub rules: Vec<RuleMapping>,
}

/// A [mappings file](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#mappings-file) records [mappings](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#mapping).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MappingsFile {
    /// Details about the source [system security plan](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#system-security-plan) and file generation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MappingsMetadata>,
    /// [Components](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#component) recorded in the file.
    pub components: Vec<Component>,
}

/// Details about the source [system security plan](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#system-security-plan) and file generation.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MappingsMetadata {
    /// Unique ID of the [system security plan](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#system-security-plan).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub system_security_plan_uuid: String,
    /// Title of the [system security plan](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#system-security-plan).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub system_security_plan_title: String,
    /// Version of the [system security plan](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#system-security-plan).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub system_security_plan_version: String,
    /// When the [system security plan](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#system-security-plan) was last modified.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub system_security_plan_last_modified: String,
    /// When Finch generated the [mappings file](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#mappings-file).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub generated_at: String,
}

impl MappingsFile {
    pub fn from_plan(plan: &Plan, components: Vec<Component>) -> Self {
        return Self {
            metadata: Some(MappingsMetadata {
                system_security_plan_uuid: plan.uuid.clone(),
                system_security_plan_title: plan.metadata.title.clone(),
                system_security_plan_version: plan.metadata.version.clone(),
                system_security_plan_last_modified: plan.metadata.last_modified.clone(),
                generated_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            }),
            components,
        };
    }
}

/// A [mapping](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#mapping) relates a [scanner](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#scanner) rule, a [component](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#component), and an implemented [control](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#control).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RuleMapping {
    /// ID of the [scanner](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#scanner) rule.
    pub id: String,
    /// ID of the implemented [control](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#control) that supports this [mapping](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#mapping).
    pub control: String,
    /// [Justification](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#justification) for how the implemented [control](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#control) mitigates the [risk](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#risk) associated with the [scanner](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#scanner) rule.
    pub justification: String,
    /// [Expiry](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#expiry) in YYYY-MM-DD format. This is the last date on which the [mapping](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#mapping) is valid.
    pub expiry: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOCUMENT: &str = r#"
metadata:
  generated_at: "2026-11-26T00:00:00Z"
components:
- uuid: c1fa4af7-c3bf-4748-b88f-cf74569e1cac
  title: Gateway
  type: service
  description: Example gateway
  purpose: Filter requests
  controls_implemented_by_this_component: [sc-7]
  security_rules_mitigated_by_this_component:
  - tool: ZAP
    rules:
    - id: "10038"
      control: sc-7
      justification: Example justification
      expiry: "2028-02-29"
"#;

    #[test]
    fn round_trips_mappings() {
        let document: MappingsFile = serde_yaml_ng::from_str(DOCUMENT).unwrap();
        let yaml: String = serde_yaml_ng::to_string(&document).unwrap();
        let restored: MappingsFile = serde_yaml_ng::from_str(&yaml).unwrap();
        assert_eq!(restored, document);
        assert_eq!(restored.components[0].rules[0].rules[0].id, "10038");
    }

    #[test]
    fn rejects_unknown_fields_at_every_level() {
        for (line, extra) in [
            ("components:", "typo: true\n"),
            ("  generated_at:", "  typo: true\n"),
            ("  title:", "  typo: true\n"),
            ("    rules:", "    typo: true\n"),
            ("      expiry:", "      typo: true\n"),
        ] {
            let yaml: String = DOCUMENT.replace(line, &format!("{extra}{line}"));
            let result: Result<MappingsFile, serde_yaml_ng::Error> = serde_yaml_ng::from_str(&yaml);
            assert!(result.unwrap_err().to_string().contains("unknown field"));
        }
    }

    #[test]
    fn rejects_bare_rules_and_missing_fields() {
        let yaml: String = DOCUMENT.replace(
            "    - id: \"10038\"\n      control: sc-7\n      justification: Example justification\n      expiry: \"2028-02-29\"",
            "    - \"10038\"",
        );
        let result: Result<MappingsFile, serde_yaml_ng::Error> = serde_yaml_ng::from_str(&yaml);
        assert!(result.is_err());
        for field in ["id", "control", "justification", "expiry"] {
            let rule = DOCUMENT.split_once("    - id:").unwrap().1;
            let missing: String = format!("id:{rule}")
                .lines()
                .filter(|line| !line.trim_start().starts_with(&format!("{field}:")))
                .map(str::trim_start)
                .collect::<Vec<_>>()
                .join("\n");
            let result: Result<RuleMapping, serde_yaml_ng::Error> =
                serde_yaml_ng::from_str(&missing);
            assert!(result.unwrap_err().to_string().contains("missing field"));
        }
    }
}
