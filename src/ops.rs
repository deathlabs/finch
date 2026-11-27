//! Mutate an in-memory mappings document. File-backed callers should perform
//! these operations inside [crate::store::update].
//!
//! Add operations return [Error::AlreadyExists] for duplicates. Operations leave
//! the document unchanged on error.

mod delete;

pub use delete::{
    DeleteComponent, DeleteControl, DeleteRule, delete_component, delete_control, delete_rule,
};

use chrono::NaiveDate;
use clap::Args;
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

use crate::catalog::Scanner;
use crate::error::Error;
use crate::model::{Component, RuleMapping, ScannerRules};

/// Input for getting a [component](GLOSSARY.md#component).
#[derive(Args, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GetComponent {
    /// Name of the [component](GLOSSARY.md#component).
    #[arg(long, help = "Name of the component")]
    pub component_name: String,
}

/// Get the component with the specified name.
pub fn get_component<'a>(
    components: &'a [Component],
    input: &GetComponent,
) -> Result<&'a Component, Error> {
    let index: usize = find_component_index(components, &input.component_name)?;
    return Ok(&components[index]);
}

/// Input for getting an implemented [control](GLOSSARY.md#control).
#[derive(Args, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GetControl {
    /// Name of the [component](GLOSSARY.md#component).
    #[arg(long, help = "Name of the component")]
    pub component_name: String,
    /// ID of the implemented [control](GLOSSARY.md#control).
    #[arg(long, help = "ID of the implemented control")]
    pub control: String,
}

/// Input for getting a [mapping](GLOSSARY.md#mapping).
#[derive(Args, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GetRule {
    /// Name of the [component](GLOSSARY.md#component).
    #[arg(long, help = "Name of the component")]
    pub component_name: String,
    /// ID of the implemented [control](GLOSSARY.md#control).
    #[arg(long, help = "ID of the implemented control")]
    pub control: String,
    /// Name of the [scanner](GLOSSARY.md#scanner).
    #[arg(long, help = "Name of the scanner")]
    pub scanner: String,
    /// ID of the [scanner](GLOSSARY.md#scanner) rule.
    #[arg(long, help = "ID of the scanner rule")]
    pub rule: String,
}

/// Get the stored control ID from the named component.
pub fn get_control<'a>(components: &'a [Component], input: &GetControl) -> Result<&'a str, Error> {
    let control: &str = non_empty(&input.control, "control ID")?;
    let index: usize = find_component_index(components, &input.component_name)?;
    return find_control(&components[index], control);
}

/// Get a mapping by component, scanner, normalized rule ID, and control.
pub fn get_rule<'a>(
    components: &'a [Component],
    input: &GetRule,
) -> Result<&'a RuleMapping, Error> {
    let index: usize = find_component_index(components, &input.component_name)?;
    let component: &Component = &components[index];
    let (group, rule): (usize, usize) =
        find_rule_indices(component, &input.control, &input.scanner, &input.rule)?;
    return Ok(&component.rules[group].rules[rule]);
}

fn find_rule_indices(
    component: &Component,
    control: &str,
    scanner: &str,
    rule: &str,
) -> Result<(usize, usize), Error> {
    let control: &str = non_empty(control, "control ID")?;
    let scanner: Scanner = Scanner::find(scanner)?;
    let id: String = scanner.normalize_rule(rule)?;
    let mut matches: Option<(usize, usize)> = None;
    for (group_index, group) in component.rules.iter().enumerate() {
        if !scanner.name().eq_ignore_ascii_case(&group.tool) {
            continue;
        }
        for (rule_index, rule) in group.rules.iter().enumerate() {
            if rule.control != control
                || !scanner
                    .normalize_rule(&rule.id)
                    .is_ok_and(|existing| existing == id)
            {
                continue;
            }
            if matches.replace((group_index, rule_index)).is_some() {
                return Err(Error::Invalid(format!(
                    "expected one {} rule {id:?} citing control {control:?} on {:?}, but found multiple",
                    scanner.name(),
                    component.title
                )));
            }
        }
    }
    return matches.ok_or_else(|| {
        return Error::NotFound(format!(
            "expected an existing {} rule {id:?} citing control {control:?} on {:?}",
            scanner.name(),
            component.title
        ));
    });
}

/// Input for adding a [component](GLOSSARY.md#component).
#[derive(Args, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateComponent {
    /// Name of the new [component](GLOSSARY.md#component).
    #[arg(long)]
    pub component_name: String,
    /// Type of the [component](GLOSSARY.md#component).
    #[arg(long = "type")]
    pub component_type: String,
    /// Description of the [component](GLOSSARY.md#component).
    #[arg(long)]
    pub description: String,
    /// Purpose of the [component](GLOSSARY.md#component).
    #[arg(long)]
    pub purpose: String,
}

