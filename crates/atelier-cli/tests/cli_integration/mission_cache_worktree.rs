use super::*;

fn canonical_tree_snapshot(
    state_dir: &std::path::Path,
) -> std::collections::BTreeMap<std::path::PathBuf, Vec<u8>> {
    fn collect(
        root: &std::path::Path,
        dir: &std::path::Path,
        snapshot: &mut std::collections::BTreeMap<std::path::PathBuf, Vec<u8>>,
    ) {
        let mut entries = std::fs::read_dir(dir)
            .unwrap()
            .map(|entry| entry.unwrap())
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let relative = path.strip_prefix(root).unwrap();
            if relative.components().count() == 1
                && matches!(
                    relative.to_str(),
                    Some("runtime" | "cache" | "locks" | "diagnostics")
                )
            {
                continue;
            }
            if path.is_dir() {
                collect(root, &path, snapshot);
            } else {
                snapshot.insert(relative.to_path_buf(), std::fs::read(path).unwrap());
            }
        }
    }

    let mut snapshot = std::collections::BTreeMap::new();
    collect(state_dir, state_dir, &mut snapshot);
    snapshot
}

fn wait_for_test_marker(path: &std::path::Path) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !path.exists() {
        assert!(
            std::time::Instant::now() < deadline,
            "timed out waiting for {}",
            path.display()
        );
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn assert_bulk_process_holds_exclusive_lock(state_dir: &std::path::Path) {
    use fs2::FileExt;
    use std::fs::OpenOptions;

    let lock_path = state_dir.join(atelier_records::mutation_lock::CANONICAL_MUTATION_LOCK_PATH);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(lock_path)
        .unwrap();
    assert!(
        FileExt::try_lock_shared(&file).is_err(),
        "bulk process did not retain its exclusive canonical transaction"
    );
}

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
    move_reviewed_mission_to_ready(dir, &mission_id);
    mission_id
}

fn move_mission_to_ready(dir: &std::path::Path, mission_id: &str) {
    move_reviewed_mission_to_ready(dir, mission_id);
}

fn set_dependency_fixture_status(dir: &std::path::Path, issue_id: &str, status: &str) {
    edit_canonical_issue(dir, issue_id, |markdown| {
        replace_front_matter_scalar(&markdown, "status", status)
    });
}

fn configure_dependency_fixture_terminal_status(dir: &std::path::Path) {
    let policy_path = dir.join(".atelier/workflow.yaml");
    let policy = std::fs::read_to_string(&policy_path).unwrap();
    let policy = policy.replace(
        "  done:\n    category: done\n",
        "  done:\n    category: done\n  accepted:\n    category: done\n",
    );
    let policy = policy.replacen(
        "  task:\n    applies_to: [bug, feature, task]\n    initial_status: todo\n    done_statuses: [done]",
        "  task:\n    applies_to: [bug, feature, task]\n    initial_status: todo\n    done_statuses: [accepted]",
        1,
    );
    std::fs::write(policy_path, policy).unwrap();
}

