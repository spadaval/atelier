use super::*;

// ==================== Issue Creation Tests ====================

#[test]
fn test_create_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "create", "Test issue"]);

    assert!(success);
    assert!(
        stdout.contains("Created issue atelier-"),
        "Expected project-scoped issue id in output, got: {}",
        stdout
    );
    let issue_id = issue_id_by_title(dir.path(), "Test issue");
    assert!(stdout.contains(&format!(".atelier/issues/{issue_id}.md")));
    assert!(stdout.contains(&format!("atelier check {issue_id}")));
    assert!(stdout.contains(&format!("atelier issue show {issue_id}")));
    let issue_text = read_canonical_record(dir.path(), "issues", &issue_id);
    assert!(issue_text.contains("## Goal\n\nGoal was not specified."));
    assert!(issue_text.contains("## Done when\n\nDone when was not specified."));
    assert!(!issue_text.contains("## Context"));

    let (success, lint_out, stderr) = run_atelier(dir.path(), &["check", &issue_id]);
    assert!(!success, "placeholder scaffold should be under-specified");
    assert!(stderr.contains("Lint failed"));
    assert!(lint_out.contains("Issue section Goal must be present and non-empty"));
    assert!(lint_out.contains("Issue section Done when must be present and non-empty"));
    assert!(!lint_out.contains("Issue section Context must be present and non-empty"));
}

#[test]
fn test_create_issue_with_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "create", "High priority issue", "-p", "high"],
    );

    assert!(success);

    // Verify it was created with correct priority
    let (_, list_out, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(list_out.contains("high"));
}

#[test]
fn test_create_issue_with_description_is_rejected() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier_raw(
        dir.path(),
        &[
            "issue",
            "create",
            "Issue with desc",
            "-d",
            "Detailed description here",
        ],
    );

    assert!(!success, "issue create -d should be removed");
    assert!(stderr.contains("unexpected argument") || stderr.contains("Usage:"));
}

#[test]
fn test_issue_create_help_is_markdown_first() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "create", "--help"]);
    assert!(success, "issue create help failed: {stderr}");
    assert!(stdout.contains("--description <DESCRIPTION>"), "{stdout}");
    assert!(stdout.contains("--template"), "{stdout}");
}

#[test]
fn test_issue_create_scaffold_edit_lint_show_flow() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Markdown first issue"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Markdown first issue");
    assert!(stdout.contains(&format!(".atelier/issues/{issue_id}.md")));

    let path = canonical_issue_path(dir.path(), &issue_id);
    let text = std::fs::read_to_string(&path).unwrap();
    let text = text
        .replace(
            "Goal was not specified.",
            "Describe the markdown-first issue.",
        )
        .replace(
            "Done when was not specified.",
            "Issue sections are populated by editing canonical Markdown.\n\n## Context\n\n- `atelier check <id>` passes after section edits.",
        );
    std::fs::write(&path, text).unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["check", &issue_id]);
    assert!(success, "lint after markdown edit failed: {stderr}");
    let (success, show, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(show.contains("Describe the markdown-first issue."));
    assert!(show.contains("Issue sections are populated by editing canonical Markdown."));
    assert!(show.contains("atelier check <id>"));
}

#[test]
fn test_create_subissue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent issue", "--issue-type", "epic"],
    );
    let parent_id = issue_ref(dir.path(), 1);
    let (success, stdout, _) = run_atelier(
        dir.path(),
        &["issue", "create", "Child issue", "--parent", &parent_id],
    );

    assert!(success);
    assert!(
        stdout.contains("Created subissue atelier-"),
        "Expected project-scoped issue id in output, got: {}",
        stdout
    );
    let child_id = issue_id_by_title(dir.path(), "Child issue");
    assert!(stdout.contains(&format!(".atelier/issues/{child_id}.md")));
    assert!(stdout.contains(&format!("atelier check {child_id}")));

    // Verify parent-child relationship in show
    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", &parent_id]);
    assert!(show_out.contains("Child") || show_out.contains("subissue"));
}

#[test]
fn test_issue_create_rejects_invalid_hierarchy_shapes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Parent epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Parent epic");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Mission child",
            "--issue-type",
            "mission",
            "--parent",
            &epic_id,
        ],
    );
    assert!(!success, "mission child should be rejected");
    assert!(
        stderr.contains("workflow_issue_hierarchy_invalid")
            && stderr.contains("mission issue")
            && stderr.contains("cannot have parent"),
        "{stderr}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Epic child",
            "--issue-type",
            "epic",
            "--parent",
            &epic_id,
        ],
    );
    assert!(!success, "epic child should be rejected");
    assert!(
        stderr.contains("workflow_issue_hierarchy_invalid")
            && stderr.contains("epic issue")
            && stderr.contains("cannot have parent"),
        "{stderr}"
    );

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Task parent"]);
    assert!(success, "task parent create failed: {stderr}");
    let task_id = issue_id_by_title(dir.path(), "Task parent");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Task child", "--parent", &task_id],
    );
    assert!(!success, "task parent should be rejected");
    assert!(stderr.contains("only epics can own child work"), "{stderr}");
}

#[test]
fn test_issue_update_and_lint_reject_invalid_hierarchy_shapes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Parent epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    let parent_id = issue_id_by_title(dir.path(), "Parent epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Child work", "--parent", &parent_id],
    );
    assert!(success, "child create failed: {stderr}");
    edit_canonical_issue(dir.path(), &parent_id, |markdown| {
        replace_front_matter_scalar(&markdown, "status", "blocked")
    });

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", &parent_id, "--issue-type", "task"],
    );
    assert!(!success, "parent with children should not become task");
    assert!(
        stderr.contains("workflow_issue_hierarchy_invalid")
            && stderr.contains("cannot own child work"),
        "{stderr}"
    );

    let parent_path = canonical_issue_path(dir.path(), &parent_id);
    let parent_markdown = std::fs::read_to_string(&parent_path).unwrap();
    let corrupted = parent_markdown.replace("issue_type: \"epic\"", "issue_type: \"task\"");
    std::fs::write(
        &parent_path,
        replace_markdown_body(
            &replace_front_matter_scalar(&corrupted, "status", "todo"),
            "## Goal\n\nCorrupted hierarchy\n\n## Done when\n\nHierarchy validates.",
        ),
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(!success, "lint should reject corrupted parent type");
    let transcript = format!("{stdout}\n{stderr}");
    assert!(
        transcript.contains("workflow_issue_hierarchy_invalid")
            && transcript.contains("only epics can own child work"),
        "{transcript}"
    );
}

#[test]
fn test_create_issue_rejects_work_flag() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Work path issue", "--work"],
    );
    assert!(!success, "issue create --work should be rejected");
    assert!(stderr.contains("unexpected argument '--work'"));
}

#[test]
fn test_configured_custom_issue_link_is_context_only() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    append_custom_issue_links(dir.path(), &["informs"]);

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Context mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Context mission");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Mission work"]);
    assert!(success, "work create failed: {stderr}");
    let work_id = issue_id_by_title(dir.path(), "Mission work");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Context note"]);
    assert!(success, "context create failed: {stderr}");
    let context_id = issue_id_by_title(dir.path(), "Context note");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &mission_id, &work_id, "--role", "advances"],
    );
    assert!(success, "advances link failed: {stderr}");
    let (success, link_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &mission_id,
            &context_id,
            "--role",
            "informs",
        ],
    );
    assert!(success, "custom link failed: {stderr}");
    assert!(link_out.contains("Linked"));

    let mission_markdown = read_canonical_record(dir.path(), "issues", &mission_id);
    assert!(mission_markdown.contains("type: \"advances\""));
    assert!(mission_markdown.contains("type: \"informs\""));

    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &mission_id]);
    assert!(success, "show failed: {stderr}");
    assert!(show_out.contains("informs"), "{show_out}");
    assert!(show_out.contains("Context note"), "{show_out}");

    let (success, mission_out, stderr) = run_atelier(dir.path(), &["work", "mission", &mission_id]);
    assert!(success, "work mission failed: {stderr}");
    assert!(mission_out.contains("Mission work"), "{mission_out}");
    assert!(
        !mission_out.contains("Context note"),
        "custom context links must not count as mission work:\n{mission_out}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "unlink",
            &mission_id,
            &context_id,
            "--role",
            "informs",
        ],
    );
    assert!(success, "custom unlink failed: {stderr}");
    let mission_markdown = read_canonical_record(dir.path(), "issues", &mission_id);
    assert!(!mission_markdown.contains("type: \"informs\""));
}

#[test]
fn test_unconfigured_custom_issue_link_is_rejected() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Source issue"]);
    run_atelier(dir.path(), &["issue", "create", "Target issue"]);
    let source_id = issue_id_by_title(dir.path(), "Source issue");
    let target_id = issue_id_by_title(dir.path(), "Target issue");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &source_id, &target_id, "--role", "informs"],
    );

    assert!(!success, "unconfigured custom role should be rejected");
    assert!(
        stderr.contains("Invalid issue link role 'informs'")
            && stderr.contains("Configured custom context-only roles: (none)")
            && stderr.contains("[issue_links].custom_context_types"),
        "{stderr}"
    );
}

