//! Every command that writes records checks them against the rules
//! `atelier check` applies and refuses before writing anything, instead of
//! accepting a record that then blocks every later command (atelier-ilba).
use std::{fs, path::Path};
use tempfile::{tempdir, TempDir};
#[path = "support/cli.rs"]
mod support_cli;
use support_cli::{cli, commit, git, ok};

/// A repository under the starter policy with one committed base commit.
fn repository() -> TempDir {
    let dir = tempdir().unwrap();
    git(dir.path(), &["init", "-qb", "main"]);
    git(dir.path(), &["config", "user.name", "Author"]);
    git(dir.path(), &["config", "user.email", "author@example.test"]);
    ok(dir.path(), &["init", "--slug", "atelier"]);
    commit(dir.path(), "init");
    dir
}

fn issue(dir: &Path, title: &str) -> String {
    ok(
        dir,
        &[
            "-q",
            "issue",
            "create",
            title,
            "--body",
            "## Goal\n\nShip it.\n\n## Done when\n\nIt ships.",
        ],
    )
}

/// The issue `title`, created and committed on a new branch `other`, which
/// is left for `main` again.
fn issue_on_other_branch(dir: &Path, title: &str) -> String {
    git(dir, &["switch", "-qc", "other"]);
    let id = issue(dir, title);
    commit(dir, "an issue only on other");
    git(dir, &["switch", "-q", "main"]);
    id
}

fn record(dir: &Path, id: &str) -> String {
    fs::read_to_string(dir.join(format!(".atelier/issues/{id}.md"))).unwrap()
}

/// Linking a blocker to a mission in progress used to be written, after
/// which every command refused the records with
/// `workflow_mission_dependency_bypass`.
#[test]
fn a_write_that_check_would_refuse_is_refused_and_nothing_is_written() {
    let dir = repository();
    let dir = dir.path();
    fs::create_dir_all(dir.join("docs")).unwrap();
    fs::write(dir.join("docs/strategy.md"), "# Strategy\n").unwrap();
    let mission = ok(
        dir,
        &[
            "-q",
            "issue",
            "create",
            "Mission",
            "--issue-type",
            "mission",
            "--strategy",
            "docs/strategy.md",
            "--body",
            "## Outcome\n\nA thing works.",
        ],
    );
    ok(dir, &["issue", "transition", &mission, "ready"]);
    ok(dir, &["issue", "transition", &mission, "start"]);
    let work = issue(dir, "Open work");
    commit(dir, "plan");
    let before = record(dir, &mission);

    let (passed, output) = cli(
        dir,
        &["issue", "link", &mission, &work, "--role", "blocked_by"],
    );
    assert!(
        !passed
            && output.contains("record_invalid: writing these records would fail `atelier check` with 1 new finding, so nothing was written:")
            && output.contains(&format!("workflow_mission_dependency_bypass: mission {mission} has executable status 'in_progress' despite 1 incomplete dependency path: {mission} -> {work}")),
        "{output}"
    );
    assert_eq!(record(dir, &mission), before);
    assert!(git(dir, &["status", "--porcelain"]).is_empty());
    ok(dir, &["check"]);
    ok(dir, &["issue", "show", &work]);
}

/// A link to an issue that exists only on another branch is refused when
/// it would be written, and the refusal names that branch.
#[test]
fn a_link_to_an_issue_on_another_branch_is_refused_naming_the_branch() {
    let dir = repository();
    let dir = dir.path();
    let here = issue(dir, "Here");
    commit(dir, "plan");
    let elsewhere = issue_on_other_branch(dir, "Elsewhere");
    let named = format!(
        "issue {elsewhere} not found on this branch; it exists only on branch other. Next: merge that branch in first, or run this there"
    );

    let (passed, output) = cli(
        dir,
        &["issue", "link", &here, &elsewhere, "--role", "related"],
    );
    assert!(!passed && output.contains(&named), "{output}");
    let finding = ok(
        dir,
        &[
            "-q",
            "review",
            "finding",
            &here,
            "--identity",
            "Reviewer",
            "--recommendation",
            "defer",
            "Later",
        ],
    );
    commit(dir, "finding");
    let (passed, output) = cli(
        dir,
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "defer",
            "--follow-up",
            &elsewhere,
            "Later",
        ],
    );
    assert!(!passed && output.contains(&named), "{output}");
    assert!(git(dir, &["status", "--porcelain"]).is_empty());

    // A record that already holds such a link, by a hand edit or a Git
    // merge, is reported naming the branch too.
    let path = dir.join(format!(".atelier/issues/{here}.md"));
    let text = fs::read_to_string(&path).unwrap().replace(
        "  relates: []",
        &format!("  relates:\n  - kind: \"issue\"\n    id: \"{elsewhere}\"\n    type: \"related\""),
    );
    fs::write(&path, text).unwrap();
    let (passed, output) = cli(dir, &["check"]);
    assert!(
        !passed
            && output.contains(&format!(
                "has related reference to missing issue {elsewhere}; {elsewhere} exists only on branch other, not on this branch. Merge that branch in first, or make the link there"
            )),
        "{output}"
    );
}

