# Cargo Dependency Tree

**Last Updated:** 2026-04-07

This document contains the complete dependency tree for the dure project.

## Top-Level Dependencies

dure v0.0.1 (/home/wj/work/dure/mobile)
├── anyhow v1.0.102                                                                                  # Flexible error handling
├── asupersync v0.2.9                                                                                # Structured concurrency runtime (replaces tokio)
├── async-tungstenite v0.34.0 (https://github.com/nikescar/async-tungstenite#ddf7cbc1)             # WebSocket client/server
├── asyncapi-rust v0.2.0                                                                             # AsyncAPI spec support
├── base64 v0.22.1                                                                                   # Base64 encoding/decoding
├── bincode v1.3.3                                                                                   # Binary serialization
├── chacha20poly1305 v0.10.1                                                                         # ChaCha20-Poly1305 encryption
├── chrono v0.4.44                                                                                   # Date and time
│   ├── iana-time-zone v0.1.65                                                                       # IANA timezones
│   └── num-traits v0.2.19                                                                           # Numeric traits
│       [build-dependencies]
│       └── autocfg v1.5.0                                                                           # Build config detection
├── crossbeam-queue v0.3.12                                                                          # Lock-free queue
│   └── crossbeam-utils v0.8.21                                                                      # Concurrency utils
├── dashmap v6.1.0                                                                                   # Concurrent HashMap
│   ├── cfg-if v1.0.4                                                                                # Conditional compilation
│   ├── crossbeam-utils v0.8.21                                                                      # Concurrency utils
│   ├── hashbrown v0.14.5                                                                            # Fast HashMap
│   ├── lock_api v0.4.14                                                                             # Lock trait API
│   │   └── scopeguard v1.2.0                                                                        # RAII scope guard
│   ├── once_cell v1.21.4                                                                            # Lazy statics
│   └── parking_lot_core v0.9.12                                                                     # Parking lot core
│       ├── cfg-if v1.0.4                                                                            # Conditional compilation
│       ├── libc v0.2.183                                                                            # C library bindings
│       └── smallvec v1.15.1                                                                         # Small vector optimization
├── directories v5.0.1                                                                               # Platform directories
│   └── dirs-sys v0.4.1                                                                              # Directory system layer
│       ├── libc v0.2.183                                                                            # C library bindings
│       └── option-ext v0.2.0                                                                        # Option extensions
├── dirs v5.0.1                                                                                      # User directories
│   └── dirs-sys v0.4.1 (*)
├── eframe v0.33.3                                                                                   # egui framework
│   ├── ahash v0.8.12                                                                                # Fast hash function
│   │   ├── cfg-if v1.0.4                                                                            # Conditional compilation
│   │   ├── getrandom v0.3.4                                                                         # OS random number
│   │   │   ├── cfg-if v1.0.4                                                                        # Conditional compilation
│   │   │   └── libc v0.2.183                                                                        # C library bindings
│   │   ├── once_cell v1.21.4                                                                        # Lazy statics
│   │   ├── serde v1.0.228 (*)
│   │   └── zerocopy v0.8.48                                                                         # Zero-copy parsing
│   │   [build-dependencies]
│   │   └── version_check v0.9.5                                                                     # Rustc version check
│   ├── document-features v0.2.12 (proc-macro)                                                       # Feature docs
│   │   └── litrs v1.0.0                                                                             # Literal parsing
│   ├── egui v0.33.3                                                                                 # Immediate mode GUI
│   │   ├── accesskit v0.21.1                                                                        # Accessibility API
│   │   │   ├── enumn v0.1.14 (proc-macro)                                                           # Enum from int
│   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   └── syn v2.0.117 (*)
│   │   │   └── serde v1.0.228 (*)
│   │   ├── ahash v0.8.12 (*)
│   │   ├── bitflags v2.11.0                                                                         # Bitflag macros
│   │   │   └── serde_core v1.0.228                                                                  # Serde core traits
│   │   ├── emath v0.33.3                                                                            # egui math
│   │   │   ├── bytemuck v1.25.0                                                                     # Type casting
│   │   │   │   └── bytemuck_derive v1.10.2 (proc-macro)                                             # Bytemuck derives
│   │   │   │       ├── proc-macro2 v1.0.106 (*)
│   │   │   │       ├── quote v1.0.45 (*)
│   │   │   │       └── syn v2.0.117 (*)
│   │   │   └── serde v1.0.228 (*)
│   │   ├── epaint v0.33.3                                                                           # egui painting
│   │   │   ├── ab_glyph v0.2.32                                                                     # Font rasterization
│   │   │   │   ├── ab_glyph_rasterizer v0.1.10                                                      # Glyph rasterizer
│   │   │   │   └── owned_ttf_parser v0.25.1                                                         # Owned TTF parser
│   │   │   │       └── ttf-parser v0.25.1                                                           # TTF parser
│   │   │   │           └── core_maths v0.1.1                                                        # Math functions
│   │   │   │               └── libm v0.2.16                                                         # Math library
│   │   │   ├── ahash v0.8.12 (*)
│   │   │   ├── bytemuck v1.25.0 (*)
│   │   │   ├── ecolor v0.33.3                                                                       # egui colors
│   │   │   │   ├── bytemuck v1.25.0 (*)
│   │   │   │   ├── emath v0.33.3 (*)
│   │   │   │   └── serde v1.0.228 (*)
│   │   │   ├── emath v0.33.3 (*)
│   │   │   ├── epaint_default_fonts v0.33.3                                                         # Default fonts
│   │   │   ├── log v0.4.29                                                                          # Logging facade
│   │   │   ├── nohash-hasher v0.2.0                                                                 # Identity hasher
│   │   │   ├── parking_lot v0.12.5                                                                  # Fast mutex/rwlock
│   │   │   │   ├── lock_api v0.4.14 (*)
│   │   │   │   └── parking_lot_core v0.9.12 (*)
│   │   │   ├── profiling v1.0.17                                                                    # Profiling macros
│   │   │   └── serde v1.0.228 (*)
│   │   ├── log v0.4.29                                                                              # Logging facade
│   │   ├── nohash-hasher v0.2.0                                                                     # Identity hasher
│   │   ├── profiling v1.0.17                                                                        # Profiling macros
│   │   ├── ron v0.11.0                                                                              # Rusty Object Notation
│   │   │   ├── base64 v0.22.1                                                                       # Base64 encoding
│   │   │   ├── bitflags v2.11.0 (*)
│   │   │   ├── serde v1.0.228 (*)
│   │   │   ├── serde_derive v1.0.228 (proc-macro) (*)
│   │   │   └── unicode-ident v1.0.24                                                                # Unicode identifiers
│   │   ├── serde v1.0.228 (*)
│   │   ├── smallvec v1.15.1                                                                         # Small vector optimization
│   │   └── unicode-segmentation v1.13.2                                                             # Unicode segmentation
│   ├── egui-winit v0.33.3                                                                           # egui+winit integration
│   │   ├── accesskit_winit v0.29.2                                                                  # Accessibility winit
│   │   │   ├── accesskit v0.21.1 (*)
│   │   │   ├── accesskit_unix v0.17.2                                                               # Accessibility Unix
│   │   │   │   ├── accesskit v0.21.1 (*)
│   │   │   │   ├── accesskit_atspi_common v0.14.2                                                   # AT-SPI common
│   │   │   │   │   ├── accesskit v0.21.1 (*)
│   │   │   │   │   ├── accesskit_consumer v0.31.0                                                   # Accessibility consumer
│   │   │   │   │   │   ├── accesskit v0.21.1 (*)
│   │   │   │   │   │   └── hashbrown v0.15.5                                                        # Fast HashMap
│   │   │   │   │   │       └── foldhash v0.1.5                                                      # Folding hash
│   │   │   │   │   ├── atspi-common v0.9.0                                                          # AT-SPI protocol
│   │   │   │   │   │   ├── enumflags2 v0.7.12                                                       # Enum bitflags
│   │   │   │   │   │   │   ├── enumflags2_derive v0.7.12 (proc-macro)                               # Enumflags derives
│   │   │   │   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   │   │   │   │   └── syn v2.0.117 (*)
│   │   │   │   │   │   │   └── serde v1.0.228 (*)
│   │   │   │   │   │   ├── serde v1.0.228 (*)
│   │   │   │   │   │   ├── static_assertions v1.1.0                                                 # Compile-time assertions
│   │   │   │   │   │   ├── zbus v5.14.0                                                             # D-Bus protocol
│   │   │   │   │   │   │   ├── async-broadcast v0.7.2                                               # Async broadcast channel
│   │   │   │   │   │   │   │   ├── event-listener v5.4.1                                            # Async event
│   │   │   │   │   │   │   │   │   ├── concurrent-queue v2.5.0                                      # Concurrent queue
│   │   │   │   │   │   │   │   │   │   └── crossbeam-utils v0.8.21                                  # Concurrency utils
│   │   │   │   │   │   │   │   │   ├── parking v2.2.1                                               # Thread parking
│   │   │   │   │   │   │   │   │   └── pin-project-lite v0.2.17                                     # Pin projection
│   │   │   │   │   │   │   │   ├── event-listener-strategy v0.5.4                                   # Event strategy
│   │   │   │   │   │   │   │   │   ├── event-listener v5.4.1 (*)
│   │   │   │   │   │   │   │   │   └── pin-project-lite v0.2.17                                     # Pin projection
│   │   │   │   │   │   │   │   ├── futures-core v0.3.32                                             # Futures core
│   │   │   │   │   │   │   │   └── pin-project-lite v0.2.17                                         # Pin projection
│   │   │   │   │   │   │   ├── async-executor v1.14.0                                               # Async executor
│   │   │   │   │   │   │   │   ├── async-task v4.7.1                                                # Async task
│   │   │   │   │   │   │   │   ├── concurrent-queue v2.5.0 (*)
│   │   │   │   │   │   │   │   ├── fastrand v2.3.0                                                  # Fast RNG
│   │   │   │   │   │   │   │   ├── futures-lite v2.6.1                                              # Lightweight futures
│   │   │   │   │   │   │   │   │   ├── fastrand v2.3.0                                              # Fast RNG
│   │   │   │   │   │   │   │   │   ├── futures-core v0.3.32                                         # Futures core
│   │   │   │   │   │   │   │   │   ├── futures-io v0.3.32                                           # Futures I/O
│   │   │   │   │   │   │   │   │   ├── parking v2.2.1                                               # Thread parking
│   │   │   │   │   │   │   │   │   └── pin-project-lite v0.2.17                                     # Pin projection
│   │   │   │   │   │   │   │   ├── pin-project-lite v0.2.17                                         # Pin projection
│   │   │   │   │   │   │   │   └── slab v0.4.12                                                     # Slab allocator
│   │   │   │   │   │   │   ├── async-io v2.6.0                                                      # Async I/O
│   │   │   │   │   │   │   │   ├── cfg-if v1.0.4                                                    # Conditional compilation
│   │   │   │   │   │   │   │   ├── concurrent-queue v2.5.0 (*)
│   │   │   │   │   │   │   │   ├── futures-io v0.3.32                                               # Futures I/O
│   │   │   │   │   │   │   │   ├── futures-lite v2.6.1 (*)
│   │   │   │   │   │   │   │   ├── parking v2.2.1                                                   # Thread parking
│   │   │   │   │   │   │   │   ├── polling v3.11.0                                                  # Portable polling
│   │   │   │   │   │   │   │   │   ├── cfg-if v1.0.4                                                # Conditional compilation
│   │   │   │   │   │   │   │   │   └── rustix v1.1.4                                                # Safe POSIX bindings
│   │   │   │   │   │   │   │   │       ├── bitflags v2.11.0 (*)
│   │   │   │   │   │   │   │   │       └── linux-raw-sys v0.12.1                                    # Linux raw syscalls
│   │   │   │   │   │   │   │   ├── rustix v1.1.4 (*)
│   │   │   │   │   │   │   │   └── slab v0.4.12                                                     # Slab allocator
│   │   │   │   │   │   │   │   [build-dependencies]
│   │   │   │   │   │   │   │   └── autocfg v1.5.0                                                   # Build config detection
│   │   │   │   │   │   │   ├── async-lock v3.4.2                                                    # Async locks
│   │   │   │   │   │   │   │   ├── event-listener v5.4.1 (*)
│   │   │   │   │   │   │   │   ├── event-listener-strategy v0.5.4 (*)
│   │   │   │   │   │   │   │   └── pin-project-lite v0.2.17                                         # Pin projection
│   │   │   │   │   │   │   ├── async-process v2.5.0                                                 # Async process
│   │   │   │   │   │   │   │   ├── async-channel v2.5.0                                             # Async channel
│   │   │   │   │   │   │   │   │   ├── concurrent-queue v2.5.0 (*)
│   │   │   │   │   │   │   │   │   ├── event-listener-strategy v0.5.4 (*)
│   │   │   │   │   │   │   │   │   ├── futures-core v0.3.32                                         # Futures core
│   │   │   │   │   │   │   │   │   └── pin-project-lite v0.2.17                                     # Pin projection
│   │   │   │   │   │   │   │   ├── async-io v2.6.0 (*)
│   │   │   │   │   │   │   │   ├── async-lock v3.4.2 (*)
│   │   │   │   │   │   │   │   ├── async-signal v0.2.13                                             # Async signals
│   │   │   │   │   │   │   │   │   ├── async-io v2.6.0 (*)
│   │   │   │   │   │   │   │   │   ├── cfg-if v1.0.4                                                # Conditional compilation
│   │   │   │   │   │   │   │   │   ├── futures-core v0.3.32                                         # Futures core
│   │   │   │   │   │   │   │   │   ├── futures-io v0.3.32                                           # Futures I/O
│   │   │   │   │   │   │   │   │   ├── rustix v1.1.4 (*)
│   │   │   │   │   │   │   │   │   └── signal-hook-registry v1.4.8                                  # Signal hooks
│   │   │   │   │   │   │   │   │       ├── errno v0.3.14                                            # System errno
│   │   │   │   │   │   │   │   │       │   └── libc v0.2.183                                        # C library bindings
│   │   │   │   │   │   │   │   │       └── libc v0.2.183                                            # C library bindings
│   │   │   │   │   │   │   │   ├── async-task v4.7.1                                                # Async task
│   │   │   │   │   │   │   │   ├── cfg-if v1.0.4                                                    # Conditional compilation
│   │   │   │   │   │   │   │   ├── event-listener v5.4.1 (*)
│   │   │   │   │   │   │   │   ├── futures-lite v2.6.1 (*)
│   │   │   │   │   │   │   │   └── rustix v1.1.4 (*)
│   │   │   │   │   │   │   ├── async-recursion v1.1.1 (proc-macro)                                  # Async recursion
│   │   │   │   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   │   │   │   │   └── syn v2.0.117 (*)
│   │   │   │   │   │   │   ├── async-task v4.7.1                                                    # Async task
│   │   │   │   │   │   │   ├── async-trait v0.1.89 (proc-macro)                                     # Async traits
│   │   │   │   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   │   │   │   │   └── syn v2.0.117 (*)
│   │   │   │   │   │   │   ├── blocking v1.6.2                                                      # Blocking thread pool
│   │   │   │   │   │   │   │   ├── async-channel v2.5.0 (*)
│   │   │   │   │   │   │   │   ├── async-task v4.7.1                                                # Async task
│   │   │   │   │   │   │   │   ├── futures-io v0.3.32                                               # Futures I/O
│   │   │   │   │   │   │   │   ├── futures-lite v2.6.1 (*)
│   │   │   │   │   │   │   │   └── piper v0.2.5
│   │   │   │   │   │   │   │       ├── atomic-waker v1.1.2
│   │   │   │   │   │   │   │       ├── fastrand v2.3.0                                              # Fast RNG
│   │   │   │   │   │   │   │       └── futures-io v0.3.32                                           # Futures I/O
│   │   │   │   │   │   │   ├── enumflags2 v0.7.12 (*)
│   │   │   │   │   │   │   ├── event-listener v5.4.1 (*)
│   │   │   │   │   │   │   ├── futures-core v0.3.32                                                 # Futures core
│   │   │   │   │   │   │   ├── futures-lite v2.6.1 (*)
│   │   │   │   │   │   │   ├── hex v0.4.3
│   │   │   │   │   │   │   ├── libc v0.2.183                                                        # C library bindings
│   │   │   │   │   │   │   ├── ordered-stream v0.2.0                                                # Ordered async stream
│   │   │   │   │   │   │   │   ├── futures-core v0.3.32                                             # Futures core
│   │   │   │   │   │   │   │   └── pin-project-lite v0.2.17                                         # Pin projection
│   │   │   │   │   │   │   ├── rustix v1.1.4 (*)
│   │   │   │   │   │   │   ├── serde v1.0.228 (*)
│   │   │   │   │   │   │   ├── serde_repr v0.1.20 (proc-macro)
│   │   │   │   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   │   │   │   │   └── syn v2.0.117 (*)
│   │   │   │   │   │   │   ├── tracing v0.1.44                                                      # Application tracing
│   │   │   │   │   │   │   │   ├── log v0.4.29                                                      # Logging facade
│   │   │   │   │   │   │   │   ├── pin-project-lite v0.2.17                                         # Pin projection
│   │   │   │   │   │   │   │   ├── tracing-attributes v0.1.31 (proc-macro)
│   │   │   │   │   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   │   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   │   │   │   │   │   └── syn v2.0.117 (*)
│   │   │   │   │   │   │   │   └── tracing-core v0.1.36                                             # Tracing core
│   │   │   │   │   │   │   │       └── once_cell v1.21.4                                            # Lazy statics
│   │   │   │   │   │   │   ├── uuid v1.23.0
│   │   │   │   │   │   │   │   └── serde_core v1.0.228                                              # Serde core traits
│   │   │   │   │   │   │   ├── winnow v0.7.15
│   │   │   │   │   │   │   ├── zbus_macros v5.14.0 (proc-macro)
│   │   │   │   │   │   │   │   ├── proc-macro-crate v3.5.0
│   │   │   │   │   │   │   │   │   └── toml_edit v0.25.9+spec-1.1.0
│   │   │   │   │   │   │   │   │       ├── indexmap v2.13.0
│   │   │   │   │   │   │   │   │       │   ├── equivalent v1.0.2
│   │   │   │   │   │   │   │   │       │   └── hashbrown v0.16.1                                    # Fast HashMap
│   │   │   │   │   │   │   │   │       ├── toml_datetime v1.1.1+spec-1.1.0
│   │   │   │   │   │   │   │   │       ├── toml_parser v1.1.1+spec-1.1.0
│   │   │   │   │   │   │   │   │       │   └── winnow v1.0.1
│   │   │   │   │   │   │   │   │       └── winnow v1.0.1
│   │   │   │   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   │   │   │   │   ├── syn v2.0.117 (*)
│   │   │   │   │   │   │   │   ├── zbus_names v4.3.1
│   │   │   │   │   │   │   │   │   ├── serde v1.0.228                                               # Serialization framework
│   │   │   │   │   │   │   │   │   │   ├── serde_core v1.0.228                                      # Serde core traits
│   │   │   │   │   │   │   │   │   │   └── serde_derive v1.0.228 (proc-macro) (*)
│   │   │   │   │   │   │   │   │   ├── winnow v0.7.15
│   │   │   │   │   │   │   │   │   └── zvariant v5.10.0                                             # D-Bus variant type
│   │   │   │   │   │   │   │   │       ├── endi v1.1.1
│   │   │   │   │   │   │   │   │       ├── enumflags2 v0.7.12 (*)
│   │   │   │   │   │   │   │   │       ├── serde v1.0.228 (*)
│   │   │   │   │   │   │   │   │       ├── winnow v0.7.15
│   │   │   │   │   │   │   │   │       ├── zvariant_derive v5.10.0 (proc-macro)                     # Zvariant derives
│   │   │   │   │   │   │   │   │       │   ├── proc-macro-crate v3.5.0 (*)
│   │   │   │   │   │   │   │   │       │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   │   │   │   │       │   ├── quote v1.0.45 (*)
│   │   │   │   │   │   │   │   │       │   ├── syn v2.0.117 (*)
│   │   │   │   │   │   │   │   │       │   └── zvariant_utils v3.3.0                                # Zvariant utilities
│   │   │   │   │   │   │   │   │       │       ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   │   │   │   │       │       ├── quote v1.0.45 (*)
│   │   │   │   │   │   │   │   │       │       ├── serde v1.0.228 (*)
│   │   │   │   │   │   │   │   │       │       ├── syn v2.0.117 (*)
│   │   │   │   │   │   │   │   │       │       └── winnow v0.7.15
│   │   │   │   │   │   │   │   │       └── zvariant_utils v3.3.0 (*)
│   │   │   │   │   │   │   │   ├── zvariant v5.10.0 (*)
│   │   │   │   │   │   │   │   └── zvariant_utils v3.3.0 (*)
│   │   │   │   │   │   │   ├── zbus_names v4.3.1 (*)
│   │   │   │   │   │   │   └── zvariant v5.10.0 (*)
│   │   │   │   │   │   ├── zbus-lockstep v0.5.2
│   │   │   │   │   │   │   ├── zbus_xml v5.1.0
│   │   │   │   │   │   │   │   ├── quick-xml v0.38.4
│   │   │   │   │   │   │   │   │   ├── memchr v2.8.0                                                # Memory search
│   │   │   │   │   │   │   │   │   └── serde v1.0.228 (*)
│   │   │   │   │   │   │   │   ├── serde v1.0.228 (*)
│   │   │   │   │   │   │   │   ├── zbus_names v4.3.1 (*)
│   │   │   │   │   │   │   │   └── zvariant v5.10.0 (*)
│   │   │   │   │   │   │   └── zvariant v5.10.0 (*)
│   │   │   │   │   │   ├── zbus-lockstep-macros v0.5.2 (proc-macro)
│   │   │   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   │   │   │   ├── syn v2.0.117 (*)
│   │   │   │   │   │   │   ├── zbus-lockstep v0.5.2 (*)
│   │   │   │   │   │   │   ├── zbus_xml v5.1.0 (*)
│   │   │   │   │   │   │   └── zvariant v5.10.0 (*)
│   │   │   │   │   │   ├── zbus_names v4.3.1 (*)
│   │   │   │   │   │   └── zvariant v5.10.0 (*)
│   │   │   │   │   ├── serde v1.0.228 (*)
│   │   │   │   │   ├── thiserror v1.0.69
│   │   │   │   │   │   └── thiserror-impl v1.0.69 (proc-macro)
│   │   │   │   │   │       ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   │       ├── quote v1.0.45 (*)
│   │   │   │   │   │       └── syn v2.0.117 (*)
│   │   │   │   │   └── zvariant v5.10.0 (*)
│   │   │   │   ├── async-channel v2.5.0 (*)
│   │   │   │   ├── async-executor v1.14.0 (*)
│   │   │   │   ├── async-task v4.7.1                                                                # Async task
│   │   │   │   ├── atspi v0.25.0
│   │   │   │   │   ├── atspi-common v0.9.0 (*)
│   │   │   │   │   ├── atspi-connection v0.9.0
│   │   │   │   │   │   ├── atspi-common v0.9.0 (*)
│   │   │   │   │   │   ├── atspi-proxies v0.9.0
│   │   │   │   │   │   │   ├── atspi-common v0.9.0 (*)
│   │   │   │   │   │   │   ├── serde v1.0.228 (*)
│   │   │   │   │   │   │   └── zbus v5.14.0 (*)
│   │   │   │   │   │   ├── futures-lite v2.6.1 (*)
│   │   │   │   │   │   └── zbus v5.14.0 (*)
│   │   │   │   │   └── atspi-proxies v0.9.0 (*)
│   │   │   │   ├── futures-lite v2.6.1 (*)
│   │   │   │   ├── futures-util v0.3.32
│   │   │   │   │   ├── futures-core v0.3.32                                                         # Futures core
│   │   │   │   │   ├── futures-io v0.3.32                                                           # Futures I/O
│   │   │   │   │   ├── futures-macro v0.3.32 (proc-macro)
│   │   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   │   │   └── syn v2.0.117 (*)
│   │   │   │   │   ├── futures-sink v0.3.32
│   │   │   │   │   ├── futures-task v0.3.32
│   │   │   │   │   ├── memchr v2.8.0                                                                # Memory search
│   │   │   │   │   ├── pin-project-lite v0.2.17                                                     # Pin projection
│   │   │   │   │   └── slab v0.4.12                                                                 # Slab allocator
│   │   │   │   ├── serde v1.0.228 (*)
│   │   │   │   └── zbus v5.14.0 (*)
│   │   │   ├── raw-window-handle v0.6.2
│   │   │   └── winit v0.30.13                                                                       # Window creation
│   │   │       ├── ahash v0.8.12 (*)
│   │   │       ├── bitflags v2.11.0 (*)
│   │   │       ├── bytemuck v1.25.0 (*)
│   │   │       ├── calloop v0.13.0
│   │   │       │   ├── bitflags v2.11.0 (*)
│   │   │       │   ├── log v0.4.29                                                                  # Logging facade
│   │   │       │   ├── polling v3.11.0 (*)
│   │   │       │   ├── rustix v0.38.44                                                              # Safe POSIX bindings
│   │   │       │   │   ├── bitflags v2.11.0 (*)
│   │   │       │   │   └── linux-raw-sys v0.4.15                                                    # Linux raw syscalls
│   │   │       │   ├── slab v0.4.12                                                                 # Slab allocator
│   │   │       │   └── thiserror v1.0.69 (*)
│   │   │       ├── cursor-icon v1.2.0
│   │   │       ├── dpi v0.1.2
│   │   │       ├── libc v0.2.183                                                                    # C library bindings
│   │   │       ├── memmap2 v0.9.10                                                                  # Memory-mapped I/O
│   │   │       │   └── libc v0.2.183                                                                # C library bindings
│   │   │       ├── percent-encoding v2.3.2
│   │   │       ├── raw-window-handle v0.6.2
│   │   │       ├── rustix v0.38.44 (*)
│   │   │       ├── sctk-adwaita v0.10.1
│   │   │       │   ├── ab_glyph v0.2.32 (*)
│   │   │       │   ├── log v0.4.29                                                                  # Logging facade
│   │   │       │   ├── memmap2 v0.9.10 (*)
│   │   │       │   ├── smithay-client-toolkit v0.19.2
│   │   │       │   │   ├── bitflags v2.11.0 (*)
│   │   │       │   │   ├── calloop v0.13.0 (*)
│   │   │       │   │   ├── calloop-wayland-source v0.3.0
│   │   │       │   │   │   ├── calloop v0.13.0 (*)
│   │   │       │   │   │   ├── rustix v0.38.44 (*)
│   │   │       │   │   │   ├── wayland-backend v0.3.15
│   │   │       │   │   │   │   ├── downcast-rs v1.2.1
│   │   │       │   │   │   │   ├── rustix v1.1.4 (*)
│   │   │       │   │   │   │   ├── scoped-tls v1.0.1
│   │   │       │   │   │   │   ├── smallvec v1.15.1                                                 # Small vector optimization
│   │   │       │   │   │   │   └── wayland-sys v0.31.11
│   │   │       │   │   │   │       ├── dlib v0.5.3
│   │   │       │   │   │   │       │   └── libloading v0.8.9                                        # Library loading
│   │   │       │   │   │   │       │       └── cfg-if v1.0.4                                        # Conditional compilation
│   │   │       │   │   │   │       ├── log v0.4.29                                                  # Logging facade
│   │   │       │   │   │   │       └── once_cell v1.21.4                                            # Lazy statics
│   │   │       │   │   │   │       [build-dependencies]
│   │   │       │   │   │   │       └── pkg-config v0.3.32                                           # Package config
│   │   │       │   │   │   │   [build-dependencies]
│   │   │       │   │   │   │   └── cc v1.2.58
│   │   │       │   │   │   │       ├── find-msvc-tools v0.1.9
│   │   │       │   │   │   │       └── shlex v1.3.0
│   │   │       │   │   │   └── wayland-client v0.31.14                                              # Wayland client
│   │   │       │   │   │       ├── bitflags v2.11.0 (*)
│   │   │       │   │   │       ├── rustix v1.1.4 (*)
│   │   │       │   │   │       ├── wayland-backend v0.3.15 (*)
│   │   │       │   │   │       └── wayland-scanner v0.31.10 (proc-macro)
│   │   │       │   │   │           ├── proc-macro2 v1.0.106 (*)
│   │   │       │   │   │           ├── quick-xml v0.39.2
│   │   │       │   │   │           │   └── memchr v2.8.0                                            # Memory search
│   │   │       │   │   │           └── quote v1.0.45 (*)
│   │   │       │   │   ├── cursor-icon v1.2.0
│   │   │       │   │   ├── libc v0.2.183                                                            # C library bindings
│   │   │       │   │   ├── log v0.4.29                                                              # Logging facade
│   │   │       │   │   ├── memmap2 v0.9.10 (*)
│   │   │       │   │   ├── rustix v0.38.44 (*)
│   │   │       │   │   ├── thiserror v1.0.69 (*)
│   │   │       │   │   ├── wayland-backend v0.3.15 (*)
│   │   │       │   │   ├── wayland-client v0.31.14 (*)
│   │   │       │   │   ├── wayland-csd-frame v0.3.0
│   │   │       │   │   │   ├── bitflags v2.11.0 (*)
│   │   │       │   │   │   ├── cursor-icon v1.2.0
│   │   │       │   │   │   └── wayland-backend v0.3.15 (*)
│   │   │       │   │   ├── wayland-cursor v0.31.14
│   │   │       │   │   │   ├── rustix v1.1.4 (*)
│   │   │       │   │   │   ├── wayland-client v0.31.14 (*)
│   │   │       │   │   │   └── xcursor v0.3.10
│   │   │       │   │   ├── wayland-protocols v0.32.12
│   │   │       │   │   │   ├── bitflags v2.11.0 (*)
│   │   │       │   │   │   ├── wayland-backend v0.3.15 (*)
│   │   │       │   │   │   ├── wayland-client v0.31.14 (*)
│   │   │       │   │   │   └── wayland-scanner v0.31.10 (proc-macro) (*)
│   │   │       │   │   ├── wayland-protocols-wlr v0.3.12
│   │   │       │   │   │   ├── bitflags v2.11.0 (*)
│   │   │       │   │   │   ├── wayland-backend v0.3.15 (*)
│   │   │       │   │   │   ├── wayland-client v0.31.14 (*)
│   │   │       │   │   │   ├── wayland-protocols v0.32.12 (*)
│   │   │       │   │   │   └── wayland-scanner v0.31.10 (proc-macro) (*)
│   │   │       │   │   ├── wayland-scanner v0.31.10 (proc-macro) (*)
│   │   │       │   │   └── xkeysym v0.2.1
│   │   │       │   └── tiny-skia v0.11.4
│   │   │       │       ├── arrayref v0.3.9
│   │   │       │       ├── arrayvec v0.7.6
│   │   │       │       ├── bytemuck v1.25.0 (*)
│   │   │       │       ├── cfg-if v1.0.4                                                            # Conditional compilation
│   │   │       │       ├── log v0.4.29                                                              # Logging facade
│   │   │       │       ├── png v0.17.16                                                             # PNG format
│   │   │       │       │   ├── bitflags v1.3.2                                                      # Bitflag macros
│   │   │       │       │   ├── crc32fast v1.5.0                                                     # Fast CRC32
│   │   │       │       │   │   └── cfg-if v1.0.4                                                    # Conditional compilation
│   │   │       │       │   ├── fdeflate v0.3.7
│   │   │       │       │   │   └── simd-adler32 v0.3.9
│   │   │       │       │   ├── flate2 v1.1.9                                                        # DEFLATE compression
│   │   │       │       │   │   ├── crc32fast v1.5.0 (*)
│   │   │       │       │   │   └── miniz_oxide v0.8.9                                               # DEFLATE/inflate
│   │   │       │       │   │       ├── adler2 v2.0.1
│   │   │       │       │   │       └── simd-adler32 v0.3.9
│   │   │       │       │   └── miniz_oxide v0.8.9 (*)
│   │   │       │       └── tiny-skia-path v0.11.4
│   │   │       │           ├── arrayref v0.3.9
│   │   │       │           ├── bytemuck v1.25.0 (*)
│   │   │       │           └── strict-num v0.1.1
│   │   │       │               └── float-cmp v0.9.0
│   │   │       ├── smithay-client-toolkit v0.19.2 (*)
│   │   │       ├── smol_str v0.2.2
│   │   │       ├── tracing v0.1.44 (*)
│   │   │       ├── wayland-backend v0.3.15 (*)
│   │   │       ├── wayland-client v0.31.14 (*)
│   │   │       ├── wayland-protocols v0.32.12 (*)
│   │   │       ├── wayland-protocols-plasma v0.3.12
│   │   │       │   ├── bitflags v2.11.0 (*)
│   │   │       │   ├── wayland-backend v0.3.15 (*)
│   │   │       │   ├── wayland-client v0.31.14 (*)
│   │   │       │   ├── wayland-protocols v0.32.12 (*)
│   │   │       │   └── wayland-scanner v0.31.10 (proc-macro) (*)
│   │   │       ├── x11-dl v2.21.0                                                                   # X11 library
│   │   │       │   ├── libc v0.2.183                                                                # C library bindings
│   │   │       │   └── once_cell v1.21.4                                                            # Lazy statics
│   │   │       │   [build-dependencies]
│   │   │       │   └── pkg-config v0.3.32                                                           # Package config
│   │   │       ├── x11rb v0.13.2                                                                    # X11 protocol
│   │   │       │   ├── as-raw-xcb-connection v1.0.1
│   │   │       │   ├── gethostname v1.1.0
│   │   │       │   │   └── rustix v1.1.4 (*)
│   │   │       │   ├── libc v0.2.183                                                                # C library bindings
│   │   │       │   ├── libloading v0.8.9 (*)
│   │   │       │   ├── once_cell v1.21.4                                                            # Lazy statics
│   │   │       │   ├── rustix v1.1.4 (*)
│   │   │       │   └── x11rb-protocol v0.13.2
│   │   │       └── xkbcommon-dl v0.4.2
│   │   │           ├── bitflags v2.11.0 (*)
│   │   │           ├── dlib v0.5.3 (*)
│   │   │           ├── log v0.4.29                                                                  # Logging facade
│   │   │           ├── once_cell v1.21.4                                                            # Lazy statics
│   │   │           └── xkeysym v0.2.1
│   │   │       [build-dependencies]
│   │   │       └── cfg_aliases v0.2.1
│   │   ├── arboard v3.6.1
│   │   │   ├── image v0.25.10                                                                       # Image encoding/decoding
│   │   │   │   ├── bytemuck v1.25.0 (*)
│   │   │   │   ├── byteorder-lite v0.1.0
│   │   │   │   ├── color_quant v1.1.0
│   │   │   │   ├── gif v0.14.1
│   │   │   │   │   ├── color_quant v1.1.0
│   │   │   │   │   └── weezl v0.1.12
│   │   │   │   ├── image-webp v0.2.4
│   │   │   │   │   ├── byteorder-lite v0.1.0
│   │   │   │   │   └── quick-error v2.0.1
│   │   │   │   ├── moxcms v0.8.1
│   │   │   │   │   ├── num-traits v0.2.19 (*)
│   │   │   │   │   └── pxfm v0.1.28
│   │   │   │   ├── num-traits v0.2.19 (*)
│   │   │   │   ├── png v0.18.1                                                                      # PNG format
│   │   │   │   │   ├── bitflags v2.11.0 (*)
│   │   │   │   │   ├── crc32fast v1.5.0 (*)
│   │   │   │   │   ├── fdeflate v0.3.7 (*)
│   │   │   │   │   ├── flate2 v1.1.9 (*)
│   │   │   │   │   └── miniz_oxide v0.8.9 (*)
│   │   │   │   ├── zune-core v0.5.1
│   │   │   │   └── zune-jpeg v0.5.15
│   │   │   │       └── zune-core v0.5.1
│   │   │   ├── log v0.4.29                                                                          # Logging facade
│   │   │   ├── parking_lot v0.12.5 (*)
│   │   │   ├── percent-encoding v2.3.2
│   │   │   └── x11rb v0.13.2 (*)
│   │   ├── bytemuck v1.25.0 (*)
│   │   ├── egui v0.33.3 (*)
│   │   ├── log v0.4.29                                                                              # Logging facade
│   │   ├── profiling v1.0.17                                                                        # Profiling macros
│   │   ├── raw-window-handle v0.6.2
│   │   ├── serde v1.0.228 (*)
│   │   ├── smithay-clipboard v0.7.3
│   │   │   ├── libc v0.2.183                                                                        # C library bindings
│   │   │   ├── smithay-client-toolkit v0.20.0
│   │   │   │   ├── bitflags v2.11.0 (*)
│   │   │   │   ├── calloop v0.14.4
│   │   │   │   │   ├── bitflags v2.11.0 (*)
│   │   │   │   │   ├── polling v3.11.0 (*)
│   │   │   │   │   ├── rustix v1.1.4 (*)
│   │   │   │   │   ├── slab v0.4.12                                                                 # Slab allocator
│   │   │   │   │   └── tracing v0.1.44 (*)
│   │   │   │   ├── calloop-wayland-source v0.4.1
│   │   │   │   │   ├── calloop v0.14.4 (*)
│   │   │   │   │   ├── rustix v1.1.4 (*)
│   │   │   │   │   ├── wayland-backend v0.3.15 (*)
│   │   │   │   │   └── wayland-client v0.31.14 (*)
│   │   │   │   ├── cursor-icon v1.2.0
│   │   │   │   ├── libc v0.2.183                                                                    # C library bindings
│   │   │   │   ├── log v0.4.29                                                                      # Logging facade
│   │   │   │   ├── memmap2 v0.9.10 (*)
│   │   │   │   ├── rustix v1.1.4 (*)
│   │   │   │   ├── thiserror v2.0.18
│   │   │   │   │   └── thiserror-impl v2.0.18 (proc-macro)
│   │   │   │   │       ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │       ├── quote v1.0.45 (*)
│   │   │   │   │       └── syn v2.0.117 (*)
│   │   │   │   ├── wayland-backend v0.3.15 (*)
│   │   │   │   ├── wayland-client v0.31.14 (*)
│   │   │   │   ├── wayland-csd-frame v0.3.0 (*)
│   │   │   │   ├── wayland-cursor v0.31.14 (*)
│   │   │   │   ├── wayland-protocols v0.32.12 (*)
│   │   │   │   ├── wayland-protocols-experimental v20250721.0.1
│   │   │   │   │   ├── bitflags v2.11.0 (*)
│   │   │   │   │   ├── wayland-backend v0.3.15 (*)
│   │   │   │   │   ├── wayland-client v0.31.14 (*)
│   │   │   │   │   ├── wayland-protocols v0.32.12 (*)
│   │   │   │   │   └── wayland-scanner v0.31.10 (proc-macro) (*)
│   │   │   │   ├── wayland-protocols-misc v0.3.12
│   │   │   │   │   ├── bitflags v2.11.0 (*)
│   │   │   │   │   ├── wayland-backend v0.3.15 (*)
│   │   │   │   │   ├── wayland-client v0.31.14 (*)
│   │   │   │   │   ├── wayland-protocols v0.32.12 (*)
│   │   │   │   │   └── wayland-scanner v0.31.10 (proc-macro) (*)
│   │   │   │   ├── wayland-protocols-wlr v0.3.12 (*)
│   │   │   │   ├── wayland-scanner v0.31.10 (proc-macro) (*)
│   │   │   │   └── xkeysym v0.2.1
│   │   │   └── wayland-backend v0.3.15 (*)
│   │   ├── web-time v1.1.0
│   │   ├── webbrowser v1.2.0                                                                        # Open browser
│   │   │   ├── log v0.4.29                                                                          # Logging facade
│   │   │   └── url v2.5.8
│   │   │       ├── form_urlencoded v1.2.2
│   │   │       │   └── percent-encoding v2.3.2
│   │   │       ├── idna v1.1.0
│   │   │       │   ├── idna_adapter v1.2.1
│   │   │       │   │   ├── icu_normalizer v2.1.1
│   │   │       │   │   │   ├── icu_collections v2.1.1
│   │   │       │   │   │   │   ├── displaydoc v0.2.5 (proc-macro)
│   │   │       │   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │       │   │   │   │   │   ├── quote v1.0.45 (*)
│   │   │       │   │   │   │   │   └── syn v2.0.117 (*)
│   │   │       │   │   │   │   ├── potential_utf v0.1.4
│   │   │       │   │   │   │   │   └── zerovec v0.11.5
│   │   │       │   │   │   │   │       ├── yoke v0.8.1
│   │   │       │   │   │   │   │       │   ├── stable_deref_trait v1.2.1
│   │   │       │   │   │   │   │       │   ├── yoke-derive v0.8.1 (proc-macro)
│   │   │       │   │   │   │   │       │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │       │   │   │   │   │       │   │   ├── quote v1.0.45 (*)
│   │   │       │   │   │   │   │       │   │   ├── syn v2.0.117 (*)
│   │   │       │   │   │   │   │       │   │   └── synstructure v0.13.2
│   │   │       │   │   │   │   │       │   │       ├── proc-macro2 v1.0.106 (*)
│   │   │       │   │   │   │   │       │   │       ├── quote v1.0.45 (*)
│   │   │       │   │   │   │   │       │   │       └── syn v2.0.117 (*)
│   │   │       │   │   │   │   │       │   └── zerofrom v0.1.6
│   │   │       │   │   │   │   │       │       └── zerofrom-derive v0.1.6 (proc-macro)
│   │   │       │   │   │   │   │       │           ├── proc-macro2 v1.0.106 (*)
│   │   │       │   │   │   │   │       │           ├── quote v1.0.45 (*)
│   │   │       │   │   │   │   │       │           ├── syn v2.0.117 (*)
│   │   │       │   │   │   │   │       │           └── synstructure v0.13.2 (*)
│   │   │       │   │   │   │   │       ├── zerofrom v0.1.6 (*)
│   │   │       │   │   │   │   │       └── zerovec-derive v0.11.2 (proc-macro)
│   │   │       │   │   │   │   │           ├── proc-macro2 v1.0.106 (*)
│   │   │       │   │   │   │   │           ├── quote v1.0.45 (*)
│   │   │       │   │   │   │   │           └── syn v2.0.117 (*)
│   │   │       │   │   │   │   ├── yoke v0.8.1 (*)
│   │   │       │   │   │   │   ├── zerofrom v0.1.6 (*)
│   │   │       │   │   │   │   └── zerovec v0.11.5 (*)
│   │   │       │   │   │   ├── icu_normalizer_data v2.1.1
│   │   │       │   │   │   ├── icu_provider v2.1.1
│   │   │       │   │   │   │   ├── displaydoc v0.2.5 (proc-macro) (*)
│   │   │       │   │   │   │   ├── icu_locale_core v2.1.1
│   │   │       │   │   │   │   │   ├── displaydoc v0.2.5 (proc-macro) (*)
│   │   │       │   │   │   │   │   ├── litemap v0.8.1
│   │   │       │   │   │   │   │   ├── tinystr v0.8.2
│   │   │       │   │   │   │   │   │   ├── displaydoc v0.2.5 (proc-macro) (*)
│   │   │       │   │   │   │   │   │   └── zerovec v0.11.5 (*)
│   │   │       │   │   │   │   │   ├── writeable v0.6.2
│   │   │       │   │   │   │   │   └── zerovec v0.11.5 (*)
│   │   │       │   │   │   │   ├── writeable v0.6.2
│   │   │       │   │   │   │   ├── yoke v0.8.1 (*)
│   │   │       │   │   │   │   ├── zerofrom v0.1.6 (*)
│   │   │       │   │   │   │   ├── zerotrie v0.2.3
│   │   │       │   │   │   │   │   ├── displaydoc v0.2.5 (proc-macro) (*)
│   │   │       │   │   │   │   │   ├── yoke v0.8.1 (*)
│   │   │       │   │   │   │   │   └── zerofrom v0.1.6 (*)
│   │   │       │   │   │   │   └── zerovec v0.11.5 (*)
│   │   │       │   │   │   ├── smallvec v1.15.1                                                     # Small vector optimization
│   │   │       │   │   │   └── zerovec v0.11.5 (*)
│   │   │       │   │   └── icu_properties v2.1.2
│   │   │       │   │       ├── icu_collections v2.1.1 (*)
│   │   │       │   │       ├── icu_locale_core v2.1.1 (*)
│   │   │       │   │       ├── icu_properties_data v2.1.2
│   │   │       │   │       ├── icu_provider v2.1.1 (*)
│   │   │       │   │       ├── zerotrie v0.2.3 (*)
│   │   │       │   │       └── zerovec v0.11.5 (*)
│   │   │       │   ├── smallvec v1.15.1                                                             # Small vector optimization
│   │   │       │   └── utf8_iter v1.0.4
│   │   │       └── percent-encoding v2.3.2
│   │   └── winit v0.30.13 (*)
│   ├── egui_glow v0.33.3                                                                            # OpenGL bindings
│   │   ├── bytemuck v1.25.0 (*)
│   │   ├── egui v0.33.3 (*)
│   │   ├── glow v0.16.0                                                                             # OpenGL bindings
│   │   ├── log v0.4.29                                                                              # Logging facade
│   │   ├── memoffset v0.9.1
│   │   │   [build-dependencies]
│   │   │   └── autocfg v1.5.0                                                                       # Build config detection
│   │   └── profiling v1.0.17                                                                        # Profiling macros
│   ├── glow v0.16.0                                                                                 # OpenGL bindings
│   ├── glutin v0.32.3                                                                               # OpenGL context
│   │   ├── bitflags v2.11.0 (*)
│   │   ├── glutin_egl_sys v0.7.1
│   │   │   [build-dependencies]
│   │   │   └── gl_generator v0.14.0
│   │   │       ├── khronos_api v3.1.0
│   │   │       ├── log v0.4.29                                                                      # Logging facade
│   │   │       └── xml-rs v0.8.28
│   │   ├── glutin_glx_sys v0.6.1
│   │   │   └── x11-dl v2.21.0 (*)
│   │   │   [build-dependencies]
│   │   │   └── gl_generator v0.14.0 (*)
│   │   ├── libloading v0.8.9 (*)
│   │   ├── once_cell v1.21.4                                                                        # Lazy statics
│   │   ├── raw-window-handle v0.6.2
│   │   ├── wayland-sys v0.31.11 (*)
│   │   └── x11-dl v2.21.0 (*)
│   │   [build-dependencies]
│   │   └── cfg_aliases v0.2.1
│   ├── glutin-winit v0.5.0
│   │   ├── glutin v0.32.3 (*)
│   │   ├── raw-window-handle v0.6.2
│   │   └── winit v0.30.13 (*)
│   │   [build-dependencies]
│   │   └── cfg_aliases v0.2.1
│   ├── home v0.5.12
│   ├── image v0.25.10 (*)
│   ├── log v0.4.29                                                                                  # Logging facade
│   ├── parking_lot v0.12.5 (*)
│   ├── profiling v1.0.17                                                                            # Profiling macros
│   ├── raw-window-handle v0.6.2
│   ├── ron v0.11.0 (*)
│   ├── serde v1.0.228 (*)
│   ├── static_assertions v1.1.0                                                                     # Compile-time assertions
│   ├── web-time v1.1.0
│   └── winit v0.30.13 (*)
├── egui v0.33.3 (*)
├── egui-i18n v0.2.0                                                                                 # egui i18n
│   ├── fluent v0.17.0
│   │   ├── fluent-bundle v0.16.0
│   │   │   ├── fluent-langneg v0.13.1
│   │   │   │   └── unic-langid v0.9.6
│   │   │   │       └── unic-langid-impl v0.9.6
│   │   │   │           └── tinystr v0.8.2 (*)
│   │   │   ├── fluent-syntax v0.12.0
│   │   │   │   ├── memchr v2.8.0                                                                    # Memory search
│   │   │   │   └── thiserror v2.0.18 (*)
│   │   │   ├── intl-memoizer v0.5.3
│   │   │   │   ├── type-map v0.5.1
│   │   │   │   │   └── rustc-hash v2.1.2
│   │   │   │   └── unic-langid v0.9.6 (*)
│   │   │   ├── intl_pluralrules v7.0.2
│   │   │   │   └── unic-langid v0.9.6 (*)
│   │   │   ├── rustc-hash v2.1.2
│   │   │   ├── self_cell v1.2.2
│   │   │   ├── smallvec v1.15.1                                                                     # Small vector optimization
│   │   │   └── unic-langid v0.9.6 (*)
│   │   └── unic-langid v0.9.6 (*)
│   ├── fluent-bundle v0.16.0 (*)
│   ├── intl-memoizer v0.5.3 (*)
│   ├── log v0.4.29                                                                                  # Logging facade
│   ├── once_cell v1.21.4                                                                            # Lazy statics
│   └── unic-langid v0.9.6 (*)
├── egui-material3 v0.0.10                                                                           # Material3 theme (https://github.com/nikescar/egui-material3/#35cdd449)
│   ├── base64 v0.22.1                                                                               # Base64 encoding
│   ├── dark-light v1.1.1
│   │   ├── dconf_rs v0.3.0
│   │   ├── detect-desktop-environment v0.2.0
│   │   ├── dirs v4.0.0                                                                              # User directories
│   │   │   └── dirs-sys v0.3.7                                                                      # Directory system layer
│   │   │       └── libc v0.2.183                                                                    # C library bindings
│   │   ├── rust-ini v0.18.0
│   │   │   ├── cfg-if v1.0.4                                                                        # Conditional compilation
│   │   │   └── ordered-multimap v0.4.3
│   │   │       ├── dlv-list v0.3.0
│   │   │       └── hashbrown v0.12.3                                                                # Fast HashMap
│   │   │           └── ahash v0.7.8                                                                 # Fast hash function
│   │   │               ├── getrandom v0.2.17                                                        # OS random number
│   │   │               │   ├── cfg-if v1.0.4                                                        # Conditional compilation
│   │   │               │   └── libc v0.2.183                                                        # C library bindings
│   │   │               └── once_cell v1.21.4                                                        # Lazy statics
│   │   │               [build-dependencies]
│   │   │               └── version_check v0.9.5                                                     # Rustc version check
│   │   └── zbus v4.4.0                                                                              # D-Bus protocol
│   │       ├── async-broadcast v0.7.2 (*)
│   │       ├── async-executor v1.14.0 (*)
│   │       ├── async-fs v2.2.0
│   │       │   ├── async-lock v3.4.2 (*)
│   │       │   ├── blocking v1.6.2 (*)
│   │       │   └── futures-lite v2.6.1 (*)
│   │       ├── async-io v2.6.0 (*)
│   │       ├── async-lock v3.4.2 (*)
│   │       ├── async-task v4.7.1                                                                    # Async task
│   │       ├── async-trait v0.1.89 (proc-macro) (*)
│   │       ├── blocking v1.6.2 (*)
│   │       ├── enumflags2 v0.7.12 (*)
│   │       ├── event-listener v5.4.1 (*)
│   │       ├── futures-core v0.3.32                                                                 # Futures core
│   │       ├── futures-sink v0.3.32
│   │       ├── futures-util v0.3.32 (*)
│   │       ├── hex v0.4.3
│   │       ├── nix v0.29.0
│   │       │   ├── bitflags v2.11.0 (*)
│   │       │   ├── cfg-if v1.0.4                                                                    # Conditional compilation
│   │       │   ├── libc v0.2.183                                                                    # C library bindings
│   │       │   └── memoffset v0.9.1 (*)
│   │       │   [build-dependencies]
│   │       │   └── cfg_aliases v0.2.1
│   │       ├── ordered-stream v0.2.0 (*)
│   │       ├── rand v0.8.5                                                                          # Random numbers
│   │       │   ├── libc v0.2.183                                                                    # C library bindings
│   │       │   ├── rand_chacha v0.3.1
│   │       │   │   ├── ppv-lite86 v0.2.21
│   │       │   │   │   └── zerocopy v0.8.48                                                         # Zero-copy parsing
│   │       │   │   └── rand_core v0.6.4
│   │       │   │       └── getrandom v0.2.17 (*)
│   │       │   └── rand_core v0.6.4 (*)
│   │       ├── serde v1.0.228 (*)
│   │       ├── serde_repr v0.1.20 (proc-macro) (*)
│   │       ├── sha1 v0.10.6
│   │       │   ├── cfg-if v1.0.4                                                                    # Conditional compilation
│   │       │   ├── cpufeatures v0.2.17
│   │       │   └── digest v0.10.7
│   │       │       ├── block-buffer v0.10.4
│   │       │       │   └── generic-array v0.14.7
│   │       │       │       └── typenum v1.19.0
│   │       │       │       [build-dependencies]
│   │       │       │       └── version_check v0.9.5                                                 # Rustc version check
│   │       │       └── crypto-common v0.1.7
│   │       │           ├── generic-array v0.14.7 (*)
│   │       │           └── typenum v1.19.0
│   │       ├── static_assertions v1.1.0                                                             # Compile-time assertions
│   │       ├── tracing v0.1.44 (*)
│   │       ├── xdg-home v1.3.0
│   │       │   └── libc v0.2.183                                                                    # C library bindings
│   │       ├── zbus_macros v4.4.0 (proc-macro)
│   │       │   ├── proc-macro-crate v3.5.0 (*)
│   │       │   ├── proc-macro2 v1.0.106 (*)
│   │       │   ├── quote v1.0.45 (*)
│   │       │   ├── syn v2.0.117 (*)
│   │       │   └── zvariant_utils v2.1.0                                                            # Zvariant utilities
│   │       │       ├── proc-macro2 v1.0.106 (*)
│   │       │       ├── quote v1.0.45 (*)
│   │       │       └── syn v2.0.117 (*)
│   │       ├── zbus_names v3.0.0
│   │       │   ├── serde v1.0.228 (*)
│   │       │   ├── static_assertions v1.1.0                                                         # Compile-time assertions
│   │       │   └── zvariant v4.2.0                                                                  # D-Bus variant type
│   │       │       ├── endi v1.1.1
│   │       │       ├── enumflags2 v0.7.12 (*)
│   │       │       ├── serde v1.0.228 (*)
│   │       │       ├── static_assertions v1.1.0                                                     # Compile-time assertions
│   │       │       └── zvariant_derive v4.2.0 (proc-macro)                                          # Zvariant derives
│   │       │           ├── proc-macro-crate v3.5.0 (*)
│   │       │           ├── proc-macro2 v1.0.106 (*)
│   │       │           ├── quote v1.0.45 (*)
│   │       │           ├── syn v2.0.117 (*)
│   │       │           └── zvariant_utils v2.1.0 (*)
│   │       └── zvariant v4.2.0 (*)
│   ├── eframe v0.33.3 (*)
│   ├── egui v0.33.3 (*)
│   ├── hex v0.4.3
│   ├── image v0.25.10 (*)
│   ├── lazy_static v1.5.0
│   ├── once_cell v1.21.4                                                                            # Lazy statics
│   ├── resvg v0.47.0
│   │   ├── gif v0.14.1 (*)
│   │   ├── image-webp v0.2.4 (*)
│   │   ├── log v0.4.29                                                                              # Logging facade
│   │   ├── pico-args v0.5.0
│   │   ├── rgb v0.8.53
│   │   │   └── bytemuck v1.25.0 (*)
│   │   ├── svgtypes v0.16.1
│   │   │   ├── kurbo v0.13.0
│   │   │   │   ├── arrayvec v0.7.6
│   │   │   │   └── smallvec v1.15.1                                                                 # Small vector optimization
│   │   │   └── siphasher v1.0.2
│   │   ├── tiny-skia v0.12.0
│   │   │   ├── arrayref v0.3.9
│   │   │   ├── arrayvec v0.7.6
│   │   │   ├── bytemuck v1.25.0 (*)
│   │   │   ├── cfg-if v1.0.4                                                                        # Conditional compilation
│   │   │   ├── log v0.4.29                                                                          # Logging facade
│   │   │   ├── png v0.18.1 (*)
│   │   │   └── tiny-skia-path v0.12.0
│   │   │       ├── arrayref v0.3.9
│   │   │       ├── bytemuck v1.25.0 (*)
│   │   │       └── strict-num v0.1.1 (*)
│   │   ├── usvg v0.47.0
│   │   │   ├── base64 v0.22.1                                                                       # Base64 encoding
│   │   │   ├── data-url v0.3.2
│   │   │   ├── flate2 v1.1.9 (*)
│   │   │   ├── fontdb v0.23.0
│   │   │   │   ├── fontconfig-parser v0.5.8
│   │   │   │   │   └── roxmltree v0.20.0
│   │   │   │   ├── log v0.4.29                                                                      # Logging facade
│   │   │   │   ├── memmap2 v0.9.10 (*)
│   │   │   │   ├── slotmap v1.1.1
│   │   │   │   │   [build-dependencies]
│   │   │   │   │   └── version_check v0.9.5                                                         # Rustc version check
│   │   │   │   ├── tinyvec v1.11.0
│   │   │   │   │   └── tinyvec_macros v0.1.1
│   │   │   │   └── ttf-parser v0.25.1 (*)
│   │   │   ├── imagesize v0.14.0
│   │   │   ├── kurbo v0.13.0 (*)
│   │   │   ├── log v0.4.29                                                                          # Logging facade
│   │   │   ├── pico-args v0.5.0
│   │   │   ├── roxmltree v0.21.1
│   │   │   │   └── memchr v2.8.0                                                                    # Memory search
│   │   │   ├── rustybuzz v0.20.1
│   │   │   │   ├── bitflags v2.11.0 (*)
│   │   │   │   ├── bytemuck v1.25.0 (*)
│   │   │   │   ├── core_maths v0.1.1 (*)
│   │   │   │   ├── log v0.4.29                                                                      # Logging facade
│   │   │   │   ├── smallvec v1.15.1                                                                 # Small vector optimization
│   │   │   │   ├── ttf-parser v0.25.1 (*)
│   │   │   │   ├── unicode-bidi-mirroring v0.4.0
│   │   │   │   ├── unicode-ccc v0.4.0
│   │   │   │   ├── unicode-properties v0.1.4
│   │   │   │   └── unicode-script v0.5.8
│   │   │   ├── simplecss v0.2.2
│   │   │   │   └── log v0.4.29                                                                      # Logging facade
│   │   │   ├── siphasher v1.0.2
│   │   │   ├── strict-num v0.1.1 (*)
│   │   │   ├── svgtypes v0.16.1 (*)
│   │   │   ├── tiny-skia-path v0.12.0 (*)
│   │   │   ├── ttf-parser v0.25.1 (*)
│   │   │   ├── unicode-bidi v0.3.18
│   │   │   ├── unicode-script v0.5.8
│   │   │   ├── unicode-vo v0.1.0
│   │   │   └── xmlwriter v0.1.0
│   │   └── zune-jpeg v0.5.15 (*)
│   ├── serde v1.0.228 (*)
│   ├── serde_json v1.0.149                                                                          # JSON serialization
│   │   ├── itoa v1.0.18
│   │   ├── memchr v2.8.0                                                                            # Memory search
│   │   ├── serde_core v1.0.228                                                                      # Serde core traits
│   │   └── zmij v1.0.21
│   └── tiny-skia v0.12.0 (*)
│   [build-dependencies]
│   ├── once_cell v1.21.4                                                                            # Lazy statics
│   ├── serde_json v1.0.149 (*)
│   └── ureq v2.12.1                                                                                 # HTTP client
│       ├── base64 v0.22.1                                                                           # Base64 encoding
│       ├── flate2 v1.1.9 (*)
│       ├── log v0.4.29                                                                              # Logging facade
│       ├── once_cell v1.21.4                                                                        # Lazy statics
│       ├── rustls v0.23.37                                                                          # TLS library
│       │   ├── log v0.4.29                                                                          # Logging facade
│       │   ├── once_cell v1.21.4                                                                    # Lazy statics
│       │   ├── ring v0.17.14                                                                        # Crypto primitives
│       │   │   ├── cfg-if v1.0.4                                                                    # Conditional compilation
│       │   │   ├── getrandom v0.2.17                                                                # OS random number
│       │   │   │   ├── cfg-if v1.0.4                                                                # Conditional compilation
│       │   │   │   └── libc v0.2.183                                                                # C library bindings
│       │   │   └── untrusted v0.9.0
│       │   │   [build-dependencies]
│       │   │   └── cc v1.2.58 (*)
│       │   ├── rustls-pki-types v1.14.0
│       │   │   └── zeroize v1.8.2
│       │   ├── rustls-webpki v0.103.10
│       │   │   ├── ring v0.17.14 (*)
│       │   │   ├── rustls-pki-types v1.14.0 (*)
│       │   │   └── untrusted v0.9.0
│       │   ├── subtle v2.6.1
│       │   └── zeroize v1.8.2
│       ├── rustls-pki-types v1.14.0 (*)
│       ├── serde v1.0.228 (*)
│       ├── serde_json v1.0.149 (*)
│       ├── url v2.5.8 (*)
│       └── webpki-roots v0.26.11
│           └── webpki-roots v1.0.6
│               └── rustls-pki-types v1.14.0 (*)
├── egui_extras v0.33.3                                                                              # egui extra widgets
│   ├── ahash v0.8.12 (*)
│   ├── egui v0.33.3 (*)
│   ├── ehttp v0.6.0                                                                                 # HTTP for egui
│   │   ├── document-features v0.2.12 (proc-macro) (*)
│   │   └── ureq v2.12.1                                                                             # HTTP client
│   │       ├── base64 v0.22.1                                                                       # Base64 encoding
│   │       ├── flate2 v1.1.9 (*)
│   │       ├── log v0.4.29                                                                          # Logging facade
│   │       ├── once_cell v1.21.4                                                                    # Lazy statics
│   │       ├── rustls v0.23.37 (*)
│   │       ├── rustls-pki-types v1.14.0 (*)
│   │       ├── url v2.5.8 (*)
│   │       └── webpki-roots v0.26.11 (*)
│   ├── enum-map v2.7.3
│   │   └── enum-map-derive v0.17.0 (proc-macro)
│   │       ├── proc-macro2 v1.0.106 (*)
│   │       ├── quote v1.0.45 (*)
│   │       └── syn v2.0.117 (*)
│   ├── image v0.25.10 (*)
│   ├── log v0.4.29                                                                                  # Logging facade
│   ├── mime_guess2 v2.3.1
│   │   ├── mime v0.3.17
│   │   └── unicase v2.9.0
│   │   [build-dependencies]
│   │   ├── phf v0.11.3
│   │   │   └── phf_shared v0.11.3
│   │   │       ├── siphasher v1.0.2
│   │   │       └── unicase v2.9.0
│   │   ├── phf_shared v0.11.3 (*)
│   │   └── unicase v2.9.0
│   ├── profiling v1.0.17                                                                            # Profiling macros
│   └── resvg v0.45.1
│       ├── log v0.4.29                                                                              # Logging facade
│       ├── pico-args v0.5.0
│       ├── rgb v0.8.53 (*)
│       ├── svgtypes v0.15.3
│       │   ├── kurbo v0.11.3
│       │   │   ├── arrayvec v0.7.6
│       │   │   └── smallvec v1.15.1                                                                 # Small vector optimization
│       │   └── siphasher v1.0.2
│       ├── tiny-skia v0.11.4 (*)
│       └── usvg v0.45.1
│           ├── base64 v0.22.1                                                                       # Base64 encoding
│           ├── data-url v0.3.2
│           ├── flate2 v1.1.9 (*)
│           ├── imagesize v0.13.0
│           ├── kurbo v0.11.3 (*)
│           ├── log v0.4.29                                                                          # Logging facade
│           ├── pico-args v0.5.0
│           ├── roxmltree v0.20.0
│           ├── simplecss v0.2.2 (*)
│           ├── siphasher v1.0.2
│           ├── strict-num v0.1.1 (*)
│           ├── svgtypes v0.15.3 (*)
│           ├── tiny-skia-path v0.11.4 (*)
│           └── xmlwriter v0.1.0
├── ehttp v0.5.0                                                                                     # HTTP for egui
│   ├── document-features v0.2.12 (proc-macro) (*)
│   └── ureq v2.12.1 (*)
├── env_logger v0.11.10                                                                              # Env logger
│   ├── env_filter v1.0.1
│   │   └── log v0.4.29                                                                              # Logging facade
│   └── log v0.4.29                                                                                  # Logging facade
├── flate2 v1.1.9 (*)
├── gtk v0.18.2                                                                                      # GTK3 bindings
│   ├── atk v0.18.2
│   │   ├── atk-sys v0.18.2
│   │   │   ├── glib-sys v0.18.1
│   │   │   │   └── libc v0.2.183                                                                    # C library bindings
│   │   │   │   [build-dependencies]
│   │   │   │   └── system-deps v6.2.2
│   │   │   │       ├── cfg-expr v0.15.8
│   │   │   │       │   ├── smallvec v1.15.1                                                         # Small vector optimization
│   │   │   │       │   └── target-lexicon v0.12.16
│   │   │   │       ├── heck v0.5.0
│   │   │   │       ├── pkg-config v0.3.32                                                           # Package config
│   │   │   │       ├── toml v0.8.2
│   │   │   │       │   ├── serde v1.0.228 (*)
│   │   │   │       │   ├── serde_spanned v0.6.9
│   │   │   │       │   │   └── serde v1.0.228 (*)
│   │   │   │       │   ├── toml_datetime v0.6.3
│   │   │   │       │   │   └── serde v1.0.228 (*)
│   │   │   │       │   └── toml_edit v0.20.2
│   │   │   │       │       ├── indexmap v2.13.0 (*)
│   │   │   │       │       ├── serde v1.0.228 (*)
│   │   │   │       │       ├── serde_spanned v0.6.9 (*)
│   │   │   │       │       ├── toml_datetime v0.6.3 (*)
│   │   │   │       │       └── winnow v0.5.40
│   │   │   │       └── version-compare v0.2.1
│   │   │   ├── gobject-sys v0.18.0
│   │   │   │   ├── glib-sys v0.18.1 (*)
│   │   │   │   └── libc v0.2.183                                                                    # C library bindings
│   │   │   │   [build-dependencies]
│   │   │   │   └── system-deps v6.2.2 (*)
│   │   │   └── libc v0.2.183                                                                        # C library bindings
│   │   │   [build-dependencies]
│   │   │   └── system-deps v6.2.2 (*)
│   │   ├── glib v0.18.5                                                                             # GLib bindings
│   │   │   ├── bitflags v2.11.0 (*)
│   │   │   ├── futures-channel v0.3.32
│   │   │   │   └── futures-core v0.3.32                                                             # Futures core
│   │   │   ├── futures-core v0.3.32                                                                 # Futures core
│   │   │   ├── futures-executor v0.3.32
│   │   │   │   ├── futures-core v0.3.32                                                             # Futures core
│   │   │   │   ├── futures-task v0.3.32
│   │   │   │   └── futures-util v0.3.32 (*)
│   │   │   ├── futures-task v0.3.32
│   │   │   ├── futures-util v0.3.32 (*)
│   │   │   ├── gio-sys v0.18.1
│   │   │   │   ├── glib-sys v0.18.1 (*)
│   │   │   │   ├── gobject-sys v0.18.0 (*)
│   │   │   │   └── libc v0.2.183                                                                    # C library bindings
│   │   │   │   [build-dependencies]
│   │   │   │   └── system-deps v6.2.2 (*)
│   │   │   ├── glib-macros v0.18.5 (proc-macro)
│   │   │   │   ├── heck v0.4.1
│   │   │   │   ├── proc-macro-crate v2.0.2
│   │   │   │   │   ├── toml_datetime v0.6.3 (*)
│   │   │   │   │   └── toml_edit v0.20.2 (*)
│   │   │   │   ├── proc-macro-error v1.0.4
│   │   │   │   │   ├── proc-macro-error-attr v1.0.4 (proc-macro)
│   │   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   │   └── quote v1.0.45 (*)
│   │   │   │   │   │   [build-dependencies]
│   │   │   │   │   │   └── version_check v0.9.5                                                     # Rustc version check
│   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   │   └── syn v1.0.109                                                                 # Rust syntax parser
│   │   │   │   │       ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │       └── unicode-ident v1.0.24                                                    # Unicode identifiers
│   │   │   │   │   [build-dependencies]
│   │   │   │   │   └── version_check v0.9.5                                                         # Rustc version check
│   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   └── syn v2.0.117 (*)
│   │   │   ├── glib-sys v0.18.1 (*)
│   │   │   ├── gobject-sys v0.18.0 (*)
│   │   │   ├── libc v0.2.183                                                                        # C library bindings
│   │   │   ├── memchr v2.8.0                                                                        # Memory search
│   │   │   ├── once_cell v1.21.4                                                                    # Lazy statics
│   │   │   ├── smallvec v1.15.1                                                                     # Small vector optimization
│   │   │   └── thiserror v1.0.69 (*)
│   │   └── libc v0.2.183                                                                            # C library bindings
│   ├── cairo-rs v0.18.5
│   │   ├── bitflags v2.11.0 (*)
│   │   ├── cairo-sys-rs v0.18.2
│   │   │   ├── glib-sys v0.18.1 (*)
│   │   │   └── libc v0.2.183                                                                        # C library bindings
│   │   │   [build-dependencies]
│   │   │   └── system-deps v6.2.2 (*)
│   │   ├── glib v0.18.5 (*)
│   │   ├── libc v0.2.183                                                                            # C library bindings
│   │   ├── once_cell v1.21.4                                                                        # Lazy statics
│   │   └── thiserror v1.0.69 (*)
│   ├── field-offset v0.3.6
│   │   └── memoffset v0.9.1 (*)
│   │   [build-dependencies]
│   │   └── rustc_version v0.4.1
│   │       └── semver v1.0.27
│   ├── futures-channel v0.3.32 (*)
│   ├── gdk v0.18.2                                                                                  # GDK bindings
│   │   ├── cairo-rs v0.18.5 (*)
│   │   ├── gdk-pixbuf v0.18.5
│   │   │   ├── gdk-pixbuf-sys v0.18.0
│   │   │   │   ├── gio-sys v0.18.1 (*)
│   │   │   │   ├── glib-sys v0.18.1 (*)
│   │   │   │   ├── gobject-sys v0.18.0 (*)
│   │   │   │   └── libc v0.2.183                                                                    # C library bindings
│   │   │   │   [build-dependencies]
│   │   │   │   └── system-deps v6.2.2 (*)
│   │   │   ├── gio v0.18.4                                                                          # GIO bindings
│   │   │   │   ├── futures-channel v0.3.32 (*)
│   │   │   │   ├── futures-core v0.3.32                                                             # Futures core
│   │   │   │   ├── futures-io v0.3.32                                                               # Futures I/O
│   │   │   │   ├── futures-util v0.3.32 (*)
│   │   │   │   ├── gio-sys v0.18.1 (*)
│   │   │   │   ├── glib v0.18.5 (*)
│   │   │   │   ├── libc v0.2.183                                                                    # C library bindings
│   │   │   │   ├── once_cell v1.21.4                                                                # Lazy statics
│   │   │   │   ├── pin-project-lite v0.2.17                                                         # Pin projection
│   │   │   │   ├── smallvec v1.15.1                                                                 # Small vector optimization
│   │   │   │   └── thiserror v1.0.69 (*)
│   │   │   ├── glib v0.18.5 (*)
│   │   │   ├── libc v0.2.183                                                                        # C library bindings
│   │   │   └── once_cell v1.21.4                                                                    # Lazy statics
│   │   ├── gdk-sys v0.18.2
│   │   │   ├── cairo-sys-rs v0.18.2 (*)
│   │   │   ├── gdk-pixbuf-sys v0.18.0 (*)
│   │   │   ├── gio-sys v0.18.1 (*)
│   │   │   ├── glib-sys v0.18.1 (*)
│   │   │   ├── gobject-sys v0.18.0 (*)
│   │   │   ├── libc v0.2.183                                                                        # C library bindings
│   │   │   └── pango-sys v0.18.0
│   │   │       ├── glib-sys v0.18.1 (*)
│   │   │       ├── gobject-sys v0.18.0 (*)
│   │   │       └── libc v0.2.183                                                                    # C library bindings
│   │   │       [build-dependencies]
│   │   │       └── system-deps v6.2.2 (*)
│   │   │   [build-dependencies]
│   │   │   ├── pkg-config v0.3.32                                                                   # Package config
│   │   │   └── system-deps v6.2.2 (*)
│   │   ├── gio v0.18.4 (*)
│   │   ├── glib v0.18.5 (*)
│   │   ├── libc v0.2.183                                                                            # C library bindings
│   │   └── pango v0.18.3                                                                            # Text layout
│   │       ├── gio v0.18.4 (*)
│   │       ├── glib v0.18.5 (*)
│   │       ├── libc v0.2.183                                                                        # C library bindings
│   │       ├── once_cell v1.21.4                                                                    # Lazy statics
│   │       └── pango-sys v0.18.0 (*)
│   ├── gdk-pixbuf v0.18.5 (*)
│   ├── gio v0.18.4 (*)
│   ├── glib v0.18.5 (*)
│   ├── gtk-sys v0.18.2
│   │   ├── atk-sys v0.18.2 (*)
│   │   ├── cairo-sys-rs v0.18.2 (*)
│   │   ├── gdk-pixbuf-sys v0.18.0 (*)
│   │   ├── gdk-sys v0.18.2 (*)
│   │   ├── gio-sys v0.18.1 (*)
│   │   ├── glib-sys v0.18.1 (*)
│   │   ├── gobject-sys v0.18.0 (*)
│   │   ├── libc v0.2.183                                                                            # C library bindings
│   │   └── pango-sys v0.18.0 (*)
│   │   [build-dependencies]
│   │   └── system-deps v6.2.2 (*)
│   ├── gtk3-macros v0.18.2 (proc-macro)
│   │   ├── proc-macro-crate v1.3.1
│   │   │   ├── once_cell v1.21.4                                                                    # Lazy statics
│   │   │   └── toml_edit v0.19.15
│   │   │       ├── indexmap v2.13.0 (*)
│   │   │       ├── toml_datetime v0.6.3 (*)
│   │   │       └── winnow v0.5.40
│   │   ├── proc-macro-error v1.0.4 (*)
│   │   ├── proc-macro2 v1.0.106 (*)
│   │   ├── quote v1.0.45 (*)
│   │   └── syn v2.0.117 (*)
│   ├── libc v0.2.183                                                                                # C library bindings
│   └── pango v0.18.3 (*)
│   [build-dependencies]
│   └── pkg-config v0.3.32                                                                           # Package config
├── image v0.25.10 (*)
├── log v0.4.29                                                                                      # Logging facade
├── md5 v0.7.0                                                                                       # MD5 hashing
├── num_cpus v1.17.0                                                                                 # CPU count
│   └── libc v0.2.183                                                                                # C library bindings
├── opener v0.7.2                                                                                    # Open files/URLs
│   └── bstr v1.12.1
│       ├── memchr v2.8.0                                                                            # Memory search
│       └── regex-automata v0.4.14
├── poll-promise v0.3.0                                                                              # Async for egui
│   ├── document-features v0.2.12 (proc-macro) (*)
│   └── static_assertions v1.1.0                                                                     # Compile-time assertions
├── rand v0.8.5 (*)
├── serde v1.0.228 (*)
├── serde_json v1.0.149 (*)
├── sys-locale v0.3.2                                                                                # System locale
├── tao v0.33.0                                                                                      # Event loop
│   ├── bitflags v2.11.0 (*)
│   ├── crossbeam-channel v0.5.15
│   │   └── crossbeam-utils v0.8.21                                                                  # Concurrency utils
│   ├── dlopen2 v0.7.0
│   │   ├── dlopen2_derive v0.4.3 (proc-macro)
│   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   ├── quote v1.0.45 (*)
│   │   │   └── syn v2.0.117 (*)
│   │   ├── libc v0.2.183                                                                            # C library bindings
│   │   └── once_cell v1.21.4                                                                        # Lazy statics
│   ├── dpi v0.1.2
│   ├── gdkwayland-sys v0.18.2
│   │   ├── gdk-sys v0.18.2 (*)
│   │   ├── glib-sys v0.18.1 (*)
│   │   ├── gobject-sys v0.18.0 (*)
│   │   └── libc v0.2.183                                                                            # C library bindings
│   │   [build-dependencies]
│   │   ├── pkg-config v0.3.32                                                                       # Package config
│   │   └── system-deps v6.2.2 (*)
│   ├── gdkx11-sys v0.18.2
│   │   ├── gdk-sys v0.18.2 (*)
│   │   ├── glib-sys v0.18.1 (*)
│   │   ├── libc v0.2.183                                                                            # C library bindings
│   │   └── x11 v2.21.0
│   │       └── libc v0.2.183                                                                        # C library bindings
│   │       [build-dependencies]
│   │       └── pkg-config v0.3.32                                                                   # Package config
│   │   [build-dependencies]
│   │   └── system-deps v6.2.2 (*)
│   ├── gtk v0.18.2 (*)
│   ├── lazy_static v1.5.0
│   ├── libc v0.2.183                                                                                # C library bindings
│   ├── log v0.4.29                                                                                  # Logging facade
│   ├── parking_lot v0.12.5 (*)
│   ├── raw-window-handle v0.6.2
│   ├── url v2.5.8 (*)
│   └── x11-dl v2.21.0 (*)
├── tar v0.4.45                                                                                      # TAR archives
│   ├── filetime v0.2.27
│   │   ├── cfg-if v1.0.4                                                                            # Conditional compilation
│   │   └── libc v0.2.183                                                                            # C library bindings
│   ├── libc v0.2.183                                                                                # C library bindings
│   └── xattr v1.6.1
│       └── rustix v1.1.4 (*)
├── trash v5.2.5                                                                                     # Recycle bin
│   ├── chrono v0.4.44 (*)
│   ├── libc v0.2.183                                                                                # C library bindings
│   ├── log v0.4.29                                                                                  # Logging facade
│   ├── once_cell v1.21.4                                                                            # Lazy statics
│   ├── scopeguard v1.2.0                                                                            # RAII scope guard
│   └── urlencoding v2.1.3
├── tray-icon v0.18.0                                                                                # System tray
│   ├── crossbeam-channel v0.5.15 (*)
│   ├── dirs v5.0.1 (*)
│   ├── libappindicator v0.9.0
│   │   ├── glib v0.18.5 (*)
│   │   ├── gtk v0.18.2 (*)
│   │   ├── gtk-sys v0.18.2 (*)
│   │   ├── libappindicator-sys v0.9.0
│   │   │   ├── gtk-sys v0.18.2 (*)
│   │   │   ├── libloading v0.7.4                                                                    # Library loading
│   │   │   │   └── cfg-if v1.0.4                                                                    # Conditional compilation
│   │   │   └── once_cell v1.21.4                                                                    # Lazy statics
│   │   └── log v0.4.29                                                                              # Logging facade
│   ├── muda v0.15.3
│   │   ├── crossbeam-channel v0.5.15 (*)
│   │   ├── dpi v0.1.2
│   │   ├── gtk v0.18.2 (*)
│   │   ├── keyboard-types v0.7.0
│   │   │   ├── bitflags v2.11.0 (*)
│   │   │   ├── serde v1.0.228 (*)
│   │   │   └── unicode-segmentation v1.13.2                                                         # Unicode segmentation
│   │   ├── once_cell v1.21.4                                                                        # Lazy statics
│   │   └── thiserror v1.0.69 (*)
│   ├── once_cell v1.21.4                                                                            # Lazy statics
│   ├── png v0.17.16 (*)
│   └── thiserror v1.0.69 (*)
├── ureq v2.12.1 (*)
├── webbrowser v1.2.0 (*)
└── zip v2.4.2                                                                                       # ZIP archives
    ├── crc32fast v1.5.0 (*)
    ├── displaydoc v0.2.5 (proc-macro) (*)
    ├── flate2 v1.1.9 (*)
    ├── indexmap v2.13.0 (*)
    ├── memchr v2.8.0                                                                                # Memory search
    ├── thiserror v2.0.18 (*)
    └── zopfli v0.8.3
        ├── bumpalo v3.20.2
        ├── crc32fast v1.5.0 (*)
        ├── log v0.4.29                                                                              # Logging facade
        └── simd-adler32 v0.3.9
[build-dependencies]
└── winres v0.1.12                                                                                   # Windows resources
    └── toml v0.5.11
        └── serde v1.0.228 (*)
[dev-dependencies]
└── tempfile v3.27.0                                                                                 # Temp files
    ├── fastrand v2.3.0                                                                              # Fast RNG
    ├── getrandom v0.4.2                                                                             # OS random number
    │   ├── cfg-if v1.0.4                                                                            # Conditional compilation
    │   └── libc v0.2.183                                                                            # C library bindings
    ├── once_cell v1.21.4                                                                            # Lazy statics
    └── rustix v1.1.4 (*)
