use super::*;

// ==================== Init Tests ====================

#[test]
fn test_init_creates_atelier_directory() {
    let dir = tempdir().unwrap();
    let (success, stdout, _) = run_atelier(dir.path(), &["init"]);

    assert!(success);
    assert!(stdout.contains("Created") || stdout.contains("initialized"));
    assert!(stdout.contains("Created"));
    assert!(stdout.contains(".atelier/workflow.yaml"));
    assert!(stdout.contains("atelier check"));
    assert!(stdout.contains("atelier issue create \"Task\""));
    let lint_pos = stdout.find("atelier check").unwrap();
    let issue_pos = stdout.find("atelier issue create \"Task\"").unwrap();
    assert!(
        lint_pos < issue_pos,
        "fresh init must verify setup before suggesting issue creation:\n{stdout}"
    );
    assert!(dir.path().join(".atelier").exists());
    assert!(dir.path().join(".atelier").join("config.toml").exists());
    assert!(dir.path().join(".atelier").join("workflow.yaml").exists());
    assert!(!dir
        .path()
        .join(".atelier")
        .join("hook-config.json")
        .exists());
    assert!(!dir.path().join(".claude").exists());
    assert!(!dir.path().join(".mcp.json").exists());

    let config = std::fs::read_to_string(dir.path().join(".atelier/config.toml")).unwrap();
    assert!(config.contains("state_root = \".atelier\""));
    assert!(!config.contains("compatibility_state_root"));
    assert!(!config.contains("runtime_dir"));
    assert!(!config.contains("runtime_database"));
    assert!(!config.contains("cache_dir"));
}

#[test]
fn test_init_twice_is_idempotent() {
    let dir = tempdir().unwrap();

    run_atelier(dir.path(), &["init"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["init"]);

    assert!(success);
    assert!(stdout.contains("Atelier initialized successfully"));
    assert!(!dir.path().join(".claude").exists());
}

#[test]
fn test_integrations_command_is_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    assert!(!dir.path().join(".claude").exists());
    assert!(!dir.path().join(".mcp.json").exists());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["integrations", "claude", "install"]);
    assert!(
        !success,
        "removed integrations command unexpectedly succeeded"
    );
    assert!(stdout.is_empty(), "{stdout}");
    assert!(
        stderr.contains("unrecognized subcommand 'integrations'"),
        "{stderr}"
    );
    assert!(!stderr.contains("was removed"), "{stderr}");
    assert!(!stderr.contains("external assistant hooks"), "{stderr}");
    assert!(!stderr.contains("Claude"), "{stderr}");

    assert!(!dir.path().join(".claude").exists());
    assert!(!dir.path().join(".mcp.json").exists());
    assert!(!dir.path().join(".atelier/hook-config.json").exists());
}

#[test]
fn test_prune_help_describes_both_retention_classes() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["prune", "--help"]);
    assert!(success, "prune help failed: {stderr}");
    assert!(
        stdout.contains("Override canonical record retention for this prune pass"),
        "{stdout}"
    );
}

#[test]
fn test_prune_quiet_reports_compact_equivalent_counts() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    commit_all(dir.path(), "quiet prune fixture");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["--quiet", "prune"]);
    assert!(success, "quiet prune failed: {stderr}");
    assert!(stdout.starts_with("mode=dry-run eligible="), "{stdout}");
    assert!(stdout.contains(" protected="), "{stdout}");
    assert!(stdout.contains(" removed=0 failures=0"), "{stdout}");
    assert_eq!(stdout.lines().count(), 1, "{stdout}");
}

