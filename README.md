# `finch`
 
[![CI Pipeline](https://github.com/deathlabs/finch/actions/workflows/check.yml/badge.svg)](https://github.com/deathlabs/finch/actions/workflows/check.yml)  
 
Shape what your [scanners](GLOSSARY.md) can see. 
 
## Overview
Finch gives [scanners](GLOSSARY.md) the [context](GLOSSARY.md) they need about your [hosting platform's](GLOSSARY.md) security [controls](GLOSSARY.md) to identify and ignore false positives. 
 
## User Flows
 
### Creating a Mappings File
 
This user flow is focused on the [security team](GLOSSARY.md#security-team) user persona and assumes they have a [System Security Plan (SSP)](GLOSSARY.md#system-security-plan) for their [hosting platform](GLOSSARY.md#hosting-platform).
 
**Step 1.** The [security team](GLOSSARY.md#security-team) identifies the [environment](GLOSSARY.md#environment) that the [mappings file](GLOSSARY.md#mappings-file) will cover.
 
**Step 2.** The [security team](GLOSSARY.md#security-team) reviews the [SSP](GLOSSARY.md#system-security-plan) to identify the [components](GLOSSARY.md#component) and implemented [controls](GLOSSARY.md#control) that apply to that [environment](GLOSSARY.md#environment).
 
**Step 3.** The [security team](GLOSSARY.md#security-team) creates a [mappings file](GLOSSARY.md#mappings-file) using one of `finch`'s two modes: the Terminal User Interface (TUI) or the Model Context Protocol (MCP) server.
 
**How to Invoke TUI Mode**
```bash
finch --mode tui
```

![](docs/demo/tui.gif)
 
**How to Invoke the MCP Server**
```bash
finch --mode mcp
```
 
**Step 4.** The [security team](GLOSSARY.md#security-team) publishes their [mappings file](GLOSSARY.md#mappings-file).
 
### Proposing a Component
 
This user flow assumes the [security team](GLOSSARY.md#security-team) has published the [hosting platform's](GLOSSARY.md#hosting-platform) [System Security Plan (SSP)](GLOSSARY.md#system-security-plan) so [users](GLOSSARY.md#user) can reference it. It also assumes a [mappings file](GLOSSARY.md#mappings-file) for the [hosting platform](GLOSSARY.md#hosting-platform) already exists.
 
**Step 1.** A [user](GLOSSARY.md#user) downloads the [SSP](GLOSSARY.md#system-security-plan) for their [hosting platform](GLOSSARY.md#hosting-platform).
 
**Step 2.** The [user](GLOSSARY.md#user) downloads the [mappings file](GLOSSARY.md#mappings-file) for their [hosting platform](GLOSSARY.md#hosting-platform).
 
**Step 3.** The [user](GLOSSARY.md#user) reviews the [SSP](GLOSSARY.md#system-security-plan) to identify which implemented [controls](GLOSSARY.md#control) the [component](GLOSSARY.md#component) they want to propose addresses.
 
**Step 4.** The [user](GLOSSARY.md#user) adds the [component](GLOSSARY.md#component) to their local copy of the [mappings file](GLOSSARY.md#mappings-file).
 
```bash
finch create component \
  --mappings-file mappings.yaml \
  --component "$COMPONENT_NAME" 
```
 
**Step 5.** The [user](GLOSSARY.md#user) proposes the change to the [security team](GLOSSARY.md#security-team).
 
**Step 6.** The [security team](GLOSSARY.md#security-team) reviews the proposal. They check that the [component](GLOSSARY.md#component) matches the entry in the [SSP](GLOSSARY.md#system-security-plan) and applies to the [environment](GLOSSARY.md#environment) in question.
 
**Step 7.** If the [security team](GLOSSARY.md#security-team) approves the proposal, they publish a new [mappings file version](GLOSSARY.md#mappings-file-version).
 
### Proposing a New Control-to-Component Mapping
 
This user flow assumes the [security team](GLOSSARY.md#security-team) has published the [SSP](GLOSSARY.md#system-security-plan) and a [mappings file](GLOSSARY.md#mappings-file) for the target [environment](GLOSSARY.md#environment). It also assumes the [component](GLOSSARY.md#component) in question already exists in the [mappings file](GLOSSARY.md#mappings-file).
 
**Step 1.** A [user](GLOSSARY.md#user) downloads the [SSP](GLOSSARY.md#system-security-plan).
 
**Step 2.** The [user](GLOSSARY.md#user) downloads the [mappings file](GLOSSARY.md#mappings-file).
 
**Step 3.** The [user](GLOSSARY.md#user) reviews the [SSP](GLOSSARY.md#system-security-plan) to identify an implemented [control](GLOSSARY.md#control) that the [component](GLOSSARY.md#component) supports in the target [environment](GLOSSARY.md#environment).
 
**Step 4.** The [user](GLOSSARY.md#user) runs `finch` to associate the [control](GLOSSARY.md#control) with the [component](GLOSSARY.md#component) in their local copy of the [mappings file](GLOSSARY.md#mappings-file).
 
```bash
finch create mapping \
  --mappings-file mappings.yaml \
  --component "$COMPONENT_UUID" \
  --control "$CONTROL_ID"
```
 
**Step 5.** The [user](GLOSSARY.md#user) proposes the change to the [security team](GLOSSARY.md#security-team).
 
**Step 6.** The [security team](GLOSSARY.md#security-team) reviews the proposal. They check that the [SSP](GLOSSARY.md#system-security-plan) documents the [component's](GLOSSARY.md#component) support for the implemented [control](GLOSSARY.md#control) in the target [environment](GLOSSARY.md#environment).
 
**Step 7.** If the [security team](GLOSSARY.md#security-team) approves the proposal, they publish a new [mappings file version](GLOSSARY.md#mappings-file-version).
 
### Proposing a New Rule-to-Component Mapping
 
This user flow assumes the [security team](GLOSSARY.md#security-team) has published the [SSP](GLOSSARY.md#system-security-plan) and a [mappings file](GLOSSARY.md#mappings-file) for the target [environment](GLOSSARY.md#environment). It also assumes the [component](GLOSSARY.md#component) in question already exists in the [mappings file](GLOSSARY.md#mappings-file).
 
**Step 1.** A [user](GLOSSARY.md#user) downloads the [SSP](GLOSSARY.md#system-security-plan).
 
**Step 2.** The [user](GLOSSARY.md#user) downloads the [mappings file](GLOSSARY.md#mappings-file).
 
**Step 3.** The [user](GLOSSARY.md#user) reviews the [SSP](GLOSSARY.md#system-security-plan) to identify an implemented [control](GLOSSARY.md#control) that mitigates the [risk](GLOSSARY.md#risk) associated with the [scanner](GLOSSARY.md#scanner) rule.
 
**Step 4.** The [user](GLOSSARY.md#user) runs `finch` to add a rule-to-[component](GLOSSARY.md#component) [mapping](GLOSSARY.md#mapping) to their local copy of the [mappings file](GLOSSARY.md#mappings-file).
 
```bash
finch create mapping \
  --mappings-file mappings.yaml \
  --component "$COMPONENT_UUID" \
  --control "$CONTROL_ID" \
  --scanner "$SCANNER" \
  --rule "$RULE_ID" \
  --justification "$JUSTIFICATION" \
  --expiry "$EXPIRY_DATE"
```
 
**Step 5.** The [user](GLOSSARY.md#user) proposes the change to the [security team](GLOSSARY.md#security-team).
 
**Step 6.** The [security team](GLOSSARY.md#security-team) reviews the proposal. They check that the [SSP](GLOSSARY.md#system-security-plan) documents the [control](GLOSSARY.md#control) as implemented and that the [control](GLOSSARY.md#control) mitigates the [risk](GLOSSARY.md#risk) associated with the [scanner](GLOSSARY.md#scanner) rule.
 
**Step 7.** If the [security team](GLOSSARY.md#security-team) approves the proposal, they publish a new [mappings file version](GLOSSARY.md#mappings-file-version).
 
### Generating a New Suppression
 
This user flow assumes the [security team](GLOSSARY.md#security-team) has published a [mappings file version](GLOSSARY.md#mappings-file-version) for the [environment](GLOSSARY.md#environment) in question. It also assumes the [user](GLOSSARY.md#user) has an app with a Continuous Integration pipeline that runs a [scanner](GLOSSARY.md#scanner).
 
**Step 1.** A [user](GLOSSARY.md#user) selects the [mappings file version](GLOSSARY.md#mappings-file-version) that matches their app's [environment](GLOSSARY.md#environment).
 
**Step 2.** The [user](GLOSSARY.md#user) configures their app's pipeline to download that [mappings file version](GLOSSARY.md#mappings-file-version) before the [scanner](GLOSSARY.md#scanner) runs.
 
**Step 3.** The [user](GLOSSARY.md#user) adds a `finch` step to their pipeline for each [scanner](GLOSSARY.md#scanner) rule they want to suppress.
 
```bash
finch create suppression \
  --mappings-file mappings.yaml \
  --scanner "$SCANNER" \
  --rule "$RULE_ID"
```

![](docs/demo/generate.gif)
 
**Step 4.** `finch` checks the [mappings file](GLOSSARY.md#mappings-file) for a [mapping](GLOSSARY.md#mapping) that matches the [scanner](GLOSSARY.md#scanner) and rule. It skips [mappings](GLOSSARY.md#mapping) that have expired.
 
**Step 5.** If `finch` finds a valid [mapping](GLOSSARY.md#mapping), it generates a [suppression](GLOSSARY.md#suppression) in the format the [scanner](GLOSSARY.md#scanner) requires.
 
**Step 6.** The pipeline passes the [suppression](GLOSSARY.md#suppression) to the [scanner](GLOSSARY.md#scanner), and the [scanner](GLOSSARY.md#scanner) leaves the suppressed rule out of its findings for the app scan.
 