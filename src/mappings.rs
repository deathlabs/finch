use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_yaml_ng::{Mapping, Value};
use tempfile::NamedTempFile;
use uuid::Uuid;

const CONTROLS: &str = "controls_implemented_by_this_component";

#[derive(Serialize)]
struct Component<'a> {
    uuid: String,
    title: &'a str,
    #[serde(rename = "type")]
    component_type: &'a str,
    purpose: &'a str,
    controls_implemented_by_this_component: [&'a str; 0],
    security_rules_mitigated_by_this_component: [&'a str; 0],
}

pub(crate) fn create_component(path: &Path, title: &str) -> Result<(), Box<dyn Error>> {
    let title: &str = title.trim();
    if title.is_empty() {
        return Err("expected a non-empty component name".into());
    }

    return update_components(path, |components| {
        if components
            .iter()
            .any(|component| component.get("title").and_then(Value::as_str) == Some(title))
        {
            return Err(
                format!("expected a new component name, but {title:?} already exists").into(),
            );
        }

        let component: Component<'_> = Component {
            uuid: Uuid::new_v4().to_string(),
            title,
            component_type: "",
            purpose: "",
            controls_implemented_by_this_component: [],
            security_rules_mitigated_by_this_component: [],
        };
        components.push(serde_yaml_ng::to_value(component)?);
        return Ok(());
    });
}

pub(crate) fn create_control_mapping(
    path: &Path,
    component_uuid: Uuid,
    control_id: &str,
) -> Result<(), Box<dyn Error>> {
    let control_id: &str = control_id.trim();
    if control_id.is_empty() {
        return Err("expected a non-empty control ID".into());
    }

    return update_components(path, |components| {
        let component: &mut Mapping = components
            .iter_mut()
            .filter_map(Value::as_mapping_mut)
            .find(|component| {
                return component
                    .get("uuid")
                    .and_then(Value::as_str)
                    .and_then(|uuid| Uuid::parse_str(uuid).ok())
                    == Some(component_uuid);
            })
            .ok_or_else(|| format!("expected an existing component with UUID {component_uuid}"))?;
        let controls: &mut Vec<Value> = component
            .entry(Value::String(CONTROLS.to_owned()))
            .or_insert_with(|| Value::Sequence(Vec::new()))
            .as_sequence_mut()
            .ok_or("expected a list of controls for the component")?;
        if controls
            .iter()
            .any(|control| control.as_str() == Some(control_id))
        {
            return Err(format!(
                "control {control_id:?} is already associated with {component_uuid}"
            )
            .into());
        }
        controls.push(Value::String(control_id.to_owned()));
        return Ok(());
    });
}

fn update_components(
    path: &Path,
    update: impl FnOnce(&mut Vec<Value>) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    let path: PathBuf = fs::canonicalize(path)?;
    let file: File = File::open(&path)?;
    let metadata: fs::Metadata = file.metadata()?;
    let mut document: Value = serde_yaml_ng::from_reader(BufReader::new(file))?;
    let components: &mut Vec<Value> = document
        .get_mut("components")
        .and_then(Value::as_sequence_mut)
        .ok_or("expected a mappings file with a components list")?;
    update(components)?;

    let parent: &Path = path.parent().ok_or("expected a mappings file path")?;
    let mut temporary_file: NamedTempFile = NamedTempFile::new_in(parent)?;
    serde_yaml_ng::to_writer(&mut temporary_file, &document)?;
    temporary_file.flush()?;
    temporary_file
        .as_file()
        .set_permissions(metadata.permissions())?;
    temporary_file.persist(&path)?;
    return Ok(());
}
