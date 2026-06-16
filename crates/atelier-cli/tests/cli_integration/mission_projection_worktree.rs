use super::*;

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

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Invalid closeout",
            "--issue-type",
            "epic",
        ],
    );
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
fn test_mission_close_sees_issue_closeout_bookkeeping_committed_by_issue_close() {
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
        dirty.trim().is_empty(),
        "issue close should commit canonical bookkeeping before mission close:\n{dirty}"
    );

    let (success, status_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", &mission_id]);
    assert!(
        success,
        "mission status should tolerate tracker bookkeeping: {stderr}"
    );
    assert!(status_out.contains("Closeout Gates"));
    assert!(
        status_out.contains("Closeout: ready")
            && status_out.contains("All required closeout gates pass."),
        "mission status should be ready after issue close commits bookkeeping:\n{status_out}"
    );

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

#[test]
fn test_orientation_commands_enter_degraded_mode_for_malformed_records() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let valid_body = "## Description\n\nValid orientation body.\n\n## Outcome\n\nValid linked work remains visible during degraded orientation.\n\n## Evidence\n\n- `atelier mission status <id>` lists valid linked work.";
    let malformed_body = "## Description\n\nMalformed orientation body.\n\n## Outcome\n\nMalformed linked work is reported as a degraded blocker.\n\n## Evidence\n\n- `atelier lint <id>` reports the malformed record.";

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Degraded orientation"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Degraded orientation");
    let mission_id = mission_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", mission_id, "--status", "active"],
    );
    assert!(success, "mission activate failed: {stderr}");

    let (success, valid_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Valid degraded work",
            "--description",
            valid_body,
        ],
    );
    assert!(success, "valid issue create failed: {stderr}");
    assert!(valid_out.contains("Created issue atelier-"));
    let valid_id = issue_id_by_title(dir.path(), "Valid degraded work");
    let valid_id = valid_id.as_str();

    let (success, malformed_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Malformed degraded work",
            "--description",
            malformed_body,
        ],
    );
    assert!(success, "malformed issue create failed: {stderr}");
    assert!(malformed_out.contains("Created issue atelier-"));
    let malformed_id = issue_id_by_title(dir.path(), "Malformed degraded work");
    let malformed_id = malformed_id.as_str();

    for issue_id in [valid_id, malformed_id] {
        let (success, _, stderr) =
            run_atelier(dir.path(), &["mission", "add-work", mission_id, issue_id]);
        assert!(success, "mission add work failed for {issue_id}: {stderr}");
    }
    commit_all(dir.path(), "valid degraded orientation baseline");

    let malformed_path = dir
        .path()
        .join(".atelier")
        .join("issues")
        .join(format!("{malformed_id}.md"));
    let markdown = std::fs::read_to_string(&malformed_path).unwrap();
    std::fs::write(&malformed_path, remove_issue_section(&markdown, "Outcome")).unwrap();
    commit_all(dir.path(), "malformed degraded orientation record");

    let (status_success, status_out, status_err) = run_atelier(dir.path(), &["status"]);
    assert!(
        status_success,
        "status should degrade instead of failing: {status_err}"
    );
    assert!(status_out.contains("Atelier Status"));
    assert!(status_out.contains(&format!("Active mission: {mission_id}")));
    assert_degraded_repair_guidance(&status_err, malformed_id);

    let (mission_success, mission_out, mission_err) =
        run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(
        mission_success,
        "mission status should degrade instead of failing: {mission_err}"
    );
    assert!(mission_out.contains("Mission Status"));
    assert!(mission_out.contains(valid_id));
    assert!(mission_out.contains(malformed_id));
    assert!(mission_out.contains("Reliability"));
    assert!(mission_out.contains("Malformed Work: found"));
    assert!(mission_out.contains("Linked Issue Records: malformed"));
    assert!(mission_out.contains("Missing required issue body section 'Outcome'"));
    assert_degraded_repair_guidance(&mission_err, malformed_id);

    let (mission_show_success, mission_show_out, mission_show_err) =
        run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(
        mission_show_success,
        "mission show should degrade instead of failing: {mission_show_err}"
    );
    assert!(mission_show_out.contains("Valid degraded work"));
    assert!(mission_show_out.contains("Malformed degraded work"));
    assert_degraded_repair_guidance(&mission_show_err, malformed_id);

    let (show_success, show_out, show_err) =
        run_atelier(dir.path(), &["issue", "show", malformed_id]);
    assert!(
        show_success,
        "issue show should degrade instead of failing: {show_err}"
    );
    assert!(show_out.contains("Tracker Degraded"));
    assert!(show_out.contains("Fallback: showing the last valid local projection"));
    assert!(show_out.contains("Missing required issue body section 'Outcome'"));
    assert!(show_out.contains(&format!("Next: atelier lint {malformed_id}")));
    assert_degraded_repair_guidance(&show_err, malformed_id);

    let (doctor_success, doctor_out, doctor_err) = run_atelier(dir.path(), &["doctor"]);
    assert!(doctor_success, "doctor should remain usable: {doctor_err}");
    assert!(doctor_out.contains("Projection rebuild:"));
    assert!(doctor_out.contains("rebuild_ready: not ok"));

    let (lint_success, lint_out, lint_err) = run_atelier(dir.path(), &["lint"]);
    assert!(
        !lint_success,
        "global lint must fail closed for malformed records"
    );
    let lint_transcript = format!("{lint_out}\n{lint_err}");
    assert_degraded_lint_diagnostic(&lint_transcript, malformed_id);

    let (focused_success, focused_out, focused_err) =
        run_atelier(dir.path(), &["lint", malformed_id]);
    assert!(
        !focused_success,
        "focused lint must fail closed for malformed records"
    );
    let focused_transcript = format!("{focused_out}\n{focused_err}");
    assert_degraded_lint_diagnostic(&focused_transcript, malformed_id);

    let (close_success, _close_out, close_err) = run_atelier(
        dir.path(),
        &["issue", "close", malformed_id, "--reason", "done"],
    );
    assert!(!close_success, "issue closeout must fail closed");
    assert!(close_err.contains("Canonical tracker Markdown is invalid"));
    assert!(close_err.contains("atelier lint"));

    let (workflow_success, _workflow_out, workflow_err) =
        run_atelier(dir.path(), &["workflow", "check"]);
    assert!(!workflow_success, "workflow check must fail closed");
    assert!(workflow_err.contains("Canonical tracker Markdown is invalid"));
    assert!(workflow_err.contains("atelier lint"));
}

fn assert_degraded_repair_guidance(stderr: &str, issue_id: &str) {
    for needle in [
        "Tracker degraded".to_string(),
        "orientation only".to_string(),
        "Recovery: 1. run `atelier lint`".to_string(),
        "4. rerun the blocked command".to_string(),
        format!(".atelier/issues/{issue_id}.md"),
        "Missing required issue body section 'Outcome'".to_string(),
    ] {
        assert!(
            stderr.contains(&needle),
            "degraded stderr missing {needle:?}:\n{stderr}"
        );
    }
}

fn assert_degraded_lint_diagnostic(transcript: &str, issue_id: &str) {
    for needle in [
        format!(".atelier/issues/{issue_id}.md"),
        "Missing required issue body section 'Outcome'".to_string(),
        "section Outcome".to_string(),
    ] {
        assert!(
            transcript.contains(&needle),
            "lint transcript missing {needle:?}:\n{transcript}"
        );
    }
}

#[test]
fn test_mission_list_human_overview_orders_and_summarizes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, older_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Older ready"]);
    assert!(success, "older mission create failed: {stderr}");
    assert!(older_out.contains("Mission atelier-"));
    let older_id = record_id_by_title(dir.path(), "missions", "Older ready");
    let older_id = older_id.as_str();

    std::thread::sleep(std::time::Duration::from_millis(5));
    let (success, active_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Active mission"]);
    assert!(success, "active mission create failed: {stderr}");
    assert!(active_out.contains("Mission atelier-"));
    let active_id = record_id_by_title(dir.path(), "missions", "Active mission");
    let active_id = active_id.as_str();

    std::thread::sleep(std::time::Duration::from_millis(5));
    let (success, closed_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Newest closed"]);
    assert!(success, "closed mission create failed: {stderr}");
    assert!(closed_out.contains("Mission atelier-"));
    let closed_id = record_id_by_title(dir.path(), "missions", "Newest closed");
    let closed_id = closed_id.as_str();
    let (success, closed_work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Newest closed work"]);
    assert!(success, "closed work create failed: {stderr}");
    assert!(closed_work_out.contains("Created issue atelier-"));
    let closed_work_id = issue_id_by_title(dir.path(), "Newest closed work");
    let closed_work_id = closed_work_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-work", closed_id, closed_work_id],
    );
    assert!(success, "closed mission add work failed: {stderr}");
    close_issue_with_evidence(dir.path(), closed_work_id, Some("done"));

    let (success, closed_evidence, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "newest closed evidence",
        ],
    );
    assert!(success, "closed evidence create failed: {stderr}");
    assert!(closed_evidence.contains("[evidence] pass - newest closed evidence"));
    let closed_evidence_id = record_id_by_title(dir.path(), "evidence", "newest closed evidence");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "attach",
            &closed_evidence_id,
            "mission",
            closed_id,
        ],
    );
    assert!(success, "closed evidence attach failed: {stderr}");
    commit_all(dir.path(), "close newest mission");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "close", &closed_id, "--reason", "done"],
    );
    assert!(success, "close mission failed: {stderr}");

    let (success, epic_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Mission epic", "--issue-type", "epic"],
    );
    assert!(success, "epic issue create failed: {stderr}");
    assert!(epic_out.contains("Created issue atelier-"));
    let epic_id = issue_id_by_title(dir.path(), "Mission epic");
    let epic_id = epic_id.as_str();

    let (success, ready_out, stderr) =
        run_atelier(dir.path(), &["issue", "subissue", epic_id, "Ready work"]);
    assert!(success, "ready subissue create failed: {stderr}");
    assert!(ready_out.contains(epic_id));

    let (success, blocked_out, stderr) =
        run_atelier(dir.path(), &["issue", "subissue", epic_id, "Blocked work"]);
    assert!(success, "blocked subissue create failed: {stderr}");
    assert!(blocked_out.contains(epic_id));
    let blocked_id = issue_id_by_title(dir.path(), "Blocked work");
    let blocked_id = blocked_id.as_str();
    let (success, blocker_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Work blocker"]);
    assert!(success, "work blocker create failed: {stderr}");
    assert!(blocker_out.contains("Created issue atelier-"));
    let blocker_id = issue_id_by_title(dir.path(), "Work blocker");
    let blocker_id = blocker_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "block", &blocked_id, &blocker_id]);
    assert!(success, "block issue failed: {stderr}");

    let (success, done_out, stderr) =
        run_atelier(dir.path(), &["issue", "subissue", epic_id, "Done work"]);
    assert!(success, "done subissue create failed: {stderr}");
    assert!(done_out.contains(epic_id));
    let done_id = issue_id_by_title(dir.path(), "Done work");
    let done_id = done_id.as_str();
    close_issue_with_evidence(dir.path(), done_id, Some("done"));

    let (success, loose_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Loose mission work"]);
    assert!(success, "loose issue create failed: {stderr}");
    assert!(loose_out.contains("Created issue atelier-"));
    let loose_id = issue_id_by_title(dir.path(), "Loose mission work");
    let loose_id = loose_id.as_str();

    let (success, mission_blocker_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Mission blocker"]);
    assert!(success, "mission blocker create failed: {stderr}");
    assert!(mission_blocker_out.contains("Created issue atelier-"));
    let mission_blocker_id = issue_id_by_title(dir.path(), "Mission blocker");
    let mission_blocker_id = mission_blocker_id.as_str();

    for issue_id in [&epic_id, &loose_id] {
        let (success, _, stderr) =
            run_atelier(dir.path(), &["mission", "add-work", &active_id, issue_id]);
        assert!(success, "link work {issue_id} failed: {stderr}");
    }
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-blocker", &active_id, &mission_blocker_id],
    );
    assert!(success, "link mission blocker failed: {stderr}");

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--result",
            "pass",
            "older mission evidence",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - older mission evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "older mission evidence");
    let evidence_id = evidence_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", &evidence_id, "mission", &older_id],
    );
    assert!(success, "link evidence failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["mission", "list"]);
    assert!(success, "mission list failed: {stderr}");
    assert!(stdout.contains("Missions"));
    assert!(stdout.contains("2 ready missions | 1 blocked"));
    assert!(!stdout.contains("Updated:"));
    assert!(stdout.contains("Evidence gaps:"));
    assert!(!stdout.contains("ready="));
    assert!(stdout.contains("Ready"));
    assert!(!stdout.contains("Closed"));

    let active_row = format!("{active_id} [ready] - Active mission");
    let older_row = format!("{older_id} [ready] - Older ready");
    let closed_row = format!("{closed_id} [closed] - Newest closed");
    let active_pos = stdout.find(&active_row).expect("missing active row");
    let older_pos = stdout.find(&older_row).expect("missing older row");
    assert!(
        active_pos < older_pos,
        "newer ready mission should sort first:\n{stdout}"
    );
    assert!(!stdout.contains(&closed_row));
    assert!(
        stdout.contains(&format!(
            "[epic] {epic_id} [todo] medium - Mission epic | ready 1, blocked 1, done 1"
        )),
        "missing linked epic summary:\n{stdout}"
    );
    assert!(stdout.contains("Other linked work: 1 ready"));
    assert!(stdout.contains("Mission blockers: 1 open"));
    assert!(stdout.contains("No linked epics."));
    assert!(!stdout.contains("Loose mission work"));
    assert!(stdout.contains(&format!("atelier mission status {active_id}")));
    assert!(stdout.contains(&format!("atelier mission show {active_id}")));
    assert!(stdout.contains("atelier mission status"));
    assert!(stdout.contains("atelier mission create \"...\""));

    let (success, all_out, stderr) =
        run_atelier(dir.path(), &["mission", "list", "--status", "all"]);
    assert!(success, "all mission list failed: {stderr}");
    assert!(all_out.contains("3 missions | 1 closed, 2 ready | 1 blocked"));
    assert!(all_out.contains(&active_row));
    assert!(all_out.contains(&older_row));
    assert!(all_out.contains(&closed_row));
    let active_pos = all_out.find(&active_row).expect("missing active row");
    let closed_pos = all_out.find(&closed_row).expect("missing closed row");
    assert!(
        active_pos < closed_pos,
        "current missions should sort before closed missions:\n{all_out}"
    );

    let (success, ready_out, stderr) =
        run_atelier(dir.path(), &["mission", "list", "--status", "ready"]);
    assert!(success, "filtered mission list failed: {stderr}");
    assert!(ready_out.contains(&active_row));
    assert!(ready_out.contains(&older_row));
    assert!(!ready_out.contains(&closed_row));

    let (success, open_out, stderr) =
        run_atelier(dir.path(), &["mission", "list", "--status", "open"]);
    assert!(!success, "mission status alias should be rejected");
    assert!(open_out.is_empty());
    assert!(stderr.contains("Invalid mission status 'open'"));

    let (success, empty_out, stderr) =
        run_atelier(dir.path(), &["mission", "list", "--status", "draft"]);
    assert!(success, "empty filtered mission list failed: {stderr}");
    assert!(empty_out.contains("0 missions"));
    assert!(empty_out.contains("(none)"));
    assert!(empty_out.contains("atelier mission create \"...\""));

    let (success, list_out, stderr) = run_atelier(dir.path(), &["mission", "list"]);
    assert!(success, "mission list failed: {stderr}");
    assert!(list_out.contains(&active_row));
}

