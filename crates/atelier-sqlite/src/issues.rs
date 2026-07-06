use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use rusqlite::params;

use super::{validate_issue_type, validate_priority, validate_status, Database, IssueCacheQuery};
use super::{MAX_DESCRIPTION_LEN, MAX_TITLE_LEN};
use crate::record_id;
use atelier_core::{Issue, IssuePriority};

fn issue_from_cache(row: super::IssueCacheRow) -> Issue {
    Issue {
        id: row.id,
        title: row.title,
        description: None,
        status: row.status,
        issue_type: row.issue_type,
        priority: row.priority,
        fields: Default::default(),
        parent_id: row.parent_id,
        created_at: row.created_at,
        updated_at: row.updated_at,
        closed_at: row.closed_at,
    }
}

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

        let fields_json = serde_json::to_string(&issue.fields)?;
        self.conn.execute(
            "INSERT INTO issues (id, title, description, status, issue_type, priority, fields_json, parent_id, created_at, updated_at, closed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                issue.id,
                issue.title,
                issue.description,
                issue.status,
                issue.issue_type,
                issue.priority,
                fields_json,
                issue.parent_id,
                issue.created_at.to_rfc3339(),
                issue.updated_at.to_rfc3339(),
                issue.closed_at.as_ref().map(DateTime::<Utc>::to_rfc3339),
            ],
        )?;
        Ok(())
    }

    pub fn insert_issue_import(&self, issue: &Issue) -> Result<()> {
        if let Some(description) = &issue.description {
            validate_description_length(description)?;
        }
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
            "INSERT INTO issues (id, title, description, priority, issue_type, parent_id, status, fields_json, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'todo', '{}', ?7, ?7)",
            params![id, title, description, priority, issue_type, parent_id, now],
        )?;
        Ok(id)
    }

    pub fn get_subissues(&self, parent_id: impl ToString) -> Result<Vec<Issue>> {
        let parent_id = parent_id.to_string();
        Ok(self
            .query_issue_cache(&IssueCacheQuery::default())?
            .into_iter()
            .filter(|issue| issue.parent_id.as_deref() == Some(parent_id.as_str()))
            .map(issue_from_cache)
            .collect())
    }

    pub fn get_issue(&self, id: impl ToString) -> Result<Option<Issue>> {
        let id = id.to_string();
        Ok(self.issue_cache_row(&id)?.map(issue_from_cache))
    }

    pub fn resolve_issue_ref(&self, issue_ref: &str) -> Result<Option<String>> {
        let normalized = issue_ref.trim();
        if normalized.is_empty() {
            return Ok(None);
        }

        if record_id::validate_record_id(normalized).is_ok()
            && self.get_issue(normalized)?.is_some()
        {
            return Ok(Some(normalized.to_string()));
        }

        if !is_partial_issue_key(normalized) {
            return Ok(None);
        }

        let suffix = format!("%-{normalized}");
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM issue_index WHERE id LIKE ?1 ORDER BY id LIMIT 2")?;
        let matches = stmt
            .query_map([suffix], |row| row.get::<_, String>(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        match matches.as_slice() {
            [id] => Ok(Some(id.clone())),
            [] => Ok(None),
            _ => bail!(
                "Issue key {normalized} is ambiguous: {}",
                matches.join(", ")
            ),
        }
    }

    /// Get an issue by ID, returning an error if not found.
    pub fn require_issue(&self, id: impl ToString) -> Result<Issue> {
        let id = id.to_string();
        self.get_issue(&id)?
            .ok_or_else(|| anyhow::anyhow!("Issue {} not found", format_issue_id(&id)))
    }

    pub fn list_issues(
        &self,
        status_filter: Option<&str>,
        label_filter: Option<&str>,
        priority_filter: Option<&str>,
    ) -> Result<Vec<Issue>> {
        let status = status_filter.filter(|status| *status != "all");
        if let Some(status) = status {
            validate_status(status)?;
        }
        let priority = priority_filter
            .map(IssuePriority::from_cli_input)
            .transpose()?
            .map(|priority| priority.label().to_string());
        Ok(self
            .query_issue_cache(&IssueCacheQuery {
                status,
                issue_type: None,
                priority: priority.as_deref(),
                label: label_filter,
            })?
            .into_iter()
            .map(issue_from_cache)
            .collect())
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
            "UPDATE issues SET status = 'done', closed_at = ?1, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(rows > 0)
    }
    pub fn reopen_issue(&self, id: impl ToString) -> Result<bool> {
        let id = id.to_string();
        let now = Utc::now().to_rfc3339();
        let rows = self.conn.execute(
            "UPDATE issues SET status = 'todo', closed_at = NULL, updated_at = ?1 WHERE id = ?2",
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

    /// Search issues by query string across titles and descriptions.
    pub fn search_issues(&self, query: &str) -> Result<Vec<Issue>> {
        let query = query.to_lowercase();
        Ok(self
            .list_issues(Some("all"), None, None)?
            .into_iter()
            .filter(|issue| issue.title.to_lowercase().contains(&query))
            .collect())
    }
}

fn format_issue_id(id: &str) -> String {
    id.to_string()
}

fn validate_description_length(description: &str) -> Result<()> {
    if description.len() <= MAX_DESCRIPTION_LEN {
        return Ok(());
    }

    bail!(
        "Description exceeds maximum length of {} bytes",
        MAX_DESCRIPTION_LEN
    )
}

fn is_partial_issue_key(value: &str) -> bool {
    !value.is_empty()
        && !value.contains('-')
        && value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
}
