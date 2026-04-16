//! Rust FFI bindings for darkhttpd - a simple, single-threaded, static content webserver
//!
//! This crate provides both low-level FFI bindings and a safe, idiomatic Rust wrapper
//! for the darkhttpd HTTP server.
//!
//! # Example
//!
//! ```no_run
//! use darkhttpd_sys::DarkHttpd;
//!
//! let mut server = DarkHttpd::new();
//! server.serve("/var/www/htdocs", 8080).expect("Failed to start server");
//!
//! // Server runs until stopped
//! // server.stop();
//! ```

#![allow(unsafe_code)] // Required for FFI

mod ffi;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr;

/// Errors that can occur when working with DarkHttpd
#[derive(Debug, thiserror::Error)]
pub enum DarkHttpdError {
    #[error("Failed to convert string to C string: {0}")]
    StringConversion(#[from] std::ffi::NulError),

    #[error("Initialization failed with code: {0}")]
    InitializationFailed(i32),

    #[error("Server is already initialized")]
    AlreadyInitialized,

    #[error("Server is not initialized")]
    NotInitialized,
}

/// A safe wrapper around the darkhttpd C library
///
/// This provides an idiomatic Rust interface to darkhttpd while managing
/// the underlying C resources safely.
pub struct DarkHttpd {
    initialized: bool,
    running: bool,
}

impl DarkHttpd {
    /// Create a new DarkHttpd instance
    pub fn new() -> Self {
        Self {
            initialized: false,
            running: false,
        }
    }

    /// Start serving files from the specified directory on the given port
    ///
    /// # Arguments
    /// * `path` - The directory to serve files from
    /// * `port` - The port to listen on
    ///
    /// # Example
    /// ```no_run
    /// use darkhttpd_sys::DarkHttpd;
    ///
    /// let mut server = DarkHttpd::new();
    /// server.serve("/var/www/htdocs", 8080).expect("Failed to start server");
    /// ```
    pub fn serve(&mut self, path: &str, port: u16) -> Result<(), DarkHttpdError> {
        if self.initialized {
            return Err(DarkHttpdError::AlreadyInitialized);
        }

        let args = vec![
            CString::new("darkhttpd")?,
            CString::new(path)?,
            CString::new("--port")?,
            CString::new(port.to_string())?,
        ];

        self.init_with_args(&args)?;
        self.start();

        Ok(())
    }

    /// Initialize darkhttpd with custom command-line arguments
    ///
    /// # Arguments
    /// * `path` - The directory to serve files from
    /// * `args` - Additional command-line arguments (e.g., "--port", "8080")
    ///
    /// # Example
    /// ```no_run
    /// use darkhttpd_sys::DarkHttpd;
    ///
    /// let mut server = DarkHttpd::new();
    /// server.serve_with_args("/var/www/htdocs", &["--port", "8080", "--log", "access.log"])
    ///     .expect("Failed to start server");
    /// ```
    pub fn serve_with_args(&mut self, path: &str, args: &[&str]) -> Result<(), DarkHttpdError> {
        if self.initialized {
            return Err(DarkHttpdError::AlreadyInitialized);
        }

        let mut c_args = vec![CString::new("darkhttpd")?, CString::new(path)?];

        for arg in args {
            c_args.push(CString::new(*arg)?);
        }

        self.init_with_args(&c_args)?;
        self.start();

        Ok(())
    }

    /// Internal method to initialize with C strings
    fn init_with_args(&mut self, args: &[CString]) -> Result<(), DarkHttpdError> {
        let mut argv: Vec<*mut c_char> = args.iter().map(|s| s.as_ptr() as *mut c_char).collect();
        argv.push(ptr::null_mut());

        let argc = args.len() as c_int;

        // SAFETY: We've constructed valid C strings and a null-terminated argv array
        let result = unsafe { ffi::darkhttpd_init(argc, argv.as_mut_ptr()) };

        if result != 0 {
            return Err(DarkHttpdError::InitializationFailed(result));
        }

        self.initialized = true;
        Ok(())
    }

    /// Start the server (begin accepting connections)
    pub fn start(&mut self) {
        if self.initialized && !self.running {
            // SAFETY: We've verified initialization
            unsafe { ffi::darkhttpd_start() };
            self.running = true;
        }
    }

    /// Stop the server (stop accepting new connections)
    pub fn stop(&mut self) {
        if self.running {
            // SAFETY: We've verified the server is running
            unsafe { ffi::darkhttpd_stop() };
            self.running = false;
        }
    }

    /// Check if the server is currently running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Run one iteration of the event loop
    ///
    /// This should be called repeatedly to process incoming connections.
    /// Returns `true` if the server is still running, `false` if it should stop.
    ///
    /// # Example
    /// ```no_run
    /// use darkhttpd_sys::DarkHttpd;
    ///
    /// let mut server = DarkHttpd::new();
    /// server.serve("/var/www/htdocs", 8080).expect("Failed to start");
    ///
    /// while server.poll() {
    ///     // Server is running
    /// }
    /// ```
    pub fn poll(&mut self) -> bool {
        if !self.initialized {
            return false;
        }

        // SAFETY: We've verified initialization
        unsafe { ffi::darkhttpd_poll_once() };

        // Update running state from C
        let running = unsafe { ffi::darkhttpd_is_running() != 0 };
        self.running = running;

        running
    }

    /// Run the server until stopped (blocking)
    ///
    /// This is a convenience method that calls `poll()` in a loop.
    ///
    /// # Example
    /// ```no_run
    /// use darkhttpd_sys::DarkHttpd;
    ///
    /// let mut server = DarkHttpd::new();
    /// server.serve("/var/www/htdocs", 8080).expect("Failed to start");
    /// server.run(); // Blocks until server is stopped
    /// ```
    pub fn run(&mut self) {
        while self.poll() {
            // Continue running
        }
    }
}

impl Default for DarkHttpd {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DarkHttpd {
    fn drop(&mut self) {
        if self.initialized {
            self.stop();
            // SAFETY: We've verified initialization and stopped the server
            unsafe { ffi::darkhttpd_cleanup() };
        }
    }
}

// Re-export the raw FFI module for advanced use cases
pub mod raw {
    pub use crate::ffi::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_server() {
        let server = DarkHttpd::new();
        assert!(!server.is_running());
    }

    #[test]
    fn test_default() {
        let server = DarkHttpd::default();
        assert!(!server.is_running());
    }
}
