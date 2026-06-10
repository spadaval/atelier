mod comments;
mod dependencies;
mod issues;
mod labels;
mod records;
mod relations;
mod sessions;
mod work;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::Connection;
use std::path::Path;

use crate::models::Issue;
use crate::record_id;
use crate::record_store;

const SCHEMA_VERSION: i32 = 17;

/// Well-known relation types. Unknown types are accepted with a warning;
/// these are the recognized conventions.
pub const WELL_KNOWN_RELATION_TYPES: &[&str] = &[
    "related",    // generic bidirectional link (default, backward compatible)
    "assumption", // "shares underlying assumption" — concept clustering
    "falsifies",  // "this evidence falsifies that assumption"
    "derived",    // "this conclusion was derived from that assumption"
];

pub const WELL_KNOWN_LINK_TYPES: &[&str] = &[
    "advances",
    "has_checkpoint",
    "contributes_to",
    "planned_by",
    "validates",
    "evidenced_by",
    "implements",
    "part_of",
    "supersedes",
    "derived_from",
    "duplicates",
    "related",
];

/// Valid values for issue priority.
pub const VALID_PRIORITIES: &[&str] = &["low", "medium", "high", "critical"];

/// Valid values for issue status.
pub const VALID_STATUSES: &[&str] = &["open", "closed", "archived"];

/// Valid values for canonical issue type.
pub const VALID_ISSUE_TYPES: &[&str] = &[
    "bug",
    "closeout",
    "decision",
    "epic",
    "feature",
    "spike",
    "task",
    "validation",
];

/// Maximum lengths for string inputs.
pub const MAX_TITLE_LEN: usize = 512;
pub const MAX_LABEL_LEN: usize = 128;
pub const MAX_DESCRIPTION_LEN: usize = 64 * 1024; // 64KB
pub const MAX_COMMENT_LEN: usize = 1024 * 1024; // 1MB

/// Validate that a status value is known, returning an error if not.
pub fn validate_status(status: &str) -> Result<()> {
    if VALID_STATUSES.contains(&status) {
        Ok(())
    } else {
        anyhow::bail!(
            "Invalid status '{}'. Valid values: {}",
            status,
            VALID_STATUSES.join(", ")
        )
    }
}

/// Validate that a priority value is known, returning an error if not.
pub fn validate_priority(priority: &str) -> Result<()> {
    if VALID_PRIORITIES.contains(&priority) {
        Ok(())
    } else {
        anyhow::bail!(
            "Invalid priority '{}'. Valid values: {}",
            priority,
            VALID_PRIORITIES.join(", ")
        )
    }
}

/// Validate that an issue type value is known, returning an error if not.
pub fn validate_issue_type(issue_type: &str) -> Result<()> {
    if VALID_ISSUE_TYPES.contains(&issue_type) {
        Ok(())
    } else {
        anyhow::bail!(
            "Invalid issue_type '{}'. Valid values: {}",
            issue_type,
            VALID_ISSUE_TYPES.join(", ")
        )
    }
}

/// Validate relation type: empty strings are rejected; unknown types emit a
/// warning but are still accepted (warn-but-accept pattern).
pub fn validate_relation_type(relation_type: &str) -> Result<()> {
    if relation_type.is_empty() {
        anyhow::bail!("Relation type cannot be empty");
    }
    if !WELL_KNOWN_RELATION_TYPES.contains(&relation_type) {
        tracing::warn!(
            "Unknown relation type '{}'. Known types: {}",
            relation_type,
            WELL_KNOWN_RELATION_TYPES.join(", ")
        );
    }
    Ok(())
}

pub fn validate_record_kind(kind: &str) -> Result<()> {
    record_store::validate_record_kind(kind)
}

pub fn validate_link_type(relation_type: &str) -> Result<()> {
    if relation_type.is_empty() {
        anyhow::bail!("Link type cannot be empty");
    }
    if WELL_KNOWN_LINK_TYPES.contains(&relation_type) {
        Ok(())
    } else {
        anyhow::bail!(
            "Invalid link type '{}'. Valid values: {}",
            relation_type,
            WELL_KNOWN_LINK_TYPES.join(", ")
        )
    }
}

