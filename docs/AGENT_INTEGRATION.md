# AI Agent Integration Guide

This guide covers how AI coding agents can effectively use `dure` for distributed e-commerce platform management and operations.

---

## Table of Contents

- [Overview](#overview)
- [Quick Start for Agents](#quick-start-for-agents)
- [JSON Mode](#json-mode)
- [Workflow Patterns](#workflow-patterns)
- [Parsing JSON Output](#parsing-json-output)
- [Error Handling](#error-handling)
- [Robot Mode Flags](#robot-mode-flags)
- [Agent-Specific Configuration](#agent-specific-configuration)
- [Best Practices](#best-practices)

---

## Overview

`dure` is designed with AI coding agents in mind:

- **JSON output** for all commands (`--json` flag)
- **Machine-readable errors** with structured error codes
- **Non-interactive** - no prompts in normal operation
- **Deterministic** - same input produces same output
- **Fast** - millisecond response times for most operations
- **E-commerce focused** - Products, orders, payments, and hosting management

### Key Principles

1. **Always use `--json`** for programmatic access
2. **Check exit codes** for success/failure
3. **Parse structured errors** for recovery hints
4. **Use `RUST_LOG=error`** to suppress internal dependency logs
5. **Verify operations** by checking response status

---

## Quick Start for Agents

```bash
# Check hosting status
dure hosting status --json

# List sites
dure site list --json

# Manage products
dure product list "MyStore" --json
dure product create "MyStore" "Product Name" "Category" "image.jpg" "Description" --json

# Process orders
dure order list "MyStore" --json
dure order create "MyStore" "product-id-1,product-id-2" "1,2" --json

# Manage members
dure member list "MyStore" --json
dure member info "MyStore" "@username" --json

# DNS operations
dure dns a example.com --json
dure ns status --json
```

---

## JSON Mode

### Enabling JSON Output

```bash
# Flag on any command
dure product list "MyStore" --json
dure hosting status --json
dure member info "MyStore" "@alice" --json

# Suppress logging for clean JSON output
RUST_LOG=error dure product list "MyStore" --json
```

### Environment Defaults

Set environment variables for routine agent use:

```bash
# Suppress internal logs
export RUST_LOG=error

# Commands will output clean JSON
dure product list "MyStore" --json
dure order list "MyStore" --json
```

### JSON Output Characteristics

- **Always valid JSON** - parseable even on errors
- **Arrays for lists** - `product list`, `order list`, `member list`
- **Objects for single items** - `hosting status`, `member info`
- **Structured errors** - error object with code and hints

### Example Output

```bash
$ dure product list "MyStore" --json
```
```json
{
  "success": true,
  "products": [
    {
      "id": "prod-abc123",
      "name": "Premium Widget",
      "category": "Electronics",
      "price": 49.99,
      "stock": 150,
      "status": "available",
      "created_at": "2026-04-01T12:00:00Z"
    },
    {
      "id": "prod-def456",
      "name": "Deluxe Gadget",
      "category": "Electronics",
      "price": 99.99,
      "stock": 75,
      "status": "available",
      "created_at": "2026-04-02T14:30:00Z"
    }
  ],
  "total": 2
}
```

---

## Workflow Patterns

### E-commerce Management Workflow

```
┌─────────────────────────────────────────────────────────────┐
│  1. CHECK STATUS                                            │
│     dure hosting status --json                              │
│     → Verify site is running and accessible                 │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  2. MANAGE PRODUCTS                                         │
│     dure product list "MyStore" --json                      │
│     → Review inventory, check stock levels                  │
│     dure product create "MyStore" "..." --json             │
│     → Add new products as needed                            │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  3. PROCESS ORDERS                                          │
│     dure order list "MyStore" --json                        │
│     → Check for new orders                                  │
│     → Update order status, process payments                 │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  4. MANAGE MEMBERS                                          │
│     dure member list "MyStore" --json                       │
│     → Review member activity                                │
│     → Manage permissions and roles                          │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  5. MONITOR                                                 │
│     dure audit status --json                                │
│     → Review audit logs and system health                   │
└─────────────────────────────────────────────────────────────┘
```

### Product Management

```bash
# List products with filtering
dure product list "MyStore" --json | jq '.products[] | select(.stock < 10)'

# Create new product
dure product create "MyStore" \
  "Premium Widget" \
  "Electronics" \
  "widget.jpg" \
  "High-quality premium widget" \
  --json

# Update product
dure product modify "MyStore" prod-abc123 \
  "Premium Widget Pro" \
  "Electronics" \
  "widget-pro.jpg" \
  "Updated description" \
  --json
```

### Order Processing

```bash
# List pending orders
dure order list "MyStore" --json | jq '.orders[] | select(.status == "pending")'

# Create order (guest to store)
dure order create "MyStore" "prod-abc123,prod-def456" "2,1" --json

# Process payment
dure payment create "MyStore" order-xyz789 "credit_card" --json
dure payment verify "MyStore" order-xyz789 "$payment_data" --json
```

### Member Management

```bash
# List all members
dure member list "MyStore" --json

# Get member details
dure member info "MyStore" "@alice" --json

# Manage roles
dure role list "MyStore" --json
dure role assign "MyStore" "@alice" "Manager" --json
```

---

## Parsing JSON Output

### Python Example

```python
import subprocess
import json

def dure_command(*args):
    """Run dure command and return parsed JSON."""
    result = subprocess.run(
        ['dure', *args, '--json'],
        capture_output=True,
        text=True,
        env={'RUST_LOG': 'error'}
    )
    if result.returncode != 0:
        error = json.loads(result.stdout) if result.stdout else {"error": result.stderr}
        raise RuntimeError(f"dure error: {error.get('message', 'Unknown')}")
    return json.loads(result.stdout)

# List products
products = dure_command('product', 'list', 'MyStore')
for product in products.get('products', []):
    print(f"{product['id']}: {product['name']} - ${product['price']}")

# Check inventory
low_stock = [p for p in products.get('products', []) if p['stock'] < 10]
print(f"Low stock items: {len(low_stock)}")
```

### JavaScript/Node Example

```javascript
const { execSync } = require('child_process');

function dure(...args) {
  const env = { ...process.env, RUST_LOG: 'error' };
  const result = execSync(`dure ${args.join(' ')} --json`, {
    encoding: 'utf-8',
    stdio: ['pipe', 'pipe', 'pipe'],
    env: env
  });
  return JSON.parse(result);
}

// List products
const products = dure('product', 'list', 'MyStore');
console.log(`Found ${products.products.length} products`);

// Filter by category
const electronics = products.products.filter(p => p.category === 'Electronics');
console.log(`Electronics: ${electronics.length}`);
```

### jq Examples

```bash
# Get IDs of all products
dure product list "MyStore" --json | jq -r '.products[].id'

# Get low-stock products
dure product list "MyStore" --json | jq '.products[] | select(.stock < 10)'

# Count products by category
dure product list "MyStore" --json | \
  jq '.products | group_by(.category) | map({category: .[0].category, count: length})'

# Get pending orders
dure order list "MyStore" --json | jq '.orders[] | select(.status == "pending")'
```

---

## Error Handling

### Exit Codes

| Code | Category | Example |
|------|----------|---------|
| 0 | Success | Command completed |
| 1 | Internal | Unexpected error |
| 2 | Database | Not initialized |
| 3 | Entity | Product/order not found |
| 4 | Validation | Invalid product data |
| 5 | Communication | WebSocket/API error |
| 6 | Network | Connection failure |
| 7 | Config | Missing configuration |
| 8 | I/O | File system error |

### Structured Error Response

```json
{
  "success": false,
  "error": {
    "code": 3,
    "kind": "not_found",
    "message": "Product not found: prod-xyz999",
    "recovery_hints": [
      "Check the product ID spelling",
      "Use 'dure product list' to find valid IDs"
    ]
  }
}
```

### Error Recovery Patterns

```python
def safe_product_create(store, name, category, image, description):
    """Create product with retry on transient errors."""
    for attempt in range(3):
        try:
            return dure_command('product', 'create', store, name, category, image, description)
        except RuntimeError as e:
            if 'database locked' in str(e) and attempt < 2:
                time.sleep(0.5)
                continue
            raise
```

---

## Robot Mode Flags

These flags enable machine-friendly output:

| Flag | Description |
|------|-------------|
| `--json` | JSON output for all data |
| `-y, --yes` | Skip confirmations |
| `--no-color` | Disable ANSI colors |
| `-q, --quiet` | Suppress non-error output |

### Combining Flags

```bash
# Machine-friendly product creation
dure product create "MyStore" "Product" "Category" "img.jpg" "Desc" --json -y

# Quiet mode with JSON
dure order list "MyStore" --quiet --json
# Outputs JSON, no status messages
```

---

## Agent-Specific Configuration

### Claude Code / Anthropic Agents

```bash
# Set environment for clean output
export RUST_LOG=error

# Workflow
dure hosting status --json
dure product list "MyStore" --json
dure order list "MyStore" --json | jq '.orders[] | select(.status == "pending")'
```

### Cursor AI

```bash
# Initialize
export RUST_LOG=error

# Use with Cursor's tool system
dure product list "MyStore" --json
dure member info "MyStore" "@user" --json
```

### Aider

```bash
# Aider integration
export RUST_LOG=error

# Check status before session
dure hosting status --json | head -5
```

### GitHub Copilot Workspace

```bash
# Copilot-friendly workflow
dure product list "MyStore" --json
dure order list "MyStore" --json | jq '.orders[] | select(.status == "pending")'
```

---

## Best Practices

### DO

1. **Always use `--json`** for programmatic access
2. **Check exit codes** before parsing output
3. **Set `RUST_LOG=error`** to suppress logs
4. **Use `jq`** for JSON filtering and transformation
5. **Handle errors gracefully** with try-catch or error checking
6. **Validate input data** before submitting
7. **Check response status** in JSON output
8. **Use specific commands** rather than parsing human output

### DON'T

1. **Don't parse human output** - use `--json` instead
2. **Don't ignore errors** - check exit codes and error messages
3. **Don't assume success** - verify response status in JSON
4. **Don't skip validation** - validate product/order data before creation
5. **Don't expose sensitive data** - handle API keys securely
6. **Don't create duplicates** - check existence first

### Session Management

```bash
# Session start
dure hosting status --json > /tmp/session_start.json

# Session end checklist
# Review changes
# Verify operations completed
# Check audit logs
```

### Concurrent Agent Safety

```bash
# Handle database locks gracefully
# Retry on transient errors
# Use appropriate timeouts
```

---

## Integration Examples

### Product Inventory Management

```bash
# Check low stock
LOW_STOCK=$(dure product list "MyStore" --json | jq '[.products[] | select(.stock < 10)]')
echo "$LOW_STOCK" | jq '.[].name'

# Send notification if low stock found
if [ $(echo "$LOW_STOCK" | jq 'length') -gt 0 ]; then
  echo "Low stock alert: $(echo "$LOW_STOCK" | jq 'length') products"
fi
```

### Order Processing Automation

```bash
# Get pending orders
PENDING=$(dure order list "MyStore" --json | jq '[.orders[] | select(.status == "pending")]')

# Process each order
echo "$PENDING" | jq -r '.[].id' | while read ORDER_ID; do
  echo "Processing order: $ORDER_ID"
  # Add payment processing logic here
done
```

### Member Activity Monitoring

```bash
# List members
MEMBERS=$(dure member list "MyStore" --json)

# Find inactive members (example)
echo "$MEMBERS" | jq '.members[] | select(.last_seen | . < now - 86400*30)'
```

---

## Troubleshooting

### Common Issues

**"Command not found: dure"**
```bash
# Ensure binary is in PATH
which dure

# Or use full path
/path/to/dure product list "MyStore" --json
```

**JSON parsing errors**
```bash
# Ensure RUST_LOG is set to suppress non-JSON output
export RUST_LOG=error
dure product list "MyStore" --json | jq .
```

**"Database not initialized"**
```bash
# Check database exists
ls -la ~/.local/share/dure/

# Run initialization (if needed)
# (Command TBD based on final design)
```

**"Product not found"**
```bash
# List to find correct ID
dure product list "MyStore" --json | jq '.products[].id'

# Verify product exists before operations
```

### Debug Logging

```bash
# Enable debug output
RUST_LOG=debug dure product list "MyStore" --json 2>debug.log

# Verbose mode
dure product list "MyStore" --json -v
```

---

## See Also

- [CLI_REFERENCE.md](CLI_REFERENCE.md) - Complete command reference
- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Quick command lookup
- [README.md](../README.md) - Project overview
- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture
