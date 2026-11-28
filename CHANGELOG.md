# Changelog

## [0.1.0] - 2026-11-25
### Added
- Add MCP tools to get, add, and delete [components](docs/src/content/GLOSSARY.md#component), [controls](docs/src/content/GLOSSARY.md#control), and rule [mappings](docs/src/content/GLOSSARY.md#mapping). Use `update_component` to edit a [component](docs/src/content/GLOSSARY.md#component). Use `finch start mcp --mappings-file PATH` to select a [mappings file](docs/src/content/GLOSSARY.md#mappings-file). The previous and planned Rust release versions are both 0.1.0.
- Added Rust-based CLI scaffolding.
- Add `--mode` with `cli`, `tui`, and `mcp` options. Use `cli` by default.
- Print a mode message for CLI.
- Add `finch create component --component-name NAME` to add a [component](docs/src/content/GLOSSARY.md#component) to an existing [mappings file](docs/src/content/GLOSSARY.md#mappings-file).
- Add `finch create mapping --component-name NAME --control ID` to associate a [control](docs/src/content/GLOSSARY.md#control) with a [component](docs/src/content/GLOSSARY.md#component).
- Use `finch create mapping` to record a [scanner](docs/src/content/GLOSSARY.md#scanner) rule, [control](docs/src/content/GLOSSARY.md#control), [justification](docs/src/content/GLOSSARY.md#justification), and [expiry](docs/src/content/GLOSSARY.md#expiry) for a [component](docs/src/content/GLOSSARY.md#component).
- Use `--mode tui` to load an OSCAL [System Security Plan](docs/src/content/GLOSSARY.md#system-security-plan), select [controls](docs/src/content/GLOSSARY.md#control) and [scanner](docs/src/content/GLOSSARY.md#scanner) rules for [components](docs/src/content/GLOSSARY.md#component), and export a YAML [mappings file](docs/src/content/GLOSSARY.md#mappings-file). The terminal interface uses `rusty-bubbletea` 2.0.8.
- Use `--ssp PATH` to open a plan when the terminal interface starts.
- Show a numbered part heading above each action. Start the step numbering at 1 within each part.
- Use bold for part and step numbers and regular weight for their titles.
- Update the [component](docs/src/content/GLOSSARY.md#component) title, type, description, and purpose prompts.
- Clarify the path prompts for the [SSP](docs/src/content/GLOSSARY.md#system-security-plan) and the [mappings file](docs/src/content/GLOSSARY.md#mappings-file).
- End each step instruction with a period and the yes-or-no prompt with a question mark.
- Update the selection prompts for [controls](docs/src/content/GLOSSARY.md#control), [scanners](docs/src/content/GLOSSARY.md#scanner), and ZAP rules.
- Add `finch get schema` to print the JSON Schema for a [mappings file](docs/src/content/GLOSSARY.md#mappings-file). The previous and planned Rust release versions are both 0.1.0.
- Add date-based classification for [mappings](docs/src/content/GLOSSARY.md#mapping). A [mapping](docs/src/content/GLOSSARY.md#mapping) becomes Expired on the day after its [expiry](docs/src/content/GLOSSARY.md#expiry).

- Add `finch delete` for [components](docs/src/content/GLOSSARY.md#component), [controls](docs/src/content/GLOSSARY.md#control), and rule [mappings](docs/src/content/GLOSSARY.md#mapping). Pass `-f` to delete dependent entries. The previous and planned Rust release versions are both 0.1.0.
- Add `finch get component` to print a [component](docs/src/content/GLOSSARY.md#component), its implemented [controls](docs/src/content/GLOSSARY.md#control), and its rule [mappings](docs/src/content/GLOSSARY.md#mapping) in JSON by default. Use `-o yaml` to print YAML. The previous and planned Rust release versions are both 0.1.0.
- Add `finch get control` and `finch get rule` to print the stored [control](docs/src/content/GLOSSARY.md#control) ID or rule [mapping](docs/src/content/GLOSSARY.md#mapping). Both support JSON and YAML. The previous and planned Rust release versions are both 0.1.0.

### Changed

* Start the MCP server over Streamable HTTP with `finch start mcp`. The server provides the JSON Schema for a [mappings file](docs/src/content/GLOSSARY.md#mappings-file). The previous and planned Rust release versions are both 0.1.0.
* Write MCP server logs as JSON to standard error. Record every HTTP request and include the JSON-RPC payload when parsed. Omit tool arguments from the log. The previous and planned Rust release versions are both 0.1.0.


* Breaking CLI change in the Rust 0.1.0 development series. The previous and planned release versions are both 0.1.0. Use `finch start tui`, `finch start mcp`, `finch add component`, `finch add control`, and `finch add rule`.
* Breaking schema change. Each [scanner](docs/src/content/GLOSSARY.md#scanner) rule requires an ID, an implemented [control](docs/src/content/GLOSSARY.md#control), a [justification](docs/src/content/GLOSSARY.md#justification), and an [expiry](docs/src/content/GLOSSARY.md#expiry). Unknown keys cause errors.
* The CLI and TUI share the same library for [mappings file](docs/src/content/GLOSSARY.md#mappings-file) operations. The library reports duplicate entries and locks file updates.
* The TUI prompts for the implemented [control](docs/src/content/GLOSSARY.md#control), [justification](docs/src/content/GLOSSARY.md#justification), and [expiry](docs/src/content/GLOSSARY.md#expiry) for each selected rule.
