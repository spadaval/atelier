use super::*;

fn run_atelier_with_deadlock_timeout(dir: &Path, args: &[&str]) -> std::process::Output {
    ensure_git_for_workflow_fixture(dir, args);
    let mut child = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn timed Atelier command");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(15);
    loop {
        if child
            .try_wait()
            .expect("failed to inspect timed Atelier command")
            .is_some()
        {
            return child
                .wait_with_output()
                .expect("failed to collect timed Atelier command output");
        }
        if std::time::Instant::now() >= deadline {
            child
                .kill()
                .expect("failed to kill deadlocked Atelier command");
            let output = child
                .wait_with_output()
                .expect("failed to collect deadlocked Atelier command output");
            panic!(
                "Atelier command exceeded deadlock deadline; stdout: {}; stderr: {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn evidence_tree_contains(root: &Path, text: &str) -> bool {
    let evidence_dir = root.join("evidence");
    fs::read_dir(evidence_dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter_map(|entry| fs::read_to_string(entry.path()).ok())
        .any(|contents| contents.contains(text))
}

// ==================== Additional Edge Case Coverage ====================

// --- relate.rs: Error cases ---
#[test]
fn test_relate_nonexistent_first_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Existing"]);
    let existing_id = issue_ref(dir.path(), 1);

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", "999", &existing_id, "--role", "blocked_by"],
    );

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_relate_nonexistent_second_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Existing"]);
    let existing_id = issue_ref(dir.path(), 1);

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &existing_id, "999", "--role", "blocked_by"],
    );

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_relate_already_related() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue A"]);
    run_atelier(dir.path(), &["issue", "create", "Issue B"]);
    let issue_a = issue_ref(dir.path(), 1);
    let issue_b = issue_ref(dir.path(), 2);
    run_atelier(
        dir.path(),
        &["issue", "link", &issue_a, &issue_b, "--role", "blocked_by"],
    );

    // Try to relate again
    let (success, stdout, _) = run_atelier(
        dir.path(),
        &["issue", "link", &issue_a, &issue_b, "--role", "blocked_by"],
    );

    assert!(success);
    assert!(stdout.contains("already") || stdout.contains("related"));
}

#[test]
fn test_unrelate_no_relation() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue A"]);
    run_atelier(dir.path(), &["issue", "create", "Issue B"]);
    let issue_a = issue_ref(dir.path(), 1);
    let issue_b = issue_ref(dir.path(), 2);

    // Try to unrelate issues that aren't related
    let (success, stdout, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "unlink",
            &issue_a,
            &issue_b,
            "--role",
            "blocked_by",
        ],
    );

    assert!(success);
    assert!(
        stdout.contains("No link") && stdout.contains("blocked_by") && stdout.contains("exists"),
        "Expected idempotent unblock message, got: {}",
        stdout
    );
}

#[test]
fn test_related_no_relations() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Solo issue"]);
    let issue_id = issue_ref(dir.path(), 1);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);

    assert!(success);
    assert!(
        stdout.contains("Blocked by") && stdout.contains("(none)"),
        "Expected empty blocker list message, got: {}",
        stdout
    );
}

#[test]
fn test_related_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", "999"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

// --- label.rs: Error cases ---
#[test]
fn test_label_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "update", "999", "--label", "bug"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_label_already_exists() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    let issue_id = issue_ref(dir.path(), 1);
    run_atelier(
        dir.path(),
        &["issue", "update", &issue_id, "--label", "bug"],
    );

    // Try to add same label again
    let (success, stdout, _) = run_atelier(
        dir.path(),
        &["issue", "update", &issue_id, "--label", "bug"],
    );

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

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", "999", "--remove-label", "bug"],
    );

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_unlabel_nonexistent_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    let issue_id = issue_ref(dir.path(), 1);

    let (success, stdout, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &issue_id,
            "--remove-label",
            "nonexistent",
        ],
    );

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

// --- block.rs: Error cases ---
#[test]
fn test_block_self() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    let issue_id = issue_ref(dir.path(), 1);

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &issue_id,
            &issue_id,
            "--role",
            "blocked_by",
        ],
    );

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
    let issue_id = issue_ref(dir.path(), 1);

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &issue_id, "999", "--role", "blocked_by"],
    );

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

// --- session.rs: Session status deleted issue ---
// --- show.rs: Show with related issues ---
#[test]
fn test_show_with_related_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Main issue"]);
    run_atelier(dir.path(), &["issue", "create", "Related issue"]);
    let main_id = issue_ref(dir.path(), 1);
    let related_id = issue_ref(dir.path(), 2);
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &main_id,
            &related_id,
            "--role",
            "blocked_by",
        ],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", &main_id]);

    assert!(success);
    assert!(stdout.contains("Related") || stdout.contains("#2") || stdout.contains("Main issue"));
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

    let issue_id = issue_ref(dir.path(), 1);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
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
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", "Long desc issue"]);

    assert!(success);
    let issue_id = issue_ref(dir.path(), 1);
    set_issue_description(dir.path(), &issue_id, &long_desc);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
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
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list", "--limit", "100"]);
    assert!(success);
    assert!(stdout.contains("Issue number 99"));

    let issue_50 = issue_id_by_title(dir.path(), "Issue number 50");
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", &issue_50]);
    assert!(success);
    assert!(stdout.contains("Issue number 50"));
}

/// Test a broad epic child set
#[test]
fn test_stress_many_epic_children() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Level 0", "--issue-type", "feature"],
    );
    let parent_id = issue_ref(dir.path(), 1);

    for i in 1..=20 {
        let title = format!("Level {}", i);
        let (success, _, _) = run_atelier(
            dir.path(),
            &["issue", "create", &title, "--parent", &parent_id],
        );
        assert!(success, "Failed to create epic child {}", i);
    }

    let root_id = issue_ref(dir.path(), 1);
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", &root_id]);
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

/// Test SQL injection-shaped issue titles do not corrupt storage.
#[test]
fn test_security_sql_injection_titles_keep_db_intact() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Normal issue"]);
    run_atelier(dir.path(), &["issue", "create", "%' OR 1=1 --"]);

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
    let issue_id = issue_ref(dir.path(), 1);
    let (_, _, _) = run_atelier(dir.path(), &["issue", "note", &issue_id, ""]);

    // Empty label
    let (_, _, _) = run_atelier(dir.path(), &["issue", "update", &issue_id, "--label", ""]);
}

/// Test integer overflow in IDs
#[test]
fn test_edge_large_ids() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test"]);
    let issue_id = issue_ref(dir.path(), 1);

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
        let id = issue_ref(dir.path(), i + 1);
        run_atelier(dir.path(), &["issue", "note", &id, "Rapid comment"]);
        run_atelier(dir.path(), &["issue", "update", &id, "--label", "rapid"]);
    }

    // Verify all operations completed
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("Rapid issue 19"));
}

