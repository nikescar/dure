# Quickstart (Agents)

Goal: quickly integrate AI agents with Dure's e-commerce operations using structured JSON output.

## 1) Check hosting status

```bash
dure hosting status --json
```

## 2) List sites

Machine-readable:

```bash
dure site list --json
```

## 3) Manage products

```bash
# List products
dure product list "MyStore" --json

# Create product
dure product create "MyStore" "Product Name" "Category" "image.jpg" "Description" --json

# Modify product
dure product modify "MyStore" 123456 "New Name" "New Category" "new-image.jpg" "New description" --json
```

## 4) Process orders

```bash
# List orders
dure order list "MyStore" --json

# Create order
dure order create "MyStore" "product-id-1,product-id-2" "1,2" --json
```

## 5) Member management

```bash
# List members
dure member list "MyStore" --json

# Get member info
dure member info "MyStore" "@username" --json
```

## 6) DNS operations

```bash
# Query DNS records
dure dns a example.com --json
dure dns txt example.com --json

# Manage nameserver
dure ns status --json
dure ns status www.example.com --json
```

## Common patterns

- All commands support `--json` for machine-readable output
- Use `RUST_LOG=error` to suppress internal Rust dependency logs:
  ```bash
  RUST_LOG=error dure product list "MyStore" --json
  ```
- When scripting, route stderr separately; errors may be emitted as structured JSON on stderr.

## Environment configuration

Set default output format:

```bash
export DURE_OUTPUT_FORMAT=json
dure site list  # defaults to JSON
```

## Agent integration

Dure is designed for AI coding agents. Every command supports `--json` for structured output:

```bash
dure product list "MyStore" --json | jq '.products[] | select(.price < 100)'
dure order list "MyStore" --json | jq '.orders[] | select(.status == "pending")'
```
