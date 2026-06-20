use super::*;

// ==================== Additional Edge Case Coverage ====================

// --- relate.rs: Error cases ---
#[test]
fn test_relate_nonexistent_first_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Existing"]);

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "relate", "999", "1"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_relate_nonexistent_second_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Existing"]);

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "relate", "1", "999"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_relate_already_related() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue A"]);
    run_atelier(dir.path(), &["issue", "create", "Issue B"]);
    run_atelier(dir.path(), &["issue", "relate", "1", "2"]);

    // Try to relate again
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "relate", "1", "2"]);

    assert!(success);
    assert!(stdout.contains("already") || stdout.contains("related"));
}

#[test]
fn test_unrelate_no_relation() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue A"]);
    run_atelier(dir.path(), &["issue", "create", "Issue B"]);

    // Try to unrelate issues that aren't related
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "unrelate", "1", "2"]);

    assert!(success);
    assert!(
        stdout.contains("already-absent"),
        "Expected idempotent unblock message, got: {}",
        stdout
    );
}

#[test]
fn test_related_no_relations() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Solo issue"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "related", "1"]);

    assert!(success);
    assert!(
        stdout.contains("No dependencies found."),
        "Expected empty blocker list message, got: {}",
        stdout
    );
}

#[test]
fn test_related_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "related", "999"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

// --- label.rs: Error cases ---
#[test]
fn test_label_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "label", "999", "bug"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_label_already_exists() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    run_atelier(dir.path(), &["issue", "label", "1", "bug"]);

    // Try to add same label again
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "label", "1", "bug"]);

    assert!(success);
    assert!(
        stdout.contains("Updated issue"),
        "duplicate labels are idempotent updates now, got:\n{stdout}"
    );

    let issue_id = issue_id_by_title(dir.path(), "Issue");
    let issue_text = read_canonical_record(dir.path(), "issues", &issue_id);
    assert_eq!(
        issue_text.matches("- \"bug\"").count(),
        1,
        "duplicate label update should not duplicate canonical labels:\n{issue_text}"
    );
}

#[test]
fn test_unlabel_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "unlabel", "999", "bug"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_unlabel_nonexistent_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "unlabel", "1", "nonexistent"]);

    assert!(success);
    assert!(
        stdout.contains("Updated issue"),
        "removing an absent label should be idempotent, got: {}",
        stdout
    );
}

// --- create.rs: Invalid priority ---
#[test]
fn test_create_invalid_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Issue", "-p", "invalid"]);

    assert!(!success);
    assert!(
        stderr.contains("Invalid") || stderr.contains("priority") || stderr.contains("invalid")
    );
}

// --- create.rs: Unknown template ---
#[test]
fn test_create_unknown_template() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Issue", "-t", "unknown"]);

    assert!(!success);
    assert!(
        stderr.contains("Unknown") || stderr.contains("template") || stderr.contains("unknown")
    );
}

// --- block.rs: Error cases ---
#[test]
fn test_block_self() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "block", "1", "1"]);

    assert!(!success, "Blocking an issue by itself should fail");
    assert!(
        stderr.contains("cannot block itself"),
        "Error should mention self-blocking, got stderr: {}",
        stderr
    );
}

#[test]
fn test_block_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "block", "1", "999"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

// --- session.rs: Session status deleted issue ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_status_deleted_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "To delete"]);
    run_atelier(dir.path(), &["session", "start"]);
    run_atelier(dir.path(), &["session", "work", "1"]);
    run_atelier(dir.path(), &["issue", "delete", "1", "-f"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["session", "status"]);

    assert!(success);
    // Should show issue not found or empty working status
    assert!(stdout.contains("not found") || stdout.contains("#1") || stdout.contains("Session"));
}

// --- show.rs: Show with related issues ---
#[test]
fn test_show_with_related_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Main issue"]);
    run_atelier(dir.path(), &["issue", "create", "Related issue"]);
    run_atelier(dir.path(), &["issue", "relate", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success);
    assert!(stdout.contains("Related") || stdout.contains("#2") || stdout.contains("Main issue"));
}

// --- milestone.rs: Edge cases ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_add_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["milestone", "add", "1", "999"]);

    // Command succeeds but warns about nonexistent issue
    assert!(success);
    assert!(
        stdout.contains("not found")
            || stdout.contains("999")
            || stdout.contains("Warning")
            || stdout.contains("skipping")
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_delete_nonexistent() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["milestone", "delete", "999"]);

    // Command succeeds but reports not found
    assert!(success);
    assert!(stdout.contains("not found") || stdout.contains("999"));
}

// ==================== Security & Stress Tests ====================

/// Test with very long title — within limit should succeed, over limit should fail
#[test]
fn test_stress_very_long_title() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Within the 512-char limit: should succeed
    let ok_title = "A".repeat(512);
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "create", &ok_title]);
    assert!(success);
    assert!(stdout.contains("atelier-"));

    // Verify it can be listed and shown
    let (success, _, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);

    let (success, _, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success);

    // Over the 512-char limit: should fail
    let too_long = "A".repeat(513);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", &too_long]);
    assert!(!success);
}

/// Test with very long description
/// Note: Windows has ~8191 char command line limit, so we use a smaller size
#[test]
fn test_stress_very_long_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Use 5000 chars - safe for Windows command line limits
    let long_desc = "B".repeat(5000);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "create", "Long desc issue", "-d", &long_desc],
    );

    assert!(success);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success);
    // Should contain at least part of the description
    assert!(stdout.contains("BBBB"));
}

/// Test creating many issues (stress test)
#[test]
fn test_stress_many_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create 100 issues
    for i in 0..100 {
        let title = format!("Issue number {}", i);
        let (success, _, _) = run_atelier(dir.path(), &["issue", "create", &title]);
        assert!(success, "Failed to create issue {}", i);
    }

    // Verify list works
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("Issue number 99"));

    // Verify search works on large DB
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "search", "number 50"]);
    assert!(success);
    assert!(stdout.contains("50"));
}

/// Test deeply nested subissues
#[test]
fn test_stress_deep_nesting() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create root issue
    run_atelier(dir.path(), &["issue", "create", "Level 0"]);

    // Create 20 levels of nesting
    for i in 1..=20 {
        let parent_id = i.to_string();
        let title = format!("Level {}", i);
        let (success, _, _) = run_atelier(dir.path(), &["issue", "subissue", &parent_id, &title]);
        assert!(success, "Failed to create subissue at level {}", i);
    }

    // Verify tree command handles deep nesting
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "tree"]);
    assert!(success);
    assert!(stdout.contains("Level 20"));
}

/// Test SQL injection in title (should be safely escaped)
#[test]
fn test_security_sql_injection_title() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let malicious_titles = [
        "'; DROP TABLE issues; --",
        "\" OR 1=1 --",
        "Robert'); DROP TABLE issues;--",
        "1; DELETE FROM issues WHERE 1=1; --",
        "' UNION SELECT * FROM sqlite_master --",
    ];

    for title in malicious_titles {
        let (success, _, _) = run_atelier(dir.path(), &["issue", "create", title]);
        assert!(success, "Failed to create issue with title: {}", title);
    }

    // Verify all issues exist and DB is intact
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("DROP TABLE")); // Title should be stored literally
}

/// Test SQL injection in search
#[test]
fn test_security_sql_injection_search() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Normal issue"]);

    let malicious_searches = [
        "' OR '1'='1",
        "'; DROP TABLE issues; --",
        "\" OR \"\"=\"",
        "%' OR 1=1 --",
    ];

    for query in malicious_searches {
        let (success, _, _) = run_atelier(dir.path(), &["issue", "search", query]);
        // Should not crash, may or may not find results
        assert!(success, "Search crashed with query: {}", query);
    }

    // DB should still be intact
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("Normal issue"));
}