#[test]
fn test_prune_protects_current_base_and_unowned_git_branches() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    commit_all(dir.path(), "prune Git fixture");
    let base = String::from_utf8(
        Command::new("git")
            .current_dir(dir.path())
            .args(["branch", "--show-current"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim()
    .to_string();
    Command::new("git")
        .current_dir(dir.path())
        .args(["branch", "merged-but-unowned"])
        .status()
        .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["checkout", "-b", "unmerged-branch"])
        .status()
        .unwrap();
    fs::write(dir.path().join("unmerged.txt"), "unmerged\n").unwrap();
    commit_all(dir.path(), "unmerged branch work");
    Command::new("git")
        .current_dir(dir.path())
        .args(["checkout", &base])
        .status()
        .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune"]);
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(stdout.contains("Git Branches And Worktrees"), "{stdout}");
    assert!(
        stdout.contains("protected branch merged-but-unowned"),
        "{stdout}"
    );
    assert!(
        stdout.contains("no terminal owner record association"),
        "{stdout}"
    );
    assert!(
        stdout.contains("protected branch unmerged-branch"),
        "{stdout}"
    );
    assert!(
        stdout.contains(&format!("protected branch {base}")),
        "{stdout}"
    );
}

#[test]
fn test_prune_protects_active_and_recent_terminal_owner_branches() {
    let dir = tempdir().unwrap();
    let remote = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    for title in ["Active branch owner", "Recent terminal branch owner"] {
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", title]);
        assert!(success, "issue create failed: {stderr}");
    }
    let active_id = issue_ref(dir.path(), 1);
    let recent_id = issue_ref(dir.path(), 2);
    make_issue_terminal_before_retention(dir.path(), &recent_id, 1);
    commit_all(dir.path(), "active and recent owner fixture");
    let active_branch = format!("task/{active_id}");
    let recent_branch = format!("task/{recent_id}");
    for args in [
        vec!["init", "--bare", remote.path().to_str().unwrap()],
        vec!["remote", "add", "origin", remote.path().to_str().unwrap()],
        vec!["push", "-u", "origin", "main"],
        vec!["branch", &active_branch],
        vec!["push", "-u", "origin", &active_branch],
        vec!["branch", &recent_branch],
        vec!["push", "-u", "origin", &recent_branch],
    ] {
        let status = Command::new("git")
            .current_dir(dir.path())
            .args(&args)
            .status()
            .unwrap();
        assert!(status.success(), "git {:?} failed", args);
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(
        stdout.contains(&format!("protected branch {active_branch}"))
            && stdout.contains(&format!("owner {active_id} has active workflow state")),
        "{stdout}"
    );
    assert!(
        stdout.contains(&format!("protected branch {recent_branch}"))
            && stdout.contains(&format!(
                "terminal owner {recent_id} is within retention window"
            )),
        "{stdout}"
    );
}

#[test]
fn test_prune_protects_terminal_epic_branch_with_active_descendant() {
    let dir = tempdir().unwrap();
    let remote = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Terminal epic owner",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Active epic child", "--parent", &epic_id],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_ref(dir.path(), 2);
    make_issue_terminal_before_retention(dir.path(), &epic_id, 45);
    commit_all(dir.path(), "terminal epic with active descendant fixture");
    let branch = format!("epic/{epic_id}");
    let worktree = dir.path().join("terminal-epic-worktree");
    for args in [
        vec!["init", "--bare", remote.path().to_str().unwrap()],
        vec!["remote", "add", "origin", remote.path().to_str().unwrap()],
        vec!["push", "-u", "origin", "main"],
        vec!["branch", &branch],
        vec!["push", "-u", "origin", &branch],
        vec!["worktree", "add", worktree.to_str().unwrap(), &branch],
    ] {
        let status = Command::new("git")
            .current_dir(dir.path())
            .args(&args)
            .status()
            .unwrap();
        assert!(status.success(), "git {:?} failed", args);
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
    assert!(success, "prune dry-run failed: {stderr}");
    let reason = format!("owner {epic_id} has active descendant {child_id}");
    assert!(
        stdout.contains(&format!("protected branch {branch}")) && stdout.contains(&reason),
        "{stdout}"
    );
    assert!(
        stdout.contains("protected worktree") && stdout.contains(&reason),
        "{stdout}"
    );
    assert!(
        stdout.contains(&format!("protected issue {epic_id}"))
            && stdout.contains(&format!("active descendant {child_id}")),
        "{stdout}"
    );

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["prune", "--apply", "--retention-days", "30"]);
    assert!(success, "prune apply failed: {stderr}");
    assert!(
        stdout.contains(&format!("protected branch {branch}")) && stdout.contains(&reason),
        "{stdout}"
    );
    assert!(
        stdout.contains("protected worktree") && stdout.contains(&reason),
        "{stdout}"
    );
    assert!(
        stdout.contains(&format!("protected issue {epic_id}"))
            && stdout.contains(&format!("active descendant {child_id}")),
        "{stdout}"
    );
    assert!(
        canonical_issue_path(dir.path(), &epic_id).exists(),
        "apply removed terminal parent with an active descendant"
    );
    assert!(
        worktree.exists(),
        "apply removed active descendant worktree"
    );
    let branch_exists = Command::new("git")
        .current_dir(dir.path())
        .args([
            "show-ref",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ])
        .status()
        .unwrap();
    assert!(
        branch_exists.success(),
        "apply removed active descendant branch"
    );
}

#[test]
fn test_prune_protects_unmerged_and_unpushed_terminal_owner_branches() {
    let dir = tempdir().unwrap();
    let remote = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    for title in ["Unmerged branch owner", "Unpushed branch owner"] {
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", title]);
        assert!(success, "issue create failed: {stderr}");
    }
    let unmerged_id = issue_ref(dir.path(), 1);
    let unpushed_id = issue_ref(dir.path(), 2);
    make_issue_terminal_before_retention(dir.path(), &unmerged_id, 45);
    make_issue_terminal_before_retention(dir.path(), &unpushed_id, 45);
    commit_all(dir.path(), "terminal branch safety fixture");
    let unmerged_branch = format!("task/{unmerged_id}");
    let unpushed_branch = format!("task/{unpushed_id}");
    for args in [
        vec!["init", "--bare", remote.path().to_str().unwrap()],
        vec!["remote", "add", "origin", remote.path().to_str().unwrap()],
        vec!["push", "-u", "origin", "main"],
        vec!["branch", &unmerged_branch],
        vec!["push", "-u", "origin", &unmerged_branch],
        vec!["branch", &unpushed_branch],
        vec!["push", "-u", "origin", &unpushed_branch],
        vec!["checkout", &unmerged_branch],
    ] {
        let status = Command::new("git")
            .current_dir(dir.path())
            .args(&args)
            .status()
            .unwrap();
        assert!(status.success(), "git {:?} failed", args);
    }
    fs::write(dir.path().join("unmerged-owner.txt"), "unmerged\n").unwrap();
    commit_all(dir.path(), "unmerged owner work");
    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["push"])
        .status()
        .unwrap();
    assert!(status.success());

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["checkout", &unpushed_branch])
        .status()
        .unwrap();
    assert!(status.success());
    fs::write(dir.path().join("unpushed-owner.txt"), "unpushed\n").unwrap();
    commit_all(dir.path(), "unpushed owner work");
    for args in [
        vec!["checkout", "main"],
        vec!["merge", "--ff-only", &unpushed_branch],
    ] {
        let status = Command::new("git")
            .current_dir(dir.path())
            .args(&args)
            .status()
            .unwrap();
        assert!(status.success(), "git {:?} failed", args);
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(
        stdout.contains(&format!("protected branch {unmerged_branch}"))
            && stdout.contains("contains commits not integrated into owner base main"),
        "{stdout}"
    );
    assert!(
        stdout.contains(&format!("protected branch {unpushed_branch}"))
            && stdout.contains(&format!(
                "contains commits not present in upstream origin/{unpushed_branch}"
            )),
        "{stdout}"
    );
}

#[test]
fn test_prune_protects_non_current_dirty_worktree() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    commit_all(dir.path(), "dirty worktree prune fixture");
    let worktree = dir.path().join("dirty-worktree");
    let status = Command::new("git")
        .current_dir(dir.path())
        .args([
            "worktree",
            "add",
            "-b",
            "dirty-worktree",
            worktree.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert!(status.success());
    fs::write(worktree.join("dirty.txt"), "dirty\n").unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--apply"]);
    assert!(success, "prune apply failed: {stderr}");
    assert!(stdout.contains("protected worktree"), "{stdout}");
    assert!(stdout.contains("dirty worktree"), "{stdout}");
    assert!(worktree.exists(), "apply removed dirty worktree");
}

#[test]
fn test_prune_protects_missing_locked_worktree_and_branch_without_inspecting_path() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    commit_all(dir.path(), "locked worktree prune fixture");
    let branch = "locked-worktree";
    let worktree = dir.path().join(branch);
    for args in [
        vec!["worktree", "add", "-b", branch, worktree.to_str().unwrap()],
        vec!["worktree", "lock", worktree.to_str().unwrap()],
    ] {
        let status = Command::new("git")
            .current_dir(dir.path())
            .args(&args)
            .status()
            .unwrap();
        assert!(status.success(), "git {:?} failed", args);
    }
    fs::remove_dir_all(&worktree).unwrap();

    for args in [vec!["prune"], vec!["prune", "--apply"]] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "prune {:?} failed: {stderr}", args);
        assert!(
            stdout.contains(&format!("protected branch {branch} - locked worktree")),
            "{stdout}"
        );
        assert!(
            stdout.contains(&format!(
                "protected worktree {} - locked worktree",
                worktree.display()
            )),
            "{stdout}"
        );
        assert!(!worktree.exists(), "prune recreated missing worktree");
    }

    let branch_exists = Command::new("git")
        .current_dir(dir.path())
        .args([
            "show-ref",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ])
        .status()
        .unwrap();
    assert!(branch_exists.success(), "apply removed locked branch");
    let registrations = Command::new("git")
        .current_dir(dir.path())
        .args(["worktree", "list", "--porcelain"])
        .output()
        .unwrap();
    assert!(registrations.status.success());
    let registrations = String::from_utf8(registrations.stdout).unwrap();
    assert!(
        registrations.contains(&format!("worktree {}", worktree.display()))
            && registrations.contains("locked"),
        "apply removed locked registration:\n{registrations}"
    );
}

