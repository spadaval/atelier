use anyhow::Result;

use crate::db::Database;
use crate::utils::{format_issue_id, truncate};

pub fn list_blocked(db: &Database) -> Result<()> {
    let issues = db.list_blocked_issues()?;

    if issues.is_empty() {
        println!("No blocked issues.");
        return Ok(());
    }

    println!("Blocked issues:");
    for issue in issues {
        let blockers = db.get_blockers(&issue.id)?;
        let blocker_strs: Vec<String> = blockers.iter().map(|b| format_issue_id(b)).collect();
        println!(
            "  {:<5} {} (blocked by: {})",
            format_issue_id(&issue.id),
            truncate(&issue.title, 40),
            blocker_strs.join(", ")
        );
    }

    Ok(())
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

        list_blocked(&db).unwrap();
        let blocked = db.list_blocked_issues().unwrap();
        assert!(blocked.is_empty());
    }

    #[test]
    fn test_list_blocked_with_issues() {
        let (db, _dir) = setup_test_db();
        let issue1 = db.create_issue("Blocked issue", None, "medium").unwrap();
        let issue2 = db.create_issue("Blocker", None, "medium").unwrap();
        db.add_dependency(&issue1, &issue2).unwrap();

        list_blocked(&db).unwrap();
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

        list_blocked(&db).unwrap();
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