#[test]
fn test_issue_to_issue_validates_link_is_rejected() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Validation source"]);
    run_atelier(dir.path(), &["issue", "create", "Validation target"]);
    let source_id = issue_id_by_title(dir.path(), "Validation source");
    let target_id = issue_id_by_title(dir.path(), "Validation target");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &source_id,
            &target_id,
            "--role",
            "validates",
        ],
    );

    assert!(!success, "issue-to-issue validates should be rejected");
    assert!(
        stderr.contains("Proof roles are reserved for evidence links")
            && stderr.contains("atelier evidence record"),
        "{stderr}"
    );
}

// ==================== Issue Listing Tests ====================

#[test]
fn test_list_empty() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);

    assert!(success);
    assert!(
        stdout.contains("No issues match the selected filters."),
        "Expected empty inventory in output, got: {}",
        stdout
    );
}

#[test]
fn test_list_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Issue 1", "--issue-type", "epic"],
    );
    let parent_id = issue_ref(dir.path(), 1);
    run_atelier(
        dir.path(),
        &[
            "issue", "create", "Issue 2", "--parent", &parent_id, "-p", "high",
        ],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);

    assert!(success);
    assert!(stdout.contains("Issue Inventory"));
    assert!(stdout.contains("Showing 2 of 2 issues"));
    assert!(stdout.contains("atelier-"));
    assert!(stdout.contains("task"));
    // Inventory includes both the draft epic and its child as flat records.
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));

    let (success, quiet_out, stderr) = run_atelier(dir.path(), &["--quiet", "issue", "list"]);
    assert!(success, "quiet inventory failed: {stderr}");
    assert!(!quiet_out.contains("Work Queue"));
    assert!(!quiet_out.contains("Issue 1"));
    assert_eq!(quiet_out.lines().count(), 2);
    assert!(quiet_out.lines().all(|line| line.starts_with("atelier-")));
}

#[test]
fn test_panel_surfaces_are_complete_without_terminal_color() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Colorless issue"]);
    let issue_id = issue_id_by_title(dir.path(), "Colorless issue");

    for args in [
        vec!["status"],
        vec!["work"],
        vec!["work", "ready"],
        vec!["issue", "show", &issue_id],
    ] {
        let (success, stdout, stderr) =
            run_atelier_with_env(dir.path(), &args, &[("NO_COLOR", "1")]);
        assert!(success, "{args:?} failed: {stderr}");
        assert!(
            !stdout.contains("\u{1b}["),
            "NO_COLOR/non-terminal output must not contain ANSI escapes:\n{stdout}"
        );
        assert!(
            stdout.contains("Next Commands")
                || stdout.contains("Dashboards")
                || stdout.contains("Ready Work")
                || stdout.contains("Current Work"),
            "colorless output must retain structural text:\n{stdout}"
        );
    }
}

#[test]
fn test_legacy_work_queue_is_removed_without_alias_or_mutation() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let before = super::mission_worktree::canonical_tree_snapshot(&dir.path().join(".atelier"));
    let (success, help, stderr) = run_atelier(dir.path(), &["work", "--help"]);
    assert!(success, "{stderr}");
    assert!(!help.contains("queue"), "{help}");
    for args in [vec!["work", "queue"], vec!["work", "queue", "--ready"]] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(!success);
        assert!(stdout.is_empty(), "{stdout}");
        assert!(stderr.contains("unrecognized subcommand"), "{stderr}");
    }
    assert_eq!(
        super::mission_worktree::canonical_tree_snapshot(&dir.path().join(".atelier")),
        before
    );
}

#[test]
fn test_issue_show_rollup_counts_all_children_and_bounds_only_visible_rows() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Complete scope", "--issue-type", "epic"],
    );
    assert!(success, "{stderr}");
    let owner = issue_id_by_title(dir.path(), "Complete scope");
    for number in 0..7 {
        let title = format!("Scoped ready {number}");
        let (success, _, stderr) =
            run_atelier(dir.path(), &["issue", "create", &title, "--parent", &owner]);
        assert!(success, "{stderr}");
    }
    let (success, shown, stderr) = run_atelier(dir.path(), &["issue", "show", &owner]);
    assert!(success, "{stderr}");
    let rollup = shown.split("Objective Rollup").nth(1).unwrap();
    assert!(rollup.contains("7 scoped issues"), "{rollup}");
    assert!(rollup.contains("ready 7"), "{rollup}");
    let ready_rows = rollup
        .split("Ready Work")
        .nth(1)
        .unwrap()
        .split("Recent Activity Facts")
        .next()
        .unwrap();
    assert_eq!(
        ready_rows.matches("Scoped ready").count(),
        5,
        "{ready_rows}"
    );
    assert!(
        ready_rows.contains("2 more ready work items omitted"),
        "{ready_rows}"
    );
}

#[test]
fn test_work_ready_excludes_blocked_and_quiet_matches_human_order() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Low ready", "-p", "low"]);
    run_atelier(dir.path(), &["issue", "create", "High ready", "-p", "high"]);
    run_atelier(
        dir.path(),
        &["issue", "create", "Blocked ready", "-p", "critical"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Ready blocker", "-p", "medium"],
    );
    let low_id = issue_id_by_title(dir.path(), "Low ready");
    let high_id = issue_id_by_title(dir.path(), "High ready");
    let blocked_id = issue_id_by_title(dir.path(), "Blocked ready");
    let blocker_id = issue_id_by_title(dir.path(), "Ready blocker");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &blocked_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );
    assert!(success, "issue link failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["work", "ready"]);
    assert!(success, "work ready failed: {stderr}");
    assert!(stdout.contains("High ready"), "{stdout}");
    assert!(stdout.contains("Low ready"), "{stdout}");
    assert!(stdout.contains("Ready blocker"), "{stdout}");
    assert!(!stdout.contains("Blocked ready"), "{stdout}");
    assert!(
        stdout.find("High ready").unwrap() < stdout.find("Low ready").unwrap(),
        "high priority ready work should sort before low priority work:\n{stdout}"
    );

    let (success, quiet, stderr) = run_atelier(dir.path(), &["--quiet", "work", "ready"]);
    assert!(success, "quiet work ready failed: {stderr}");
    let quiet_ids = quiet.lines().collect::<Vec<_>>();
    assert!(!quiet_ids.contains(&blocked_id.as_str()), "{quiet}");
    assert_eq!(
        quiet_ids.first().copied(),
        Some(high_id.as_str()),
        "{quiet}"
    );
    assert!(quiet_ids.contains(&low_id.as_str()), "{quiet}");
    assert!(quiet_ids.contains(&blocker_id.as_str()), "{quiet}");
}

#[test]
fn test_issue_show_subissues_use_blocker_order_and_state_labels() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent epic", "--issue-type", "epic"],
    );
    let parent_id = issue_ref(dir.path(), 1);
    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Implementation child",
            "--parent",
            &parent_id,
        ],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Contract child", "--parent", &parent_id],
    );
    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "External blocked child",
            "--parent",
            &parent_id,
        ],
    );
    run_atelier(dir.path(), &["issue", "create", "External blocker"]);
    let implementation_id = issue_ref(dir.path(), 2);
    let contract_id = issue_ref(dir.path(), 3);
    let external_child_id = issue_ref(dir.path(), 4);
    let external_blocker_id = issue_ref(dir.path(), 5);
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &implementation_id,
            &contract_id,
            "--role",
            "blocked_by",
        ],
    );
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &external_child_id,
            &external_blocker_id,
            "--role",
            "blocked_by",
        ],
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &parent_id]);
    assert!(success, "issue show failed: {stderr}");
    let contract_pos = stdout.find("Contract child").unwrap_or(usize::MAX);
    let implementation_pos = stdout.find("Implementation child").unwrap_or(usize::MAX);
    assert!(
        contract_pos < implementation_pos,
        "visible blocker child should appear before dependent child:\n{stdout}"
    );
    assert!(
        stdout.contains(&format!("ready {contract_id} [todo]")),
        "{stdout}"
    );
    assert!(
        stdout.contains(&format!("blocked {implementation_id} [todo]")),
        "{stdout}"
    );
    assert!(
        stdout.contains(&format!(
            "blocked {external_child_id} [todo] medium - External blocked child | 1 blocker"
        )),
        "{stdout}"
    );
    assert!(!stdout.contains(&external_blocker_id), "{stdout}");
    assert!(stdout.contains("Next Commands"));
}

#[test]
fn test_list_filter_by_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Open issue"]);
    run_atelier(dir.path(), &["issue", "create", "Closed issue"]);
    close_issue_with_evidence(dir.path(), "2", None);

    let (_, open_list, _) = run_atelier(dir.path(), &["issue", "list", "-s", "todo"]);
    assert!(open_list.contains("Open issue"));
    assert!(!open_list.contains("Closed issue"));

    let (_, closed_list, _) = run_atelier(dir.path(), &["issue", "list", "-s", "done"]);
    assert!(closed_list.contains("Closed issue"));
    assert!(!closed_list.contains("Open issue"));
}

#[test]
fn test_list_filter_by_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Bug issue"]);
    run_atelier(dir.path(), &["issue", "create", "Feature issue"]);
    let bug_id = issue_ref(dir.path(), 1);
    let feature_id = issue_ref(dir.path(), 2);
    run_atelier(dir.path(), &["issue", "update", &bug_id, "--label", "bug"]);
    run_atelier(
        dir.path(),
        &["issue", "update", &feature_id, "--label", "feature"],
    );

    let (_, bug_list, _) = run_atelier(dir.path(), &["issue", "list", "-l", "bug"]);
    assert!(bug_list.contains("Bug issue"));
    assert!(!bug_list.contains("Feature issue"));
}

// ==================== Issue Show Tests ====================

