use std::path::Path;

use schemars::schema_for;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::error::Error;
use crate::model::MappingsFile;
use crate::{ops, store};

macro_rules! define_tools {
    ($($name:literal, $description:literal, $input:ty, $access:ident, $operation:ident;)+) => {
        pub(super) fn list() -> Value {
            return json!({"tools": [$(
                {
                    "name": $name,
                    "description": $description,
                    "inputSchema": schema_for!($input)
                }
            ),+]});
        }

        pub(super) fn call(params: &Value, path: &Path) -> Result<Value, String> {
            let name: &str = params["name"].as_str()
                .ok_or("Expected a tool name string.")?;
            let empty_arguments: Value = json!({});
            let arguments: &Value = params.get("arguments").unwrap_or(&empty_arguments);
            if !arguments.is_object() {
                return Err("Expected a tool arguments object.".into());
            }
            let result: Result<String, Error> = match name {
                $($name => $access(path, parse_arguments::<$input>(arguments)?, |document, input| {
                    return invoke!($access, document, input, $operation);
                }),)+
                _ => return Err(format!("Expected a listed tool name, got {name:?}.")),
            };
            return Ok(match result {
                Ok(text) => tool_result(text, false),
                Err(error) => tool_result(error.to_string(), true),
            });
        }
    };
}

macro_rules! invoke {
    (read, $document:ident, $input:ident, $operation:ident) => {
        ops::$operation(&$document.components, $input).map(|entry| {
            return json!(entry).to_string();
        })
    };
    (write, $document:ident, $input:ident, $operation:ident) => {
        ops::$operation(&mut $document.components, $input)
    };
}

define_tools! {
    "get_component", "Get a component and its controls and rule mappings.", ops::GetComponent, read, get_component;
    "get_control", "Get an implemented control ID.", ops::GetControl, read, get_control;
    "get_rule", "Get a scanner rule mapping.", ops::GetRule, read, get_rule;
    "add_component", "Add a component with a unique name.", ops::CreateComponent, write, create_component;
    "add_control", "Add an implemented control to a component.", ops::MapControl, write, map_control;
    "add_rule", "Add a scanner rule mapping for an implemented control.", ops::MapRule, write, map_rule;
    "update_component", "Change a component name, type, description, or purpose.", ops::UpdateComponent, write, update_component;
    "delete_component", "Delete a component. Set force to delete its dependent entries.", ops::DeleteComponent, write, delete_component;
    "delete_control", "Delete an implemented control. Set force to delete dependent rule mappings.", ops::DeleteControl, write, delete_control;
    "delete_rule", "Delete a scanner rule mapping.", ops::DeleteRule, write, delete_rule;
}

fn parse_arguments<T: DeserializeOwned>(arguments: &Value) -> Result<T, String> {
    return T::deserialize(arguments).map_err(|error| {
        return format!("Expected arguments matching the tool input schema: {error}");
    });
}

fn read<T>(
    path: &Path,
    input: T,
    operation: impl FnOnce(&MappingsFile, &T) -> Result<String, Error>,
) -> Result<String, Error> {
    return operation(&store::load(path)?, &input);
}

fn write<T>(
    path: &Path,
    input: T,
    operation: impl FnOnce(&mut MappingsFile, &T) -> Result<(), Error>,
) -> Result<String, Error> {
    store::update(path, |document| {
        return operation(document, &input);
    })?;
    return Ok("Saved the mappings file.".into());
}

fn tool_result(text: String, is_error: bool) -> Value {
    return json!({"content": [{"type": "text", "text": text}], "isError": is_error});
}
