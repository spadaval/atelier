use super::*;
use proptest::prelude::*;

fn setup_test_db() -> (Database, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("state.db");
    let db = Database::open(&db_path).unwrap();
    (db, dir)
}

#[test]
fn title_description_priority_label_and_comment_roundtrip() {
    let (db, _dir) = setup_test_db();
    let id = db
        .create_issue(
            "Title with punctuation?!",
            Some("Description with punctuation."),
            "critical",
        )
        .unwrap();
    db.add_label(&id, "label-1").unwrap();
    db.record_legacy_import_comment(&id, "Comment with punctuation.", "note")
        .unwrap();

    let issue = db.get_issue(&id).unwrap().unwrap();
    assert_eq!(issue.title, "Title with punctuation?!");
    assert_eq!(
        issue.description,
        Some("Description with punctuation.".to_string())
    );
    assert_eq!(issue.priority, "critical");
    assert!(db.get_labels(&id).unwrap().contains(&"label-1".to_string()));
    assert_eq!(
        db.list_legacy_import_comments(&id).unwrap()[0].content,
        "Comment with punctuation."
    );
}

#[test]
fn create_close_reopen_search_and_blocking_examples() {
    let (db, _dir) = setup_test_db();
    let source_id = db
        .create_issue("unique marker source", None, "medium")
        .unwrap();
    let blocker_id = db.create_issue("Blocker", None, "medium").unwrap();
    db.create_issue("Other", None, "medium").unwrap();

    assert_eq!(db.list_issues(None, None, None).unwrap().len(), 3);
    db.close_issue(&source_id).unwrap();
    assert_eq!(db.get_issue(&source_id).unwrap().unwrap().status, "done");
    db.reopen_issue(&source_id).unwrap();
    assert_eq!(db.get_issue(&source_id).unwrap().unwrap().status, "todo");

    db.add_dependency(&source_id, &blocker_id).unwrap();
    assert!(db.get_blockers(&source_id).unwrap().contains(&blocker_id));
    assert!(db
        .search_issues("unique marker")
        .unwrap()
        .iter()
        .any(|issue| issue.id == source_id));
}

proptest! {

    /// Circular dependencies should be prevented
    #[test]
    #[ignore = "reason: extended property test run only in extended profile; owner: quality; product: yes; blocking: no"]
    fn prop_extended_no_circular_deps(chain_len in 2usize..6) {
        let (db, _dir) = setup_test_db();

        // Create a chain of issues
        let mut ids = Vec::new();
        for i in 0..chain_len {
            let id = db.create_issue(&format!("Issue {}", i), None, "medium").unwrap();
            ids.push(id);
        }

        // Create a linear dependency chain: 0 <- 1 <- 2 <- ... <- n-1
        for i in 0..chain_len - 1 {
            db.add_dependency(&ids[i], &ids[i + 1]).unwrap();
        }

        // Trying to close the cycle (n-1 <- 0) should fail
        let result = db.add_dependency(&ids[chain_len - 1], &ids[0]);
        prop_assert!(result.is_err(), "Circular dependency should be rejected");
    }

    /// Deleting a parent should cascade to all children
    #[test]
    #[ignore = "reason: extended property test run only in extended profile; owner: quality; product: yes; blocking: no"]
    fn prop_extended_cascade_deletes_children(child_count in 1usize..5) {
        let (db, _dir) = setup_test_db();

        // Create parent
        let parent_id = db.create_issue("Parent", None, "medium").unwrap();

        // Create children
        let mut child_ids = Vec::new();
        for i in 0..child_count {
            let id = db.create_subissue(&parent_id, &format!("Child {}", i), None, "low").unwrap();
            child_ids.push(id);
        }

        // Verify children exist
        let issues_before = db.list_issues(None, None, None).unwrap();
        prop_assert_eq!(issues_before.len(), child_count + 1);

        // Delete parent
        db.delete_issue(&parent_id).unwrap();

        // All children should be gone too
        let issues_after = db.list_issues(None, None, None).unwrap();
        prop_assert_eq!(issues_after.len(), 0);

        // Verify each child is gone
        for child_id in child_ids {
            let child = db.get_issue(&child_id).unwrap();
            prop_assert!(child.is_none(), "Child should be deleted");
        }
    }

    /// Ready list should never contain issues with unresolved blockers
    #[test]
    #[ignore = "reason: extended property test run only in extended profile; owner: quality; product: yes; blocking: no"]
    fn prop_extended_ready_list_correctness(issue_count in 2usize..8) {
        let (db, _dir) = setup_test_db();

        // Create issues
        let mut ids = Vec::new();
        for i in 0..issue_count {
            let id = db.create_issue(&format!("Issue {}", i), None, "medium").unwrap();
            ids.push(id);
        }

        // Create some dependencies (each issue blocked by next, except last)
        for i in 0..issue_count - 1 {
            let _ = db.add_dependency(&ids[i], &ids[i + 1]);
        }

        // Get ready issues
        let ready = db.list_ready_issues().unwrap();

        // Verify: no ready issue should have unresolved blockers
        for issue in &ready {
            let blockers = db.get_blockers(&issue.id).unwrap();
            for blocker_id in blockers {
                if let Some(blocker) = db.get_issue(&blocker_id).unwrap() {
                    prop_assert_ne!(
                        blocker.status, "todo",
                        "Ready issue {} has unresolved blocker {}",
                        issue.id, blocker_id
                    );
                }
            }
        }
    }

    /// Search wildcards should be escaped properly
    #[test]
    #[ignore = "reason: extended property test run only in extended profile; owner: quality; product: yes; blocking: no"]
    fn prop_extended_search_wildcards_escaped(
        prefix in "[a-zA-Z]{3,5}",
        suffix in "[a-zA-Z]{3,5}"
    ) {
        let (db, _dir) = setup_test_db();

        // Create an issue with % and _ in title
        let special_title = format!("{}%test_marker{}", prefix, suffix);
        db.create_issue(&special_title, None, "medium").unwrap();

        // Create another issue that would match if wildcards weren't escaped
        db.create_issue("other content here", None, "medium").unwrap();

        // Search for the special characters literally
        let results = db.search_issues("%test_").unwrap();

        // Should find only the issue with literal % and _
        prop_assert!(results.iter().all(|i| i.title.contains("%test_")));
    }
}
