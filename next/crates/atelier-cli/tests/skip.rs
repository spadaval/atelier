//! `ATELIER_SKIP` on the commands: a rule passed with its reason printed
//! and, where the command commits, in a trailer; a blank reason refused;
//! and what is not a rule refused all the same, saying so.

mod common;

use common::*;
use std::fs;
use std::path::Path;

const REASON: &str = "Sumeet said to";

/// `atelier` as `thread` with `ATELIER_SKIP` set to `skip`.
fn skipping(at: &Path, thread: &str, skip: &str, args: &[&str]) -> Run {
    let output = atelier_command(at, thread, args)
        .env("ATELIER_SKIP", skip)
        .output()
        .unwrap();
    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

/// What the command prints for each rule it passed.
fn skipped(rules: &[&str]) -> String {
    let lines = rules
        .iter()
        .map(|r| format!("Skipped with ATELIER_SKIP: {r}: {REASON}\n"));
    lines.collect()
}

/// The message of the commit `rev`.
fn message(at: &Path, rev: &str) -> String {
    git(at, &["log", "-1", "--format=%B", rev])
}

fn trailers(rules: &[&str]) -> String {
    let lines = rules.iter().map(|r| format!("Atelier-Skip: {r}: {REASON}"));
    lines.collect::<Vec<_>>().join("\n")
}

#[test]
fn a_skipped_rule_is_printed_and_trailed_in_the_commit_of_start_submit_verdict_and_merge() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    // Refused: the way out first, the skip second.
    let run = atelier(&root, "t1", &["start", ID]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.ends_with(&format!(
            "A planner makes it ready with: atelier ready {ID} --by <name>\n{SKIP}"
        )),
        "{}",
        run.stderr
    );
    // A draft started: its whitespace made one line, the reason is kept.
    let draft = format!("{ID} is ready (it is draft)");
    let run = skipping(&root, "t1", "  Sumeet\n said   to ", &["start", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stdout.starts_with(&skipped(&[&draft])),
        "{}",
        run.stdout
    );
    assert_eq!(field(&root, ID, "status"), "building");
    assert_eq!(
        message(&root, ID),
        format!("Start {ID}\n\n{}", trailers(&[&draft]))
    );

    // Submitted with nothing ticked and no Report.
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    git(&root, &["add", "hello.txt"]);
    git(&root, &["commit", "-qm", "Say hello"]);
    let unmet = [
        "Done when: `hello` prints hello.",
        "Done when: A test covers it.",
        "Report: written under `## Report`",
    ];
    let run = skipping(&root, "t1", REASON, &["submit"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(run.stdout.starts_with(&skipped(&unmet)), "{}", run.stdout);
    assert_eq!(field(&root, "HEAD", "status"), "in_review");
    assert_eq!(
        message(&root, "HEAD"),
        format!("Submit {ID} for review\n\n{}", trailers(&unmet))
    );

    // `verdict changes` with no fix-now finding in the file.
    assert_eq!(atelier(&root, "t2", &["review", ID]).code, 0);
    let none = "Your review file holds an open fix-now finding (it holds none; with nothing \
                to fix, run `atelier verdict approve`)";
    let run = skipping(&root, "t2", REASON, &["verdict", "changes"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(run.stdout.starts_with(&skipped(&[none])), "{}", run.stdout);
    assert!(
        message(&root, "HEAD").ends_with(&format!("changes requested\n\n{}", trailers(&[none]))),
        "{}",
        message(&root, "HEAD")
    );

    // Merged with no approval, after the trailer the pre-push hook knows.
    let (status, approval) = (
        format!("{ID} is approved (it is changes requested)"),
        "A review of the latest round approves it (none does)",
    );
    let run = skipping(&root, "t1", REASON, &["merge"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        skipped(&[&status, approval])
            + &format!(
                "Merged {ID} into main with merge commit {}, which marks it done.\n",
                git(&root, &["rev-parse", "--short=7", "main"])
            )
    );
    assert_eq!(field(&root, "main", "status"), "done");
    assert!(message(&root, "main").ends_with(&format!(
        "\n\nAtelier-Merge: {ID}\n{}",
        trailers(&[&status, approval])
    )));
}

#[test]
fn set_with_a_skip_changes_a_started_items_base_record() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    git(&root, &["switch", "-q", "main"]);
    let run = skipping(&root, "p1", REASON, &["set", ID, "--priority", "low"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let rule = format!("{ID} is draft or ready (it is building)");
    assert!(run.stdout.starts_with(&skipped(&[&rule])), "{}", run.stdout);
    // The base's record, not the branch's: it is still ready there.
    let text = read(&root, &item_path(ID));
    assert_eq!(
        (front(&text, "status"), front(&text, "priority")),
        ("ready".into(), "low".into())
    );
}

#[test]
fn a_command_with_nothing_to_skip_runs_as_without_the_variable() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let plain = atelier(&root, "t1", &["status"]);
    let run = skipping(&root, "t1", REASON, &["status"]);
    assert_eq!((run.code, &run.stdout), (0, &plain.stdout));
    let run = skipping(&root, "t1", REASON, &["start", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(!run.stdout.contains("Skipped"), "{}", run.stdout);
    assert_eq!(message(&root, ID), format!("Start {ID}"));
}

#[test]
fn a_blank_skip_is_refused_by_every_command() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    let blank =
        "ATELIER_SKIP is set and blank, and a skip needs a reason: ATELIER_SKIP=\"<why>\". \
                 Nothing changed.\n";
    for skip in ["", "  \n\t "] {
        for args in [
            &["status"][..],
            &["check"][..],
            &["start", ID][..],
            &["ready", ID, "--by", "s"][..],
            &["merge"][..],
            &["check", "--hook", "pre-commit"][..],
        ] {
            let run = skipping(&root, "t1", skip, args);
            assert_eq!((run.code, run.stderr.as_str()), (1, blank), "{args:?}");
            assert_eq!(run.stdout, "", "{args:?}");
        }
    }
    assert_eq!(git(&root, &["status", "--porcelain"]), "");
    assert_eq!(git(&root, &["branch", "--list", ID]), "");
}

#[test]
fn what_is_not_a_rule_is_refused_with_the_variable_set_and_says_so() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let refused = |args: &[&str], code: i32, says: &str| {
        let plain = atelier(&root, "t1", args);
        assert_eq!(plain.code, code, "{args:?}: {}", plain.stderr);
        assert!(plain.stderr.contains(says), "{args:?}: {}", plain.stderr);
        assert!(
            !plain.stderr.contains("ATELIER_SKIP"),
            "{args:?}: {}",
            plain.stderr
        );
        let run = skipping(&root, "t1", REASON, args);
        assert_eq!(run.code, code, "{args:?}: {}", run.stderr);
        assert_eq!(run.stdout, plain.stdout, "{args:?}");
        assert_eq!(run.stderr, plain.stderr + NOT_SKIPPABLE, "{args:?}");
    };
    // An ID that names nothing, a finding that does not exist, an argument
    // missing or malformed.
    refused(
        &["start", "demo-zz99"],
        1,
        "No item demo-zz99 is on the base branch",
    );
    refused(
        &["drop", &format!("{ID}/1-aaaa.9"), "--reason", "x"],
        1,
        "has a finding 1-aaaa.9 (none does)",
    );
    refused(&["start"], 2, "the following required arguments");
    refused(&["start", "Not An Id"], 2, "atelier: ");
    refused(&["drop", ID], 1, "`atelier drop` needs a reason");
    // Run off an item's branch, a command has no item to act on.
    refused(&["submit"], 1, "runs on an item's branch");
    // A record that does not parse, and one that does and is not valid.
    fs::write(root.join(item_path(ID)), "no front matter\n").unwrap();
    write_item(&root, "demo-cd34", "ready", &[], &["demo-zz99"], "");
    refused(&["check"], 1, "These do not read as records");
    fs::remove_dir_all(root.join(".atelier/items/demo-cd34")).unwrap();
    git(&root, &["checkout", "-q", "--", "."]);
    // A Git command that failed: the branch cannot be switched to while
    // another worktree has it.
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    let other = scratch.worktree("other");
    git(&root, &["switch", "-q", "main"]);
    git(&other, &["switch", "-q", ID]);
    let run = skipping(&root, "t2", REASON, &["start", ID]);
    assert_eq!(run.code, 2, "{}", run.stderr);
    assert!(run.stderr.ends_with(NOT_SKIPPABLE), "{}", run.stderr);
}
