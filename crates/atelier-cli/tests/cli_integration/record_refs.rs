//! Record references and the errors about them, the same in every command.

use super::*;

/// Runs atelier and returns its exit code and stderr.
fn exit_code(dir: &Path, args: &[&str]) -> (Option<i32>, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir)
        .args(args)
        .output()
        .expect("Failed to execute atelier");
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

fn create(dir: &Path, args: &[&str]) {
    let (success, _, stderr) = run_atelier(dir, args);
    assert!(success, "fixture {args:?} failed: {stderr}");
}

struct Fixture {
    dir: tempfile::TempDir,
    mission: String,
    parent: String,
    child: String,
    evidence: String,
    review: String,
}

/// A mission, a parent that advances it with one child, one evidence record
/// and one review, all committed.
fn fixture() -> Fixture {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    create(
        dir.path(),
        &[
            "issue",
            "create",
            "Reference mission",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nOutcome.\n\n## Success criteria\n\nCriteria.",
        ],
    );
    let mission = issue_id_by_title(dir.path(), "Reference mission");
    create(
        dir.path(),
        &[
            "issue",
            "create",
            "Reference parent",
            "--issue-type",
            "feature",
        ],
    );
    let parent = issue_id_by_title(dir.path(), "Reference parent");
    create(
        dir.path(),
        &["issue", "create", "Reference child", "--parent", &parent],
    );
    let child = issue_id_by_title(dir.path(), "Reference child");
    create(dir.path(), &["issue", "link", &parent, &mission]);
    create(
        dir.path(),
        &[
            "evidence",
            "record",
            "--result",
            "pass",
            "--kind",
            "test",
            "reference evidence",
        ],
    );
    let evidence = record_id_by_title(dir.path(), "evidence", "reference evidence");
    commit_all(dir.path(), "reference fixture");
    create(
        dir.path(),
        &[
            "review",
            "record",
            &parent,
            "--identity",
            "reviewer",
            "reference review",
        ],
    );
    let review = record_id_by_title(dir.path(), "reviews", "reference review");
    commit_all(dir.path(), "reference review");
    Fixture {
        dir,
        mission,
        parent,
        child,
        evidence,
        review,
    }
}

#[test]
fn short_ids_resolve_ignoring_case_for_issues_evidence_and_reviews() {
    let f = fixture();
    let dir = f.dir.path();
    let upper = |id: &str| issue_key(id).to_ascii_uppercase();

    let (success, stdout, stderr) = run_atelier(dir, &["issue", "show", &upper(&f.child)]);
    assert!(success, "{stderr}");
    assert!(stdout.starts_with(&format!("{} ", f.child)), "{stdout}");
    let (success, stdout, stderr) =
        run_atelier(dir, &["issue", "show", &f.child.to_ascii_uppercase()]);
    assert!(success, "{stderr}");
    assert!(stdout.starts_with(&format!("{} ", f.child)), "{stdout}");

    let (success, stdout, stderr) = run_atelier(dir, &["evidence", "show", &upper(&f.evidence)]);
    assert!(success, "{stderr}");
    assert!(stdout.starts_with(&format!("{} ", f.evidence)), "{stdout}");

    let (success, stdout, stderr) = run_atelier(dir, &["review", "show", &upper(&f.review)]);
    assert!(success, "{stderr}");
    assert!(stdout.contains(&f.review), "{stdout}");

    let (success, stdout, stderr) = run_atelier(dir, &["review", "list", issue_key(&f.parent)]);
    assert!(success, "{stderr}");
    assert!(stdout.contains(&f.review), "{stdout}");

    let (success, stdout, stderr) = run_atelier(dir, &["work", "parent", &upper(&f.parent)]);
    assert!(success, "{stderr}");
    assert!(stdout.contains(&f.child), "{stdout}");
}

#[test]
fn unknown_ids_give_one_not_found_message_and_exit_1_everywhere() {
    let f = fixture();
    let dir = f.dir.path();
    let unknown = "qqqqqqqq";
    for (args, kind, list) in [
        (
            vec!["issue", "show", unknown],
            "issue",
            "atelier issue list",
        ),
        (
            vec!["issue", "note", unknown, "text"],
            "issue",
            "atelier issue list",
        ),
        (
            vec!["issue", "link", &f.child, unknown],
            "issue",
            "atelier issue list",
        ),
        (
            vec!["work", "parent", unknown],
            "issue",
            "atelier issue list",
        ),
        (vec!["check", unknown], "issue", "atelier issue list"),
        (vec!["land", unknown], "issue", "atelier issue list"),
        (vec!["start", unknown], "issue", "atelier issue list"),
        (
            vec!["review", "list", unknown],
            "issue",
            "atelier issue list",
        ),
        (
            vec!["evidence", "show", unknown],
            "evidence record",
            "atelier evidence list",
        ),
        (
            vec!["evidence", "attach", unknown, "issue", &f.child],
            "evidence record",
            "atelier evidence list",
        ),
        (
            vec!["review", "show", unknown],
            "review",
            "atelier review list",
        ),
    ] {
        let (code, stderr) = exit_code(dir, &args);
        assert_eq!(code, Some(1), "{args:?}: {stderr}");
        assert!(
            stderr.contains(&format!("{kind} {unknown} not found. Next: {list}")),
            "{args:?}: {stderr}"
        );
    }

    let prefix = &issue_key(&f.child)[..3];
    let (code, stderr) = exit_code(dir, &["issue", "show", prefix]);
    assert_eq!(code, Some(1), "{stderr}");
    assert!(
        stderr.contains(&format!("issue {prefix} not found. Did you mean "))
            && stderr.contains(&f.child),
        "{stderr}"
    );
}

