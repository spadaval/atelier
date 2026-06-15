use super::support::*;

#[test]
fn test_issue_next_uses_current_workflow_commands() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(dir.path(), &["issue", "create", "Next item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Next item");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "next"]);
    assert!(success, "issue next failed: {stderr}");
    assert!(stdout.contains("Next Actions"));
    assert!(stdout.contains(&format!("atelier issue show {issue_id}")));
    assert!(stdout.contains(&format!("atelier start {issue_id}")));
    assert!(stdout.contains("atelier status"));
    assert!(
        !stdout.contains("session work"),
        "issue next must not suggest removed session workflow:\n{stdout}"
    );
}

#[test]
fn test_issue_transition_options_and_successful_execution_follow_workflow_policy() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Root workflow item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Root workflow item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "tracker setup");
    let git_before = git_status_short(dir.path());

    let (success, transition_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(success, "transition failed: {stderr}");
    assert!(transition_out.contains("Issue Transitions"));
    assert!(
        transition_out.contains("start [allowed]"),
        "{transition_out}"
    );
    assert!(
        transition_out.contains("block [allowed]"),
        "{transition_out}"
    );
    assert!(transition_out.contains(&format!("atelier issue transition {issue_id} start")));
    let git_after = git_status_short(dir.path());
    assert_eq!(
        git_before, git_after,
        "--options should not dirty the worktree:\nbefore:\n{git_before}\nafter:\n{git_after}"
    );

    let (success, start_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "transition start failed: {stderr}");
    assert!(
        start_out.contains("Applied transition start"),
        "{start_out}"
    );
    assert!(start_out.contains("To:       in_progress"), "{start_out}");

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(
        issue_text.contains("status: \"in_progress\""),
        "{issue_text}"
    );

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_applied",
        &[
            "Applied transition start",
            "transition: \"start\"",
            "to: \"in_progress\"",
        ],
    );
}

#[test]
fn test_issue_transition_options_do_not_write_but_blocked_transitions_do() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Options read-only item",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Options read-only item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "options read-only baseline");

    let git_before = git_status_short(dir.path());
    let (success, options_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(options_out.contains("Issue Transitions"), "{options_out}");
    let git_after = git_status_short(dir.path());
    assert_eq!(
        git_before, git_after,
        "--options should leave the git worktree unchanged:\nbefore:\n{git_before}\nafter:\n{git_after}"
    );

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "start failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_validation"],
    );
    assert!(
        !success,
        "request_validation should fail without a completed review"
    );
    assert!(stdout.contains("Blockers"), "{stdout}");
    assert!(stderr.contains("review_ready"), "{stderr}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_blocked",
        &[
            "Blocked transition request_validation from in_progress",
            "transition: \"request_validation\"",
            "reason: \"validator review_ready failed:",
        ],
    );
}

#[test]
fn test_root_start_applies_workflow_transition_without_runtime_current_work() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Root start item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Root start item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "workflow-ready start item");

    let (success, start_out, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "root start failed: {stderr}");
    assert!(
        start_out.contains("Applied transition start"),
        "{start_out}"
    );
    assert!(start_out.contains("To:       in_progress"), "{start_out}");
    assert!(
        start_out.contains("Tracked work is now derived from issue workflow status."),
        "{start_out}"
    );

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(
        issue_text.contains("status: \"in_progress\""),
        "{issue_text}"
    );

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(status_out.contains("Current work:  1 issue"));
    assert!(status_out.contains(&format!("{issue_id} - Root start item [in_progress]")));
    assert_eq!(active_work_association_count(dir.path(), &issue_id), 0);

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_applied",
        &[
            "Applied transition start",
            "transition: \"start\"",
            "to: \"in_progress\"",
        ],
    );
    assert!(
        !activities
            .iter()
            .any(|text| text.contains("event_type: \"work_started\"")),
        "root start should not record runtime work association activity:\n{}",
        activities.join("\n--- activity ---\n")
    );
}

