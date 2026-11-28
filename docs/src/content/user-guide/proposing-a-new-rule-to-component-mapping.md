# Proposing a New Rule-to-Component Mapping

This [user](../GLOSSARY.md#user) flow assumes the [security team](../GLOSSARY.md#security-team) has published the [SSP](../GLOSSARY.md#system-security-plan) and a [mappings file](../GLOSSARY.md#mappings-file) for the target [environment](../GLOSSARY.md#environment). It also assumes the [component](../GLOSSARY.md#component) in question already exists in the [mappings file](../GLOSSARY.md#mappings-file).

**Step 1.** A [user](../GLOSSARY.md#user) downloads the [SSP](../GLOSSARY.md#system-security-plan).

**Step 2.** The [user](../GLOSSARY.md#user) downloads the [mappings file](../GLOSSARY.md#mappings-file).

**Step 3.** The [user](../GLOSSARY.md#user) reviews the [SSP](../GLOSSARY.md#system-security-plan) to identify an implemented [control](../GLOSSARY.md#control) that mitigates the [risk](../GLOSSARY.md#risk) associated with the [scanner](../GLOSSARY.md#scanner) rule.

**Step 4.** The [user](../GLOSSARY.md#user) runs `finch` to add the rule to their local copy of the [mappings file](../GLOSSARY.md#mappings-file). The [component](../GLOSSARY.md#component) must already list the [control](../GLOSSARY.md#control).

```bash
finch add --mappings-file mappings.yaml rule \
  --component-name "$COMPONENT_NAME" \
  --control "$CONTROL_ID" \
  --scanner "$SCANNER" \
  --rule "$RULE_ID" \
  --justification "$JUSTIFICATION" \
  --expiry "$EXPIRY_DATE"
```

**Step 5.** The [user](../GLOSSARY.md#user) proposes the change to the [security team](../GLOSSARY.md#security-team).

**Step 6.** The [security team](../GLOSSARY.md#security-team) reviews the proposal. They check that the [SSP](../GLOSSARY.md#system-security-plan) documents the [control](../GLOSSARY.md#control) as implemented and that the [control](../GLOSSARY.md#control) mitigates the [risk](../GLOSSARY.md#risk) associated with the [scanner](../GLOSSARY.md#scanner) rule.

**Step 7.** If the [security team](../GLOSSARY.md#security-team) approves the proposal, they publish a new [mappings file version](../GLOSSARY.md#mappings-file-version).