/// Input for associating an implemented [control](GLOSSARY.md#control) with a [component](GLOSSARY.md#component).
#[derive(Args, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MapControl {
    /// Name of the [component](GLOSSARY.md#component).
    #[arg(long)]
    pub component_name: String,
    /// ID of the implemented [control](GLOSSARY.md#control).
    #[arg(long)]
    pub control: String,
}

/// Input for adding a [mapping](GLOSSARY.md#mapping).
#[derive(Args, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MapRule {
    /// Name of the [component](GLOSSARY.md#component).
    #[arg(long)]
    pub component_name: String,
    /// ID of the implemented [control](GLOSSARY.md#control) that supports this [mapping](GLOSSARY.md#mapping).
    #[arg(long)]
    pub control: String,
    /// Name of the [scanner](GLOSSARY.md#scanner).
    #[arg(long)]
    pub scanner: String,
    /// ID of the [scanner](GLOSSARY.md#scanner) rule.
    #[arg(long)]
    pub rule: String,
    /// [Justification](GLOSSARY.md#justification) for how the implemented [control](GLOSSARY.md#control) mitigates the [risk](GLOSSARY.md#risk) associated with the rule.
    #[arg(long)]
    pub justification: String,
    /// [Expiry](GLOSSARY.md#expiry) in YYYY-MM-DD format. This is the last date on which the [mapping](GLOSSARY.md#mapping) is valid.
    #[arg(long, value_name = "YYYY-MM-DD")]
    pub expiry: String,
}

/// Add a component. Its name must be unique within the document.
pub fn create_component(
    components: &mut Vec<Component>,
    input: &CreateComponent,
) -> Result<(), Error> {
    let title: &str = non_empty(&input.component_name, "component name")?;
    let component_type: &str = non_empty(&input.component_type, "component type")?;
    let description: &str = non_empty(&input.description, "description")?;
    let purpose: &str = non_empty(&input.purpose, "purpose")?;
    if components.iter().any(|component| component.title == title) {
        return Err(Error::AlreadyExists(format!(
            "expected a new component name, but {title:?} already exists"
        )));
    }
    components.push(Component {
        uuid: Uuid::new_v4().to_string(),
        title: title.to_owned(),
        component_type: component_type.to_owned(),
        description: description.to_owned(),
        purpose: purpose.to_owned(),
        controls: Vec::new(),
        rules: Vec::new(),
    });
    return Ok(());
}

/// Associate a control ID with an existing component.
pub fn map_control(components: &mut [Component], input: &MapControl) -> Result<(), Error> {
    let control: &str = non_empty(&input.control, "control ID")?;
    let component: &mut Component = find_component(components, &input.component_name)?;
    if component
        .controls
        .iter()
        .any(|existing| existing == control)
    {
        return Err(Error::AlreadyExists(format!(
            "control {control:?} is already associated with {:?}",
            component.title
        )));
    }
    component.controls.push(control.to_owned());
    return Ok(());
}

/// Add a rule for a control the component already implements. Within the
/// component, the scanner, normalized rule ID and control identify a mapping.
pub fn map_rule(components: &mut [Component], input: &MapRule) -> Result<(), Error> {
    let (scanner, rule_mapping): (Scanner, RuleMapping) = input.validated()?;
    let component: &mut Component = find_component(components, &input.component_name)?;
    require_control(component, &rule_mapping.control)?;

    for group in &component.rules {
        if !scanner.name().eq_ignore_ascii_case(&group.tool) {
            continue;
        }
        for existing in &group.rules {
            if existing.control == rule_mapping.control
                && scanner.normalize_rule(&existing.id)? == rule_mapping.id
            {
                return Err(Error::AlreadyExists(format!(
                    "expected a new mapping for scanner {:?}, rule {:?}, and control {:?}",
                    scanner.name(),
                    rule_mapping.id,
                    rule_mapping.control
                )));
            }
        }
    }
    if let Some(group) = component
        .rules
        .iter_mut()
        .find(|group| scanner.name().eq_ignore_ascii_case(&group.tool))
    {
        group.rules.push(rule_mapping);
    } else {
        component.rules.push(ScannerRules {
            tool: scanner.name().to_owned(),
            rules: vec![rule_mapping],
        });
    }
    return Ok(());
}