/// Test that JSON command-result mode is rejected and the human subset works
#[test]
fn test_command_result_json_mode_is_rejected_and_human_subset_works() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier_raw(dir.path(), &["--json", "status"]);
    assert!(!success, "--json should not be accepted");
    assert!(stderr.contains("unexpected argument '--json'"));

    for args in [vec!["issue", "list", "--json"], vec!["check", "--json"]] {
        let (success, _, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(!success, "{args:?} should reject --json");
        assert!(
            stderr.contains("unexpected argument '--json'"),
            "{args:?} stderr did not reject --json: {stderr}"
        );
    }

    for args in [
        vec!["issue", "--help"],
        vec!["issue", "show", "--help"],
        vec!["check", "--help"],
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
            "--body",
            "## Goal\n\nWork item.\n\n## Done when\n\nFactory task remains valid for human command-result checks.\n\n## Context\n\n- `atelier check` passes for the command-result fixture.",
        ],
    );
    assert!(success, "create failed: {stderr}");
    assert!(stdout.contains("Created issue atelier-"));
    assert!(stdout.contains("Type:     feature"));
    assert!(stdout.contains("\nNext\n----\n"));
    let task_id = issue_id_by_title(dir.path(), "Agent Factory task");

    for args in [
        vec!["issue", "show", &task_id, "--json"],
        vec!["issue", "update", &task_id, "--priority", "high", "--json"],
    ] {
        let (success, _, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(!success, "{args:?} should reject --json");
        assert!(
            stderr.contains("unexpected argument '--json'"),
            "{args:?} stderr did not reject --json: {stderr}"
        );
    }

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "update", &task_id, "--claim"]);
    assert!(!success, "claim update should be rejected");
    assert!(stderr.contains("unexpected argument '--claim'"));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", &task_id, "--priority", "high"],
    );
    assert!(success, "update failed: {stderr}");
    assert!(stdout.contains(&format!("Updated issue {task_id}")));
    assert!(stdout.contains("Priority: high"));

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "note", &task_id, "handoff note"]);
    assert!(success, "issue note failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &task_id, "--full"]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains(&task_id));
    assert!(stdout.contains("handoff note"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["work", "ready"]);
    assert!(success, "ready failed: {stderr}");
    assert!(stdout.contains(&task_id));
    assert!(stdout.contains("Ready Work"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Blocker",
            "--body",
            "## Goal\n\nBlocking fixture issue.\n\n## Done when\n\nBlocker issue participates in dependency command-result checks.\n\n## Context\n\n- `atelier check` passes for the command-result fixture.",
        ],
    );
    assert!(success, "blocker create failed: {stderr}");
    let blocker_id = issue_ref(dir.path(), 2);
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &task_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );
    assert!(success, "issue link failed: {stderr}");
    assert!(stdout.contains(&task_id));
    assert!(stdout.contains(&blocker_id));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &task_id, "--full"]);
    assert!(success, "issue show failed: {stderr}");
    assert!(stdout.contains(&blocker_id));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "unlink",
            &task_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );
    assert!(success, "issue unlink failed: {stderr}");
    assert!(stdout.contains(&task_id));
    assert!(stdout.contains(&blocker_id));

    for args in [vec!["issue", "list", "--status", "all"], vec!["check"]] {
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
            "issue",
            "create",
            "Corrective mission",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nMission body\n\n## Success criteria\n\nWrong-kind command output is corrective",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Corrective mission");
    let mission_id = mission_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Corrective issue",
            "--body",
            "## Goal\n\nIssue fixture.\n\n## Done when\n\nWrong-kind command output is corrective.\n\n## Context\n\n- Focused CLI checks pass.",
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
            "--result",
            "pass",
            "--kind",
            "test",
            "wrong-kind fixture evidence",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    let evidence_id = record_id_by_title(dir.path(), "evidence", "wrong-kind fixture evidence");
    let evidence_id = evidence_id.as_str();

    let (success, mission_show, stderr) =
        run_atelier(dir.path(), &["issue", "show", mission_id, "--full"]);
    assert!(
        success,
        "mission ID should resolve through issue show: {stderr}"
    );
    assert!(mission_show.starts_with(&format!("{mission_id} mission ")));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            mission_id,
            "start",
            "--reason",
            "wrong kind",
        ],
    );
    assert!(
        !success,
        "mission ID should reject non-close issue transitions"
    );
    assert!(
        stderr.contains(&format!(
            "Transition 'start' is not available from status 'draft' for issue {mission_id}"
        )),
        "mission transition error should name the draft workflow state: {stderr}"
    );

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", evidence_id]);
    assert!(!success, "evidence ID should not resolve as an issue");
    assert!(
        stderr.contains(&format!(
            "{evidence_id} is an evidence record, not an issue."
        )),
        "wrong-kind evidence lookup should name actual and expected kinds: {stderr}"
    );
    assert!(
        stderr.contains(&format!("atelier evidence show {evidence_id}")),
        "wrong-kind evidence lookup should suggest evidence show: {stderr}"
    );

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", issue_id]);
    assert!(
        success,
        "issue ID should resolve through issue status: {stderr}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "note", issue_id, "canonical issue note"],
    );
    assert!(success, "issue ID should accept issue note: {stderr}");

    let unknown_id = "atelier-zzzz";
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", unknown_id]);
    assert!(!success, "unknown ID should fail");
    assert!(
        stderr.contains("issue atelier-zzzz not found"),
        "unknown ID should keep concise not-found error: {stderr}"
    );
    assert!(
        !stderr.contains("record, not"),
        "unknown ID should not imply a wrong-kind match: {stderr}"
    );
}

#[test]
fn test_mission_unlink_removes_added_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Repair mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Repair mission");
    let mission_id = mission_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Accidental work"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Accidental work");
    let issue_id = issue_id.as_str();

    let (success, add_out, stderr) =
        run_atelier(dir.path(), &["issue", "link", mission_id, issue_id]);
    assert!(success, "mission add-work failed: {stderr}");
    assert!(add_out.contains(&format!("Linked {mission_id} -> {issue_id} (advances)")));

    let (success, linked_out, stderr) =
        run_atelier(dir.path(), &["issue", "show", mission_id, "--full"]);
    assert!(success, "mission status after add-work failed: {stderr}");
    assert!(linked_out.contains("Accidental work"));

    let (success, unlink_out, stderr) =
        run_atelier(dir.path(), &["issue", "unlink", mission_id, issue_id]);
    assert!(success, "mission unlink failed: {stderr}");
    assert!(unlink_out.contains(&format!("Unlinked {mission_id} -> {issue_id} (advances)")));

    let (success, show_out, stderr) =
        run_atelier(dir.path(), &["issue", "show", mission_id, "--full"]);
    assert!(success, "mission status after unlink failed: {stderr}");
    assert!(!show_out.contains("Accidental work"));
    assert!(show_out.contains("Scope : 0 issues\n"), "{show_out}");

    let mission_markdown = read_canonical_record(dir.path(), "issues", mission_id);
    assert!(!mission_markdown.contains(&format!(
        "  - kind: \"issue\"\n    id: \"{issue_id}\"\n    type: \"advances\""
    )));
}

