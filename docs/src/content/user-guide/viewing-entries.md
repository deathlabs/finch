# Viewing Entries

Print a [component](../GLOSSARY.md#component) in JSON, including its implemented [controls](../GLOSSARY.md#control) and rule [mappings](../GLOSSARY.md#mapping).

```bash
finch get component --component-name "$COMPONENT_NAME"
finch get control --component-name "$COMPONENT_NAME" --control "$CONTROL_ID"
finch get rule --component-name "$COMPONENT_NAME" --control "$CONTROL_ID" --scanner "$SCANNER" --rule "$RULE_ID"
```

The [control](../GLOSSARY.md#control) lookup prints the stored ID. The rule lookup prints the [mapping](../GLOSSARY.md#mapping) for the specified [scanner](../GLOSSARY.md#scanner) and [control](../GLOSSARY.md#control).

Use `-o yaml` or `--output-format yaml` to print YAML. Use `--mappings-file PATH` to read a [mappings file](../GLOSSARY.md#mappings-file) other than `mappings.yaml`.
