//! Webhook command implementation

use crate::calc::audit;
use crate::calc::db;
use anyhow::Result;
use diesel::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::storage::models::{session, webhook};

/// Execute webhook status command
pub fn execute_webhook_status() -> Result<()> {
    let mut conn = db::establish_connection();

    webhook::init_webhook_tables(&mut conn)?;

    let config = webhook::get_webhook_config(&mut conn)?;

    println!("Webhook Configuration:");
    println!("  Logging enabled: {}", config.logging_enabled);

    let patterns = webhook::list_webhook_patterns(&mut conn)?;
    println!("  Allow patterns: {}", patterns.len());

    let requests = webhook::list_webhook_requests(&mut conn, None, 1)?;
    println!("  Total requests logged: {}", requests.len());

    Ok(())
}

/// Execute webhook enable-logging command
pub fn execute_webhook_enable_logging() -> Result<()> {
    let mut conn = db::establish_connection();

    webhook::init_webhook_tables(&mut conn)?;

    let config = webhook::WebhookConfig {
        logging_enabled: true,
    };
    webhook::update_webhook_config(&mut conn, &config)?;
    println!("✓ Webhook logging enabled");

    Ok(())
}

/// Execute webhook disable-logging command
pub fn execute_webhook_disable_logging() -> Result<()> {
    let mut conn = db::establish_connection();

    webhook::init_webhook_tables(&mut conn)?;

    let config = webhook::WebhookConfig {
        logging_enabled: false,
    };
    webhook::update_webhook_config(&mut conn, &config)?;
    println!("✓ Webhook logging disabled");

    Ok(())
}

/// Execute webhook add-pattern command
pub fn execute_webhook_add_pattern(pattern: String) -> Result<()> {
    let mut conn = db::establish_connection();

    webhook::init_webhook_tables(&mut conn)?;

    let id = webhook::add_webhook_pattern(&mut conn, &pattern)?;

    // Record audit event
    let _ = audit::push_cli("system", "cli", "webhook add-pattern", &pattern);

    println!("✓ Added webhook pattern: {} (ID: {})", pattern, id);

    Ok(())
}

/// Execute webhook list-patterns command
pub fn execute_webhook_list_patterns() -> Result<()> {
    let mut conn = db::establish_connection();

    webhook::init_webhook_tables(&mut conn)?;

    let patterns = webhook::list_webhook_patterns(&mut conn)?;

    if patterns.is_empty() {
        println!("No webhook patterns configured");
        return Ok(());
    }

    println!("Webhook Allow Patterns:");
    println!("{:<6} {:<30} {:<20}", "ID", "Pattern", "Created");
    println!("{}", "-".repeat(56));

    for pattern in patterns {
        let created = format_timestamp(pattern.created_at);
        println!("{:<6} {:<30} {:<20}", pattern.id, pattern.pattern, created);
    }

    Ok(())
}

/// Execute webhook delete-pattern command
pub fn execute_webhook_delete_pattern(id: i64) -> Result<()> {
    let mut conn = db::establish_connection();

    webhook::init_webhook_tables(&mut conn)?;

    webhook::delete_webhook_pattern(&mut conn, id)?;

    // Record audit event
    let _ = audit::push_cli("system", "cli", "webhook delete-pattern", &id.to_string());

    println!("✓ Deleted webhook pattern ID: {}", id);

    Ok(())
}

/// Execute webhook list-requests command
pub fn execute_webhook_list_requests(limit: Option<usize>, pattern: Option<String>) -> Result<()> {
    let mut conn = db::establish_connection();

    webhook::init_webhook_tables(&mut conn)?;

    let requests =
        webhook::list_webhook_requests(&mut conn, pattern.as_deref(), limit.unwrap_or(10))?;

    if requests.is_empty() {
        println!("No webhook requests logged");
        return Ok(());
    }

    println!("Recent Webhook Requests:");
    println!(
        "{:<6} {:<20} {:<30} {:<15} {:<20}",
        "ID", "Pattern", "Path", "Remote", "Received"
    );
    println!("{}", "-".repeat(91));

    for req in requests {
        let received = format_timestamp(req.received_at);
        let path = truncate(&req.path, 28);
        let pattern_str = truncate(&req.pattern, 18);
        let remote = truncate(&req.remote_addr, 13);

        println!(
            "{:<6} {:<20} {:<30} {:<15} {:<20}",
            req.id, pattern_str, path, remote, received
        );
    }

    Ok(())
}

/// Execute webhook list-sessions command
pub fn execute_webhook_list_sessions(session_type: Option<String>) -> Result<()> {
    let mut conn = db::establish_connection();

    session::init_session_tables(&mut conn)?;

    let sessions = match session_type.as_deref() {
        Some(t) => session::list_sessions_by_type(&mut conn, t)?,
        None => session::list_active_sessions(&mut conn, 3600)?,
    };

    if sessions.is_empty() {
        println!("No sessions found");
        return Ok(());
    }

    println!("Sessions:");
    println!(
        "{:<30} {:<15} {:<6} {:<10} {:<20}",
        "Session ID", "Domain", "Type", "Requests", "Last Seen"
    );
    println!("{}", "-".repeat(81));

    for sess in sessions {
        let last_seen = format_timestamp(sess.last_seen);
        let session_id = truncate(&sess.session_id, 28);
        let domain = truncate(&sess.domain, 13);

        println!(
            "{:<30} {:<15} {:<6} {:<10} {:<20}",
            session_id, domain, sess.session_type, sess.request_count, last_seen
        );
    }

    Ok(())
}

/// Execute webhook cleanup command
pub fn execute_webhook_cleanup(max_age: Option<u64>) -> Result<()> {
    let mut conn = db::establish_connection();

    webhook::init_webhook_tables(&mut conn)?;
    session::init_session_tables(&mut conn)?;

    let max_age_secs = max_age.unwrap_or(86400); // Default: 24 hours

    let deleted_requests = webhook::cleanup_old_webhook_requests(&mut conn, max_age_secs)?;
    let deleted_sessions = session::cleanup_old_sessions(&mut conn, max_age_secs)?;

    println!("✓ Cleanup complete:");
    println!("  Deleted {} old webhook requests", deleted_requests);
    println!("  Deleted {} old sessions", deleted_sessions);

    Ok(())
}

/// Format timestamp as relative time
fn format_timestamp(timestamp: u64) -> String {
    let time = UNIX_EPOCH + Duration::from_secs(timestamp);
    let now = SystemTime::now();

    if let Ok(duration) = now.duration_since(time) {
        let secs = duration.as_secs();
        if secs < 60 {
            format!("{}s ago", secs)
        } else if secs < 3600 {
            format!("{}m ago", secs / 60)
        } else if secs < 86400 {
            format!("{}h ago", secs / 3600)
        } else {
            format!("{}d ago", secs / 86400)
        }
    } else {
        "in future".to_string()
    }
}

/// Truncate string to max length
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
