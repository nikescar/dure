# Schemas

Dure provides JSON schemas describing the primary machine-readable outputs for AI agent integration.

## Emit schemas

```bash
dure schema all --format json
dure schema product --format json
dure schema order --format json
dure schema member --format json
dure schema error --format json
```

## Primary schemas

### Product schema

```bash
dure schema product --format json
```

Describes product structure:
- Product ID
- Name, category, description
- Images and metadata
- Pricing information

### Order schema

```bash
dure schema order --format json
```

Describes order structure:
- Order ID
- Product IDs and quantities
- Order status
- Payment information

### Member schema

```bash
dure schema member --format json
```

Describes member structure:
- Member ID
- Roles and permissions
- Join date and status
- Device information

### Site schema

```bash
dure schema site --format json
```

Describes site structure:
- Site domain
- Hosting configuration
- Database settings
- SSL certificates

### Error schema

```bash
dure schema error --format json
```

Describes error response format (see `docs/agent/ERRORS.md`).

## Building from source

If you need to build Dure from source:

```bash
# Desktop build
cargo build --release

# Check schema support
./target/release/dure schema all --format json
```

## Using schemas in AI agents

Schemas enable AI agents to:
1. Validate JSON responses
2. Generate type-safe code
3. Understand data structures
4. Handle errors gracefully

Example integration:

```bash
# Get product schema
dure schema product --format json > product-schema.json

# Validate product data
dure product list "MyStore" --json | jq -e '.products[] | .id'
```
