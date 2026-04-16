# AGENTS.md — dure

> Guidelines for AI coding agents working in this Rust codebase.

---

## RULE 0 - THE FUNDAMENTAL OVERRIDE PREROGATIVE

If I tell you to do something, even if it goes against what follows below, YOU MUST LISTEN TO ME. I AM IN CHARGE, NOT YOU.

---

## RULE NUMBER 1: NO FILE DELETION

**YOU ARE NEVER ALLOWED TO DELETE A FILE WITHOUT EXPRESS PERMISSION.** Even a new file that you yourself created, such as a test code file. You have a horrible track record of deleting critically important files or otherwise throwing away tons of expensive work. As a result, you have permanently lost any and all rights to determine that a file or folder should be deleted.

**YOU MUST ALWAYS ASK AND RECEIVE CLEAR, WRITTEN PERMISSION BEFORE EVER DELETING A FILE OR FOLDER OF ANY KIND.**

---

## Irreversible Git & Filesystem Actions — DO NOT EVER BREAK GLASS

1. **Absolutely forbidden commands:** `git reset --hard`, `git clean -fd`, `rm -rf`, or any command that can delete or overwrite code/data must never be run unless the user explicitly provides the exact command and states, in the same message, that they understand and want the irreversible consequences.
2. **No guessing:** If there is any uncertainty about what a command might delete or overwrite, stop immediately and ask the user for specific approval. "I think it's safe" is never acceptable.
3. **Safer alternatives first:** When cleanup or rollbacks are needed, request permission to use non-destructive options (`git status`, `git diff`, `git stash`, copying to backups) before ever considering a destructive command.
4. **Mandatory explicit plan:** Even after explicit user authorization, restate the command verbatim, list exactly what will be affected, and wait for a confirmation that your understanding is correct. Only then may you execute it—if anything remains ambiguous, refuse and escalate.
5. **Document the confirmation:** When running any approved destructive command, record (in the session notes / final response) the exact user text that authorized it, the command actually run, and the execution time. If that record is absent, the operation did not happen.

---

## Git Branch: ONLY Use `main`, NEVER `master`

**The default branch is `main`. The `master` branch exists only for legacy URL compatibility.**

- **All work happens on `main`** — commits, PRs, feature branches all merge to `main`
- **Never reference `master` in code or docs** — if you see `master` anywhere, it's a bug that needs fixing
- **The `master` branch must stay synchronized with `main`** — after pushing to `main`, also push to `master`:
  ```bash
  git push origin main:master
  ```

**If you see `master` referenced anywhere:**
1. Update it to `main`
2. Ensure `master` is synchronized: `git push origin main:master`

---

## Toolchain: Rust & Cargo

We only use **Cargo** in this project, NEVER any other package manager.

- **Edition:** Rust 2024 (nightly required — see `rust-toolchain.toml`)
- **Dependency versions:** Explicit versions for stability
- **Configuration:** Cargo.toml only (single crate, not a workspace)
- **Unsafe code:** Forbidden (`#![forbid(unsafe_code)]` via crate lints)

### Async Runtime: asupersync (MANDATORY — NO TOKIO)

**This project uses [asupersync](/dp/asupersync) exclusively for all async/concurrent operations. Tokio and the entire tokio ecosystem are FORBIDDEN.**

- **Structured concurrency**: `Cx`, `Scope`, `region()` — no orphan tasks
- **Cancel-correct channels**: Two-phase `reserve()/send()` — no data loss on cancellation
- **Sync primitives**: `asupersync::sync::Mutex`, `RwLock`, `OnceCell`, `Pool` — cancel-aware
- **Deterministic testing**: `LabRuntime` with virtual time, DPOR, oracles
- **Native HTTP**: `asupersync::http::h1` for network operations (replaces reqwest)

**Forbidden crates**: `tokio`, `hyper`, `reqwest`, `axum`, `tower` (tokio adapter), `async-std`, `smol`, or any crate that transitively depends on tokio.

