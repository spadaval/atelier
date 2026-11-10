//! Deferred findings of done items: `list --deferred` names each one nobody
//! has settled, and `new --from` and `drop` settle it once, on the base or
//! on the branch of an item its implementer holds.

mod common;

use common::*;
use std::fs;
use std::path::Path;
use std::process::Command;

const DONE: &str = "demo-d0ne";
const REVIEW: &str = ".atelier/items/demo-d0ne/reviews/1-aaaa.md";

/// An approving review of `DONE` with a finding of each kind: two deferred
/// by the reviewer, one with no action, and a fix-now one the item's
/// Decisions defer.
const FINDINGS: &str = "## Summary\nFine.\n\n\
    ## Finding 1: The greeting ignores the locale\nRecommendation: defer\n\
    Greeting is English only.\n\n```\nTaken: in a fence, as text\n```\n\n\
    ## Finding 2: Old wording in the help\nRecommendation: defer\n\n\
    ## Finding 3: A typo\nRecommendation: no action\n\n\
    ## Finding 4: The slow path\nRecommendation: fix now\n";

const DECIDED: &str = "Done.\n\n## Decisions\n\n### 2026-11-10: Fix the slow path now?\n\n\
    Finding: 1-aaaa.4\nAnswer by t5: Later.\nOutcome: defer\n";

fn write_review(root: &Path, path: &str, verdict: &str, findings: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        format!(
            "---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: abc1234\n\
             reviewers:\n- t2\nmodel: null\nverdict: {verdict}\n\
             created_at: 2026-11-10T09:00:00Z\n---\n{findings}"
        ),
    )
    .unwrap();
}

/// A repository on `main` holding `items` and the done item `DONE` with
/// its review.
fn with_done(items: &[(&str, &str)]) -> Scratch {
    let scratch = Scratch::new(items);
    let root = scratch.root();
    write_item(&root, DONE, "done", &["t1"], &[], DECIDED);
    write_review(&root, REVIEW, "approve", FINDINGS);
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Merge demo-d0ne"]);
    scratch
}

/// The last line of `output`, without its `Then: ` prefix.
fn then(output: &str) -> &str {
    let last = output.lines().last().unwrap();
    last.strip_prefix("Then: ")
        .unwrap_or_else(|| panic!("{output}"))
}

fn shell(root: &Path, line: &str) {
    let output = identity(Command::new("sh").current_dir(root).args(["-c", line]))
        .output()
        .unwrap();
    assert!(output.status.success(), "{line}: {output:?}");
}