#[test]
fn test_mission_start_requires_cycle_safe_transitive_dependency_closure() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    configure_dependency_fixture_terminal_status(dir.path());

    let mission_id = create_mission_fixture(dir.path(), "Dependency gated mission");
    for title in [
        "Direct mission prerequisite",
        "Transitive mission prerequisite",
        "Unrelated internal mission work",
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", title]);
        assert!(success, "issue fixture create failed for {title}: {stderr}");
    }
    let direct_id = issue_id_by_title(dir.path(), "Direct mission prerequisite");
    let transitive_id = issue_id_by_title(dir.path(), "Transitive mission prerequisite");
    let internal_id = issue_id_by_title(dir.path(), "Unrelated internal mission work");
    for args in [
        vec![
            "issue",
            "link",
            &mission_id,
            &direct_id,
            "--role",
            "blocked_by",
        ],
        vec![
            "issue",
            "link",
            &direct_id,
            &transitive_id,
            "--role",
            "blocked_by",
        ],
        vec![
            "issue",
            "link",
            &mission_id,
            &internal_id,
            "--role",
            "advances",
        ],
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "dependency fixture link failed: {stderr}");
    }
    approve_current_mission_revision(dir.path(), &mission_id);
    commit_all(dir.path(), "mission dependency closure fixture");

    let (success, direct_options, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &mission_id, "--verbose"],
    );
    assert!(success, "mission transition options failed: {stderr}");
    assert!(
        direct_options.contains("start [blocked]"),
        "{direct_options}"
    );
    assert!(
        direct_options.contains(&format!("{mission_id} -> {direct_id}")),
        "missing direct blocking path:\n{direct_options}"
    );

    let (success, rejected, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(
        !success,
        "mission start should reject an open direct blocker"
    );
    assert!(
        rejected.contains(&format!("{mission_id} -> {direct_id}")),
        "{rejected}\n{stderr}"
    );
    commit_all(dir.path(), "record rejected direct mission start");

    set_dependency_fixture_status(dir.path(), &direct_id, "accepted");
    set_dependency_fixture_status(dir.path(), &transitive_id, "done");
    commit_all(dir.path(), "direct mission prerequisite terminal");
    let (success, transitive_options, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &mission_id, "--verbose"],
    );
    assert!(success, "mission transition options failed: {stderr}");
    assert!(
        transitive_options.contains("start [blocked]"),
        "{transitive_options}"
    );
    assert!(
        transitive_options.contains(&format!("{mission_id} -> {direct_id} -> {transitive_id}")),
        "missing transitive blocking path:\n{transitive_options}"
    );

    let (success, rejected, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(
        !success,
        "mission start should reject an open transitive blocker"
    );
    assert!(
        rejected.contains(&format!("{mission_id} -> {direct_id} -> {transitive_id}")),
        "{rejected}\n{stderr}"
    );
    commit_all(dir.path(), "record rejected transitive mission start");

    set_dependency_fixture_status(dir.path(), &transitive_id, "accepted");
    commit_all(dir.path(), "mission dependency closure terminal");
    let (success, started, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(success, "closed dependency mission start failed: {stderr}");
    assert!(started.contains("Applied transition start"), "{started}");
    assert!(
        read_canonical_record(dir.path(), "issues", &internal_id).contains("status: \"todo\""),
        "unrelated internal mission work should remain incomplete"
    );
}

#[test]
fn test_issue_ready_work_and_direct_start_require_transitive_dependency_closure() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    configure_dependency_fixture_terminal_status(dir.path());

    for title in [
        "Dependency gated issue",
        "Direct issue prerequisite",
        "Transitive issue prerequisite",
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", title]);
        assert!(success, "issue fixture create failed for {title}: {stderr}");
    }
    let issue_id = issue_id_by_title(dir.path(), "Dependency gated issue");
    let direct_id = issue_id_by_title(dir.path(), "Direct issue prerequisite");
    let transitive_id = issue_id_by_title(dir.path(), "Transitive issue prerequisite");
    for args in [
        vec![
            "issue",
            "link",
            &issue_id,
            &direct_id,
            "--role",
            "blocked_by",
        ],
        vec![
            "issue",
            "link",
            &direct_id,
            &transitive_id,
            "--role",
            "blocked_by",
        ],
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "dependency fixture link failed: {stderr}");
    }
    commit_all(dir.path(), "open issue dependency closure fixture");

    let (success, ready, stderr) = run_atelier(dir.path(), &["--quiet", "work", "ready"]);
    assert!(success, "work ready failed: {stderr}");
    assert!(
        !ready.lines().any(|line| line == issue_id),
        "directly blocked issue was reported ready:\n{ready}"
    );
    let (success, rejected, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(
        !success,
        "direct start should reject an open direct blocker"
    );
    assert!(
        rejected.contains(&format!("{issue_id} -> {direct_id}")),
        "{rejected}\n{stderr}"
    );
    commit_all(dir.path(), "record rejected direct issue start");

    set_dependency_fixture_status(dir.path(), &direct_id, "accepted");
    set_dependency_fixture_status(dir.path(), &transitive_id, "done");
    commit_all(dir.path(), "issue dependency closure fixture");

    let (success, ready, stderr) = run_atelier(dir.path(), &["--quiet", "work", "ready"]);
    assert!(success, "work ready failed: {stderr}");
    assert!(
        !ready.lines().any(|line| line == issue_id),
        "transitively blocked issue was reported ready:\n{ready}"
    );

    let (success, rejected, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(
        !success,
        "direct start should reject an open transitive blocker"
    );
    assert!(
        rejected.contains(&format!("{issue_id} -> {direct_id} -> {transitive_id}")),
        "{rejected}\n{stderr}"
    );
    commit_all(dir.path(), "record rejected transitive issue start");

    set_dependency_fixture_status(dir.path(), &transitive_id, "accepted");
    commit_all(dir.path(), "issue dependency closure terminal");
    let (success, ready, stderr) = run_atelier(dir.path(), &["--quiet", "work", "ready"]);
    assert!(success, "work ready failed: {stderr}");
    assert!(
        ready.lines().any(|line| line == issue_id),
        "dependency-ready issue was omitted:\n{ready}"
    );
    let (success, started, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "dependency-ready issue start failed: {stderr}");
    assert!(started.contains("Applied transition start"), "{started}");
}

#[test]
fn test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let long_title = "X".repeat(100);

    for args in [
        vec![
            "issue",
            "create",
            "Overview mission",
            "--issue-type",
            "mission",
            "--priority",
            "high",
        ],
        vec![
            "issue",
            "create",
            long_title.as_str(),
            "--issue-type",
            "mission",
            "--priority",
            "medium",
        ],
        vec![
            "issue",
            "create",
            "Overview epic",
            "--issue-type",
            "epic",
            "--priority",
            "medium",
        ],
        vec!["issue", "create", "Direct mission work"],
        vec!["issue", "create", "Unassigned work"],
        vec!["issue", "create", "Epic blocker"],
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "fixture command {args:?} failed: {stderr}");
    }

    let mission_id = issue_id_by_title(dir.path(), "Overview mission");
    let long_title_mission_id = issue_id_by_title(dir.path(), &long_title);
    let epic_id = issue_id_by_title(dir.path(), "Overview epic");
    let direct_id = issue_id_by_title(dir.path(), "Direct mission work");
    let blocker_id = issue_id_by_title(dir.path(), "Epic blocker");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Collapsed epic child",
            "--parent",
            &epic_id,
        ],
    );
    assert!(success, "child fixture create failed: {stderr}");

    for args in [
        vec!["issue", "link", &mission_id, &epic_id, "--role", "advances"],
        vec![
            "issue",
            "link",
            &mission_id,
            &direct_id,
            "--role",
            "advances",
        ],
        vec![
            "issue",
            "link",
            &epic_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "fixture command {args:?} failed: {stderr}");
    }

    let (success, overview, stderr) = run_atelier(dir.path(), &["work", "missions"]);
    assert!(success, "work missions failed: {stderr}");
    assert!(
        overview.starts_with("Mission Overview\n================"),
        "{overview}"
    );
    assert!(
        overview.contains(&format!(
            "{mission_id}  todo  high  Overview mission\n  Status: draft"
        )),
        "{overview}"
    );
    assert!(
        overview.contains("Progress: 0 active · 2 todo · 0 done · 1 blocked"),
        "{overview}"
    );
    assert!(
        overview.contains(&format!("  {epic_id}  epic  todo  medium  Overview epic")),
        "{overview}"
    );
    assert!(
        overview.contains("Children: 1 issue · 0 active · 1 todo · 0 done · 0 blocked"),
        "{overview}"
    );
    assert!(overview.contains("Blockers: 1 open blocker"), "{overview}");
    assert!(
        overview.contains("Direct work: 1 root · 0 active · 1 todo · 0 done · 0 blocked"),
        "{overview}"
    );
    assert!(
        overview.contains(&format!("Drill down: atelier work epic {epic_id}"))
            && overview.contains(&format!("Drill down: atelier work mission {mission_id}")),
        "{overview}"
    );
    assert!(
        overview.contains("Outside visible missions")
            && overview.contains("Unassigned: 2 nonterminal issues")
            && overview.contains("Linked only to done missions: 0 nonterminal issues"),
        "{overview}"
    );
    assert!(!overview.contains("Collapsed epic child"), "{overview}");
    assert!(!overview.contains('\u{1b}'), "{overview:?}");

    let (success, no_color, stderr) =
        run_atelier_with_env(dir.path(), &["work", "missions"], &[("NO_COLOR", "")]);
    assert!(success, "NO_COLOR work missions failed: {stderr}");
    assert_eq!(no_color, overview);
    assert!(!no_color.contains('\u{1b}'), "{no_color:?}");

    let (success, narrow, stderr) = run_atelier_with_env(
        dir.path(),
        &["work", "missions"],
        &[("COLUMNS", "40"), ("NO_COLOR", "")],
    );
    assert!(success, "40-column work missions failed: {stderr}");
    assert!(
        narrow.lines().all(|line| line.chars().count() <= 40),
        "40-column output contains an over-width line:\n{narrow}"
    );
    assert!(
        narrow.contains(&format!(
            "{mission_id}\n  State: todo\n  Priority: high\n  Title: Overview mission"
        )),
        "{narrow}"
    );
    assert!(
        narrow.contains(&format!(
            "  {epic_id}  epic\n    State: todo\n    Priority: medium\n    Title: Overview epic"
        )),
        "{narrow}"
    );
    assert!(
        narrow.contains(&format!(
            "    Drill down:\n      atelier work epic {epic_id}"
        )) && narrow.contains(&format!(
            "  Drill down:\n    atelier work mission {mission_id}"
        )),
        "{narrow}"
    );
    assert!(narrow.contains("  Status: draft"), "{narrow}");
    assert!(narrow.contains("Outside visible missions"), "{narrow}");
    assert!(!narrow.contains("Collapsed epic child"), "{narrow}");
    assert!(!narrow.contains('\u{1b}'), "{narrow:?}");
    let rendered_long_title = narrow
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && line.chars().all(|character| character == 'X'))
        .collect::<String>();
    assert_eq!(rendered_long_title, long_title, "{narrow}");

    let (success, quiet, stderr) = run_atelier(dir.path(), &["--quiet", "work", "missions"]);
    assert!(success, "quiet work missions failed: {stderr}");
    assert_eq!(
        quiet.lines().collect::<Vec<_>>(),
        vec![mission_id.as_str(), long_title_mission_id.as_str()]
    );
    assert!(!quiet.contains('\u{1b}'), "{quiet:?}");
}

