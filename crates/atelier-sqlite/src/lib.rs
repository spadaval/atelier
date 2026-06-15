//! SQLite projection and runtime-state boundary.
//!
//! Rebuild freshness, query indexes, graph/search/readiness queries, and local
//! runtime recovery move here during the migration.

use anyhow::{Context, Result};
pub use atelier_core::RecordId;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;

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

    pub fn add_comment(&self, issue_id: &str, content: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO comments (issue_id, content) VALUES (?1, ?2)",
            params![issue_id, content],
        )?;
        Ok(())
    }

    pub fn get_comments(&self, issue_id: &str) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT content FROM comments WHERE issue_id = ?1 ORDER BY rowid ASC")?;
        let comments = stmt
            .query_map([issue_id], |row| row.get(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(comments)
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
            CREATE TABLE IF NOT EXISTS comments (
                issue_id TEXT NOT NULL,
                content TEXT NOT NULL
            );
            "#,
        )?;
        Ok(())
    }
}

/// Table ownership in the single rebuildable runtime database.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TableOwner {
    Projection,
    Runtime,
}

pub const PROJECTION_TABLES: &[&str] = &[
    "issues",
    "labels",
    "dependencies",
    "relations",
    "records",
    "record_links",
    "projection_index_sources",
];

pub const RUNTIME_TABLES: &[&str] = &["runtime_metadata"];

pub fn table_owner(table: &str) -> Option<TableOwner> {
    if PROJECTION_TABLES.contains(&table) {
        Some(TableOwner::Projection)
    } else if RUNTIME_TABLES.contains(&table) {
        Some(TableOwner::Runtime)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_tables_have_explicit_ownership() {
        assert_eq!(table_owner("issues"), Some(TableOwner::Projection));
        assert_eq!(table_owner("runtime_metadata"), Some(TableOwner::Runtime));
    }

    #[test]
    fn removed_tables_are_not_part_of_target_schema() {
        assert_eq!(table_owner("sessions"), None);
        assert_eq!(table_owner("work_associations"), None);
        assert_eq!(table_owner("claims"), None);
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
    }
}
