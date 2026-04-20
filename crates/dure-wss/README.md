# Dure WSS - Standalone WebSocket Secure Server

A lightweight, standalone binary for running HTTPS/WSS servers without the full Dure GUI application.

## Features

- **HTTPS/WSS Server**: Combined HTTP and WebSocket server with TLS
- **ACME Support**: Automatic TLS certificate management
- **WebAuthn**: Hardware security key authentication
- **Session Tracking**: Database-backed session management
- **Webhook Support**: HTTP POST request logging with pattern matching
- **API Documentation**: OpenAPI (Swagger UI) and AsyncAPI docs
- **Static File Serving**: Serve web applications

## Architecture

This crate provides a standalone binary that depends on the main `dure` library for WSS functionality, but **excludes GUI dependencies** like:
- egui/eframe (UI framework)
- egui-material3 (Material Design components)
- Platform-specific UI dependencies

### Dependencies Isolation

**WSS-Specific Dependencies** (only in dure-wss):
- `async-tungstenite` - WebSocket protocol
- `asupersync` - Async runtime with TLS
- `rustls`, `rustls-pemfile`, `rcgen` - TLS certificate handling
- `webauthn-rs`, `go-webauthn` - WebAuthn authentication
- `utoipa`, `utoipa-swagger-ui` - OpenAPI documentation
- `asyncapi-rust` - AsyncAPI documentation
- `darkhttpd-sys` - Static file server

**Shared Dependencies** (from main `dure` library):
- Database models (sessions, webhooks, ACME certificates)
- Site-to-site messaging
- Common utilities

## Building

### Build WSS binary only (without GUI dependencies)
```bash
cargo build --package dure-wss --release
```

### Binary location
```
target/release/dure-wss
```

### Binary size comparison
- `dure-desktop` (full GUI): ~50-80 MB
- `dure-wss` (server only): ~15-25 MB (estimated)

## Usage

### Start WSS server
```bash
# Using ACME certificate from database
dure-wss server --domain example.com

# Custom bind address
dure-wss server --domain example.com --addr 0.0.0.0:8443

# Custom database path
dure-wss server --domain example.com --db-path /var/lib/dure/wss.db

# Skip downloading static files
dure-wss server --domain example.com --no-download

# Custom stats interval
dure-wss server --domain example.com --stats-interval 120
```

### Test client
```bash
# WebSocket connection
dure-wss client wss://example.com

# HTTPS GET request
dure-wss client https://example.com --mode get --path /api/status

# HTTPS POST request
dure-wss client https://example.com --mode post --path /api/webhook --body '{"event":"test"}'

# Self-signed certificate (skip verification)
dure-wss client wss://localhost:8443 --insecure
```

## Environment Variables

- `RUST_LOG` - Logging level (debug, info, warn, error)
- `DATABASE_URL` - Override default database path

## Database

Default database location:
- Linux: `~/.local/share/dure/wss.db`
- macOS: `~/Library/Application Support/pe.nikescar.dure/wss.db`
- Windows: `%APPDATA%\nikescar\dure\data\wss.db`

## Integration with Main Dure Application

The main `dure` desktop application can still manage WSS servers through its GUI. The `dure-wss` binary is for:
- Dedicated server deployments
- Docker/containerized environments
- Headless servers without display
- CI/CD testing

## Future Plans

- [ ] Fully decouple from main `dure` library (move models to `dure-wss`)
- [ ] Add `dure-wss status` command for server monitoring
- [ ] Add `dure-wss stop` command for graceful shutdown
- [ ] Support for multiple domains in single process
- [ ] Prometheus metrics export
- [ ] Health check endpoints

## License

Dual-licensed under MIT OR Apache-2.0 (same as main Dure project).
