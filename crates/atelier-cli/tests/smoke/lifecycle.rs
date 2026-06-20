use super::harness::SmokeHarness;

// ===========================================================================
// Timer roundtrip
// ===========================================================================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_timer_roundtrip() {
    let h = SmokeHarness::new();

    h.run_ok(&["issue", "create", "Timer roundtrip issue"]);

    // Start the timer (top-level `start` command).
    let start = h.run_ok(&["issue", "transition", "1", "start"]);
    assert!(
        start.stdout_contains("Started")
            || start.stdout_contains("timer")
            || start.stdout_contains("Timer"),
        "start should confirm timer start.\nstdout: {}",
        start.stdout,
    );

    // Show while running — `timer` is the status command.
    let show_running = h.run_ok(&["timer"]);
    assert!(
        show_running.stdout_contains("running")
            || show_running.stdout_contains("active")
            || show_running.stdout_contains("Active")
            || show_running.stdout_contains("Timer")
            || show_running.stdout_contains("tracking")
            || show_running.stdout_contains("#1"),
        "timer while running should indicate active state.\nstdout: {}",
        show_running.stdout,
    );

    // Stop the timer (top-level `stop` command).
    let stop = h.run_ok(&["stop"]);
    assert!(
        stop.stdout_contains("Stopped")
            || stop.stdout_contains("stopped")
            || stop.stdout_contains("timer")
            || stop.stdout_contains("Timer"),
        "stop should confirm timer stop.\nstdout: {}",
        stop.stdout,
    );

    // Show after stopping.
    let show_stopped = h.run(&["timer"]);
    let combined = format!("{}{}", show_stopped.stdout, show_stopped.stderr);
    assert!(
        combined.contains("No timer")
            || combined.contains("no timer")
            || combined.contains("No active")
            || combined.contains("Total")
            || show_stopped.success,
        "timer after stop should report stopped state.\nstdout: {}\nstderr: {}",
        show_stopped.stdout,
        show_stopped.stderr,
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_timer_start_already_running() {
    let h = SmokeHarness::new();
    h.run_ok(&["issue", "create", "Double-start issue"]);

    h.run_ok(&["issue", "transition", "1", "start"]);

    let result = h.run(&["issue", "transition", "1", "start"]);
    let combined = format!("{}{}", result.stdout, result.stderr);
    assert!(
        result.success
            || combined.contains("already")
            || combined.contains("running")
            || combined.contains("active"),
        "Second timer start should handle gracefully.\nstdout: {}\nstderr: {}",
        result.stdout,
        result.stderr,
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_timer_stop_not_running() {
    let h = SmokeHarness::new();

    let result = h.run(&["stop"]);
    let combined = format!("{}{}", result.stdout, result.stderr);
    assert!(
        result.success
            || combined.contains("No active")
            || combined.contains("no active")
            || combined.contains("not running")
            || combined.contains("No timer"),
        "stop with no running timer should handle gracefully.\nstdout: {}\nstderr: {}",
        result.stdout,
        result.stderr,
    );
}

// ===========================================================================
// Objective status with hierarchy
// ===========================================================================

#[test]
fn test_issue_status_with_subissues() {
    let h = SmokeHarness::new();

    h.run_ok(&["issue", "create", "Parent lifecycle issue"]);
    let parent_id = h.issue_id(1);
    h.run_ok(&["issue", "subissue", "1", "Child lifecycle issue"]);

    let status = h.run_ok(&["issue", "status", &parent_id]);
    assert!(status.stdout.contains("Parent lifecycle issue"));
    assert!(status.stdout.contains("Child lifecycle issue"));
}

#[test]
fn test_issue_status_deep_nesting() {
    let h = SmokeHarness::new();

    h.run_ok(&["issue", "create", "Root issue"]);
    let root_id = h.issue_id(1);
    h.run_ok(&["issue", "subissue", "1", "Child issue"]);
    h.run_ok(&["issue", "subissue", "2", "Grandchild issue"]);

    let status = h.run_ok(&["issue", "status", &root_id]);
    assert!(status.stdout.contains("Root issue"));
    assert!(status.stdout.contains("Child issue"));
    assert!(status.stdout.contains("Grandchild issue"));
}

#[test]
fn test_issue_status_reports_sibling_children() {
    let h = SmokeHarness::new();

    h.run_ok(&[
        "issue",
        "create",
        "Filterable parent",
        "--issue-type",
        "epic",
    ]);
    let parent_id = h.issue_id(1);
    h.run_ok(&["issue", "subissue", "1", "Todo child"]);
    h.run_ok(&[
        "issue",
        "create",
        "Sibling child",
        "--parent",
        &parent_id,
        "--issue-type",
        "spike",
    ]);

    let status = h.run_ok(&["issue", "status", &parent_id]);
    assert!(status.stdout.contains("Todo child"));
    assert!(status.stdout_contains("Sibling child"));
    assert!(status.stdout.contains("Ready Work"));
}

// ===========================================================================
// Dependency chains
// ===========================================================================

#[test]
fn test_dependency_chain_and_ready() {
    let h = SmokeHarness::new();

    h.run_ok(&["issue", "create", "Issue A"]);
    h.run_ok(&["issue", "create", "Issue B"]);
    h.run_ok(&["issue", "create", "Issue C"]);

    // A blocked by B, B blocked by C
    h.run_ok(&["issue", "block", "1", "2"]);
    h.run_ok(&["issue", "block", "2", "3"]);

    // Only C should be ready
    let ready = h.run_ok(&["issue", "list", "--ready"]);
    assert!(ready.stdout.contains("Issue C"));
    assert!(!ready.stdout.contains("Issue A"));
    assert!(!ready.stdout.contains("Issue B"));

    // Close C, then B should become ready
    h.close_issue_with_evidence("3");
    let ready2 = h.run_ok(&["issue", "list", "--ready"]);
    assert!(ready2.stdout.contains("Issue B"));
    assert!(!ready2.stdout.contains("Issue A"));

    // Close B, then A should become ready
    h.close_issue_with_evidence("2");
    let ready3 = h.run_ok(&["issue", "list", "--ready"]);
    assert!(ready3.stdout.contains("Issue A"));
}

#[test]
fn test_circular_dependency_prevented() {
    let h = SmokeHarness::new();

    h.run_ok(&["issue", "create", "Issue 1"]);
    h.run_ok(&["issue", "create", "Issue 2"]);
    h.run_ok(&["issue", "create", "Issue 3"]);

    h.run_ok(&["issue", "block", "1", "2"]);
    h.run_ok(&["issue", "block", "2", "3"]);

    // Attempting to create cycle 3 -> 1 should fail
    let result = h.run(&["issue", "block", "3", "1"]);
    assert!(
        !result.success || result.stderr.contains("circular"),
        "Circular dependency should be rejected.\nstdout: {}\nstderr: {}",
        result.stdout,
        result.stderr,
    );
}

// ===========================================================================
// Milestone lifecycle
// ===========================================================================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_full_lifecycle() {
    let h = SmokeHarness::new();

    // Create milestone
    let create = h.run_ok(&["milestone", "create", "v1.0", "-d", "First release"]);
    assert!(create.stdout.contains("Created milestone"));

    // Add issues
    h.run_ok(&["issue", "create", "Feature 1"]);
    h.run_ok(&["issue", "create", "Feature 2"]);
    h.run_ok(&["milestone", "add", "1", "1"]);
    h.run_ok(&["milestone", "add", "1", "2"]);

    // Show milestone
    let show = h.run_ok(&["milestone", "show", "1"]);
    assert!(show.stdout.contains("v1.0"));
    assert!(show.stdout.contains("Feature 1"));
    assert!(show.stdout.contains("Feature 2"));

    // Close issues and milestone
    h.close_issue_with_evidence("1");
    h.close_issue_with_evidence("2");
    h.run_ok(&["milestone", "close", "1"]);

    let show_closed = h.run_ok(&["milestone", "show", "1"]);
    assert!(show_closed.stdout.contains("closed"));
}
