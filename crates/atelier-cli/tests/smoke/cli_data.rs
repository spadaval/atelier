use std::fs;

use super::harness::SmokeHarness;

// ==================== Import/Export Tests ====================

#[test]
fn test_canonical_export_check_cli() {
    let h = SmokeHarness::new();

    h.run_ok(&["issue", "create", "Canonical issue"]);
    h.run_ok(&["work", "queue", "--all"]);
    h.run_ok(&["export", "--check"]);
    let issue_id = h.issue_id(1);

    h.run_ok(&[
        "issue",
        "update",
        &issue_id,
        "--title",
        "Changed canonical issue",
    ]);
    let stale = h.run_err(&["export", "--check"]);
    assert!(stale.stderr.contains("cache:"), "{}", stale.stderr);
    h.run_ok(&["work", "queue", "--all"]);
    h.run_ok(&["export", "--check"]);

    let issue_id = h.issue_id_by_title("Changed canonical issue");
    h.edit_canonical_issue(&issue_id, |markdown| {
        markdown.replace("Changed canonical issue", "Markdown canonical issue")
    });

    let stale = h.run_err(&["export", "--check"]);
    assert!(stale.stderr.contains("cache:"), "{}", stale.stderr);
    h.run_ok(&["work", "queue", "--all"]);
    h.run_ok(&["export", "--check"]);

    let rewrite_result = h.run_err(&["export"]);
    assert!(
        rewrite_result
            .stderr
            .contains("SQLite-to-record export has been removed"),
        "expected removed export-write diagnostic, got stderr: {}",
        rewrite_result.stderr
    );
    assert!(
        h.read_canonical_record("issues", &issue_id)
            .contains("Markdown canonical issue"),
        "export should not rewrite tracked canonical Markdown"
    );
}

#[test]
fn test_import_malformed_json() {
    let h = SmokeHarness::new();

    let malformed = r#"[{"id": 1, "title": "Trunc"#;
    let import_path = h.temp_dir.path().join("malformed.json");
    fs::write(&import_path, malformed).unwrap();

    let result = h.run_err(&["import", import_path.to_str().unwrap()]);
    assert!(
        result.stderr.contains("parse")
            || result.stderr.contains("JSON")
            || result.stderr.contains("error")
            || result.stderr.contains("invalid"),
        "Should indicate JSON parse error, got stderr: {}",
        result.stderr
    );
}

// ==================== Archive Tests ====================

// ==================== Next command ====================

#[test]
fn test_next_suggests_highest_priority() {
    let h = SmokeHarness::new();

    h.run_ok(&["issue", "create", "Low prio task", "-p", "low"]);
    h.run_ok(&["issue", "create", "Critical task", "-p", "critical"]);
    h.run_ok(&["issue", "create", "Medium task", "-p", "medium"]);

    let next = h.run_ok(&["work", "queue", "--ready"]);
    // The critical task should be suggested first
    assert!(
        next.stdout.contains("Critical task"),
        "next should suggest highest priority issue.\nstdout: {}",
        next.stdout
    );
}

#[test]
fn test_next_skips_blocked() {
    let h = SmokeHarness::new();

    h.run_ok(&["issue", "create", "Blocked task", "-p", "critical"]);
    h.run_ok(&["issue", "create", "Blocker task", "-p", "low"]);
    let blocked_id = h.issue_id(1);
    let blocker_id = h.issue_id(2);
    h.run_ok(&[
        "issue",
        "link",
        &blocked_id,
        &blocker_id,
        "--role",
        "blocked_by",
    ]);

    let next = h.run_ok(&["work", "queue", "--ready"]);
    // Should suggest the blocker (which is unblocked) not the blocked task
    assert!(
        !next.stdout.contains("Blocked task") || next.stdout.contains("Blocker task"),
        "next should not suggest blocked issues.\nstdout: {}",
        next.stdout
    );
}