#[test]
fn test_work_missions_hides_done_by_default_and_all_includes_done_without_expanding_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Published overview mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission fixture create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Published overview mission");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Work linked only to done mission"],
    );
    assert!(success, "linked work fixture create failed: {stderr}");
    let work_id = issue_id_by_title(dir.path(), "Work linked only to done mission");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &mission_id, &work_id, "--role", "advances"],
    );
    assert!(success, "mission advances fixture failed: {stderr}");

    let mission_path = canonical_issue_path(dir.path(), &mission_id);
    let markdown = std::fs::read_to_string(&mission_path).unwrap();
    let published = markdown.replacen("status: \"draft\"", "status: \"publish_review\"", 1);
    assert_ne!(published, markdown, "mission fixture status was not draft");
    std::fs::write(&mission_path, published).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(
        success,
        "rebuild published mission fixture failed: {stderr}"
    );

    let (success, default, stderr) = run_atelier(dir.path(), &["work", "missions"]);
    assert!(success, "default Mission Overview failed: {stderr}");
    assert!(
        default.contains("No missions match the current overview."),
        "{default}"
    );
    assert!(!default.contains(&mission_id), "{default}");
    assert!(!default.contains("Published overview mission"), "{default}");
    assert!(
        !default.contains("Work linked only to done mission"),
        "{default}"
    );
    assert!(
        default.contains("Linked only to done missions: 1 nonterminal issue"),
        "{default}"
    );

    let (success, default_quiet, stderr) =
        run_atelier(dir.path(), &["--quiet", "work", "missions"]);
    assert!(success, "default quiet Mission Overview failed: {stderr}");
    assert!(default_quiet.is_empty(), "{default_quiet:?}");

    let (success, all, stderr) = run_atelier(dir.path(), &["work", "missions", "--all"]);
    assert!(success, "--all Mission Overview failed: {stderr}");
    assert!(
        all.contains(&format!(
            "{mission_id}  done  medium  Published overview mission"
        )),
        "{all}"
    );
    assert!(all.contains("Status: publish_review"), "{all}");
    assert!(all.contains("Direct work: 1 root"), "{all}");
    assert!(!all.contains("Work linked only to done mission"), "{all}");
    assert!(!all.contains("Outside visible missions"), "{all}");

    let (success, all_quiet, stderr) =
        run_atelier(dir.path(), &["--quiet", "work", "missions", "--all"]);
    assert!(success, "--all quiet Mission Overview failed: {stderr}");
    assert_eq!(all_quiet.trim(), mission_id);
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
            "      start:\n        from: [todo, blocked]\n        to: in_progress\n        description: \"Start active work on this item.\"\n        validators:\n          - blockers.transitive_none_open\n",
            "      start:\n        from: [todo, blocked]\n        to: in_progress\n        description: \"Start active work on this item.\"\n        validators: [evidence.attached]\n",
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
fn test_mission_request_publish_uses_configured_objective_validators() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let remote = tempdir().unwrap();
    git_push(remote.path(), &["init", "--bare", "-q"]);
    add_origin_remote(dir.path(), remote.path());
    git_push(dir.path(), &["push", "-u", "origin", "main"]);
    let mission_id = create_mission_fixture(dir.path(), "Configured validator publish");

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Configured terminal work"]);
    assert!(success, "issue create failed: {stderr}");
    let work_id = issue_id_by_title(dir.path(), "Configured terminal work");
    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "link", &mission_id, &work_id]);
    assert!(success, "mission link failed: {stderr}");
    approve_current_mission_revision(dir.path(), &mission_id);
    commit_all(dir.path(), "configured validator publish fixture");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(success, "mission start failed: {stderr}");
    commit_all(dir.path(), "configured validator mission branch");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "transition", &work_id, "start"]);
    assert!(success, "work start failed: {stderr}");
    commit_all(dir.path(), "configured validator work branch");

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
        &["issue", "transition", &mission_id, "request_publish"],
    );
    assert!(success, "mission request_publish failed: {stderr}");
    assert!(
        close_out.contains("Applied transition request_publish"),
        "{close_out}"
    );
    assert!(
        close_out.contains("To:       publish_review"),
        "{close_out}"
    );
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
fn test_mission_publish_enforces_gates() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let remote = tempdir().unwrap();
    git_push(remote.path(), &["init", "--bare", "-q"]);
    add_origin_remote(dir.path(), remote.path());
    git_push(dir.path(), &["push", "-u", "origin", "main"]);

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Strict publish",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("mission objective atelier-"));
    let mission_id = issue_id_by_title(dir.path(), "Strict publish");
    move_mission_to_ready(dir.path(), &mission_id);

    let (success, work_out, stderr) = run_atelier(dir.path(), &["issue", "create", "Publish work"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Publish work");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "link", &mission_id, &work_id]);
    assert!(success, "mission add work failed: {stderr}");
    approve_current_mission_revision(dir.path(), &mission_id);
    commit_all(dir.path(), "ready strict mission publish");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(success, "mission start failed: {stderr}");
    commit_all(dir.path(), "strict mission publish branch");

    let (success, closeout_blocked_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &mission_id, "request_publish"],
    );
    assert!(!success, "mission publish should fail with open work");
    assert!(closeout_blocked_out.contains("Issue Transition"));
    assert!(closeout_blocked_out.contains("objective.work_terminal"));
    assert!(closeout_blocked_out.contains("validator objective.work_terminal failed"));
    assert!(stderr.contains("Transition 'request_publish' is blocked"));
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "transition", &work_id, "start"]);
    assert!(success, "work start failed: {stderr}");
    commit_all(dir.path(), "strict mission work branch");

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
    commit_all(dir.path(), "ready strict mission publish");
    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &mission_id, "request_publish"],
    );
    assert!(
        success,
        "mission publish should succeed after gates pass: {stderr}"
    );
    assert!(close_out.contains("Applied transition request_publish"));
    assert!(close_out.contains("To:       publish_review"));
}

