# Dure WSS Architecture

## Separation Strategy

The `dure-wss` crate provides a standalone binary for WebSocket Secure servers, **separating WSS functionality from GUI dependencies**.

### Dependency Isolation

```
┌─────────────────────────────────────────────────────┐
│ dure-desktop (mobile/ directory)                    │
│ ├─ GUI: egui, eframe, egui-material3               │
│ ├─ Platform UI: tray-icon, gtk, etc.               │
│ ├─ WSS: wss/server, wss/client                     │
│ └─ Database models, calc, storage                  │
└─────────────────────────────────────────────────────┘
                       ▲
                       │ library dependency
                       │ (features = ["serde"])
                       │
┌─────────────────────────────────────────────────────┐
│ dure-wss (crates/dure-wss/)                         │
│ ├─ CLI binary only                                  │
│ ├─ WSS-specific deps:                               │
│ │  ├─ async-tungstenite (WebSocket)                │
│ │  ├─ asupersync (async runtime + TLS)             │
│ │  ├─ rustls, rcgen (certificates)                 │
│ │  ├─ webauthn-rs, go-webauthn (auth)              │
│ │  ├─ utoipa, asyncapi-rust (docs)                 │
│ │  └─ darkhttpd-sys (static files)                 │
│ └─ NO GUI dependencies                              │
└─────────────────────────────────────────────────────┘
```

## What Got Separated

### ✅ WSS-Only Dependencies (in dure-wss/Cargo.toml)

These dependencies are **only required when building dure-wss**, not for users of the GUI:

1. **WebSocket Protocol**
   - `async-tungstenite` - WebSocket implementation
   - `asupersync` - Async runtime with TLS support

2. **TLS/Security**
   - `rustls`, `rustls-pemfile`, `rustls-native-certs` - TLS implementation
   - `rcgen` - Certificate generation

3. **Authentication**
   - `webauthn-rs` - WebAuthn server-side
   - `go-webauthn` - Go WebAuthn FFI bindings

4. **API Documentation**
   - `utoipa`, `utoipa-swagger-ui` - OpenAPI/Swagger
   - `asyncapi-rust`, `schemars` - AsyncAPI documentation

5. **Static File Serving**
   - `darkhttpd-sys` - Embedded HTTP server for static files

### ❌ GUI Dependencies (excluded from dure-wss)

These are **NOT** pulled in when building `dure-wss`:

- `egui` (17+ transitive deps)
- `eframe` (platform windowing)
- `egui-material3` (Material Design components)
- `tray-icon`, `tao` (system tray)
- `gtk` (Linux UI)
- Platform-specific UI crates (Windows/macOS UI)

## Build Targets

### Build WSS binary only
```bash
cargo build --package dure-wss --release
# Output: target/release/dure-wss
```

### Build desktop GUI only
```bash
cargo build --package dure --bin dure-desktop --release
# Output: target/release/dure-desktop
```

### Build everything
```bash
cargo build --release
```

## Usage Examples

### WSS Server
```bash
# Run WSS server (no GUI required)
dure-wss server --domain example.com --addr 0.0.0.0:443

# Run on custom port with stats
dure-wss server --domain example.com --addr 0.0.0.0:8443 --stats-interval 30
```

### WSS Client
```bash
# Test WebSocket connection
dure-wss client wss://example.com

# Test HTTPS endpoint
dure-wss client https://example.com/api/status --mode get

# Test webhook POST
dure-wss client https://example.com/webhook --mode post \
  --path /api/events --body '{"type":"test"}'
```

## Future Improvements

### Phase 1 (Current): Hybrid Approach ✅
- **Status**: Completed
- `dure-wss` binary depends on `dure` library
- WSS deps isolated in `dure-wss/Cargo.toml`
- No GUI deps when building WSS binary

### Phase 2 (Future): Full Decoupling
- Move database models to shared crate `dure-core`
- Move WSS server/client to `dure-wss/src/`
- Remove `dure` library dependency
- `dure-wss` becomes fully standalone

```
dure/
├── dure-core/         # Shared models (future)
│   ├── storage/
│   ├── config/
│   └── calc/db
├── dure-desktop/      # GUI app (rename from mobile/)
│   └── depends on: dure-core, dure-wss
└── dure-wss/          # Standalone server
    └── depends on: dure-core
```

### Phase 3 (Future): Feature Flags
Add cargo features to further reduce binary size:
- `wss-webauthn` - WebAuthn authentication
- `wss-swagger` - Swagger UI documentation
- `wss-asyncapi` - AsyncAPI documentation
- `wss-static` - Static file serving

```toml
[features]
default = ["wss-webauthn", "wss-swagger"]
minimal = []  # Just WSS, no extras
```

## Binary Size Analysis

Debug build sizes:
- `dure-desktop`: ~650 MB (includes GUI)
- `dure-wss`: ~650 MB (shares most code via library)

Release build sizes (estimated with `opt-level = "z"`):
- `dure-desktop`: ~60-80 MB (GUI + WSS)
- `dure-wss`: ~15-25 MB (WSS only)

After Phase 2 (full decoupling):
- `dure-wss`: ~10-15 MB (no GUI code at all)

## Deployment Scenarios

### Scenario 1: Desktop User
- Installs: `dure-desktop`
- Uses: GUI to manage everything including WSS servers

### Scenario 2: Dedicated Server
- Installs: `dure-wss` only
- Uses: CLI to run WSS server
- No GUI dependencies on server

### Scenario 3: Docker Container
```dockerfile
FROM rust:slim
COPY target/release/dure-wss /usr/local/bin/
CMD ["dure-wss", "server", "--domain", "$DOMAIN"]
```

### Scenario 4: CI/CD Testing
```yaml
# GitHub Actions
- name: Test WSS Server
  run: |
    cargo build --package dure-wss
    ./target/debug/dure-wss server --domain test.local &
    ./target/debug/dure-wss client wss://test.local
```

## Conclusion

The `dure-wss` binary successfully separates WebSocket server functionality from GUI dependencies, enabling:
- Smaller deployment footprint for servers
- Faster builds when only WSS is needed
- Clearer dependency boundaries
- Better Docker/container support