/// A failing capture keeps the failure its command printed last; a passing
/// capture keeps its size, digest and test count instead of the log; and
/// both records pass `git diff --check` (atelier-bphy, atelier-57wj,
/// atelier-auca).
#[test]
fn evidence_capture_keeps_the_failure_tail_and_summarizes_a_pass() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Tail issue", "--issue-type", "task"],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Tail issue");
    let target = format!("issue/{issue_id}");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--summary",
            "failing suite",
            "--target",
            &target,
            "--",
            "sh",
            "-c",
            "i=0; while [ $i -lt 600 ]; do printf 'PASS test_%03d\\n' \"$i\" >&2; i=$((i + 1)); done; printf 'FAIL x\\n' >&2; exit 1",
        ],
    );
    assert!(success, "failing capture record failed: {stderr}");
    let failing_id = record_id_by_title(dir.path(), "evidence", "failing suite");
    let failing = read_canonical_record(dir.path(), "evidence", &failing_id);
    let stderr_section = &failing[failing.find("## Stderr").unwrap()..];
    assert!(
        stderr_section.starts_with("## Stderr\n\nBytes: 8407\nTruncated: yes\nSHA-256: "),
        "{stderr_section}"
    );
    assert!(stderr_section.contains("```text\nPASS test_000\n"));
    assert!(stderr_section.contains(" bytes omitted …\n"));
    assert!(
        stderr_section.ends_with("PASS test_599\nFAIL x\n```\n"),
        "{stderr_section}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--summary",
            "passing suite",
            "--target",
            &target,
            "--",
            "sh",
            "-c",
            "printf 'Compiling x\\n' >&2; printf '     Summary [   1.0s] 3 tests run: 3 passed, 0 skipped\\n' >&2",
        ],
    );
    assert!(success, "passing capture record failed: {stderr}");
    let passing_id = record_id_by_title(dir.path(), "evidence", "passing suite");
    let passing = read_canonical_record(dir.path(), "evidence", &passing_id);
    assert!(!passing.contains("```text"), "{passing}");
    assert!(
        !passing[passing.find("## Stdout").unwrap()..].contains("Compiling x"),
        "{passing}"
    );
    assert!(passing.contains(
        "## Stdout\n\nBytes: 0\nSHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n\n## Stderr\n\nBytes: 68\nSHA-256: "
    ), "{passing}");
    assert!(
        passing.ends_with("\nTest summary: Summary [   1.0s] 3 tests run: 3 passed, 0 skipped\n"),
        "{passing}"
    );
    let (success, shown, stderr) = run_atelier(dir.path(), &["evidence", "show", &passing_id]);
    assert!(success, "evidence show failed: {stderr}");
    assert!(
        shown.contains("\nStderr: Summary [   1.0s] 3 tests run: 3 passed, 0 skipped\n"),
        "{shown}"
    );

    let git = |args: &[&str]| {
        Command::new("git")
            .current_dir(dir.path())
            .args(args)
            .output()
            .expect("failed to run git")
    };
    assert!(git(&["add", "-A", ".atelier/evidence"]).status.success());
    let check = git(&["diff", "--cached", "--check"]);
    assert!(
        check.status.success(),
        "git diff --check: {}",
        String::from_utf8_lossy(&check.stdout)
    );
}

/// Failing output that imitates the record's own Markdown stays in its
/// stream, and a failing whitespace-only stream is not called a summarized
/// pass (reviews atelier-yif8, atelier-7yla).
#[test]
fn evidence_show_keeps_hostile_and_blank_failing_output_in_its_stream() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Hostile issue", "--issue-type", "task"],
    );
    assert!(success, "issue create failed: {stderr}");
    let target = format!("issue/{}", issue_id_by_title(dir.path(), "Hostile issue"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--summary",
            "hostile output",
            "--target",
            &target,
            "--",
            "sh",
            "-c",
            "printf 'before\\n## Stderr\\n\\nBytes: 99\\n```\\nafter\\n'; printf 'real stderr\\n' >&2; exit 1",
        ],
    );
    assert!(success, "hostile capture failed: {stderr}");
    let id = record_id_by_title(dir.path(), "evidence", "hostile output");
    let (success, shown, stderr) = run_atelier(dir.path(), &["evidence", "show", &id]);
    assert!(success, "evidence show failed: {stderr}");
    // The brief view shows stderr, the stream the failure printed to.
    assert!(
        shown.contains("\nStderr (12 bytes):\n  real stderr\nMore with --full"),
        "{shown}"
    );
    let (success, full, stderr) = run_atelier(dir.path(), &["evidence", "show", &id, "--full"]);
    assert!(success, "evidence show --full failed: {stderr}");
    assert!(
        full.contains("```text\nbefore\n## Stderr\n\nBytes: 99\n```\nafter\n````"),
        "{full}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--summary",
            "blank failing output",
            "--target",
            &target,
            "--",
            "sh",
            "-c",
            "printf '  \\n\\n'; exit 1",
        ],
    );
    assert!(success, "blank capture failed: {stderr}");
    let id = record_id_by_title(dir.path(), "evidence", "blank failing output");
    let (success, shown, stderr) = run_atelier(dir.path(), &["evidence", "show", &id]);
    assert!(success, "evidence show failed: {stderr}");
    assert!(
        shown.contains("\nStdout: 4 bytes, whitespace only\n"),
        "{shown}"
    );
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
        &["issue", "create", "Capture epic", "--issue-type", "feature"],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Capture epic");
    let epic_id = epic_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Capture mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Capture mission");
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
    let issue_evidence_id = record_id_by_title(dir.path(), "evidence", "issue command proof");
    assert!(
        issue_capture.starts_with(&format!(
            "{issue_evidence_id} ✓ pass validation for issue/{issue_id} @"
        )),
        "{issue_capture}"
    );
    assert!(issue_capture.contains(": sh -c "), "{issue_capture}");
    assert!(
        issue_capture.ends_with(" (exit 0)\nevidence record: passed\n"),
        "{issue_capture}"
    );
    assert!(!issue_capture.contains("\npass stdout"));
    let (success, shown, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &issue_evidence_id]);
    assert!(success, "evidence query failed: {stderr}");
    assert!(shown.contains("issue command proof"));
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
    assert!(issue_evidence_markdown.contains("## Stdout\n\nBytes: 12\nSHA-256: "));
    assert!(issue_evidence_markdown.contains("## Stderr\n\nBytes: 12\nSHA-256: "));
    assert!(!issue_evidence_markdown.contains("\npass stdout\n"));

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
    assert!(
        record_capture.contains(&format!(" ✓ pass validation for issue/{issue_id} @")),
        "{record_capture}"
    );
    assert_eq!(record_capture.lines().count(), 2, "{record_capture}");

    let positional_summary = "unified positional manual proof";
    let (success, positional_record_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--result",
            "pass",
            "--kind",
            "validation",
            "--target",
            &format!("issue/{issue_id}"),
            positional_summary,
        ],
    );
    assert!(success, "positional manual record failed: {stderr}");
    assert!(
        positional_record_out.contains(&format!(" ✓ pass validation for issue/{issue_id} @")),
        "{positional_record_out}"
    );
    assert!(positional_record_out
        .ends_with(": unified positional manual proof\nevidence record: passed\n"));
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
            "--result",
            "pass",
            "--kind",
            "validation",
            "--summary",
            "flag summary",
            "positional summary",
        ],
    );
    assert!(!success, "conflicting summaries should fail");
    assert!(
        stderr.contains("the argument '--summary <SUMMARY>' cannot be used with '[SUMMARY_TEXT]'"),
        "the parser should reject conflicting summaries: {stderr}"
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
            &format!("issue/{epic_id}"),
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
    assert!(epic_show.starts_with(&format!(
        "{epic_evidence_id} evidence recorded ⚠ fail validation for issue/{epic_id} @"
    )));
    assert!(epic_show.contains(" (exit 7)\n"), "{epic_show}");
    assert!(
        epic_show.contains("\nStderr (15 bytes):\n  failing stderr\n"),
        "{epic_show}"
    );
    let (success, epic_full, stderr) = run_atelier(
        dir.path(),
        &["evidence", "show", &epic_evidence_id, "--full"],
    );
    assert!(success, "epic evidence show --full failed: {stderr}");
    assert!(epic_full.contains("Exit Status: 7"));
    assert!(epic_full.contains(&format!("Target:      issue/{epic_id}\n")));
    assert!(epic_full.contains("failing stdout"));
    assert!(epic_full.contains("failing stderr"));

    let manual_epic_summary =
        "manual epic contract audit line-by-line classification maps epic outcome lines to proof";
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--result",
            "pass",
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
            "issue",
            epic_id,
        ],
    );
    assert!(success, "manual epic evidence attach failed: {stderr}");
    let (success, manual_epic_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &manual_epic_evidence_id]);
    assert!(success, "manual epic evidence show failed: {stderr}");
    assert!(
        manual_epic_show.contains(&format!(" ✓ pass validation for issue/{epic_id} @")),
        "{manual_epic_show}"
    );

    let manual_issue_summary = "unified manual issue proof";
    let (success, manual_record_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--result",
            "pass",
            "--kind",
            "validation",
            "--summary",
            manual_issue_summary,
            "--target",
            &format!("issue/{issue_id}"),
        ],
    );
    assert!(success, "unified manual record failed: {stderr}");
    assert!(manual_record_out.contains(&format!(" ✓ pass validation for issue/{issue_id} @")));
    assert!(manual_record_out.contains(": unified manual issue proof\n"));
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
            &format!("issue/{mission_id}"),
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
    assert!(mission_show.starts_with(&format!(
        "{mission_evidence_id} evidence recorded ⚠ fail validation for issue/{mission_id} @"
    )));
    assert!(mission_show.contains(" (exit 2)\n"));
    let (success, mission_full, stderr) = run_atelier(
        dir.path(),
        &["evidence", "show", &mission_evidence_id, "--full"],
    );
    assert!(success, "mission evidence show --full failed: {stderr}");
    assert!(mission_full.contains("blocked-line-000"));
    assert!(mission_full.contains(" bytes omitted …"));
    assert!(mission_full.contains("blocked-line-349"));
    assert!(mission_full.contains("Truncated: yes"));

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "evidence list failed: {stderr}");
    let row = |id: &str| {
        evidence_list
            .lines()
            .find(|line| line.starts_with(id))
            .unwrap_or_else(|| panic!("no row for {id}:\n{evidence_list}"))
    };
    assert!(row(&issue_evidence_id).starts_with(&format!(
        "{issue_evidence_id} ✓ pass validation for issue/{issue_id} @"
    )));
    assert!(row(&issue_evidence_id).ends_with(" · sh -c 'printf '\"'\"'pass stdout\\n'\"'\"'; printf '\"'\"'pass stderr\\n'\"'\"' >&2'"), "{}", row(&issue_evidence_id));
    assert!(row(&epic_evidence_id).starts_with(&format!(
        "{epic_evidence_id} ⚠ fail validation for issue/{epic_id} @"
    )));
    assert!(row(&manual_epic_evidence_id).contains(&format!(" for issue/{epic_id} @")));
    assert!(row(&mission_evidence_id).starts_with(&format!(
        "{mission_evidence_id} ⚠ fail validation for issue/{mission_id} @"
    )));
    assert!(!evidence_list.contains("(validates)"), "{evidence_list}");
    assert!(!evidence_list.contains("(none)"), "{evidence_list}");
    assert!(evidence_list.contains("Show the newest proof's detail: atelier evidence show "));
    assert!(!evidence_list.contains("<evidence-id>"), "{evidence_list}");
}

