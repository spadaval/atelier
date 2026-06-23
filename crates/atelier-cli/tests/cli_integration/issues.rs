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
    assert!(stdout.contains(&format!("atelier lint {issue_id}")));
    assert!(stdout.contains(&format!("atelier issue show {issue_id}")));
    let issue_text = read_canonical_record(dir.path(), "issues", &issue_id);
    assert!(issue_text.contains("## Description\n\nNo description provided."));
    assert!(issue_text.contains("## Outcome\n\nOutcome was not specified."));
    assert!(!issue_text.contains("## Evidence"));

    let (success, lint_out, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(!success, "placeholder scaffold should be under-specified");
    assert!(stderr.contains("Lint failed"));
    assert!(lint_out.contains("Issue section Description must be present and non-empty"));
    assert!(lint_out.contains("Issue section Outcome must be present and non-empty"));
    assert!(!lint_out.contains("Issue section Evidence must be present and non-empty"));
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
            "No description provided.",
            "Describe the markdown-first issue.",
        )
        .replace(
            "Outcome was not specified.",
            "Issue sections are populated by editing canonical Markdown.\n\n## Evidence\n\n- `atelier lint <id>` passes after section edits.",
        );
    std::fs::write(&path, text).unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(success, "lint after markdown edit failed: {stderr}");
    let (success, show, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(show.contains("Describe the markdown-first issue."));
    assert!(show.contains("Issue sections are populated by editing canonical Markdown."));
    assert!(show.contains("atelier lint <id>"));
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
    assert!(stdout.contains(&format!("atelier lint {child_id}")));

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
    std::fs::write(
        &parent_path,
        parent_markdown.replace("issue_type: \"epic\"", "issue_type: \"task\""),
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint"]);
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

    let (success, search_out, stderr) = run_atelier(dir.path(), &["search", "informs"]);
    assert!(success, "search failed: {stderr}");
    assert!(search_out.contains("Context mission"), "{search_out}");
    assert!(search_out.contains("Context note"), "{search_out}");

    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &mission_id]);
    assert!(success, "show failed: {stderr}");
    assert!(show_out.contains("informs"), "{show_out}");
    assert!(show_out.contains("Context note"), "{show_out}");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["issue", "status", &mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Total: 1"), "{status_out}");
    assert!(status_out.contains("Mission work"), "{status_out}");
    assert!(
        !status_out.contains("Context note"),
        "custom context links must not count as mission work:\n{status_out}"
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

// ==================== Issue Listing Tests ====================

#[test]
fn test_list_empty() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);

    assert!(success);
    assert!(
        stdout.contains("No issues found."),
        "Expected 'No issues found.' in output, got: {}",
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
    assert!(stdout.contains("Issue Queue"));
    assert!(stdout.contains("2 total"));
    assert!(stdout.contains("atelier-"));
    assert!(stdout.contains("[task] atelier-"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));

    let (success, quiet_out, stderr) = run_atelier(dir.path(), &["--quiet", "issue", "list"]);
    assert!(success, "quiet issue list failed: {stderr}");
    assert!(!quiet_out.contains("Issue Queue"));
    assert!(!quiet_out.contains("Issue 1"));
    assert_eq!(quiet_out.lines().count(), 2);
    assert!(quiet_out.lines().all(|line| line.starts_with("atelier-")));
}

#[test]
fn test_issue_list_orders_visible_blockers_before_blocked_rows() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Blocked work", "-p", "high"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Direct blocker", "-p", "low"],
    );
    let blocked_id = issue_id_by_title(dir.path(), "Blocked work");
    let blocker_id = issue_id_by_title(dir.path(), "Direct blocker");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "block", &blocked_id, &blocker_id]);
    assert!(success, "issue block failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success, "issue list failed: {stderr}");
    let blocker_pos = stdout.find("Direct blocker").unwrap_or(usize::MAX);
    let blocked_pos = stdout.find("Blocked work").unwrap_or(usize::MAX);
    assert!(
        blocker_pos < blocked_pos,
        "blocker should appear before blocked work:\n{stdout}"
    );
    assert!(stdout.contains("ready [task]"), "{stdout}");
    assert!(stdout.contains("blocked [task]"), "{stdout}");
    assert!(!stdout.contains("todo/todo"), "{stdout}");
    assert!(stdout.contains(&format!("details: atelier issue blocked {blocked_id}")));
}