#[test]
fn test_show_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    let issue_id = issue_id_by_title(dir.path(), "Test issue");
    edit_canonical_record(dir.path(), "issues", &issue_id, |text| {
        text.replace("Goal was not specified.", "Description")
            .replace(
                "Done when was not specified.",
                "The issue show command renders parsed sections.\n\n## Context\n\n- Show output contains the section headings.\n\n## Notes\n\nCLI display context.",
            )
    });

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", &issue_id]);

    assert!(success);
    assert!(stdout.contains("Test issue"));
    assert!(stdout.contains("Goal"));
    assert!(stdout.contains("Done when"));
    assert!(stdout.contains("The issue show command renders parsed sections."));
    assert!(stdout.contains("Context"));
    assert!(stdout.contains("- Show output contains the section headings."));
    assert!(stdout.contains("Notes"));
    assert!(stdout.contains("CLI display context."));
    assert!(!stdout.contains("Acceptance Criteria"));
}

#[test]
fn test_issue_show_surfaces_evidence_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_evidence_gated_workflow(dir.path());
    init_git_repo(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Evidence status issue"]);
    let issue_id = issue_id_by_title(dir.path(), "Evidence status issue");
    edit_canonical_record(dir.path(), "issues", &issue_id, |text| {
        text.replace(
            "Goal was not specified.",
            "Exercise issue show evidence state.",
        )
        .replace(
            "Done when was not specified.",
            "Issue show renders attached proof state and next commands.\n\n## Context\n\n- Manual check: issue show and transition options report missing and attached validation evidence.",
        )
    });
    commit_all(dir.path(), "evidence status issue setup");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "issue show without evidence failed: {stderr}");
    assert!(!stdout.contains("Evidence Status"));
    assert!(!stdout.contains("Attached Proof: missing - no validating evidence link found"));

    move_issue_to_review(dir.path(), &issue_id);
    let (success, transitions, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--verbose"]);
    assert!(
        success,
        "transition options without evidence failed: {stderr}"
    );
    assert!(transitions.contains("close"));
    assert!(transitions.contains("fail  evidence.attached"));
    assert!(transitions.contains("expected at least 1 fresh passing evidence record(s); found 0"));
    assert!(
        transitions.contains("Hint: Record fresh proof with `atelier evidence record --target issue/<id> --kind test -- <command>`"),
        "{transitions}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--result",
            "pass",
            "--target",
            &format!("issue/{issue_id}"),
            "--kind",
            "validation",
            "Issue show evidence status validated",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    commit_all(dir.path(), "attach validation evidence");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "issue show with evidence failed: {stderr}");
    assert!(!stdout.contains("Evidence Status"));
    assert!(!stdout.contains("Attached Proof: attached - passing validating evidence is linked"));

    let (success, transitions, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--verbose"]);
    assert!(success, "transition options with evidence failed: {stderr}");
    assert!(transitions.contains("pass  evidence.attached"));
}

#[test]
fn test_issue_commands_accept_partial_issue_key() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Partial key issue"]);
    let issue_id = issue_id_by_title(dir.path(), "Partial key issue");
    let key = issue_key(&issue_id);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", key]);

    assert!(success, "show by partial key failed: {stderr}");
    assert!(stdout.contains(&issue_id));
    assert!(stdout.contains("Partial key issue"));
}

#[test]
fn test_issue_reference_surfaces_accept_partial_issue_keys() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Parent key issue",
            "--issue-type",
            "epic",
        ],
    );
    run_atelier(dir.path(), &["issue", "create", "Related key issue"]);
    let parent_id = issue_id_by_title(dir.path(), "Parent key issue");
    let related_id = issue_id_by_title(dir.path(), "Related key issue");
    let parent_key = issue_key(&parent_id);
    let related_key = issue_key(&related_id);

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Child key issue", "--parent", parent_key],
    );
    assert!(success, "subissue by partial key failed: {stderr}");
    assert!(stdout.contains(&parent_id));
    let child_id = issue_id_by_title(dir.path(), "Child key issue");
    assert!(!child_id.is_empty());

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "link", parent_key, related_key]);
    assert!(success, "relate by partial keys failed: {stderr}");
    assert!(stdout.contains(&parent_id));
    assert!(stdout.contains(&related_id));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", parent_key]);
    assert!(success, "related by partial key failed: {stderr}");
    assert!(stdout.contains(&related_id));

    migrate_default_issue_workflow(dir.path());
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "transition", parent_key]);
    assert!(
        success,
        "transition options by partial key failed: {stderr}"
    );
    assert!(stdout.contains(&format!("Issue Transitions {parent_id}")));
}

#[test]
fn test_bundle_apply_accepts_partial_issue_key_refs() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Existing issue target"]);
    let issue_id = issue_id_by_title(dir.path(), "Existing issue target");
    let issue_key = issue_key(&issue_id);
    let bundle_path = dir.path().join("partial-key-bundle.json");
    std::fs::write(
        &bundle_path,
        format!(
            r#"{{
  "schema": "atelier.bundle",
  "schema_version": 2,
  "title": "Partial key bundle apply",
  "resources": {{
    "issues": [
      {{
        "client_ref": "issue.partial",
        "title": "Partial key dependent", "body": "\u0023\u0023 Goal\n\nPartial key dependent\n\n## Done when\n\nChecks pass.",
        "issue_type": "task",
        "priority": "medium",
        "depends_on": [{{ "id": "{issue_key}" }}]
      }}
    ]
  }}
}}"#
        ),
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["bundle", "apply", bundle_path.to_str().unwrap(), "--yes"],
    );
    assert!(
        success,
        "bundle apply by partial issue key failed: {stderr}"
    );
    assert!(stdout.contains("Bundle applied."));

    let dependent_id = issue_id_by_title(dir.path(), "Partial key dependent");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &dependent_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(stdout.contains(&issue_id));
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &dependent_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(stdout.contains("Status:   todo"), "{stdout}");
}

#[test]
fn test_issue_create_update_and_transition_use_custom_issue_type() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_incident_issue_type_workflow(dir.path());

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Incident response",
            "--issue-type",
            "incident",
        ],
    );
    assert!(success, "custom issue create failed: {stderr}");
    assert!(stdout.contains("Type:     incident"), "{stdout}");
    let issue_id = issue_id_by_title(dir.path(), "Incident response");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "custom issue transition failed: {stderr}");
    assert!(stdout.contains("To:       in_progress"), "{stdout}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "custom issue show failed: {stderr}");
    assert!(stdout.contains("Type:     incident"), "{stdout}");
    assert!(stdout.contains("Status:   in_progress"), "{stdout}");
}

#[test]
fn test_issue_create_mission_type_requires_workflow_policy_declaration() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_workflow_without_mission_issue_type(dir.path());

    let (success, stdout, stderr) = run_atelier_raw(
        dir.path(),
        &[
            "issue",
            "create",
            "Typed objective",
            "--issue-type",
            "mission",
        ],
    );
    assert!(
        !success,
        "undeclared mission issue create should fail: {stdout}"
    );
    let transcript = format!("{stdout}\n{stderr}");
    assert!(
        transcript.contains(".atelier/workflow.yaml"),
        "{transcript}"
    );
    assert!(transcript.contains("issue_types"), "{transcript}");
    assert!(transcript.contains("mission"), "{transcript}");
    assert!(
        !dir.path().join(".atelier").join("missions").exists(),
        "mission type creation must not write first-class mission records"
    );
}

#[test]
fn test_issue_create_mission_type_uses_declared_workflow_policy() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Typed objective",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "declared mission issue create failed: {stderr}");
    assert!(
        stdout.contains("Created mission objective atelier-"),
        "{stdout}"
    );
    assert!(stdout.contains("Type:     mission"), "{stdout}");
    assert!(stdout.contains(".atelier/issues/"), "{stdout}");
    assert!(!stdout.contains(".atelier/missions/"), "{stdout}");

    let issue_id = issue_id_by_title(dir.path(), "Typed objective");
    let issue_text = read_canonical_record(dir.path(), "issues", &issue_id);
    assert!(
        issue_text.contains("schema: \"atelier.issue\""),
        "{issue_text}"
    );
    assert!(
        issue_text.contains("issue_type: \"mission\""),
        "{issue_text}"
    );

    let (success, show, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "declared mission issue show failed: {stderr}");
    assert!(show.contains("Type:     mission"), "{show}");
    assert!(
        show.contains("Status:   draft"),
        "declared mission should use its configured initial status: {show}"
    );
}

#[test]
fn test_issue_create_mission_sectioned_body_preserves_outcome_and_evidence() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let outcome = "Mission authoring preserves the supplied outcome.";
    let evidence = "- `atelier check` accepts the created mission record.";
    let body = format!("## Outcome\n\nMission result.\n\n## Success criteria\n\n{outcome}\n\n## Epics\n\n{evidence}");
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Sectioned mission body",
            "--issue-type",
            "mission",
            "--body",
            &body,
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Sectioned mission body");
    let mission = read_canonical_record(dir.path(), "issues", &mission_id);

    assert_eq!(mission.matches("## Outcome").count(), 1, "{mission}");
    assert_eq!(
        mission.matches("## Success criteria").count(),
        1,
        "{mission}"
    );
    assert_eq!(mission.matches("## Epics").count(), 1, "{mission}");
    assert!(mission.contains("## Outcome\n\nMission result."));
    assert!(mission.contains(&format!("## Success criteria\n\n{outcome}")));
    assert!(mission.contains(&format!("## Epics\n\n{evidence}")));

    let (success, check_out, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(
        success,
        "atelier check rejected mission created by a successful command:\nstdout:\n{check_out}\nstderr:\n{stderr}\ncreate output:\n{stdout}"
    );
}