#[test]
fn test_dirty_worktree_blocks_mission_publish() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Dirty publish",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("mission objective atelier-"));
    let mission_id = issue_id_by_title(dir.path(), "Dirty publish");
    move_mission_to_ready(dir.path(), &mission_id);
    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Dirty terminal work"]);
    assert!(success, "work create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Dirty terminal work");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "link", &mission_id, &work_id]);
    assert!(success, "mission add work failed: {stderr}");
    approve_current_mission_revision(dir.path(), &mission_id);
    commit_all(dir.path(), "dirty mission publish ready");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(success, "mission start failed: {stderr}");
    commit_all(dir.path(), "dirty mission branch");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "transition", &work_id, "start"]);
    assert!(success, "work start failed: {stderr}");
    close_issue_with_evidence(dir.path(), &work_id, Some("done"));
    attach_pass_evidence(
        dir.path(),
        "mission",
        &mission_id,
        "dirty closeout mission proof",
    );
    commit_all(dir.path(), "ready dirty mission publish");
    std::fs::write(dir.path().join("untracked-publish.txt"), "dirty").unwrap();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &mission_id, "request_publish"],
    );
    assert!(!success, "dirty worktree must block mission publish");
    assert!(stdout.contains("Issue Transition"));
    assert!(stdout.contains("validator git.worktree_clean failed"));
    assert!(
        stdout.contains("git checkout has") || stdout.contains("Dirty state: dirty"),
        "{stdout}"
    );
    assert!(stdout.contains("untracked-publish.txt"));
    assert!(stderr.contains("Transition 'request_publish' is blocked"));
}

#[test]
fn test_mission_start_from_side_branch_does_not_require_base_checkout() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Side branch mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("mission objective atelier-"));
    let mission_id = issue_id_by_title(dir.path(), "Side branch mission");
    move_mission_to_ready(dir.path(), &mission_id);
    commit_all(dir.path(), "ready side branch mission");

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "-c", "side-start"])
        .status()
        .unwrap();
    assert!(status.success(), "git switch -c side-start failed");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(
        success,
        "mission start should work from a side branch: {stderr}"
    );
    assert_eq!(
        git_current_branch(dir.path()),
        format!("mission/{mission_id}")
    );
    assert!(
        stdout.contains(&format!("created branch mission/{mission_id} from main")),
        "{stdout}"
    );
}

#[test]
fn test_mission_publish_still_blocks_hand_edited_issue_markdown() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Dirty canonical tracker publish",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("mission objective atelier-"));
    let mission_id = issue_id_by_title(dir.path(), "Dirty canonical tracker publish");
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
    approve_current_mission_revision(dir.path(), &mission_id);
    commit_all(dir.path(), "dirty canonical mission publish ready");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(success, "mission start failed: {stderr}");
    commit_all(dir.path(), "dirty canonical mission branch");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "work start failed: {stderr}");
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
        &["issue", "transition", &mission_id, "request_publish"],
    );
    assert!(
        !success,
        "hand-edited canonical issue markdown must block publish"
    );
    assert!(stdout.contains("Issue Transition"));
    assert!(stdout.contains("validator git.worktree_clean failed"));
    assert!(
        stdout.contains("git checkout has") || stdout.contains("Dirty state: dirty"),
        "{stdout}"
    );
    assert!(stdout.contains(&format!(".atelier/issues/{issue_id}.md")));
    assert!(stderr.contains("Transition 'request_publish' is blocked"));
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
    remove_cache_state(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(!success, "rebuild should reject issue schema drift");
    assert!(
        stderr.contains("Unsupported schema 'atelier.evidence'")
            && stderr.contains("expected atelier.issue"),
        "unexpected rebuild error: {stderr}"
    );
}

#[test]
fn test_cache_query_distinguishes_schema_drift_from_malformed_records() {
    let schema_dir = tempdir().unwrap();
    init_atelier(schema_dir.path());

    let (success, _, stderr) = run_atelier(schema_dir.path(), &["issue", "create", "Schema drift"]);
    assert!(success, "issue create failed: {stderr}");
    let schema_issue_id = issue_id_by_title(schema_dir.path(), "Schema drift");
    edit_canonical_issue(schema_dir.path(), &schema_issue_id, |markdown| {
        markdown.replace("schema_version: 1", "schema_version: 99")
    });
    remove_cache_state(schema_dir.path());

    let (success, _, stderr) = run_atelier(schema_dir.path(), &["work", "queue"]);
    assert!(!success, "schema drift should block cache-backed query");
    assert!(
        stderr.contains("schema this atelier binary does not understand")
            && stderr.contains("target/debug/atelier")
            && stderr.contains("update the installed `atelier` binary")
            && stderr.contains("Unsupported schema_version 99"),
        "schema drift diagnostic should name stale-binary repair: {stderr}"
    );
    assert!(
        !stderr.contains("fix tracker record files before querying"),
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
    remove_cache_state(malformed_dir.path());

    let (success, _, stderr) = run_atelier(malformed_dir.path(), &["work", "queue"]);
    assert!(
        !success,
        "malformed records should block cache-backed query"
    );
    assert!(
        stderr.contains("recovery: 1. run `atelier check`")
            && stderr.contains("2. fix the named record file")
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
fn test_cache_rebuilds_changed_sources_before_issue_queries() {
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
        stderr.contains("Local cache was stale; rebuilt SQLite cache")
            || stderr
                .contains("Local cache was stale; repaired changed record sources incrementally"),
        "missing automatic rebuild diagnostic: {stderr}"
    );
}

#[test]
fn test_cache_query_rebuilds_missing_cache_on_demand() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Lazy missing cache"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    remove_cache_state(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--status", "all"]);

    assert!(success, "query should rebuild missing cache: {stderr}");
    assert!(stdout.contains("Lazy missing cache"));
    assert!(
        stderr.contains("Local cache was missing; rebuilt SQLite cache"),
        "missing lazy rebuild diagnostic: {stderr}"
    );
}

#[test]
fn test_cache_decision_query_never_returns_known_stale_rows() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Known stale decision"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");
    edit_canonical_issue(dir.path(), &issue_id, |markdown| {
        markdown.replace("schema_version: 1", "schema_version: 99")
    });

    let (success, stdout, stderr) = run_atelier(dir.path(), &["work", "queue", "--status", "all"]);

    assert!(!success, "decision query must reject invalid source state");
    assert!(
        !stdout.contains("Known stale decision"),
        "known-stale cache row escaped into decision output: {stdout}"
    );
    assert!(
        stderr
            .contains("tracker record files use a schema this atelier binary does not understand"),
        "missing source-schema diagnostic: {stderr}"
    );
    assert!(
        stderr.contains("Tracker record files are invalid"),
        "missing record-file validity prefix: {stderr}"
    );
}