#[test]
fn test_evidence_capture_allows_reentrant_mutation_children() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Capture target", "--issue-type", "task"],
    );
    assert!(success, "target create failed: {stderr}");
    let target_id = issue_id_by_title(dir.path(), "Capture target");
    let executable = env!("CARGO_BIN_EXE_atelier");

    let mutation = run_atelier_with_deadlock_timeout(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--summary",
            "Nested mutation proof",
            "--target",
            &format!("issue/{target_id}"),
            "--",
            executable,
            "issue",
            "create",
            "Nested child issue",
            "--issue-type",
            "task",
        ],
    );
    assert!(
        mutation.status.success(),
        "mutation capture failed: {}",
        String::from_utf8_lossy(&mutation.stderr)
    );
    assert!(
        issue_id_by_title(dir.path(), "Nested child issue").starts_with("atelier-"),
        "nested mutation did not commit"
    );
    assert!(
        record_id_by_title(dir.path(), "evidence", "Nested mutation proof").starts_with("atelier-")
    );

    let (healthy, _, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(
        healthy,
        "nested capture left invalid canonical state: {stderr}"
    );
}

#[test]
fn test_evidence_capture_preflight_rejects_invalid_association_without_running_child() {
    let initialized = tempdir().unwrap();
    init_atelier(initialized.path());
    let marker = initialized.path().join("invalid-target-child-ran");
    let invalid_target = run_atelier_with_deadlock_timeout(
        initialized.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--target",
            "issue/atelier-missing",
            "--",
            "sh",
            "-c",
            &format!("sleep 1; touch {}", marker.display()),
        ],
    );
    assert!(!invalid_target.status.success());
    assert!(
        String::from_utf8_lossy(&invalid_target.stderr).contains("command was not executed"),
        "{}",
        String::from_utf8_lossy(&invalid_target.stderr)
    );
    assert!(!marker.exists(), "invalid target still executed child");

    let uninitialized = tempdir().unwrap();
    let marker = uninitialized.path().join("non-repo-child-ran");
    let non_repo = run_atelier_with_deadlock_timeout(
        uninitialized.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--",
            "sh",
            "-c",
            &format!("sleep 1; touch {}", marker.display()),
        ],
    );
    assert!(!non_repo.status.success());
    assert!(
        String::from_utf8_lossy(&non_repo.stderr).contains("command was not executed"),
        "{}",
        String::from_utf8_lossy(&non_repo.stderr)
    );
    assert!(
        !marker.exists(),
        "non-repository capture still executed child"
    );
}

#[test]
fn test_evidence_capture_postflight_refuses_association_swap_and_deleted_target() {
    let swapped = tempdir().unwrap();
    init_atelier(swapped.path());
    let swap = run_atelier_with_deadlock_timeout(
        swapped.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--summary",
            "Must not land after association swap",
            "--",
            "sh",
            "-c",
            "mv .atelier .atelier-before-capture; cp -a .atelier-before-capture .atelier",
        ],
    );
    assert!(!swap.status.success());
    assert!(
        String::from_utf8_lossy(&swap.stderr).contains("evidence_capture_association_changed"),
        "{}",
        String::from_utf8_lossy(&swap.stderr)
    );
    assert!(!evidence_tree_contains(
        &swapped.path().join(".atelier"),
        "Must not land after association swap"
    ));
    assert!(!evidence_tree_contains(
        &swapped.path().join(".atelier-before-capture"),
        "Must not land after association swap"
    ));

    let deleted = tempdir().unwrap();
    init_atelier(deleted.path());
    let (success, _, stderr) = run_atelier(
        deleted.path(),
        &[
            "issue",
            "create",
            "Delete capture target",
            "--issue-type",
            "task",
        ],
    );
    assert!(success, "target create failed: {stderr}");
    let target_id = issue_id_by_title(deleted.path(), "Delete capture target");
    let deletion = run_atelier_with_deadlock_timeout(
        deleted.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--summary",
            "Must not land after target deletion",
            "--target",
            &format!("issue/{target_id}"),
            "--",
            "rm",
            &format!(".atelier/issues/{target_id}.md"),
        ],
    );
    assert!(!deletion.status.success());
    assert!(
        String::from_utf8_lossy(&deletion.stderr).contains("postflight target validation failed"),
        "{}",
        String::from_utf8_lossy(&deletion.stderr)
    );
    assert!(!evidence_tree_contains(
        &deleted.path().join(".atelier"),
        "Must not land after target deletion"
    ));

    let replaced = tempdir().unwrap();
    init_atelier(replaced.path());
    let (success, _, stderr) = run_atelier(
        replaced.path(),
        &[
            "issue",
            "create",
            "Replace capture target",
            "--issue-type",
            "task",
        ],
    );
    assert!(success, "target create failed: {stderr}");
    let target_id = issue_id_by_title(replaced.path(), "Replace capture target");
    let target_path = format!(".atelier/issues/{target_id}.md");
    let replacement = run_atelier_with_deadlock_timeout(
        replaced.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--summary",
            "Must not land on replacement identity",
            "--target",
            &format!("issue/{target_id}"),
            "--",
            "sh",
            "-c",
            &format!(
                "sed -e 's/^created_at: .*/created_at: \"2000-01-01T00:00:00Z\"/' {target_path} > {target_path}.replacement && mv {target_path}.replacement {target_path}"
            ),
        ],
    );
    assert!(!replacement.status.success());
    assert!(
        String::from_utf8_lossy(&replacement.stderr).contains("evidence_capture_target_changed"),
        "{}",
        String::from_utf8_lossy(&replacement.stderr)
    );
    assert!(!evidence_tree_contains(
        &replaced.path().join(".atelier"),
        "Must not land on replacement identity"
    ));
}

