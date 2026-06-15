use super::*;

// ==================== Search Tests ====================

#[test]
fn test_search_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Authentication bug"]);
    run_atelier(dir.path(), &["issue", "create", "Dark mode feature"]);
    run_atelier(dir.path(), &["issue", "create", "Auth improvements"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "search", "auth"]);

    assert!(success);
    assert!(stdout.contains("Search Results: auth"));
    assert!(stdout.contains("Standalone"));
    assert!(stdout.contains("2 total"));
    assert!(stdout.contains("Authentication") || stdout.contains("Auth"));
    assert!(!stdout.contains("Dark mode"));
}

// ==================== Error Handling Tests ====================

#[test]
fn test_command_without_init() {
    let dir = tempdir().unwrap();
    // Don't init

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list"]);

    // The CLI walks up parent directories to find .atelier, so from a temp dir
    // inside the project tree, it may find the project's own .atelier.
    // In that case it succeeds with the project DB. If truly isolated, it fails.
    if !success {
        assert!(
            stderr.contains("Not an Atelier repository") || stderr.contains("atelier init"),
            "Error should mention missing repo, got stderr: {}",
            stderr
        );
    } else {
        // Found a parent .atelier - list should work normally
        assert!(
            stdout.contains("No issues") || stdout.contains("#"),
            "Should show valid list output, got: {}",
            stdout
        );
    }
}

#[test]
fn test_invalid_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Issue", "-p", "invalid"]);

    assert!(!success, "Creating issue with invalid priority should fail");
    assert!(
        stderr.contains("Invalid") || stderr.contains("priority"),
        "Error should mention invalid priority, got stderr: {}",
        stderr
    );
}

// ==================== Security Tests ====================

#[test]
fn test_sql_injection_in_title_cli() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let malicious = "'; DROP TABLE issues; --";
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", malicious]);

    assert!(success);

    // Verify database is intact
    let (success2, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success2);
    assert!(stdout.contains(malicious));
}

#[test]
fn test_special_characters_in_fields() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let special = "Test <>&\"'\\n\\t issue";
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", special]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("Test"));
}

#[test]
fn test_unicode_in_cli() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let unicode = "测试问题 🐛 émoji";
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", unicode]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("测试") || show_out.contains("🐛"));
}

