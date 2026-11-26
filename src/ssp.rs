use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;

use crate::error::Error;

#[derive(Clone, Debug, Deserialize)]
struct PlanDocument {
    #[serde(rename = "system-security-plan")]
    plan: Plan,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Plan {
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub metadata: PlanMetadata,
    #[serde(default)]
    control_implementation: ControlImplementation,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PlanMetadata {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub last_modified: String,
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
    pub fn controls(&self) -> Vec<String> {
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

pub fn load(path: &Path) -> Result<Plan, Error> {
    let file: File = File::open(path)?;
    let document: PlanDocument = serde_json::from_reader(BufReader::new(file))?;
    return Ok(document.plan);
}