/// Test null bytes in input
/// Note: OS rejects null bytes in command args - this is correct security behavior
#[test]
fn test_security_null_bytes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Null bytes are rejected at the OS level (can't pass via command line)
    // This is actually GOOD security behavior - we test that the app
    // handles other special chars correctly instead
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", "Test with special: \t\r"]);
    assert!(success);

    let (success, _, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
}

/// Test newlines and control characters in input
#[test]
fn test_security_control_characters() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let control_inputs = [
        "Line1\nLine2",
        "Tab\there",
        "Return\rhere",
        "Bell\x07sound",
        "Escape\x1b[31mred",
    ];

    for input in control_inputs {
        let (success, _, _) = run_atelier(dir.path(), &["issue", "create", input]);
        assert!(success, "Failed with input containing control chars");
    }

    let (success, _, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
}

/// Test empty strings
#[test]
fn test_edge_empty_strings() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Empty title - should either fail or succeed (both acceptable, just don't crash)
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "create", ""]);
    if success {
        // If it accepted empty title, verify the issue was created
        assert!(
            stdout.contains("Created issue"),
            "If success, should show created message, got: {}",
            stdout
        );
    }

    // Empty comment
    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    let (_, _, _) = run_atelier(dir.path(), &["issue", "comment", "1", ""]);

    // Empty label
    let (_, _, _) = run_atelier(dir.path(), &["issue", "label", "1", ""]);
}

/// Test integer overflow in IDs
#[test]
fn test_edge_large_ids() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test"]);

    // Very large IDs - should fail with "not found" since issue doesn't exist
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", "9223372036854775807"]); // i64::MAX
    assert!(!success, "Show with non-existent large ID should fail");
    assert!(
        stderr.contains("not found"),
        "Error should say not found, got: {}",
        stderr
    );

    // Overflow ID - should fail with parse error or not found
    let (success, _, _) = run_atelier(dir.path(), &["issue", "show", "99999999999999999999999"]);
    assert!(!success, "Show with overflow ID should fail");

    // Negative IDs - clap may reject or db returns not found
    let (success, _, _) = run_atelier(dir.path(), &["issue", "show", "-1"]);
    assert!(!success, "Show with negative ID should fail");
}

/// Test concurrent-like rapid operations
#[test]
fn test_stress_rapid_operations() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Rapid create/close/reopen cycle
    for i in 0..20 {
        let title = format!("Rapid issue {}", i);
        run_atelier(dir.path(), &["issue", "create", &title]);
        let id = (i + 1).to_string();
        run_atelier(dir.path(), &["issue", "comment", &id, "Rapid comment"]);
        run_atelier(dir.path(), &["issue", "label", &id, "rapid"]);
    }

    // Verify all operations completed
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("Rapid issue 19"));
}

/// Test export/import round-trip preserves data
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_integrity_export_import_roundtrip() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create complex data
    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Parent",
            "-p",
            "high",
            "-d",
            "Parent desc",
        ],
    );
    run_atelier(dir.path(), &["issue", "subissue", "1", "Child"]);
    run_atelier(dir.path(), &["issue", "label", "1", "important"]);
    run_atelier(dir.path(), &["issue", "comment", "1", "Test comment"]);

    // Export
    let export_path = dir.path().join("backup.json");
    let (success, _, _) = run_atelier(
        dir.path(),
        &["export", "-o", export_path.to_str().unwrap(), "-f", "json"],
    );
    assert!(success);

    // Import to new location
    let dir2 = tempdir().unwrap();
    init_atelier(dir2.path());
    std::fs::copy(&export_path, dir2.path().join("backup.json")).unwrap();

    let (success, _, _) = run_atelier(
        dir2.path(),
        &["import", dir2.path().join("backup.json").to_str().unwrap()],
    );
    assert!(success);

    // Verify data integrity - title and structure preserved
    let parent_id = issue_id_by_title(dir2.path(), "Parent");
    let (success, stdout, _) = run_atelier(dir2.path(), &["issue", "show", &parent_id]);
    assert!(success);
    assert!(stdout.contains("Parent"));

    // Verify child was imported
    let (success, stdout, _) = run_atelier(dir2.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("Child") || stdout.contains("#2"));
}

#[test]
fn test_command_result_json_mode_is_rejected_and_human_subset_works() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier_raw(dir.path(), &["--json", "doctor"]);
    assert!(!success, "--json should not be accepted");
    assert!(stderr.contains("unexpected argument '--json'"));

    for args in [
        vec!["issue", "list", "--json"],
        vec!["issue", "show", "1", "--json"],
        vec!["issue", "update", "1", "--priority", "high", "--json"],
        vec!["mission", "list", "--json"],
        vec!["workflow", "check", "--json"],
        vec!["doctor", "--json"],
    ] {
        let (success, _, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(!success, "{args:?} should reject --json");
        assert!(
            stderr.contains("unexpected argument '--json'"),
            "{args:?} stderr did not reject --json: {stderr}"
        );
    }

    for args in [
        vec!["issue", "--help"],
        vec!["mission", "--help"],
        vec!["workflow", "--help"],
        vec!["doctor", "--help"],
    ] {
        let (success, stdout, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(success, "{args:?} help failed: {stderr}");
        assert!(
            !stdout.contains("--json"),
            "{args:?} help still advertises --json:\n{stdout}"
        );
    }

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Agent Factory task",
            "--issue-type",
            "feature",
            "--label",
            "agent-factory",
            "--description",
            "## Description\n\nWork item.\n\n## Outcome\n\nFactory task remains valid for human command-result checks.\n\n## Evidence\n\n- `atelier lint` passes for the command-result fixture.",
        ],
    );
    assert!(success, "create failed: {stderr}");
    assert!(stdout.contains("Created issue atelier-"));
    assert!(stdout.contains("Type:     feature"));
    assert!(stdout.contains("Next Commands"));
    let task_id = issue_id_by_title(dir.path(), "Agent Factory task");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "update", "1", "--claim"]);
    assert!(!success, "claim update should be rejected");
    assert!(stderr.contains("unexpected argument '--claim'"));

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "update", "1", "--priority", "high"]);
    assert!(success, "update failed: {stderr}");
    assert!(stdout.contains(&format!("Updated issue {task_id}")));
    assert!(stdout.contains("Priority: high"));

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "note", &task_id, "handoff note"]);
    assert!(success, "issue note failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &task_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains(&task_id));
    assert!(stdout.contains("handoff note"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success, "ready failed: {stderr}");
    assert!(stdout.contains("1 total"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Blocker",
            "--description",
            "## Description\n\nBlocking fixture issue.\n\n## Outcome\n\nBlocker issue participates in dependency command-result checks.\n\n## Evidence\n\n- `atelier lint` passes for the command-result fixture.",
        ],
    );
    assert!(success, "blocker create failed: {stderr}");
    let blocker_id = issue_ref(dir.path(), 2);
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "block", "1", "2"]);
    assert!(success, "issue block failed: {stderr}");
    assert!(stdout.contains(&task_id));
    assert!(stdout.contains(&blocker_id));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "blocked", "1"]);
    assert!(success, "issue blocked failed: {stderr}");
    assert!(stdout.contains(&blocker_id));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "unblock", "1", "2"]);
    assert!(success, "issue unblock failed: {stderr}");
    assert!(stdout.contains(&task_id));
    assert!(stdout.contains(&blocker_id));

    for args in [
        vec!["issue", "list", "--status", "all"],
        vec!["search", "Factory"],
        vec!["lint"],
        vec!["export", "--check"],
        vec!["doctor"],
        vec!["rebuild"],
    ] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "{args:?} failed: {stderr}");
        assert!(!stdout.trim_start().starts_with('{'), "{args:?}");
    }

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", "missing"]);
    assert!(!success);
    assert!(stderr.contains("missing"));
}

