use super::support::*;

#[test]
fn test_workflow_init_is_removed_and_root_init_owns_starter_policy() {
    let dir = tempdir().unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["init"]);
    assert!(success, "root init failed: {stderr}");
    assert!(stdout.contains(".atelier/workflow.yaml"));
    assert!(stdout.contains("atelier lint"));

    let policy_path = dir.path().join(".atelier").join("workflow.yaml");
    let policy = std::fs::read_to_string(&policy_path).unwrap();
    assert!(policy.contains("  todo:\n    category: todo"));
    assert!(policy.contains("  archived:\n    category: done"));
    assert!(policy.contains("    initial_status: todo"));
    assert!(policy.contains("    done_statuses: [done, archived]"));
    assert!(policy.contains("  lightweight_spike:"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "init"]);
    assert!(!success, "workflow init should be removed");
    assert!(stdout.is_empty(), "{stdout}");
    assert!(
        stderr.contains("unrecognized subcommand 'init'"),
        "{stderr}"
    );
    assert!(
        stderr.contains("`atelier workflow init` was removed"),
        "{stderr}"
    );
    assert!(stderr.contains("atelier init"), "{stderr}");
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
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
    assert!(!success, "workflow check should reject legacy status");
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
    assert!(stdout.contains("Issue Types:    7"));
    assert!(stdout.contains("Statuses:       7"));
    assert!(stdout.contains("Workflows:      3"));
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
        "# Agent Instructions\n\n- `atelier session start`\n",
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);

    assert!(
        !success,
        "workflow check should reject stale AGENTS command"
    );
    assert!(stdout.contains("Docs/Help Drift: detected"), "{stdout}");
    assert!(stdout.contains("AGENTS.md"), "{stdout}");
    assert!(stdout.contains("atelier session"), "{stdout}");
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
        "{}\n## Core\n\n- `atelier session start`\n",
        valid_command_surface_doc()
    );
    write_command_surface_doc(dir.path(), &surface);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);

    assert!(
        !success,
        "workflow check should reject removed command as normal guidance"
    );
    assert!(stdout.contains("Docs/Help Drift: detected"), "{stdout}");
    assert!(stdout.contains("atelier session"), "{stdout}");
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
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
    assert!(!success, "workflow check should reject status mismatch");
    assert!(
        stderr.contains("workflow_issue_status_invalid"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains(&issue_id), "stderr: {stderr}");
    assert!(stderr.contains("qa_hold"), "stderr: {stderr}");
    assert!(
        stderr.contains("allowed statuses: archived, blocked, done, in_progress, todo, validation"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_issue_orientation_uses_workflow_categories_and_exact_statuses() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for title in [
        "Todo category item",
        "Active status item",
        "Done category item",
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", title]);
        assert!(success, "issue create failed for {title}: {stderr}");
    }
    let todo_id = issue_id_by_title(dir.path(), "Todo category item");
    let active_id = issue_id_by_title(dir.path(), "Active status item");
    let done_id = issue_id_by_title(dir.path(), "Done category item");
    migrate_default_issue_workflow(dir.path());

    let active_path = canonical_issue_path(dir.path(), &active_id);
    let active_text = std::fs::read_to_string(&active_path).unwrap();
    std::fs::write(
        &active_path,
        active_text.replace("status: \"todo\"", "status: \"in_progress\""),
    )
    .unwrap();
    let done_path = canonical_issue_path(dir.path(), &done_id);
    let done_text = std::fs::read_to_string(&done_path).unwrap();
    std::fs::write(
        &done_path,
        done_text.replace("status: \"todo\"", "status: \"done\""),
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, todo_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "todo"]);
    assert!(success, "todo filter failed: {stderr}");
    assert!(todo_out.contains("Category: todo=1"), "{todo_out}");
    assert!(todo_out.contains("Status: todo=1"), "{todo_out}");
    assert!(todo_out.contains(&todo_id), "{todo_out}");
    assert!(!todo_out.contains(&active_id), "{todo_out}");
    assert!(!todo_out.contains(&done_id), "{todo_out}");

    let (success, active_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "in_progress"]);
    assert!(success, "in_progress filter failed: {stderr}");
    assert!(active_out.contains("Category: active=1"), "{active_out}");
    assert!(active_out.contains("Status: in_progress=1"), "{active_out}");
    assert!(active_out.contains("active/in_progress"), "{active_out}");
    assert!(active_out.contains(&active_id), "{active_out}");
    assert!(!active_out.contains(&todo_id), "{active_out}");

    let (success, active_category_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--category", "active"]);
    assert!(success, "active category filter failed: {stderr}");
    assert!(
        active_category_out.contains("Category: active=1"),
        "{active_category_out}"
    );
    assert!(
        active_category_out.contains("Status: in_progress=1"),
        "{active_category_out}"
    );
    assert!(
        active_category_out.contains("active/in_progress"),
        "{active_category_out}"
    );
    assert!(
        active_category_out.contains(&active_id),
        "{active_category_out}"
    );
    assert!(
        !active_category_out.contains(&todo_id),
        "{active_category_out}"
    );

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--category", "in_progress"]);
    assert!(!success, "in_progress category alias should be rejected");
    assert!(
        stderr.contains("Invalid issue category 'in_progress'"),
        "{stderr}"
    );

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "list", "--status", "active"]);
    assert!(
        !success,
        "active status/category ambiguity should be rejected"
    );
    assert!(stderr.contains("Invalid issue status 'active'"), "{stderr}");

    let (success, done_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "done"]);
    assert!(success, "done filter failed: {stderr}");
    assert!(done_out.contains("Category: done=1"), "{done_out}");
    assert!(done_out.contains("Status: done=1"), "{done_out}");
    assert!(done_out.contains("done/done"), "{done_out}");
    assert!(done_out.contains(&done_id), "{done_out}");

    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &active_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(
        show_out.contains(&format!(
            "{active_id} [task] active/in_progress - Active status item"
        )),
        "{show_out}"
    );
    assert!(show_out.contains("Status:   in_progress"), "{show_out}");
    assert!(show_out.contains("Category: active"), "{show_out}");

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Workflow mission"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Workflow mission");
    for issue_id in [&todo_id, &active_id, &done_id] {
        let (success, _, stderr) =
            run_atelier(dir.path(), &["mission", "add-work", &mission_id, issue_id]);
        assert!(success, "mission add-work failed for {issue_id}: {stderr}");
    }
    let (success, mission_out, stderr) = run_atelier(dir.path(), &["mission", "show", &mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(mission_out.contains("todo/todo"), "{mission_out}");
    assert!(mission_out.contains("active/in_progress"), "{mission_out}");
    assert!(mission_out.contains("done/done"), "{mission_out}");
}

#[test]
fn test_issue_ready_queue_requires_allowed_in_progress_transition() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Ready transition"]);
    assert!(success, "ready issue create failed: {stderr}");
    let ready_id = issue_id_by_title(dir.path(), "Ready transition");
    migrate_default_issue_workflow(dir.path());

    let (success, ready_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(success, "ready list failed: {stderr}");
    assert!(ready_out.contains(&ready_id), "{ready_out}");

    let policy_path = dir.path().join(".atelier").join("workflow.yaml");
    let policy = std::fs::read_to_string(&policy_path).unwrap();
    std::fs::write(
        &policy_path,
        policy.replacen(
            "      start:\n        from: [todo, blocked]\n        to: in_progress\n",
            "      start:\n        from: [todo, blocked]\n        to: in_progress\n        validators: [proof_attached]\n",
            1,
        ),
    )
    .unwrap();

    let (success, blocked_ready_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(
        success,
        "ready list with blocked transition should remain readable: {stderr}"
    );
    assert!(
        !blocked_ready_out.contains(&ready_id),
        "{blocked_ready_out}"
    );
    assert!(
        blocked_ready_out.contains("No issues found."),
        "{blocked_ready_out}"
    );

    let (success, options_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &ready_id, "--options"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(options_out.contains("start [blocked]"), "{options_out}");
    assert!(options_out.contains("proof_attached"), "{options_out}");
}

#[test]
fn test_lint_rejects_missing_required_issue_section() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Missing outcome lint"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Missing outcome lint");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(&issue_path, remove_issue_section(&markdown, "Outcome")).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(!success, "lint should fail for missing Outcome");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Outcome")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing structural diagnostic in stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stderr.contains("Lint failed"));
}

#[test]
fn test_lint_rejects_empty_required_issue_section() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Empty outcome lint"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Empty outcome lint");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    let invalid = markdown.replace("## Outcome\n\nOutcome was not specified.", "## Outcome\n\n");
    std::fs::write(&issue_path, invalid).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(!success, "lint should fail for empty Outcome");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Outcome")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing structural diagnostic in stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[test]
