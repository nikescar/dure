# Examples (Agents)

This file shows small, copy/pasteable flows for AI agents integrating with Dure.

## List products (JSON)

```bash
dure product list "MyStore" --json | jq '.[0]'
```

## List sites

```bash
dure site list --json | jq .
```

## Check hosting status

```bash
dure hosting status --json | jq .
```

## DNS operations (JSON)

```bash
dure dns a example.com --json | jq .
dure dns txt example.com --json | jq .
```

## Order operations

```bash
# List orders
dure order list "MyStore" --json | jq .

# Create order
dure order create "MyStore" "product-id-1,product-id-2" "1,2" --json | jq .
```

## Member management

```bash
# List members
dure member list "MyStore" --json | jq .

# Get member info
dure member info "MyStore" "@username" --json | jq .
```

## Audit operations

```bash
# Review action history
dure audit status --json | jq .
```

## Determinism smoke check

If the workspace is not changing, these should match:

```bash
dure product list "MyStore" --json | jq -S . > a.json
dure product list "MyStore" --json | jq -S . > b.json
diff -u a.json b.json
```
