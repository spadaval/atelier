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
    assert!(stderr.contains("check_findings: check failed"));
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
fn test_priority_flags_accept_stored_tokens_and_words_alike() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Token priority", "-p", "P1"],
    );
    assert!(success, "-p P1 should create an issue: {stderr}");
    let token_id = issue_ref(dir.path(), 1);
    let markdown = read_canonical_record(dir.path(), "issues", &token_id);
    assert!(
        markdown.contains("priority: \"P1\"\n"),
        "-p P1 should store P1:\n{markdown}"
    );
    let (_, show, _) = run_atelier(dir.path(), &["issue", "show", &token_id]);
    assert!(
        show.lines()
            .next()
            .unwrap()
            .contains(" high Token priority"),
        "views print the word:\n{show}"
    );

    run_atelier(
        dir.path(),
        &["issue", "create", "Word priority", "-p", "high"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Low priority", "-p", "low"],
    );
    let low_id = issue_ref(dir.path(), 3);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", &low_id, "--priority", "P0"],
    );
    assert!(success, "--priority P0 should update: {stderr}");
    let markdown = read_canonical_record(dir.path(), "issues", &low_id);
    assert!(
        markdown.contains("priority: \"P0\"\n"),
        "--priority P0 should store P0:\n{markdown}"
    );

    let (success, by_word, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--priority", "high"]);
    assert!(success, "{stderr}");
    let (success, by_token, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--priority", "P1"]);
    assert!(success, "{stderr}");
    assert_eq!(by_word, by_token);
    assert!(by_token.contains("Token priority") && by_token.contains("Word priority"));
    assert!(!by_token.contains("Low priority"));

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Bad", "-p", "P4"]);
    assert!(!success);
    assert!(
        stderr.contains("Invalid priority 'P4'. Valid values: P0 or critical, P1 or high, P2 or medium, P3 or low"),
        "{stderr}"
    );
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
    assert!(!stdout.contains("--description"), "{stdout}");
    assert!(stdout.contains("--body <BODY>"), "{stdout}");
    assert!(stdout.contains("--issue-type <ISSUE_TYPE>"), "{stdout}");
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
    let (success, show, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);
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
        &["issue", "create", "Parent issue", "--issue-type", "feature"],
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

    // Any non-mission issue can be a parent.
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Task parent"]);
    assert!(success, "task parent create failed: {stderr}");
    let parent_id = issue_id_by_title(dir.path(), "Task parent");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Task child", "--parent", &parent_id],
    );
    assert!(success, "a task can parent work: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Task child");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Mission child",
            "--issue-type",
            "mission",
            "--parent",
            &parent_id,
        ],
    );
    assert!(!success, "mission child should be rejected");
    assert!(
        stderr.contains("workflow_issue_hierarchy_invalid")
            && stderr.contains("mission issue")
            && stderr.contains("cannot have parent"),
        "{stderr}"
    );

    // Parents are one level deep.
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Grandchild", "--parent", &child_id],
    );
    assert!(!success, "grandchild should be rejected");
    assert!(stderr.contains("parents are one level deep"), "{stderr}");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Mission", "--issue-type", "mission"],
    );
    assert!(success, "{stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Mission");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Under a mission",
            "--parent",
            &mission_id,
        ],
    );
    assert!(
        !success && stderr.contains("cannot be child of mission"),
        "{stderr}"
    );
}

#[test]
fn test_issue_update_and_lint_reject_invalid_hierarchy_shapes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Parent", "--issue-type", "feature"],
    );
    assert!(success, "parent create failed: {stderr}");
    let parent_id = issue_id_by_title(dir.path(), "Parent");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Child work", "--parent", &parent_id],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Child work");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Other"]);
    assert!(success, "{stderr}");
    let other_id = issue_id_by_title(dir.path(), "Other");

    // A parent stays a parent of any work type, but cannot become a mission
    // or a child itself.
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", &parent_id, "--issue-type", "task"],
    );
    assert!(success, "a parent may be any work type: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", &parent_id, "--parent", &other_id],
    );
    assert!(!success, "a parent cannot become a child");
    assert!(stderr.contains("parents are one level deep"), "{stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", &other_id, "--parent", &child_id],
    );
    assert!(
        !success && stderr.contains("parents are one level deep"),
        "{stderr}"
    );
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", &parent_id, "--issue-type", "mission"],
    );
    assert!(
        !success && stderr.contains("missions are not parents"),
        "a parent cannot become a mission: {stderr}"
    );
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(
        success,
        "the refused retype left the records valid: {stdout}{stderr}"
    );

    // Bundle preview enforces the same one-level rule as apply.
    let bundle = dir.path().join("nested.json");
    for issues in [
        r###"[{"client_ref":"a","title":"A","body":"## Goal\n\nA.\n\n## Done when\n\nA."},{"client_ref":"b","title":"B","body":"## Goal\n\nB.\n\n## Done when\n\nB.","parent":{"client_ref":"a"}},{"client_ref":"c","title":"C","body":"## Goal\n\nC.\n\n## Done when\n\nC.","parent":{"client_ref":"b"}}]"###.to_string(),
        format!(r###"[{{"client_ref":"d","title":"D","body":"## Goal\n\nD.\n\n## Done when\n\nD.","parent":{{"id":"{child_id}"}}}}]"###),
    ] {
        fs::write(&bundle, format!(r###"{{"schema":"atelier.bundle","schema_version":2,"title":"Nested","resources":{{"issues":{issues}}}}}"###)).unwrap();
        let (success, _, stderr) = run_atelier(dir.path(), &["bundle", "preview", bundle.to_str().unwrap()]);
        assert!(!success && stderr.contains("parents are one level deep"), "{stderr}");
    }

    // A hand-edited record that nests a child under a child fails check.
    edit_canonical_issue(dir.path(), &child_id, |markdown| {
        markdown.replace(
            "  children: []",
            &format!("  children:\n  - kind: \"issue\"\n    id: \"{other_id}\""),
        )
    });
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(!success, "lint should reject a nested hierarchy");
    let transcript = format!("{stdout}\n{stderr}");
    assert!(
        transcript.contains("workflow_issue_hierarchy_invalid")
            && transcript.contains("one level deep"),
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

    let (success, show_out, stderr) =
        run_atelier(dir.path(), &["issue", "show", &mission_id, "--full"]);
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
        &["issue", "create", "Issue 1", "--issue-type", "feature"],
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
        vec!["issue", "show", &issue_id, "--full"],
    ] {
        let (success, stdout, stderr) =
            run_atelier_with_env(dir.path(), &args, &[("NO_COLOR", "1")]);
        assert!(success, "{args:?} failed: {stderr}");
        assert!(
            !stdout.contains("\u{1b}["),
            "NO_COLOR/non-terminal output must not contain ANSI escapes:\n{stdout}"
        );
        assert!(
            stdout.contains("\nNext\n----\n")
                || stdout.contains("\nCommands:\n")
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
        &[
            "issue",
            "create",
            "Complete scope",
            "--issue-type",
            "feature",
        ],
    );
    assert!(success, "{stderr}");
    let owner = issue_id_by_title(dir.path(), "Complete scope");
    for number in 0..7 {
        let title = format!("Scoped ready {number}");
        let (success, _, stderr) =
            run_atelier(dir.path(), &["issue", "create", &title, "--parent", &owner]);
        assert!(success, "{stderr}");
    }
    let (success, shown, stderr) = run_atelier(dir.path(), &["issue", "show", &owner, "--full"]);
    assert!(success, "{stderr}");
    let rollup = shown.split("Parent Rollup").nth(1).unwrap();
    assert!(
        rollup.contains("Scope         : 7 issues: 7 ready\n"),
        "{rollup}"
    );
    let ready_rows = rollup
        .split("Ready Work")
        .nth(1)
        .unwrap()
        .split("Recently Updated")
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
        &["issue", "create", "Parent epic", "--issue-type", "feature"],
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

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &parent_id, "--full"]);
    assert!(success, "issue show failed: {stderr}");
    let subissues = stdout.split("Subissues\n").nth(1).expect("Subissues panel");
    let subissues = subissues.split("Impact\n").next().unwrap();
    for title in [
        "Contract child",
        "Implementation child",
        "External blocked child",
    ] {
        assert!(
            subissues.contains(title),
            "missing child in Subissues: {subissues}"
        );
    }
    let contract_pos = stdout.find("Contract child").unwrap_or(usize::MAX);
    let implementation_pos = stdout.find("Implementation child").unwrap_or(usize::MAX);
    assert!(
        contract_pos < implementation_pos,
        "visible blocker child should appear before dependent child:\n{stdout}"
    );
    // Subissues show each child's work state, marked ⚠ when blocked.
    assert!(
        subissues.contains(&format!("\n  {contract_id} ready medium")),
        "{subissues}"
    );
    assert!(
        subissues.contains(&format!("\n  {implementation_id} ⚠ blocked medium")),
        "{subissues}"
    );
    assert!(
        subissues.contains(&format!(
            "\n  {external_child_id} ⚠ blocked medium External blocked child | 1 blocker"
        )),
        "{subissues}"
    );
    // The rollup's state panels leave the state to their heading.
    let blocked = stdout
        .split("Blocked Work\n")
        .nth(1)
        .expect("Blocked Work panel");
    assert!(
        blocked.contains(&format!("\n  {implementation_id} ⚠ todo medium")),
        "{stdout}"
    );
    assert!(!stdout.contains(&external_blocker_id), "{stdout}");
    assert!(stdout.contains("\nNext\n----\n"));
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

/// A mission, a parent advancing it with two children, and a blocker on the
/// first child. Returns (mission, parent, first child, second child, blocker).
fn brief_show_fixture(dir: &std::path::Path) -> (String, String, String, String, String) {
    let create = |args: &[&str]| {
        let (success, stdout, stderr) = run_atelier(dir, args);
        assert!(success, "{args:?} failed: {stderr}");
        stdout.trim().to_string()
    };
    let mission = create(&[
        "-q",
        "issue",
        "create",
        "Brief mission",
        "--issue-type",
        "mission",
        "--body",
        "## Outcome\n\nAgents read one view.\n\n## Success criteria\n\n- S1: brief by default.\n- S2: several IDs.\n\n## Background\n\nTranscripts.",
    ]);
    let parent = create(&[
        "-q",
        "issue",
        "create",
        "Brief parent",
        "--body",
        "## Goal\n\nParent goal.\n\n## Done when\n\nParent done.",
    ]);
    create(&[
        "-q", "issue", "link", &parent, &mission, "--role", "advances",
    ]);
    let child = create(&[
        "-q",
        "issue",
        "create",
        "First child",
        "--parent",
        &parent,
        "--body",
        "## Goal\n\nChild goal.\n\n## Done when\n\nChild done.\n\n## Context\n\nOnly in the full view.",
    ]);
    let sibling = create(&[
        "-q",
        "issue",
        "create",
        "Second child",
        "--parent",
        &parent,
        "--body",
        "## Goal\n\nSibling goal.\n\n## Done when\n\nSibling done.",
    ]);
    let blocker = create(&[
        "-q",
        "issue",
        "create",
        "Blocker",
        "--body",
        "## Goal\n\nBlocker goal.\n\n## Done when\n\nBlocker done.",
    ]);
    create(&["-q", "issue", "link", &blocker, &child, "--role", "blocks"]);
    (mission, parent, child, sibling, blocker)
}

fn show_ok(dir: &std::path::Path, args: &[&str]) -> String {
    let (success, stdout, stderr) = run_atelier(dir, args);
    assert!(success, "{args:?} failed: {stderr}");
    stdout
}

#[test]
fn test_issue_show_is_brief_by_default() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (mission, parent, child, sibling, blocker) = brief_show_fixture(dir.path());

    assert_eq!(
        show_ok(dir.path(), &["issue", "show", &child]),
        format!(
            "{child} task todo medium First child\n\
             Mission: {mission} draft medium Brief mission\n\
             Parent: {parent} todo medium Brief parent\n\
             Blocked by: {blocker} todo medium Blocker\n\
             Transitions: block, defer, supersede --reason | blocked: start - task {child} has direct blocker path {child} -> {blocker}\n\
             Goal: Child goal.\n\
             Done when: Child done.\n\
             More with --full: Context\n"
        )
    );
    assert_eq!(
        show_ok(dir.path(), &["issue", "show", &parent]),
        format!(
            "{parent} task todo medium Brief parent\n\
             Mission: {mission} draft medium Brief mission\n\
             Children:\n  \
             {sibling} ready medium Second child\n  \
             {child} ⚠ blocked medium First child\n\
             Transitions: block, defer, start, supersede --reason\n\
             Goal: Parent goal.\n\
             Done when: Parent done.\n"
        )
    );
    // A mission shows its scope counts and its Outcome and Success criteria.
    assert_eq!(
        show_ok(dir.path(), &["issue", "show", &mission]),
        format!(
            "{mission} mission draft medium Brief mission\n\
             Work: 3 issues: 1 ready, 1 context, 1 blocked\n\
             Transitions: ready, supersede --reason\n\
             Outcome: Agents read one view.\n\
             Success criteria:\n  \
             - S1: brief by default.\n  \
             - S2: several IDs.\n\
             More with --full: Background\n"
        )
    );

    // A parent's blockers reach its children, and a done blocker is marked.
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &blocker, &parent, "--role", "blocks"],
    );
    assert!(success, "link failed: {stderr}");
    edit_canonical_record(dir.path(), "issues", &blocker, |text| {
        text.replace("status: \"todo\"", "status: \"done\"")
    });
    let sibling_view = show_ok(dir.path(), &["issue", "show", &sibling]);
    assert!(
        sibling_view.contains(&format!(
            "\nBlocked via parent {parent}: {blocker} ✓ done medium Blocker\n"
        )),
        "{sibling_view}"
    );
    let child_view = show_ok(dir.path(), &["issue", "show", &child]);
    assert!(
        child_view.contains(&format!("\nBlocked by: {blocker} ✓ done medium Blocker\n")),
        "{child_view}"
    );
    assert!(
        child_view.contains("\nTransitions: block, defer, start, supersede --reason\n"),
        "{child_view}"
    );

    // A transition that needs an argument and has another gap stays blocked,
    // with the other gap as its reason: a parent closes after its children.
    edit_canonical_record(dir.path(), "issues", &parent, |text| {
        text.replace("status: \"todo\"", "status: \"in_progress\"")
    });
    let parent_view = show_ok(dir.path(), &["issue", "show", &parent]);
    assert!(
        parent_view.contains(
            "\nTransitions: block, request_review, supersede --reason | blocked: close - validator work.terminal failed: open work: "
        ),
        "{parent_view}"
    );

    for view in [&sibling_view, &child_view] {
        for full_only in [
            "Transition Readiness",
            "\nNext\n----\n",
            "Impact",
            "Priority:",
        ] {
            assert!(!view.contains(full_only), "{full_only} in brief:\n{view}");
        }
    }
}