impl MapRule {
    fn validated(&self) -> Result<(Scanner, RuleMapping), Error> {
        let scanner: Scanner = Scanner::find(&self.scanner)?;
        return Ok((
            scanner,
            RuleMapping {
                id: scanner.normalize_rule(&self.rule)?,
                control: non_empty(&self.control, "control ID")?.to_owned(),
                justification: non_empty(&self.justification, "justification")?.to_owned(),
                expiry: validate_expiry(&self.expiry)?
                    .format("%Y-%m-%d")
                    .to_string(),
            },
        ));
    }
}

/// Date-based state of a [mapping](GLOSSARY.md#mapping).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpiryState {
    /// The supplied date is on or before the [expiry](GLOSSARY.md#expiry).
    Valid,
    /// The supplied date is after the [expiry](GLOSSARY.md#expiry).
    Expired,
}

/// Classify a [mapping](GLOSSARY.md#mapping) by date for the
/// [Mapping state machine](STATE_MACHINES.md#mapping).
pub fn classify_expiry(rule: &RuleMapping, date: NaiveDate) -> Result<ExpiryState, Error> {
    let expiry: NaiveDate = validate_expiry(&rule.expiry)?;
    if expiry < date {
        return Ok(ExpiryState::Expired);
    }
    return Ok(ExpiryState::Valid);
}

pub fn validate_expiry(expiry: &str) -> Result<NaiveDate, Error> {
    let expiry: &str = expiry.trim();
    let invalid = || Error::Invalid("expected a valid expiry date in YYYY-MM-DD format".to_owned());
    let date: NaiveDate = NaiveDate::parse_from_str(expiry, "%Y-%m-%d").map_err(|_| invalid())?;
    if expiry.len() != 10
        || date.format("%Y-%m-%d").to_string() != expiry
        || expiry.starts_with("0000")
    {
        return Err(invalid());
    }
    return Ok(date);
}

pub(crate) fn non_empty<'a>(value: &'a str, field: &str) -> Result<&'a str, Error> {
    let value: &str = value.trim();
    if value.is_empty() {
        return Err(Error::Invalid(format!("expected a non-empty {field}")));
    }
    return Ok(value);
}

pub(crate) fn require_control(component: &Component, control: &str) -> Result<(), Error> {
    find_control(component, control)?;
    return Ok(());
}

fn find_control<'a>(component: &'a Component, control: &str) -> Result<&'a str, Error> {
    return component
        .controls
        .iter()
        .map(String::as_str)
        .find(|existing| *existing == control)
        .ok_or_else(|| {
            Error::NotFound(format!(
                "expected control {control:?} to be associated with {:?}",
                component.title
            ))
        });
}

fn find_component<'a>(
    components: &'a mut [Component],
    name: &str,
) -> Result<&'a mut Component, Error> {
    let index: usize = find_component_index(components, name)?;
    return Ok(&mut components[index]);
}

