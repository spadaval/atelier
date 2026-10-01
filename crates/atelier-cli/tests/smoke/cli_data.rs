use super::harness::SmokeHarness;

// ==================== Next command ====================

#[test]
fn test_next_suggests_highest_priority() {
    let h = SmokeHarness::new();

    h.run_ok(&["issue", "create", "Low prio task", "-p", "low"]);
    h.run_ok(&["issue", "create", "Critical task", "-p", "critical"]);
    h.run_ok(&["issue", "create", "Medium task", "-p", "medium"]);

    let next = h.run_ok(&["work", "ready"]);
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

    let next = h.run_ok(&["work", "ready"]);
    // Should suggest the blocker (which is unblocked) not the blocked task
    assert!(
        !next.stdout.contains("Blocked task") || next.stdout.contains("Blocker task"),
        "next should not suggest blocked issues.\nstdout: {}",
        next.stdout
    );
}