#[test]
fn test_issue_show_brief_decision_leads_with_question_and_decision() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let decision = show_ok(
        dir.path(),
        &[
            "-q",
            "issue",
            "create",
            "Pick a format",
            "--issue-type",
            "decision",
            "--body",
            "## Question\n\nWhich format?\n\n## Options\n\nA or B.\n\n## Decision\n\nA.",
        ],
    );
    let decision = decision.trim();
    let view = show_ok(dir.path(), &["issue", "show", decision]);
    assert!(
        view.ends_with("Question: Which format?\nDecision: A.\nMore with --full: Options\n"),
        "{view}"
    );

    // An open decision waits on the human, named the same way as a child row
    // and in a mission's work counts.
    let create = |args: &[&str]| show_ok(dir.path(), args).trim().to_string();
    let parent = create(&[
        "-q",
        "issue",
        "create",
        "Format parent",
        "--body",
        "## Goal\n\nG.\n\n## Done when\n\nD.",
    ]);
    let open = create(&[
        "-q",
        "issue",
        "create",
        "Open format question",
        "--issue-type",
        "decision",
        "--parent",
        &parent,
        "--body",
        "## Question\n\nWhich?",
    ]);
    let mission = create(&[
        "-q",
        "issue",
        "create",
        "Format mission",
        "--issue-type",
        "mission",
        "--body",
        "## Outcome\n\nO.\n\n## Success criteria\n\nS.",
    ]);
    create(&[
        "-q", "issue", "link", &parent, &mission, "--role", "advances",
    ]);
    let parent_view = show_ok(dir.path(), &["issue", "show", &parent]);
    assert!(
        parent_view.contains(&format!(
            "\nChildren: {open} needs human medium Open format question\n"
        )),
        "{parent_view}"
    );
    let mission_view = show_ok(dir.path(), &["issue", "show", &mission]);
    assert!(
        mission_view.contains("\nWork: 2 issues: 1 needs a human, 1 context\n"),
        "{mission_view}"
    );
}

#[test]
fn test_issue_show_brief_done_items_show_their_close_but_open_items_report_failures() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let id = show_ok(
        dir.path(),
        &[
            "-q",
            "issue",
            "create",
            "Finished work",
            "--body",
            "## Goal\n\nFinish.\n\n## Done when\n\nFinished.",
        ],
    );
    let id = id.trim();
    // A done record stands its close in for transitions; this one has no
    // Close Reason, so the line is the date alone.
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    for status in ["done", "superseded"] {
        edit_canonical_record(dir.path(), "issues", id, |text| {
            text.lines()
                .map(|line| {
                    if line.starts_with("status: ") {
                        format!("status: \"{status}\"")
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
                + "\n"
        });
        assert_eq!(
            show_ok(dir.path(), &["issue", "show", id]),
            format!(
                "{id} task ✓ {status} medium Finished work\n\
                 Closed {today}\n\
                 Goal: Finish.\n\
                 Done when: Finished.\n"
            )
        );
    }

    // On open work, a failure to read the transitions is reported, not
    // shown as none.
    edit_canonical_record(dir.path(), "issues", id, |text| {
        replace_front_matter_scalar(&text, "status", "todo").replacen(
            "priority:",
            "workflow_branch:\n  branch_base: gone\n  work_branch: feature/gone\npriority:",
            1,
        )
    });
    let view = show_ok(dir.path(), &["issue", "show", id]);
    assert!(
        view.contains("\nTransitions: unavailable - git rev-parse --verify refs/heads/gone"),
        "{view}"
    );
}

#[test]
fn test_issue_show_takes_several_ids_and_resolves_all_before_printing() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (mission, parent, child, _, _) = brief_show_fixture(dir.path());

    let brief = |id: &str| show_ok(dir.path(), &["issue", "show", id]);
    assert_eq!(
        show_ok(dir.path(), &["issue", "show", &child, &parent, &mission]),
        format!("{}\n{}\n{}", brief(&child), brief(&parent), brief(&mission))
    );

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "show", &child, "atelier-zzzz", &parent],
    );
    assert!(!success, "an unknown ID should fail the whole show");
    assert_eq!(stdout, "", "nothing prints before every ID resolves");
    assert!(stderr.contains("issue atelier-zzzz not found"), "{stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show"]);
    assert!(!success, "show needs at least one ID");
    assert!(stderr.contains("<IDS>..."), "{stderr}");
}

#[test]
fn test_issue_show_full_gives_the_whole_record_for_each_id() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (_, parent, child, _, _) = brief_show_fixture(dir.path());

    let full = |id: &str| show_ok(dir.path(), &["issue", "show", id, "--full"]);
    let child_full = full(&child);
    for section in [
        "Category:",
        "Hierarchy",
        "Transition Readiness",
        "Context\n-------\nOnly in the full view.",
        "Blocked by",
        "\nNext\n----\n",
    ] {
        assert!(
            child_full.contains(section),
            "{section} missing:\n{child_full}"
        );
    }
    assert_eq!(
        show_ok(dir.path(), &["issue", "show", "--full", &child, &parent]),
        format!("{child_full}\n{}", full(&parent))
    );
}

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

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);

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
    assert!(transitions.contains("expected at least 1 fresh passing evidence record; found 0"));
    assert!(
        transitions.contains(&format!("Hint: Record fresh proof with `atelier evidence record --target issue/{issue_id} --kind test -- <command>`")),
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
            "feature",
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

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            parent_key,
            related_key,
            "--role",
            "related",
        ],
    );
    assert!(success, "relate by partial keys failed: {stderr}");
    assert!(stdout.contains(&parent_id));
    assert!(stdout.contains(&related_id));

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", parent_key, "--full"]);
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
        "blocked_by": [{{ "id": "{issue_key}" }}]
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
    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &dependent_id, "--full"]);
    assert!(success, "issue show failed: {stderr}");
    assert!(stdout.contains(&issue_id));
    assert!(
        stdout.starts_with(&format!("{dependent_id} task todo ")),
        "{stdout}"
    );
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

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);
    assert!(success, "custom issue show failed: {stderr}");
    assert!(
        stdout.starts_with(&format!("{issue_id} incident in_progress ")),
        "{stdout}"
    );
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
    assert!(stdout.contains("Created mission atelier-"), "{stdout}");
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

    let (success, show, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);
    assert!(success, "declared mission issue show failed: {stderr}");
    assert!(
        show.starts_with(&format!("{issue_id} mission draft ")),
        "declared mission should use its configured initial status: {show}"
    );
}

#[test]
fn test_issue_create_mission_sectioned_body_preserves_outcome_and_evidence() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let outcome = "Mission authoring preserves the supplied outcome.";
    let evidence = "- `atelier check` accepts the created mission record.";
    let body =
        format!("## Outcome\n\nMission result.\n\n## Success criteria\n\n{outcome}\n\n{evidence}");
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
    assert!(mission.contains("## Outcome\n\nMission result."));
    assert!(mission.contains(&format!("## Success criteria\n\n{outcome}\n\n{evidence}")));

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
    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);
    assert!(success, "custom bundled issue show failed: {stderr}");
    assert!(
        stdout.starts_with(&format!("{issue_id} incident todo ")),
        "{stdout}"
    );
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
    assert!(stderr.contains("decision"), "{stderr}");
}

#[test]
fn test_issue_type_update_rejects_incompatible_existing_status_atomically() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Draft mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Draft mission");

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
    assert!(
        issue_text.contains("issue_type: \"mission\""),
        "{issue_text}"
    );
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
        "blocked_by": [
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
            "feature",
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

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &target_id, "--full"]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Target issue"));
    // The title line states type, status and priority; no line repeats them.
    assert!(stdout.starts_with(&format!("{target_id} feature todo medium Target issue\n")));
    assert!(!stdout.contains("Status:"));
    assert!(!stdout.contains("Type:"));
    assert!(!stdout.contains("Priority:"));
    assert!(stdout.contains(&format!(".atelier/issues/{target_id}.md")));
    // The subissue inherits its parent's open blocker.
    assert!(
        stdout.contains("1 total | status: blocked=1 | priority: low=1"),
        "{stdout}"
    );
    assert!(stdout.contains("Blocking issue"));
    assert!(stdout.contains(" | open blocker\n"));
    assert!(stdout.contains("Downstream issue"));
    assert!(stdout.contains("Notes"));
    assert!(stdout.contains("[note] Recent note"));
    assert!(stdout.contains("\nNext\n----\n"));
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

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("JSON issue"));
    assert!(stdout.contains("Goal"));
    assert!(stdout.contains("JSON description"));
    assert!(stdout.contains("Blocked by"));
    assert!(stdout.contains("Blocking"));
    assert!(stdout.contains("\nNext\n----\n"));
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

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);
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
            "## Outcome\n\nCanonical mission body\n\n## Success criteria\n\nCanonical constraint",
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
    assert!(evidence_out.starts_with(&format!("{evidence_id} evidence recorded ")));
    assert!(!evidence_out.contains("Status:"));
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

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);

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

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);
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

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);
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
    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);

    assert!(
        show_out.starts_with(&format!("{issue_id} task ✓ done ")),
        "{show_out}"
    );
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

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);
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

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);
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

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);
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

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);
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
            "--body",
            "Created body",
            "--priority",
            "high",
        ],
    );
    assert!(success, "create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Create-only durable");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Create-only durable"));
    assert!(stdout.contains("Created body"));
    assert!(stdout.starts_with(&format!("{issue_id} task todo high ")));
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
    assert!(
        stdout.contains(&format!(
            "Blocked Work\n------------\n  {blocked_id} ⚠ todo medium Blocked issue | 1 blocker\n"
        )),
        "{stdout}"
    );
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
    assert!(
        stderr.contains(
            "Invalid category 'does-not-exist'. Valid values: active, blocked, done, todo"
        ),
        "{stderr}"
    );

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
    // Help names the repository workflow's categories and statuses.
    assert!(
        help.contains("[possible values: active, blocked, done, todo]"),
        "{help}"
    );
    assert!(help.contains("in_progress, review"), "{help}");
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

    let (_, blocked_out, _) = run_atelier(dir.path(), &["issue", "show", &blocked_id, "--full"]);
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

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &blocked_id, "--full"]);
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

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &blocked_id, "--full"]);
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
fn test_work_ready_selects_unblocked_child_and_shows_its_parent_as_context() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent epic", "--issue-type", "feature"],
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
    // The parent shows only as context, after the ready work.
    let (ready_part, context) = stdout.split_once("Parents and Missions").expect(&stdout);
    assert!(!ready_part.contains("Parent epic") && context.contains("Parent epic"));
    assert!(stdout.contains("Ready child"));
    assert!(!stdout.contains("Sequenced child"));
}

#[test]
fn test_work_ready_still_shows_ready_children_when_another_issue_is_active() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent epic", "--issue-type", "feature"],
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
    // Global ready work shows the parent only as context and keeps its ready child.
    let ready_part = ready_out.split("Parents and Missions").next().unwrap();
    assert!(!ready_part.contains("Parent epic"), "{ready_out}");
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
        &["issue", "create", "Parent epic", "--issue-type", "feature"],
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
        &["issue", "update", &issue_id, "--issue-type", "feature"],
    );
    assert!(success, "issue type update failed: {stderr}");
    assert!(stdout.contains("Type:     feature"));

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "show", &issue_id, "--full"]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Container work"));
    assert!(stdout.starts_with(&format!("{issue_id} feature ")));
}

#[test]
fn test_removed_issue_type_is_rejected() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Mutable task"]);
    assert!(success, "baseline issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);

    for removed_type in ["closeout"] {
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
            "feature",
            "--status",
            "backlog",
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
    assert!(dashboard.contains(&format!(
        "{mission_id} mission draft medium Dashboard cleanup"
    )));
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
    assert!(
        dashboard.contains(" backlog medium Future mission outcome"),
        "{dashboard}"
    );
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
    assert!(transition.contains("start: allowed"), "{transition}");
    assert!(!transition.contains("publish"), "{transition}");

    let (success, verbose, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &mission_id, "--verbose"],
    );
    assert!(success, "verbose issue transition failed: {stderr}");
    assert!(verbose.contains("Validators"), "{verbose}");
    assert!(verbose.contains("Description"), "{verbose}");
}

