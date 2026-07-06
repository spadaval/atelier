mod cache;
mod comments;
mod dependencies;
mod issues;
mod labels;
mod record_id;
mod records;
pub mod source_freshness;
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
use rusqlite::{Connection, OpenFlags};
use std::fs;
use std::path::{Path, PathBuf};

use atelier_core::{IssuePriority, ISSUE_PRIORITY_LABELS};
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
pub const MAX_LABEL_LEN: usize = 128;
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
