//! Hosting storage model
//!
//! Stores hosting configuration and state for domain registration,
//! DNS management, VM creation, and service deployment.

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Integer, Nullable, Text};
use serde::{Deserialize, Serialize};

/// Hosting configuration status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostingStatus {
    /// Configuration only, not yet initialized
    Configured,
    /// Initialization in progress
    Initializing,
    /// Active and operational
    Active,
    /// Temporarily closed (firewall blocked)
    Closed,
    /// Selected for operations
    Selected,
    /// Error state
    Error,
}

impl HostingStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            HostingStatus::Configured => "configured",
            HostingStatus::Initializing => "initializing",
            HostingStatus::Active => "active",
            HostingStatus::Closed => "closed",
            HostingStatus::Selected => "selected",
            HostingStatus::Error => "error",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "configured" => HostingStatus::Configured,
            "initializing" => HostingStatus::Initializing,
            "active" => HostingStatus::Active,
            "closed" => HostingStatus::Closed,
            "selected" => HostingStatus::Selected,
            "error" => HostingStatus::Error,
            _ => HostingStatus::Configured,
        }
    }
}

/// DNS/Domain provider types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsProvider {
    Porkbun,
    Cloudflare,
    DuckDNS,
    GcpCloudDns,
}

impl DnsProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            DnsProvider::Porkbun => "porkbun",
            DnsProvider::Cloudflare => "cloudflare",
            DnsProvider::DuckDNS => "duckdns",
            DnsProvider::GcpCloudDns => "gcp_clouddns",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "porkbun" => Some(DnsProvider::Porkbun),
            "cloudflare" => Some(DnsProvider::Cloudflare),
            "duckdns" => Some(DnsProvider::DuckDNS),
            "gcp_clouddns" | "gcp" => Some(DnsProvider::GcpCloudDns),
            _ => None,
        }
    }
}

/// VM provider types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VmProvider {
    Gcp,
    Cafe24Vps,
    None,
}

impl VmProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            VmProvider::Gcp => "gcp",
            VmProvider::Cafe24Vps => "cafe24vps",
            VmProvider::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "gcp" => Some(VmProvider::Gcp),
            "cafe24vps" | "cafe24" => Some(VmProvider::Cafe24Vps),
            "none" | "" => Some(VmProvider::None),
            _ => None,
        }
    }
}

/// Hosting configuration record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hosting {
    pub id: i64,
    pub domain: String,
    pub status: HostingStatus,

    // Domain registration
    pub domain_registrar: Option<String>,
    pub domain_registrar_token: Option<String>,
    pub domain_registered: bool,

    // DNS configuration
    pub dns_provider: String,
    pub dns_provider_token: Option<String>,
    pub ns_addresses: Option<String>, // JSON array of NS addresses
    pub dns_configured: bool,

    // VM configuration
    pub vm_provider: String,
    pub vm_provider_token: Option<String>,
    pub vm_instance_id: Option<String>,
    pub vm_ip_address: Option<String>,
    pub vm_ssh_user: Option<String>,
    pub vm_ssh_key_path: Option<String>,
    pub vm_created: bool,

    // Service status
    pub service_installed: bool,
    pub service_running: bool,

    // Metadata
    pub created_at: i64,
    pub updated_at: i64,
    pub error_message: Option<String>,
}

impl Hosting {
    pub fn new(domain: String, dns_provider: String, vm_provider: String) -> Self {
        let now = chrono::Utc::now().timestamp();

        Self {
            id: 0,
            domain,
            status: HostingStatus::Configured,
            domain_registrar: None,
            domain_registrar_token: None,
            domain_registered: false,
            dns_provider,
            dns_provider_token: None,
            ns_addresses: None,
            dns_configured: false,
            vm_provider,
            vm_provider_token: None,
            vm_instance_id: None,
            vm_ip_address: None,
            vm_ssh_user: None,
            vm_ssh_key_path: None,
            vm_created: false,
            service_installed: false,
            service_running: false,
            created_at: now,
            updated_at: now,
            error_message: None,
        }
    }
}