/// ADR 0022: one line per item, no state the section heading already names,
/// ✓ and ⚠ next to the status word, and a leading state only where a list
/// mixes states.
#[test]
fn test_work_views_render_one_dense_line_per_item() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let create = |args: &[&str]| {
        let mut command = vec!["--quiet", "issue", "create"];
        command.extend_from_slice(args);
        let (success, stdout, stderr) = run_atelier(dir.path(), &command);
        assert!(success, "create {args:?} failed: {stderr}");
        stdout.trim().to_string()
    };
    let mission = create(&["Dense mission", "--issue-type", "mission"]);
    let active = create(&["Active item"]);
    let ready = create(&["Ready item"]);
    let blocked = create(&["Blocked item"]);
    let done = create(&["Done item"]);
    let backlog = create(&["Backlog item", "--status", "backlog"]);
    let human = create(&["Human item", "--label", "needs-human"]);
    let parent = create(&["Parent feature", "--issue-type", "feature"]);
    let child = create(&["Child task", "--parent", &parent]);
    let unassigned = create(&["Unassigned blocked", "--label", "dense"]);
    for id in [&active, &ready, &blocked, &done, &backlog, &human, &parent] {
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["issue", "link", &mission, id, "--role", "advances"],
        );
        assert!(success, "advances link failed: {stderr}");
    }
    for (id, blocker) in [(&blocked, &ready), (&unassigned, &ready)] {
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["issue", "link", id, blocker, "--role", "blocked_by"],
        );
        assert!(success, "blocker link failed: {stderr}");
    }
    set_issue_status(dir.path(), &active, "in_progress");
    set_issue_status(dir.path(), &done, "done");
    let section = |output: &str, title: &str| -> Vec<String> {
        output
            .split(&format!("\n{title}\n{}\n", "-".repeat(title.len())))
            .nth(1)
            .unwrap_or_else(|| panic!("no {title} section:\n{output}"))
            .split("\n\n")
            .next()
            .unwrap()
            .lines()
            .map(str::to_string)
            .collect()
    };
    let row = |id: &str, status: &str, title: &str| format!("  {id} {status} medium {title}");
    let blocked_row = format!(
        "{} | blockers: {ready} | 1 blocker",
        row(&blocked, "⚠ todo", "Blocked item")
    );

    let (success, summary, stderr) = run_atelier(dir.path(), &["work", "mission", &mission]);
    assert!(success, "work mission failed: {stderr}");
    assert!(
        summary.starts_with(&format!("{mission} mission draft medium Dense mission\n")),
        "{summary}"
    );
    // The title line names the status, so no Status line repeats it.
    assert!(!summary.contains("Status:"), "{summary}");
    assert!(!summary.contains("shown for context"), "{summary}");
    assert_eq!(
        section(&summary, "Active Work"),
        [row(&active, "in_progress", "Active item")]
    );
    let mut ready_rows = section(&summary, "Ready Work");
    ready_rows.sort();
    let mut expected_ready = vec![
        row(&ready, "todo", "Ready item"),
        row(&child, "todo", "Child task"),
    ];
    expected_ready.sort();
    assert_eq!(ready_rows, expected_ready);
    assert_eq!(
        section(&summary, "Needs a Human"),
        [row(&human, "todo", "Human item")]
    );
    assert_eq!(
        section(&summary, "Parents and Missions"),
        [row(&parent, "todo", "Parent feature")]
    );
    assert_eq!(
        section(&summary, "Blocked Work"),
        std::slice::from_ref(&blocked_row)
    );
    assert_eq!(
        section(&summary, "Backlog Work"),
        [row(&backlog, "backlog", "Backlog item")]
    );
    assert_eq!(
        section(&summary, "Done Work"),
        [row(&done, "✓ done", "Done item")]
    );

    // Unfiltered, the list mixes states, so a row leads with its state when
    // its status word does not already say it.
    let (success, scoped, stderr) =
        run_atelier(dir.path(), &["work", "mission", &mission, "--all"]);
    assert!(success, "work mission --all failed: {stderr}");
    let mut scoped_rows = section(&scoped, "Scoped Work");
    scoped_rows.sort();
    let mut expected_scoped = vec![
        format!(
            "  active{}",
            &row(&active, "in_progress", "Active item")[1..]
        ),
        format!("  ready{}", &row(&ready, "todo", "Ready item")[1..]),
        format!("  ready{}", &row(&child, "todo", "Child task")[1..]),
        format!("  needs human{}", &row(&human, "todo", "Human item")[1..]),
        format!("  context{}", &row(&parent, "todo", "Parent feature")[1..]),
        format!("  blocked{}", &blocked_row[1..]),
        row(&backlog, "backlog", "Backlog item"),
        row(&done, "✓ done", "Done item"),
    ];
    expected_scoped.sort();
    assert_eq!(scoped_rows, expected_scoped, "{scoped}");

    let (success, all, stderr) = run_atelier(dir.path(), &["work", "all"]);
    assert!(success, "work all failed: {stderr}");
    assert!(!all.contains("shown for context"), "{all}");
    assert_eq!(
        section(&all, "Active Work"),
        [row(&active, "in_progress", "Active item")]
    );
    assert_eq!(
        section(&all, "Needs a Human"),
        [row(&human, "todo", "Human item")]
    );
    // A draft mission is backlog, not context.
    assert_eq!(
        section(&all, "Parents and Missions (context)"),
        [row(&parent, "todo", "Parent feature")]
    );
    let mut blocked_rows = section(&all, "Blocked Work");
    blocked_rows.sort();
    let mut expected_blocked = vec![
        row(&blocked, "⚠ todo", "Blocked item") + " | 1 blocker",
        row(&unassigned, "⚠ todo", "Unassigned blocked") + " | 1 blocker",
    ];
    expected_blocked.sort();
    assert_eq!(blocked_rows, expected_blocked, "{all}");
    let backlog_rows = section(&all, "Backlog Work");
    assert!(
        backlog_rows.contains(&row(&backlog, "backlog", "Backlog item")),
        "{all}"
    );
    assert!(
        backlog_rows.contains(&row(&mission, "draft", "Dense mission")),
        "{all}"
    );

    let (success, parent_view, stderr) = run_atelier(dir.path(), &["work", "parent", &parent]);
    assert!(success, "work parent failed: {stderr}");
    // The title line states the status, so no Status line repeats it.
    assert!(
        parent_view.starts_with(&format!("{parent} feature todo medium Parent feature\n")),
        "{parent_view}"
    );
    assert!(!parent_view.contains("Status"), "{parent_view}");
    assert_eq!(
        section(&parent_view, "Ready Work"),
        [row(&child, "todo", "Child task")]
    );
    assert_eq!(
        section(&parent_view, "Parents and Missions"),
        [row(&parent, "todo", "Parent feature")]
    );

    let (success, status, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    // The heading names the active category; the row keeps the status.
    assert_eq!(
        section(&status, "Current Work"),
        [row(&active, "in_progress", "Active item")]
    );
    assert!(
        section(&status, "Ready Work").contains(&row(&ready, "todo", "Ready item")),
        "{status}"
    );
    assert!(
        status.contains(&format!(
            "\n  {unassigned} ⚠ todo medium Unassigned blocked | 1 blocker\n"
        )),
        "{status}"
    );
    assert!(!status.contains("shown for context"), "{status}");
}

#[test]
fn test_quiet_work_all_includes_every_open_bucket_in_panel_order() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for (title, issue_type) in [
        ("Quiet mission", "mission"),
        ("Quiet epic", "feature"),
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
    // The parent with an open child is context; the draft mission is backlog.
    let expected = [
        vec![active_id],
        ready_ids,
        vec![epic_id],
        vec![blocked_id],
        vec![mission_id],
    ]
    .concat();

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
fn test_removed_mission_generation_flags_are_unknown_on_create_and_update() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let body = "## Outcome\n\nTyped mission body.\n\n## Success criteria\n\nObserved result.";
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Typed mission",
            "--issue-type",
            "mission",
            "--body",
            body,
        ],
    );
    assert!(success, "typed mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Typed mission");
    let path = canonical_issue_path(dir.path(), &mission_id);
    let original = std::fs::read(&path).unwrap();
    for flag in ["--constraint", "--risk", "--validation"] {
        for args in [
            vec![
                "issue",
                "create",
                "Obsolete generated mission",
                "--issue-type",
                "mission",
                flag,
                "obsolete",
            ],
            vec!["issue", "update", &mission_id, flag, "obsolete"],
        ] {
            let (success, stdout, stderr) = run_atelier_raw(dir.path(), &args);
            assert!(!success, "{args:?} unexpectedly succeeded: {stdout}");
            assert!(
                stderr.contains(&format!("unexpected argument '{flag}'")),
                "{stderr}"
            );
        }
    }
    assert_eq!(std::fs::read(path).unwrap(), original);
    assert_eq!(
        std::fs::read_dir(dir.path().join(".atelier/issues"))
            .unwrap()
            .count(),
        1
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
        vec![
            "issue",
            "create",
            "Active outcome",
            "--issue-type",
            "feature",
        ],
        vec![
            "issue",
            "create",
            "Retired outcome",
            "--issue-type",
            "feature",
        ],
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
    // A count line with nothing to count is left out.
    assert!(!status.contains("Ready work:"), "{status}");
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
        run_atelier(dir.path(), &["work", "parent", &active_epic_id]);
    assert!(success, "active epic view failed: {stderr}");
    assert!(active_epic.contains("Blocked Work"), "{active_epic}");
    assert!(active_epic.contains(&blocked_id), "{active_epic}");
    assert!(!active_epic.contains("Ready Work"), "{active_epic}");

    for issue_id in [&open_id, &retired_blocked_id] {
        let (success, options, stderr) =
            run_atelier(dir.path(), &["issue", "transition", issue_id]);
        assert!(success, "retired child transition options failed: {stderr}");
        assert!(options.contains("start: blocked"), "{options}");
        assert!(options.contains(&format!("closed ancestor {retired_epic_id}")));
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
            output.contains(&format!("closed ancestor {retired_epic_id}")),
            "stdout:\n{start_out}\nstderr:\n{stderr}"
        );
    }

    let workflow_path = dir.path().join(".atelier/workflow.yaml");
    let workflow = std::fs::read_to_string(&workflow_path).unwrap();
    let task_close = "      close:\n        from: [in_progress, review]\n        to: done\n        required_fields: [close_reason]\n        description: Done claim; commit it on the work branch before review. A parent closes after its children.\n        validators:\n        - blockers.none_open\n        - work.terminal\n      supersede:\n        from: [backlog, todo, blocked, in_progress]\n";
    assert!(workflow.contains(task_close));
    std::fs::write(
        &workflow_path,
        workflow.replace(
            task_close,
            "      close:\n        from: [todo, in_progress, review]\n        to: done\n        required_fields: [close_reason]\n        description: Retire work without re-entering execution.\n      supersede:\n        from: [backlog, todo, blocked, in_progress]\n",
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
        run_atelier(dir.path(), &["work", "parent", &retired_epic_id]);
    assert!(success, "retired epic view failed: {stderr}");
    assert!(retired_epic.contains("Backlog Work"), "{retired_epic}");
    assert!(retired_epic.contains(&retired_blocked_id), "{retired_epic}");
    assert!(retired_epic.contains(&retired_active_id), "{retired_epic}");
    assert!(retired_epic.contains("Done Work"), "{retired_epic}");
    assert!(retired_epic.contains(&open_id), "{retired_epic}");
    assert!(retired_epic.contains(&done_id), "{retired_epic}");
    assert!(!retired_epic.contains("Ready Work"), "{retired_epic}");

    let (success, retired_show, stderr) =
        run_atelier(dir.path(), &["issue", "show", &retired_epic_id, "--full"]);
    assert!(success, "retired epic issue show failed: {stderr}");
    assert!(
        retired_show.contains("Scope         : 4 issues: 2 done, 2 backlog\n"),
        "{retired_show}"
    );
    assert!(
        retired_show.contains("Evidence Gates: 1 validating evidence record linked,"),
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
        subissues.contains(&format!("\n  {retired_blocked_id} backlog")),
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
        .replace("    - feature\n    - spike\n", "    - feature\n")
        .replace(
            "    applies_to:\n    - mission\n",
            "    applies_to:\n    - spike\n",
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

#[test]
fn test_retired_template_rejected_and_explicit_creation_preserved() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    for option in ["--template", "-t"] {
        let (success, _, stderr) =
            run_atelier_raw(dir.path(), &["issue", "create", "Obsolete", option, "bug"]);
        assert!(!success);
        assert!(stderr.contains("unexpected argument"), "{stderr}");
        assert_eq!(
            std::fs::read_dir(dir.path().join(".atelier/issues"))
                .unwrap()
                .count(),
            0
        );
    }
    let body = "## Goal\n\nRepair the observable defect.\n\n## Done when\n\nThe regression passes.";
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Explicit bug",
            "--issue-type",
            "bug",
            "--body",
            body,
            "--priority",
            "high",
            "--label",
            "regression",
        ],
    );
    assert!(success, "{stderr}");
    let id = issue_id_by_title(dir.path(), "Explicit bug");
    let record = read_canonical_record(dir.path(), "issues", &id);
    for text in [
        "issue_type: \"bug\"",
        "priority: \"P1\"",
        "regression",
        "Repair the observable defect.",
        "The regression passes.",
    ] {
        assert!(record.contains(text), "missing {text}: {record}");
    }
    let (success, _, stderr) = run_atelier(dir.path(), &["check", &id]);
    assert!(success, "{stderr}");
}

#[test]
fn test_scoped_dashboard_counts_and_quiet_rows_exceed_display_limit() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Count mission",
            "--issue-type",
            "mission",
        ],
    );
    let mission = issue_id_by_title(dir.path(), "Count mission");
    run_atelier(
        dir.path(),
        &["issue", "create", "Count epic", "--issue-type", "feature"],
    );
    let epic = issue_id_by_title(dir.path(), "Count epic");
    run_atelier(
        dir.path(),
        &["issue", "link", &epic, &mission, "--role", "advances"],
    );
    let mut ids = Vec::new();
    for n in 0..24 {
        let title = format!("Count child {n:02}");
        let (success, _, stderr) =
            run_atelier(dir.path(), &["issue", "create", &title, "--parent", &epic]);
        assert!(success, "{stderr}");
        ids.push(issue_id_by_title(dir.path(), &title));
    }
    ids.sort();
    // The mission's scope holds the parent; the parent's scope is its
    // children, though its view still lists the parent as a row.
    for (kind, id, context, size) in [
        ("mission", &mission, 1, "25 issues: 24 ready, 1 context"),
        ("parent", &epic, 0, "24 issues: 24 ready"),
    ] {
        let (success, quiet, stderr) = run_atelier(dir.path(), &["--quiet", "work", kind, id]);
        assert!(success, "{stderr}");
        assert!(quiet.contains("ready=24"), "{quiet}");
        assert!(quiet.contains(&format!("context={context}")), "{quiet}");
        let (success, human, stderr) = run_atelier(dir.path(), &["work", kind, id]);
        assert!(success, "{stderr}");
        assert!(human.contains(&format!(": {size}\n")), "{human}");
        assert!(human.contains("16 more"), "{human}");
        assert!(
            human.contains(&format!(
                "Parents and Missions\n--------------------\n  {epic} "
            )),
            "{human}"
        );
    }
    // `work parent` and `issue show` give the parent one size.
    let (success, shown, stderr) = run_atelier(dir.path(), &["issue", "show", &epic, "--full"]);
    assert!(success, "{stderr}");
    assert!(
        shown.contains("Scope         : 24 issues: 24 ready\n"),
        "{shown}"
    );
    let (success, quiet, stderr) = run_atelier(
        dir.path(),
        &["--quiet", "work", "mission", &mission, "--ready"],
    );
    assert!(success, "{stderr}");
    let quiet = quiet.lines().collect::<Vec<_>>();
    let mut every = quiet.clone();
    every.sort_unstable();
    assert_eq!(every, ids.iter().map(String::as_str).collect::<Vec<_>>());
    // Quiet rows come in the work order the human view lists.
    let (success, human, stderr) =
        run_atelier(dir.path(), &["work", "mission", &mission, "--ready"]);
    assert!(success, "{stderr}");
    let shown = human
        .lines()
        .filter_map(|line| line.trim_start().split_whitespace().next())
        .filter(|token| ids.iter().any(|id| id == token))
        .collect::<Vec<_>>();
    assert_eq!(quiet, shown, "{human}");
}

#[test]
fn test_inventory_combines_custom_status_category_and_real_metadata_before_limiting() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let policy = dir.path().join(".atelier/workflow.yaml");
    std::fs::write(
        &policy,
        std::fs::read_to_string(&policy)
            .unwrap()
            .replace("in_progress", "working"),
    )
    .unwrap();
    let mut matching = Vec::new();
    for (n, kind, label, priority, status) in [
        (0, "bug", "cli", "high", "working"),
        (1, "bug", "cli", "high", "working"),
        (2, "bug", "docs", "high", "working"),
        (3, "task", "cli", "high", "working"),
        (4, "bug", "cli", "low", "todo"),
    ] {
        let title = format!("Unicode Δ inventory {n}");
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &[
                "issue",
                "create",
                &title,
                "--issue-type",
                kind,
                "--label",
                label,
                "--priority",
                priority,
            ],
        );
        assert!(success, "{stderr}");
        let id = issue_id_by_title(dir.path(), &title);
        set_issue_status(dir.path(), &id, status);
        if n < 2 {
            matching.push(id);
        }
    }
    matching.sort();
    let filters = [
        "--status",
        "working",
        "--category",
        "active",
        "--issue-type",
        "bug",
        "--label",
        "cli",
        "--priority",
        "high",
    ];
    let mut args = vec!["--quiet", "issue", "list"];
    args.extend(filters);
    let (success, output, stderr) = run_atelier(dir.path(), &args);
    assert!(success, "{stderr}");
    assert_eq!(
        output.lines().collect::<Vec<_>>(),
        matching.iter().map(String::as_str).collect::<Vec<_>>()
    );
    args.extend(["--limit", "1"]);
    let (success, output, stderr) = run_atelier(dir.path(), &args);
    assert!(success, "{stderr}");
    assert_eq!(output.trim(), matching[0]);
    args.remove(0);
    let (success, output, stderr) = run_atelier(dir.path(), &args);
    assert!(success, "{stderr}");
    assert!(output.contains("Showing 1 of 2 issues"), "{output}");
    assert!(output.contains("Unicode Δ inventory"), "{output}");
    assert!(output.contains("raise --limit"), "{output}");
}

