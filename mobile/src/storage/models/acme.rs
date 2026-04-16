//! ACME certificate storage model (using lego)
//! Desktop-only module

#![cfg(not(any(target_os = "android", target_arch = "wasm32")))]

use crate::calc::lego::Certificate;
use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Integer, Text};

/// Initialize ACME certificates table (migration handled by diesel_migrations)
pub fn init_acme_table(_conn: &mut SqliteConnection) -> Result<()> {
    // Table creation is handled by Diesel migrations
    // This function is kept for API compatibility
    Ok(())
}

/// Row struct for querying certificates
#[derive(QueryableByName)]
struct CertificateRow {
    #[diesel(sql_type = Text)]
    domain: String,
    #[diesel(sql_type = Text)]
    cert_path: String,
    #[diesel(sql_type = Text)]
    key_path: String,
    #[diesel(sql_type = Text)]
    issuer_path: String,
    #[diesel(sql_type = BigInt)]
    issued_at: i64,
    #[diesel(sql_type = BigInt)]
    expires_at: i64,
    #[diesel(sql_type = Integer)]
    is_valid: i32,
}

impl From<CertificateRow> for Certificate {
    fn from(row: CertificateRow) -> Self {
        Certificate {
            domain: row.domain,
            cert_path: row.cert_path,
            key_path: row.key_path,
            issuer_path: row.issuer_path,
            issued_at: row.issued_at as u64,
            expires_at: row.expires_at as u64,
            is_valid: row.is_valid != 0,
        }
    }
}

/// Store certificate information
pub fn store_certificate(conn: &mut SqliteConnection, cert: &Certificate) -> Result<()> {
    diesel::sql_query(
        "INSERT OR REPLACE INTO acme_certificates
         (domain, cert_path, key_path, issuer_path, issued_at, expires_at, is_valid)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )
    .bind::<Text, _>(&cert.domain)
    .bind::<Text, _>(&cert.cert_path)
    .bind::<Text, _>(&cert.key_path)
    .bind::<Text, _>(&cert.issuer_path)
    .bind::<BigInt, _>(cert.issued_at as i64)
    .bind::<BigInt, _>(cert.expires_at as i64)
    .bind::<Integer, _>(if cert.is_valid { 1 } else { 0 })
    .execute(conn)
    .context("Failed to store certificate")?;

    Ok(())
}

/// Retrieve certificate information by domain
pub fn get_certificate(conn: &mut SqliteConnection, domain: &str) -> Result<Option<Certificate>> {
    let rows = diesel::sql_query(
        "SELECT domain, cert_path, key_path, issuer_path,
                issued_at, expires_at, is_valid
         FROM acme_certificates
         WHERE domain = ?1",
    )
    .bind::<Text, _>(domain)
    .load::<CertificateRow>(conn)
    .context("Failed to query certificate")?;

    Ok(rows.into_iter().next().map(Certificate::from))
}

/// List all certificates
pub fn list_certificates(conn: &mut SqliteConnection) -> Result<Vec<Certificate>> {
    let rows = diesel::sql_query(
        "SELECT domain, cert_path, key_path, issuer_path,
                issued_at, expires_at, is_valid
         FROM acme_certificates
         ORDER BY domain",
    )
    .load::<CertificateRow>(conn)
    .context("Failed to list certificates")?;

    Ok(rows.into_iter().map(Certificate::from).collect())
}

/// Delete certificate record by domain
pub fn delete_certificate(conn: &mut SqliteConnection, domain: &str) -> Result<()> {
    diesel::sql_query("DELETE FROM acme_certificates WHERE domain = ?1")
        .bind::<Text, _>(domain)
        .execute(conn)
        .context("Failed to delete certificate")?;

    Ok(())
}

/// Get certificates that need renewal (within 30 days of expiry)
pub fn get_certificates_needing_renewal(conn: &mut SqliteConnection) -> Result<Vec<Certificate>> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(std::time::Duration::from_secs(0))
        .as_secs();

    const RENEWAL_THRESHOLD: u64 = 30 * 24 * 60 * 60; // 30 days

    let rows = diesel::sql_query(
        "SELECT domain, cert_path, key_path, issuer_path,
                issued_at, expires_at, is_valid
         FROM acme_certificates
         WHERE expires_at <= ?1
         ORDER BY expires_at",
    )
    .bind::<BigInt, _>((now + RENEWAL_THRESHOLD) as i64)
    .load::<CertificateRow>(conn)
    .context("Failed to query certificates needing renewal")?;

    Ok(rows.into_iter().map(Certificate::from).collect())
}