#[test]
fn test_issue_list_ready_excludes_blocked_and_quiet_matches_human_order() {
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
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "block", &blocked_id, &blocker_id]);
    assert!(success, "issue block failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success, "issue list --ready failed: {stderr}");
    assert!(stdout.contains("High ready"), "{stdout}");
    assert!(stdout.contains("Low ready"), "{stdout}");
    assert!(stdout.contains("Ready blocker"), "{stdout}");
    assert!(!stdout.contains("Blocked ready"), "{stdout}");
    assert!(
        stdout.find("High ready").unwrap() < stdout.find("Low ready").unwrap(),
        "high priority ready work should sort before low priority work:\n{stdout}"
    );

    let (success, quiet, stderr) =
        run_atelier(dir.path(), &["--quiet", "issue", "list", "--ready"]);
    assert!(success, "quiet issue list --ready failed: {stderr}");
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
        &["issue", "block", &implementation_id, &contract_id],
    );
    run_atelier(
        dir.path(),
        &["issue", "block", &external_child_id, &external_blocker_id],
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
            "blocked {external_child_id} [todo] medium - External blocked child (1 blocker; details: atelier issue blocked {external_child_id})"
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
        text.replace("No description provided.", "Description")
            .replace(
                "Outcome was not specified.",
                "The issue show command renders parsed sections.\n\n## Evidence\n\n- Show output contains the section headings.\n\n## Notes\n\nCLI display context.",
            )
    });

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", &issue_id]);

    assert!(success);
    assert!(stdout.contains("Test issue"));
    assert!(stdout.contains("Description"));
    assert!(stdout.contains("Outcome"));
    assert!(stdout.contains("The issue show command renders parsed sections."));
    assert!(stdout.contains("Evidence"));
    assert!(stdout.contains("- Show output contains the section headings."));
    assert!(stdout.contains("Notes"));
    assert!(stdout.contains("CLI display context."));
    assert!(!stdout.contains("Acceptance Criteria"));
}

#[test]
fn test_issue_show_surfaces_evidence_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Evidence status issue"]);
    let issue_id = issue_id_by_title(dir.path(), "Evidence status issue");
    edit_canonical_record(dir.path(), "issues", &issue_id, |text| {
        text.replace(
            "No description provided.",
            "Exercise issue show evidence state.",
        )
        .replace(
            "Outcome was not specified.",
            "Issue show renders attached proof state and next commands.\n\n## Evidence\n\n- Manual check: issue show and transition options report missing and attached validation evidence.",
        )
    });
    commit_all(dir.path(), "evidence status issue setup");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "issue show without evidence failed: {stderr}");
    assert!(!stdout.contains("Evidence Status"));
    assert!(!stdout.contains("Attached Proof: missing - no validating evidence link found"));

    move_issue_to_validation(dir.path(), &issue_id);
    let (success, transitions, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(
        success,
        "transition options without evidence failed: {stderr}"
    );
    assert!(transitions.contains("close"));
    assert!(transitions.contains("fail  evidence.attached"));
    assert!(transitions.contains("expected at least 1 validating evidence record(s); found 0"));
    assert!(
        transitions.contains("Hint: record proof with `atelier evidence record --target issue/<id> --kind validation \"...\"`"),
        "{transitions}"
    );

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
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
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
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
        run_atelier(dir.path(), &["issue", "block", parent_key, related_key]);
    assert!(success, "relate by partial keys failed: {stderr}");
    assert!(stdout.contains(&parent_id));
    assert!(stdout.contains(&related_id));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "blocked", parent_key]);
    assert!(success, "related by partial key failed: {stderr}");
    assert!(stdout.contains(&related_id));

    migrate_default_issue_workflow(dir.path());
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", parent_key, "--options"],
    );
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
  "schema_version": 1,
  "title": "Partial key bundle apply",
  "resources": {{
    "issues": [
      {{
        "client_ref": "issue.partial",
        "title": "Partial key dependent",
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
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "blocked", &dependent_id]);
    assert!(success, "issue blocked failed: {stderr}");
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

    let (success, _stdout, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "custom issue rebuild failed: {stderr}");
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
        show.contains("Status:   ready"),
        "declared mission should use its configured initial status: {show}"
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
  "schema_version": 1,
  "title": "Custom type bundle",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.incident",
        "title": "Bundled incident",
        "issue_type": "incident",
        "priority": "high"
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
    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Validation task"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Validation task");
    edit_canonical_issue(dir.path(), &issue_id, |markdown| {
        replace_front_matter_scalar(&markdown, "status", "validation")
    });
    let (success, _stdout, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", &issue_id, "--issue-type", "spike"],
    );
    assert!(!success, "incompatible issue type update should fail");
    assert!(
        stderr.contains("status 'validation' that is not allowed")
            || stderr.contains("not allowed by the workflow policy")
            || stderr.contains("status 'validation' which is not valid"),
        "{stderr}"
    );

    let issue_text = read_canonical_record(dir.path(), "issues", &issue_id);
    assert!(issue_text.contains("issue_type: \"task\""), "{issue_text}");
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
  "schema_version": 1,
  "title": "Invalid bundle",
  "resources": {
    "issues": [],
    "plans": [{ "client_ref": "plan.invalid", "title": "No plan resources" }],
    "milestones": [{ "client_ref": "milestone.invalid", "title": "No milestones" }]
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
  "schema_version": 1,
  "title": "Duplicate client ref bundle",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.duplicate",
        "title": "Duplicate one",
        "issue_type": "task",
        "priority": "high"
      },
      {
        "client_ref": "issue.duplicate",
        "title": "Duplicate two",
        "issue_type": "task",
        "priority": "high"
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
  "schema_version": 1,
  "title": "Missing client ref bundle",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.ref-user",
        "title": "Missing reference user",
        "issue_type": "task",
        "priority": "high",
        "depends_on": [{ "client_ref": "issue.missing" }]
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
  "schema_version": 1,
  "title": "Invalid status bundle",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.invalid-status",
        "title": "Invalid status",
        "issue_type": "task",
        "priority": "high",
        "status": "not_real"
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
    run_atelier(dir.path(), &["issue", "block", &target_id, &blocking_id]);
    run_atelier(dir.path(), &["issue", "block", &downstream_id, &target_id]);
    run_atelier(dir.path(), &["issue", "note", &target_id, "Recent note"]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &target_id]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Target issue"));
    assert!(stdout.contains("Status:   todo"));
    assert!(stdout.contains("Type:"));
    assert!(stdout.contains("Priority: medium"));
    assert!(stdout.contains(&format!(".atelier/issues/{target_id}.md")));
    assert!(stdout.contains("1 total | status: todo=1 | priority: low=1"));
    assert!(stdout.contains("Blocking issue"));
    assert!(stdout.contains("(open blocker)"));
    assert!(stdout.contains("Downstream issue"));
    assert!(stdout.contains("Recent Activity"));
    assert!(stdout.contains("Recent note"));
    assert!(stdout.contains("Next Commands"));
    assert!(stdout.contains("atelier issue note"));
    assert!(!stdout.contains("atelier issue com"));
    assert!(stdout.contains("atelier issue transition"));
}

#[test]
fn test_issue_show_recent_activity_humanizes_structured_bodies() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Activity issue"]);
    let issue_id = issue_id_by_title(dir.path(), "Activity issue");
    run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Recent Activity"), "{stdout}");
    assert!(
        stdout.contains("Transition start: todo -> in_progress"),
        "{stdout}"
    );
    assert!(!stdout.contains("transition: \"start\""), "{stdout}");
}

#[test]
fn test_issue_show_summarizes_dirty_checkout_state() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Dirty checkout issue",
            "--issue-type",
            "epic",
        ],
    );
    let issue_id = issue_id_by_title(dir.path(), "Dirty checkout issue");
    std::fs::write(dir.path().join("tracked.txt"), "clean").unwrap();
    commit_all(dir.path(), "baseline");
    std::fs::write(dir.path().join("tracked.txt"), "dirty").unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Checkout"), "{stdout}");
    assert!(
        stdout.contains("State:    dirty checkout: 1 path:"),
        "{stdout}"
    );
    assert!(stdout.contains("tracked.txt"), "{stdout}");
}