// ==================== Archive Tests ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_archive_closed_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue to archive"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["archive", "add", "1"]);

    assert!(success);
    assert!(stdout.contains("Archived") || stdout.contains("archived"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_archive_open_issue_fails() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Open issue"]);
    let (success, stdout, stderr) = run_atelier(dir.path(), &["archive", "add", "1"]);

    // Should fail or warn - can't archive open issues
    assert!(
        !success
            || stderr.contains("closed")
            || stderr.contains("open")
            || stdout.contains("not closed")
            || stdout.contains("Cannot")
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_archive_list() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue to archive"]);
    run_atelier(dir.path(), &["issue", "create", "Open issue"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);
    run_atelier(dir.path(), &["archive", "add", "1"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["archive", "list"]);

    assert!(success);
    assert!(stdout.contains("Issue to archive") || stdout.contains("#1"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_unarchive_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue to archive"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);
    run_atelier(dir.path(), &["archive", "add", "1"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["archive", "remove", "1"]);

    assert!(success);
    assert!(
        stdout.contains("Unarchived")
            || stdout.contains("restored")
            || stdout.contains("removed")
            || stdout.contains("Restored")
    );

    // Should now be in closed list, not archived
    let (_, closed_list, _) = run_atelier(dir.path(), &["issue", "list", "-s", "closed"]);
    assert!(closed_list.contains("Issue to archive"));
}

// ==================== Milestone Tests ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_create() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(
        dir.path(),
        &["milestone", "create", "v1.0", "-d", "First release"],
    );

    assert!(success);
    assert!(stdout.contains("v1.0") || stdout.contains("#1") || stdout.contains("Created"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_list() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    run_atelier(dir.path(), &["milestone", "create", "v2.0"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["milestone", "list"]);

    assert!(success);
    assert!(stdout.contains("v1.0"));
    assert!(stdout.contains("v2.0"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_show() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["milestone", "create", "v1.0", "-d", "First release"],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["milestone", "show", "1"]);

    assert!(success);
    assert!(stdout.contains("v1.0"));
    assert!(stdout.contains("First release") || stdout.contains("description"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_add_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    run_atelier(dir.path(), &["issue", "create", "Feature 1"]);
    run_atelier(dir.path(), &["issue", "create", "Feature 2"]);

    let (success, _, _) = run_atelier(dir.path(), &["milestone", "add", "1", "1", "2"]);

    assert!(success);

    // Check milestone shows the issues
    let (_, show_out, _) = run_atelier(dir.path(), &["milestone", "show", "1"]);
    assert!(show_out.contains("Feature 1") || show_out.contains("#1"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_close() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["milestone", "close", "1"]);

    assert!(success);
    assert!(stdout.contains("Closed") || stdout.contains("closed"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_delete() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    let (success, _, _) = run_atelier(dir.path(), &["milestone", "delete", "1"]);

    assert!(success);

    // Should no longer appear in list
    let (_, list_out, _) = run_atelier(dir.path(), &["milestone", "list", "-s", "all"]);
    assert!(!list_out.contains("v1.0") || list_out.contains("No milestones"));
}

// ==================== Timer Tests ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_timer_start() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue to time"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["start", "1"]);

    assert!(success);
    assert!(stdout.contains("Started") || stdout.contains("timer") || stdout.contains("#1"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_timer_stop() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue to time"]);
    run_atelier(dir.path(), &["start", "1"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["stop"]);

    assert!(success);
    assert!(stdout.contains("Stopped") || stdout.contains("stopped") || stdout.contains("timer"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_timer_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue to time"]);
    run_atelier(dir.path(), &["start", "1"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["timer"]);

    assert!(success);
    assert!(
        stdout.contains("#1") || stdout.contains("Issue to time") || stdout.contains("running")
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_timer_status_no_timer() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["timer"]);

    assert!(success);
    assert!(
        stdout.contains("No timer running"),
        "Expected 'No timer running' message, got: {}",
        stdout
    );
}

// ==================== Relate Tests ====================

#[test]
fn test_relate_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue 1"]);
    run_atelier(dir.path(), &["issue", "create", "Issue 2"]);

    let (success, _, _) = run_atelier(dir.path(), &["issue", "relate", "1", "2"]);

    assert!(success);
}

#[test]
fn test_related_list() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue 1"]);
    run_atelier(dir.path(), &["issue", "create", "Issue 2"]);
    run_atelier(dir.path(), &["issue", "relate", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "related", "1"]);

    assert!(success);
    assert!(stdout.contains("Issue 2") || stdout.contains("#2"));
}

#[test]
fn test_unrelate_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue 1"]);
    run_atelier(dir.path(), &["issue", "create", "Issue 2"]);
    run_atelier(dir.path(), &["issue", "relate", "1", "2"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "unrelate", "1", "2"]);

    assert!(success);

    let (_, related_out, _) = run_atelier(dir.path(), &["issue", "related", "1"]);
    assert!(!related_out.contains("Issue 2") || related_out.contains("No related"));
}

#[test]
fn test_issue_help_hides_non_lifecycle_assumption_commands() {
    let dir = tempdir().unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "--help"]);

    assert!(success, "issue help failed: {}", stderr);
    assert!(!stdout.contains("impact"));
    assert!(!stdout.contains("cascade"));
    assert!(!stdout.contains("falsify"));
}

#[test]
fn test_cascade_commands_are_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Source issue"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Child issue"]);

    let (top_success, _, top_stderr) = run_atelier(dir.path(), &["cascade", "1"]);
    assert!(!top_success);
    assert!(top_stderr.contains("unrecognized subcommand"));

    let (issue_success, _, issue_stderr) = run_atelier(dir.path(), &["issue", "cascade", "1"]);
    assert!(!issue_success);
    assert!(issue_stderr.contains("unrecognized subcommand"));
}

#[test]
fn test_falsify_commands_are_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Source issue"]);

    let (top_success, _, top_stderr) = run_atelier(dir.path(), &["falsify", "1"]);
    assert!(!top_success);
    assert!(top_stderr.contains("unrecognized subcommand"));

    let (issue_success, _, issue_stderr) = run_atelier(dir.path(), &["issue", "falsify", "1"]);
    assert!(!issue_success);
    assert!(issue_stderr.contains("unrecognized subcommand"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_issue_impact_reports_downstream_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Source issue"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Child issue"]);
    run_atelier(dir.path(), &["issue", "create", "Derived issue"]);
    run_atelier(dir.path(), &["issue", "create", "Caused issue"]);
    run_atelier(
        dir.path(),
        &["issue", "relate", "1", "3", "--type", "derived"],
    );
    run_atelier(
        dir.path(),
        &["issue", "relate", "1", "4", "--type", "caused-by"],
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "impact", "1"]);

    assert!(success, "issue impact failed: {}", stderr);
    assert!(stdout.contains(&issue_ref(dir.path(), 2)));
    assert!(stdout.contains("Child issue"));
    assert!(stdout.contains(&issue_ref(dir.path(), 3)));
    assert!(stdout.contains("Derived issue"));
    assert!(stdout.contains(&issue_ref(dir.path(), 4)));
    assert!(stdout.contains("Caused issue"));
    assert!(stdout.contains("downstream impact"));
}

// ==================== Tree Tests ====================

#[test]
fn test_tree_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent issue"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Child issue"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "tree"]);

    assert!(success);
    assert!(stdout.contains("Parent issue"));
    assert!(stdout.contains("Child issue"));
}

#[test]
fn test_tree_with_status_filter() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Open parent"]);
    run_atelier(dir.path(), &["issue", "create", "Closed parent"]);
    close_issue_with_evidence(dir.path(), "2", None);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "tree", "-s", "todo"]);

    assert!(success);
    assert!(stdout.contains("Open parent"));
    // Closed issues should not appear
    assert!(!stdout.contains("Closed parent"));
}

#[test]
fn test_tree_compact_collapses_deep_hierarchy() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Level 0"]);
    for level in 1..=6 {
        let parent = issue_ref(dir.path(), level);
        let title = format!("Level {level}");
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "subissue", &parent, &title]);
        assert!(success, "subissue failed: {stderr}");
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "tree", "--compact"]);

    assert!(success, "compact tree failed: {stderr}");
    assert!(stdout.contains("Compact Issue Hierarchy"));
    assert!(stdout.contains("Level 3"));
    assert!(stdout.contains("descendants collapsed"));
    assert!(!stdout.contains("Level 4"));
}