**Pattern**: All async functions take `&Cx` as first parameter. All database operations return `Outcome<T, E>` (not `Result`). The `Cx` flows down from the consumer's runtime — sqlmodel does NOT create its own runtime.

### Key Dependencies

| Crate | Purpose |
|-------|---------|
| `egui` + `eframe` | Cross-platform immediate-mode GUI framework (0.33) |
| `egui-material3` | Material3 design system for egui |
| `egui-i18n` | Internationalization with Fluent |
| `diesel` + `diesel_migrations` | Database ORM with embedded migrations (SQLite/PostgreSQL support) |
| `ureq` + `ehttp` | HTTP client operations |
| `serde` + `serde_json` + `bincode` | Data serialization |
| `schemars` | JSON Schema generation for structured output |
| `clap` | CLI argument parsing (for CLI mode) |
| `anyhow` + `thiserror` | Error handling (anyhow for CLI, thiserror for typed errors) |
| `sha2` | Content hashing for deduplication |
| `env_logger` + `android_logger` | Platform-specific logging |
| `tray-icon` + `webbrowser` + `trash` | Desktop-specific features (platform-gated) |
| `ndk-context` + `jni` + `android-activity` | Android platform support (platform-gated) |
| `wasm-bindgen` + `web-sys` + `js-sys` | WASM platform support (platform-gated) |

### Release Profile

The release build optimizes for binary size (this is a CLI tool for distribution):

```toml
[profile.release]
opt-level = "z"     # Optimize for size (lean binary for distribution)
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit for better optimization
panic = "abort"     # Smaller binary, no unwinding overhead
strip = true        # Remove debug symbols
```

---

## Code Editing Discipline

### No Script-Based Changes

**NEVER** run a script that processes/changes code files in this repo. Brittle regex-based transformations create far more problems than they solve.

- **Always make code changes manually**, even when there are many instances
- For many simple changes: use parallel subagents
- For subtle/complex changes: do them methodically yourself

### No File Proliferation

If you want to change something or add a feature, **revise existing code files in place**.

**NEVER** create variations like:
- `mainV2.rs`
- `main_improved.rs`
- `main_enhanced.rs`

New files are reserved for **genuinely new functionality** that makes zero sense to include in any existing file. The bar for creating new files is **incredibly high**.

---

## Backwards Compatibility

We do not care about backwards compatibility—we're in early development with no users. We want to do things the **RIGHT** way with **NO TECH DEBT**.

- Never create "compatibility shims"
- Never create wrapper functions for deprecated APIs
- Just fix the code directly

---

## Compiler Checks (CRITICAL)

**After any substantive code changes, you MUST verify no errors were introduced:**

```bash
# Check for compiler errors and warnings
cargo check --all-targets

# Check for clippy lints (pedantic + nursery are enabled)
cargo clippy --all-targets -- -D warnings

# Verify formatting
cargo fmt --check
```

If you see errors, **carefully understand and resolve each issue**. Read sufficient context to fix them the RIGHT way.

---

## Testing

### Testing Policy

Every module includes inline `#[cfg(test)]` unit tests alongside the implementation. Tests must cover:
- Happy path
- Edge cases (empty input, max values, boundary conditions)
- Error conditions

Integration and end-to-end tests live in the `tests/` directory.

### Unit Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run tests for a specific module
cargo test storage
cargo test cli
cargo test sync
cargo test format
cargo test model
cargo test validation

# Run tests with all features enabled
cargo test --all-features
```

### Test Categories

| Directory / Pattern | Focus Areas |
|---------------------|-------------|
| `src/` (inline `#[cfg(test)]`) | Unit tests for each module: model, storage, sync, config, error, format, util, validation |
| `tests/e2e_*.rs` | End-to-end CLI tests: lifecycle, labels, deps, sync, history, search, comments, epics, workspaces, errors, completions |
| `tests/conformance*.rs` | Go/Rust parity: schema compatibility, text output matching, edge cases, labels+comments, workflows |
| `tests/storage_*.rs` | Storage layer: CRUD, list filters, ready queries, deps, history, blocked cache, export atomicity, invariants, ID/hash parity |
| `tests/proptest_*.rs` | Property-based tests: ID generation, hash determinism, time parsing, validation rules |
| `tests/repro_*.rs` | Regression tests: specific bugs reproduced and prevented |
| `tests/jsonl_import_export.rs` | JSONL round-trip fidelity |
| `tests/markdown_import.rs` | Markdown import parsing |
| `benches/storage_perf.rs` | Storage operation benchmarks (criterion) |

### Test Fixtures

Shared test fixtures live in `tests/fixtures/` and `tests/common/` for reusable test harness helpers (temp DB creation, test data builders).

---

## Third-Party Library Usage

If you aren't 100% sure how to use a third-party library, **SEARCH ONLINE** to find the latest documentation and current best practices.

---

## dure — This Project

**This is the project you're working on.** dure is a distributed e-commerce client and hosting solution built with Rust and egui. It enables small shop owners to run e-commerce operations without traditional centralized server infrastructure.

### What It Does

Provides distributed e-commerce platform with identity management, guest/store fronts (WASM), hosting management (DNS, DB, site deployment), and store management (products, orders, shipments, accounts). Supports multi-platform deployment: Desktop (Linux, Windows, macOS), Android, and WASM. Designed for AI-assisted operations with structured output modes.

### Architecture

```
CLI/GUI (clap derive + egui)
    │
    ├── Commands ────── hosting, role, member, device, channel, message,
    │                   reaction, dm, thread, poll, product, order, payment, review
    │                       │
    │                       ▼
    ├── Storage ─────── SQLite/PostgreSQL (Diesel ORM)
    │                       │
    │                       ├── Schema (embedded migrations via diesel_migrations)
    │                       ├── DB Module (mobile/src/calc/db.rs - unified connection)
    │                       └── Queries (type-safe queries via Diesel)
    │
    ├── Model ──────── Hosting, Server, Role, Member, Device, Channel, Message,
    │                   Product, Order, Payment, Review
    │                       │
    │                       └── Content hashing (SHA-256 dedup)
    │
    ├── Config ─────── Layered config (file + env + CLI flags)
    │                       │
    │                       └── DNS, Web, DB provider configurations
    │
    ├── Format ─────── Rich (panels, tables, colors), Plain, CSV, Markdown, Syntax
    │
    ├── Output ─────── Mode detection (TTY → Rich, pipe → Plain, --json → JSON)
    │                       │
    │                       └── Components (reusable output widgets)
    │
    ├── Validation ─── Input validation (domain names, IDs, configs)
    │
    └── Error ──────── Structured errors with exit codes (DureError + ErrorCode)
```

### Project Structure

```
dure/
├── Cargo.toml                     # Workspace manifest
├── mobile/                        # Main application code
│   ├── src/                       # Rust source code
│   │   ├── main.rs                # Application entry point (CLI/GUI/WASM modes)
│   │   ├── lib.rs                 # Library root, module declarations
│   │   ├── cli/
│   │   │   ├── mod.rs             # CLI argument parsing, output mode detection
│   │   │   └── commands/          # Command implementations: hosting, role, member,
│   │   │                          # device, channel, message, reaction, dm, thread,
│   │   │                          # poll, product, order, payment, review
│   │   ├── model/
│   │   │   └── mod.rs             # Hosting, Server, Role, Member, Device, Channel,
│   │   │                          # Message, Product, Order, Payment, Review types
│   │   ├── storage/
│   │   │   ├── mod.rs             # Storage trait
│   │   │   ├── sqlite.rs          # SQLite backend for desktop/Android
│   │   │   ├── schema.rs          # DDL migrations
│   │   │   ├── events.rs          # Append-only audit log
│   │   │   └── queries/           # Reusable query fragments
│   │   ├── config/
│   │   │   ├── mod.rs             # Layered configuration
│   │   │   └── routing.rs         # DNS, Web, DB provider config resolution
│   │   ├── error/
│   │   │   ├── mod.rs             # DureError enum
│   │   │   ├── structured.rs      # StructuredError with ErrorCode + exit codes
│   │   │   └── context.rs         # Error context helpers
│   │   ├── format/
│   │   │   ├── mod.rs             # Format module root
│   │   │   ├── rich.rs            # Rich terminal output (panels, tables)
│   │   │   ├── text.rs            # Plain text formatting
│   │   │   ├── csv.rs             # CSV export
│   │   │   ├── markdown.rs        # Markdown formatting
│   │   │   ├── syntax.rs          # Syntax highlighting
│   │   │   ├── theme.rs           # Color themes
│   │   │   ├── context.rs         # Format context (width, mode)
│   │   │   └── output.rs          # Output helpers
│   │   ├── output/
│   │   │   ├── mod.rs             # Output mode detection (Rich/Plain/JSON/Quiet)
│   │   │   ├── context.rs         # Output context
│   │   │   ├── theme.rs           # Output theming
│   │   │   └── components/        # Reusable output widgets
│   │   ├── validation/
│   │   │   └── mod.rs             # Input validation rules (domain names, IDs, configs)
│   │   ├── util/
│   │   │   ├── mod.rs             # Utility module root
│   │   │   ├── id.rs              # Hash-based short ID generation
│   │   │   ├── hash.rs            # SHA-256 content hashing
│   │   │   ├── time.rs            # Timestamp parsing/formatting
│   │   │   └── progress.rs        # Progress spinners
│   │   └── logging.rs             # Logging setup (env_logger/android_logger)
│   ├── app/                       # Android app configuration
│   ├── assets/                    # Application assets
│   └── Cargo.toml                 # Package manifest
├── docs/                          # Documentation (see docs/INDEX.md)
│   ├── INDEX.md                   # Complete documentation index
│   ├── PROJECT_SUMMARY.md         # Architecture overview
│   ├── QUICK_REFERENCE.md         # Commands and patterns
│   └── ...                        # Additional documentation
├── deploy/                        # Deployment configurations
├── fastlane/                      # Mobile CI/CD automation
├── snap/                          # Snap package configuration
└── scripts/                       # Build and utility scripts
```

### Key Files by Module

| Module | Key Files | Purpose |
|--------|-----------|---------|
| `cli` | `cli/mod.rs` | Clap argument parsing, output mode detection, dispatch logic |
| `cli/commands` | `commands/*.rs` | Command implementations: hosting, role, member, device, channel, message, reaction, dm, thread, poll, product, order, payment, review |
| `model` | `model/mod.rs` | `Hosting`, `Server`, `Role`, `Member`, `Device`, `Channel`, `Message`, `Product`, `Order`, `Payment`, `Review` types, content hashing, serde derives |
| `storage` | `storage/diesel_schema.rs` | Diesel-generated schema definitions for all tables |
| `storage` | `storage/models/` | Storage models for each domain (dns, hosting, session, etc.) |
| `calc` | `calc/db.rs` | Unified database connection and migration management (Diesel-based) |
| `config` | `config/mod.rs` | Layered config: file + env vars + CLI flags, DNS/Web/DB provider resolution |
| `error` | `error/structured.rs` | `StructuredError` with `ErrorCode` enum and deterministic exit codes |
| `validation` | `validation/mod.rs` | Input validation: domain names, IDs, configs, API keys |
| `util` | `util/id.rs` | Hash-based short ID generation |
| `util` | `util/hash.rs` | SHA-256 content hashing for deduplication |
| `format` | `format/rich.rs` | Rich terminal output (panels, tables, colors) |