#[test]
fn test_issue_show_human_shape_exposes_actionable_context() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "JSON issue"]);
    let issue_id = issue_id_by_title(dir.path(), "JSON issue");
    edit_canonical_record(dir.path(), "issues", &issue_id, |text| {
        text.replace("No description provided.", "JSON description")
    });

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("JSON issue"));
    assert!(stdout.contains("Description"));
    assert!(stdout.contains("JSON description"));
    assert!(stdout.contains("Blocked by"));
    assert!(stdout.contains("Blocking"));
    assert!(stdout.contains("Recent Activity"));
    assert!(stdout.contains("Next Commands"));
}

#[test]
fn test_issue_show_reads_detail_body_from_record_store() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Canonical detail issue"]);
    let issue_id = issue_id_by_title(dir.path(), "Canonical detail issue");
    edit_canonical_record(dir.path(), "issues", &issue_id, |text| {
        text.replace("No description provided.", "Canonical Markdown body")
    });
    let conn = rusqlite::Connection::open(dir.path().join(".atelier/runtime/state.db")).unwrap();
    conn.execute(
        "UPDATE issues SET description = 'SQLite shadow body' WHERE id = ?1",
        [&issue_id],
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Canonical Markdown body"));
    assert!(!stdout.contains("SQLite shadow body"));
}

