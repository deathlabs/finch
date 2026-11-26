use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Clone, Debug, Deserialize)]
struct PlanDocument {
    #[serde(rename = "system-security-plan")]
    plan: Plan,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) struct Plan {
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub metadata: PlanMetadata,
    #[serde(default)]
    control_implementation: ControlImplementation,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) struct PlanMetadata {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    version: String,
    #[serde(default)]
    last_modified: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ControlImplementation {
    #[serde(default)]
    implemented_requirements: Vec<Requirement>,
}

#[derive(Clone, Debug, Deserialize)]
struct Requirement {
    #[serde(rename = "control-id")]
    control_id: String,
}

impl Plan {
    pub(super) fn controls(&self) -> Vec<String> {
        let mut controls: Vec<String> = self
            .control_implementation
            .implemented_requirements
            .iter()
            .map(|requirement| requirement.control_id.clone())
            .collect();
        controls.sort();
        controls.dedup();
        return controls;
    }
}

#[derive(Clone, Default, Serialize)]
pub(super) struct Component {
    pub uuid: String,
    pub title: String,
    #[serde(rename = "type")]
    pub component_type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub purpose: String,
    #[serde(rename = "controls_implemented_by_this_component")]
    pub controls: Vec<String>,
    #[serde(rename = "security_rules_mitigated_by_this_component")]
    pub rules: Vec<ToolRules>,
}

#[derive(Clone, Serialize)]
pub(super) struct ToolRules {
    pub tool: String,
    pub rules: Vec<String>,
}

#[derive(Serialize)]
pub(super) struct ExportDocument {
    metadata: ExportMetadata,
    components: Vec<Component>,
}

#[derive(Serialize)]
struct ExportMetadata {
    #[serde(skip_serializing_if = "String::is_empty")]
    system_security_plan_uuid: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    system_security_plan_title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    system_security_plan_version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    system_security_plan_last_modified: String,
    generated_at: String,
}

impl ExportDocument {
    pub(super) fn new(plan: &Plan, components: &[Component]) -> Self {
        return Self {
            metadata: ExportMetadata {
                system_security_plan_uuid: plan.uuid.clone(),
                system_security_plan_title: plan.metadata.title.clone(),
                system_security_plan_version: plan.metadata.version.clone(),
                system_security_plan_last_modified: plan.metadata.last_modified.clone(),
                generated_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            },
            components: components.to_vec(),
        };
    }
}

pub(super) fn load(path: &Path) -> Result<Plan, Box<dyn Error>> {
    let file: File = File::open(path)?;
    let document: PlanDocument = serde_json::from_reader(BufReader::new(file))?;
    return Ok(document.plan);
}

pub(super) fn save(path: &Path, document: &ExportDocument) -> Result<(), Box<dyn Error>> {
    let parent: &Path = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut file: tempfile::NamedTempFile = tempfile::NamedTempFile::new_in(parent)?;
    serde_yaml_ng::to_writer(&mut file, document)?;
    file.flush()?;
    file.persist(path)?;
    return Ok(());
}

pub(super) fn completions(prefix: &str) -> Result<Vec<String>, walkdir::Error> {
    let root: &Path = Path::new(".");
    let mut matches: Vec<String> = Vec::new();
    for entry in WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| entry.file_name() != ".git" && entry.file_name() != "target")
    {
        let entry: walkdir::DirEntry = entry?;
        if !entry.file_type().is_file()
            || !entry
                .path()
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
        {
            continue;
        }
        let path: &Path = entry.path();
        let path: &Path = if prefix.starts_with("./") {
            path
        } else {
            path.strip_prefix(root).unwrap_or(path)
        };
        if let Some(path) = path.to_str()
            && path.starts_with(prefix)
        {
            matches.push(path.to_owned());
        }
    }
    return Ok(matches);
}