### Feature Flags

```toml
[features]
default = ["self_update"]
self_update = ["dep:self_update"]   # Self-update from GitHub releases (rustls TLS, signature verification)
```

### Core Types Quick Reference

| Type | Purpose |
|------|---------|
| `Hosting` | Hosting configuration: domain, DNS provider, web provider, DB provider, API keys |
| `Server` | Server instance with roles, members, channels |
| `Role` | User role with permissions, color, hoist, mentionable settings |
| `Member` | Server member with roles, join date (owner member vs store member) |
| `Device` | User device registration and authentication |
| `Channel` | Communication channel (text/voice/forum) |
| `Message` | Chat message with reactions, threads, attachments |
| `Product` | E-commerce product: name, category, image, contents, price |
| `Order` | Customer order with products, counts, status (pending/processing/shipping/completed) |
| `Payment` | Payment record with method, status, gateway integration |
| `Review` | Product/store review with rating, contents |
| `Event` | Append-only audit entry for all mutations |
| `DureError` | Unified error enum (thiserror-derived) with structured variants |
| `ErrorCode` | Deterministic exit code mapping |
| `StructuredError` | JSON-serializable error with code, message, context |
| `OutputMode` | Enum: `Rich`, `Plain`, `Json`, `Quiet` — auto-detected from terminal state |

### Key Design Decisions

- **Distributed architecture** — No centralized server dependency; peer-to-peer communication via WebSocket
- **Multi-platform support** — Desktop (Linux, Windows, macOS), Android, WASM with shared codebase
- **egui for UI** — Cross-platform immediate-mode GUI with Material3 design
- **Diesel ORM** — Type-safe database operations with SQLite (desktop/Android/WASM) and optional PostgreSQL
- **Embedded migrations** — All migrations bundled in binary via diesel_migrations, no external CLI needed
- **Unified DB module** — Single connection point at mobile/src/calc/db.rs for all database operations
- **Multiple output modes** — Rich (TTY), Plain (pipe/NO_COLOR), JSON (--json), Quiet (--quiet) — auto-detected
- **Append-only audit log** — Every mutation recorded in events table for full traceability
- **Layered configuration** — File + env vars + CLI flags for DNS/Web/DB provider settings
- **Platform-specific compilation** — Uses `#[cfg(target_os = "...")]` instead of feature flags
- **Async runtime** — tokio multi-threaded runtime for I/O operations
- **Hosting flexibility** — Supports Firebase, Supabase, GCE, Cafe24 VPS backends

---

## Output Modes

dure supports multiple output modes for different use cases:

| Mode | When Active | Description |
|------|-------------|-------------|
| **Rich** | TTY with colors | Colored panels, tables, styled text |
| **Plain** | `NO_COLOR` env or `--no-color` | Text output without ANSI codes |
| **JSON** | `--json` flag | Machine-readable structured output |
| **Quiet** | `--quiet` or `-q` | Minimal output |

### Mode Detection

The output mode is automatically detected:

1. `--json` flag → **JSON mode**
2. `--quiet` flag → **Quiet mode**
3. `NO_COLOR` env var or `--no-color` → **Plain mode**
4. Non-TTY stdout (piped output) → **Plain mode**
5. Otherwise → **Rich mode** (default for interactive terminals)

### For Coding Agents

**CRITICAL:** Always use `--json` flag when parsing dure output programmatically.

```bash
# CORRECT - stable, parseable output
dure product list "MyStore" --json | jq '.products[0]'
dure hosting status --json

# WRONG - output format may vary based on terminal state
dure product list "MyStore" | head -1
```

JSON mode guarantees:
- Stable schema
- No ANSI escape codes
- Clean stdout (diagnostics go to stderr)
- Exit codes for success/failure

Schema discovery:
- `dure schema all --format json` emits JSON Schema documents
- `dure schema product --format json` for product schema

