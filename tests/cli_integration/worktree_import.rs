use super::support::*;

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
    assert!(start_out.contains("Tracked work is now derived from issue workflow status."));

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(status_out.contains("Atelier Status"));
    assert!(status_out.contains("Current work:  1 issue"));
    assert!(status_out.contains(&format!("{issue_id} - Work item [in_progress]")));
    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert!(
        !activities
            .iter()
            .any(|text| text.contains("event_type: \"work_started\"")),
        "root start should not record work_started activity:\n{}",
        activities.join("\n--- activity ---\n")
    );

    let worktree_path = dir.path().join(".atelier-worktrees").join(&issue_id);
    let worktree_arg = worktree_path.to_string_lossy().to_string();
    let (success, worktree_out, stderr) = run_atelier(
        dir.path(),
        &["worktree", "for", &issue_id, "--path", &worktree_arg],
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
        child_status_out.contains(&format!("{issue_id} - Work item [in_progress]")),
        "worktree-local status should derive current work from Markdown: {child_status_out}"
    );

    let (success, status_out, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(success, "worktree status failed: {stderr}");
    assert!(status_out.contains(&worktree_arg));
    assert!(status_out.contains(&format!("{issue_id} [active]")));

    let (success, status_human, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(success, "human worktree status failed: {stderr}");
    assert!(status_human.contains("Worktree Status"));
    assert!(status_human.contains(&worktree_arg));
    assert!(status_human.contains("Branch:"));
    assert!(status_human.contains("State:"));
    assert!(status_human.contains("Associated Work"));
    assert!(status_human.contains(&format!("{issue_id} [active]")));
    assert!(!status_human.contains("work:"));
    assert!(!status_human.contains("export:"));

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["worktree", "remove", "--force", &worktree_arg])
        .status()
        .unwrap();
    assert!(status.success(), "manual git worktree remove failed");
    assert!(!worktree_path.exists());
    let (success, repair_out, stderr) = run_atelier(dir.path(), &["worktree", "repair", &issue_id]);
    assert!(success, "worktree repair failed: {stderr}");
    assert!(repair_out.contains("Cleared stale worktree association"));
    let (success, repaired_status, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(success, "worktree status after repair failed: {stderr}");
    assert!(!repaired_status.contains(&format!("{issue_id} [active]")));

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
    assert!(failed_status.contains(&format!("{failed_issue_id} [active]")));
}

#[test]
fn test_removed_repair_ignores_stale_runtime_association_for_status() {
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

    let stale_path = dir.path().join(".atelier-worktrees").join(&issue_id);
    let stale_path_arg = stale_path.to_string_lossy().to_string();
    let conn = rusqlite::Connection::open(dir.path().join(".atelier/runtime/state.db")).unwrap();
    conn.execute(
        "INSERT INTO work_associations (issue_id, status, branch, worktree_path, started_at)
         VALUES (?1, 'active', ?2, ?3, ?4)",
        rusqlite::params![
            &issue_id,
            format!("codex/{issue_id}"),
            &stale_path_arg,
            "2026-06-14T12:00:00Z"
        ],
    )
    .unwrap();
    assert_eq!(active_work_association_count(dir.path(), &issue_id), 1);
    drop(conn);
    assert!(!stale_path.exists());

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(
        success,
        "status failed with stale runtime association: {stderr}"
    );
    assert!(
        status_out.contains("Current work:  none"),
        "stale runtime association should not define current work:\n{status_out}"
    );

    let (success, _, stderr) = run_atelier(dir.path(), &["repair", &issue_id]);
    assert!(!success, "removed root repair should reject");
    assert!(stderr.contains("`atelier repair` was removed"), "{stderr}");
    assert_eq!(active_work_association_count(dir.path(), &issue_id), 1);
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
        stderr.contains("worktree association was not changed"),
        "failure should say association was not changed: {stderr}"
    );

    let (success, status_out, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(
        success,
        "worktree status after failed setup failed: {stderr}"
    );
    assert!(
        !status_out.contains(&format!("{issue_id} [active]")),
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
    assert!(root_status_out.contains(&format!("{issue_id} [active]")));

    let (success, child_status_out, stderr) = run_atelier(&worktree_path, &["status"]);
    assert!(success, "child status after retry failed: {stderr}");
    assert!(
        child_status_out.contains("Current work:  none"),
        "worktree association alone should not define current work: {child_status_out}"
    );
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
