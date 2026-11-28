# Proposing a Component

This [user](../GLOSSARY.md#user) flow assumes the [security team](../GLOSSARY.md#security-team) has published the [hosting platform's](../GLOSSARY.md#hosting-platform) [System Security Plan](../GLOSSARY.md#system-security-plan) ([SSP](../GLOSSARY.md#system-security-plan)) so [users](../GLOSSARY.md#user) can reference it. It also assumes a [mappings file](../GLOSSARY.md#mappings-file) for the [hosting platform](../GLOSSARY.md#hosting-platform) already exists.

**Step 1.** A [user](../GLOSSARY.md#user) downloads the [SSP](../GLOSSARY.md#system-security-plan) for their [hosting platform](../GLOSSARY.md#hosting-platform).

**Step 2.** The [user](../GLOSSARY.md#user) downloads the [mappings file](../GLOSSARY.md#mappings-file) for their [hosting platform](../GLOSSARY.md#hosting-platform).

**Step 3.** The [user](../GLOSSARY.md#user) reviews the [SSP](../GLOSSARY.md#system-security-plan) to identify which implemented [controls](../GLOSSARY.md#control) the [component](../GLOSSARY.md#component) they want to propose addresses.

**Step 4.** The [user](../GLOSSARY.md#user) adds the [component](../GLOSSARY.md#component) to their local copy of the [mappings file](../GLOSSARY.md#mappings-file).

```bash
finch add --mappings-file mappings.yaml component \
  --component-name "$COMPONENT_NAME" \
  --type service \
  --description "$DESCRIPTION" \
  --purpose "$PURPOSE"
```

**Step 5.** The [user](../GLOSSARY.md#user) proposes the change to the [security team](../GLOSSARY.md#security-team).

**Step 6.** The [security team](../GLOSSARY.md#security-team) reviews the proposal. They check that the [component](../GLOSSARY.md#component) matches the entry in the [SSP](../GLOSSARY.md#system-security-plan) and applies to the [environment](../GLOSSARY.md#environment) in question.

**Step 7.** If the [security team](../GLOSSARY.md#security-team) approves the proposal, they publish a new [mappings file version](../GLOSSARY.md#mappings-file-version).