#[test]
fn test_evidence_capture_postflight_accepts_same_target_material_mutation() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Mutable capture target",
            "--issue-type",
            "task",
        ],
    );
    assert!(success, "target create failed: {stderr}");
    let target_id = issue_id_by_title(dir.path(), "Mutable capture target");
    let capture = run_atelier_with_deadlock_timeout(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--summary",
            "Proof after same-target mutation",
            "--target",
            &format!("issue/{target_id}"),
            "--",
            env!("CARGO_BIN_EXE_atelier"),
            "issue",
            "update",
            &target_id,
            "--title",
            "Mutated capture target",
        ],
    );
    assert!(
        capture.status.success(),
        "same-target capture failed: {}",
        String::from_utf8_lossy(&capture.stderr)
    );
    let evidence_id =
        record_id_by_title(dir.path(), "evidence", "Proof after same-target mutation");
    let (success, shown, stderr) = run_atelier(dir.path(), &["evidence", "show", &evidence_id]);
    assert!(success, "evidence show failed: {stderr}");
    assert!(
        shown.contains(&format!(" for issue/{target_id} @")),
        "{shown}"
    );
    assert_eq!(
        issue_id_by_title(dir.path(), "Mutated capture target"),
        target_id
    );

    let exclusive_capture = run_atelier_with_deadlock_timeout(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--summary",
            "Proof after same-repository exclusive writer",
            "--target",
            &format!("issue/{target_id}"),
            "--",
            env!("CARGO_BIN_EXE_atelier"),
            "issue",
            "note",
            &target_id,
            "Note written under an exclusive transaction",
        ],
    );
    assert!(
        exclusive_capture.status.success(),
        "same-repository exclusive capture failed: {}",
        String::from_utf8_lossy(&exclusive_capture.stderr)
    );
    assert!(record_id_by_title(
        dir.path(),
        "evidence",
        "Proof after same-repository exclusive writer"
    )
    .starts_with("atelier-"));
    let (healthy, _, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(healthy, "same-target capture left invalid state: {stderr}");
}

#[test]
fn test_evidence_list_elides_command_transcripts() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--summary",
            "long command proof",
            "--",
            "sh",
            "-c",
            "printf '%s%s\\n' proof transcript",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "evidence list failed: {stderr}");
    // A row shows the whole command, never its output.
    assert!(
        evidence_list.contains(" · sh -c 'printf '\"'\"'%s%s\\n'\"'\"' proof transcript'\n"),
        "{evidence_list}"
    );
    assert!(!evidence_list.contains("prooftranscript"));
    assert!(evidence_list.contains("Show the newest proof's detail: atelier evidence show "));
    assert!(!evidence_list.contains("<evidence-id>"), "{evidence_list}");
}

#[test]
fn test_evidence_list_bounds_no_summary_command_capture() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let long_command = format!("printf proof # {}", "x".repeat(5_200));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--",
            "sh",
            "-c",
            &long_command,
        ],
    );
    assert!(success, "no-summary command capture failed: {stderr}");

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "evidence list failed: {stderr}");
    assert!(
        evidence_list.contains(" · sh -c 'printf proof # xxx"),
        "{evidence_list}"
    );
    assert!(evidence_list.contains("x…\n"), "{evidence_list}");
    assert!(!evidence_list.contains("(command-backed proof)"));
    assert!(!evidence_list.contains(&"x".repeat(256)));
    assert!(
        evidence_list.len() < 1_024,
        "one no-summary record must not create a list firehose ({} bytes):\n{evidence_list}",
        evidence_list.len()
    );
}

#[test]
fn test_evidence_list_bounds_default_output() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let mut evidence_ids = Vec::new();
    for index in 0..21 {
        let summary = format!("bounded evidence proof {index:02}");
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &[
                "evidence",
                "record",
                "--result",
                "pass",
                "--kind",
                "validation",
                &summary,
            ],
        );
        assert!(success, "evidence record {index} failed: {stderr}");
        let evidence_id = record_id_by_title(dir.path(), "evidence", &summary);
        evidence_ids.push((index, evidence_id));
    }
    evidence_ids.sort_by(|(_, left), (_, right)| left.cmp(right));

    for (rank, (_, evidence_id)) in evidence_ids.iter().enumerate() {
        let timestamp = format!("2024-01-{:02}T00:00:00+00:00", rank + 1);
        edit_canonical_record(dir.path(), "evidence", evidence_id, |markdown| {
            let markdown = replace_front_matter_scalar(&markdown, "created_at", &timestamp);
            replace_front_matter_scalar(&markdown, "updated_at", &timestamp)
        });
    }
    let oldest_by_timestamp = format!("bounded evidence proof {:02}", evidence_ids[0].0);
    let newest_by_timestamp = format!(
        "bounded evidence proof {:02}",
        evidence_ids.last().unwrap().0
    );

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "evidence list failed: {stderr}");
    let rows = |list: &str| {
        list.lines()
            .filter(|line| line.starts_with("atelier-"))
            .count()
    };
    assert_eq!(rows(&evidence_list), 20, "{evidence_list}");
    assert!(!evidence_list.contains(" total"), "{evidence_list}");
    assert!(
        evidence_list.contains(&newest_by_timestamp),
        "newest timestamp should be visible despite sorting last by ID:\n{evidence_list}"
    );
    assert!(
        !evidence_list.contains(&oldest_by_timestamp),
        "oldest timestamp should be hidden by default limit:\n{evidence_list}"
    );
    assert!(evidence_list
        .contains("  ... 1 more evidence record omitted; see `atelier evidence list --all`\n"));
    assert!(evidence_list.contains("List failing proof: atelier evidence list --result fail"));
    assert!(evidence_list.contains("List matching IDs: atelier --quiet evidence list\n"));

    for (args, count) in [
        (vec!["evidence", "list", "--limit", "3"], 3),
        (vec!["evidence", "list", "--all"], 21),
    ] {
        let (success, list, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "{args:?} failed: {stderr}");
        assert_eq!(rows(&list), count, "{args:?}:\n{list}");
        assert_eq!(list.contains(" omitted; see "), count < 21, "{list}");
    }
    let (success, quiet_limited, stderr) =
        run_atelier(dir.path(), &["--quiet", "evidence", "list", "--limit", "2"]);
    assert!(success, "quiet limited list failed: {stderr}");
    assert_eq!(quiet_limited.lines().count(), 2);

    let (success, quiet_list, stderr) = run_atelier(dir.path(), &["--quiet", "evidence", "list"]);
    assert!(success, "quiet evidence list failed: {stderr}");
    assert_eq!(quiet_list.lines().count(), 21);
    assert!(quiet_list.lines().any(|line| line == evidence_ids[0].1));
    assert!(quiet_list
        .lines()
        .any(|line| line == evidence_ids.last().unwrap().1));
}

