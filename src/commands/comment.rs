use anyhow::Result;

use crate::db::Database;
use crate::utils::format_issue_id;

const KNOWN_COMMENT_KINDS: &[&str] = &[
    "note",
    "plan",
    "observation",
    "blocker",
    "resolution",
    "result",
    "handoff",
    "human",
];

pub fn validate_comment_kind(kind: &str) -> bool {
    KNOWN_COMMENT_KINDS.contains(&kind)
}

pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {
    db.require_issue(issue_id)?;
    reject_invalid_comment_kind(kind)?;
    if !validate_comment_kind(kind) {
        tracing::warn!(
            "unknown comment kind '{}'. Known kinds: {}",
            kind,
            KNOWN_COMMENT_KINDS.join(", ")
        );
    }
    db.add_comment(issue_id, content, kind)?;
    crate::commands::activity_log::record_comment(issue_id, kind, content)?;
    println!("Added comment to issue {}", format_issue_id(issue_id));
    Ok(())
}

pub fn run_canonical(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {
    db.require_issue(issue_id)?;
    reject_invalid_comment_kind(kind)?;
    if !validate_comment_kind(kind) {
        tracing::warn!(
            "unknown comment kind '{}'. Known kinds: {}",
            kind,
            KNOWN_COMMENT_KINDS.join(", ")
        );
    }
    crate::commands::activity_log::record_comment(issue_id, kind, content)?;
    println!("Added comment to issue {}", format_issue_id(issue_id));
    Ok(())
}

fn reject_invalid_comment_kind(kind: &str) -> Result<()> {
    if kind == "decision" {
        anyhow::bail!(
            "Invalid comment kind 'decision'. Valid kinds: note, plan, observation, blocker, resolution, result, handoff, human"
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

    // ==================== Unit Tests ====================

    #[test]
    fn test_add_comment_to_existing_issue() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let result = run(&db, &issue_id, "This is a comment", "note");
        assert!(result.is_ok());

        let comments = db.get_comments(issue_id).unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].content, "This is a comment");
        assert_eq!(comments[0].kind, "note");
    }

    #[test]
    fn test_add_comment_to_nonexistent_issue() {
        let (db, _dir) = setup_test_db();

        let result = run(&db, "atelier-missing", "Comment on nothing", "note");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_add_multiple_comments() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        run(&db, &issue_id, "First comment", "note").unwrap();
        run(&db, &issue_id, "Second comment", "note").unwrap();
        run(&db, &issue_id, "Third comment", "note").unwrap();

        let comments = db.get_comments(issue_id).unwrap();
        assert_eq!(comments.len(), 3);
        assert_eq!(comments[0].content, "First comment");
        assert_eq!(comments[1].content, "Second comment");
        assert_eq!(comments[2].content, "Third comment");
    }

    #[test]
    fn test_add_empty_comment() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let result = run(&db, &issue_id, "", "note");
        assert!(result.is_ok());

        let comments = db.get_comments(issue_id).unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].content, "");
    }

    #[test]
    fn test_add_unicode_comment() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let unicode_content = "こんにちは 🎉 مرحبا αβγδ ← → ↑ ↓";
        let result = run(&db, &issue_id, unicode_content, "note");
        assert!(result.is_ok());

        let comments = db.get_comments(issue_id).unwrap();
        assert_eq!(comments[0].content, unicode_content);
    }

    #[test]
    fn test_add_very_long_comment() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let long_content = "a".repeat(100000);
        let result = run(&db, &issue_id, &long_content, "note");
        assert!(result.is_ok());

        let comments = db.get_comments(issue_id).unwrap();
        assert_eq!(comments[0].content.len(), 100000);
    }

    #[test]
    fn test_add_comment_with_newlines() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let multiline = "Line 1\nLine 2\nLine 3\n\nLine 5";
        let result = run(&db, &issue_id, multiline, "note");
        assert!(result.is_ok());

        let comments = db.get_comments(issue_id).unwrap();
        assert_eq!(comments[0].content, multiline);
    }

    #[test]
    fn test_add_comment_with_special_chars() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let special = "Quotes: \"test\" 'test' `test` | Symbols: @#$%^&*() | SQL: '; DROP TABLE;--";
        let result = run(&db, &issue_id, special, "note");
        assert!(result.is_ok());

        let comments = db.get_comments(issue_id).unwrap();
        assert_eq!(comments[0].content, special);
    }

    #[test]
    fn test_add_comment_sql_injection() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let malicious = "'); DELETE FROM comments; --";
        run(&db, &issue_id, malicious, "note").unwrap();

        // Verify comment was stored literally, not executed
        let comments = db.get_comments(issue_id).unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].content, malicious);

        // Verify database integrity
        let issues = db.list_issues(None, None, None).unwrap();
        assert!(!issues.is_empty());
    }

    #[test]
    fn test_comment_on_closed_issue() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();
        db.close_issue(&issue_id).unwrap();

        // Should still be able to comment on closed issues
        let result = run(&db, &issue_id, "Comment on closed issue", "note");
        assert!(result.is_ok());

        let comments = db.get_comments(issue_id).unwrap();
        assert_eq!(comments.len(), 1);
    }

    #[test]
    fn test_comment_with_null_bytes() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let with_null = "before\0after";
        let result = run(&db, &issue_id, with_null, "note");
        assert!(result.is_ok());

        let comments = db.get_comments(issue_id).unwrap();
        assert_eq!(comments[0].content, with_null);
    }

    #[test]
    fn test_comment_kind_roundtrip() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        run(&db, &issue_id, "A plan was made", "plan").unwrap();

        let comments = db.get_comments(issue_id).unwrap();
        assert_eq!(comments[0].kind, "plan");
    }

    #[test]
    fn test_invalid_comment_kind_is_rejected() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test issue", None, "medium").unwrap();

        let error = run(&db, &issue_id, "Do not store this", "decision").unwrap_err();
        assert!(error
            .to_string()
            .contains("Invalid comment kind 'decision'"));
    }

    #[test]
    fn test_validate_known_kinds() {
        assert!(validate_comment_kind("note"));
        assert!(validate_comment_kind("plan"));
        assert!(validate_comment_kind("observation"));
        assert!(validate_comment_kind("blocker"));
        assert!(validate_comment_kind("resolution"));
        assert!(validate_comment_kind("result"));
        assert!(validate_comment_kind("handoff"));
        assert!(validate_comment_kind("human"));
    }

    #[test]
    fn test_validate_unknown_kinds() {
        assert!(!validate_comment_kind(""));
        assert!(!validate_comment_kind("unknown"));
        assert!(!validate_comment_kind("NOTE"));
        assert!(!validate_comment_kind("Plan"));
    }

    #[test]
    fn test_comment_roundtrip_order_and_missing_issue() {
        let (db, _dir) = setup_test_db();
        let issue_id = db.create_issue("Test", None, "medium").unwrap();

        run(&db, &issue_id, "Comment 0", "note").unwrap();
        run(&db, &issue_id, "Comment 1", "note").unwrap();

        let comments = db.get_comments(issue_id).unwrap();
        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].content, "Comment 0");
        assert_eq!(comments[1].content, "Comment 1");

        assert!(run(&db, "atelier-missing-1000", "Comment", "note").is_err());
    }
}
