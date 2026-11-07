//! The bookkeeping of one standard lifecycle: the scenario that mission
//! atelier-n4mj's `lifecycle-cost` criterion measures
//! (docs/strategies/lighter-workflow.md).
//!
//! The scenario's steps are fixed. An item that cuts bookkeeping lowers the
//! expected counts below, and the diff of these constants shows the
//! reduction. It may change the commands that take a step when the product
//! changes them, but never how a step commits its records: that would move
//! the counts without a product change. The baseline, measured before any
//! product change, is recorded as evidence on atelier-5fus.
use std::{fs, path::Path};
use tempfile::tempdir;
#[path = "support/cli.rs"]
mod support_cli;
use support_cli::{commit, git, ok};

/// Commits on the work branch that change only files under `.atelier/`:
/// start; receipt and close; the findings; the follow-up issue; the author's
/// dispositions; the reviewer's acceptances and approval. Consecutive
/// `--commit-records` commands add to one commit unless a record pins it or
/// it already holds a version of the same record (atelier-5822).
const RECORD_ONLY_COMMITS: usize = 6;

/// For each finding, by its recommendation: the finding plus every review
/// event that names it. A finding carries its disposition (atelier-hwxm).
const REVIEW_FILES_PER_FINDING: [(&str, usize); 3] =
    [("fix_now", 1), ("defer", 1), ("no_action", 1)];

const AUTHOR: &str = "Author";
const REVIEWER: &str = "Reviewer";
const WORK: &str = "## Goal\n\nShip the change.\n\n## Done when\n\nThe scenario passes.";

#[test]
fn one_standard_lifecycle_costs_the_recorded_bookkeeping() {
    let dir = tempdir().unwrap();
    let repo = dir.path();

    // A repository with the starter policy and one planned issue on the base.
    git(repo, &["init", "-qb", "main"]);
    git(repo, &["config", "user.name", AUTHOR]);
    git(repo, &["config", "user.email", "author@example.test"]);
    ok(repo, &["init", "--slug", "atelier"]);
    commit(repo, "Initialize Atelier");
    let id = ok(repo, &["-q", "issue", "create", "Change", "--body", WORK]);
    commit(repo, "Plan the change");
    let base = git(repo, &["rev-parse", "HEAD"]);

    // The author starts the issue, implements it, records one manual receipt
    // and closes it.
    ok(repo, &["start", &id]);
    commit(repo, "Start the change");
    fs::write(repo.join("src.rs"), "first version\n").unwrap();
    commit(repo, "Implement the change");
    ok(
        repo,
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{id}"),
            "--kind",
            "validation",
            "--result",
            "pass",
            "--commit-records",
            "Checked the change by hand",
        ],
    );
    ok(
        repo,
        &[
            "issue",
            "transition",
            &id,
            "close",
            "--reason",
            "Shipped the change.",
            "--commit-records",
        ],
    );

    // An independent reviewer raises three findings at the head and does not
    // approve.
    let reviewed = git(repo, &["rev-parse", "HEAD"]);
    let findings = REVIEW_FILES_PER_FINDING.map(|(recommendation, _)| {
        ok(
            repo,
            &[
                "-q",
                "review",
                "finding",
                &id,
                "--identity",
                REVIEWER,
                "--commit",
                &reviewed,
                "--recommendation",
                recommendation,
                "--commit-records",
                &format!("{recommendation}: src.rs:1 a problem"),
            ],
        )
    });
    let [fix_now, defer, no_action] = &findings;

    // The author fixes the fix_now finding, files the follow-up the defer
    // names, and disposes of all three. The follow-up is filed before the
    // dispositions so its own commit does not split them (atelier-5822).
    fs::write(repo.join("src.rs"), "fixed version\n").unwrap();
    commit(repo, "Fix the finding");
    let fix = git(repo, &["rev-parse", "HEAD"]);
    let follow_up = ok(
        repo,
        &["-q", "issue", "create", "Follow up", "--body", WORK],
    );
    commit(repo, "Plan the follow-up");
    dispose(
        repo,
        fix_now,
        AUTHOR,
        &["fix_now", "--resolution-commit", &fix],
    );
    dispose(repo, defer, AUTHOR, &["defer", "--follow-up", &follow_up]);
    dispose(repo, no_action, AUTHOR, &["no_action"]);

    // The reviewer reviews the fix at the head it was asked to review, the
    // one the review request brief names, accepts the defer and no_action
    // decisions, and approves that head.
    let head = git(repo, &["rev-parse", "HEAD"]);
    dispose(repo, defer, REVIEWER, &["defer", "--follow-up", &follow_up]);
    dispose(repo, no_action, REVIEWER, &["no_action"]);
    ok(
        repo,
        &[
            "review",
            "approve",
            &id,
            "--identity",
            REVIEWER,
            "--commit",
            &head,
            "--commit-records",
            "Reviewed the fix and the change",
        ],
    );

    let tip = git(repo, &["rev-parse", "HEAD"]);
    let output = ok(repo, &["merge", &id]);
    assert!(output.contains("merge: passed"), "{output}");
    git(repo, &["merge-base", "--is-ancestor", &tip, "main"]);

    let record_only = record_only_commits(repo, &base, &tip);
    assert_eq!(
        record_only.len(),
        RECORD_ONLY_COMMITS,
        "record-only commits on the work branch:\n{}",
        record_only.join("\n")
    );
    let per_finding = REVIEW_FILES_PER_FINDING
        .iter()
        .zip(&findings)
        .map(|((recommendation, _), finding)| (*recommendation, review_files_naming(repo, finding)))
        .collect::<Vec<_>>();
    assert_eq!(per_finding, REVIEW_FILES_PER_FINDING);
}

/// Records a disposition of `finding` as `identity`; `decision` is the
/// decision and its fields.
fn dispose(repo: &Path, finding: &str, identity: &str, decision: &[&str]) {
    let mut args = vec![
        "review",
        "disposition",
        finding,
        "--identity",
        identity,
        "--decision",
    ];
    args.extend(decision);
    args.extend(["--commit-records", "The decision and why"]);
    ok(repo, &args);
}

/// The subjects of the non-merge commits in `base..tip` whose every changed
/// file is under `.atelier/`.
fn record_only_commits(repo: &Path, base: &str, tip: &str) -> Vec<String> {
    let range = format!("{base}..{tip}");
    git(repo, &["rev-list", "--no-merges", "--reverse", &range])
        .lines()
        .filter(|commit| {
            let files = git(
                repo,
                &["diff-tree", "--no-commit-id", "--name-only", "-r", commit],
            );
            !files.is_empty() && files.lines().all(|file| file.starts_with(".atelier/"))
        })
        .map(|commit| git(repo, &["log", "-1", "--format=%s", commit]))
        .collect()
}

/// How many files under `.atelier/reviews` contain the full ID `finding`: the
/// finding itself and each event that names it.
fn review_files_naming(repo: &Path, finding: &str) -> usize {
    fn count(dir: &Path, finding: &str) -> usize {
        fs::read_dir(dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .map(|path| {
                if path.is_dir() {
                    count(&path, finding)
                } else {
                    usize::from(fs::read_to_string(&path).unwrap().contains(finding))
                }
            })
            .sum()
    }
    count(&repo.join(".atelier/reviews"), finding)
}
