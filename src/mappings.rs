use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use serde::Serialize;
use serde_yaml_ng::{Mapping, Value};
use tempfile::NamedTempFile;
use uuid::Uuid;

const SCANNER_RULES: &str = "security_rules_mitigated_by_this_component";
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
    component_name: &str,
    control_id: &str,
) -> Result<(), Box<dyn Error>> {
    let control_id: &str = control_id.trim();
    if control_id.is_empty() {
        return Err("expected a non-empty control ID".into());
    }

    return update_components(path, |components| {
        let component: &mut Mapping = find_component(components, component_name)?;
        let controls: &mut Vec<Value> = get_or_create_list(component, CONTROLS)?;
        if controls
            .iter()
            .any(|control| control.as_str() == Some(control_id))
        {
            return Err(format!(
                "control {control_id:?} is already associated with {component_name:?}"
            )
            .into());
        }
        controls.push(Value::String(control_id.to_owned()));
        return Ok(());
    });
}

#[derive(Serialize)]
struct RuleMapping<'a> {
    id: &'a str,
    control: &'a str,
    justification: &'a str,
    expiry: &'a str,
}

pub(crate) fn create_rule_mapping(
    path: &Path,
    component_name: &str,
    control: &str,
    scanner: &str,
    rule: &str,
    justification: &str,
    expiry: &str,
) -> Result<(), Box<dyn Error>> {
    let scanner: &str = scanner.trim();
    let rule_mapping: RuleMapping<'_> = RuleMapping {
        id: rule.trim(),
        control: control.trim(),
        justification: justification.trim(),
        expiry,
    };
    if [
        scanner,
        rule_mapping.id,
        rule_mapping.control,
        rule_mapping.justification,
    ]
    .contains(&"")
    {
        return Err("expected a non-empty scanner, rule ID, control ID, and justification".into());
    }
    let expiry_date: NaiveDate = NaiveDate::parse_from_str(expiry, "%Y-%m-%d")
        .map_err(|_| "expected a valid expiry date in YYYY-MM-DD format")?;
    if expiry_date.format("%Y-%m-%d").to_string() != expiry || expiry.len() != 10 {
        return Err("expected a valid expiry date in YYYY-MM-DD format".into());
    }

    return update_components(path, |components| {
        let component: &mut Mapping = find_component(components, component_name)?;
        let controls: &mut Vec<Value> = get_or_create_list(component, CONTROLS)?;
        if !controls
            .iter()
            .any(|value| value.as_str() == Some(rule_mapping.control))
        {
            controls.push(Value::String(rule_mapping.control.to_owned()));
        }

        let scanners: &mut Vec<Value> = get_or_create_list(component, SCANNER_RULES)?;
        let scanner_group: &mut Mapping = find_or_create_scanner(scanners, scanner);
        let rules: &mut Vec<Value> = get_or_create_list(scanner_group, "rules")?;
        if rules.iter().any(|rule| {
            return rule.get("id").and_then(Value::as_str) == Some(rule_mapping.id)
                && rule.get("control").and_then(Value::as_str) == Some(rule_mapping.control);
        }) {
            return Err(format!(
                "expected a new mapping for scanner {scanner:?}, rule {:?}, and control {:?}",
                rule_mapping.id, rule_mapping.control
            )
            .into());
        }
        rules.push(serde_yaml_ng::to_value(rule_mapping)?);
        return Ok(());
    });
}

fn find_or_create_scanner<'a>(scanners: &'a mut Vec<Value>, scanner: &str) -> &'a mut Mapping {
    let group_index: usize = scanners
        .iter()
        .position(|group| {
            return group
                .get("tool")
                .and_then(Value::as_str)
                .is_some_and(|tool| tool.eq_ignore_ascii_case(scanner));
        })
        .unwrap_or_else(|| {
            let mut group: Mapping = Mapping::new();
            group.insert(Value::from("tool"), Value::from(scanner));
            scanners.push(Value::Mapping(group));
            return scanners.len() - 1;
        });
    return scanners[group_index]
        .as_mapping_mut()
        .expect("scanner group is a mapping");
}

fn find_component<'a>(
    components: &'a mut [Value],
    component_name: &str,
) -> Result<&'a mut Mapping, Box<dyn Error>> {
    let component_name: &str = component_name.trim();
    if component_name.is_empty() {
        return Err("expected a non-empty component name".into());
    }

    let mut matches = components
        .iter_mut()
        .filter_map(Value::as_mapping_mut)
        .filter(|component| component.get("title").and_then(Value::as_str) == Some(component_name));
    let component: &mut Mapping = matches
        .next()
        .ok_or_else(|| format!("expected an existing component named {component_name:?}"))?;
    if matches.next().is_some() {
        return Err(
            format!("expected one component named {component_name:?}, but found multiple").into(),
        );
    }
    return Ok(component);
}

fn get_or_create_list<'a>(
    mapping: &'a mut Mapping,
    key: &str,
) -> Result<&'a mut Vec<Value>, Box<dyn Error>> {
    return mapping
        .entry(Value::from(key))
        .or_insert_with(|| Value::Sequence(Vec::new()))
        .as_sequence_mut()
        .ok_or_else(|| format!("expected a list at {key}").into());
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