#[test]
fn test_prune_removes_merged_and_pushed_terminal_owner_branch_and_worktree() {
    let dir = tempdir().unwrap();
    let remote = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Terminal branch owner"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 45);
    let policy = dir.path().join(".atelier/workflow.yaml");
    fs::write(
        &policy,
        fs::read_to_string(&policy).unwrap().replace(
            "merge_strategy: fast_forward_only",
            "merge_strategy: squash",
        ),
    )
    .unwrap();
    commit_all(dir.path(), "terminal owner fixture");
    let branch = format!("task/{issue_id}");
    let worktree = dir.path().join("terminal-owner-worktree");
    for args in [
        vec!["init", "--bare", remote.path().to_str().unwrap()],
        vec!["remote", "add", "origin", remote.path().to_str().unwrap()],
        vec!["push", "-u", "origin", "main"],
        vec!["branch", &branch],
        vec!["push", "-u", "origin", &branch],
        vec!["worktree", "add", worktree.to_str().unwrap(), &branch],
    ] {
        let status = Command::new("git")
            .current_dir(dir.path())
            .args(&args)
            .status()
            .unwrap();
        assert!(status.success(), "git {:?} failed", args);
    }
    fs::write(worktree.join("squash-merged.txt"), "merged branch work\n").unwrap();
    commit_all(&worktree, "terminal owner branch work");
    let status = Command::new("git")
        .current_dir(&worktree)
        .args(["push"])
        .status()
        .unwrap();
    assert!(status.success());
    for args in [
        vec!["merge", "--squash", &branch],
        vec!["commit", "-m", &format!("Squash merge {branch} into main")],
    ] {
        let status = Command::new("git")
            .current_dir(dir.path())
            .args(&args)
            .status()
            .unwrap();
        assert!(status.success(), "git {:?} failed", args);
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(
        stdout.contains(&format!("eligible branch {branch}")),
        "{stdout}"
    );
    assert!(stdout.contains("eligible worktree"), "{stdout}");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["prune", "--apply", "--retention-days", "30"]);
    assert!(success, "prune apply failed: {stderr}");
    assert!(
        stdout.contains(&format!("removed branch {branch}")),
        "{stdout}"
    );
    assert!(stdout.contains("removed worktree"), "{stdout}");
    assert!(!worktree.exists(), "apply left eligible worktree in place");
    let branch_exists = Command::new("git")
        .current_dir(dir.path())
        .args([
            "show-ref",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ])
        .status()
        .unwrap();
    assert!(
        !branch_exists.success(),
        "apply left eligible branch in place"
    );
}

#[test]
fn test_prune_dry_run_reports_terminal_canonical_issue_without_removing_it() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Old terminal work"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 45);
    commit_all(dir.path(), "old terminal issue fixture");

    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(stdout.contains("Canonical Records"), "{stdout}");
    assert!(stdout.contains("eligible issue"), "{stdout}");
    assert!(stdout.contains(&issue_id), "{stdout}");
    assert!(
        stdout.contains(&format!("git log --all -- .atelier/issues/{issue_id}.md")),
        "{stdout}"
    );
    assert!(issue_path.exists(), "dry-run removed canonical issue");
}

#[test]
fn test_prune_uses_configured_canonical_retention_days_by_default() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    set_prune_canonical_retention_days(dir.path(), 7);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Configured retention target"],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 10);
    commit_all(dir.path(), "configured prune retention fixture");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune"]);
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(stdout.contains("Canonical Records"), "{stdout}");
    assert!(stdout.contains("Retention: 7 day(s)"), "{stdout}");
    assert!(stdout.contains("eligible issue"), "{stdout}");
    assert!(stdout.contains(&issue_id), "{stdout}");
}

#[test]
fn test_prune_retention_flag_overrides_project_config_for_canonical_records() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    set_prune_canonical_retention_days(dir.path(), 30);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Override retention target"],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 10);
    commit_all(dir.path(), "override prune retention fixture");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune"]);
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(stdout.contains("Canonical Records"), "{stdout}");
    assert!(stdout.contains("Retention: 30 day(s)"), "{stdout}");
    assert!(stdout.contains("Candidates: none"), "{stdout}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "7"]);
    assert!(success, "prune override dry-run failed: {stderr}");
    assert!(stdout.contains("Retention: 7 day(s)"), "{stdout}");
    assert!(stdout.contains("eligible issue"), "{stdout}");
    assert!(stdout.contains(&issue_id), "{stdout}");
}

#[test]
fn test_prune_apply_removes_terminal_canonical_issue_and_keeps_git_recovery() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Old terminal apply"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 45);
    commit_all(dir.path(), "old terminal issue fixture");

    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["prune", "--apply", "--retention-days", "30"]);
    assert!(success, "prune apply failed: {stderr}");
    assert!(stdout.contains("removed issue"), "{stdout}");
    assert!(stdout.contains(&issue_id), "{stdout}");
    assert!(
        stdout.contains(&format!("git show <commit>:.atelier/issues/{issue_id}.md")),
        "{stdout}"
    );
    assert!(
        !issue_path.exists(),
        "apply left pruned issue in active tree"
    );

    let output = Command::new("git")
        .current_dir(dir.path())
        .args(["show", &format!("HEAD:.atelier/issues/{issue_id}.md")])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "Git history should recover pruned issue: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("Old terminal apply"),
        "Git recovery output should contain the old record"
    );
}

#[test]
fn test_prune_history_is_committed_and_refreshed_between_commands() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Uncommitted terminal"]);
    assert!(success, "{stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 45);
    for staged in [false, true] {
        if staged {
            assert!(Command::new("git")
                .current_dir(dir.path())
                .args(["add", ".atelier"])
                .status()
                .unwrap()
                .success());
        }
        let (success, stdout, stderr) =
            run_atelier(dir.path(), &["prune", "--apply", "--retention-days", "30"]);
        assert!(success, "{stderr}");
        assert!(
            stdout.contains("not present in Git history at HEAD"),
            "{stdout}"
        );
        assert!(canonical_issue_path(dir.path(), &issue_id).exists());
    }
    commit_all(dir.path(), "make terminal record recoverable");
    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["prune", "--apply", "--retention-days", "30"]);
    assert!(success, "{stderr}");
    assert!(stdout.contains("removed issue"), "{stdout}");
    assert!(!canonical_issue_path(dir.path(), &issue_id).exists());
}

