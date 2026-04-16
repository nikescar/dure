//! Authentication message types

use asyncapi_rust::schemars::JsonSchema;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Authentication login request
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct AuthLoginRequest {
    /// Device ID (from machine-id)
    pub device_id: String,
    /// Device public key for encryption
    pub public_key: String,
    /// Optional session ID for reconnection
    pub session_id: Option<String>,
    /// Client version
    pub client_version: Option<String>,
}

/// Authentication response
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct AuthResponse {
    /// Whether authentication was successful
    pub success: bool,
    /// Session ID for this connection
    pub session_id: Option<String>,
    /// Server public key for encryption
    pub server_public_key: Option<String>,
    /// Error message if authentication failed
    pub error: Option<String>,
    /// Authenticated user/device information
    pub device_info: Option<DeviceInfo>,
    /// Session expiry time
    pub expires_at: Option<DateTime<Utc>>,
}

/// Device information
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct DeviceInfo {
    /// Device ID
    pub device_id: String,
    /// Device name/hostname
    pub device_name: Option<String>,
    /// Device platform (linux, windows, macos, android)
    pub platform: Option<String>,
    /// Last seen timestamp
    pub last_seen: Option<DateTime<Utc>>,
}

/// Logout request
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct AuthLogoutRequest {
    /// Session ID to logout
    pub session_id: String,
}

/// Logout response
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct AuthLogoutResponse {
    /// Whether logout was successful
    pub success: bool,
    /// Optional message
    pub message: Option<String>,
}