#[test]
fn test_mission_status_cli_reports_control_state() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Autonomy status"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Autonomy status");
    let mission_id = mission_id.as_str();

    let (success, epic_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Status epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    assert!(epic_out.contains("Created issue atelier-"));
    let epic_id = issue_id_by_title(dir.path(), "Status epic");
    let epic_id = epic_id.as_str();

    let (success, ready_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "subissue",
            epic_id,
            "Ready status work",
            "--description",
            "## Description\n\nReady status body.\n\n## Outcome\n\nMission status reports ready linked work.\n\n## Evidence\n\n- `atelier mission status <mission-id>` lists this work as ready.",
        ],
    );
    assert!(success, "ready work create failed: {stderr}");
    assert!(ready_out.contains(epic_id));

    let (success, blocked_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "subissue",
            epic_id,
            "Blocked status work",
            "--description",
            "## Description\n\nBlocked status body.\n\n## Outcome\n\nMission status reports blocked linked work.\n\n## Evidence\n\n- `atelier mission status <mission-id>` lists this work as blocked.",
        ],
    );
    assert!(success, "blocked work create failed: {stderr}");
    assert!(blocked_out.contains(epic_id));
    let blocked_id = issue_id_by_title(dir.path(), "Blocked status work");
    let blocked_id = blocked_id.as_str();
    let (success, blocker_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Status blocker",
            "--description",
            "## Description\n\nStatus blocker body.\n\n## Outcome\n\nMission status reports this issue as an open blocker.\n\n## Evidence\n\n- `atelier mission status <mission-id>` lists this blocker.",
        ],
    );
    assert!(success, "blocker create failed: {stderr}");
    assert!(blocker_out.contains("Created issue atelier-"));
    let blocker_id = issue_id_by_title(dir.path(), "Status blocker");
    let blocker_id = blocker_id.as_str();
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "block", blocked_id, blocker_id]);
    assert!(success, "block issue failed: {stderr}");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, epic_id]);
    assert!(success, "mission add work failed: {stderr}");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains(&format!(
        "Mission Status {mission_id} [ready] - Autonomy status"
    )));
    assert!(status_out.contains("Health:   blocked"));
    assert!(status_out.contains("Tracker:  ok"));
    assert!(status_out.contains("Work"));
    assert!(status_out.contains("ready"));
    assert!(status_out.contains("blocked"));
    assert!(status_out.contains("Selectable Work"));
    assert!(status_out.contains(&format!(
        "Ready status work | ready: no open blockers; parent {epic_id}; proof missing"
    )));
    assert!(status_out.contains("Blocked Work"));
    assert!(status_out.contains(&format!(
        "Blocked status work | blocked by {blocker_id}; parent {epic_id}; proof missing"
    )));
    assert!(status_out.contains("Blockers"));
    assert!(status_out.contains("Evidence"));
    assert!(status_out.contains("Direct mission evidence: none"));
    assert!(status_out.contains("Reliability"));
    assert!(status_out.contains("Projection Freshness: current"));
    assert!(status_out.contains("Malformed Work: none"));
    assert!(status_out.contains("Missing Outcome Sections: none"));
    assert!(status_out.contains("Missing Evidence Sections: none"));
    assert!(status_out.contains("Attached Proof: missing"));
    assert!(status_out.contains("Open Blockers: 1 open"));
    assert!(status_out.contains(&format!("atelier mission status --closeout {mission_id}")));
    assert!(status_out.contains("atelier lint"));
    assert!(status_out.contains("atelier doctor"));
    assert!(status_out.contains("Closeout Gates"));
    assert!(!status_out.contains("Advanced Validator Detail"));
    assert!(!status_out.contains("advanced closeout validator failure detected."));
    let (success, verbose_status_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", "--verbose", mission_id]);
    assert!(success, "verbose mission status failed: {stderr}");
    assert!(verbose_status_out.contains("Advanced Validator Detail"));
    assert!(verbose_status_out.contains("advanced closeout validator failure detected."));
    assert!(status_out.contains("Next Commands"));
    assert!(status_out.contains(&format!(
        "Inspect mission record (durable intent and linked work): atelier mission show {mission_id}"
    )));
    assert!(status_out.contains(&format!(
        "Refresh mission status (current blockers and closeout gates): atelier mission status {mission_id}"
    )));
    assert!(status_out.contains("Resolve open blockers before assigning more implementation work"));
    assert!(!status_out.contains("ready item(s)): atelier issue list --ready"));
    assert!(!status_out.contains("selectable issue(s)): atelier start"));
    assert!(status_out.contains("Record validation proof ("));
    assert!(status_out.contains(
        "atelier evidence record --target issue/<id> --kind validation --result pass \"...\""
    ));
    assert!(
        !status_out.contains("workflow validate"),
        "normal mission next commands must not route to raw workflow validators:\n{status_out}"
    );

    let (success, quiet_out, stderr) =
        run_atelier(dir.path(), &["--quiet", "mission", "status", mission_id]);
    assert!(success, "quiet mission status failed: {stderr}");
    assert!(quiet_out.contains(&format!("{mission_id} health=blocked")));
    assert!(quiet_out.contains("evidence_gaps="));
    assert!(quiet_out.contains("tracker=ok"));

    let (success, dashboard_out, stderr) = run_atelier(dir.path(), &["mission", "status"]);
    assert!(success, "mission status dashboard failed: {stderr}");
    assert!(dashboard_out.contains("Mission Status"));
    assert!(dashboard_out.contains("1 ready mission | 1 blocked | tracker ok"));
    assert!(dashboard_out.contains(&format!("{mission_id} [blocked] ready - Autonomy status")));

    let closeout_mission = {
        let (success, out, stderr) =
            run_atelier(dir.path(), &["mission", "create", "Closeout mission"]);
        assert!(success, "closeout mission create failed: {stderr}");
        assert!(out.contains("Mission atelier-"));
        record_id_by_title(dir.path(), "missions", "Closeout mission")
    };
    let closeout_mission = closeout_mission.as_str();
    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Finished mission work"]);
    assert!(success, "finished work create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Finished mission work");
    let work_id = work_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-work", closeout_mission, work_id],
    );
    assert!(success, "closeout mission add work failed: {stderr}");
    close_issue_with_evidence(dir.path(), work_id, Some("done"));
    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "closeout evidence",
        ],
    );
    assert!(success, "closeout evidence record failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - closeout evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "closeout evidence");
    let evidence_id = evidence_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "attach",
            evidence_id,
            "mission",
            closeout_mission,
        ],
    );
    assert!(success, "closeout evidence attach failed: {stderr}");
    commit_all(dir.path(), "closeout status ready");

    let (success, closeout_status, stderr) =
        run_atelier(dir.path(), &["mission", "status", closeout_mission]);
    assert!(success, "closeout mission status failed: {stderr}");
    assert!(closeout_status.contains("Health:   closeout"));
    assert!(
        closeout_status.contains("Closeout: ready"),
        "unexpected closeout mission status:\n{closeout_status}"
    );
    assert!(closeout_status.contains("Reliability"));
    assert!(closeout_status.contains("Attached Proof: complete"));
    assert!(closeout_status.contains("Docs/Help Drift: clear"));
    assert!(closeout_status.contains("Ignored Test Review: current"));
    assert!(closeout_status.contains("Open Blockers: none"));
    assert!(closeout_status.contains(&format!(
        "Close mission (all closeout gates pass): atelier mission close {closeout_mission} --reason \"...\""
    )));

    let mission_path = dir
        .path()
        .join(".atelier")
        .join("missions")
        .join(format!("{mission_id}.md"));
    let mission_markdown = std::fs::read_to_string(&mission_path).unwrap();
    std::fs::write(
        &mission_path,
        mission_markdown.replace("Autonomy status", "Autonomy status stale"),
    )
    .unwrap();
    let (success, stale_status, stderr) =
        run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "stale mission status failed: {stderr}");
    assert!(stale_status.contains("Autonomy status stale"));
    assert!(stale_status.contains("Tracker:  ok"));
    assert!(stale_status.contains("Worktree: dirty"));
    assert!(!stale_status.contains("advanced closeout validator failure detected."));
}

