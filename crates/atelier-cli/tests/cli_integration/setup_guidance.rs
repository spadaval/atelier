use super::*;

// ==================== Init Tests ====================

#[test]
fn test_init_creates_atelier_directory() {
    let dir = tempdir().unwrap();
    let (success, stdout, _) = run_atelier(dir.path(), &["init", "--slug", "atelier"]);

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
fn test_init_policy_defaults_the_merge_strategy_from_its_merge_checks() {
    use atelier_workflow::MergeStrategy;
    let dir = tempdir().unwrap();
    run_atelier(dir.path(), &["init", "--slug", "atelier"]);
    let path = dir.path().join(".atelier/workflow.yaml");
    let starter = std::fs::read_to_string(&path).unwrap();
    let strategy = || {
        atelier_workflow::load(dir.path())
            .unwrap()
            .branch_policy
            .merge_strategy
    };
    // No test check proves a merged tree, so the starter keeps fast-forward.
    assert!(!starter.contains("merge_strategy"), "{starter}");
    assert_eq!(strategy(), MergeStrategy::FastForwardOnly);
    // A shell.check makes merge_commit the default; an explicit choice wins.
    std::fs::write(
        &path,
        format!("{starter}merge_checks:\n  - shell.check: {{ command: 'true' }}\n"),
    )
    .unwrap();
    assert_eq!(strategy(), MergeStrategy::MergeCommit);
    std::fs::write(
        &path,
        format!("{starter}merge_checks:\n  - shell.check: {{ command: 'true' }}\n").replace(
            "base_branch: main\n",
            "base_branch: main\n  merge_strategy: fast_forward_only\n",
        ),
    )
    .unwrap();
    assert_eq!(strategy(), MergeStrategy::FastForwardOnly);
}

#[test]
fn test_init_twice_is_idempotent() {
    let dir = tempdir().unwrap();

    run_atelier(dir.path(), &["init", "--slug", "atelier"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["init", "--slug", "atelier"]);

    assert!(success);
    assert!(stdout.contains("Atelier initialized successfully"));
    assert!(!dir.path().join(".claude").exists());
}

#[test]
fn test_init_slug_comes_from_origin_then_the_main_worktree() {
    let root = tempdir().unwrap();
    let git = |dir: &Path, args: &[&str]| {
        let out = std::process::Command::new("git")
            .current_dir(dir)
            .args(args)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    let main = root.path().join("astromech");
    std::fs::create_dir(&main).unwrap();
    git(&main, &["init", "-q", "-b", "main"]);
    git(
        &main,
        &[
            "-c",
            "user.name=A",
            "-c",
            "user.email=a@example.test",
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "base",
        ],
    );
    let linked = root.path().join("astromech-atelier");
    git(&main, &["worktree", "add", "-q", linked.to_str().unwrap()]);
    let (success, stdout, stderr) = run_atelier(&linked, &["init"]);
    assert!(success, "{stderr}");
    assert!(
        stdout.contains("Project slug: astromech (from the main worktree's directory;"),
        "{stdout}"
    );
    let config = std::fs::read_to_string(linked.join(".atelier/config.toml")).unwrap();
    assert!(config.contains("project_slug = \"astromech\""), "{config}");

    let cloned = root.path().join("local-checkout");
    std::fs::create_dir(&cloned).unwrap();
    git(&cloned, &["init", "-q"]);
    git(
        &cloned,
        &[
            "remote",
            "add",
            "origin",
            "git@github.com:someone/elite-tools.git",
        ],
    );
    let (success, stdout, stderr) = run_atelier(&cloned, &["init"]);
    assert!(success, "{stderr}");
    assert!(
        stdout.contains("Project slug: elite-tools (from the origin remote;"),
        "{stdout}"
    );

    // A new project inside a tracked parent keeps its own directory name.
    let nested = cloned.join("newproj");
    std::fs::create_dir(&nested).unwrap();
    let (success, stdout, stderr) = run_atelier(&nested, &["init"]);
    assert!(success, "{stderr}");
    assert!(
        stdout.contains("Project slug: newproj (from the directory name;"),
        "{stdout}"
    );

    let bare = root.path().join("toolkit.git");
    git(
        root.path(),
        &["init", "-q", "--bare", "-b", "main", bare.to_str().unwrap()],
    );
    git(&main, &["push", "-q", bare.to_str().unwrap(), "HEAD:main"]);
    let bare_linked = root.path().join("toolkit-feature");
    git(
        &bare,
        &[
            "worktree",
            "add",
            "-q",
            bare_linked.to_str().unwrap(),
            "main",
        ],
    );
    let (success, stdout, stderr) = run_atelier(&bare_linked, &["init"]);
    assert!(success, "{stderr}");
    assert!(
        stdout.contains("Project slug: toolkit (from the bare repository's name;"),
        "{stdout}"
    );

    let hidden = root.path().join("hub/.bare");
    git(
        root.path(),
        &[
            "init",
            "-q",
            "--bare",
            "-b",
            "main",
            hidden.to_str().unwrap(),
        ],
    );
    git(
        &main,
        &["push", "-q", hidden.to_str().unwrap(), "HEAD:main"],
    );
    let hub_main = root.path().join("hub/main");
    git(
        &hidden,
        &["worktree", "add", "-q", hub_main.to_str().unwrap(), "main"],
    );
    let (success, stdout, stderr) = run_atelier(&hub_main, &["init"]);
    assert!(success, "{stderr}");
    assert!(
        stdout.contains("Project slug: hub (from the bare repository's name;"),
        "{stdout}"
    );

    // A separate git dir is not a bare repository; the directory name counts.
    let separate = root.path().join("sepwork");
    git(
        root.path(),
        &[
            "init",
            "-q",
            "--separate-git-dir",
            root.path().join("sep.gitdir").to_str().unwrap(),
            separate.to_str().unwrap(),
        ],
    );
    let (success, stdout, stderr) = run_atelier(&separate, &["init"]);
    assert!(success, "{stderr}");
    assert!(
        stdout.contains("Project slug: sepwork (from the directory name;"),
        "{stdout}"
    );

    let plain = root.path().join("plain-dir");
    std::fs::create_dir(&plain).unwrap();
    let (success, stdout, _) = run_atelier(&plain, &["init", "--slug", "chosen"]);
    assert!(
        success && stdout.contains("Project slug: chosen (from --slug;"),
        "{stdout}"
    );
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
        stdout.contains("Override record retention, in days, for this prune pass"),
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
        stdout.contains("no closed owner record association"),
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
                "closed owner {recent_id} is within retention window"
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
            "feature",
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
    let branch = format!("feature/{epic_id}");
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
    // Git and Atelier print resolved paths; macOS TMPDIR is a symlink.
    let resolved = dir.path().canonicalize().unwrap().join(branch);
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
                resolved.display()
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
        registrations.contains(&format!("worktree {}", resolved.display()))
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
            "base_branch: main\n",
            "base_branch: main\n  merge_strategy: squash\n",
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
    assert!(stdout.contains("Records\n-------"), "{stdout}");
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
    assert!(stdout.contains("Records\n-------"), "{stdout}");
    assert!(stdout.contains("Retention: 7 days"), "{stdout}");
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
    assert!(stdout.contains("Records\n-------"), "{stdout}");
    assert!(stdout.contains("Retention: 30 days"), "{stdout}");
    assert!(stdout.contains("Candidates: none"), "{stdout}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "7"]);
    assert!(success, "prune override dry-run failed: {stderr}");
    assert!(stdout.contains("Retention: 7 days"), "{stdout}");
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

#[test]
fn test_prune_rechecks_late_recorded_owner_association() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Late owner"]);
    assert!(success, "{stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 45);
    commit_all(dir.path(), "old record without a branch");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
    assert!(success, "{stderr}");
    assert!(
        stdout.contains(&format!("eligible issue {issue_id}")),
        "{stdout}"
    );
    let branch = "custom/late-owner";
    assert!(Command::new("git")
        .current_dir(dir.path())
        .args(["branch", branch])
        .status()
        .unwrap()
        .success());
    let store = atelier_records::RecordStore::new(dir.path().join(".atelier"));
    let mut record = store.load_issue_by_id(&issue_id).unwrap();
    record.issue.workflow_branch = Some(atelier_core::WorkflowBranch {
        work_branch: branch.into(),
        branch_base: "main".into(),
    });
    store.write_issue_atomic(&record).unwrap();
    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
    assert!(success, "{stderr}");
    assert!(
        stdout.contains(&format!("protected branch {branch}")),
        "{stdout}"
    );
    assert!(
        stdout.contains("owner record has uncommitted changes"),
        "{stdout}"
    );
    commit_all(dir.path(), "record late owner association");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
    assert!(success, "{stderr}");
    assert!(
        stdout.contains(&format!("protected branch {branch}")),
        "{stdout}"
    );
    assert!(stdout.contains("has no upstream"), "{stdout}");
    assert!(
        !stdout.contains("no closed owner record association"),
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
            "base_branch: main\n",
            "base_branch: main\n  merge_strategy: squash\n",
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
            && stderr.contains("no records removed")
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
        &["issue", "create", "Open epic", "--issue-type", "feature"],
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
        stderr.contains("prune --apply requires a clean tracked checkout")
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
fn test_root_help_lists_every_command_and_lifecycle_common_commands() {
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

    let commands: Vec<&str> = stdout
        .split("\nCommands:\n")
        .nth(1)
        .and_then(|section| section.split("\n\n").next())
        .expect("root help lists its commands")
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .collect();
    assert_eq!(
        commands,
        [
            "init", "setup", "status", "work", "issue", "start", "evidence", "review", "land",
            "merge", "bundle", "check", "doctor", "brief", "debrief", "prune", "migrate", "help",
        ],
        "{stdout}"
    );
    // Each description is the command's own about, not a second copy.
    assert!(
        stdout.contains("  init      Initialize atelier in the current directory\n"),
        "{stdout}"
    );
    assert!(!stdout.contains("\n\n\n"), "{stdout}");

    let common_commands: Vec<&str> = stdout
        .split("Common commands:\n")
        .nth(1)
        .expect("root help has common commands")
        .lines()
        .map(str::trim)
        .take_while(|line| !line.is_empty())
        .collect();
    assert_eq!(
        common_commands,
        [
            "atelier status",
            "atelier work ready",
            "atelier work mission <id>",
            "atelier issue show <id>",
            "atelier start <id>",
            "atelier evidence record --target issue/<id> --kind test -- <command>",
            "atelier issue transition <id> close --reason \"...\"",
            "atelier review approve <id> --identity <name> \"...\"",
            "atelier land",
            "atelier merge <id> --check",
        ],
        "{stdout}"
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
            && missions_help
                .contains("Include done missions, and show every mission and unassigned item"),
        "{missions_help}"
    );

    init_atelier(dir.path());
    let (success, bare_work, stderr) = run_atelier(dir.path(), &["work"]);
    assert!(success, "bare work failed: {stderr}");
    assert_eq!(bare_work, work_help, "bare work prints work help");
    for view in ["active ", "all ", "missions "] {
        assert!(
            bare_work
                .lines()
                .any(|line| line.trim_start().starts_with(view)),
            "bare work omits {view}:\n{bare_work}"
        );
    }
    let (success, names, stderr) = run_atelier(dir.path(), &["work", "--quiet"]);
    assert!(success, "quiet bare work failed: {stderr}");
    assert_eq!(
        names,
        "mission\nparent\ndecisions\nready\nhuman\nblocked\nactive\nall\nmissions\n"
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
fn test_root_status_lists_the_top_five_ready_items_without_an_owner() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    for n in 0..7 {
        let (success, _, stderr) =
            run_atelier(dir.path(), &["issue", "create", &format!("Ready item {n}")]);
        assert!(success, "issue create failed: {stderr}");
    }
    let (success, stdout, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(stdout.contains("Ready work:       7"), "{stdout}");
    let ready = stdout
        .split("Ready Work\n----------\n")
        .nth(1)
        .and_then(|rest| rest.split("\n\n").next())
        .unwrap_or_default();
    assert_eq!(
        ready
            .lines()
            .filter(|line| line.contains("Ready item"))
            .count(),
        5,
        "{stdout}"
    );
    assert!(
        ready.contains("2 more ready work items omitted"),
        "{stdout}"
    );
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
    assert!(stdout.contains("Ready Work\n----------\n"), "{stdout}");
    assert!(stdout.contains("Ready item"), "{stdout}");
    assert!(stdout.contains("Current Work\n------------\n"));
    // No mission is open or closed, so no count line names them.
    assert!(!stdout.contains("Current missions:"), "{stdout}");
    assert!(stdout.contains("\nNext\n----\n"));
    assert!(stdout.contains("Choose ready work"));
    let ready_id = issue_out
        .split_whitespace()
        .find(|word| word.starts_with("atelier-"))
        .unwrap();
    assert!(
        stdout.contains(&format!(
            "Inspect the top ready item's transitions: atelier issue transition {ready_id}"
        )),
        "{stdout}"
    );
    assert!(!stdout.contains("<issue-id>"), "{stdout}");
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

/// Placeholders such as `<issue-id>` or `<transition>` in `text`.
fn id_placeholders(text: &str) -> Vec<String> {
    text.split('<')
        .skip(1)
        .filter_map(|rest| rest.split_once('>'))
        .map(|(inner, _)| format!("<{inner}>"))
        .filter(|token| token.ends_with("id>") || token == "<transition>")
        .collect()
}

/// Hints name the real ID wherever the view knows it (atelier-6xxb).
#[test]
fn test_hints_name_real_ids_instead_of_placeholders() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Hint mission", "--issue-type", "mission"],
    );
    assert!(success, "{stderr}");
    let mission = issue_id_by_title(dir.path(), "Hint mission");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Hint work"]);
    assert!(success, "{stderr}");
    let work = issue_id_by_title(dir.path(), "Hint work");
    // A second ready candidate, ranked first by priority, so the views must
    // pick the top one rather than the only one.
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Urgent work", "-p", "high"],
    );
    assert!(success, "{stderr}");
    let urgent = issue_id_by_title(dir.path(), "Urgent work");
    let first_listed = [&mission, &work, &urgent]
        .into_iter()
        .min()
        .unwrap()
        .clone();
    let not_first = [&mission, &work, &urgent]
        .into_iter()
        .filter(|id| **id != first_listed)
        .cloned()
        .collect::<Vec<_>>();

    let mut views = Vec::new();
    for args in [&["status"][..], &["prime"], &["work"]] {
        let (success, stdout, stderr) = run_atelier(dir.path(), args);
        assert!(success, "{args:?}: {stderr}");
        views.push((args.join(" "), stdout));
    }
    for (_, status) in &views[..2] {
        assert!(
            status.contains(&format!(
                "Inspect the top ready item's transitions: atelier issue transition {urgent}\n"
            )),
            "{status}"
        );
        assert!(
            !status.contains(&format!("atelier issue transition {work}")),
            "{status}"
        );
    }
    // Bare `work` is its generated subcommand help (atelier-od8b), so it
    // carries no hint to fill in.
    assert!(views[2].1.contains("Commands:"), "{}", views[2].1);

    let (success, list, stderr) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success, "{stderr}");
    // Inventory lists in ID order.
    assert!(
        list.contains(&format!(
            "Next\n----\n  Inspect the first record: atelier issue show {first_listed}\n"
        )),
        "{list}"
    );
    for id in &not_first {
        assert!(
            !list.contains(&format!("atelier issue show {id}")),
            "{list}"
        );
    }
    views.push(("issue list".into(), list));
    let (success, empty, stderr) = run_atelier(dir.path(), &["issue", "list", "--status", "done"]);
    assert!(success, "{stderr}");
    assert!(
        empty.contains("No issues match the selected filters."),
        "{empty}"
    );
    assert!(
        !empty.contains("Next"),
        "an empty list has no step to offer: {empty}"
    );
    views.push(("issue list --status done".into(), empty));

    for args in [
        vec!["issue", "show", &work],
        vec!["issue", "show", &work, "--full"],
    ] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "{args:?}: {stderr}");
        views.push((args.join(" "), stdout));
    }
    // Transition Readiness names each runnable transition's command once;
    // the footer does not repeat them.
    let full = &views.last().unwrap().1;
    assert_eq!(
        full.matches(&format!("atelier issue transition {work} start\n"))
            .count(),
        1,
        "{full}"
    );
    assert!(!full.contains("Execute"), "{full}");

    let (success, _, error) = run_atelier(
        dir.path(),
        &["issue", "update", &work, "--parent", &mission],
    );
    assert!(!success);
    assert!(
        error.contains(&format!(
            "`atelier issue link {work} {mission} --role advances`"
        )),
        "{error}"
    );
    views.push(("hierarchy error".into(), error));
    // A mission given a mission parent: that parent cannot advance it, so
    // only the source slot stays a placeholder.
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Other mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "{stderr}");
    let other = issue_id_by_title(dir.path(), "Other mission");
    let (success, _, error) = run_atelier(
        dir.path(),
        &["issue", "update", &other, "--parent", &mission],
    );
    assert!(!success);
    assert!(
        error.contains(&format!(
            "`atelier issue link <issue-id> {other} --role advances`"
        )),
        "{error}"
    );
    // A mission given a work parent: that work advances it.
    let (success, _, error) =
        run_atelier(dir.path(), &["issue", "update", &other, "--parent", &work]);
    assert!(!success);
    assert!(
        error.contains(&format!(
            "`atelier issue link {work} {other} --role advances`"
        )),
        "{error}"
    );
    let (success, _, error) = run_atelier(dir.path(), &["issue", "update", &work]);
    assert!(!success);
    assert!(
        error.contains(&format!("`atelier issue note {work} \"...\"`")),
        "{error}"
    );
    views.push(("nothing to update".into(), error));

    for (view, text) in views {
        assert_eq!(
            id_placeholders(&text),
            Vec::<String>::new(),
            "{view}:\n{text}"
        );
    }

    // A new issue has no ID to link yet, so only that slot stays a placeholder.
    let (success, _, error) = run_atelier(
        dir.path(),
        &["issue", "create", "Under the mission", "--parent", &mission],
    );
    assert!(!success);
    assert!(
        error.contains(&format!(
            "create it without --parent, then link mission work with `atelier issue link <new-id> {mission} --role advances`"
        )),
        "{error}"
    );
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
        transition_out.contains("start: allowed"),
        "{transition_out}"
    );
    assert!(
        transition_out.contains("Requirements: satisfied"),
        "{transition_out}"
    );
    assert!(
        transition_out.contains("block: allowed"),
        "{transition_out}"
    );
    assert!(
        !transition_out.contains("Planned Actions"),
        "{transition_out}"
    );
    assert!(transition_out.contains("  Command: "), "{transition_out}");
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
            "feature",
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
    assert!(status_out.contains(&format!(
        "\nCurrent Work\n------------\n  {issue_id} in_progress "
    )));
    assert!(status_out.contains(&issue_id.to_string()));
}

#[test]
fn test_root_start_allows_multiple_current_work_issues_in_same_worktree() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Shared epic", "--issue-type", "feature"],
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
        active_out.contains(&format!("git switch -c feature/{epic_id}")),
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
        section_rows(&status_out, "Current Work").len() == 2,
        "{status_out}"
    );
    assert!(status_out.contains(&active_id.to_string()));
    assert!(status_out.contains(&next_id.to_string()));
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
        section_rows(&status_out, "Current Work").len() == 2,
        "{status_out}"
    );
    assert!(status_out.contains(&first_id.to_string()));
    assert!(status_out.contains(&second_id.to_string()));
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
    assert!(options_out.contains("start: blocked"), "{options_out}");
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
    // The hint is the error's next step, shown once, on stderr.
    assert!(!stdout.contains("Record fresh proof"), "{stdout}");
    assert!(stderr.contains("Next: Record fresh proof"), "{stderr}");
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
            "feature",
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
            "feature",
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
    assert!(
        status_out.contains("\nCurrent Work\n------------\n(none)\n"),
        "{status_out}"
    );
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
        status_out.contains(&format!(
            "\nCurrent Work\n------------\n  {issue_id} in_progress "
        )),
        "{status_out}"
    );
    assert!(status_out.contains(&issue_id.to_string()));
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
    assert!(
        stderr.contains("Invalid transition 'ship_it'. Valid values: block, defer, start"),
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
            "feature",
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
    // Close needs only its reason, so it is allowed and names the flag.
    assert!(options_out.contains("close: allowed"), "{options_out}");
    assert!(
        options_out.contains("  Requirements: needs --reason"),
        "{options_out}"
    );
    assert!(options_out.contains("  Decision: allowed"), "{options_out}");
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
    assert!(!update_help.contains("--status"));
    assert!(update_help.contains("--body <BODY>"));
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
            "feature",
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
    let (success, impact_out, stderr) =
        run_atelier(dir.path(), &["issue", "show", &source_id, "--full"]);
    assert!(success, "issue show failed: {stderr}");
    assert!(impact_out.contains("Impact"));
    assert!(impact_out.contains("Target graph item"));

    let (success, status_out, stderr) =
        run_atelier(dir.path(), &["issue", "show", &source_id, "--full"]);
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
    let (success, show_out, stderr) =
        run_atelier(dir.path(), &["issue", "show", &source_id, "--full"]);
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
            "mission",
        ],
    );
    assert!(success, "objective create failed: {stderr}");
    let objective_id = issue_id_by_title(dir.path(), "Relationship Objective");
    let objective_id = objective_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Linked Epic", "--issue-type", "feature"],
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
        &["issue", "link", objective_id, epic_id, "--role", "related"],
    );
    assert!(success, "issue link failed: {stderr}");

    let (success, status_out, stderr) =
        run_atelier(dir.path(), &["issue", "show", objective_id, "--full"]);
    assert!(success, "issue status failed: {stderr}");
    assert!(status_out.contains("Mission Rollup"));
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