#[test]
fn test_status_footer_uses_complete_work_buckets() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    run_atelier(dir.path(), &["issue", "create", "Status guidance"]);
    let id = issue_id_by_title(dir.path(), "Status guidance");
    for (status, expected) in [
        ("todo", "Choose ready work (1 ready issue available)"),
        (
            "blocked",
            "Inspect blocked work (no ready work is available)",
        ),
        ("done", ""),
        ("in_progress", ""),
    ] {
        set_issue_status(dir.path(), &id, status);
        let (success, output, stderr) = run_atelier(dir.path(), &["status"]);
        assert!(success, "{stderr}");
        if expected.is_empty() {
            // With no action to offer, status prints no footer.
            assert!(!output.contains("\nNext\n----\n"), "{output}");
        } else {
            assert!(
                output.contains(&format!("\nNext\n----\n  {expected}: ")),
                "{output}"
            );
        }
        if status == "in_progress" {
            assert!(!output.contains("Current work:"), "{output}");
            assert!(
                output.contains(&format!(
                    "\nCurrent Work\n------------\n  {id} in_progress "
                )),
                "{output}"
            );
            assert!(output.contains("Status guidance"), "{output}");
        }
    }
}

#[test]
fn new_records_use_the_configured_slug_and_legacy_ids_stay_valid() {
    let parent = tempdir().unwrap();
    let dir = parent.path().join("astromech");
    fs::create_dir(&dir).unwrap();
    init_git_repo(&dir);
    let (success, _, stderr) = run_atelier(&dir, &["init"]);
    assert!(success, "{stderr}");
    let config = fs::read_to_string(dir.join(".atelier/config.toml")).unwrap();
    assert!(config.contains("project_slug = \"astromech\""), "{config}");

    // A record minted under the old fixed slug remains valid alongside new IDs.
    let (success, legacy, stderr) = run_atelier(
        &dir,
        &[
            "-q",
            "issue",
            "create",
            "Legacy",
            "--body",
            "## Goal\n\nOld.\n\n## Done when\n\nKept.",
        ],
    );
    assert!(success, "{stderr}");
    let legacy = legacy.trim().to_string();
    assert!(legacy.starts_with("astromech-"), "{legacy}");
    let renamed = legacy.replacen("astromech-", "atelier-", 1);
    let text = fs::read_to_string(canonical_issue_path(&dir, &legacy))
        .unwrap()
        .replace(&legacy, &renamed);
    fs::remove_file(canonical_issue_path(&dir, &legacy)).unwrap();
    fs::write(canonical_issue_path(&dir, &renamed), text).unwrap();

    let (success, created, stderr) = run_atelier(
        &dir,
        &[
            "-q",
            "issue",
            "create",
            "Slugged",
            "--body",
            "## Goal\n\nNew.\n\n## Done when\n\nSlugged.",
        ],
    );
    assert!(success, "{stderr}");
    let created = created.trim().to_string();
    assert!(created.starts_with("astromech-"), "{created}");

    let bundle = dir.join("bundle.json");
    fs::write(
        &bundle,
        r###"{"schema":"atelier.bundle","schema_version":2,"title":"Slug","resources":{"issues":[{"client_ref":"one","title":"Bundled","issue_type":"task","body":"## Goal\n\nBundle.\n\n## Done when\n\nSlugged."}]}}"###,
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(
        &dir,
        &["bundle", "apply", bundle.to_str().unwrap(), "--yes"],
    );
    assert!(success, "{stderr}");
    let bundled = issue_id_by_title(&dir, "Bundled");
    assert!(bundled.starts_with("astromech-"), "{bundled}");

    let (success, _, stderr) = run_atelier(
        &dir,
        &["issue", "link", &created, &renamed, "--role", "blocked_by"],
    );
    assert!(success, "{stderr}");
    let (success, stdout, stderr) = run_atelier(&dir, &["check"]);
    assert!(success, "{stdout}{stderr}");
    let (success, shown, stderr) = run_atelier(&dir, &["issue", "show", &renamed]);
    assert!(success && shown.contains("Legacy"), "{shown}{stderr}");
}

#[test]
fn advances_joins_a_mission_and_work_the_same_way_from_bundle_and_cli() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let create = |title: &str, issue_type: &str, body: &str| {
        let (success, stdout, stderr) = run_atelier(
            dir.path(),
            &[
                "-q",
                "issue",
                "create",
                title,
                "--issue-type",
                issue_type,
                "--body",
                body,
            ],
        );
        assert!(success, "{stderr}");
        stdout.trim().to_string()
    };
    let mission = create(
        "Mission",
        "mission",
        "## Outcome\n\nShip.\n\n## Success criteria\n\n- S1: shipped.",
    );
    let task = create("Task", "task", "## Goal\n\nWork.\n\n## Done when\n\nDone.");
    let other = create("Other", "task", "## Goal\n\nWork.\n\n## Done when\n\nDone.");
    let relates_to = |id: &str, target: &str| {
        read_canonical_record(dir.path(), "issues", id).contains(&format!(
            "  - kind: \"issue\"\n    id: \"{target}\"\n    type: \"advances\""
        ))
    };

    // CLI: the record that declares the link stores it, and the mission
    // record is left unchanged (atelier-ihss).
    let mission_before = read_canonical_record(dir.path(), "issues", &mission);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &task, &mission, "--role", "advances"],
    );
    assert!(success, "{stderr}");
    assert!(relates_to(&task, &mission));
    assert_eq!(
        read_canonical_record(dir.path(), "issues", &mission),
        mission_before
    );
    let (success, overview, stderr) = run_atelier(dir.path(), &["work", "mission", &mission]);
    assert!(success && overview.contains(&task), "{overview}{stderr}");
    // CLI: two non-missions cannot advance each other.
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &task, &other, "--role", "advances"],
    );
    assert!(
        !success && stderr.contains("advances links one mission"),
        "{stderr}"
    );
    assert!(!relates_to(&task, &other));

    // Bundle: an epic declares the mission it advances, as the CLI allows.
    let bundle = dir.path().join("advances.json");
    fs::write(
        &bundle,
        format!(
            r###"{{"schema":"atelier.bundle","schema_version":2,"title":"Advances","resources":{{"issues":[{{"client_ref":"epic","title":"Bundled epic","issue_type":"feature","body":"## Goal\n\nPart.\n\n## Done when\n\nShown.","advances":[{{"id":"{mission}"}}]}}]}}}}"###
        ),
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["bundle", "apply", bundle.to_str().unwrap(), "--yes"],
    );
    assert!(success, "{stderr}");
    let epic = issue_id_by_title(dir.path(), "Bundled epic");
    assert!(relates_to(&epic, &mission));
    assert_eq!(
        read_canonical_record(dir.path(), "issues", &mission),
        mission_before
    );
    let (success, overview, stderr) = run_atelier(dir.path(), &["work", "mission", &mission]);
    assert!(success && overview.contains(&epic), "{overview}{stderr}");

    // Bundle: declaring one link from both endpoints is a duplicate edge.
    fs::write(
        &bundle,
        r###"{"schema":"atelier.bundle","schema_version":2,"title":"Both","resources":{"issues":[{"client_ref":"m","title":"Both mission","issue_type":"mission","body":"## Outcome\n\nShip.\n\n## Success criteria\n\n- S1: shipped.","advances":[{"client_ref":"e"}]},{"client_ref":"e","title":"Both epic","issue_type":"feature","body":"## Goal\n\nPart.\n\n## Done when\n\nShown.","advances":[{"client_ref":"m"}]}]}}"###,
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["bundle", "apply", bundle.to_str().unwrap(), "--yes"],
    );
    assert!(
        !success && stderr.contains("Duplicate bundle relationship"),
        "{stderr}"
    );

    // Bundle: work-to-work advances is refused before writing.
    fs::write(
        &bundle,
        format!(
            r###"{{"schema":"atelier.bundle","schema_version":2,"title":"Bad","resources":{{"issues":[{{"client_ref":"bad","title":"Bad epic","issue_type":"feature","body":"## Goal\n\nPart.\n\n## Done when\n\nShown.","advances":[{{"id":"{task}"}}]}}]}}}}"###
        ),
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["bundle", "apply", bundle.to_str().unwrap(), "--yes"],
    );
    assert!(
        !success && stderr.contains("advances links one mission"),
        "{stderr}"
    );
    let (success, list, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success && !list.contains("Bad epic"), "{list}");
}

#[test]
fn links_leave_the_shared_mission_and_blocker_unchanged_and_read_from_either_side() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let create = |title: &str, issue_type: &str, body: &str| {
        let (success, stdout, stderr) = run_atelier(
            dir.path(),
            &[
                "-q",
                "issue",
                "create",
                title,
                "--issue-type",
                issue_type,
                "--body",
                body,
            ],
        );
        assert!(success, "{stderr}");
        stdout.trim().to_string()
    };
    let task_body = "## Goal\n\nWork.\n\n## Done when\n\nDone.";
    let mission = create(
        "Shared mission",
        "mission",
        "## Outcome\n\nShip.\n\n## Success criteria\n\n- S1: shipped.",
    );
    let blocker = create("Shared blocker", "task", task_body);
    let work = create("New work", "task", task_body);
    let bytes = |id: &str| fs::read(canonical_record_path(dir.path(), "issues", id)).unwrap();
    let (mission_before, blocker_before) = (bytes(&mission), bytes(&blocker));
    let run = |args: &[&str]| {
        let (success, stdout, stderr) = run_atelier(dir.path(), args);
        assert!(success, "{args:?}: {stdout}{stderr}");
        stdout
    };

    run(&["issue", "link", &work, &mission, "--role", "advances"]);
    run(&["issue", "link", &work, &blocker, "--role", "blocked_by"]);
    assert_eq!(bytes(&mission), mission_before);
    assert_eq!(bytes(&blocker), blocker_before);

    // The other records see the links at read time.
    let shown = run(&["issue", "show", &blocker, "--full"]);
    let blocking = &shown[shown.find("Blocking").unwrap()..];
    assert!(
        blocking[..blocking.find("Linked Issues").unwrap()].contains(&work),
        "{shown}"
    );
    let shown = run(&["issue", "show", &mission, "--full"]);
    assert!(shown.contains(&format!("advances {work} todo")), "{shown}");
    let shown = run(&["issue", "show", &work, "--full"]);
    let linked = &shown[shown.find("Linked Issues").unwrap()..];
    assert!(
        linked.contains(&format!("advances {mission}")) && !linked.contains("blocked_by"),
        "{shown}"
    );
    assert!(run(&["work", "blocked"]).contains(&work));
    assert!(run(&["work", "mission", &mission]).contains(&work));
    run(&["check"]);

    // The same edge declared from the other record already exists, and
    // unlinking from either side removes it from the record that stores it.
    assert!(run(&["issue", "link", &blocker, &work, "--role", "blocks"]).contains("already exists"));
    assert!(
        run(&["issue", "link", &mission, &work, "--role", "advances"]).contains("already exists")
    );
    assert_eq!(bytes(&mission), mission_before);
    assert_eq!(bytes(&blocker), blocker_before);
    run(&["issue", "unlink", &blocker, &work, "--role", "blocks"]);
    run(&["issue", "unlink", &mission, &work, "--role", "advances"]);
    let stored = read_canonical_record(dir.path(), "issues", &work);
    assert!(stored.contains("  relates: []\n"), "{stored}");
    assert!(run(&["work", "ready"]).contains(&work));
}

#[test]
fn test_someday_work_is_created_in_backlog_and_kept_out_of_ready() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let workflow = fs::read_to_string(dir.path().join(".atelier/workflow.yaml")).unwrap();
    assert!(
        workflow.contains("  backlog:\n    category: todo"),
        "{workflow}"
    );

    let (success, someday, stderr) = run_atelier(
        dir.path(),
        &[
            "-q",
            "issue",
            "create",
            "Someday idea",
            "--status",
            "backlog",
        ],
    );
    assert!(success, "{stderr}");
    let someday = someday.trim().to_string();
    let (success, near, _) = run_atelier(dir.path(), &["-q", "issue", "create", "Near-term"]);
    assert!(success);
    let near = near.trim().to_string();
    let record = fs::read_to_string(canonical_issue_path(dir.path(), &someday)).unwrap();
    assert!(record.contains("status: \"backlog\""), "{record}");

    let ready = |dir: &Path| run_atelier(dir, &["--quiet", "work", "ready"]).1;
    let listed = ready(dir.path());
    assert!(listed.lines().any(|line| line == near), "{listed}");
    assert!(!listed.lines().any(|line| line == someday), "{listed}");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "transition", &someday, "plan"]);
    assert!(success, "{stderr}");
    assert!(ready(dir.path()).lines().any(|line| line == someday));
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "transition", &someday, "defer"]);
    assert!(success, "{stderr}");
    assert!(!ready(dir.path()).lines().any(|line| line == someday));

    // An objective's todo is behind the checked ready transition.
    for issue_type in ["mission"] {
        let (success, stdout, stderr) = run_atelier(
            dir.path(),
            &[
                "issue",
                "create",
                "Unready objective",
                "--issue-type",
                issue_type,
                "--status",
                "todo",
            ],
        );
        assert!(!success, "{issue_type}: {stdout}");
        assert!(stderr.contains("use one of: draft."), "{stderr}");
    }
    for refused in ["in_progress", "done", "nonsense"] {
        let (success, stdout, stderr) = run_atelier(
            dir.path(),
            &["issue", "create", "Bad start", "--status", refused],
        );
        assert!(!success, "{refused}: {stdout}");
        assert!(
            stderr.contains("issue_create_status_invalid") && stderr.contains("backlog, todo"),
            "{stderr}"
        );
    }
}