#[test]
fn test_mission_status_deduplicates_duplicate_reachability() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Duplicate reachability"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Duplicate reachability");

    let (success, epic_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Duplicate epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    assert!(epic_out.contains("Created issue atelier-"));
    let epic_id = issue_id_by_title(dir.path(), "Duplicate epic");

    let (success, child_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "subissue",
            epic_id.as_str(),
            "Duplicate child",
            "--description",
            "## Description\n\nDuplicate reachability child.\n\n## Outcome\n\nMission status counts this child once.\n\n## Evidence\n\n- `atelier mission status <mission-id>` counts this child once and reports duplicate reachability.",
        ],
    );
    assert!(success, "child create failed: {stderr}");
    assert!(child_out.contains(&epic_id));
    let child_id = issue_id_by_title(dir.path(), "Duplicate child");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-work", mission_id.as_str(), epic_id.as_str()],
    );
    assert!(success, "mission add epic failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "add-work",
            mission_id.as_str(),
            child_id.as_str(),
        ],
    );
    assert!(success, "mission add child failed: {stderr}");

    let (success, status_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", mission_id.as_str()]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Total: 2 ready"));
    assert!(status_out.contains(&format!(
        "Graph Hygiene: warning - duplicate reachability for 1 issue(s): {child_id} ({epic_id} + direct)"
    )));
    assert!(status_out.contains(
        "Totals count each unique issue once. Keep mission links on root issues or epics and let child issues flow through hierarchy."
    ));
}

#[test]
fn test_active_mission_focus_guides_status_and_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    migrate_default_issue_workflow(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Active focus"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Active focus");
    let mission_id = mission_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Mission work"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Mission work");
    let issue_id = issue_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, issue_id]);
    assert!(success, "mission add work failed: {stderr}");

    let (success, start_out, stderr) = run_atelier(dir.path(), &["mission", "start", mission_id]);
    assert!(success, "mission start failed: {stderr}");
    assert!(start_out.contains(&format!("Active mission: {mission_id}")));
    assert!(start_out.contains(&format!("atelier mission status {mission_id}")));

    let (success, status_out, stderr) = run_atelier(dir.path(), &["mission", "status"]);
    assert!(success, "active mission status failed: {stderr}");
    assert!(status_out.contains(&format!(
        "Mission Status {mission_id} [active] - Active focus"
    )));

    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    migrate_default_issue_workflow(dir.path());
    Command::new("git")
        .current_dir(dir.path())
        .args(["add", "."])
        .status()
        .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["commit", "-q", "-m", "active mission"])
        .status()
        .unwrap();

    let (success, work_out, stderr) = run_atelier(dir.path(), &["start", issue_id]);
    assert!(success, "root start failed: {stderr}");
    assert!(work_out.contains(&format!("Mission: {mission_id} (active)")));
    assert!(work_out.contains(&format!("Started work on {issue_id}")));
}

#[test]
fn test_mission_start_requires_explicit_switch_and_warns_for_outside_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    migrate_default_issue_workflow(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "First mission"]);
    assert!(success, "first mission create failed: {stderr}");
    let first_id = record_id_by_title(dir.path(), "missions", "First mission");
    let first_id = first_id.as_str();
    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Second mission"]);
    assert!(success, "second mission create failed: {stderr}");
    let second_id = record_id_by_title(dir.path(), "missions", "Second mission");
    let second_id = second_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "start", first_id]);
    assert!(success, "first mission start failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "start", second_id]);
    assert!(!success, "second mission start without switch should fail");
    assert!(
        stderr.contains("--switch"),
        "expected switch guidance in stderr: {stderr}"
    );
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "start", second_id, "--switch"]);
    assert!(success, "mission switch failed: {stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Outside work"]);
    assert!(success, "outside issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Outside work");
    let issue_id = issue_id.as_str();
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    migrate_default_issue_workflow(dir.path());
    Command::new("git")
        .current_dir(dir.path())
        .args(["add", "."])
        .status()
        .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["commit", "-q", "-m", "switched mission"])
        .status()
        .unwrap();

    let (success, work_out, stderr) = run_atelier(dir.path(), &["start", issue_id]);
    assert!(success, "outside root start failed: {stderr}");
    assert!(work_out.contains(&format!(
        "Warning: {issue_id} is outside active mission {second_id}"
    )));
}

#[test]
fn test_mission_list_default_current_empty_state() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, closed_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Closed only"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(closed_out.contains("Mission atelier-"));
    let closed_id = record_id_by_title(dir.path(), "missions", "Closed only");
    let (success, closed_work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Closed only work"]);
    assert!(success, "closed-only work create failed: {stderr}");
    assert!(closed_work_out.contains("Created issue atelier-"));
    let closed_work_id = issue_id_by_title(dir.path(), "Closed only work");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-work", &closed_id, &closed_work_id],
    );
    assert!(success, "closed-only mission add work failed: {stderr}");
    close_issue_with_evidence(dir.path(), &closed_work_id, Some("done"));

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "closed only evidence",
        ],
    );
    assert!(success, "evidence create failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - closed only evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "closed only evidence");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", &evidence_id, "mission", &closed_id],
    );
    assert!(success, "evidence attach failed: {stderr}");
    commit_all(dir.path(), "close only mission");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "close", &closed_id, "--reason", "done"],
    );
    assert!(success, "close mission failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["mission", "list"]);
    assert!(success, "mission list failed: {stderr}");
    assert!(stdout.contains("0 missions | 0 blocked"));
    assert!(stdout.contains("(none)"));
    assert!(!stdout.contains("Closed only"));

    let (success, closedstdout, stderr) =
        run_atelier(dir.path(), &["mission", "list", "--status", "closed"]);
    assert!(success, "closed mission list failed: {stderr}");
    assert!(closedstdout.contains("1 closed mission | 0 blocked"));
    assert!(closedstdout.contains("Closed only"));
}

#[test]
fn test_first_class_record_rebuild_rejects_schema_drift() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Guard schema"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Guard schema");
    edit_canonical_record(dir.path(), "missions", &mission_id, |markdown| {
        markdown.replace("schema: \"atelier.mission\"", "schema: \"atelier.issue\"")
    });
    remove_projection_state(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(!success, "rebuild should reject mission schema drift");
    assert!(
        stderr.contains("Unsupported schema 'atelier.issue'")
            && stderr.contains("expected atelier.mission"),
        "unexpected rebuild error: {stderr}"
    );
}

#[test]
fn test_projection_query_distinguishes_schema_drift_from_malformed_records() {
    let schema_dir = tempdir().unwrap();
    init_atelier(schema_dir.path());

    let (success, _, stderr) = run_atelier(schema_dir.path(), &["issue", "create", "Schema drift"]);
    assert!(success, "issue create failed: {stderr}");
    let schema_issue_id = issue_id_by_title(schema_dir.path(), "Schema drift");
    edit_canonical_issue(schema_dir.path(), &schema_issue_id, |markdown| {
        markdown.replace("schema_version: 1", "schema_version: 99")
    });
    remove_projection_state(schema_dir.path());

    let (success, _, stderr) = run_atelier(schema_dir.path(), &["issue", "list"]);
    assert!(!success, "schema drift should block projection query");
    assert!(
        stderr.contains("schema this atelier binary does not understand")
            && stderr.contains("target/debug/atelier")
            && stderr.contains("update the installed `atelier` binary")
            && stderr.contains("Unsupported schema_version 99"),
        "schema drift diagnostic should name stale-binary repair: {stderr}"
    );
    assert!(
        !stderr.contains("fix canonical tracker records before querying"),
        "schema drift should not be presented as ordinary malformed records: {stderr}"
    );

    let malformed_dir = tempdir().unwrap();
    init_atelier(malformed_dir.path());
    let (success, _, stderr) = run_atelier(
        malformed_dir.path(),
        &["issue", "create", "Malformed source"],
    );
    assert!(success, "issue create failed: {stderr}");
    let malformed_issue_id = issue_id_by_title(malformed_dir.path(), "Malformed source");
    corrupt_issue_title_yaml(
        malformed_dir.path(),
        &malformed_issue_id,
        "Malformed source",
    );
    remove_projection_state(malformed_dir.path());

    let (success, _, stderr) = run_atelier(malformed_dir.path(), &["issue", "list"]);
    assert!(!success, "malformed records should block projection query");
    assert!(
        stderr.contains("recovery: 1. run `atelier lint`")
            && stderr.contains("2. fix the named canonical Markdown record")
            && stderr.contains("4. rerun the blocked command")
            && stderr.contains("Invalid YAML front matter"),
        "malformed diagnostic should stay record-focused: {stderr}"
    );
    assert!(
        !stderr.contains("schema this atelier binary does not understand"),
        "malformed records should not be presented as stale binary drift: {stderr}"
    );
}

#[test]
fn test_projection_index_rebuilds_changed_sources_before_issue_queries() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Indexed title"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    let (success, list_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(success, "fresh list failed: {stderr}");
    assert!(list_out.contains("Indexed title"));

    edit_canonical_issue(dir.path(), &issue_id, |markdown| {
        markdown.replace("Indexed title", "Markdown title")
    });

    let (success, list_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(success, "stale list should transparently rebuild: {stderr}");
    assert!(list_out.contains("Markdown title"));
    assert!(
        stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "missing automatic rebuild diagnostic: {stderr}"
    );
}

