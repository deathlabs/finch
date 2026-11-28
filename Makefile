# ---------------------------------------------------------
# Set default values.
# ---------------------------------------------------------

.DEFAULT_GOAL := build
SBOM_FILE ?= sbom.json
SCAN_FILE ?= scan.json
GRYPE_FAILURE_THRESHOLD ?= high

# ---------------------------------------------------------
# Build and install Finch.
# ---------------------------------------------------------

.PHONY: build
.SILENT: build
build:
	cargo install --path .

# ---------------------------------------------------------
# Update dependencies.
# ---------------------------------------------------------

.PHONY: update
.SILENT: update
update:
	cargo update

# ---------------------------------------------------------
# Generate an SBOM for the artifact.
# ---------------------------------------------------------

.PHONY: sbom
.SILENT: sbom
sbom:
	syft dir:. -o "cyclonedx-json=$(SBOM_FILE)"

# ---------------------------------------------------------
# Scan the artifact's SBOM.
# ---------------------------------------------------------

.PHONY: scan
.SILENT: scan
scan: sbom
	grype "sbom:$(SBOM_FILE)" \
		--fail-on "$(GRYPE_FAILURE_THRESHOLD)" \
		-o table \
		-o "cyclonedx-json=$(SCAN_FILE)"

# ---------------------------------------------------------
# Serve the docs locally.
# ---------------------------------------------------------

.PHONY: docs
.SILENT: docs
docs:
	./scripts/docs.sh
