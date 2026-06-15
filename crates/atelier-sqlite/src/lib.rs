//! SQLite projection and runtime contracts for Atelier.

use anyhow::{anyhow, bail, Context, Result};
use atelier_core::{DomainRecord, Issue, RecordLink};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

pub const CRATE_NAME: &str = "atelier-sqlite";

const MAX_PROBLEM_SAMPLES: usize = 5;
const SOURCE_RECORD_DIRS: &[&str] = &["issues", "missions", "milestones", "plans", "evidence"];
const ROOT_PROJECTION_SOURCES: &[&str] = &["workflow.yaml", "config.toml"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SqliteLayer;

pub fn dependency_crate_names() -> [&'static str; 3] {
    [
        atelier_core::crate_name(),
        atelier_records::CRATE_NAME,
        atelier_workflow::CRATE_NAME,
    ]
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceEntry {
    pub path: String,
    pub size_bytes: i64,
    pub modified_micros: Option<i64>,
    pub sha256: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FreshnessProblem {
    MissingMetadata { path: String },
    MissingSource { path: String },
    ChangedSource { path: String },
    UnindexedSource { path: String },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FreshnessReport {
    pub checked: bool,
    pub source_count: usize,
    pub problems: Vec<FreshnessProblem>,
}

impl FreshnessReport {
    pub fn is_fresh(&self) -> bool {
        self.problems.is_empty()
    }

    pub fn problem_messages(&self) -> Vec<String> {
        let mut messages = Vec::new();
        let mut missing_metadata = Vec::new();
        let mut missing_sources = Vec::new();
        let mut changed_sources = Vec::new();
        let mut unindexed_sources = Vec::new();

        for problem in &self.problems {
            match problem {
                FreshnessProblem::MissingMetadata { path } => missing_metadata.push(path.as_str()),
                FreshnessProblem::MissingSource { path } => missing_sources.push(path.as_str()),
                FreshnessProblem::ChangedSource { path } => changed_sources.push(path.as_str()),
                FreshnessProblem::UnindexedSource { path } => unindexed_sources.push(path.as_str()),
            }
        }

        push_missing_metadata_message(&mut messages, &missing_metadata);
        push_path_group_message(
            &mut messages,
            &missing_sources,
            "indexed source is missing",
            "indexed sources are missing",
        );
        push_path_group_message(
            &mut messages,
            &changed_sources,
            "indexed source changed",
            "indexed sources changed",
        );
        push_path_group_message(
            &mut messages,
            &unindexed_sources,
            "canonical source is not indexed",
            "canonical sources are not indexed",
        );
        if !messages.is_empty() {
            messages.push(
                "recovery: 1. run `atelier lint`; 2. fix any named canonical Markdown records; 3. run `atelier doctor --fix` to repair local runtime/projection state; 4. rerun the blocked command"
                    .to_string(),
            );
        }
        messages
    }
}

fn push_missing_metadata_message(messages: &mut Vec<String>, paths: &[&str]) {
    match paths {
        [] => {}
        [path] => messages.push(format!(
            "runtime projection metadata is missing for canonical source: {path}"
        )),
        _ => messages.push(format!(
            "runtime projection metadata is missing for {} canonical sources (showing first {}): {}",
            paths.len(),
            paths.len().min(MAX_PROBLEM_SAMPLES),
            paths
                .iter()
                .take(MAX_PROBLEM_SAMPLES)
                .copied()
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

fn push_path_group_message(
    messages: &mut Vec<String>,
    paths: &[&str],
    singular: &str,
    plural: &str,
) {
    match paths {
        [] => {}
        [path] => messages.push(format!("{singular}: {path}")),
        _ => messages.push(format!(
            "{} {} (showing first {}): {}",
            paths.len(),
            plural,
            paths.len().min(MAX_PROBLEM_SAMPLES),
            paths
                .iter()
                .take(MAX_PROBLEM_SAMPLES)
                .copied()
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MissionWorkSummary {
    pub mission_id: String,
    pub ready: usize,
    pub blocked: usize,
    pub done: usize,
    pub backlog: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WorkflowStatusRow {
    pub status: String,
    pub category: String,
}

pub struct ProjectionIndex<'a> {
    conn: &'a Connection,
}

impl<'a> ProjectionIndex<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn init_schema(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS projection_sources (
                path TEXT PRIMARY KEY,
                kind TEXT,
                record_id TEXT,
                size_bytes INTEGER NOT NULL,
                modified_micros INTEGER,
                sha256 TEXT NOT NULL,
                indexed_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_projection_sources_hash
                ON projection_sources(sha256);

            CREATE TABLE IF NOT EXISTS projection_workflow_statuses (
                status TEXT PRIMARY KEY,
                category TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS projection_issue_search (
                issue_id TEXT PRIMARY KEY,
                search_text TEXT NOT NULL
            );
            "#,
        )?;
        Ok(())
    }

    pub fn replace_sources(&self, entries: &[SourceEntry]) -> Result<()> {
        let indexed_at = Utc::now().to_rfc3339();
        self.conn.execute("DELETE FROM projection_sources", [])?;
        for entry in entries {
            let (kind, record_id) = source_kind_and_record_id(&entry.path);
            self.conn.execute(
                "INSERT INTO projection_sources
                 (path, kind, record_id, size_bytes, modified_micros, sha256, indexed_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    entry.path,
                    kind,
                    record_id,
                    entry.size_bytes,
                    entry.modified_micros,
                    entry.sha256,
                    indexed_at
                ],
            )?;
        }
        Ok(())
    }

    pub fn sources(&self) -> Result<Vec<SourceEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT path, size_bytes, modified_micros, sha256
             FROM projection_sources
             ORDER BY path",
        )?;
        let entries = stmt
            .query_map([], source_entry_from_row)?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(entries)
    }

    pub fn refresh_sources(&self, state_dir: &Path) -> Result<()> {
        self.replace_sources(&snapshot_sources(state_dir)?)
    }

    pub fn check_freshness(&self, state_dir: &Path) -> Result<FreshnessReport> {
        if !state_dir.exists() {
            return Ok(FreshnessReport {
                checked: false,
                source_count: 0,
                problems: Vec::new(),
            });
        }

        let current = snapshot_sources(state_dir)?;
        let stored = self.sources()?;
        let current_by_path = current
            .iter()
            .map(|entry| (entry.path.clone(), entry))
            .collect::<BTreeMap<_, _>>();
        let stored_by_path = stored
            .iter()
            .map(|entry| (entry.path.clone(), entry))
            .collect::<BTreeMap<_, _>>();

        let mut problems = Vec::new();
        if stored.is_empty() && !current.is_empty() {
            for entry in &current {
                problems.push(FreshnessProblem::MissingMetadata {
                    path: entry.path.clone(),
                });
            }
        } else {
            for stored_entry in &stored {
                match current_by_path.get(&stored_entry.path) {
                    Some(current_entry) if current_entry.sha256 == stored_entry.sha256 => {}
                    Some(_) => problems.push(FreshnessProblem::ChangedSource {
                        path: stored_entry.path.clone(),
                    }),
                    None => problems.push(FreshnessProblem::MissingSource {
                        path: stored_entry.path.clone(),
                    }),
                }
            }
            for current_entry in &current {
                if !stored_by_path.contains_key(&current_entry.path) {
                    problems.push(FreshnessProblem::UnindexedSource {
                        path: current_entry.path.clone(),
                    });
                }
            }
        }

        Ok(FreshnessReport {
            checked: true,
            source_count: current.len(),
            problems,
        })
    }

    pub fn issue(&self, id: &str) -> Result<Option<Issue>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, status, issue_type, priority, parent_id, created_at, updated_at, closed_at
             FROM issues WHERE id = ?1",
        )?;
        Ok(stmt.query_row([id], issue_from_row).ok())
    }

    pub fn list_issues(
        &self,
        status_filter: Option<&str>,
        label_filter: Option<&str>,
        priority_filter: Option<&str>,
    ) -> Result<Vec<Issue>> {
        let mut sql = String::from(
            "SELECT DISTINCT i.id, i.title, i.description, i.status, i.issue_type, i.priority, i.parent_id, i.created_at, i.updated_at, i.closed_at FROM issues i",
        );
        let mut conditions = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if label_filter.is_some() {
            sql.push_str(" JOIN labels l ON i.id = l.issue_id");
        }

        if let Some(status) = status_filter {
            if status != "all" {
                conditions.push("i.status = ?".to_string());
                params_vec.push(Box::new(status.to_string()));
            }
        }
        if let Some(label) = label_filter {
            conditions.push("l.label = ?".to_string());
            params_vec.push(Box::new(label.to_string()));
        }
        if let Some(priority) = priority_filter {
            conditions.push("i.priority = ?".to_string());
            params_vec.push(Box::new(priority.to_string()));
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }
        sql.push_str(" ORDER BY i.id DESC");

        let mut stmt = self.conn.prepare(&sql)?;
        let params_refs = params_vec.iter().map(|p| p.as_ref()).collect::<Vec<_>>();
        let rows = stmt.query_map(params_refs.as_slice(), issue_from_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn ready_issues(&self) -> Result<Vec<Issue>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT i.id, i.title, i.description, i.status, i.issue_type, i.priority, i.parent_id, i.created_at, i.updated_at, i.closed_at
            FROM issues i
            WHERE i.status NOT IN ('done', 'archived')
            AND NOT EXISTS (
                SELECT 1 FROM dependencies d
                JOIN issues blocker ON d.blocker_id = blocker.id
                WHERE d.blocked_id = i.id AND blocker.status NOT IN ('done', 'archived')
            )
            ORDER BY i.id
            "#,
        )?;
        let rows = stmt.query_map([], issue_from_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn blocked_issues(&self) -> Result<Vec<Issue>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT DISTINCT i.id, i.title, i.description, i.status, i.issue_type, i.priority, i.parent_id, i.created_at, i.updated_at, i.closed_at
            FROM issues i
            JOIN dependencies d ON i.id = d.blocked_id
            JOIN issues blocker ON d.blocker_id = blocker.id
            WHERE i.status NOT IN ('done', 'archived') AND blocker.status NOT IN ('done', 'archived')
            ORDER BY i.id
            "#,
        )?;
        let rows = stmt.query_map([], issue_from_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn blockers(&self, issue_id: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT blocker_id FROM dependencies WHERE blocked_id = ?1 ORDER BY blocker_id",
        )?;
        let rows = stmt.query_map([issue_id], |row| row.get(0))?;
        rows.collect::<std::result::Result<Vec<String>, _>>()
            .map_err(Into::into)
    }

    pub fn blocking(&self, issue_id: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT blocked_id FROM dependencies WHERE blocker_id = ?1 ORDER BY blocked_id",
        )?;
        let rows = stmt.query_map([issue_id], |row| row.get(0))?;
        rows.collect::<std::result::Result<Vec<String>, _>>()
            .map_err(Into::into)
    }

    pub fn search_issues(&self, query: &str) -> Result<Vec<Issue>> {
        let escaped = query.replace('%', "\\%").replace('_', "\\_");
        let pattern = format!("%{}%", escaped);
        let mut stmt = self.conn.prepare(
            r#"
            SELECT DISTINCT i.id, i.title, i.description, i.status, i.issue_type, i.priority, i.parent_id, i.created_at, i.updated_at, i.closed_at
            FROM issues i
            LEFT JOIN projection_issue_search s ON i.id = s.issue_id
            LEFT JOIN comments c ON i.id = c.issue_id
            WHERE i.title LIKE ?1 ESCAPE '\' COLLATE NOCASE
               OR i.description LIKE ?1 ESCAPE '\' COLLATE NOCASE
               OR s.search_text LIKE ?1 ESCAPE '\' COLLATE NOCASE
               OR c.content LIKE ?1 ESCAPE '\' COLLATE NOCASE
            ORDER BY i.id DESC
            "#,
        )?;
        let rows = stmt.query_map([&pattern], issue_from_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn replace_issue_search_text(&self, issue_id: &str, search_text: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO projection_issue_search (issue_id, search_text) VALUES (?1, ?2)",
            params![issue_id, search_text],
        )?;
        Ok(())
    }

    pub fn record(&self, kind: &str, id: &str) -> Result<Option<DomainRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, kind, title, status, body, data_json, created_at, updated_at
             FROM records WHERE kind = ?1 AND id = ?2",
        )?;
        Ok(stmt.query_row(params![kind, id], record_from_row).ok())
    }

    pub fn list_records(&self, kind: &str, status: Option<&str>) -> Result<Vec<DomainRecord>> {
        let (sql, params): (&str, Vec<Box<dyn rusqlite::ToSql>>) = match status {
            Some(status) => (
                "SELECT id, kind, title, status, body, data_json, created_at, updated_at
                 FROM records WHERE kind = ?1 AND status = ?2 ORDER BY id",
                vec![Box::new(kind.to_string()), Box::new(status.to_string())],
            ),
            None => (
                "SELECT id, kind, title, status, body, data_json, created_at, updated_at
                 FROM records WHERE kind = ?1 ORDER BY id",
                vec![Box::new(kind.to_string())],
            ),
        };
        let param_refs = params.iter().map(|p| p.as_ref()).collect::<Vec<_>>();
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map(param_refs.as_slice(), record_from_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn record_kind_for_id(&self, id: &str) -> Result<Option<String>> {
        if self.issue(id)?.is_some() {
            return Ok(Some("issue".to_string()));
        }
        let mut stmt = self
            .conn
            .prepare("SELECT kind FROM records WHERE id = ?1 ORDER BY kind LIMIT 1")?;
        Ok(stmt.query_row(params![id], |row| row.get(0)).ok())
    }

    pub fn record_links(&self, kind: &str, id: &str) -> Result<Vec<RecordLink>> {
        let mut stmt = self.conn.prepare(
            "SELECT source_kind, source_id, target_kind, target_id, relation_type, created_at
             FROM record_links
             WHERE (source_kind = ?1 AND source_id = ?2) OR (target_kind = ?1 AND target_id = ?2)
             ORDER BY created_at, source_kind, source_id, target_kind, target_id, relation_type",
        )?;
        let rows = stmt.query_map(params![kind, id], record_link_from_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn all_record_links(&self) -> Result<Vec<RecordLink>> {
        let mut stmt = self.conn.prepare(
            "SELECT source_kind, source_id, target_kind, target_id, relation_type, created_at
             FROM record_links
             ORDER BY source_kind, source_id, target_kind, target_id, relation_type",
        )?;
        let rows = stmt.query_map([], record_link_from_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn mission_work_summary(&self, mission_id: &str) -> Result<MissionWorkSummary> {
        let mut summary = MissionWorkSummary {
            mission_id: mission_id.to_string(),
            ready: 0,
            blocked: 0,
            done: 0,
            backlog: 0,
        };
        let mut stmt = self.conn.prepare(
            r#"
            SELECT i.id, i.status
            FROM record_links l
            JOIN issues i ON i.id = l.target_id
            WHERE l.source_kind = 'mission'
              AND l.source_id = ?1
              AND l.target_kind = 'issue'
              AND l.relation_type = 'advances'
            ORDER BY i.id
            "#,
        )?;
        let rows = stmt.query_map([mission_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            let (issue_id, status) = row?;
            if matches!(status.as_str(), "done" | "archived") {
                summary.done += 1;
            } else {
                let has_open_blocker = self
                    .blockers(&issue_id)?
                    .into_iter()
                    .any(|blocker_id| !self.issue_done_or_archived(&blocker_id).unwrap_or(false));
                if has_open_blocker {
                    summary.blocked += 1;
                } else if status == "todo" {
                    summary.ready += 1;
                } else {
                    summary.backlog += 1;
                }
            }
        }
        Ok(summary)
    }

    pub fn replace_workflow_statuses(&self, statuses: &[WorkflowStatusRow]) -> Result<()> {
        self.conn
            .execute("DELETE FROM projection_workflow_statuses", [])?;
        for row in statuses {
            self.conn.execute(
                "INSERT INTO projection_workflow_statuses (status, category) VALUES (?1, ?2)",
                params![row.status, row.category],
            )?;
        }
        Ok(())
    }

    pub fn workflow_statuses(&self) -> Result<Vec<WorkflowStatusRow>> {
        let mut stmt = self
            .conn
            .prepare("SELECT status, category FROM projection_workflow_statuses ORDER BY status")?;
        let rows = stmt.query_map([], |row| {
            Ok(WorkflowStatusRow {
                status: row.get(0)?,
                category: row.get(1)?,
            })
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn issue_done_or_archived(&self, id: &str) -> Result<bool> {
        Ok(self
            .issue(id)?
            .is_some_and(|issue| matches!(issue.status.as_str(), "done" | "archived")))
    }
}

pub fn snapshot_sources(state_dir: &Path) -> Result<Vec<SourceEntry>> {
    if !state_dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    for file in ROOT_PROJECTION_SOURCES {
        let path = state_dir.join(file);
        if path.is_file() {
            entries.push(source_entry(state_dir, &path)?);
        }
    }
    for dir in SOURCE_RECORD_DIRS {
        let source_dir = state_dir.join(dir);
        if source_dir.exists() {
            collect_source_files(state_dir, &source_dir, &mut entries)?;
        }
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(entries)
}

fn collect_source_files(root: &Path, dir: &Path, entries: &mut Vec<SourceEntry>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(".activity"))
            {
                continue;
            }
            collect_source_files(root, &path, entries)?;
        } else if path.is_file() {
            entries.push(source_entry(root, &path)?);
        }
    }
    Ok(())
}

fn source_entry(root: &Path, path: &Path) -> Result<SourceEntry> {
    let relative = path
        .strip_prefix(root)
        .with_context(|| format!("Failed to relativize {}", path.display()))?;
    let relative = canonical_relative_path(relative)?;
    let metadata =
        fs::metadata(path).with_context(|| format!("Failed to stat {}", path.display()))?;
    let bytes = fs::read(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(SourceEntry {
        path: relative,
        size_bytes: metadata
            .len()
            .try_into()
            .context("source file is too large")?,
        modified_micros: metadata
            .modified()
            .ok()
            .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
            .map(|duration| {
                (duration.as_secs() as i64)
                    .saturating_mul(1_000_000)
                    .saturating_add(i64::from(duration.subsec_micros()))
            }),
        sha256: format!("{:x}", hasher.finalize()),
    })
}

fn canonical_relative_path(path: &Path) -> Result<String> {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::Normal(part) => parts.push(
                part.to_str()
                    .ok_or_else(|| {
                        anyhow!("canonical state path is not UTF-8: {}", path.display())
                    })?
                    .to_string(),
            ),
            _ => bail!("canonical state path is not relative: {}", path.display()),
        }
    }
    Ok(parts.join("/"))
}

fn source_kind_and_record_id(path: &str) -> (Option<String>, Option<String>) {
    let mut parts = path.split('/');
    let kind = match parts.next() {
        Some("issues") => Some("issue".to_string()),
        Some("missions") => Some("mission".to_string()),
        Some("milestones") => Some("milestone".to_string()),
        Some("plans") => Some("plan".to_string()),
        Some("evidence") => Some("evidence".to_string()),
        Some("workflow.yaml") => Some("workflow".to_string()),
        Some("config.toml") => Some("config".to_string()),
        _ => None,
    };
    let record_id = PathBuf::from(path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(str::to_string);
    (kind, record_id)
}

fn source_entry_from_row(row: &rusqlite::Row) -> rusqlite::Result<SourceEntry> {
    Ok(SourceEntry {
        path: row.get(0)?,
        size_bytes: row.get(1)?,
        modified_micros: row.get(2)?,
        sha256: row.get(3)?,
    })
}

fn issue_from_row(row: &rusqlite::Row) -> rusqlite::Result<Issue> {
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

fn record_from_row(row: &rusqlite::Row) -> rusqlite::Result<DomainRecord> {
    Ok(DomainRecord {
        id: row.get(0)?,
        kind: row.get(1)?,
        title: row.get(2)?,
        status: row.get(3)?,
        body: row.get(4)?,
        data_json: row.get(5)?,
        created_at: parse_datetime(row.get::<_, String>(6)?),
        updated_at: parse_datetime(row.get::<_, String>(7)?),
    })
}

fn record_link_from_row(row: &rusqlite::Row) -> rusqlite::Result<RecordLink> {
    Ok(RecordLink {
        source_kind: row.get(0)?,
        source_id: row.get(1)?,
        target_kind: row.get(2)?,
        target_id: row.get(3)?,
        relation_type: row.get(4)?,
        created_at: parse_datetime(row.get::<_, String>(5)?),
    })
}

fn parse_datetime(s: String) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn open_projection(path: &Path) -> Connection {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let conn = Connection::open(path).unwrap();
        ProjectionIndex::init_schema(&conn).unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE issues (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                issue_type TEXT NOT NULL,
                priority TEXT NOT NULL,
                parent_id TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                closed_at TEXT
            );
            CREATE TABLE labels (issue_id TEXT NOT NULL, label TEXT NOT NULL);
            CREATE TABLE dependencies (blocker_id TEXT NOT NULL, blocked_id TEXT NOT NULL);
            CREATE TABLE comments (id INTEGER PRIMARY KEY AUTOINCREMENT, issue_id TEXT NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL);
            CREATE TABLE records (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                title TEXT NOT NULL,
                status TEXT NOT NULL,
                body TEXT,
                data_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE record_links (
                source_kind TEXT NOT NULL,
                source_id TEXT NOT NULL,
                target_kind TEXT NOT NULL,
                target_id TEXT NOT NULL,
                relation_type TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            "#,
        )
        .unwrap();
        conn
    }

    fn insert_issue(conn: &Connection, id: &str, title: &str, status: &str) {
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO issues
             (id, title, description, status, issue_type, priority, parent_id, created_at, updated_at, closed_at)
             VALUES (?1, ?2, ?3, ?4, 'task', 'high', NULL, ?5, ?5, NULL)",
            params![id, title, format!("Body for {title}"), status, now],
        )
        .unwrap();
    }

    #[test]
    fn freshness_reports_changed_missing_and_unindexed_sources() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let issues = state_dir.join("issues");
        fs::create_dir_all(&issues).unwrap();
        fs::write(issues.join("atelier-aaaa.md"), "one").unwrap();
        fs::write(issues.join("atelier-bbbb.md"), "two").unwrap();
        fs::write(state_dir.join("workflow.yaml"), "statuses: []\n").unwrap();
        let conn = open_projection(&dir.path().join(".atelier/runtime/state.db"));
        let projection = ProjectionIndex::new(&conn);

        projection.refresh_sources(&state_dir).unwrap();
        assert!(projection.check_freshness(&state_dir).unwrap().is_fresh());

        fs::write(issues.join("atelier-aaaa.md"), "changed").unwrap();
        fs::remove_file(issues.join("atelier-bbbb.md")).unwrap();
        fs::write(issues.join("atelier-cccc.md"), "new").unwrap();

        let report = projection.check_freshness(&state_dir).unwrap();
        assert_eq!(report.source_count, 3);
        assert!(report.problems.contains(&FreshnessProblem::ChangedSource {
            path: "issues/atelier-aaaa.md".to_string()
        }));
        assert!(report.problems.contains(&FreshnessProblem::MissingSource {
            path: "issues/atelier-bbbb.md".to_string()
        }));
        assert!(report
            .problems
            .contains(&FreshnessProblem::UnindexedSource {
                path: "issues/atelier-cccc.md".to_string()
            }));
    }

    #[test]
    fn source_snapshot_includes_workflow_and_skips_activity_sidecars() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        fs::create_dir_all(state_dir.join("issues/atelier-aaaa.activity")).unwrap();
        fs::write(state_dir.join("issues/atelier-aaaa.md"), "issue").unwrap();
        fs::write(
            state_dir.join("issues/atelier-aaaa.activity/20260615T000000000000Z.md"),
            "activity",
        )
        .unwrap();
        fs::write(state_dir.join("workflow.yaml"), "workflow").unwrap();
        fs::write(state_dir.join("config.toml"), "config").unwrap();

        let paths = snapshot_sources(&state_dir)
            .unwrap()
            .into_iter()
            .map(|entry| entry.path)
            .collect::<Vec<_>>();

        assert_eq!(
            paths,
            vec![
                "config.toml".to_string(),
                "issues/atelier-aaaa.md".to_string(),
                "workflow.yaml".to_string()
            ]
        );
    }

    #[test]
    fn query_api_covers_ready_search_graph_workflow_and_mission_summary() {
        let dir = tempdir().unwrap();
        let conn = open_projection(&dir.path().join("state.db"));
        let projection = ProjectionIndex::new(&conn);
        insert_issue(&conn, "atelier-ready", "Ready work", "todo");
        insert_issue(&conn, "atelier-blocker", "Blocking work", "todo");
        insert_issue(&conn, "atelier-blocked", "Blocked work", "todo");
        insert_issue(&conn, "atelier-done", "Finished work", "done");
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO dependencies (blocker_id, blocked_id) VALUES ('atelier-blocker', 'atelier-blocked')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO records (id, kind, title, status, body, data_json, created_at, updated_at)
             VALUES ('atelier-mission', 'mission', 'Mission', 'ready', NULL, '{}', ?1, ?1)",
            [&now],
        )
        .unwrap();
        for issue in [
            "atelier-ready",
            "atelier-blocked",
            "atelier-done",
            "atelier-blocker",
        ] {
            conn.execute(
                "INSERT INTO record_links
                 (source_kind, source_id, target_kind, target_id, relation_type, created_at)
                 VALUES ('mission', 'atelier-mission', 'issue', ?1, 'advances', ?2)",
                params![issue, now],
            )
            .unwrap();
        }
        projection
            .replace_workflow_statuses(&[
                WorkflowStatusRow {
                    status: "todo".to_string(),
                    category: "todo".to_string(),
                },
                WorkflowStatusRow {
                    status: "done".to_string(),
                    category: "done".to_string(),
                },
            ])
            .unwrap();
        projection
            .replace_issue_search_text("atelier-ready", "Outcome search marker")
            .unwrap();

        assert_eq!(
            projection
                .ready_issues()
                .unwrap()
                .into_iter()
                .map(|issue| issue.id)
                .collect::<Vec<_>>(),
            vec!["atelier-blocker".to_string(), "atelier-ready".to_string()]
        );
        assert_eq!(
            projection.search_issues("marker").unwrap()[0].id,
            "atelier-ready"
        );
        assert_eq!(
            projection.blockers("atelier-blocked").unwrap(),
            vec!["atelier-blocker".to_string()]
        );
        assert_eq!(projection.workflow_statuses().unwrap().len(), 2);
        assert_eq!(
            projection.mission_work_summary("atelier-mission").unwrap(),
            MissionWorkSummary {
                mission_id: "atelier-mission".to_string(),
                ready: 2,
                blocked: 1,
                done: 1,
                backlog: 0,
            }
        );
    }
}
