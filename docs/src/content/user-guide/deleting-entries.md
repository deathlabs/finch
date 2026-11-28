# Deleting Entries

A [component](../GLOSSARY.md#component) with implemented [controls](../GLOSSARY.md#control) cannot be deleted unless you pass `-f`. A [control](../GLOSSARY.md#control) with dependent [mappings](../GLOSSARY.md#mapping) also requires `-f`. Finch lists the dependents when it rejects a deletion.

With `-f`, deleting a [component](../GLOSSARY.md#component) also removes its [controls](../GLOSSARY.md#control) and [mappings](../GLOSSARY.md#mapping). Deleting a [control](../GLOSSARY.md#control) with `-f` removes the [mappings](../GLOSSARY.md#mapping) that cite it on that [component](../GLOSSARY.md#component).

```bash
finch delete component --component-name "$COMPONENT_NAME" -f
finch delete control --component-name "$COMPONENT_NAME" --control "$CONTROL_ID" -f
finch delete rule --component-name "$COMPONENT_NAME" --control "$CONTROL_ID" --scanner "$SCANNER" --rule "$RULE_ID"
```