#[test]
fn test_projection_index_bounds_many_changed_sources_and_rebuilds() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let mut issue_ids = Vec::new();
    for index in 0..12 {
        let title = format!("Bulk indexed {index}");
        let (success, issue_out, stderr) = run_atelier(dir.path(), &["issue", "create", &title]);
        assert!(success, "issue create failed: {stderr}");
        assert!(issue_out.contains("Created issue atelier-"));
        issue_ids.push(issue_ref(dir.path(), index + 1));
    }
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    for (index, issue_id) in issue_ids.iter().enumerate() {
        edit_canonical_issue(dir.path(), issue_id, |markdown| {
            markdown.replace(
                &format!("title: \"Bulk indexed {index}\""),
                &format!("title: \"Bulk markdown {index}\""),
            )
        });
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(!success, "export check should report stale projection");
    assert!(
        stderr.contains("12 indexed sources changed")
            && stderr.contains("showing first 5")
            && stderr.contains("recovery: 1. run `atelier lint`")
            && stderr.contains("3. run `atelier doctor --fix`")
            && stderr.contains("4. rerun the blocked command"),
        "stale diagnostics should be bounded and actionable: {stderr}"
    );
    assert!(
        stderr.lines().count() < 12,
        "stale diagnostics should not dump every changed source: {stderr}"
    );

    let (success, list_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(
        success,
        "many changed sources should transparently rebuild: {stderr}"
    );
    assert!(list_out.contains("Bulk markdown 0"));
    assert!(list_out.contains("Bulk markdown 11"));
    assert!(
        stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "missing automatic rebuild diagnostic: {stderr}"
    );
}

#[test]
fn test_projection_index_rebuilds_deleted_and_unindexed_sources_before_issue_queries() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, first_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "First indexed issue"]);
    assert!(success, "first create failed: {stderr}");
    assert!(first_out.contains("Created issue atelier-"));
    let first_id = issue_ref(dir.path(), 1);
    let (success, second_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Second indexed issue"]);
    assert!(success, "second create failed: {stderr}");
    assert!(second_out.contains("Created issue atelier-"));
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    let first_path = canonical_issue_path(dir.path(), &first_id);
    let first_markdown = read_canonical_record(dir.path(), "issues", &first_id);
    std::fs::remove_file(&first_path).unwrap();

    let (success, list_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(
        success,
        "deleted source list should transparently rebuild: {stderr}"
    );
    assert!(!list_out.contains("First indexed issue"));
    assert!(list_out.contains("Second indexed issue"));
    assert!(
        stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "missing automatic rebuild diagnostic: {stderr}"
    );

    std::fs::write(&first_path, first_markdown).unwrap();
    let unindexed_path = dir.path().join(".atelier/issues/atelier-zzzz.md");
    std::fs::write(
        &unindexed_path,
        r#"---
created_at: "2026-06-10T12:00:00+00:00"
id: "atelier-zzzz"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Unindexed issue"
updated_at: "2026-06-10T12:00:00+00:00"
---

## Description

Body

## Outcome

The unindexed issue is discoverable after rebuild.

## Evidence

- `atelier search Unindexed` shows the record.
"#,
    )
    .unwrap();

    let (success, search_out, stderr) = run_atelier(dir.path(), &["search", "Unindexed"]);
    assert!(
        success,
        "unindexed search should transparently rebuild: {stderr}"
    );
    assert!(search_out.contains("Unindexed issue"));
    assert!(
        stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "missing automatic rebuild diagnostic: {stderr}"
    );
}

#[test]
fn test_projection_index_rebuilds_dep_list_and_lint_but_ignores_derived_files() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let first_body = "## Description\n\nProjection root body.\n\n## Outcome\n\nProjection root remains queryable after rebuild.\n\n## Evidence\n\n- manual check: `atelier lint` output prints `Lint passed.` after automatic rebuild.";
    let second_body = "## Description\n\nProjection leaf body.\n\n## Outcome\n\nProjection leaf remains linked after rebuild.\n\n## Evidence\n\n- manual check: `atelier issue blocked <id>` output shows the linked root.";

    let (success, first_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Projection root",
            "--description",
            first_body,
        ],
    );
    assert!(success, "first create failed: {stderr}");
    assert!(first_out.contains("Created issue atelier-"));
    let first_id = issue_ref(dir.path(), 1);
    let (success, second_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Projection leaf",
            "--description",
            second_body,
        ],
    );
    assert!(success, "second create failed: {stderr}");
    assert!(second_out.contains("Created issue atelier-"));
    let second_id = issue_ref(dir.path(), 2);
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "block", &second_id, &first_id]);
    assert!(success, "issue block failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    ensure_issue_closeout_sections(dir.path(), &first_id);
    ensure_issue_closeout_sections(dir.path(), &second_id);

    std::fs::write(dir.path().join(".atelier/manifest.json"), "{}\n").unwrap();
    std::fs::write(dir.path().join(".atelier/graph.json"), "{}\n").unwrap();
    let (success, ready_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(
        success,
        "derived files should not stale issue list --ready: {stderr}"
    );
    assert!(ready_out.contains("Projection root"));

    edit_canonical_issue(dir.path(), &first_id, |markdown| {
        markdown.replace("Projection root", "Projection root changed")
    });

    let (success, dep_out, stderr) = run_atelier(dir.path(), &["issue", "blocked", &second_id]);
    assert!(
        success,
        "stale issue blocked should transparently rebuild: {stderr}"
    );
    assert!(dep_out.contains("Projection root changed"));
    assert!(
        stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "missing automatic rebuild diagnostic: {stderr}"
    );

    let (success, lint_out, stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(
        success,
        "lint should run after automatic rebuild:\nstdout:\n{lint_out}\nstderr:\n{stderr}"
    );
    assert!(lint_out.contains("Lint passed."));
}

#[test]
fn test_rebuild_temp_files_are_ignored_by_query_lint_export_and_doctor() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let body = "## Description\n\nTemp rebuild filter body.\n\n## Outcome\n\nQuery, lint, export, and doctor ignore rebuild temp files.\n\n## Evidence\n\n- manual check: `atelier lint` output prints `Lint passed.`, `atelier export --check` exits 0, and `atelier doctor` exits 0 while rebuild temp files exist.";
    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Temp rebuild filter",
            "--description",
            body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    ensure_issue_closeout_sections(dir.path(), &issue_id);
    write_ignored_canonical_artifacts(dir.path(), &issue_id);

    edit_canonical_issue(dir.path(), &issue_id, |markdown| {
        markdown.replace("Temp rebuild filter", "Temp rebuild filter changed")
    });

    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "query should ignore rebuild tmp file: {stderr}");
    assert!(show_out.contains("Temp rebuild filter changed"));
    assert!(
        !stderr.contains("rebuild-tmp"),
        "query diagnostics must not report rebuild tmp path: {stderr}"
    );

    let commands: &[&[&str]] = &[&["lint"], &["export", "--check"], &["doctor"]];
    for args in commands {
        let (success, stdout, stderr) = run_atelier(dir.path(), args);
        assert!(
            success,
            "{args:?} should ignore rebuild tmp file:\nstdout: {stdout}\nstderr: {stderr}"
        );
        let combined = format!("{stdout}\n{stderr}");
        assert!(
            !combined.contains("rebuild-tmp")
                && !combined.contains(".md.lock")
                && !combined.contains(".md-journal")
                && !combined.contains("projection.lock"),
            "{args:?} diagnostics must not report ignored local artifacts: {combined}"
        );
    }
}

#[test]
fn test_projection_index_rejects_invalid_markdown_without_rebuild() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Invalid Markdown source"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    let markdown = read_canonical_record(dir.path(), "issues", &issue_id);
    corrupt_issue_title_yaml(dir.path(), &issue_id, "Invalid Markdown source");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(
        !success,
        "invalid canonical Markdown should fail export check"
    );
    assert!(
        stderr.contains("canonical tracker Markdown is invalid")
            && stderr.contains("while running a deterministic export diagnostic")
            && stderr.contains("atelier lint")
            && stderr.contains("2. fix the named canonical Markdown record")
            && stderr.contains("3. run `atelier doctor --fix`")
            && stderr.contains("4. rerun the blocked command")
            && stderr.contains(&format!(".atelier/issues/{issue_id}.md")),
        "unexpected invalid export error: {stderr}"
    );
    assert!(
        !stderr.contains("indexed source changed"),
        "invalid canonical errors must not be obscured by stale metadata: {stderr}"
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(
        success,
        "invalid canonical Markdown should show degraded fallback: {stderr}"
    );
    assert!(stdout.contains("Tracker Degraded"));
    assert!(stdout.contains("Invalid YAML front matter"));
    assert!(stdout.contains("Fallback: showing the last valid local projection"));
    assert!(stdout.contains(&format!("Next: atelier lint {issue_id}")));
    assert!(
        stderr.contains("Tracker degraded")
            && stderr.contains("Recovery: 1. run `atelier lint`")
            && stderr.contains("4. rerun the blocked command")
            && stderr.contains("atelier lint")
            && stderr.contains("Invalid YAML front matter")
            && stderr.contains(&format!(".atelier/issues/{issue_id}.md")),
        "unexpected invalid Markdown error: {stderr}"
    );
    assert!(
        !stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "invalid Markdown must not be silently repaired: {stderr}"
    );

    write_canonical_record(dir.path(), "issues", &issue_id, markdown);
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(
        success,
        "restored canonical Markdown should query: {stderr}"
    );
    assert!(show_out.contains("Invalid Markdown source"));
}

#[test]
fn test_lint_validates_canonical_markdown_even_when_projection_metadata_is_fresh() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Lint canonical source"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);

    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = read_canonical_record(dir.path(), "issues", &issue_id);
    let invalid_markdown = markdown.replace(
        "title: \"Lint canonical source\"",
        "title: [Lint canonical source",
    );
    write_canonical_record(dir.path(), "issues", &issue_id, invalid_markdown.clone());
    write_ignored_canonical_artifacts(dir.path(), &issue_id);

    let metadata = std::fs::metadata(&issue_path).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(invalid_markdown.as_bytes());
    let invalid_hash = format!("{:x}", hasher.finalize());
    let conn = rusqlite::Connection::open(dir.path().join(".atelier/runtime/state.db")).unwrap();
    conn.execute(
        "UPDATE projection_index_sources
         SET size_bytes = ?1, sha256 = ?2
         WHERE path = ?3",
        rusqlite::params![
            i64::try_from(metadata.len()).unwrap(),
            invalid_hash,
            format!("issues/{issue_id}.md")
        ],
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(
        !success,
        "lint must reject malformed canonical Markdown, stdout: {stdout}"
    );
    assert!(
        stdout.contains("Invalid YAML front matter")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "unexpected lint output: {stdout}\nstderr: {stderr}"
    );
    let transcript = format!("{stdout}\n{stderr}");
    assert!(
        !transcript.contains("rebuild-tmp")
            && !transcript.contains(".md.lock")
            && !transcript.contains(".md-journal")
            && !transcript.contains("projection.lock"),
        "lint must ignore local artifacts while reporting malformed committed Markdown: {transcript}"
    );
    assert!(stderr.contains("Lint failed"));
    assert!(
        !stdout.contains("Lint passed."),
        "lint must not pass from stale SQLite rows: {stdout}"
    );
}

#[test]
fn test_lint_validates_canonical_markdown_when_state_db_is_missing() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let body = "## Description\n\nDescription\n\n## Outcome\n\nLint rebuilds a missing state database from canonical Markdown.\n\n## Evidence\n\n- `atelier lint` prints `Lint passed.` after rebuilding state.db.";

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Lint without state db",
            "--description",
            body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    remove_projection_state(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(success, "lint should rebuild missing state.db: {stderr}");
    assert!(stdout.contains("Lint passed."));
    assert!(
        stderr.contains("Runtime projection database was missing; rebuilt local SQLite projection"),
        "missing rebuild diagnostic: {stderr}"
    );
}

#[test]
fn test_status_recovers_when_runtime_directory_is_missing() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    migrate_default_issue_workflow(dir.path());

    let body = "## Description\n\nDescription\n\n## Outcome\n\nStatus recovers current work from canonical Markdown after ignored runtime deletion.\n\n## Evidence\n\n- `atelier status` prints the current-work issue after recreating `.atelier/runtime/state.db`.";
    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Runtime directory recovery",
            "--description",
            body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Runtime directory recovery");
    commit_all(dir.path(), "runtime recovery issue");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "start failed: {stderr}");
    std::fs::remove_dir_all(dir.path().join(".atelier/runtime")).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(
        success,
        "status should recreate missing runtime dir: {stderr}"
    );
    assert!(
        stderr.contains("Runtime projection database was missing; rebuilt local SQLite projection")
    );
    assert!(stdout.contains("Current work:  1 issue(s)"), "{stdout}");
    assert!(stdout.contains(&format!("  {issue_id} - Runtime directory recovery")));
    assert!(dir.path().join(".atelier/runtime/state.db").exists());
}

#[test]
fn test_focused_lint_validates_missing_relationship_targets() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let body = "## Description\n\nFocused lint missing target body.\n\n## Outcome\n\nFocused lint reports a missing relationship target.\n\n## Evidence\n\n- `atelier lint <issue-id>` reports the missing issue relationship target.";
    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Focused lint missing target",
            "--description",
            body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);

    edit_canonical_issue(dir.path(), &issue_id, |markdown| {
        markdown.replace(
            "  blocks: []",
            "  blocks:\n  - kind: \"issue\"\n    id: \"atelier-missing\"",
        )
    });

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(
        !success,
        "focused lint should reject missing relationship target, stdout: {stdout}"
    );
    let transcript = format!("{stdout}\n{stderr}");
    assert!(
        transcript.contains("has blocks reference to missing issue atelier-missing")
            && transcript.contains(&issue_id)
            && transcript.contains("Canonical tracker Markdown is invalid"),
        "unexpected focused lint error: {transcript}"
    );
}