#[test]
fn test_wrong_kind_record_ids_report_actual_kind_and_correct_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "create",
            "Corrective mission",
            "--body",
            "Mission body",
            "--validation",
            "Wrong-kind command output is corrective",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Corrective mission");
    let mission_id = mission_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Corrective issue",
            "--description",
            "## Description\n\nIssue fixture.\n\n## Outcome\n\nWrong-kind command output is corrective.\n\n## Evidence\n\n- Focused CLI checks pass.",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = record_id_by_title(dir.path(), "issues", "Corrective issue");
    let issue_id = issue_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "wrong-kind fixture evidence",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    let evidence_id = record_id_by_title(dir.path(), "evidence", "wrong-kind fixture evidence");
    let evidence_id = evidence_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", mission_id]);
    assert!(!success, "mission ID should not resolve as an issue");
    assert!(
        stderr.contains(&format!(
            "{mission_id} is a mission record, not an issue record"
        )),
        "wrong-kind issue read should name actual and expected kinds: {stderr}"
    );
    assert!(
        stderr.contains(&format!("atelier mission show {mission_id}")),
        "wrong-kind issue read should suggest mission show: {stderr}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            mission_id,
            "close",
            "--reason",
            "wrong kind",
        ],
    );
    assert!(!success, "mission ID should not transition as an issue");
    assert!(
        stderr.contains(&format!(
            "{mission_id} is a mission record, not an issue record"
        )),
        "wrong-kind issue mutation should name actual and expected kinds: {stderr}"
    );
    assert!(
        stderr.contains(&format!("atelier mission show {mission_id}")),
        "wrong-kind issue mutation should suggest mission show: {stderr}"
    );

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", evidence_id]);
    assert!(!success, "evidence ID should not resolve as an issue");
    assert!(
        stderr.contains(&format!(
            "{evidence_id} is a evidence record, not an issue record"
        )),
        "wrong-kind evidence lookup should name actual and expected kinds: {stderr}"
    );
    assert!(
        stderr.contains(&format!("atelier evidence show {evidence_id}")),
        "wrong-kind evidence lookup should suggest evidence show: {stderr}"
    );

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "status", issue_id]);
    assert!(!success, "issue ID should not resolve as a mission");
    assert!(
        stderr.contains(&format!(
            "{issue_id} is a issue record, not a mission record"
        )),
        "wrong-kind mission read should name actual and expected kinds: {stderr}"
    );
    assert!(
        stderr.contains(&format!("atelier issue show {issue_id}")),
        "wrong-kind mission read should suggest issue show: {stderr}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "note", issue_id, "wrong kind mission note"],
    );
    assert!(!success, "issue ID should not accept a mission note");
    assert!(
        stderr.contains(&format!(
            "{issue_id} is a issue record, not a mission record"
        )),
        "wrong-kind mission mutation should name actual and expected kinds: {stderr}"
    );
    assert!(
        stderr.contains(&format!("atelier issue show {issue_id}")),
        "wrong-kind mission mutation should suggest issue show: {stderr}"
    );

    let unknown_id = "atelier-zzzz";
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", unknown_id]);
    assert!(!success, "unknown ID should fail");
    assert!(
        stderr.contains("was not found"),
        "unknown ID should keep concise not-found error: {stderr}"
    );
    assert!(
        !stderr.contains("record, not"),
        "unknown ID should not imply a wrong-kind match: {stderr}"
    );
}

#[test]
fn test_first_class_records_export_rebuild_and_validate() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "create",
            "Ship records",
            "--body",
            "Mission body",
            "--constraint",
            "Keep issues accountable",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Ship records");
    let mission_id = mission_id.as_str();
    let (success, mission_update, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "update",
            mission_id,
            "--body",
            "Updated mission body",
            "--risk",
            "Projection drift",
            "--validation",
            "Run focused mission checks",
        ],
    );
    assert!(success, "mission update failed: {stderr}");
    assert!(mission_update.contains("Status: ready"));

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &["evidence", "record", "--kind", "test", "cargo test passed"],
    );
    assert!(success, "evidence record failed: {stderr}");
    assert!(evidence_out.contains("[evidence] recorded - cargo test passed"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "cargo test passed");
    let evidence_id = evidence_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", mission_id]);
    assert!(!success, "mission ID should not resolve as an issue");
    assert!(
        stderr.contains(&format!(
            "{mission_id} is a mission record, not an issue record"
        )),
        "wrong-kind error should name actual and expected kinds: {stderr}"
    );
    assert!(
        stderr.contains(&format!("atelier mission show {mission_id}")),
        "wrong-kind error should suggest mission show: {stderr}"
    );

    let unknown_id = "atelier-zzzz";
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", unknown_id]);
    assert!(!success, "unknown ID should fail");
    assert!(
        stderr.contains("was not found"),
        "unknown ID should keep concise not-found error: {stderr}"
    );
    assert!(
        !stderr.contains("is a mission record"),
        "unknown ID should not imply a wrong-kind match: {stderr}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", evidence_id, "mission", mission_id],
    );
    assert!(success, "evidence attach failed: {stderr}");

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Wire mission work",
            "--issue-type",
            "task",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Wire mission work");
    let issue_id = issue_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, issue_id]);
    assert!(success, "mission-work link failed: {stderr}");
    let (success, blocker_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Resolve mission blocker",
            "--issue-type",
            "task",
        ],
    );
    assert!(success, "blocker issue create failed: {stderr}");
    assert!(blocker_out.contains("Created issue atelier-"));
    let blocker_id = issue_id_by_title(dir.path(), "Resolve mission blocker");
    let blocker_id = blocker_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-blocker", mission_id, blocker_id],
    );
    assert!(success, "mission-blocker link failed: {stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed: {stderr}");

    let mission_path = dir
        .path()
        .join(".atelier")
        .join("missions")
        .join(format!("{mission_id}.md"));
    let mission_markdown = std::fs::read_to_string(&mission_path).unwrap();
    assert!(mission_markdown.contains("schema: \"atelier.mission\""));
    assert!(mission_markdown.contains("schema_version: 1"));
    assert!(!mission_markdown.contains("\ndata: "));
    assert!(mission_markdown.contains("labels:\n- \"mission\"\n"));
    assert!(mission_markdown.contains("## Intent\n\nUpdated mission body"));
    assert!(mission_markdown.contains("## Constraints\n\n- Keep issues accountable"));
    assert!(mission_markdown.contains("## Risks\n\n- Projection drift"));
    assert!(mission_markdown.contains("## Validation\n\n- Run focused mission checks"));
    assert!(mission_markdown.contains("relationships:"));
    assert!(!mission_markdown.contains("  attachments:\n  - kind: \"issue\""));
    assert!(mission_markdown.contains(&format!(
        "  - kind: \"issue\"\n    id: \"{blocker_id}\"\n    type: \"blocked_by\""
    )));
    assert!(mission_markdown.contains(&format!(
        "  - kind: \"issue\"\n    id: \"{issue_id}\"\n    type: \"advances\""
    )));

    let evidence_path = dir
        .path()
        .join(".atelier")
        .join("evidence")
        .join(format!("{evidence_id}.md"));
    let evidence_markdown = std::fs::read_to_string(&evidence_path).unwrap();
    assert!(evidence_markdown.contains("schema: \"atelier.evidence\""));
    assert!(!evidence_markdown.contains("\ndata: "));
    assert!(evidence_markdown.contains("evidence_type: \"test\""));
    assert!(evidence_markdown.contains(&format!("id: \"{mission_id}\"")));

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, view_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(view_out.contains("Records: evidence=1"));
    assert!(view_out.contains("Work: ready=1 blocked=0 done=0 backlog=0"));
    assert!(view_out.contains(&blocker_id));

    let (success, show_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(show_out.contains("Evidence"));
    assert!(show_out.contains("Mission Blockers"));

    let (success, human_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "human mission show failed: {stderr}");
    assert!(human_out.contains(&format!("Mission {mission_id} [ready] - Ship records")));
    assert!(human_out.contains("Constraints"));
    assert!(human_out.contains("Keep issues accountable"));
    assert!(human_out.contains("Progress"));
    assert!(human_out.contains("Records: evidence=1"));
    assert!(human_out.contains("Work: ready=1 blocked=0 done=0 backlog=0"));
    assert!(human_out.contains("Evidence"));
    assert!(human_out.contains("cargo test passed"));
    assert!(human_out.contains("Mission Blockers"));
    assert!(human_out.contains("Resolve mission blocker"));
    assert!(human_out.contains("(open blocker)"));
    assert!(human_out.contains("Linked Work"));
    assert!(human_out.contains("Ready (1)"));
    assert!(human_out.contains("Wire mission work"));
    assert!(human_out.contains("Evidence Gaps"));
    assert!(human_out.contains("(none)"));
    assert!(human_out.contains("Next Commands"));
    assert!(human_out.contains("atelier mission status"));

    let (success, evidence_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", evidence_id]);
    assert!(success, "human evidence show failed: {stderr}");
    assert!(evidence_show.contains(&format!(
        "{evidence_id} [evidence] recorded - cargo test passed"
    )));
    assert!(evidence_show.contains("Status:"));
    assert!(evidence_show.contains("Kind:"));
    assert!(evidence_show.contains("Summary"));

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "human evidence list failed: {stderr}");
    assert!(evidence_list.contains("Evidence"));
    assert!(evidence_list.contains("1 total"));
    assert!(evidence_list.contains("cargo test passed"));
}

