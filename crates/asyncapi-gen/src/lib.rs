//! Dure AsyncAPI specification generator
//!
//! This crate contains the WebSocket message types and AsyncAPI specification
//! for the Dure distributed e-commerce platform.

pub mod messages;
pub mod asyncapi_spec;

pub use asyncapi_spec::DureApi;
pub use messages::{ClientMessage, ServerMessage};
