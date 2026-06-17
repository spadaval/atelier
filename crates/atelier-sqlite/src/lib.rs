mod comments;
mod dependencies;
mod issues;
mod labels;
pub mod projection_index;
mod record_id;
mod records;
mod relations;
mod sessions;
mod work;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use std::fs;
use std::path::{Path, PathBuf};

use atelier_core::Issue;
use atelier_records as record_store;

const SCHEMA_VERSION: i32 = 20;

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
    "blocked_by",
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

/// SQLite tables rebuilt from canonical Markdown records.
pub const CANONICAL_PROJECTION_TABLES: &[&str] = &[
    "issues",
    "labels",
    "dependencies",
    "relations",
    "records",
    "record_labels",
    "record_links",
    "evidence",
    "plans",
    "milestones",
    "projection_sources",
];

/// Transitional tables retained for command compatibility during migration.
pub const COMPATIBILITY_TABLES: &[&str] = &[];

/// Valid values for issue priority.
pub const VALID_PRIORITIES: &[&str] = &["low", "medium", "high", "critical"];

/// Valid values for canonical issue type.
pub const VALID_ISSUE_TYPES: &[&str] = &["bug", "epic", "feature", "spike", "task", "validation"];

/// Maximum lengths for string inputs.
pub const MAX_TITLE_LEN: usize = 512;
pub const MAX_LABEL_LEN: usize = 128;
pub const MAX_DESCRIPTION_LEN: usize = 64 * 1024; // 64KB
pub const MAX_COMMENT_LEN: usize = 1024 * 1024; // 1MB

