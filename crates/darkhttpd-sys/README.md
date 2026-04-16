# darkhttpd-sys

Rust FFI bindings for [darkhttpd](https://unix4lyfe.org/darkhttpd/) - a simple, single-threaded, static content webserver.

## About

This crate provides both low-level FFI bindings and a safe, idiomatic Rust wrapper for the darkhttpd HTTP server. darkhttpd is a lightweight, efficient static file server written in C.

## Features

- **Simple to use**: Just point it at a directory and go
- **Lightweight**: Small memory footprint, single-threaded event loop
- **Fast**: Uses sendfile() on supported platforms
- **Safe Rust API**: Idiomatic wrapper with automatic resource cleanup
- **Direct C access**: Raw FFI bindings available for advanced use

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
darkhttpd-sys = { path = "../darkhttpd-sys" }
```

### Basic Example

```rust
use darkhttpd_sys::DarkHttpd;

fn main() {
    let mut server = DarkHttpd::new();
    
    // Serve files from /var/www/htdocs on port 8080
    server.serve("/var/www/htdocs", 8080)
        .expect("Failed to start server");
    
    println!("Server running on http://localhost:8080");
    
    // Run until stopped (blocking)
    server.run();
}
```

### With Custom Arguments

```rust
use darkhttpd_sys::DarkHttpd;

fn main() {
    let mut server = DarkHttpd::new();
    
    // Start with custom arguments
    server.serve_with_args(
        "/var/www/htdocs",
        &[
            "--port", "8080",
            "--log", "access.log",
            "--maxconn", "100",
        ]
    ).expect("Failed to start server");
    
    server.run();
}
```

### Non-blocking Event Loop

```rust
use darkhttpd_sys::DarkHttpd;
use std::thread;
use std::time::Duration;

fn main() {
    let mut server = DarkHttpd::new();
    server.serve("/var/www/htdocs", 8080)
        .expect("Failed to start server");
    
    // Run the event loop manually
    while server.poll() {
        // Do other work here
        thread::sleep(Duration::from_millis(10));
    }
}
```

## Supported darkhttpd Options

The following command-line options are supported via `serve_with_args`:

- `--port <port>` - Port to listen on
- `--addr <ip>` - IP address to bind to
- `--maxconn <count>` - Maximum simultaneous connections
- `--log <file>` - Access log file
- `--chroot` - Chroot to web root (requires root)
- `--uid <user>` - Drop privileges to user
- `--gid <group>` - Drop privileges to group
- `--daemon` - Run as daemon
- `--index <filename>` - Default index filename
- `--mimetypes <file>` - Additional MIME types file
- `--forward <host> <url>` - 301 redirect mapping
- `--header <header>` - Custom response header

See the [darkhttpd README](../darkhttpd/README.md) for full documentation.

## Architecture

This crate consists of three layers:

1. **C Library** (`darkhttpd_lib.c`): The original darkhttpd C code with added library API functions
2. **Raw FFI** (`src/ffi.rs`): Low-level `extern "C"` bindings
3. **Safe Wrapper** (`src/lib.rs`): Idiomatic Rust API with automatic cleanup

The C code is compiled statically and linked into your Rust binary.

## Safety

The safe wrapper ensures:

- Proper initialization and cleanup via RAII
- No use-after-free (resources cleaned up in `Drop`)
- Type-safe API with error handling
- Thread-safety considerations (darkhttpd is single-threaded)

## Building

This crate requires:

- Rust 2021 edition or later
- A C compiler (gcc, clang, or MSVC)
- Standard C library headers

Build with:

```bash
cargo build --release
```

## License

This crate is dual-licensed under MIT OR Apache-2.0, matching the dure project.

The darkhttpd C code is licensed under the ISC license (see `../darkhttpd/COPYING`).

## Credits

- Original darkhttpd by Emil Mikulic <emikulic@gmail.com>
- Rust bindings by the dure contributors
