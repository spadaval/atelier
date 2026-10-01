use super::*;

pub(super) fn canonical_tree_snapshot(
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
            if relative.components().count() == 1 && matches!(relative.to_str(), Some("runtime")) {
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
  "schema_version": 2,
  "title": "Mission fixture",
  "resources": {{
    "issues": [
      {{
        "client_ref": "mission.fixture",
        "title": {title:?},
        "issue_type": "mission",
        "body": "\u0023\u0023 Outcome\n\nMission fixture body.\n\n## Success criteria\n\nThe intended mission result is observable.",
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
    move_mission_directly_to_ready(dir, &mission_id);
    mission_id
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
    let work_done = "    - validation\n    initial_status: todo\n    done_statuses:\n    - done\n";
    assert!(policy.contains(work_done));
    let policy = policy.replacen(
        work_done,
        "    - validation\n    initial_status: todo\n    done_statuses:\n    - accepted\n",
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
fn test_work_missions_renders_flat_complete_scope_and_plain_quiet_output() {
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
        overview.contains("Scope: 3 issues: 0 active, 2 ready, 1 blocked, 0 done, 0 backlog"),
        "{overview}"
    );
    assert!(overview.contains("Open blockers: 1"), "{overview}");
    assert!(
        overview.contains(&format!("Drill down: atelier work mission {mission_id}")),
        "{overview}"
    );
    assert!(
        overview.contains("Unassigned: 2 nonterminal issues"),
        "{overview}"
    );
    assert!(!overview.contains(&epic_id), "{overview}");
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
            "  Drill down:\n    atelier work mission {mission_id}"
        )),
        "{narrow}"
    );
    assert!(narrow.contains("  Status: draft"), "{narrow}");
    assert!(
        narrow.contains("Unassigned: 2 nonterminal issues"),
        "{narrow}"
    );
    assert!(!narrow.contains(&epic_id), "{narrow}");
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
fn test_quiet_work_missions_is_complete_beyond_the_human_display_limit() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let mut expected = Vec::new();
    for number in 0..23 {
        let title = format!("Complete mission {number:02}");
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["issue", "create", &title, "--issue-type", "mission"],
        );
        assert!(success, "mission fixture create failed: {stderr}");
        expected.push(issue_id_by_title(dir.path(), &title));
    }
    expected.sort();
    let (success, quiet, stderr) = run_atelier(dir.path(), &["--quiet", "work", "missions"]);
    assert!(success, "quiet mission listing failed: {stderr}");
    assert_eq!(quiet.lines().collect::<Vec<_>>(), expected);
    let (success, human, stderr) = run_atelier(dir.path(), &["work", "missions"]);
    assert!(success, "human mission listing failed: {stderr}");
    assert!(human.contains("3 more missions omitted"), "{human}");
    assert!(!human.contains(expected.last().unwrap()), "{human}");
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
    let published = markdown.replacen("status: \"draft\"", "status: \"done\"", 1);
    assert_ne!(published, markdown, "mission fixture status was not draft");
    std::fs::write(&mission_path, published).unwrap();

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
    assert!(all.contains("Status: done"), "{all}");
    assert!(all.contains("Scope: 1 issues: 0 active, 1 ready"), "{all}");
    assert!(!all.contains("Work linked only to done mission"), "{all}");
    assert!(!all.contains("Outside visible missions"), "{all}");

    let (success, all_quiet, stderr) =
        run_atelier(dir.path(), &["--quiet", "work", "missions", "--all"]);
    assert!(success, "--all quiet Mission Overview failed: {stderr}");
    assert_eq!(all_quiet.trim(), mission_id);
}

#[test]
fn test_ready_work_remains_visible_when_proof_blocks_start_transition() {
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
        policy.replace(
            "        description: Start work.\n        validators:\n        - blockers.transitive_none_open\n",
            "        description: Start work.\n        validators: [evidence.attached]\n",
        ),
    )
    .unwrap();

    let (success, blocked_ready_out, stderr) = run_atelier(dir.path(), &["work", "ready"]);
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
    std::fs::write(&issue_path, remove_issue_section(&markdown, "Done when")).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["check", &issue_id]);
    assert!(!success, "lint should fail for missing Outcome");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Done when")
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
    let invalid = markdown.replace(
        "## Done when\n\nDone when was not specified.",
        "## Done when\n\n",
    );
    std::fs::write(&issue_path, invalid).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["check", &issue_id]);
    assert!(!success, "lint should fail for empty Outcome");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Done when")
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
    std::fs::write(&issue_path, remove_issue_section(&markdown, "Done when")).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["check", &issue_id]);
    assert!(!success, "lint should fail for missing Outcome");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Done when")
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
    let invalid = markdown.replace("## Goal\n\nGoal was not specified.", "## Goal\n\n");
    std::fs::write(&issue_path, invalid).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["check", &issue_id]);
    assert!(!success, "lint should fail for empty Description");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Goal")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing Description diagnostic in stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[test]
