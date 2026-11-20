# `finch`  
[![CI Pipeline](https://github.com/deathlabs/finch/actions/workflows/check.yml/badge.svg)](https://github.com/deathlabs/finch/actions/workflows/check.yml)  
Deploy faster without having to write security exceptions for vulnerabilities mitigated by your hosting platform. 

## How it works

1. A platform operator runs `finch serve`, loads the platform's System Security Plan (OSCAL JSON), defines the platform's components, and maps each component to the scanner rules it mitigates. The result is exported as `mappings.yaml`.
2. A CI pipeline runs `finch generate` against that file to produce scanner-native suppression files before the scanner runs.

### `finch generate`

```bash
finch generate --mappings mappings.yaml --tool zap --output-dir finch-out
```

| Flag | Default | Description |
| --- | --- | --- |
| `--mappings` | (required) | Mappings file exported by `finch serve` |
| `--tool` | `zap` | Scanner to generate files for (supported: `zap`) |
| `--output-dir` | `finch-out` | Directory to write generated files to |

For ZAP, three files are written:

| File | Purpose |
| --- | --- |
| `zap-rules.conf` | Pass to ZAP's scan scripts with `-c`. Suppressed rules are ignored and do not appear in the ZAP report. |
| `zap-alert-filters.yaml` | An Automation Framework `alertFilter` job. Suppressed alerts stay in the report, marked "False Positive". |
| `suppression-report.json` | For each suppressed rule: the platform components and controls that justify it, plus the SSP the mappings came from. Keep it in the body of evidence. |

`finch generate` fails closed. If the mappings file has an unknown key, a rule ID that is not a ZAP rule ID (digits only), or a component that maps rules but implements no controls, nothing is written.

See the fictional mappings for [Platform A](examples/platform-a-mappings.yaml), [Platform B](examples/platform-b-mappings.yaml), and [Platform C](examples/platform-c-mappings.yaml). Platform A includes Semgrep mappings for illustration; `finch generate` currently supports ZAP only. These examples do not establish that a real platform mitigates a finding. A ZAP rule can report multiple alert types, and a mapping suppresses the entire rule. Verify each mapping against the platform and scan configuration before use.

[Platform U](examples/platform-u-mappings.yaml) illustrates a UDS Core 1.17 service inventory. Its SSP metadata and component UUIDs are examples. Services vary by bundle and configuration. It contains no control or scanner rule mappings; add those only after verifying the deployed platform and its SSP. See the [UDS Core concepts documentation](https://docs.defenseunicorns.com/core/v1-17/concepts/overview/) for the capability layers.

### Using the output in GitLab CI

Full and API scans run ZAP's active rules (for example 40012 and 40018). The baseline scan runs passive rules only, so it never raises them.

```yaml
finch-suppressions:
  stage: prep
  script:
    - curl --fail --silent --show-error "$PLATFORM_MAPPINGS_URL" --output mappings.yaml
    - finch generate --mappings mappings.yaml --tool zap --output-dir finch-out
  artifacts:
    paths: [finch-out/]

zap-full-scan:
  stage: test
  image: ghcr.io/zaproxy/zaproxy:stable
  needs: [finch-suppressions]
  script:
    - mkdir -p /zap/wrk
    - cp finch-out/zap-rules.conf /zap/wrk/
    - zap-full-scan.py -t "$TARGET_URL" -c zap-rules.conf -J zap-report.json
  artifacts:
    when: always
    paths: [zap-report.json, finch-out/suppression-report.json]
```

Point `TARGET_URL` at the application as it is served through the platform. Findings mitigated by the gateway or mesh only count as mitigated if the scan goes through them.
