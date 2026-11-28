#!/bin/sh

# Stop the script when a command fails or a variable is unset.
set -o errexit
set -o nounset

# Enter the docs directory from the repository root.
cd docs/src

# Build the docs.
uv run --locked zensical build

# Serve the docs.
exec uv run --locked zensical serve \
	--dev-addr "${DEV_ADDR:-localhost:8001}"