#[test]
fn test_bundle_apply_accepts_configured_custom_issue_type() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_incident_issue_type_workflow(dir.path());
    let bundle_path = dir.path().join("custom-type-bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 2,
  "title": "Custom type bundle",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.incident",
        "title": "Bundled incident",
        "issue_type": "incident",
        "priority": "high",
        "body": "\u0023\u0023 Goal\n\nBundled incident\n\n## Done when\n\nRun the focused regression tests."
      }
    ]
  }
}"#,
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["bundle", "apply", bundle_path.to_str().unwrap(), "--yes"],
    );
    assert!(success, "custom bundle apply failed: {stderr}");
    assert!(stdout.contains("Bundle applied."));

    let issue_id = issue_id_by_title(dir.path(), "Bundled incident");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "custom bundled issue show failed: {stderr}");
    assert!(stdout.contains("Type:     incident"), "{stdout}");
}

#[test]
fn test_unregistered_issue_type_reports_configured_values() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_incident_issue_type_workflow(dir.path());
    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Nope", "--issue-type", "ghost"],
    );
    assert!(!success, "unregistered issue type should fail");
    assert!(
        stderr.contains("must declare issue_type 'ghost'"),
        "{stderr}"
    );
    assert!(stderr.contains("incident"), "{stderr}");
    assert!(!stderr.contains("decision"), "{stderr}");
}

#[test]
fn test_issue_type_update_rejects_incompatible_existing_status_atomically() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Draft epic", "--issue-type", "epic"],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Draft epic");

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", &issue_id, "--issue-type", "spike"],
    );
    assert!(!success, "incompatible issue type update should fail");
    assert!(
        stderr.contains("status 'draft' which is not valid")
            || stderr.contains("not allowed by the workflow policy"),
        "{stderr}"
    );

    let issue_text = read_canonical_record(dir.path(), "issues", &issue_id);
    assert!(issue_text.contains("issue_type: \"epic\""), "{issue_text}");
    assert!(
        !issue_text.contains("issue_type: \"spike\""),
        "{issue_text}"
    );
}

#[test]
fn test_issue_type_help_uses_workflow_policy_wording() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "create", "--help"]);
    assert!(success, "issue create help failed: {stderr}");
    assert!(
        stdout.contains(".atelier/workflow.yaml issue_types"),
        "{stdout}"
    );
    assert!(!stdout.contains("bug, epic, feature, spike, task, validation"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "update", "--help"]);
    assert!(success, "issue update help failed: {stderr}");
    assert!(
        stdout.contains(".atelier/workflow.yaml issue_types"),
        "{stdout}"
    );
    assert!(!stdout.contains("bug, epic, feature, spike, task, validation"));
}

#[test]
fn test_plan_apply_command_is_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _stdout, stderr) = run_atelier(dir.path(), &["plan", "apply", "bundle.json"]);

    assert!(!success, "plan apply should be removed");
    assert!(
        stderr.contains("unrecognized subcommand") || stderr.contains("unexpected argument"),
        "{stderr}"
    );
}

#[test]
fn test_bundle_preview_rejects_plan_and_milestone_resources() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bundle_path = dir.path().join("invalid-bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 2,
  "title": "Invalid bundle",
  "resources": {
    "issues": [],
    "plans": [
      {
        "client_ref": "plan.invalid",
        "title": "No plan resources"
      }
    ],
    "milestones": [
      {
        "client_ref": "milestone.invalid",
        "title": "No milestones"
      }
    ]
  }
}"#,
    )
    .unwrap();

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["bundle", "preview", bundle_path.to_str().unwrap()],
    );

    assert!(
        !success,
        "bundle preview should reject v1 plan/milestone resources"
    );
    assert!(
        stderr.contains("unknown field `plans`") || stderr.contains("unknown field `milestones`"),
        "{stderr}"
    );
}

#[test]
fn test_bundle_preview_rejects_duplicate_client_refs() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bundle_path = dir.path().join("duplicate-client-ref-bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 2,
  "title": "Duplicate client ref bundle",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.duplicate",
        "title": "Duplicate one",
        "issue_type": "task",
        "priority": "high",
        "body": "\u0023\u0023 Goal\n\nDuplicate one\n\n## Done when\n\nRun the focused regression tests."
      },
      {
        "client_ref": "issue.duplicate",
        "title": "Duplicate two",
        "issue_type": "task",
        "priority": "high",
        "body": "\u0023\u0023 Goal\n\nDuplicate two\n\n## Done when\n\nRun the focused regression tests."
      }
    ]
  }
}"#,
    )
    .unwrap();

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["bundle", "preview", bundle_path.to_str().unwrap()],
    );

    assert!(
        !success,
        "bundle preview should reject duplicate client_ref"
    );
    assert!(
        stderr.contains("Duplicate client_ref 'issue.duplicate'"),
        "{stderr}"
    );
}

#[test]
fn test_bundle_preview_rejects_missing_client_ref() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bundle_path = dir.path().join("missing-client-ref-bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 2,
  "title": "Missing client ref bundle",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.ref-user",
        "title": "Missing reference user",
        "issue_type": "task",
        "priority": "high",
        "depends_on": [
          {
            "client_ref": "issue.missing"
          }
        ],
        "body": "\u0023\u0023 Goal\n\nMissing reference user\n\n## Done when\n\nRun the focused regression tests."
      }
    ]
  }
}"#,
    )
    .unwrap();

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["bundle", "preview", bundle_path.to_str().unwrap()],
    );

    assert!(!success, "bundle preview should reject missing client_ref");
    assert!(
        stderr.contains("Reference 'issue.missing' for issue.ref-user does not resolve"),
        "{stderr}"
    );
}

#[test]
fn test_bundle_preview_rejects_status_outside_workflow_policy() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bundle_path = dir.path().join("invalid-status-bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 2,
  "title": "Invalid status bundle",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.invalid-status",
        "title": "Invalid status",
        "issue_type": "task",
        "priority": "high",
        "status": "not_real",
        "body": "\u0023\u0023 Goal\n\nInvalid status\n\n## Done when\n\nRun the focused regression tests."
      }
    ]
  }
}"#,
    )
    .unwrap();

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["bundle", "preview", bundle_path.to_str().unwrap()],
    );

    assert!(
        !success,
        "bundle preview should reject unknown workflow status"
    );
    assert!(
        stderr.contains("status is not defined in .atelier/workflow.yaml"),
        "{stderr}"
    );
}

#[test]
fn test_show_issue_rich_human_output() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Target issue",
            "--issue-type",
            "epic",
            "-p",
            "medium",
        ],
    );
    let target_id = issue_ref(dir.path(), 1);
    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Child issue",
            "--parent",
            &target_id,
            "-p",
            "low",
        ],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Blocking issue", "-p", "high"],
    );
    let blocking_id = issue_ref(dir.path(), 3);
    run_atelier(
        dir.path(),
        &["issue", "create", "Downstream issue", "-p", "low"],
    );
    let downstream_id = issue_ref(dir.path(), 4);
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &target_id,
            &blocking_id,
            "--role",
            "blocked_by",
        ],
    );
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &downstream_id,
            &target_id,
            "--role",
            "blocked_by",
        ],
    );
    run_atelier(dir.path(), &["issue", "note", &target_id, "Recent note"]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &target_id]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Target issue"));
    assert!(stdout.contains("Status:   draft"));
    assert!(stdout.contains("Type:"));
    assert!(stdout.contains("Priority: medium"));
    assert!(stdout.contains(&format!(".atelier/issues/{target_id}.md")));
    assert!(stdout.contains("1 total | status: ready=1 | priority: low=1"));
    assert!(stdout.contains("Blocking issue"));
    assert!(stdout.contains("(open blocker)"));
    assert!(stdout.contains("Downstream issue"));
    assert!(stdout.contains("Notes"));
    assert!(stdout.contains("[note] Recent note"));
    assert!(stdout.contains("Next Commands"));
    assert!(stdout.contains("atelier issue note"));
    assert!(!stdout.contains("atelier issue com"));
    assert!(stdout.contains("atelier issue transition"));
}

#[test]
fn test_issue_show_human_shape_exposes_actionable_context() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "JSON issue"]);
    let issue_id = issue_id_by_title(dir.path(), "JSON issue");
    edit_canonical_record(dir.path(), "issues", &issue_id, |text| {
        text.replace("Goal was not specified.", "JSON description")
    });

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("JSON issue"));
    assert!(stdout.contains("Goal"));
    assert!(stdout.contains("JSON description"));
    assert!(stdout.contains("Blocked by"));
    assert!(stdout.contains("Blocking"));
    assert!(stdout.contains("Next Commands"));
}

#[test]
fn test_issue_show_reads_detail_body_from_record_store() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Canonical detail issue"]);
    let issue_id = issue_id_by_title(dir.path(), "Canonical detail issue");
    edit_canonical_record(dir.path(), "issues", &issue_id, |text| {
        text.replace("Goal was not specified.", "Canonical Markdown body")
    });
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Canonical Markdown body"));
}