#[test]
fn test_cache_orientation_names_degraded_last_good_state() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Degraded orientation"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);
    edit_canonical_issue(dir.path(), &issue_id, |markdown| {
        markdown.replace("status: todo", "status: in_progress")
    });
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");
    edit_canonical_issue(dir.path(), &issue_id, |markdown| {
        markdown.replace("schema_version: 1", "schema_version: 99")
    });

    let (success, stdout, stderr) = run_atelier(dir.path(), &["status"]);

    assert!(success, "orientation should use last good cache: {stderr}");
    assert!(stdout.contains("Atelier Status"));
    assert!(
        stderr.contains("using the existing local cache for orientation only"),
        "missing degraded orientation diagnostic: {stderr}"
    );
    assert!(
        stderr.contains("Record-file diagnostic:"),
        "missing record-file diagnostic label: {stderr}"
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "degraded issue show failed: {stderr}");
    assert!(
        stdout.contains("Issue record file is malformed:"),
        "{stdout}"
    );
    assert!(!stdout.contains("Canonical issue record"), "{stdout}");
}

#[test]
fn test_cache_bounds_many_changed_sources_and_rebuilds() {
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
    assert!(
        !success,
        "export check should report a stale cache diagnostic"
    );
    assert!(
        stderr.contains("12 indexed sources changed")
            && stderr.contains("Cache diagnostic is stale")
            && stderr.contains("showing first 5")
            && stderr.contains("recovery: 1. run `atelier check`")
            && stderr.contains("3. run `atelier check --fix`")
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
        stderr.contains("Local cache was stale; rebuilt SQLite cache")
            || stderr
                .contains("Local cache was stale; repaired changed record sources incrementally"),
        "missing automatic rebuild diagnostic: {stderr}"
    );
    let (success, stdout, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "fresh export check failed: {stderr}");
    assert!(stdout.contains("Record files and domain cache are current"));
}

#[test]
fn test_cache_repairs_deleted_and_unindexed_sources_before_issue_queries() {
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
        stderr.contains("Local cache was stale; repaired changed record sources incrementally")
            || stderr.contains("Local cache was stale; rebuilt SQLite cache")
            || stderr
                .contains("Local cache was stale; repaired changed record sources incrementally"),
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
        stderr.contains("Local cache was stale; rebuilt SQLite cache")
            || stderr
                .contains("Local cache was stale; repaired changed record sources incrementally"),
        "missing automatic rebuild diagnostic: {stderr}"
    );
}

#[test]
fn test_cache_rebuilds_dep_list_and_lint_but_ignores_derived_files() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let first_body = "## Description\n\nCache root body.\n\n## Outcome\n\nCache root remains queryable after rebuild.\n\n## Evidence\n\n- manual check: `atelier lint` output prints `Lint passed.` after automatic rebuild.";
    let second_body = "## Description\n\nCache leaf body.\n\n## Outcome\n\nCache leaf remains linked after rebuild.\n\n## Evidence\n\n- manual check: `atelier issue show <id>` output shows the linked root.";

    let (success, first_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Cache root", "--description", first_body],
    );
    assert!(success, "first create failed: {stderr}");
    assert!(first_out.contains("Created issue atelier-"));
    let first_id = issue_ref(dir.path(), 1);
    let (success, second_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Cache leaf",
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
    assert!(ready_out.contains("Cache root"));

    edit_canonical_issue(dir.path(), &first_id, |markdown| {
        markdown.replace("Cache root", "Cache root changed")
    });

    let (success, dep_out, stderr) = run_atelier(dir.path(), &["issue", "show", &second_id]);
    assert!(
        success,
        "stale issue show should transparently rebuild: {stderr}"
    );
    assert!(dep_out.contains("Cache root changed"));
    assert!(
        stderr.contains("Local cache was stale; rebuilt SQLite cache")
            || stderr
                .contains("Local cache was stale; repaired changed record sources incrementally"),
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
                && !combined.contains("cache.lock"),
            "{args:?} diagnostics must not report ignored local artifacts: {combined}"
        );
    }
}

#[test]
fn test_lint_validates_record_markdown_even_when_cache_metadata_is_fresh() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let issue_id = "atelier-lint1".to_string();
    write_canonical_record(
        dir.path(),
        "issues",
        &issue_id,
        r#"---
created_at: "2026-06-10T12:00:00+00:00"
id: "atelier-lint1"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Lint canonical source"
updated_at: "2026-06-10T12:00:00+00:00"
---

## Description

Lint source fixture.

## Outcome

Lint rejects malformed canonical state.

## Evidence

- Command transcript from `atelier lint` reports the malformed record.
"#
        .to_string(),
    );
    let (success, _stdout, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "fixture rebuild failed: {stderr}");

    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = read_canonical_record(dir.path(), "issues", &issue_id);
    let invalid_markdown = markdown.replace(
        "title: \"Lint canonical source\"",
        "title: [Lint canonical source",
    );
    write_canonical_record(dir.path(), "issues", &issue_id, invalid_markdown.clone());
    write_ignored_canonical_artifacts(dir.path(), &issue_id);

    let metadata = std::fs::metadata(&issue_path).unwrap();
    let modified_micros = metadata
        .modified()
        .unwrap()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros() as i64;
    let mut hasher = Sha256::new();
    hasher.update(invalid_markdown.as_bytes());
    let invalid_hash = format!("{:x}", hasher.finalize());
    let conn = rusqlite::Connection::open(dir.path().join(".atelier/runtime/state.db")).unwrap();
    conn.execute(
        "UPDATE record_source_index
         SET size_bytes = ?1, modified_micros = ?2, content_hash = ?3
         WHERE path = ?4",
        rusqlite::params![
            i64::try_from(metadata.len()).unwrap(),
            modified_micros,
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
            && !transcript.contains("cache.lock"),
        "lint must ignore local artifacts while reporting malformed committed Markdown: {transcript}"
    );
    assert!(stderr.contains("Lint failed"));
    assert!(
        !stdout.contains("Lint passed."),
        "lint must not pass from stale SQLite rows: {stdout}"
    );
}