#[test]
fn test_tree_compact_omits_wide_sibling_sets() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Wide parent"]);
    let parent = issue_ref(dir.path(), 1);
    for index in 1..=8 {
        let title = format!("Child {index}");
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "subissue", &parent, &title]);
        assert!(success, "subissue failed: {stderr}");
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "tree", "--compact"]);

    assert!(success, "compact tree failed: {stderr}");
    assert!(stdout.contains("Wide parent children=8"));
    assert!(stdout.contains("... 2 more children omitted"));
}

#[test]
fn test_tree_compact_omits_wide_root_sets() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for index in 1..=8 {
        let title = format!("Root {index}");
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", &title]);
        assert!(success, "root create failed: {stderr}");
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "tree", "--compact"]);

    assert!(success, "compact tree failed: {stderr}");
    assert_eq!(
        stdout.lines().filter(|line| line.contains("Root ")).count(),
        6
    );
    assert!(stdout.contains("... 2 more root issues omitted"));
}

#[test]
fn test_tree_compact_summarizes_mixed_open_closed_subtrees() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Mixed parent"]);
    let parent = issue_ref(dir.path(), 1);
    run_atelier(dir.path(), &["issue", "subissue", &parent, "Open child"]);
    run_atelier(dir.path(), &["issue", "subissue", &parent, "Closed child"]);
    let closed = issue_id_by_title(dir.path(), "Closed child");
    close_issue_with_evidence(dir.path(), &closed, None);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "tree", "--compact"]);
    assert!(success, "compact tree failed: {stderr}");
    assert!(stdout.contains("Mixed parent children=2"));
    assert!(stdout.contains("[done"));

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "tree", "--compact", "-s", "todo"]);
    assert!(success, "compact open tree failed: {stderr}");
    assert!(stdout.contains("Mixed parent children=1"));
    assert!(!stdout.contains("Closed child"));
}

// ==================== Next Tests ====================

#[test]
fn test_next_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Low priority", "-p", "low"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "High priority", "-p", "high"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Critical priority", "-p", "critical"],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    // Should suggest critical or high priority issue
    assert!(
        stdout.contains("Critical priority")
            || stdout.contains("High priority")
            || stdout.contains("#3")
            || stdout.contains("#2")
    );
}

#[test]
fn test_next_no_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    assert!(
        stdout.contains("No issues found."),
        "Expected empty ready-list message, got: {}",
        stdout
    );
}