#[test]
fn test_issue_sections_are_canonical_after_direct_markdown_edit() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Canonical section source"]);
    let issue_id = issue_id_by_title(dir.path(), "Canonical section source");
    let issue_path = dir
        .path()
        .join(".atelier")
        .join("issues")
        .join(format!("{issue_id}.md"));
    let edited_body = "Edited direct Markdown section";
    let edited_outcome = "Direct edits are projected from issue body sections.";
    let edited_evidence = "- `atelier issue show` reads the edited record file.";
    let issue_text = std::fs::read_to_string(&issue_path)
        .unwrap()
        .replace("Goal was not specified.", edited_body)
        .replace(
            "Done when was not specified.",
            &format!("{edited_outcome}\n\n## Context\n\n{edited_evidence}"),
        );
    std::fs::write(&issue_path, issue_text).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains(edited_body), "{stdout}");
    assert!(stdout.contains(edited_outcome), "{stdout}");
    assert!(stdout.contains(edited_evidence), "{stdout}");
}

#[test]
fn test_first_class_detail_views_read_payloads_from_record_store() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Canonical mission",
            "--issue-type",
            "mission",
            "--body",
            "Canonical mission body",
            "--constraint",
            "Canonical constraint",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Canonical mission");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--result",
            "pass",
            "--kind",
            "test",
            "Canonical evidence summary",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    let evidence_id = record_id_by_title(dir.path(), "evidence", "Canonical evidence summary");

    let (success, mission_out, stderr) = run_atelier(dir.path(), &["issue", "show", &mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(mission_out.contains("Canonical mission body"));
    assert!(mission_out.contains("Canonical constraint"));

    let (success, evidence_out, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &evidence_id]);
    assert!(success, "evidence show failed: {stderr}");
    assert!(evidence_out.contains("Canonical evidence summary"));
    assert!(evidence_out.contains("Status:      recorded"));
    assert!(evidence_out.contains("Kind:        test"));
    assert!(!evidence_out.contains("Kind:        sqlite"));

    for args in [
        vec![
            "plan",
            "create",
            "Canonical plan",
            "--body",
            "Canonical plan body",
        ],
        vec!["plan", "show", "atelier-plnn"],
        vec!["plan", "list"],
        vec!["plan", "revise", "atelier-plnn", "body"],
        vec!["plan", "link", "atelier-plnn", "mission", &mission_id],
    ] {
        let (success, _stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(
            !success,
            "removed plan command unexpectedly succeeded: {args:?}"
        );
        assert!(
            stderr.contains("unrecognized subcommand 'plan'"),
            "removed plan command should be rejected by clap: {stderr}"
        );
    }
    assert!(!dir.path().join(".atelier/plans").exists());
}

#[test]
fn test_show_closed_issue_includes_close_reason() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Closed issue"]);
    let issue_id = issue_ref(dir.path(), 1);
    close_issue_with_evidence(dir.path(), &issue_id, Some("Done enough"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);

    assert!(success, "issue show failed: {stderr}");
    assert!(stdout.contains("Closed issue"));
    assert!(stdout.contains("Closed:"));
    assert!(stdout.contains("Close Reason"));
    assert!(stdout.contains("Done enough"));
}

#[test]
fn test_show_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", "999"]);

    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

// ==================== Issue Update Tests ====================

#[test]
fn test_update_issue_title() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Original title"]);
    let issue_id = issue_ref(dir.path(), 1);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "update", &issue_id, "--title", "Updated title"],
    );

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(show_out.contains("Updated title"));
}

#[test]
fn test_update_issue_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue", "-p", "low"]);
    let issue_id = issue_ref(dir.path(), 1);
    run_atelier(
        dir.path(),
        &["issue", "update", &issue_id, "-p", "critical"],
    );

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(show_out.contains("critical"));
}

#[test]
fn test_update_issue_remove_label_replaces_unlabel_helper() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Label lifecycle", "--label", "keep-me"],
    );
    let issue_id = issue_ref(dir.path(), 1);
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &issue_id,
            "--label",
            "remove-me",
            "--remove-label",
            "keep-me",
        ],
    );
    assert!(success, "update label replacement failed: {stderr}");
    assert!(stdout.contains("Updated issue"));

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    let labels_line = show_out
        .lines()
        .find(|line| line.starts_with("Labels:"))
        .unwrap_or("");
    assert!(labels_line.contains("remove-me"), "{show_out}");
    assert!(!labels_line.contains("keep-me"), "{show_out}");
}

// ==================== Issue Close/Reopen Tests ====================

#[test]
fn test_close_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    let issue_id = close_issue_with_evidence(dir.path(), "1", Some("done"));
    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", &issue_id]);

    assert!(show_out.contains("Status:   done"), "{show_out}");
}

// ==================== Issue Delete Tests ====================

// ==================== Labels Tests ====================

#[test]
fn test_add_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    let issue_id = issue_ref(dir.path(), 1);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "update", &issue_id, "--label", "bug"],
    );

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(show_out.contains("bug"));
}

#[test]
fn test_remove_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    let issue_id = issue_ref(dir.path(), 1);
    run_atelier(
        dir.path(),
        &["issue", "update", &issue_id, "--label", "bug"],
    );
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "update", &issue_id, "--remove-label", "bug"],
    );

    assert!(success);

    let issue_id = issue_id_by_title(dir.path(), "Test issue");
    let issue_text = read_canonical_record(dir.path(), "issues", &issue_id);
    assert!(
        issue_text.contains("labels: []"),
        "removed label should not remain in canonical labels:\n{issue_text}"
    );
}

// ==================== Comments Tests ====================

#[test]
fn test_add_comment() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    let issue_id = issue_ref(dir.path(), 1);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "note", &issue_id, "This is a comment"],
    );

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(show_out.contains("This is a comment"));
}

#[test]
fn test_issue_notes_and_close_reason_are_written_to_the_record() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Noted issue"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Noted issue");

    for (kind, body) in [
        ("human", "Human note body"),
        ("plan", "Plan body"),
        ("handoff", "Handoff body"),
    ] {
        let (success, stdout, stderr) = run_atelier(
            dir.path(),
            &["issue", "note", &issue_id, body, "--kind", kind],
        );
        assert!(success, "issue note {kind} failed: {stderr}");
        assert!(
            stdout.contains(&format!("Added note to issue {issue_id}")),
            "{stdout}"
        );
    }
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "note", &issue_id, "Default kind body"],
    );
    assert!(success, "issue note failed: {stderr}");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "note",
            &issue_id,
            "Invalid body",
            "--kind",
            "decision",
        ],
    );
    assert!(!success, "invalid note kind should be rejected");
    assert!(stderr.contains("Invalid note kind 'decision'"), "{stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "note", &issue_id, "   "]);
    assert!(!success, "empty note should be rejected");
    assert!(stderr.contains("must not be empty"), "{stderr}");

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let record = read_canonical_record(dir.path(), "issues", &issue_id);
    assert!(
        record.contains(&format!(
            "## Notes\n\n- {today} [human] Human note body\n- {today} [plan] Plan body\n- {today} [handoff] Handoff body\n- {today} [note] Default kind body"
        )),
        "{record}"
    );
    assert!(!record.contains("Invalid body"), "{record}");

    move_issue_to_review(dir.path(), &issue_id);
    attach_issue_pass_evidence(dir.path(), &issue_id);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &issue_id,
            "close",
            "--reason",
            "Close reason body",
        ],
    );
    assert!(success, "issue transition close failed: {stderr}");

    let record = read_canonical_record(dir.path(), "issues", &issue_id);
    assert!(
        record.contains("## Close Reason\n\nClose reason body"),
        "{record}"
    );
    assert!(!dir
        .path()
        .join(format!(".atelier/issues/{issue_id}.activity"))
        .exists());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Handoff body"), "{stdout}");
    assert!(stdout.contains("Close Reason"), "{stdout}");
    assert!(stdout.contains("Close reason body"), "{stdout}");
}

#[test]
fn test_issue_show_reads_notes_and_close_reason_from_the_record() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Record notes"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Record notes");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "note",
            &issue_id,
            "Canonical comment",
            "--kind",
            "human",
        ],
    );
    assert!(success, "note failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "note", &issue_id, "Canonical handoff"],
    );
    assert!(success, "issue note failed: {stderr}");
    move_issue_to_review(dir.path(), &issue_id);
    attach_issue_pass_evidence(dir.path(), &issue_id);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &issue_id,
            "close",
            "--reason",
            "Canonical close",
        ],
    );
    assert!(success, "close failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Close Reason"));
    assert!(stdout.contains("Canonical close"));
    assert!(stdout.contains("Canonical handoff"));
}

#[test]
fn test_issue_create_is_durable_from_record_files() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Create-only durable",
            "--description",
            "Created body",
            "--priority",
            "high",
        ],
    );
    assert!(success, "create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Create-only durable");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Create-only durable"));
    assert!(stdout.contains("Created body"));
    assert!(stdout.contains("Priority: high"));
}

// ==================== Dependencies Tests ====================

#[test]
fn test_block_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    let blocked_id = issue_ref(dir.path(), 1);
    let blocker_id = issue_ref(dir.path(), 2);
    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &blocked_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );

    assert!(success);

    let (_, blocked_out, _) = run_atelier(dir.path(), &["issue", "show", &blocked_id]);
    assert!(blocked_out.contains("Blocked issue"));
}