#[test]
fn blocked_start_names_the_record_type_not_mission() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let create = |title: &str| {
        let (success, stdout, stderr) = run_atelier(
            dir.path(),
            &[
                "-q",
                "issue",
                "create",
                title,
                "--body",
                "## Goal\n\nWork.\n\n## Done when\n\nDone.",
            ],
        );
        assert!(success, "{stderr}");
        stdout.trim().to_string()
    };
    let task = create("Blocked task");
    let blocker = create("Open blocker");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &task, &blocker, "--role", "blocked_by"],
    );
    assert!(success, "{stderr}");
    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &task, "start"]);
    assert!(!success, "{stdout}");
    let output = format!("{stdout}{stderr}");
    assert!(
        output.contains(&format!(
            "task {task} has direct blocker path {task} -> {blocker}"
        )),
        "{output}"
    );
    assert!(!output.contains("mission"), "{output}");
}

#[test]
fn test_prune_keeps_decision_and_research_records_and_check_explains_pruned_links() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    let create = |args: &[&str]| {
        let (success, stdout, stderr) = run_atelier(dir.path(), args);
        assert!(success, "{args:?}: {stderr}");
        stdout.trim().to_string()
    };
    let plain = create(&["-q", "issue", "create", "Routine work"]);
    let research = create(&[
        "-q", "issue", "create", "Toolbox survey", "--body",
        "## Goal\n\nSurvey.\n\n## Done when\n\nSurveyed.\n\n## Background\n\nTool-by-tool findings.",
    ]);
    let decision = create(&[
        "-q",
        "issue",
        "create",
        "Pick the signer",
        "--issue-type",
        "decision",
        "--body",
        "## Question\n\nWhich signer?\n\n## Decision\n\nSigstore.",
    ]);
    for id in [&plain, &research, &decision] {
        make_issue_terminal_before_retention(dir.path(), id, 30);
    }
    commit_all(dir.path(), "old terminal records");

    let (success, preview, stderr) = run_atelier(dir.path(), &["prune"]);
    assert!(success, "{stderr}");
    for (id, expected) in [
        (&research, "keeps research content (Background)"),
        (&decision, "keeps decision content"),
    ] {
        let line = preview
            .lines()
            .find(|line| line.contains(id.as_str()))
            .expect(&preview);
        assert!(
            line.contains("protected") && line.contains(expected),
            "{line}"
        );
    }
    let line = preview
        .lines()
        .find(|line| line.contains(plain.as_str()))
        .expect(&preview);
    assert!(line.contains("eligible"), "{line}");

    let (success, _, stderr) = run_atelier(dir.path(), &["prune", "--apply"]);
    assert!(success, "{stderr}");
    assert!(!canonical_issue_path(dir.path(), &plain).exists());
    assert!(canonical_issue_path(dir.path(), &research).exists());
    commit_all(dir.path(), "prune routine work");
    let pruned_in = git_rev_parse(dir.path(), "HEAD");

    // A record from elsewhere that still links the pruned record gets a
    // check error naming the prune and the recovery.
    let late = create(&["-q", "issue", "create", "Late link"]);
    edit_canonical_issue(dir.path(), &late, |markdown| {
        markdown.replace(
            "  relates: []",
            &format!("  relates:\n  - kind: \"issue\"\n    id: \"{plain}\"\n    type: \"related\""),
        )
    });
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    let output = format!("{stdout}{stderr}");
    assert!(!success, "{output}");
    assert!(
        output.contains(&format!(
            "{plain} was deleted in commit {}",
            &pruned_in[..7]
        )) && output.contains(&format!(
            "git restore --source={}^ -- ':/.atelier/issues/{plain}.md'",
            &pruned_in[..7]
        )),
        "{output}"
    );
    // The restore command works from a subdirectory too.
    let subdir = dir.path().join("crates");
    fs::create_dir_all(&subdir).unwrap();
    let status = std::process::Command::new("git")
        .current_dir(&subdir)
        .args([
            "restore",
            &format!("--source={}^", &pruned_in[..7]),
            "--",
            &format!(":/.atelier/issues/{plain}.md"),
        ])
        .status()
        .unwrap();
    assert!(status.success());
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(success, "{stdout}{stderr}");
    // A record still committed but missing from the checkout gets git restore,
    // and commands that load the records lightly explain it the same way.
    commit_all(dir.path(), "restore the record and keep the link");
    fs::remove_file(canonical_issue_path(dir.path(), &plain)).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["review", "list"]);
    assert!(
        !success && stderr.contains("is committed at HEAD but missing"),
        "{stderr}"
    );
}

