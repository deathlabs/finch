use schemars::{Schema, schema_for};

use crate::model::MappingsFile;

/// Generate the JSON Schema for a [mappings file](https://github.com/deathlabs/finch/blob/main/docs/src/content/GLOSSARY.md#mappings-file).
pub fn mappings() -> Schema {
    return schema_for!(MappingsFile);
}
