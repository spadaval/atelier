use super::*;

fn create_mission_fixture(dir: &std::path::Path, title: &str) -> String {
    let bundle_path = dir.join(format!("mission-fixture-{}.json", title.replace(' ', "-")));
    std::fs::write(
        &bundle_path,
        format!(
            r#"{{
  "schema": "atelier.bundle",
  "schema_version": 1,
  "title": "Mission fixture",
  "resources": {{
    "issues": [
      {{
        "client_ref": "mission.fixture",
        "title": {title:?},
        "issue_type": "mission",
        "description": "Mission fixture body.",
        "labels": ["mission"]
      }}
    ]
  }}
}}"#
        ),
    )
    .unwrap();
    let (success, _stdout, stderr) = run_atelier(
        dir,
        &["bundle", "apply", bundle_path.to_str().unwrap(), "--yes"],
    );
    assert!(success, "mission fixture bundle apply failed: {stderr}");
    let mission_id = issue_id_by_title(dir, title);
    let (success, _stdout, stderr) =
        run_atelier(dir, &["issue", "transition", &mission_id, "ready"]);
    assert!(success, "mission fixture ready transition failed: {stderr}");
    mission_id
}

fn move_mission_to_ready(dir: &std::path::Path, mission_id: &str) {
    let (success, _stdout, stderr) =
        run_atelier(dir, &["issue", "transition", mission_id, "ready"]);
    assert!(success, "mission ready transition failed: {stderr}");
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
        run_atelier(dir.path(), &["work", "queue", "--status", "all"]);
    assert!(success, "ready list failed: {stderr}");
    assert!(ready_out.contains(&ready_id), "{ready_out}");

    let policy_path = dir.path().join(".atelier").join("workflow.yaml");
    let policy = std::fs::read_to_string(&policy_path).unwrap();
    std::fs::write(
        &policy_path,
        policy.replacen(
            "      start:\n        from: [todo, blocked]\n        to: in_progress\n",
            "      start:\n        from: [todo, blocked]\n        to: in_progress\n        validators: [evidence.attached]\n",
            1,
        ),
    )
    .unwrap();

    let (success, blocked_ready_out, stderr) =
        run_atelier(dir.path(), &["work", "queue", "--ready"]);
    assert!(
        success,
        "ready list with blocked transition should remain readable: {stderr}"
    );
    assert!(blocked_ready_out.contains(&ready_id), "{blocked_ready_out}");
    assert!(
        blocked_ready_out.contains("Ready transition"),
        "{blocked_ready_out}"
    );

    let (success, options_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &ready_id]);
    assert!(success, "transition options failed: {stderr}");
    assert!(options_out.contains("start [blocked]"), "{options_out}");
    assert!(options_out.contains("evidence.attached"), "{options_out}");
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
fn test_lint_rejects_missing_outcome_section() {
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
        "missing Outcome diagnostic in stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stderr.contains("Lint failed"));
}

#[test]
fn test_lint_rejects_empty_description_section() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Empty description lint"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Empty description lint");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    let invalid = markdown.replace(
        "## Description\n\nNo description provided.",
        "## Description\n\n",
    );
    std::fs::write(&issue_path, invalid).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(!success, "lint should fail for empty Description");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Description")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing Description diagnostic in stdout:\n{stdout}\nstderr:\n{stderr}"
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
        "## Outcome\n\nOutcome was not specified.",
        "## Outcome\n\nOutcome was not specified.\n\n## Outcome\n\nSecond outcome should be rejected.",
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

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
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
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "start failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_review"],
    );
    assert!(success, "request_review failed: {stderr}");
    complete_room_review(dir.path(), &issue_id);
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
        &[
            "issue",
            "transition",
            &issue_id,
            "close",
            "--reason",
            "done",
        ],
    );
    assert!(!success, "transition close should refuse invalid issue");
    assert!(
        stderr.contains(&format!("issue {issue_id}"))
            && stderr.contains("section Outcome")
            && stderr.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing closeout diagnostic, stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[test]
fn test_mission_terminal_status_and_options_use_configured_objective_validators() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let mission_id = create_mission_fixture(dir.path(), "Configured validator blockers");

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Configured open work"]);
    assert!(success, "issue create failed: {stderr}");
    let work_id = issue_id_by_title(dir.path(), "Configured open work");
    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "link", &mission_id, &work_id]);
    assert!(success, "mission link failed: {stderr}");
    commit_all(dir.path(), "configured validator blocked fixture");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["issue", "show", &mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Health        : ready"), "{status_out}");
    assert!(status_out.contains("Ready Work"), "{status_out}");
    assert!(status_out.contains(&work_id), "{status_out}");

    let (success, options_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &mission_id, "--verbose"],
    );
    assert!(success, "mission transition options failed: {stderr}");
    assert!(
        options_out.contains("objective.work_present"),
        "{options_out}"
    );
    assert!(
        options_out.contains("objective.work_terminal"),
        "{options_out}"
    );
    assert!(options_out.contains(&work_id), "{options_out}");
    assert!(
        options_out.contains("open advancing work via advances"),
        "{options_out}"
    );
}