// ==================== Export/Import Tests ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_export_json() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue 1"]);
    run_atelier(dir.path(), &["issue", "create", "Issue 2"]);

    let export_path = dir.path().join("export.json");
    let (success, _, _) = run_atelier(
        dir.path(),
        &["export", "-o", export_path.to_str().unwrap(), "-f", "json"],
    );

    assert!(success);
    assert!(export_path.exists());

    // Verify JSON content
    let content = std::fs::read_to_string(&export_path).unwrap();
    assert!(content.contains("Issue 1"));
    assert!(content.contains("Issue 2"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_export_markdown() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Issue 1", "-d", "Description 1"],
    );

    let export_path = dir.path().join("export.md");
    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "export",
            "-o",
            export_path.to_str().unwrap(),
            "-f",
            "markdown",
        ],
    );

    assert!(success);
    assert!(export_path.exists());

    let content = std::fs::read_to_string(&export_path).unwrap();
    assert!(
        content.contains("Issue 1"),
        "Exported markdown should contain issue title, got: {}",
        &content[..content.len().min(200)]
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_import_json() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create issues and export
    run_atelier(dir.path(), &["issue", "create", "Exported Issue"]);
    let export_path = dir.path().join("export.json");
    run_atelier(
        dir.path(),
        &["export", "-o", export_path.to_str().unwrap(), "-f", "json"],
    );

    // Create a fresh atelier instance and import
    let dir2 = tempdir().unwrap();
    init_atelier(dir2.path());

    let (success, _, _) = run_atelier(dir2.path(), &["import", export_path.to_str().unwrap()]);

    assert!(success);

    // Verify imported issue exists
    let (_, list_out, _) = run_atelier(dir2.path(), &["issue", "list", "-s", "all"]);
    assert!(list_out.contains("Exported Issue") || list_out.contains("#1"));
}

// ==================== Tested Command Tests ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_tested_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "tested"]);

    assert!(success);
    assert!(
        stdout.contains("Marked tests as run"),
        "Expected 'Marked tests as run' in output, got: {}",
        stdout
    );
}

// ==================== Additional Create Edge Cases ====================

#[test]
fn test_create_with_template() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", "Bug report", "-t", "bug"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("Bug report"));
}

#[test]
fn test_issue_create_unifies_type_and_template_defaults() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, help_out, stderr) = run_atelier(dir.path(), &["issue", "create", "--help"]);
    assert!(success, "issue create help failed: {stderr}");
    assert!(help_out.contains("Work type/body preset"));
    assert!(help_out.contains("Explicit work type"));

    let (success, parent_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Parent task"]);
    assert!(success, "parent create failed: {stderr}");
    assert!(parent_out.contains("Created issue atelier-"));
    let parent_id = issue_id_by_title(dir.path(), "Parent task");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Parented bug",
            "--template",
            "bug",
            "--parent",
            &parent_id,
        ],
    );
    assert!(success, "parented bug create failed: {stderr}");
    let bug_id = issue_id_by_title(dir.path(), "Parented bug");
    let bug_record = std::fs::read_to_string(canonical_issue_path(dir.path(), &bug_id)).unwrap();
    assert!(bug_record.contains("issue_type: \"bug\""));
    assert!(
        bug_record.contains("priority: \"P1\"")
            || bug_record.contains("priority: high")
            || bug_record.contains("priority: \"high\"")
    );
    assert!(bug_record.contains("- \"bug\""));
    assert!(bug_record.contains("Steps to reproduce"));
    let parent_record =
        std::fs::read_to_string(canonical_issue_path(dir.path(), &parent_id)).unwrap();
    assert!(parent_record.contains(&format!("id: \"{bug_id}\"")));

    let (success, validation_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Validation work",
            "--issue-type",
            "validation",
            "--description",
            "Validation body",
        ],
    );
    assert!(success, "validation create failed: {stderr}");
    assert!(validation_out.contains("Type:     validation"));
    let validation_id = issue_id_by_title(dir.path(), "Validation work");
    let validation_record =
        std::fs::read_to_string(canonical_issue_path(dir.path(), &validation_id)).unwrap();
    assert!(validation_record.contains("issue_type: \"validation\""));
    assert!(validation_record.contains("Validation body"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Conflicting work type",
            "--template",
            "bug",
            "--issue-type",
            "feature",
        ],
    );
    assert!(!success, "conflicting create options should fail");
    assert!(
        stderr.contains("Conflicting work type options")
            && stderr.contains("--issue-type feature")
            && stderr.contains("--template bug"),
        "conflict error should be actionable: {stderr}"
    );
}

#[test]
fn test_create_all_priorities() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for priority in &["low", "medium", "high", "critical"] {
        let (success, _, _) = run_atelier(
            dir.path(),
            &[
                "issue",
                "create",
                &format!("{} issue", priority),
                "-p",
                priority,
            ],
        );
        assert!(success, "Failed to create {} priority issue", priority);
    }

    let (_, list_out, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(list_out.contains("low"));
    assert!(list_out.contains("medium"));
    assert!(list_out.contains("high"));
    assert!(list_out.contains("critical"));
}