#[test]
fn test_focused_lint_validates_dependency_cycles() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let first_body = "## Description\n\nFocused lint cycle root body.\n\n## Outcome\n\nFocused lint reports dependency cycles.\n\n## Evidence\n\n- `atelier lint <issue-id>` reports relationships.blocks contains a cycle.";
    let second_body = "## Description\n\nFocused lint cycle leaf body.\n\n## Outcome\n\nFocused lint reports dependency cycles.\n\n## Evidence\n\n- `atelier lint <issue-id>` reports relationships.blocks contains a cycle.";
    let (success, first_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Focused lint cycle root",
            "--description",
            first_body,
        ],
    );
    assert!(success, "first issue create failed: {stderr}");
    assert!(first_out.contains("Created issue atelier-"));
    let first_id = issue_ref(dir.path(), 1);
    let (success, second_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Focused lint cycle leaf",
            "--description",
            second_body,
        ],
    );
    assert!(success, "second issue create failed: {stderr}");
    assert!(second_out.contains("Created issue atelier-"));
    let second_id = issue_ref(dir.path(), 2);

    for (issue_id, blocked_id) in [(&first_id, &second_id), (&second_id, &first_id)] {
        edit_canonical_issue(dir.path(), issue_id, |markdown| {
            markdown.replace(
                "  blocks: []",
                &format!("  blocks:\n  - kind: \"issue\"\n    id: \"{blocked_id}\""),
            )
        });
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &first_id]);
    assert!(
        !success,
        "focused lint should reject dependency cycle, stdout: {stdout}"
    );
    let transcript = format!("{stdout}\n{stderr}");
    assert!(
        transcript.contains("relationships.blocks contains a cycle"),
        "unexpected focused lint cycle error: {transcript}"
    );
}

#[test]
fn test_lint_has_stable_diagnostics_for_hard_invalid_markdown_records() {
    assert_lint_rejects_issue_edit(
        "Invalid status fixture",
        |markdown, _issue_id| markdown.replace("status: \"todo\"", "status: \"bad-status\""),
        &["Invalid status", "Invalid status 'bad-status'"],
    );
    assert_lint_rejects_issue_edit(
        "Invalid type fixture",
        |markdown, _issue_id| markdown.replace("issue_type: \"task\"", "issue_type: \"bogus\""),
        &["Invalid issue_type", "Invalid issue_type 'bogus'"],
    );
    assert_lint_rejects_issue_edit(
        "Invalid priority fixture",
        |markdown, _issue_id| markdown.replace("priority: \"P2\"", "priority: \"urgent\""),
        &[
            "Invalid priority",
            "unsupported canonical priority 'urgent'",
        ],
    );
    assert_lint_rejects_issue_edit(
        "Invalid schema fixture",
        |markdown, _issue_id| {
            markdown.replace("schema: \"atelier.issue\"", "schema: \"atelier.graph\"")
        },
        &["Unsupported schema 'atelier.graph'"],
    );
    assert_lint_rejects_issue_edit(
        "Invalid schema version fixture",
        |markdown, _issue_id| markdown.replace("schema_version: 1", "schema_version: 99"),
        &["Unsupported schema_version 99"],
    );
    assert_lint_rejects_issue_edit(
        "ID path mismatch fixture",
        |markdown, issue_id| {
            markdown.replace(&format!("id: \"{issue_id}\""), "id: \"atelier-zzzz\"")
        },
        &["does not match canonical path"],
    );

    assert_lint_rejects_canonical_mutation(
        "Malformed activity sidecar fixture",
        |dir, issue_id| {
            let activity_path = dir
                .join(".atelier/issues")
                .join(format!("{issue_id}.activity"))
                .join("bad.md");
            std::fs::create_dir_all(activity_path.parent().unwrap()).unwrap();
            std::fs::write(activity_path, "not front matter\n").unwrap();
        },
        &["Missing YAML front matter", ".activity/bad.md"],
    );
    assert_lint_rejects_canonical_mutation(
        "Unsupported committed file fixture",
        |dir, _issue_id| {
            std::fs::write(dir.join(".atelier/issues/junk.txt"), "junk\n").unwrap();
        },
        &[
            "Unsupported canonical issue file",
            ".atelier/issues/junk.txt",
        ],
    );
    assert_lint_rejects_canonical_mutation(
        "Duplicate ID fixture",
        |dir, issue_id| {
            let (success, mission_out, stderr) =
                run_atelier(dir, &["mission", "create", "Duplicate ID mission"]);
            assert!(success, "mission create failed: {stderr}");
            let mission_id = mission_out
                .lines()
                .find_map(|line| {
                    line.strip_prefix("Mission ")
                        .and_then(|rest| rest.split(':').next())
                })
                .expect("mission create output should include an id")
                .to_string();
            let old_path = dir
                .join(".atelier/missions")
                .join(format!("{mission_id}.md"));
            let new_path = dir.join(".atelier/missions").join(format!("{issue_id}.md"));
            let mission_markdown = std::fs::read_to_string(&old_path).unwrap().replace(
                &format!("id: \"{mission_id}\""),
                &format!("id: \"{issue_id}\""),
            );
            std::fs::write(&new_path, mission_markdown).unwrap();
            std::fs::remove_file(old_path).unwrap();
        },
        &["Duplicate record ID in canonical projection"],
    );
}

fn assert_lint_rejects_issue_edit(
    title: &str,
    edit: impl FnOnce(&str, &str) -> String,
    expected: &[&str],
) {
    assert_lint_rejects_canonical_mutation(
        title,
        |dir, issue_id| {
            edit_canonical_issue(dir, issue_id, |markdown| edit(&markdown, issue_id));
        },
        expected,
    );
}

fn assert_lint_rejects_canonical_mutation(
    title: &str,
    mutate: impl FnOnce(&Path, &str),
    expected: &[&str],
) {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let body = format!(
        "## Description\n\n{title} body.\n\n## Outcome\n\nCanonical lint rejects the targeted malformed record.\n\n## Evidence\n\n- `atelier lint` reports the targeted malformed canonical record."
    );

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", title, "--description", &body],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);

    mutate(dir.path(), &issue_id);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(
        !success,
        "lint should reject {title}, stdout: {stdout}, stderr: {stderr}"
    );
    let transcript = format!("{stdout}\n{stderr}");
    assert!(
        transcript.contains("Canonical tracker Markdown is invalid")
            || transcript.contains("Lint found"),
        "lint should identify canonical markdown failure for {title}: {transcript}"
    );
    for needle in expected {
        assert!(
            transcript.contains(needle),
            "lint diagnostic for {title} missing {needle:?}: {transcript}"
        );
    }
}

#[test]
fn test_bulk_plan_apply_records_links_export_and_rebuild() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bulk_path = dir.path().join("bulk-plan.json");
    std::fs::write(
        &bulk_path,
        r#"{
  "schema": "atelier.bulk-plan",
  "schema_version": 1,
  "title": "Bulk apply smoke",
  "apply": { "export": "auto" },
  "records": {
    "issues": [
      {
        "client_ref": "issue.blocker",
        "title": "Complete prerequisite",
        "issue_type": "task",
        "priority": "medium",
        "status": "done",
        "labels": ["bulk"]
      },
      {
        "client_ref": "issue.work",
        "title": "Implement bulk output",
        "issue_type": "feature",
        "priority": "high",
        "status": "in_progress",
        "depends_on": [{ "client_ref": "issue.blocker" }],
        "acceptance": ["summary maps client refs"],
        "evidence_required": ["export check passes"]
      }
    ],
    "missions": [
      {
        "client_ref": "mission.bulk",
        "title": "Bulk mission",
        "body": "Mission from bulk plan",
        "labels": ["bulk", "mission"],
        "work": [{ "client_ref": "issue.work" }],
        "plans": [{ "client_ref": "plan.bulk" }],
        "milestones": [{ "client_ref": "milestone.bulk" }]
      }
    ],
    "milestones": [
      {
        "client_ref": "milestone.bulk",
        "title": "Bulk checkpoint",
        "desired_state": "Records are durable",
        "scope": ["records"],
        "validation_criteria": ["rebuild preserves links"],
        "missions": [{ "client_ref": "mission.bulk" }],
        "contributing_work": [{ "client_ref": "issue.work" }]
      }
    ],
    "plans": [
      {
        "client_ref": "plan.bulk",
        "title": "Bulk plan",
        "body": "Apply the graph.",
        "applies_to": [{ "client_ref": "mission.bulk" }]
      }
    ],
    "evidence": [
      {
        "client_ref": "evidence.bulk",
        "title": "Bulk evidence",
        "evidence_type": "test",
        "result": "pass",
        "body": "The apply smoke test passed.",
        "validates": [{ "client_ref": "mission.bulk" }]
      }
    ]
  }
}"#,
    )
    .unwrap();
    let bulk_arg = bulk_path.to_str().unwrap();

    let (success, dry_run_out, stderr) =
        run_atelier(dir.path(), &["plan", "apply", bulk_arg, "--dry-run"]);
    assert!(success, "bulk dry-run failed: {stderr}");
    assert!(dry_run_out.contains("Bulk plan preview is valid."));
    assert!(dry_run_out.contains("Applied:       false"));
    assert!(dry_run_out.contains("missions: 1"));

    let (success, apply_out, stderr) = run_atelier(dir.path(), &["plan", "apply", bulk_arg]);
    assert!(success, "bulk apply failed: {stderr}");
    assert!(apply_out.contains("Bulk plan applied."));
    assert!(apply_out.contains("Applied:       true"));
    assert!(apply_out.contains("atelier mission show"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Bulk mission");

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check after bulk apply failed: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild after bulk apply failed: {stderr}");

    let (success, view_out, stderr) = run_atelier(dir.path(), &["mission", "show", &mission_id]);
    assert!(success, "mission show after bulk apply failed: {stderr}");
    assert!(view_out.contains("Records: plans=1 milestones=1 evidence=1"));
    assert!(view_out.contains("Work: ready=0 blocked=0 done=0 backlog=1"));
    let mission_markdown = std::fs::read_to_string(
        dir.path()
            .join(".atelier/missions")
            .join(format!("{mission_id}.md")),
    )
    .unwrap();
    assert!(mission_markdown.contains("- \"bulk\"\n"));
    assert!(mission_markdown.contains("- \"mission\"\n"));
}

#[test]
fn test_work_commands_are_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for args in [
        vec!["work"],
        vec!["work", "start", "atelier-z1p8"],
        vec!["work", "status"],
    ] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(!success, "{args:?} unexpectedly succeeded");
        let transcript = format!("{stdout}\n{stderr}");
        assert!(
            transcript.contains("unrecognized subcommand 'work'")
                && transcript.contains("Usage: atelier"),
            "missing removed-command transcript for {args:?}: {transcript}"
        );
    }
}