#[test]
fn test_mission_close_uses_configured_objective_validators() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let mission_id = create_mission_fixture(dir.path(), "Configured validator close");

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Configured terminal work"]);
    assert!(success, "issue create failed: {stderr}");
    let work_id = issue_id_by_title(dir.path(), "Configured terminal work");
    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "link", &mission_id, &work_id]);
    assert!(success, "mission link failed: {stderr}");
    commit_all(dir.path(), "configured validator close fixture");

    close_issue_with_evidence(dir.path(), &work_id, Some("done"));
    attach_pass_evidence(
        dir.path(),
        "mission",
        &mission_id,
        "configured mission proof",
    );
    commit_all(dir.path(), "configured validator close ready");

    let (success, options_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &mission_id, "--verbose"],
    );
    assert!(success, "mission transition options failed: {stderr}");
    assert!(
        options_out.contains("pass  objective.work_present"),
        "{options_out}"
    );
    assert!(
        options_out.contains("pass  objective.work_terminal"),
        "{options_out}"
    );
    assert!(
        options_out.contains("pass  objective.blockers_none_open"),
        "{options_out}"
    );

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &mission_id,
            "close",
            "--reason",
            "configured validators passed",
        ],
    );
    assert!(success, "mission close failed: {stderr}");
    assert!(
        close_out.contains("Applied transition close"),
        "{close_out}"
    );
    assert!(close_out.contains("To:       closed"), "{close_out}");
    assert!(close_out.contains("atelier issue show"), "{close_out}");
}

#[test]
fn test_root_status_reports_current_mission_counts_without_active_focus() {
    let zero = tempdir().unwrap();
    init_atelier(zero.path());
    let (success, zero_out, stderr) = run_atelier(zero.path(), &["status"]);
    assert!(success, "zero mission status failed: {stderr}");
    assert!(zero_out.contains("Current missions: 0"), "{zero_out}");
    assert!(!zero_out.contains("Active mission:"), "{zero_out}");

    let one = tempdir().unwrap();
    init_atelier(one.path());
    create_mission_fixture(one.path(), "One current objective");
    let (success, one_out, stderr) = run_atelier(one.path(), &["status"]);
    assert!(success, "one mission status failed: {stderr}");
    assert!(one_out.contains("Current missions: 1"), "{one_out}");
    assert!(!one_out.contains("Active mission:"), "{one_out}");

    let many = tempdir().unwrap();
    init_atelier(many.path());
    create_mission_fixture(many.path(), "First current objective");
    create_mission_fixture(many.path(), "Second current objective");
    let (success, many_out, stderr) = run_atelier(many.path(), &["status"]);
    assert!(success, "many mission status failed: {stderr}");
    assert!(many_out.contains("Current missions: 2"), "{many_out}");
    assert!(!many_out.contains("Active mission:"), "{many_out}");
}

#[test]
fn test_mission_closeout_enforces_gates() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Strict closeout",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("mission objective atelier-"));
    let mission_id = issue_id_by_title(dir.path(), "Strict closeout");
    move_mission_to_ready(dir.path(), &mission_id);

    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Closeout work"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Closeout work");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "link", &mission_id, &work_id]);
    assert!(success, "mission add work failed: {stderr}");

    let (success, closeout_blocked_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &mission_id,
            "close",
            "--reason",
            "done",
        ],
    );
    assert!(!success, "mission close should fail with open work");
    assert!(closeout_blocked_out.contains("Issue Transition"));
    assert!(closeout_blocked_out.contains("objective.work_terminal"));
    assert!(closeout_blocked_out.contains("validator objective.work_terminal failed"));
    assert!(stderr.contains("Transition 'close' is blocked"));

    close_issue_with_evidence(dir.path(), &work_id, Some("done"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", &mission_id, "--status", "closed"],
    );
    assert!(
        !success,
        "mission update --status closed should not be the ordinary closeout path"
    );
    assert!(stderr.contains("issue status changes use `atelier issue transition"));

    attach_pass_evidence(dir.path(), "mission", &mission_id, "strict mission proof");
    commit_all(dir.path(), "ready strict mission closeout");
    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &mission_id,
            "close",
            "--reason",
            "ready to close",
        ],
    );
    assert!(
        success,
        "mission close should succeed after gates pass: {stderr}"
    );
    assert!(close_out.contains("Applied transition close"));
    assert!(close_out.contains("To:       closed"));
}

#[test]
fn test_dirty_worktree_blocks_mission_closeout() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Dirty closeout",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("mission objective atelier-"));
    let mission_id = issue_id_by_title(dir.path(), "Dirty closeout");
    move_mission_to_ready(dir.path(), &mission_id);
    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Dirty terminal work"]);
    assert!(success, "work create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Dirty terminal work");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "link", &mission_id, &work_id]);
    assert!(success, "mission add work failed: {stderr}");
    close_issue_with_evidence(dir.path(), &work_id, Some("done"));
    attach_pass_evidence(
        dir.path(),
        "mission",
        &mission_id,
        "dirty closeout mission proof",
    );
    commit_all(dir.path(), "ready dirty mission closeout");
    std::fs::write(dir.path().join("untracked-closeout.txt"), "dirty").unwrap();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &mission_id,
            "close",
            "--reason",
            "done",
        ],
    );
    assert!(!success, "dirty worktree must block mission closeout");
    assert!(stdout.contains("Issue Transition"));
    assert!(stdout.contains("validator git.worktree_clean failed"));
    assert!(
        stdout.contains("git checkout has") || stdout.contains("Dirty state: dirty"),
        "{stdout}"
    );
    assert!(stdout.contains("untracked-closeout.txt"));
    assert!(stderr.contains("Transition 'close' is blocked"));
}