#[test]
fn test_work_blocked_lists_directly_blocked_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    let blocked_id = issue_ref(dir.path(), 1);
    let blocker_id = issue_ref(dir.path(), 2);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &blocked_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );
    assert!(success, "issue link failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["work", "blocked"]);
    assert!(success, "work blocked failed: {stderr}");
    assert!(stdout.contains("Blocked issue"));
    assert!(stdout.contains("blocked"));
    assert!(stdout.contains("1 blocker"));
    assert!(!stdout.contains(&format!("blocked by {blocker_id}")));

    let (success, quiet, stderr) = run_atelier(dir.path(), &["--quiet", "work", "blocked"]);
    assert!(success, "quiet work blocked failed: {stderr}");
    assert_eq!(quiet.trim(), blocked_id);
}

#[test]
fn test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Todo task",
            "--label",
            "inventory",
            "--priority",
            "high",
        ],
    );
    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Mission record",
            "--issue-type",
            "mission",
        ],
    );
    run_atelier(dir.path(), &["issue", "create", "Done task"]);
    let done_id = issue_id_by_title(dir.path(), "Done task");
    set_issue_status(dir.path(), &done_id, "done");

    let (success, all, stderr) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success, "issue list failed: {stderr}");
    assert!(all.contains("Issue Inventory"), "{all}");
    assert!(all.contains("Todo task"), "{all}");
    assert!(all.contains("Done task"), "{all}");
    assert!(all.contains("Mission record"), "{all}");
    assert!(!all.contains("Subissues"), "{all}");
    assert!(!all.contains("blocker"), "{all}");

    let (success, todo, stderr) = run_atelier(dir.path(), &["issue", "list", "--status", "todo"]);
    assert!(success, "status filter failed: {stderr}");
    assert!(todo.contains("Todo task"), "{todo}");
    assert!(!todo.contains("Done task"), "{todo}");

    let (success, done, stderr) = run_atelier(dir.path(), &["issue", "list", "--category", "done"]);
    assert!(success, "category filter failed: {stderr}");
    assert!(done.contains("Done task"), "{done}");
    assert!(!done.contains("Todo task"), "{done}");

    let (success, missions, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--issue-type", "mission"]);
    assert!(success, "issue type filter failed: {stderr}");
    assert!(missions.contains("Mission record"), "{missions}");
    assert!(!missions.contains("Todo task"), "{missions}");

    for (flag, value) in [("--label", "inventory"), ("--priority", "high")] {
        let (success, filtered, stderr) = run_atelier(dir.path(), &["issue", "list", flag, value]);
        assert!(success, "{flag} filter failed: {stderr}");
        assert!(filtered.contains("Todo task"), "{filtered}");
        assert!(!filtered.contains("Done task"), "{filtered}");
    }

    let (success, empty, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--label", "does-not-exist"]);
    assert!(success, "empty inventory failed: {stderr}");
    assert!(
        empty.contains("No issues match the selected filters."),
        "{empty}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "list", "--category", "does-not-exist"],
    );
    assert!(!success, "unknown category unexpectedly succeeded");
    assert!(stderr.contains("Invalid issue category"), "{stderr}");

    let (success, quiet, stderr) = run_atelier(
        dir.path(),
        &["--quiet", "issue", "list", "--issue-type", "mission"],
    );
    assert!(success, "quiet issue type filter failed: {stderr}");
    assert_eq!(quiet.lines().count(), 1, "{quiet}");
    assert!(quiet.trim().starts_with("atelier-"), "{quiet}");

    let (success, limited, stderr) =
        run_atelier(dir.path(), &["--quiet", "issue", "list", "--limit", "2"]);
    assert!(success, "limited quiet inventory failed: {stderr}");
    let ids = limited.lines().collect::<Vec<_>>();
    assert_eq!(ids.len(), 2, "{limited}");
    assert!(ids.windows(2).all(|pair| pair[0] < pair[1]), "{limited}");

    for removed in ["--ready", "--blocked"] {
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "list", removed]);
        assert!(!success, "removed {removed} form unexpectedly succeeded");
        assert!(stderr.contains("unexpected argument"), "{stderr}");
    }

    let (success, help, stderr) = run_atelier(dir.path(), &["issue", "list", "--help"]);
    assert!(success, "issue list help failed: {stderr}");
    for supported in [
        "--status",
        "--category",
        "--issue-type",
        "--label",
        "--priority",
        "--limit",
    ] {
        assert!(help.contains(supported), "missing {supported}: {help}");
    }
    assert!(!help.contains("--ready"), "{help}");
    assert!(!help.contains("--blocked"), "{help}");
}

#[test]
fn test_unblock_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    let blocked_id = issue_ref(dir.path(), 1);
    let blocker_id = issue_ref(dir.path(), 2);
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &blocked_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );
    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "unlink",
            &blocked_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );

    assert!(success);

    let (_, blocked_out, _) = run_atelier(dir.path(), &["issue", "show", &blocked_id]);
    assert!(blocked_out.contains("Blocked by"));
    assert!(blocked_out.contains("(none)"));
}

#[test]
fn test_issue_blocker_mutations_are_durable_from_record_files() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Alias blocked"]);
    assert!(success, "blocked create failed: {stderr}");
    let blocked_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Alias blocker"]);
    assert!(success, "blocker create failed: {stderr}");
    let blocker_id = issue_ref(dir.path(), 2);

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &blocked_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );
    assert!(success, "issue link failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &blocked_id]);
    assert!(success, "issue show after issue link failed: {stderr}");
    assert!(stdout.contains(&blocker_id), "{stdout}");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "unlink",
            &blocked_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );
    assert!(success, "issue unlink failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &blocked_id]);
    assert!(success, "issue show after issue unlink failed: {stderr}");
    assert!(
        stdout.contains("Blocked by") && stdout.contains("(none)"),
        "dependency should be removed: {stdout}"
    );
}

#[test]
fn test_ready_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    run_atelier(dir.path(), &["issue", "create", "Ready issue"]);
    let blocked_id = issue_ref(dir.path(), 1);
    let blocker_id = issue_ref(dir.path(), 2);
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &blocked_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["work", "ready"]);

    assert!(success);
    assert!(stdout.contains("Ready issue"));
    assert!(stdout.contains("Blocker issue")); // Blocker is also ready
    assert!(!stdout.contains("Blocked issue"));
}

#[test]
fn test_issue_ready_command_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "ready"]);

    assert!(!success);
    assert!(
        stderr.contains("unrecognized subcommand") || stderr.contains("unexpected argument"),
        "expected clap unknown command error, got: {stderr}"
    );
}

#[test]
fn test_quiet_work_ready_outputs_ids_only() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Ready issue"]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["--quiet", "work", "ready"]);

    assert!(success, "quiet ready list failed: {stderr}");
    assert_eq!(stdout.lines().count(), 1);
    assert!(stdout.lines().all(|line| line.starts_with("atelier-")));
    assert!(!stdout.contains("Ready issue"));
}

#[test]
fn test_work_ready_selects_unblocked_child_without_draft_epic_context() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent epic", "--issue-type", "epic"],
    );
    let parent_id = issue_ref(dir.path(), 1);
    run_atelier(
        dir.path(),
        &["issue", "create", "Ready child", "--parent", &parent_id],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Sequenced child", "--parent", &parent_id],
    );
    let ready_id = issue_ref(dir.path(), 2);
    let sequenced_id = issue_ref(dir.path(), 3);
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &sequenced_id,
            &ready_id,
            "--role",
            "blocked_by",
        ],
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["work", "ready"]);

    assert!(success, "ready list failed: {stderr}");
    assert!(!stdout.contains("Parent epic"));
    assert!(stdout.contains("Ready child"));
    assert!(!stdout.contains("Sequenced child"));
}

#[test]
fn test_work_ready_still_shows_ready_children_when_another_issue_is_active() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent epic", "--issue-type", "epic"],
    );
    let parent_id = issue_ref(dir.path(), 1);
    run_atelier(
        dir.path(),
        &["issue", "create", "Ready child", "--parent", &parent_id],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Sequenced child", "--parent", &parent_id],
    );
    let ready_id = issue_ref(dir.path(), 2);
    let sequenced_id = issue_ref(dir.path(), 3);
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &sequenced_id,
            &ready_id,
            "--role",
            "blocked_by",
        ],
    );
    run_atelier(dir.path(), &["issue", "create", "Active item"]);
    let active_id = issue_ref(dir.path(), 4);

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &active_id, "start"]);
    assert!(success, "start active issue failed: {stderr}");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(
        status_out.contains("Ready work:    1"),
        "status should keep ready work visible while active work is present:\n{status_out}"
    );

    let (success, ready_out, stderr) = run_atelier(dir.path(), &["work", "ready"]);
    assert!(success, "ready list failed: {stderr}");
    // Global ready work excludes the draft epic and retains its ready child.
    assert!(!ready_out.contains("Parent epic"), "{ready_out}");
    assert!(ready_out.contains("Ready child"), "{ready_out}");
    assert!(!ready_out.contains("Sequenced child"), "{ready_out}");
    assert!(!ready_out.contains("Active item"), "{ready_out}");
}

#[test]
fn test_work_blocked_identifies_child_with_external_blocker() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent epic", "--issue-type", "epic"],
    );
    let parent_id = issue_ref(dir.path(), 1);
    run_atelier(
        dir.path(),
        &["issue", "create", "Blocked child", "--parent", &parent_id],
    );
    run_atelier(dir.path(), &["issue", "create", "Outside blocker"]);
    let child_id = issue_ref(dir.path(), 2);
    let blocker_id = issue_ref(dir.path(), 3);
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &child_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["work", "blocked"]);

    assert!(success, "work blocked failed: {stderr}");
    assert!(stdout.contains("Blocked child"));
    assert!(stdout.contains("1 blocker"));
    assert!(!stdout.contains(&format!("blocked by {blocker_id}")));
    assert!(!stdout.contains("open blocker"));
}