#[test]
fn test_status_preserves_current_work_after_runtime_database_rebuild() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Rebuild current work"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Rebuild current work");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "rebuild current work item");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "start failed: {stderr}");

    let (success, status_before, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status before rebuild failed: {stderr}");
    assert!(status_before.contains("Current work:  1 issue"));
    assert!(status_before.contains(&format!("{issue_id} - Rebuild current work [in_progress]")));

    remove_runtime_and_cache_dirs(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed after runtime deletion: {stderr}");
    assert!(dir.path().join(".atelier/runtime/state.db").exists());

    let (success, status_after, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status after rebuild failed: {stderr}");
    assert!(status_after.contains("Current work:  1 issue"));
    assert!(status_after.contains(&format!("{issue_id} - Rebuild current work [in_progress]")));
}

#[test]
fn test_root_start_allows_multiple_status_derived_current_work_items() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Active item"]);
    assert!(success, "active issue create failed: {stderr}");
    let active_id = issue_id_by_title(dir.path(), "Active item");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Next item"]);
    assert!(success, "next issue create failed: {stderr}");
    let next_id = issue_id_by_title(dir.path(), "Next item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "two startable items");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &active_id]);
    assert!(success, "initial start failed: {stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &next_id]);
    assert!(success, "second current-work issue should start: {stderr}");

    let next_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &next_id)).unwrap();
    assert!(
        next_text.contains("status: \"in_progress\""),
        "second start should transition the second issue:\n{next_text}"
    );
    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(status_out.contains("Current work:  2 issues"));
    assert!(status_out.contains(&format!("{active_id} - Active item [in_progress]")));
    assert!(status_out.contains(&format!("{next_id} - Next item [in_progress]")));
}

#[test]
fn test_root_start_same_issue_does_not_create_runtime_association() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Restarted item"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Restarted item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "restartable item");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "first start failed: {stderr}");
    assert_eq!(active_work_association_count(dir.path(), &issue_id), 0);
}

#[test]
fn test_removed_abandon_rejects_and_preserves_issue_status() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "First item"]);
    assert!(success, "first issue create failed: {stderr}");
    let first_id = issue_id_by_title(dir.path(), "First item");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Second item"]);
    assert!(success, "second issue create failed: {stderr}");
    let second_id = issue_id_by_title(dir.path(), "Second item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "switchable items");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &first_id]);
    assert!(success, "first start failed: {stderr}");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["abandon", &first_id, "--reason", "switching"]);
    assert!(!success, "removed abandon should reject");
    assert!(stderr.contains("`atelier abandon` was removed"), "{stderr}");

    let (success, second_out, stderr) = run_atelier(dir.path(), &["start", &second_id]);
    assert!(success, "second start should not need abandon: {stderr}");
    assert!(
        second_out.contains("Tracked work is now derived from issue workflow status."),
        "{second_out}"
    );
    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(status_out.contains("Current work:  2 issues"));
    assert!(status_out.contains(&format!("{first_id} - First item [in_progress]")));
    assert!(status_out.contains(&format!("{second_id} - Second item [in_progress]")));
}