#[test]
fn test_issue_options_do_what_their_help_says() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let create = |args: &[&str]| {
        let (success, stdout, stderr) = run_atelier(dir.path(), args);
        assert!(success, "{stderr}");
        stdout.trim().to_string()
    };
    let task = create(&["-q", "issue", "create", "Task", "--body", "Plain goal text"]);
    let parent = create(&["-q", "issue", "create", "Parent", "--issue-type", "feature"]);

    // --body revises the typed sections of any type and keeps notes.
    create(&["-q", "issue", "note", &task, "Keep this note"]);
    create(&[
        "issue",
        "update",
        &task,
        "--body",
        "## Goal\n\nRevised goal.\n\n## Done when\n\nRevised proof.\n\n## Context\n\nWhy.",
    ]);
    let record = fs::read_to_string(canonical_issue_path(dir.path(), &task)).unwrap();
    assert!(
        record.contains("Revised goal.")
            && record.contains("## Context")
            && record.contains("Keep this note"),
        "{record}"
    );
    create(&[
        "issue",
        "update",
        &parent,
        "--body",
        "## Goal\n\nRevised outcome.\n\n## Done when\n\nA scenario.\n\n## Unknowns\n\nOne question.",
    ]);
    let record = fs::read_to_string(canonical_issue_path(dir.path(), &parent)).unwrap();
    assert!(
        record.contains("Revised outcome.") && record.contains("## Unknowns"),
        "{record}"
    );
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &task,
            "--body",
            "## Outcome\n\nWrong shape.",
        ],
    );
    assert!(
        !success && stderr.contains("Invalid sectioned --body"),
        "{stderr}"
    );
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &task,
            "--body",
            "## Goal\n\nG.\n\n## Done when\n\nD.\n\n## Notes\n\nSneaky.",
        ],
    );
    assert!(
        !success && stderr.contains("atelier issue note"),
        "{stderr}"
    );
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "update", &task, "--status", "done"]);
    assert!(
        !success && stderr.contains("unexpected argument"),
        "{stderr}"
    );
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "X", "--description", "d"]);
    assert!(
        !success && stderr.contains("unexpected argument"),
        "{stderr}"
    );

    // blocks is the inverse of blocked_by, and -q prints the first ID given.
    let waiting = create(&["-q", "issue", "create", "Waits"]);
    assert_eq!(
        create(&["-q", "issue", "link", &task, &waiting, "--role", "blocks"]),
        task
    );
    let ready = run_atelier(dir.path(), &["--quiet", "work", "ready"]).1;
    assert!(!ready.lines().any(|line| line == waiting), "{ready}");
    create(&[
        "-q",
        "issue",
        "unlink",
        &waiting,
        &task,
        "--role",
        "blocked_by",
    ]);
    let ready = run_atelier(dir.path(), &["--quiet", "work", "ready"]).1;
    assert!(ready.lines().any(|line| line == waiting), "{ready}");
    assert_eq!(
        create(&["-q", "issue", "transition", &waiting, "block"]),
        "blocked"
    );

    // Revised bodies follow the record rules check applies, and done records
    // keep the acceptance their proof was judged against.
    let mission = create(&[
        "-q",
        "issue",
        "create",
        "Mission",
        "--issue-type",
        "mission",
        "--body",
        "## Outcome\n\nShipped.\n\n## Success criteria\n\nS1.",
    ]);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &mission,
            "--body",
            "## Outcome\n\nNo criteria any more.",
        ],
    );
    assert!(
        !success && stderr.contains("Mission requires Success criteria"),
        "{stderr}"
    );
    let huge = format!("## Goal\n\n{}\n\n## Done when\n\nD.", "x".repeat(70 * 1024));
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "update", &task, "--body", &huge]);
    assert!(!success && stderr.contains("maximum length"), "{stderr}");
    let finished = create(&["-q", "issue", "create", "Finished"]);
    create(&["-q", "issue", "transition", &finished, "start"]);
    create(&[
        "-q",
        "issue",
        "transition",
        &finished,
        "close",
        "--reason",
        "Done",
    ]);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &finished,
            "--body",
            "## Goal\n\nG.\n\n## Done when\n\nD.",
        ],
    );
    assert!(!success && stderr.contains("issue_body_done"), "{stderr}");

    let bundle = dir.path().join("old.json");
    fs::write(&bundle, r###"{"schema":"atelier.bundle","schema_version":2,"resources":{"issues":[{"client_ref":"a","title":"A","body":"## Goal\n\nG.\n\n## Done when\n\nD.","depends_on":[]}]}}"###).unwrap();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["bundle", "preview", bundle.to_str().unwrap()]);
    assert!(
        !success && stderr.contains("depends_on is now blocked_by"),
        "{stderr}"
    );

    let (_, help, _) = run_atelier(dir.path(), &["issue", "link", "--help"]);
    assert!(
        help.contains("Readiness: blocked_by") && help.contains("Context only"),
        "{help}"
    );
    let (_, help, _) = run_atelier(dir.path(), &["issue", "create", "--help"]);
    assert!(!help.contains("imported source"), "{help}");
    let (_, help, _) = run_atelier(dir.path(), &["issue", "transition", "--help"]);
    assert!(help.contains("atelier start <id>"), "{help}");
}

#[test]
fn test_mission_strategy_is_set_from_the_cli_and_checked_from_the_mission_side() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    fs::create_dir_all(dir.path().join("docs/strategies")).unwrap();
    fs::write(
        dir.path().join("docs/strategies/plan.md"),
        "---\nstatus: draft\n---\n\n# Plan\n",
    )
    .unwrap();
    let (success, mission, stderr) = run_atelier(
        dir.path(),
        &[
            "-q",
            "issue",
            "create",
            "Ship it",
            "--issue-type",
            "mission",
            "--strategy",
            "docs/strategies/plan.md",
        ],
    );
    assert!(success, "{stderr}");
    let mission = mission.trim().to_string();
    let strategy_path =
        |dir: &Path| fs::read_to_string(canonical_issue_path(dir, &mission)).unwrap();
    assert!(
        strategy_path(dir.path()).contains("strategy:\n  path: docs/strategies/plan.md\n"),
        "{}",
        strategy_path(dir.path())
    );
    // The strategy need not name its mission.
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(success, "{stdout}{stderr}");

    // A strategy that names another mission disagrees with the canonical link.
    fs::write(
        dir.path().join("docs/strategies/plan.md"),
        "---\nmission: atelier-zzzz\n---\n\n# Plan\n",
    )
    .unwrap();
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(!success, "{stdout}");
    assert!(
        format!("{stdout}{stderr}").contains("front matter names mission atelier-zzzz"),
        "{stdout}{stderr}"
    );

    // Moving the link is one edit on the mission.
    fs::write(dir.path().join("docs/strategies/next.md"), "# Next\n").unwrap();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &mission,
            "--strategy",
            "docs/strategies/next.md",
        ],
    );
    assert!(success, "{stderr}");
    assert!(strategy_path(dir.path()).contains(
        "path: docs/strategies/next.md
"
    ));
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(success, "{stdout}{stderr}");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &mission,
            "--strategy",
            "docs/strategies/missing.md",
        ],
    );
    assert!(!success && stderr.contains("does not exist"), "{stderr}");

    // A blank key names no mission, as when the strategy is written first.
    fs::write(
        dir.path().join("docs/strategies/next.md"),
        "---\nmission:\n---\n\n# Next\n",
    )
    .unwrap();
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(success, "{stdout}{stderr}");

    // Two missions sharing a strategy can only be satisfied without the key.
    let (success, other, stderr) = run_atelier(
        dir.path(),
        &[
            "-q",
            "issue",
            "create",
            "Sibling",
            "--issue-type",
            "mission",
            "--strategy",
            "docs/strategies/next.md",
        ],
    );
    assert!(success, "{stderr}");
    fs::write(
        dir.path().join("docs/strategies/next.md"),
        format!("---\nmission: {mission}\n---\n\n# Next\n"),
    )
    .unwrap();
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    let output = format!("{stdout}{stderr}");
    assert!(
        !success && output.contains("remove the `mission:` key"),
        "{output}"
    );
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", other.trim(), "--no-strategy"],
    );
    assert!(success, "{stderr}");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(success, "{stdout}{stderr}");

    // A strategy deleted later fails check but does not block body edits.
    fs::remove_file(dir.path().join("docs/strategies/next.md")).unwrap();
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(
        !success
            && format!("{stdout}{stderr}")
                .contains("strategy docs/strategies/next.md does not exist"),
        "{stdout}{stderr}"
    );
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &mission,
            "--body",
            "## Outcome\n\nRevised outcome.",
        ],
    );
    assert!(success, "{stderr}");

    let (_, task, _) = run_atelier(dir.path(), &["-q", "issue", "create", "Task"]);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            task.trim(),
            "--strategy",
            "docs/strategies/next.md",
        ],
    );
    assert!(
        !success && stderr.contains("--strategy applies to missions"),
        "{stderr}"
    );
}

#[test]
fn test_check_rejects_repeated_keys_in_record_front_matter() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let create = |title: &str| {
        let (success, id, stderr) = run_atelier(dir.path(), &["-q", "issue", "create", title]);
        assert!(success, "{stderr}");
        id.trim().to_string()
    };
    let (source, target, other) = (create("Source"), create("Target"), create("Other"));
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &source, &target, "--role", "related"],
    );
    assert!(success, "{stderr}");
    let path = canonical_issue_path(dir.path(), &source);
    let original = fs::read_to_string(&path).unwrap();
    for (case, edited) in [
        (
            "nested",
            original.replacen(
                &format!("    id: \"{target}\"\n"),
                &format!("    id: \"{target}\"\n    id: \"{other}\"\n"),
                1,
            ),
        ),
        (
            "top-level",
            original.replacen("priority:", "priority: \"low\"\npriority:", 1),
        ),
        (
            "typed",
            original.replacen("priority:", "1: \"a\"\n\"1\": \"b\"\npriority:", 1),
        ),
    ] {
        assert_ne!(edited, original, "{case}");
        fs::write(&path, &edited).unwrap();
        let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
        let output = format!("{stdout}{stderr}");
        let key = match case {
            "nested" => "id",
            "typed" => "1",
            _ => "priority",
        };
        assert!(
            !success
                && output.contains(&format!("duplicate key '{key}'"))
                && output.contains(&format!("{source}.md")),
            "{case}: {output}"
        );
    }
    fs::write(&path, &original).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(success, "{stderr}");
}

#[test]
fn test_migrate_epics_retypes_epic_records_and_the_policy() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    // A repository from before atelier-9vq3: the policy declares epic, and an
    // epic record has the old Outcome/Proof/Delivers shape.
    let policy_path = dir.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&policy_path)
        .unwrap()
        .replace(
            "  bug:\n    label: Bug\n",
            "  bug:\n    label: Bug\n  epic:\n    label: Epic\n",
        )
        .replace(
            "    applies_to:\n    - mission\n",
            "    applies_to:\n    - mission\n    - epic\n",
        )
        .replace(
            "before review. A parent closes after its children.\n        validators:\n        - blockers.none_open\n        - work.terminal\n",
            "before review.\n        validators:\n        - blockers.none_open\n",
        );
    // Only the decision workflow's decide keeps the check.
    assert_eq!(policy.matches("- work.terminal").count(), 1, "{policy}");
    fs::write(&policy_path, policy).unwrap();
    let (success, child, stderr) = run_atelier(dir.path(), &["-q", "issue", "create", "Child"]);
    assert!(success, "{stderr}");
    let child = child.trim().to_string();
    let epic = "atelier-ep1c";
    fs::write(
        canonical_issue_path(dir.path(), epic),
        format!(
            "---\ncreated_at: \"2026-10-01T00:00:00+00:00\"\nid: \"{epic}\"\nissue_type: \"epic\"\nlabels: []\npriority: \"P2\"\nrelationships:\n  blocks: []\n  children:\n  - kind: \"issue\"\n    id: \"{child}\"\n  attachments: []\n  relates: []\nschema: \"atelier.issue\"\nschema_version: 2\nstatus: \"draft\"\ntitle: \"Old epic\"\nupdated_at: \"2026-10-01T00:00:00+00:00\"\n---\n\n## Outcome\n\nThe increment works.\n\n## Delivers\n\nS1\n\n## Unknowns\n\nOne question.\n"
        ),
    )
    .unwrap();
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(
        !success && format!("{stdout}{stderr}").contains("atelier migrate epics"),
        "{stdout}{stderr}"
    );

    // Quiet output is the IDs a dry run would retype.
    let (success, quiet, stderr) = run_atelier(dir.path(), &["-q", "migrate", "epics"]);
    assert!(success, "{stderr}");
    assert_eq!(quiet, format!("{epic}\n"));

    let (success, preview, stderr) = run_atelier(dir.path(), &["migrate", "epics"]);
    assert!(success, "{stderr}");
    assert!(
        preview.contains(&format!("{epic}: epic -> feature, status draft -> backlog"))
            && preview.contains("Dry run"),
        "{preview}"
    );
    assert!(fs::read_to_string(&policy_path).unwrap().contains("epic"));

    let (success, _, stderr) = run_atelier(dir.path(), &["migrate", "epics", "--apply"]);
    assert!(success, "{stderr}");
    let migrated_policy = fs::read_to_string(&policy_path).unwrap();
    assert!(!migrated_policy.contains("epic"), "{migrated_policy}");
    assert_eq!(
        migrated_policy.matches("- work.terminal").count(),
        2,
        "{migrated_policy}"
    );
    let record = fs::read_to_string(canonical_issue_path(dir.path(), epic)).unwrap();
    for expected in [
        "issue_type: \"feature\"",
        "status: \"backlog\"",
        "## Goal\n\nThe increment works.",
        "## Done when\n\nEvery child issue is done, delivering:\n\nS1",
        "## Unknowns\n\nOne question.",
        &format!("id: \"{child}\""),
    ] {
        assert!(record.contains(expected), "{expected}: {record}");
    }
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(success, "{stdout}{stderr}");
    let (_, again, _) = run_atelier(dir.path(), &["migrate", "epics"]);
    assert!(again.contains("No epic records"), "{again}");

    // A mission with the removed Epics section gets its own guidance.
    let mission = "atelier-m1ss";
    fs::write(
        canonical_issue_path(dir.path(), mission),
        format!("---\ncreated_at: \"2026-10-01T00:00:00+00:00\"\nid: \"{mission}\"\nissue_type: \"mission\"\nlabels: []\npriority: \"P2\"\nrelationships:\n  blocks: []\n  children: []\n  attachments: []\n  relates: []\nschema: \"atelier.issue\"\nschema_version: 2\nstatus: \"draft\"\ntitle: \"Old mission\"\nupdated_at: \"2026-10-01T00:00:00+00:00\"\n---\n\n## Outcome\n\nShipped.\n\n## Epics\n\nA list.\n"),
    )
    .unwrap();
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(
        !success
            && format!("{stdout}{stderr}").contains("missions no longer have an Epics section"),
        "{stdout}{stderr}"
    );
}