fn test_lint_rejects_missing_acceptance_even_when_notes_name_a_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let body = "## Goal\n\nDescription\n\n## Done when\n\nDone when was not specified.\n\n## Context\n\n- Validation complete.\n\n## Notes\n\n- `cargo test --test cli_integration vague_evidence` passes.";

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

    let (success, stdout, stderr) = run_atelier(dir.path(), &["check", &issue_id]);
    assert!(!success, "lint should fail for vague Evidence");
    let transcript = format!("{stdout}\n{stderr}");
    for needle in [
        &issue_id,
        "Issue section Done when must be present and non-empty",
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
    let body = "## Goal\n\nDescription\n\n## Done when\n\nLint accepts concrete Evidence entries without optional Notes.\n\n## Context\n\n- `cargo test --test cli_integration concrete_evidence` passes.\n- Manual check confirms the lint diagnostic names the issue, section, and path.";

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

    let (success, stdout, stderr) = run_atelier(dir.path(), &["check", &issue_id]);
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
        "## Done when\n\nDone when was not specified.",
        "## Done when\n\nDone when was not specified.\n\n## Done when\n\nSecond outcome should be rejected.",
    );
    std::fs::write(&issue_path, invalid).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(!success, "lint should fail for duplicate Outcome");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Done when")
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
    std::fs::write(&issue_path, remove_issue_section(&markdown, "Done when")).unwrap();

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(!success, "root start should refuse invalid issue");
    assert!(
        stderr.contains(&format!("issue {issue_id}"))
            && stderr.contains("section Done when")
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
    move_epic_directly_to_ready(dir.path(), &issue_id);
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
fn test_mission_close_options_report_configured_work_terminal_validator() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let policy_path = dir.path().join(".atelier/workflow.yaml");
    let policy = std::fs::read_to_string(&policy_path).unwrap();
    let objective_close = "        validators:\n        - blockers.none_open\n      supersede:\n        from: [draft, todo, blocked, in_progress]\n";
    assert!(policy.contains(objective_close));
    std::fs::write(
        &policy_path,
        policy.replace(
            objective_close,
            "        validators:\n        - blockers.none_open\n        - work.terminal\n      supersede:\n        from: [draft, todo, blocked, in_progress]\n",
        ),
    )
    .unwrap();
    let mission_id = create_mission_fixture(dir.path(), "Configured validator blockers");

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Configured open work"]);
    assert!(success, "issue create failed: {stderr}");
    let work_id = issue_id_by_title(dir.path(), "Configured open work");
    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "link", &mission_id, &work_id]);
    assert!(success, "mission link failed: {stderr}");
    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(success, "mission start transition failed: {stderr}");

    let (success, options_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &mission_id, "--verbose"],
    );
    assert!(success, "mission transition options failed: {stderr}");
    assert!(options_out.contains("close [blocked]"), "{options_out}");
    assert!(options_out.contains("fail  work.terminal"), "{options_out}");
    assert!(
        options_out.contains(&format!("open work: {work_id} [todo]")),
        "{options_out}"
    );
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
fn test_mission_start_is_removed_without_compatibility_guidance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "start", "atelier-missing"]);
    assert!(!success, "mission start should be removed");
    assert!(stderr.contains("unrecognized subcommand"), "{stderr}");
    assert!(!stderr.contains("--switch"), "{stderr}");
}

#[test]
fn test_record_load_rejects_schema_drift() {
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

    let (success, _, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(!success, "status should reject issue schema drift");
    assert!(
        stderr.contains("Unsupported schema 'atelier.evidence'")
            && stderr.contains("expected atelier.issue"),
        "unexpected load error: {stderr}"
    );
}

#[test]
fn test_queries_fail_naming_the_invalid_record_file() {
    let schema_dir = tempdir().unwrap();
    init_atelier(schema_dir.path());

    let (success, _, stderr) = run_atelier(schema_dir.path(), &["issue", "create", "Schema drift"]);
    assert!(success, "issue create failed: {stderr}");
    let schema_issue_id = issue_id_by_title(schema_dir.path(), "Schema drift");
    edit_canonical_issue(schema_dir.path(), &schema_issue_id, |markdown| {
        markdown.replace("schema_version: 2", "schema_version: 99")
    });

    let (success, _, stderr) = run_atelier(schema_dir.path(), &["issue", "list"]);
    assert!(!success, "schema drift should block the query");
    assert!(
        stderr.contains(&format!(
            "Invalid record file .atelier/issues/{schema_issue_id}.md"
        )) && stderr.contains("Unsupported schema_version 99"),
        "schema drift diagnostic should name the file: {stderr}"
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

    for args in [
        &["issue", "list"][..],
        &["status"],
        &["issue", "list"],
        &["issue", "show", &malformed_issue_id],
    ] {
        let (success, _, stderr) = run_atelier(malformed_dir.path(), args);
        assert!(!success, "malformed records should block {args:?}");
        assert!(
            stderr.contains(&format!(
                "Invalid record file .atelier/issues/{malformed_issue_id}.md"
            )) && stderr.contains("Invalid YAML front matter"),
            "{args:?} diagnostic should name the file: {stderr}"
        );
    }
}

#[test]
fn test_check_reports_every_invalid_record_file() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let titles = ["First broken", "Second broken"];
    for title in titles {
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", title]);
        assert!(success, "issue create failed: {stderr}");
    }
    let ids = titles
        .map(|title| issue_id_by_title(dir.path(), title))
        .to_vec();
    for (id, title) in ids.iter().zip(titles) {
        corrupt_issue_title_yaml(dir.path(), id, title);
    }
    std::fs::write(
        dir.path().join(".atelier/evidence/atelier-bad1.md"),
        "not an evidence record",
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);

    assert!(!success, "check must fail: {stdout}");
    assert!(stdout.contains("Lint found 3 issue(s)"), "{stdout}");
    for path in [
        format!(".atelier/issues/{}.md", ids[0]),
        format!(".atelier/issues/{}.md", ids[1]),
        ".atelier/evidence/atelier-bad1.md".to_string(),
    ] {
        assert!(
            stdout.contains(&format!("Invalid record file {path}")),
            "missing {path}: {stdout}\n{stderr}"
        );
    }
}

#[test]
fn test_lint_validates_record_markdown_and_ignores_local_artifacts() {
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
schema_version: 2
status: "todo"
title: "Lint canonical source"
updated_at: "2026-06-10T12:00:00+00:00"
---

## Goal

Lint source fixture.

## Done when

Lint rejects malformed canonical state.

## Context

- Command transcript from `atelier check` reports the malformed record.
"#
        .to_string(),
    );
    let markdown = read_canonical_record(dir.path(), "issues", &issue_id);
    let invalid_markdown = markdown.replace(
        "title: \"Lint canonical source\"",
        "title: [Lint canonical source",
    );
    write_canonical_record(dir.path(), "issues", &issue_id, invalid_markdown);
    write_ignored_canonical_artifacts(dir.path(), &issue_id);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
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
        !transcript.contains(".md.lock")
            && !transcript.contains(".tmp")
            && !transcript.contains("canonical-mutation.lock"),
        "lint must ignore local artifacts while reporting malformed committed Markdown: {transcript}"
    );
    assert!(stderr.contains("Lint failed"));
    assert!(!stdout.contains("Lint passed."), "{stdout}");
}