#[test]
fn test_prune_protects_owner_record_with_staged_or_unstaged_changes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Dirty terminal owner"]);
    assert!(success, "{stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 45);
    commit_all(dir.path(), "old owner record");
    let (_remote, branch) = merged_pushed_owner_branch(dir.path(), &issue_id);
    let record = canonical_issue_path(dir.path(), &issue_id);
    fs::write(
        &record,
        format!(
            "{}\nUncommitted notes.\n",
            fs::read_to_string(&record).unwrap()
        ),
    )
    .unwrap();
    for staged in [false, true] {
        if staged {
            assert!(Command::new("git")
                .current_dir(dir.path())
                .args(["add", ".atelier"])
                .status()
                .unwrap()
                .success());
        }
        let (success, stdout, stderr) =
            run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
        assert!(success, "{stderr}");
        assert!(
            stdout.contains(&format!("protected branch {branch}")),
            "{stdout}"
        );
        assert!(
            stdout.contains("owner record has uncommitted changes"),
            "{stdout}"
        );
    }
    assert!(Command::new("git")
        .current_dir(dir.path())
        .args(["restore", "--staged", "--worktree", ".atelier"])
        .status()
        .unwrap()
        .success());
    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
    assert!(success, "{stderr}");
    assert!(
        stdout.contains(&format!("eligible branch {branch}")),
        "{stdout}"
    );
}

/// Gives `issue_id` a pushed `task/<id>` owner branch whose work is
/// squash-merged into main. Returns the bare remote (keep it alive) and the
/// branch name.
fn merged_pushed_owner_branch(dir: &Path, issue_id: &str) -> (tempfile::TempDir, String) {
    // Historical squash integration remains prunable when its policy says so.
    let policy = dir.join(".atelier/workflow.yaml");
    fs::write(
        &policy,
        fs::read_to_string(&policy).unwrap().replace(
            "merge_strategy: fast_forward_only",
            "merge_strategy: squash",
        ),
    )
    .unwrap();
    commit_all(dir, "historical squash policy");
    let remote = tempdir().unwrap();
    let branch = format!("task/{issue_id}");
    let git = |args: &[&str]| {
        let status = Command::new("git")
            .current_dir(dir)
            .args(args)
            .status()
            .unwrap();
        assert!(status.success(), "git {args:?} failed");
    };
    git(&["init", "--bare", remote.path().to_str().unwrap()]);
    git(&["remote", "add", "origin", remote.path().to_str().unwrap()]);
    git(&["push", "-q", "-u", "origin", "main"]);
    git(&["switch", "-q", "-c", &branch]);
    fs::write(dir.join(format!("{issue_id}-work.txt")), "merged work\n").unwrap();
    commit_all(dir, "owner branch work");
    git(&["push", "-q", "-u", "origin", &branch]);
    git(&["switch", "-q", "main"]);
    git(&["merge", "-q", "--squash", &branch]);
    git(&[
        "commit",
        "-q",
        "-m",
        &format!("Squash merge {branch} into main"),
    ]);
    git(&["push", "-q"]);
    (remote, branch)
}

fn branch_exists(dir: &Path, branch: &str) -> bool {
    Command::new("git")
        .current_dir(dir)
        .args([
            "show-ref",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ])
        .status()
        .unwrap()
        .success()
}

#[test]
#[cfg(feature = "test-hooks")]
fn test_prune_apply_restores_every_record_on_staging_failure() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    for title in ["Old terminal one", "Old terminal two"] {
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", title]);
        assert!(success, "issue create failed: {stderr}");
    }
    let mut ids = ["Old terminal one", "Old terminal two"]
        .map(|title| issue_id_by_title(dir.path(), title))
        .to_vec();
    ids.sort();
    for id in &ids {
        make_issue_terminal_before_retention(dir.path(), id, 45);
    }
    commit_all(dir.path(), "old terminal issues");
    let (_remote, branch) = merged_pushed_owner_branch(dir.path(), &ids[0]);

    let (success, _, stderr) = run_atelier_with_env(
        dir.path(),
        &["prune", "--apply", "--retention-days", "30"],
        &[("ATELIER_TEST_PRUNE_REMOVE_FAILURE", ids[1].as_str())],
    );

    assert!(!success, "a removal failure must fail prune");
    assert!(
        stderr.contains(&format!("failed to remove .atelier/issues/{}.md", ids[1]))
            && stderr.contains("every staged record was restored")
            && stderr.contains("no canonical records removed")
            && stderr.contains(&format!("Git cleanup already removed branch {branch}"))
            && stderr.contains("`git restore` does not restore branches or worktrees"),
        "{stderr}"
    );
    assert!(!branch_exists(dir.path(), &branch));
    assert!(canonical_issue_path(dir.path(), &ids[0]).exists());
    assert!(canonical_issue_path(dir.path(), &ids[1]).exists());
    let restored = Command::new("git")
        .current_dir(dir.path())
        .args(["restore", ".atelier"])
        .status()
        .unwrap();
    assert!(restored.success());
    assert!(canonical_issue_path(dir.path(), &ids[0]).exists());
}

#[test]
fn test_prune_keeps_done_child_listed_by_open_epic() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Open epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Open epic");
    move_epic_directly_to_ready(dir.path(), &epic_id);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Old done child", "--parent", &epic_id],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Old done child");
    make_issue_terminal_before_retention(dir.path(), &child_id, 45);
    commit_all(dir.path(), "open epic with old done child");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["prune", "--apply", "--retention-days", "30"]);
    assert!(success, "prune apply failed: {stderr}");
    assert!(
        stdout.contains(&format!(
            "referenced by retained issue {epic_id} (children)"
        )),
        "{stdout}"
    );
    assert!(canonical_issue_path(dir.path(), &child_id).exists());
    let (success, _, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(success, "records must still load after prune: {stderr}");
}

#[test]
fn test_prune_apply_refuses_canonical_cleanup_when_tracked_state_is_dirty() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Dirty prune target"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 45);
    commit_all(dir.path(), "old terminal issue fixture");
    fs::write(dir.path().join("tracked.txt"), "dirty tracked file\n").unwrap();
    commit_all(dir.path(), "tracked file fixture");
    let (_remote, branch) = merged_pushed_owner_branch(dir.path(), &issue_id);
    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(
        stdout.contains(&format!("eligible branch {branch}")),
        "the owner branch must be eligible so the refusal is what keeps it: {stdout}"
    );
    fs::write(
        dir.path().join("tracked.txt"),
        "dirty tracked file changed\n",
    )
    .unwrap();

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["prune", "--apply", "--retention-days", "30"]);
    assert!(!success, "dirty canonical prune unexpectedly succeeded");
    assert!(
        stderr.contains("canonical prune requires a clean tracked checkout")
            && stderr.contains("git status --short --branch"),
        "{stderr}"
    );
    assert!(
        canonical_issue_path(dir.path(), &issue_id).exists(),
        "dirty apply removed canonical issue"
    );
    assert!(
        branch_exists(dir.path(), &branch),
        "a refused canonical prune must not delete branches"
    );
}

