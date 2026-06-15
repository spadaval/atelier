use anyhow::Result;

use crate::utils::format_issue_id;
use atelier_records::RecordStore;
use atelier_sqlite::Database;

#[cfg(test)]
pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {
    db.require_issue(issue_id)?;

    if db.add_label(issue_id, label)? {
        println!(
            "Added label '{}' to issue {}",
            label,
            format_issue_id(issue_id)
        );
    } else {
        println!(
            "Label '{}' already exists on issue {}",
            label,
            format_issue_id(issue_id)
        );
    }
    Ok(())
}

pub fn add_canonical(
    db: &Database,
    store: &RecordStore,
    issue_id: &str,
    label: &str,
) -> Result<()> {
    db.require_issue(issue_id)?;

    if store.add_issue_label(issue_id, label)? {
        println!(
            "Added label '{}' to issue {}",
            label,
            format_issue_id(issue_id)
        );
    } else {
        println!(
            "Label '{}' already exists on issue {}",
            label,
            format_issue_id(issue_id)
        );
    }
    Ok(())
}

#[cfg(test)]
pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {
    db.require_issue(issue_id)?;

    if db.remove_label(issue_id, label)? {
        println!(
            "Removed label '{}' from issue {}",
            label,
            format_issue_id(issue_id)
        );
    } else {
        println!(
            "Label '{}' not found on issue {}",
            label,
            format_issue_id(issue_id)
        );
    }
    Ok(())
}

pub fn remove_canonical(
    db: &Database,
    store: &RecordStore,
    issue_id: &str,
    label: &str,
) -> Result<()> {
    db.require_issue(issue_id)?;

    if store.remove_issue_label(issue_id, label)? {
        println!(
            "Removed label '{}' from issue {}",
            label,
            format_issue_id(issue_id)
        );
    } else {
        println!(
            "Label '{}' not found on issue {}",
            label,
            format_issue_id(issue_id)
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

    // ==================== Add Label Tests ====================

    #[test]
    fn test_add_label_to_existing_issue() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let result = add(&db, &issue_id, "bug");
        assert!(result.is_ok());

        let labels = db.get_labels(issue_id).unwrap();
        assert!(labels.contains(&"bug".to_string()));
    }

    #[test]
    fn test_add_label_to_nonexistent_issue() {
        let (db, _dir) = setup_test_db();

        let result = add(&db, "atelier-missing", "bug");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_add_duplicate_label() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        add(&db, &issue_id, "bug").unwrap();
        let result = add(&db, &issue_id, "bug"); // Duplicate
        assert!(result.is_ok()); // Should succeed but not add duplicate

        let labels = db.get_labels(issue_id).unwrap();
        assert_eq!(labels.len(), 1);
    }

    #[test]
    fn test_add_multiple_labels() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        add(&db, &issue_id, "bug").unwrap();
        add(&db, &issue_id, "urgent").unwrap();
        add(&db, &issue_id, "backend").unwrap();

        let labels = db.get_labels(issue_id).unwrap();
        assert_eq!(labels.len(), 3);
        assert!(labels.contains(&"bug".to_string()));
        assert!(labels.contains(&"urgent".to_string()));
        assert!(labels.contains(&"backend".to_string()));
    }

    #[test]
    fn test_add_empty_label() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let result = add(&db, &issue_id, "");
        assert!(result.is_ok());

        let labels = db.get_labels(issue_id).unwrap();
        assert!(labels.contains(&"".to_string()));
    }

    #[test]
    fn test_add_unicode_label() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let result = add(&db, &issue_id, "バグ");
        assert!(result.is_ok());

        let labels = db.get_labels(issue_id).unwrap();
        assert!(labels.contains(&"バグ".to_string()));
    }

    #[test]
    fn test_add_label_with_special_chars() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let result = add(&db, &issue_id, "high-priority");
        assert!(result.is_ok());

        let result = add(&db, &issue_id, "v2.0");
        assert!(result.is_ok());

        let result = add(&db, &issue_id, "team:backend");
        assert!(result.is_ok());

        let labels = db.get_labels(issue_id).unwrap();
        assert_eq!(labels.len(), 3);
    }

    #[test]
    fn test_add_label_sql_injection() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let malicious = "'; DROP TABLE labels; --";
        let result = add(&db, &issue_id, malicious);
        assert!(result.is_ok());

        // Verify label was stored literally
        let labels = db.get_labels(issue_id).unwrap();
        assert!(labels.contains(&malicious.to_string()));

        // Verify database integrity
        let issues = db.list_issues(None, None, None).unwrap();
        assert!(!issues.is_empty());
    }

    // ==================== Remove Label Tests ====================

    #[test]
    fn test_remove_existing_label() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        add(&db, &issue_id, "bug").unwrap();
        let result = remove(&db, &issue_id, "bug");
        assert!(result.is_ok());

        let labels = db.get_labels(issue_id).unwrap();
        assert!(!labels.contains(&"bug".to_string()));
    }

    #[test]
    fn test_remove_nonexistent_label() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let result = remove(&db, &issue_id, "nonexistent");
        assert!(result.is_ok()); // Should succeed but report not found
    }

    #[test]
    fn test_remove_label_from_nonexistent_issue() {
        let (db, _dir) = setup_test_db();

        let result = remove(&db, "atelier-missing", "bug");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_remove_one_of_many_labels() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        add(&db, &issue_id, "bug").unwrap();
        add(&db, &issue_id, "urgent").unwrap();
        add(&db, &issue_id, "backend").unwrap();

        remove(&db, &issue_id, "urgent").unwrap();

        let labels = db.get_labels(issue_id).unwrap();
        assert_eq!(labels.len(), 2);
        assert!(labels.contains(&"bug".to_string()));
        assert!(labels.contains(&"backend".to_string()));
        assert!(!labels.contains(&"urgent".to_string()));
    }

    #[test]
    fn test_add_label_to_closed_issue() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();
        db.close_issue(&issue_id).unwrap();

        let result = add(&db, &issue_id, "bug");
        assert!(result.is_ok());

        let labels = db.get_labels(issue_id).unwrap();
        assert!(labels.contains(&"bug".to_string()));
    }

    #[test]
    fn test_label_roundtrip_independence_and_missing_issue() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test", None, "medium").unwrap();

        add(&db, &issue_id, "one").unwrap();
        add(&db, &issue_id, "two").unwrap();
        remove(&db, &issue_id, "one").unwrap();

        let remaining = db.get_labels(issue_id).unwrap();
        assert!(!remaining.contains(&"one".to_string()));
        assert!(remaining.contains(&"two".to_string()));

        assert!(add(&db, "atelier-missing-1000", "label").is_err());
        assert!(remove(&db, "atelier-missing-1000", "label").is_err());
    }
}
