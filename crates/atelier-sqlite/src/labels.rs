use anyhow::Result;
use rusqlite::params;

use super::{Database, MAX_LABEL_LEN};

impl Database {
    pub fn add_label(&self, issue_id: impl ToString, label: &str) -> Result<bool> {
        let issue_id = issue_id.to_string();
        if label.len() > MAX_LABEL_LEN {
            anyhow::bail!(
                "Label exceeds maximum length of {} characters",
                MAX_LABEL_LEN
            );
        }
        let result = self.conn.execute(
            "INSERT OR IGNORE INTO issue_label_index (issue_id, label) VALUES (?1, ?2)",
            params![issue_id, label],
        )?;
        Ok(result > 0)
    }
    pub fn remove_label(&self, issue_id: impl ToString, label: &str) -> Result<bool> {
        let issue_id = issue_id.to_string();
        let rows = self.conn.execute(
            "DELETE FROM issue_label_index WHERE issue_id = ?1 AND label = ?2",
            params![issue_id, label],
        )?;
        Ok(rows > 0)
    }

    pub fn get_labels(&self, issue_id: impl ToString) -> Result<Vec<String>> {
        self.issue_cache_labels(&issue_id.to_string())
    }
}
