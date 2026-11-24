# ---------------------------------------------------------
# Set default values.
# ---------------------------------------------------------

.DEFAULT_GOAL := build
SBOM_FILE ?= sbom.json
SCAN_FILE ?= scan.json
GRYPE_FAILURE_THRESHOLD ?= high
VEX_FILE ?=

# ---------------------------------------------------------
# Build the artifact.
# ---------------------------------------------------------

.PHONY: build
.SILENT: build

build: 
	go install -ldflags="-s -w" . 

# ---------------------------------------------------------
# Update dependencies.
# ---------------------------------------------------------

.PHONY: update
.SILENT: update
update:
	go get -u ./...
	go mod tidy

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
	if [ -n "$(VEX_FILE)" ]; then \
		grype "sbom:$(SBOM_FILE)" --vex "$(VEX_FILE)" --fail-on "$(GRYPE_FAILURE_THRESHOLD)" -o table -o "cyclonedx-json=$(SCAN_FILE)"; \
	else \
		grype "sbom:$(SBOM_FILE)" --fail-on "$(GRYPE_FAILURE_THRESHOLD)" -o table -o "cyclonedx-json=$(SCAN_FILE)"; \
	fi
