//! WebSocket server storage model
//! Desktop-only module

#![cfg(not(any(target_os = "android", target_arch = "wasm32")))]

use crate::calc::wss::{WssServerConfig, WssSession};
use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Integer, Text};

/// Initialize WebSocket server tables (migration handled by diesel_migrations)
pub fn init_wss_tables(_conn: &mut SqliteConnection) -> Result<()> {
    // Table creation is handled by Diesel migrations
    // This function is kept for API compatibility
    Ok(())
}

/// Store server configuration
pub fn store_server_config(conn: &mut SqliteConnection, config: &WssServerConfig) -> Result<()> {
    diesel::sql_query(
        "INSERT OR REPLACE INTO wss_servers
         (domain, bind_addr, bind_port, server_id, ping_interval, idle_timeout, max_connections)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )
    .bind::<Text, _>(&config.domain)
    .bind::<Text, _>(&config.bind_addr)
    .bind::<Integer, _>(config.bind_port as i32)
    .bind::<Text, _>(&config.server_id)
    .bind::<Integer, _>(config.ping_interval as i32)
    .bind::<Integer, _>(config.idle_timeout as i32)
    .bind::<Integer, _>(config.max_connections as i32)
    .execute(conn)
    .context("Failed to store server config")?;

    Ok(())
}

/// Get server configuration by domain
pub fn get_server_config(
    conn: &mut SqliteConnection,
    domain: &str,
) -> Result<Option<WssServerConfig>> {
    #[derive(QueryableByName)]
    struct ServerConfigRow {
        #[diesel(sql_type = Text)]
        domain: String,
        #[diesel(sql_type = Text)]
        bind_addr: String,
        #[diesel(sql_type = Integer)]
        bind_port: i32,
        #[diesel(sql_type = Text)]
        server_id: String,
        #[diesel(sql_type = Integer)]
        ping_interval: i32,
        #[diesel(sql_type = Integer)]
        idle_timeout: i32,
        #[diesel(sql_type = Integer)]
        max_connections: i32,
    }

    let rows = diesel::sql_query(
        "SELECT domain, bind_addr, bind_port, server_id, ping_interval, idle_timeout, max_connections
         FROM wss_servers
         WHERE domain = ?1",
    )
    .bind::<Text, _>(domain)
    .load::<ServerConfigRow>(conn)
    .context("Failed to query server config")?;

    Ok(rows.into_iter().next().map(|row| WssServerConfig {
        domain: row.domain,
        bind_addr: row.bind_addr,
        bind_port: row.bind_port as u16,
        server_id: row.server_id,
        ping_interval: row.ping_interval as u64,
        idle_timeout: row.idle_timeout as u64,
        max_connections: row.max_connections as usize,
    }))
}

/// List all server configurations
pub fn list_server_configs(conn: &mut SqliteConnection) -> Result<Vec<WssServerConfig>> {
    #[derive(QueryableByName)]
    struct ServerConfigRow {
        #[diesel(sql_type = Text)]
        domain: String,
        #[diesel(sql_type = Text)]
        bind_addr: String,
        #[diesel(sql_type = Integer)]
        bind_port: i32,
        #[diesel(sql_type = Text)]
        server_id: String,
        #[diesel(sql_type = Integer)]
        ping_interval: i32,
        #[diesel(sql_type = Integer)]
        idle_timeout: i32,
        #[diesel(sql_type = Integer)]
        max_connections: i32,
    }

    let rows = diesel::sql_query(
        "SELECT domain, bind_addr, bind_port, server_id, ping_interval, idle_timeout, max_connections
         FROM wss_servers
         ORDER BY domain",
    )
    .load::<ServerConfigRow>(conn)
    .context("Failed to list server configs")?;

    Ok(rows
        .into_iter()
        .map(|row| WssServerConfig {
            domain: row.domain,
            bind_addr: row.bind_addr,
            bind_port: row.bind_port as u16,
            server_id: row.server_id,
            ping_interval: row.ping_interval as u64,
            idle_timeout: row.idle_timeout as u64,
            max_connections: row.max_connections as usize,
        })
        .collect())
}

/// Delete server configuration
pub fn delete_server_config(conn: &mut SqliteConnection, domain: &str) -> Result<()> {
    diesel::sql_query("DELETE FROM wss_servers WHERE domain = ?1")
        .bind::<Text, _>(domain)
        .execute(conn)
        .context("Failed to delete server config")?;

    Ok(())
}

