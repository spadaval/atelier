use anyhow::{bail, Result};

use crate::utils::format_issue_id;
use crate::{commands, db::Database};

pub fn close(db: &Database, id: &str) -> Result<()> {
    if db.close_issue(&id)? {
        println!("Closed issue {}", format_issue_id(id));
    } else {
        bail!("Issue {} not found", format_issue_id(id));
    }

    Ok(())
}

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
        match commands::agent_factory::close(db, &issue.id.to_string(), None, false) {
            Ok(()) => closed_count += 1,
            Err(e) => tracing::warn!("Failed to close {}: {}", format_issue_id(&issue.id), e),
        }
    }

    println!("Closed {} issue(s).", closed_count);
    Ok(())
}

pub fn reopen(db: &Database, id: &str) -> Result<()> {
    if db.reopen_issue(&id)? {
        println!("Reopened issue {}", format_issue_id(id));
    } else {
        bail!("Issue {} not found", format_issue_id(id));
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

    // ==================== Close Tests ====================

    #[test]
    fn test_close_existing_issue() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let result = close(&db, issue_id.as_str());
        assert!(result.is_ok());

        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.status, "closed");
        assert!(issue.closed_at.is_some());
    }

    #[test]
    fn test_close_nonexistent_issue() {
        let (db, _dir) = setup_test_db();

        let result = close(&db, "atelier-missing");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_close_already_closed_issue() {
        let (db, _dir) = setup_test_db();

        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();
        db.close_issue(issue_id.as_str()).unwrap();

        // Closing again should be fine (idempotent at db level)
        let result = close(&db, issue_id.as_str());
        assert!(result.is_ok());
    }

    // ==================== Reopen Tests ====================

    #[test]
    fn test_reopen_closed_issue() {
        let (db, _dir) = setup_test_db();

        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();
        db.close_issue(issue_id.as_str()).unwrap();

        let result = reopen(&db, issue_id.as_str());
        assert!(result.is_ok());

        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.status, "open");
        assert!(issue.closed_at.is_none());
    }

    #[test]
    fn test_reopen_nonexistent_issue() {
        let (db, _dir) = setup_test_db();

        let result = reopen(&db, "atelier-missing");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_reopen_already_open_issue() {
        let (db, _dir) = setup_test_db();

        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        // Reopening an open issue - succeeds (idempotent operation)
        let result = reopen(&db, issue_id.as_str());
        assert!(result.is_ok());

        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.status, "open");
    }

    // ==================== Close/Reopen Cycle Tests ====================

    #[test]
    fn test_close_reopen_cycle() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        // Close
        close(&db, issue_id.as_str()).unwrap();
        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.status, "closed");

        // Reopen
        reopen(&db, issue_id.as_str()).unwrap();
        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.status, "open");

        // Close again
        close(&db, issue_id.as_str()).unwrap();
        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.status, "closed");
    }

    #[test]
    fn test_close_reopen_and_missing_issue_status_paths() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Status issue", None, "medium").unwrap();

        close(&db, issue_id.as_str()).unwrap();
        assert_eq!(
            db.get_issue(issue_id.as_str()).unwrap().unwrap().status,
            "closed"
        );

        reopen(&db, issue_id.as_str()).unwrap();
        assert_eq!(
            db.get_issue(issue_id.as_str()).unwrap().unwrap().status,
            "open"
        );

        assert!(close(&db, "atelier-missing-1000").is_err());
        assert!(reopen(&db, "atelier-missing-1000").is_err());
    }
}