#[test]
fn test_status_works_without_runtime_directory() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    migrate_default_issue_workflow(dir.path());

    let body = "## Goal\n\nDescription\n\n## Done when\n\nStatus reads current work from record files after ignored runtime deletion.\n\n## Context\n\n- `atelier status` prints the current-work issue.";
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
    assert!(success, "status should not need the runtime dir: {stderr}");
    assert!(stdout.contains("Current work:  1 issue(s)"), "{stdout}");
    assert!(stdout.contains(&issue_id));
}

#[test]
fn test_focused_lint_validates_missing_relationship_targets() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let body = "## Goal\n\nFocused lint missing target body.\n\n## Done when\n\nFocused lint reports a missing relationship target.\n\n## Context\n\n- `atelier check <issue-id>` reports the missing issue relationship target.";
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

    let (success, stdout, stderr) = run_atelier(dir.path(), &["check", &issue_id]);
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

    let first_body = "## Goal\n\nFocused lint cycle root body.\n\n## Done when\n\nFocused lint reports dependency cycles.\n\n## Context\n\n- `atelier check <issue-id>` reports relationships.blocks contains a cycle.";
    let second_body = "## Goal\n\nFocused lint cycle leaf body.\n\n## Done when\n\nFocused lint reports dependency cycles.\n\n## Context\n\n- `atelier check <issue-id>` reports relationships.blocks contains a cycle.";
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

    let (success, stdout, stderr) = run_atelier(dir.path(), &["check", &first_id]);
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
        |markdown, _issue_id| markdown.replace("schema_version: 2", "schema_version: 99"),
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
                    "--result",
                    "pass",
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
        "## Goal\n\n{title} body.\n\n## Done when\n\nCanonical lint rejects the targeted malformed record.\n\n## Context\n\n- `atelier check` reports the targeted malformed canonical record."
    );

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", title, "--description", &body],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);

    mutate(dir.path(), &issue_id);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
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
fn test_bundle_apply_records_links() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bundle_path = dir.path().join("bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 2,
  "title": "Bundle apply smoke",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.blocker",
        "title": "Complete prerequisite",
        "issue_type": "task",
        "priority": "medium",
        "status": "done",
        "labels": [
          "bundle"
        ],
        "blocks": [
          {
            "client_ref": "issue.blocked"
          }
        ],
        "body": "\u0023\u0023 Goal\n\nComplete prerequisite\n\n## Done when\n\nRun the focused regression tests."
      },
      {
        "client_ref": "issue.blocked",
        "title": "Blocked bundle work",
        "issue_type": "task",
        "priority": "medium",
        "body": "\u0023\u0023 Goal\n\nBlocked bundle work\n\n## Done when\n\nRun the focused regression tests."
      },
      {
        "client_ref": "issue.epic",
        "title": "Bundle epic",
        "issue_type": "epic",
        "priority": "high",
        "body": "\u0023\u0023 Outcome\n\nBundle epic\n\n## Proof\n\nRun the focused regression tests."
      },
      {
        "client_ref": "issue.work",
        "title": "Implement bundle output",
        "issue_type": "feature",
        "priority": "high",
        "status": "in_progress",
        "parent": {
          "client_ref": "issue.epic"
        },
        "depends_on": [
          {
            "client_ref": "issue.blocker"
          }
        ],
        "notes": [
          {
            "body": "Zulu authored first"
          },
          {
            "body": "Alpha authored second"
          }
        ],
        "body": "\u0023\u0023 Goal\n\nImplement bundle output\n\n## Done when\n\n- summary maps client refs"
      },
      {
        "client_ref": "mission.bundle",
        "title": "Bundle mission",
        "issue_type": "mission",
        "priority": "medium",
        "labels": [
          "bundle",
          "mission"
        ],
        "advances": [
          {
            "client_ref": "issue.epic"
          }
        ],
        "body": "\u0023\u0023 Outcome\n\nMission from bundle\n\n## Success criteria\n\nThe mission outcome is observable."
      }
    ],
    "evidence": [
      {
        "client_ref": "evidence.bundle",
        "title": "Bundle evidence",
        "evidence_type": "test",
        "result": "pass",
        "body": "The apply smoke test passed.",
        "validates": [
          {
            "client_ref": "mission.bundle"
          }
        ],
        "commit": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
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

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(success, "query after bundle apply failed: {stderr}");

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
    let note_bodies = std::fs::read_to_string(
        dir.path()
            .join(".atelier/issues")
            .join(format!("{work_id}.md")),
    )
    .unwrap();
    assert!(note_bodies.contains("## Notes\n\n- "), "{note_bodies}");
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
  "schema_version": 2,
  "title": "Duplicate normalized relationship",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.blocker",
        "title": "Bundle blocker",
        "blocks": [
          {
            "client_ref": "issue.blocked"
          }
        ],
        "body": "\u0023\u0023 Goal\n\nBundle blocker\n\n## Done when\n\nRun the focused regression tests."
      },
      {
        "client_ref": "issue.blocked",
        "title": "Bundle blocked",
        "depends_on": [
          {
            "client_ref": "issue.blocker"
          }
        ],
        "body": "\u0023\u0023 Goal\n\nBundle blocked\n\n## Done when\n\nRun the focused regression tests."
      }
    ]
  }
}"#,
    )
    .unwrap();

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
  "schema_version": 2,
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
  "schema_version": 2,
  "title": "Invalid mission parent",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.parent",
        "title": "Parent epic",
        "issue_type": "epic",
        "body": "\u0023\u0023 Outcome\n\nParent epic\n\n## Proof\n\nRun the focused regression tests."
      },
      {
        "client_ref": "mission.child",
        "title": "Invalid mission child",
        "issue_type": "mission",
        "parent": {
          "client_ref": "issue.parent"
        },
        "body": "\u0023\u0023 Outcome\n\nInvalid mission child\n\n## Success criteria\n\nThe mission outcome is observable."
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
fn test_bundle_apply_rejects_executable_missions_with_open_blockers_and_preserves_state() {
    {
        let status = "in_progress";
        let dir = tempdir().unwrap();
        init_atelier(dir.path());
        let bundle_path = dir.path().join(format!("executable-mission-{status}.json"));
        std::fs::write(
            &bundle_path,
            format!(
                r#"{{
  "schema": "atelier.bundle",
  "schema_version": 2,
  "title": "Executable mission without receipts",
  "resources": {{
    "issues": [
      {{
        "client_ref": "issue.blocker",
        "title": "Open bundled blocker", "body": "\u0023\u0023 Goal\n\nOpen blocker\n\n## Done when\n\nFocused test passes.",
        "issue_type": "task",
        "status": "todo"
      }},
      {{
        "client_ref": "mission.executable",
        "title": "Unreviewed executable mission", "body": "\u0023\u0023 Outcome\n\nMission result\n\n## Success criteria\n\nChecks pass.",
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
        let canonical_before = canonical_tree_snapshot(&state_dir);

        let (success, _stdout, stderr) = run_atelier(
            dir.path(),
            &["bundle", "apply", bundle_path.to_str().unwrap(), "--yes"],
        );

        assert!(!success, "blocked {status} mission must be rejected");
        assert!(
            stderr.contains("workflow_mission_dependency_bypass"),
            "{stderr}"
        );
        assert_eq!(stderr.matches("Next:").count(), 1, "{stderr}");
        assert_eq!(canonical_tree_snapshot(&state_dir), canonical_before);
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
  "schema_version": 2,
  "title": "Initial statuses",
  "resources": {
    "issues": [
      {
        "client_ref": "mission.draft",
        "title": "Draft bundled mission",
        "issue_type": "mission",
        "status": "draft",
        "body": "\u0023\u0023 Outcome\n\nDraft bundled mission\n\n## Success criteria\n\nThe mission outcome is observable."
      },
      {
        "client_ref": "issue.todo",
        "title": "Todo bundled task",
        "issue_type": "task",
        "status": "todo",
        "body": "\u0023\u0023 Goal\n\nTodo bundled task\n\n## Done when\n\nRun the focused regression tests."
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
        "applied initial statuses must load cleanly: {stderr}"
    );
}

#[test]
fn test_bundle_apply_accepts_graph_edit_to_ready_mission() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    let mission_id = create_mission_fixture(dir.path(), "Reviewed executable mission");
    let bundle_path = dir.path().join("stale-reviewed-mission.json");
    std::fs::write(
        &bundle_path,
        format!(
            r#"{{
  "schema": "atelier.bundle",
  "schema_version": 2,
  "title": "Stale reviewed mission",
  "resources": {{
    "issues": [
      {{
        "client_ref": "issue.completed-blocker",
        "title": "Completed graph addition", "body": "\u0023\u0023 Goal\n\nCompleted graph addition\n\n## Done when\n\nTests pass.",
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
    let canonical_before = canonical_tree_snapshot(&state_dir);

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["bundle", "apply", bundle_path.to_str().unwrap(), "--yes"],
    );

    assert!(success, "ordinary graph edit should apply: {stderr}");
    assert_ne!(canonical_tree_snapshot(&state_dir), canonical_before);
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
  "schema_version": 2,
  "title": "Paused bundle",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.bulk",
        "title": "Bulk installed task",
        "issue_type": "task",
        "status": "todo",
        "body": "\u0023\u0023 Goal\n\nBulk installed task\n\n## Done when\n\nRun the focused regression tests."
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
fn test_bundle_backup_cleanup_failure_reports_committed_success_and_blocks_retry() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    let bundle_path = dir.path().join("cleanup-failure-bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 2,
  "title": "Cleanup failure bundle",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.cleanup",
        "title": "Cleanup committed task",
        "issue_type": "task",
        "status": "todo",
        "body": "\u0023\u0023 Goal\n\nCleanup committed task\n\n## Done when\n\nRun the focused regression tests."
      }
    ],
    "evidence": [
      {
        "client_ref": "evidence.cleanup",
        "title": "Cleanup committed evidence",
        "evidence_type": "test",
        "result": "pass",
        "body": "Both bundle directories committed before cleanup failed.",
        "commit": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
      }
    ]
  }
}"#,
    )
    .unwrap();
    let state_dir = dir.path().join(".atelier");
    let issue_count = count_markdown_records(dir.path(), "issues");
    let evidence_count = count_markdown_records(dir.path(), "evidence");

    let (success, stdout, stderr) = run_atelier_with_env(
        dir.path(),
        &["bundle", "apply", bundle_path.to_str().unwrap(), "--yes"],
        &[("ATELIER_TEST_BUNDLE_BACKUP_CLEANUP_FAILURE", "1")],
    );

    assert!(success, "post-commit cleanup must not fail apply: {stderr}");
    assert!(stdout.contains("Bundle applied."), "{stdout}");
    assert!(
        stderr.contains("Bundle apply committed successfully"),
        "{stderr}"
    );
    assert!(
        stderr.contains("Do not retry this create-only bundle"),
        "{stderr}"
    );
    assert_eq!(
        count_markdown_records(dir.path(), "issues"),
        issue_count + 1
    );
    assert_eq!(
        count_markdown_records(dir.path(), "evidence"),
        evidence_count + 1
    );
    assert!(canonical_directory_contains(
        dir.path(),
        "issues",
        "Cleanup committed task"
    ));
    assert!(canonical_directory_contains(
        dir.path(),
        "evidence",
        "Cleanup committed evidence"
    ));

    let mut backups = std::fs::read_dir(state_dir.join("runtime"))
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(".atelier-bundle-backup-"))
        })
        .collect::<Vec<_>>();
    backups.sort();
    assert_eq!(
        backups.len(),
        1,
        "expected one retained backup: {backups:?}"
    );
    assert!(
        stderr.contains(&backups[0].display().to_string()),
        "{stderr}"
    );
    let ignored = std::process::Command::new("git")
        .current_dir(dir.path())
        .args(["check-ignore", "-q", backups[0].to_str().unwrap()])
        .status()
        .unwrap();
    assert!(ignored.success(), "retained backup must be git-ignored");

    let (success, _, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(
        success,
        "committed bundle must repair and check cleanly: {stderr}"
    );
    let counts_before_retry = (
        count_markdown_records(dir.path(), "issues"),
        count_markdown_records(dir.path(), "evidence"),
    );
    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["bundle", "apply", bundle_path.to_str().unwrap(), "--yes"],
    );
    assert!(!success, "retry with retained backup must be refused");
    assert!(stderr.contains("bundle_recovery_required"), "{stderr}");
    assert!(
        stderr.contains(&backups[0].display().to_string()),
        "{stderr}"
    );
    assert_eq!(
        (
            count_markdown_records(dir.path(), "issues"),
            count_markdown_records(dir.path(), "evidence"),
        ),
        counts_before_retry,
        "refused retry created duplicate records"
    );
}

#[test]
fn test_bundle_stage_cleanup_failure_reports_committed_success_and_blocks_retry() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    let bundle_path = dir.path().join("stage-cleanup-failure-bundle.json");
    std::fs::write(
        &bundle_path,
        r#"{
  "schema": "atelier.bundle",
  "schema_version": 2,
  "title": "Stage cleanup failure bundle",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.stage-cleanup",
        "title": "Stage cleanup committed task",
        "issue_type": "task",
        "status": "todo",
        "body": "\u0023\u0023 Goal\n\nStage cleanup committed task\n\n## Done when\n\nRun the focused regression tests."
      }
    ],
    "evidence": [
      {
        "client_ref": "evidence.stage-cleanup",
        "title": "Stage cleanup committed evidence",
        "evidence_type": "test",
        "result": "pass",
        "body": "Both live directories committed before stage cleanup failed.",
        "commit": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
      }
    ]
  }
}"#,
    )
    .unwrap();
    let state_dir = dir.path().join(".atelier");
    let counts_before = (
        count_markdown_records(dir.path(), "issues"),
        count_markdown_records(dir.path(), "evidence"),
    );

    let (success, stdout, stderr) = run_atelier_with_env(
        dir.path(),
        &["bundle", "apply", bundle_path.to_str().unwrap(), "--yes"],
        &[("ATELIER_TEST_BUNDLE_STAGE_CLEANUP_FAILURE", "1")],
    );

    assert!(
        success,
        "post-commit stage cleanup must not fail apply: {stderr}"
    );
    assert!(stdout.contains("Bundle applied."), "{stdout}");
    assert!(
        stderr.contains("Bundle apply committed successfully")
            && stderr.contains("staging cleanup failed"),
        "{stderr}"
    );
    assert!(
        stderr.contains("Do not retry this create-only bundle"),
        "{stderr}"
    );
    assert_eq!(
        (
            count_markdown_records(dir.path(), "issues"),
            count_markdown_records(dir.path(), "evidence"),
        ),
        (counts_before.0 + 1, counts_before.1 + 1)
    );
    assert!(canonical_directory_contains(
        dir.path(),
        "issues",
        "Stage cleanup committed task"
    ));
    assert!(canonical_directory_contains(
        dir.path(),
        "evidence",
        "Stage cleanup committed evidence"
    ));

    let mut stages = std::fs::read_dir(dir.path())
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(".atelier-bundle-stage-"))
        })
        .collect::<Vec<_>>();
    stages.sort();
    assert_eq!(stages.len(), 1, "expected one retained stage: {stages:?}");
    assert!(stages[0].is_dir());
    assert!(
        stderr.contains(&stages[0].display().to_string()),
        "{stderr}"
    );
    let ignored = std::process::Command::new("git")
        .current_dir(dir.path())
        .args(["check-ignore", "-q", stages[0].to_str().unwrap()])
        .status()
        .unwrap();
    assert!(ignored.success(), "retained stage must be git-ignored");

    let (success, _, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(
        success,
        "committed bundle must repair and check cleanly: {stderr}"
    );
    let counts_before_retry = (
        count_markdown_records(dir.path(), "issues"),
        count_markdown_records(dir.path(), "evidence"),
    );
    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["bundle", "apply", bundle_path.to_str().unwrap(), "--yes"],
    );
    assert!(!success, "retry with retained stage must be refused");
    assert!(stderr.contains("bundle_recovery_required"), "{stderr}");
    assert!(
        stderr.contains(&stages[0].display().to_string()),
        "{stderr}"
    );
    assert_eq!(
        (
            count_markdown_records(dir.path(), "issues"),
            count_markdown_records(dir.path(), "evidence"),
        ),
        counts_before_retry,
        "refused retry created duplicate records"
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
  "schema_version": 2,
  "title": "Mid apply failure",
  "resources": {
    "issues": [
      {
        "client_ref": "issue.invalid-parent",
        "title": "Should not persist",
        "issue_type": "task",
        "priority": "high",
        "parent": {
          "client_ref": "evidence.invalid-parent"
        },
        "body": "\u0023\u0023 Goal\n\nShould not persist\n\n## Done when\n\nRun the focused regression tests."
      }
    ],
    "evidence": [
      {
        "client_ref": "evidence.invalid-parent",
        "title": "Should not persist evidence",
        "evidence_type": "test",
        "result": "pass",
        "body": "This staged record must not install.",
        "commit": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
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
    let malformed = markdown.replace("\n## Done when\n\nDone when was not specified.\n", "\n");
    std::fs::write(&issue_path, malformed).unwrap();
    commit_all(dir.path(), "malformed issue section");

    let (lint_success, lintstdout, lint_stderr) = run_atelier(dir.path(), &["check"]);
    assert!(!lint_success, "lint should report malformed issue sections");
    let lint_transcript = format!("{lintstdout}\n{lint_stderr}");
    for needle in [
        "Missing required issue body section 'Done when'",
        &issue_id,
        "section Done when",
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
        "Missing required issue body section 'Done when'",
        &issue_id,
        "section Done when",
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

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(success, "list failed: {stderr}");
    assert!(stdout.contains("validation"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["work", "ready"]);
    assert!(success, "ready failed: {stderr}");
    assert!(stdout.contains(&issue_id));
    assert!(stdout.contains("Typed issue"));

    let issue_record = std::fs::read_to_string(
        dir.path()
            .join(".atelier/issues")
            .join(format!("{issue_id}.md")),
    )
    .unwrap();
    assert!(issue_record.contains("issue_type: \"validation\"\n"));
    assert!(issue_record.contains("labels:\n- \"epic\"\n"));
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

// ==================== atelier start ====================

fn start_fixture() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    dir
}

fn create_issue(dir: &std::path::Path, args: &[&str]) -> String {
    let title = args[0];
    let mut command = vec!["issue", "create"];
    command.extend_from_slice(args);
    let (success, _, stderr) = run_atelier(dir, &command);
    assert!(success, "issue create {title} failed: {stderr}");
    issue_id_by_title(dir, title)
}

fn issue_front_matter(dir: &std::path::Path, issue_id: &str) -> serde_json::Value {
    canonical_record_front_matter(dir, "issues", issue_id)
}

#[test]
fn test_start_child_checks_out_parent_epic_branch_and_records_it_on_the_epic() {
    let dir = start_fixture();
    let epic_id = create_issue(dir.path(), &["Start epic", "--issue-type", "epic"]);
    let child_id = create_issue(dir.path(), &["Start child", "--parent", &epic_id]);
    commit_all(dir.path(), "start fixture");
    let head_before = git_rev_parse(dir.path(), "HEAD");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &child_id]);

    assert!(success, "start failed: {stderr}");
    assert!(
        stdout.contains(&format!("git switch -c epic/{epic_id}")),
        "{stdout}"
    );
    assert!(
        stdout.contains(&format!("Applied transition start to {child_id}")),
        "{stdout}"
    );
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    assert_eq!(
        git_rev_parse(dir.path(), "HEAD"),
        head_before,
        "start must not commit"
    );
    let epic = issue_front_matter(dir.path(), &epic_id);
    assert_eq!(
        epic["workflow_branch"],
        serde_json::json!({ "work_branch": format!("epic/{epic_id}"), "branch_base": "main" })
    );
    assert_eq!(
        epic["status"], "draft",
        "starting a child leaves the epic status"
    );
    let child = issue_front_matter(dir.path(), &child_id);
    assert_eq!(child["status"], "in_progress");
    assert!(child
        .get("fields")
        .is_none_or(|fields| fields.get("workflow_branch").is_none()));
}

#[test]
fn test_start_standalone_issue_and_epic_own_their_branches() {
    for (issue_type, prefix, status) in [("bug", "bug", "in_progress"), ("epic", "epic", "draft")] {
        let dir = start_fixture();
        let issue_id = create_issue(dir.path(), &["Owner", "--issue-type", issue_type]);
        commit_all(dir.path(), "start fixture");

        let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &issue_id]);

        assert!(success, "{issue_type} start failed: {stderr}");
        let branch = format!("{prefix}/{issue_id}");
        assert!(
            stdout.contains(&format!("git switch -c {branch}")),
            "{stdout}"
        );
        assert_eq!(git_current_branch(dir.path()), branch);
        let record = issue_front_matter(dir.path(), &issue_id);
        assert_eq!(record["workflow_branch"]["work_branch"], branch);
        assert_eq!(record["workflow_branch"]["branch_base"], "main");
        // A draft epic has no start transition until it is made ready.
        assert_eq!(record["status"], status, "{stdout}");
    }
}

#[test]
fn test_start_rerun_is_idempotent() {
    let dir = start_fixture();
    let issue_id = create_issue(dir.path(), &["Rerun", "--issue-type", "task"]);
    commit_all(dir.path(), "start fixture");
    let (success, _, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "first start failed: {stderr}");
    let record_before = read_canonical_record(dir.path(), "issues", &issue_id);
    let status_before = git_status_short(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &issue_id]);

    assert!(success, "second start failed: {stderr}");
    assert!(stdout.contains("no git operations"), "{stdout}");
    assert!(
        stdout.contains("Status:   in_progress (no start transition from this status)"),
        "{stdout}"
    );
    assert!(!stdout.contains("Recorded workflow_branch"), "{stdout}");
    assert_eq!(
        read_canonical_record(dir.path(), "issues", &issue_id),
        record_before
    );
    assert_eq!(git_status_short(dir.path()), status_before);
    assert_eq!(git_current_branch(dir.path()), format!("task/{issue_id}"));
}

#[test]
fn test_start_refuses_dirty_checkout_before_any_change() {
    let dir = start_fixture();
    let issue_id = create_issue(dir.path(), &["Dirty start", "--issue-type", "task"]);
    commit_all(dir.path(), "start fixture");
    std::fs::write(dir.path().join("scratch.txt"), "uncommitted").unwrap();
    let record_before = read_canonical_record(dir.path(), "issues", &issue_id);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &issue_id]);

    assert!(!success, "dirty start should fail:\n{stdout}");
    assert!(stderr.contains("start_checkout_dirty"), "{stderr}");
    assert!(stderr.contains("scratch.txt"), "{stderr}");
    assert!(!stdout.contains("git switch"), "{stdout}");
    assert_eq!(git_current_branch(dir.path()), "main");
    assert_eq!(
        read_canonical_record(dir.path(), "issues", &issue_id),
        record_before
    );
    assert_eq!(git_status_short(dir.path()).trim(), "?? scratch.txt");
}