#[test]
fn test_mission_relationship_filtering_keeps_supporting_records_out_of_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Filtered mission"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Filtered mission");
    let mission_id = mission_id.as_str();

    let (success, work_out, stderr) = run_atelier(dir.path(), &["issue", "create", "Counted work"]);
    assert!(success, "work issue create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Counted work");
    let work_id = work_id.as_str();

    let (success, support_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Supporting reference"]);
    assert!(success, "support issue create failed: {stderr}");
    assert!(support_out.contains("Created issue atelier-"));
    let support_id = issue_id_by_title(dir.path(), "Supporting reference");
    let support_id = support_id.as_str();

    let (success, blocker_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Direct blocker"]);
    assert!(success, "blocker issue create failed: {stderr}");
    assert!(blocker_out.contains("Created issue atelier-"));
    let blocker_id = issue_id_by_title(dir.path(), "Direct blocker");
    let blocker_id = blocker_id.as_str();

    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, work_id]);
    assert!(success, "mission add-work failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-blocker", mission_id, blocker_id],
    );
    assert!(success, "mission add-blocker failed: {stderr}");

    let mission_path = dir
        .path()
        .join(".atelier")
        .join("missions")
        .join(format!("{mission_id}.md"));
    let mission_markdown = std::fs::read_to_string(&mission_path).unwrap();
    std::fs::write(
        &mission_path,
        mission_markdown.replace(
            "schema: \"atelier.mission\"",
            &format!(
                "  - kind: \"issue\"\n    id: \"{support_id}\"\n    type: \"related\"\nschema: \"atelier.mission\""
            ),
        ),
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(
        success,
        "rebuild after supporting relation edit failed: {stderr}"
    );

    let (success, show_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(show_out.contains("Work: ready=1 blocked=0 done=0 backlog=0"));
    assert!(show_out.contains("Mission Blockers: 1"));
    assert!(show_out.contains("Linked Work"));
    assert!(show_out.contains("Counted work"));
    assert!(show_out.contains("Supporting Records"));
    assert!(show_out.contains("Supporting reference (related)"));

    let linked_work = show_out
        .split("Linked Work")
        .nth(1)
        .and_then(|text| text.split("Supporting Records").next())
        .unwrap_or("");
    assert!(
        !linked_work.contains("Supporting reference"),
        "supporting relation was rendered as linked work:\n{show_out}"
    );

    let (success, status_out, stderr) = run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Total: 1 ready"));
    assert!(status_out.contains("Mission blockers: 1 open"));
}

#[test]
fn test_mission_unlink_removes_added_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Repair mission"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Repair mission");
    let mission_id = mission_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Accidental work"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Accidental work");
    let issue_id = issue_id.as_str();

    let (success, add_out, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, issue_id]);
    assert!(success, "mission add-work failed: {stderr}");
    assert!(add_out.contains(&format!("Added work {issue_id} to mission {mission_id}")));

    let (success, linked_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "mission show after add-work failed: {stderr}");
    assert!(linked_out.contains("Linked Work"));
    assert!(linked_out.contains("Accidental work"));

    let (success, unlink_out, stderr) =
        run_atelier(dir.path(), &["mission", "unlink", mission_id, issue_id]);
    assert!(success, "mission unlink failed: {stderr}");
    assert!(unlink_out.contains(&format!(
        "Unlinked work {issue_id} from mission {mission_id}"
    )));

    let (success, show_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "mission show after unlink failed: {stderr}");
    assert!(show_out.contains("Linked Work"));
    assert!(!show_out.contains("Accidental work"));
    assert!(show_out.contains("Work: ready=0 blocked=0 done=0 backlog=0"));

    let mission_markdown = read_canonical_record(dir.path(), "missions", mission_id);
    assert!(!mission_markdown.contains(&format!(
        "  - kind: \"issue\"\n    id: \"{issue_id}\"\n    type: \"advances\""
    )));
}

