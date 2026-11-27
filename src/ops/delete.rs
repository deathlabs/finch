use clap::Args;
use schemars::JsonSchema;
use serde::Deserialize;

use super::{find_component, find_component_index, find_rule_indices, non_empty, require_control};
use crate::error::Error;
use crate::model::{Component, ScannerRules};

/// Input for deleting a [component](GLOSSARY.md#component).
#[derive(Args, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteComponent {
    /// Name of the [component](GLOSSARY.md#component).
    #[arg(long, help = "Name of the component")]
    pub component_name: String,
    /// Delete dependent [controls](GLOSSARY.md#control) and [mappings](GLOSSARY.md#mapping).
    #[arg(short = 'f', help = "Delete dependent controls and rule mappings")]
    #[serde(default)]
    pub force: bool,
}

/// Input for deleting an implemented [control](GLOSSARY.md#control).
#[derive(Args, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteControl {
    /// Name of the [component](GLOSSARY.md#component).
    #[arg(long, help = "Name of the component")]
    pub component_name: String,
    /// ID of the implemented [control](GLOSSARY.md#control).
    #[arg(long, help = "ID of the implemented control")]
    pub control: String,
    /// Delete dependent [mappings](GLOSSARY.md#mapping).
    #[arg(short = 'f', help = "Delete dependent rule mappings")]
    #[serde(default)]
    pub force: bool,
}

/// Input for deleting a [mapping](GLOSSARY.md#mapping).
#[derive(Args, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteRule {
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

/// Delete one component. Force permits deletion of its controls and mappings.
pub fn delete_component(
    components: &mut Vec<Component>,
    input: &DeleteComponent,
) -> Result<(), Error> {
    let index: usize = find_component_index(components, &input.component_name)?;
    let component: &Component = &components[index];
    if !input.force && !component.controls.is_empty() {
        return Err(Error::HasDependents(format!(
            "cannot delete component {:?}: controls depend on it ({})",
            component.title,
            component.controls.join(", ")
        )));
    }
    if !input.force && component.rules.iter().any(|group| !group.rules.is_empty()) {
        return Err(Error::HasDependents(format!(
            "cannot delete component {:?}: rule mappings depend on it",
            component.title
        )));
    }
    components.remove(index);
    return Ok(());
}

/// Delete a control from one component. Force also deletes mappings that cite it.
pub fn delete_control(components: &mut [Component], input: &DeleteControl) -> Result<(), Error> {
    let control: &str = non_empty(&input.control, "control ID")?;
    let component: &mut Component = find_component(components, &input.component_name)?;
    require_control(component, control)?;
    if input.force {
        component.rules.retain_mut(|group| {
            let was_empty: bool = group.rules.is_empty();
            group.rules.retain(|rule| rule.control != control);
            return !group.rules.is_empty() || was_empty;
        });
    } else {
        let mut dependents: Vec<String> = Vec::new();
        for group in &component.rules {
            for rule in &group.rules {
                if rule.control == control {
                    dependents.push(format!("{} {}", group.tool, rule.id));
                }
            }
        }
        if !dependents.is_empty() {
            return Err(Error::HasDependents(format!(
                "cannot delete control {control:?} from {:?}: rules depend on it ({})",
                component.title,
                dependents.join(", ")
            )));
        }
    }
    component.controls.retain(|existing| existing != control);
    return Ok(());
}

/// Delete a mapping by component, scanner, normalized rule ID, and control.
pub fn delete_rule(components: &mut [Component], input: &DeleteRule) -> Result<(), Error> {
    let component: &mut Component = find_component(components, &input.component_name)?;
    let (group_index, rule_index): (usize, usize) =
        find_rule_indices(component, &input.control, &input.scanner, &input.rule)?;
    let group: &mut ScannerRules = &mut component.rules[group_index];
    group.rules.remove(rule_index);
    if group.rules.is_empty() {
        component.rules.remove(group_index);
    }
    return Ok(());
}