/// Store session
pub fn store_session(conn: &mut SqliteConnection, session: &WssSession) -> Result<()> {
    diesel::sql_query(
        "INSERT OR REPLACE INTO wss_sessions
         (session_id, domain, connected_at, last_seen, message_count, reconnect_count)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )
    .bind::<Text, _>(&session.session_id)
    .bind::<Text, _>(&session.domain)
    .bind::<BigInt, _>(session.connected_at as i64)
    .bind::<BigInt, _>(session.last_seen as i64)
    .bind::<BigInt, _>(session.message_count as i64)
    .bind::<BigInt, _>(session.reconnect_count as i64)
    .execute(conn)
    .context("Failed to store session")?;

    Ok(())
}

/// Get session by ID
pub fn get_session(conn: &mut SqliteConnection, session_id: &str) -> Result<Option<WssSession>> {
    #[derive(QueryableByName)]
    struct SessionRow {
        #[diesel(sql_type = Text)]
        session_id: String,
        #[diesel(sql_type = Text)]
        domain: String,
        #[diesel(sql_type = BigInt)]
        connected_at: i64,
        #[diesel(sql_type = BigInt)]
        last_seen: i64,
        #[diesel(sql_type = BigInt)]
        message_count: i64,
        #[diesel(sql_type = BigInt)]
        reconnect_count: i64,
    }

    let rows = diesel::sql_query(
        "SELECT session_id, domain, connected_at, last_seen, message_count, reconnect_count
         FROM wss_sessions
         WHERE session_id = ?1",
    )
    .bind::<Text, _>(session_id)
    .load::<SessionRow>(conn)
    .context("Failed to query session")?;

    Ok(rows.into_iter().next().map(|row| WssSession {
        session_id: row.session_id,
        domain: row.domain,
        connected_at: row.connected_at as u64,
        last_seen: row.last_seen as u64,
        message_count: row.message_count as u64,
        reconnect_count: row.reconnect_count as u32,
    }))
}

/// List sessions for a domain
pub fn list_sessions_for_domain(
    conn: &mut SqliteConnection,
    domain: &str,
) -> Result<Vec<WssSession>> {
    #[derive(QueryableByName)]
    struct SessionRow {
        #[diesel(sql_type = Text)]
        session_id: String,
        #[diesel(sql_type = Text)]
        domain: String,
        #[diesel(sql_type = BigInt)]
        connected_at: i64,
        #[diesel(sql_type = BigInt)]
        last_seen: i64,
        #[diesel(sql_type = BigInt)]
        message_count: i64,
        #[diesel(sql_type = BigInt)]
        reconnect_count: i64,
    }

    let rows = diesel::sql_query(
        "SELECT session_id, domain, connected_at, last_seen, message_count, reconnect_count
         FROM wss_sessions
         WHERE domain = ?1
         ORDER BY last_seen DESC",
    )
    .bind::<Text, _>(domain)
    .load::<SessionRow>(conn)
    .context("Failed to list sessions")?;

    Ok(rows
        .into_iter()
        .map(|row| WssSession {
            session_id: row.session_id,
            domain: row.domain,
            connected_at: row.connected_at as u64,
            last_seen: row.last_seen as u64,
            message_count: row.message_count as u64,
            reconnect_count: row.reconnect_count as u32,
        })
        .collect())
}

/// Delete session
pub fn delete_session(conn: &mut SqliteConnection, session_id: &str) -> Result<()> {
    diesel::sql_query("DELETE FROM wss_sessions WHERE session_id = ?1")
        .bind::<Text, _>(session_id)
        .execute(conn)
        .context("Failed to delete session")?;

    Ok(())
}

/// Cleanup old sessions
pub fn cleanup_old_sessions(conn: &mut SqliteConnection, max_age_secs: u64) -> Result<usize> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(std::time::Duration::from_secs(0))
        .as_secs();

    let cutoff = now - max_age_secs;

    let deleted = diesel::sql_query("DELETE FROM wss_sessions WHERE last_seen < ?1")
        .bind::<BigInt, _>(cutoff as i64)
        .execute(conn)
        .context("Failed to cleanup old sessions")?;

    Ok(deleted)
}