#[test]
fn test_needs_human_work_is_kept_out_of_the_agent_ready_list() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let create = |args: &[&str]| run_atelier(dir.path(), args).1.trim().to_string();
    let agent = create(&["-q", "issue", "create", "Agent work"]);
    let human = create(&[
        "-q",
        "issue",
        "create",
        "Approve the signing account",
        "-l",
        "needs-human",
    ]);

    let ready = create(&["--quiet", "work", "ready"]);
    assert!(ready.lines().any(|line| line == agent), "{ready}");
    assert!(!ready.lines().any(|line| line == human), "{ready}");
    assert_eq!(create(&["--quiet", "work", "human"]), human);
    let all = create(&["--quiet", "work", "all"]);
    assert!(all.lines().any(|line| line == human), "{all}");
    let (success, page, stderr) = run_atelier(dir.path(), &["work", "ready"]);
    assert!(success, "{stderr}");
    let split = page.find("Needs a Human").expect(&page);
    assert!(
        page[..split].contains(&agent) && page[split..].contains(&human),
        "{page}"
    );

    let status = create(&["--quiet", "status"]);
    assert!(status.contains("ready=1 human=1"), "{status}");

    // Mission views keep the same split.
    let mission = create(&[
        "-q",
        "issue",
        "create",
        "Ship",
        "--issue-type",
        "mission",
        "--body",
        "## Outcome\n\nShipped.\n\n## Success criteria\n\nS1.",
    ]);
    for id in [&agent, &human] {
        create(&["-q", "issue", "link", id, &mission, "--role", "advances"]);
    }
    let ready = create(&["--quiet", "work", "mission", &mission, "--ready"]);
    assert_eq!(ready, agent, "{ready}");
    let summary = create(&["--quiet", "work", "mission", &mission]);
    assert!(summary.contains("ready=1 human=1"), "{summary}");
    let show = create(&["issue", "show", &mission, "--full"]);
    assert!(
        show.contains("Scope         : 2 issues: 1 ready, 1 needs a human\n")
            && show.contains("Needs a Human"),
        "{show}"
    );

    init_git_repo(dir.path());
    ensure_initial_git_commit(dir.path());
    let (success, status, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "{stderr}");
    // The human's item is counted apart and not offered as ready work.
    assert!(
        status.contains("Needs a human: 1") && status.contains("Ready work:    1"),
        "{status}"
    );
    let ready = status
        .split("Ready Work\n")
        .nth(1)
        .and_then(|rest| rest.split("\n\n").next())
        .unwrap_or_default();
    assert!(
        ready.contains(&agent) && !ready.contains(&human),
        "{status}"
    );
    commit_all(dir.path(), "plan");
    let (success, started, stderr) = run_atelier(dir.path(), &["start", &human]);
    assert!(success, "{stderr}");
    assert!(started.contains("is labelled needs-human"), "{started}");
}

#[test]
fn test_children_inherit_their_parents_open_blockers() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    let create = |args: &[&str]| run_atelier(dir.path(), args).1.trim().to_string();
    let remote_data = create(&["-q", "issue", "create", "Remote data parent"]);
    let parent = create(&["-q", "issue", "create", "Find a module near me"]);
    let child = create(&[
        "-q",
        "issue",
        "create",
        "Nearest module query",
        "--parent",
        &parent,
    ]);
    create(&[
        "-q",
        "issue",
        "link",
        &parent,
        &remote_data,
        "--role",
        "blocked_by",
    ]);
    let mission = create(&[
        "-q",
        "issue",
        "create",
        "Ship",
        "--issue-type",
        "mission",
        "--body",
        "## Outcome\n\nShipped.\n\n## Success criteria\n\nS1.",
    ]);
    create(&[
        "-q", "issue", "link", &parent, &mission, "--role", "advances",
    ]);

    let ready = create(&["--quiet", "work", "ready"]);
    assert!(!ready.lines().any(|line| line == child), "{ready}");
    let (_, blocked, _) = run_atelier(dir.path(), &["work", "blocked"]);
    assert!(blocked.contains("Nearest module query"), "{blocked}");
    let ready = create(&["--quiet", "work", "mission", &mission, "--ready"]);
    assert!(!ready.lines().any(|line| line == child), "{ready}");
    let (_, dashboard, _) = run_atelier(dir.path(), &["work", "mission", &mission]);
    assert!(
        dashboard.contains(&format!("Nearest module query | blockers: {remote_data}")),
        "{dashboard}"
    );

    let (_, show, _) = run_atelier(dir.path(), &["issue", "show", &child]);
    let at = show
        .find(&format!("Blocked via parent {parent}"))
        .expect(&show);
    assert!(show[at..].contains("Remote data parent"), "{show}");

    commit_all(dir.path(), "plan");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &child]);
    assert!(
        !success,
        "start must refuse a child whose parent is blocked: {stdout}"
    );
    assert!(
        stderr.contains(&format!("{child} -> {parent} -> {remote_data}")),
        "{stderr}"
    );

    // A parent blocked by its own child sequences its children; it does not
    // block them.
    let sequenced = create(&["-q", "issue", "create", "Sequenced parent"]);
    let first = create(&[
        "-q",
        "issue",
        "create",
        "First step",
        "--parent",
        &sequenced,
    ]);
    let second = create(&[
        "-q",
        "issue",
        "create",
        "Second step",
        "--parent",
        &sequenced,
    ]);
    create(&[
        "-q",
        "issue",
        "link",
        &sequenced,
        &first,
        "--role",
        "blocked_by",
    ]);
    let ready = create(&["--quiet", "work", "ready"]);
    assert!(
        ready.lines().any(|line| line == first) && ready.lines().any(|line| line == second),
        "{ready}"
    );

    // Only the issue itself inherits: a done blocker stays done even if its
    // own parent becomes blocked later.
    let other_parent = create(&["-q", "issue", "create", "Other parent"]);
    let finished = create(&[
        "-q",
        "issue",
        "create",
        "Finished child",
        "--parent",
        &other_parent,
    ]);
    let downstream = create(&["-q", "issue", "create", "Downstream"]);
    create(&[
        "-q",
        "issue",
        "link",
        &downstream,
        &finished,
        "--role",
        "blocked_by",
    ]);
    set_issue_status(dir.path(), &finished, "done");
    let later = create(&["-q", "issue", "create", "Later blocker"]);
    create(&[
        "-q",
        "issue",
        "link",
        &other_parent,
        &later,
        "--role",
        "blocked_by",
    ]);
    let ready = create(&["--quiet", "work", "ready"]);
    assert!(ready.lines().any(|line| line == downstream), "{ready}");

    // Once the parent's blocker is done, the child is ready.
    set_issue_status(dir.path(), &remote_data, "done");
    let ready = create(&["--quiet", "work", "ready"]);
    assert!(ready.lines().any(|line| line == child), "{ready}");
}

#[test]
fn test_bundles_update_existing_records_and_report_client_ref_ids() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bundle = dir.path().join("plan.json");
    fs::write(
        &bundle,
        r###"{"schema":"atelier.bundle","schema_version":2,"title":"Plan","resources":{"issues":[
          {"client_ref":"a","title":"First slice","body":"## Goal\n\nA.\n\n## Done when\n\nA passes."},
          {"client_ref":"b","title":"Second slice","body":"## Goal\n\nB.\n\n## Done when\n\nB passes.","blocked_by":[{"client_ref":"a"}]}
        ]}}"###,
    )
    .unwrap();
    let (success, preview, stderr) =
        run_atelier(dir.path(), &["bundle", "preview", bundle.to_str().unwrap()]);
    assert!(success, "{stderr}");
    assert!(preview.contains("a -> (new)  First slice"), "{preview}");
    let (success, applied, stderr) = run_atelier(
        dir.path(),
        &["bundle", "apply", bundle.to_str().unwrap(), "--yes"],
    );
    assert!(success, "{stderr}");
    let first = issue_id_by_title(dir.path(), "First slice");
    let second = issue_id_by_title(dir.path(), "Second slice");
    assert!(
        applied.contains(&format!("a -> {first}  First slice"))
            && applied.contains(&format!("b -> {second}  Second slice")),
        "{applied}"
    );
    run_atelier(
        dir.path(),
        &["issue", "note", &first, "Kept across updates"],
    );

    // Revise the graph after review without minting new IDs: change one
    // body and title, keep the other's body, and add a record and a link.
    let revision = dir.path().join("revision.json");
    fs::write(
        &revision,
        format!(
            r###"{{"schema":"atelier.bundle","schema_version":2,"title":"Revision","resources":{{"issues":[
              {{"client_ref":"a","operation":"update","id":"{first}","title":"First slice, revised","body":"## Goal\n\nA, revised.\n\n## Done when\n\nA passes twice.","labels":["reviewed"],"notes":[{{"body":"Revised after review"}}]}},
              {{"client_ref":"b","operation":"update","id":"{second}","priority":"high","blocked_by":[{{"client_ref":"c"}}]}},
              {{"client_ref":"c","title":"New groundwork","body":"## Goal\n\nC.\n\n## Done when\n\nC passes."}}
            ]}}}}"###
        ),
    )
    .unwrap();
    let (success, applied, stderr) = run_atelier(
        dir.path(),
        &["bundle", "apply", revision.to_str().unwrap(), "--yes"],
    );
    assert!(success, "{stderr}");
    let third = issue_id_by_title(dir.path(), "New groundwork");
    assert!(
        applied.contains(&format!("a -> {first} (updated)  First slice, revised"))
            && applied.contains(&format!("c -> {third}  New groundwork")),
        "{applied}"
    );
    let a = fs::read_to_string(canonical_issue_path(dir.path(), &first)).unwrap();
    for expected in [
        "title: \"First slice, revised\"",
        "A, revised.",
        "\"reviewed\"",
        "Kept across updates",
        "Revised after review",
    ] {
        assert!(a.contains(expected), "{expected}: {a}");
    }
    let b = fs::read_to_string(canonical_issue_path(dir.path(), &second)).unwrap();
    assert!(
        b.contains("priority: \"P1\"") && b.contains("B passes."),
        "{b}"
    );
    let (success, ready, _) = run_atelier(dir.path(), &["--quiet", "work", "ready"]);
    let ready = ready.lines().collect::<Vec<_>>();
    assert!(
        success
            && ready.contains(&first.as_str())
            && ready.contains(&third.as_str())
            && !ready.contains(&second.as_str()),
        "{ready:?}"
    );
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(success, "{stdout}{stderr}");

    // Updates are validated like creates, and status changes stay transitions.
    for (body, expected) in [
        (format!(r###"{{"client_ref":"a","operation":"update","id":"{first}","body":"## Outcome\n\nWrong shape."}}"###), "Invalid body for a"),
        (format!(r###"{{"client_ref":"a","operation":"update","id":"{first}","status":"done"}}"###), "cannot set status"),
        (r###"{"client_ref":"a","operation":"update","id":"atelier-zzzz"}"###.to_string(), "missing issue atelier-zzzz"),
        (format!(r###"{{"client_ref":"a","operation":"update","id":"{first}","issue_type":"bug"}}"###), "cannot change"),
        (format!(r###"{{"client_ref":"a","operation":"update","id":"{first}","labels":["x"]}},{{"client_ref":"a2","operation":"update","id":"{}","labels":["y"]}}"###, &first[first.len() - 4..]), "more than once"),
        (format!(r###"{{"client_ref":"m","title":"Mission","issue_type":"mission","body":"## Outcome\n\nShip.","strategy":{{"path":"docs/missing.md"}}}}"###), "does not exist"),
        (format!(r###"{{"client_ref":"m","title":"Mission","issue_type":"mission","body":"## Outcome\n\nShip."}}"###), "Mission requires Success criteria"),
        (r###"{"client_ref":"a","operation":"update","title":"No id","body":"## Goal\n\nG.\n\n## Done when\n\nD."}"###.to_string(), "needs the id"),
    ] {
        fs::write(
            &revision,
            format!(r###"{{"schema":"atelier.bundle","schema_version":2,"title":"Bad","resources":{{"issues":[{body}]}}}}"###),
        )
        .unwrap();
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["bundle", "apply", revision.to_str().unwrap(), "--yes"],
        );
        assert!(!success && stderr.contains(expected), "{expected}: {stderr}");
    }

    // Done records keep the acceptance their proof was judged against, and
    // moving a record to another parent is an issue update.
    run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &third,
            "supersede",
            "--reason",
            "Not needed",
        ],
    );
    let parent = issue_id_by_title(dir.path(), "First slice, revised");
    let (success, child, _) = run_atelier(
        dir.path(),
        &["-q", "issue", "create", "Child", "--parent", &second],
    );
    assert!(success);
    let child = child.trim().to_string();
    for (body, expected) in [
        (
            format!(
                r###"{{"client_ref":"c","operation":"update","id":"{third}","body":"## Goal\n\nC2.\n\n## Done when\n\nC2."}}"###
            ),
            "issue_body_done".to_string(),
        ),
        (
            format!(
                r###"{{"client_ref":"k","operation":"update","id":"{child}","parent":{{"id":"{parent}"}}}}"###
            ),
            format!("use `atelier issue update {child} --parent {parent}`"),
        ),
        (
            format!(
                r###"{{"client_ref":"s","operation":"update","id":"{second}","parent":{{"id":"{parent}"}}}}"###
            ),
            "has children of its own".to_string(),
        ),
    ] {
        fs::write(
            &revision,
            format!(r###"{{"schema":"atelier.bundle","schema_version":2,"title":"Bad","resources":{{"issues":[{body}]}}}}"###),
        )
        .unwrap();
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["bundle", "preview", revision.to_str().unwrap()],
        );
        assert!(
            !success && stderr.contains(&expected),
            "{expected}: {stderr}"
        );
    }

    // Restating the current parent through an update entry's client_ref, and
    // naming a parent by an ID suffix, are both accepted.
    fs::write(
        &revision,
        format!(
            r###"{{"schema":"atelier.bundle","schema_version":2,"title":"Restate","resources":{{"issues":[
              {{"client_ref":"p","operation":"update","id":"{second}"}},
              {{"client_ref":"k","operation":"update","id":"{child}","parent":{{"client_ref":"p"}}}},
              {{"client_ref":"n","title":"Suffix child","body":"## Goal\n\nN.\n\n## Done when\n\nN.","parent":{{"id":"{}"}}}}
            ]}}}}"###,
            &second[second.len() - 4..]
        ),
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["bundle", "apply", revision.to_str().unwrap(), "--yes"],
    );
    assert!(success, "{stderr}");
    let suffix_child = issue_id_by_title(dir.path(), "Suffix child");
    let parent_record = fs::read_to_string(canonical_issue_path(dir.path(), &second)).unwrap();
    assert!(parent_record.contains(&suffix_child), "{parent_record}");
    // A parent named through an update entry for a record that is already a
    // child is refused at preview too.
    fs::write(
        &revision,
        format!(
            r###"{{"schema":"atelier.bundle","schema_version":2,"title":"Nest","resources":{{"issues":[
              {{"client_ref":"k","operation":"update","id":"{child}"}},
              {{"client_ref":"g","title":"Grandchild","body":"## Goal\n\nG.\n\n## Done when\n\nG.","parent":{{"client_ref":"k"}}}}
            ]}}}}"###
        ),
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["bundle", "preview", revision.to_str().unwrap()],
    );
    assert!(
        !success && stderr.contains("parents are one level deep"),
        "{stderr}"
    );
}

#[test]
fn test_missions_and_parents_carry_background_research() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let create = |args: &[&str]| {
        let (success, stdout, stderr) = run_atelier(dir.path(), args);
        assert!(success, "{args:?}: {stderr}");
        stdout.trim().to_string()
    };
    let mission = create(&[
        "-q", "issue", "create", "Replace the toolbox", "--issue-type", "mission", "--body",
        "## Outcome\n\nOne tool.\n\n## Success criteria\n\nS1.\n\n## Background\n\n| Tool | Verdict |\n| --- | --- |\n| EDMC | replace |",
    ]);
    let parent = create(&[
        "-q", "issue", "create", "EDDN sender", "--issue-type", "feature", "--body",
        "## Goal\n\nSend to EDDN.\n\n## Done when\n\nChildren done.\n\n## Background\n\nSender rules: [EDDN docs](https://example.test/eddn).",
    ]);
    create(&[
        "-q",
        "issue",
        "create",
        "Journal events",
        "--parent",
        &parent,
    ]);
    for (id, expected) in [(&mission, "| EDMC | replace |"), (&parent, "Sender rules:")] {
        let show = create(&["issue", "show", id, "--full"]);
        let at = show.find("Background").expect(&show);
        assert!(show[at..].contains(expected), "{show}");
    }
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(success, "{stdout}{stderr}");
    // Background is optional and revisable like any other section.
    create(&[
        "issue", "update", &parent, "--body",
        "## Goal\n\nSend to EDDN.\n\n## Done when\n\nChildren done.\n\n## Background\n\nRevised rules.",
    ]);
    assert!(create(&["issue", "show", &parent, "--full"]).contains("Revised rules."));
}