#[test]
fn test_work_lifecycle_human_output_and_guards() {
    let dir = tempdir().unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["init", "-q"])
        .status()
        .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["config", "user.email", "test@example.com"])
        .status()
        .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["config", "user.name", "Test"])
        .status()
        .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["branch", "-M", "main"])
        .status()
        .unwrap();
    init_atelier(dir.path());
    migrate_default_issue_workflow(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "create", "Work item"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = stdout
        .split_whitespace()
        .find(|part| part.starts_with("atelier-"))
        .unwrap()
        .to_string();

    let (success, _, _) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(!success, "dirty worktree should reject root start");

    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    migrate_default_issue_workflow(dir.path());
    std::fs::write(
        dir.path().join("atelier.workflow.yaml"),
        r#"schema: atelier.workflow_config
schema_version: 1
record_types: {}
workflows: {}
validators: {}
hooks:
  write_setup_marker:
    event: worktree_setup
    command:
      argv: [sh, -c, "printf setup > .atelier/setup-marker"]
      env: {}
"#,
    )
    .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["add", "."])
        .status()
        .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["commit", "-q", "-m", "init"])
        .status()
        .unwrap();

    let (success, start_out, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "root start failed: {stderr}");
    assert!(start_out.contains(&format!("Started work on {issue_id}")));
    assert!(start_out.contains("Branch:"));
    assert!(start_out.contains("Worktree:"));

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(status_out.contains("Atelier Status"));
    assert!(status_out.contains("Current work:  1 issue(s)"));
    assert!(status_out.contains(&format!("  {issue_id} - Work item")));

    let (success, abandon_out, stderr) = run_atelier(
        dir.path(),
        &["abandon", &issue_id, "--reason", "switching worktrees"],
    );
    assert!(!success, "abandon should be removed:\n{abandon_out}");
    assert!(
        stderr.contains("unrecognized subcommand 'abandon'"),
        "{stderr}"
    );
    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "work_started",
        &["branch: ", "worktree_path: "],
    );
    assert!(
        !activities
            .iter()
            .any(|activity| activity.contains("event_type: \"work_abandoned\"")),
        "removed abandon command must not record work_abandoned activity:\n{}",
        activities.join("\n--- activity ---\n")
    );

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Worktree item"]);
    assert!(success, "worktree issue create failed: {stderr}");
    let worktree_issue_id = issue_id_by_title(dir.path(), "Worktree item");
    commit_all(dir.path(), "worktree setup baseline");

    let worktree_path = dir
        .path()
        .join(".atelier-worktrees")
        .join(&worktree_issue_id);
    let worktree_arg = worktree_path.to_string_lossy().to_string();
    let (success, worktree_out, stderr) = run_atelier(
        dir.path(),
        &[
            "worktree",
            "for",
            &worktree_issue_id,
            "--path",
            &worktree_arg,
        ],
    );
    assert!(success, "worktree for failed: {stderr}");
    assert!(worktree_out.contains(&worktree_arg));
    assert!(worktree_path.join(".atelier/runtime/state.db").exists());
    assert!(
        !worktree_path.join(".atelier/setup-marker").exists(),
        "root atelier.workflow.yaml hooks should not run during worktree setup"
    );
    let (success, child_status_out, stderr) = run_atelier(&worktree_path, &["status"]);
    assert!(success, "worktree-local status failed: {stderr}");
    assert!(
        child_status_out.contains(&format!("  {worktree_issue_id} - Worktree item")),
        "worktree-local status should derive current work from issue status: {child_status_out}"
    );

    let (success, status_out, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(success, "worktree status failed: {stderr}");
    assert!(status_out.contains(&worktree_arg));
    assert!(status_out.contains(&format!("{worktree_issue_id} [in_progress]")));

    let (success, status_human, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(success, "human worktree status failed: {stderr}");
    assert!(status_human.contains("Worktree Status"));
    assert!(status_human.contains(&worktree_arg));
    assert!(status_human.contains("Branch:"));
    assert!(status_human.contains("State:"));
    assert!(status_human.contains("Associated Work"));
    assert!(status_human.contains(&format!("{worktree_issue_id} [in_progress]")));
    assert!(!status_human.contains("work:"));
    assert!(!status_human.contains("export:"));

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["worktree", "remove", "--force", &worktree_arg])
        .status()
        .unwrap();
    assert!(status.success(), "manual git worktree remove failed");
    assert!(!worktree_path.exists());
    let (success, repair_out, stderr) =
        run_atelier(dir.path(), &["worktree", "repair", &worktree_issue_id]);
    assert!(success, "worktree repair failed: {stderr}");
    assert!(repair_out.contains("Cleared stale worktree path"));
    let (success, repaired_status, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(success, "worktree status after repair failed: {stderr}");
    assert!(!repaired_status.contains(&worktree_issue_id));

    migrate_default_issue_workflow(dir.path());
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Failed setup worktree"]);
    assert!(success, "failed setup issue create failed: {stderr}");
    let failed_issue_id = issue_id_by_title(dir.path(), "Failed setup worktree");
    std::fs::write(
        dir.path().join("atelier.workflow.yaml"),
        r#"schema: atelier.workflow_config
schema_version: 1
record_types: {}
workflows: {}
validators: {}
hooks:
  fail_setup:
    event: worktree_setup
    command:
      argv: [sh, -c, "exit 12"]
      env: {}
"#,
    )
    .unwrap();
    commit_all(dir.path(), "failing worktree hook");
    let failed_worktree_path = dir.path().join(".atelier-worktrees").join(&failed_issue_id);
    let failed_worktree_arg = failed_worktree_path.to_string_lossy().to_string();
    let (success, failed_out, stderr) = run_atelier(
        dir.path(),
        &[
            "worktree",
            "for",
            &failed_issue_id,
            "--path",
            &failed_worktree_arg,
        ],
    );
    assert!(
        success,
        "root atelier.workflow.yaml hook should be ignored, not fail setup: {stderr}"
    );
    assert!(failed_out.contains(&failed_worktree_arg));
    let (success, failed_status, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(
        success,
        "worktree status after failed setup failed: {stderr}"
    );
    assert!(failed_status.contains(&format!("{failed_issue_id} [in_progress]")));
}

#[test]
fn test_start_prepares_child_standalone_and_epic_owner_branches_before_transition() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    Command::new("git")
        .current_dir(dir.path())
        .args(["branch", "-M", "main"])
        .status()
        .unwrap();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Owner epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Owner epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Child work", "--parent", &epic_id],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Child work");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Standalone work"]);
    assert!(success, "standalone create failed: {stderr}");
    let standalone_id = issue_id_by_title(dir.path(), "Standalone work");
    commit_all(dir.path(), "initial tracker state");

    let (success, child_out, stderr) = run_atelier(dir.path(), &["start", &child_id]);
    assert!(success, "child start failed: {stderr}");
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    assert!(child_out.contains(&format!("Started work on {child_id} Child work")));
    assert!(child_out.contains(&format!("Branch owner: epic {epic_id} (epic)")));
    assert!(child_out.contains(&format!("Effective branch: epic/{epic_id}")));
    assert!(child_out.contains("Base branch: main"));
    assert!(child_out.contains(&format!(
        "Record proof: atelier evidence record --target issue/{child_id}"
    )));
    let (success, child_show, stderr) = run_atelier(dir.path(), &["issue", "show", &child_id]);
    assert!(success, "child show failed: {stderr}");
    assert!(child_show.contains("Status:   in_progress"), "{child_show}");

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "main"])
        .status()
        .unwrap();
    assert!(status.success(), "switch back to main failed");
    let (success, standalone_out, stderr) = run_atelier(dir.path(), &["start", &standalone_id]);
    assert!(success, "standalone start failed: {stderr}");
    assert_eq!(
        git_current_branch(dir.path()),
        format!("codex/{standalone_id}")
    );
    assert!(standalone_out.contains(&format!("Branch owner: issue {standalone_id} (task)")));
    assert!(standalone_out.contains(&format!("Effective branch: codex/{standalone_id}")));
    let (success, standalone_show, stderr) =
        run_atelier(dir.path(), &["issue", "show", &standalone_id]);
    assert!(success, "standalone show failed: {stderr}");
    assert!(
        standalone_show.contains("Status:   in_progress"),
        "{standalone_show}"
    );

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "main"])
        .status()
        .unwrap();
    assert!(status.success(), "switch back to main failed");
    let (success, epic_out, stderr) = run_atelier(dir.path(), &["start", &epic_id]);
    assert!(success, "epic start failed: {stderr}");
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    assert!(epic_out.contains(&format!("Branch owner: epic {epic_id} (epic)")));
    let (success, epic_show, stderr) = run_atelier(dir.path(), &["issue", "show", &epic_id]);
    assert!(success, "epic show failed: {stderr}");
    assert!(epic_show.contains("Status:   in_progress"), "{epic_show}");
}

#[test]
fn test_branch_lifecycle_context_surfaces_on_status_issue_transition_and_mission_status() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    Command::new("git")
        .current_dir(dir.path())
        .args(["branch", "-M", "main"])
        .status()
        .unwrap();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Lifecycle epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Lifecycle epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Lifecycle child", "--parent", &epic_id],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Lifecycle child");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Lifecycle solo"]);
    assert!(success, "standalone create failed: {stderr}");
    let standalone_id = issue_id_by_title(dir.path(), "Lifecycle solo");
    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Lifecycle mission"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Lifecycle mission");
    for id in [&epic_id, &standalone_id] {
        let (success, _, stderr) =
            run_atelier(dir.path(), &["mission", "add-work", &mission_id, id]);
        assert!(success, "mission add-work failed for {id}: {stderr}");
    }
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "start", &mission_id, "--switch"]);
    assert!(success, "mission start failed: {stderr}");
    commit_all(dir.path(), "initial lifecycle context tracker state");

    let (success, base_status, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "base status failed: {stderr}");
    assert!(base_status.contains("Branch Lifecycle"), "{base_status}");
    assert!(
        base_status.contains("Current branch: main"),
        "{base_status}"
    );
    assert!(
        base_status.contains("Base branch:    main"),
        "{base_status}"
    );
    assert!(
        base_status.contains("Branch owner:   (unknown)"),
        "{base_status}"
    );
    assert!(!base_status.contains("branch for-epic"), "{base_status}");

    for (id, owner, expected, scope) in [
        (
            child_id.as_str(),
            format!("Owner:    epic {epic_id} (epic)"),
            format!("Expected: epic/{epic_id}"),
            "Scope:    nested under epic; merge is deferred to epic close",
        ),
        (
            standalone_id.as_str(),
            format!("Owner:    issue {standalone_id} (task)"),
            format!("Expected: codex/{standalone_id}"),
            "Scope:    owns its merge branch",
        ),
        (
            epic_id.as_str(),
            format!("Owner:    epic {epic_id} (epic)"),
            format!("Expected: epic/{epic_id}"),
            "Scope:    owns its merge branch",
        ),
    ] {
        let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", id]);
        assert!(success, "issue show failed for {id}: {stderr}");
        assert!(show_out.contains("Branch Lifecycle"), "{show_out}");
        assert!(show_out.contains(&owner), "{show_out}");
        assert!(show_out.contains(&expected), "{show_out}");
        assert!(show_out.contains(scope), "{show_out}");
        assert!(
            show_out.contains(&format!("Next:     atelier start {id}")),
            "{show_out}"
        );
        assert!(
            show_out.contains(&format!(
                "Close:    atelier issue close {id} --reason \"...\""
            )),
            "{show_out}"
        );
        assert!(!show_out.contains("branch for-epic"), "{show_out}");

        let (success, options_out, stderr) =
            run_atelier(dir.path(), &["issue", "transition", id, "--options"]);
        assert!(success, "transition options failed for {id}: {stderr}");
        assert!(options_out.contains("Branch Context"), "{options_out}");
        assert!(options_out.contains(&owner), "{options_out}");
        assert!(options_out.contains(&expected), "{options_out}");
        assert!(
            options_out.contains(&format!("Corrective lifecycle command: atelier start {id}")),
            "{options_out}"
        );
        assert!(!options_out.contains("branch for-epic"), "{options_out}");
    }

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &child_id]);
    assert!(success, "child start failed: {stderr}");
    let (success, epic_status, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "epic branch status failed: {stderr}");
    assert!(
        epic_status.contains(&format!("Current branch: epic/{epic_id}")),
        "{epic_status}"
    );
    assert!(
        epic_status.contains(&format!("Branch owner:   epic {epic_id} (epic)")),
        "{epic_status}"
    );
    assert!(
        epic_status.contains(&format!(
            "{child_id} - owner epic {epic_id} (epic) | expected epic/{epic_id} | ok"
        )),
        "{epic_status}"
    );

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "main"])
        .status()
        .unwrap();
    assert!(status.success(), "switch to main failed");
    let (success, _, stderr) = run_atelier(dir.path(), &["start", &standalone_id]);
    assert!(success, "standalone start failed: {stderr}");
    let (success, issue_status, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "issue branch status failed: {stderr}");
    assert!(
        issue_status.contains(&format!("Current branch: codex/{standalone_id}")),
        "{issue_status}"
    );
    assert!(
        issue_status.contains(&format!("Branch owner:   issue {standalone_id} (task)")),
        "{issue_status}"
    );
    assert!(
        issue_status.contains(&format!(
            "{standalone_id} - owner issue {standalone_id} (task) | expected codex/{standalone_id} | ok"
        )),
        "{issue_status}"
    );
    assert!(
        issue_status.contains(&format!("{child_id} - owner epic {epic_id} (epic)"))
            && issue_status.contains(&format!("mismatch; run `atelier start {child_id}`")),
        "{issue_status}"
    );

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "main"])
        .status()
        .unwrap();
    assert!(status.success(), "switch to wrong branch failed");
    let (success, wrong_status, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "wrong branch status failed: {stderr}");
    assert!(
        wrong_status.contains("Current branch: main"),
        "{wrong_status}"
    );
    assert!(
        wrong_status.contains(&format!("mismatch; run `atelier start {child_id}`")),
        "{wrong_status}"
    );
    assert!(
        wrong_status.contains(&format!("mismatch; run `atelier start {standalone_id}`")),
        "{wrong_status}"
    );

    let (success, mission_status, stderr) =
        run_atelier(dir.path(), &["mission", "status", &mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(
        mission_status.contains("Branch Lifecycle"),
        "{mission_status}"
    );
    assert!(
        mission_status.contains(&format!("epic {epic_id} (epic) -> epic/{epic_id}")),
        "{mission_status}"
    );
    assert!(
        mission_status.contains(&format!(
            "issue {standalone_id} (task) -> codex/{standalone_id}"
        )),
        "{mission_status}"
    );
    assert!(mission_status.contains("Dirty state:"), "{mission_status}");
    assert!(
        mission_status.contains(&format!(
            "{child_id} expected epic/{epic_id}; run `atelier start {child_id}`"
        )),
        "{mission_status}"
    );
    assert!(
        mission_status.contains(&format!(
            "{standalone_id} expected codex/{standalone_id}; run `atelier start {standalone_id}`"
        )),
        "{mission_status}"
    );
    assert!(
        !mission_status.contains("branch for-epic"),
        "{mission_status}"
    );
}

#[test]
fn test_start_dirty_worktree_leaves_tracker_state_unchanged() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    Command::new("git")
        .current_dir(dir.path())
        .args(["branch", "-M", "main"])
        .status()
        .unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Dirty start"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Dirty start");
    commit_all(dir.path(), "initial tracker state");
    std::fs::write(dir.path().join("dirty.txt"), "dirty\n").unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(!success, "dirty start unexpectedly succeeded:\n{stdout}");
    assert!(
        stderr.contains("Worktree has uncommitted changes") && stderr.contains("dirty.txt"),
        "{stderr}"
    );
    assert_eq!(git_current_branch(dir.path()), "main");
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(show_out.contains("Status:   todo"), "{show_out}");
}

