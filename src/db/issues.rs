use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::params;

use super::{issue_from_row, validate_issue_type, validate_priority, validate_status, Database};
use super::{MAX_DESCRIPTION_LEN, MAX_TITLE_LEN};
use crate::models::Issue;
use crate::record_id;

impl Database {
    pub fn insert_issue_rebuild(&self, issue: &Issue) -> Result<()> {
        validate_priority(&issue.priority)?;
        validate_status(&issue.status)?;
        validate_issue_type(&issue.issue_type)?;
        if issue.title.len() > MAX_TITLE_LEN {
            anyhow::bail!(
                "Title exceeds maximum length of {} characters",
                MAX_TITLE_LEN
            );
        }
        if let Some(description) = &issue.description {
            if description.len() > MAX_DESCRIPTION_LEN {
                anyhow::bail!(
                    "Description exceeds maximum length of {} bytes",
                    MAX_DESCRIPTION_LEN
                );
            }
        }

        self.conn.execute(
            "INSERT INTO issues (id, title, description, status, issue_type, priority, parent_id, created_at, updated_at, closed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                issue.id,
                issue.title,
                issue.description,
                issue.status,
                issue.issue_type,
                issue.priority,
                issue.parent_id,
                issue.created_at.to_rfc3339(),
                issue.updated_at.to_rfc3339(),
                issue.closed_at.as_ref().map(DateTime::<Utc>::to_rfc3339),
            ],
        )?;
        Ok(())
    }

    pub fn insert_issue_import(&self, issue: &Issue) -> Result<()> {
        self.insert_issue_rebuild(issue)?;
        Ok(())
    }

    pub fn create_issue(
        &self,
        title: &str,
        description: Option<&str>,
        priority: &str,
    ) -> Result<String> {
        self.create_issue_with_parent(title, description, priority, "task", None)
    }

    pub fn create_subissue(
        &self,
        parent_id: &str,
        title: &str,
        description: Option<&str>,
        priority: &str,
    ) -> Result<String> {
        self.create_issue_with_parent(title, description, priority, "task", Some(parent_id))
    }

    pub fn create_issue_with_type(
        &self,
        title: &str,
        description: Option<&str>,
        priority: &str,
        issue_type: &str,
    ) -> Result<String> {
        self.create_issue_with_parent(title, description, priority, issue_type, None)
    }

    pub fn create_subissue_with_type(
        &self,
        parent_id: &str,
        title: &str,
        description: Option<&str>,
        priority: &str,
        issue_type: &str,
    ) -> Result<String> {
        self.create_issue_with_parent(title, description, priority, issue_type, Some(parent_id))
    }

    fn create_issue_with_parent(
        &self,
        title: &str,
        description: Option<&str>,
        priority: &str,
        issue_type: &str,
        parent_id: Option<&str>,
    ) -> Result<String> {
        validate_priority(priority)?;
        validate_issue_type(issue_type)?;
        if title.len() > MAX_TITLE_LEN {
            anyhow::bail!(
                "Title exceeds maximum length of {} characters",
                MAX_TITLE_LEN
            );
        }
        if let Some(d) = description {
            if d.len() > MAX_DESCRIPTION_LEN {
                anyhow::bail!(
                    "Description exceeds maximum length of {} bytes",
                    MAX_DESCRIPTION_LEN
                );
            }
        }
        if let Some(parent_id) = parent_id {
            record_id::validate_record_id(parent_id)?;
        }
        let id =
            record_id::allocate_issue_id(|candidate| Ok(self.get_issue(candidate)?.is_some()))?;
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO issues (id, title, description, priority, issue_type, parent_id, status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'open', ?7, ?7)",
            params![id, title, description, priority, issue_type, parent_id, now],
        )?;
        Ok(id)
    }

    pub fn get_subissues(&self, parent_id: impl ToString) -> Result<Vec<Issue>> {
        let parent_id = parent_id.to_string();
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, status, issue_type, priority, parent_id, created_at, updated_at, closed_at FROM issues WHERE parent_id = ?1 ORDER BY id",
        )?;

        let issues = stmt
            .query_map([parent_id], issue_from_row)?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(issues)
    }

    pub fn get_issue(&self, id: impl ToString) -> Result<Option<Issue>> {
        let id = id.to_string();
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, status, issue_type, priority, parent_id, created_at, updated_at, closed_at FROM issues WHERE id = ?1",
        )?;

        let issue = stmt.query_row([id], issue_from_row).ok();

        Ok(issue)
    }

    pub fn resolve_issue_ref(&self, issue_ref: &str) -> Result<Option<String>> {
        let normalized = issue_ref.trim();
        if normalized.is_empty() {
            return Ok(None);
        }

        if record_id::validate_record_id(normalized).is_err() {
            return Ok(None);
        }

        if self.get_issue(normalized)?.is_some() {
            return Ok(Some(normalized.to_string()));
        }

        Ok(None)
    }

    /// Get an issue by ID, returning an error if not found.
    pub fn require_issue(&self, id: impl ToString) -> Result<Issue> {
        let id = id.to_string();
        self.get_issue(&id)?.ok_or_else(|| {
            anyhow::anyhow!("Issue {} not found", crate::utils::format_issue_id(&id))
        })
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
                validate_status(status)?;
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
        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();

        let issues = stmt
            .query_map(params_refs.as_slice(), issue_from_row)?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(issues)
    }

    pub fn update_issue(
        &self,
        id: impl ToString,
        title: Option<&str>,
        description: Option<&str>,
        priority: Option<&str>,
    ) -> Result<bool> {
        if let Some(t) = title {
            if t.len() > MAX_TITLE_LEN {
                anyhow::bail!(
                    "Title exceeds maximum length of {} characters",
                    MAX_TITLE_LEN
                );
            }
        }
        if let Some(d) = description {
            if d.len() > MAX_DESCRIPTION_LEN {
                anyhow::bail!(
                    "Description exceeds maximum length of {} bytes",
                    MAX_DESCRIPTION_LEN
                );
            }
        }
        if let Some(p) = priority {
            validate_priority(p)?;
        }

        let now = Utc::now().to_rfc3339();
        let mut updates = vec!["updated_at = ?1".to_string()];
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(now)];

        if let Some(t) = title {
            updates.push(format!("title = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(t.to_string()));
        }

        if let Some(d) = description {
            updates.push(format!("description = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(d.to_string()));
        }

        if let Some(p) = priority {
            updates.push(format!("priority = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(p.to_string()));
        }

        params_vec.push(Box::new(id.to_string()));
        let sql = format!(
            "UPDATE issues SET {} WHERE id = ?{}",
            updates.join(", "),
            params_vec.len()
        );

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();
        let rows = self.conn.execute(&sql, params_refs.as_slice())?;
        Ok(rows > 0)
    }

    pub fn close_issue(&self, id: impl ToString) -> Result<bool> {
        let id = id.to_string();
        let now = Utc::now().to_rfc3339();
        let rows = self.conn.execute(
            "UPDATE issues SET status = 'closed', closed_at = ?1, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(rows > 0)
    }

    pub fn reopen_issue(&self, id: impl ToString) -> Result<bool> {
        let id = id.to_string();
        let now = Utc::now().to_rfc3339();
        let rows = self.conn.execute(
            "UPDATE issues SET status = 'open', closed_at = NULL, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(rows > 0)
    }

    pub fn delete_issue(&self, id: impl ToString) -> Result<bool> {
        let id = id.to_string();
        let rows = self
            .conn
            .execute("DELETE FROM issues WHERE id = ?1", [id])?;
        Ok(rows > 0)
    }

    pub fn update_parent(&self, id: impl ToString, parent_id: Option<&str>) -> Result<bool> {
        let id = id.to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let rows = self.conn.execute(
            "UPDATE issues SET parent_id = ?1, updated_at = ?2 WHERE id = ?3",
            params![parent_id, now, id],
        )?;
        Ok(rows > 0)
    }

    pub fn update_parent_import(
        &self,
        id: impl ToString,
        parent_id: Option<&str>,
        updated_at: &DateTime<Utc>,
    ) -> Result<bool> {
        let id = id.to_string();
        let rows = self.conn.execute(
            "UPDATE issues SET parent_id = ?1, updated_at = ?2 WHERE id = ?3",
            params![parent_id, updated_at.to_rfc3339(), id],
        )?;
        Ok(rows > 0)
    }

    /// Search issues by query string across titles, descriptions, and comments
    pub fn search_issues(&self, query: &str) -> Result<Vec<Issue>> {
        let escaped = query.replace('%', "\\%").replace('_', "\\_");
        let pattern = format!("%{}%", escaped);
        let mut stmt = self.conn.prepare(
            r#"
            SELECT DISTINCT i.id, i.title, i.description, i.status, i.issue_type, i.priority, i.parent_id, i.created_at, i.updated_at, i.closed_at
            FROM issues i
            LEFT JOIN comments c ON i.id = c.issue_id
            WHERE i.title LIKE ?1 ESCAPE '\' COLLATE NOCASE
               OR i.description LIKE ?1 ESCAPE '\' COLLATE NOCASE
               OR c.content LIKE ?1 ESCAPE '\' COLLATE NOCASE
            ORDER BY i.id DESC
            "#,
        )?;

        let issues = stmt
            .query_map([&pattern], issue_from_row)?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(issues)
    }
}