#[test]
fn test_off_base_branch_blocks_mission_closeout() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Off base closeout",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("mission objective atelier-"));
    let mission_id = issue_id_by_title(dir.path(), "Off base closeout");
    move_mission_to_ready(dir.path(), &mission_id);

    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Off base terminal work"]);
    assert!(success, "work create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Off base terminal work");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "link", &mission_id, &work_id]);
    assert!(success, "mission add work failed: {stderr}");
    close_issue_with_evidence(dir.path(), &work_id, Some("done"));
    attach_pass_evidence(dir.path(), "mission", &mission_id, "off-base mission proof");
    commit_all(dir.path(), "ready off-base mission closeout");

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "-c", "side-closeout"])
        .status()
        .unwrap();
    assert!(status.success(), "git switch -c side-closeout failed");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &mission_id,
            "close",
            "--reason",
            "done",
        ],
    );
    assert!(!success, "mission close must require the base branch");
    assert!(stdout.contains("Issue Transition"), "{stdout}");
    assert!(stdout.contains("validator git.on_base failed"), "{stdout}");
    assert!(
        stdout.contains("current branch is side-closeout; expected configured base branch main"),
        "{stdout}"
    );
    assert!(stderr.contains("Transition 'close' is blocked"), "{stderr}");
    assert_eq!(git_current_branch(dir.path()), "side-closeout");
}

#[test]
fn test_mission_close_still_blocks_hand_edited_issue_markdown() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Dirty canonical tracker closeout",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("mission objective atelier-"));
    let mission_id = issue_id_by_title(dir.path(), "Dirty canonical tracker closeout");
    move_mission_to_ready(dir.path(), &mission_id);

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Hand edited canonical work"],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Hand edited canonical work");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "link", &mission_id, &issue_id]);
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
            "The issue outcome is complete and ready for terminal checks.",
            "The issue outcome was hand-edited after closeout.",
        )
    });

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &mission_id,
            "close",
            "--reason",
            "done",
        ],
    );
    assert!(
        !success,
        "hand-edited canonical issue markdown must block closeout"
    );
    assert!(stdout.contains("Issue Transition"));
    assert!(stdout.contains("validator git.worktree_clean failed"));
    assert!(
        stdout.contains("git checkout has") || stdout.contains("Dirty state: dirty"),
        "{stdout}"
    );
    assert!(stdout.contains(&format!(".atelier/issues/{issue_id}.md")));
    assert!(stderr.contains("Transition 'close' is blocked"));
}

#[test]
fn test_mission_start_is_removed_without_compatibility_guidance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "start", "atelier-missing"]);
    assert!(!success, "mission start should be removed");
    assert!(stderr.contains("unrecognized subcommand"), "{stderr}");
    assert!(!stderr.contains("--switch"), "{stderr}");
}