#[test]
fn test_separate_worktrees_can_have_different_active_issues() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Parallel focus"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Parallel focus");
    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "start", mission_id.as_str()]);
    assert!(success, "mission start failed: {stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Root work"]);
    assert!(success, "root issue create failed: {stderr}");
    let root_id = issue_id_by_title(dir.path(), "Root work");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Worktree work"]);
    assert!(success, "worktree issue create failed: {stderr}");
    let worktree_id = issue_id_by_title(dir.path(), "Worktree work");
    for issue_id in [&root_id, &worktree_id] {
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["mission", "add-work", mission_id.as_str(), issue_id],
        );
        assert!(success, "mission add work failed for {issue_id}: {stderr}");
    }
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "parallel worktree items");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &root_id]);
    assert!(success, "root start failed: {stderr}");
    let worktree_path = dir.path().join(".atelier-worktrees").join(&worktree_id);
    let worktree_arg = worktree_path.to_string_lossy().to_string();
    let (success, worktree_out, stderr) = run_atelier(
        dir.path(),
        &["worktree", "for", &worktree_id, "--path", &worktree_arg],
    );
    assert!(success, "worktree for failed: {stderr}");
    assert!(worktree_out.contains(&worktree_arg));

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(status_out.contains("Current work:  1 issue"));
    assert!(status_out.contains(&format!("{root_id} - Root work [in_progress]")));
    assert!(!status_out.contains(&format!("{worktree_id} - Worktree work [in_progress]")));

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", mission_id.as_str()]);
    assert!(success, "mission status failed: {stderr}");
    let active_work_section = mission_out
        .split("Current Work")
        .nth(1)
        .expect("current work section missing")
        .split("Next Commands")
        .next()
        .expect("next commands section missing");
    assert!(active_work_section.contains(&root_id), "{mission_out}");
    assert!(!active_work_section.contains(&worktree_id), "{mission_out}");

    let (success, worktree_status, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(success, "worktree status failed: {stderr}");
    assert!(worktree_status.contains(&worktree_arg), "{worktree_status}");
    assert!(
        worktree_status.contains(&format!("{worktree_id} [active]")),
        "{worktree_status}"
    );
}

#[test]
fn test_root_start_reports_workflow_validator_failure() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Validator-gated start"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Validator-gated start");
    migrate_default_issue_workflow(dir.path());
    let policy_path = dir.path().join(".atelier").join("workflow.yaml");
    let policy = std::fs::read_to_string(&policy_path).unwrap();
    std::fs::write(
        &policy_path,
        policy.replace(
            "      start:\n        from: [todo, blocked]\n        to: in_progress\n",
            "      start:\n        from: [todo, blocked]\n        to: in_progress\n        validators: [proof_attached]\n",
        ),
    )
    .unwrap();
    commit_all(dir.path(), "validator-gated start policy");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(!success, "root start should fail when validators block it");
    assert!(stdout.contains("Blockers"), "{stdout}");
    assert!(stderr.contains("proof_attached"), "{stderr}");

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(issue_text.contains("status: \"todo\""), "{issue_text}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_blocked",
        &[
            "Blocked transition start from todo",
            "transition: \"start\"",
            "reason: \"validator proof_attached failed:",
        ],
    );
    assert!(
        !activities
            .iter()
            .any(|text| text.contains("event_type: \"work_started\"")),
        "blocked start should not record work_started:\n{}",
        activities.join("\n--- activity ---\n")
    );
}

#[test]
fn test_issue_transition_blocked_attempt_records_activity_without_evidence() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Validator blocked item",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Validator blocked item");
    migrate_default_issue_workflow(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "start failed: {stderr}");

    let evidence_dir = dir.path().join(".atelier").join("evidence");
    let evidence_before = std::fs::read_dir(&evidence_dir)
        .unwrap()
        .filter(|entry| {
            entry
                .as_ref()
                .ok()
                .and_then(|entry| entry.path().extension().map(|ext| ext == "md"))
                .unwrap_or(false)
        })
        .count();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_validation"],
    );
    assert!(
        !success,
        "request_validation should fail without a completed review"
    );
    assert!(stdout.contains("Blockers"), "{stdout}");
    assert!(stderr.contains("review_ready"), "{stderr}");
    assert!(stderr.contains("blocked"), "{stderr}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_blocked",
        &[
            "Blocked transition request_validation from in_progress",
            "transition: \"request_validation\"",
            "reason: \"validator review_ready failed:",
        ],
    );

    let evidence_after = std::fs::read_dir(&evidence_dir)
        .unwrap()
        .filter(|entry| {
            entry
                .as_ref()
                .ok()
                .and_then(|entry| entry.path().extension().map(|ext| ext == "md"))
                .unwrap_or(false)
        })
        .count();
    assert_eq!(
        evidence_before, evidence_after,
        "blocked transition created evidence"
    );
}