#[test]
fn test_prune_requires_a_repository() {
    let dir = tempdir().unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["prune"]);

    assert!(!success, "prune outside a repository must fail");
    assert!(stderr.contains("Not an Atelier repository"), "{stderr}");
}

#[test]
fn test_prune_protects_old_evidence_attached_to_retained_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Active evidence owner"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    let evidence_id = attach_pass_evidence(
        dir.path(),
        "issue",
        &issue_id,
        "old evidence remains required by active work",
    );
    make_record_before_retention(dir.path(), "evidence", &evidence_id, 45);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(stdout.contains("protected evidence-record"), "{stdout}");
    assert!(stdout.contains(&evidence_id), "{stdout}");
    assert!(stdout.contains("attached to retained issue"), "{stdout}");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["prune", "--apply", "--retention-days", "30"]);
    assert!(success, "prune apply failed: {stderr}");
    assert!(stdout.contains("protected evidence-record"), "{stdout}");
    assert!(
        canonical_record_path(dir.path(), "evidence", &evidence_id).exists(),
        "apply removed protected evidence"
    );
}

#[test]
fn test_top_level_help_only_shows_core_commands() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["--help"]);
    assert!(success, "help failed: {stderr}");
    assert!(stdout.contains("Mission and proof oriented work coordination for agents"));
    for removed in ["history", "forgejo"] {
        assert!(
            !stdout
                .lines()
                .any(|line| line.trim_start().starts_with(&format!("{removed} "))),
            "root help should not expose removed {removed} command:\n{stdout}"
        );
    }
    assert!(!stdout.contains("Inspect canonical repo"));

    for heading in [
        "Setup:",
        "Orientation:",
        "Issues:",
        "Planning:",
        "Records:",
        "Maintenance:",
        "Common commands:",
        "Options:",
    ] {
        assert!(stdout.contains(heading), "missing help heading {heading}");
    }

    for command in [
        "init", "status", "work", "issue", "start", "bundle", "evidence", "check",
    ] {
        assert!(stdout.contains(command), "missing core command {command}");
    }
    assert!(!stdout.contains("Search issues, relationships, and activity"));
    for command in ["prime", "land", "setup"] {
        assert!(
            stdout
                .lines()
                .any(|line| line.trim_start().starts_with(&format!("{command} "))),
            "missing session command {command}: {stdout}"
        );
    }
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("dep ")),
        "root help should not expose dep:\n{stdout}"
    );
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("note ")),
        "root help should not expose generic note:\n{stdout}"
    );
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("link ")),
        "root help should not expose generic link:\n{stdout}"
    );
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("integrations ")),
        "root help should not expose integrations:\n{stdout}"
    );
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("rebuild ")),
        "root help should not present rebuild as a normal operator command:\n{stdout}"
    );
    for hidden in ["branch", "maintenance", "doctor"] {
        assert!(
            !stdout.lines().any(|line| {
                let command = line.trim_start();
                command == hidden || command.starts_with(&format!("{hidden} "))
            }),
            "root help should not expose hidden admin command {hidden}:\n{stdout}"
        );
    }

    for common in [
        "atelier work ready",
        "atelier work blocked",
        "atelier work missions",
        "atelier issue list",
        "atelier work mission <mission-id>",
        "atelier issue show <id>",
        "atelier issue create \"...\" --issue-type mission",
        "atelier issue show <mission-id>",
        "atelier bundle preview <file>",
        "atelier bundle apply <file> --yes",
        "atelier start <issue-id>",
        "atelier issue transition <issue-id>",
        "atelier check",
        "atelier check <issue-id>",
    ] {
        assert!(
            stdout.contains(common),
            "missing common command example {common}"
        );
    }
    let common_commands = stdout
        .split("Common commands:")
        .nth(1)
        .and_then(|section| section.split("Options:").next())
        .unwrap_or("");
    assert!(
        !common_commands.contains("atelier doctor"),
        "common commands should not teach doctor as routine work:\n{stdout}"
    );
    assert!(!stdout.contains("workflow validate"));

    assert!(
        !stdout.contains("\nCommands:\n"),
        "top-level help should use categorized commands, not a flat command dump:\n{stdout}"
    );

    for removed in [
        "archive",
        "abandon",
        "integrations",
        "repair",
        "timer",
        "daemon",
        "cpitd",
        "usage",
        "agent",
        "locks",
        "sync",
        "workflow",
        "mission",
        "search",
    ] {
        assert!(
            !stdout.lines().any(|line| {
                let command = line.trim_start();
                command == removed || command.starts_with(&format!("{removed} "))
            }),
            "removed command {removed} is still visible in help:\n{stdout}"
        );
    }
}

#[test]
fn test_mission_overview_help_distinguishes_plural_and_scoped_views() {
    let dir = tempdir().unwrap();

    let (success, work_help, stderr) = run_atelier_raw(dir.path(), &["work", "--help"]);
    assert!(success, "work help failed: {stderr}");
    assert!(
        work_help.contains("Show the Mission Overview across current missions"),
        "{work_help}"
    );
    assert!(
        work_help.contains("Show one mission dashboard"),
        "{work_help}"
    );

    let (success, missions_help, stderr) =
        run_atelier_raw(dir.path(), &["work", "missions", "--help"]);
    assert!(success, "work missions help failed: {stderr}");
    assert!(
        missions_help.contains("Show the Mission Overview across current missions")
            && missions_help.contains("Include done missions in the Mission Overview"),
        "{missions_help}"
    );

    init_atelier(dir.path());
    let (success, dashboards, stderr) = run_atelier(dir.path(), &["work"]);
    assert!(success, "bare work guidance failed: {stderr}");
    assert!(
        dashboards.contains("missions Flat mission summaries with complete scope counts")
            && dashboards.contains("Compare missions: atelier work missions")
            && dashboards.contains("mission  Live mission orchestration dashboard"),
        "{dashboards}"
    );
}

#[test]
fn test_obsolete_command_surfaces_are_removed_without_aliases() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for args in [
        vec!["mission", "list"],
        vec!["search", "needle"],
        vec!["issue", "table"],
    ] {
        let (success, stdout, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(!success, "{args:?} should be removed:\n{stdout}");
        assert!(
            stderr.contains("unrecognized subcommand"),
            "{args:?} should fail as an unrecognized command, got:\n{stderr}"
        );
    }
}