#[test]
fn test_first_class_record_rebuild_rejects_schema_drift() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Guard schema", "--issue-type", "mission"],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("mission objective atelier-"));
    let mission_id = issue_id_by_title(dir.path(), "Guard schema");
    edit_canonical_record(dir.path(), "issues", &mission_id, |markdown| {
        markdown.replace("schema: \"atelier.issue\"", "schema: \"atelier.evidence\"")
    });
    remove_projection_state(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(!success, "rebuild should reject issue schema drift");
    assert!(
        stderr.contains("Unsupported schema 'atelier.evidence'")
            && stderr.contains("expected atelier.issue"),
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

    let (success, _, stderr) = run_atelier(schema_dir.path(), &["work", "queue"]);
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

    let (success, _, stderr) = run_atelier(malformed_dir.path(), &["work", "queue"]);
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
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, list_out, stderr) =
        run_atelier(dir.path(), &["work", "queue", "--status", "all"]);
    assert!(success, "fresh list failed: {stderr}");
    assert!(list_out.contains("Indexed title"));

    edit_canonical_issue(dir.path(), &issue_id, |markdown| {
        markdown.replace("Indexed title", "Markdown title")
    });

    let (success, list_out, stderr) =
        run_atelier(dir.path(), &["work", "queue", "--status", "all"]);
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
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

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
        run_atelier(dir.path(), &["work", "queue", "--status", "all"]);
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
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let first_path = canonical_issue_path(dir.path(), &first_id);
    let first_markdown = read_canonical_record(dir.path(), "issues", &first_id);
    std::fs::remove_file(&first_path).unwrap();

    let (success, list_out, stderr) =
        run_atelier(dir.path(), &["work", "queue", "--status", "all"]);
    assert!(
        success,
        "deleted source list should transparently rebuild: {stderr}"
    );
    assert!(!list_out.contains("First indexed issue"));
    assert!(list_out.contains("Second indexed issue"));
    assert!(
        stderr
            .contains("Projection index was stale; repaired local SQLite projection incrementally")
            || stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "missing automatic repair diagnostic: {stderr}"
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

- `atelier issue show atelier-zzzz` shows the record.
"#,
    )
    .unwrap();

    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", "atelier-zzzz"]);
    assert!(
        success,
        "unindexed issue show should transparently rebuild: {stderr}"
    );
    assert!(show_out.contains("Unindexed issue"));
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
    let second_body = "## Description\n\nProjection leaf body.\n\n## Outcome\n\nProjection leaf remains linked after rebuild.\n\n## Evidence\n\n- manual check: `atelier issue show <id>` output shows the linked root.";

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
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &second_id,
            &first_id,
            "--role",
            "blocked_by",
        ],
    );
    assert!(success, "issue link failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");
    ensure_issue_completion_sections(dir.path(), &first_id);
    ensure_issue_completion_sections(dir.path(), &second_id);

    std::fs::write(dir.path().join(".atelier/manifest.json"), "{}\n").unwrap();
    std::fs::write(dir.path().join(".atelier/graph.json"), "{}\n").unwrap();
    let (success, ready_out, stderr) =
        run_atelier(dir.path(), &["work", "queue", "--status", "all"]);
    assert!(
        success,
        "derived files should not stale work queue --ready: {stderr}"
    );
    assert!(ready_out.contains("Projection root"));

    edit_canonical_issue(dir.path(), &first_id, |markdown| {
        markdown.replace("Projection root", "Projection root changed")
    });

    let (success, dep_out, stderr) = run_atelier(dir.path(), &["issue", "show", &second_id]);
    assert!(
        success,
        "stale issue show should transparently rebuild: {stderr}"
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
fn test_rebuild_temp_files_are_ignored_by_query_lint_and_doctor() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let body = "## Description\n\nTemp rebuild filter body.\n\n## Outcome\n\nQuery, lint, and doctor ignore rebuild temp files.\n\n## Evidence\n\n- manual check: `atelier lint` output prints `Lint passed.`, `atelier doctor` exits 0, and `atelier doctor --fix` exits 0 while rebuild temp files exist.";
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
    let (success, _, stderr) = run_atelier(dir.path(), &["doctor", "--fix"]);
    assert!(success, "doctor --fix failed: {stderr}");
    ensure_issue_completion_sections(dir.path(), &issue_id);
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

    let commands: &[&[&str]] = &[&["lint"], &["doctor"], &["doctor", "--fix"]];
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
        "UPDATE projection_sources
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

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
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
    assert!(stdout.contains(&format!("{issue_id}")));
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
        &["workflow_issue_type_unknown", "issue_type 'bogus'"],
    );
    assert_lint_rejects_issue_edit(
        "Invalid priority fixture",
        |markdown, _issue_id| markdown.replace("priority: \"P2\"", "priority: \"urgent\""),
        &[
            "Invalid priority",
            "Invalid priority 'urgent'. Valid values: critical, high, medium, low",
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
            let (success, _evidence_out, stderr) = run_atelier(
                dir,
                &[
                    "evidence",
                    "record",
                    "--kind",
                    "test",
                    "Duplicate ID evidence",
                ],
            );
            assert!(success, "evidence create failed: {stderr}");
            let evidence_id = record_id_by_title(dir, "evidence", "Duplicate ID evidence");
            let old_path = dir
                .join(".atelier/evidence")
                .join(format!("{evidence_id}.md"));
            let new_path = dir.join(".atelier/evidence").join(format!("{issue_id}.md"));
            let evidence_markdown = std::fs::read_to_string(&old_path).unwrap().replace(
                &format!("id: \"{evidence_id}\""),
                &format!("id: \"{issue_id}\""),
            );
            std::fs::write(&new_path, evidence_markdown).unwrap();
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
fn test_bundle_apply_records_links_export_and_rebuild() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bundle_path = dir.path().join("bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 1,
  "title": "Bundle apply smoke",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.blocker",
        "title": "Complete prerequisite",
        "issue_type": "task",
        "priority": "medium",
        "status": "done",
        "labels": ["bundle"]
      },
      {
        "client_ref": "issue.work",
        "title": "Implement bundle output",
        "issue_type": "feature",
        "priority": "high",
        "status": "in_progress",
        "depends_on": [{ "client_ref": "issue.blocker" }],
        "outcome": ["summary maps client refs"],
        "evidence": ["export check passes"]
      },
      {
        "client_ref": "mission.bundle",
        "title": "Bundle mission",
        "issue_type": "mission",
        "priority": "medium",
        "labels": ["bundle", "mission"],
        "advances": [{ "client_ref": "issue.work" }],
        "description": "Mission from bundle"
      }
    ],
    "evidence": [
      {
        "client_ref": "evidence.bundle",
        "title": "Bundle evidence",
        "evidence_type": "test",
        "result": "pass",
        "body": "The apply smoke test passed.",
        "validates": [{ "client_ref": "mission.bundle" }]
      }
    ]
  }
}"#,
    )
    .unwrap();
    let bundle_arg = bundle_path.to_str().unwrap();

    let (success, dry_run_out, stderr) =
        run_atelier(dir.path(), &["bundle", "preview", bundle_arg]);
    assert!(success, "bundle preview failed: {stderr}");
    assert!(dry_run_out.contains("Bundle preview is valid."));
    assert!(dry_run_out.contains("Applied:       false"));
    assert!(dry_run_out.contains("Preview:       true"));
    assert!(dry_run_out.contains("issues: 3"), "{dry_run_out}");

    let (success, apply_out, stderr) =
        run_atelier(dir.path(), &["bundle", "apply", bundle_arg, "--yes"]);
    assert!(success, "bundle apply failed: {stderr}");
    assert!(apply_out.contains("Bundle applied."));
    assert!(apply_out.contains("Applied:       true"));
    assert!(apply_out.contains("atelier issue show"));
    let mission_id = issue_id_by_title(dir.path(), "Bundle mission");

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check after bundle apply failed: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild after bundle apply failed: {stderr}");

    let (success, view_out, stderr) = run_atelier(dir.path(), &["issue", "show", &mission_id]);
    assert!(success, "mission show after bundle apply failed: {stderr}");
    assert!(view_out.contains("Type:     mission"));
    let mission_markdown = std::fs::read_to_string(
        dir.path()
            .join(".atelier/issues")
            .join(format!("{mission_id}.md")),
    )
    .unwrap();
    assert!(mission_markdown.contains("issue_type: \"mission\""));
    assert!(mission_markdown.contains("- \"bundle\"\n"));
    assert!(mission_markdown.contains("- \"mission\"\n"));
}