#[test]
fn test_issue_transition_close_reports_blockers_and_records_blocked_activity() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Blocked close item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Blocked close item");
    migrate_default_issue_workflow(dir.path());

    for args in [
        vec!["issue", "transition", &issue_id, "start"],
        vec!["issue", "transition", &issue_id, "request_review"],
        vec!["issue", "transition", &issue_id, "request_validation"],
    ] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "transition {:?} failed: {stderr}", args);
    }

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "close"]);
    assert!(!success, "close should be blocked without reason and proof");
    assert!(stdout.contains("Blockers"), "{stdout}");
    assert!(
        stderr.contains("missing required field close_reason"),
        "{stderr}"
    );
    assert!(stderr.contains("proof_attached"), "{stderr}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_blocked",
        &[
            "Blocked transition close from validation",
            "transition: \"close\"",
            "reason: \"missing required field close_reason;",
        ],
    );
}

#[test]
fn test_issue_close_uses_terminal_transition_and_clears_active_work() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Closable workflow item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Closable workflow item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "workflow-ready close item");

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
    attach_issue_pass_evidence(dir.path(), &issue_id);
    commit_all(dir.path(), "ready for close");

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &issue_id, "--reason", "done"],
    );
    assert!(success, "issue close failed: {stderr}");
    assert!(
        close_out.contains("Applied transition close"),
        "{close_out}"
    );
    assert!(close_out.contains("To:       done"), "{close_out}");

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(issue_text.contains("status: \"done\""), "{issue_text}");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(status_out.contains("Current work:  none"), "{status_out}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_applied",
        &[
            "Applied transition close",
            "transition: \"close\"",
            "to: \"done\"",
        ],
    );
}

#[test]
fn test_issue_close_requires_to_when_done_target_is_ambiguous_and_can_archive() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Archivable workflow item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Archivable workflow item");
    migrate_default_issue_workflow(dir.path());
    let policy_path = dir.path().join(".atelier").join("workflow.yaml");
    let policy = std::fs::read_to_string(&policy_path).unwrap();
    std::fs::write(
        &policy_path,
        policy.replace(
            "      close:\n        from: [validation]\n        to: done\n        required_fields: [close_reason]\n        validators:\n          - proof_attached\n          - blockers_clear\n          - lint_clear\n          - durable_current\n          - closeout_clean\n        guidance: [close_with_proof]\n",
            "      close:\n        from: [validation]\n        to: done\n        required_fields: [close_reason]\n        validators:\n          - proof_attached\n          - blockers_clear\n          - lint_clear\n          - durable_current\n          - closeout_clean\n        guidance: [close_with_proof]\n      archive:\n        from: [validation]\n        to: archived\n        required_fields: [close_reason]\n        validators:\n          - proof_attached\n          - blockers_clear\n          - lint_clear\n          - durable_current\n          - closeout_clean\n        guidance: [close_with_proof]\n",
        ),
    )
    .unwrap();
    commit_all(dir.path(), "archivable workflow policy");

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
    attach_issue_pass_evidence(dir.path(), &issue_id);
    commit_all(dir.path(), "ready for archive");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &issue_id, "--reason", "needs archive"],
    );
    assert!(!success, "ambiguous close should require --to");
    assert!(
        stderr.contains("multiple terminal done targets"),
        "{stderr}"
    );
    assert!(stderr.contains("available: archived, done"), "{stderr}");

    let (success, archive_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "close",
            &issue_id,
            "--to",
            "archived",
            "--reason",
            "archived by policy",
        ],
    );
    assert!(success, "archived close failed: {stderr}");
    assert!(
        archive_out.contains("Applied transition archive"),
        "{archive_out}"
    );
    assert!(archive_out.contains("To:       archived"), "{archive_out}");

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(issue_text.contains("status: \"archived\""), "{issue_text}");
}