#[test]
fn test_lint_repairs_unindexed_issue_before_indexed_rules_run() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    std::fs::create_dir_all(dir.path().join(".atelier/issues")).unwrap();
    std::fs::write(
        dir.path().join(".atelier/issues/atelier-base1.md"),
        r#"---
created_at: "2026-06-10T12:00:00+00:00"
id: "atelier-base1"
issue_type: "task"
labels: []
priority: "P1"
relationships: { blocks: [], children: [], attachments: [], relates: [] }
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Indexed lint base"
updated_at: "2026-06-10T12:00:00+00:00"
---

## Description

Base record.

## Outcome

Base record is indexed.

## Evidence

- Command transcript from `atelier lint` passes.
"#,
    )
    .unwrap();
    let (success, _stdout, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "fixture rebuild failed: {stderr}");

    std::fs::write(
        dir.path().join(".atelier/issues/atelier-new1.md"),
        r#"---
created_at: "2026-06-10T12:00:00+00:00"
id: "atelier-new1"
issue_type: "task"
labels: []
priority: "P1"
relationships: { blocks: [], children: [], attachments: [], relates: [] }
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Unindexed invalid issue"
updated_at: "2026-06-10T12:00:00+00:00"
---

## Description

This new record is absent from the stale cache.

## Outcome

Lint must repair the cache before checking this scoped record.

## Evidence

Evidence will be added.
"#,
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", "atelier-new1"]);
    assert!(!success, "lint must reject the unindexed invalid issue");
    assert!(
        stdout.contains("atelier-new1") && stdout.contains("section Evidence"),
        "unexpected lint output:\n{stdout}\n{stderr}"
    );
    assert!(
        !stderr.contains("repaired changed record sources incrementally"),
        "lint should diagnose record files without eagerly repairing cache: {stderr}"
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
    remove_cache_state(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(
        success,
        "lint should validate without rebuilding state.db: {stderr}"
    );
    assert!(stdout.contains("Lint passed."));
    assert!(
        !stderr.contains("rebuilt SQLite cache"),
        "lint should not eagerly rebuild disposable cache: {stderr}"
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
    assert!(stderr.contains("Local cache was missing; rebuilt SQLite cache"));
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
            && transcript.contains("Tracker record files are invalid"),
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
        &["does not match record-file path"],
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
        &["Unsupported issue record file", ".atelier/issues/junk.txt"],
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
        &["Duplicate record ID in record files"],
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
        transcript.contains("Tracker record files are invalid")
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
        "labels": ["bundle"],
        "blocks": [{ "client_ref": "issue.blocked" }]
      },
      {
        "client_ref": "issue.blocked",
        "title": "Blocked bundle work",
        "issue_type": "task",
        "priority": "medium"
      },
      {
        "client_ref": "issue.epic",
        "title": "Bundle epic",
        "issue_type": "epic",
        "priority": "high"
      },
      {
        "client_ref": "issue.work",
        "title": "Implement bundle output",
        "issue_type": "feature",
        "priority": "high",
        "status": "in_progress",
        "parent": { "client_ref": "issue.epic" },
        "depends_on": [{ "client_ref": "issue.blocker" }],
        "notes": [
          { "body": "Zulu authored first" },
          { "body": "Alpha authored second" }
        ],
        "outcome": ["summary maps client refs"],
        "evidence": ["export check passes"]
      },
      {
        "client_ref": "mission.bundle",
        "title": "Bundle mission",
        "issue_type": "mission",
        "priority": "medium",
        "labels": ["bundle", "mission"],
        "advances": [{ "client_ref": "issue.epic" }],
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
    let db_path = dir.path().join(".atelier/runtime/state.db");
    let db_before_preview = std::fs::read(&db_path).unwrap();

    let (success, dry_run_out, stderr) =
        run_atelier(dir.path(), &["bundle", "preview", bundle_arg]);
    assert!(success, "bundle preview failed: {stderr}");
    assert!(dry_run_out.contains("Bundle preview is valid."));
    assert!(dry_run_out.contains("Applied:       false"));
    assert!(dry_run_out.contains("Preview:       true"));
    assert!(dry_run_out.contains("issues: 5"), "{dry_run_out}");
    assert!(dry_run_out.contains("relationships: 5"), "{dry_run_out}");
    assert!(dry_run_out.contains("notes: 2"), "{dry_run_out}");
    let expected_edges = [
        "evidence/evidence.bundle -> issue/mission.bundle (validates)",
        "issue/issue.blocker -> issue/issue.blocked (blocks)",
        "issue/issue.blocker -> issue/issue.work (blocks)",
        "issue/issue.epic -> issue/issue.work (parent)",
        "issue/mission.bundle -> issue/issue.epic (advances)",
    ];
    let mut previous_position = 0;
    for edge in expected_edges {
        let position = dry_run_out
            .find(edge)
            .unwrap_or_else(|| panic!("preview missing {edge:?}: {dry_run_out}"));
        assert!(
            position >= previous_position,
            "preview relationships are not deterministic: {dry_run_out}"
        );
        previous_position = position;
    }
    assert_eq!(
        std::fs::read(&db_path).unwrap(),
        db_before_preview,
        "bundle preview mutated SQLite"
    );
    assert_eq!(
        std::fs::read_dir(dir.path().join(".atelier/issues"))
            .unwrap()
            .count(),
        0,
        "bundle preview created canonical issues"
    );
    assert_eq!(
        std::fs::read_dir(dir.path().join(".atelier/evidence"))
            .unwrap()
            .count(),
        0,
        "bundle preview created canonical evidence"
    );
    let (success, repeated_preview, stderr) =
        run_atelier(dir.path(), &["bundle", "preview", bundle_arg]);
    assert!(success, "repeated bundle preview failed: {stderr}");
    assert_eq!(
        dry_run_out, repeated_preview,
        "preview must be deterministic"
    );

    let (success, apply_out, stderr) =
        run_atelier(dir.path(), &["bundle", "apply", bundle_arg, "--yes"]);
    assert!(success, "bundle apply failed: {stderr}");
    assert!(apply_out.contains("Bundle applied."));
    assert!(apply_out.contains("Applied:       true"));
    assert!(apply_out.contains("relationships: 5"), "{apply_out}");
    assert!(apply_out.contains("notes: 2"), "{apply_out}");
    assert_eq!(apply_out.matches("(blocks)").count(), 2, "{apply_out}");
    for role in ["parent", "advances", "validates"] {
        assert_eq!(
            apply_out.matches(&format!("({role})")).count(),
            1,
            "{apply_out}"
        );
    }
    assert!(apply_out.contains("atelier issue show"));
    let mission_id = issue_id_by_title(dir.path(), "Bundle mission");
    let epic_id = issue_id_by_title(dir.path(), "Bundle epic");
    let work_id = issue_id_by_title(dir.path(), "Implement bundle output");
    let blocker_id = issue_id_by_title(dir.path(), "Complete prerequisite");
    let blocked_id = issue_id_by_title(dir.path(), "Blocked bundle work");

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(!success, "bundle apply should leave cache detectably stale");

    let (success, _, stderr) = run_atelier(dir.path(), &["work", "queue", "--status", "all"]);
    assert!(success, "lazy query after bundle apply failed: {stderr}");
    assert!(
        stderr.contains("Local cache was stale; rebuilt SQLite cache")
            || stderr
                .contains("Local cache was stale; repaired changed record sources incrementally"),
        "missing one-time lazy rebuild diagnostic: {stderr}"
    );

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check after lazy rebuild failed: {stderr}");

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
    assert!(mission_markdown.contains(&format!("id: \"{epic_id}\"")));
    let epic_markdown = std::fs::read_to_string(
        dir.path()
            .join(".atelier/issues")
            .join(format!("{epic_id}.md")),
    )
    .unwrap();
    assert!(epic_markdown.contains(&format!("id: \"{work_id}\"")));
    let blocker_markdown = std::fs::read_to_string(
        dir.path()
            .join(".atelier/issues")
            .join(format!("{blocker_id}.md")),
    )
    .unwrap();
    assert!(blocker_markdown.contains(&format!("id: \"{work_id}\"")));
    assert!(blocker_markdown.contains(&format!("id: \"{blocked_id}\"")));
    let mut note_paths = std::fs::read_dir(
        dir.path()
            .join(".atelier/issues")
            .join(format!("{work_id}.activity")),
    )
    .unwrap()
    .map(|entry| entry.unwrap().path())
    .collect::<Vec<_>>();
    note_paths.sort();
    let note_bodies = note_paths
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let first_note = note_bodies.find("Zulu authored first").unwrap();
    let second_note = note_bodies.find("Alpha authored second").unwrap();
    assert!(
        first_note < second_note,
        "bundle notes were not appended in authored order: {note_bodies}"
    );
    let evidence_markdown = std::fs::read_dir(dir.path().join(".atelier/evidence"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let evidence_markdown = std::fs::read_to_string(evidence_markdown).unwrap();
    assert!(evidence_markdown.contains(&format!("id: \"{mission_id}\"")));
    assert!(evidence_markdown.contains("role: \"validates\""));
}

#[test]
fn test_bundle_preview_rejects_duplicate_normalized_relationships_without_mutation() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bundle_path = dir.path().join("duplicate-relationship-bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 1,
  "title": "Duplicate normalized relationship",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.blocker",
        "title": "Bundle blocker",
        "blocks": [{ "client_ref": "issue.blocked" }]
      },
      {
        "client_ref": "issue.blocked",
        "title": "Bundle blocked",
        "depends_on": [{ "client_ref": "issue.blocker" }]
      }
    ]
  }
}"#,
    )
    .unwrap();
    let db_path = dir.path().join(".atelier/runtime/state.db");
    let db_before_preview = std::fs::read(&db_path).unwrap();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["bundle", "preview", bundle_path.to_str().unwrap()],
    );

    assert!(!success, "duplicate normalized edge should fail preview");
    assert!(
        stderr.contains("Duplicate bundle relationship after normalization")
            && stderr.contains("(blocks)"),
        "stdout: {stdout}\nstderr: {stderr}"
    );
    assert_eq!(std::fs::read(&db_path).unwrap(), db_before_preview);
    assert_eq!(
        std::fs::read_dir(dir.path().join(".atelier/issues"))
            .unwrap()
            .count(),
        0
    );
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
fn test_bundle_apply_rejects_executable_missions_without_review_and_preserves_state() {
    for status in ["ready", "in_progress"] {
        let dir = tempdir().unwrap();
        init_atelier(dir.path());
        let bundle_path = dir.path().join(format!("executable-mission-{status}.json"));
        std::fs::write(
            &bundle_path,
            format!(
                r#"{{
  "schema": "atelier.bundle",
  "schema_version": 1,
  "title": "Executable mission without receipts",
  "resources": {{
    "issues": [
      {{
        "client_ref": "issue.blocker",
        "title": "Open bundled blocker",
        "issue_type": "task",
        "status": "todo"
      }},
      {{
        "client_ref": "mission.executable",
        "title": "Unreviewed executable mission",
        "issue_type": "mission",
        "status": {status:?},
        "depends_on": [{{ "client_ref": "issue.blocker" }}]
      }}
    ]
  }}
}}"#
            ),
        )
        .unwrap();
        let state_dir = dir.path().join(".atelier");
        let db_path = state_dir.join("runtime/state.db");
        let canonical_before = canonical_tree_snapshot(&state_dir);
        let db_before = std::fs::read(&db_path).unwrap();

        let (success, _stdout, stderr) = run_atelier(
            dir.path(),
            &["bundle", "apply", bundle_path.to_str().unwrap(), "--yes"],
        );

        assert!(!success, "unreviewed {status} mission must be rejected");
        assert!(
            stderr.contains("workflow_mission_plan_review_bypass"),
            "{stderr}"
        );
        assert_eq!(stderr.matches("Next:").count(), 1, "{stderr}");
        assert_eq!(canonical_tree_snapshot(&state_dir), canonical_before);
        assert_eq!(std::fs::read(db_path).unwrap(), db_before);
    }
}

