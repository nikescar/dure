//! Application configuration management
//!
//! Loads configuration from YAML file on desktop and Android platforms.
//! WASM platform uses defaults only.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Cloud platform configuration (GCP, Firebase, Supabase)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloudPlatformConfig {
    pub name: String,
    pub platform_type: String, // "gcp", "firebase", "supabase"

    // GCP specific
    pub gcp_project_id: Option<String>,
    pub gcp_billing_account: Option<String>,
    pub gcp_region: Option<String>,
    pub gcp_oauth_client_id: Option<String>,
    pub gcp_oauth_client_secret: Option<String>,
    pub gcp_oauth_access_token: Option<String>,
    pub gcp_oauth_refresh_token: Option<String>,
    pub gcp_oauth_token_expiry: Option<i64>, // Unix timestamp

    // Firebase specific
    pub firebase_project_id: Option<String>,
    pub firebase_api_key: Option<String>,

    // Supabase specific
    pub supabase_project_ref: Option<String>,
    pub supabase_api_url: Option<String>,
    pub supabase_anon_key: Option<String>,

    // Common fields
    pub api_token: Option<String>,
    pub service_account_json: Option<String>,
}

/// Platform configuration (deprecated, use platforms list)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatformConfig {
    pub name: String,
}

/// SSL certificate configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CertConfig {
    /// Email for Let's Encrypt notifications
    pub email: Option<String>,
    /// Accept Let's Encrypt Terms of Service (https://letsencrypt.org/documents/LE-SA-v1.4-April-3-2024.pdf)
    pub lego_tos_accepted: Option<bool>,
    /// Path to certificate file (.crt) - auto-updated by acme commands
    pub cert_path: Option<String>,
    /// Path to private key file (.key) - auto-updated by acme commands
    pub key_path: Option<String>,
    /// Path to issuer certificate (.issuer.crt) - auto-updated by acme commands
    pub issuer_path: Option<String>,
}

/// DNS provider credentials for Cloudflare
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloudflareConfig {
    /// Cloudflare API Token (preferred) or Email
    pub api_token: Option<String>,
    /// Cloudflare Email (legacy, use with api_key)
    pub email: Option<String>,
    /// Cloudflare API Key (legacy, use with email)
    pub api_key: Option<String>,
}

/// DNS provider credentials for DuckDNS
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DuckDnsConfig {
    /// DuckDNS token
    pub token: Option<String>,
}

/// DNS provider credentials for Google Cloud DNS
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoogleCloudConfig {
    /// GCP project ID
    pub project: Option<String>,
    /// Path to service account JSON file
    pub service_account_file: Option<String>,
    /// Service account to impersonate (optional)
    pub impersonate_service_account: Option<String>,
}

/// DNS provider credentials for Porkbun
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PorkbunConfig {
    /// Porkbun API Key
    pub api_key: Option<String>,
    /// Porkbun Secret API Key
    pub secret_api_key: Option<String>,
}

/// Domain configuration including SSL certificates
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DomainConfig {
    /// Primary domain name (e.g., "example.com" or "dure.app")
    pub name: String,

    /// DNS provider type: "cloudflare", "duckdns", "gcloud", or "porkbun"
    pub dns_provider: String,

    /// SSL certificate configuration
    #[serde(default)]
    pub cert: CertConfig,

    /// Cloudflare DNS provider credentials (if dns_provider = "cloudflare")
    #[serde(default)]
    pub cloudflare: CloudflareConfig,

    /// DuckDNS provider credentials (if dns_provider = "duckdns")
    #[serde(default)]
    pub duckdns: DuckDnsConfig,

    /// Google Cloud DNS provider credentials (if dns_provider = "gcloud")
    #[serde(default)]
    pub gcloud: GoogleCloudConfig,

    /// Porkbun DNS provider credentials (if dns_provider = "porkbun")
    #[serde(default)]
    pub porkbun: PorkbunConfig,
}

/// SSH host configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshHostConfig {
    /// SSH connection string (e.g., "username@dure.com")
    pub host: String,
    /// Optional password for authentication
    pub password: Option<String>,
    /// Optional path to private key file
    pub private_key_path: Option<String>,
    /// SSH port (default: 22)
    #[serde(default = "default_ssh_port")]
    pub port: u16,
    /// Initialization status
    #[serde(default)]
    pub initialized: bool,
    /// Last connection status
    #[serde(skip)]
    pub last_status: Option<String>,
}

fn default_ssh_port() -> u16 {
    22
}

impl Default for SshHostConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            password: None,
            private_key_path: None,
            port: default_ssh_port(),
            initialized: false,
            last_status: None,
        }
    }
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerConfig {
    pub web_provider: String,
    pub web_provider_token: String,
    pub db_provider: String,
    /// Enable TLS/SSL for HTTPS and WSS (default: true for production, false for development)
    #[serde(default = "default_use_tls")]
    pub use_tls: bool,
    /// Enable HTTP request/response debug logging (default: false)
    #[serde(default)]
    pub debug_http: bool,
}

fn default_use_tls() -> bool {
    true
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub platform: PlatformConfig,
    #[serde(default)]
    pub platforms: Vec<CloudPlatformConfig>,
    #[serde(default)]
    pub domain: DomainConfig,
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub ssh_hosts: Vec<SshHostConfig>,
    #[serde(default)]
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    pub ns: crate::calc::ns::NsConfig,
}

impl AppConfig {
    /// Load configuration from file
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Load configuration from file or return default
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_or_default(path: &PathBuf) -> Self {
        Self::load(path).unwrap_or_default()
    }

    /// Save configuration to file
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let yaml = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml)?;
        Ok(())
    }

    /// WASM always uses default (no file system access)
    #[cfg(target_arch = "wasm32")]
    pub fn load_or_default(_path: &PathBuf) -> Self {
        Self::default()
    }
}