/// Validate that a status value is known, returning an error if not.
pub fn validate_status(status: &str) -> Result<()> {
    let mut chars = status.chars();
    if matches!(chars.next(), Some(first) if first.is_ascii_lowercase())
        && chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        Ok(())
    } else {
        anyhow::bail!(
            "Invalid status '{}'. Status values must match ^[a-z][a-z0-9_]*$",
            status,
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

pub fn validate_relationship_type(relation_type: &str) -> Result<()> {
    if WELL_KNOWN_LINK_TYPES.contains(&relation_type) {
        Ok(())
    } else {
        validate_relation_type(relation_type)
    }
}

pub struct Database {
    pub conn: Connection,
    pub(crate) path: PathBuf,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create database directory {}", parent.display())
            })?;
        }
        let conn = Connection::open(path).context("Failed to open database")?;
        let db = Database {
            conn,
            path: path.to_path_buf(),
        };
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
        let version = self.current_schema_version();

        if version < SCHEMA_VERSION {
            self.install_core_schema()?;
            self.apply_schema_migrations(version)?;
            self.set_schema_version()?;
        }

        // Enable foreign keys
        self.conn.execute("PRAGMA foreign_keys = ON", [])?;
        self.init_projection_index_schema()?;

        Ok(())
    }

    fn current_schema_version(&self) -> i32 {
        self.conn
            .query_row(
                "SELECT COALESCE(MAX(user_version), 0) FROM pragma_user_version",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0)
    }

    fn install_core_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            -- Core issues table
            CREATE TABLE IF NOT EXISTS issues (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'todo',
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
            CREATE INDEX IF NOT EXISTS idx_deps_blocker ON dependencies(blocker_id);
            CREATE INDEX IF NOT EXISTS idx_deps_blocked ON dependencies(blocked_id);
            CREATE INDEX IF NOT EXISTS idx_issues_parent ON issues(parent_id);
            CREATE INDEX IF NOT EXISTS idx_relations_1 ON relations(issue_id_1);
            CREATE INDEX IF NOT EXISTS idx_relations_2 ON relations(issue_id_2);
            "#,
        )?;
        Ok(())
    }

    fn apply_schema_migrations(&self, version: i32) -> Result<()> {
        self.migrate(
            "ALTER TABLE issues ADD COLUMN parent_id INTEGER REFERENCES issues(id) ON DELETE CASCADE",
        );
        self.migrate_typed_issue_columns(version)?;
        self.migrate_first_class_record_tables(version);
        self.drop_removed_runtime_tables(version);
        self.drop_local_only_tables(version);
        self.drop_runtime_metadata(version);
        self.rebuild_projection_index_schema(version);
        Ok(())
    }

    fn migrate_typed_issue_columns(&self, version: i32) -> Result<()> {
        if version < 13 {
            self.migrate(
                "ALTER TABLE relations ADD COLUMN relation_type TEXT NOT NULL DEFAULT 'related'",
            );
        }
        if version < 14 {
            self.migrate("ALTER TABLE issues ADD COLUMN issue_type TEXT NOT NULL DEFAULT 'task'");
        }
        if version < 15 {
            self.migrate_issue_ids_to_text()?;
        }
        Ok(())
    }

    fn migrate_first_class_record_tables(&self, version: i32) {
        if version < 16 {
            self.migrate_batch(
                r#"
                CREATE TABLE IF NOT EXISTS records (
                    id TEXT PRIMARY KEY,
                    kind TEXT NOT NULL,
                    title TEXT NOT NULL,
                    status TEXT NOT NULL DEFAULT 'open',
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    source_path TEXT NOT NULL DEFAULT ''
                );
                CREATE INDEX IF NOT EXISTS idx_records_kind ON records(kind);
                CREATE INDEX IF NOT EXISTS idx_records_status ON records(status);
                CREATE INDEX IF NOT EXISTS idx_records_source_path ON records(source_path);

                CREATE TABLE IF NOT EXISTS record_labels (
                    kind TEXT NOT NULL,
                    id TEXT NOT NULL,
                    label TEXT NOT NULL,
                    PRIMARY KEY (kind, id, label)
                );

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

                CREATE TABLE IF NOT EXISTS evidence (
                    id TEXT PRIMARY KEY,
                    evidence_type TEXT,
                    captured_at TEXT,
                    command TEXT,
                    path TEXT,
                    uri TEXT,
                    proof_scope TEXT,
                    agent_identity TEXT,
                    independence_level TEXT,
                    exit_code INTEGER,
                    exit_status TEXT,
                    success INTEGER,
                    spawn_error TEXT
                );
                CREATE TABLE IF NOT EXISTS plans (
                    id TEXT PRIMARY KEY,
                    revision INTEGER,
                    owner TEXT
                );
                CREATE TABLE IF NOT EXISTS milestones (
                    id TEXT PRIMARY KEY,
                    desired_state TEXT
                );
                "#,
            );
        }
    }

    fn drop_removed_runtime_tables(&self, version: i32) {
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
    }

    fn drop_local_only_tables(&self, version: i32) {
        if version < 18 {
            self.migrate_batch(
                r#"
                DROP INDEX IF EXISTS idx_comments_issue;
                DROP INDEX IF EXISTS idx_work_associations_status;
                DROP TABLE IF EXISTS comments;
                DROP TABLE IF EXISTS sessions;
                DROP TABLE IF EXISTS sessions_new;
                DROP TABLE IF EXISTS work_associations;
                "#,
            );
        }
    }

    fn drop_runtime_metadata(&self, version: i32) {
        if version < 19 {
            self.migrate("DROP TABLE IF EXISTS runtime_metadata");
        }
    }

    fn rebuild_projection_index_schema(&self, version: i32) {
        if version < 20 {
            self.migrate_batch(
                r#"
                DROP TABLE IF EXISTS projection_index_sources;
                DROP TABLE IF EXISTS projection_sources;
                DROP TABLE IF EXISTS records;
                DROP TABLE IF EXISTS record_labels;
                DROP TABLE IF EXISTS record_links;
                DROP TABLE IF EXISTS evidence;
                DROP TABLE IF EXISTS plans;
                DROP TABLE IF EXISTS milestones;

                CREATE TABLE records (
                    kind TEXT NOT NULL,
                    id TEXT NOT NULL,
                    title TEXT NOT NULL,
                    status TEXT NOT NULL DEFAULT 'open',
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    source_path TEXT NOT NULL,
                    PRIMARY KEY (kind, id)
                );
                CREATE INDEX IF NOT EXISTS idx_records_kind ON records(kind);
                CREATE INDEX IF NOT EXISTS idx_records_status ON records(status);
                CREATE INDEX IF NOT EXISTS idx_records_source_path ON records(source_path);

                CREATE TABLE record_labels (
                    kind TEXT NOT NULL,
                    id TEXT NOT NULL,
                    label TEXT NOT NULL,
                    PRIMARY KEY (kind, id, label)
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
                CREATE INDEX IF NOT EXISTS idx_record_links_source ON record_links(source_kind, source_id);
                CREATE INDEX IF NOT EXISTS idx_record_links_target ON record_links(target_kind, target_id);

                CREATE TABLE evidence (
                    id TEXT PRIMARY KEY,
                    evidence_type TEXT,
                    captured_at TEXT,
                    command TEXT,
                    path TEXT,
                    uri TEXT,
                    proof_scope TEXT,
                    agent_identity TEXT,
                    independence_level TEXT,
                    exit_code INTEGER,
                    exit_status TEXT,
                    success INTEGER,
                    spawn_error TEXT
                );
                CREATE TABLE plans (
                    id TEXT PRIMARY KEY,
                    revision INTEGER,
                    owner TEXT
                );
                CREATE TABLE milestones (
                    id TEXT PRIMARY KEY,
                    desired_state TEXT
                );
                "#,
            );
        }
    }

    fn set_schema_version(&self) -> Result<()> {
        self.conn
            .execute(&format!("PRAGMA user_version = {}", SCHEMA_VERSION), [])?;
        Ok(())
    }

    fn migrate_issue_ids_to_text(&self) -> Result<()> {
        if self.issue_id_column_is_text()? {
            return Ok(());
        }

        let mappings = self.legacy_issue_id_mappings()?;
        self.create_v15_text_id_tables()?;
        self.copy_v15_issue_rows(&mappings)?;
        self.copy_v15_issue_children(&mappings)?;
        self.copy_v15_issue_scoped_rows(&mappings)?;
        self.copy_v15_graph_edges(&mappings)?;
        self.replace_v15_tables()?;
        Ok(())
    }

    fn issue_id_column_is_text(&self) -> Result<bool> {
        let mut stmt = self.conn.prepare("PRAGMA table_info(issues)")?;
        let columns = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(1)?, row.get::<_, String>(2)?))
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(columns
            .iter()
            .any(|(name, column_type)| name == "id" && column_type.eq_ignore_ascii_case("TEXT")))
    }

    fn legacy_issue_id_mappings(&self) -> Result<Vec<(i64, String)>> {
        let mut mappings = Vec::new();
        let mut stmt = self.conn.prepare("SELECT id FROM issues ORDER BY id")?;
        let ids = stmt
            .query_map([], |row| row.get::<_, i64>(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        for id in ids {
            mappings.push((id, record_id::legacy_issue_id(id)));
        }
        Ok(mappings)
    }

    fn create_v15_text_id_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            PRAGMA foreign_keys = OFF;

            CREATE TABLE issues_v15 (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'todo',
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
        Ok(())
    }

    fn copy_v15_issue_rows(&self, mappings: &[(i64, String)]) -> Result<()> {
        for (old_id, new_id) in mappings {
            self.conn.execute(
                "INSERT INTO issues_v15
                 SELECT ?2, title, description, status, issue_type, priority, NULL, created_at, updated_at, closed_at
                 FROM issues WHERE id = ?1",
                rusqlite::params![old_id, new_id],
            )?;
        }
        Ok(())
    }

    fn copy_v15_issue_children(&self, mappings: &[(i64, String)]) -> Result<()> {
        for (old_id, new_id) in mappings {
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
        Ok(())
    }

    fn copy_v15_issue_scoped_rows(&self, mappings: &[(i64, String)]) -> Result<()> {
        for (old_id, new_id) in mappings {
            self.conn.execute(
                "INSERT INTO labels_v15 SELECT ?2, label FROM labels WHERE issue_id = ?1",
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
        Ok(())
    }

    fn copy_v15_graph_edges(&self, mappings: &[(i64, String)]) -> Result<()> {
        self.conn.execute(
            "INSERT INTO dependencies_v15
             SELECT printf('atelier-%04s', lower(hex(blocker_id))), printf('atelier-%04s', lower(hex(blocked_id)))
             FROM dependencies WHERE 0",
            [],
        )?;
        for (old_blocker, new_blocker) in mappings {
            for (old_blocked, new_blocked) in mappings {
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
        Ok(())
    }

    fn replace_v15_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            DROP TABLE labels;
            DROP TABLE dependencies;
            DROP TABLE IF EXISTS comments;
            DROP TABLE IF EXISTS sessions;
            DROP TABLE time_entries;
            DROP TABLE relations;
            DROP TABLE milestone_issues;
            DROP TABLE issues;

            ALTER TABLE issues_v15 RENAME TO issues;
            ALTER TABLE labels_v15 RENAME TO labels;
            ALTER TABLE dependencies_v15 RENAME TO dependencies;
            ALTER TABLE time_entries_v15 RENAME TO time_entries;
            ALTER TABLE relations_v15 RENAME TO relations;
            ALTER TABLE milestone_issues_v15 RENAME TO milestone_issues;

            CREATE INDEX IF NOT EXISTS idx_issues_status ON issues(status);
            CREATE INDEX IF NOT EXISTS idx_issues_priority ON issues(priority);
            CREATE INDEX IF NOT EXISTS idx_labels_issue ON labels(issue_id);
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
/// Issue row stored in the rebuildable SQLite projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionIssue {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
}

impl ProjectionIssue {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        description: Option<String>,
        priority: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description,
            status: "todo".to_string(),
            priority: priority.into(),
        }
    }
}

/// Rebuildable projection database API owned by `atelier-sqlite`.
pub struct ProjectionIndex {
    conn: Connection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionSource {
    pub path: PathBuf,
    pub modified_unix_millis: i64,
    pub len: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Freshness {
    Current,
    MissingSource(PathBuf),
    Modified(PathBuf),
    Untracked(PathBuf),
}

impl ProjectionIndex {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("failed to create projection directory {}", parent.display())
            })?;
        }

        let conn = Connection::open(path).context("failed to open projection database")?;
        let index = Self { conn };
        index.init_schema()?;
        Ok(index)
    }

    pub fn insert_issue(&self, issue: &ProjectionIssue) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO issues (id, title, description, status, priority)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                issue.id,
                issue.title,
                issue.description,
                issue.status,
                issue.priority
            ],
        )?;
        Ok(())
    }

    pub fn get_issue(&self, id: &str) -> Result<Option<ProjectionIssue>> {
        self.conn
            .query_row(
                "SELECT id, title, description, status, priority FROM issues WHERE id = ?1",
                [id],
                |row| {
                    Ok(ProjectionIssue {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        description: row.get(2)?,
                        status: row.get(3)?,
                        priority: row.get(4)?,
                    })
                },
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn list_issues(
        &self,
        status: Option<&str>,
        priority: Option<&str>,
    ) -> Result<Vec<ProjectionIssue>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, status, priority FROM issues ORDER BY id ASC",
        )?;
        let issues = stmt
            .query_map([], |row| {
                Ok(ProjectionIssue {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    status: row.get(3)?,
                    priority: row.get(4)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(issues
            .into_iter()
            .filter(|issue| status.is_none_or(|wanted| wanted == "all" || issue.status == wanted))
            .filter(|issue| priority.is_none_or(|wanted| issue.priority == wanted))
            .collect())
    }

    pub fn add_dependency(&self, blocked_id: &str, blocker_id: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO dependencies (blocked_id, blocker_id) VALUES (?1, ?2)",
            params![blocked_id, blocker_id],
        )?;
        Ok(())
    }

    pub fn get_blockers(&self, blocked_id: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT blocker_id FROM dependencies WHERE blocked_id = ?1 ORDER BY blocker_id ASC",
        )?;
        let blockers = stmt
            .query_map([blocked_id], |row| row.get(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(blockers)
    }

    pub fn search_issues(&self, query: &str) -> Result<Vec<ProjectionIssue>> {
        let pattern = format!("%{}%", query.replace('%', "\\%").replace('_', "\\_"));
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, status, priority
             FROM issues
             WHERE title LIKE ?1 ESCAPE '\\' COLLATE NOCASE
                OR description LIKE ?1 ESCAPE '\\' COLLATE NOCASE
             ORDER BY id ASC",
        )?;
        let issues = stmt
            .query_map([pattern], |row| {
                Ok(ProjectionIssue {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    status: row.get(3)?,
                    priority: row.get(4)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(issues)
    }

    pub fn list_blocked_issues(&self) -> Result<Vec<ProjectionIssue>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT i.id, i.title, i.description, i.status, i.priority
             FROM issues i
             JOIN dependencies d ON d.blocked_id = i.id
             JOIN issues blocker ON blocker.id = d.blocker_id
             WHERE i.status != 'done' AND blocker.status != 'done'
             ORDER BY i.id ASC",
        )?;
        let issues = stmt
            .query_map([], |row| {
                Ok(ProjectionIssue {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    status: row.get(3)?,
                    priority: row.get(4)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(issues)
    }

    pub fn list_ready_issues(&self) -> Result<Vec<ProjectionIssue>> {
        let blocked: std::collections::HashSet<String> = self
            .list_blocked_issues()?
            .into_iter()
            .map(|issue| issue.id)
            .collect();
        Ok(self
            .list_issues(Some("todo"), None)?
            .into_iter()
            .filter(|issue| !blocked.contains(&issue.id))
            .collect())
    }

    pub fn add_label(&self, issue_id: &str, label: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO labels (issue_id, label) VALUES (?1, ?2)",
            params![issue_id, label],
        )?;
        Ok(())
    }

    pub fn get_labels(&self, issue_id: &str) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT label FROM labels WHERE issue_id = ?1 ORDER BY label ASC")?;
        let labels = stmt
            .query_map([issue_id], |row| row.get(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(labels)
    }

    pub fn replace_sources(&self, sources: &[ProjectionSource]) -> Result<()> {
        self.conn.execute("DELETE FROM projection_sources", [])?;
        for source in sources {
            self.conn.execute(
                "INSERT INTO projection_sources
                 (path, kind, id, size_bytes, modified_micros, sha256, indexed_at)
                 VALUES (?1, '', '', ?2, ?3, '', '')",
                params![
                    source.path.to_string_lossy(),
                    source.len as i64,
                    source.modified_unix_millis
                ],
            )?;
        }
        Ok(())
    }

    pub fn freshness(&self) -> Result<Vec<Freshness>> {
        let mut stmt = self.conn.prepare(
            "SELECT path, modified_micros, size_bytes FROM projection_sources ORDER BY path ASC",
        )?;
        let sources = stmt
            .query_map([], |row| {
                Ok(ProjectionSource {
                    path: PathBuf::from(row.get::<_, String>(0)?),
                    modified_unix_millis: row.get(1)?,
                    len: row.get::<_, i64>(2)? as u64,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let mut results = Vec::new();
        for source in sources {
            match std::fs::metadata(&source.path) {
                Ok(metadata) => {
                    let modified = modified_unix_millis(&metadata)?;
                    if modified != source.modified_unix_millis || metadata.len() != source.len {
                        results.push(Freshness::Modified(source.path));
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    results.push(Freshness::MissingSource(source.path));
                }
                Err(error) => return Err(error).with_context(|| "failed to read source metadata"),
            }
        }
        if results.is_empty() {
            results.push(Freshness::Current);
        }
        Ok(results)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS issues (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                priority TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS dependencies (
                blocked_id TEXT NOT NULL,
                blocker_id TEXT NOT NULL,
                PRIMARY KEY (blocked_id, blocker_id)
            );
            CREATE TABLE IF NOT EXISTS labels (
                issue_id TEXT NOT NULL,
                label TEXT NOT NULL,
                PRIMARY KEY (issue_id, label)
            );
            CREATE TABLE IF NOT EXISTS projection_sources (
                path TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                id TEXT NOT NULL,
                size_bytes INTEGER NOT NULL,
                modified_micros INTEGER,
                sha256 TEXT NOT NULL,
                indexed_at TEXT NOT NULL
            );
            "#,
        )?;
        Ok(())
    }
}

pub fn source_from_path(path: impl AsRef<Path>) -> Result<ProjectionSource> {
    let path = path.as_ref();
    let metadata = std::fs::metadata(path)
        .with_context(|| format!("failed to read source metadata {}", path.display()))?;
    Ok(ProjectionSource {
        path: path.to_path_buf(),
        modified_unix_millis: modified_unix_millis(&metadata)?,
        len: metadata.len(),
    })
}

fn modified_unix_millis(metadata: &std::fs::Metadata) -> Result<i64> {
    Ok(metadata
        .modified()
        .context("source modified timestamp is unavailable")?
        .duration_since(std::time::UNIX_EPOCH)
        .context("source modified timestamp predates UNIX epoch")?
        .as_millis() as i64)
}

/// Table ownership in the rebuildable projection database.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TableOwner {
    Projection,
}

pub const PROJECTION_TABLES: &[&str] = &[
    "issues",
    "labels",
    "dependencies",
    "relations",
    "records",
    "record_labels",
    "record_links",
    "evidence",
    "plans",
    "milestones",
    "projection_sources",
];

pub fn table_owner(table: &str) -> Option<TableOwner> {
    if PROJECTION_TABLES.contains(&table) {
        Some(TableOwner::Projection)
    } else {
        None
    }
}

#[cfg(test)]
mod projection_index_api_tests {
    use super::*;

    #[test]
    fn schema_tables_have_explicit_ownership() {
        assert_eq!(table_owner("issues"), Some(TableOwner::Projection));
    }

    #[test]
    fn removed_tables_are_not_part_of_target_schema() {
        assert_eq!(table_owner("runtime_metadata"), None);
        assert_eq!(table_owner("sessions"), None);
        assert_eq!(table_owner("work_associations"), None);
        assert_eq!(table_owner("comments"), None);
        assert_eq!(table_owner("claims"), None);
    }

    #[test]
    fn opened_database_does_not_create_runtime_tables() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("state.db")).unwrap();

        let count: i64 = db
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'runtime_metadata'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(count, 0);
    }

    #[test]
    fn projection_index_stores_and_queries_issues() {
        let dir = tempfile::tempdir().unwrap();
        let index = ProjectionIndex::open(&dir.path().join("state.db")).unwrap();
        index
            .insert_issue(&ProjectionIssue::new("atelier-a", "A", None, "high"))
            .unwrap();
        index
            .insert_issue(&ProjectionIssue::new("atelier-b", "B", None, "medium"))
            .unwrap();
        index.add_dependency("atelier-b", "atelier-a").unwrap();

        assert_eq!(index.get_issue("atelier-a").unwrap().unwrap().title, "A");
        assert_eq!(
            index.list_issues(Some("todo"), Some("high")).unwrap().len(),
            1
        );
        assert_eq!(index.list_ready_issues().unwrap().len(), 1);
        assert_eq!(index.list_blocked_issues().unwrap().len(), 1);
        assert_eq!(index.get_blockers("atelier-b").unwrap(), vec!["atelier-a"]);
        assert_eq!(index.search_issues("A").unwrap().len(), 1);
    }

    #[test]
    fn projection_index_reports_source_freshness() {
        let dir = tempfile::tempdir().unwrap();
        let source_path = dir.path().join("issue.md");
        std::fs::write(&source_path, "one").unwrap();
        let index = ProjectionIndex::open(&dir.path().join("state.db")).unwrap();
        index
            .replace_sources(&[source_from_path(&source_path).unwrap()])
            .unwrap();

        assert_eq!(index.freshness().unwrap(), vec![Freshness::Current]);
        std::fs::write(&source_path, "two changed").unwrap();
        assert!(matches!(
            index.freshness().unwrap().as_slice(),
            [Freshness::Modified(path)] if path == &source_path
        ));
        std::fs::remove_file(&source_path).unwrap();
        assert!(matches!(
            index.freshness().unwrap().as_slice(),
            [Freshness::MissingSource(path)] if path == &source_path
        ));
    }
}