#[test]
fn test_issue_sections_are_canonical_after_direct_markdown_edit_and_rebuild() {
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
    let edited_evidence = "- `atelier rebuild` refreshes derived search text.";
    let issue_text = std::fs::read_to_string(&issue_path)
        .unwrap()
        .replace("No description provided.", edited_body)
        .replace(
            "Outcome was not specified.",
            &format!("{edited_outcome}\n\n## Evidence\n\n{edited_evidence}"),
        );
    std::fs::write(&issue_path, issue_text).unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains(edited_body), "{stdout}");
    assert!(stdout.contains(edited_outcome), "{stdout}");
    assert!(stdout.contains(edited_evidence), "{stdout}");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["search", "projected from issue body"]);
    assert!(success, "search failed: {stderr}");
    assert!(stdout.contains(&issue_id), "{stdout}");

    let conn = rusqlite::Connection::open(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let projected_text: String = conn
        .query_row(
            "SELECT description FROM issues WHERE id = ?1",
            [&issue_id],
            |row| row.get(0),
        )
        .unwrap();
    assert!(projected_text.contains(edited_body));
    assert!(!projected_text.contains(edited_outcome));
    assert!(!projected_text.contains("## Description"));
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
            "--kind",
            "test",
            "Canonical evidence summary",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    let evidence_id = record_id_by_title(dir.path(), "evidence", "Canonical evidence summary");

    let conn = rusqlite::Connection::open(dir.path().join(".atelier/runtime/state.db")).unwrap();
    conn.execute(
        "UPDATE records SET title = 'SQLite mission title', status = 'sqlite_status' WHERE id = ?1",
        [mission_id.as_str()],
    )
    .unwrap();
    conn.execute(
        "UPDATE records SET title = 'SQLite evidence title', status = 'sqlite_status' WHERE id = ?1",
        [evidence_id.as_str()],
    )
    .unwrap();

    let (success, mission_out, stderr) = run_atelier(dir.path(), &["issue", "show", &mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(mission_out.contains("Canonical mission body"));
    assert!(mission_out.contains("Canonical constraint"));
    assert!(!mission_out.contains("SQLite mission title"));
    assert!(!mission_out.contains("sqlite_status"));

    let (success, evidence_out, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &evidence_id]);
    assert!(success, "evidence show failed: {stderr}");
    assert!(evidence_out.contains("Canonical evidence summary"));
    assert!(evidence_out.contains("Status:      recorded"));
    assert!(evidence_out.contains("Kind:        test"));
    assert!(!evidence_out.contains("SQLite evidence summary"));
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
fn test_issue_search_reads_payloads_from_record_store_and_activity() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Canonical search issue"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Canonical search issue");
    set_issue_description(dir.path(), &issue_id, "canonical body needle");
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild after description edit failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "note", &issue_id, "canonical activity needle"],
    );
    assert!(success, "issue note failed: {stderr}");

    let conn = rusqlite::Connection::open(dir.path().join(".atelier/runtime/state.db")).unwrap();
    conn.execute(
        "UPDATE issues SET description = 'sqlite body needle' WHERE id = ?1",
        [&issue_id],
    )
    .unwrap();

    let (success, body_out, stderr) = run_atelier(dir.path(), &["search", "canonical body needle"]);
    assert!(success, "canonical body search failed: {stderr}");
    assert!(body_out.contains("Canonical search issue"));

    let (success, activity_out, stderr) =
        run_atelier(dir.path(), &["search", "canonical activity needle"]);
    assert!(success, "canonical activity search failed: {stderr}");
    assert!(activity_out.contains("Canonical search issue"));

    let (success, shadow_body_out, stderr) =
        run_atelier(dir.path(), &["search", "sqlite body needle"]);
    assert!(success, "sqlite shadow body search failed: {stderr}");
    assert!(shadow_body_out.contains("No issues found"));
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
fn test_show_issue_prefers_activity_sidecars_for_recent_activity() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Activity issue"]);
    let issue_id = issue_id_by_title(dir.path(), "Activity issue");
    let activity_dir = dir
        .path()
        .join(".atelier")
        .join("issues")
        .join(format!("{issue_id}.activity"));
    std::fs::create_dir_all(&activity_dir).unwrap();
    std::fs::write(
        activity_dir.join("20260610T181920123456Z.md"),
        format!(
            "---\nschema: \"atelier.activity\"\nschema_version: 1\nid: \"20260610T181920123456Z\"\nsubject_kind: \"issue\"\nsubject_id: \"{issue_id}\"\nevent_type: \"comment\"\nactor: \"tester\"\ncreated_at: \"2026-06-10T18:19:20.123456Z\"\nsummary: \"Canonical activity\"\n---\n\nSidecar body\n"
        ),
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Canonical activity"));
    assert!(stdout.contains("Sidecar body"));
    assert!(!stdout.contains("Legacy note"));
}

#[test]
fn test_history_repo_wide_supports_filters_bounded_output_and_drill_downs() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "First issue"]);
    run_atelier(dir.path(), &["issue", "create", "Second issue"]);
    let first = issue_id_by_title(dir.path(), "First issue");
    let second = issue_id_by_title(dir.path(), "Second issue");
    write_activity_fixture(
        dir.path(),
        &first,
        "20260610T181920123456Z",
        "comment",
        "First comment",
        "First body",
    );
    write_activity_fixture(
        dir.path(),
        &second,
        "20260610T181921123456Z",
        "evidence_attached",
        "Evidence attached",
        "evidence_id: \"ev-1\"\nresult: \"pass\"",
    );
    write_activity_fixture(
        dir.path(),
        &second,
        "20260610T181922123456Z",
        "comment",
        "Second comment",
        "Second body",
    );

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--event-kind",
            "evidence_attached",
            "--limit",
            "1",
        ],
    );
    assert!(success, "history failed: {stderr}");
    assert!(stdout.contains("History"));
    assert!(stdout.contains("Scope:          repository"));
    assert!(stdout.contains("Source:         canonical .atelier"));
    assert!(stdout.contains("Ordering:       newest first"));
    assert!(stdout.contains("Showing:        1 of 1 matching events"));
    assert!(stdout.contains("Evidence attached"));
    assert!(!stdout.contains("First comment"));
    assert!(stdout.contains("Next Commands"));
    assert!(stdout.contains("atelier issue show <id>"));
    assert!(stdout.contains("atelier history --mission <id>"));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--issue",
            first.as_str(),
            "--event-kind",
            "comment",
            "--since",
            "2026-06-10",
        ],
    );
    assert!(success, "filtered history failed: {stderr}");
    assert!(stdout.contains("First comment"));
    assert!(!stdout.contains("Evidence attached"));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["history", "--event-kind", "comment", "--limit", "1"],
    );
    assert!(success, "history failed: {stderr}");
    assert!(stdout.contains("Second comment"));
    assert!(!stdout.contains("First comment"));
    assert!(stdout.contains("Omitted:"));
}