#[test]
fn test_subissue_with_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent"]);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "subissue", "1", "Child", "-p", "critical"],
    );

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "2"]);
    assert!(show_out.contains("critical"));
}

#[test]
fn test_subissue_with_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent"]);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "subissue", "1", "Child", "-d", "Child description"],
    );

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "2"]);
    assert!(show_out.contains("Child description"));
}

// ==================== Additional Delete Edge Cases ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_delete_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "delete", "999", "-f"]);

    // Should fail or warn about nonexistent issue
    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_delete_with_subissues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Child"]);

    let (success, _, _) = run_atelier(dir.path(), &["issue", "delete", "1", "-f"]);

    assert!(success);

    // Both parent and child should be gone
    let (_, list_out, _) = run_atelier(dir.path(), &["issue", "list", "-s", "all"]);
    assert!(!list_out.contains("Parent"));
    assert!(!list_out.contains("Child"));
}

// ==================== Additional Session Edge Cases ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_work_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["session", "start"]);
    let (success, _, stderr) = run_atelier(dir.path(), &["session", "work", "999"]);

    // Should fail or warn
    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_end_without_start() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["session", "end"]);

    // Should fail or report no active session
    assert!(
        !success || stdout.contains("No active") || stderr.contains("No active"),
        "Ending without starting should fail or report no active session, got stdout: {}, stderr: {}", stdout, stderr
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_status_without_session() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["session", "status"]);

    assert!(success);
    assert!(
        stdout.contains("No active session"),
        "Expected 'No active session' message, got: {}",
        stdout
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_multiple_starts() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["session", "start"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["session", "start"]);

    // Second start should either warn or start a new session
    assert!(success);
    assert!(stdout.contains("already") || stdout.contains("Session") || stdout.contains("started"));
}

// ==================== Additional Next Edge Cases ====================

#[test]
fn test_next_with_blocked_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Blocked issue", "-p", "critical"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Blocker issue", "-p", "low"],
    );
    run_atelier(dir.path(), &["issue", "block", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    // Should suggest the blocker, not the blocked issue
    assert!(
        stdout.contains("Blocker issue"),
        "Next should recommend the unblocked blocker, got: {}",
        stdout
    );
    assert!(
        !stdout.contains("Next: #1"),
        "Next should not recommend the blocked issue as top pick"
    );
}

#[test]
fn test_next_all_closed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue 1"]);
    close_issue_with_evidence(dir.path(), "1", None);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    assert!(
        stdout.contains("No issues found."),
        "Expected empty ready-list message, got: {}",
        stdout
    );
}

// ==================== Additional Archive Edge Cases ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_archive_older_days() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Old issue"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);

    // Try to archive issues older than 0 days (should include our just-closed issue)
    let (success, _, _) = run_atelier(dir.path(), &["archive", "older", "0"]);

    assert!(success);
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_archive_already_archived() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);
    run_atelier(dir.path(), &["archive", "add", "1"]);

    // Try to archive again
    let (success, stdout, stderr) = run_atelier(dir.path(), &["archive", "add", "1"]);

    // Should report already archived or fail
    assert!(
        stdout.contains("already") || stderr.contains("already") || !success,
        "Archiving twice should indicate already archived, got stdout: {}, stderr: {}",
        stdout,
        stderr
    );
}