---

## Dure — Distributed E-commerce Platform

Dure provides a distributed e-commerce platform with hosting management, store operations, and guest interactions. It supports multiple hosting providers and WebSocket-based peer-to-peer communication.

**Important:** Development follows standard git workflows with proper testing and validation before commits.

### Conventions

- **Configuration management:** DNS, Web, and DB provider configs stored in layered configuration (file + env + CLI flags)
- **Member identification:** Owner members use IDs without dots; Store members use domain-based IDs with dots
- **Output format:** Always use `--json` flag for structured output in automation and agent workflows
- **Platform compilation:** Use `#[cfg(target_os = "...")]` for platform-specific code, not feature flags

### Typical Agent Flow

1. **Setup hosting infrastructure:**
   ```bash
   dure hosting init --json  # Initialize hosting with DNS, web, DB providers
   dure hosting show --json  # Verify configuration
   ```

2. **Configure store (for service providers):**
   ```bash
   dure role create "HostName" "Manager" --json
   dure channel create "HostName" "store-channel" --type text --json
   ```

3. **Manage products and orders:**
   ```bash
   dure product create "Server" "Product Name" "Category" "image.png" "Description" --json
   dure order list "Server" --json
   dure payment list "Server" --json
   ```

4. **Monitor and interact:**
   ```bash
   dure message list "#channel" --json
   dure review list "Server" --json
   ```

5. **Commit changes:**
   ```bash
   git add <files>
   git commit -m "Description"
   git push
   ```

### Mapping Cheat Sheet

| Concept | Structure |
|---------|-----------|
| Object hierarchy | `Hosting -> Server -> Role/Channel/Product/Order` |
| Member ID (owner) | `user123` (no dot) |
| Member ID (store) | `store.example.com` (domain with dot) |
| Channel reference | `#channel-name` |
| Server modes | `--serv` (server), `--tray` (GUI), CLI (default) |
| Hosting providers | Firebase, Supabase, GCE, Cafe24 VPS |

---

## UBS — Ultimate Bug Scanner

**Golden Rule:** `ubs <changed-files>` before every commit. Exit 0 = safe. Exit >0 = fix & re-run.

### Commands

```bash
ubs file.rs file2.rs                    # Specific files (< 1s) — USE THIS
ubs $(git diff --name-only --cached)    # Staged files — before commit
ubs --only=rust,toml src/               # Language filter (3-5x faster)
ubs --ci --fail-on-warning .            # CI mode — before PR
ubs .                                   # Whole project (ignores target/, Cargo.lock)
```

### Output Format

```
⚠️  Category (N errors)
    file.rs:42:5 – Issue description
    💡 Suggested fix
Exit code: 1
```

Parse: `file:line:col` → location | 💡 → how to fix | Exit 0/1 → pass/fail

### Fix Workflow

1. Read finding → category + fix suggestion
2. Navigate `file:line:col` → view context
3. Verify real issue (not false positive)
4. Fix root cause (not symptom)
5. Re-run `ubs <file>` → exit 0
6. Commit

### Bug Severity

- **Critical (always fix):** Memory safety, use-after-free, data races, SQL injection
- **Important (production):** Unwrap panics, resource leaks, overflow checks
- **Contextual (judgment):** TODO/FIXME, println! debugging

---

## RCH — Remote Compilation Helper

RCH offloads `cargo build`, `cargo test`, `cargo clippy`, and other compilation commands to a fleet of 8 remote Contabo VPS workers instead of building locally. This prevents compilation storms from overwhelming csd when many agents run simultaneously.

**RCH is installed at `~/.local/bin/rch` and is hooked into Claude Code's PreToolUse automatically.** Most of the time you don't need to do anything if you are Claude Code — builds are intercepted and offloaded transparently.

To manually offload a build:
```bash
rch exec -- cargo build --release
rch exec -- cargo test
rch exec -- cargo clippy
```

