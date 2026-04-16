//! Webhook storage model

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Integer, Text};

/// Webhook configuration
#[derive(Debug, Clone)]
pub struct WebhookConfig {
    pub logging_enabled: bool,
}

/// Webhook allow pattern
#[derive(Debug, Clone)]
pub struct WebhookAllowPattern {
    pub id: i64,
    pub pattern: String,
    pub created_at: u64,
}

/// Webhook request log entry
#[derive(Debug, Clone)]
pub struct WebhookRequest {
    pub id: i64,
    pub pattern: String,
    pub path: String,
    pub method: String,
    pub headers: String, // JSON string
    pub body: String,
    pub remote_addr: String,
    pub received_at: u64,
}

/// Initialize webhook tables (migration handled by diesel_migrations)
pub fn init_webhook_tables(_conn: &mut SqliteConnection) -> Result<()> {
    // Table creation is handled by Diesel migrations
    // This function is kept for API compatibility
    Ok(())
}

/// Get webhook configuration
pub fn get_webhook_config(conn: &mut SqliteConnection) -> Result<WebhookConfig> {
    #[derive(QueryableByName)]
    struct ConfigRow {
        #[diesel(sql_type = Integer)]
        logging_enabled: i32,
    }

    let rows = diesel::sql_query("SELECT logging_enabled FROM webhook_config WHERE id = 1")
        .load::<ConfigRow>(conn)
        .context("Failed to query webhook config")?;

    if rows.is_empty() {
        return Ok(WebhookConfig {
            logging_enabled: false,
        });
    }

    Ok(WebhookConfig {
        logging_enabled: rows[0].logging_enabled != 0,
    })
}

/// Update webhook configuration
pub fn update_webhook_config(conn: &mut SqliteConnection, config: &WebhookConfig) -> Result<()> {
    diesel::sql_query("UPDATE webhook_config SET logging_enabled = ?1 WHERE id = 1")
        .bind::<Integer, _>(if config.logging_enabled { 1 } else { 0 })
        .execute(conn)
        .context("Failed to update webhook config")?;

    Ok(())
}

/// Add webhook pattern
pub fn add_webhook_pattern(conn: &mut SqliteConnection, pattern: &str) -> Result<i64> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(std::time::Duration::from_secs(0))
        .as_secs();

    diesel::sql_query("INSERT INTO webhook_allow_patterns (pattern, created_at) VALUES (?1, ?2)")
        .bind::<Text, _>(pattern)
        .bind::<BigInt, _>(now as i64)
        .execute(conn)
        .context("Failed to add webhook pattern")?;

    #[derive(QueryableByName)]
    struct IdRow {
        #[diesel(sql_type = BigInt)]
        id: i64,
    }

    let rows = diesel::sql_query("SELECT id FROM webhook_allow_patterns WHERE pattern = ?1")
        .bind::<Text, _>(pattern)
        .load::<IdRow>(conn)?;

    rows.first()
        .map(|r| r.id)
        .ok_or_else(|| anyhow::anyhow!("Failed to get pattern ID after insert"))
}

/// List webhook patterns
pub fn list_webhook_patterns(conn: &mut SqliteConnection) -> Result<Vec<WebhookAllowPattern>> {
    #[derive(QueryableByName)]
    struct PatternRow {
        #[diesel(sql_type = BigInt)]
        id: i64,
        #[diesel(sql_type = Text)]
        pattern: String,
        #[diesel(sql_type = BigInt)]
        created_at: i64,
    }

    let rows = diesel::sql_query(
        "SELECT id, pattern, created_at
         FROM webhook_allow_patterns
         ORDER BY created_at DESC",
    )
    .load::<PatternRow>(conn)
    .context("Failed to list webhook patterns")?;

    Ok(rows
        .into_iter()
        .map(|row| WebhookAllowPattern {
            id: row.id,
            pattern: row.pattern,
            created_at: row.created_at as u64,
        })
        .collect())
}

/// Delete webhook pattern
pub fn delete_webhook_pattern(conn: &mut SqliteConnection, id: i64) -> Result<()> {
    diesel::sql_query("DELETE FROM webhook_allow_patterns WHERE id = ?1")
        .bind::<BigInt, _>(id)
        .execute(conn)
        .context("Failed to delete webhook pattern")?;

    Ok(())
}

