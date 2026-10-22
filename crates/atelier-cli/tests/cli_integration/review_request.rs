//! `atelier review request` (atelier-rc8p): the reviewer brief, generated from
//! records, Git and policy.

use super::*;

fn ok(dir: &Path, args: &[&str]) -> String {
    let (success, stdout, stderr) = run_atelier_raw(dir, args);
    assert!(success, "atelier {args:?} failed: {stderr}");
    stdout
}

fn git(dir: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success(), "git {args:?} failed");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// A started owner with one code commit after the start, and a Close Reason
/// style rationale in its notes that the brief must not carry.
fn owner(dir: &Path) -> String {
    init_atelier(dir);
    init_git_repo(dir);
    let id = ok(
        dir,
        &[
            "issue",
            "create",
            "-q",
            "--issue-type",
            "feature",
            "--body",
            "## Goal\n\nShip the parser.\n\n## Done when\n\nThe parser test passes.",
            "Parser",
        ],
    )
    .trim()
    .to_string();
    commit_all(dir, "plan");
    ok(dir, &["start", &id]);
    ok(
        dir,
        &[
            "issue",
            "note",
            &id,
            "--kind",
            "handoff",
            "The author insists this is perfect.",
        ],
    );
    fs::write(dir.join("parser.rs"), "fn parse() {}\n").unwrap();
    commit_all(dir, "add the parser");
    id
}

#[test]
fn the_brief_names_the_requirements_range_and_record_commands() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    let id = owner(dir);
    let head = git(dir, &["rev-parse", "HEAD"]);

    let brief = ok(
        dir,
        &[
            "review",
            "request",
            &id,
            "--identity",
            "Independent reviewer",
        ],
    );
    assert!(
        brief.starts_with(&format!(
            "# Review request: {id} feature in_progress Parser\n"
        )),
        "{brief}"
    );
    for expected in [
        "## Goal\n\nShip the parser.\n",
        "## Done when\n\nThe parser test passes.\n",
        &format!("- Head, to pin records to: `{head}`\n"),
        "forked from `main`",
        "## Findings so far\n\nNone.\n",
        &format!("atelier review finding {id} --identity \"Independent reviewer\" --commit {head} --recommendation fix_now|defer|no_action --commit-records"),
        &format!("atelier review approve {id} --identity \"Independent reviewer\" --commit {head} --commit-records"),
    ] {
        assert!(brief.contains(expected), "missing `{expected}`: {brief}");
    }
    // The author's rationale is not the reviewer's to weigh.
    assert!(!brief.contains("insists"), "{brief}");
    assert!(!brief.contains(env!("CARGO_MANIFEST_DIR")), "{brief}");
    // Without an identity the command shows a placeholder.
    let anonymous = ok(dir, &["review", "request", &id]);
    assert!(
        anonymous.contains("--identity \"<your identity>\""),
        "{anonymous}"
    );
}