#[test]
fn test_removed_abandon_rejects_without_changing_status() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Abandonable workflow item"],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Abandonable workflow item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "workflow-ready abandon item");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "start failed: {stderr}");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["abandon", &issue_id, "--reason", "paused for handoff"],
    );
    assert!(!success, "removed abandon should fail");
    assert!(stderr.contains("`atelier abandon` was removed"), "{stderr}");

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(
        issue_text.contains("status: \"in_progress\""),
        "{issue_text}"
    );

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(
        status_out.contains("Current work:  1 issue"),
        "{status_out}"
    );
    assert!(status_out.contains(&format!(
        "{issue_id} - Abandonable workflow item [in_progress]"
    )));
}

#[test]
fn test_issue_transition_rejects_unknown_transition_name() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Unknown transition item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Unknown transition item");
    migrate_default_issue_workflow(dir.path());

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "ship_it"]);
    assert!(!success, "unknown transition should fail");
    assert!(stderr.contains("Unknown transition 'ship_it'"), "{stderr}");
    assert!(
        stderr.contains("available from 'todo' are: block, start"),
        "{stderr}"
    );
}

#[test]
fn test_issue_transition_requires_workflow_policy_file() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Missing policy item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Missing policy item");

    std::fs::remove_file(dir.path().join(".atelier").join("workflow.yaml")).unwrap();

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(
        !success,
        "transition options should fail without workflow policy"
    );
    assert!(stderr.contains("workflow_config_missing"), "{stderr}");
}

#[test]
fn test_issue_transition_rejects_unmigrated_issue_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Unmigrated transition item"],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Unmigrated transition item");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let issue_text = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(
        &issue_path,
        issue_text.replace("status: \"todo\"", "status: \"open\""),
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(!success, "transition options should reject legacy status");
    assert!(stderr.contains("status 'open'"), "{stderr}");
    assert!(
        stderr.contains("not allowed by the workflow policy"),
        "{stderr}"
    );
}

#[test]
fn test_issue_transition_options_render_guidance_and_exact_command() {
    let dir = tempdir().unwrap();
    init_atelier_without_workflow(dir.path());
    migrate_default_issue_workflow(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Guided transition item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Guided transition item");
    for args in [
        vec!["issue", "transition", &issue_id, "start"],
        vec!["issue", "transition", &issue_id, "request_review"],
        vec!["issue", "transition", &issue_id, "request_validation"],
    ] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "transition {:?} failed: {stderr}", args);
    }

    let (success, options_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(options_out.contains("close [blocked]"), "{options_out}");
    assert!(options_out.contains("Guidance"), "{options_out}");
    assert!(
        options_out.contains(&format!(
            "Closing {} requires attached evidence and no open blockers.",
            issue_id
        )),
        "{options_out}"
    );
    assert!(options_out.contains(&format!(
        "atelier issue transition {issue_id} close --reason \"...\""
    )));
}

#[test]
fn test_issue_help_uses_reduced_lifecycle_surface() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "--help"]);
    assert!(success, "issue help failed: {stderr}");
    for command in [
        "create",
        "list",
        "show",
        "transition",
        "update",
        "close",
        "block",
        "unblock",
        "blocked",
    ] {
        assert!(
            stdout
                .lines()
                .any(|line| line.trim_start().starts_with(command)),
            "missing reduced issue command {command}:\n{stdout}"
        );
    }
    for hidden in [
        "quick",
        "subissue",
        "reopen",
        "label",
        "unlabel",
        "close-all",
        "delete",
        "next",
        "tested",
    ] {
        assert!(
            !stdout
                .lines()
                .any(|line| line.trim_start().starts_with(hidden)),
            "folded or moved command {hidden} is still visible:\n{stdout}"
        );
    }

    let (success, update_help, stderr) = run_atelier(dir.path(), &["issue", "update", "--help"]);
    assert!(success, "issue update help failed: {stderr}");
    assert!(!update_help.contains("--claim"));
    assert!(!update_help.contains("--status"));
    assert!(!update_help.contains("--description"));
}

