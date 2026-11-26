# `finch`
 
[![CI Pipeline](https://github.com/deathlabs/finch/actions/workflows/ci.yml/badge.svg)](https://github.com/deathlabs/finch/actions/workflows/ci.yml)
 
Shape what your [scanners](GLOSSARY.md#scanner) can see.
 
## Overview
Finch gives scanners the [context](GLOSSARY.md#context) they need about your [hosting platform's](GLOSSARY.md#hosting-platform) security [controls](GLOSSARY.md#control) to identify and ignore false positives.
 
## User Flows
 
### Creating a Mappings File
 
This user flow is focused on the [security team](GLOSSARY.md#security-team) user persona and assumes they have a [System Security Plan (SSP)](GLOSSARY.md#system-security-plan) for their hosting platform.
 
**Step 1.** The security team identifies the [environment](GLOSSARY.md#environment) that the [mappings file](GLOSSARY.md#mappings-file) will cover.
 
**Step 2.** The security team reviews the SSP to identify the [components](GLOSSARY.md#component) and implemented controls that apply to that environment.
 
**Step 3.** The security team creates a mappings file using one of `finch`'s two modes: the Terminal User Interface (TUI) or the Model Context Protocol (MCP) server.
 
**How to Invoke TUI Mode**
```bash
finch --mode tui
```

![](docs/demo/tui.gif)
 
**How to Invoke the MCP Server**
```bash
finch --mode mcp
```
 
**Step 4.** The security team publishes their mappings file.
 
### Proposing a Component
 
This user flow assumes the security team has published the hosting platform's System Security Plan (SSP) so [users](GLOSSARY.md#user) can reference it. It also assumes a mappings file for the hosting platform already exists.
 
**Step 1.** A user downloads the SSP for their hosting platform.
 
**Step 2.** The user downloads the mappings file for their hosting platform.
 
**Step 3.** The user reviews the SSP to identify which implemented controls the component they want to propose addresses.
 
**Step 4.** The user adds the component to their local copy of the mappings file.
 
```bash
finch create component \
  --mappings-file mappings.yaml \
  --component-name "$COMPONENT_NAME"
```

**Step 5.** The user proposes the change to the security team.
 
**Step 6.** The security team reviews the proposal. They check that the component matches the entry in the SSP and applies to the environment in question.
 
**Step 7.** If the security team approves the proposal, they publish a new [mappings file version](GLOSSARY.md#mappings-file-version).
 
### Proposing a New Control-to-Component Mapping
 
This user flow assumes the security team has published the SSP and a mappings file for the target environment. It also assumes the component in question already exists in the mappings file.
 
**Step 1.** A user downloads the SSP.
 
**Step 2.** The user downloads the mappings file.
 
**Step 3.** The user reviews the SSP to identify an implemented control that the component supports in the target environment.
 
**Step 4.** The user runs `finch` to associate the control with the component in their local copy of the mappings file.
 
```bash
finch create mapping \
  --mappings-file mappings.yaml \
  --component-name "$COMPONENT_NAME" \
  --control "$CONTROL_ID"
```
 
**Step 5.** The user proposes the change to the security team.
 
**Step 6.** The security team reviews the proposal. They check that the SSP documents the component's support for the implemented control in the target environment.
 
**Step 7.** If the security team approves the proposal, they publish a new mappings file version.
 
### Proposing a New Rule-to-Component Mapping
 
This user flow assumes the security team has published the SSP and a mappings file for the target environment. It also assumes the component in question already exists in the mappings file.
 
**Step 1.** A user downloads the SSP.
 
**Step 2.** The user downloads the mappings file.
 
**Step 3.** The user reviews the SSP to identify an implemented control that mitigates the [risk](GLOSSARY.md#risk) associated with the scanner rule.
 
**Step 4.** The user runs `finch` to add a rule-to-component [mapping](GLOSSARY.md#mapping) to their local copy of the mappings file.
 
```bash
finch create mapping \
  --mappings-file mappings.yaml \
  --component-name "$COMPONENT_NAME" \
  --control "$CONTROL_ID" \
  --scanner "$SCANNER" \
  --rule "$RULE_ID" \
  --justification "$JUSTIFICATION" \
  --expiry "$EXPIRY_DATE"
```
 
**Step 5.** The user proposes the change to the security team.
 
**Step 6.** The security team reviews the proposal. They check that the SSP documents the control as implemented and that the control mitigates the risk associated with the scanner rule.
 
**Step 7.** If the security team approves the proposal, they publish a new mappings file version.
 
### Generating a New Suppression
 
This user flow assumes the security team has published a mappings file version for the environment in question. It also assumes the user has an app with a Continuous Integration pipeline that runs a scanner.
 
**Step 1.** A user selects the mappings file version that matches their app's environment.
 
**Step 2.** The user configures their app's pipeline to download that mappings file version before the scanner runs.
 
**Step 3.** The user adds a `finch` step to their pipeline for each scanner rule they want to suppress.
 
```bash
finch create suppression \
  --mappings-file mappings.yaml \
  --scanner "$SCANNER" \
  --rule "$RULE_ID"
```

![](docs/demo/generate.gif)
 
**Step 4.** `finch` checks the mappings file for a mapping that matches the scanner and rule. It skips mappings that have expired.
 
**Step 5.** If `finch` finds a valid mapping, it generates a [suppression](GLOSSARY.md#suppression) in the format the scanner requires.
 
**Step 6.** The pipeline passes the suppression to the scanner, and the scanner leaves the suppressed rule out of its [findings](GLOSSARY.md#finding) for the app scan.
 