fn test_lint_rejects_missing_evidence_section() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Missing evidence lint"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Missing evidence lint");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(&issue_path, remove_issue_section(&markdown, "Evidence")).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(!success, "lint should fail for missing Evidence");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Evidence")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing Evidence diagnostic in stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stderr.contains("Lint failed"));
}

#[test]
fn test_lint_rejects_empty_evidence_section() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Empty evidence lint"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Empty evidence lint");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    let invalid = markdown.replace(
        "## Evidence\n\nEvidence was not specified.",
        "## Evidence\n\n",
    );
    std::fs::write(&issue_path, invalid).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(!success, "lint should fail for empty Evidence");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Evidence")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing Evidence diagnostic in stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[test]
fn test_lint_rejects_vague_evidence_even_when_notes_name_a_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let body = "## Description\n\nDescription\n\n## Outcome\n\nLint flags vague Evidence entries.\n\n## Evidence\n\n- Validation complete.\n\n## Notes\n\n- `cargo test --test cli_integration vague_evidence` passes.";

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Vague evidence lint",
            "--description",
            body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Vague evidence lint");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(!success, "lint should fail for vague Evidence");
    let transcript = format!("{stdout}\n{stderr}");
    for needle in [
        &issue_id,
        "section Evidence",
        ".atelier/issues/",
        "observable proof target",
        "command, transcript, evidence record, test, review artifact, file change, or manual check",
    ] {
        assert!(
            transcript.contains(needle),
            "vague Evidence diagnostic missing {needle:?}: {transcript}"
        );
    }
}