#[test]
fn test_start_validator_failure_does_not_switch_branches() {
    let dir = start_fixture();
    let issue_id = create_issue(dir.path(), &["Gated start", "--issue-type", "task"]);
    let blocker_id = create_issue(dir.path(), &["Open blocker", "--issue-type", "task"]);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &issue_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );
    assert!(success, "blocker link failed: {stderr}");
    commit_all(dir.path(), "start fixture");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &issue_id]);

    assert!(!success, "blocked start should fail:\n{stdout}");
    assert!(
        stdout.contains("fail  blockers.transitive_none_open"),
        "{stdout}"
    );
    assert!(stderr.contains("Transition 'start' is blocked"), "{stderr}");
    assert!(!stdout.contains("git switch"), "{stdout}");
    assert_eq!(git_current_branch(dir.path()), "main");
    let output = Command::new("git")
        .current_dir(dir.path())
        .args(["branch", "--list", &format!("task/{issue_id}")])
        .output()
        .unwrap();
    assert!(output.stdout.is_empty(), "branch must not be created");
    assert!(git_status_short(dir.path()).is_empty());
}

#[test]
fn test_start_refuses_missions() {
    let dir = start_fixture();
    let mission_id = create_issue(dir.path(), &["Coordinated", "--issue-type", "mission"]);
    commit_all(dir.path(), "start fixture");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &mission_id]);

    assert!(!success, "mission start should fail:\n{stdout}");
    assert!(
        stderr.contains("missions are coordinated, not worked on a branch"),
        "{stderr}"
    );
    assert_eq!(git_current_branch(dir.path()), "main");
    assert!(git_status_short(dir.path()).is_empty());
}