#[test]
fn test_evidence_capture_records_command_metadata_and_attaches_targets() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Capture issue", "--issue-type", "task"],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Capture issue");
    let issue_id = issue_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Capture epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Capture epic");
    let epic_id = epic_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Capture mission"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Capture mission");
    let mission_id = mission_id.as_str();

    let (success, issue_capture, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--summary",
            "issue command proof",
            "--target",
            &format!("issue/{issue_id}"),
            "--",
            "sh",
            "-c",
            "printf 'pass stdout\\n'; printf 'pass stderr\\n' >&2",
        ],
    );
    assert!(success, "issue command record failed: {stderr}");
    assert!(issue_capture.contains("[evidence] recorded - issue command proof"));
    assert!(issue_capture.contains("Command:     sh -c"));
    assert!(issue_capture.contains("Exit Status: 0"));
    assert!(issue_capture.contains(&format!("Target:      issue/{issue_id} (validates)")));
    assert!(issue_capture.contains("Captured:"));
    assert!(issue_capture.contains("pass stdout"));
    assert!(issue_capture.contains("pass stderr"));
    let issue_evidence_id = record_id_by_title(dir.path(), "evidence", "issue command proof");
    let issue_evidence_front_matter =
        canonical_evidence_front_matter(dir.path(), &issue_evidence_id);
    assert!(issue_evidence_front_matter["proof_scope"].is_null());
    assert!(issue_evidence_front_matter["independence_level"].is_null());
    assert!(issue_evidence_front_matter["agent_identity"].is_null());
    assert!(issue_evidence_front_matter["residual_risks"].is_null());
    assert!(issue_evidence_front_matter["follow_up_ids"].is_null());
    assert_eq!(issue_evidence_front_matter["evidence_type"], "validation");
    assert_eq!(issue_evidence_front_matter["status"], "recorded");
    assert!(issue_evidence_front_matter["command"]
        .as_str()
        .unwrap()
        .starts_with("sh -c"));
    let issue_evidence_markdown = read_canonical_record(dir.path(), "evidence", &issue_evidence_id);
    assert!(!issue_evidence_markdown.contains("\noutput:"));
    assert!(issue_evidence_markdown.contains("## Command\n\n```console\nsh -c"));
    assert!(issue_evidence_markdown.contains("## Stdout\n\nBytes: 12\nTruncated: no"));
    assert!(issue_evidence_markdown.contains("## Stderr\n\nBytes: 12\nTruncated: no"));

    let (success, record_capture, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--summary",
            "unified command proof",
            "--target",
            &format!("issue/{issue_id}"),
            "--",
            "sh",
            "-c",
            "printf 'record stdout\\n'",
        ],
    );
    assert!(success, "unified command record failed: {stderr}");
    assert!(record_capture.contains("[evidence] recorded - unified command proof"));
    assert!(record_capture.contains("Command:     sh -c"));
    assert!(record_capture.contains("Exit Status: 0"));
    assert!(record_capture.contains(&format!("Target:      issue/{issue_id} (validates)")));
    assert!(record_capture.contains("record stdout"));

    let positional_summary = "unified positional manual proof";
    let (success, positional_record_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--target",
            &format!("issue/{issue_id}"),
            positional_summary,
        ],
    );
    assert!(success, "positional manual record failed: {stderr}");
    assert!(positional_record_out.contains("[evidence] recorded - unified positional manual proof"));
    assert!(positional_record_out.contains(&format!("Target:      issue/{issue_id} (validates)")));
    let positional_evidence_id = record_id_by_title(dir.path(), "evidence", positional_summary);
    let positional_evidence_front_matter =
        canonical_evidence_front_matter(dir.path(), &positional_evidence_id);
    assert_eq!(
        positional_evidence_front_matter["evidence_type"],
        "validation"
    );
    assert!(positional_evidence_front_matter["proof_scope"].is_null());
    assert!(positional_evidence_front_matter["independence_level"].is_null());
    assert!(positional_evidence_front_matter["agent_identity"].is_null());
    assert!(positional_evidence_front_matter["residual_risks"].is_null());
    assert!(positional_evidence_front_matter["follow_up_ids"].is_null());
    assert_eq!(positional_evidence_front_matter["status"], "recorded");
    let positional_markdown =
        read_canonical_record(dir.path(), "evidence", &positional_evidence_id);
    assert!(!positional_markdown.contains("command: null"));
    assert!(!positional_markdown.contains("path: null"));
    assert!(!positional_markdown.contains("uri: null"));
    assert!(!positional_markdown.contains("target: null"));
    assert!(!positional_markdown.contains("output:"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--summary",
            "flag summary",
            "positional summary",
        ],
    );
    assert!(!success, "conflicting summaries should fail");
    assert!(
        stderr.contains("use either --summary or a positional summary"),
        "conflict error should be actionable: {stderr}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--summary",
            "epic failing command proof",
            "--target",
            &format!("epic/{epic_id}"),
            "--",
            "sh",
            "-c",
            "printf 'failing stdout\\n'; printf 'failing stderr\\n' >&2; exit 7",
        ],
    );
    assert!(success, "epic failing command record failed: {stderr}");
    let epic_evidence_id = record_id_by_title(dir.path(), "evidence", "epic failing command proof");
    let (success, epic_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &epic_evidence_id]);
    assert!(success, "epic evidence show failed: {stderr}");
    assert!(epic_show.contains("Status:      recorded"));
    assert!(epic_show.contains("Exit Status: 7"));
    assert!(epic_show.contains(&format!("Target:      epic/{epic_id} (validates)")));
    assert!(epic_show.contains("failing stdout"));
    assert!(epic_show.contains("failing stderr"));

    let manual_epic_summary =
        "manual epic contract audit line-by-line classification maps epic outcome lines to proof";
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            manual_epic_summary,
        ],
    );
    assert!(success, "manual evidence record failed: {stderr}");
    let manual_epic_evidence_id = record_id_by_title(dir.path(), "evidence", manual_epic_summary);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "attach",
            &manual_epic_evidence_id,
            "epic",
            epic_id,
        ],
    );
    assert!(success, "manual epic evidence attach failed: {stderr}");
    let (success, manual_epic_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &manual_epic_evidence_id]);
    assert!(success, "manual epic evidence show failed: {stderr}");
    assert!(manual_epic_show.contains(&format!("Target:      epic/{epic_id} (validates)")));

    let manual_issue_summary = "unified manual issue proof";
    let (success, manual_record_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--summary",
            manual_issue_summary,
            "--target",
            &format!("issue/{issue_id}"),
        ],
    );
    assert!(success, "unified manual record failed: {stderr}");
    assert!(manual_record_out.contains("[evidence] recorded - unified manual issue proof"));
    assert!(manual_record_out.contains(&format!("Target:      issue/{issue_id} (validates)")));
    let manual_issue_evidence_id = record_id_by_title(dir.path(), "evidence", manual_issue_summary);
    let manual_issue_front_matter =
        canonical_evidence_front_matter(dir.path(), &manual_issue_evidence_id);
    assert_eq!(manual_issue_front_matter["target"]["kind"], "issue");
    assert_eq!(manual_issue_front_matter["target"]["id"], issue_id);
    assert_eq!(manual_issue_front_matter["target"]["role"], "validates");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--summary",
            "mission blocked command proof",
            "--target",
            &format!("mission/{mission_id}"),
            "--",
            "sh",
            "-c",
            "i=0; while [ $i -lt 350 ]; do printf 'blocked-line-%03d\\n' \"$i\"; i=$((i + 1)); done; printf 'blocked stderr\\n' >&2; exit 2",
        ],
    );
    assert!(success, "mission blocked command record failed: {stderr}");
    let mission_evidence_id =
        record_id_by_title(dir.path(), "evidence", "mission blocked command proof");
    let (success, mission_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &mission_evidence_id]);
    assert!(success, "mission evidence show failed: {stderr}");
    assert!(mission_show.contains("Status:      recorded"));
    assert!(mission_show.contains("Exit Status: 2"));
    assert!(mission_show.contains(&format!("Target:      mission/{mission_id} (validates)")));
    assert!(mission_show.contains("blocked-line-000"));
    assert!(!mission_show.contains("blocked-line-349"));
    assert!(mission_show.contains("Stdout: "));
    assert!(mission_show.contains("truncated: yes"));

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "evidence list failed: {stderr}");
    assert!(evidence_list.contains(&issue_evidence_id));
    assert!(evidence_list.contains("exit=0"));
    assert!(evidence_list.contains(&format!("target=issue/{issue_id}")));
    assert!(evidence_list.contains("command=sh -c"));
    assert!(evidence_list.contains(&epic_evidence_id));
    assert!(evidence_list.contains("exit=7"));
    assert!(evidence_list.contains(&format!("target=epic/{epic_id}")));
    assert!(evidence_list.contains(&manual_epic_evidence_id));
    assert!(evidence_list.contains(&mission_evidence_id));
    assert!(evidence_list.contains("exit=2"));
    assert!(evidence_list.contains(&format!("target=mission/{mission_id}")));
}