#[test]
fn test_lint_accepts_concrete_evidence_without_optional_notes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let body = "## Description\n\nDescription\n\n## Outcome\n\nLint accepts concrete Evidence entries without optional Notes.\n\n## Evidence\n\n- `cargo test --test cli_integration concrete_evidence` passes.\n- Manual check confirms the lint diagnostic names the issue, section, and path.";

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Concrete evidence lint",
            "--description",
            body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Concrete evidence lint");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(success, "lint should accept concrete Evidence: {stderr}");
    assert!(stdout.contains("Lint passed."));
}

#[test]
fn test_lint_rejects_duplicate_recognized_issue_heading() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Duplicate outcome lint"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Duplicate outcome lint");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    let invalid = markdown.replace(
        "## Evidence",
        "## Outcome\n\nSecond outcome should be rejected.\n\n## Evidence",
    );
    std::fs::write(&issue_path, invalid).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(!success, "lint should fail for duplicate Outcome");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Outcome")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing structural diagnostic in stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[test]
fn test_root_start_refuses_structurally_invalid_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Invalid work start"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Invalid work start");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(&issue_path, remove_issue_section(&markdown, "Outcome")).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(!success, "root start should refuse invalid issue");
    assert!(
        stderr.contains(&format!("issue {issue_id}"))
            && stderr.contains("section Outcome")
            && stderr.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing refusal diagnostic, stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[test]
fn test_issue_closeout_refuses_structurally_invalid_issue() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Invalid closeout"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Invalid closeout");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "workflow-ready invalid closeout");
    let (success, _, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "start failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_review"],
    );
    assert!(success, "request_review failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_validation"],
    );
    assert!(success, "request_validation failed: {stderr}");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(&issue_path, remove_issue_section(&markdown, "Outcome")).unwrap();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &issue_id, "--reason", "done"],
    );
    assert!(!success, "issue close should refuse invalid issue");
    assert!(
        stderr.contains(&format!("issue {issue_id}"))
            && stderr.contains("section Outcome")
            && stderr.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing closeout diagnostic, stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[test]
fn test_mission_closeout_enforces_gates_and_reopen_skips_close_validators() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Strict closeout"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Strict closeout");

    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Closeout work"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Closeout work");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", &mission_id, &work_id]);
    assert!(success, "mission add work failed: {stderr}");

    let (success, closeout_blocked_out, stderr) = run_atelier(
        dir.path(),
        &["mission", "close", &mission_id, "--reason", "done"],
    );
    assert!(!success, "mission close should fail with open work");
    assert!(closeout_blocked_out.contains("Mission closeout blocked"));
    assert!(closeout_blocked_out.contains("open mission work"));
    assert!(stderr.contains("mission closeout blocked"));

    close_issue_with_evidence(dir.path(), &work_id, Some("done"));
    commit_all(dir.path(), "ready to close");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", &mission_id, "--status", "closed"],
    );
    assert!(
        !success,
        "mission update --status closed should not be the ordinary closeout path"
    );
    assert!(stderr.contains("atelier mission close"));

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "close",
            &mission_id,
            "--reason",
            "ready to close",
        ],
    );
    assert!(
        success,
        "mission close should succeed after gates pass: {stderr}"
    );
    assert!(close_out.contains("Status: closed"));
    assert!(close_out.contains("Closeout Notes"));
    assert!(close_out.contains("- Close reason: ready to close"));
    commit_all(dir.path(), "closed mission");

    std::fs::write(dir.path().join("dirty-after-close.txt"), "dirty").unwrap();
    let (success, reopen_out, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", &mission_id, "--status", "ready"],
    );
    assert!(
        success,
        "mission reopen should skip closeout validators: {stderr}"
    );
    assert!(reopen_out.contains("Status: ready"));
}