/// `migrate epics` writes records no command has loaded; a retyped epic
/// whose child exists only on another branch is refused naming the branch,
/// and nothing is written.
#[test]
fn migrate_refuses_a_record_that_links_an_issue_on_another_branch() {
    let dir = repository();
    let dir = dir.path();
    let child = issue_on_other_branch(dir, "Child elsewhere");
    let policy_path = dir.join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&policy_path)
        .unwrap()
        .replace(
            "  bug:\n    label: Bug\n",
            "  bug:\n    label: Bug\n  epic:\n    label: Epic\n",
        )
        .replace(
            "    applies_to:\n    - mission\n",
            "    applies_to:\n    - mission\n    - epic\n",
        );
    fs::write(&policy_path, policy).unwrap();
    let epic = "atelier-ep1c";
    let epic_path = dir.join(format!(".atelier/issues/{epic}.md"));
    let text = format!(
        "---\ncreated_at: \"2026-10-01T00:00:00+00:00\"\nid: \"{epic}\"\nissue_type: \"epic\"\nlabels: []\npriority: \"P2\"\nrelationships:\n  blocks: []\n  children:\n  - kind: \"issue\"\n    id: \"{child}\"\n  attachments: []\n  relates: []\nschema: \"atelier.issue\"\nschema_version: 2\nstatus: \"draft\"\ntitle: \"Old epic\"\nupdated_at: \"2026-10-01T00:00:00+00:00\"\n---\n\n## Outcome\n\nThe increment works.\n"
    );
    fs::create_dir_all(epic_path.parent().unwrap()).unwrap();
    fs::write(&epic_path, &text).unwrap();
    commit(dir, "an old epic");

    let (passed, output) = cli(dir, &["migrate", "epics", "--apply"]);
    assert!(
        !passed
            && output.contains("record_invalid: writing these records would fail `atelier check`")
            && output.contains(&format!(
                "has children reference to missing issue {child}; {child} exists only on branch other, not on this branch"
            )),
        "{output}"
    );
    assert_eq!(fs::read_to_string(&epic_path).unwrap(), text);
    assert!(git(dir, &["status", "--porcelain"]).is_empty());
}

/// The records a merge puts on the base pass the same check: a record that
/// `atelier check` refuses, added on the branch by hand, is not merged.
#[test]
fn merge_refuses_records_that_would_fail_check_on_the_base() {
    let dir = repository();
    let dir = dir.path();
    let id = issue(dir, "Owner");
    commit(dir, "plan");
    ok(dir, &["start", &id]);
    fs::write(dir.join("src.rs"), "code\n").unwrap();
    let path = dir.join(format!(".atelier/issues/{id}.md"));
    let text = fs::read_to_string(&path)
        .unwrap()
        .replace("labels: []", "labels:\n- \"size-xxl\"");
    fs::write(&path, text).unwrap();
    commit(dir, "code and an unknown size");
    let base = git(dir, &["rev-parse", "main"]);

    let (passed, output) = cli(dir, &["merge", &id, "--check"]);
    assert!(
        !passed
            && output.contains("record_invalid: merging puts records on the base that would fail `atelier check` with 1 new finding, so nothing was merged:")
            && output.contains(&format!(
                "{id}: Size label size-xxl is not one of size-s, size-m, size-l, size-xl"
            )),
        "{output}"
    );
    assert_eq!(git(dir, &["rev-parse", "main"]), base);
}