#[test]
fn test_root_active_pointer_cleanup_commands_are_removed() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["repair", "--help"]);
    assert!(!success, "repair should be removed:\n{stdout}");
    assert!(
        stderr.contains("unrecognized subcommand 'repair'"),
        "{stderr}"
    );

    let (success, stdout, stderr) = run_atelier_raw(
        dir.path(),
        &["abandon", "atelier-test", "--reason", "handoff"],
    );
    assert!(!success, "abandon should be removed:\n{stdout}");
    assert!(
        stderr.contains("unrecognized subcommand 'abandon'"),
        "{stderr}"
    );
}

#[test]
fn test_generic_note_command_rejects_with_record_specific_guidance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["note", "add", "issue", "atelier-missing", "legacy note"],
    );

    assert!(!success, "generic note command should be removed");
    assert!(
        stderr.contains("unrecognized subcommand 'note'"),
        "{stderr}"
    );
    assert!(!stderr.contains("was removed"), "{stderr}");
    assert!(
        !stderr.contains("atelier issue note atelier-missing"),
        "{stderr}"
    );
}

#[test]
fn test_generic_link_command_rejects_with_record_specific_guidance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["link", "add", "issue", "atelier-a", "issue", "atelier-b"],
    );

    assert!(!success, "generic link command should be removed");
    assert!(
        stderr.contains("unrecognized subcommand 'link'"),
        "{stderr}"
    );
    assert!(!stderr.contains("was removed"), "{stderr}");
    assert!(!stderr.contains("atelier issue link"), "{stderr}");
    assert!(!stderr.contains("atelier issue link"), "{stderr}");
    assert!(!stderr.contains("atelier evidence attach"), "{stderr}");
}

#[test]
fn test_evidence_help_hides_predecessor_subcommands() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["evidence", "--help"]);
    assert!(success, "evidence help failed: {stderr}");
    assert!(stdout.contains("record"));
    assert!(stdout.contains("attach"));
    assert!(!stdout.contains("\n  add "));
    assert!(!stdout.contains("\n  capture "));
}

#[test]
fn test_evidence_record_help_shows_issue_targeted_manual_and_command_flows() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(
        dir.path(),
        &["evidence", "record", "--result", "pass", "--help"],
    );
    assert!(success, "evidence record help failed: {stderr}");
    assert!(stdout.contains("issue/<id>"));
    assert!(stdout.contains(
        "atelier evidence record --target issue/<id> --kind validation --result pass \"checked claim X\""
    ));
    assert!(stdout.contains("atelier evidence record --target issue/<id> --kind test -- <command>"));
    assert!(stdout.contains("Use `evidence attach` only when you need to reuse"));
}

#[test]
fn test_graph_command_is_removed() {
    let dir = tempdir().unwrap();

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["graph", "--help"]);
    assert!(!success, "graph help should fail after removal: {stdout}");
    assert!(
        stderr.contains("unrecognized subcommand 'graph'"),
        "{stderr}"
    );
    assert!(!stderr.contains("graph impact"), "{stderr}");
    assert!(!stderr.contains("graph tree"), "{stderr}");
}

#[test]
fn test_mission_create_help_names_generated_sections() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["issue", "create", "--help"]);
    assert!(success, "mission create help failed: {stderr}");

    assert!(stdout.contains("--issue-type <ISSUE_TYPE>"));
    assert!(!stdout.contains("Mission intent/body text; requires --issue-type mission"));
    assert!(!stdout.contains("Constraints section bullet"));
    assert!(!stdout.contains("Risks section bullet"));
    assert!(!stdout.contains("Validation section bullet"));
}

#[test]
fn test_mission_help_exposes_close_with_reason() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["issue", "transition", "--help"]);
    assert!(success, "mission close help failed: {stderr}");
    assert!(stdout.contains("--reason <CLOSE_REASON>"));
    assert!(stdout.contains("Close reason used by transitions that require it"));
    assert!(!stdout.contains("closeout"));
}

#[test]
fn test_root_status_summarizes_checkout_orientation() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(dir.path(), &["issue", "create", "Ready item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(stdout.contains("Atelier Status"));
    assert!(stdout.contains("Ready work:"));
    assert!(stdout.contains("Current work:"));
    assert!(stdout.contains("Current missions:"));
    assert!(stdout.contains("Next Actions"));
    assert!(stdout.contains("Choose ready work"));
    assert!(stdout.contains("Inspect selected work transitions"));
    assert!(!stdout.contains(
        "Start selected work (ready work exists): atelier issue transition <issue-id> start"
    ));
    assert!(!stdout.contains("atelier doctor"));
    assert!(!stdout.contains("workflow validate"));
    assert!(!stdout.contains("Evidence Status"));
    assert!(!stdout.contains("Attached Proof:"));
    assert!(!stdout.contains("Recent Activity"));
    assert!(!stdout.contains("no active mission focus"));
    let removed_ready_command = ["atelier", "issue", "next"].join(" ");
    assert!(!stdout.contains(&removed_ready_command));
    assert!(!stdout.contains("Active roles"), "{stdout}");
    assert!(!stdout.contains("atelier session"));

    let (success, quiet, stderr) = run_atelier(dir.path(), &["--quiet", "status"]);
    assert!(success, "quiet status failed: {stderr}");
    assert!(quiet.contains("work="));
    assert!(quiet.contains("ready="));
}

#[test]
fn test_issue_ready_list_uses_current_workflow_commands() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(dir.path(), &["issue", "create", "Next item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Next item");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["work", "ready"]);
    assert!(success, "issue ready list failed: {stderr}");
    assert!(stdout.contains("Ready Work"));
    assert!(stdout.contains(&issue_id));
    assert!(stdout.contains("Next item"));
    assert!(
        !stdout.contains("session work"),
        "ready list must not suggest removed session workflow:\n{stdout}"
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
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--verbose"]);
    assert!(success, "transition failed: {stderr}");
    assert!(transition_out.contains("Issue Transitions"));
    assert!(
        transition_out.contains("start [allowed]"),
        "{transition_out}"
    );
    assert!(
        transition_out.contains("Requirements: satisfied"),
        "{transition_out}"
    );
    assert!(
        transition_out.contains("block [allowed]"),
        "{transition_out}"
    );
    assert!(
        !transition_out.contains("Planned Actions"),
        "{transition_out}"
    );
    assert!(transition_out.contains("Commands"), "{transition_out}");
    assert!(transition_out.contains(&format!("atelier issue transition {issue_id} start")));
    let git_after = git_status_short(dir.path());
    assert_eq!(
        git_before, git_after,
        "no-argument transition should not dirty the worktree:\nbefore:\n{git_before}\nafter:\n{git_after}"
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
}

#[test]
fn test_issue_transition_options_and_blocked_attempts_do_not_write() {
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
    move_epic_directly_to_ready(dir.path(), &issue_id);
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "options read-only baseline");

    let git_before = git_status_short(dir.path());
    let (success, options_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--verbose"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(options_out.contains("Issue Transitions"), "{options_out}");
    let git_after = git_status_short(dir.path());
    assert_eq!(
        git_before, git_after,
        "no-argument transition should leave the git worktree unchanged:\nbefore:\n{git_before}\nafter:\n{git_after}"
    );

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "start failed: {stderr}");

    let record_before = read_canonical_record(dir.path(), "issues", &issue_id);
    let git_before = git_status_short(dir.path());
    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "close"]);
    assert!(!success, "close should be blocked by its required fields");
    assert!(stdout.contains("Blockers"), "{stdout}");
    assert!(
        stderr.contains("missing required field close_reason"),
        "{stderr}"
    );
    assert_eq!(
        record_before,
        read_canonical_record(dir.path(), "issues", &issue_id),
        "blocked transition should not rewrite the issue record"
    );
    assert_eq!(
        git_before,
        git_status_short(dir.path()),
        "blocked transition should leave the git worktree unchanged"
    );
}