#[test]
fn test_dirty_worktree_blocks_mission_closeout() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Dirty closeout"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Dirty closeout");
    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Dirty closeout work"]);
    assert!(success, "work create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Dirty closeout work");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", &mission_id, &work_id]);
    assert!(success, "mission add work failed: {stderr}");
    close_issue_with_evidence(dir.path(), &work_id, Some("done"));
    commit_all(dir.path(), "ready except dirty");
    std::fs::write(dir.path().join("untracked-closeout.txt"), "dirty").unwrap();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["mission", "close", &mission_id, "--reason", "done"],
    );
    assert!(!success, "dirty worktree must block mission closeout");
    assert!(stdout.contains("Mission closeout blocked"));
    assert!(stdout.contains("worktree: dirty"));
    assert!(stdout.contains("commit or remove untracked worktree changes"));
    assert!(stdout.contains("untracked-closeout.txt"));
    assert!(stderr.contains("mission closeout blocked"));
}

#[test]
fn test_mission_close_ignores_tracker_generated_issue_closeout_bookkeeping() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &["mission", "create", "Tracker bookkeeping closeout"],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Tracker bookkeeping closeout");

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Closeout bookkeeping work"],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Closeout bookkeeping work");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", &mission_id, &issue_id]);
    assert!(success, "mission add work failed: {stderr}");
    attach_pass_evidence(
        dir.path(),
        "mission",
        &mission_id,
        "mission bookkeeping proof",
    );
    close_issue_with_evidence(dir.path(), &issue_id, Some("done"));

    let dirty = git_status_short(dir.path());
    assert!(
        dirty.contains(&format!(".atelier/issues/{issue_id}.md")),
        "issue close should leave canonical bookkeeping dirty before commit:\n{dirty}"
    );
    assert!(
        dirty.contains(&format!(".atelier/issues/{issue_id}.activity/")),
        "issue close should leave canonical activity dirty before commit:\n{dirty}"
    );

    let (success, status_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", &mission_id]);
    assert!(
        success,
        "mission status should tolerate tracker bookkeeping: {stderr}"
    );
    assert!(status_out.contains("Closeout Gates"));
    assert!(status_out.contains("Worktree: clean - ignored"));
    assert!(status_out.contains(&format!(".atelier/issues/{issue_id}.md")));
    assert!(status_out.contains(&format!(".atelier/issues/{issue_id}.activity/")));

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &["mission", "close", &mission_id, "--reason", "done"],
    );
    assert!(
        success,
        "mission close should ignore tracker-generated closeout bookkeeping: {stderr}"
    );
    assert!(close_out.contains("Status: closed"));
    assert!(close_out.contains("Closeout Notes"));
}