#[test]
fn the_brief_carries_findings_dispositions_and_the_range_since_the_last_review() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    let id = owner(dir);
    let reviewed = git(dir, &["rev-parse", "HEAD"]);
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
            "fix_now",
            "high: parser.rs:1 parse() does nothing",
        ],
    )
    .trim()
    .to_string();
    let closed = ok(
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
            "low: naming",
        ],
    )
    .trim()
    .to_string();
    ok(
        dir,
        &[
            "review",
            "disposition",
            &closed,
            "--identity",
            "Lead",
            "--decision",
            "no_action",
            "Fine as is",
        ],
    );
    ok(
        dir,
        &[
            "review",
            "record",
            &id,
            "--identity",
            "Reviewer",
            "Read the diff",
        ],
    );
    commit_all(dir, "record review");
    fs::write(dir.join("parser.rs"), "fn parse() { todo!() }\n").unwrap();
    commit_all(dir, "second change");
    let head = git(dir, &["rev-parse", "HEAD"]);

    let brief = ok(dir, &["review", "request", &id]);
    let findings = brief
        .split("## Findings so far\n\n")
        .nth(1)
        .unwrap()
        .split("\n\n")
        .next()
        .unwrap();
    let lines = findings.lines().collect::<Vec<_>>();
    assert!(
        lines[0].starts_with(&format!("- {finding} fix_now | open, no disposition @ "))
            && lines[0].ends_with("Reviewer: high: parser.rs:1 parse() does nothing"),
        "{findings}"
    );
    assert!(
        lines[1].starts_with(&format!("- {closed} no_action | no_action by Lead @ ")),
        "{findings}"
    );
    assert!(brief.contains("## Earlier reviews\n\n- "), "{brief}");
    assert!(brief.contains(" review @ "), "{brief}");
    // The last review pinned an earlier commit, so the range is what followed it.
    assert!(
        brief.contains(&format!(
            "- Review: `{}..{}` (2 commits, since the last review by Reviewer (",
            &reviewed[..8],
            &head[..8]
        )),
        "{brief}"
    );
    assert!(brief.contains("- Whole branch: `"), "{brief}");

    let from = ok(dir, &["review", "request", &id, "--from", "HEAD~1"]);
    assert!(from.contains("from --from"), "{from}");
    let (success, _, stderr) = run_atelier_raw(dir, &["review", "request", &id, "--from", "nope"]);
    assert!(!success && stderr.contains("--from nope"), "{stderr}");
}

#[test]
fn working_rules_come_from_policy_and_the_configured_rules_file() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    let id = owner(dir);
    fs::write(
        dir.join("review-rules.md"),
        "Use the shared scratch directory.\n",
    )
    .unwrap();
    let config = dir.join(".atelier/config.toml");
    let text = fs::read_to_string(&config).unwrap();
    fs::write(
        &config,
        format!("{text}\n[review]\nrules_file = \"review-rules.md\"\n"),
    )
    .unwrap();
    commit_all(dir, "configure review rules");

    let brief = ok(dir, &["review", "request", &id]);
    assert!(
        brief.contains("### This repository's rules\n\nUse the shared scratch directory.\n"),
        "{brief}"
    );

    fs::write(
        &config,
        format!("{text}\n[review]\nrules_file = \"../outside.md\"\n"),
    )
    .unwrap();
    let (success, _, stderr) = run_atelier_raw(dir, &["review", "request", &id]);
    assert!(!success && stderr.contains("review.rules_file"), "{stderr}");
}

#[test]
fn an_item_with_no_branch_has_nothing_to_review() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    init_atelier(dir);
    init_git_repo(dir);
    let id = ok(
        dir,
        &[
            "issue",
            "create",
            "-q",
            "--body",
            "## Goal\n\nG\n\n## Done when\n\nD",
            "Unstarted",
        ],
    )
    .trim()
    .to_string();
    commit_all(dir, "plan");
    let (success, _, stderr) = run_atelier_raw(dir, &["review", "request", &id]);
    assert!(
        !success && stderr.contains("review_request_unstarted"),
        "{stderr}"
    );
}

#[test]
fn merging_the_base_into_the_branch_does_not_grow_the_review_range() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    let id = owner(dir);
    ok(
        dir,
        &[
            "review",
            "record",
            &id,
            "--identity",
            "Reviewer",
            "Read the diff",
        ],
    );
    commit_all(dir, "record review");
    let branch = git(dir, &["rev-parse", "--abbrev-ref", "HEAD"]);
    git(dir, &["checkout", "-q", "main"]);
    for n in 0..3 {
        fs::write(dir.join(format!("base{n}.txt")), "base\n").unwrap();
        commit_all(dir, &format!("base change {n}"));
    }
    git(dir, &["checkout", "-q", &branch]);
    git(dir, &["merge", "-q", "--no-edit", "main"]);

    let brief = ok(dir, &["review", "request", &id]);
    // The record commit and the merge are the branch's own; the base's three are not.
    assert!(
        brief.contains("(2 commits, since the last review"),
        "{brief}"
    );
    assert!(brief.contains("^main`"), "{brief}");
}
