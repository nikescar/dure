# darkhttpd-sys Crate Implementation Summary

## Overview

Successfully created a Rust crate (`darkhttpd-sys`) that wraps the darkhttpd C program using FFI (Foreign Function Interface). This allows the C-based HTTP server to be used as a Rust library.

## What Was Created

### 1. Project Structure

```
darkhttpd-sys/
├── Cargo.toml              # Crate manifest with dependencies
├── build.rs                # Build script to compile C code
├── darkhttpd_lib.c         # Modified darkhttpd C code with library API
├── README.md               # Comprehensive documentation
├── COPYING.darkhttpd       # Original ISC license from darkhttpd
├── .gitignore              # Git ignore rules
├── src/
│   ├── lib.rs              # Safe Rust wrapper API
│   └── ffi.rs              # Raw FFI bindings
├── examples/
│   ├── basic.rs            # Basic usage example
│   ├── advanced.rs         # Advanced configuration example
│   └── nonblocking.rs      # Non-blocking event loop example
└── tests/
    └── integration.rs      # Integration tests
```

### 2. Key Features

#### C Library Modifications
- Renamed `main()` to `darkhttpd_main_original()` to avoid symbol conflicts
- Added library API functions:
  - `darkhttpd_run()` - Run the full server (equivalent to main)
  - `darkhttpd_init()` - Initialize without starting
  - `darkhttpd_start()` - Start accepting connections
  - `darkhttpd_stop()` - Stop accepting connections
  - `darkhttpd_poll_once()` - Run one event loop iteration
  - `darkhttpd_is_running()` - Check if server is running
  - `darkhttpd_cleanup()` - Cleanup and free resources

#### Rust FFI Layer (`src/ffi.rs`)
- Raw `extern "C"` declarations
- Comprehensive safety documentation
- Type-safe bindings using `libc` types

#### Safe Rust Wrapper (`src/lib.rs`)
- `DarkHttpd` struct with RAII semantics
- Idiomatic Rust API with proper error handling
- Automatic cleanup via `Drop` implementation
- Three usage patterns:
  - `serve()` - Simple path + port
  - `serve_with_args()` - Full command-line arguments
  - `poll()` / `run()` - Manual event loop control

#### Build System (`build.rs`)
- Uses `cc` crate to compile C code
- Statically links darkhttpd into Rust binary
- Automatic rebuild on C source changes

### 3. API Examples

**Basic Usage:**
```rust
let mut server = DarkHttpd::new();
server.serve("/var/www/htdocs", 8080)?;
server.run(); // Blocks until stopped
```

**With Arguments:**
```rust
server.serve_with_args(
    "/var/www/htdocs",
    &["--port", "8080", "--log", "access.log"]
)?;
```

**Non-blocking:**
```rust
while server.poll() {
    // Do other work
}
```

### 4. Testing

- Unit tests for basic functionality (✅ passing)
- Integration tests for server lifecycle (✅ passing)
- Doc tests for all examples (✅ passing)
- Note: Due to darkhttpd's global state, only one server instance can be initialized per process

### 5. Documentation

- Comprehensive README with examples
- Inline documentation for all public APIs
- Three working examples demonstrating different usage patterns
- Safety documentation for FFI boundaries

## Technical Details

### Dependencies
- **cc** (build): Compiles C code
- **libc**: C type definitions
- **thiserror**: Error handling
- **ctrlc** (dev): Example signal handling

### Compatibility
- Requires C compiler (gcc/clang/msvc)
- Inherits darkhttpd platform support (Linux, BSD, macOS, Solaris)
- Uses workspace-level unsafe_code override (FFI requires unsafe)

### Integration with dure Workspace
- Added to workspace members in root `Cargo.toml`
- Overrides workspace `unsafe_code` lint (required for FFI)
- Follows workspace conventions for licensing and structure

## Known Limitations

1. **Single Instance**: darkhttpd uses global state, so only one initialized server instance per process
2. **Thread Safety**: darkhttpd is single-threaded; the wrapper is not `Send` or `Sync`
3. **Error Reporting**: Some C errors go to stderr/syslog rather than being propagated to Rust
4. **Signal Handling**: darkhttpd installs its own signal handlers which may conflict with Rust code

## Future Enhancements

Possible improvements (not implemented):
- Thread-safe wrapper using process isolation
- Better error propagation from C to Rust
- Async/await interface using tokio
- Multiple server instances using separate processes
- Configuration builder API instead of string arguments

## Build & Test Commands

```bash
# Build the crate
cargo build --release

# Run tests
cargo test

# Run examples
cargo run --example basic /var/www/htdocs
cargo run --example advanced
cargo run --example nonblocking

# Check documentation
cargo doc --open
```

## License

Dual-licensed:
- Rust code: MIT OR Apache-2.0 (matching dure)
- darkhttpd C code: ISC (original license, see COPYING.darkhttpd)

## Conclusion

The `darkhttpd-sys` crate successfully wraps the darkhttpd C program using FFI, providing both raw bindings and a safe, idiomatic Rust API. It's ready for use within the dure project or as a standalone library for serving static files in Rust applications.
