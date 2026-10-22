//! `--commit-records` and `--stage-records` (atelier-4ixu): a step's records
//! land in one commit, holding only the records the commands wrote.

use super::*;

fn git(dir: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success(), "git {args:?} failed");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn ok(dir: &Path, args: &[&str]) -> String {
    let (success, stdout, stderr) = run_atelier_raw(dir, args);
    assert!(success, "atelier {args:?} failed: {stderr}");
    stdout
}

fn changed_in_head(dir: &Path) -> Vec<String> {
    git(
        dir,
        &["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"],
    )
    .lines()
    .map(str::to_owned)
    .collect()
}

fn commit_count(dir: &Path) -> usize {
    git(dir, &["rev-list", "--count", "HEAD"]).parse().unwrap()
}

/// A committed repository with one item whose records are clean, plus an
/// unrelated tracked file.
fn fixture() -> (tempfile::TempDir, String) {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let id = ok(
        dir.path(),
        &[
            "issue",
            "create",
            "-q",
            "--body",
            "## Goal\n\nG\n\n## Done when\n\nD",
            "Grouped records",
        ],
    )
    .trim()
    .to_string();
    fs::write(dir.path().join("code.rs"), "one\n").unwrap();
    commit_all(dir.path(), "item and code");
    (dir, id)
}

fn evidence_args<'a>(id: &'a str, summary: &'a str, flag: &'a str) -> Vec<&'a str> {
    vec![
        "evidence",
        "record",
        "--target",
        id,
        "--kind",
        "validation",
        "--result",
        "pass",
        flag,
        summary,
    ]
}

#[test]
fn commit_records_commits_only_the_records_the_command_wrote() {
    let (dir, id) = fixture();
    let dir = dir.path();
    fs::write(dir.join("code.rs"), "two\n").unwrap();
    fs::write(dir.join("notes.txt"), "untracked\n").unwrap();
    let before = commit_count(dir);

    let target = format!("issue/{id}");
    let out = ok(
        dir,
        &evidence_args(&target, "Checked it", "--commit-records"),
    );

    assert!(out.contains("Committed 1 record"), "{out}");
    assert_eq!(commit_count(dir), before + 1);
    let changed = changed_in_head(dir);
    assert!(
        changed
            .iter()
            .all(|path| path.starts_with(".atelier/evidence/")),
        "{changed:?}"
    );
    assert!(!changed.is_empty());
    assert!(git(dir, &["log", "-1", "--format=%s"]).starts_with("Record evidence "));
    let status = git(dir, &["status", "--porcelain"]);
    assert!(
        status.contains("M code.rs") && status.contains("?? notes.txt"),
        "{status}"
    );
    assert_eq!(git(dir, &["diff", "--cached", "--name-only"]), "");
}

#[test]
fn a_staged_change_that_is_not_a_record_refuses_before_anything_is_written() {
    let (dir, id) = fixture();
    let dir = dir.path();
    fs::write(dir.join("code.rs"), "staged\n").unwrap();
    git(dir, &["add", "code.rs"]);
    let before = commit_count(dir);
    let records = git(dir, &["ls-files", ".atelier/evidence"]);

    let target = format!("issue/{id}");
    for flag in ["--commit-records", "--stage-records"] {
        let (success, _, stderr) = run_atelier_raw(dir, &evidence_args(&target, "Refused", flag));
        assert!(
            !success && stderr.contains("record_commit_index_not_clean"),
            "{stderr}"
        );
        assert!(stderr.contains("code.rs"), "{stderr}");
    }
    assert_eq!(commit_count(dir), before);
    assert_eq!(git(dir, &["status", "--porcelain", ".atelier"]), "");
    assert_eq!(git(dir, &["ls-files", ".atelier/evidence"]), records);
    assert_eq!(git(dir, &["diff", "--cached", "--name-only"]), "code.rs");
}

#[test]
fn staged_records_are_committed_with_the_next_commit_records() {
    let (dir, id) = fixture();
    let dir = dir.path();
    let before = commit_count(dir);
    let target = format!("issue/{id}");

    let staged = ok(
        dir,
        &evidence_args(&target, "First check", "--stage-records"),
    );
    assert!(staged.contains("Staged 1 record"), "{staged}");
    assert_eq!(commit_count(dir), before);
    ok(
        dir,
        &evidence_args(&target, "Second check", "--stage-records"),
    );
    ok(
        dir,
        &evidence_args(&target, "Third check", "--commit-records"),
    );

    assert_eq!(commit_count(dir), before + 1);
    assert_eq!(changed_in_head(dir).len(), 3);
    let message = git(dir, &["log", "-1", "--format=%B"]);
    assert!(
        message.contains("and 2 more") && message.matches("- evidence ").count() == 3,
        "{message}"
    );
    assert_eq!(git(dir, &["status", "--porcelain"]), "");
    // The group is spent: a later commit holds only its own records.
    ok(
        dir,
        &evidence_args(&target, "Fourth check", "--commit-records"),
    );
    assert_eq!(changed_in_head(dir).len(), 1);
}

#[test]
fn a_review_step_and_the_close_each_commit_their_own_records() {
    let (dir, id) = fixture();
    let dir = dir.path();
    let finding = ok(
        dir,
        &[
            "review",
            "finding",
            &id,
            "-q",
            "--identity",
            "Reviewer",
            "--recommendation",
            "no_action",
            "--stage-records",
            "Nothing to do",
        ],
    )
    .trim()
    .to_string();
    let before = commit_count(dir);
    ok(
        dir,
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "no_action",
            "--commit-records",
            "Agreed",
        ],
    );
    assert_eq!(commit_count(dir), before + 1);
    let changed = changed_in_head(dir);
    assert_eq!(changed.len(), 2, "{changed:?}");
    assert!(changed
        .iter()
        .all(|path| path.starts_with(".atelier/reviews/")));

    for transition in ["start", "request_review"] {
        ok(dir, &["issue", "transition", &id, transition]);
    }
    commit_all(dir, "ready");
    let before = commit_count(dir);
    ok(
        dir,
        &[
            "issue",
            "transition",
            &id,
            "close",
            "--reason",
            "Done",
            "--commit-records",
        ],
    );
    assert_eq!(commit_count(dir), before + 1);
    assert_eq!(
        changed_in_head(dir),
        vec![format!(".atelier/issues/{id}.md")]
    );
    assert_eq!(
        git(dir, &["log", "-1", "--format=%s"]),
        format!("Record close {id}")
    );
}
