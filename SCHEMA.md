# Finch Schema

The Rust model generates the [JSON Schema](schema/mappings.schema.json) for the [mappings file](docs/src/content/GLOSSARY.md#mappings-file). [GLOSSARY.md](docs/src/content/GLOSSARY.md) defines the terms. [STATE_MACHINES.md](STATE_MACHINES.md) describes the target lifecycles.

Run `cargo run --quiet -- get schema > schema/mappings.schema.json` from the repository root to regenerate the schema.

## Rules the schema cannot express

These rules are enforced by the functions below. Loading a [mappings file](docs/src/content/GLOSSARY.md#mappings-file) checks its structure. It does not run the operations that add entries.

| Rule | Requirement | Enforcement | Tests |
| --- | --- | --- | --- |
| R1 | A new [component](docs/src/content/GLOSSARY.md#component) must have a nonblank name, type, description, and purpose. Its name must be unique in the file. | `ops::create_component` | `create_component_rejects_empty_fields_and_duplicates` |
| R2 | Adding an implemented [control](docs/src/content/GLOSSARY.md#control) or [mapping](docs/src/content/GLOSSARY.md#mapping) requires a name that identifies exactly one existing [component](docs/src/content/GLOSSARY.md#component). | `ops::find_component` | `map_control_rejects_invalid_missing_and_duplicate_entries`, `map_rule_rejects_missing_component_and_control`, `mapping_operations_require_one_named_component` |
| R3 | Adding an implemented [control](docs/src/content/GLOSSARY.md#control) requires a nonblank ID. The [component](docs/src/content/GLOSSARY.md#component) must not already list that ID. | `ops::map_control` | `map_control_rejects_invalid_missing_and_duplicate_entries` |
| R4 | A new [mapping](docs/src/content/GLOSSARY.md#mapping) must name a supported [scanner](docs/src/content/GLOSSARY.md#scanner), use a rule ID valid for that [scanner](docs/src/content/GLOSSARY.md#scanner), and give a nonblank [justification](docs/src/content/GLOSSARY.md#justification). | `catalog::Scanner::find`, `catalog::Scanner::normalize_rule`, `ops::MapRule::validated` | `validates_and_normalizes_catalog_rules`, `map_rule_rejects_invalid_inputs_and_normalized_duplicates` |
| R5 | A new [mapping](docs/src/content/GLOSSARY.md#mapping) must cite an implemented [control](docs/src/content/GLOSSARY.md#control) already listed by its [component](docs/src/content/GLOSSARY.md#component). The same [scanner](docs/src/content/GLOSSARY.md#scanner), normalized rule ID, and [control](docs/src/content/GLOSSARY.md#control) cannot be added twice to the same [component](docs/src/content/GLOSSARY.md#component). | `ops::map_rule`, `ops::require_control` | `map_rule_rejects_missing_component_and_control`, `map_rule_rejects_invalid_inputs_and_normalized_duplicates` |
| R6 | The [expiry](docs/src/content/GLOSSARY.md#expiry) of a new [mapping](docs/src/content/GLOSSARY.md#mapping) must be a real calendar date in YYYY-MM-DD format. | `ops::validate_expiry` | `map_rule_rejects_invalid_inputs_and_normalized_duplicates`, `expiry_is_inclusive` |
| R7 | A [mapping](docs/src/content/GLOSSARY.md#mapping) is Expired only when the supplied date is later than its [expiry](docs/src/content/GLOSSARY.md#expiry). | `ops::classify_expiry` | `expiry_is_inclusive` |
| R8 | Deleting a [component](docs/src/content/GLOSSARY.md#component) or [control](docs/src/content/GLOSSARY.md#control) with dependents requires explicit force. A forced deletion also removes the dependent [controls](docs/src/content/GLOSSARY.md#control) and [mappings](docs/src/content/GLOSSARY.md#mapping) for the selected [component](docs/src/content/GLOSSARY.md#component) or [control](docs/src/content/GLOSSARY.md#control). | `ops::delete_component`, `ops::delete_control` | `cli_delete_checks_dependencies_and_force_limits_the_cascade` |

## Proposed rules

* Require every [suppression](docs/src/content/GLOSSARY.md#suppression) to cite a [component](docs/src/content/GLOSSARY.md#component) that implements at least one [control](docs/src/content/GLOSSARY.md#control), so the output has a basis.
* Reject an Expired [mapping](docs/src/content/GLOSSARY.md#mapping) during generation and write no files, so expired [mappings](docs/src/content/GLOSSARY.md#mapping) cannot produce [suppressions](docs/src/content/GLOSSARY.md#suppression).