pub struct Database {
    pub(crate) conn: Connection,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path).context("Failed to open database")?;
        let db = Database { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Execute a closure within a database transaction.
    /// If the closure returns Ok, the transaction is committed.
    /// If the closure returns Err, the transaction is rolled back.
    pub fn transaction<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> Result<T>,
    {
        self.conn.execute("BEGIN TRANSACTION", [])?;
        match f() {
            Ok(result) => {
                self.conn.execute("COMMIT", [])?;
                Ok(result)
            }
            Err(e) => {
                if let Err(rollback_err) = self.conn.execute("ROLLBACK", []) {
                    tracing::warn!("ROLLBACK failed: {}", rollback_err);
                }
                Err(e)
            }
        }
    }

    /// Run a migration statement, logging unexpected errors.
    /// Expected errors (duplicate column, table already exists) are logged at debug level.
    fn migrate(&self, sql: &str) {
        if let Err(e) = self.conn.execute(sql, []) {
            let msg = e.to_string();
            if msg.contains("duplicate column") || msg.contains("already exists") {
                tracing::debug!("migration skipped (already applied): {}", msg);
            } else {
                tracing::warn!("migration error ({}): {}", sql.trim(), msg);
            }
        }
    }

    /// Run a batch migration statement, logging unexpected errors.
    fn migrate_batch(&self, sql: &str) {
        if let Err(e) = self.conn.execute_batch(sql) {
            let msg = e.to_string();
            if msg.contains("duplicate column") || msg.contains("already exists") {
                tracing::debug!("migration batch skipped (already applied): {}", msg);
            } else {
                tracing::warn!("migration batch error: {}", msg);
            }
        }
    }

    fn init_schema(&self) -> Result<()> {
        // Check if we need to initialize
        let version: i32 = self
            .conn
            .query_row(
                "SELECT COALESCE(MAX(user_version), 0) FROM pragma_user_version",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        if version < SCHEMA_VERSION {
            self.conn.execute_batch(
                r#"
                -- Core issues table
                CREATE TABLE IF NOT EXISTS issues (
                    id TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    description TEXT,
                    status TEXT NOT NULL DEFAULT 'open',
                    issue_type TEXT NOT NULL DEFAULT 'task',
                    priority TEXT NOT NULL DEFAULT 'medium',
                    parent_id TEXT,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    closed_at TEXT,
                    FOREIGN KEY (parent_id) REFERENCES issues(id) ON DELETE CASCADE
                );

                -- Labels (many-to-many)
                CREATE TABLE IF NOT EXISTS labels (
                    issue_id TEXT NOT NULL,
                    label TEXT NOT NULL,
                    PRIMARY KEY (issue_id, label),
                    FOREIGN KEY (issue_id) REFERENCES issues(id) ON DELETE CASCADE
                );

                -- Dependencies (blocker blocks blocked)
                CREATE TABLE IF NOT EXISTS dependencies (
                    blocker_id TEXT NOT NULL,
                    blocked_id TEXT NOT NULL,
                    PRIMARY KEY (blocker_id, blocked_id),
                    FOREIGN KEY (blocker_id) REFERENCES issues(id) ON DELETE CASCADE,
                    FOREIGN KEY (blocked_id) REFERENCES issues(id) ON DELETE CASCADE
                );

                -- Comments
                CREATE TABLE IF NOT EXISTS comments (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    issue_id TEXT NOT NULL,
                    content TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    FOREIGN KEY (issue_id) REFERENCES issues(id) ON DELETE CASCADE
                );

                -- Sessions (for context preservation)
                CREATE TABLE IF NOT EXISTS sessions (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    started_at TEXT NOT NULL,
                    ended_at TEXT,
                    active_issue_id TEXT,
                    handoff_notes TEXT,
                    FOREIGN KEY (active_issue_id) REFERENCES issues(id) ON DELETE SET NULL
                );

                -- Relations (related issues, bidirectional, typed)
                CREATE TABLE IF NOT EXISTS relations (
                    issue_id_1 TEXT NOT NULL,
                    issue_id_2 TEXT NOT NULL,
                    relation_type TEXT NOT NULL DEFAULT 'related',
                    created_at TEXT NOT NULL,
                    PRIMARY KEY (issue_id_1, issue_id_2, relation_type),
                    FOREIGN KEY (issue_id_1) REFERENCES issues(id) ON DELETE CASCADE,
                    FOREIGN KEY (issue_id_2) REFERENCES issues(id) ON DELETE CASCADE
                );

                -- Indexes
                CREATE INDEX IF NOT EXISTS idx_issues_status ON issues(status);
                CREATE INDEX IF NOT EXISTS idx_issues_priority ON issues(priority);
                CREATE INDEX IF NOT EXISTS idx_labels_issue ON labels(issue_id);
                CREATE INDEX IF NOT EXISTS idx_comments_issue ON comments(issue_id);
                CREATE INDEX IF NOT EXISTS idx_deps_blocker ON dependencies(blocker_id);
                CREATE INDEX IF NOT EXISTS idx_deps_blocked ON dependencies(blocked_id);
                CREATE INDEX IF NOT EXISTS idx_issues_parent ON issues(parent_id);
                CREATE INDEX IF NOT EXISTS idx_relations_1 ON relations(issue_id_1);
                CREATE INDEX IF NOT EXISTS idx_relations_2 ON relations(issue_id_2);
                "#,
            )?;

            // Migration: add parent_id column if upgrading from v1
            self.migrate(
                "ALTER TABLE issues ADD COLUMN parent_id INTEGER REFERENCES issues(id) ON DELETE CASCADE",
            );

            // Migration v7: Recreate sessions table with ON DELETE SET NULL for active_issue_id
            if version < 7 {
                self.migrate_batch(
                    r#"
                    CREATE TABLE IF NOT EXISTS sessions_new (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        started_at TEXT NOT NULL,
                        ended_at TEXT,
                        active_issue_id INTEGER,
                        handoff_notes TEXT,
                        FOREIGN KEY (active_issue_id) REFERENCES issues(id) ON DELETE SET NULL
                    );
                    INSERT OR IGNORE INTO sessions_new SELECT * FROM sessions;
                    DROP TABLE IF EXISTS sessions;
                    ALTER TABLE sessions_new RENAME TO sessions;
                    "#,
                );
            }

            // Migration v8: Add last_action column to sessions table
            if version < 8 {
                self.migrate("ALTER TABLE sessions ADD COLUMN last_action TEXT");
            }

            // Migration v9: Drop leftover sessions_new table from a bug where
            // user_version was always read as 0 (wrong column name in the query),
            // causing the v7 migration to re-run on every open and leave behind
            // a stale sessions_new table.
            if version < 9 {
                self.migrate("DROP TABLE IF EXISTS sessions_new");
            }

            // Migration v10: Add kind column to comments for typed audit trail
            if version < 10 {
                self.migrate("ALTER TABLE comments ADD COLUMN kind TEXT DEFAULT 'note'");
            }

            // Migration v11: Token usage tracking table
            if version < 11 {
                // Token usage tracking was removed from the command surface.
            }

            // Migration v12: Add agent_id column to sessions for multi-agent tracking
            if version < 12 {
                self.migrate("ALTER TABLE sessions ADD COLUMN agent_id TEXT");
            }

            // Migration v13: Add relation_type column to relations for typed links
            if version < 13 {
                self.migrate(
                    "ALTER TABLE relations ADD COLUMN relation_type TEXT NOT NULL DEFAULT 'related'",
                );
            }

            // Migration v14: Add explicit canonical issue_type to issues.
            if version < 14 {
                self.migrate(
                    "ALTER TABLE issues ADD COLUMN issue_type TEXT NOT NULL DEFAULT 'task'",
                );
            }

            if version < 15 {
                self.migrate_issue_ids_to_text()?;
            }

            if version < 16 {
                self.migrate_batch(
                    r#"
                    CREATE TABLE IF NOT EXISTS records (
                        id TEXT PRIMARY KEY,
                        kind TEXT NOT NULL,
                        title TEXT NOT NULL,
                        status TEXT NOT NULL DEFAULT 'open',
                        body TEXT,
                        data_json TEXT NOT NULL DEFAULT '{}',
                        created_at TEXT NOT NULL,
                        updated_at TEXT NOT NULL
                    );
                    CREATE INDEX IF NOT EXISTS idx_records_kind ON records(kind);
                    CREATE INDEX IF NOT EXISTS idx_records_status ON records(status);

                    CREATE TABLE IF NOT EXISTS record_links (
                        source_kind TEXT NOT NULL,
                        source_id TEXT NOT NULL,
                        target_kind TEXT NOT NULL,
                        target_id TEXT NOT NULL,
                        relation_type TEXT NOT NULL,
                        created_at TEXT NOT NULL,
                        PRIMARY KEY (source_kind, source_id, target_kind, target_id, relation_type)
                    );
                    CREATE INDEX IF NOT EXISTS idx_record_links_source ON record_links(source_kind, source_id);
                    CREATE INDEX IF NOT EXISTS idx_record_links_target ON record_links(target_kind, target_id);

                    CREATE TABLE IF NOT EXISTS work_associations (
                        issue_id TEXT PRIMARY KEY,
                        status TEXT NOT NULL,
                        branch TEXT,
                        worktree_path TEXT,
                        started_at TEXT NOT NULL,
                        finished_at TEXT,
                        FOREIGN KEY (issue_id) REFERENCES issues(id) ON DELETE CASCADE
                    );
                    CREATE INDEX IF NOT EXISTS idx_work_associations_status ON work_associations(status);
                    "#,
                );
            }

            // Migration v17: remove inherited runtime/storage tables whose CLI
            // surfaces were deleted.
            if version < 17 {
                self.migrate_batch(
                    r#"
                    DROP INDEX IF EXISTS idx_token_usage_agent;
                    DROP INDEX IF EXISTS idx_token_usage_session;
                    DROP INDEX IF EXISTS idx_token_usage_timestamp;
                    DROP TABLE IF EXISTS token_usage;
                    DROP INDEX IF EXISTS idx_time_entries_issue;
                    DROP TABLE IF EXISTS time_entries;
                    DROP INDEX IF EXISTS idx_milestone_issues_m;
                    DROP INDEX IF EXISTS idx_milestone_issues_i;
                    DROP TABLE IF EXISTS milestone_issues;
                    DROP TABLE IF EXISTS milestones;
                    "#,
                );
            }

            self.conn
                .execute(&format!("PRAGMA user_version = {}", SCHEMA_VERSION), [])?;
        }

        // Enable foreign keys
        self.conn.execute("PRAGMA foreign_keys = ON", [])?;
        self.init_projection_index_schema()?;

        Ok(())
    }

    fn migrate_issue_ids_to_text(&self) -> Result<()> {
        let id_type: String = self
            .conn
            .query_row("PRAGMA table_info(issues)", [], |_| Ok(String::new()))
            .unwrap_or_default();
        let mut stmt = self.conn.prepare("PRAGMA table_info(issues)")?;
        let columns = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(1)?, row.get::<_, String>(2)?))
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        if columns
            .iter()
            .any(|(name, column_type)| name == "id" && column_type.eq_ignore_ascii_case("TEXT"))
        {
            return Ok(());
        }
        drop(id_type);
        drop(stmt);

        let mut mappings = Vec::new();
        let mut stmt = self.conn.prepare("SELECT id FROM issues ORDER BY id")?;
        let ids = stmt
            .query_map([], |row| row.get::<_, i64>(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        for id in ids {
            mappings.push((id, record_id::legacy_issue_id(id)));
        }
        drop(stmt);

        self.conn.execute_batch(
            r#"
            PRAGMA foreign_keys = OFF;

            CREATE TABLE issues_v15 (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'open',
                issue_type TEXT NOT NULL DEFAULT 'task',
                priority TEXT NOT NULL DEFAULT 'medium',
                parent_id TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                closed_at TEXT,
                FOREIGN KEY (parent_id) REFERENCES issues(id) ON DELETE CASCADE
            );
            CREATE TABLE labels_v15 (
                issue_id TEXT NOT NULL,
                label TEXT NOT NULL,
                PRIMARY KEY (issue_id, label),
                FOREIGN KEY (issue_id) REFERENCES issues(id) ON DELETE CASCADE
            );
            CREATE TABLE dependencies_v15 (
                blocker_id TEXT NOT NULL,
                blocked_id TEXT NOT NULL,
                PRIMARY KEY (blocker_id, blocked_id),
                FOREIGN KEY (blocker_id) REFERENCES issues(id) ON DELETE CASCADE,
                FOREIGN KEY (blocked_id) REFERENCES issues(id) ON DELETE CASCADE
            );
            CREATE TABLE comments_v15 (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                issue_id TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                kind TEXT DEFAULT 'note',
                FOREIGN KEY (issue_id) REFERENCES issues(id) ON DELETE CASCADE
            );
            CREATE TABLE sessions_v15 (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                started_at TEXT NOT NULL,
                ended_at TEXT,
                active_issue_id TEXT,
                handoff_notes TEXT,
                last_action TEXT,
                agent_id TEXT,
                FOREIGN KEY (active_issue_id) REFERENCES issues(id) ON DELETE SET NULL
            );
            CREATE TABLE time_entries_v15 (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                issue_id TEXT NOT NULL,
                started_at TEXT NOT NULL,
                ended_at TEXT,
                duration_seconds INTEGER,
                FOREIGN KEY (issue_id) REFERENCES issues(id) ON DELETE CASCADE
            );
            CREATE TABLE relations_v15 (
                issue_id_1 TEXT NOT NULL,
                issue_id_2 TEXT NOT NULL,
                relation_type TEXT NOT NULL DEFAULT 'related',
                created_at TEXT NOT NULL,
                PRIMARY KEY (issue_id_1, issue_id_2, relation_type),
                FOREIGN KEY (issue_id_1) REFERENCES issues(id) ON DELETE CASCADE,
                FOREIGN KEY (issue_id_2) REFERENCES issues(id) ON DELETE CASCADE
            );
            CREATE TABLE milestone_issues_v15 (
                milestone_id INTEGER NOT NULL,
                issue_id TEXT NOT NULL,
                PRIMARY KEY (milestone_id, issue_id),
                FOREIGN KEY (milestone_id) REFERENCES milestones(id) ON DELETE CASCADE,
                FOREIGN KEY (issue_id) REFERENCES issues(id) ON DELETE CASCADE
            );
            "#,
        )?;

        for (old_id, new_id) in &mappings {
            self.conn.execute(
                "INSERT INTO issues_v15
                 SELECT ?2, title, description, status, issue_type, priority, NULL, created_at, updated_at, closed_at
                 FROM issues WHERE id = ?1",
                rusqlite::params![old_id, new_id],
            )?;
        }
        for (old_id, new_id) in &mappings {
            let parent: Option<i64> = self
                .conn
                .query_row(
                    "SELECT parent_id FROM issues WHERE id = ?1",
                    rusqlite::params![old_id],
                    |row| row.get(0),
                )
                .ok()
                .flatten();
            if let Some(parent) = parent {
                self.conn.execute(
                    "UPDATE issues_v15 SET parent_id = ?1 WHERE id = ?2",
                    rusqlite::params![record_id::legacy_issue_id(parent), new_id],
                )?;
            }
        }
        for (old_id, new_id) in &mappings {
            self.conn.execute(
                "INSERT INTO labels_v15 SELECT ?2, label FROM labels WHERE issue_id = ?1",
                rusqlite::params![old_id, new_id],
            )?;
            self.conn.execute(
                "INSERT INTO comments_v15 SELECT id, ?2, content, created_at, kind FROM comments WHERE issue_id = ?1",
                rusqlite::params![old_id, new_id],
            )?;
            self.conn.execute(
                "INSERT INTO time_entries_v15 SELECT id, ?2, started_at, ended_at, duration_seconds FROM time_entries WHERE issue_id = ?1",
                rusqlite::params![old_id, new_id],
            )?;
            self.conn.execute(
                "INSERT INTO milestone_issues_v15 SELECT milestone_id, ?2 FROM milestone_issues WHERE issue_id = ?1",
                rusqlite::params![old_id, new_id],
            )?;
        }
        self.conn.execute(
            "INSERT INTO dependencies_v15
             SELECT printf('atelier-%04s', lower(hex(blocker_id))), printf('atelier-%04s', lower(hex(blocked_id)))
             FROM dependencies WHERE 0",
            [],
        )?;
        for (old_blocker, new_blocker) in &mappings {
            for (old_blocked, new_blocked) in &mappings {
                self.conn.execute(
                    "INSERT OR IGNORE INTO dependencies_v15
                     SELECT ?2, ?4 FROM dependencies WHERE blocker_id = ?1 AND blocked_id = ?3",
                    rusqlite::params![old_blocker, new_blocker, old_blocked, new_blocked],
                )?;
                self.conn.execute(
                    "INSERT OR IGNORE INTO relations_v15
                     SELECT ?2, ?4, relation_type, created_at FROM relations WHERE issue_id_1 = ?1 AND issue_id_2 = ?3",
                    rusqlite::params![old_blocker, new_blocker, old_blocked, new_blocked],
                )?;
            }
        }
        for (old_id, new_id) in &mappings {
            self.conn.execute(
                "INSERT INTO sessions_v15
                 SELECT id, started_at, ended_at, ?2, handoff_notes, last_action, agent_id
                 FROM sessions WHERE active_issue_id = ?1",
                rusqlite::params![old_id, new_id],
            )?;
        }
        self.conn.execute(
            "INSERT INTO sessions_v15
             SELECT id, started_at, ended_at, NULL, handoff_notes, last_action, agent_id
             FROM sessions WHERE active_issue_id IS NULL",
            [],
        )?;

        self.conn.execute_batch(
            r#"
            DROP TABLE labels;
            DROP TABLE dependencies;
            DROP TABLE comments;
            DROP TABLE sessions;
            DROP TABLE time_entries;
            DROP TABLE relations;
            DROP TABLE milestone_issues;
            DROP TABLE issues;

            ALTER TABLE issues_v15 RENAME TO issues;
            ALTER TABLE labels_v15 RENAME TO labels;
            ALTER TABLE dependencies_v15 RENAME TO dependencies;
            ALTER TABLE comments_v15 RENAME TO comments;
            ALTER TABLE sessions_v15 RENAME TO sessions;
            ALTER TABLE time_entries_v15 RENAME TO time_entries;
            ALTER TABLE relations_v15 RENAME TO relations;
            ALTER TABLE milestone_issues_v15 RENAME TO milestone_issues;

            CREATE INDEX IF NOT EXISTS idx_issues_status ON issues(status);
            CREATE INDEX IF NOT EXISTS idx_issues_priority ON issues(priority);
            CREATE INDEX IF NOT EXISTS idx_labels_issue ON labels(issue_id);
            CREATE INDEX IF NOT EXISTS idx_comments_issue ON comments(issue_id);
            CREATE INDEX IF NOT EXISTS idx_deps_blocker ON dependencies(blocker_id);
            CREATE INDEX IF NOT EXISTS idx_deps_blocked ON dependencies(blocked_id);
            CREATE INDEX IF NOT EXISTS idx_issues_parent ON issues(parent_id);
            CREATE INDEX IF NOT EXISTS idx_time_entries_issue ON time_entries(issue_id);
            CREATE INDEX IF NOT EXISTS idx_relations_1 ON relations(issue_id_1);
            CREATE INDEX IF NOT EXISTS idx_relations_2 ON relations(issue_id_2);
            CREATE INDEX IF NOT EXISTS idx_milestone_issues_i ON milestone_issues(issue_id);

            PRAGMA foreign_keys = ON;
            "#,
        )?;
        Ok(())
    }
}

pub(crate) fn parse_datetime(s: String) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

/// Maps a database row to an Issue struct.
/// Expects columns in order: id, title, description, status, issue_type, priority, parent_id, created_at, updated_at, closed_at
pub(crate) fn issue_from_row(row: &rusqlite::Row) -> rusqlite::Result<Issue> {
    Ok(Issue {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        status: row.get(3)?,
        issue_type: row.get(4)?,
        priority: row.get(5)?,
        parent_id: row.get(6)?,
        created_at: parse_datetime(row.get::<_, String>(7)?),
        updated_at: parse_datetime(row.get::<_, String>(8)?),
        closed_at: row.get::<_, Option<String>>(9)?.map(parse_datetime),
    })
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod proptest_tests;