#[test]
fn test_root_start_applies_workflow_transition_and_records_active_work() {
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
        start_out.contains(&format!("git switch -c task/{issue_id}")),
        "{start_out}"
    );

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(
        issue_text.contains("status: \"in_progress\""),
        "{issue_text}"
    );

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(status_out.contains("Current work:  1 issue(s)"));
    assert!(status_out.contains(&format!("{issue_id}")));
}

#[test]
fn test_root_start_allows_multiple_current_work_issues_in_same_worktree() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Shared epic", "--issue-type", "epic"],
    );
    assert!(success, "epic issue create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Shared epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Active item", "--parent", &epic_id],
    );
    assert!(success, "active issue create failed: {stderr}");
    let active_id = issue_id_by_title(dir.path(), "Active item");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Next item", "--parent", &epic_id],
    );
    assert!(success, "next issue create failed: {stderr}");
    let next_id = issue_id_by_title(dir.path(), "Next item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "two startable items");

    let (success, active_out, stderr) = run_atelier(dir.path(), &["start", &active_id]);
    assert!(success, "initial start failed: {stderr}");
    assert!(!active_out.contains("Session:"), "{active_out}");
    assert!(
        active_out.contains(&format!("git switch -c epic/{epic_id}")),
        "{active_out}"
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &next_id]);
    assert!(success, "second current work issue should start: {stderr}");
    assert!(
        stdout.contains(&format!("Applied transition start to {next_id}")),
        "{stdout}"
    );
    assert!(stdout.contains("no git operations"), "{stdout}");

    let next_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &next_id)).unwrap();
    assert!(
        next_text.contains("status: \"in_progress\""),
        "started issue must transition to in_progress:\n{next_text}"
    );
    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(
        status_out.contains("Current work:  2 issue(s)"),
        "{status_out}"
    );
    assert!(status_out.contains(&format!("{active_id}")));
    assert!(status_out.contains(&format!("{next_id}")));
}

#[test]
fn test_root_start_same_issue_reports_already_started_work() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Restarted item"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Restarted item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "restartable item");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "first start failed: {stderr}");
    let record_before = read_canonical_record(dir.path(), "issues", &issue_id);
    let (success, restart_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(!success, "second start should be guarded:\n{restart_out}");
    assert!(
        stderr.contains("Transition 'start' is not available from status 'in_progress'"),
        "{stderr}"
    );
    assert_eq!(
        record_before,
        read_canonical_record(dir.path(), "issues", &issue_id),
        "unavailable transition should not rewrite the issue record"
    );
}

#[test]
fn test_removed_abandon_rejects_and_starting_another_issue_is_allowed() {
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

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &first_id, "start"]);
    assert!(success, "first start failed: {stderr}");
    let (success, abandon_out, stderr) =
        run_atelier(dir.path(), &["abandon", &first_id, "--reason", "switching"]);
    assert!(!success, "abandon should be removed:\n{abandon_out}");
    assert!(
        stderr.contains("unrecognized subcommand 'abandon'"),
        "{stderr}"
    );

    let (success, second_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &second_id, "start"]);
    assert!(success, "second start should not require abandon: {stderr}");
    assert!(
        second_out.contains(&format!("Applied transition start to {second_id}")),
        "{second_out}"
    );
    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(
        status_out.contains("Current work:  2 issue(s)"),
        "{status_out}"
    );
    assert!(status_out.contains(&format!("{first_id}")));
    assert!(status_out.contains(&format!("{second_id}")));
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
            "        description: Start work.\n        validators:\n        - blockers.transitive_none_open\n",
            "        description: Start work.\n        validators: [evidence.attached]\n",
        ),
    )
    .unwrap();
    commit_all(dir.path(), "validator-gated start policy");

    let (success, options_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--verbose"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(options_out.contains("start [blocked]"), "{options_out}");
    assert!(
        options_out.contains("expected at least 1 fresh passing evidence"),
        "{options_out}"
    );
    assert!(
        options_out.contains("Hint: Record fresh proof"),
        "{options_out}"
    );

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(!success, "root start should fail when validators block it");
    assert!(stdout.contains("Blockers"), "{stdout}");
    assert!(
        stdout.contains("expected at least 1 fresh passing evidence"),
        "{stdout}"
    );
    assert!(stdout.contains("Hint: Record fresh proof"), "{stdout}");
    assert!(stderr.contains("evidence.attached"), "{stderr}");

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(issue_text.contains("status: \"todo\""), "{issue_text}");
}

#[test]
fn test_issue_transition_blocked_attempt_leaves_canonical_records_unchanged() {
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
    move_epic_directly_to_ready(dir.path(), &issue_id);
    add_close_validator(dir.path(), "git.worktree_clean");

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
    let record_before = read_canonical_record(dir.path(), "issues", &issue_id);

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
    assert!(!success, "close should be blocked by its validators");
    assert!(stdout.contains("Blockers"), "{stdout}");
    assert!(stderr.contains("git.worktree_clean"), "{stderr}");
    assert!(stderr.contains("blocked"), "{stderr}");

    assert_eq!(
        record_before,
        read_canonical_record(dir.path(), "issues", &issue_id),
        "blocked transition should not rewrite the issue record"
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
fn test_issue_close_uses_terminal_transition_and_clears_active_work() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Closable workflow item",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Closable workflow item");
    move_epic_directly_to_ready(dir.path(), &issue_id);
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "workflow-ready close item");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "start failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_review"],
    );
    assert!(success, "request_review failed: {stderr}");
    attach_issue_pass_evidence(dir.path(), &issue_id);
    commit_all(dir.path(), "ready for close");
    let head_before = git_rev_parse(dir.path(), "HEAD");
    let branch_before = git_current_branch(dir.path());

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
    assert!(success, "issue transition close failed: {stderr}");
    assert!(
        close_out.contains("Applied transition close"),
        "{close_out}"
    );
    assert!(close_out.contains("To:       done"), "{close_out}");
    assert_eq!(
        git_rev_parse(dir.path(), "HEAD"),
        head_before,
        "close must not commit"
    );
    assert_eq!(
        git_current_branch(dir.path()),
        branch_before,
        "close must not switch branches"
    );
    assert_eq!(
        git_status_short(dir.path()).trim(),
        format!("M .atelier/issues/{issue_id}.md"),
        "close must write only the issue record"
    );

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(issue_text.contains("status: \"done\""), "{issue_text}");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(status_out.contains("Current work:  none"), "{status_out}");
    assert!(
        issue_text.contains("## Close Reason\n\ndone"),
        "{issue_text}"
    );
}