#[test]
fn test_history_mission_scope_includes_linked_work_descendants_and_evidence() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "History mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "History mission");

    run_atelier(
        dir.path(),
        &["issue", "create", "History epic", "--issue-type", "epic"],
    );
    let epic_id = issue_id_by_title(dir.path(), "History epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "History child", "--parent", &epic_id],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "History child");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "link", &mission_id, &epic_id]);
    assert!(success, "mission add-work failed: {stderr}");
    let (success, note_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "note", &mission_id, "Mission note body"],
    );
    assert!(success, "mission note failed: {stderr}");
    assert!(note_out.contains("Added note to issue"));
    write_activity_fixture(
        dir.path(),
        &child_id,
        "20260610T191920123456Z",
        "note",
        "Child note",
        "Child body",
    );
    let (success, _evidence_out, stderr) = run_atelier(
        dir.path(),
        &["evidence", "record", "--kind", "test", "Cargo test passed"],
    );
    assert!(success, "evidence record failed: {stderr}");
    let evidence_id = record_id_by_title(dir.path(), "evidence", "Cargo test passed");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "attach",
            &evidence_id,
            "issue",
            child_id.as_str(),
        ],
    );
    assert!(success, "evidence attach failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--mission",
            mission_id.as_str(),
            "--event-kind",
            "evidence_attached",
        ],
    );

    assert!(success, "history failed: {stderr}");
    assert!(stdout.contains(&format!("Scope:          mission {mission_id}")));
    assert!(stdout.contains(&format!("Attached evidence {evidence_id}")));
    assert!(stdout.contains(&child_id));
    assert!(stdout.contains(&format!("atelier issue show {mission_id}")));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--mission",
            mission_id.as_str(),
            "--event-kind",
            "note",
        ],
    );
    assert!(success, "mission note history failed: {stderr}");
    assert!(stdout.contains("Mission note body"));
    assert!(stdout.contains(&mission_id));
    assert!(stdout.contains("Child note"));
    assert!(stdout.contains(&child_id));
}

#[test]
fn test_history_issue_scope_defaults_single_issue_and_can_include_descendants() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent history", "--issue-type", "epic"],
    );
    let parent_id = issue_id_by_title(dir.path(), "Parent history");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Child history", "--parent", &parent_id],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Child history");
    write_activity_fixture(
        dir.path(),
        &parent_id,
        "20260610T181920123456Z",
        "note",
        "Parent note",
        "Parent body",
    );
    write_activity_fixture(
        dir.path(),
        &child_id,
        "20260610T181921123456Z",
        "note",
        "Child note",
        "Child body",
    );

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--issue",
            parent_id.as_str(),
            "--event-kind",
            "note",
        ],
    );
    assert!(success, "issue history failed: {stderr}");
    assert!(stdout.contains(&format!("Scope:          issue {parent_id}")));
    assert!(stdout.contains("Parent note"));
    assert!(!stdout.contains("Child note"));
    assert!(stdout.contains(&format!(
        "atelier history --issue {parent_id} --include-descendants"
    )));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--issue",
            parent_id.as_str(),
            "--include-descendants",
            "--event-kind",
            "note",
        ],
    );
    assert!(success, "descendant issue history failed: {stderr}");
    assert!(stdout.contains("Parent note"));
    assert!(stdout.contains("Child note"));
}

#[test]
fn test_history_empty_states_and_invalid_limit() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["history"]);
    assert!(success, "empty history failed: {stderr}");
    assert!(stdout.contains("No canonical history found for repository."));
    assert!(stdout.contains("Source:"));
    assert!(stdout.contains("Next Commands"));

    run_atelier(dir.path(), &["issue", "create", "Filtered history"]);
    let issue_id = issue_id_by_title(dir.path(), "Filtered history");
    write_activity_fixture(
        dir.path(),
        &issue_id,
        "20260610T181920123456Z",
        "note",
        "Filter note",
        "Filter body",
    );
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--issue",
            issue_id.as_str(),
            "--event-kind",
            "evidence_attached",
        ],
    );
    assert!(success, "filtered empty history failed: {stderr}");
    assert!(stdout.contains("History exists for"));
    assert!(stdout.contains("no events matched the current filters"));

    let (success, _, stderr) = run_atelier(dir.path(), &["history", "--limit", "0"]);
    assert!(!success, "zero limit should fail");
    assert!(stderr.contains("--limit must be greater than 0"));
}

#[test]
fn test_show_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", "999"]);

    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

