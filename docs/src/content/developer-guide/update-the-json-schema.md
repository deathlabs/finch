# Update the JSON Schema

Read the [schema reference](https://github.com/deathlabs/finch/blob/main/SCHEMA.md) before changing the model. When model types change, regenerate the JSON Schema for the [mappings file](../GLOSSARY.md#mappings-file).

```bash
cargo run --quiet -- get schema > schema/mappings.schema.json
```
