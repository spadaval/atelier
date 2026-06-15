use anyhow::Result;
use std::path::Path;

use crate::db::Database;

pub fn check(db: &Database, repo_root: &Path) -> Result<atelier_workflow::WorkflowCheckReport> {
    let issues = db.list_issues(Some("all"), None, None)?;
    atelier_workflow::check_issues(repo_root, issues)
}