/// Row struct for querying hosting records
#[derive(QueryableByName)]
struct HostingRow {
    #[diesel(sql_type = BigInt)]
    id: i64,
    #[diesel(sql_type = Text)]
    domain: String,
    #[diesel(sql_type = Text)]
    status: String,
    #[diesel(sql_type = Nullable<Text>)]
    domain_registrar: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    domain_registrar_token: Option<String>,
    #[diesel(sql_type = Integer)]
    domain_registered: i32,
    #[diesel(sql_type = Text)]
    dns_provider: String,
    #[diesel(sql_type = Nullable<Text>)]
    dns_provider_token: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    ns_addresses: Option<String>,
    #[diesel(sql_type = Integer)]
    dns_configured: i32,
    #[diesel(sql_type = Text)]
    vm_provider: String,
    #[diesel(sql_type = Nullable<Text>)]
    vm_provider_token: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    vm_instance_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    vm_ip_address: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    vm_ssh_user: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    vm_ssh_key_path: Option<String>,
    #[diesel(sql_type = Integer)]
    vm_created: i32,
    #[diesel(sql_type = Integer)]
    service_installed: i32,
    #[diesel(sql_type = Integer)]
    service_running: i32,
    #[diesel(sql_type = BigInt)]
    created_at: i64,
    #[diesel(sql_type = BigInt)]
    updated_at: i64,
    #[diesel(sql_type = Nullable<Text>)]
    error_message: Option<String>,
}

impl From<HostingRow> for Hosting {
    fn from(row: HostingRow) -> Self {
        Hosting {
            id: row.id,
            domain: row.domain,
            status: HostingStatus::from_str(&row.status),
            domain_registrar: row.domain_registrar,
            domain_registrar_token: row.domain_registrar_token,
            domain_registered: row.domain_registered != 0,
            dns_provider: row.dns_provider,
            dns_provider_token: row.dns_provider_token,
            ns_addresses: row.ns_addresses,
            dns_configured: row.dns_configured != 0,
            vm_provider: row.vm_provider,
            vm_provider_token: row.vm_provider_token,
            vm_instance_id: row.vm_instance_id,
            vm_ip_address: row.vm_ip_address,
            vm_ssh_user: row.vm_ssh_user,
            vm_ssh_key_path: row.vm_ssh_key_path,
            vm_created: row.vm_created != 0,
            service_installed: row.service_installed != 0,
            service_running: row.service_running != 0,
            created_at: row.created_at,
            updated_at: row.updated_at,
            error_message: row.error_message,
        }
    }
}

/// Initialize hosting table (migration handled by diesel_migrations)
pub fn init_hosting_table(_conn: &mut SqliteConnection) -> Result<()> {
    // Table creation is handled by Diesel migrations
    // This function is kept for API compatibility
    Ok(())
}

