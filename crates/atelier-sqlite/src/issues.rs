use anyhow::{bail, Result};

use super::{validate_status, Database, IssueCacheQuery};
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
        fields: row.fields,
        parent_id: row.parent_id,
        created_at: row.created_at,
        updated_at: row.updated_at,
        closed_at: row.closed_at,
    }
}

impl Database {
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

fn is_partial_issue_key(value: &str) -> bool {
    !value.is_empty()
        && !value.contains('-')
        && value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
}
