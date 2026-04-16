# Quick Start Guide - darkhttpd-sys

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
darkhttpd-sys = { path = "../darkhttpd-sys" }
```

Or from within the dure workspace:

```toml
[dependencies]
darkhttpd-sys = { path = "../darkhttpd-sys" }
```

## Basic Usage

### Serve a directory on a specific port

```rust
use darkhttpd_sys::DarkHttpd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = DarkHttpd::new();
    server.serve("/var/www/htdocs", 8080)?;
    server.run();
    Ok(())
}
```

### Stop server on Ctrl+C

```rust
use darkhttpd_sys::DarkHttpd;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = DarkHttpd::new();
    server.serve(".", 8080)?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    while running.load(Ordering::SeqCst) && server.poll() {
        // Server is running
    }

    Ok(())
}
```

### Custom configuration

```rust
use darkhttpd_sys::DarkHttpd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = DarkHttpd::new();
    
    server.serve_with_args(
        "/var/www/htdocs",
        &[
            "--port", "8080",
            "--log", "/var/log/darkhttpd.log",
            "--maxconn", "100",
            "--header", "Access-Control-Allow-Origin: *",
        ]
    )?;
    
    server.run();
    Ok(())
}
```

## Common Options

All darkhttpd command-line options are supported via `serve_with_args()`:

| Option | Description | Example |
|--------|-------------|---------|
| `--port` | Port to bind to | `"--port", "8080"` |
| `--addr` | IP address to bind to | `"--addr", "127.0.0.1"` |
| `--log` | Access log file | `"--log", "access.log"` |
| `--maxconn` | Max simultaneous connections | `"--maxconn", "100"` |
| `--daemon` | Run as daemon | `"--daemon"` |
| `--chroot` | Chroot to web root | `"--chroot"` |
| `--uid` | Drop privileges to user | `"--uid", "www"` |
| `--gid` | Drop privileges to group | `"--gid", "www"` |
| `--index` | Default index file | `"--index", "default.htm"` |
| `--header` | Custom response header | `"--header", "X-Custom: value"` |

## Error Handling

The crate uses a custom `DarkHttpdError` type:

```rust
use darkhttpd_sys::{DarkHttpd, DarkHttpdError};

fn start_server() -> Result<(), DarkHttpdError> {
    let mut server = DarkHttpd::new();
    
    match server.serve("/var/www/htdocs", 8080) {
        Ok(_) => {
            println!("Server started successfully");
            server.run();
            Ok(())
        }
        Err(DarkHttpdError::InitializationFailed(code)) => {
            eprintln!("Failed to initialize: {}", code);
            Err(DarkHttpdError::InitializationFailed(code))
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}
```

## Advanced: Manual Event Loop

For integration with other event loops:

```rust
use darkhttpd_sys::DarkHttpd;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = DarkHttpd::new();
    server.serve(".", 8080)?;
    
    let mut iterations = 0;
    while server.poll() {
        // Do other work here
        iterations += 1;
        
        if iterations % 1000 == 0 {
            println!("Processed {} iterations", iterations);
        }
        
        thread::sleep(Duration::from_millis(1));
    }
    
    Ok(())
}
```

## Important Notes

1. **Global State**: darkhttpd uses global variables, so only one server instance can be initialized per process
2. **Blocking**: The `run()` method blocks until the server is stopped (via signal or `stop()`)
3. **Cleanup**: Resources are automatically cleaned up when `DarkHttpd` is dropped
4. **Thread Safety**: The wrapper is not thread-safe (darkhttpd is single-threaded)

## Examples

See the `examples/` directory for complete working examples:

- `basic.rs` - Simple file server
- `advanced.rs` - Server with custom configuration
- `nonblocking.rs` - Manual event loop with Ctrl+C handling

Run examples with:

```bash
cargo run --example basic /path/to/serve
cargo run --example advanced
cargo run --example nonblocking
```

## Troubleshooting

### Port already in use

If you get an initialization error, the port might be in use:

```rust
match server.serve(".", 8080) {
    Err(DarkHttpdError::InitializationFailed(_)) => {
        eprintln!("Port 8080 is already in use");
    }
    Ok(_) => server.run(),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Permission denied

Ports below 1024 require root privileges:

```bash
# Use a higher port
cargo run --example basic /var/www/htdocs 8080

# Or run with sudo (not recommended)
sudo cargo run --example basic /var/www/htdocs 80
```

### Chroot requires root

The `--chroot` option requires root privileges:

```rust
// This requires root
server.serve_with_args(
    "/var/www/htdocs",
    &["--chroot", "--port", "80"]
)?;
```

## More Information

- See `README.md` for detailed documentation
- See `IMPLEMENTATION.md` for technical details
- See darkhttpd documentation: https://unix4lyfe.org/darkhttpd/