#[test]
fn test_bundle_apply_accepts_non_executable_initial_statuses() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bundle_path = dir.path().join("initial-statuses.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 1,
  "title": "Initial statuses",
  "resources": {
    "issues": [
      {
        "client_ref": "mission.draft",
        "title": "Draft bundled mission",
        "issue_type": "mission",
        "status": "draft"
      },
      {
        "client_ref": "issue.todo",
        "title": "Todo bundled task",
        "issue_type": "task",
        "status": "todo"
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

    assert!(success, "initial-status bundle should apply: {stderr}");
    assert!(stdout.contains("Bundle applied."), "{stdout}");
    let (success, _, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(
        success,
        "applied initial statuses must rebuild cleanly: {stderr}"
    );
}

#[test]
fn test_bundle_apply_rejects_graph_edit_that_stales_executable_mission_review() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    let mission_id = create_mission_fixture(dir.path(), "Reviewed executable mission");
    let (success, _, stderr) = run_atelier(dir.path(), &["check", "--fix"]);
    assert!(success, "fixture cache refresh failed: {stderr}");
    let bundle_path = dir.path().join("stale-reviewed-mission.json");
    std::fs::write(
        &bundle_path,
        format!(
            r#"{{
  "schema": "atelier.bundle",
  "schema_version": 1,
  "title": "Stale reviewed mission",
  "resources": {{
    "issues": [
      {{
        "client_ref": "issue.completed-blocker",
        "title": "Completed graph addition",
        "issue_type": "task",
        "status": "done",
        "blocks": [{{ "id": {mission_id:?} }}]
      }}
    ]
  }}
}}"#
        ),
    )
    .unwrap();
    let state_dir = dir.path().join(".atelier");
    let db_path = state_dir.join("runtime/state.db");
    let canonical_before = canonical_tree_snapshot(&state_dir);
    let db_before = std::fs::read(&db_path).unwrap();

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["bundle", "apply", bundle_path.to_str().unwrap(), "--yes"],
    );

    assert!(!success, "stale executable mission review must be rejected");
    assert!(
        stderr.contains("workflow_mission_plan_review_bypass"),
        "{stderr}"
    );
    assert_eq!(stderr.matches("Next:").count(), 1, "{stderr}");
    assert_eq!(canonical_tree_snapshot(&state_dir), canonical_before);
    assert_eq!(std::fs::read(db_path).unwrap(), db_before);
}

