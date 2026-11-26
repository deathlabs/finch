use schemars::{Schema, schema_for};

use crate::model::MappingsFile;

/// Generate the JSON Schema for a [mappings file](GLOSSARY.md#mappings-file).
pub fn mappings() -> Schema {
    return schema_for!(MappingsFile);
}
