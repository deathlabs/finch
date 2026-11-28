# Mappings File Schema

Every rule entry is an object. Include an `id`, [`control`](../GLOSSARY.md#control), [`justification`](../GLOSSARY.md#justification), and [`expiry`](../GLOSSARY.md#expiry) for each rule. Finch rejects unrecognized keys. See [the example](https://github.com/deathlabs/finch/blob/main/mappings.yaml).

The `add` command reports an error if the entry already exists. The `--mappings-file` option defaults to `mappings.yaml`.
