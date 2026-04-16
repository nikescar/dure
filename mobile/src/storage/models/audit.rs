//! Audit storage model - DB table and CRUD operations
//!
//! Records every user action (CLI/GUI/WASM) for compliance:
//! "Who, What, Where, and When"

use anyhow::{Context, Result};
use diesel::prelude::*;

/// Category of audit event
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditCategory {
    /// Normal user action (command, UI interaction)
    UserAction,
    /// Authentication event (login attempt, success, failure)
    Auth,
    /// Privilege change (identity switch, elevated privilege)
    PrivilegeChange,
    /// System event (startup, config change)
    System,
}

impl AuditCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditCategory::UserAction => "user_action",
            AuditCategory::Auth => "auth",
            AuditCategory::PrivilegeChange => "privilege_change",
            AuditCategory::System => "system",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "auth" => AuditCategory::Auth,
            "privilege_change" => AuditCategory::PrivilegeChange,
            "system" => AuditCategory::System,
            _ => AuditCategory::UserAction,
        }
    }
}

/// Outcome of the audited action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditOutcome {
    Success,
    Failure,
    /// Action was denied (insufficient privilege)
    Denied,
}

impl AuditOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditOutcome::Success => "success",
            AuditOutcome::Failure => "failure",
            AuditOutcome::Denied => "denied",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "failure" => AuditOutcome::Failure,
            "denied" => AuditOutcome::Denied,
            _ => AuditOutcome::Success,
        }
    }
}

/// An immutable audit record
#[derive(Debug, Clone)]
pub struct AuditRecord {
    /// Auto-assigned row id
    pub id: i64,
    /// Unix timestamp (seconds)
    pub timestamp: i64,
    /// Category: user_action | auth | privilege_change | system
    pub category: String,
    /// Actor identity (actor_id or "anonymous")
    pub actor_id: String,
    /// Device identifier
    pub device_id: String,
    /// Surface that originated the action: cli | gui | wasm
    pub surface: String,
    /// The action / command performed (e.g. "audit show", "login")
    pub action: String,
    /// Object or resource affected (e.g. domain name, record id)
    pub object: String,
    /// Outcome: success | failure | denied
    pub outcome: String,
    /// Optional detail message
    pub detail: String,
    /// Remote IP address (empty string when not applicable)
    pub ip_address: String,
}

impl AuditRecord {
    pub fn category(&self) -> AuditCategory {
        AuditCategory::from_str(&self.category)
    }

    pub fn outcome(&self) -> AuditOutcome {
        AuditOutcome::from_str(&self.outcome)
    }
}

/// Builder for inserting a new audit record
pub struct AuditEvent {
    pub category: AuditCategory,
    pub actor_id: String,
    pub device_id: String,
    pub surface: String,
    pub action: String,
    pub object: String,
    pub outcome: AuditOutcome,
    pub detail: String,
    pub ip_address: String,
}

impl AuditEvent {
    pub fn new(action: impl Into<String>) -> Self {
        Self {
            category: AuditCategory::UserAction,
            actor_id: String::new(),
            device_id: String::new(),
            surface: "cli".to_string(),
            action: action.into(),
            object: String::new(),
            outcome: AuditOutcome::Success,
            detail: String::new(),
            ip_address: String::new(),
        }
    }

    pub fn category(mut self, c: AuditCategory) -> Self {
        self.category = c;
        self
    }

    pub fn actor(mut self, id: impl Into<String>) -> Self {
        self.actor_id = id.into();
        self
    }

    pub fn device(mut self, id: impl Into<String>) -> Self {
        self.device_id = id.into();
        self
    }

    pub fn surface(mut self, s: impl Into<String>) -> Self {
        self.surface = s.into();
        self
    }

    pub fn object(mut self, o: impl Into<String>) -> Self {
        self.object = o.into();
        self
    }

    pub fn outcome(mut self, o: AuditOutcome) -> Self {
        self.outcome = o;
        self
    }

    pub fn detail(mut self, d: impl Into<String>) -> Self {
        self.detail = d.into();
        self
    }

    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = ip.into();
        self
    }
}

/// Ensure the audit_records table exists (idempotent).
///
/// Should be called once at startup before any `record()` calls.
pub fn init_audit_table(conn: &mut SqliteConnection) -> Result<()> {
    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS audit_records (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp   INTEGER NOT NULL,
            category    TEXT    NOT NULL DEFAULT 'user_action',
            actor_id    TEXT    NOT NULL DEFAULT '',
            device_id   TEXT    NOT NULL DEFAULT '',
            surface     TEXT    NOT NULL DEFAULT 'cli',
            action      TEXT    NOT NULL,
            object      TEXT    NOT NULL DEFAULT '',
            outcome     TEXT    NOT NULL DEFAULT 'success',
            detail      TEXT    NOT NULL DEFAULT '',
            ip_address  TEXT    NOT NULL DEFAULT ''
        )",
    )
    .execute(conn)
    .context("Failed to create audit_records table")?;

    Ok(())
}

/// Record a new audit event.
pub fn record(conn: &mut SqliteConnection, event: AuditEvent) -> Result<i64> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    diesel::sql_query(
        "INSERT INTO audit_records
         (timestamp, category, actor_id, device_id, surface, action, object, outcome, detail, ip_address)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
    )
    .bind::<diesel::sql_types::BigInt, _>(now)
    .bind::<diesel::sql_types::Text, _>(event.category.as_str())
    .bind::<diesel::sql_types::Text, _>(&event.actor_id)
    .bind::<diesel::sql_types::Text, _>(&event.device_id)
    .bind::<diesel::sql_types::Text, _>(&event.surface)
    .bind::<diesel::sql_types::Text, _>(&event.action)
    .bind::<diesel::sql_types::Text, _>(&event.object)
    .bind::<diesel::sql_types::Text, _>(event.outcome.as_str())
    .bind::<diesel::sql_types::Text, _>(&event.detail)
    .bind::<diesel::sql_types::Text, _>(&event.ip_address)
    .execute(conn)
    .context("Failed to insert audit record")?;

    let id = diesel::sql_query("SELECT last_insert_rowid() AS id")
        .get_result::<LastRowId>(conn)
        .context("Failed to get last insert rowid")?;

    Ok(id.id)
}

