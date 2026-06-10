use anyhow::{bail, Result};

use crate::commands::create::validate_priority;
use crate::db::Database;
use crate::utils::format_issue_id;

pub fn run(
    db: &Database,
    id: &str,
    title: Option<&str>,
    description: Option<&str>,
    priority: Option<&str>,
) -> Result<()> {
    if title.is_none() && description.is_none() && priority.is_none() {
        bail!("Nothing to update. Use --title, --description, or --priority");
    }

    if let Some(p) = priority {
        if !validate_priority(p) {
            bail!(
                "Invalid priority '{}'. Must be one of: low, medium, high, critical",
                p
            );
        }
    }

    if db.update_issue(&id, title, description, priority)? {
        println!("Updated issue {}", format_issue_id(id));
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

    // ==================== Unit Tests ====================

    #[test]
    fn test_update_title() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Original title", None, "medium").unwrap();

        let result = run(&db, issue_id.as_str(), Some("New title"), None, None);
        assert!(result.is_ok());

        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.title, "New title");
    }

    #[test]
    fn test_update_description() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test", None, "medium").unwrap();

        let result = run(&db, issue_id.as_str(), None, Some("New description"), None);
        assert!(result.is_ok());

        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.description, Some("New description".to_string()));
    }

    #[test]
    fn test_update_priority() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test", None, "medium").unwrap();

        let result = run(&db, issue_id.as_str(), None, None, Some("critical"));
        assert!(result.is_ok());

        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.priority, "critical");
    }

    #[test]
    fn test_update_all_fields() {
        let (db, _dir) = setup_test_db();
        let issue_id = db
            .create_issue("Original", Some("Old desc"), "low")
            .unwrap();

        let result = run(
            &db,
            &issue_id,
            Some("New title"),
            Some("New description"),
            Some("high"),
        );
        assert!(result.is_ok());

        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.title, "New title");
        assert_eq!(issue.description, Some("New description".to_string()));
        assert_eq!(issue.priority, "high");
    }

    #[test]
    fn test_update_nothing_fails() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test", None, "medium").unwrap();

        let result = run(&db, issue_id.as_str(), None, None, None);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Nothing to update"));
    }

    #[test]
    fn test_update_nonexistent_issue() {
        let (db, _dir) = setup_test_db();

        let result = run(&db, "atelier-missing", Some("New title"), None, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_update_invalid_priority() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test", None, "medium").unwrap();

        let result = run(&db, issue_id.as_str(), None, None, Some("urgent"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid priority"));
    }

    #[test]
    fn test_update_preserves_unchanged_fields() {
        let (db, _dir) = setup_test_db();
        let issue_id = db
            .create_issue("Original title", Some("Original desc"), "high")
            .unwrap();

        // Only update title
        run(&db, issue_id.as_str(), Some("New title"), None, None).unwrap();

        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.title, "New title");
        assert_eq!(issue.description, Some("Original desc".to_string()));
        assert_eq!(issue.priority, "high");
    }

    #[test]
    fn test_update_unicode_title() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Original", None, "medium").unwrap();

        let result = run(
            &db,
            issue_id.as_str(),
            Some("新しいタイトル 🎉"),
            None,
            None,
        );
        assert!(result.is_ok());

        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.title, "新しいタイトル 🎉");
    }

    #[test]
    fn test_update_empty_description() {
        let (db, _dir) = setup_test_db();
        let issue_id = db
            .create_issue("Test", Some("Has description"), "medium")
            .unwrap();

        let result = run(&db, issue_id.as_str(), None, Some(""), None);
        assert!(result.is_ok());

        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.description, Some("".to_string()));
    }

    #[test]
    fn test_update_sql_injection() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Original", None, "medium").unwrap();

        let malicious = "'; DROP TABLE issues; --";
        let result = run(&db, issue_id.as_str(), Some(malicious), None, None);
        assert!(result.is_ok());

        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.title, malicious);

        // Verify database is intact
        let issues = db.list_issues(None, None, None).unwrap();
        assert!(!issues.is_empty());
    }

    #[test]
    fn test_update_closed_issue() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test", None, "medium").unwrap();
        db.close_issue(issue_id.as_str()).unwrap();

        let result = run(
            &db,
            issue_id.as_str(),
            Some("Updated closed issue"),
            None,
            None,
        );
        assert!(result.is_ok());

        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.title, "Updated closed issue");
        assert_eq!(issue.status, "closed"); // Status should remain closed
    }

    #[test]
    fn test_update_title_priority_and_unicode_description() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Original", None, "medium").unwrap();

        run(
            &db,
            issue_id.as_str(),
            Some("Updated title"),
            Some("Unicode description cafe"),
            Some("critical"),
        )
        .unwrap();

        let issue = db.get_issue(issue_id.as_str()).unwrap().unwrap();
        assert_eq!(issue.title, "Updated title");
        assert_eq!(
            issue.description,
            Some("Unicode description cafe".to_string())
        );
        assert_eq!(issue.priority, "critical");
    }

    #[test]
    fn test_update_invalid_priority_and_missing_issue_fail() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test", None, "medium").unwrap();

        assert!(run(&db, issue_id.as_str(), None, None, Some("urgent")).is_err());
        assert!(run(&db, "atelier-missing-1000", Some("New title"), None, None).is_err());
    }
}