#[test]
fn test_bundle_rejects_removed_mission_resource_shape() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bundle_path = dir.path().join("removed-mission-bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 1,
  "title": "Removed mission shape",
  "resources": {
    "missions": [
      {
        "client_ref": "mission.removed",
        "title": "Removed mission",
        "body": "Old mission resource"
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

    assert!(!success, "removed mission resource should be rejected");
    assert!(
        stderr.contains("resources.missions is no longer supported"),
        "{stderr}"
    );
    assert!(
        stderr.contains("issue_type \"mission\"") && stderr.contains("advances"),
        "{stderr}"
    );
}

#[test]
fn test_bundle_rejects_mission_parent_scope() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bundle_path = dir.path().join("mission-parent-bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 1,
  "title": "Invalid mission parent",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.parent",
        "title": "Parent epic",
        "issue_type": "epic"
      },
      {
        "client_ref": "mission.child",
        "title": "Invalid mission child",
        "issue_type": "mission",
        "parent": { "client_ref": "issue.parent" }
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

    assert!(!success, "mission parent should be rejected");
    assert!(
        stderr.contains("Mission issue mission.child cannot have parent"),
        "{stderr}"
    );
}

#[test]
fn test_bundle_apply_mid_apply_failure_leaves_canonical_files_unchanged() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let before_issues = count_markdown_records(dir.path(), "issues");
    let before_evidence = count_markdown_records(dir.path(), "evidence");
    let bundle_path = dir.path().join("invalid-parent-bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 1,
  "title": "Mid apply failure",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.invalid-parent",
        "title": "Should not persist",
        "issue_type": "task",
        "priority": "high",
        "parent": { "client_ref": "evidence.invalid-parent" }
      }
    ],
    "evidence": [
      {
        "client_ref": "evidence.invalid-parent",
        "title": "Should not persist evidence",
        "evidence_type": "test",
        "result": "pass",
        "body": "This staged record must not install."
      }
    ]
  }
}"#,
    )
    .unwrap();

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["bundle", "apply", bundle_path.to_str().unwrap(), "--yes"],
    );

    assert!(!success, "invalid bundle should fail");
    assert!(
        stderr.contains("Issue parent for issue.invalid-parent must resolve to an issue"),
        "{stderr}"
    );
    assert_eq!(count_markdown_records(dir.path(), "issues"), before_issues);
    assert_eq!(
        count_markdown_records(dir.path(), "evidence"),
        before_evidence
    );
    assert!(
        !canonical_directory_contains(dir.path(), "issues", "Should not persist"),
        "staged issue leaked into canonical files"
    );
    assert!(
        !canonical_directory_contains(dir.path(), "evidence", "Should not persist evidence"),
        "staged evidence leaked into canonical files"
    );
}

#[test]
fn test_work_commands_are_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for args in [
        vec!["work", "start", "atelier-z1p8"],
        vec!["work", "status"],
    ] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(!success, "{args:?} unexpectedly succeeded");
        let transcript = format!("{stdout}\n{stderr}");
        assert!(
            transcript.contains("unrecognized subcommand") && transcript.contains("Usage: atelier"),
            "missing removed-command transcript for {args:?}: {transcript}"
        );
    }
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

    let (success, child_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &child_id, "start"]);
    assert!(success, "child start failed: {stderr}");
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    assert!(child_out.contains(&format!("Started work on {child_id} Child work")));
    assert!(child_out.contains(&format!("Branch owner: epic {epic_id} (epic)")));
    assert!(child_out.contains(&format!("Source branch: epic/{epic_id}")));
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
    let (success, standalone_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &standalone_id, "start"],
    );
    assert!(success, "standalone start failed: {stderr}");
    assert_eq!(
        git_current_branch(dir.path()),
        format!("task/{standalone_id}")
    );
    assert!(standalone_out.contains(&format!("Branch owner: issue {standalone_id} (task)")));
    assert!(standalone_out.contains(&format!("Source branch: task/{standalone_id}")));
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
    let (success, epic_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &epic_id, "start"]);
    assert!(success, "epic start failed: {stderr}");
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    assert!(epic_out.contains(&format!("Branch owner: epic {epic_id} (epic)")));
    let (success, epic_show, stderr) = run_atelier(dir.path(), &["issue", "show", &epic_id]);
    assert!(success, "epic show failed: {stderr}");
    assert!(epic_show.contains("Status:   in_progress"), "{epic_show}");
}