/// Insert or update hosting configuration
pub fn upsert_hosting(conn: &mut SqliteConnection, hosting: &Hosting) -> Result<i64> {
    let now = chrono::Utc::now().timestamp();

    diesel::sql_query(
        "INSERT INTO hosting (
            domain, status, domain_registrar, domain_registrar_token, domain_registered,
            dns_provider, dns_provider_token, ns_addresses, dns_configured,
            vm_provider, vm_provider_token, vm_instance_id, vm_ip_address,
            vm_ssh_user, vm_ssh_key_path, vm_created,
            service_installed, service_running, created_at, updated_at, error_message
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)
        ON CONFLICT(domain) DO UPDATE SET
            status = excluded.status,
            domain_registrar = excluded.domain_registrar,
            domain_registrar_token = excluded.domain_registrar_token,
            domain_registered = excluded.domain_registered,
            dns_provider = excluded.dns_provider,
            dns_provider_token = excluded.dns_provider_token,
            ns_addresses = excluded.ns_addresses,
            dns_configured = excluded.dns_configured,
            vm_provider = excluded.vm_provider,
            vm_provider_token = excluded.vm_provider_token,
            vm_instance_id = excluded.vm_instance_id,
            vm_ip_address = excluded.vm_ip_address,
            vm_ssh_user = excluded.vm_ssh_user,
            vm_ssh_key_path = excluded.vm_ssh_key_path,
            vm_created = excluded.vm_created,
            service_installed = excluded.service_installed,
            service_running = excluded.service_running,
            updated_at = excluded.updated_at,
            error_message = excluded.error_message",
    )
    .bind::<Text, _>(&hosting.domain)
    .bind::<Text, _>(hosting.status.as_str())
    .bind::<Nullable<Text>, _>(&hosting.domain_registrar)
    .bind::<Nullable<Text>, _>(&hosting.domain_registrar_token)
    .bind::<Integer, _>(if hosting.domain_registered { 1 } else { 0 })
    .bind::<Text, _>(&hosting.dns_provider)
    .bind::<Nullable<Text>, _>(&hosting.dns_provider_token)
    .bind::<Nullable<Text>, _>(&hosting.ns_addresses)
    .bind::<Integer, _>(if hosting.dns_configured { 1 } else { 0 })
    .bind::<Text, _>(&hosting.vm_provider)
    .bind::<Nullable<Text>, _>(&hosting.vm_provider_token)
    .bind::<Nullable<Text>, _>(&hosting.vm_instance_id)
    .bind::<Nullable<Text>, _>(&hosting.vm_ip_address)
    .bind::<Nullable<Text>, _>(&hosting.vm_ssh_user)
    .bind::<Nullable<Text>, _>(&hosting.vm_ssh_key_path)
    .bind::<Integer, _>(if hosting.vm_created { 1 } else { 0 })
    .bind::<Integer, _>(if hosting.service_installed { 1 } else { 0 })
    .bind::<Integer, _>(if hosting.service_running { 1 } else { 0 })
    .bind::<BigInt, _>(hosting.created_at)
    .bind::<BigInt, _>(now)
    .bind::<Nullable<Text>, _>(&hosting.error_message)
    .execute(conn)
    .context("Failed to upsert hosting")?;

    // Get the ID
    #[derive(QueryableByName)]
    struct IdRow {
        #[diesel(sql_type = BigInt)]
        id: i64,
    }

    let rows = diesel::sql_query("SELECT id FROM hosting WHERE domain = ?1")
        .bind::<Text, _>(&hosting.domain)
        .load::<IdRow>(conn)?;

    rows.first()
        .map(|r| r.id)
        .ok_or_else(|| anyhow::anyhow!("Failed to get hosting ID after upsert"))
}

/// Get hosting by domain
pub fn get_hosting_by_domain(conn: &mut SqliteConnection, domain: &str) -> Result<Option<Hosting>> {
    let rows = diesel::sql_query("SELECT * FROM hosting WHERE domain = ?1")
        .bind::<Text, _>(domain)
        .load::<HostingRow>(conn)?;

    Ok(rows.into_iter().next().map(Hosting::from))
}

/// List all hostings
pub fn list_hostings(conn: &mut SqliteConnection) -> Result<Vec<Hosting>> {
    let rows = diesel::sql_query("SELECT * FROM hosting ORDER BY created_at DESC")
        .load::<HostingRow>(conn)?;

    Ok(rows.into_iter().map(Hosting::from).collect())
}

/// Delete hosting by domain
pub fn delete_hosting(conn: &mut SqliteConnection, domain: &str) -> Result<()> {
    diesel::sql_query("DELETE FROM hosting WHERE domain = ?1")
        .bind::<Text, _>(domain)
        .execute(conn)
        .context("Failed to delete hosting")?;

    Ok(())
}

/// Update hosting status
pub fn update_hosting_status(
    conn: &mut SqliteConnection,
    domain: &str,
    status: HostingStatus,
    error_message: Option<String>,
) -> Result<()> {
    let now = chrono::Utc::now().timestamp();

    diesel::sql_query(
        "UPDATE hosting SET status = ?1, error_message = ?2, updated_at = ?3 WHERE domain = ?4",
    )
    .bind::<Text, _>(status.as_str())
    .bind::<Nullable<Text>, _>(&error_message)
    .bind::<BigInt, _>(now)
    .bind::<Text, _>(domain)
    .execute(conn)
    .context("Failed to update hosting status")?;

    Ok(())
}