// ==================== Additional Milestone Edge Cases ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_remove_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    run_atelier(dir.path(), &["issue", "create", "Feature"]);
    run_atelier(dir.path(), &["milestone", "add", "1", "1"]);

    let (success, _, _) = run_atelier(dir.path(), &["milestone", "remove", "1", "1"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["milestone", "show", "1"]);
    assert!(!show_out.contains("Feature") || show_out.contains("No issues"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_show_nonexistent() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["milestone", "show", "999"]);

    assert!(!success || stderr.contains("not found") || stderr.contains("No milestone"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_list_closed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    run_atelier(dir.path(), &["milestone", "create", "v2.0"]);
    run_atelier(dir.path(), &["milestone", "close", "1"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["milestone", "list", "-s", "closed"]);

    assert!(success);
    assert!(stdout.contains("v1.0"));
    assert!(!stdout.contains("v2.0"));
}

// ==================== Additional List Edge Cases ====================

#[test]
fn test_list_filter_by_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Low issue", "-p", "low"]);
    run_atelier(dir.path(), &["issue", "create", "High issue", "-p", "high"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list", "-p", "high"]);

    assert!(success);
    assert!(stdout.contains("High issue"));
    assert!(!stdout.contains("Low issue"));
}

#[test]
fn test_list_all_statuses() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Open issue"]);
    run_atelier(dir.path(), &["issue", "create", "Closed issue"]);
    close_issue_with_evidence(dir.path(), "2", None);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list", "-s", "all"]);

    assert!(success);
    assert!(stdout.contains("Open issue"));
    assert!(stdout.contains("Closed issue"));
}

// ==================== Additional Update Edge Cases ====================

#[test]
fn test_update_description_flag_is_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", "1", "-d", "New description"],
    );

    assert!(!success);
    assert!(stderr.contains("unexpected argument '-d'"), "{stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "update", "--help"]);
    assert!(success, "issue update help failed: {stderr}");
    assert!(!stdout.contains("--description"), "{stdout}");
}

#[test]
fn test_update_nonexistent() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "update", "999", "--title", "New"]);

    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

// ==================== Additional Show Edge Cases ====================

#[test]
fn test_show_with_labels() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    run_atelier(dir.path(), &["issue", "label", "1", "bug"]);
    run_atelier(dir.path(), &["issue", "label", "1", "urgent"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success);
    assert!(stdout.contains("bug"));
    assert!(stdout.contains("urgent"));
}

#[test]
fn test_show_with_comments() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    run_atelier(dir.path(), &["issue", "comment", "1", "First comment"]);
    run_atelier(dir.path(), &["issue", "comment", "1", "Second comment"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success);
    assert!(stdout.contains("First comment"));
    assert!(stdout.contains("Second comment"));
}

#[test]
fn test_show_with_blockers() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker"]);
    run_atelier(dir.path(), &["issue", "block", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success);
    assert!(
        stdout.contains("Blocker")
            || stdout.contains(&issue_ref(dir.path(), 2))
            || stdout.contains("blocked")
    );
}

// ==================== Additional Search Edge Cases ====================

#[test]
fn test_search_no_results() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "search", "nonexistent"]);

    assert!(success);
    assert!(
        stdout.contains("No issues found matching"),
        "Expected 'No issues found matching' message, got: {}",
        stdout
    );
}

#[test]
fn test_search_in_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Generic title",
            "-d",
            "specific_keyword_here",
        ],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "search", "specific_keyword"]);

    assert!(success);
    assert!(stdout.contains("Generic title") || stdout.contains("#1"));
}

// ==================== Init Edge Cases ====================

#[test]
fn test_init_force_update() {
    let dir = tempdir().unwrap();

    run_atelier(dir.path(), &["init"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["init", "--force"]);

    assert!(success);
    assert!(stdout.contains("Atelier initialized successfully"));
    assert!(dir.path().join(".atelier/runtime/state.db").exists());
    assert!(!dir.path().join(".atelier").join("rules").exists());
    assert!(!dir.path().join(".claude").exists());
    assert!(!dir.path().join(".mcp.json").exists());
}

// ==================== Complex Workflow Tests ====================

#[test]
fn test_full_issue_lifecycle() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create
    run_atelier(
        dir.path(),
        &["issue", "create", "Lifecycle test", "-p", "high"],
    );

    // Add labels
    run_atelier(dir.path(), &["issue", "label", "1", "feature"]);

    // Add comment
    run_atelier(dir.path(), &["issue", "comment", "1", "Working on this"]);

    // Update
    run_atelier(dir.path(), &["issue", "update", "1", "-p", "critical"]);

    // Close
    close_issue_with_evidence(dir.path(), "1", None);

    // Verify final state
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success);
    assert!(stdout.contains("Lifecycle test"));
    assert!(stdout.contains("critical"));
    assert!(stdout.contains("feature"));
    assert!(stdout.contains("Working on this"));
    assert!(stdout.contains("done"));
}

#[test]
fn test_dependency_chain() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create a chain: 1 <- 2 <- 3 (3 blocks 2, 2 blocks 1)
    run_atelier(dir.path(), &["issue", "create", "Final task"]);
    run_atelier(dir.path(), &["issue", "create", "Middle task"]);
    run_atelier(dir.path(), &["issue", "create", "First task"]);

    run_atelier(dir.path(), &["issue", "block", "1", "2"]);
    run_atelier(dir.path(), &["issue", "block", "2", "3"]);

    // Only issue 3 should be ready
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success);
    assert!(stdout.contains("First task") || stdout.contains("#3"));
    assert!(!stdout.contains("Final task"));
    assert!(!stdout.contains("Middle task"));

    // Close 3, now 2 should be ready
    close_issue_with_evidence(dir.path(), "3", None);
    let (_, stdout, _) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(stdout.contains("Middle task") || stdout.contains("#2"));
}

