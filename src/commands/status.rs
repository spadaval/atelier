use anyhow::Result;

use crate::utils::format_issue_id;
use crate::{commands, db::Database};

pub fn close_all(
    db: &Database,
    label_filter: Option<&str>,
    priority_filter: Option<&str>,
) -> Result<()> {
    let issues = db.list_issues(Some("open"), label_filter, priority_filter)?;

    if issues.is_empty() {
        println!("No matching open issues found.");
        return Ok(());
    }

    let mut closed_count = 0;
    for issue in &issues {
        match commands::agent_factory::close(db, &issue.id.to_string(), None) {
            Ok(()) => closed_count += 1,
            Err(e) => tracing::warn!("Failed to close {}: {}", format_issue_id(&issue.id), e),
        }
    }

    println!("Closed {} issue(s).", closed_count);
    Ok(())
}