#[test]
fn test_start_refuses_existing_owner_branch_without_the_issue_record() {
    let dir = start_fixture();
    let epic_id = create_issue(dir.path(), &["Branch owner", "--issue-type", "epic"]);
    commit_all(dir.path(), "epic fixture");
    let (success, _, stderr) = run_atelier(dir.path(), &["start", &epic_id]);
    assert!(success, "epic start failed: {stderr}");
    commit_all(dir.path(), "epic started");
    let output = Command::new("git")
        .current_dir(dir.path())
        .args(["switch", "main"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let child_id = create_issue(dir.path(), &["Late child", "--parent", &epic_id]);
    commit_all(dir.path(), "child created on main only");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &child_id]);

    assert!(
        !success,
        "start should refuse a branch without the record:\n{stdout}"
    );
    assert!(stderr.contains("start_branch_missing_record"), "{stderr}");
    assert_eq!(git_current_branch(dir.path()), "main");
    assert!(git_status_short(dir.path()).is_empty());
}

#[test]
fn test_start_reads_the_owner_branch_not_a_same_named_tag() {
    let dir = start_fixture();
    let epic_id = create_issue(dir.path(), &["Tagged owner", "--issue-type", "epic"]);
    commit_all(dir.path(), "epic fixture");
    git_ok(dir.path(), &["switch", "-c", "develop"]);
    let (success, _, stderr) = run_atelier(dir.path(), &["start", &epic_id]);
    assert!(success, "epic start failed: {stderr}");
    commit_all(dir.path(), "epic started");
    git_ok(dir.path(), &["switch", "main"]);
    let child_id = create_issue(dir.path(), &["Tag-only child", "--parent", &epic_id]);
    commit_all(dir.path(), "child created on main only");
    git_ok(dir.path(), &["tag", &format!("epic/{epic_id}"), "main"]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &child_id]);

    assert!(
        !success,
        "start must read the owner branch, not a same-named tag:\n{stdout}"
    );
    assert!(stderr.contains("start_branch_missing_record"), "{stderr}");
    assert_eq!(git_current_branch(dir.path()), "main");
    assert!(git_status_short(dir.path()).is_empty());
}

