use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, OptionalExtension};

use crate::{parse_datetime, Database};

/// Domain-cache schema identity. A mismatch means the ignored cache must be rebuilt.
pub const CACHE_APPLICATION_ID: i32 = 0x4154_4c52; // "ATLR"
pub const CACHE_SCHEMA_VERSION: i32 = 1;

pub const DOMAIN_CACHE_TABLES: &[&str] = &[
    "issue_index",
    "issue_label_index",
    "issue_block_index",
    "issue_relation_index",
    "evidence_index",
    "evidence_target_index",
    "review_room_index",
    "record_source_index",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CacheIncompatibility {
    ApplicationId { found: i32, expected: i32 },
    SchemaVersion { found: i32, expected: i32 },
}

impl std::fmt::Display for CacheIncompatibility {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApplicationId { found, expected } => write!(
                formatter,
                "cache application id {found} is incompatible with expected id {expected}; rebuild the disposable cache"
            ),
            Self::SchemaVersion { found, expected } => write!(
                formatter,
                "cache schema version {found} is incompatible with expected version {expected}; rebuild the disposable cache"
            ),
        }
    }
}

impl std::error::Error for CacheIncompatibility {}

pub fn cache_incompatibility(error: &anyhow::Error) -> Option<&CacheIncompatibility> {
    error.downcast_ref()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCacheRow {
    pub id: String,
    pub title: String,
    pub status: String,
    pub issue_type: String,
    pub priority: String,
    pub parent_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IssueCacheQuery<'a> {
    pub status: Option<&'a str>,
    pub issue_type: Option<&'a str>,
    pub priority: Option<&'a str>,
    pub label: Option<&'a str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueBlockCacheRow {
    pub blocker_id: String,
    pub blocked_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueRelationCacheRow {
    pub source_issue_id: String,
    pub target_issue_id: String,
    pub relation_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceCacheRow {
    pub id: String,
    pub title: String,
    pub status: String,
    pub evidence_type: String,
    pub captured_at: DateTime<Utc>,
    pub proof_scope: Option<String>,
    pub agent_identity: Option<String>,
    pub independence_level: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EvidenceCacheQuery<'a> {
    pub status: Option<&'a str>,
    pub evidence_type: Option<&'a str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceTargetCacheRow {
    pub evidence_id: String,
    pub target_kind: String,
    pub target_id: String,
    pub role: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewRoomCacheRow {
    pub id: String,
    pub issue_id: String,
    pub title: String,
    pub status: String,
    pub source_branch: String,
    pub target_branch: String,
    pub approvals: i64,
    pub unresolved_blocking: i64,
    pub unresolved_nonblocking: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordSourceCacheRow {
    pub path: String,
    pub record_kind: String,
    pub record_id: String,
    pub size_bytes: i64,
    pub modified_micros: Option<i64>,
    pub content_hash: Option<String>,
    pub indexed_at: DateTime<Utc>,
}

impl Database {
    pub(crate) fn install_domain_cache_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;

            CREATE TABLE issue_index (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                status TEXT NOT NULL,
                issue_type TEXT NOT NULL,
                priority TEXT NOT NULL,
                parent_id TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                closed_at TEXT
            );
            CREATE INDEX idx_issue_index_status_priority
                ON issue_index(status, priority, updated_at DESC, id);
            CREATE INDEX idx_issue_index_type_status
                ON issue_index(issue_type, status, id);
            CREATE INDEX idx_issue_index_parent
                ON issue_index(parent_id, id);

            CREATE TABLE issue_label_index (
                issue_id TEXT NOT NULL,
                label TEXT NOT NULL,
                PRIMARY KEY (issue_id, label),
                FOREIGN KEY (issue_id) REFERENCES issue_index(id) ON DELETE CASCADE
            );
            CREATE INDEX idx_issue_label_index_label
                ON issue_label_index(label, issue_id);

            CREATE TABLE issue_block_index (
                blocker_id TEXT NOT NULL,
                blocked_id TEXT NOT NULL,
                PRIMARY KEY (blocker_id, blocked_id)
            );
            CREATE INDEX idx_issue_block_index_blocked
                ON issue_block_index(blocked_id, blocker_id);

            CREATE TABLE issue_relation_index (
                source_issue_id TEXT NOT NULL,
                target_issue_id TEXT NOT NULL,
                relation_type TEXT NOT NULL,
                created_at TEXT NOT NULL,
                PRIMARY KEY (source_issue_id, target_issue_id, relation_type)
            );
            CREATE INDEX idx_issue_relation_index_target
                ON issue_relation_index(target_issue_id, relation_type, source_issue_id);

            CREATE TABLE evidence_index (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                status TEXT NOT NULL,
                evidence_type TEXT NOT NULL,
                captured_at TEXT NOT NULL,
                proof_scope TEXT,
                agent_identity TEXT,
                independence_level TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE INDEX idx_evidence_index_status_type
                ON evidence_index(status, evidence_type, captured_at DESC, id);

            CREATE TABLE evidence_target_index (
                evidence_id TEXT NOT NULL,
                target_kind TEXT NOT NULL,
                target_id TEXT NOT NULL,
                role TEXT NOT NULL,
                PRIMARY KEY (evidence_id, target_kind, target_id, role),
                FOREIGN KEY (evidence_id) REFERENCES evidence_index(id) ON DELETE CASCADE
            );
            CREATE INDEX idx_evidence_target_index_target
                ON evidence_target_index(target_kind, target_id, role, evidence_id);

            CREATE TABLE review_room_index (
                id TEXT PRIMARY KEY,
                issue_id TEXT NOT NULL,
                title TEXT NOT NULL,
                status TEXT NOT NULL,
                source_branch TEXT NOT NULL,
                target_branch TEXT NOT NULL,
                approvals INTEGER NOT NULL,
                unresolved_blocking INTEGER NOT NULL,
                unresolved_nonblocking INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE INDEX idx_review_room_index_issue
                ON review_room_index(issue_id, updated_at DESC, id);
            CREATE INDEX idx_review_room_index_status
                ON review_room_index(status, issue_id);

            CREATE TABLE record_source_index (
                path TEXT PRIMARY KEY,
                record_kind TEXT NOT NULL,
                record_id TEXT NOT NULL,
                size_bytes INTEGER NOT NULL CHECK (size_bytes >= 0),
                modified_micros INTEGER,
                content_hash TEXT,
                indexed_at TEXT NOT NULL,
                UNIQUE (record_kind, record_id)
            );
            CREATE INDEX idx_record_source_index_record
                ON record_source_index(record_kind, record_id);
            CREATE INDEX idx_record_source_index_hash
                ON record_source_index(content_hash) WHERE content_hash IS NOT NULL;
            "#,
        )?;
        self.conn.execute(
            &format!("PRAGMA application_id = {CACHE_APPLICATION_ID}"),
            [],
        )?;
        self.conn
            .execute(&format!("PRAGMA user_version = {CACHE_SCHEMA_VERSION}"), [])?;
        Ok(())
    }

    pub fn index_issue(
        &self,
        issue: &IssueCacheRow,
        labels: &[String],
        blocks: &[IssueBlockCacheRow],
        relations: &[IssueRelationCacheRow],
        source: &RecordSourceCacheRow,
    ) -> Result<()> {
        self.transaction(|| {
            self.remove_issue_rows(&issue.id)?;
            self.conn.execute(
                "INSERT INTO issue_index
                 (id, title, status, issue_type, priority, parent_id, created_at, updated_at, closed_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    issue.id,
                    issue.title,
                    issue.status,
                    issue.issue_type,
                    issue.priority,
                    issue.parent_id,
                    issue.created_at.to_rfc3339(),
                    issue.updated_at.to_rfc3339(),
                    issue.closed_at.map(|value| value.to_rfc3339()),
                ],
            )?;
            for label in labels {
                self.conn.execute(
                    "INSERT INTO issue_label_index (issue_id, label) VALUES (?1, ?2)",
                    params![issue.id, label],
                )?;
            }
            for block in blocks {
                if block.blocker_id != issue.id {
                    anyhow::bail!(
                        "issue block row {} -> {} is not owned by indexed issue {}",
                        block.blocker_id,
                        block.blocked_id,
                        issue.id
                    );
                }
                self.conn.execute(
                    "INSERT INTO issue_block_index (blocker_id, blocked_id) VALUES (?1, ?2)",
                    params![block.blocker_id, block.blocked_id],
                )?;
            }
            for relation in relations {
                if relation.source_issue_id != issue.id {
                    anyhow::bail!(
                        "issue relation row {} -> {} is not owned by indexed issue {}",
                        relation.source_issue_id,
                        relation.target_issue_id,
                        issue.id
                    );
                }
                self.conn.execute(
                    "INSERT INTO issue_relation_index
                     (source_issue_id, target_issue_id, relation_type, created_at)
                     VALUES (?1, ?2, ?3, ?4)",
                    params![
                        relation.source_issue_id,
                        relation.target_issue_id,
                        relation.relation_type,
                        relation.created_at.to_rfc3339(),
                    ],
                )?;
            }
            self.replace_source(source, "issue", &issue.id)
        })
    }

    pub fn remove_indexed_issue(&self, id: &str, source_path: &str) -> Result<()> {
        self.transaction(|| {
            self.remove_issue_rows(id)?;
            self.remove_source(source_path, "issue", id)
        })
    }

    fn remove_issue_rows(&self, id: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM issue_relation_index WHERE source_issue_id = ?1",
            [id],
        )?;
        self.conn
            .execute("DELETE FROM issue_block_index WHERE blocker_id = ?1", [id])?;
        self.conn
            .execute("DELETE FROM issue_index WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn issue_cache_row(&self, id: &str) -> Result<Option<IssueCacheRow>> {
        self.conn
            .query_row(
                "SELECT id, title, status, issue_type, priority, parent_id,
                        created_at, updated_at, closed_at
                 FROM issue_index WHERE id = ?1",
                [id],
                issue_cache_row,
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn resolve_issue_cache_ref(&self, issue_ref: &str) -> Result<Option<String>> {
        let normalized = issue_ref.trim();
        if normalized.is_empty() {
            return Ok(None);
        }
        if self.issue_cache_row(normalized)?.is_some() {
            return Ok(Some(normalized.to_string()));
        }
        if !normalized
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
        {
            return Ok(None);
        }

        let suffix = format!("%-{normalized}");
        let mut statement = self
            .conn
            .prepare("SELECT id FROM issue_index WHERE id LIKE ?1 ORDER BY id LIMIT 2")?;
        let matches = statement
            .query_map([suffix], |row| row.get::<_, String>(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        match matches.as_slice() {
            [id] => Ok(Some(id.clone())),
            [] => Ok(None),
            _ => anyhow::bail!(
                "Issue key {normalized} is ambiguous: {}",
                matches.join(", ")
            ),
        }
    }

    pub fn query_issue_cache(&self, query: &IssueCacheQuery<'_>) -> Result<Vec<IssueCacheRow>> {
        let mut sql = String::from(
            "SELECT DISTINCT i.id, i.title, i.status, i.issue_type, i.priority, i.parent_id,
                    i.created_at, i.updated_at, i.closed_at
             FROM issue_index i",
        );
        let mut clauses = Vec::new();
        let mut values: Vec<String> = Vec::new();
        if query.label.is_some() {
            sql.push_str(" JOIN issue_label_index l ON l.issue_id = i.id");
        }
        for (column, value) in [
            ("i.status", query.status),
            ("i.issue_type", query.issue_type),
            ("i.priority", query.priority),
            ("l.label", query.label),
        ] {
            if let Some(value) = value {
                values.push(value.to_string());
                clauses.push(format!("{column} = ?{}", values.len()));
            }
        }
        if !clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&clauses.join(" AND "));
        }
        sql.push_str(" ORDER BY i.updated_at DESC, i.id");
        let refs = values
            .iter()
            .map(|value| value as &dyn rusqlite::ToSql)
            .collect::<Vec<_>>();
        let mut statement = self.conn.prepare(&sql)?;
        let rows = statement.query_map(refs.as_slice(), issue_cache_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn issue_cache_labels(&self, id: &str) -> Result<Vec<String>> {
        let mut statement = self
            .conn
            .prepare("SELECT label FROM issue_label_index WHERE issue_id = ?1 ORDER BY label")?;
        let rows = statement
            .query_map([id], |row| row.get(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn issue_cache_blockers(&self, blocked_id: &str) -> Result<Vec<String>> {
        let mut statement = self.conn.prepare(
            "SELECT blocker_id FROM issue_block_index
             WHERE blocked_id = ?1 ORDER BY blocker_id",
        )?;
        let rows = statement
            .query_map([blocked_id], |row| row.get(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn issue_cache_relations(&self, source_id: &str) -> Result<Vec<IssueRelationCacheRow>> {
        let mut statement = self.conn.prepare(
            "SELECT source_issue_id, target_issue_id, relation_type, created_at
             FROM issue_relation_index
             WHERE source_issue_id = ?1
             ORDER BY target_issue_id, relation_type",
        )?;
        let rows = statement
            .query_map([source_id], issue_relation_cache_row)?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Returns whether an issue participates in graph state that cannot be
    /// repaired safely from only that issue's source file.
    pub fn issue_cache_has_graph_edges(&self, id: &str) -> Result<bool> {
        let edge_count: i64 = self.conn.query_row(
            "SELECT
                (SELECT COUNT(*) FROM issue_block_index
                 WHERE blocker_id = ?1 OR blocked_id = ?1) +
                (SELECT COUNT(*) FROM issue_relation_index
                 WHERE source_issue_id = ?1 OR target_issue_id = ?1) +
                (SELECT COUNT(*) FROM issue_index
                 WHERE parent_id = ?1 OR (id = ?1 AND parent_id IS NOT NULL))",
            [id],
            |row| row.get(0),
        )?;
        Ok(edge_count > 0)
    }

    pub fn index_evidence(
        &self,
        evidence: &EvidenceCacheRow,
        targets: &[EvidenceTargetCacheRow],
        source: &RecordSourceCacheRow,
    ) -> Result<()> {
        self.transaction(|| {
            self.conn
                .execute("DELETE FROM evidence_index WHERE id = ?1", [&evidence.id])?;
            self.conn.execute(
                "INSERT INTO evidence_index
                 (id, title, status, evidence_type, captured_at, proof_scope, agent_identity,
                  independence_level, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    evidence.id,
                    evidence.title,
                    evidence.status,
                    evidence.evidence_type,
                    evidence.captured_at.to_rfc3339(),
                    evidence.proof_scope,
                    evidence.agent_identity,
                    evidence.independence_level,
                    evidence.created_at.to_rfc3339(),
                    evidence.updated_at.to_rfc3339(),
                ],
            )?;
            for target in targets {
                if target.evidence_id != evidence.id {
                    anyhow::bail!(
                        "evidence target row {} -> {}/{} is not owned by indexed evidence {}",
                        target.evidence_id,
                        target.target_kind,
                        target.target_id,
                        evidence.id
                    );
                }
                self.conn.execute(
                    "INSERT INTO evidence_target_index
                     (evidence_id, target_kind, target_id, role) VALUES (?1, ?2, ?3, ?4)",
                    params![
                        target.evidence_id,
                        target.target_kind,
                        target.target_id,
                        target.role,
                    ],
                )?;
            }
            self.replace_source(source, "evidence", &evidence.id)
        })
    }

    pub fn remove_indexed_evidence(&self, id: &str, source_path: &str) -> Result<()> {
        self.transaction(|| {
            self.conn
                .execute("DELETE FROM evidence_index WHERE id = ?1", [id])?;
            self.remove_source(source_path, "evidence", id)
        })
    }

    pub fn evidence_cache_row(&self, id: &str) -> Result<Option<EvidenceCacheRow>> {
        self.conn
            .query_row(
                "SELECT id, title, status, evidence_type, captured_at, proof_scope,
                        agent_identity, independence_level, created_at, updated_at
                 FROM evidence_index WHERE id = ?1",
                [id],
                evidence_cache_row,
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn query_evidence_cache(
        &self,
        query: &EvidenceCacheQuery<'_>,
    ) -> Result<Vec<EvidenceCacheRow>> {
        let mut sql = String::from(
            "SELECT id, title, status, evidence_type, captured_at, proof_scope,
                    agent_identity, independence_level, created_at, updated_at
             FROM evidence_index",
        );
        let mut clauses = Vec::new();
        let mut values = Vec::new();
        for (column, value) in [
            ("status", query.status),
            ("evidence_type", query.evidence_type),
        ] {
            if let Some(value) = value {
                values.push(value.to_string());
                clauses.push(format!("{column} = ?{}", values.len()));
            }
        }
        if !clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&clauses.join(" AND "));
        }
        sql.push_str(" ORDER BY captured_at DESC, id");
        let refs = values
            .iter()
            .map(|value| value as &dyn rusqlite::ToSql)
            .collect::<Vec<_>>();
        let mut statement = self.conn.prepare(&sql)?;
        let rows = statement.query_map(refs.as_slice(), evidence_cache_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn evidence_cache_targets(&self, id: &str) -> Result<Vec<EvidenceTargetCacheRow>> {
        let mut statement = self.conn.prepare(
            "SELECT evidence_id, target_kind, target_id, role
             FROM evidence_target_index WHERE evidence_id = ?1
             ORDER BY target_kind, target_id, role",
        )?;
        let rows = statement
            .query_map([id], evidence_target_cache_row)?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn evidence_cache_for_target(
        &self,
        target_kind: &str,
        target_id: &str,
        role: Option<&str>,
    ) -> Result<Vec<EvidenceCacheRow>> {
        let mut sql = String::from(
            "SELECT e.id, e.title, e.status, e.evidence_type, e.captured_at, e.proof_scope,
                    e.agent_identity, e.independence_level, e.created_at, e.updated_at
             FROM evidence_index e
             JOIN evidence_target_index t ON t.evidence_id = e.id
             WHERE t.target_kind = ?1 AND t.target_id = ?2",
        );
        let rows = if let Some(role) = role {
            sql.push_str(" AND t.role = ?3 ORDER BY e.captured_at DESC, e.id");
            let mut statement = self.conn.prepare(&sql)?;
            let rows = statement
                .query_map(params![target_kind, target_id, role], evidence_cache_row)?
                .collect::<std::result::Result<Vec<_>, _>>()?;
            rows
        } else {
            sql.push_str(" ORDER BY e.captured_at DESC, e.id");
            let mut statement = self.conn.prepare(&sql)?;
            let rows = statement
                .query_map(params![target_kind, target_id], evidence_cache_row)?
                .collect::<std::result::Result<Vec<_>, _>>()?;
            rows
        };
        Ok(rows)
    }

    pub fn index_review_room(
        &self,
        room: &ReviewRoomCacheRow,
        source: &RecordSourceCacheRow,
    ) -> Result<()> {
        if room.approvals < 0 || room.unresolved_blocking < 0 || room.unresolved_nonblocking < 0 {
            anyhow::bail!("review room aggregate counts cannot be negative");
        }
        self.transaction(|| {
            self.conn.execute(
                "INSERT INTO review_room_index
                 (id, issue_id, title, status, source_branch, target_branch, approvals,
                  unresolved_blocking, unresolved_nonblocking, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                 ON CONFLICT(id) DO UPDATE SET
                    issue_id = excluded.issue_id,
                    title = excluded.title,
                    status = excluded.status,
                    source_branch = excluded.source_branch,
                    target_branch = excluded.target_branch,
                    approvals = excluded.approvals,
                    unresolved_blocking = excluded.unresolved_blocking,
                    unresolved_nonblocking = excluded.unresolved_nonblocking,
                    created_at = excluded.created_at,
                    updated_at = excluded.updated_at",
                params![
                    room.id,
                    room.issue_id,
                    room.title,
                    room.status,
                    room.source_branch,
                    room.target_branch,
                    room.approvals,
                    room.unresolved_blocking,
                    room.unresolved_nonblocking,
                    room.created_at.to_rfc3339(),
                    room.updated_at.to_rfc3339(),
                ],
            )?;
            self.replace_source(source, "review", &room.id)
        })
    }

    pub fn remove_indexed_review_room(&self, id: &str, source_path: &str) -> Result<()> {
        self.transaction(|| {
            self.conn
                .execute("DELETE FROM review_room_index WHERE id = ?1", [id])?;
            self.remove_source(source_path, "review", id)
        })
    }

    pub fn review_room_cache_row(&self, id: &str) -> Result<Option<ReviewRoomCacheRow>> {
        self.conn
            .query_row(
                "SELECT id, issue_id, title, status, source_branch, target_branch, approvals,
                        unresolved_blocking, unresolved_nonblocking, created_at, updated_at
                 FROM review_room_index WHERE id = ?1",
                [id],
                review_room_cache_row,
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn review_room_cache_for_issue(&self, issue_id: &str) -> Result<Vec<ReviewRoomCacheRow>> {
        let mut statement = self.conn.prepare(
            "SELECT id, issue_id, title, status, source_branch, target_branch, approvals,
                    unresolved_blocking, unresolved_nonblocking, created_at, updated_at
             FROM review_room_index WHERE issue_id = ?1
             ORDER BY updated_at DESC, id",
        )?;
        let rows = statement
            .query_map([issue_id], review_room_cache_row)?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn record_source_cache_row(&self, path: &str) -> Result<Option<RecordSourceCacheRow>> {
        self.conn
            .query_row(
                "SELECT path, record_kind, record_id, size_bytes, modified_micros,
                        content_hash, indexed_at
                 FROM record_source_index WHERE path = ?1",
                [path],
                record_source_cache_row,
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn record_source_cache_rows(&self) -> Result<Vec<RecordSourceCacheRow>> {
        let mut statement = self.conn.prepare(
            "SELECT path, record_kind, record_id, size_bytes, modified_micros,
                    content_hash, indexed_at
             FROM record_source_index ORDER BY path",
        )?;
        let rows = statement
            .query_map([], record_source_cache_row)?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Refresh filesystem metadata for an already indexed canonical record.
    ///
    /// This deliberately cannot create a source row: the corresponding domain
    /// indexer remains the sole owner of adding records to the cache.
    pub fn refresh_record_source_metadata(&self, source: &RecordSourceCacheRow) -> Result<()> {
        let changed = self.conn.execute(
            "UPDATE record_source_index
             SET size_bytes = ?4, modified_micros = ?5, content_hash = ?6, indexed_at = ?7
             WHERE path = ?1 AND record_kind = ?2 AND record_id = ?3",
            params![
                source.path,
                source.record_kind,
                source.record_id,
                source.size_bytes,
                source.modified_micros,
                source.content_hash,
                source.indexed_at.to_rfc3339(),
            ],
        )?;
        if changed != 1 {
            anyhow::bail!(
                "source {} is not indexed as {}/{}",
                source.path,
                source.record_kind,
                source.record_id
            );
        }
        Ok(())
    }

    fn replace_source(
        &self,
        source: &RecordSourceCacheRow,
        expected_kind: &str,
        expected_id: &str,
    ) -> Result<()> {
        if source.record_kind != expected_kind || source.record_id != expected_id {
            anyhow::bail!(
                "source {} identifies {}/{}, expected {}/{}",
                source.path,
                source.record_kind,
                source.record_id,
                expected_kind,
                expected_id
            );
        }
        self.conn.execute(
            "INSERT INTO record_source_index
             (path, record_kind, record_id, size_bytes, modified_micros, content_hash, indexed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(path) DO UPDATE SET
                record_kind = excluded.record_kind,
                record_id = excluded.record_id,
                size_bytes = excluded.size_bytes,
                modified_micros = excluded.modified_micros,
                content_hash = excluded.content_hash,
                indexed_at = excluded.indexed_at",
            params![
                source.path,
                source.record_kind,
                source.record_id,
                source.size_bytes,
                source.modified_micros,
                source.content_hash,
                source.indexed_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    fn remove_source(&self, path: &str, expected_kind: &str, expected_id: &str) -> Result<()> {
        let changed = self.conn.execute(
            "DELETE FROM record_source_index
             WHERE path = ?1 AND record_kind = ?2 AND record_id = ?3",
            params![path, expected_kind, expected_id],
        )?;
        if changed == 0 {
            anyhow::bail!(
                "source {} was not indexed as {}/{}",
                path,
                expected_kind,
                expected_id
            );
        }
        Ok(())
    }
}

fn issue_cache_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<IssueCacheRow> {
    Ok(IssueCacheRow {
        id: row.get(0)?,
        title: row.get(1)?,
        status: row.get(2)?,
        issue_type: row.get(3)?,
        priority: row.get(4)?,
        parent_id: row.get(5)?,
        created_at: parse_datetime(row.get(6)?),
        updated_at: parse_datetime(row.get(7)?),
        closed_at: row.get::<_, Option<String>>(8)?.map(parse_datetime),
    })
}

fn issue_relation_cache_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<IssueRelationCacheRow> {
    Ok(IssueRelationCacheRow {
        source_issue_id: row.get(0)?,
        target_issue_id: row.get(1)?,
        relation_type: row.get(2)?,
        created_at: parse_datetime(row.get(3)?),
    })
}

fn evidence_cache_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<EvidenceCacheRow> {
    Ok(EvidenceCacheRow {
        id: row.get(0)?,
        title: row.get(1)?,
        status: row.get(2)?,
        evidence_type: row.get(3)?,
        captured_at: parse_datetime(row.get(4)?),
        proof_scope: row.get(5)?,
        agent_identity: row.get(6)?,
        independence_level: row.get(7)?,
        created_at: parse_datetime(row.get(8)?),
        updated_at: parse_datetime(row.get(9)?),
    })
}

fn evidence_target_cache_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<EvidenceTargetCacheRow> {
    Ok(EvidenceTargetCacheRow {
        evidence_id: row.get(0)?,
        target_kind: row.get(1)?,
        target_id: row.get(2)?,
        role: row.get(3)?,
    })
}

fn review_room_cache_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ReviewRoomCacheRow> {
    Ok(ReviewRoomCacheRow {
        id: row.get(0)?,
        issue_id: row.get(1)?,
        title: row.get(2)?,
        status: row.get(3)?,
        source_branch: row.get(4)?,
        target_branch: row.get(5)?,
        approvals: row.get(6)?,
        unresolved_blocking: row.get(7)?,
        unresolved_nonblocking: row.get(8)?,
        created_at: parse_datetime(row.get(9)?),
        updated_at: parse_datetime(row.get(10)?),
    })
}

fn record_source_cache_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<RecordSourceCacheRow> {
    Ok(RecordSourceCacheRow {
        path: row.get(0)?,
        record_kind: row.get(1)?,
        record_id: row.get(2)?,
        size_bytes: row.get(3)?,
        modified_micros: row.get(4)?,
        content_hash: row.get(5)?,
        indexed_at: parse_datetime(row.get(6)?),
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use rusqlite::Connection;
    use tempfile::TempDir;

    use super::*;

    fn database() -> (TempDir, Database) {
        let directory = tempfile::tempdir().unwrap();
        let database = Database::open(&directory.path().join("state.db")).unwrap();
        (directory, database)
    }

    fn timestamp(seconds: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(seconds, 0).unwrap()
    }

    fn source(kind: &str, id: &str, path: &str, revision: i64) -> RecordSourceCacheRow {
        RecordSourceCacheRow {
            path: path.to_string(),
            record_kind: kind.to_string(),
            record_id: id.to_string(),
            size_bytes: revision,
            modified_micros: Some(revision),
            content_hash: Some(format!("hash-{revision}")),
            indexed_at: timestamp(revision),
        }
    }

    fn issue(id: &str, title: &str, revision: i64) -> IssueCacheRow {
        IssueCacheRow {
            id: id.to_string(),
            title: title.to_string(),
            status: "todo".to_string(),
            issue_type: "task".to_string(),
            priority: "P1".to_string(),
            parent_id: None,
            created_at: timestamp(1),
            updated_at: timestamp(revision),
            closed_at: None,
        }
    }

    fn evidence(id: &str, title: &str, revision: i64) -> EvidenceCacheRow {
        EvidenceCacheRow {
            id: id.to_string(),
            title: title.to_string(),
            status: "pass".to_string(),
            evidence_type: "test".to_string(),
            captured_at: timestamp(revision),
            proof_scope: Some("cache schema".to_string()),
            agent_identity: Some("implementer".to_string()),
            independence_level: Some("implementer".to_string()),
            created_at: timestamp(1),
            updated_at: timestamp(revision),
        }
    }

    fn room(id: &str, title: &str, revision: i64) -> ReviewRoomCacheRow {
        ReviewRoomCacheRow {
            id: id.to_string(),
            issue_id: "atelier-issue".to_string(),
            title: title.to_string(),
            status: "open".to_string(),
            source_branch: "epic/cache".to_string(),
            target_branch: "main".to_string(),
            approvals: revision,
            unresolved_blocking: 0,
            unresolved_nonblocking: 0,
            created_at: timestamp(1),
            updated_at: timestamp(revision),
        }
    }

    #[test]
    fn new_database_has_only_domain_tables_and_explicit_indexes() {
        let (_directory, database) = database();
        let mut statement = database
            .conn
            .prepare(
                "SELECT name FROM sqlite_master
                 WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
            )
            .unwrap();
        let tables = statement
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .collect::<std::result::Result<BTreeSet<_>, _>>()
            .unwrap();
        let expected = DOMAIN_CACHE_TABLES
            .iter()
            .map(|table| table.to_string())
            .collect::<BTreeSet<_>>();
        assert_eq!(tables, expected);

        for forbidden in [
            "records",
            "record_labels",
            "record_links",
            "projection_sources",
        ] {
            assert!(!tables.contains(forbidden));
        }

        let application_id: i32 = database
            .conn
            .query_row("PRAGMA application_id", [], |row| row.get(0))
            .unwrap();
        let schema_version: i32 = database
            .conn
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .unwrap();
        assert_eq!(application_id, CACHE_APPLICATION_ID);
        assert_eq!(schema_version, CACHE_SCHEMA_VERSION);

        let index_count: i64 = database
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type = 'index' AND name LIKE 'idx_%'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(index_count, 12);
    }

    #[test]
    fn schema_has_no_body_or_generic_payload_columns() {
        let (_directory, database) = database();
        let forbidden = [
            "body",
            "description",
            "outcome",
            "notes",
            "data",
            "data_json",
            "fields_json",
            "payload",
            "payload_json",
            "events",
            "events_json",
        ];
        for table in DOMAIN_CACHE_TABLES {
            let mut statement = database
                .conn
                .prepare(&format!("PRAGMA table_info({table})"))
                .unwrap();
            let columns = statement
                .query_map([], |row| row.get::<_, String>(1))
                .unwrap()
                .collect::<std::result::Result<BTreeSet<_>, _>>()
                .unwrap();
            for column in forbidden {
                assert!(
                    !columns.contains(column),
                    "{table} unexpectedly stores forbidden column {column}"
                );
            }
        }
    }

    #[test]
    fn incompatible_cache_version_is_classified_without_migration() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("state.db");
        let connection = Connection::open(&path).unwrap();
        connection
            .execute(
                &format!("PRAGMA application_id = {CACHE_APPLICATION_ID}"),
                [],
            )
            .unwrap();
        connection.execute("PRAGMA user_version = 99", []).unwrap();
        connection
            .execute("CREATE TABLE old_cache (payload_json TEXT)", [])
            .unwrap();
        drop(connection);

        let error = Database::open(&path).err().unwrap();
        assert_eq!(
            cache_incompatibility(&error),
            Some(&CacheIncompatibility::SchemaVersion {
                found: 99,
                expected: CACHE_SCHEMA_VERSION,
            })
        );
        let connection = Connection::open(path).unwrap();
        let old_table_count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE name = 'old_cache'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(old_table_count, 1);
    }

    #[test]
    fn foreign_or_unversioned_layout_is_classified_for_rebuild() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("state.db");
        let connection = Connection::open(&path).unwrap();
        connection
            .execute("CREATE TABLE records (id TEXT)", [])
            .unwrap();
        drop(connection);

        let error = Database::open(&path).err().unwrap();
        assert_eq!(
            cache_incompatibility(&error),
            Some(&CacheIncompatibility::ApplicationId {
                found: 0,
                expected: CACHE_APPLICATION_ID,
            })
        );
    }

    #[test]
    fn issue_rows_add_replace_query_and_delete_atomically() {
        let (_directory, database) = database();
        let id = "atelier-issue";
        database
            .index_issue(
                &issue(id, "first", 2),
                &["cache".to_string()],
                &[IssueBlockCacheRow {
                    blocker_id: id.to_string(),
                    blocked_id: "atelier-next".to_string(),
                }],
                &[IssueRelationCacheRow {
                    source_issue_id: id.to_string(),
                    target_issue_id: "atelier-peer".to_string(),
                    relation_type: "related".to_string(),
                    created_at: timestamp(2),
                }],
                &source("issue", id, "issues/atelier-issue.md", 2),
            )
            .unwrap();
        assert_eq!(
            database.issue_cache_row(id).unwrap().unwrap().title,
            "first"
        );
        assert_eq!(database.issue_cache_labels(id).unwrap(), vec!["cache"]);
        assert_eq!(
            database.issue_cache_blockers("atelier-next").unwrap(),
            vec![id]
        );

        database
            .index_issue(
                &issue(id, "second", 3),
                &["schema".to_string()],
                &[],
                &[],
                &source("issue", id, "issues/atelier-issue.md", 3),
            )
            .unwrap();
        assert_eq!(
            database.issue_cache_row(id).unwrap().unwrap().title,
            "second"
        );
        assert_eq!(database.issue_cache_labels(id).unwrap(), vec!["schema"]);
        assert!(database
            .issue_cache_blockers("atelier-next")
            .unwrap()
            .is_empty());
        assert!(database.issue_cache_relations(id).unwrap().is_empty());
        assert_eq!(
            database
                .query_issue_cache(&IssueCacheQuery {
                    status: Some("todo"),
                    label: Some("schema"),
                    ..IssueCacheQuery::default()
                })
                .unwrap()
                .len(),
            1
        );

        let error = database
            .index_issue(
                &issue(id, "rolled back", 4),
                &["duplicate".to_string(), "duplicate".to_string()],
                &[],
                &[],
                &source("issue", id, "issues/atelier-issue.md", 4),
            )
            .unwrap_err();
        assert!(error.to_string().contains("UNIQUE"));
        assert_eq!(
            database.issue_cache_row(id).unwrap().unwrap().title,
            "second"
        );
        assert_eq!(
            database
                .record_source_cache_row("issues/atelier-issue.md")
                .unwrap()
                .unwrap()
                .size_bytes,
            3
        );

        database
            .remove_indexed_issue(id, "issues/atelier-issue.md")
            .unwrap();
        assert!(database.issue_cache_row(id).unwrap().is_none());
        assert!(database.record_source_cache_rows().unwrap().is_empty());
    }

    #[test]
    fn evidence_rows_add_replace_reverse_query_and_delete_atomically() {
        let (_directory, database) = database();
        let id = "atelier-proof";
        let target = EvidenceTargetCacheRow {
            evidence_id: id.to_string(),
            target_kind: "issue".to_string(),
            target_id: "atelier-issue".to_string(),
            role: "validates".to_string(),
        };
        database
            .index_evidence(
                &evidence(id, "first", 2),
                std::slice::from_ref(&target),
                &source("evidence", id, "evidence/atelier-proof.md", 2),
            )
            .unwrap();
        assert_eq!(
            database
                .evidence_cache_for_target("issue", "atelier-issue", Some("validates"))
                .unwrap()
                .len(),
            1
        );

        database
            .index_evidence(
                &evidence(id, "second", 3),
                &[],
                &source("evidence", id, "evidence/atelier-proof.md", 3),
            )
            .unwrap();
        assert_eq!(
            database.evidence_cache_row(id).unwrap().unwrap().title,
            "second"
        );
        assert!(database.evidence_cache_targets(id).unwrap().is_empty());
        assert_eq!(
            database
                .query_evidence_cache(&EvidenceCacheQuery {
                    status: Some("pass"),
                    evidence_type: Some("test"),
                })
                .unwrap()
                .len(),
            1
        );

        let duplicate = EvidenceTargetCacheRow {
            evidence_id: id.to_string(),
            ..target
        };
        let error = database
            .index_evidence(
                &evidence(id, "rolled back", 4),
                &[duplicate.clone(), duplicate],
                &source("evidence", id, "evidence/atelier-proof.md", 4),
            )
            .unwrap_err();
        assert!(error.to_string().contains("UNIQUE"));
        assert_eq!(
            database.evidence_cache_row(id).unwrap().unwrap().title,
            "second"
        );

        database
            .remove_indexed_evidence(id, "evidence/atelier-proof.md")
            .unwrap();
        assert!(database.evidence_cache_row(id).unwrap().is_none());
        assert!(database.record_source_cache_rows().unwrap().is_empty());
    }

    #[test]
    fn review_room_rows_add_replace_query_and_delete_atomically() {
        let (_directory, database) = database();
        let id = "atelier-review";
        database
            .index_review_room(
                &room(id, "first", 1),
                &source("review", id, "reviews/atelier-review.md", 1),
            )
            .unwrap();
        assert_eq!(
            database
                .review_room_cache_for_issue("atelier-issue")
                .unwrap()
                .len(),
            1
        );

        database
            .index_review_room(
                &room(id, "second", 2),
                &source("review", id, "reviews/atelier-review.md", 2),
            )
            .unwrap();
        assert_eq!(
            database.review_room_cache_row(id).unwrap().unwrap().title,
            "second"
        );

        let error = database
            .index_review_room(
                &room(id, "rolled back", 3),
                &source("review", "wrong-id", "reviews/atelier-review.md", 3),
            )
            .unwrap_err();
        assert!(error.to_string().contains("expected review/atelier-review"));
        assert_eq!(
            database.review_room_cache_row(id).unwrap().unwrap().title,
            "second"
        );

        database
            .remove_indexed_review_room(id, "reviews/atelier-review.md")
            .unwrap();
        assert!(database.review_room_cache_row(id).unwrap().is_none());
        assert!(database.record_source_cache_rows().unwrap().is_empty());
    }
}