#[test]
fn test_mission_start_prepares_mission_branch_from_base() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    write_mission_branch_workflow(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Integration mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Integration mission");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "ready"]);
    assert!(success, "mission ready failed: {stderr}");
    commit_all(dir.path(), "mission branch baseline");

    let (success, start_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(success, "mission start failed: {stderr}");

    assert_eq!(
        git_current_branch(dir.path()),
        format!("mission/{mission_id}")
    );
    assert!(
        start_out.contains("Action:   git.prepare_branch"),
        "{start_out}"
    );
    assert!(
        start_out.contains(&format!("created branch mission/{mission_id} from main")),
        "{start_out}"
    );
    assert!(
        start_out.contains(&format!("Branch owner: mission {mission_id} (mission)")),
        "{start_out}"
    );
    assert!(
        start_out.contains(&format!("Source branch: mission/{mission_id}")),
        "{start_out}"
    );
    assert!(start_out.contains("Base branch: main"), "{start_out}");
    assert!(start_out.contains("Target branch: main"), "{start_out}");
}

#[test]
fn test_epic_start_from_mission_branch_uses_current_branch_base() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    write_mission_branch_workflow(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Scoped mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Scoped mission");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Scoped epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Scoped epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &mission_id, &epic_id, "--role", "advances"],
    );
    assert!(success, "mission link failed: {stderr}");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "ready"]);
    assert!(success, "mission ready failed: {stderr}");
    commit_all(dir.path(), "mission scoped baseline");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(success, "mission start failed: {stderr}");
    commit_all(dir.path(), "mission branch started");

    let (success, start_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &epic_id, "start"]);
    assert!(success, "epic start from mission branch failed: {stderr}");

    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    assert!(
        start_out.contains("Action:   git.prepare_branch"),
        "{start_out}"
    );
    assert!(
        start_out.contains(&format!(
            "created branch epic/{epic_id} from mission/{mission_id}"
        )),
        "{start_out}"
    );
    assert!(
        start_out.contains(&format!("Source branch: epic/{epic_id}")),
        "{start_out}"
    );
    assert!(
        start_out.contains(&format!("Base branch: mission/{mission_id}")),
        "{start_out}"
    );
    assert!(
        start_out.contains(&format!("Target branch: mission/{mission_id}")),
        "{start_out}"
    );
    let front_matter = canonical_record_front_matter(dir.path(), "issues", &epic_id);
    let workflow_branch = &front_matter["fields"]["workflow_branch"];
    assert_eq!(workflow_branch["owner_issue_id"], epic_id);
    assert_eq!(workflow_branch["work_branch"], format!("epic/{epic_id}"));
    assert_eq!(
        workflow_branch["branch_base"],
        format!("mission/{mission_id}")
    );
    assert_eq!(
        workflow_branch["review_target"],
        format!("mission/{mission_id}")
    );
    assert_eq!(
        workflow_branch["integration_target"],
        format!("mission/{mission_id}")
    );
    assert_eq!(workflow_branch["owner_kind"], "epic");
    assert_eq!(workflow_branch["merge_strategy"], "squash");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild after epic start failed: {stderr}");
    let (success, options_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &epic_id, "--verbose"]);
    assert!(success, "transition options after rebuild failed: {stderr}");
    assert!(
        options_out.contains(&format!("Source:   epic/{epic_id}"))
            && options_out.contains(&format!("Base:     mission/{mission_id}"))
            && options_out.contains(&format!("Target:   mission/{mission_id}")),
        "{options_out}"
    );
}

#[test]
fn test_epic_close_integrates_into_recorded_mission_branch() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    write_mission_branch_workflow(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Close target mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Close target mission");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Close target epic",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Close target epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &mission_id, &epic_id, "--role", "advances"],
    );
    assert!(success, "mission link failed: {stderr}");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "ready"]);
    assert!(success, "mission ready failed: {stderr}");
    commit_all(dir.path(), "mission close target baseline");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(success, "mission start failed: {stderr}");
    commit_all(dir.path(), "mission branch ready for epic close");
    let mission_head_before_epic = git_rev_parse(dir.path(), &format!("mission/{mission_id}"));
    let main_head_before_close = git_rev_parse(dir.path(), "main");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "transition", &epic_id, "start"]);
    assert!(success, "epic start failed: {stderr}");
    std::fs::write(
        dir.path().join("mission-close-target.txt"),
        "epic work for mission branch\n",
    )
    .unwrap();
    commit_all(dir.path(), "epic work for mission close target");
    move_issue_to_validation(dir.path(), &epic_id);
    ensure_all_issue_completion_sections(dir.path());
    attach_issue_pass_evidence(dir.path(), &epic_id);
    commit_all(dir.path(), "epic proof ready for mission close target");

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &epic_id, "close", "--reason", "done"],
    );
    assert!(success, "epic close failed: {stderr}");
    assert_eq!(
        git_current_branch(dir.path()),
        format!("mission/{mission_id}")
    );
    assert!(
        close_out.contains("Action:   tracker.commit")
            && close_out.contains("Action:   branch_integrate squash commit"),
        "{close_out}"
    );
    assert_ne!(
        git_rev_parse(dir.path(), &format!("mission/{mission_id}")),
        mission_head_before_epic
    );
    assert_eq!(git_rev_parse(dir.path(), "main"), main_head_before_close);
    let mission_log = git_log_oneline(dir.path(), &format!("mission/{mission_id}"), 2);
    assert!(
        mission_log.contains(&format!(
            "Squash merge epic/{epic_id} into mission/{mission_id}"
        )),
        "{mission_log}"
    );
    assert!(git_status_short(dir.path()).trim().is_empty());
}

#[test]
fn test_epic_start_rejects_wrong_branch_for_mission_scope() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    write_mission_branch_workflow(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Wrong branch mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Wrong branch mission");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Wrong branch epic",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Wrong branch epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &mission_id, &epic_id, "--role", "advances"],
    );
    assert!(success, "mission link failed: {stderr}");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "ready"]);
    assert!(success, "mission ready failed: {stderr}");
    commit_all(dir.path(), "wrong branch baseline");

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "-c", "side-start"])
        .status()
        .unwrap();
    assert!(status.success(), "git switch -c side-start failed");
    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &epic_id, "start"]);

    assert!(!success, "epic start should reject non-mission branch");
    let output = format!("{stdout}\n{stderr}");
    assert!(output.contains("git.on_mission_branch"), "{output}");
    assert!(
        output.contains(&format!(
            "current branch is side-start; expected mission/<id> for issue {epic_id}"
        )),
        "{output}"
    );
    assert_eq!(git_current_branch(dir.path()), "side-start");
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &epic_id]);
    assert!(success, "epic show failed: {stderr}");
    assert!(show_out.contains("Status:   todo"), "{show_out}");
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

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(!success, "dirty start unexpectedly succeeded:\n{stdout}");
    assert!(
        stderr.contains("checkout has uncommitted")
            && stderr.contains("non-tracker changes")
            && stderr.contains("dirty.txt"),
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
    let expected_branch = format!("task/{issue_id}");
    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["worktree", "add", "-b", &expected_branch])
        .arg(&other_worktree)
        .arg("main")
        .status()
        .unwrap();
    assert!(status.success(), "git worktree add failed");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(
        !success,
        "checkout-failure start unexpectedly succeeded:\n{stdout}"
    );
    assert!(
        stderr.contains("action git.prepare_branch failed while switching")
            && stderr.contains("retry `atelier issue transition")
            && stderr.contains("start`"),
        "{stderr}"
    );
    assert_eq!(git_current_branch(dir.path()), "main");
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(show_out.contains("Status:   todo"), "{show_out}");
}

