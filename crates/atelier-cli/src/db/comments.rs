use anyhow::Result;
#[cfg(test)]
use chrono::Utc;

use super::{Database, MAX_COMMENT_LEN};
use crate::activity::{create_issue_activity, list_issue_activities, ActivityEventType};
use crate::models::Comment;

impl Database {
    #[cfg(test)]
    pub fn add_comment(&self, issue_id: impl ToString, content: &str, kind: &str) -> Result<i64> {
        let issue_id = issue_id.to_string();
        if content.len() > MAX_COMMENT_LEN {
            anyhow::bail!(
                "Comment exceeds maximum length of {} bytes",
                MAX_COMMENT_LEN
            );
        }
        let now = Utc::now();
        self.record_activity_comment(&issue_id, content, kind, now)?;
        Ok(now.timestamp_micros())
    }

    pub fn add_comment_at(
        &self,
        issue_id: impl ToString,
        content: &str,
        kind: &str,
        created_at: &str,
    ) -> Result<i64> {
        let issue_id = issue_id.to_string();
        if content.len() > MAX_COMMENT_LEN {
            anyhow::bail!(
                "Comment exceeds maximum length of {} bytes",
                MAX_COMMENT_LEN
            );
        }
        let created_at = chrono::DateTime::parse_from_rfc3339(created_at)
            .map(|value| value.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());
        self.record_activity_comment(&issue_id, content, kind, created_at)?;
        Ok(created_at.timestamp_micros())
    }

    pub fn get_comments(&self, issue_id: impl ToString) -> Result<Vec<Comment>> {
        let issue_id = issue_id.to_string();
        let Some((state_dir, require_issue_file)) = self.current_runtime_state_dir()? else {
            return Ok(Vec::new());
        };
        if require_issue_file
            && !state_dir
                .join("issues")
                .join(format!("{issue_id}.md"))
                .is_file()
        {
            return Ok(Vec::new());
        }
        if !require_issue_file && self.get_issue(&issue_id)?.is_none() {
            return Ok(Vec::new());
        }
        let mut comments = Vec::new();
        for (idx, activity) in list_issue_activities(&state_dir, &issue_id)?
            .into_iter()
            .filter(|activity| {
                matches!(
                    activity.event_type,
                    ActivityEventType::Comment
                        | ActivityEventType::Note
                        | ActivityEventType::Handoff
                        | ActivityEventType::Plan
                        | ActivityEventType::CloseReason
                )
            })
            .enumerate()
        {
            comments.push(Comment {
                id: idx as i64,
                issue_id: issue_id.clone(),
                content: activity.body,
                created_at: activity.created_at,
                kind: comment_kind(activity.event_type).to_string(),
            });
        }
        Ok(comments)
    }

    fn record_activity_comment(
        &self,
        issue_id: &str,
        content: &str,
        kind: &str,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<()> {
        let Some((state_dir, require_issue_file)) = self.current_runtime_state_dir()? else {
            return Ok(());
        };
        let issue_file = state_dir.join("issues").join(format!("{issue_id}.md"));
        if require_issue_file && !issue_file.is_file() {
            return Ok(());
        }
        if !require_issue_file && self.get_issue(issue_id)?.is_none() {
            return Ok(());
        }
        let event_type = match kind {
            "note" => ActivityEventType::Note,
            "handoff" => ActivityEventType::Handoff,
            "plan" => ActivityEventType::Plan,
            "close-reason" => ActivityEventType::CloseReason,
            _ => ActivityEventType::Comment,
        };
        create_issue_activity(
            &state_dir,
            issue_id,
            event_type,
            &std::env::var("ATELIER_AGENT")
                .or_else(|_| std::env::var("USER"))
                .unwrap_or_else(|_| "agent".to_string()),
            created_at,
            activity_summary(event_type),
            content,
        )
        .map(|_| ())
    }

    fn current_runtime_state_dir(&self) -> Result<Option<(std::path::PathBuf, bool)>> {
        let Some(state_dir) = crate::storage_layout::find_canonical_dir_from_cwd()? else {
            return Ok(self
                .path
                .parent()
                .map(|parent| (parent.join(".atelier"), false)));
        };
        let layout = crate::storage_layout::StorageLayout::new(
            state_dir
                .parent()
                .unwrap_or_else(|| std::path::Path::new(".")),
        );
        if self.path == layout.runtime_db_path() {
            Ok(Some((state_dir, true)))
        } else {
            Ok(self
                .path
                .parent()
                .map(|parent| (parent.join(".atelier"), false)))
        }
    }
}

fn activity_summary(event_type: ActivityEventType) -> &'static str {
    match event_type {
        ActivityEventType::Note => "Added note",
        ActivityEventType::Handoff => "Added handoff",
        ActivityEventType::Plan => "Added plan",
        ActivityEventType::CloseReason => "Recorded close reason",
        _ => "Added comment",
    }
}

fn comment_kind(event_type: ActivityEventType) -> &'static str {
    match event_type {
        ActivityEventType::Note => "note",
        ActivityEventType::Handoff => "handoff",
        ActivityEventType::Plan => "plan",
        ActivityEventType::CloseReason => "close-reason",
        _ => "comment",
    }
}