fn find_component_index(components: &[Component], name: &str) -> Result<usize, Error> {
    let name: &str = non_empty(name, "component name")?;
    let mut matches = components
        .iter()
        .enumerate()
        .filter(|(_, component)| component.title == name);
    let (index, _): (usize, &Component) = matches.next().ok_or_else(|| {
        return Error::NotFound(format!("expected an existing component named {name:?}"));
    })?;
    if matches.next().is_some() {
        return Err(Error::Invalid(format!(
            "expected one component named {name:?}, but found multiple"
        )));
    }
    return Ok(index);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn component_input() -> CreateComponent {
        return CreateComponent {
            component_name: "Gateway".to_owned(),
            component_type: "service".to_owned(),
            description: "Example gateway".to_owned(),
            purpose: "Filter requests".to_owned(),
        };
    }

    fn control_input() -> MapControl {
        return MapControl {
            component_name: "Gateway".to_owned(),
            control: "sc-7".to_owned(),
        };
    }

    fn rule_input() -> MapRule {
        return MapRule {
            component_name: "Gateway".to_owned(),
            scanner: "zap".to_owned(),
            rule: "010038".to_owned(),
            control: "sc-7".to_owned(),
            justification: "Example justification".to_owned(),
            expiry: "2028-02-29".to_owned(),
        };
    }

    #[test]
    fn expiry_is_inclusive() {
        let (_, mut rule): (Scanner, RuleMapping) = rule_input().validated().unwrap();
        for (day, expected) in [(28, ExpiryState::Valid), (29, ExpiryState::Valid)] {
            let date: NaiveDate = NaiveDate::from_ymd_opt(2028, 2, day).unwrap();
            assert_eq!(classify_expiry(&rule, date).unwrap(), expected);
        }
        let next_day: NaiveDate = NaiveDate::from_ymd_opt(2028, 3, 1).unwrap();
        assert_eq!(
            classify_expiry(&rule, next_day).unwrap(),
            ExpiryState::Expired
        );
        rule.expiry = "2027-02-29".to_owned();
        assert!(matches!(
            classify_expiry(&rule, next_day),
            Err(Error::Invalid(_))
        ));
    }

    #[test]
    fn create_component_rejects_empty_fields_and_duplicates() {
        let mut components: Vec<Component> = Vec::new();
        for input in [
            CreateComponent {
                component_name: " ".to_owned(),
                ..component_input()
            },
            CreateComponent {
                component_type: " ".to_owned(),
                ..component_input()
            },
            CreateComponent {
                description: " ".to_owned(),
                ..component_input()
            },
            CreateComponent {
                purpose: " ".to_owned(),
                ..component_input()
            },
        ] {
            assert!(matches!(
                create_component(&mut components, &input),
                Err(Error::Invalid(_))
            ));
        }
        assert!(components.is_empty());

        let input: CreateComponent = component_input();
        create_component(&mut components, &input).unwrap();
        let original: Vec<Component> = components.clone();
        assert!(matches!(
            create_component(&mut components, &input),
            Err(Error::AlreadyExists(_))
        ));
        assert_eq!(components, original);
    }

    #[test]
    fn map_control_rejects_invalid_missing_and_duplicate_entries() {
        let mut components: Vec<Component> = Vec::new();
        let input: MapControl = control_input();
        assert!(matches!(
            map_control(&mut components, &input),
            Err(Error::NotFound(_))
        ));
        create_component(&mut components, &component_input()).unwrap();

        let empty: MapControl = MapControl {
            control: " ".to_owned(),
            ..control_input()
        };
        assert!(matches!(
            map_control(&mut components, &empty),
            Err(Error::Invalid(_))
        ));
        map_control(&mut components, &input).unwrap();
        assert!(matches!(
            map_control(&mut components, &input),
            Err(Error::AlreadyExists(_))
        ));
        assert_eq!(components[0].controls, ["sc-7"]);
    }

    #[test]
    fn mapping_operations_require_one_named_component() {
        let mut components: Vec<Component> = Vec::new();
        create_component(&mut components, &component_input()).unwrap();
        components.push(components[0].clone());
        let original: Vec<Component> = components.clone();
        assert!(matches!(
            map_control(&mut components, &control_input()),
            Err(Error::Invalid(_))
        ));
        assert!(matches!(
            map_rule(&mut components, &rule_input()),
            Err(Error::Invalid(_))
        ));
        assert_eq!(components, original);
    }

    #[test]
    fn map_rule_rejects_missing_component_and_control() {
        let mut components: Vec<Component> = Vec::new();
        let input: MapRule = rule_input();
        assert!(matches!(
            map_rule(&mut components, &input),
            Err(Error::NotFound(_))
        ));
        create_component(&mut components, &component_input()).unwrap();
        let original: Vec<Component> = components.clone();
        assert!(matches!(
            map_rule(&mut components, &input),
            Err(Error::NotFound(_))
        ));
        assert_eq!(components, original);
    }

    #[test]
    fn map_rule_rejects_invalid_inputs_and_normalized_duplicates() {
        let mut components: Vec<Component> = Vec::new();
        create_component(&mut components, &component_input()).unwrap();
        map_control(&mut components, &control_input()).unwrap();
        for input in [
            MapRule {
                scanner: "other".to_owned(),
                ..rule_input()
            },
            MapRule {
                rule: "10038a".to_owned(),
                ..rule_input()
            },
            MapRule {
                expiry: "2027-02-29".to_owned(),
                ..rule_input()
            },
            MapRule {
                expiry: "2028-2-29".to_owned(),
                ..rule_input()
            },
            MapRule {
                justification: " ".to_owned(),
                ..rule_input()
            },
        ] {
            assert!(matches!(
                map_rule(&mut components, &input),
                Err(Error::Invalid(_))
            ));
        }
        assert!(components[0].rules.is_empty());
        map_rule(&mut components, &rule_input()).unwrap();
        let original: Vec<Component> = components.clone();
        let duplicate: MapRule = MapRule {
            scanner: "ZAP".to_owned(),
            rule: "10038".to_owned(),
            ..rule_input()
        };
        assert!(matches!(
            map_rule(&mut components, &duplicate),
            Err(Error::AlreadyExists(_))
        ));
        assert_eq!(components, original);
        assert_eq!(components[0].rules[0].rules[0].id, "10038");
    }
}
