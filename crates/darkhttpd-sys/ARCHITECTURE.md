# darkhttpd-sys Architecture

## Layer Diagram

```
┌─────────────────────────────────────────────────────────┐
│                   User Application                      │
│                    (Your Rust Code)                     │
└─────────────────────────────────────────────────────────┘
                           │
                           │ Safe Rust API
                           ▼
┌─────────────────────────────────────────────────────────┐
│              DarkHttpd Struct (src/lib.rs)              │
│                                                         │
│  • serve(path, port)                                    │
│  • serve_with_args(path, args)                          │
│  • start() / stop() / is_running()                      │
│  • poll() - manual event loop                           │
│  • run() - blocking event loop                          │
│  • Drop impl - automatic cleanup                        │
└─────────────────────────────────────────────────────────┘
                           │
                           │ Raw FFI calls (unsafe)
                           ▼
┌─────────────────────────────────────────────────────────┐
│              FFI Bindings (src/ffi.rs)                  │
│                                                         │
│  extern "C" {                                           │
│    fn darkhttpd_run(argc, argv) -> c_int               │
│    fn darkhttpd_init(argc, argv) -> c_int              │
│    fn darkhttpd_start()                                 │
│    fn darkhttpd_stop()                                  │
│    fn darkhttpd_is_running() -> c_int                   │
│    fn darkhttpd_poll_once()                             │
│    fn darkhttpd_cleanup()                               │
│  }                                                      │
└─────────────────────────────────────────────────────────┘
                           │
                           │ C ABI
                           ▼
┌─────────────────────────────────────────────────────────┐
│         C Library (darkhttpd_lib.c)                     │
│                                                         │
│  • Modified darkhttpd.c (3200+ lines)                   │
│  • Library API wrappers added                           │
│  • main() renamed to darkhttpd_main_original()          │
│  • Static linking via build.rs                          │
└─────────────────────────────────────────────────────────┘
                           │
                           │ System calls
                           ▼
┌─────────────────────────────────────────────────────────┐
│              Operating System (Linux/BSD/etc)           │
│                                                         │
│  • socket(), bind(), listen(), accept()                 │
│  • sendfile() / read() / write()                        │
│  • signal handlers                                      │
│  • file system operations                               │
└─────────────────────────────────────────────────────────┘
```

## Build Process

```
┌──────────────┐
│ Cargo Build  │
└──────┬───────┘
       │
       ▼
┌──────────────────────────────────────┐
│         build.rs                     │
│                                      │
│  1. Reads darkhttpd_lib.c            │
│  2. Calls cc::Build                  │
│  3. Compiles to libdarkhttpd.a       │
│  4. Links statically to Rust binary  │
└──────┬───────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────┐
│         Rust Compiler                │
│                                      │
│  • Compiles lib.rs and ffi.rs        │
│  • Links with libdarkhttpd.a         │
│  • Produces final binary/library     │
└──────────────────────────────────────┘
```

## Data Flow

### Initialization Flow

```
User Code
   │
   │ DarkHttpd::new()
   ▼
DarkHttpd struct created
(initialized: false, running: false)
   │
   │ server.serve("/path", 8080)
   ▼
Convert Rust strings to CString
   │
   │ ["darkhttpd", "/path", "--port", "8080"]
   ▼
darkhttpd_init(argc, argv)  [FFI call]
   │
   ▼
C: parse_commandline()
C: init_sockin() - bind socket
C: setup signals
   │
   │ Return 0 (success)
   ▼
DarkHttpd struct updated
(initialized: true, running: false)
   │
   │ darkhttpd_start() [FFI call]
   ▼
C: running = 1
   │
   ▼
DarkHttpd struct updated
(initialized: true, running: true)
```

### Request Handling Flow

```
User Code
   │
   │ while server.poll() { ... }
   ▼
darkhttpd_poll_once() [FFI call]
   │
   ▼
C: httpd_poll()
   │
   ├─→ select() / poll() on sockets
   │
   ├─→ accept new connections
   │
   ├─→ recv HTTP requests
   │
   ├─→ parse requests
   │
   ├─→ send responses (files, directory listings)
   │
   └─→ timeout idle connections
   │
   │ Return (one iteration complete)
   ▼
Check darkhttpd_is_running() [FFI call]
   │
   ▼
Return bool to Rust
```