// ==================== Targeted Coverage Tests ====================

// --- next.rs: Multiple ready issues with runners-up ---
#[test]
fn test_next_with_multiple_ready_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create multiple issues with different priorities
    run_atelier(
        dir.path(),
        &["issue", "create", "Low prio task", "-p", "low"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Medium prio task", "-p", "medium"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "High prio task", "-p", "high"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Critical task", "-p", "critical"],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    // Should recommend highest priority first
    assert!(stdout.contains("Critical") || stdout.contains("#4"));
    assert!(stdout.contains("Issue Queue"));
}

// --- next.rs: Issue with description preview ---
#[test]
fn test_next_with_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Task with description",
            "-p",
            "high",
            "-d",
            "This is a detailed description for the task",
        ],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    assert!(stdout.contains("description") || stdout.contains("Task with description"));
}

// --- next.rs: Progress with subissues ---
#[test]
fn test_next_with_subissue_progress() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create parent with subissues
    run_atelier(
        dir.path(),
        &["issue", "create", "Parent task", "-p", "high"],
    );
    run_atelier(dir.path(), &["issue", "subissue", "1", "Sub 1"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Sub 2"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Sub 3"]);

    // Close one subissue to create progress
    close_issue_with_evidence(dir.path(), "2", None);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    // Should show progress
    assert!(
        stdout.contains("Progress")
            || stdout.contains("1/3")
            || stdout.contains("subissue")
            || stdout.contains("Parent task")
    );
}

// --- next.rs: Only subissues ready (no parents) ---
#[test]
fn test_next_only_subissues_ready() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create parent that is blocked
    run_atelier(dir.path(), &["issue", "create", "Blocker"]);
    run_atelier(dir.path(), &["issue", "create", "Parent"]);
    run_atelier(dir.path(), &["issue", "block", "2", "1"]);

    // Create unblocked subissue under the blocked parent
    run_atelier(dir.path(), &["issue", "subissue", "2", "Subissue"]);

    // Close the blocker - now parent has only subissue as ready issue
    close_issue_with_evidence(dir.path(), "1", None);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    // Should show something - either parent or subissue
    assert!(
        stdout.contains("Next")
            || stdout.contains("#2")
            || stdout.contains("#3")
            || stdout.contains("Parent")
            || stdout.contains("Subissue")
    );
}

// --- import.rs: Import with parent relationships ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_import_with_parent_relationships() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create and export issues with parent-child relationship
    run_atelier(dir.path(), &["issue", "create", "Parent issue"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Child issue"]);

    let export_path = dir.path().join("export.json");
    run_atelier(
        dir.path(),
        &["export", "-o", export_path.to_str().unwrap(), "-f", "json"],
    );

    // Initialize a fresh directory and import
    let dir2 = tempdir().unwrap();
    init_atelier(dir2.path());

    // Copy export file to new location
    std::fs::copy(&export_path, dir2.path().join("import.json")).unwrap();

    let import_path = dir2.path().join("import.json");
    let (success, stdout, _) = run_atelier(dir2.path(), &["import", import_path.to_str().unwrap()]);

    assert!(success);
    assert!(stdout.contains("Imported") || stdout.contains("import"));

    // Verify the parent-child relationship was preserved
    let (_, tree_out, _) = run_atelier(dir2.path(), &["issue", "tree"]);
    assert!(tree_out.contains("Parent") && tree_out.contains("Child"));
}

// --- import.rs: Import issues with labels and comments ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_import_with_labels_and_comments() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create issue with labels and comments
    run_atelier(dir.path(), &["issue", "create", "Labeled issue"]);
    run_atelier(dir.path(), &["issue", "label", "1", "bug"]);
    run_atelier(dir.path(), &["issue", "label", "1", "urgent"]);
    run_atelier(dir.path(), &["issue", "comment", "1", "First comment"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);

    let export_path = dir.path().join("export.json");
    run_atelier(
        dir.path(),
        &["export", "-o", export_path.to_str().unwrap(), "-f", "json"],
    );

    // Import to fresh directory
    let dir2 = tempdir().unwrap();
    init_atelier(dir2.path());

    std::fs::copy(&export_path, dir2.path().join("import.json")).unwrap();

    let import_path = dir2.path().join("import.json");
    let (success, _, _) = run_atelier(dir2.path(), &["import", import_path.to_str().unwrap()]);

    assert!(success);

    // Verify labels and status were preserved
    let labeled_id = issue_id_by_title(dir2.path(), "Labeled issue");
    let (_, show_out, _) = run_atelier(dir2.path(), &["issue", "show", &labeled_id]);
    assert!(show_out.contains("bug") || show_out.contains("Label"));
    assert!(show_out.contains("done") || show_out.contains("Done"));
}

// --- session.rs: Session with handoff notes from previous session ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_start_shows_handoff_notes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Start and end first session with handoff notes
    run_atelier(dir.path(), &["session", "start"]);
    run_atelier(
        dir.path(),
        &["session", "end", "--notes", "Remember to check auth module"],
    );

    // Start new session - should show handoff notes
    let (success, stdout, _) = run_atelier(dir.path(), &["session", "start"]);

    assert!(success);
    assert!(
        stdout.contains("Remember to check auth module")
            || stdout.contains("Handoff")
            || stdout.contains("Previous")
            || stdout.contains("notes")
    );
}

