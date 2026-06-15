use anyhow::Result;
#[cfg(test)]
use std::env;

use super::Database;
use atelier_core::WorkAssociation;

impl Database {
    pub fn start_work_association(
        &self,
        issue_id: &str,
        branch: Option<&str>,
        worktree_path: Option<&str>,
    ) -> Result<()> {
        self.require_issue(issue_id)?;
        let _ = (branch, worktree_path);
        Ok(())
    }

    #[cfg(test)]
    pub fn get_active_work_association(&self) -> Result<Option<WorkAssociation>> {
        let path = env::current_dir()?.to_string_lossy().to_string();
        self.active_work_association_for_worktree_path(Some(&path))
    }

    pub fn active_work_association_for_worktree_path(
        &self,
        worktree_path: Option<&str>,
    ) -> Result<Option<WorkAssociation>> {
        let _ = worktree_path;
        Ok(None)
    }

    pub fn get_work_association(&self, issue_id: &str) -> Result<Option<WorkAssociation>> {
        let _ = issue_id;
        Ok(None)
    }

    pub fn list_work_associations(&self) -> Result<Vec<WorkAssociation>> {
        Ok(Vec::new())
    }

    pub fn remove_work_association(&self, issue_id: &str) -> Result<bool> {
        let _ = issue_id;
        Ok(false)
    }
}