#[test]
fn test_issue_update_issue_type_persists() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Container work"]);
    let issue_id = issue_ref(dir.path(), 1);

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", &issue_id, "--issue-type", "epic"],
    );
    assert!(success, "issue type update failed: {stderr}");
    assert!(stdout.contains("Type:     epic"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Container work"));
    assert!(stdout.contains("Type:     epic"));
}

#[test]
fn test_removed_issue_type_is_rejected() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Mutable task"]);
    assert!(success, "baseline issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);

    for removed_type in ["decision", "closeout"] {
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &[
                "issue",
                "create",
                "Artifact task",
                "--issue-type",
                removed_type,
            ],
        );

        assert!(
            !success,
            "removed issue type {removed_type} should be rejected"
        );
        assert!(
            stderr.contains(&format!("must declare issue_type '{removed_type}'")),
            "{stderr}"
        );

        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["issue", "update", &issue_id, "--issue-type", removed_type],
        );

        assert!(
            !success,
            "removed issue type {removed_type} should be rejected on update"
        );
        assert!(
            stderr.contains(&format!("must declare issue_type '{removed_type}'")),
            "{stderr}"
        );
    }
}

#[test]
fn test_work_mission_dashboard_is_scoped_and_operational() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Mission: Dashboard cleanup",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Mission: Dashboard cleanup");
    let mission_path = canonical_issue_path(dir.path(), &mission_id);
    let mission_text = std::fs::read_to_string(&mission_path).unwrap();
    let mission_text = mission_text.replace(
        "status: \"draft\"\n",
        "status: \"draft\"\nstrategy:\n  path: \"docs/strategies/dashboard-cleanup.md\"\n",
    );
    std::fs::write(&mission_path, mission_text).unwrap();
    run_atelier(dir.path(), &["issue", "create", "Ready mission work"]);
    run_atelier(dir.path(), &["issue", "create", "Blocked mission work"]);
    run_atelier(dir.path(), &["issue", "create", "Mission blocker"]);
    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Future mission outcome",
            "--issue-type",
            "epic",
        ],
    );
    let ready_id = issue_id_by_title(dir.path(), "Ready mission work");
    let blocked_id = issue_id_by_title(dir.path(), "Blocked mission work");
    let blocker_id = issue_id_by_title(dir.path(), "Mission blocker");
    let draft_epic_id = issue_id_by_title(dir.path(), "Future mission outcome");
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &mission_id,
            &ready_id,
            "--role",
            "advances",
        ],
    );
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &mission_id,
            &draft_epic_id,
            "--role",
            "advances",
        ],
    );
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &mission_id,
            &blocked_id,
            "--role",
            "advances",
        ],
    );
    run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &blocked_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );

    let (success, dashboard, stderr) = run_atelier(dir.path(), &["work", "mission", &mission_id]);
    assert!(success, "work mission failed: {stderr}");
    assert!(dashboard.contains(&format!("{mission_id} [mission] draft - Dashboard cleanup")));
    assert!(
        dashboard.contains("docs/strategies/dashboard-cleanup.md"),
        "{dashboard}"
    );
    assert!(!dashboard.contains("Work Mission"), "{dashboard}");
    assert!(!dashboard.contains("Proof gaps"), "{dashboard}");
    assert!(!dashboard.contains("Health"), "{dashboard}");
    assert!(!dashboard.contains("Close readiness"), "{dashboard}");
    assert!(!dashboard.contains("Closeout"), "{dashboard}");
    assert!(dashboard.contains("Ready mission work"), "{dashboard}");
    assert!(dashboard.contains("Blocked mission work"), "{dashboard}");
    assert!(dashboard.contains("Backlog Work"), "{dashboard}");
    assert!(dashboard.contains("Future mission outcome"), "{dashboard}");
    assert!(dashboard.contains("[draft]"), "{dashboard}");
    assert!(
        dashboard.contains(&format!("blockers: {blocker_id}")),
        "{dashboard}"
    );
    assert!(
        !dashboard.contains("atelier work queue --all"),
        "{dashboard}"
    );

    let (success, ready, stderr) =
        run_atelier(dir.path(), &["work", "mission", &mission_id, "--ready"]);
    assert!(success, "work mission --ready failed: {stderr}");
    assert!(ready.contains("Ready Work"), "{ready}");
    assert!(ready.contains(&ready_id), "{ready}");
    assert!(!ready.contains(&draft_epic_id), "{ready}");
    assert!(!ready.contains("Blocked mission work"), "{ready}");

    let (success, blocked, stderr) =
        run_atelier(dir.path(), &["work", "mission", &mission_id, "--blocked"]);
    assert!(success, "work mission --blocked failed: {stderr}");
    assert!(!blocked.contains("Ready mission work"), "{blocked}");
    assert!(blocked.contains("Blocked mission work"), "{blocked}");
    assert!(blocked.contains(&blocker_id), "{blocked}");

    set_issue_status(dir.path(), &ready_id, "done");
    set_issue_status(dir.path(), &blocked_id, "done");
    set_issue_status(dir.path(), &blocker_id, "done");
    set_issue_status(dir.path(), &draft_epic_id, "done");
    move_mission_directly_to_ready(dir.path(), &mission_id);
    commit_all(dir.path(), "reviewed dashboard mission");
    let (success, closeout_ready, stderr) =
        run_atelier(dir.path(), &["work", "mission", &mission_id]);
    assert!(success, "closeout-ready work mission failed: {stderr}");
    assert!(!closeout_ready.contains("Publish"), "{closeout_ready}");

    let (success, transition, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id]);
    assert!(success, "issue transition failed: {stderr}");
    assert!(transition.contains("start [allowed]"), "{transition}");
    assert!(!transition.contains("publish"), "{transition}");

    let (success, verbose, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &mission_id, "--verbose"],
    );
    assert!(success, "verbose issue transition failed: {stderr}");
    assert!(verbose.contains("Validators"), "{verbose}");
    assert!(verbose.contains("Description"), "{verbose}");
}

#[test]
fn test_quiet_work_all_includes_every_open_bucket_in_panel_order() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for (title, issue_type) in [
        ("Quiet mission", "mission"),
        ("Quiet epic", "epic"),
        ("Unscoped ready", "task"),
        ("Unscoped active", "task"),
        ("Unscoped blocked", "task"),
        ("Completed history", "task"),
    ] {
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["issue", "create", title, "--issue-type", issue_type],
        );
        assert!(success, "work-all fixture create failed: {stderr}");
    }
    let mission_id = issue_id_by_title(dir.path(), "Quiet mission");
    let epic_id = issue_id_by_title(dir.path(), "Quiet epic");
    let ready_id = issue_id_by_title(dir.path(), "Unscoped ready");
    let active_id = issue_id_by_title(dir.path(), "Unscoped active");
    let blocked_id = issue_id_by_title(dir.path(), "Unscoped blocked");
    let done_id = issue_id_by_title(dir.path(), "Completed history");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Scoped ready", "--parent", &epic_id],
    );
    assert!(success, "scoped fixture create failed: {stderr}");
    let scoped_id = issue_id_by_title(dir.path(), "Scoped ready");
    set_issue_status(dir.path(), &active_id, "in_progress");
    set_issue_status(dir.path(), &blocked_id, "blocked");
    set_issue_status(dir.path(), &done_id, "done");

    let mut ready_ids = vec![ready_id, scoped_id];
    ready_ids.sort();
    let mut backlog_ids = vec![mission_id, epic_id];
    backlog_ids.sort();
    let expected = [vec![active_id], ready_ids, vec![blocked_id], backlog_ids].concat();

    let (success, human, stderr) = run_atelier(dir.path(), &["work", "all"]);
    assert!(success, "human work all failed: {stderr}");
    let human_ids: Vec<_> = human
        .lines()
        .filter_map(|line| {
            expected
                .iter()
                .find(|id| line.contains(id.as_str()))
                .cloned()
        })
        .collect();
    assert_eq!(human_ids, expected, "{human}");
    assert!(!human.contains(&done_id), "{human}");

    let (success, quiet, stderr) = run_atelier(dir.path(), &["--quiet", "work", "all"]);
    assert!(success, "quiet work all failed: {stderr}");
    let quiet_ids: Vec<_> = quiet.lines().map(str::to_owned).collect();
    assert_eq!(quiet_ids, expected, "{quiet}");
    assert_eq!(
        quiet_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len(),
        quiet_ids.len()
    );
}

