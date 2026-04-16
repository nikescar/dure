# Architecture Overview

This document describes the internal architecture of `dure`, a distributed e-commerce platform built with Rust and egui.

---

## Table of Contents

- [Design Philosophy](#design-philosophy)
- [High-Level Architecture](#high-level-architecture)
- [Module Structure](#module-structure)
- [Data Flow](#data-flow)
- [Storage Layer](#storage-layer)
- [Sync System](#sync-system)
- [Configuration System](#configuration-system)
- [Error Handling](#error-handling)
- [CLI Layer](#cli-layer)
- [Key Patterns](#key-patterns)
- [Safety Invariants](#safety-invariants)
- [Extension Points](#extension-points)

---

## Design Philosophy

### Core Principles

1. **Distributed Architecture**: No centralized server dependency; peer-to-peer communication
2. **Multi-Platform**: Desktop, Android, and WASM with shared codebase
3. **Agent-Friendly**: Machine-readable output (JSON) for AI coding agents
4. **User Control**: Manual hosting setup; no automatic cloud operations
5. **Flexible Hosting**: Supports Firebase, Supabase, GCE, Cafe24 VPS backends

### Comparison with Traditional E-commerce Platforms

| Feature | Dure | Shopify | Wix | Magento |
|---------|------|---------|-----|---------|
| Hosting | Distributed | Managed | Managed | Self/Cloud |
| Transaction Fees | 0% | 2% | 0% | 0% |
| Setup Time | Hours | 1-2 days | Hours | Weeks |
| Payment Options | Portone, KakaoPay | Many | Limited | Many |
| Inventory Mgmt | Excellent | Good | Basic | Excellent |

---

## High-Level Architecture

```
┌─────────────────────────────────┐  ┌─────────────────────────────────┐  ┌─────────────────────────────────┐
│        GUI Layer (egui)         │  │      CLI Layer (clap)           │  │     WASM Layer (wasm-bindgen)   │
│  ┌──────┐ ┌──────┐ ┌──────┐     │  │  ┌──────┐ ┌──────┐ ┌──────┐    │  │  ┌──────┐ ┌──────┐              │
│  │Store │ │Order │ │Chat  │     │  │  │hosting│product│message│    │  │  │Store │ │Cart  │              │
│  └──┬───┘ └──┬───┘ └──┬───┘     │  │  └──┬───┘ └──┬───┘ └──┬───┘    │  │  │Front │ │      │              │
└─────┼────────┼────────┼─────────┘  └─────┼────────┼────────┼────────┘  └──┴──────┴──┴──────┴──────────────┘
      │        │        │                   │        │        │                  │        │
      v        v        v                   v        v        v                  v        v
┌──────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                      Business Logic                                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐               │
│  │  Validation  │  │  Formatting  │  │  ID Gen      │  │  i18n        │  │  Analytics   │               │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘               │
└──────────┬─────────────────────────┬──────────────────────────┬─────────────────────┬───────────────────┘
           │                         │                          │                     │
           v                         v                          v                     v
┌─────────────────────────┐  ┌──────────────────┐  ┌─────────────────────┐  ┌──────────────────┐
│   Storage Layer         │  │  API Clients     │  │  WebSocket Client   │  │  SSH Client      │
│  ┌──────────────┐       │  │                  │  │                     │  │                  │
│  │  SQLite      │       │  │  DNS Service API │  │  WSS Connection     │  │  VM Management   │
│  │  Event Log   │       │  │  (Cloudflare/    │  │  (Firebase/         │  │  (GCE/Cafe24)    │
│  │  Cache       │       │  │   Porkbun/       │  │   Supabase/         │  │                  │
│  │  Config      │       │  │   DuckDNS)       │  │   GCE/Cafe24)       │  │                  │
│  └──────┬───────┘       │  │                  │  │                     │  │                  │
└─────────┼───────────────┘  │  Hosting Svc API │  │  Peer-to-peer       │  │  Dure Server     │
          │                  │  (Firebase/      │  │  Communication      │  │  Setup           │
          v                  │   Supabase)      │  │                     │  │                  │
┌─────────────────────────┐  └──────────────────┘  └─────────────────────┘  └──────────────────┘
│  Local Storage          │
│  ┌──────────────────┐   │
│  │  dure.db         │   │  (No remote data storage - all data local)
│  │  (SQLite)        │   │
│  └──────────────────┘   │
└─────────────────────────┘
```

---

## Module Structure

```
mobile/src/
├── main.rs           # Entry point (GUI/CLI/Server mode dispatch)
├── lib.rs            # Crate root, module exports
│
├── cli/              # Command-line interface layer
│   ├── mod.rs        # Clap definitions (Cli, Commands, Args)
│   └── commands/     # CLI command handlers (delegate to calc/)
│       └── ...       # Thin wrappers calling business logic
│
├── calc/             # Business logic (shared across GUI/CLI/WASM)
│   ├── mod.rs        # Module exports
│   ├── hosting.rs    # Hosting setup and management
│   ├── role.rs       # Role operations
│   ├── member.rs     # Member management
│   ├── device.rs     # Device operations
│   ├── channel.rs    # Channel management
│   ├── message.rs    # Messaging operations
│   ├── product.rs    # Product CRUD
│   ├── order.rs      # Order management
│   ├── payment.rs    # Payment processing
│   └── review.rs     # Review operations
│
├── model/            # Data types
│   └── mod.rs        # Hosting, Server, Role, Member, Device, Channel,
│                     # Message, Product, Order, Payment, Review
│
├── storage/          # Persistence layer
│   ├── mod.rs        # Module exports
│   ├── sqlite.rs     # SqliteStorage implementation (desktop/Android)
│   ├── schema.rs     # Database schema definitions
│   └── events.rs     # Audit event storage
│
├── config/           # Configuration system
│   ├── mod.rs        # Layered config resolution
│   └── routing.rs    # DNS, Web, DB provider config resolution
│
├── error/            # Error handling
│   ├── mod.rs        # DureError enum
│   ├── structured.rs # JSON error output
│   └── context.rs    # Error context helpers
│
├── format/           # Output formatting
│   ├── mod.rs        # Module exports
│   ├── rich.rs       # Rich terminal output (panels, tables)
│   ├── text.rs       # Plain text formatting
│   ├── csv.rs        # CSV export
│   └── output.rs     # Output helpers
│
├── output/           # Output mode detection
│   ├── mod.rs        # Rich/Plain/JSON/Quiet mode detection
│   ├── context.rs    # Output context
│   ├── theme.rs      # Output theming
│   └── components/   # Reusable output widgets
│
├── util/             # Utilities
│   ├── mod.rs        # Module exports
│   ├── id.rs         # Hash-based ID generation
│   ├── hash.rs       # Content hashing
│   ├── time.rs       # Timestamp utilities
│   └── progress.rs   # Progress indicators
│
├── validation/       # Input validation
│   └── mod.rs        # Domain names, IDs, configs, API keys
│
└── logging.rs        # Logging setup (env_logger/android_logger)
```

---

## Data Flow

### Product Creation (Store Owner)

```
User Input            GUI/CLI              calc/              Storage         WebSocket
    │                   │                    │                   │                │
    │  Create Product   │                    │                   │                │
    │ ─────────────────>│                    │                   │                │
    │                   │                    │                   │                │
    │                   │  product::create() │                   │                │
    │                   │ ──────────────────>│                   │                │
    │                   │                    │                   │                │
    │                   │                    │  Validate input   │                │
    │                   │                    │                   │                │
    │                   │                    │  Generate ID      │                │
    │                   │                    │ ─────────────────>│                │
    │                   │                    │                   │                │
    │                   │                    │                   │  INSERT        │
    │                   │                    │                   │                │
    │                   │                    │                   │  Record event  │
    │                   │                    │                   │                │
    │                   │                    │  Broadcast update │                │
    │                   │                    │ ──────────────────────────────────>│
    │                   │                    │                   │                │
    │  Success + ID     │                    │                   │                │
    │ <──────────────────────────────────────│                   │                │
```

### Order Processing (Guest → Store)

```
Guest Client      Guest WSS         Store WSS       Store Owner      Payment Gateway
    │                │                 │                 │                 │
    │  Place Order   │                 │                 │                 │
    │ ──────────────>│                 │                 │                 │
    │                │  Forward order  │                 │                 │
    │                │ ───────────────>│                 │                 │
    │                │                 │  Notify owner   │                 │
    │                │                 │ ───────────────>│                 │
    │                │                 │                 │                 │
    │                │                 │                 │  Approve        │
    │                │                 │                 │                 │
    │                │                 │  Init payment   │                 │
    │                │ <───────────────────────────────────────────────────│
    │                │                 │                 │                 │
    │  Payment req   │                 │                 │                 │
    │ <──────────────│                 │                 │                 │
    │                │                 │                 │                 │
    │  Process       │                 │                 │                 │
    │ ──────────────────────────────────────────────────────────────────>│
    │                │                 │                 │                 │
    │                │                 │                 │  Payment OK     │
    │ <──────────────────────────────────────────────────────────────────│
    │                │                 │                 │                 │
    │  Order confirm │                 │  Order confirm  │                 │
    │ <──────────────│ <───────────────│                 │                 │
```

---

## Storage Layer

### SqliteStorage

The primary storage implementation using the fsqlite stack (`fsqlite`,
`fsqlite-types`, and `fsqlite-error`) for desktop and Android platforms.

```rust
pub struct SqliteStorage {
    conn: Connection,
}
```

**Key Features:**

- **WAL Mode**: Concurrent reads during writes
- **Busy Timeout**: Configurable lock timeout (default 30s)
- **Transactional Mutations**: Safe transaction protocol
- **Platform Support**: Desktop and Android (WASM uses different approach)

### Transaction Protocol

All mutations follow this pattern:

```rust
storage.mutate("operation", actor, |tx, ctx| {
    // 1. Perform the operation
    tx.execute(...)?;

    // 2. Record events for audit trail
    ctx.record_event(EventType::Created, &entity.id, None);

    // 3. Invalidate cache if needed
    ctx.invalidate_cache();

    Ok(result)
})
```

### Database Schema

```sql
-- Core tables
hostings            -- Hosting configurations
servers             -- Server instances
roles               -- User roles and permissions
members             -- Server members (owner/store)
devices             -- User devices
channels            -- Communication channels
messages            -- Chat messages
products            -- E-commerce products
orders              -- Customer orders
payments            -- Payment records
reviews             -- Product/store reviews
events              -- Audit log

-- Operational tables
config              -- Key-value configuration
cache               -- Performance caching
```

### Member Types

Two types of members are distinguished:
- **Owner members**: IDs without dots (e.g., `user123`) - for owner's devices
- **Store members**: Domain-based IDs with dots (e.g., `store.example.com`) - for other stores

When a store member receives a message, it syncs with the store WSS.

---

## Communication System

### WebSocket Protocol

Dure uses WebSocket connections for peer-to-peer communication between clients and servers.

**Connection Types:**
- Guest ↔ Guest WSS (own WebSocket server)
- Guest WSS ↔ Store WSS (peer-to-peer messaging)
- Store Owner ↔ Store WSS (own server management)

**Message Flow:**
```json
{
  "type": "message|order|payment|review",
  "from": "member_id",
  "to": "member_id|channel_id",
  "content": {...},
  "timestamp": "2026-04-02T12:00:00Z"
}
```

### API Integrations

#### DNS Service API

Manages domain registration and DNS records:
- **Providers**: DuckDNS, Cloudflare, Porkbun
- **Operations**: Register domain, update NS records, manage A/AAAA/TXT records

#### Hosting Service API

Manages web hosting and database services:
- **Providers**: Firebase, Supabase, GCE, Cafe24 VPS
- **Operations**: Create VM, deploy static site, configure database

#### SSH Connection

Remote server management for self-hosted deployments:
- **Target**: GCE or Cafe24 VPS instances
- **Operations**: Install Dure WebSocket server, configure services

### Path Validation

Sync operations enforce a strict path allowlist:

```rust
pub const ALLOWED_EXTENSIONS: &[&str] = &[".jsonl", ".json", ".db", ".yaml"];
pub const ALLOWED_EXACT_NAMES: &[&str] = &["metadata.json", "config.yaml"];

pub fn is_sync_path_allowed(path: &Path, beads_dir: &Path) -> bool {
    // Must be inside .beads/
    // Must have allowed extension
    // Must not be in .git/
}
```

---

## Configuration System

### Layer Hierarchy

Configuration sources in precedence order (highest wins):

```
1. CLI overrides        (--json, --token, --profile)
2. Environment vars     (DURE_DB, DURE_DATABASE, RUST_LOG)
3. Project config       (.dure/config.yaml)
4. User config          (~/.config/dure/config.yaml)
5. DB config table      (config table in SQLite)
6. Defaults
```

### Configuration Structure

```yaml
# .dure/config.yaml

device:
  name: "mylaptop"

# DNS configuration
dns:
  domain_registar: ""          # ""/Cloudflare/Porkbun
  domain_registar_token: ""
  name: "www.example.com"      # Domain name
  dns_provider: "CLOUDFLARE_DNS"  # DUCKDNS/CLOUDFLARE_DNS/PORKBUN/GCP_CLOUDDNS
  dns_provider_token: ""

# Web hosting configuration
web:
  web_provider: "GCE"          # GCE/VPS/CLOUDFLARE_PAGES/FIREBASE_HOSTING
  web_provider_token: ""
  db_provider: "GCE"           # GCE/GCP_CLOUDSQL/SUPABASE
```

### Key Configuration Options

| Key | Default | Description |
|-----|---------|-------------|
| `device.name` | hostname | Device identifier |
| `dns.domain_registar` | "" | Domain registrar service |
| `dns.dns_provider` | "" | DNS service provider |
| `web.web_provider` | "" | Web hosting provider |
| `web.db_provider` | "" | Database provider |
| `display.color` | auto | ANSI color output |
| `lock-timeout` | `30000` | SQLite busy timeout (ms) |

---

## Error Handling

### Error Types

```rust
pub enum DureError {
    // Storage errors
    DatabaseNotFound { path: PathBuf },
    DatabaseLocked { path: PathBuf },
    SchemaMismatch { expected: i32, found: i32 },

    // Entity errors
    HostingNotFound { id: String },
    ProductNotFound { id: String },
    OrderNotFound { id: String },
    MemberNotFound { id: String },
    ChannelNotFound { id: String },
    IdCollision { id: String },

    // Validation errors
    Validation { field: String, reason: String },
    InvalidDomain { domain: String },
    InvalidApiKey { provider: String },
    InvalidMemberId { id: String, reason: String },

    // Communication errors
    WebSocketConnection { server: String, reason: String },
    ApiCallFailed { provider: String, operation: String },
    SshConnectionFailed { host: String },

    // I/O errors
    Io(std::io::Error),
    Json(serde_json::Error),
}
```

### Exit Codes

| Code | Category | Description |
|------|----------|-------------|
| 0 | Success | Command completed |
| 1 | Internal | Unexpected error |
| 2 | Database | Not initialized, locked |
| 3 | Entity | Not found, ambiguous ID |
| 4 | Validation | Invalid input |
| 5 | Communication | WebSocket/API error |
| 6 | Network | Connection failure |
| 7 | Config | Missing configuration |
| 8 | I/O | File system error |

### Structured Error Output

```json
{
  "error_code": 3,
  "kind": "not_found",
  "message": "Product not found: prod-xyz999",
  "recovery_hints": [
    "Check the product ID spelling",
    "Use 'dure product list' to find valid IDs"
  ]
}
```

---

## CLI Layer

### Command Structure

Uses Clap's derive macros for CLI mode:

```rust
#[derive(Parser)]
#[command(name = "dure")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true)]
    pub json: bool,

    #[arg(long, global = true)]
    pub serv: bool,  // Server mode

    #[arg(long, global = true)]
    pub tray: bool,  // GUI mode

    // ... other global options
}

#[derive(Subcommand)]
pub enum Commands {
    Hosting(HostingArgs),
    Role(RoleArgs),
    Member(MemberArgs),
    Device(DeviceArgs),
    Channel(ChannelArgs),
    Message(MessageArgs),
    Product(ProductArgs),
    Order(OrderArgs),
    Payment(PaymentArgs),
    Review(ReviewArgs),
    // ... additional commands
}
```

### Command Flow

```rust
fn main() {
    let cli = Cli::parse();

    // Mode selection
    if cli.serv {
        // Server mode - run WebSocket server
        run_server_mode()?;
        return;
    }

    if cli.tray || cli.command.is_none() {
        // GUI mode - launch egui application
        run_gui_mode()?;
        return;
    }

    // CLI mode
    init_logging(cli.verbose, cli.quiet, None)?;

    // Dispatch to command handler (delegates to calc/)
    let result = match cli.command {
        Commands::Product(args) => commands::product::execute(args, &cli),
        Commands::Order(args) => commands::order::execute(args, &cli),
        Commands::Hosting(args) => commands::hosting::execute(args, &cli),
        // ...
    };

    // Handle errors
    if let Err(e) = result {
        handle_error(&e, cli.json);
    }
}
```

---

## Key Patterns

### ID Generation

Hash-based short IDs for human readability:

```rust
pub struct IdConfig {
    pub prefix: String,         // e.g., "prod", "order", "msg"
    pub min_hash_length: usize, // 3
    pub max_hash_length: usize, // 8
    pub max_collision_prob: f64, // 0.25
}

// Generated: prod-abc123, order-def456, msg-ghi789
```

**Algorithm:**
1. Generate random bytes
2. Encode as alphanumeric hash
3. Start with min_length
4. Extend if collision detected
5. Fail if max_length reached

### Content Hashing

Deterministic hash for deduplication:

```rust
impl Product {
    pub fn compute_content_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.name.as_bytes());
        hasher.update(self.category.as_bytes());
        hasher.update(self.description.as_deref().unwrap_or("").as_bytes());
        // ... other fields
        format!("{:x}", hasher.finalize())
    }
}
```

**Excluded from hash:**
- `id` (generated)
- `created_at`, `updated_at` (timestamps)
- `reviews`, `orders` (relations)

### Atomic File Writes

Safe file updates using temp + rename:

```rust
fn atomic_write(path: &Path, content: &[u8]) -> Result<()> {
    let temp_path = path.with_extension("tmp");

    // Write to temp file
    let mut file = File::create(&temp_path)?;
    file.write_all(content)?;
    file.sync_all()?;

    // Atomic rename
    fs::rename(&temp_path, path)?;

    Ok(())
}
```

---

## Safety Invariants

### Data Safety Principles

Dure follows strict safety principles to protect user data and prevent accidental loss:

> All user data is stored locally. No automatic operations that could result in
> data loss. Explicit user confirmation required for destructive operations.

### Health States

(Not applicable to dure's architecture)

### Authority Model

(Not applicable to dure's architecture)

### Invariant Matrix

(Not applicable to dure's architecture)

### Primary-Data Repair Rules

(Not applicable to dure's architecture)

### Derived-State Rebuild Rules

(Not applicable to dure's architecture)

### Cross-Surface Reporting Contract

(Not applicable to dure's architecture)

### Incident Evidence Bundle

(Not applicable to dure's architecture)

### File System Safety

1. **Local storage**
   - All user data in local SQLite database
   - No automatic cloud uploads

2. **Path validation**
   - Validated paths for config and data files
   - No operations outside designated directories

3. **Atomic writes**
   - Temp file + rename pattern
   - No partial writes

### Database Safety

1. **WAL mode**
   - Concurrent readers
   - Crash recovery

2. **Immediate transactions**
   - Exclusive lock for writes
   - No dirty reads

3. **Schema versioning**
   - Version check on open
   - Migration support

### See Also

---

## Extension Points

### Adding New Commands

1. Create business logic in `mobile/src/calc/myfeature.rs`
2. Create CLI wrapper in `mobile/src/cli/commands/mycommand.rs`
3. Add args struct to `mobile/src/cli/mod.rs`
4. Add variant to `Commands` enum
5. Add dispatch in `main.rs`

### Adding New Entity Types

1. Add struct to model in `mobile/src/model/mod.rs`
2. Update `compute_content_hash()` if content-relevant
3. Add table in `schema.rs`
4. Update INSERT/SELECT in `sqlite.rs`
5. Add serialization in format modules
6. Add business logic in `calc/` module

### Custom Validators

Extend validators in `validation/mod.rs`:

```rust
impl DureValidator {
    pub fn validate_domain(&self, domain: &str) -> Result<()> {
        // Custom validation logic
    }
}
```

---

## Dependencies

| Crate | Purpose |
|-------|---------|
| `egui` + `eframe` | Cross-platform immediate-mode GUI framework (0.33) |
| `egui-material3` | Material3 design system for egui |
| `egui-i18n` | Internationalization with Fluent |
| `clap` | CLI parsing with derive macros (for CLI mode) |
| `fsqlite` + `fsqlite-types` + `fsqlite-error` | SQLite engine facade plus shared storage types/errors |
| `serde` + `serde_json` + `bincode` | Serialization |
| `schemars` | JSON Schema generation for structured output |
| `sha2` | Content hashing |
| `thiserror` + `anyhow` | Error types and context |
| `ureq` + `ehttp` | HTTP client operations |
| `env_logger` + `android_logger` | Platform-specific logging |
| `tray-icon` + `webbrowser` + `trash` | Desktop-specific features (platform-gated) |
| `ndk-context` + `jni` + `android-activity` | Android platform support (platform-gated) |
| `wasm-bindgen` + `web-sys` + `js-sys` | WASM platform support (platform-gated) |

---

## See Also

- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Commands, patterns, and common tasks
- [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) - Architecture overview
- [INDEX.md](INDEX.md) - Complete documentation index
- [../README.md](../README.md) - Project README