#[test]
fn test_work_ready_lists_startable_work_and_shows_parents_and_missions_as_context() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let create = |args: &[&str]| run_atelier(dir.path(), args).1.trim().to_string();
    let mission = create(&[
        "-q",
        "issue",
        "create",
        "Replace the toolbox",
        "--issue-type",
        "mission",
        "--body",
        "## Outcome\n\nOne tool.\n\n## Success criteria\n\nS1.",
    ]);
    // Missions start as draft; a mission ready to start is still context.
    create(&["-q", "issue", "transition", &mission, "ready"]);
    let parent = create(&[
        "-q",
        "issue",
        "create",
        "Trade parent",
        "--issue-type",
        "feature",
    ]);
    let child = create(&["-q", "issue", "create", "Trade child", "--parent", &parent]);
    create(&[
        "-q", "issue", "link", &parent, &mission, "--role", "advances",
    ]);

    let ready = create(&["--quiet", "work", "ready"]);
    assert_eq!(
        ready.lines().collect::<Vec<_>>(),
        [child.as_str()],
        "{ready}"
    );
    let (_, page, _) = run_atelier(dir.path(), &["work", "ready"]);
    let context = page.find("Parents and Missions (context)").expect(&page);
    assert!(
        page[context..].contains(&parent) && page[context..].contains(&mission),
        "{page}"
    );
    assert!(!page[..context].contains(&parent), "{page}");
    let mission_ready = create(&["--quiet", "work", "mission", &mission, "--ready"]);
    assert_eq!(mission_ready, child, "{mission_ready}");
    let status = create(&["--quiet", "status"]);
    assert!(status.contains("ready=1"), "{status}");
    // Summaries still add up to the scope.
    let summary = create(&["--quiet", "work", "mission", &mission]);
    assert!(summary.contains("ready=1 human=0 context=1"), "{summary}");
    let (_, missions, _) = run_atelier(dir.path(), &["work", "missions"]);
    assert!(
        missions.contains("Scope: 2 issues: 1 ready, 1 context\n"),
        "{missions}"
    );
    let show = create(&["issue", "show", &mission, "--full"]);
    assert!(
        show.contains("Scope         : 2 issues: 1 ready, 1 context\n"),
        "{show}"
    );

    // A parent whose children are all done is ready to start and close.
    let mut record = fs::read_to_string(canonical_issue_path(dir.path(), &child)).unwrap();
    record = record.replace("status: \"todo\"", "status: \"done\"");
    fs::write(canonical_issue_path(dir.path(), &child), record).unwrap();
    let ready = create(&["--quiet", "work", "ready"]);
    assert_eq!(
        ready.lines().collect::<Vec<_>>(),
        [parent.as_str()],
        "{ready}"
    );

    // Parked work stays in backlog even with an open blocker.
    let parked = create(&["-q", "issue", "create", "Parked", "--status", "backlog"]);
    let blocker = create(&["-q", "issue", "create", "Blocker"]);
    create(&[
        "-q",
        "issue",
        "link",
        &parked,
        &blocker,
        "--role",
        "blocked_by",
    ]);
    let blocked = create(&["--quiet", "work", "blocked"]);
    assert!(!blocked.lines().any(|line| line == parked), "{blocked}");
    let (_, all, _) = run_atelier(dir.path(), &["work", "all"]);
    let backlog = all.find("Backlog Work").expect(&all);
    assert!(all[backlog..].contains("Parked"), "{all}");
}

#[test]
fn test_decisions_wait_for_the_human_and_close_with_a_recorded_decision() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    let create = |args: &[&str]| {
        let (success, stdout, stderr) = run_atelier(dir.path(), args);
        assert!(success, "{args:?}: {stderr}");
        stdout.trim().to_string()
    };
    let question = "## Question\n\nShould epic become a parent issue?\n\n## Options\n\n- (a) drop epic\n- (b) rename it";
    let decision = create(&[
        "-q",
        "issue",
        "create",
        "Decide whether epic should become a parent",
        "--issue-type",
        "decision",
        "--body",
        question,
    ]);
    let work = create(&["-q", "issue", "create", "Ordinary work"]);

    let ready = create(&["--quiet", "work", "ready"]);
    assert_eq!(ready, work, "decisions are not agent work: {ready}");
    assert_eq!(create(&["--quiet", "work", "human"]), decision);
    let (_, show, _) = run_atelier(dir.path(), &["issue", "show", &decision, "--full"]);
    assert!(
        show.contains("Question") && show.contains("(a) drop epic"),
        "{show}"
    );

    // A decision cannot close until it says what was decided.
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &decision,
            "decide",
            "--reason",
            "Chose (a)",
        ],
    );
    assert!(
        !success && stderr.contains("Decision section is missing"),
        "{stderr}"
    );
    create(&[
        "issue",
        "update",
        &decision,
        "--body",
        &format!(
            "{question}\n\n## Decision\n\n(a): drop epic; any issue can parent, one level deep."
        ),
    ]);
    // Decisions need no branch: they close on the base as a planning change.
    commit_all(dir.path(), "file the decision");
    create(&[
        "issue",
        "transition",
        &decision,
        "decide",
        "--reason",
        "Chose (a)",
    ]);
    commit_all(dir.path(), "record the decision on the base");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["land"]);
    assert!(success, "{stdout}{stderr}");
    let ready = create(&["--quiet", "work", "human"]);
    assert!(ready.is_empty(), "{ready}");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(success, "{stdout}{stderr}");

    // Decisions own no branch.
    let open = create(&[
        "-q",
        "issue",
        "create",
        "Open question",
        "--issue-type",
        "decision",
    ]);
    let record = fs::read_to_string(canonical_issue_path(dir.path(), &open)).unwrap();
    assert!(
        record.contains("## Question\n\nQuestion was not specified."),
        "{record}"
    );
    commit_all(dir.path(), "file an open question");
    let (success, _, stderr) = run_atelier(dir.path(), &["start", &open]);
    assert!(
        !success && stderr.contains("decisions need no branch"),
        "{stderr}"
    );

    // A blocked decision is blocked, and decide checks blockers and children.
    let prerequisite = create(&["-q", "issue", "create", "Prerequisite research"]);
    create(&[
        "-q",
        "issue",
        "link",
        &open,
        &prerequisite,
        "--role",
        "blocked_by",
    ]);
    let blocked = create(&["--quiet", "work", "blocked"]);
    assert!(blocked.lines().any(|line| line == open), "{blocked}");
    assert!(create(&["--quiet", "work", "human"]).is_empty());
    create(&[
        "issue",
        "update",
        &open,
        "--body",
        "## Question\n\nWhich?\n\n## Decision\n\nThis one.",
    ]);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &open,
            "decide",
            "--reason",
            "Too early",
        ],
    );
    assert!(!success && stderr.contains(&prerequisite), "{stderr}");

    // Retyping a "Decide ..." task does not invent a Decision, and retyping
    // back keeps the Options readable in Context.
    let task = create(&[
        "-q",
        "issue",
        "create",
        "Decide the slug rule",
        "--body",
        "## Goal\n\nPick a slug rule.\n\n## Done when\n\nThe rule is documented.",
    ]);
    create(&["issue", "update", &task, "--issue-type", "decision"]);
    let record = fs::read_to_string(canonical_issue_path(dir.path(), &task)).unwrap();
    assert!(
        record.contains("## Question\n\nPick a slug rule.") && !record.contains("## Decision"),
        "{record}"
    );
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &task, "decide", "--reason", "x"],
    );
    assert!(
        !success && stderr.contains("Decision section is missing"),
        "{stderr}"
    );
    create(&[
        "issue",
        "update",
        &task,
        "--body",
        "## Question\n\nPick a slug rule.\n\n## Options\n\n- origin\n- directory",
    ]);
    create(&["issue", "update", &task, "--issue-type", "task"]);
    let record = fs::read_to_string(canonical_issue_path(dir.path(), &task)).unwrap();
    assert!(record.contains("Options:\n\n- origin"), "{record}");
}

#[test]
fn test_children_of_a_parked_parent_are_parked_until_it_is_planned() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    let create = |args: &[&str]| run_atelier(dir.path(), args).1.trim().to_string();
    let parent = create(&[
        "-q",
        "issue",
        "create",
        "Later increment",
        "--status",
        "backlog",
    ]);
    let child = create(&[
        "-q",
        "issue",
        "create",
        "Its first slice",
        "--parent",
        &parent,
    ]);

    let ready = create(&["--quiet", "work", "ready"]);
    assert!(!ready.lines().any(|line| line == child), "{ready}");
    let (_, all, _) = run_atelier(dir.path(), &["work", "all"]);
    let backlog = all.find("Backlog Work").expect(&all);
    assert!(all[backlog..].contains("Its first slice"), "{all}");

    commit_all(dir.path(), "plan a later increment");
    for args in [
        vec!["start", child.as_str()],
        vec!["issue", "transition", &child, "start"],
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &args);
        assert!(!success, "{args:?} must refuse a parked child");
        assert!(
            stderr.contains(&format!("parent {parent} is parked in 'backlog'")),
            "{stderr}"
        );
    }

    // A blocked child cannot start under a parked parent either.
    let blocked = create(&[
        "-q",
        "issue",
        "create",
        "Blocked slice",
        "--parent",
        &parent,
    ]);
    create(&["-q", "issue", "transition", &blocked, "block"]);
    commit_all(dir.path(), "a blocked child");
    let (success, _, stderr) = run_atelier(dir.path(), &["start", &blocked]);
    assert!(
        !success && stderr.contains("is parked in 'backlog'"),
        "{stderr}"
    );

    create(&["-q", "issue", "transition", &parent, "plan"]);
    let ready = create(&["--quiet", "work", "ready"]);
    assert!(ready.lines().any(|line| line == child), "{ready}");

    // Deferring a parent later does not hide or stall a child in flight.
    commit_all(dir.path(), "plan the parent");
    let (success, _, stderr) = run_atelier(dir.path(), &["start", &child]);
    assert!(success, "{stderr}");
    commit_all(dir.path(), "start the child");
    create(&["-q", "issue", "transition", &parent, "defer"]);
    let active = create(&["--quiet", "work", "active"]);
    assert!(active.lines().any(|line| line == child), "{active}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &child, "request_review"],
    );
    assert!(success, "a child in flight keeps its transitions: {stderr}");

    // A parent that can never start, such as an open decision, parks nothing.
    let question = create(&[
        "-q",
        "issue",
        "create",
        "Which store?",
        "--issue-type",
        "decision",
        "--body",
        "## Question\n\nWhich store?",
    ]);
    let spike = create(&[
        "-q",
        "issue",
        "create",
        "Store spike",
        "--issue-type",
        "spike",
        "--parent",
        &question,
    ]);
    let ready = create(&["--quiet", "work", "ready"]);
    assert!(ready.lines().any(|line| line == spike), "{ready}");
}

#[test]
fn test_size_labels_are_validated_shown_and_replaced() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, id, stderr) = run_atelier(
        dir.path(),
        &["-q", "issue", "create", "Sized work", "-l", "size-m"],
    );
    assert!(success, "{stderr}");
    let id = id.trim().to_string();
    let (_, show, _) = run_atelier(dir.path(), &["issue", "show", &id, "--full"]);
    assert!(show.contains("Size:     M"), "{show}");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "update", &id, "-l", "size-xl"]);
    assert!(success, "{stderr}");
    let record = fs::read_to_string(canonical_issue_path(dir.path(), &id)).unwrap();
    assert!(
        record.contains("\"size-xl\"") && !record.contains("\"size-m\""),
        "{record}"
    );
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(success, "{stdout}{stderr}");

    // Bad sizes are refused when added, and sizes are normalized to lowercase.
    for args in [
        vec!["issue", "create", "Badly sized", "-l", "size-huge"],
        vec![
            "issue",
            "create",
            "Two sizes",
            "-l",
            "size-s",
            "-l",
            "size-l",
        ],
        vec!["issue", "update", id.as_str(), "-l", "size-huge"],
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &args);
        assert!(
            !success && stderr.contains("size-s, size-m, size-l, size-xl"),
            "{args:?}: {stderr}"
        );
    }
    let (success, upper, stderr) = run_atelier(
        dir.path(),
        &["-q", "issue", "create", "Shouty", "-l", "size-L"],
    );
    assert!(success, "{stderr}");
    let record = fs::read_to_string(canonical_issue_path(dir.path(), upper.trim())).unwrap();
    assert!(record.contains("\"size-l\""), "{record}");

    // A hand-edited bad size is caught by check.
    edit_canonical_issue(dir.path(), upper.trim(), |markdown| {
        markdown.replace("\"size-l\"", "\"size-huge\"")
    });
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    let output = format!("{stdout}{stderr}");
    assert!(
        !success && output.contains("size-huge is not one of"),
        "{output}"
    );
}

/// A note is dated by the local day that `Created`/`Updated` show, not the
/// UTC day. UTC-12 and UTC+14 are always on different days, so at least one
/// of them differs from the UTC day.
#[test]
fn notes_are_dated_by_the_local_day() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    run_atelier(dir.path(), &["issue", "create", "Dated work"]);
    let issue_id = issue_id_by_title(dir.path(), "Dated work");

    // POSIX TZ offsets are west-positive: `UTC+12` is twelve hours behind.
    for (tz, east_seconds) in [("UTC+12", -12 * 3600), ("UTC-14", 14 * 3600)] {
        let zone = chrono::FixedOffset::east_opt(east_seconds).unwrap();
        let local_day = || chrono::Utc::now().with_timezone(&zone).date_naive();
        let before = local_day();
        let text = format!("note in {tz}");
        let (success, _, stderr) = run_atelier_with_env(
            dir.path(),
            &["issue", "note", &issue_id, &text],
            &[("TZ", tz)],
        );
        assert!(success, "note in {tz} failed: {stderr}");
        let after = local_day();
        let record = read_canonical_record(dir.path(), "issues", &issue_id);
        let dated = |day: chrono::NaiveDate| record.contains(&format!("- {day} [note] {text}\n"));
        assert!(
            dated(before) || dated(after),
            "note in {tz} should be dated {before}: {record}"
        );
    }
}