#[derive(QueryableByName)]
struct LastRowId {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    id: i64,
}

/// Return the most recent `limit` audit records, newest first.
pub fn list_recent(conn: &mut SqliteConnection, limit: i64) -> Result<Vec<AuditRecord>> {
    load_rows(
        conn,
        "SELECT id, timestamp, category, actor_id, device_id, surface, action,
                object, outcome, detail, ip_address
         FROM audit_records
         ORDER BY id DESC
         LIMIT ?1",
        limit,
    )
}

/// Return all audit records for a given actor, newest first.
pub fn list_by_actor(conn: &mut SqliteConnection, actor_id: &str) -> Result<Vec<AuditRecord>> {
    use diesel::sql_types::Text;

    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        id: i64,
        #[diesel(sql_type = Text)]
        timestamp_text: String,
        #[diesel(sql_type = Text)]
        category: String,
        #[diesel(sql_type = Text)]
        actor_id: String,
        #[diesel(sql_type = Text)]
        device_id: String,
        #[diesel(sql_type = Text)]
        surface: String,
        #[diesel(sql_type = Text)]
        action: String,
        #[diesel(sql_type = Text)]
        object: String,
        #[diesel(sql_type = Text)]
        outcome: String,
        #[diesel(sql_type = Text)]
        detail: String,
        #[diesel(sql_type = Text)]
        ip_address: String,
    }

    let rows = diesel::sql_query(
        "SELECT id,
                CAST(timestamp AS TEXT) AS timestamp_text,
                category, actor_id, device_id, surface, action,
                object, outcome, detail, ip_address
         FROM audit_records
         WHERE actor_id = ?1
         ORDER BY id DESC",
    )
    .bind::<Text, _>(actor_id)
    .load::<Row>(conn)
    .context("Failed to list audit records by actor")?;

    Ok(rows
        .into_iter()
        .map(|r| AuditRecord {
            id: r.id,
            timestamp: r.timestamp_text.parse().unwrap_or(0),
            category: r.category,
            actor_id: r.actor_id,
            device_id: r.device_id,
            surface: r.surface,
            action: r.action,
            object: r.object,
            outcome: r.outcome,
            detail: r.detail,
            ip_address: r.ip_address,
        })
        .collect())
}

/// Delete all audit records.
pub fn clear_all(conn: &mut SqliteConnection) -> Result<usize> {
    let deleted = diesel::sql_query("DELETE FROM audit_records")
        .execute(conn)
        .context("Failed to clear audit records")?;
    Ok(deleted)
}

// ─── internal helper ─────────────────────────────────────────────────────────

fn load_rows(conn: &mut SqliteConnection, sql: &str, limit: i64) -> Result<Vec<AuditRecord>> {
    use diesel::sql_types::{BigInt, Text};

    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = BigInt)]
        id: i64,
        #[diesel(sql_type = BigInt)]
        timestamp: i64,
        #[diesel(sql_type = Text)]
        category: String,
        #[diesel(sql_type = Text)]
        actor_id: String,
        #[diesel(sql_type = Text)]
        device_id: String,
        #[diesel(sql_type = Text)]
        surface: String,
        #[diesel(sql_type = Text)]
        action: String,
        #[diesel(sql_type = Text)]
        object: String,
        #[diesel(sql_type = Text)]
        outcome: String,
        #[diesel(sql_type = Text)]
        detail: String,
        #[diesel(sql_type = Text)]
        ip_address: String,
    }

    let rows = diesel::sql_query(sql)
        .bind::<BigInt, _>(limit)
        .load::<Row>(conn)
        .context("Failed to list audit records")?;

    Ok(rows
        .into_iter()
        .map(|r| AuditRecord {
            id: r.id,
            timestamp: r.timestamp,
            category: r.category,
            actor_id: r.actor_id,
            device_id: r.device_id,
            surface: r.surface,
            action: r.action,
            object: r.object,
            outcome: r.outcome,
            detail: r.detail,
            ip_address: r.ip_address,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::db;

    #[test]
    fn test_audit_record_roundtrip() {
        let mut conn = db::establish_connection();
        init_audit_table(&mut conn).unwrap();

        let event = AuditEvent::new("test action")
            .actor("user-1")
            .device("dev-1")
            .surface("cli")
            .object("domain.com")
            .outcome(AuditOutcome::Success)
            .detail("unit test");

        let id = record(&mut conn, event).unwrap();
        assert!(id > 0);

        let records = list_recent(&mut conn, 10).unwrap();
        assert!(!records.is_empty());
        let r = records.iter().find(|r| r.id == id).unwrap();
        assert_eq!(r.action, "test action");
        assert_eq!(r.actor_id, "user-1");
        assert_eq!(r.outcome, "success");
    }

    #[test]
    fn test_audit_clear() {
        let mut conn = db::establish_connection();
        init_audit_table(&mut conn).unwrap();

        record(&mut conn, AuditEvent::new("clear-test")).unwrap();
        clear_all(&mut conn).unwrap();
        let records = list_recent(&mut conn, 100).unwrap();
        assert!(records.is_empty());
    }
}