Quick commands:
```bash
rch doctor                    # Health check
rch workers probe --all       # Test connectivity to all 8 workers
rch status                    # Overview of current state
rch queue                     # See active/waiting builds
```

If rch or its workers are unavailable, it fails open — builds run locally as normal.

**Note for Codex/GPT-5.2:** Codex does not have the automatic PreToolUse hook, but you can (and should) still manually offload compute-intensive compilation commands using `rch exec -- <command>`. This avoids local resource contention when multiple agents are building simultaneously.

---

## ast-grep vs ripgrep

**Use `ast-grep` when structure matters.** It parses code and matches AST nodes, ignoring comments/strings, and can **safely rewrite** code.

- Refactors/codemods: rename APIs, change import forms
- Policy checks: enforce patterns across a repo
- Editor/automation: LSP mode, `--json` output

**Use `ripgrep` when text is enough.** Fastest way to grep literals/regex.

- Recon: find strings, TODOs, log lines, config values
- Pre-filter: narrow candidate files before ast-grep

### Rule of Thumb

- Need correctness or **applying changes** → `ast-grep`
- Need raw speed or **hunting text** → `rg`
- Often combine: `rg` to shortlist files, then `ast-grep` to match/modify

### Rust Examples

```bash
# Find structured code (ignores comments)
ast-grep run -l Rust -p 'fn $NAME($$$ARGS) -> $RET { $$$BODY }'

# Find all unwrap() calls
ast-grep run -l Rust -p '$EXPR.unwrap()'

# Quick textual hunt
rg -n 'println!' -t rust

# Combine speed + precision
rg -l -t rust 'unwrap\(' | xargs ast-grep run -l Rust -p '$X.unwrap()' --json
```

---

## Morph Warp Grep — AI-Powered Code Search

**Use `mcp__morph-mcp__warp_grep` for exploratory "how does X work?" questions.** An AI agent expands your query, greps the codebase, reads relevant files, and returns precise line ranges with full context.

**Use `ripgrep` for targeted searches.** When you know exactly what you're looking for.

**Use `ast-grep` for structural patterns.** When you need AST precision for matching/rewriting.

### When to Use What

| Scenario | Tool | Why |
|----------|------|-----|
| "How does the sync engine handle conflicts?" | `warp_grep` | Exploratory; don't know where to start |
| "Where is the content hash computed?" | `warp_grep` | Need to understand architecture |
| "Find all uses of `BeadsError::IssueNotFound`" | `ripgrep` | Targeted literal search |
| "Find files with `println!`" | `ripgrep` | Simple pattern |
| "Replace all `unwrap()` with `expect()`" | `ast-grep` | Structural refactor |

### warp_grep Usage

```
mcp__morph-mcp__warp_grep(
  repoPath: "/path/to/dure",
  query: "How does the WebSocket authentication work?"
)
```

Returns structured results with file paths, line ranges, and extracted code snippets.

### Anti-Patterns

- **Don't** use `warp_grep` to find a specific function name → use `ripgrep`
- **Don't** use `ripgrep` to understand "how does X work" → wastes time with manual reads
- **Don't** use `ripgrep` for codemods → risks collateral edits

<!-- bv-agent-instructions-v1 -->

---

## Dure Workflow Integration

This project is a distributed e-commerce platform. Users can be Guests (customers) or Service Providers (store owners).

### Essential Commands

```bash
# Server mode (WebSocket server)
dure --serv

# Client mode (GUI - default)
dure --tray

# CLI mode examples
dure hosting list --json
dure channel list --server "Host1" --json
dure message list "#general" --json
dure product list "Server" --json
dure order list "Server" --json
```

### Workflow Pattern

