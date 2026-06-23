use super::harness::SmokeHarness;

// ===========================================================================
// Timer roundtrip
// ===========================================================================

// ===========================================================================
// Objective status with hierarchy
// ===========================================================================

#[test]
fn test_issue_status_with_subissues() {
    let h = SmokeHarness::new();

    h.run_ok(&[
        "issue",
        "create",
        "Parent lifecycle issue",
        "--issue-type",
        "epic",
    ]);
    let parent_id = h.issue_id(1);
    h.run_ok(&[
        "issue",
        "create",
        "Child lifecycle issue",
        "--parent",
        &parent_id,
    ]);

    let status = h.run_ok(&["issue", "status", &parent_id]);
    assert!(status.stdout.contains("Parent lifecycle issue"));
    assert!(status.stdout.contains("Child lifecycle issue"));
}

#[test]
fn test_issue_create_rejects_deep_task_nesting() {
    let h = SmokeHarness::new();

    h.run_ok(&["issue", "create", "Root issue", "--issue-type", "epic"]);
    let root_id = h.issue_id(1);
    h.run_ok(&["issue", "create", "Child issue", "--parent", &root_id]);
    let child_id = h.issue_id(2);
    let result = h.run(&["issue", "create", "Grandchild issue", "--parent", &child_id]);

    assert!(!result.success);
    assert!(result.stderr.contains("only epics can own child work"));
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
    h.run_ok(&["issue", "create", "Todo child", "--parent", &parent_id]);
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
    let issue_a = h.issue_id(1);
    let issue_b = h.issue_id(2);
    let issue_c = h.issue_id(3);

    // A blocked by B, B blocked by C
    h.run_ok(&["issue", "block", &issue_a, &issue_b]);
    h.run_ok(&["issue", "block", &issue_b, &issue_c]);

    // Only C should be ready
    let ready = h.run_ok(&["issue", "list", "--ready"]);
    assert!(ready.stdout.contains("Issue C"));
    assert!(!ready.stdout.contains("Issue A"));
    assert!(!ready.stdout.contains("Issue B"));

    // Close C, then B should become ready
    h.close_issue_with_evidence(&issue_c);
    let ready2 = h.run_ok(&["issue", "list", "--ready"]);
    assert!(ready2.stdout.contains("Issue B"));
    assert!(!ready2.stdout.contains("Issue A"));

    // Close B, then A should become ready
    h.close_issue_with_evidence(&issue_b);
    let ready3 = h.run_ok(&["issue", "list", "--ready"]);
    assert!(ready3.stdout.contains("Issue A"));
}

#[test]
fn test_circular_dependency_prevented() {
    let h = SmokeHarness::new();

    h.run_ok(&["issue", "create", "Issue 1"]);
    h.run_ok(&["issue", "create", "Issue 2"]);
    h.run_ok(&["issue", "create", "Issue 3"]);
    let issue_1 = h.issue_id(1);
    let issue_2 = h.issue_id(2);
    let issue_3 = h.issue_id(3);

    h.run_ok(&["issue", "block", &issue_1, &issue_2]);
    h.run_ok(&["issue", "block", &issue_2, &issue_3]);

    // Attempting to create cycle 3 -> 1 should fail
    let result = h.run(&["issue", "block", &issue_3, &issue_1]);
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