#[test]
fn test_work_classification_agrees_across_global_mission_and_epic_views() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for args in [
        vec![
            "issue",
            "create",
            "Mission: Classification truth",
            "--issue-type",
            "mission",
        ],
        vec!["issue", "create", "Active outcome", "--issue-type", "epic"],
        vec!["issue", "create", "Retired outcome", "--issue-type", "epic"],
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "objective fixture create failed: {stderr}");
    }
    let mission_id = issue_id_by_title(dir.path(), "Mission: Classification truth");
    let active_epic_id = issue_id_by_title(dir.path(), "Active outcome");
    let retired_epic_id = issue_id_by_title(dir.path(), "Retired outcome");

    for (title, parent) in [
        ("Explicit blocker-free blocked work", &active_epic_id),
        ("Retired open speculation", &retired_epic_id),
        ("Retired blocked speculation", &retired_epic_id),
        ("Retired active speculation", &retired_epic_id),
        ("Retired completed history", &retired_epic_id),
    ] {
        let (success, _, stderr) =
            run_atelier(dir.path(), &["issue", "create", title, "--parent", parent]);
        assert!(success, "child fixture create failed: {stderr}");
    }
    let blocked_id = issue_id_by_title(dir.path(), "Explicit blocker-free blocked work");
    let open_id = issue_id_by_title(dir.path(), "Retired open speculation");
    let retired_blocked_id = issue_id_by_title(dir.path(), "Retired blocked speculation");
    let retired_active_id = issue_id_by_title(dir.path(), "Retired active speculation");
    let done_id = issue_id_by_title(dir.path(), "Retired completed history");

    for epic_id in [&active_epic_id, &retired_epic_id] {
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["issue", "link", &mission_id, epic_id, "--role", "advances"],
        );
        assert!(success, "mission advances link failed: {stderr}");
    }
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--result",
            "pass",
            "--target",
            &format!("issue/{done_id}"),
            "--kind",
            "validation",
            "Completed history remains inspectable",
        ],
    );
    assert!(success, "completed-child evidence record failed: {stderr}");
    let evidence_id = record_id_by_title(
        dir.path(),
        "evidence",
        "Completed history remains inspectable",
    );

    for (issue_id, status) in [
        (&mission_id, "in_progress"),
        (&active_epic_id, "in_progress"),
        (&blocked_id, "blocked"),
        (&retired_epic_id, "superseded"),
        (&open_id, "todo"),
        (&retired_blocked_id, "blocked"),
        (&retired_active_id, "in_progress"),
        (&done_id, "done"),
    ] {
        set_issue_status(dir.path(), issue_id, status);
    }

    let (success, status, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(status.contains("Ready work:    0"), "{status}");
    assert!(status.contains("atelier work blocked"), "{status}");

    let (success, ready, stderr) = run_atelier(dir.path(), &["--quiet", "work", "ready"]);
    assert!(success, "global work ready failed: {stderr}");
    assert!(!ready.lines().any(|id| id == blocked_id), "{ready}");
    assert!(!ready.lines().any(|id| id == open_id), "{ready}");
    let (success, blocked, stderr) = run_atelier(dir.path(), &["--quiet", "work", "blocked"]);
    assert!(success, "global work blocked failed: {stderr}");
    assert!(blocked.lines().any(|id| id == blocked_id), "{blocked}");
    assert!(!blocked.lines().any(|id| id == open_id), "{blocked}");
    assert!(
        !blocked.lines().any(|id| id == retired_blocked_id),
        "{blocked}"
    );

    let (success, mission_ready, stderr) = run_atelier(
        dir.path(),
        &["--quiet", "work", "mission", &mission_id, "--ready"],
    );
    assert!(success, "mission ready view failed: {stderr}");
    assert!(!mission_ready.lines().any(|id| id == blocked_id));
    assert!(!mission_ready.lines().any(|id| id == open_id));
    let (success, mission_blocked, stderr) = run_atelier(
        dir.path(),
        &["--quiet", "work", "mission", &mission_id, "--blocked"],
    );
    assert!(success, "mission blocked view failed: {stderr}");
    assert!(mission_blocked.lines().any(|id| id == blocked_id));
    assert!(!mission_blocked.lines().any(|id| id == open_id));
    assert!(!mission_blocked.lines().any(|id| id == retired_blocked_id));

    let (success, active_epic, stderr) =
        run_atelier(dir.path(), &["work", "epic", &active_epic_id]);
    assert!(success, "active epic view failed: {stderr}");
    assert!(active_epic.contains("Blocked Work"), "{active_epic}");
    assert!(active_epic.contains(&blocked_id), "{active_epic}");
    assert!(!active_epic.contains("Ready Work"), "{active_epic}");

    for issue_id in [&open_id, &retired_blocked_id] {
        let (success, options, stderr) =
            run_atelier(dir.path(), &["issue", "transition", issue_id]);
        assert!(success, "retired child transition options failed: {stderr}");
        assert!(options.contains("start [blocked]"), "{options}");
        assert!(options.contains(&format!("terminal ancestor {retired_epic_id}")));
        assert!(options.contains("cannot enter active-category status 'in_progress'"));
        assert!(options.contains(&format!("atelier issue show {retired_epic_id}")));
    }

    let (success, start_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &open_id, "start"]);
    assert!(
        !success,
        "retired child start should be blocked: stdout:\n{start_out}\nstderr:\n{stderr}"
    );
    for output in [&start_out, &stderr] {
        assert!(
            output.contains(&format!("terminal ancestor {retired_epic_id}")),
            "stdout:\n{start_out}\nstderr:\n{stderr}"
        );
    }

    let workflow_path = dir.path().join(".atelier/workflow.yaml");
    let workflow = std::fs::read_to_string(&workflow_path).unwrap();
    let task_close = "      close:\n        from: [in_progress, review]\n        to: done\n        required_fields: [close_reason]\n        description: Done claim; commit it on the work branch before review.\n        validators:\n        - blockers.none_open\n      supersede:\n        from: [todo, blocked, in_progress]\n";
    assert!(workflow.contains(task_close));
    std::fs::write(
        &workflow_path,
        workflow.replace(
            task_close,
            "      close:\n        from: [todo, in_progress, review]\n        to: done\n        required_fields: [close_reason]\n        description: Retire work without re-entering execution.\n      supersede:\n        from: [todo, blocked, in_progress]\n",
        ),
    )
    .unwrap();
    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &open_id,
            "close",
            "--reason",
            "retired with its ancestor",
        ],
    );
    assert!(
        success,
        "terminal transition beneath terminal ancestor failed: stdout:\n{close_out}\nstderr:\n{stderr}"
    );
    assert!(
        close_out.contains("Applied transition close"),
        "{close_out}"
    );

    for issue_id in [&active_epic_id, &blocked_id] {
        set_issue_status(dir.path(), issue_id, "done");
    }
    let (success, retired_epic, stderr) =
        run_atelier(dir.path(), &["work", "epic", &retired_epic_id]);
    assert!(success, "retired epic view failed: {stderr}");
    assert!(retired_epic.contains("Backlog Work"), "{retired_epic}");
    assert!(retired_epic.contains(&retired_blocked_id), "{retired_epic}");
    assert!(retired_epic.contains(&retired_active_id), "{retired_epic}");
    assert!(retired_epic.contains("Done Work"), "{retired_epic}");
    assert!(retired_epic.contains(&open_id), "{retired_epic}");
    assert!(retired_epic.contains(&done_id), "{retired_epic}");
    assert!(!retired_epic.contains("Ready Work"), "{retired_epic}");

    let (success, retired_show, stderr) =
        run_atelier(dir.path(), &["issue", "show", &retired_epic_id]);
    assert!(success, "retired epic issue show failed: {stderr}");
    assert!(
        retired_show.contains("active 0, ready 0, blocked 0, done 2, backlog 2"),
        "{retired_show}"
    );
    assert!(
        retired_show.contains("linked validating evidence 1"),
        "{retired_show}"
    );
    assert!(
        retired_show.contains("Retired completed history"),
        "{retired_show}"
    );
    let subissues = retired_show
        .split("Subissues\n---------")
        .nth(1)
        .and_then(|output| output.split("\nImpact\n------").next())
        .unwrap();
    assert!(
        subissues.contains("status: backlog=2, done=2"),
        "{subissues}"
    );
    assert!(
        subissues.contains(&format!("shown for context {retired_blocked_id} [backlog]")),
        "{subissues}"
    );
    assert!(!subissues.contains("[blocked]"), "{subissues}");
    assert!(!subissues.contains("[in_progress]"), "{subissues}");

    let (success, evidence_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &evidence_id]);
    assert!(success, "retired descendant evidence show failed: {stderr}");
    assert!(
        evidence_show.contains("Completed history remains inspectable"),
        "{evidence_show}"
    );
}

fn write_incident_issue_type_workflow(dir: &std::path::Path) {
    let workflow_path = dir.join(".atelier/workflow.yaml");
    let workflow = std::fs::read_to_string(&workflow_path)
        .expect("failed to read workflow policy")
        .replace(
            "  validation:\n    label: Validation\n",
            "  validation:\n    label: Validation\n  incident:\n    label: Incident\n",
        )
        .replace("    - task\n", "    - task\n    - incident\n");
    std::fs::write(&workflow_path, workflow).expect("failed to write workflow policy");
}

fn write_workflow_without_mission_issue_type(dir: &std::path::Path) {
    let workflow_path = dir.join(".atelier/workflow.yaml");
    let original = std::fs::read_to_string(&workflow_path).expect("failed to read workflow policy");
    let workflow = original
        .replace("  mission:\n    label: Mission\n", "")
        .replace(
            "    applies_to:\n    - mission\n    - epic\n",
            "    applies_to:\n    - epic\n",
        );
    assert!(!workflow.contains("mission"), "{workflow}");
    std::fs::write(&workflow_path, workflow).expect("failed to write workflow policy");
}

fn set_issue_status(dir: &std::path::Path, issue_id: &str, status: &str) {
    let path = canonical_issue_path(dir, issue_id);
    let text = std::fs::read_to_string(&path).expect("failed to read issue record");
    let updated = text
        .lines()
        .map(|line| {
            if line.starts_with("status: ") {
                format!("status: \"{status}\"")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(path, format!("{updated}\n")).expect("failed to write issue record");
}

// ==================== Session Tests ====================