#[test]
fn test_mission_close_still_blocks_hand_edited_issue_markdown() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &["mission", "create", "Dirty canonical tracker closeout"],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Dirty canonical tracker closeout");

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Hand edited canonical work"],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Hand edited canonical work");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", &mission_id, &issue_id]);
    assert!(success, "mission add work failed: {stderr}");
    attach_pass_evidence(
        dir.path(),
        "mission",
        &mission_id,
        "dirty canonical mission proof",
    );
    close_issue_with_evidence(dir.path(), &issue_id, Some("done"));
    commit_all(dir.path(), "clean baseline before manual canonical edit");

    edit_canonical_issue(dir.path(), &issue_id, |markdown| {
        markdown.replace(
            "The issue outcome is complete and ready for closeout.",
            "The issue outcome was hand-edited after closeout.",
        )
    });

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["mission", "close", &mission_id, "--reason", "done"],
    );
    assert!(
        !success,
        "hand-edited canonical issue markdown must block closeout"
    );
    assert!(stdout.contains("Mission closeout blocked"));
    assert!(stdout.contains("worktree: dirty"));
    assert!(stdout.contains(&format!(".atelier/issues/{issue_id}.md")));
    assert!(stderr.contains("mission closeout blocked"));
}

#[test]
fn test_mission_status_names_concrete_closeout_blockers() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Status blockers"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Status blockers");
    let (success, work_out, stderr) = run_atelier(dir.path(), &["issue", "create", "Still open"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Still open");
    let (success, blocker_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Open blocker"]);
    assert!(success, "blocker create failed: {stderr}");
    assert!(blocker_out.contains("Created issue atelier-"));
    let blocker_id = issue_id_by_title(dir.path(), "Open blocker");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", &mission_id, &work_id]);
    assert!(success, "mission add work failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "block", &work_id, &blocker_id]);
    assert!(success, "issue block failed: {stderr}");
    commit_all(dir.path(), "status baseline");
    std::fs::write(dir.path().join("status-dirty.txt"), "dirty").unwrap();

    let (success, status_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", &mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Closeout Gates"));
    assert!(status_out.contains("Work: open"));
    assert!(status_out.contains(&work_id));
    assert!(status_out.contains("Blockers: open"));
    assert!(status_out.contains(&blocker_id));
    assert!(status_out.contains("Worktree: dirty"));
    assert!(status_out.contains("status-dirty.txt"));
    assert!(!status_out.contains("Advanced Validator Detail"));
    assert!(!status_out.contains("advanced closeout validator failure"));

    let (success, verbose_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", "--verbose", &mission_id]);
    assert!(success, "verbose mission status failed: {stderr}");
    assert!(verbose_out.contains("Advanced Validator Detail"));
    assert!(verbose_out.contains("advanced closeout validator failure"));
    assert!(verbose_out.contains("git_worktree_clean"));
}

#[test]
fn test_mission_status_names_stale_and_malformed_record_blockers() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Record health blockers"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Record health blockers");
    let mission_id = mission_id.as_str();
    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Record health work"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Record health work");
    let issue_id = issue_id.as_str();
    let (success, evidence_issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Record evidence work"]);
    assert!(success, "evidence issue create failed: {stderr}");
    assert!(evidence_issue_out.contains("Created issue atelier-"));
    let evidence_issue_id = issue_id_by_title(dir.path(), "Record evidence work");
    let evidence_issue_id = evidence_issue_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, issue_id]);
    assert!(success, "mission add work failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-work", mission_id, evidence_issue_id],
    );
    assert!(success, "mission add evidence work failed: {stderr}");
    commit_all(dir.path(), "record health baseline");

    let issue_path = dir
        .path()
        .join(".atelier")
        .join("issues")
        .join(format!("{issue_id}.md"));
    let evidence_issue_path = dir
        .path()
        .join(".atelier")
        .join("issues")
        .join(format!("{evidence_issue_id}.md"));
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(
        &issue_path,
        markdown.replace("Record health work", "Record health work stale"),
    )
    .unwrap();
    commit_all(dir.path(), "stale record source");

    let (success, stale_status, stderr) =
        run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "stale mission status failed: {stderr}");
    assert!(
        stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "valid stale projection should be named and repaired before mission status:\nstdout:\n{stale_status}\nstderr:\n{stderr}"
    );
    assert!(stale_status.contains("Tracker:  ok"));
    assert!(stale_status.contains("Tracker State: current"));

    let stale_markdown = std::fs::read_to_string(&issue_path).unwrap();
    let malformed = stale_markdown.replace("\n## Outcome\n\nOutcome was not specified.\n", "\n");
    std::fs::write(&issue_path, malformed).unwrap();
    let evidence_markdown = std::fs::read_to_string(&evidence_issue_path).unwrap();
    let malformed_evidence =
        evidence_markdown.replace("\n## Evidence\n\nEvidence was not specified.\n", "\n");
    std::fs::write(&evidence_issue_path, malformed_evidence).unwrap();
    let conn = rusqlite::Connection::open(dir.path().join(".atelier/runtime/state.db")).unwrap();
    for (path, id) in [
        (&issue_path, issue_id),
        (&evidence_issue_path, evidence_issue_id),
    ] {
        let metadata = std::fs::metadata(path).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(std::fs::read(path).unwrap());
        let invalid_hash = format!("{:x}", hasher.finalize());
        conn.execute(
            "UPDATE projection_index_sources
             SET size_bytes = ?1, sha256 = ?2
             WHERE path = ?3",
            rusqlite::params![
                i64::try_from(metadata.len()).unwrap(),
                invalid_hash,
                format!("issues/{id}.md")
            ],
        )
        .unwrap();
    }
    commit_all(dir.path(), "malformed record source");

    let (success, malformed_status, stderr) =
        run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "malformed mission status failed: {stderr}");
    assert!(malformed_status.contains("Reliability"));
    assert!(malformed_status.contains("Malformed Work: found"));
    assert!(malformed_status.contains("Missing Outcome Sections: 1 issue(s)"));
    assert!(malformed_status.contains("Missing Evidence Sections: 1 issue(s)"));
    assert!(malformed_status.contains("Linked Issue Records: malformed"));
    assert!(malformed_status.contains("Missing required issue body section 'Outcome'"));
    assert!(malformed_status.contains("Missing required issue body section 'Evidence'"));
    assert!(malformed_status.contains("atelier lint"));
}