#[test]
fn test_evidence_quiet_output_is_a_stable_id_composition_path() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Proof target"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Proof target");

    let (success, record_out, stderr) = run_atelier(
        dir.path(),
        &[
            "--quiet",
            "evidence",
            "record",
            "--result",
            "pass",
            "--kind",
            "validation",
            "--target",
            &format!("issue/{issue_id}"),
            "quiet proof",
        ],
    );
    assert!(success, "quiet evidence record failed: {stderr}");
    let evidence_id = record_id_by_title(dir.path(), "evidence", "quiet proof");
    assert_eq!(record_out, format!("{evidence_id}\n"));

    let (success, show_out, stderr) =
        run_atelier(dir.path(), &["--quiet", "evidence", "show", &evidence_id]);
    assert!(success, "quiet evidence show failed: {stderr}");
    assert_eq!(show_out, format!("{evidence_id} pass\n"));

    let (success, list_out, stderr) = run_atelier(dir.path(), &["--quiet", "evidence", "list"]);
    assert!(success, "quiet evidence list failed: {stderr}");
    assert_eq!(list_out.trim(), evidence_id);
    assert!(!list_out.contains("Evidence\n"));

    let (success, attach_out, stderr) = run_atelier(
        dir.path(),
        &[
            "--quiet",
            "evidence",
            "attach",
            &evidence_id,
            "issue",
            &issue_id,
        ],
    );
    assert!(success, "quiet idempotent attach failed: {stderr}");
    assert_eq!(attach_out.trim(), evidence_id);
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
            "--result",
            "pass",
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
            "--result",
            "pass",
            "--kind",
            "validation",
            "--target",
            &format!("issue/{target_id}"),
            "accepted target proof",
        ],
    );
    assert!(success, "accepted evidence record failed: {stderr}");
    assert!(record_out.contains(&format!(" ✓ pass validation for issue/{target_id} @")));
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
    assert!(
        show_out.contains(&format!(" for issue/{target_id}, issue/{reuse_id} @")),
        "{show_out}"
    );
}

fn assert_corrective_evidence_role_error(stderr: &str) {
    assert!(
        stderr.contains("Invalid role 'validation'. Valid values: validates"),
        "error should name the invalid role and the accepted vocabulary: {stderr}"
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
    assert!(stdout.contains(&format!(" ⚠ fail validation for issue/{issue_id} @")));
    assert!(stdout.contains(" (exit 3)\n  rejected\n"), "{stdout}");
    assert!(stdout.ends_with(&format!(
        "evidence record: recorded fail — {issue_id} cannot close or merge until a later pass\n"
    )));

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "evidence list failed: {stderr}");
    assert!(
        evidence_list
            .lines()
            .any(|line| line.contains(" ⚠ fail validation ")
                && line.ends_with(" · sh -c 'printf '\"'\"'rejected\\n'\"'\"'; exit 3'")),
        "{evidence_list}"
    );
}

#[test]
fn test_issue_closeout_rejects_evidence_attached_to_another_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_evidence_gated_workflow(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Target proof"]);
    assert!(success, "target issue create failed: {stderr}");
    let target_id = issue_id_by_title(dir.path(), "Target proof");
    let target_id = target_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Donor proof"]);
    assert!(success, "donor issue create failed: {stderr}");
    let donor_id = issue_id_by_title(dir.path(), "Donor proof");
    let donor_id = donor_id.as_str();

    let evidence_id = attach_issue_pass_evidence(dir.path(), donor_id);

    move_issue_to_review(dir.path(), target_id);
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
        stderr.contains("expected at least 1 fresh passing evidence record")
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

    let issue_body = "## Goal\n\nEvidence gate body.\n\n## Done when\n\nThe issue can close after workflow proof is attached.\n\n## Context\n\n- A focused command transcript proves the workflow change.";
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Evidence gate proof",
            "--body",
            issue_body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Evidence gate proof");

    move_issue_to_review(dir.path(), &issue_id);
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
            "transition",
            &issue_id,
            "close",
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
fn test_issue_closeout_requires_passing_evidence_records() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_evidence_gated_workflow(dir.path());

    let issue_body = "## Goal\n\nValidation blocker body.\n\n## Done when\n\nThe issue does not close without passing evidence.\n\n## Context\n\n- A passing transcript proves closeout readiness.";
    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Blocked validation proof",
            "--body",
            issue_body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Blocked validation proof");

    move_issue_to_review(dir.path(), &issue_id);
    attach_non_pass_evidence(
        dir.path(),
        "issue",
        &issue_id,
        "blocked",
        "blocked validation transcript recorded",
    );

    let (success, transitions, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--verbose"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(transitions.contains("close"));
    assert!(transitions.contains("expected at least 1 fresh passing evidence record"));

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
    assert!(stderr.contains("expected at least 1 fresh passing evidence record"));
}

#[test]
fn test_workflow_init_is_removed_and_root_init_owns_starter_policy() {
    let dir = tempdir().unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["init", "--slug", "atelier"]);
    assert!(success, "root init failed: {stderr}");
    assert!(stdout.contains(".atelier/workflow.yaml"));
    assert!(stdout.contains("atelier check"));

    let policy_path = dir.path().join(".atelier").join("workflow.yaml");
    let policy = std::fs::read_to_string(&policy_path).unwrap();
    assert!(policy.contains("schema_version: 3"));
    assert!(policy.contains("branch_policy:"));
    assert!(policy.contains("  todo:\n    category: todo"));
    assert!(policy.contains("    initial_status: todo"));
    assert!(policy.contains("  objective:"));
    assert!(policy.contains("  work:"));
    assert!(policy.contains("applies_to:"));
    assert!(!policy.contains("actions:"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "init"]);
    assert!(!success, "workflow init should be removed");
    assert!(stdout.is_empty(), "{stdout}");
    assert!(
        stderr.contains("unrecognized subcommand 'workflow'"),
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
    let (success, _, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(
        !success,
        "loading records should reject legacy status before workflow check"
    );
    assert!(stderr.contains("workflow_issue_status_invalid"), "{stderr}");
    assert!(stderr.contains("open"), "{stderr}");
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

    let (success, _, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(
        !success,
        "loading records should reject status mismatch before workflow check"
    );
    assert!(
        stderr.contains("workflow_issue_status_invalid"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains(&issue_id), "stderr: {stderr}");
    assert!(stderr.contains("qa_hold"), "stderr: {stderr}");
    assert!(
        stderr.contains(
            "allowed statuses: backlog, blocked, done, in_progress, review, superseded, todo"
        ),
        "stderr: {stderr}"
    );
}

fn create_task(dir: &Path, title: &str) -> String {
    let (success, _, stderr) =
        run_atelier(dir, &["issue", "create", title, "--issue-type", "task"]);
    assert!(success, "issue create failed: {stderr}");
    issue_id_by_title(dir, title)
}

fn short_head(dir: &Path) -> String {
    git_rev_parse(dir, "HEAD").trim()[..8].to_string()
}

/// `evidence record` confirms in one line and ends with its verdict line; a
/// recorded fail exits 0, shows the last lines of its output, and the
/// verdict names the item it blocks (atelier-3ekh). An unknown kind warns
/// and is still recorded (atelier-b05y).
#[test]
fn evidence_record_confirms_in_one_line_and_ends_in_its_verdict() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let issue_id = create_task(dir.path(), "Confirmed issue");
    let short_id = issue_id.trim_start_matches("atelier-").to_string();
    fs::write(dir.path().join("proof-sample.txt"), "hello goodbye\n").unwrap();

    let (success, out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &short_id,
            "--kind",
            "test",
            "--",
            "grep",
            "-q",
            "goodbye",
            "proof-sample.txt",
        ],
    );
    assert!(success, "pass capture failed: {stderr}");
    assert_eq!(stderr, "", "a known kind gives no warning");
    let pass_id = record_id_by_title(dir.path(), "evidence", "grep -q goodbye proof-sample.txt");
    let pin = short_head(dir.path());
    assert_eq!(
        out,
        format!(
            "{pass_id} ✓ pass test for issue/{issue_id} @{pin}: grep -q goodbye proof-sample.txt (exit 0)\nevidence record: passed\n"
        )
    );

    let (success, out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{issue_id}"),
            "--kind",
            "test",
            "--summary",
            "failing check",
            "--",
            "sh",
            "-c",
            "for i in 1 2 3 4 5 6 7; do echo line$i >&2; done; exit 1",
        ],
    );
    assert!(success, "a recorded fail exits 0: {stderr}");
    let fail_id = record_id_by_title(dir.path(), "evidence", "failing check");
    let pin = short_head(dir.path());
    assert_eq!(
        out,
        format!(
            "{fail_id} ⚠ fail test for issue/{issue_id} @{pin}: sh -c 'for i in 1 2 3 4 5 6 7; do echo line$i >&2; done; exit 1' (exit 1); newer than pass {pass_id}\n  line3\n  line4\n  line5\n  line6\n  line7\nevidence record: recorded fail — {issue_id} cannot close or merge until a later pass\n"
        )
    );

    let (success, out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &issue_id,
            "--kind",
            "validation",
            "--result",
            "pass",
            "Observed the sample",
        ],
    );
    assert!(success, "manual record failed: {stderr}");
    let manual_id = record_id_by_title(dir.path(), "evidence", "Observed the sample");
    let pin = short_head(dir.path());
    assert_eq!(
        out,
        format!(
            "{manual_id} ✓ pass validation for issue/{issue_id} @{pin}: Observed the sample\nevidence record: passed\n"
        )
    );

    // The verdict stays the last line when the command also commits.
    let (success, out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &issue_id,
            "--kind",
            "weird",
            "--result",
            "fail",
            "--commit-records",
            "Saw it fail",
        ],
    );
    assert!(success, "manual fail failed: {stderr}");
    assert_eq!(
        stderr,
        "⚠ evidence kind 'weird' is new: no receipt, gate or default uses it, so no gate or filter for another kind matches it. Known kinds: test, validation\n"
    );
    let lines = out.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 3, "{out}");
    assert!(lines[0].contains(" ⚠ fail weird for issue/"), "{out}");
    assert!(
        lines[0].ends_with(&format!(": Saw it fail; newer than pass {manual_id}")),
        "{out}"
    );
    assert!(lines[1].starts_with("Committed "), "{out}");
    assert_eq!(
        lines[2],
        format!(
            "evidence record: recorded fail — {issue_id} cannot close or merge until a later pass"
        )
    );

    let (success, out, _) = run_atelier(
        dir.path(),
        &[
            "--quiet", "evidence", "record", "--target", &issue_id, "--kind", "test", "--", "false",
        ],
    );
    assert!(success);
    assert_eq!(
        out,
        format!("{}\n", record_id_by_title(dir.path(), "evidence", "false"))
    );
}