#[test]
fn test_start_branch_checkout_failure_leaves_tracker_state_unchanged() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    Command::new("git")
        .current_dir(dir.path())
        .args(["branch", "-M", "main"])
        .status()
        .unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Checkout failure"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Checkout failure");
    commit_all(dir.path(), "initial tracker state");
    let other_dir = tempdir().unwrap();
    let other_worktree = other_dir.path().join("other-worktree");
    let expected_branch = format!("codex/{issue_id}");
    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["worktree", "add", "-b", &expected_branch])
        .arg(&other_worktree)
        .arg("main")
        .status()
        .unwrap();
    assert!(status.success(), "git worktree add failed");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(
        !success,
        "checkout-failure start unexpectedly succeeded:\n{stdout}"
    );
    assert!(
        stderr.contains("Branch checkout failed before workflow transition")
            && stderr.contains("Next command: git status --short --branch"),
        "{stderr}"
    );
    assert_eq!(git_current_branch(dir.path()), "main");
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(show_out.contains("Status:   todo"), "{show_out}");
}

#[test]
fn test_child_issue_close_commits_on_epic_branch_without_base_merge() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    Command::new("git")
        .current_dir(dir.path())
        .args(["branch", "-M", "main"])
        .status()
        .unwrap();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Close owner epic",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Close owner epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Child close work", "--parent", &epic_id],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Child close work");
    commit_all(dir.path(), "initial tracker state");
    let main_before = git_rev_parse(dir.path(), "main");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &child_id]);
    assert!(success, "child start failed: {stderr}");
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    ensure_all_issue_closeout_sections(dir.path());
    attach_issue_pass_evidence(dir.path(), &child_id);

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &child_id, "--reason", "done"],
    );
    assert!(success, "child issue close failed: {stderr}");
    assert!(close_out.contains("Close Git Integration"), "{close_out}");
    assert!(close_out.contains(&format!("Target:        issue/{child_id}")));
    assert!(close_out.contains(&format!("Branch owner:  epic {epic_id} (epic)")));
    assert!(close_out.contains(&format!("Source branch: epic/{epic_id}")));
    assert!(close_out.contains("Base branch:   main"));
    assert!(close_out.contains("Merge strategy: squash"));
    assert!(close_out.contains("Merge result:   deferred to epic close"));
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    assert_eq!(git_rev_parse(dir.path(), "main"), main_before);
    assert!(git_log_oneline(dir.path(), &format!("epic/{epic_id}"), 1)
        .contains(&format!("Close {child_id}: Child close work")));
    let dirty = git_status_short(dir.path());
    assert!(dirty.trim().is_empty(), "{dirty}");
}

#[test]
fn test_standalone_issue_close_squash_merges_to_base() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    Command::new("git")
        .current_dir(dir.path())
        .args(["branch", "-M", "main"])
        .status()
        .unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Standalone close"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Standalone close");
    commit_all(dir.path(), "initial tracker state");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "start failed: {stderr}");
    std::fs::write(dir.path().join("standalone.txt"), "standalone work\n").unwrap();
    commit_all(dir.path(), "standalone implementation");
    ensure_all_issue_closeout_sections(dir.path());
    attach_issue_pass_evidence(dir.path(), &issue_id);

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &issue_id, "--reason", "done"],
    );
    assert!(success, "standalone close failed: {stderr}");
    assert_eq!(git_current_branch(dir.path()), "main");
    assert!(close_out.contains(&format!("Branch owner:  issue {issue_id} (task)")));
    assert!(close_out.contains(&format!("Source branch: codex/{issue_id}")));
    assert!(close_out.contains("Merge strategy: squash"));
    assert!(close_out.contains("Merge result:   squash commit"));
    let main_log = git_log_oneline(dir.path(), "main", 2);
    assert!(
        main_log.contains(&format!("Squash merge codex/{issue_id} into main")),
        "{main_log}"
    );
    assert!(
        !main_log.contains("standalone implementation"),
        "{main_log}"
    );
    assert!(dir.path().join("standalone.txt").exists());
    let dirty = git_status_short(dir.path());
    assert!(dirty.trim().is_empty(), "{dirty}");
}

#[test]
fn test_epic_close_squash_merges_to_base_after_child_proof() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    Command::new("git")
        .current_dir(dir.path())
        .args(["branch", "-M", "main"])
        .status()
        .unwrap();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Closable epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Closable epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Epic child proof", "--parent", &epic_id],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Epic child proof");
    commit_all(dir.path(), "initial tracker state");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &child_id]);
    assert!(success, "child start failed: {stderr}");
    ensure_all_issue_closeout_sections(dir.path());
    attach_issue_pass_evidence(dir.path(), &child_id);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &child_id, "--reason", "done"],
    );
    assert!(success, "child close failed: {stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &epic_id]);
    assert!(success, "epic start failed: {stderr}");
    move_issue_to_validation(dir.path(), &epic_id);
    ensure_all_issue_closeout_sections(dir.path());
    attach_issue_pass_evidence(dir.path(), &epic_id);

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &epic_id, "--reason", "done"],
    );
    assert!(success, "epic close failed: {stderr}");
    assert_eq!(git_current_branch(dir.path()), "main");
    assert!(close_out.contains(&format!("Branch owner:  epic {epic_id} (epic)")));
    assert!(close_out.contains(&format!("Source branch: epic/{epic_id}")));
    assert!(close_out.contains("Merge result:   squash commit"));
    let main_log = git_log_oneline(dir.path(), "main", 2);
    assert!(
        main_log.contains(&format!("Squash merge epic/{epic_id} into main")),
        "{main_log}"
    );
    assert!(git_status_short(dir.path()).trim().is_empty());
}

#[test]
fn test_issue_close_merge_failure_rolls_back_terminal_tracker_state() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    Command::new("git")
        .current_dir(dir.path())
        .args(["branch", "-M", "main"])
        .status()
        .unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Conflict close"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Conflict close");
    commit_all(dir.path(), "initial tracker state");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "start failed: {stderr}");
    std::fs::write(dir.path().join("conflict.txt"), "issue branch\n").unwrap();
    commit_all(dir.path(), "issue branch conflict content");
    ensure_all_issue_closeout_sections(dir.path());
    attach_issue_pass_evidence(dir.path(), &issue_id);
    commit_all(dir.path(), "issue proof ready before conflict close");

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "main"])
        .status()
        .unwrap();
    assert!(status.success(), "switch to main failed");
    std::fs::write(dir.path().join("conflict.txt"), "main branch\n").unwrap();
    commit_all(dir.path(), "main branch conflict content");
    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", &format!("codex/{issue_id}")])
        .status()
        .unwrap();
    assert!(status.success(), "switch back to issue branch failed");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &issue_id, "--reason", "done"],
    );
    assert!(
        !success,
        "conflicting close unexpectedly succeeded:\n{stdout}"
    );
    assert!(stdout.contains("Close Git Integration"), "{stdout}");
    assert!(
        stderr.contains("Close Git integration failed during squash merge")
            && stderr.contains("Recovery:")
            && stderr.contains(&format!("atelier issue close {issue_id} --reason")),
        "{stderr}"
    );
    assert_eq!(git_current_branch(dir.path()), format!("codex/{issue_id}"));
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "issue show failed after rollback: {stderr}");
    assert!(!show_out.contains("Status:   done"), "{show_out}");
    assert!(show_out.contains("Status:   in_progress"), "{show_out}");
    let dirty = git_status_short(dir.path());
    assert!(dirty.trim().is_empty(), "{dirty}");
}

#[test]
fn test_root_repair_is_removed() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    migrate_default_issue_workflow(dir.path());

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Stale active work"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(stdout.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Stale active work");
    commit_all(dir.path(), "stale active work baseline");

    let (success, repair_out, stderr) = run_atelier(dir.path(), &["repair", &issue_id]);
    assert!(!success, "repair should be removed:\n{repair_out}");
    assert!(
        stderr.contains("unrecognized subcommand 'repair'"),
        "{stderr}"
    );
}