fn git_ok(dir: &std::path::Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_start_on_existing_owner_branch_preserves_its_recorded_base() {
    let dir = start_fixture();
    let epic_id = create_issue(dir.path(), &["Shared owner", "--issue-type", "epic"]);
    let first_id = create_issue(dir.path(), &["First child", "--parent", &epic_id]);
    let second_id = create_issue(dir.path(), &["Second child", "--parent", &epic_id]);
    commit_all(dir.path(), "start fixture");
    git_ok(dir.path(), &["switch", "-c", "develop"]);
    let (success, _, stderr) = run_atelier(dir.path(), &["start", &first_id]);
    assert!(success, "first start failed: {stderr}");
    commit_all(dir.path(), "first child started");
    git_ok(dir.path(), &["switch", "main"]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &second_id]);

    assert!(success, "second start failed: {stderr}");
    assert!(
        stdout.contains(&format!("git switch epic/{epic_id}")),
        "{stdout}"
    );
    assert!(!stdout.contains("Recorded workflow_branch"), "{stdout}");
    assert!(stdout.contains("Base:     develop"), "{stdout}");
    assert_eq!(git_current_branch(dir.path()), format!("epic/{epic_id}"));
    let epic = issue_front_matter(dir.path(), &epic_id);
    assert_eq!(
        epic["workflow_branch"],
        serde_json::json!({ "work_branch": format!("epic/{epic_id}"), "branch_base": "develop" })
    );
    assert_eq!(
        issue_front_matter(dir.path(), &first_id)["status"],
        "in_progress"
    );
    assert_eq!(
        issue_front_matter(dir.path(), &second_id)["status"],
        "in_progress"
    );
    assert_eq!(
        git_status_short(dir.path()).trim(),
        format!("M .atelier/issues/{second_id}.md")
    );
}

