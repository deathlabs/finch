# Creating a Mappings File

This [user](../GLOSSARY.md#user) flow is focused on the [security team](../GLOSSARY.md#security-team) [user](../GLOSSARY.md#user) persona and assumes they have a [System Security Plan (SSP)](../GLOSSARY.md#system-security-plan) for their [hosting platform](../GLOSSARY.md#hosting-platform).

**Step 1.** The [security team](../GLOSSARY.md#security-team) identifies the [environment](../GLOSSARY.md#environment) that the [mappings file](../GLOSSARY.md#mappings-file) will cover.

**Step 2.** The [security team](../GLOSSARY.md#security-team) reviews the [SSP](../GLOSSARY.md#system-security-plan) to identify the [components](../GLOSSARY.md#component) and implemented [controls](../GLOSSARY.md#control) that apply to that [environment](../GLOSSARY.md#environment).

**Step 3.** The [security team](../GLOSSARY.md#security-team) creates a [mappings file](../GLOSSARY.md#mappings-file) in the terminal interface (TUI). For each selected rule, choose an implemented [control](../GLOSSARY.md#control) and enter a [justification](../GLOSSARY.md#justification) and an [expiry](../GLOSSARY.md#expiry) date in YYYY-MM-DD format.

**Open the Terminal Interface**

```bash
finch start tui --ssp ssp.json
```

![Finch terminal interface demo](assets/tui.gif)

**Step 4.** The [security team](../GLOSSARY.md#security-team) publishes their [mappings file](../GLOSSARY.md#mappings-file).
