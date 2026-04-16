# Robot Mode (JSON)

Dure supports machine-readable JSON output for AI agent and tooling integration.

## Choosing an output format

- JSON: `--json` flag
- Text: default output (human-readable)

All commands that support `--json` will output structured JSON data suitable for parsing by AI agents.

## Environment defaults

Control logging and output behavior via env vars:

- `RUST_LOG` - Logging level (debug, info, warn, error)
- `DURE_OUTPUT_FORMAT` - Default output format (optional)

Recommended for routine agent use:

```bash
export RUST_LOG=error
dure product list "MyStore" --json
```

This suppresses internal Rust dependency logs while preserving normal stdout/JSON output.

Example:

```bash
# Suppress logs, get clean JSON
RUST_LOG=error dure site list --json | jq .

# Verbose mode for debugging
RUST_LOG=debug dure hosting status --json
```

## stderr vs stdout

- Normal successful outputs go to stdout.
- Diagnostics/logging go to stderr.
- Failures emit a structured JSON error object on stderr (see `docs/agent/ERRORS.md`).

Practical pattern:

```bash
dure product list "MyStore" --json 2>error.json | jq .
```

## JSON output examples

### Product operations

```bash
dure product list "MyStore" --json
dure product create "MyStore" "Name" "Category" "img.jpg" "Description" --json
```

### Order operations

```bash
dure order list "MyStore" --json
dure order create "MyStore" "product-id" "1" --json
```

### DNS operations

```bash
dure dns a example.com --json
dure dns txt example.com --json
dure ns status --json
```

### Member operations

```bash
dure member list "MyStore" --json
dure member info "MyStore" "@username" --json
```

### Hosting operations

```bash
dure hosting status --json
dure hosting list --json
```

## Agent-first design

Every command is designed for AI coding agents:

```bash
# Filter products by price
dure product list "MyStore" --json | jq '.products[] | select(.price < 100)'

# Get pending orders
dure order list "MyStore" --json | jq '.orders[] | select(.status == "pending")'

# List active members
dure member list "MyStore" --json | jq '.members[] | select(.active == true)'
```