#[test]
fn test_evidence_relation_role_errors_are_corrective() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Corrective evidence target"],
    );
    assert!(success, "issue create failed: {stderr}");
    let target_id = issue_id_by_title(dir.path(), "Corrective evidence target");
    let target_id = target_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--target",
            &format!("issue/{target_id}"),
            "--role",
            "validation",
            "bad role proof",
        ],
    );
    assert!(!success, "invalid evidence record role should fail");
    assert_corrective_evidence_role_error(&stderr);

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "evidence list failed: {stderr}");
    assert!(
        evidence_list.contains("(none)"),
        "invalid targeted record should not create evidence: {evidence_list}"
    );

    let (success, record_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--target",
            &format!("issue/{target_id}"),
            "accepted target proof",
        ],
    );
    assert!(success, "accepted evidence record failed: {stderr}");
    assert!(record_out.contains(&format!("Target:      issue/{target_id} (validates)")));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "accepted target proof");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Reused evidence target"]);
    assert!(success, "reuse target issue create failed: {stderr}");
    let reuse_id = issue_id_by_title(dir.path(), "Reused evidence target");
    let reuse_id = reuse_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "attach",
            &evidence_id,
            "issue",
            reuse_id,
            "--role",
            "validation",
        ],
    );
    assert!(!success, "invalid evidence attach role should fail");
    assert_corrective_evidence_role_error(&stderr);

    let (success, attach_out, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", &evidence_id, "issue", reuse_id],
    );
    assert!(success, "accepted evidence attach failed: {stderr}");
    assert!(attach_out.contains(&format!(
        "Attached evidence {evidence_id} to issue {reuse_id} (validates)"
    )));

    let (success, show_out, stderr) = run_atelier(dir.path(), &["evidence", "show", &evidence_id]);
    assert!(success, "evidence show failed: {stderr}");
    assert!(show_out.contains(&format!("issue/{target_id} (validates)")));
    assert!(show_out.contains(&format!("issue/{reuse_id} (validates)")));
}

fn assert_corrective_evidence_role_error(stderr: &str) {
    assert!(
        stderr.contains("Invalid evidence relation role 'validation'"),
        "error should name invalid role: {stderr}"
    );
    assert!(
        stderr.contains("Accepted evidence relation vocabulary: validates"),
        "error should name accepted relation vocabulary: {stderr}"
    );
    assert!(
        stderr.contains("Evidence kinds such as validation belong in --kind, not --role"),
        "error should distinguish evidence kind from relation role: {stderr}"
    );
    assert!(
        stderr
            .contains("atelier evidence record --target issue/<id> --kind validation \"summary\""),
        "error should name normal targeted record flow: {stderr}"
    );
    assert!(
        stderr.contains("atelier evidence attach <evidence-id> issue <issue-id>"),
        "error should name existing-proof attach flow: {stderr}"
    );
}

#[test]
fn test_evidence_capture_records_nonzero_exit_as_command_metadata() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Rejected invalid input",
            "--issue-type",
            "validation",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Rejected invalid input");
    let issue_id = issue_id.as_str();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--summary",
            "invalid input is rejected",
            "--target",
            &format!("issue/{issue_id}"),
            "--",
            "sh",
            "-c",
            "printf 'rejected\\n'; exit 3",
        ],
    );
    assert!(
        success,
        "nonzero command should still record evidence: {stderr}"
    );
    assert!(stdout.contains("[evidence] recorded - invalid input is rejected"));
    assert!(stdout.contains("Exit Status: 3"));
    assert!(stdout.contains(&format!("Target:      issue/{issue_id} (validates)")));

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "evidence list failed: {stderr}");
    assert!(evidence_list.contains("recorded"));
    assert!(evidence_list.contains("exit=3"));
    assert!(evidence_list.contains("invalid input is rejected"));
}

#[test]
fn test_issue_closeout_rejects_evidence_attached_to_another_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Target proof"]);
    assert!(success, "target issue create failed: {stderr}");
    let target_id = issue_id_by_title(dir.path(), "Target proof");
    let target_id = target_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Donor proof"]);
    assert!(success, "donor issue create failed: {stderr}");
    let donor_id = issue_id_by_title(dir.path(), "Donor proof");
    let donor_id = donor_id.as_str();

    let evidence_id = attach_issue_pass_evidence(dir.path(), donor_id);

    move_issue_to_validation(dir.path(), target_id);
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            target_id,
            "close",
            "--reason",
            "done",
        ],
    );
    assert!(
        !success,
        "issue closeout must reject evidence linked only to another issue"
    );
    assert!(
        stderr.contains("expected at least 1 passing evidence record")
            || stderr.contains("expected at least 1 validating evidence record")
            || stderr.contains("no validating evidence link found"),
        "{stderr}"
    );
    assert!(stderr.contains(target_id));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", &evidence_id, "issue", target_id],
    );
    assert!(success, "target evidence attach failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            target_id,
            "close",
            "--reason",
            "target proof",
        ],
    );
    assert!(
        success,
        "target closeout should pass after direct proof: {stderr}"
    );
}

#[test]
fn test_issue_closeout_uses_attached_pass_evidence_not_evidence_text() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let issue_body = "## Description\n\nEvidence gate body.\n\n## Outcome\n\nThe issue can close after workflow proof is attached.\n\n## Evidence\n\n- A focused command transcript proves the workflow change.";
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Evidence gate proof",
            "--description",
            issue_body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Evidence gate proof");

    move_issue_to_validation(dir.path(), &issue_id);
    attach_pass_evidence(
        dir.path(),
        "issue",
        &issue_id,
        "workflow close gate regression transcript recorded",
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "close",
            &issue_id,
            "--reason",
            "workflow proof attached",
        ],
    );
    assert!(
        success,
        "issue closeout should use attached pass evidence rather than Evidence text matching: {stderr}"
    );
}

#[test]
fn test_validation_issue_closeout_uses_workflow_approval_not_contract_audit_terms() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let validation_body = "## Description\n\nValidation item body.\n\n## Outcome\n\nThe validation item can close after independent approval is attached.\n\n## Evidence\n\n- `atelier evidence show <id>` displays the approval record attached to this validation issue.";
    let (success, validation_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "High-risk validation proof",
            "--issue-type",
            "validation",
            "--description",
            validation_body,
        ],
    );
    assert!(success, "validation issue create failed: {stderr}");
    assert!(validation_out.contains("Created issue atelier-"));
    let validation_id = issue_id_by_title(dir.path(), "High-risk validation proof");

    move_issue_to_validation(dir.path(), &validation_id);
    attach_pass_evidence(
        dir.path(),
        "issue",
        &validation_id,
        "independent approval evidence recorded for workflow closeout",
    );

    let (success, transitions, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &validation_id, "--options"],
    );
    assert!(success, "transition options failed: {stderr}");
    assert!(transitions.contains("close"));
    assert!(transitions.contains("pass  evidence.attached"));
    assert!(!transitions.contains("contract-audit"));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "close",
            &validation_id,
            "--reason",
            "independent approval attached",
        ],
    );
    assert!(
        success,
        "validation issue closeout should use workflow approval plus attached evidence: {stderr}"
    );
}

#[test]
fn test_issue_closeout_requires_passing_evidence_records() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let issue_body = "## Description\n\nValidation blocker body.\n\n## Outcome\n\nThe issue does not close without passing evidence.\n\n## Evidence\n\n- A passing transcript proves closeout readiness.";
    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Blocked validation proof",
            "--description",
            issue_body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Blocked validation proof");

    move_issue_to_validation(dir.path(), &issue_id);
    attach_non_pass_evidence(
        dir.path(),
        "issue",
        &issue_id,
        "blocked",
        "blocked validation transcript recorded",
    );

    let (success, transitions, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(transitions.contains("close"));
    assert!(transitions.contains("expected at least 1 passing evidence record"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &issue_id,
            "close",
            "--reason",
            "still blocked",
        ],
    );
    assert!(
        !success,
        "closeout must reject evidence that is attached but not passing"
    );
    assert!(stderr.contains("expected at least 1 passing evidence record"));
}