/// The finding names `list --deferred` prints, in order.
fn listed(root: &Path) -> Vec<String> {
    let run = atelier(root, "t9", &["list", "--deferred"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let names = run
        .stdout
        .lines()
        .filter_map(|line| line.strip_prefix("  "));
    names
        .map(|line| line.split(':').next().unwrap().to_string())
        .collect()
}

/// What the checkout holds that its last commit does not.
fn changes(root: &Path) -> String {
    git(root, &["status", "--porcelain", "--untracked-files=all"])
}

#[test]
fn each_deferred_finding_of_a_done_item_is_listed_until_taken_or_dropped_once() {
    let scratch = with_done(&[]);
    let root = scratch.root();
    let run = atelier(&root, "t9", &["list", "--deferred"]);
    assert_eq!(
        run.stdout,
        "Deferred findings of done items that nobody has settled:\n\
         \x20 demo-d0ne/1-aaaa.1: The greeting ignores the locale\n\
         \x20 demo-d0ne/1-aaaa.2: Old wording in the help\n\
         \x20 demo-d0ne/1-aaaa.4: The slow path\n\
         Settle each once: take it into a new item, or drop it with a reason.\n\
         Then: atelier new \"<title>\" --from demo-d0ne/1-aaaa.1, or atelier drop demo-d0ne/1-aaaa.1 --reason \"<why>\"\n"
    );

    // The name the list prints is the one `new --from` takes.
    let run = atelier(
        &root,
        "t9",
        &["new", "Greet in the locale", "--from", "demo-d0ne/1-aaaa.1"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    let taken = run.stdout.clone();
    let created = run.stdout.split_whitespace().nth(3).unwrap().to_string();
    let item = item_path(&created);
    assert!(run.stdout.contains(&format!(
        "Wrote, and did not commit:\n  {item}\n  {REVIEW}\nPlanning counts once it is committed on the base.\n"
    )));
    let text = read(&root, &item);
    assert!(text.contains("title: Greet in the locale\nstatus: draft\n"));
    assert!(text.ends_with(
        "\n## Goal\n\nTaken from deferred finding demo-d0ne/1-aaaa.1, in .atelier/items/demo-d0ne/reviews/1-aaaa.md:\n\n\
         > Finding 1: The greeting ignores the locale\n> Recommendation: defer\n\
         > Greeting is English only.\n>\n> ```\n> Taken: in a fence, as text\n> ```\n\n\
         ## Done when\n\n## Scope\n"
    ));
    assert!(read(&root, REVIEW).contains(&format!(
        "## Finding 1: The greeting ignores the locale\nRecommendation: defer\nTaken: {created} by t9\nGreeting is English only.\n"
    )));
    assert_eq!(listed(&root), ["demo-d0ne/1-aaaa.2", "demo-d0ne/1-aaaa.4"]);

    // A reason is required.
    let run = atelier(&root, "t9", &["drop", "demo-d0ne/1-aaaa.2"]);
    assert_eq!(run.code, 1);
    assert!(run.stderr.contains("needs a reason"), "{}", run.stderr);
    let run = atelier(
        &root,
        "t9",
        &[
            "drop",
            "demo-d0ne/1-aaaa.2",
            "--reason",
            "The help was rewritten.",
        ],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(read(&root, REVIEW).contains(
        "## Finding 2: Old wording in the help\nRecommendation: defer\nDropped by t9: The help was rewritten.\n"
    ));
    // The finding a decision deferred is settled the same way.
    let run = atelier(
        &root,
        "t9",
        &["drop", "demo-d0ne/1-aaaa.4", "--reason", "Fast enough."],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(listed(&root).is_empty());

    // `show` prints each settlement with its finding, before it is
    // committed too.
    let run = atelier(&root, "t9", &["show", DONE]);
    assert!(
        run.stdout.contains(&format!(
        "    1-aaaa.1 (defer, taken into {created} by t9): The greeting ignores the locale\n\
         \x20   1-aaaa.2 (defer, dropped by t9: The help was rewritten.): Old wording in the help\n"
    )),
        "{}",
        run.stdout
    );

    // Committed with the line `new --from` printed, which names the review
    // file too, they stay off the list.
    shell(&root, then(&taken));
    assert_eq!(changes(&root), "");
    // And once it is committed.
    let run = atelier(&root, "t9", &["show", DONE]);
    assert!(run.stdout.contains(&format!(
        "    1-aaaa.1 (defer, taken into {created} by t9): The greeting ignores the locale\n\
         \x20   1-aaaa.2 (defer, dropped by t9: The help was rewritten.): Old wording in the help\n"
    )));

    let run = atelier(&root, "t9", &["list", "--deferred"]);
    assert_eq!(
        run.stdout,
        "No deferred finding of a done item is left to settle.\n"
    );
}

#[test]
fn settling_is_refused_with_nothing_written_and_says_why() {
    let scratch = with_done(&[]);
    let root = scratch.root();
    // An item in review, whose finding is deferred.
    write_item(&root, "demo-0pen", "in_review", &["t1"], &[], "Done.");
    write_review(
        &root,
        ".atelier/items/demo-0pen/reviews/1-bbbb.md",
        "null",
        "## Finding 1: Later\nRecommendation: defer\n",
    );
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Submit demo-0pen"]);
    assert_eq!(
        atelier(
            &root,
            "t9",
            &["drop", "demo-d0ne/1-aaaa.2", "--reason", "x"]
        )
        .code,
        0
    );
    let before = (changes(&root), read(&root, REVIEW));

    for (name, why) in [
        (
            "demo-d0ne/1-aaaa.2",
            "  [ ] Finding demo-d0ne/1-aaaa.2 is not settled yet (it was dropped by t9: x)\n",
        ),
        (
            "demo-0pen/1-bbbb.1",
            "  [ ] demo-0pen is done (it is in review; only a done item's findings are settled)\n",
        ),
        (
            "demo-d0ne/1-aaaa.3",
            "  [ ] Finding demo-d0ne/1-aaaa.3 is deferred, by its recommendation or by a decision (it is not)\n",
        ),
        (
            "demo-d0ne/1-aaaa.9",
            "  [ ] A review of demo-d0ne has a finding 1-aaaa.9 (none does)\n",
        ),
        (
            "demo-n0ne/1-aaaa.1",
            "  [ ] A review of demo-n0ne has a finding 1-aaaa.1 (none does)\n",
        ),
    ] {
        for (command, args) in [
            ("new --from", vec!["new", "Title", "--from", name]),
            ("drop", vec!["drop", name, "--reason", "y"]),
        ] {
            let run = atelier(&root, "t9", &args);
            assert_eq!(run.code, 1, "{args:?}: {}", run.stdout);
            assert_eq!(
                run.stderr,
                format!(
                    "`atelier {command} {name}` refused; nothing changed.\n{why}\
                     `atelier list --deferred` lists the deferred findings nobody has settled.\n"
                )
            );
            assert_eq!((changes(&root), read(&root, REVIEW)), before, "{args:?}");
        }
    }
    let run = atelier(&root, "t9", &["new", "Title", "--from", "demo-d0ne"]);
    assert_eq!(run.code, 1);
    assert!(run.stderr.contains("`demo-d0ne` is not a finding's name"));
    assert_eq!((changes(&root), read(&root, REVIEW)), before);
}

#[test]
fn new_from_writes_neither_the_item_nor_the_taken_line_when_one_cannot_be_written() {
    let scratch = with_done(&[]);
    let root = scratch.root();
    // A review longer than the file size the command may write, so its
    // write fails part way, after the new item is written.
    let long = FINDINGS.replace("Fine.", &"A long summary. ".repeat(600));
    write_review(&root, REVIEW, "approve", &long);
    git(&root, &["commit", "-qam", "Summarise"]);
    let before = read(&root, REVIEW);
    let output = Command::new("sh")
        .current_dir(&root)
        .env("T3CODE_THREAD_ID", "t9")
        .env_remove("CLAUDE_CODE_SESSION_ID")
        .env_remove("CODEX_SESSION_ID")
        .env_remove("GROK_SESSION_ID")
        .args([
            "-c",
            "trap '' XFSZ; ulimit -f 4; exec \"$0\" \"$@\"",
            env!("CARGO_BIN_EXE_atelier"),
            "new",
            "Greet in the locale",
            "--from",
            "demo-d0ne/1-aaaa.1",
        ])
        .output()
        .unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(
        stderr.contains("removed the new item it had written; nothing changed"),
        "{stderr}"
    );
    assert_eq!(read(&root, REVIEW), before);
    assert_eq!(changes(&root), "");
}

#[test]
fn a_settlement_on_an_approved_items_branch_is_a_change_its_merge_sends_back_to_review() {
    let scratch = with_done(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    git(&root, &["add", "hello.txt"]);
    git(&root, &["commit", "-qm", "Say hello"]);
    finish_and_submit(&root, "t1");
    assert_eq!(atelier(&root, "t2", &["review", ID]).code, 0);
    let run = atelier(&root, "t2", &["verdict", "approve"]);
    assert_eq!(run.code, 0, "{}", run.stderr);

    // Only the item's implementer settles a finding on its branch.
    let run = atelier(
        &root,
        "t3",
        &["drop", "demo-d0ne/1-aaaa.2", "--reason", "x"],
    );
    assert_eq!(run.code, 1);
    assert!(run.stderr.starts_with(
        "`atelier drop` settles a finding on the base branch main, or on the branch of an item you implement, and this checkout is on branch demo-ab12. Nothing changed.\n"
    ), "{}", run.stderr);
    assert_eq!(changes(&root), "");

    let run = atelier(
        &root,
        "t1",
        &[
            "drop",
            "demo-d0ne/1-aaaa.2",
            "--reason",
            "The help was rewritten.",
        ],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(run.stdout.contains(
        "Once committed, it reaches the base when demo-ab12 merges. It is outside demo-ab12's directory"
    ), "{}", run.stdout);
    assert_eq!(listed(&root), ["demo-d0ne/1-aaaa.1", "demo-d0ne/1-aaaa.4"]);
    shell(&root, then(&run.stdout));
    assert_eq!(changes(&root), "");
    let run = atelier(&root, "t1", &["show", DONE]);
    assert!(run.stdout.contains(
        "    1-aaaa.2 (defer, dropped by t1: The help was rewritten.): Old wording in the help\n"
    ), "{}", run.stdout);

    let run = atelier(&root, "t1", &["merge"]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(
        run.stderr.contains(&format!("(changed: {REVIEW})")),
        "{}",
        run.stderr
    );
    assert!(
        run.stderr.ends_with("Then: atelier submit\n"),
        "{}",
        run.stderr
    );
    // An item done on the base after the branch was made is not in this
    // checkout, so nothing of it is listed or settled here.
    let base = scratch.worktree("base");
    write_item(&base, "demo-l8tr", "done", &["t4"], &[], "Done.");
    write_review(
        &base,
        ".atelier/items/demo-l8tr/reviews/1-cccc.md",
        "approve",
        "## Finding 1: Later\nRecommendation: defer\n",
    );
    git(&base, &["add", "-A"]);
    git(&base, &["commit", "-qm", "Merge demo-l8tr"]);
    git(&base, &["branch", "-f", "main", "HEAD"]);
    assert_eq!(listed(&root), ["demo-d0ne/1-aaaa.1", "demo-d0ne/1-aaaa.4"]);
    let run = atelier(
        &root,
        "t1",
        &["drop", "demo-l8tr/1-cccc.1", "--reason", "x"],
    );
    assert_eq!(run.code, 1);
    assert!(run
        .stderr
        .contains("A review of demo-l8tr has a finding 1-cccc.1 (none does)"));
    // The base has not settled the branch's finding until the branch merges.
    assert_eq!(listed(&base).len(), 4);
}
