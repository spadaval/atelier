use anyhow::Result;
use rusqlite::params;

use super::Database;
use atelier_core::Issue;

impl Database {
    pub fn add_dependency(
        &self,
        blocked_id: impl ToString,
        blocker_id: impl ToString,
    ) -> Result<bool> {
        let blocked_id = blocked_id.to_string();
        let blocker_id = blocker_id.to_string();
        if blocked_id == blocker_id {
            anyhow::bail!("An issue cannot block itself");
        }

        if self.would_create_cycle(&blocked_id, &blocker_id)? {
            anyhow::bail!("Adding this dependency would create a circular dependency chain");
        }

        let result = self.conn.execute(
            "INSERT OR IGNORE INTO dependencies (blocker_id, blocked_id) VALUES (?1, ?2)",
            params![blocker_id, blocked_id],
        )?;
        Ok(result > 0)
    }

    /// Check if adding blocker_id -> blocked_id would create a cycle.
    fn would_create_cycle(&self, blocked_id: &str, blocker_id: &str) -> Result<bool> {
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![blocked_id.to_string()];

        while let Some(current) = stack.pop() {
            if current == blocker_id {
                return Ok(true);
            }

            if visited.insert(current.clone()) {
                let blocking = self.get_blocking(&current)?;
                for next in blocking {
                    if !visited.contains(&next) {
                        stack.push(next);
                    }
                }
            }
        }

        Ok(false)
    }

    #[cfg(test)]
    pub fn remove_dependency(
        &self,
        blocked_id: impl ToString,
        blocker_id: impl ToString,
    ) -> Result<bool> {
        let blocked_id = blocked_id.to_string();
        let blocker_id = blocker_id.to_string();
        let rows = self.conn.execute(
            "DELETE FROM dependencies WHERE blocker_id = ?1 AND blocked_id = ?2",
            params![blocker_id, blocked_id],
        )?;
        Ok(rows > 0)
    }

    pub fn get_blockers(&self, issue_id: impl ToString) -> Result<Vec<String>> {
        let issue_id = issue_id.to_string();
        let mut stmt = self.conn.prepare(
            "SELECT blocker_id FROM issue_block_index WHERE blocked_id = ?1 ORDER BY blocker_id",
        )?;
        let blockers = stmt
            .query_map([issue_id], |row| row.get(0))?
            .collect::<std::result::Result<Vec<String>, _>>()?;
        Ok(blockers)
    }

    pub fn get_blocking(&self, issue_id: impl ToString) -> Result<Vec<String>> {
        let issue_id = issue_id.to_string();
        let mut stmt = self.conn.prepare(
            "SELECT blocked_id FROM issue_block_index WHERE blocker_id = ?1 ORDER BY blocked_id",
        )?;
        let blocking = stmt
            .query_map([issue_id], |row| row.get(0))?
            .collect::<std::result::Result<Vec<String>, _>>()?;
        Ok(blocking)
    }

    pub fn list_blocked_issues(&self) -> Result<Vec<Issue>> {
        Ok(self
            .list_issues(Some("all"), None, None)?
            .into_iter()
            .filter(|issue| issue.closed_at.is_none())
            .filter(|issue| {
                self.get_blockers(&issue.id).is_ok_and(|blockers| {
                    blockers.into_iter().any(|id| {
                        self.get_issue(id)
                            .ok()
                            .flatten()
                            .is_some_and(|blocker| blocker.closed_at.is_none())
                    })
                })
            })
            .collect())
    }

    pub fn list_ready_issues(&self) -> Result<Vec<Issue>> {
        let blocked = self
            .list_blocked_issues()?
            .into_iter()
            .map(|issue| issue.id)
            .collect::<std::collections::HashSet<_>>();
        Ok(self
            .list_issues(Some("all"), None, None)?
            .into_iter()
            .filter(|issue| issue.closed_at.is_none() && !blocked.contains(&issue.id))
            .collect())
    }
}