/// `evidence list <item>` lists that item's receipts newest first, each
/// marked as the evidence gates judge it, and ends with the item's proof
/// verdict (atelier-u0ig).
#[test]
fn evidence_list_by_item_marks_each_receipt_as_the_gates_judge_it() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let issue_id = create_task(dir.path(), "Listed issue");
    let other_id = create_task(dir.path(), "Other issue");
    let short_id = issue_id.trim_start_matches("atelier-").to_string();
    let record = |target: &str, summary: &str, command: &str| {
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &[
                "evidence",
                "record",
                "--target",
                target,
                "--kind",
                "test",
                "--summary",
                summary,
                "--",
                "sh",
                "-c",
                command,
            ],
        );
        assert!(success, "record {summary} failed: {stderr}");
        record_id_by_title(dir.path(), "evidence", summary)
    };
    let list = |args: &[&str]| {
        let mut full = vec!["evidence", "list"];
        full.extend_from_slice(args);
        let (success, out, stderr) = run_atelier(dir.path(), &full);
        assert!(success, "{full:?} failed: {stderr}");
        out
    };
    let rows = |out: &str| -> Vec<String> {
        out.lines()
            .filter(|line| line.starts_with("atelier-"))
            .map(str::to_string)
            .collect()
    };

    let first_pass = record(&issue_id, "first pass", "true");
    let fail = record(&issue_id, "the fail", "exit 1");
    let other = record(&other_id, "other proof", "true");

    let out = list(&[&short_id]);
    assert!(
        out.starts_with(&format!("Evidence for {issue_id}\n")),
        "{out}"
    );
    assert!(!out.contains(&other), "{out}");
    let shown = rows(&out);
    assert_eq!(shown.len(), 2, "{out}");
    assert!(
        shown[0].starts_with(&format!("{fail} ⚠ fail test @")),
        "{out}"
    );
    assert!(
        shown[0].ends_with(" · ⚠ blocks completion · sh -c 'exit 1'"),
        "{out}"
    );
    assert!(
        shown[1].ends_with(&format!(
            " · ⚠ superseded by newer fail {fail} · sh -c true"
        )),
        "{out}"
    );
    assert!(
        out.contains(&format!("\nProof: ⚠ newest is fail {fail}\n")),
        "{out}"
    );

    let second_pass = record(&issue_id, "second pass", "true");
    let out = list(&[&issue_id]);
    let shown = rows(&out);
    assert!(shown[0].starts_with(&format!("{second_pass} ✓ pass test @")));
    assert!(shown[0].contains(" · ✓ fresh · "), "{out}");
    assert!(
        shown[1].contains(&format!(" · superseded by newer pass {second_pass} · ")),
        "{out}"
    );
    assert!(shown[2].starts_with(&first_pass), "{out}");
    assert!(
        out.contains(&format!("\nProof: ✓ fresh pass {second_pass}\n")),
        "{out}"
    );

    fs::write(dir.path().join("code.txt"), "changed code\n").unwrap();
    commit_all(dir.path(), "Change code");
    let code_commit = short_head(dir.path());
    let out = list(&[&issue_id]);
    let shown = rows(&out);
    assert!(
        shown[0].contains(&format!(" · ⚠ stale (code changed in {code_commit}) · ")),
        "{out}"
    );
    assert!(shown[1].contains(" · ⚠ blocks completion · "), "{out}");
    assert!(
        out.contains(&format!("\nProof: ⚠ newest is fail {fail}\n")),
        "{out}"
    );

    let out = list(&[&issue_id, "--limit", "1"]);
    assert_eq!(rows(&out).len(), 1, "{out}");
    assert!(out.contains(&format!(
        "\n  ... 2 more evidence records omitted; see `atelier evidence list {issue_id} --all`\n"
    )));
    assert!(out.contains(&format!(
        "List failing proof: atelier evidence list {issue_id} --result fail\n"
    )));

    let out = list(&[&issue_id, "--result", "fail"]);
    assert_eq!(rows(&out).len(), 1, "{out}");
    assert!(out.contains(&format!(
        "List matching IDs: atelier --quiet evidence list {issue_id} --result fail\n"
    )));
    assert!(!out.contains("List failing proof"), "{out}");

    let (success, quiet, _) = run_atelier(dir.path(), &["--quiet", "evidence", "list", &issue_id]);
    assert!(success);
    assert_eq!(quiet, format!("{second_pass}\n{fail}\n{first_pass}\n"));

    // Unfiltered rows are marked against their own item and name it.
    let out = list(&[]);
    let row = rows(&out)
        .into_iter()
        .find(|row| row.starts_with(&other))
        .unwrap();
    assert!(
        row.starts_with(&format!("{other} ✓ pass test for issue/{other_id} @")),
        "{row}"
    );
    assert!(row.contains(" · ⚠ stale (code changed in "), "{row}");
}

