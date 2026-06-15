#![allow(dead_code)]

use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::Path;

pub fn bounded_text(value: &str, max_chars: usize, fallback: &str) -> String {
    let text = value.trim().chars().take(max_chars).collect::<String>();
    if text.is_empty() {
        fallback.to_string()
    } else {
        text
    }
}

pub fn priority(value: &str) -> &str {
    match value {
        "low" | "medium" | "high" | "critical" => value,
        _ => "medium",
    }
}

pub fn issue_status(value: &str) -> &str {
    match value {
        "todo" | "in_progress" | "blocked" | "review" | "validation" | "done" | "archived" => {
            value
        }
        _ => "todo",
    }
}

pub fn open_projection(path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        r#"
        CREATE TABLE issues (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'todo',
            issue_type TEXT NOT NULL DEFAULT 'task',
            priority TEXT NOT NULL DEFAULT 'medium',
            parent_id TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            closed_at TEXT
        );

        CREATE TABLE dependencies (
            blocker_id TEXT NOT NULL,
            blocked_id TEXT NOT NULL,
            PRIMARY KEY (blocker_id, blocked_id)
        );

        CREATE TABLE comments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            issue_id TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            kind TEXT NOT NULL DEFAULT 'note'
        );

        CREATE TABLE labels (
            issue_id TEXT NOT NULL,
            label TEXT NOT NULL,
            PRIMARY KEY (issue_id, label)
        );

        CREATE TABLE records (
            id TEXT PRIMARY KEY,
            kind TEXT NOT NULL,
            title TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'open',
            body TEXT,
            data_json TEXT NOT NULL DEFAULT '{}',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE record_links (
            source_kind TEXT NOT NULL,
            source_id TEXT NOT NULL,
            target_kind TEXT NOT NULL,
            target_id TEXT NOT NULL,
            relation_type TEXT NOT NULL,
            created_at TEXT NOT NULL,
            PRIMARY KEY (source_kind, source_id, target_kind, target_id, relation_type)
        );
        "#,
    )?;
    atelier_sqlite::ProjectionIndex::init_schema(&conn)
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(error.into()))?;
    Ok(conn)
}

pub fn insert_issue(conn: &Connection, id: &str, title: &str, status: &str) -> rusqlite::Result<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT OR IGNORE INTO issues
         (id, title, description, status, issue_type, priority, parent_id, created_at, updated_at, closed_at)
         VALUES (?1, ?2, ?3, ?4, 'task', 'medium', NULL, ?5, ?5, NULL)",
        params![id, title, format!("Description for {title}"), status, now],
    )?;
    Ok(())
}

pub fn add_dependency(
    conn: &Connection,
    blocked_id: &str,
    blocker_id: &str,
) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO dependencies (blocker_id, blocked_id) VALUES (?1, ?2)",
        params![blocker_id, blocked_id],
    )?;
    Ok(())
}
