# Changelog - dure-wss

## [Unreleased]

### Added (2026-04-20)
- Initial creation of `dure-wss` crate for standalone WSS server binary
- Separated WSS-specific dependencies from main GUI application
- Created `dure-wss` CLI with `server` and `client` commands
- Added WSS dependency isolation in Cargo.toml:
  - async-tungstenite (WebSocket)
  - asupersync (async runtime)
  - rustls, rcgen (TLS)
  - webauthn-rs, go-webauthn (authentication)
  - utoipa, asyncapi-rust (API docs)
  - darkhttpd-sys (static files)
- Documentation:
  - README.md - User guide and features
  - ARCHITECTURE.md - Technical design and future roadmap
  - CHANGELOG.md - Version history

### Benefits
- Build WSS server without pulling in egui/GUI dependencies
- Faster compilation for server-only deployments
- Smaller binary size for production servers
- Better Docker/container support
- CI/CD testing isolation

### Implementation Notes
- Currently uses hybrid approach: `dure-wss` depends on `dure` library
- WSS dependencies isolated to `dure-wss/Cargo.toml` only
- Future: Full decoupling with shared `dure-core` crate

## Future Versions

### [0.2.0] - Planned
- [ ] Add `status`, `start`, `stop` commands
- [ ] Process management (PID files, daemon mode)
- [ ] Configuration file support
- [ ] Multi-domain support in single process

### [1.0.0] - Planned
- [ ] Full decoupling from main `dure` library
- [ ] Standalone database models
- [ ] Feature flags for optional components
- [ ] Prometheus metrics export
- [ ] Health check endpoints