#[test]
fn test_mission_status_shows_ignored_product_behavior_closeout_blocker() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &["mission", "create", "Ignored blocker mission"],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Ignored blocker mission");
    let mission_id = mission_id.as_str();

    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Finished behavior work"]);
    assert!(success, "work create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Finished behavior work");
    let work_id = work_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, work_id]);
    assert!(success, "mission add-work failed: {stderr}");
    close_issue_with_evidence(dir.path(), work_id, Some("done"));

    let (success, followup_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Resolve ignored behavior test"],
    );
    assert!(success, "follow-up create failed: {stderr}");
    assert!(followup_out.contains("Created issue atelier-"));
    let followup_id = issue_id_by_title(dir.path(), "Resolve ignored behavior test");
    let followup_id = followup_id.as_str();

    std::fs::create_dir_all(dir.path().join("tests")).unwrap();
    std::fs::write(
        dir.path().join("tests/product_gap.rs"),
        ignored_test_source(
            &format!(
                "ignore = \"reason: product behavior pending migration; issue: {followup_id}; product: yes\""
            ),
            "ignored_product_closeout_gap",
        ),
    )
    .unwrap();

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "ignored blocker evidence",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    assert!(evidence_out.contains("[evidence] recorded - ignored blocker evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "ignored blocker evidence");
    let evidence_id = evidence_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", evidence_id, "mission", mission_id],
    );
    assert!(success, "evidence attach failed: {stderr}");
    commit_all(dir.path(), "ignored test closeout blocker");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Terminal: blocked"));
    assert!(!status_out.contains(&format!(
        "atelier mission update {mission_id} --status closed"
    )));
}

#[test]
fn test_mission_closeout_blocks_undeferred_obsolete_command_test() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    write_valid_command_guidance(dir.path());

    fs::create_dir_all(dir.path().join("tests")).unwrap();
    fs::write(
        dir.path().join("tests/legacy_session.rs"),
        concat!(
            "#[test]\n",
            "fn legacy_session_still_works() {\n",
            "    let (success, _, _) = run_atelier(dir.path(), &[\"session\", \"start\"]);\n",
            "    assert!(success);\n",
            "}\n"
        ),
    )
    .unwrap();

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Stale test closeout"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Stale test closeout");

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "stale test evidence",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    assert!(evidence_out.contains("[evidence] recorded - stale test evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "stale test evidence");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", &evidence_id, "mission", &mission_id],
    );
    assert!(success, "evidence attach failed: {stderr}");
    commit_all(dir.path(), "stale test closeout baseline");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["mission", "close", &mission_id, "--reason", "done"],
    );

    assert!(
        !success,
        "mission closeout should block undeferred obsolete-command tests"
    );
    assert!(stdout.contains("Mission terminal checks blocked"));
    assert!(stdout.contains("docs/help drift: detected"));
    assert!(stdout.contains("update docs, help text, or command-surface tests"));
    assert!(stdout.contains("tests/legacy_session.rs"));
    assert!(stdout.contains("legacy_session_still_works"));
    assert!(stdout.contains("atelier session start"));
    assert!(stderr.contains("mission terminal checks blocked"));
}

#[test]
fn test_mission_status_reports_terminal_checks_and_explicit_approval() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "create",
            "Approval terminal checks",
            "--validation",
            "Human guidance only.",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Approval terminal checks");
    let mission_id = mission_id.as_str();

    let (success, missing_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", "--verbose", mission_id]);
    assert!(
        success,
        "status without work should render terminal checks: {stderr}"
    );
    assert!(missing_out.contains("Terminal Checks"));
    assert!(missing_out.contains("Work: missing"));
    assert!(!missing_out.contains("Mission Closeout Audit"));

    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Finished mission work"]);
    assert!(success, "work create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Finished mission work");
    let work_id = work_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, work_id]);
    assert!(success, "mission add work failed: {stderr}");
    close_issue_with_evidence(dir.path(), work_id, Some("done"));

    let (success, approval_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Mission approval validation",
            "--issue-type",
            "validation",
        ],
    );
    assert!(success, "validation issue create failed: {stderr}");
    assert!(approval_out.contains("Created issue atelier-"));
    let approval_id = issue_id_by_title(dir.path(), "Mission approval validation");
    let approval_id = approval_id.as_str();
    edit_canonical_record(dir.path(), "issues", approval_id, |text| {
        text.replace("No description provided.", "Validation item body.")
            .replace(
                "Outcome was not specified.",
                "Mission approval is represented as linked workflow state.",
            )
            .replace(
                "Evidence was not specified.",
                "- Manual check: `atelier evidence show <id>` displays the approval record attached to this issue.",
            )
    });
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-work", mission_id, approval_id],
    );
    assert!(success, "mission add validation failed: {stderr}");

    let (success, terminal_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", "--verbose", mission_id]);
    assert!(
        success,
        "terminal status should render while approval work is open: {stderr}"
    );
    assert!(terminal_out.contains("Terminal Checks"));
    assert!(terminal_out.contains(approval_id));
    assert!(terminal_out.contains("linked terminal validation work"));

    move_issue_to_validation(dir.path(), approval_id);
    attach_pass_evidence(
        dir.path(),
        "issue",
        approval_id,
        "independent mission approval captured",
    );
    commit_all(dir.path(), "mission approval ready");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            approval_id,
            "close",
            "--reason",
            "approved",
        ],
    );
    assert!(
        success,
        "validation issue transition close failed: {stderr}"
    );

    let (success, ready_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", "--verbose", mission_id]);
    assert!(success, "ready terminal status should pass: {stderr}");
    assert!(ready_out.contains("Terminal Checks"));
    assert!(ready_out.contains("All required terminal checks pass."));
}

#[test]
fn test_mission_closeout_accepts_shell_mission_without_direct_mission_evidence() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "create",
            "Shell closeout",
            "--validation",
            "Human guidance only.",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Shell closeout");
    let mission_id = mission_id.as_str();

    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Shell mission work"]);
    assert!(success, "work create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Shell mission work");
    let work_id = work_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, work_id]);
    assert!(success, "mission add work failed: {stderr}");
    close_issue_with_evidence(dir.path(), work_id, Some("done"));

    let (success, status_out, stderr) = run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Direct mission evidence: none"));
    assert!(status_out.contains("Terminal: ready"));
    assert!(status_out.contains("All required terminal checks pass."));

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "close",
            mission_id,
            "--reason",
            "linked work closed",
        ],
    );
    assert!(
        success,
        "mission closeout should succeed without direct mission evidence: {stderr}"
    );
    assert!(close_out.contains("Status: closed"));
}

#[test]
fn test_workflow_init_is_removed_and_root_init_owns_starter_policy() {
    let dir = tempdir().unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["init"]);
    assert!(success, "root init failed: {stderr}");
    assert!(stdout.contains(".atelier/workflow.yaml"));
    assert!(stdout.contains("atelier lint"));

    let policy_path = dir.path().join(".atelier").join("workflow.yaml");
    let policy = std::fs::read_to_string(&policy_path).unwrap();
    assert!(policy.contains("schema_version: 3"));
    assert!(policy.contains("branch_policy:"));
    assert!(policy.contains("  todo:\n    category: todo"));
    assert!(policy.contains("    initial_status: todo"));
    assert!(policy.contains("    done_statuses: [done]"));
    assert!(policy.contains("  task_delivery:"));
    assert!(policy.contains("  epic_delivery:"));
    assert!(policy.contains("  validation_delivery:"));
    assert!(policy.contains("  spike_review:"));
    assert!(policy.contains("applies_to:"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "init"]);
    assert!(!success, "workflow init should be removed");
    assert!(stdout.is_empty(), "{stdout}");
    assert!(
        stderr.contains("unrecognized subcommand 'init'"),
        "{stderr}"
    );
    assert!(
        !stderr.contains("was removed") && !stderr.contains("atelier init"),
        "{stderr}"
    );
}