#[test]
fn test_removed_issue_close_command_rejects_to_and_reason_flags() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Removed close command item"],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Removed close command item");

    let (success, _stdout, stderr) = run_atelier(
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
    assert!(
        !success,
        "removed issue close command should reject --to usage"
    );
    assert!(
        stderr.contains("unrecognized subcommand 'close'"),
        "{stderr}"
    );
}

#[test]
fn test_removed_abandon_preserves_issue_status() {
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

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "start failed: {stderr}");

    let (success, abandon_out, stderr) = run_atelier(
        dir.path(),
        &["abandon", &issue_id, "--reason", "paused for handoff"],
    );
    assert!(!success, "abandon should be removed:\n{abandon_out}");
    assert!(
        stderr.contains("unrecognized subcommand 'abandon'"),
        "{stderr}"
    );

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(
        issue_text.contains("status: \"in_progress\""),
        "{issue_text}"
    );

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(
        status_out.contains("Current work:  1 issue(s)"),
        "{status_out}"
    );
    assert!(status_out.contains(&format!("{issue_id}")));
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

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "transition", &issue_id]);
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
    let (success, _, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(
        !success,
        "loading records should reject legacy status before transition handling"
    );
    assert!(stderr.contains("workflow_issue_status_invalid"), "{stderr}");
    assert!(stderr.contains("status 'open'"), "{stderr}");
    assert!(stderr.contains("not valid for workflow 'work'"), "{stderr}");
}

#[test]
fn test_issue_transition_options_render_guidance_and_exact_command() {
    let dir = tempdir().unwrap();
    init_atelier_without_workflow(dir.path());
    migrate_default_issue_workflow(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Guided transition item",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Guided transition item");
    move_epic_directly_to_ready(dir.path(), &issue_id);
    for args in [
        vec!["issue", "transition", &issue_id, "start"],
        vec!["issue", "transition", &issue_id, "request_review"],
    ] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "transition {:?} failed: {stderr}", args);
    }

    let (success, options_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--verbose"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(options_out.contains("close [blocked]"), "{options_out}");
    assert!(options_out.contains("  Decision: blocked"), "{options_out}");
    assert!(options_out.contains("Description"), "{options_out}");
    assert!(
        options_out.contains("Done claim; commit it on the work branch before review."),
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
        "show",
        "transition",
        "update",
        "note",
        "link",
        "unlink",
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
        "close",
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
    assert!(update_help.contains("--status <STATUS>"));
    assert!(!update_help.contains("--description"));
}

#[test]
fn test_non_lifecycle_issue_flows_use_explicit_homes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Source graph item",
            "--issue-type",
            "epic",
        ],
    );
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
    let (success, impact_out, stderr) = run_atelier(dir.path(), &["issue", "show", &source_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(impact_out.contains("Impact"));
    assert!(impact_out.contains("Target graph item"));

    let (success, status_out, stderr) = run_atelier(dir.path(), &["issue", "show", &source_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(status_out.contains("Source graph item"));
    assert!(status_out.contains("Target graph item"));

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
}

#[test]
fn test_issue_status_includes_linked_issue_hierarchy() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Relationship Objective",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "objective create failed: {stderr}");
    let objective_id = issue_id_by_title(dir.path(), "Relationship Objective");
    let objective_id = objective_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Linked Epic", "--issue-type", "epic"],
    );
    assert!(success, "linked epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Linked Epic");
    let epic_id = epic_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Linked Child", "--parent", epic_id],
    );
    assert!(success, "child issue create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Linked Child");
    let child_id = child_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", objective_id, epic_id, "--role", "advances"],
    );
    assert!(success, "issue link failed: {stderr}");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["issue", "show", objective_id]);
    assert!(success, "issue status failed: {stderr}");
    assert!(status_out.contains("Objective Rollup"));
    assert!(status_out.contains("Relationship Objective"));
    assert!(status_out.contains("Relationship Objective"));
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
        vec!["issue", "close", "1", "--reason", "done"],
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
fn test_generic_link_rejection_is_plain_unknown_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Target issue"]);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["link", "add", "mission", "atelier-none", "issue", "1"],
    );
    assert!(!success, "generic link command should be removed");
    assert!(
        stderr.contains("unrecognized subcommand 'link'"),
        "{stderr}"
    );
    assert!(!stderr.contains("was removed"), "{stderr}");
    assert!(!stderr.contains("atelier issue link"), "{stderr}");
    assert!(!stderr.contains("atelier issue link"), "{stderr}");
    assert!(!stderr.contains("atelier evidence attach"), "{stderr}");
}

#[test]
fn test_removed_maintenance_delete_is_unknown() {
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
    assert!(!success, "maintenance delete should be removed");
    assert!(
        stderr.contains("unrecognized subcommand 'maintenance'"),
        "{stderr}"
    );
    assert!(!stderr.contains("was removed"), "{stderr}");
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
        vec!["worktree"],
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
fn test_removed_commands_fail_without_compatibility_guidance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for args in [
        vec!["workflow", "check", "--json"],
        vec!["finish"],
        vec!["current-work"],
        vec!["issue", "new", "Replacement test"],
        vec!["work", "start", "atelier-z1p8"],
        vec!["archive", "add", "atelier-z1p8"],
        vec!["session", "status"],
        vec!["timer"],
    ] {
        let (success, stdout, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(!success, "{args:?} unexpectedly succeeded");
        assert!(
            stdout.is_empty(),
            "{args:?} should not execute a compatibility path:\n{stdout}"
        );
        assert!(
            stderr.contains("unrecognized subcommand")
                || stderr.contains("unexpected argument")
                || stderr.contains("Usage:"),
            "{args:?} did not fail through Clap:\n{stderr}"
        );
        assert!(
            !stderr.contains("was removed"),
            "{args:?} emitted compatibility guidance:\n{stderr}"
        );
    }
}