fn write_activity_fixture(
    dir: &Path,
    issue_id: &str,
    activity_id: &str,
    event_type: &str,
    summary: &str,
    body: &str,
) {
    let activity_dir = dir
        .join(".atelier")
        .join("issues")
        .join(format!("{issue_id}.activity"));
    std::fs::create_dir_all(&activity_dir).unwrap();
    std::fs::write(
        activity_dir.join(format!("{activity_id}.md")),
        format!(
            "---\nschema: \"atelier.activity\"\nschema_version: 1\nid: \"{activity_id}\"\nsubject_kind: \"issue\"\nsubject_id: \"{issue_id}\"\nevent_type: \"{event_type}\"\nactor: \"tester\"\ncreated_at: \"{}-{}-{}T{}:{}:{}.123456Z\"\nsummary: \"{summary}\"\n---\n\n{body}\n",
            &activity_id[0..4],
            &activity_id[4..6],
            &activity_id[6..8],
            &activity_id[9..11],
            &activity_id[11..13],
            &activity_id[13..15],
        ),
    )
    .unwrap();
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

#[test]
fn test_import_beads_jsonl_fixture_round_trip() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let fixture_path = dir.path().join("issues.manual.jsonl");
    std::fs::write(
        &fixture_path,
        include_str!("../fixtures/beads/issues.manual.jsonl"),
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["import-beads", fixture_path.to_str().unwrap()],
    );
    assert!(success, "import-beads failed: {stderr}");
    assert!(stdout.contains("source records: 3"));
    assert!(stdout.contains("imported issues: 3"));
    assert!(stdout.contains("parent-child relationships: 2"));
    assert!(stdout.contains("blocking relationships: 1"));
    assert!(dir
        .path()
        .join(".atelier")
        .join("issues")
        .join("atelier-0001.md")
        .exists());

    let (_, list_out, _) = run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(list_out.contains("Mission: Replace Beads"));
    assert!(list_out.contains("Dogfood Atelier"));

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "atelier-0003"]);
    assert!(show_out.contains("Parent: atelier-0001"));
    assert!(show_out.contains("Blocked by"));
    assert!(show_out.contains("atelier-0002"));
    assert!(show_out.contains("(open blocker)"));
    assert!(show_out.contains("Outcome"));
    assert!(show_out.contains("AGENTS.md declares Atelier as the tracker"));
    assert!(show_out.contains("Evidence"));
    assert!(show_out.contains("atelier import-beads <path>"));
    assert!(!show_out.contains("Acceptance Criteria"));

    let (updated, _, update_err) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            "atelier-0002",
            "--title",
            "Imported Beads issue updated",
        ],
    );
    assert!(updated, "update failed: {update_err}");
    close_issue_with_evidence(dir.path(), "atelier-0002", None);

    let (_, closed_show, _) = run_atelier(dir.path(), &["issue", "show", "atelier-0002"]);
    assert!(closed_show.contains("Imported Beads issue updated"));
    assert!(closed_show.contains("Status:   done"));

    let (fresh, _, fresh_err) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(
        fresh,
        "export --check validates canonical Markdown/projection state, not SQLite-only drift: {fresh_err}"
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
fn test_issue_mutations_create_activity_sidecars() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Activity issue"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Activity issue");
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    for (kind, body) in [
        ("human", "Plain comment body"),
        ("note", "Operator note body"),
        ("plan", "Plan body"),
        ("handoff", "Handoff body"),
    ] {
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["issue", "note", &issue_id, body, "--kind", kind],
        );
        assert!(success, "issue note {kind} failed: {stderr}");
    }

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
    assert!(stderr.contains("Invalid comment kind 'decision'"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "note", &issue_id, "Append note body"],
    );
    assert!(success, "issue note failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "update", &issue_id, "--claim"]);
    assert!(!success, "issue claim should be rejected");
    assert!(stderr.contains("unexpected argument '--claim'"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &issue_id,
            "--title",
            "Activity issue renamed",
            "--priority",
            "high",
            "--label",
            "activity-label",
        ],
    );
    assert!(success, "issue update fields failed: {stderr}");

    move_issue_to_validation(dir.path(), &issue_id);
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
    assert!(success, "issue transition close reason failed: {stderr}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(&activities, "comment", &["Plain comment body"]);
    assert_activity_contains(&activities, "note", &["Operator note body"]);
    assert_activity_contains(&activities, "note", &["Append note body"]);
    assert_activity_contains(&activities, "plan", &["Plan body"]);
    assert_activity_contains(&activities, "handoff", &["Handoff body"]);
    assert_activity_contains(
        &activities,
        "field_changed",
        &[
            "field: \"title\"",
            "old: \"Activity issue\"",
            "new: \"Activity issue renamed\"",
        ],
    );
    assert_activity_contains(
        &activities,
        "field_changed",
        &["field: \"priority\"", "old: \"medium\"", "new: \"high\""],
    );
    assert_activity_contains(
        &activities,
        "field_changed",
        &["field: \"labels\"", "new: \"activity-label\""],
    );
    assert_activity_contains(
        &activities,
        "transition_applied",
        &["transition: \"close\"", "to: \"done\""],
    );
    assert_activity_contains(&activities, "close_reason", &["Close reason body"]);
}

#[test]
fn test_issue_show_json_recovers_activity_fields_after_rebuild() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Rebuild activity"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Rebuild activity");
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

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
    move_issue_to_validation(dir.path(), &issue_id);
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

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Close Reason"));
    assert!(stdout.contains("Canonical close"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["history", "--issue", &issue_id]);
    assert!(success, "history failed: {stderr}");
    assert!(stdout.contains("Canonical handoff"));
}