#[test]
fn test_start_checks_blockers_on_the_target_branch_before_switching() {
    let dir = start_fixture();
    let epic_id = create_issue(dir.path(), &["Owner epic", "--issue-type", "epic"]);
    let first_id = create_issue(dir.path(), &["Started child", "--parent", &epic_id]);
    let gated_id = create_issue(dir.path(), &["Gated child", "--parent", &epic_id]);
    let blocker_id = create_issue(dir.path(), &["Branch blocker", "--issue-type", "task"]);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &gated_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );
    assert!(success, "blocker link failed: {stderr}");
    commit_all(dir.path(), "start fixture");
    let (success, _, stderr) = run_atelier(dir.path(), &["start", &first_id]);
    assert!(success, "first start failed: {stderr}");
    commit_all(dir.path(), "first child started");
    git_ok(dir.path(), &["switch", "main"]);
    set_dependency_fixture_status(dir.path(), &blocker_id, "done");
    commit_all(dir.path(), "blocker done on main only");
    let gated_on_branch = Command::new("git")
        .current_dir(dir.path())
        .args([
            "show",
            &format!("epic/{epic_id}:.atelier/issues/{gated_id}.md"),
        ])
        .output()
        .unwrap()
        .stdout;

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &gated_id]);

    assert!(
        !success,
        "start should check the branch's blocker:\n{stdout}"
    );
    assert!(
        stdout.contains("fail  blockers.transitive_none_open"),
        "{stdout}"
    );
    assert!(stdout.contains(&blocker_id), "{stdout}");
    assert!(stderr.contains("Transition 'start' is blocked"), "{stderr}");
    assert!(!stdout.contains("git switch"), "{stdout}");
    assert_eq!(git_current_branch(dir.path()), "main");
    assert!(git_status_short(dir.path()).is_empty());
    let gated_after = Command::new("git")
        .current_dir(dir.path())
        .args([
            "show",
            &format!("epic/{epic_id}:.atelier/issues/{gated_id}.md"),
        ])
        .output()
        .unwrap()
        .stdout;
    assert_eq!(gated_after, gated_on_branch);
}

#[test]
fn test_start_refuses_done_issues_without_changes() {
    let dir = start_fixture();
    let issue_id = create_issue(dir.path(), &["Finished", "--issue-type", "task"]);
    set_dependency_fixture_status(dir.path(), &issue_id, "done");
    commit_all(dir.path(), "done fixture");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &issue_id]);

    assert!(!success, "done start should fail:\n{stdout}");
    assert!(stderr.contains("start_issue_done"), "{stderr}");
    assert!(!stdout.contains("git switch"), "{stdout}");
    assert_eq!(git_current_branch(dir.path()), "main");
    assert!(git_status_short(dir.path()).is_empty());
}
