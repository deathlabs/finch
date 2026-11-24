# `finch`

[![CI Pipeline](https://github.com/deathlabs/finch/actions/workflows/check.yml/badge.svg)](https://github.com/deathlabs/finch/actions/workflows/check.yml)

Shape what your [scanners](GLOSSARY.md) can see. Finch gives [scanners](GLOSSARY.md) [context](GLOSSARY.md) about your [hosting platform's](GLOSSARY.md) security [controls](GLOSSARY.md). It helps [platform teams](GLOSSARY.md) document [mitigations](GLOSSARY.md) and reduce the time needed for approval.

`finch` generates [scanner](GLOSSARY.md) [suppressions](GLOSSARY.md) for a [tenant's](GLOSSARY.md) app from a [platform team's](GLOSSARY.md) [mappings file](GLOSSARY.md).

## How It Works

1. A platform operator runs `finch serve` or `finch tui` and loads the platform's [system security plan](GLOSSARY.md) (SSP) in OSCAL JSON format.
2. The operator defines platform [components](GLOSSARY.md) and maps them to the [controls](GLOSSARY.md) they implement and the [scanner](GLOSSARY.md) rules they mitigate. Finch saves these mappings in `mappings.yaml`.
3. A continuous integration (CI) pipeline runs `finch generate` before the [scanner](GLOSSARY.md). Finch uses the mappings to create [suppression](GLOSSARY.md) files for the [scanner](GLOSSARY.md) and a report explaining the [suppressions](GLOSSARY.md).

### `finch tui`

![Demo of Finch creating component and rule mappings in a terminal](docs/demo/tui.gif)

Run `finch tui` to create mappings in a terminal. Enter the path to an OSCAL SSP JSON file, or pass it with `--ssp`. Follow the prompts to add [components](GLOSSARY.md), [controls](GLOSSARY.md), and [scanner](GLOSSARY.md) rules. Press Ctrl+D to finish without adding a security tool. Finch writes to `mappings.yaml` by default. Press Escape to return to an earlier field or Ctrl+C to quit.

The demo uses a fictional [SSP](docs/demo/tui-ssp.json). It does not demonstrate a real [mitigation](GLOSSARY.md). To record the demo again, install Finch and VHS with its dependencies. Then run `vhs docs/demo/tui.tape` from the repository root.

### `finch generate`

![Demo of Grype showing a finding before and after Finch generates a suppression](docs/demo/generate.gif)

The demo scans a [sample Log4j software bill of materials (SBOM)](docs/demo/log4j-sbom.json) with Grype. It shows one [finding](GLOSSARY.md) before and after Finch generates the Grype configuration. The [mappings](docs/demo/mappings.yaml) are fictional and do not demonstrate a real [mitigation](GLOSSARY.md).

To record the demo again, install Finch, Grype, and VHS with its dependencies. Update the Grype [vulnerability](GLOSSARY.md) database, then run `vhs docs/demo/generate.tape` from the repository root.

```bash
finch generate --mappings mappings.yaml --tool zap --output-dir finch-out
```

| Flag | Default | Description |
| --- | --- | --- |
| `--mappings`   | (required)  | Path to the [mappings file](GLOSSARY.md) created by Finch |
| `--tool`       | `zap`       | [Scanner](GLOSSARY.md) to generate files for: `zap` or `grype` |
| `--output-dir` | `finch-out` | Directory for the generated files |

For ZAP, three files are written:

| File | Purpose |
| --- | --- |
| `zap-rules.conf`          | Pass this file to ZAP's scan scripts with `-c`. Suppressed rules are ignored and do not appear in the ZAP report. |
| `zap-alert-filters.yaml`  | An Automation Framework `alertFilter` job. Suppressed alerts stay in the report and are marked "False Positive". |
| `suppression-report.json` | Lists the platform [components](GLOSSARY.md) and [controls](GLOSSARY.md) that justify each suppressed rule, along with the source SSP. Keep this report in the body of evidence. |

For Grype, `finch generate` writes `grype-ignore.yaml` and `grype-suppression-report.json`. Pass the ignore file to Grype with `--config`. Each [vulnerability](GLOSSARY.md) rule applies to all packages.

Finch does not write output if the [mappings file](GLOSSARY.md) has an unknown key or an invalid rule ID. It also writes no output if a [component](GLOSSARY.md) maps rules but implements no [controls](GLOSSARY.md). ZAP rule IDs contain digits only.

### Mapping Expiry

The planned mapping format includes an expiry date for each mapping. A mapping remains valid through its expiry date in UTC. After that date, generation will silently omit its [suppression](GLOSSARY.md). It will not fail the pipeline or issue a warning.

[Platform teams](GLOSSARY.md) need to review expired mappings and remove or renew them. Reporting expired mappings during validation can make cleanup visible. A warning allows unrelated changes to proceed, but a team can miss it. A validation failure requires cleanup before changes can pass, but can block unrelated changes. The validation behavior is still undecided.

See the fictional mappings for [Platform A](examples/platform-a-mappings.yaml), [Platform B](examples/platform-b-mappings.yaml), and [Platform C](examples/platform-c-mappings.yaml). Platform A includes Semgrep mappings as an example, but `finch generate` supports ZAP and Grype. These examples do not establish that a real platform mitigates a [finding](GLOSSARY.md). A ZAP rule can report multiple alert types, and a mapping suppresses the entire rule. Verify each mapping against the platform and scan configuration before use.

[Platform U](examples/platform-u-mappings.yaml) shows an example UDS Core 1.17 service inventory. Its SSP metadata and [component](GLOSSARY.md) UUIDs are examples. Services vary by bundle and configuration. The file has no [control](GLOSSARY.md) or [scanner](GLOSSARY.md) rule mappings. Add them only after you verify the deployed platform and its SSP. See the [UDS Core concepts documentation](https://docs.defenseunicorns.com/core/v1-17/concepts/overview/) for the capability layers.

### Use the Output in GitLab CI

ZAP full and API scans run active rules, such as 40012 and 40018. The baseline scan runs only passive rules, so it does not raise [findings](GLOSSARY.md) from active rules.

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

Set `TARGET_URL` to the application as served through the platform. Count [findings](GLOSSARY.md) as mitigated by the gateway or mesh only if the scan passes through them.