// --- session.rs: Session status with active issue ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_status_with_active_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Active task"]);
    run_atelier(dir.path(), &["session", "start"]);
    run_atelier(dir.path(), &["session", "work", "1"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["session", "status"]);

    assert!(success);
    assert!(stdout.contains("Active task") || stdout.contains("#1") || stdout.contains("Working"));
}

// --- create.rs: Template with user priority override ---
#[test]
fn test_template_with_priority_override() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Bug template defaults to high, override to critical
    let (success, stdout, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Critical bug",
            "-t",
            "bug",
            "-p",
            "critical",
        ],
    );

    assert!(success);
    assert!(stdout.contains("atelier-"));

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("critical"));
}

// --- create.rs: Template with user description ---
#[test]
fn test_template_with_user_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Bug with details",
            "-t",
            "bug",
            "-d",
            "User provided details here",
        ],
    );

    assert!(success);
    assert!(stdout.contains("atelier-"));

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    // Should have both template prefix and user description
    assert!(show_out.contains("User provided details") || show_out.contains("Steps to reproduce"));
}

// --- create.rs: Subissue with invalid parent ---
#[test]
fn test_subissue_invalid_parent() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "subissue", "999", "Orphan"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999") || stderr.contains("Parent"));
}

// --- relate.rs: Related issues display ---
#[test]
fn test_related_issues_display() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue A"]);
    run_atelier(dir.path(), &["issue", "create", "Issue B"]);
    run_atelier(dir.path(), &["issue", "create", "Issue C"]);

    run_atelier(dir.path(), &["issue", "relate", "1", "2"]);
    run_atelier(dir.path(), &["issue", "relate", "1", "3"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "related", "1"]);

    assert!(success);
    assert!(stdout.contains("Issue B") || stdout.contains("#2"));
    assert!(stdout.contains("Issue C") || stdout.contains("#3"));
}

// --- label.rs: Multiple labels on same issue ---
#[test]
fn test_multiple_labels() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Multi-label issue"]);
    run_atelier(dir.path(), &["issue", "label", "1", "bug"]);
    run_atelier(dir.path(), &["issue", "label", "1", "urgent"]);
    run_atelier(dir.path(), &["issue", "label", "1", "frontend"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success);
    assert!(stdout.contains("bug"));
    assert!(stdout.contains("urgent"));
    assert!(stdout.contains("frontend"));
}

// --- Export markdown format test ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_export_markdown_format() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue for markdown"]);
    run_atelier(dir.path(), &["issue", "label", "1", "test"]);
    run_atelier(dir.path(), &["issue", "comment", "1", "Test comment"]);

    let export_path = dir.path().join("export.md");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "export",
            "-o",
            export_path.to_str().unwrap(),
            "-f",
            "markdown",
        ],
    );

    assert!(success);
    assert!(
        stderr.contains("Exported"),
        "Expected 'Exported' in stderr, got: {}",
        stderr
    );

    // Verify file exists and has the issue content
    let content = std::fs::read_to_string(&export_path).unwrap();
    assert!(
        content.contains("Issue for markdown"),
        "Exported markdown should contain issue title, got: {}",
        &content[..content.len().min(200)]
    );
}

// --- Archive older days test ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_archive_older_no_matches() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create and close an issue (just now, so not old)
    run_atelier(dir.path(), &["issue", "create", "New issue"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);

    // Archive issues older than 30 days - should find none
    let (success, stdout, _) = run_atelier(dir.path(), &["archive", "older", "30"]);

    assert!(success);
    assert!(
        stdout.contains("No issues to archive") || stdout.contains("Archived 0"),
        "Should report no issues to archive, got: {}",
        stdout
    );
}
