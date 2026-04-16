//! Raw FFI bindings to darkhttpd C library
//!
//! This module provides low-level, unsafe bindings to the darkhttpd C functions.
//! For a safe, idiomatic Rust API, use the `DarkHttpd` wrapper in the parent module.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use libc::{c_char, c_int};

extern "C" {
    /// Run darkhttpd with given command-line arguments
    ///
    /// # Safety
    /// - `argv` must be a valid pointer to an array of `argc` C strings
    /// - Each string in `argv` must be null-terminated
    /// - The strings must remain valid for the duration of the call
    pub fn darkhttpd_run(argc: c_int, argv: *mut *mut c_char) -> c_int;

    /// Initialize darkhttpd with given command-line arguments without starting the server
    ///
    /// # Safety
    /// - Same safety requirements as `darkhttpd_run`
    pub fn darkhttpd_init(argc: c_int, argv: *mut *mut c_char) -> c_int;

    /// Run one iteration of the poll loop
    ///
    /// # Safety
    /// - Must be called after `darkhttpd_init`
    /// - Should only be called from a single thread
    pub fn darkhttpd_poll_once();

    /// Start the server (sets the running flag)
    ///
    /// # Safety
    /// - Must be called after `darkhttpd_init`
    pub fn darkhttpd_start();

    /// Stop the server (clears the running flag)
    ///
    /// # Safety
    /// - Can be called at any time after `darkhttpd_init`
    pub fn darkhttpd_stop();

    /// Check if the server is running
    ///
    /// # Safety
    /// - Must be called after `darkhttpd_init`
    pub fn darkhttpd_is_running() -> c_int;

    /// Cleanup and shutdown darkhttpd
    ///
    /// # Safety
    /// - Must be called after `darkhttpd_init`
    /// - Should only be called once
    /// - No other darkhttpd functions should be called after this
    pub fn darkhttpd_cleanup();
}
