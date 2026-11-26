# Finch Schema

The Rust model generates the [JSON Schema](schema/mappings.schema.json) for the [mappings file](GLOSSARY.md#mappings-file). [GLOSSARY.md](GLOSSARY.md) defines the terms. [STATE_MACHINES.md](STATE_MACHINES.md) describes the target lifecycles.

Run `cargo run --quiet -- schema > schema/mappings.schema.json` from the repository root to regenerate the schema.

## Rules the schema cannot express

These rules are enforced by the functions below. Loading a [mappings file](GLOSSARY.md#mappings-file) checks its structure. It does not run the operations that add entries.

| Rule | Requirement | Enforcement | Tests |
| --- | --- | --- | --- |
| R1 | A new [component](GLOSSARY.md#component) must have a nonblank name, type, description, and purpose. Its name must be unique in the file. | `ops::create_component` | `create_component_rejects_empty_fields_and_duplicates` |
| R2 | Adding an implemented [control](GLOSSARY.md#control) or [mapping](GLOSSARY.md#mapping) requires a name that identifies exactly one existing [component](GLOSSARY.md#component). | `ops::find_component` | `map_control_rejects_invalid_missing_and_duplicate_entries`, `map_rule_rejects_missing_component_and_control`, `mapping_operations_require_one_named_component` |
| R3 | Adding an implemented [control](GLOSSARY.md#control) requires a nonblank ID. The [component](GLOSSARY.md#component) must not already list that ID. | `ops::map_control` | `map_control_rejects_invalid_missing_and_duplicate_entries` |
| R4 | A new [mapping](GLOSSARY.md#mapping) must name a supported [scanner](GLOSSARY.md#scanner), use a rule ID valid for that [scanner](GLOSSARY.md#scanner), and give a nonblank [justification](GLOSSARY.md#justification). | `catalog::Scanner::find`, `catalog::Scanner::normalize_rule`, `ops::MapRule::validated` | `validates_and_normalizes_catalog_rules`, `map_rule_rejects_invalid_inputs_and_normalized_duplicates` |
| R5 | A new [mapping](GLOSSARY.md#mapping) must cite an implemented [control](GLOSSARY.md#control) already listed by its [component](GLOSSARY.md#component). The same [scanner](GLOSSARY.md#scanner), normalized rule ID, and [control](GLOSSARY.md#control) cannot be added twice to the same [component](GLOSSARY.md#component). | `ops::map_rule`, `ops::require_control` | `map_rule_rejects_missing_component_and_control`, `map_rule_rejects_invalid_inputs_and_normalized_duplicates` |
| R6 | The [expiry](GLOSSARY.md#expiry) of a new [mapping](GLOSSARY.md#mapping) must be a real calendar date in YYYY-MM-DD format. | `ops::validate_expiry` | `map_rule_rejects_invalid_inputs_and_normalized_duplicates`, `expiry_is_inclusive` |
| R7 | A [mapping](GLOSSARY.md#mapping) is Expired only when the supplied date is later than its [expiry](GLOSSARY.md#expiry). | `ops::classify_expiry` | `expiry_is_inclusive` |

## Proposed rules

* Require every [suppression](GLOSSARY.md#suppression) to cite a [component](GLOSSARY.md#component) that implements at least one [control](GLOSSARY.md#control), so the output has a basis.
* Reject an Expired [mapping](GLOSSARY.md#mapping) during generation and write no files, so expired [mappings](GLOSSARY.md#mapping) cannot produce [suppressions](GLOSSARY.md#suppression).