#[test]
fn prune_cutoff_is_the_local_day() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // POSIX TZ offsets are west-positive: `UTC+12` is twelve hours behind.
    for (tz, east_seconds) in [("UTC+12", -12 * 3600), ("UTC-14", 14 * 3600)] {
        let zone = chrono::FixedOffset::east_opt(east_seconds).unwrap();
        let cutoff =
            || chrono::Utc::now().with_timezone(&zone).date_naive() - chrono::Duration::days(3);
        let before = cutoff();
        let (success, stdout, stderr) = run_atelier_with_env(
            dir.path(),
            &["prune", "--retention-days", "3"],
            &[("TZ", tz)],
        );
        assert!(success, "prune in {tz} failed: {stderr}");
        let after = cutoff();
        let shows = |day: chrono::NaiveDate| stdout.contains(&format!("before {day}\n"));
        assert!(
            shows(before) || shows(after),
            "prune in {tz} should cut off before {before}: {stdout}"
        );
    }
}

// ==================== State-dependent footers (atelier-ksx6) ====================

/// The `Next` section of a view, from its heading to the end.
fn next_section(output: &str) -> &str {
    output
        .split_once("\nNext\n----\n")
        .map(|(_, rest)| rest)
        .unwrap_or_else(|| panic!("no Next section:\n{output}"))
}

fn close_issue(dir: &Path, issue_id: &str, reason: &str) -> String {
    move_issue_to_review(dir, issue_id);
    attach_issue_pass_evidence(dir, issue_id);
    let (success, stdout, stderr) = run_atelier(
        dir,
        &["issue", "transition", issue_id, "close", "--reason", reason],
    );
    assert!(success, "close {issue_id} failed: {stderr}");
    stdout
}

#[test]
fn test_done_issue_shows_its_close_and_offers_no_edit_or_transition() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Finished work"]);
    assert!(success, "{stderr}");
    let id = issue_id_by_title(dir.path(), "Finished work");
    close_issue(
        dir.path(),
        &id,
        "Shipped the fix\n\nWith more detail below.",
    );
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let closed = format!("Closed {today}: Shipped the fix");

    let (success, brief, stderr) = run_atelier(dir.path(), &["issue", "show", &id]);
    assert!(success, "{stderr}");
    assert!(brief.contains(&format!("\n{closed}\n")), "{brief}");
    assert!(!brief.contains("Transitions:"), "{brief}");

    let (success, full, stderr) = run_atelier(dir.path(), &["issue", "show", &id, "--full"]);
    assert!(success, "{stderr}");
    assert!(
        full.contains(&format!(
            "Transition Readiness\n--------------------\n  {closed}\n\n"
        )),
        "{full}"
    );
    assert!(!full.contains("no configured transitions"), "{full}");
    let next = next_section(&full);
    assert!(!next.contains("Edit issue Markdown"), "{next}");
    assert!(!next.contains("issue transition"), "{next}");
    assert!(next.contains(&format!("atelier check {id}")), "{next}");
}

#[test]
fn test_open_issue_footer_keeps_edit_and_transition_options_once() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Open work"]);
    assert!(success, "{stderr}");
    let id = issue_id_by_title(dir.path(), "Open work");
    let (success, full, stderr) = run_atelier(dir.path(), &["issue", "show", &id, "--full"]);
    assert!(success, "{stderr}");
    let next = next_section(&full);
    assert!(next.contains("Edit issue Markdown"), "{next}");
    assert_eq!(
        full.matches(&format!("atelier issue transition {id}\n"))
            .count(),
        1,
        "the options command appears once, in the footer:\n{full}"
    );
}

#[test]
fn test_close_transition_footer_offers_handoff_not_transitions() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Unstarted work"]);
    assert!(success, "{stderr}");
    let standalone = issue_id_by_title(dir.path(), "Unstarted work");
    // An owner with no recorded branch has nothing to merge.
    let closed = close_issue(dir.path(), &standalone, "Done");
    let next = next_section(&closed);
    assert!(!next.contains("issue transition"), "{closed}");
    assert!(next.contains("Check handoff: atelier land\n"), "{closed}");
    assert!(!next.contains("merge"), "{closed}");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Owning parent"]);
    assert!(success, "{stderr}");
    let parent = issue_id_by_title(dir.path(), "Owning parent");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Owned child", "--parent", &parent],
    );
    assert!(success, "{stderr}");
    let child = issue_id_by_title(dir.path(), "Owned child");
    // The branch `atelier start` would record on the parent.
    edit_canonical_issue(dir.path(), &parent, |markdown| {
        replace_front_matter_scalar(&markdown, "status", "in_progress").replacen(
            "\npriority: ",
            "\nworkflow_branch:\n  branch_base: main\n  work_branch: feature/owning-parent\npriority: ",
            1,
        )
    });
    let closed = close_issue(dir.path(), &child, "Done");
    let next = next_section(&closed);
    assert!(
        next.contains(&format!("atelier land {parent}\n")),
        "{closed}"
    );
    assert!(
        next.contains(&format!("atelier merge {parent} --check\n")),
        "{closed}"
    );
    assert!(!next.contains("issue transition"), "{closed}");
}

#[test]
fn test_issue_create_footer_offers_start_for_work_and_transitions_for_missions() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, created, stderr) = run_atelier(dir.path(), &["issue", "create", "New work"]);
    assert!(success, "{stderr}");
    let id = issue_id_by_title(dir.path(), "New work");
    let next = next_section(&created);
    assert!(
        next.contains(&format!("Start this work: atelier start {id}\n")),
        "{created}"
    );
    assert!(!next.contains("issue transition"), "{created}");

    // Backlog work has no start transition; it is planned first.
    let (success, created, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Someday work", "--status", "backlog"],
    );
    assert!(success, "{stderr}");
    let someday = issue_id_by_title(dir.path(), "Someday work");
    let next = next_section(&created);
    assert!(
        next.contains(&format!("atelier issue transition {someday}\n")),
        "{created}"
    );
    assert!(!next.contains("atelier start"), "{created}");

    let (success, created, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "New mission", "--issue-type", "mission"],
    );
    assert!(success, "{stderr}");
    let mission = issue_id_by_title(dir.path(), "New mission");
    let next = next_section(&created);
    assert!(
        next.contains(&format!("atelier issue transition {mission}\n")),
        "{created}"
    );
    assert!(!next.contains("atelier start"), "{created}");
}

#[test]
fn test_mission_views_offer_start_and_name_flags_for_omitted_rows() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Footer mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "{stderr}");
    let mission = issue_id_by_title(dir.path(), "Footer mission");
    let mut work = Vec::new();
    for number in 0..10 {
        let title = format!("Mission ready {number}");
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", &title]);
        assert!(success, "{stderr}");
        let id = issue_id_by_title(dir.path(), &title);
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["issue", "link", &mission, &id, "--role", "advances"],
        );
        assert!(success, "{stderr}");
        work.push(id);
    }

    let (success, summary, stderr) = run_atelier(dir.path(), &["work", "mission", &mission]);
    assert!(success, "{stderr}");
    let top = summary
        .split("Ready Work\n----------\n")
        .nth(1)
        .and_then(|rows| rows.split_whitespace().next())
        .unwrap_or_else(|| panic!("no ready rows:\n{summary}"))
        .to_string();
    assert!(
        summary.contains(&format!(
            "... 2 more ready work items omitted; see `atelier work mission {mission} --ready`"
        )),
        "{summary}"
    );
    let next = next_section(&summary);
    assert!(
        next.contains(&format!("Start the top ready item: atelier start {top}\n")),
        "{summary}"
    );
    assert!(!next.contains("--ready"), "{summary}");

    let (success, ready, stderr) =
        run_atelier(dir.path(), &["work", "mission", &mission, "--ready"]);
    assert!(success, "{stderr}");
    assert!(!ready.contains("omitted"), "{ready}");
    let next = next_section(&ready);
    assert!(
        !next.contains(&format!("atelier work mission {mission}")),
        "the footer repeats the view or a filter:\n{ready}"
    );
    assert!(next.contains(&format!("atelier start {top}\n")), "{ready}");

    let (success, full, stderr) = run_atelier(dir.path(), &["issue", "show", &mission, "--full"]);
    assert!(success, "{stderr}");
    assert!(
        full.contains(&format!(
            "... 5 more ready work items omitted; see `atelier work mission {mission} --ready`"
        )),
        "{full}"
    );
}

#[test]
fn test_done_mission_footer_offers_no_transitions() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Finished mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "{stderr}");
    let mission = issue_id_by_title(dir.path(), "Finished mission");
    edit_canonical_issue(dir.path(), &mission, |markdown| {
        replace_front_matter_scalar(&markdown, "status", "done")
    });
    let (success, view, stderr) = run_atelier(dir.path(), &["work", "mission", &mission]);
    assert!(success, "{stderr}");
    let next = next_section(&view);
    assert_eq!(
        next.trim_end(),
        format!("  Show mission record: atelier issue show {mission}"),
        "{view}"
    );
}

#[test]
fn test_work_parent_all_shows_every_child_its_summary_omits() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Wide parent"]);
    assert!(success, "{stderr}");
    let parent = issue_id_by_title(dir.path(), "Wide parent");
    for number in 0..10 {
        let title = format!("Wide child {number}");
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["issue", "create", &title, "--parent", &parent],
        );
        assert!(success, "{stderr}");
    }
    let (success, summary, stderr) = run_atelier(dir.path(), &["work", "parent", &parent]);
    assert!(success, "{stderr}");
    assert!(
        summary.contains(&format!(
            "... 2 more ready work items omitted; see `atelier work parent {parent} --all`"
        )),
        "{summary}"
    );
    let (success, all, stderr) = run_atelier(dir.path(), &["work", "parent", &parent, "--all"]);
    assert!(success, "{stderr}");
    assert!(!all.contains("omitted"), "{all}");
    assert_eq!(all.matches("Wide child").count(), 10, "{all}");
}

// ============ Parent proof gaps and done-record hints (atelier-0sia) ============

#[test]
fn test_work_parent_proof_gaps_count_its_scope_not_the_parent() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Gapped parent"]);
    assert!(success, "{stderr}");
    let parent = issue_id_by_title(dir.path(), "Gapped parent");
    let mut children = Vec::new();
    for title in ["Gapped child one", "Gapped child two"] {
        let (success, _, stderr) =
            run_atelier(dir.path(), &["issue", "create", title, "--parent", &parent]);
        assert!(success, "{stderr}");
        children.push(issue_id_by_title(dir.path(), title));
    }

    let proof_gaps = |view: &str| {
        view.lines()
            .find(|line| line.starts_with("Proof gaps"))
            .map(|line| line.rsplit(": ").next().unwrap().to_string())
    };
    let (success, view, stderr) = run_atelier(dir.path(), &["work", "parent", &parent]);
    assert!(success, "{stderr}");
    assert!(view.contains("Work                : 2 issues"), "{view}");
    assert_eq!(proof_gaps(&view).as_deref(), Some("2"), "{view}");
    let (success, quiet, stderr) = run_atelier(dir.path(), &["-q", "work", "parent", &parent]);
    assert!(success, "{stderr}");
    assert!(quiet.trim_end().ends_with(" proof_gaps=2"), "{quiet}");

    // Proof on every child closes the gaps, though the parent has none.
    for child in &children {
        attach_issue_pass_evidence(dir.path(), child);
    }
    let (success, view, stderr) = run_atelier(dir.path(), &["work", "parent", &parent]);
    assert!(success, "{stderr}");
    assert_eq!(proof_gaps(&view), None, "{view}");
    let (success, quiet, stderr) = run_atelier(dir.path(), &["-q", "work", "parent", &parent]);
    assert!(success, "{stderr}");
    assert!(quiet.trim_end().ends_with(" proof_gaps=0"), "{quiet}");
}

#[test]
fn test_done_parent_view_offers_no_transition_readiness() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Finished parent"]);
    assert!(success, "{stderr}");
    let parent = issue_id_by_title(dir.path(), "Finished parent");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Finished child", "--parent", &parent],
    );
    assert!(success, "{stderr}");
    let child = issue_id_by_title(dir.path(), "Finished child");

    let (success, open, stderr) = run_atelier(dir.path(), &["work", "parent", &parent]);
    assert!(success, "{stderr}");
    assert!(open.contains("Transition readiness:"), "{open}");

    for id in [&child, &parent] {
        set_issue_status(dir.path(), id, "done");
    }
    let (success, done, stderr) = run_atelier(dir.path(), &["work", "parent", &parent]);
    assert!(success, "{stderr}");
    assert!(!done.contains("Transition readiness"), "{done}");
}

#[test]
fn test_issue_link_on_a_done_record_offers_no_transition_options() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    for title in ["Linked done work", "Linked open work"] {
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", title]);
        assert!(success, "{stderr}");
    }
    let done = issue_id_by_title(dir.path(), "Linked done work");
    let open = issue_id_by_title(dir.path(), "Linked open work");
    close_issue(dir.path(), &done, "Shipped");

    let (success, linked, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &done, &open, "--role", "related"],
    );
    assert!(success, "{stderr}");
    let next = next_section(&linked);
    assert!(!next.contains("issue transition"), "{linked}");
    assert!(
        next.contains(&format!("atelier issue show {done}")),
        "{next}"
    );
    assert!(
        next.contains(&format!("atelier issue show {open}")),
        "{next}"
    );

    let (success, linked, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &open, &done, "--role", "related"],
    );
    assert!(success, "{stderr}");
    assert!(
        next_section(&linked).contains(&format!("atelier issue transition {open}")),
        "{linked}"
    );
}

#[test]
fn test_brief_done_issue_offers_no_close_reason_its_closed_line_shows() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    for title in ["Briefly closed", "Closed at length"] {
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", title]);
        assert!(success, "{stderr}");
    }
    let brief_id = issue_id_by_title(dir.path(), "Briefly closed");
    let long_id = issue_id_by_title(dir.path(), "Closed at length");
    close_issue(dir.path(), &brief_id, "Shipped the fix");
    close_issue(
        dir.path(),
        &long_id,
        "Shipped the fix\n\nWith more detail below.",
    );

    let (success, brief, stderr) = run_atelier(dir.path(), &["issue", "show", &brief_id]);
    assert!(success, "{stderr}");
    assert!(brief.contains(": Shipped the fix\n"), "{brief}");
    assert!(!brief.contains("Close Reason"), "{brief}");

    // A longer reason than the Closed line shows is still offered.
    let (success, long, stderr) = run_atelier(dir.path(), &["issue", "show", &long_id]);
    assert!(success, "{stderr}");
    let more = long
        .lines()
        .find(|line| line.starts_with("More with --full:"))
        .unwrap_or_else(|| panic!("no More line:\n{long}"));
    assert!(more.contains("Close Reason"), "{long}");
}
