# `finch`
 
[![CI Pipeline](https://github.com/deathlabs/finch/actions/workflows/ci.yml/badge.svg)](https://github.com/deathlabs/finch/actions/workflows/ci.yml)
 
Shape what your [scanners](GLOSSARY.md#scanner) can see.
 
## Overview

Finch gives [scanners](GLOSSARY.md#scanner) the [context](GLOSSARY.md#context) they need about your [hosting platform's](GLOSSARY.md#hosting-platform) security [controls](GLOSSARY.md#control) to identify and ignore false positives.
 
## Start the MCP Server

Run `finch start mcp` to start the Model Context Protocol (MCP) server.
Set your client's Streamable HTTP URL to `http://127.0.0.1:8765/mcp`.
To use a different port, run `finch start mcp --port 9000` and update the client URL.

Use `finch://schema/mappings` to read the JSON Schema for a [mappings file](GLOSSARY.md#mappings-file).

## [User](GLOSSARY.md#user) Flows
 
### Creating a [Mappings File](GLOSSARY.md#mappings-file)
 
This [user](GLOSSARY.md#user) flow is focused on the [security team](GLOSSARY.md#security-team) [user](GLOSSARY.md#user) persona and assumes they have a [System Security Plan (SSP)](GLOSSARY.md#system-security-plan) for their [hosting platform](GLOSSARY.md#hosting-platform).
 
**Step 1.** The [security team](GLOSSARY.md#security-team) identifies the [environment](GLOSSARY.md#environment) that the [mappings file](GLOSSARY.md#mappings-file) will cover.
 
**Step 2.** The [security team](GLOSSARY.md#security-team) reviews the [SSP](GLOSSARY.md#system-security-plan) to identify the [components](GLOSSARY.md#component) and implemented [controls](GLOSSARY.md#control) that apply to that [environment](GLOSSARY.md#environment).
 
**Step 3.** The [security team](GLOSSARY.md#security-team) creates a [mappings file](GLOSSARY.md#mappings-file) in the terminal interface (TUI). For each selected rule, choose an implemented [control](GLOSSARY.md#control) and enter a [justification](GLOSSARY.md#justification) and an [expiry](GLOSSARY.md#expiry) date in YYYY-MM-DD format.
 
**Open the Terminal Interface**

```bash
finch start tui --ssp ssp.json
```

![](docs/demo/tui.gif)
 
**Step 4.** The [security team](GLOSSARY.md#security-team) publishes their [mappings file](GLOSSARY.md#mappings-file).
 
### Proposing a [Component](GLOSSARY.md#component)
 
This [user](GLOSSARY.md#user) flow assumes the [security team](GLOSSARY.md#security-team) has published the [hosting platform's](GLOSSARY.md#hosting-platform) [System Security Plan](GLOSSARY.md#system-security-plan) ([SSP](GLOSSARY.md#system-security-plan)) so [users](GLOSSARY.md#user) can reference it. It also assumes a [mappings file](GLOSSARY.md#mappings-file) for the [hosting platform](GLOSSARY.md#hosting-platform) already exists.
 
**Step 1.** A [user](GLOSSARY.md#user) downloads the [SSP](GLOSSARY.md#system-security-plan) for their [hosting platform](GLOSSARY.md#hosting-platform).
 
**Step 2.** The [user](GLOSSARY.md#user) downloads the [mappings file](GLOSSARY.md#mappings-file) for their [hosting platform](GLOSSARY.md#hosting-platform).
 
**Step 3.** The [user](GLOSSARY.md#user) reviews the [SSP](GLOSSARY.md#system-security-plan) to identify which implemented [controls](GLOSSARY.md#control) the [component](GLOSSARY.md#component) they want to propose addresses.
 
**Step 4.** The [user](GLOSSARY.md#user) adds the [component](GLOSSARY.md#component) to their local copy of the [mappings file](GLOSSARY.md#mappings-file).
 
```bash
finch add --mappings-file mappings.yaml component \
  --component-name "$COMPONENT_NAME" \
  --type service \
  --description "$DESCRIPTION" \
  --purpose "$PURPOSE"
```

**Step 5.** The [user](GLOSSARY.md#user) proposes the change to the [security team](GLOSSARY.md#security-team).
 
**Step 6.** The [security team](GLOSSARY.md#security-team) reviews the proposal. They check that the [component](GLOSSARY.md#component) matches the entry in the [SSP](GLOSSARY.md#system-security-plan) and applies to the [environment](GLOSSARY.md#environment) in question.
 
**Step 7.** If the [security team](GLOSSARY.md#security-team) approves the proposal, they publish a new [mappings file version](GLOSSARY.md#mappings-file-version).
 
### Proposing a New Control-to-Component [Mapping](GLOSSARY.md#mapping)
 
This [user](GLOSSARY.md#user) flow assumes the [security team](GLOSSARY.md#security-team) has published the [SSP](GLOSSARY.md#system-security-plan) and a [mappings file](GLOSSARY.md#mappings-file) for the target [environment](GLOSSARY.md#environment). It also assumes the [component](GLOSSARY.md#component) in question already exists in the [mappings file](GLOSSARY.md#mappings-file).
 
**Step 1.** A [user](GLOSSARY.md#user) downloads the [SSP](GLOSSARY.md#system-security-plan).
 
**Step 2.** The [user](GLOSSARY.md#user) downloads the [mappings file](GLOSSARY.md#mappings-file).
 
**Step 3.** The [user](GLOSSARY.md#user) reviews the [SSP](GLOSSARY.md#system-security-plan) to identify an implemented [control](GLOSSARY.md#control) that the [component](GLOSSARY.md#component) supports in the target [environment](GLOSSARY.md#environment).
 
**Step 4.** The [user](GLOSSARY.md#user) runs `finch` to associate the [control](GLOSSARY.md#control) with the [component](GLOSSARY.md#component) in their local copy of the [mappings file](GLOSSARY.md#mappings-file).
 
```bash
finch add --mappings-file mappings.yaml control \
  --component-name "$COMPONENT_NAME" \
  --control "$CONTROL_ID"
```
 
**Step 5.** The [user](GLOSSARY.md#user) proposes the change to the [security team](GLOSSARY.md#security-team).
 
**Step 6.** The [security team](GLOSSARY.md#security-team) reviews the proposal. They check that the [SSP](GLOSSARY.md#system-security-plan) documents the [component's](GLOSSARY.md#component) support for the implemented [control](GLOSSARY.md#control) in the target [environment](GLOSSARY.md#environment).
 
**Step 7.** If the [security team](GLOSSARY.md#security-team) approves the proposal, they publish a new [mappings file version](GLOSSARY.md#mappings-file-version).
 
### Proposing a New Rule-to-Component [Mapping](GLOSSARY.md#mapping)
 
This [user](GLOSSARY.md#user) flow assumes the [security team](GLOSSARY.md#security-team) has published the [SSP](GLOSSARY.md#system-security-plan) and a [mappings file](GLOSSARY.md#mappings-file) for the target [environment](GLOSSARY.md#environment). It also assumes the [component](GLOSSARY.md#component) in question already exists in the [mappings file](GLOSSARY.md#mappings-file).
 
**Step 1.** A [user](GLOSSARY.md#user) downloads the [SSP](GLOSSARY.md#system-security-plan).
 
**Step 2.** The [user](GLOSSARY.md#user) downloads the [mappings file](GLOSSARY.md#mappings-file).
 
**Step 3.** The [user](GLOSSARY.md#user) reviews the [SSP](GLOSSARY.md#system-security-plan) to identify an implemented [control](GLOSSARY.md#control) that mitigates the [risk](GLOSSARY.md#risk) associated with the [scanner](GLOSSARY.md#scanner) rule.
 
**Step 4.** The [user](GLOSSARY.md#user) runs `finch` to add the rule to their local copy of the [mappings file](GLOSSARY.md#mappings-file). The [component](GLOSSARY.md#component) must already list the [control](GLOSSARY.md#control).
 
```bash
finch add --mappings-file mappings.yaml rule \
  --component-name "$COMPONENT_NAME" \
  --control "$CONTROL_ID" \
  --scanner "$SCANNER" \
  --rule "$RULE_ID" \
  --justification "$JUSTIFICATION" \
  --expiry "$EXPIRY_DATE"
```
 
**Step 5.** The [user](GLOSSARY.md#user) proposes the change to the [security team](GLOSSARY.md#security-team).
 
**Step 6.** The [security team](GLOSSARY.md#security-team) reviews the proposal. They check that the [SSP](GLOSSARY.md#system-security-plan) documents the [control](GLOSSARY.md#control) as implemented and that the [control](GLOSSARY.md#control) mitigates the [risk](GLOSSARY.md#risk) associated with the [scanner](GLOSSARY.md#scanner) rule.
 
**Step 7.** If the [security team](GLOSSARY.md#security-team) approves the proposal, they publish a new [mappings file version](GLOSSARY.md#mappings-file-version).
 
### Viewing Entries

Print a [component](GLOSSARY.md#component) in JSON, including its implemented [controls](GLOSSARY.md#control) and rule [mappings](GLOSSARY.md#mapping).

```bash
finch get component --component-name "$COMPONENT_NAME"
finch get control --component-name "$COMPONENT_NAME" --control "$CONTROL_ID"
finch get rule --component-name "$COMPONENT_NAME" --control "$CONTROL_ID" --scanner "$SCANNER" --rule "$RULE_ID"
```

The [control](GLOSSARY.md#control) lookup prints the stored ID. The rule lookup prints the [mapping](GLOSSARY.md#mapping) for the specified [scanner](GLOSSARY.md#scanner) and [control](GLOSSARY.md#control).

Use `-o yaml` or `--output-format yaml` to print YAML. Use `--mappings-file PATH` to read a [mappings file](GLOSSARY.md#mappings-file) other than `mappings.yaml`.

### Deleting Entries

A [component](GLOSSARY.md#component) with implemented [controls](GLOSSARY.md#control) cannot be deleted unless you pass `-f`. A [control](GLOSSARY.md#control) with dependent [mappings](GLOSSARY.md#mapping) also requires `-f`. Finch lists the dependents when it rejects a deletion.

With `-f`, deleting a [component](GLOSSARY.md#component) also removes its [controls](GLOSSARY.md#control) and [mappings](GLOSSARY.md#mapping). Deleting a [control](GLOSSARY.md#control) with `-f` removes the [mappings](GLOSSARY.md#mapping) that cite it on that [component](GLOSSARY.md#component).

```bash
finch delete component --component-name "$COMPONENT_NAME" -f
finch delete control --component-name "$COMPONENT_NAME" --control "$CONTROL_ID" -f
finch delete rule --component-name "$COMPONENT_NAME" --control "$CONTROL_ID" --scanner "$SCANNER" --rule "$RULE_ID"
```

### [Mappings File](GLOSSARY.md#mappings-file) Schema

Every rule entry is an object. Include an `id`, [`control`](GLOSSARY.md#control), [`justification`](GLOSSARY.md#justification), and [`expiry`](GLOSSARY.md#expiry) for each rule. Finch rejects unrecognized keys. See [the example](mappings.yaml).

The `add` command reports an error if the entry already exists. The `--mappings-file` option defaults to `mappings.yaml`.

### Generating a New [Suppression](GLOSSARY.md#suppression)

The [suppression](GLOSSARY.md#suppression) generator is pending in the Rust rewrite. The demo shows how a pipeline uses a [mappings file version](GLOSSARY.md#mappings-file-version) for its [environment](GLOSSARY.md#environment) to configure a [scanner](GLOSSARY.md#scanner).

![](docs/demo/generate.gif)