#[test]
fn test_issue_create_is_durable_without_manual_export() {
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

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed after create: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed after rebuild: {stderr}");
    assert!(stdout.contains("Create-only durable"));
    assert!(stdout.contains("Created body"));
    assert!(stdout.contains("Priority: high"));
}

#[test]
fn test_issue_mutations_are_durable_without_manual_export() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Mutation source"]);
    assert!(success, "source create failed: {stderr}");
    let source_id = issue_id_by_title(dir.path(), "Mutation source");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Mutation target"]);
    assert!(success, "target create failed: {stderr}");
    let target_id = issue_id_by_title(dir.path(), "Mutation target");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &source_id,
            "--title",
            "Mutation source updated",
            "--priority",
            "high",
        ],
    );
    assert!(success, "update failed: {stderr}");

    for args in [
        vec!["issue", "update", &source_id, "--label", "remove-me"],
        vec!["issue", "update", &source_id, "--remove-label", "remove-me"],
        vec!["issue", "update", &source_id, "--label", "keep-me"],
        vec!["issue", "block", &source_id, &target_id],
        vec!["issue", "unblock", &source_id, &target_id],
        vec!["issue", "block", &source_id, &target_id],
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "{args:?} failed: {stderr}");
    }

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed before rebuild: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &source_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Mutation source updated"));
    assert!(stdout.contains("Priority: high"));
    assert!(stdout.contains("keep-me"));
    assert!(stdout.contains(&target_id));

    let source_text = read_canonical_record(dir.path(), "issues", &source_id);
    assert!(!source_text.contains("- \"remove-me\""));
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
    let (success, _, _) = run_atelier(dir.path(), &["issue", "block", &blocked_id, &blocker_id]);

    assert!(success);

    let (_, blocked_out, _) = run_atelier(dir.path(), &["issue", "blocked"]);
    assert!(blocked_out.contains("Blocked issue"));
}

#[test]
fn test_issue_list_blocked_replaces_blocked_helper() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    let blocked_id = issue_ref(dir.path(), 1);
    let blocker_id = issue_ref(dir.path(), 2);
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "block", &blocked_id, &blocker_id]);
    assert!(success, "issue block failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--blocked"]);
    assert!(success, "issue list --blocked failed: {stderr}");
    assert!(stdout.contains("Blocked issue"));
    assert!(stdout.contains("blocked"));
    assert!(stdout.contains("1 blocker"));
    assert!(stdout.contains(&format!("details: atelier issue blocked {blocked_id}")));
    assert!(!stdout.contains(&format!("blocked by {blocker_id}")));

    let (success, quiet, stderr) =
        run_atelier(dir.path(), &["--quiet", "issue", "list", "--blocked"]);
    assert!(success, "quiet issue list --blocked failed: {stderr}");
    assert_eq!(quiet.trim(), blocked_id);
}

#[test]
fn test_unblock_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    let blocked_id = issue_ref(dir.path(), 1);
    let blocker_id = issue_ref(dir.path(), 2);
    run_atelier(dir.path(), &["issue", "block", &blocked_id, &blocker_id]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "unblock", &blocked_id, &blocker_id]);

    assert!(success);

    let (_, blocked_out, _) = run_atelier(dir.path(), &["issue", "blocked"]);
    assert!(!blocked_out.contains("Blocked issue"));
}

#[test]
fn test_issue_blocker_mutations_are_durable_without_manual_export() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Alias blocked"]);
    assert!(success, "blocked create failed: {stderr}");
    let blocked_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Alias blocker"]);
    assert!(success, "blocker create failed: {stderr}");
    let blocker_id = issue_ref(dir.path(), 2);

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "block", &blocked_id, &blocker_id]);
    assert!(success, "issue block failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed after issue block: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild after issue block failed: {stderr}");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "blocked", &blocked_id]);
    assert!(success, "issue blocked after issue block failed: {stderr}");
    assert!(stdout.contains(&blocker_id), "{stdout}");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "unblock", &blocked_id, &blocker_id]);
    assert!(success, "issue unblock failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed after issue unblock: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild after issue unblock failed: {stderr}");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "blocked", &blocked_id]);
    assert!(
        success,
        "issue blocked after issue unblock failed: {stderr}"
    );
    assert!(
        stdout.contains("No dependencies found."),
        "dependency should be removed after rebuild: {stdout}"
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
    run_atelier(dir.path(), &["issue", "block", &blocked_id, &blocker_id]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list", "--ready"]);

    assert!(success);
    assert!(stdout.contains("2 total"));
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
fn test_quiet_issue_list_ready_outputs_ids_only() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Ready issue"]);

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["--quiet", "issue", "list", "--ready"]);

    assert!(success, "quiet ready list failed: {stderr}");
    assert_eq!(stdout.lines().count(), 1);
    assert!(stdout.lines().all(|line| line.starts_with("atelier-")));
    assert!(!stdout.contains("Ready issue"));
}

#[test]
fn test_issue_list_ready_rejects_closed_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _stderr) = run_atelier(
        dir.path(),
        &["issue", "list", "--ready", "--status", "closed"],
    );

    assert!(!success);
}