#[test]
fn test_non_lifecycle_issue_flows_use_explicit_homes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Source graph item"]);
    let source_id = issue_ref(dir.path(), 1);
    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Target graph item",
            "--parent",
            &source_id,
        ],
    );
    run_atelier(dir.path(), &["issue", "create", "Disposable item"]);
    let disposable_id = issue_ref(dir.path(), 3);

    let (success, search_out, stderr) = run_atelier(dir.path(), &["search", "Source"]);
    assert!(success, "search failed: {stderr}");
    assert!(search_out.contains("Source graph item"));

    let (success, impact_out, stderr) = run_atelier(dir.path(), &["graph", "impact", &source_id]);
    assert!(success, "graph impact failed: {stderr}");
    assert!(impact_out.contains("downstream impact"));
    assert!(impact_out.contains("Target graph item"));

    let (success, tree_out, stderr) = run_atelier(dir.path(), &["graph", "tree", "--compact"]);
    assert!(success, "graph tree failed: {stderr}");
    assert!(tree_out.contains("Compact Issue Hierarchy"));

    let (success, note_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "note",
            &source_id,
            "Explicit note body",
            "--kind",
            "observation",
        ],
    );
    assert!(success, "issue note failed: {stderr}");
    assert!(note_out.contains("Added note to issue"));
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &source_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(show_out.contains("Explicit note body"));

    let (success, delete_out, stderr) = run_atelier(
        dir.path(),
        &["maintenance", "delete", "issue", &disposable_id, "--force"],
    );
    assert!(success, "maintenance delete failed: {stderr}");
    assert!(delete_out.contains("Deleted issue"));
}

#[test]
fn test_graph_commands_include_mission_linked_issue_hierarchy() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Graph Mission"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Graph Mission");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Mission Epic", "--issue-type", "epic"],
    );
    assert!(success, "issue create failed: {stderr}");
    let epic_id = issue_ref(dir.path(), 1);

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Mission Child", "--parent", &epic_id],
    );
    assert!(success, "child issue create failed: {stderr}");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", &mission_id, &epic_id]);
    assert!(success, "mission add-work failed: {stderr}");

    let (success, impact_out, stderr) = run_atelier(dir.path(), &["graph", "impact", &mission_id]);
    assert!(success, "graph impact failed: {stderr}");
    assert!(impact_out.contains("Mission"));
    assert!(impact_out.contains("Graph Mission"));
    assert!(impact_out.contains("Mission Epic"));
    assert!(impact_out.contains("Mission Child"));

    let (success, tree_out, stderr) = run_atelier(dir.path(), &["graph", "tree", "--compact"]);
    assert!(success, "graph tree failed: {stderr}");
    assert!(tree_out.contains("Compact Issue Hierarchy"));
    assert!(tree_out.contains("[mission ready]"));
    assert!(tree_out.contains("Graph Mission"));
    assert!(tree_out.contains("Mission Epic"));
    assert!(tree_out.contains("Mission Child"));
}

#[test]
fn test_hidden_issue_helpers_do_not_emit_compatibility_guidance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Old surface source"]);
    run_atelier(dir.path(), &["issue", "create", "Old surface target"]);

    for args in [
        vec!["issue", "quick", "Old quick"],
        vec!["issue", "subissue", "1", "Old child"],
        vec!["issue", "search", "Old"],
        vec!["issue", "comment", "1", "compat note"],
        vec!["issue", "label", "1", "old-label"],
        vec!["issue", "unlabel", "1", "old-label"],
        vec!["issue", "relate", "1", "2"],
        vec!["issue", "unrelate", "1", "2"],
        vec!["issue", "related", "1"],
        vec!["issue", "impact", "1"],
        vec!["issue", "next"],
        vec!["issue", "tree"],
        vec!["issue", "tested"],
        vec!["issue", "delete", "1", "--force"],
        vec!["issue", "close-all"],
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &args);
        assert!(!success, "{args:?} unexpectedly succeeded");
        assert!(
            stderr.contains("unrecognized subcommand"),
            "{args:?} should be removed, got:\n{stderr}"
        );
        assert!(
            !stderr.contains("was removed"),
            "{args:?} should not emit compatibility guidance:\n{stderr}"
        );
    }
}

