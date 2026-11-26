use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};

use crate::ssp::Plan;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Component {
    pub uuid: String,
    pub title: String,
    #[serde(rename = "type")]
    pub component_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub purpose: String,
    #[serde(rename = "controls_implemented_by_this_component")]
    pub controls: Vec<String>,
    #[serde(rename = "security_rules_mitigated_by_this_component")]
    pub rules: Vec<ScannerRules>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScannerRules {
    pub tool: String,
    pub rules: Vec<RuleMapping>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MappingsFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MappingsMetadata>,
    pub components: Vec<Component>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MappingsMetadata {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub system_security_plan_uuid: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub system_security_plan_title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub system_security_plan_version: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub system_security_plan_last_modified: String,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuleMapping {
    pub id: String,
    pub control: String,
    pub justification: String,
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