#[test]
fn test_issue_list_ready_treats_internal_epic_blockers_as_ready() {
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
    run_atelier(dir.path(), &["issue", "block", &sequenced_id, &ready_id]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);

    assert!(success, "ready list failed: {stderr}");
    assert!(stdout.contains("Parent epic"));
    assert!(stdout.contains("Ready child"));
    assert!(!stdout.contains("Sequenced child"));
}

#[test]
fn test_issue_list_ready_still_shows_ready_children_when_another_issue_is_active() {
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
    run_atelier(dir.path(), &["issue", "block", &sequenced_id, &ready_id]);
    run_atelier(dir.path(), &["issue", "create", "Active item"]);
    let active_id = issue_ref(dir.path(), 4);

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &active_id, "start"]);
    assert!(success, "start active issue failed: {stderr}");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(
        status_out.contains("Ready work:    2"),
        "status should still count the parent epic and ready child:\n{status_out}"
    );

    let (success, ready_out, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success, "ready list failed: {stderr}");
    assert!(ready_out.contains("Parent epic"), "{ready_out}");
    assert!(ready_out.contains("Ready child"), "{ready_out}");
    assert!(!ready_out.contains("Sequenced child"), "{ready_out}");
    assert!(!ready_out.contains("Active item"), "{ready_out}");
}

#[test]
fn test_issue_list_ready_marks_blocked_parent_headers_as_context() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Blocked parent epic",
            "--issue-type",
            "epic",
        ],
    );
    let parent_id = issue_ref(dir.path(), 1);
    run_atelier(
        dir.path(),
        &["issue", "create", "Ready child", "--parent", &parent_id],
    );
    run_atelier(dir.path(), &["issue", "create", "Outside blocker"]);
    let child_id = issue_ref(dir.path(), 2);
    let blocker_id = issue_ref(dir.path(), 3);
    run_atelier(dir.path(), &["issue", "block", &parent_id, &blocker_id]);

    let (success, ready_out, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success, "ready list failed: {stderr}");
    assert!(
        ready_out.contains("Blocked parent epic (shown for context; blocked through parent)"),
        "{ready_out}"
    );
    assert!(ready_out.contains("blocked by 1 external blocker"));
    assert!(ready_out.contains(&format!("details: atelier issue blocked {parent_id}")));
    assert!(!ready_out.contains(&format!("blocked by {blocker_id}")));
    assert!(ready_out.contains(&format!("{child_id} - Ready child")));

    let (success, blocked_out, stderr) = run_atelier(dir.path(), &["issue", "blocked", &parent_id]);
    assert!(success, "blocked detail failed: {stderr}");
    assert!(blocked_out.contains(&blocker_id), "{blocked_out}");
}

#[test]
fn test_issue_list_marks_external_epic_blockers_by_id() {
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
    run_atelier(dir.path(), &["issue", "block", &child_id, &blocker_id]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list"]);

    assert!(success, "issue list failed: {stderr}");
    assert!(stdout.contains("Parent epic"));
    assert!(stdout.contains("1 blocker"));
    assert!(stdout.contains("details: atelier issue blocked"));
    assert!(!stdout.contains(&format!("blocked by {blocker_id}")));
    assert!(!stdout.contains("open blocker"));
}

#[test]
fn test_issue_update_issue_type_persists_through_rebuild() {
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

    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");
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

fn write_incident_issue_type_workflow(dir: &std::path::Path) {
    let workflow_path = dir.join(".atelier/workflow.yaml");
    let workflow = std::fs::read_to_string(&workflow_path)
        .expect("failed to read workflow policy")
        .replace(
            "  task: { label: Task }\n  validation: { label: Validation }",
            "  task: { label: Task }\n  incident: { label: Incident }\n  validation: { label: Validation }",
        )
        .replace(
            "applies_to: [bug, feature, task]",
            "applies_to: [bug, feature, incident, task]",
        );
    std::fs::write(&workflow_path, workflow).expect("failed to write workflow policy");
}

fn write_workflow_without_mission_issue_type(dir: &std::path::Path) {
    let workflow_path = dir.join(".atelier/workflow.yaml");
    let workflow = std::fs::read_to_string(&workflow_path)
        .expect("failed to read workflow policy")
        .replace("  mission: { label: Mission }\n", "")
        .replace(
            r#"  mission_delivery:
    applies_to: [mission]
    initial_status: ready
    done_statuses: [closed]
    transitions:
      close:
        from: [ready, in_progress, validation]
        to: closed
        required_fields: [close_reason]
        description: "Closing requires configured objective validators to pass."
        validators:
          - objective.work_present
          - objective.work_terminal
          - objective.blockers_none_open
          - issue.sections_parseable
          - evidence.attached: { min_count: 1 }
          - validation.criteria_satisfied
          - lint.none_blocking
          - command_surface_current
          - ignored_tests_reviewed
          - tracker.current
          - git.on_base_branch
          - git.worktree_clean

"#,
            "",
        );
    std::fs::write(&workflow_path, workflow).expect("failed to write workflow policy");
}

// ==================== Session Tests ====================