/// Find matching pattern for a path
pub fn find_matching_pattern(conn: &mut SqliteConnection, path: &str) -> Result<Option<String>> {
    #[derive(QueryableByName)]
    struct PatternRow {
        #[diesel(sql_type = Text)]
        pattern: String,
    }

    let rows = diesel::sql_query("SELECT pattern FROM webhook_allow_patterns")
        .load::<PatternRow>(conn)
        .context("Failed to query webhook patterns")?;

    for row in rows {
        if path.starts_with(&row.pattern) {
            return Ok(Some(row.pattern));
        }
    }

    Ok(None)
}

/// Log webhook request
pub fn log_webhook_request(
    conn: &mut SqliteConnection,
    pattern: &str,
    path: &str,
    method: &str,
    headers: &str,
    body: &str,
    remote_addr: &str,
) -> Result<i64> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(std::time::Duration::from_secs(0))
        .as_secs();

    diesel::sql_query(
        "INSERT INTO webhook_requests
         (pattern, path, method, headers, body, remote_addr, received_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )
    .bind::<Text, _>(pattern)
    .bind::<Text, _>(path)
    .bind::<Text, _>(method)
    .bind::<Text, _>(headers)
    .bind::<Text, _>(body)
    .bind::<Text, _>(remote_addr)
    .bind::<BigInt, _>(now as i64)
    .execute(conn)
    .context("Failed to log webhook request")?;

    #[derive(QueryableByName)]
    struct IdRow {
        #[diesel(sql_type = BigInt)]
        id: i64,
    }

    let rows = diesel::sql_query("SELECT last_insert_rowid() as id").load::<IdRow>(conn)?;

    rows.first()
        .map(|r| r.id)
        .ok_or_else(|| anyhow::anyhow!("Failed to get request ID after insert"))
}

/// List webhook requests
pub fn list_webhook_requests(
    conn: &mut SqliteConnection,
    pattern: Option<&str>,
    limit: usize,
) -> Result<Vec<WebhookRequest>> {
    #[derive(QueryableByName)]
    struct RequestRow {
        #[diesel(sql_type = BigInt)]
        id: i64,
        #[diesel(sql_type = Text)]
        pattern: String,
        #[diesel(sql_type = Text)]
        path: String,
        #[diesel(sql_type = Text)]
        method: String,
        #[diesel(sql_type = Text)]
        headers: String,
        #[diesel(sql_type = Text)]
        body: String,
        #[diesel(sql_type = Text)]
        remote_addr: String,
        #[diesel(sql_type = BigInt)]
        received_at: i64,
    }

    let rows = if let Some(p) = pattern {
        diesel::sql_query(
            "SELECT id, pattern, path, method, headers, body, remote_addr, received_at
             FROM webhook_requests
             WHERE pattern = ?1
             ORDER BY received_at DESC
             LIMIT ?2",
        )
        .bind::<Text, _>(&p)
        .bind::<BigInt, _>(limit as i64)
        .load::<RequestRow>(conn)
        .context("Failed to list webhook requests")?
    } else {
        diesel::sql_query(
            "SELECT id, pattern, path, method, headers, body, remote_addr, received_at
             FROM webhook_requests
             ORDER BY received_at DESC
             LIMIT ?1",
        )
        .bind::<BigInt, _>(limit as i64)
        .load::<RequestRow>(conn)
        .context("Failed to list webhook requests")?
    };

    Ok(rows
        .into_iter()
        .map(|row| WebhookRequest {
            id: row.id,
            pattern: row.pattern,
            path: row.path,
            method: row.method,
            headers: row.headers,
            body: row.body,
            remote_addr: row.remote_addr,
            received_at: row.received_at as u64,
        })
        .collect())
}

/// Cleanup old webhook requests
pub fn cleanup_old_webhook_requests(
    conn: &mut SqliteConnection,
    max_age_secs: u64,
) -> Result<usize> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(std::time::Duration::from_secs(0))
        .as_secs();

    let cutoff = now - max_age_secs;

    let deleted = diesel::sql_query("DELETE FROM webhook_requests WHERE received_at < ?1")
        .bind::<BigInt, _>(cutoff as i64)
        .execute(conn)
        .context("Failed to cleanup old webhook requests")?;

    Ok(deleted)
}
