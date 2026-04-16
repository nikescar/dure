//! DNS record storage model

use crate::calc::dns::{DnsRecord, RecordType};
use anyhow::{Context, Result};
use diesel::prelude::*;

/// Initialize DNS cache table (migration handled by diesel_migrations)
pub fn init_dns_table(_conn: &mut SqliteConnection) -> Result<()> {
    // Table creation is handled by Diesel migrations
    // This function is kept for API compatibility
    Ok(())
}

/// Store DNS records in cache
pub fn cache_dns_records(conn: &mut SqliteConnection, records: &[DnsRecord]) -> Result<()> {
    for record in records {
        diesel::sql_query(
            "INSERT OR REPLACE INTO dns_cache (domain, record_type, value, ttl, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )
        .bind::<diesel::sql_types::Text, _>(&record.domain)
        .bind::<diesel::sql_types::Text, _>(&record.record_type.as_str())
        .bind::<diesel::sql_types::Text, _>(&record.value)
        .bind::<diesel::sql_types::BigInt, _>(i64::from(record.ttl))
        .bind::<diesel::sql_types::BigInt, _>(record.timestamp as i64)
        .execute(conn)
        .context("Failed to cache DNS record")?;
    }

    Ok(())
}

/// Retrieve cached DNS records
pub fn get_cached_dns_records(
    conn: &mut SqliteConnection,
    domain: &str,
    record_type: RecordType,
) -> Result<Vec<DnsRecord>> {
    use diesel::sql_types::{BigInt, Text};

    #[derive(QueryableByName)]
    struct DnsCacheRow {
        #[diesel(sql_type = Text)]
        domain: String,
        #[diesel(sql_type = Text)]
        record_type: String,
        #[diesel(sql_type = Text)]
        value: String,
        #[diesel(sql_type = BigInt)]
        ttl: i64,
        #[diesel(sql_type = BigInt)]
        timestamp: i64,
    }

    let rows = diesel::sql_query(
        "SELECT domain, record_type, value, ttl, timestamp FROM dns_cache
         WHERE domain = ?1 AND record_type = ?2",
    )
    .bind::<Text, _>(domain)
    .bind::<Text, _>(record_type.as_str())
    .load::<DnsCacheRow>(conn)
    .context("Failed to query DNS cache")?;

    let mut records = Vec::new();
    for row in rows {
        let record_type = match row.record_type.as_str() {
            "A" => RecordType::A,
            "AAAA" => RecordType::AAAA,
            "TXT" => RecordType::TXT,
            "SSHFP" => RecordType::SSHFP,
            _ => continue,
        };

        records.push(DnsRecord {
            domain: row.domain,
            record_type,
            value: row.value,
            ttl: row.ttl as u32,
            timestamp: row.timestamp as u64,
        });
    }

    // Filter out expired records
    Ok(records.into_iter().filter(|r| r.is_valid()).collect())
}

/// Clear expired DNS cache entries
pub fn clear_expired_dns_cache(conn: &mut SqliteConnection) -> Result<usize> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(std::time::Duration::from_secs(0))
        .as_secs();

    let deleted = diesel::sql_query("DELETE FROM dns_cache WHERE timestamp + ttl < ?1")
        .bind::<diesel::sql_types::BigInt, _>(now as i64)
        .execute(conn)
        .context("Failed to clear expired DNS cache")?;

    Ok(deleted)
}