#[test]
fn test_bundle_apply_exclusive_snapshot_swap_preserves_later_writer() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bundle_path = dir.path().join("paused-bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 1,
  "title": "Paused bundle",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.bulk",
        "title": "Bulk installed task",
        "issue_type": "task",
        "status": "todo"
      }
    ]
  }
}"#,
    )
    .unwrap();
    let marker = dir.path().join("bundle-snapshot.marker");
    let release = marker.with_extension("release");
    let bulk = std::process::Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir.path())
        .args(["bundle", "apply", bundle_path.to_str().unwrap(), "--yes"])
        .env("ATELIER_TEST_BULK_PAUSE_AFTER_SNAPSHOT", &marker)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    wait_for_test_marker(&marker);
    assert_bulk_process_holds_exclusive_lock(&dir.path().join(".atelier"));

    let mut writer = std::process::Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir.path())
        .args(["issue", "create", "Writer after bundle snapshot"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    assert!(
        writer.try_wait().unwrap().is_none(),
        "ordinary writer completed while bulk snapshot/swap lock was held"
    );

    std::fs::write(&release, "release").unwrap();
    let bulk_output = bulk.wait_with_output().unwrap();
    assert!(
        bulk_output.status.success(),
        "bundle failed: {}",
        String::from_utf8_lossy(&bulk_output.stderr)
    );
    let writer_output = writer.wait_with_output().unwrap();
    assert!(
        writer_output.status.success(),
        "writer failed: {}",
        String::from_utf8_lossy(&writer_output.stderr)
    );
    assert!(canonical_directory_contains(
        dir.path(),
        "issues",
        "Bulk installed task"
    ));
    assert!(canonical_directory_contains(
        dir.path(),
        "issues",
        "Writer after bundle snapshot"
    ));
}

#[test]
fn test_import_beads_exclusive_snapshot_swap_preserves_later_writer() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let import_path = dir.path().join("paused-import.jsonl");
    std::fs::write(
        &import_path,
        r#"{"_type":"issue","id":"bulk-source","title":"Bulk imported task","status":"open","priority":2,"issue_type":"task"}
"#,
    )
    .unwrap();
    let marker = dir.path().join("import-snapshot.marker");
    let release = marker.with_extension("release");
    let bulk = std::process::Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir.path())
        .args(["import-beads", import_path.to_str().unwrap()])
        .env("ATELIER_TEST_BULK_PAUSE_AFTER_SNAPSHOT", &marker)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    wait_for_test_marker(&marker);
    assert_bulk_process_holds_exclusive_lock(&dir.path().join(".atelier"));

    let mut writer = std::process::Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir.path())
        .args(["issue", "create", "Writer after import snapshot"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    assert!(
        writer.try_wait().unwrap().is_none(),
        "ordinary writer completed while import snapshot/swap lock was held"
    );

    std::fs::write(&release, "release").unwrap();
    let bulk_output = bulk.wait_with_output().unwrap();
    assert!(
        bulk_output.status.success(),
        "import failed: {}",
        String::from_utf8_lossy(&bulk_output.stderr)
    );
    let writer_output = writer.wait_with_output().unwrap();
    assert!(
        writer_output.status.success(),
        "writer failed: {}",
        String::from_utf8_lossy(&writer_output.stderr)
    );
    assert!(canonical_directory_contains(
        dir.path(),
        "issues",
        "Bulk imported task"
    ));
    assert!(canonical_directory_contains(
        dir.path(),
        "issues",
        "Writer after import snapshot"
    ));
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
    move_reviewed_mission_to_ready(dir.path(), &mission_id);
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
    move_reviewed_mission_to_ready(dir.path(), &mission_id);
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
    move_reviewed_mission_to_ready(dir.path(), &mission_id);
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
fn test_epic_start_requires_started_mission_branch_for_mission_scope() {
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
    move_reviewed_mission_to_ready(dir.path(), &mission_id);
    commit_all(dir.path(), "wrong branch baseline");

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "-c", "side-start"])
        .status()
        .unwrap();
    assert!(status.success(), "git switch -c side-start failed");
    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &epic_id, "start"]);

    assert!(!success, "epic start should reject missing mission branch");
    let output = format!("{stdout}\n{stderr}");
    assert!(
        output.contains(&format!("base branch 'mission/{mission_id}' is missing"))
            || output.contains(&format!("base branch 'mission/{mission_id}'")),
        "{output}"
    );
    assert_eq!(git_current_branch(dir.path()), "side-start");
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &epic_id]);
    assert!(success, "epic show failed: {stderr}");
    assert!(show_out.contains("Status:   todo"), "{show_out}");
}

#[test]
fn test_epic_start_from_other_branch_uses_recorded_mission_branch() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    write_mission_branch_workflow(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Side branch mission",
            "--issue-type",
            "mission",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = issue_id_by_title(dir.path(), "Side branch mission");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Side branch epic",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Side branch epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &mission_id, &epic_id, "--role", "advances"],
    );
    assert!(success, "mission link failed: {stderr}");
    move_reviewed_mission_to_ready(dir.path(), &mission_id);
    commit_all(dir.path(), "side branch mission baseline");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(success, "mission start failed: {stderr}");
    commit_all(dir.path(), "side branch mission started");

    let status = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "-c", "side-start"])
        .status()
        .unwrap();
    assert!(status.success(), "git switch -c side-start failed");
    let (success, start_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &epic_id, "start"]);

    assert!(success, "epic start failed: {stderr}");
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    assert!(
        start_out.contains(&format!(
            "created branch epic/{epic_id} from mission/{mission_id}"
        )),
        "{start_out}"
    );
    let mission_head = git_rev_parse(dir.path(), &format!("mission/{mission_id}"));
    assert_eq!(
        git_rev_parse(dir.path(), &format!("epic/{epic_id}~0")),
        mission_head
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
fn test_epic_start_from_side_branch_uses_configured_base_branch() {
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
    assert!(success, "epic start should work from side branch: {stderr}");
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    assert!(
        stdout.contains(&format!("created branch epic/{epic_id} from main")),
        "{stdout}"
    );

    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &epic_id]);
    assert!(success, "epic show failed: {stderr}");
    assert!(show_out.contains("Status:   in_progress"), "{show_out}");
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