/// The rows under a status section heading, up to the next blank line.
fn section_rows<'a>(output: &'a str, title: &str) -> Vec<&'a str> {
    output
        .split(&format!("\n{title}\n{}\n", "-".repeat(title.len())))
        .nth(1)
        .unwrap_or_default()
        .lines()
        .take_while(|line| !line.is_empty())
        .collect()
}

/// Every view's next-step footer is a `Next` section of `  <label>: <command>`
/// lines; the headings and hint prefixes it replaced never come back.
#[test]
fn rendered_views_use_one_next_footer_and_no_retired_headings() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    let (success, init_out, stderr) = run_atelier(dir.path(), &["init"]);
    assert!(success, "init failed: {stderr}");
    let mut outputs = vec![("init".to_string(), init_out)];
    let mut run = |args: Vec<&str>| {
        let (_, stdout, stderr) = run_atelier(dir.path(), &args);
        outputs.push((args.join(" "), format!("{stdout}{stderr}")));
    };
    run(vec![
        "issue",
        "create",
        "Footer mission",
        "--issue-type",
        "mission",
    ]);
    run(vec!["issue", "create", "Footer work"]);
    run(vec!["issue", "create", "Footer blocker"]);
    let mission_id = issue_id_by_title(dir.path(), "Footer mission");
    let work_id = issue_id_by_title(dir.path(), "Footer work");
    let blocker_id = issue_id_by_title(dir.path(), "Footer blocker");
    commit_all(dir.path(), "footer fixture");
    attach_issue_pass_evidence(dir.path(), &blocker_id);
    run(vec![
        "issue",
        "link",
        &mission_id,
        &work_id,
        "--role",
        "advances",
    ]);
    run(vec![
        "issue",
        "link",
        &work_id,
        &blocker_id,
        "--role",
        "blocked_by",
    ]);
    run(vec!["issue", "update", &blocker_id, "--priority", "high"]);
    commit_all(dir.path(), "footer links");
    run(vec!["issue", "show", &work_id, "--full"]);
    run(vec!["issue", "list"]);
    run(vec!["issue", "transition", &work_id]);
    run(vec!["issue", "transition", &work_id, "start"]);
    run(vec!["issue", "transition", &blocker_id, "start"]);
    run(vec!["work"]);
    run(vec!["work", "missions"]);
    run(vec!["work", "mission", &mission_id]);
    run(vec!["status"]);
    run(vec!["evidence", "list"]);
    run(vec!["prune"]);
    let fixture = workspace_root().join("docs/spec/bundle/fixtures/valid-bundle.json");
    run(vec!["bundle", "apply", fixture.to_str().unwrap(), "--yes"]);

    let started = format!("issue transition {blocker_id} start");
    let footer_views = [
        "init",
        "issue create",
        "issue link",
        "issue update",
        "issue show",
        "issue list",
        started.as_str(),
        // Bare `work` prints its help; the work views end in a footer.
        "work mission",
        "status",
        "evidence list",
        "prune",
        "bundle apply",
    ];
    for (command, output) in &outputs {
        for retired in [
            "Next Commands",
            "Next Actions",
            "Drill Down",
            "Drill down",
            "Next steps",
            "Commands\n--------",
            "Hint: Next:",
            "\n  Next:",
            "     # ",
        ] {
            assert!(
                !output.contains(retired),
                "`{command}` printed retired {retired:?}:\n{output}"
            );
        }
        if footer_views.iter().any(|view| command.contains(view)) {
            let footer = output
                .split_once("\nNext\n----\n")
                .map(|(_, footer)| footer)
                .unwrap_or_else(|| panic!("`{command}` has no Next footer:\n{output}"));
            for line in footer.lines().take_while(|line| !line.is_empty()) {
                let (label, command_text) = line
                    .strip_prefix("  ")
                    .and_then(|line| line.split_once(": "))
                    .unwrap_or_else(|| {
                        panic!("`{command}` footer line {line:?} is not `  <label>: <command>`")
                    });
                assert!(!label.is_empty() && !command_text.is_empty(), "{line:?}");
            }
        }
    }
    let missions = &outputs
        .iter()
        .find(|(command, _)| command == "work missions")
        .unwrap()
        .1;
    assert!(
        missions.contains(&format!("  More info: atelier work mission {mission_id}")),
        "{missions}"
    );
    let blocked_start = &outputs
        .iter()
        .find(|(command, _)| *command == format!("issue transition {work_id} start"))
        .unwrap()
        .1;
    assert!(
        blocked_start.contains(&format!(
            "Next: Inspect the blocker with `atelier issue show {blocker_id}`"
        )),
        "{blocked_start}"
    );
    let output_of = |wanted: String| {
        &outputs
            .iter()
            .find(|(command, _)| *command == wanted)
            .unwrap_or_else(|| panic!("no output for `{wanted}`"))
            .1
    };
    let full = output_of(format!("issue show {work_id} --full"));
    let readiness = full
        .split_once("Transition Readiness\n")
        .map(|(_, rest)| rest.split("\n\n").next().unwrap())
        .unwrap_or_else(|| panic!("no Transition Readiness:\n{full}"));
    assert!(
        readiness.contains(&format!(
            "\n    Command: atelier issue transition {work_id} block\n"
        )),
        "{readiness}"
    );
    assert!(
        !readiness
            .lines()
            .any(|line| line.starts_with("    atelier ")),
        "bare command line in Transition Readiness:\n{readiness}"
    );
    // A transition's command sits among its own fields, never after the
    // Failed Requirements list, where it would read as another requirement.
    let options = output_of(format!("issue transition {work_id}"));
    assert!(
        options.contains(&format!(
            "start: blocked\n  From: todo, blocked\n  To:   in_progress\n  Command: atelier issue transition {work_id} start\n"
        )),
        "{options}"
    );
    let lines = options.lines().collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate() {
        if line.starts_with("  Command: ") {
            assert!(
                lines[index - 1].starts_with("  To:"),
                "Command: line is not among its option's fields:\n{options}"
            );
        }
    }
}

#[test]
fn test_removed_log_flags_are_rejected_and_absent_from_help() {
    let dir = tempdir().unwrap();
    for args in [
        &["--log-level", "debug", "status"][..],
        &["--log-level", "bogus", "status"][..],
        &["--log-format", "xml", "status"][..],
        &["status", "--log-format", "json"][..],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
            .current_dir(dir.path())
            .args(args)
            .output()
            .expect("Failed to execute atelier");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(2), "{args:?}: {stderr}");
        assert!(stderr.contains("unexpected argument"), "{args:?}: {stderr}");
    }
    for help in [&["--help"][..], &["init", "--help"][..]] {
        let (success, stdout, stderr) = run_atelier_raw(dir.path(), help);
        assert!(success, "{help:?} failed: {stderr}");
        assert!(
            !stdout.contains("--log-"),
            "{help:?} lists log flags:\n{stdout}"
        );
    }
}