#[test]
fn test_worktree_setup_failure_does_not_associate_and_can_retry() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Retriable setup worktree"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Retriable setup worktree");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "valid tracker state");

    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let valid_markdown = std::fs::read_to_string(&issue_path).unwrap();
    let malformed_markdown =
        valid_markdown.replace("\n## Outcome\n\nOutcome was not specified.\n", "\n");
    std::fs::write(&issue_path, malformed_markdown).unwrap();
    commit_all(dir.path(), "malformed tracker state");

    let worktree_path = dir.path().join(".atelier-worktrees").join(&issue_id);
    let worktree_arg = worktree_path.to_string_lossy().to_string();
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["worktree", "for", &issue_id, "--path", &worktree_arg],
    );
    assert!(!success, "malformed setup unexpectedly succeeded: {stdout}");
    assert!(
        stderr.contains("Missing required issue body section 'Outcome'"),
        "failure should report invalid canonical issue content: {stderr}"
    );

    let (success, status_out, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(
        success,
        "worktree status after failed setup failed: {stderr}"
    );
    assert!(
        status_out.contains("Associated Work") && status_out.contains("  (none)"),
        "failed setup should not record parent association: {status_out}"
    );

    std::fs::write(&issue_path, valid_markdown).unwrap();
    commit_all(dir.path(), "fix tracker state");

    let (success, retry_out, stderr) = run_atelier(
        dir.path(),
        &["worktree", "for", &issue_id, "--path", &worktree_arg],
    );
    assert!(success, "retrying worktree setup failed: {stderr}");
    assert!(retry_out.contains(&worktree_arg));

    let (success, root_status_out, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(success, "root worktree status after retry failed: {stderr}");
    assert!(root_status_out.contains(&format!("{issue_id} [in_progress]")));

    let (success, child_status_out, stderr) = run_atelier(&worktree_path, &["status"]);
    assert!(success, "child status after retry failed: {stderr}");
    assert!(child_status_out.contains("Current work:  1 issue(s)"));
    assert!(child_status_out.contains(&format!("  {issue_id} - Retriable setup worktree")));
}

#[test]
fn test_mission_worktree_ownership_guards_epic_branch_commands() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    migrate_default_issue_workflow(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Owner mission"]);
    assert!(success, "owner mission create failed: {stderr}");
    let owner_mission_id = record_id_by_title(dir.path(), "missions", "Owner mission");

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Other mission"]);
    assert!(success, "other mission create failed: {stderr}");
    let other_mission_id = record_id_by_title(dir.path(), "missions", "Other mission");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Owner epic", "--issue-type", "epic"],
    );
    assert!(success, "owner epic create failed: {stderr}");
    let owner_epic_id = issue_id_by_title(dir.path(), "Owner epic");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-work", &owner_mission_id, &owner_epic_id],
    );
    assert!(success, "owner mission add epic failed: {stderr}");
    commit_all(dir.path(), "mission worktree branch baseline");

    let owner_worktree_path = dir.path().join("custom-workspaces").join("owner-space");
    let owner_worktree_arg = owner_worktree_path.to_string_lossy().to_string();
    let (success, owner_worktree_out, stderr) = run_atelier(
        dir.path(),
        &[
            "worktree",
            "for-mission",
            &owner_mission_id,
            "--path",
            &owner_worktree_arg,
        ],
    );
    assert!(success, "owner mission worktree setup failed: {stderr}");
    assert!(owner_worktree_out.contains(&owner_worktree_arg));
    assert!(owner_worktree_out.contains(&format!("Mission: {owner_mission_id}")));

    let other_worktree_path = dir.path().join("custom-workspaces").join("other-space");
    let other_worktree_arg = other_worktree_path.to_string_lossy().to_string();
    let (success, other_worktree_out, stderr) = run_atelier(
        dir.path(),
        &[
            "worktree",
            "for-mission",
            &other_mission_id,
            "--path",
            &other_worktree_arg,
        ],
    );
    assert!(success, "other mission worktree setup failed: {stderr}");
    assert!(other_worktree_out.contains(&other_worktree_arg));

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["branch", "for-epic", &owner_epic_id]);
    assert!(
        !success,
        "root checkout unexpectedly allowed branch for-epic"
    );
    let root_for_transcript = format!("{stdout}\n{stderr}");
    assert!(
        root_for_transcript.contains(&format!(
            "atelier branch for-epic must be run inside the owning mission worktree for {owner_mission_id}"
        )),
        "missing root guard transcript:\n{root_for_transcript}"
    );
    assert!(
        root_for_transcript.contains(&format!("atelier worktree for-mission {owner_mission_id}")),
        "missing root recovery guidance:\n{root_for_transcript}"
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["branch", "status"]);
    assert!(!success, "root checkout unexpectedly allowed branch status");
    let root_status_transcript = format!("{stdout}\n{stderr}");
    assert!(
        root_status_transcript
            .contains("atelier branch status must be run inside a mission worktree"),
        "missing root branch status guard:\n{root_status_transcript}"
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["branch", "merge", &owner_epic_id]);
    assert!(!success, "root checkout unexpectedly allowed branch merge");
    let root_merge_transcript = format!("{stdout}\n{stderr}");
    assert!(
        root_merge_transcript.contains(&format!(
            "atelier branch merge must be run inside the owning mission worktree for {owner_mission_id}"
        )),
        "missing root merge guard transcript:\n{root_merge_transcript}"
    );

    let (success, stdout, stderr) = run_atelier(
        &other_worktree_path,
        &["branch", "for-epic", &owner_epic_id],
    );
    assert!(
        !success,
        "wrong mission worktree unexpectedly allowed branch for-epic"
    );
    let wrong_worktree_transcript = format!("{stdout}\n{stderr}");
    assert!(
        wrong_worktree_transcript.contains(&format!(
            "Current checkout {} belongs to mission {}",
            other_worktree_path.display(),
            other_mission_id
        )),
        "missing wrong-worktree mission transcript:\n{wrong_worktree_transcript}"
    );

    let (success, branch_out, stderr) = run_atelier(
        &owner_worktree_path,
        &["branch", "for-epic", &owner_epic_id],
    );
    assert!(success, "owner mission branch for-epic failed: {stderr}");
    assert!(branch_out.contains(&format!("Switched to epic/{owner_epic_id}")));
    assert!(branch_out.contains(&format!("Mission: {owner_mission_id}")));
    assert!(branch_out.contains(&format!("Worktree: {}", owner_worktree_path.display())));

    let (success, branch_status_out, stderr) =
        run_atelier(&owner_worktree_path, &["branch", "status"]);
    assert!(success, "owner mission branch status failed: {stderr}");
    assert!(branch_status_out.contains("Epic Branch Status"));
    assert!(branch_status_out.contains(&format!("Mission: {owner_mission_id}")));
    assert!(branch_status_out.contains(&format!("Worktree: {}", owner_worktree_path.display())));
    assert!(branch_status_out.contains(&format!("epic/{owner_epic_id} - Owner epic")));

    let (success, worktree_status_out, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(success, "worktree status failed: {stderr}");
    let owner_marker = format!(
        "{owner_worktree_arg}\n{}",
        "-".repeat(owner_worktree_arg.len())
    );
    let owner_section = worktree_status_out
        .split(&owner_marker)
        .nth(1)
        .and_then(|section| {
            section
                .split(&format!(
                    "{other_worktree_arg}\n{}",
                    "-".repeat(other_worktree_arg.len())
                ))
                .next()
        })
        .expect("owner mission worktree section missing from status");
    assert!(
        owner_section.contains(&format!("Mission:  {owner_mission_id}")),
        "custom mission worktree did not retain mission ownership:\n{worktree_status_out}"
    );

    std::fs::write(
        owner_worktree_path.join("branch-proof.txt"),
        "epic branch work\n",
    )
    .unwrap();
    let status = Command::new("git")
        .current_dir(&owner_worktree_path)
        .args(["add", "branch-proof.txt"])
        .status()
        .unwrap();
    assert!(status.success(), "git add in owner mission worktree failed");
    let status = Command::new("git")
        .current_dir(&owner_worktree_path)
        .args(["commit", "-q", "-m", "epic branch work"])
        .status()
        .unwrap();
    assert!(
        status.success(),
        "git commit in owner mission worktree failed"
    );
    let status = Command::new("git")
        .current_dir(&owner_worktree_path)
        .args(["switch", &format!("mission/{owner_mission_id}")])
        .status()
        .unwrap();
    assert!(status.success(), "git switch back to mission branch failed");

    let (success, stdout, stderr) =
        run_atelier(&other_worktree_path, &["branch", "merge", &owner_epic_id]);
    assert!(
        !success,
        "wrong mission worktree unexpectedly allowed branch merge"
    );
    let wrong_merge_transcript = format!("{stdout}\n{stderr}");
    assert!(
        wrong_merge_transcript.contains(&format!(
            "Current checkout {} belongs to mission {}",
            other_worktree_path.display(),
            other_mission_id
        )),
        "missing wrong-worktree merge transcript:\n{wrong_merge_transcript}"
    );

    let (success, merge_out, stderr) =
        run_atelier(&owner_worktree_path, &["branch", "merge", &owner_epic_id]);
    assert!(success, "owner mission branch merge failed: {stderr}");
    assert!(merge_out.contains(&format!("Merged epic/{owner_epic_id}")));
    assert!(merge_out.contains(&format!("Mission: {owner_mission_id}")));
    assert!(merge_out.contains(&format!("Worktree: {}", owner_worktree_path.display())));
}

#[test]
fn test_start_refuses_shared_section_diagnostic() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Malformed section work"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(stdout.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Malformed section work");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "valid issue");

    let issue_path = dir
        .path()
        .join(".atelier/issues")
        .join(format!("{issue_id}.md"));
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    let malformed = markdown.replace("\n## Outcome\n\nOutcome was not specified.\n", "\n");
    std::fs::write(&issue_path, malformed).unwrap();
    commit_all(dir.path(), "malformed issue section");

    let (lint_success, lintstdout, lint_stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(!lint_success, "lint should report malformed issue sections");
    let lint_transcript = format!("{lintstdout}\n{lint_stderr}");
    for needle in [
        "Missing required issue body section 'Outcome'",
        &issue_id,
        "section Outcome",
        ".atelier/issues/",
    ] {
        assert!(
            lint_transcript.contains(needle),
            "lint diagnostic missing {needle:?}: {lint_transcript}"
        );
    }

    let (start_success, startstdout, start_stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(
        !start_success,
        "start should refuse malformed issue sections"
    );
    let start_transcript = format!("{startstdout}\n{start_stderr}");
    for needle in [
        "Missing required issue body section 'Outcome'",
        &issue_id,
        "section Outcome",
        ".atelier/issues/",
    ] {
        assert!(
            start_transcript.contains(needle),
            "start diagnostic missing {needle:?}: {start_transcript}"
        );
    }
}

#[test]
fn test_issue_type_is_canonical_not_label_derived() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Typed issue",
            "--issue-type",
            "validation",
            "--label",
            "epic",
        ],
    );
    assert!(success, "create failed: {stderr}");
    assert!(stdout.contains("Type:     validation"));
    let issue_id = issue_id_by_title(dir.path(), "Typed issue");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Typed issue"));
    assert!(stdout.contains("Category: todo"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(success, "list failed: {stderr}");
    assert!(stdout.contains("validation"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success, "ready failed: {stderr}");
    assert!(stdout.contains("validation"));

    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    let issue_record = std::fs::read_to_string(
        dir.path()
            .join(".atelier/issues")
            .join(format!("{issue_id}.md")),
    )
    .unwrap();
    assert!(issue_record.contains("issue_type: \"validation\"\n"));
    assert!(issue_record.contains("labels:\n- \"epic\"\n"));
}

#[test]
fn test_import_beads_reports_mapping_without_tracker_provenance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/beads/issues.manual.jsonl");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["import-beads", fixture.to_str().unwrap()]);
    assert!(success, "import-beads failed: {stderr}");
    assert!(stdout.contains("imported issues: 3"));
    assert!(stdout.contains("blocking relationships: 1"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "3"]);
    assert!(success, "mapped show failed: {stderr}");
    assert!(stdout.contains("atelier-0003"));
    assert!(stdout.contains("[task]"));
    assert!(stdout.contains("Parent: atelier-0001"));
    assert!(stdout.contains("atelier-0002"));
    assert!(!stdout.contains("beads:"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "blocked", "3"]);
    assert!(success, "mapped issue blocked failed: {stderr}");
    assert!(stdout.contains("atelier-0003"));
    assert!(stdout.contains("atelier-0002"));
}
