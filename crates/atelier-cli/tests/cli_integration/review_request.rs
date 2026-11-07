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

/// A started owner with one code commit after the start, which also writes
/// its implementer report, and a rationale in its notes that the brief must
/// not carry.
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
    edit_canonical_issue(dir, &id, |markdown| {
        format!(
            "{}\n\n## Report\n\nThe parser parses.\n",
            markdown.trim_end()
        )
    });
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

/// Asking for review means the work is done to the best of the author's
/// knowledge: the request refuses while open work it covers has an unticked
/// Done when line or an empty Report, and lists each (atelier-jnrh).
#[test]
fn the_request_refuses_unticked_done_when_lines_and_empty_reports_and_lists_them() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    init_atelier(dir);
    init_git_repo(dir);
    let create = |args: &[&str]| ok(dir, args).trim().to_string();
    let id = create(&[
        "issue",
        "create",
        "-q",
        "--issue-type",
        "feature",
        "--body",
        "## Goal\n\nShip it.\n\n## Done when\n\n- [x] The first test passes.\n- [ ] The second test passes.\n- [ ] After merge (device): it opens on the phone. Stand-in: the emulator test passes.",
        "Parent",
    ]);
    let child = create(&[
        "issue",
        "create",
        "-q",
        "--parent",
        &id,
        "--body",
        "## Goal\n\nPart.\n\n## Done when\n\n- [ ] The part works.",
        "Child",
    ]);
    let dropped = create(&[
        "issue",
        "create",
        "-q",
        "--parent",
        &id,
        "--body",
        "## Goal\n\nDropped.\n\n## Done when\n\n- [ ] Never done.",
        "Dropped",
    ]);
    let closed = create(&[
        "issue",
        "create",
        "-q",
        "--parent",
        &id,
        "--body",
        "## Goal\n\nClosed earlier.\n\n## Done when\n\n- [ ] Left unticked.",
        "Closed by an older Atelier",
    ]);
    commit_all(dir, "plan");
    ok(dir, &["start", &id]);
    // Naming the owner covers its started children.
    ok(dir, &["issue", "transition", &child, "start"]);
    ok(
        dir,
        &[
            "issue",
            "transition",
            &dropped,
            "supersede",
            "--reason",
            "Not needed.",
        ],
    );
    // An item an older Atelier closed on the branch, with its Close Reason,
    // is done already, so merge leaves it as it is and review asks nothing
    // of it.
    edit_canonical_issue(dir, &closed, |markdown| {
        format!(
            "{}\n\n## Close Reason\n\nClosed by an older Atelier.\n",
            markdown
                .replace("status: \"todo\"", "status: \"done\"")
                .trim_end()
        )
    });
    commit_all(dir, "start");

    let (success, stdout, stderr) = run_atelier_raw(dir, &["review", "request", &id]);
    assert!(!success, "{stdout}");
    let expected = format!(
        "review_request_unfinished: asking for review means the work is done to the best of your knowledge, and {id}'s open work is not:\n- {id}: 1 Done when line unticked: \"The second test passes.\"; Report empty\n- {child}: 1 Done when line unticked: \"The part works.\"; Report empty\nNext: tick each Done when line once it holds, write each item's implementer report in its `## Report` section, commit, then rerun `atelier review request {id}`\nNote: 1 After merge line need not be ticked; `atelier migrate after-merge` turns such lines into loose ends."
    );
    assert!(stderr.contains(&expected), "{stderr}");
    // The superseded child and the closed one are not work merge will mark
    // done.
    assert!(!stderr.contains(&dropped), "{stderr}");
    assert!(!stderr.contains(&closed), "{stderr}");

    for (item, done_when) in [
        (
            &id,
            (
                "- [ ] The second test passes.",
                "- [x] The second test passes.",
            ),
        ),
        (&child, ("- [ ] The part works.", "- [x] The part works.")),
    ] {
        edit_canonical_issue(dir, item, |markdown| {
            format!(
                "{}\n\n## Report\n\nDone.\n",
                markdown.replace(done_when.0, done_when.1).trim_end()
            )
        });
    }
    commit_all(dir, "tick and report");
    let brief = ok(dir, &["review", "request", &id]);
    assert!(
        brief.starts_with(&format!("# Review request: {id} ")),
        "{brief}"
    );
}

/// The request covers the item it names, and naming the owner covers the
/// owner and its started children, so a parent's children can be reviewed
/// one at a time while unstarted siblings wait (finding atelier-2ygf).
#[test]
fn the_request_covers_the_named_item_and_the_owners_started_children() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    init_atelier(dir);
    init_git_repo(dir);
    let create = |args: &[&str]| ok(dir, args).trim().to_string();
    let id = create(&[
        "issue",
        "create",
        "-q",
        "--issue-type",
        "feature",
        "--body",
        "## Goal\n\nShip it.\n\n## Done when\n\n- [ ] The whole feature works.",
        "Parent",
    ]);
    let started = create(&[
        "issue",
        "create",
        "-q",
        "--parent",
        &id,
        "--body",
        "## Goal\n\nPart one.\n\n## Done when\n\n- [x] Part one works.\n\n## Report\n\nPart one is done.",
        "Started child",
    ]);
    let waiting = create(&[
        "issue",
        "create",
        "-q",
        "--parent",
        &id,
        "--body",
        "## Goal\n\nPart two.\n\n## Done when\n\n- [ ] Part two works.",
        "Unstarted child",
    ]);
    commit_all(dir, "plan");
    ok(dir, &["start", &id]);
    ok(dir, &["issue", "transition", &started, "start"]);
    commit_all(dir, "start the parent and its first child");

    // The ticked, reported child is reviewed on its own, whatever its parent
    // and its unstarted sibling still lack.
    let brief = ok(dir, &["review", "request", &started]);
    assert!(
        brief.starts_with(&format!("# Review request: {id} ")),
        "{brief}"
    );

    // Naming the owner covers the owner and the started child, never the
    // unstarted one.
    edit_canonical_issue(dir, &id, |markdown| {
        format!(
            "{}\n\n## Report\n\nThe feature is done.\n",
            markdown
                .replace(
                    "- [ ] The whole feature works.",
                    "- [x] The whole feature works."
                )
                .trim_end()
        )
    });
    edit_canonical_issue(dir, &started, |markdown| {
        markdown.replace("- [x] Part one works.", "- [ ] Part one works.")
    });
    commit_all(dir, "tick the parent, untick the child");
    let (success, stdout, stderr) = run_atelier_raw(dir, &["review", "request", &id]);
    assert!(!success, "{stdout}");
    assert!(
        stderr.contains(&format!(
            "and {id}'s open work is not:\n- {started}: 1 Done when line unticked: \"Part one works.\"\nNext:"
        )),
        "{stderr}"
    );
    assert!(!stderr.contains(&waiting), "{stderr}");
    let (success, _, stderr) = run_atelier_raw(dir, &["review", "request", &started]);
    assert!(
        !success && stderr.contains(&format!("- {started}: 1 Done when line unticked")),
        "{stderr}"
    );
}
