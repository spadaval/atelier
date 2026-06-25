use anyhow::Result;

use crate::commands::issue_workflow::{
    issue_blocks_work, issue_status_category, load_issue_workflow_policy,
};
use crate::commands::work_order::{order_work_rows, WorkOrderRow};
use crate::utils::{format_issue_id, truncate};
use atelier_app::workflow_policy::WorkflowPolicy;
use atelier_core::Issue;
use atelier_sqlite::Database;

#[derive(Debug, Clone)]
struct BlockedListRow {
    issue: Issue,
    display_id: String,
    blockers: Vec<String>,
    status_category: Option<String>,
}

pub fn list_blocked(db: &Database, quiet: bool) -> Result<()> {
    let workflow_policy = load_issue_workflow_policy()?;
    let mut rows = db
        .list_blocked_issues()?
        .into_iter()
        .map(|issue| blocked_list_row(db, workflow_policy.as_ref(), issue))
        .collect::<Result<Vec<_>>>()?;
    rows = order_work_rows(rows, |row| WorkOrderRow {
        id: row.display_id.clone(),
        status_category: row.status_category.clone(),
        priority: row.issue.priority.clone(),
        updated_at: row.issue.updated_at,
        open_blockers: row.blockers.clone(),
    });

    if rows.is_empty() {
        println!("No blocked issues.");
        return Ok(());
    }

    if quiet {
        for row in rows {
            println!("{}", row.display_id);
        }
        return Ok(());
    }

    println!("Blocked issues");
    println!("==============");
    println!("{} total", rows.len());
    println!("Drill down: atelier issue show <id>");
    let total = rows.len();
    for row in rows.into_iter().take(5) {
        let blocker_count = row.blockers.len();
        let blocker_text = if blocker_count == 1 {
            "1 blocker".to_string()
        } else {
            format!("{blocker_count} blockers")
        };
        println!(
            "  blocked {:<12} {} ({blocker_text})",
            row.display_id,
            truncate(&row.issue.title, 40),
        );
    }
    if total > 5 {
        println!("  {} more blocked issue(s) omitted", total - 5);
    }

    Ok(())
}

fn blocked_list_row(
    db: &Database,
    workflow_policy: Option<&WorkflowPolicy>,
    issue: Issue,
) -> Result<BlockedListRow> {
    let mut blockers = db
        .get_blockers(&issue.id)?
        .into_iter()
        .filter_map(|id| db.require_issue(&id).ok())
        .filter(|blocker| issue_blocks_work(workflow_policy, blocker))
        .map(|blocker| format_issue_id(&blocker.id))
        .collect::<Vec<_>>();
    blockers.sort();
    Ok(BlockedListRow {
        display_id: format_issue_id(&issue.id),
        status_category: issue_status_category(workflow_policy, &issue.status),
        issue,
        blockers,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn setup_test_db() -> (Database, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::open(&db_path).unwrap();
        (db, dir)
    }

    // List blocked tests
    #[test]
    fn test_list_blocked_empty() {
        let (db, _dir) = setup_test_db();

        list_blocked(&db, false).unwrap();
        let blocked = db.list_blocked_issues().unwrap();
        assert!(blocked.is_empty());
    }

    #[test]
    fn test_list_blocked_with_issues() {
        let (db, _dir) = setup_test_db();
        let issue1 = db.create_issue("Blocked issue", None, "medium").unwrap();
        let issue2 = db.create_issue("Blocker", None, "medium").unwrap();
        db.add_dependency(&issue1, &issue2).unwrap();

        list_blocked(&db, false).unwrap();
        let blocked = db.list_blocked_issues().unwrap();
        assert_eq!(blocked.len(), 1);
        assert_eq!(blocked[0].id, issue1);
    }

    #[test]
    fn test_list_blocked_multiple_blockers() {
        let (db, _dir) = setup_test_db();
        let blocked = db.create_issue("Blocked", None, "medium").unwrap();
        let blocker1 = db.create_issue("Blocker 1", None, "medium").unwrap();
        let blocker2 = db.create_issue("Blocker 2", None, "medium").unwrap();
        db.add_dependency(&blocked, &blocker1).unwrap();
        db.add_dependency(&blocked, &blocker2).unwrap();

        list_blocked(&db, false).unwrap();
        let blockers = db.get_blockers(&blocked).unwrap();
        assert_eq!(blockers.len(), 2);
        assert!(blockers.contains(&blocker1));
        assert!(blockers.contains(&blocker2));
    }

    #[test]
    fn test_list_ready_excludes_blocked() {
        let (db, _dir) = setup_test_db();
        let blocked = db.create_issue("Blocked", None, "high").unwrap();
        let blocker = db.create_issue("Blocker", None, "medium").unwrap();
        db.add_dependency(&blocked, &blocker).unwrap();

        let ready = db.list_ready_issues().unwrap();
        assert!(!ready.iter().any(|i| i.id == blocked));
        assert!(ready.iter().any(|i| i.id == blocker));
    }

    #[test]
    fn test_list_ready_excludes_closed() {
        let (db, _dir) = setup_test_db();
        let issue = db.create_issue("Closed issue", None, "medium").unwrap();
        db.close_issue(&issue).unwrap();

        let ready = db.list_ready_issues().unwrap();
        assert!(!ready.iter().any(|i| i.id == issue));
    }

    #[test]
    fn test_closing_blocker_unblocks() {
        let (db, _dir) = setup_test_db();
        let blocked = db.create_issue("Blocked", None, "high").unwrap();
        let blocker = db.create_issue("Blocker", None, "medium").unwrap();
        db.add_dependency(&blocked, &blocker).unwrap();

        // Blocked issue should not be ready
        let ready = db.list_ready_issues().unwrap();
        assert!(!ready.iter().any(|i| i.id == blocked));

        // Close the blocker
        db.close_issue(&blocker).unwrap();

        // Now blocked issue should be ready
        let ready = db.list_ready_issues().unwrap();
        assert!(ready.iter().any(|i| i.id == blocked));
    }

    #[test]
    fn truncate_respects_limit() {
        assert!(truncate("abcdefghijklmnopqrstuvwxyz", 12).chars().count() <= 12);
    }
}
