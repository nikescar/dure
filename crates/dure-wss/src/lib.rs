//! Dure WSS - WebSocket Secure Server
//!
//! A standalone HTTPS/WSS server with:
//! - TLS certificate support (ACME)
//! - WebSocket connections
//! - Static file serving
//! - WebAuthn authentication
//! - Session tracking
//! - Webhook support
//! - OpenAPI/AsyncAPI documentation

// Re-export WSS functionality from main dure library
pub use dure::wss::client;
pub use dure::wss::server;

// Re-export commonly used types
pub use server::{run_with_args, RunArgs};