### Cleanup Flow

```
User Code
   │
   │ Drop DarkHttpd or explicit cleanup
   ▼
Drop impl called
   │
   │ server.stop()
   ▼
darkhttpd_stop() [FFI call]
   │
   ▼
C: running = 0
   │
   │ darkhttpd_cleanup() [FFI call]
   ▼
C: close(sockin)
C: fclose(logfile)
C: free all connections
C: free mime mappings
C: free allocated strings
   │
   ▼
All resources released
```

## Memory Management

### Rust Side
- **Stack**: `DarkHttpd` struct (2 bools)
- **Heap**: `CString` conversions (temporary, freed after FFI call)
- **Ownership**: RAII pattern, automatic cleanup via `Drop`

### C Side
- **Global State**: darkhttpd uses global variables for:
  - Connection list
  - MIME type mappings
  - Server configuration
  - Socket file descriptors
- **Heap**: `malloc()`/`free()` for:
  - Connection buffers
  - Path strings
  - Response headers
- **Cleanup**: Manual cleanup in `darkhttpd_cleanup()`

### Safety Boundary
```
 Rust (Safe)        │        C (Unsafe)
                    │
DarkHttpd struct    │    Global variables
RAII / Drop         │    Manual malloc/free
Type safety         │    Pointer arithmetic
Borrow checker      │    Manual lifetime mgmt
                    │
        FFI calls cross this boundary
        (marked unsafe in Rust code)
```

## Thread Safety

**NOT thread-safe** by design:

1. darkhttpd uses global state
2. Single event loop (not multi-threaded)
3. `DarkHttpd` is neither `Send` nor `Sync`

For multi-threaded use, spawn separate processes instead:

```rust
use std::process::Command;

// Spawn separate process for each server
Command::new("darkhttpd")
    .arg("/var/www/htdocs1")
    .arg("--port").arg("8080")
    .spawn()?;

Command::new("darkhttpd")
    .arg("/var/www/htdocs2")
    .arg("--port").arg("8081")
    .spawn()?;
```

## Error Handling

```
Rust Error Types          C Error Handling
──────────────────────────────────────────
DarkHttpdError           err()/errx() → exit
  ├─ StringConversion    warn() → stderr
  ├─ InitFailed          errno → perror
  ├─ AlreadyInit         syslog()
  └─ NotInitialized
         │
         ▼
    thiserror derive
         │
         ▼
    Display + Debug
```

**Limitation**: C errors that call `err()` will exit the process. The FFI wrapper catches return codes where possible, but some errors are fatal in the C code.

## Configuration

```
Rust API                  darkhttpd Arguments
──────────────────────────────────────────────
serve(path, port)  →      path --port <port>

serve_with_args(          Custom arg array
  path,                   passed to C
  &["--log", "x.log"]     parse_commandline()
)
```

All darkhttpd command-line options supported:
- `--port`, `--addr`, `--log`, `--maxconn`
- `--daemon`, `--chroot`, `--uid`, `--gid`
- `--index`, `--mimetypes`, `--forward`
- `--header`, `--accf`, etc.

## Dependencies

```
darkhttpd-sys
  ├─ libc (Rust binding to C stdlib)
  ├─ thiserror (error derive)
  ├─ cc [build] (C compiler wrapper)
  └─ ctrlc [dev] (for examples)
       │
       └─ nix, bitflags (transitive)
```

C code uses standard POSIX:
- `<sys/socket.h>`, `<netinet/in.h>`
- `<unistd.h>`, `<fcntl.h>`
- `<stdio.h>`, `<stdlib.h>`, `<string.h>`

## Performance Characteristics

- **Event Loop**: Single-threaded select()/poll()
- **Zero-copy**: Uses sendfile() on Linux/BSD/Solaris
- **Memory**: Small footprint (connection buffers only)
- **Latency**: Low (simple static file serving)
- **Throughput**: Limited by single thread, but efficient

Best for:
- ✅ Static file serving
- ✅ Low-medium traffic
- ✅ Simple use cases
- ✅ Embedded systems

Not ideal for:
- ❌ High concurrency (use nginx)
- ❌ Dynamic content (use full web framework)
- ❌ Multi-core scaling (single-threaded)