/// `evidence show` is brief: one title line with the receipt's standing,
/// the claim and command, and one output excerpt; `--full` prints every
/// field and the stored record (atelier-o8ni).
#[test]
fn evidence_show_is_brief_and_full_prints_the_stored_record() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let issue_id = create_task(dir.path(), "Shown issue");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &issue_id,
            "--kind",
            "test",
            "--summary",
            "the shown fail",
            "--",
            "sh",
            "-c",
            "i=1; while [ $i -le 30 ]; do echo line$i >&2; i=$((i + 1)); done; exit 1",
        ],
    );
    assert!(success, "record failed: {stderr}");
    let id = record_id_by_title(dir.path(), "evidence", "the shown fail");
    let pin = short_head(dir.path());
    let short = id.trim_start_matches("atelier-");

    let (success, brief, stderr) = run_atelier(dir.path(), &["evidence", "show", short]);
    assert!(success, "evidence show failed: {stderr}");
    let lines = brief.lines().collect::<Vec<_>>();
    assert!(
        lines[0].starts_with(&format!(
            "{id} evidence recorded ⚠ fail test for issue/{issue_id} @{pin} · "
        )),
        "{brief}"
    );
    assert!(lines[0].ends_with(" · ⚠ blocks completion"), "{brief}");
    assert_eq!(lines[1], "Summary:  the shown fail");
    assert_eq!(
        lines[2],
        "Command:  sh -c 'i=1; while [ $i -le 30 ]; do echo line$i >&2; i=$((i + 1)); done; exit 1' (exit 1)"
    );
    assert_eq!(lines[3], "Stderr (last 20 lines of 201 bytes):");
    assert_eq!(lines[4], "  line11");
    assert_eq!(lines[23], "  line30");
    assert_eq!(
        lines[24],
        "More with --full: the stored record and every field"
    );
    assert_eq!(lines.len(), 25, "{brief}");
    for absent in [
        "(none)",
        "Created:",
        "Updated:",
        "Status:",
        "Producer:",
        "line10\n",
    ] {
        assert!(!brief.contains(absent), "brief shows {absent}:\n{brief}");
    }

    let (success, full, stderr) = run_atelier(dir.path(), &["evidence", "show", &id, "--full"]);
    assert!(success, "evidence show --full failed: {stderr}");
    assert!(full.starts_with(&format!("{id} evidence recorded the shown fail\n====")));
    for field in [
        "Result:      fail\n",
        "Freshness:   ⚠ blocks completion\n",
        &format!("Commit:      {pin}\n"),
        "Exit Status: 1\n",
        &format!("Target:      issue/{issue_id}\n"),
        "Producer:    (none)\n",
        "Created:     ",
        "Updated:     ",
        "\nRecord\n------\n## Summary\n\nthe shown fail\n",
    ] {
        assert!(full.contains(field), "--full lacks {field:?}:\n{full}");
    }
    // The stored transcript is printed once.
    assert_eq!(full.matches("line30").count(), 1, "{full}");
}

/// `evidence attach` takes the target forms `evidence record --target`
/// takes, and keeps its two-argument form (atelier-b05y).
#[test]
fn evidence_attach_takes_the_target_forms_record_takes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let first = create_task(dir.path(), "First target");
    let second = create_task(dir.path(), "Second target");
    let third = create_task(dir.path(), "Third target");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "Reusable proof",
        ],
    );
    assert!(success, "record failed: {stderr}");
    let id = record_id_by_title(dir.path(), "evidence", "Reusable proof");

    let second_short = second.trim_start_matches("atelier-").to_string();
    for (args, target) in [
        (vec![format!("issue/{first}")], &first),
        (vec![second_short], &second),
        (vec!["issue".to_string(), third.clone()], &third),
    ] {
        let mut full = vec!["evidence".to_string(), "attach".to_string(), id.clone()];
        full.extend(args);
        let full = full.iter().map(String::as_str).collect::<Vec<_>>();
        let (success, out, stderr) = run_atelier(dir.path(), &full);
        assert!(success, "{full:?} failed: {stderr}");
        assert_eq!(
            out,
            format!("Attached evidence {id} to issue {target} (validates)\n")
        );
    }
    let (success, _, stderr) = run_atelier(dir.path(), &["evidence", "attach", &id, "bogus/x"]);
    assert!(!success);
    assert_eq!(
        stderr,
        "Error: Invalid target kind 'bogus'. Valid values: issue, evidence\n"
    );

    let (success, out, _) = run_atelier(dir.path(), &["--quiet", "evidence", "list", &third]);
    assert!(success);
    assert_eq!(out, format!("{id}\n"));
}

/// `evidence list`'s verdict agrees with the evidence gates the close
/// transition runs on the same receipts: a newer fail blocks both, a fresh
/// pass after it clears both, and once a code commit makes that pass stale
/// the fail blocks both again (atelier-u0ig).
#[test]
fn evidence_list_verdict_agrees_with_the_close_gate() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let issue_id = create_task(dir.path(), "Gated issue");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "start failed: {stderr}");
    let record = |summary: &str, command: &str| {
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &[
                "evidence",
                "record",
                "--target",
                &issue_id,
                "--kind",
                "test",
                "--summary",
                summary,
                "--",
                "sh",
                "-c",
                command,
            ],
        );
        assert!(success, "record {summary} failed: {stderr}");
        record_id_by_title(dir.path(), "evidence", summary)
    };
    let verdict = || {
        let (success, out, stderr) = run_atelier(dir.path(), &["evidence", "list", &issue_id]);
        assert!(success, "list failed: {stderr}");
        out.lines()
            .find(|line| line.starts_with("Proof: "))
            .unwrap()
            .to_string()
    };
    let close_refusal = || {
        let (success, out, stderr) =
            run_atelier_raw(dir.path(), &["issue", "transition", &issue_id, "close"]);
        assert!(!success, "close has no Close Reason, so it refuses: {out}");
        stderr
    };

    let pass = record("first pass", "true");
    assert_eq!(verdict(), format!("Proof: ✓ fresh pass {pass}"));
    let refusal = close_refusal();
    assert!(!refusal.contains("newer failing evidence"), "{refusal}");

    let fail = record("the fail", "exit 1");
    assert_eq!(verdict(), format!("Proof: ⚠ newest is fail {fail}"));
    assert!(
        close_refusal().contains(&format!("newer failing evidence {fail}")),
        "{}",
        close_refusal()
    );

    let second = record("second pass", "true");
    assert_eq!(verdict(), format!("Proof: ✓ fresh pass {second}"));
    let refusal = close_refusal();
    assert!(!refusal.contains("newer failing evidence"), "{refusal}");

    fs::write(dir.path().join("code.txt"), "changed code\n").unwrap();
    commit_all(dir.path(), "Change code");
    assert_eq!(verdict(), format!("Proof: ⚠ newest is fail {fail}"));
    // The later pass no longer proves the claim, so the fail blocks again.
    let refusal = close_refusal();
    assert!(
        refusal.contains(&format!("newer failing evidence {fail}")),
        "{refusal}"
    );
}