#[test]
fn wrong_kind_short_ids_name_the_actual_kind_and_its_show_command() {
    let f = fixture();
    let dir = f.dir.path();
    let (code, stderr) = exit_code(dir, &["issue", "show", issue_key(&f.evidence)]);
    assert_eq!(code, Some(1), "{stderr}");
    assert!(
        stderr.contains(&format!(
            "{0} is an evidence record, not an issue. Use `atelier evidence show {0}`.",
            f.evidence
        )),
        "{stderr}"
    );
    let (code, stderr) = exit_code(dir, &["evidence", "show", issue_key(&f.review)]);
    assert_eq!(code, Some(1), "{stderr}");
    assert!(
        stderr.contains(&format!(
            "{0} is a review, not an evidence record. Use `atelier review show {0}`.",
            f.review
        )),
        "{stderr}"
    );
    let (code, stderr) = exit_code(dir, &["check", issue_key(&f.review)]);
    assert_eq!(code, Some(1), "{stderr}");
    assert!(
        stderr.contains(&format!("{} is a review, not an issue.", f.review)),
        "{stderr}"
    );
}

#[test]
fn scoped_views_name_the_view_that_shows_a_refused_issue() {
    let f = fixture();
    let dir = f.dir.path();
    let (code, stderr) = exit_code(dir, &["work", "mission", &f.parent]);
    assert_eq!(code, Some(1), "{stderr}");
    assert!(
        stderr.contains(&format!(
            "{0} is a feature, not a mission. Use `atelier work parent {0}` or `atelier work mission {1}`.",
            f.parent, f.mission
        )),
        "{stderr}"
    );
    let (code, stderr) = exit_code(dir, &["work", "parent", &f.mission]);
    assert_eq!(code, Some(1), "{stderr}");
    assert!(
        stderr.contains(&format!(
            "{0} is a mission, not a parent. Use `atelier work mission {0}`.",
            f.mission
        )),
        "{stderr}"
    );
    let (code, stderr) = exit_code(dir, &["work", "parent", &f.child]);
    assert_eq!(code, Some(1), "{stderr}");
    assert!(
        stderr.contains(&format!(
            "{} has no children, so it is not a parent. Use `atelier work parent {}` or `atelier work mission {}`.",
            f.child, f.parent, f.mission
        )),
        "{stderr}"
    );
}

#[test]
fn work_mission_gives_a_vowel_initial_issue_type_its_article() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let workflow_path = dir.path().join(".atelier/workflow.yaml");
    let workflow = std::fs::read_to_string(&workflow_path)
        .unwrap()
        .replace(
            "  validation:\n    label: Validation\n",
            "  validation:\n    label: Validation\n  incident:\n    label: Incident\n",
        )
        .replace("    - task\n", "    - task\n    - incident\n");
    std::fs::write(&workflow_path, workflow).unwrap();
    create(
        dir.path(),
        &["issue", "create", "Outage", "--issue-type", "incident"],
    );
    let incident = issue_id_by_title(dir.path(), "Outage");
    let (code, stderr) = exit_code(dir.path(), &["work", "mission", &incident]);
    assert_eq!(code, Some(1), "{stderr}");
    assert!(
        stderr.contains(&format!(
            "{0} is an incident, not a mission. Use `atelier issue show {0}`.",
            incident
        )),
        "{stderr}"
    );
}

#[test]
fn invalid_status_and_category_list_the_workflow_values() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (code, stderr) = exit_code(dir.path(), &["issue", "list", "--status", "bogus"]);
    assert_eq!(code, Some(1), "{stderr}");
    assert!(
        stderr.contains("Invalid status 'bogus'. Valid values: ")
            && stderr.contains("backlog")
            && stderr.contains("done")
            && stderr.trim_end().ends_with(", all"),
        "{stderr}"
    );
    let (code, stderr) = exit_code(dir.path(), &["issue", "list", "--category", "open"]);
    assert_eq!(code, Some(1), "{stderr}");
    assert!(
        stderr.contains("Invalid category 'open'. Valid values: active, blocked, done, todo"),
        "{stderr}"
    );
}

#[test]
fn mutually_exclusive_flags_are_usage_errors() {
    let f = fixture();
    let dir = f.dir.path();
    for (args, conflict) in [
        (
            vec!["work", "mission", &f.mission, "--ready", "--blocked"],
            "the argument '--ready' cannot be used with '--blocked'",
        ),
        (
            vec!["work", "mission", &f.mission, "--done", "--all"],
            "the argument '--done' cannot be used with '--all'",
        ),
        (
            vec![
                "issue",
                "update",
                &f.child,
                "--parent",
                &f.parent,
                "--no-parent",
            ],
            "the argument '--parent <PARENT>' cannot be used with '--no-parent'",
        ),
    ] {
        let (code, stderr) = exit_code(dir, &args);
        assert_eq!(code, Some(2), "{args:?}: {stderr}");
        assert!(stderr.contains(conflict), "{args:?}: {stderr}");
    }
}