#[test]
fn test_branch_actions_prepare_and_integrate_epic_workflow() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    write_branch_action_workflow(dir.path());
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
            "Action integrated epic",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Action integrated epic");
    commit_all(dir.path(), "initial tracker state");

    let (success, start_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &epic_id, "start"]);
    assert!(success, "epic action start failed: {stderr}");
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    assert!(
        start_out.contains("Action:   git.prepare_branch"),
        "{start_out}"
    );

    std::fs::write(dir.path().join("epic-action.txt"), "epic action work\n").unwrap();
    commit_all(dir.path(), "epic action implementation");
    move_issue_to_validation(dir.path(), &epic_id);
    ensure_all_issue_completion_sections(dir.path());
    attach_issue_pass_evidence(dir.path(), &epic_id);
    commit_all(dir.path(), "epic action proof ready");

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &epic_id, "close", "--reason", "done"],
    );
    assert!(success, "epic action close failed: {stderr}");
    assert_eq!(git_current_branch(dir.path()), "main");
    assert!(
        close_out.contains("Action:   tracker.commit"),
        "{close_out}"
    );
    assert!(
        close_out.contains("Action:   branch_integrate squash commit"),
        "{close_out}"
    );
    let main_log = git_log_oneline(dir.path(), "main", 2);
    assert!(
        main_log.contains(&format!("Squash merge epic/{epic_id} into main")),
        "{main_log}"
    );
    assert!(git_status_short(dir.path()).trim().is_empty());
}

#[test]
fn test_child_branch_prepare_action_checks_out_parent_epic_branch() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    write_branch_action_workflow(dir.path());
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
            "Parent action epic",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Parent action epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Child action checkout",
            "--parent",
            &epic_id,
        ],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Child action checkout");
    commit_all(dir.path(), "initial tracker state");

    let (success, start_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &child_id, "start"]);
    assert!(success, "child action start failed: {stderr}");
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    assert!(
        start_out.contains("Action:   git.prepare_branch"),
        "{start_out}"
    );
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &child_id]);
    assert!(success, "child show failed: {stderr}");
    assert!(show_out.contains("Status:   in_progress"), "{show_out}");
}

#[test]
fn test_epic_start_requires_base_branch() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Base gated epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Base gated epic");
    commit_all(dir.path(), "initial epic tracker state");

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "-c", "side-start"])
        .status()
        .unwrap();
    assert!(status.success(), "git switch -c side-start failed");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &epic_id, "start"]);
    assert!(!success, "epic start must require the base branch");
    let output = format!("{stdout}\n{stderr}");
    assert!(output.contains("git.on_base"), "{output}");
    assert!(
        output.contains("current branch is side-start; expected configured base branch main"),
        "{output}"
    );
    assert_eq!(git_current_branch(dir.path()), "side-start");

    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &epic_id]);
    assert!(success, "epic show failed: {stderr}");
    assert!(show_out.contains("Status:   todo"), "{show_out}");
}