#[test]
fn test_generic_link_rejection_names_record_specific_replacements() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Target issue"]);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["link", "add", "mission", "atelier-none", "issue", "1"],
    );
    assert!(!success, "generic link command should be removed");
    assert!(stderr.contains("`atelier link` was removed"));
    assert!(stderr.contains("atelier mission add-work"));
    assert!(stderr.contains("atelier issue block"));
    assert!(stderr.contains("atelier evidence attach"));
}

#[test]
fn test_explicit_homes_reject_non_issue_targets_until_supported() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "maintenance",
            "delete",
            "mission",
            "atelier-none",
            "--force",
        ],
    );
    assert!(
        !success,
        "maintenance delete unexpectedly accepted a mission target"
    );
    assert!(stderr.contains("supports issue records only"));
}

#[test]
fn test_removed_aliases_fail_as_unknown_commands() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for args in [
        vec!["show"],
        vec!["ready"],
        vec!["sync"],
        vec!["mission", "view"],
        vec!["work", "status"],
        vec!["work", "worktree"],
    ] {
        let (success, _, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(!success, "{args:?} unexpectedly succeeded");
        assert!(
            stderr.contains("unrecognized subcommand") || stderr.contains("unexpected argument"),
            "{args:?} did not fail as an unknown command:\n{stderr}"
        );
    }
}

#[test]
fn test_removed_commands_suggest_supported_replacements() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for (args, expected) in [
        (
            vec!["workflow", "check", "--json"],
            vec![
                "`atelier workflow check` is not the normal workflow-readiness path",
                "atelier issue transition <id> --options",
                "atelier mission status [<id>]",
            ],
        ),
        (
            vec!["finish"],
            vec![
                "`atelier finish` was removed",
                "atelier issue close <id> --reason",
                "atelier issue transition <id> --options",
            ],
        ),
        (
            vec!["current-work"],
            vec![
                "`atelier current-work` was removed",
                "atelier status",
                "atelier issue transition <id> --options",
            ],
        ),
        (
            vec!["issue", "new", "Replacement test"],
            vec!["`atelier issue new` was removed", "atelier issue create"],
        ),
        (
            vec!["work", "start", "atelier-z1p8"],
            vec![
                "`atelier work start` was removed",
                "atelier start <issue-id>",
                "atelier worktree for <issue-id>",
            ],
        ),
        (
            vec!["archive", "add", "atelier-z1p8"],
            vec![
                "`atelier archive` was removed",
                "atelier issue close <id> --to archived --reason",
            ],
        ),
        (
            vec!["session", "status"],
            vec![
                "`atelier session` was removed",
                "atelier start <issue-id>",
                "atelier issue note <id>",
            ],
        ),
        (
            vec!["timer"],
            vec![
                "`atelier timer` was removed",
                "atelier status",
                "atelier history --issue <id>",
            ],
        ),
    ] {
        let (success, stdout, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(!success, "{args:?} unexpectedly succeeded");
        assert!(
            stdout.is_empty(),
            "{args:?} should not execute a compatibility path:\n{stdout}"
        );
        for expected_text in expected {
            assert!(
                stderr.contains(expected_text),
                "{args:?} stderr missing {expected_text:?}:\n{stderr}"
            );
        }
    }
}

// ==================== Issue Creation Tests ====================
