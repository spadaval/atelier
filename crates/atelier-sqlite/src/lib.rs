mod cache;
mod comments;
mod dependencies;
mod issues;
mod labels;
pub mod projection_index;
mod record_id;
mod records;
pub use records::RecordSummary;
mod relations;

pub use cache::{
    cache_incompatibility, CacheIncompatibility, EvidenceCacheQuery, EvidenceCacheRow,
    EvidenceTargetCacheRow, IssueBlockCacheRow, IssueCacheQuery, IssueCacheRow,
    IssueRelationCacheRow, RecordSourceCacheRow, ReviewRoomCacheRow, CACHE_APPLICATION_ID,
    CACHE_SCHEMA_VERSION, DOMAIN_CACHE_TABLES,
};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OpenFlags, OptionalExtension};
use std::fs;
use std::path::{Path, PathBuf};

use atelier_core::{Issue, IssuePriority, ISSUE_PRIORITY_LABELS};
use atelier_records as record_store;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CacheFileState {
    Missing,
    Ready { version: i32 },
    ApplicationIdMismatch { found: i32, expected: i32 },
    VersionMismatch { found: i32, expected: i32 },
    Corrupt { detail: String },
}

/// Inspect an existing disposable cache without creating or migrating it.
pub fn inspect_cache_file(path: &Path) -> CacheFileState {
    if !path.exists() {
        return CacheFileState::Missing;
    }

    let connection = match Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY) {
        Ok(connection) => connection,
        Err(error) => {
            return CacheFileState::Corrupt {
                detail: error.to_string(),
            };
        }
    };
    let application_id = match connection.query_row("PRAGMA application_id", [], |row| row.get(0)) {
        Ok(application_id) => application_id,
        Err(error) => {
            return CacheFileState::Corrupt {
                detail: error.to_string(),
            };
        }
    };
    let version = match connection.query_row("PRAGMA user_version", [], |row| row.get(0)) {
        Ok(version) => version,
        Err(error) => {
            return CacheFileState::Corrupt {
                detail: error.to_string(),
            };
        }
    };
    let integrity =
        match connection.query_row("PRAGMA quick_check", [], |row| row.get::<_, String>(0)) {
            Ok(integrity) => integrity,
            Err(error) => {
                return CacheFileState::Corrupt {
                    detail: error.to_string(),
                };
            }
        };
    if integrity != "ok" {
        return CacheFileState::Corrupt { detail: integrity };
    }
    if application_id != CACHE_APPLICATION_ID {
        return CacheFileState::ApplicationIdMismatch {
            found: application_id,
            expected: CACHE_APPLICATION_ID,
        };
    }
    if version != CACHE_SCHEMA_VERSION {
        return CacheFileState::VersionMismatch {
            found: version,
            expected: CACHE_SCHEMA_VERSION,
        };
    }
    CacheFileState::Ready { version }
}

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
    "contributes_to",
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
pub const VALID_PRIORITIES: &[&str] = ISSUE_PRIORITY_LABELS;

/// Valid values for canonical issue type.
pub const VALID_ISSUE_TYPES: &[&str] = &[
    "bug",
    "epic",
    "feature",
    "mission",
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
    IssuePriority::from_cli_input(priority)
        .map(|_| ())
        .map_err(Into::into)
}

/// Validate that an issue type value is syntactically valid.
pub fn validate_issue_type(issue_type: &str) -> Result<()> {
    let mut chars = issue_type.chars();
    if matches!(chars.next(), Some(first) if first.is_ascii_lowercase())
        && chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        Ok(())
    } else {
        anyhow::bail!(
            "Invalid issue_type '{}'. Issue type values must match ^[a-z][a-z0-9_]*$",
            issue_type,
        )
    }
}

/// Validate relation type: empty strings are rejected; unknown types emit a
/// warning but are still accepted (warn-but-accept pattern).
pub fn validate_relation_type(relation_type: &str) -> Result<()> {
    if relation_type.is_empty() {
        anyhow::bail!("Relation type cannot be empty");
    }
    validate_relation_type_syntax(relation_type)?;
    if !WELL_KNOWN_RELATION_TYPES.contains(&relation_type) {
        tracing::warn!(
            "Unknown relation type '{}'. Known types: {}",
            relation_type,
            WELL_KNOWN_RELATION_TYPES.join(", ")
        );
    }
    Ok(())
}

fn validate_relation_type_syntax(relation_type: &str) -> Result<()> {
    let mut chars = relation_type.chars();
    let Some(first) = chars.next() else {
        anyhow::bail!("Relation type cannot be empty");
    };
    if !first.is_ascii_lowercase() {
        anyhow::bail!(
            "Invalid relation type '{}'. Values must start with a lowercase ASCII letter",
            relation_type
        );
    }
    if !chars
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '_' | '-' | '.'))
    {
        anyhow::bail!(
            "Invalid relation type '{}'. Values may contain only lowercase ASCII letters, digits, '_', '-', or '.'",
            relation_type
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
        validate_relation_type_syntax(relation_type)
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
    pub(crate) conn: Connection,
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

    fn init_schema(&self) -> Result<()> {
        let application_id = self.current_application_id();
        let version = self.current_schema_version();
        let table_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master
             WHERE type = 'table' AND name NOT LIKE 'sqlite_%'",
            [],
            |row| row.get(0),
        )?;

        if application_id == 0 && version == 0 && table_count == 0 {
            return self.install_domain_cache_schema();
        }
        if application_id != cache::CACHE_APPLICATION_ID {
            return Err(CacheIncompatibility::ApplicationId {
                found: application_id,
                expected: cache::CACHE_APPLICATION_ID,
            }
            .into());
        }
        if version != cache::CACHE_SCHEMA_VERSION {
            return Err(CacheIncompatibility::SchemaVersion {
                found: version,
                expected: cache::CACHE_SCHEMA_VERSION,
            }
            .into());
        }
        self.conn.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(())
    }

    fn current_application_id(&self) -> i32 {
        self.conn
            .query_row("PRAGMA application_id", [], |row| row.get(0))
            .unwrap_or(0)
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
}

pub(crate) fn parse_datetime(s: String) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

/// Maps a database row to an Issue struct.
/// Expects columns in order: id, title, description, status, issue_type, priority, fields_json, parent_id, created_at, updated_at, closed_at
pub(crate) fn issue_from_row(row: &rusqlite::Row) -> rusqlite::Result<Issue> {
    let fields_json = row.get::<_, String>(6)?;
    let fields = serde_json::from_str(&fields_json).unwrap_or_default();
    Ok(Issue {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        status: row.get(3)?,
        issue_type: row.get(4)?,
        priority: row.get(5)?,
        fields,
        parent_id: row.get(7)?,
        created_at: parse_datetime(row.get::<_, String>(8)?),
        updated_at: parse_datetime(row.get::<_, String>(9)?),
        closed_at: row.get::<_, Option<String>>(10)?.map(parse_datetime),
    })
}

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
    "projection_sources",
];

pub fn table_owner(table: &str) -> Option<TableOwner> {
    if PROJECTION_TABLES.contains(&table) {
        Some(TableOwner::Projection)
    } else {
        None
    }
}