#[test]
fn test_branch_integrate_action_failure_rolls_back_status_with_recovery() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    write_branch_action_workflow(dir.path());
    Command::new("git")
        .current_dir(dir.path())
        .args(["branch", "-M", "main"])
        .status()
        .unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Action conflict"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Action conflict");
    commit_all(dir.path(), "initial tracker state");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "action start failed: {stderr}");
    std::fs::write(dir.path().join("action-conflict.txt"), "issue branch\n").unwrap();
    commit_all(dir.path(), "issue branch conflict content");
    ensure_all_issue_completion_sections(dir.path());
    attach_issue_pass_evidence(dir.path(), &issue_id);
    commit_all(dir.path(), "issue action proof ready before conflict close");

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "main"])
        .status()
        .unwrap();
    assert!(status.success(), "switch to main failed");
    std::fs::write(dir.path().join("action-conflict.txt"), "main branch\n").unwrap();
    commit_all(dir.path(), "main branch conflict content");
    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", &format!("task/{issue_id}")])
        .status()
        .unwrap();
    assert!(status.success(), "switch back to issue branch failed");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &issue_id,
            "close",
            "--reason",
            "done",
        ],
    );
    assert!(
        !success,
        "conflicting action close unexpectedly succeeded:\n{stdout}"
    );
    assert!(
        stderr.contains("action branch_integrate failed during squash merge")
            && stderr.contains("Recovery:")
            && stderr.contains(&format!("transition for {issue_id}")),
        "{stderr}"
    );
    assert_eq!(git_current_branch(dir.path()), format!("task/{issue_id}"));
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "issue show failed after action rollback: {stderr}");
    assert!(show_out.contains("Status:   in_progress"), "{show_out}");
    assert!(!show_out.contains("Status:   done"), "{show_out}");
    assert!(git_status_short(dir.path()).trim().is_empty());
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

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &child_id, "start"]);
    assert!(success, "child start failed: {stderr}");
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    ensure_all_issue_completion_sections(dir.path());
    attach_issue_pass_evidence(dir.path(), &child_id);

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &child_id,
            "close",
            "--reason",
            "done",
        ],
    );
    assert!(success, "child transition close failed: {stderr}");
    assert!(
        close_out.contains("Action:   tracker.commit"),
        "{close_out}"
    );
    assert!(
        close_out.contains("Action:   branch_integrate deferred to parent branch close"),
        "{close_out}"
    );
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    assert_eq!(git_rev_parse(dir.path(), "main"), main_before);
    assert!(git_log_oneline(dir.path(), &format!("epic/{epic_id}"), 1)
        .contains(&format!("Transition {child_id} close: Child close work")));
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

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "start failed: {stderr}");
    std::fs::write(dir.path().join("standalone.txt"), "standalone work\n").unwrap();
    commit_all(dir.path(), "standalone implementation");
    ensure_all_issue_completion_sections(dir.path());
    attach_issue_pass_evidence(dir.path(), &issue_id);

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &issue_id,
            "close",
            "--reason",
            "done",
        ],
    );
    assert!(success, "standalone close failed: {stderr}");
    assert_eq!(git_current_branch(dir.path()), "main");
    assert!(
        close_out.contains("Action:   tracker.commit"),
        "{close_out}"
    );
    assert!(
        close_out.contains("Action:   branch_integrate squash commit"),
        "{close_out}"
    );
    let main_log = git_log_oneline(dir.path(), "main", 2);
    assert!(
        main_log.contains(&format!("Squash merge task/{issue_id} into main")),
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
fn test_epic_branch_commands_use_current_checkout() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    migrate_default_issue_workflow(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Branch mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Branch mission");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Branch epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Branch epic");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "link", &mission_id, &epic_id]);
    assert!(success, "mission add epic failed: {stderr}");
    commit_all(dir.path(), "epic branch baseline");

    let (success, branch_out, stderr) = run_atelier(dir.path(), &["branch", "for-epic", &epic_id]);
    assert!(success, "branch for-epic failed: {stderr}");
    assert!(branch_out.contains(&format!("Switched to epic/{epic_id}")));
    assert!(branch_out.contains(&format!("Mission: {mission_id}")));
    assert!(branch_out.contains(&format!("Checkout: {}", dir.path().display())));

    let (success, branch_status_out, stderr) = run_atelier(dir.path(), &["branch", "status"]);
    assert!(success, "branch status failed: {stderr}");
    assert!(branch_status_out.contains("Epic Branch Status"));
    assert!(branch_status_out.contains(&format!("Checkout: {}", dir.path().display())));
    assert!(branch_status_out.contains(&format!("epic/{epic_id} - Branch epic")));
    assert!(branch_status_out.contains(&format!("mission {mission_id}")));

    std::fs::write(dir.path().join("branch-proof.txt"), "epic branch work\n").unwrap();
    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["add", "branch-proof.txt"])
        .status()
        .unwrap();
    assert!(status.success(), "git add failed");
    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["commit", "-q", "-m", "epic branch work"])
        .status()
        .unwrap();
    assert!(status.success(), "git commit failed");
    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "main"])
        .status()
        .unwrap();
    assert!(status.success(), "git switch back to main failed");

    let (success, merge_out, stderr) = run_atelier(dir.path(), &["branch", "merge", &epic_id]);
    assert!(success, "branch merge failed: {stderr}");
    assert!(merge_out.contains(&format!("Merged epic/{epic_id}")));
    assert!(merge_out.contains(&format!("Mission: {mission_id}")));
    assert!(merge_out.contains(&format!("Checkout: {}", dir.path().display())));
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

    let (start_success, startstdout, start_stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
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

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Typed issue"));
    assert!(stdout.contains("Category: todo"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["work", "queue", "--status", "all"]);
    assert!(success, "list failed: {stderr}");
    assert!(stdout.contains("validation"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["work", "queue", "--ready"]);
    assert!(success, "ready failed: {stderr}");
    assert!(stdout.contains("validation"));

    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");
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

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "atelier-0003"]);
    assert!(success, "mapped show failed: {stderr}");
    assert!(stdout.contains("atelier-0003"));
    assert!(stdout.contains("[task]"));
    assert!(stdout.contains("Parent: atelier-0001"));
    assert!(stdout.contains("atelier-0002"));
    assert!(!stdout.contains("beads:"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "atelier-0003"]);
    assert!(success, "mapped issue show failed: {stderr}");
    assert!(stdout.contains("atelier-0003"));
    assert!(stdout.contains("atelier-0002"));
}

fn count_markdown_records(dir: &std::path::Path, directory: &str) -> usize {
    let record_dir = dir.join(".atelier").join(directory);
    std::fs::read_dir(&record_dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", record_dir.display()))
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("md"))
        .count()
}

fn canonical_directory_contains(dir: &std::path::Path, directory: &str, needle: &str) -> bool {
    let record_dir = dir.join(".atelier").join(directory);
    std::fs::read_dir(&record_dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", record_dir.display()))
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("md"))
        .any(|entry| {
            std::fs::read_to_string(entry.path())
                .map(|text| text.contains(needle))
                .unwrap_or(false)
        })
}
