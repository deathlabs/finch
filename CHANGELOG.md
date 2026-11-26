# Changelog

## [0.1.0] - 2026-11-25
### Added
- Added Rust-based CLI scaffolding.
- Add `--mode` with `cli`, `tui`, and `mcp` options. Use `cli` by default.
- Print a mode message for CLI and MCP.
- Add `finch create component --component-name NAME` to add a [component](GLOSSARY.md#component) to an existing [mappings file](GLOSSARY.md#mappings-file).
- Add `finch create mapping --component-name NAME --control ID` to associate a [control](GLOSSARY.md#control) with a [component](GLOSSARY.md#component).
- Use `finch create mapping` to record a [scanner](GLOSSARY.md#scanner) rule, [control](GLOSSARY.md#control), [justification](GLOSSARY.md#justification), and [expiry](GLOSSARY.md#expiry) for a [component](GLOSSARY.md#component).
- Use `--mode tui` to load an OSCAL [System Security Plan](GLOSSARY.md#system-security-plan), select [controls](GLOSSARY.md#control) and [scanner](GLOSSARY.md#scanner) rules for [components](GLOSSARY.md#component), and export a YAML [mappings file](GLOSSARY.md#mappings-file). The terminal interface uses `rusty-bubbletea` 2.0.8.
- Use `--ssp PATH` to open a plan when the terminal interface starts.
- Show a numbered part heading above each action. Start the step numbering at 1 within each part.
- Use bold for part and step numbers and regular weight for their titles.
- Update the [component](GLOSSARY.md#component) title, type, description, and purpose prompts.
- Clarify the path prompts for the [SSP](GLOSSARY.md#system-security-plan) and the [mappings file](GLOSSARY.md#mappings-file).
- End each step instruction with a period and the yes-or-no prompt with a question mark.
- Update the selection prompts for [controls](GLOSSARY.md#control), [scanners](GLOSSARY.md#scanner), and ZAP rules.
- Add `finch schema` to print the JSON Schema for a [mappings file](GLOSSARY.md#mappings-file). The previous and planned Rust release versions are both 0.1.0.
- Add date-based classification for [mappings](GLOSSARY.md#mapping). A [mapping](GLOSSARY.md#mapping) becomes Expired on the day after its [expiry](GLOSSARY.md#expiry).

### Changed

* Breaking CLI change in the Rust 0.1.0 development series. The previous and planned release versions are both 0.1.0. Use `finch tui`, `finch mcp`, and `finch create` with the `component`, `control`, or `rule` subcommand.
* Breaking schema change. Each [scanner](GLOSSARY.md#scanner) rule requires an ID, an implemented [control](GLOSSARY.md#control), a [justification](GLOSSARY.md#justification), and an [expiry](GLOSSARY.md#expiry). Unknown keys cause errors.
* The CLI and TUI share the same library for [mappings file](GLOSSARY.md#mappings-file) operations. The library reports duplicate entries and locks file updates.
* The TUI prompts for the implemented [control](GLOSSARY.md#control), [justification](GLOSSARY.md#justification), and [expiry](GLOSSARY.md#expiry) for each selected rule.
