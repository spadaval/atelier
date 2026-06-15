use anyhow::Result;

use super::Database;
use crate::models::Session;

impl Database {
    /// Convenience wrapper for tests — production code uses `start_session_with_agent`.
    #[cfg(test)]
    pub fn start_session(&self) -> Result<i64> {
        self.start_session_with_agent(None)
    }

    pub fn start_session_with_agent(&self, agent_id: Option<&str>) -> Result<i64> {
        let _ = agent_id;
        Ok(0)
    }

    pub fn get_current_session(&self) -> Result<Option<Session>> {
        Ok(None)
    }

    pub fn set_session_issue(&self, session_id: i64, issue_id: impl ToString) -> Result<bool> {
        let _ = (session_id, issue_id.to_string());
        Ok(false)
    }
}