#[test]
fn test_workflow_check_rejects_legacy_issue_statuses_without_migration_path() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Legacy status"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Legacy status");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let issue_text = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(
        &issue_path,
        issue_text.replace("status: \"todo\"", "status: \"open\""),
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(
        !success,
        "rebuild should reject legacy status before workflow check"
    );
    assert!(stderr.contains("workflow_issue_status_invalid"), "{stderr}");
    assert!(stderr.contains("open"), "{stderr}");
}

#[test]
fn test_issue_create_after_workflow_init_uses_configured_initial_status() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    migrate_default_issue_workflow(dir.path());

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Workflow initialized issue"],
    );
    assert!(
        success,
        "workflow initialized issue create failed: {stderr}"
    );
    assert!(stdout.contains("Created issue atelier-"), "{stdout}");
    let issue_id = issue_id_by_title(dir.path(), "Workflow initialized issue");
    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(issue_text.contains("status: \"todo\""), "{issue_text}");

    let (success, options, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(options.contains("Status:   todo"), "{options}");
    assert!(options.contains("start [allowed]"), "{options}");

    commit_all(dir.path(), "workflow-ready issue");
    let (success, start_out, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "root start failed: {stderr}");
    assert!(
        start_out.contains("Applied transition start"),
        "{start_out}"
    );
    assert!(start_out.contains("From:     todo"), "{start_out}");
    assert!(start_out.contains("To:       in_progress"), "{start_out}");
}

#[test]
fn test_workflow_check_reports_policy_and_issue_record_health() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Workflow check task"]);
    assert!(success, "issue create failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Workflow check spike",
            "--issue-type",
            "spike",
        ],
    );
    assert!(success, "spike issue create failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
    assert!(success, "workflow check failed: {stderr}");
    assert!(stdout.contains("Workflow Check"));
    assert!(stdout.contains("Path:           .atelier/workflow.yaml"));
    assert!(stdout.contains("Policy:         pass"));
    assert!(stdout.contains("Applicability:"));
    assert!(stdout.contains("Statuses:       6"));
    assert!(stdout.contains("Workflows:      4"));
    assert!(stdout.contains("Record Health:  pass"));
    assert!(stdout.contains("Issues Checked: 2"));
    assert!(stdout.contains("Docs/Help Drift: clear"));
}

#[test]
fn test_workflow_check_rejects_stale_agent_guidance_commands() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_valid_command_guidance(dir.path());
    fs::write(
        dir.path().join("AGENTS.md"),
        "# Agent Instructions\n\n- `atelier timer start`\n",
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);

    assert!(
        !success,
        "workflow check should reject stale AGENTS command"
    );
    assert!(stdout.contains("Docs/Help Drift: detected"), "{stdout}");
    assert!(stdout.contains("AGENTS.md"), "{stdout}");
    assert!(stdout.contains("atelier timer"), "{stdout}");
    assert!(
        stderr.contains("workflow_command_surface_drift"),
        "{stderr}"
    );
}

#[test]
fn test_workflow_check_rejects_stale_agent_guidance_options() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_valid_command_guidance(dir.path());
    fs::write(
        dir.path().join("AGENTS.md"),
        "# Agent Instructions\n\n- `atelier issue list --not-a-real-option`\n",
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);

    assert!(!success, "workflow check should reject stale AGENTS option");
    assert!(stdout.contains("Docs/Help Drift: detected"), "{stdout}");
    assert!(stdout.contains("AGENTS.md"), "{stdout}");
    assert!(stdout.contains("--not-a-real-option"), "{stdout}");
    assert!(
        stderr.contains("workflow_command_surface_drift"),
        "{stderr}"
    );
}

#[test]
fn test_workflow_check_allows_hidden_command_only_in_hidden_context() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_valid_command_guidance(dir.path());

    let surface = format!(
        "{}\n## Low-Level Debug And Repair\n\n- `atelier diagnostics slow`\n",
        valid_command_surface_doc()
    );
    write_command_surface_doc(dir.path(), &surface);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);

    assert!(
        success,
        "workflow check should accept hidden command context: {stderr}"
    );
    assert!(stdout.contains("Docs/Help Drift: clear"), "{stdout}");
}

#[test]
fn test_workflow_check_rejects_hidden_command_as_normal_workflow() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_valid_command_guidance(dir.path());

    let surface = format!(
        "{}\n## Core\n\n- `atelier diagnostics slow`\n",
        valid_command_surface_doc()
    );
    write_command_surface_doc(dir.path(), &surface);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);

    assert!(
        !success,
        "workflow check should reject hidden command as normal guidance"
    );
    assert!(stdout.contains("Docs/Help Drift: detected"), "{stdout}");
    assert!(stdout.contains("atelier diagnostics"), "{stdout}");
    assert!(
        stderr.contains("workflow_command_surface_drift"),
        "{stderr}"
    );
}

#[test]
fn test_workflow_check_allows_removed_command_only_in_removal_history_context() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_valid_command_guidance(dir.path());

    let surface = format!(
        "{}\n## Removed Behavior\n\n- `atelier session start`\n",
        valid_command_surface_doc()
    );
    write_command_surface_doc(dir.path(), &surface);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);

    assert!(
        success,
        "workflow check should accept removal-history context: {stderr}"
    );
    assert!(stdout.contains("Docs/Help Drift: clear"), "{stdout}");
}

#[test]
fn test_workflow_check_rejects_removed_command_as_normal_workflow() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_valid_command_guidance(dir.path());

    let surface = format!(
        "{}\n## Core\n\n- `atelier timer start`\n",
        valid_command_surface_doc()
    );
    write_command_surface_doc(dir.path(), &surface);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);

    assert!(
        !success,
        "workflow check should reject removed command as normal guidance"
    );
    assert!(stdout.contains("Docs/Help Drift: detected"), "{stdout}");
    assert!(stdout.contains("atelier timer"), "{stdout}");
    assert!(
        stderr.contains("workflow_command_surface_drift"),
        "{stderr}"
    );
}

#[test]
fn test_workflow_check_rejects_nonexistent_option_in_hidden_context() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_valid_command_guidance(dir.path());

    let surface = format!(
        "{}\n## Low-Level Debug And Repair\n\n- `atelier diagnostics slow --not-a-real-option`\n",
        valid_command_surface_doc()
    );
    write_command_surface_doc(dir.path(), &surface);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);

    assert!(
        !success,
        "workflow check should reject nonexistent option in hidden context"
    );
    assert!(stdout.contains("Docs/Help Drift: detected"), "{stdout}");
    assert!(stdout.contains("--not-a-real-option"), "{stdout}");
    assert!(
        stderr.contains("workflow_command_surface_drift"),
        "{stderr}"
    );
}

#[test]
fn test_workflow_check_rejects_issue_status_outside_selected_workflow() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Workflow mismatch"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Workflow mismatch");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let issue_text = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(
        &issue_path,
        issue_text.replace("status: \"todo\"", "status: \"qa_hold\""),
    )
    .unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(
        !success,
        "rebuild should reject status mismatch before workflow check"
    );
    assert!(
        stderr.contains("workflow_issue_status_invalid"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains(&issue_id), "stderr: {stderr}");
    assert!(stderr.contains("qa_hold"), "stderr: {stderr}");
    assert!(
        stderr.contains("allowed statuses: blocked, done, in_progress, todo, validation"),
        "stderr: {stderr}"
    );
}