#### 1. Setup Hosting
Create Hosting Server and Ready for Service.
1. Select DNS Registar and get API key(Porkbun, Cloudflare) or Domain Name(DuckDns)
2. Select NS Server and API key(Duckdns, Porkbun, Cloudflare)
3. Select VM Cloud(GCE or Cafe24) and get API Key. Prepare SSH Account and Key or Password
4. Initial Remote Host Setup for Dure Websocket Server

#### 2. Setup Dure (Guest)
Set Configuration to Dure WebServer.
1. Start to connect other dure server
2. Browsing products from other server
3. Order from other server

#### 3. Setup Store (Service Provider)
Set Configuration to Store WebServer.
1. Start to connect Own dure server
2. Create Products to server
3. Start to sell

### Key Concepts

- **Multi-platform**: Desktop (Linux, Windows, macOS), Android, WASM
- **Three modes**: Server mode (--serv), GUI mode (--tray), CLI mode
- **Hosting providers**: Firebase, Supabase, GCE, Cafe24 VPS
- **Member types**: Owner members (no dot in ID) vs Store members (dot allowed in ID)
- **Output modes**: Rich (TTY), Plain (pipe), JSON (--json), Quiet (--quiet)

### Session Protocol

**Before ending any session, run this checklist:**

```bash
git status              # Check what changed
cargo check             # Verify compilation
cargo clippy            # Check linting
cargo test              # Run tests (if applicable)
git add <files>         # Stage changes
git commit -m "..."     # Commit with descriptive message
git push                # Push to remote
```

### Best Practices

- Use `--json` flag for structured output in automation
- Test on target platforms when making platform-specific changes
- Follow Material3 design guidelines for UI components
- Validate API configurations before committing
- Document public APIs with examples

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **Commit changes** - Ensure all work is committed and pushed
5. **Hand off** - Provide context for next session


---

## cass — Cross-Agent Session Search

`cass` indexes prior agent conversations (Claude Code, Codex, Cursor, Gemini, ChatGPT, etc.) so we can reuse solved problems.

**Rules:** Never run bare `cass` (TUI). Always use `--robot` or `--json`.

### Examples

```bash
cass health
cass search "async runtime" --robot --limit 5
cass view /path/to/session.jsonl -n 42 --json
cass expand /path/to/session.jsonl -n 42 -C 3 --json
cass capabilities --json
cass robot-docs guide
```

### Tips

- Use `--fields minimal` for lean output
- Filter by agent with `--agent`
- Use `--days N` to limit to recent history

stdout is data-only, stderr is diagnostics; exit code 0 means success.

Treat cass as a way to avoid re-solving problems other agents already handled.

---

Note for Codex/GPT-5.2:

You constantly bother me and stop working with concerned questions that look similar to this:

```
Unexpected changes (need guidance)

- Working tree still shows edits I did not make in Cargo.toml, Cargo.lock, src/cli/commands/upgrade.rs, src/storage/sqlite.rs, tests/conformance.rs, tests/storage_deps.rs. Please advise whether to keep/commit/revert these before any further work. I did not touch them.

Next steps (pick one)

1. Decide how to handle the unrelated modified files above so we can resume cleanly.
2. Triage beads_rust-orko (clippy/cargo warnings) and beads_rust-ydqr (rustfmt failures).
3. If you want a full suite run later, fix conformance/clippy blockers and re‑run cargo test --all.
```

NEVER EVER DO THAT AGAIN. The answer is literally ALWAYS the same: those are changes created by the potentially dozen of other agents working on the project at the same time. This is not only a common occurence, it happens multiple times PER MINUTE. The way to deal with it is simple: you NEVER, under ANY CIRCUMSTANCE, stash, revert, overwrite, or otherwise disturb in ANY way the work of other agents. Just treat those changes identically to changes that you yourself made. Just fool yourself into thinking YOU made the changes and simply don't recall it for some reason.

---

## Note on Built-in TODO Functionality

If I ask you to explicitly use your built-in TODO functionality, don't complain about this. You can use built-in TODOs if I tell you specifically to do so. Always comply with such orders.
