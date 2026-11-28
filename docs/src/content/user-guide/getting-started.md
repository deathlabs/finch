# Getting Started

## Install Finch

Install Rust and Cargo. From the root of the Finch repository, run:

```bash
make build
finch --help
```

## Start the MCP Server

Run `finch start mcp` to start the Model Context Protocol (MCP) server.
Set your client's Streamable HTTP URL to `http://127.0.0.1:8765/mcp`.
To use a different port, run `finch start mcp --port 9000` and update the client URL.

Use `finch://schema/mappings` to read the JSON Schema for a [mappings file](../GLOSSARY.md#mappings-file).
