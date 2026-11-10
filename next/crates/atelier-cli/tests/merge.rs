//! `merge` driven in scratch repositories: the merge commit, what stops
//! it, the inputs that move while its checks run, and the forge, reached
//! through a fake `gh` and a local bare repository: what `submit` pushes,
//! and `merge`'s wait for CI.

mod common;

use common::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

/// A merge check that waits: it creates `$MERGE_MARK`, then waits until
/// `$MERGE_RELEASE` exists, or the mark is gone with the scratch directory,
/// so a check whose merge was killed does not outlive its test.
const WAIT: &str = "touch \"$MERGE_MARK\"; \
    while [ ! -f \"$MERGE_RELEASE\" ] && [ -f \"$MERGE_MARK\" ]; do sleep 0.05; done";

/// A scratch repository whose base config has `config` added, and whose
/// item t1 started, changed with `change` and submitted.
fn submitted_with(config: &str, change: impl Fn(&Path)) -> Scratch {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    if !config.is_empty() {
        append(&root, ".atelier/config.toml", config);
        git(&root, &["commit", "-qam", "Configure"]);
    }
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    change(&root);
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Say hello"]);
    finish_and_submit(&root, "t1");
    scratch
}

/// As [`submitted_with`], then approved by t2.
fn approved_with(config: &str, change: impl Fn(&Path)) -> Scratch {
    let scratch = submitted_with(config, change);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t2", &["review", ID]).code, 0);
    let run = atelier(&root, "t2", &["verdict", "approve"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    scratch
}

fn approved() -> Scratch {
    approved_with("", |_| {})
}

/// The config of a check that waits, with what it runs.
fn waiting_check() -> String {
    format!("\n[[merge_check]]\nrun = '{WAIT}'\n")
}

/// `atelier merge` as `thread` with `args` after it, its waiting check
/// released from the start.
fn merge(at: &Path, thread: &str, args: &[&str]) -> Run {
    let mut all = vec!["merge"];
    all.extend(args);
    // Beside the checkout, in the scratch directory.
    let release = at.parent().unwrap().join("released");
    fs::write(&release, "").unwrap();
    let output = atelier_command(at, thread, &all)
        .env("MERGE_MARK", at.parent().unwrap().join("mark"))
        .env("MERGE_RELEASE", &release)
        .output()
        .unwrap();
    run(output)
}

fn run(output: std::process::Output) -> Run {
    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

/// A merge stopped inside its waiting check until released. Dropped, as
/// when a test fails, it is released, so no process outlives the test.
struct Waiting {
    child: Option<Child>,
    release: PathBuf,
}

impl Waiting {
    fn start(at: &Path, thread: &str, scratch: &Path, name: &str) -> Waiting {
        Waiting::spawn(atelier_command(at, thread, &["merge"]), scratch, name)
    }

    /// Runs `merge`, which `command` is, until it waits in its check.
    fn spawn(mut command: Command, scratch: &Path, name: &str) -> Waiting {
        let mark = scratch.join(format!("{name}.mark"));
        let release = scratch.join(format!("{name}.release"));
        let child = command
            .env("MERGE_MARK", &mark)
            .env("MERGE_RELEASE", &release)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let waiting = Waiting {
            child: Some(child),
            release,
        };
        let since = Instant::now();
        while !mark.exists() {
            assert!(since.elapsed() < Duration::from_secs(60), "never checked");
            std::thread::sleep(Duration::from_millis(20));
        }
        waiting
    }

    fn finish(mut self) -> Run {
        fs::write(&self.release, "").unwrap();
        run(self.child.take().unwrap().wait_with_output().unwrap())
    }

    /// Kills the merge, leaving its check to end on its release.
    fn kill(&mut self) {
        let mut child = self.child.take().unwrap();
        child.kill().unwrap();
        child.wait().unwrap();
    }
}

impl Drop for Waiting {
    fn drop(&mut self) {
        let _ = fs::write(&self.release, "");
        if let Some(mut child) = self.child.take() {
            let _ = child.wait();
        }
    }
}

fn head(at: &Path, rev: &str) -> String {
    git(at, &["rev-parse", rev])
}

/// Everything a merge could have changed: the base's tip, the branch's
/// tip, the checkout's HEAD and its status.
fn state(root: &Path) -> [String; 4] {
    [
        head(root, "main"),
        head(root, ID),
        head(root, "HEAD"),
        git(root, &["status", "--porcelain"]),
    ]
}

#[test]
fn an_approved_item_merges_in_one_commit_that_marks_it_done() {
    // The check passes only on the merged tree, where the branch's file is.
    let scratch = approved_with("\n[[merge_check]]\nrun = \"test -f hello.txt\"\n", |_| {});
    let root = scratch.root();
    let base = scratch.worktree("base");
    git(&base, &["switch", "-q", "main"]);
    let (tip, branch) = (head(&root, "main"), head(&root, ID));

    // Status shows what merge will run.
    let status = atelier(&root, "t1", &["status"]);
    assert!(
        status.stdout.ends_with(
            "Status: approved\n`atelier merge` runs these merge checks on the tree merging makes:\n  test -f hello.txt\nThen: atelier merge\n"
        ),
        "{}",
        status.stdout
    );
    let run = merge(&root, "t1", &[]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let merged = head(&root, "main");
    assert_eq!(
        run.stdout,
        format!(
            "Merged {ID} into main with merge commit {}, which marks it done.\n",
            &merged[..7]
        )
    );
    // One merge commit, of the base and the branch, whose message ends
    // with the trailer; it changes the item's record and nothing else.
    assert_eq!(
        git(&root, &["rev-list", "--parents", "-n1", "main"]),
        format!("{merged} {tip} {branch}")
    );
    let message = git(&root, &["show", "-s", "--format=%B", "main"]);
    assert!(
        message.starts_with(&format!("Merge {ID}: Say hello\n")),
        "{message}"
    );
    assert!(
        message.ends_with(&format!("\n\nAtelier-Merge: {ID}")),
        "{message}"
    );
    assert_eq!(
        git(&root, &["diff", "--name-only", ID, "main"]),
        item_path(ID)
    );
    assert_eq!(field(&root, "main", "status"), "done");
    assert_eq!(field(&root, "main", "trivial"), "null");
    // The base's checkout moved with it, and nothing is left over.
    assert_eq!(head(&base, "HEAD"), merged);
    assert_eq!(git(&base, &["status", "--porcelain"]), "");
    assert_eq!(git(&root, &["status", "--porcelain"]), "");
    assert_eq!(git(&root, &["worktree", "list"]).lines().count(), 2);

    let after = atelier(&root, "t1", &["status"]);
    assert!(
        after
            .stdout
            .ends_with("Status: done\nNothing is left to do on it.\n"),
        "{}",
        after.stdout
    );
    // Run again, it has nothing to do and says so.
    let again = merge(&root, "t1", &[]);
    assert_eq!(again.code, 0);
    assert_eq!(
        again.stdout,
        format!("{ID} is already merged into main; nothing is left to do on it.\n")
    );
    assert_eq!(head(&root, "main"), merged);
}

/// Changes the approved item with `change`, commits it, and checks that
/// `merge` changes nothing, says `said` changed since round 1 and to run
/// `submit`, and that `status` says the same.
fn stale_after(change: impl Fn(&Path), said: &str) {
    let scratch = approved();
    let root = scratch.root();
    let reviewed = git(&root, &["show", &format!("{ID}:{}", review_of(&root, 1).1)]);
    let reviewed = front(&reviewed, "commit");
    change(&root);
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Change after approval"]);
    let before = state(&root);

    let run = merge(&root, "t1", &[]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    let needs = format!(
        "To merge it:\n  [ ] The approval still covers the head: since {}, which round 1 reviewed, nothing changed outside this item's directory, nor its Goal, Done when or Scope (changed: {said})\n\
         What changed needs a fresh review, so send it back to review.\nThen: atelier submit\n",
        &reviewed[..7]
    );
    assert_eq!(
        run.stderr,
        format!("`atelier merge` refused; nothing changed.\n{needs}")
    );
    assert_eq!(state(&root), before);
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(
        status.ends_with(&format!("Status: approved\n{needs}")),
        "{status}"
    );
    // The way out it names works.
    assert_eq!(atelier(&root, "t1", &["submit"]).code, 0);
}

#[test]
fn a_source_change_after_approval_makes_it_stale() {
    stale_after(
        |root| fs::write(root.join("hello.txt"), "hello!\n").unwrap(),
        "hello.txt",
    );
}

#[test]
fn a_config_change_after_approval_makes_it_stale() {
    stale_after(
        |root| {
            append(
                root,
                ".atelier/config.toml",
                "\n[[merge_check]]\nrun = \"true\"\n",
            )
        },
        ".atelier/config.toml",
    );
}

#[test]
fn a_change_to_another_item_after_approval_makes_it_stale() {
    stale_after(
        |root| write_item(root, "demo-cd34", "draft", &[], &[], ""),
        ".atelier/items/demo-cd34/item.md",
    );
}

#[test]
fn a_change_to_this_items_goal_done_when_or_scope_after_approval_makes_it_stale() {
    for (from, to, heading) in [
        ("Print hello.", "Print hello twice.", "Goal"),
        ("A test covers it.", "Two tests cover it.", "Done when"),
        ("Only hello.", "Hello and goodbye.", "Scope"),
    ] {
        stale_after(
            |root| {
                let path = root.join(item_path(ID));
                let text = fs::read_to_string(&path).unwrap();
                fs::write(&path, text.replacen(from, to, 1)).unwrap();
            },
            &format!("this item's {heading}"),
        );
    }
}

/// A review of round 1 with no verdict.
const UNDECIDED: &str = "---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: abc1234\n\
    reviewers:\n- r9\nmodel: null\nverdict: null\ncreated_at: 2026-11-10T09:00:00Z\n---\n## Summary\ns\n";

#[test]
fn a_report_or_a_review_file_added_after_approval_does_not_block_the_merge() {
    let scratch = approved();
    let root = scratch.root();
    append(&root, &item_path(ID), "\nAlso checked by hand.\n");
    // A second reviewer's file of round 1, which it left without a verdict.
    fs::write(root.join(REVIEWS).join("1-zzzz.md"), UNDECIDED).unwrap();
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Report more"]);
    let run = merge(&root, "t1", &[]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(field(&root, "main", "status"), "done");
}

#[test]
fn a_failing_merge_check_on_the_base_tip_stops_the_merge_with_its_command_and_output() {
    let check = "echo first line; echo the cause >&2; exit 3";
    // The work branch removes the check from its own config, and the base's
    // still runs.
    let scratch = approved_with(&format!("\n[[merge_check]]\nrun = \"{check}\"\n"), |root| {
        let path = root.join(".atelier/config.toml");
        let text = fs::read_to_string(&path).unwrap();
        let kept = text.split("\n[[merge_check]]").next().unwrap().to_string();
        fs::write(&path, kept).unwrap();
    });
    let root = scratch.root();
    assert!(!read(&root, ".atelier/config.toml").contains("merge_check"));
    let before = state(&root);

    let run = merge(&root, "t1", &[]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier merge` refused: the merge check `{check}` failed (exit 3) on {ID} merged into main. Nothing changed.\n\
             The end of its output:\n  first line\n  the cause\n\
             Fix it on branch {ID} and commit; if main broke it, bring main in first with `git merge main`. `atelier status` then says what comes next.\n"
        )
    );
    assert_eq!(state(&root), before);
    assert_eq!(git(&root, &["worktree", "list"]).lines().count(), 1);
}

#[test]
fn an_unanswered_question_stops_the_merge_and_is_named() {
    let scratch = approved();
    let root = scratch.root();
    append(
        &root,
        &item_path(ID),
        "\n## Decisions\n\n### 2026-11-10: Should hello be loud?\n",
    );
    git(&root, &["commit", "-qam", "Ask"]);
    let before = state(&root);
    let run = merge(&root, "t1", &[]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr
            .contains("  [ ] Question answered: 2026-11-10: Should hello be loud?\n"),
        "{}",
        run.stderr
    );
    assert_eq!(state(&root), before);

    append(&root, &item_path(ID), "Answer by d1: No.\n");
    git(&root, &["commit", "-qam", "Answer"]);
    assert_eq!(merge(&root, "t1", &[]).code, 0);
}

#[test]
fn merge_judges_the_committed_item_not_the_checkouts() {
    // The committed Goal changed since the review; the checkout puts the
    // reviewed Goal back without committing it.
    let scratch = approved();
    let root = scratch.root();
    let path = root.join(item_path(ID));
    let reviewed = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        reviewed.replacen("Print hello.", "Print hello twice.", 1),
    )
    .unwrap();
    git(&root, &["commit", "-qam", "Change the Goal"]);
    fs::write(&path, &reviewed).unwrap();
    let before = state(&root);
    let run = merge(&root, "t1", &[]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.contains("(changed: this item's Goal)\n"),
        "{}",
        run.stderr
    );
    assert_eq!(state(&root), before);
    // Status judges the same commit, and says the same.
    let status = atelier(&root, "t1", &["status"]).stdout;
    let said = status.contains("(changed: this item's Goal)\n");
    assert!(
        said && status.ends_with("Then: atelier submit\n"),
        "{status}"
    );
}

#[test]
fn uncommitted_changes_in_the_checkout_stop_the_merge() {
    let scratch = approved();
    let root = scratch.root();
    fs::write(root.join("hello.txt"), "hello?\n").unwrap();
    let before = state(&root);
    let run = merge(&root, "t1", &[]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.contains(
            "  [ ] Every change to a tracked file or to this item's directory is committed (these are not: hello.txt)\nCommit them, or set them aside with `git stash`.\n"
        ),
        "{}",
        run.stderr
    );
    assert_eq!(state(&root), before);
}

#[test]
fn a_branch_that_conflicts_with_the_base_stops_the_merge() {
    let scratch = approved();
    let root = scratch.root();
    let base = scratch.worktree("base");
    git(&base, &["switch", "-q", "main"]);
    fs::write(base.join("hello.txt"), "goodbye\n").unwrap();
    git(&base, &["add", "hello.txt"]);
    git(&base, &["commit", "-qm", "Say goodbye"]);
    let before = state(&root);
    let run = merge(&root, "t1", &[]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.starts_with(&format!(
            "`atelier merge` refused: {ID} does not merge cleanly into main; these conflict: hello.txt. Nothing changed.\n"
        )),
        "{}",
        run.stderr
    );
    assert_eq!(state(&root), before);
}

/// An approved item whose base runs the waiting check, and `merge` held in
/// it.
fn held() -> (Scratch, Waiting) {
    let scratch = approved_with(&waiting_check(), |_| {});
    let root = scratch.root();
    let waiting = Waiting::start(&root, "t1", scratch.dir.path(), "merge");
    (scratch, waiting)
}

/// Checks that `run` wrote nothing because `what` changed, and that `merge`
/// run again then merges.
fn wrote_nothing(scratch: &Scratch, run: Run, what: &str, base: &str) {
    let root = scratch.root();
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier merge` wrote nothing: {what} changed while its checks ran, so they do not cover what would merge now.\nRun `atelier merge` again.\n"
        )
    );
    assert_eq!(head(&root, "main"), base);
    assert_eq!(field(&root, "main", "status"), "ready");
}

#[test]
fn a_base_that_moves_while_the_checks_run_is_not_written() {
    let (scratch, waiting) = held();
    let root = scratch.root();
    let base = scratch.worktree("base");
    git(&base, &["switch", "-q", "main"]);
    fs::write(base.join("other.txt"), "other\n").unwrap();
    git(&base, &["add", "other.txt"]);
    git(&base, &["commit", "-qm", "Meanwhile"]);
    let moved = head(&root, "main");
    let run = waiting.finish();
    wrote_nothing(&scratch, run, "the base branch main", &moved);
    // Run again, it merges onto the base as it now is.
    assert_eq!(merge(&root, "t1", &[]).code, 0);
    assert_eq!(head(&root, "main^1"), moved);
}

#[test]
fn a_branch_that_moves_while_the_checks_run_is_not_written() {
    let (scratch, waiting) = held();
    let root = scratch.root();
    let tip = head(&root, "main");
    append(&root, &item_path(ID), "\nMore report.\n");
    git(&root, &["commit", "-qam", "Meanwhile"]);
    let branch = head(&root, ID);
    let run = waiting.finish();
    wrote_nothing(&scratch, run, &format!("branch {ID}"), &tip);
    assert_eq!(merge(&root, "t1", &[]).code, 0);
    assert_eq!(head(&root, "main^2"), branch);
}

#[test]
fn a_commit_made_after_merge_read_its_facts_is_not_merged() {
    // The commit lands after `merge` has checked the approval and before
    // its checks run, so only the confirmation at the end can catch it.
    let scratch = approved();
    let root = scratch.root();
    let tip = head(&root, "main");
    let release = scratch.dir.path().join("read");
    let child = atelier_command(&root, "t1", &["merge"])
        .env("ATELIER_TEST_HOLD_AFTER_READ", &release)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let held = release.with_extension("held");
    let since = Instant::now();
    while !held.exists() {
        assert!(since.elapsed() < Duration::from_secs(60), "never held");
        std::thread::sleep(Duration::from_millis(20));
    }
    fs::write(root.join("hello.txt"), "unreviewed\n").unwrap();
    git(&root, &["commit", "-qam", "Unreviewed"]);
    fs::write(&release, "").unwrap();
    let run = run(child.wait_with_output().unwrap());
    wrote_nothing(&scratch, run, &format!("branch {ID}"), &tip);
    // Run again, it sees the change and says to submit.
    let again = merge(&root, "t1", &[]);
    assert_eq!(again.code, 1);
    assert!(
        again.stderr.ends_with("Then: atelier submit\n"),
        "{}",
        again.stderr
    );
}

#[test]
fn a_checkout_that_changes_while_the_checks_run_is_not_written() {
    let (scratch, waiting) = held();
    let root = scratch.root();
    let tip = head(&root, "main");
    fs::write(root.join("hello.txt"), "hello?\n").unwrap();
    let run = waiting.finish();
    wrote_nothing(
        &scratch,
        run,
        &format!("the checkout at {}", root.display()),
        &tip,
    );
    assert_eq!(read(&root, "hello.txt"), "hello?\n");
}

#[test]
fn of_two_merges_at_once_the_second_waits_for_the_first() {
    const OTHER: &str = "demo-cd34";
    let scratch = Scratch::new(&[(ID, "ready"), (OTHER, "ready")]);
    let root = scratch.root();
    append(&root, ".atelier/config.toml", &waiting_check());
    git(&root, &["commit", "-qam", "Configure"]);
    let base = head(&root, "main");
    let other = scratch.worktree("other");
    for (at, id) in [(&root, ID), (&other, OTHER)] {
        assert_eq!(atelier(at, "t1", &["start", id]).code, 0);
        fs::write(at.join(format!("{id}.txt")), "work\n").unwrap();
        let path = at.join(item_path(id));
        let text = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
        fs::write(&path, text + "Done.\n").unwrap();
        git(at, &["add", "-A"]);
        git(at, &["commit", "-qm", "Work"]);
        assert_eq!(atelier(at, "t1", &["submit"]).code, 0);
        assert_eq!(atelier(at, "t2", &["review", id]).code, 0);
        assert_eq!(atelier(at, "t2", &["verdict", "approve"]).code, 0);
    }

    let first = Waiting::start(&root, "t1", scratch.dir.path(), "first");
    // The second's check does not wait; the lock does.
    let released = scratch.dir.path().join("released");
    fs::write(&released, "").unwrap();
    let mut second = atelier_command(&other, "t1", &["merge"])
        .env("MERGE_MARK", scratch.dir.path().join("second.mark"))
        .env("MERGE_RELEASE", &released)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let since = Instant::now();
    while since.elapsed() < Duration::from_secs(2) {
        assert!(
            second.try_wait().unwrap().is_none(),
            "the second merge ran while the first held the lock"
        );
        std::thread::sleep(Duration::from_millis(50));
    }
    let first = first.finish();
    assert_eq!(first.code, 0, "{}", first.stderr);
    let second = run(second.wait_with_output().unwrap());
    assert_eq!(second.code, 0, "{}", second.stderr);
    assert_eq!(
        second.stderr,
        "Another `atelier merge` is running in this clone; this one waits for it to finish.\n"
    );
    // One line of merges on the base: the first, then the second on it.
    assert_eq!(head(&root, "main^1^1"), base);
    assert_eq!(head(&root, "main^1^2"), head(&root, ID));
    assert_eq!(head(&root, "main^2"), head(&root, OTHER));
}

#[test]
fn a_merge_killed_during_its_checks_leaves_nothing_the_next_run_trips_on() {
    let (scratch, mut waiting) = held();
    let root = scratch.root();
    let before = state(&root);
    waiting.kill();
    assert_eq!(state(&root), before);
    // Its merged checkout is left behind, and the next run removes it.
    assert_eq!(git(&root, &["worktree", "list"]).lines().count(), 2);
    let run = merge(&root, "t1", &[]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(git(&root, &["worktree", "list"]).lines().count(), 1);
    let common = root.join(".git");
    let left: Vec<_> = fs::read_dir(&common)
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .filter(|name| name.starts_with("atelier-merge-"))
        .collect();
    assert!(left.is_empty(), "{left:?}");
}

#[test]
fn a_trivial_merge_needs_no_review_and_stores_its_reason() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Say hello"]);
    // Unticked and with no Report, it is refused for both.
    let run = merge(&root, "t1", &["--trivial", "a typo"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.contains("  [ ] Done when: A test covers it.\n")
            && run
                .stderr
                .contains("  [ ] Report: written under `## Report`\n")
            && run
                .stderr
                .ends_with("Then: atelier merge --trivial \"<why>\"\n"),
        "{}",
        run.stderr
    );
    let path = root.join(item_path(ID));
    let text = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, text + "It prints hello.\n").unwrap();
    git(&root, &["commit", "-qam", "Report"]);
    // No reason is no trivial merge.
    let run = merge(&root, "t1", &["--trivial", " "]);
    assert_eq!(run.code, 1);
    assert!(run
        .stderr
        .starts_with("`atelier merge --trivial` needs a reason"));
    // Without --trivial, a building item is not merged.
    assert_eq!(merge(&root, "t1", &[]).code, 1);

    let run = merge(&root, "t1", &["--trivial", "a typo"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(field(&root, "main", "status"), "done");
    assert_eq!(field(&root, "main", "trivial"), "a typo");
    assert_eq!(head(&root, "main^2"), head(&root, ID));
}

#[test]
fn a_trivial_merge_is_refused_on_an_item_with_a_review_file() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    let path = root.join(item_path(ID));
    let text = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, text + "It prints hello.\n").unwrap();
    fs::create_dir_all(root.join(REVIEWS)).unwrap();
    fs::write(root.join(REVIEWS).join("1-aaaa.md"), UNDECIDED).unwrap();
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Reviewed"]);
    let before = state(&root);
    let run = merge(&root, "t1", &["--trivial", "a typo"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.contains("  [ ] Nobody has reviewed it (it has a review file, so it merges once approved, with `atelier merge`)\n"),
        "{}",
        run.stderr
    );
    assert_eq!(state(&root), before);
}

#[test]
fn merge_takes_no_flag_but_trivial() {
    let scratch = approved();
    let root = scratch.root();
    for flag in ["--push", "--wait", "--check", "--sync"] {
        let run = merge(&root, "t1", &[flag]);
        assert_eq!(run.code, 2, "{flag}");
    }
    let help = atelier(&root, "t1", &["merge", "--help"]).stdout;
    let flags: Vec<&str> = help
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('-'))
        .collect();
    assert_eq!(flags.len(), 2, "{help}");
    assert!(flags[0].starts_with("--trivial <WHY>") && flags[1].starts_with("-h, --help"));
}

/// A fake `gh` that answers from files in `$FAKE_GH`: `runs/<branch>` for
/// the CI runs of a branch's pull requests, `pulls` for the open pull
/// requests. A POST to the pull requests opens #7 and adds a line to
/// `opened`. Every call is logged to `calls`. While `down` exists a runs
/// lookup fails, and while `stall` exists it never answers, after writing
/// its process ID to `stalled`; while `no-pulls` exists opening a pull
/// request fails.
const FAKE_GH: &str = r#"#!/bin/sh
echo "$*" >> "$FAKE_GH/calls"
if [ "$1" = auth ]; then exit 0; fi
method=GET
previous=""
answer=""
for arg in "$@"; do
  if [ "$previous" = -X ]; then method="$arg"; fi
  previous="$arg"
  case "$arg" in
    branch=*) answer="runs/${arg#branch=}" ;;
    */pulls) answer=pulls ;;
  esac
done
case "$answer" in
  runs/*)
    if [ -f "$FAKE_GH/stall" ]; then echo $$ > "$FAKE_GH/stalled"; exec sleep 600; fi
    if [ -f "$FAKE_GH/down" ]; then echo "HTTP 502: Bad Gateway" >&2; exit 1; fi
    ;;
esac
if [ "$answer" = pulls ] && [ "$method" = POST ]; then
  if [ -f "$FAKE_GH/no-pulls" ]; then echo "HTTP 422: Validation Failed" >&2; exit 1; fi
  echo '[{"number":7,"html_url":"https://github.com/owner/repo/pull/7"}]' > "$FAKE_GH/pulls"
  echo opened >> "$FAKE_GH/opened"
  echo '{"number":7,"html_url":"https://github.com/owner/repo/pull/7"}'
  exit 0
fi
if [ -f "$FAKE_GH/$answer" ]; then
  cat "$FAKE_GH/$answer"
elif [ "$answer" = pulls ]; then
  echo '[]'
else
  echo '{"total_count":0,"workflow_runs":[]}'
fi
"#;

const PULL: &str = "Pull request #7: https://github.com/owner/repo/pull/7\n";

/// A repository whose base config names a forge, with `ci_workflow` when
/// given. Its `origin` is a GitHub URL, pushes to it go to a local bare
/// repository, and every command runs with the fake `gh` first on `PATH`.
struct Forge {
    scratch: Scratch,
    gh: PathBuf,
    remote: PathBuf,
}

impl Forge {
    /// An item submitted, and approved by t2.
    fn new(ci_workflow: Option<&str>) -> Forge {
        Forge::with(ci_workflow, "")
    }

    /// As [`Forge::new`], with `checks` added to the base's config.
    fn with(ci_workflow: Option<&str>, checks: &str) -> Forge {
        let forge = Forge::set_up(ci_workflow, checks, &[ID]);
        let root = forge.scratch.root();
        forge.start(&root, ID);
        let run = forge.submit(&root, ID);
        assert_eq!(run.code, 0, "{}", run.stderr);
        forge.approve(&root, ID);
        forge
    }

    /// The repository with `ids` ready, and nothing started.
    fn set_up(ci_workflow: Option<&str>, checks: &str, ids: &[&str]) -> Forge {
        let items: Vec<(&str, &str)> = ids.iter().map(|id| (*id, "ready")).collect();
        let scratch = Scratch::new(&items);
        let root = scratch.root();
        let workflow = ci_workflow.map_or(String::new(), |w| format!("ci_workflow = \"{w}\"\n"));
        let config =
            format!("{checks}\n[forge]\nprovider = \"github\"\nremote = \"origin\"\n{workflow}");
        append(&root, ".atelier/config.toml", &config);
        git(&root, &["commit", "-qam", "Configure"]);
        let gh = scratch.dir.path().join("gh");
        fs::create_dir_all(gh.join("bin")).unwrap();
        fs::create_dir_all(gh.join("runs")).unwrap();
        let fake = gh.join("bin/gh");
        fs::write(&fake, FAKE_GH).unwrap();
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&fake, fs::Permissions::from_mode(0o755)).unwrap();
        let remote = scratch.dir.path().join("remote.git");
        git(
            &root,
            &[
                "init",
                "-q",
                "--bare",
                "-b",
                "main",
                remote.to_str().unwrap(),
            ],
        );
        git(&root, &["push", "-q", remote.to_str().unwrap(), "main"]);
        let url = "git@github.com:owner/repo.git";
        git(&root, &["remote", "add", "origin", url]);
        let rewrite = format!("url.{}.pushInsteadOf", remote.display());
        git(&root, &["config", &rewrite, url]);
        Forge {
            scratch,
            gh,
            remote,
        }
    }

    /// Starts `id` in the checkout `at` as t1, and commits a change.
    fn start(&self, at: &Path, id: &str) {
        let run = self.atelier(at, "t1", &["start", id]);
        assert_eq!(run.code, 0, "{}", run.stderr);
        fs::write(at.join(format!("{id}.txt")), "work\n").unwrap();
        git(at, &["add", "-A"]);
        git(at, &["commit", "-qm", "Work"]);
    }

    /// Ticks `id`'s Done when lines, writes its Report and submits it.
    fn submit(&self, at: &Path, id: &str) -> Run {
        let path = at.join(item_path(id));
        let text = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
        fs::write(&path, text + "Done.\n").unwrap();
        self.atelier(at, "t1", &["submit"])
    }

    fn approve(&self, at: &Path, id: &str) {
        assert_eq!(self.atelier(at, "t2", &["review", id]).code, 0);
        let run = self.atelier(at, "t2", &["verdict", "approve"]);
        assert_eq!(run.code, 0, "{}", run.stderr);
    }

    fn atelier(&self, at: &Path, thread: &str, args: &[&str]) -> Run {
        run(self.command(at, thread, args).output().unwrap())
    }

    /// `atelier` with the fake gh first on `PATH`, and a wait for CI of 20
    /// seconds, polling every 50 milliseconds.
    fn command(&self, at: &Path, thread: &str, args: &[&str]) -> Command {
        let path = format!(
            "{}:{}",
            self.gh.join("bin").display(),
            std::env::var("PATH").unwrap()
        );
        let mut command = atelier_command(at, thread, args);
        command
            .env("PATH", path)
            .env("FAKE_GH", &self.gh)
            .env("ATELIER_TEST_CI_WAIT", "20000,50");
        command
    }

    /// `atelier merge` as t1 in the main checkout.
    fn merge(&self) -> Run {
        self.atelier(&self.scratch.root(), "t1", &["merge"])
    }

    /// `atelier merge` as t1, with a wait of `wait` milliseconds.
    fn merge_waiting(&self, wait: u64) -> Run {
        let mut command = self.command(&self.scratch.root(), "t1", &["merge"]);
        command.env("ATELIER_TEST_CI_WAIT", format!("{wait},50"));
        run(command.output().unwrap())
    }

    /// `atelier merge` of `id` in `at`, started and left once it has
    /// asked for CI twice, so it is waiting.
    fn waiting(&self, at: &Path, id: &str) -> Child {
        let child = self
            .command(at, "t1", &["merge"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let since = Instant::now();
        while self.polls(id) < 2 {
            assert!(since.elapsed() < Duration::from_secs(20), "never waited");
            std::thread::sleep(Duration::from_millis(20));
        }
        child
    }

    /// The forge's copy of `branch`, if it has one.
    fn remote(&self, branch: &str) -> Option<String> {
        let output = git_output(
            &self.scratch.root(),
            &[
                "--git-dir",
                self.remote.to_str().unwrap(),
                "rev-parse",
                "--verify",
                "--quiet",
                &format!("refs/heads/{branch}"),
            ],
        );
        output
            .status
            .success()
            .then(|| String::from_utf8(output.stdout).unwrap().trim().to_string())
    }

    /// Answers the runs lookup of `branch` with `runs`, each a commit, a
    /// status and a conclusion, oldest first.
    fn ci(&self, branch: &str, runs: &[(&str, &str, Option<&str>)]) {
        let runs: Vec<String> = (1..)
            .zip(runs)
            .map(|(n, (commit, status, conclusion))| {
                let conclusion = conclusion.map_or("null".to_string(), |c| format!("\"{c}\""));
                format!(
                    "{{\"id\":{n},\"head_sha\":\"{commit}\",\"status\":\"{status}\",\"conclusion\":{conclusion},\
                     \"html_url\":\"https://github.com/owner/repo/actions/runs/{n}\",\"created_at\":\"2026-11-10T10:0{n}:00Z\"}}"
                )
            })
            .collect();
        fs::write(
            self.gh.join("runs").join(branch),
            format!(
                "{{\"total_count\":{},\"workflow_runs\":[{}]}}",
                runs.len(),
                runs.join(",")
            ),
        )
        .unwrap();
    }

    /// Sets or clears the fake gh's flag `name`.
    fn flag(&self, name: &str, on: bool) {
        let path = self.gh.join(name);
        if on {
            fs::write(path, "").unwrap();
        } else {
            let _ = fs::remove_file(path);
        }
    }

    /// How many pull requests it opened.
    fn opened(&self) -> usize {
        fs::read_to_string(self.gh.join("opened"))
            .unwrap_or_default()
            .lines()
            .count()
    }

    /// How many times the runs of `branch` were asked for.
    fn polls(&self, branch: &str) -> usize {
        fs::read_to_string(self.gh.join("calls"))
            .unwrap_or_default()
            .lines()
            .filter(|call| call.contains(&format!("branch={branch} ")))
            .count()
    }

    /// The commit `submit` made: the one before `verdict approve`'s.
    fn submitted(&self) -> String {
        head(&self.scratch.root(), &format!("{ID}^"))
    }
}

fn merged_and_pushed(root: &Path) -> String {
    format!(
        "Merged {ID} into main with merge commit {}, which marks it done.\nPushed main to origin.\n",
        &head(root, "main")[..7]
    )
}

#[test]
fn with_a_forge_submit_pushes_the_branch_and_opens_its_pull_request() {
    let forge = Forge::set_up(Some("ci.yml"), "", &[ID]);
    let root = forge.scratch.root();
    forge.start(&root, ID);
    let run = forge.submit(&root, ID);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        format!(
            "Submitted {ID} for review.\nPushed branch {ID} to origin. {PULL}\
             Start a reviewer with: Run `atelier review {ID}` and follow what it prints.\n"
        )
    );
    assert_eq!(forge.remote(ID), Some(head(&root, ID)));
    assert_eq!(field(&root, ID, "status"), "in_review");
    assert_eq!(forge.opened(), 1);

    // Submitted again, from approved, it pushes again and reuses the pull
    // request.
    forge.approve(&root, ID);
    let run = forge.submit(&root, ID);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stdout
            .contains(&format!("Pushed branch {ID} to origin. {PULL}")),
        "{}",
        run.stdout
    );
    assert_eq!(forge.remote(ID), Some(head(&root, ID)));
    assert_eq!(forge.opened(), 1);
}

#[test]
fn a_push_that_fails_at_submit_is_a_warning_and_merge_pushes_and_opens_before_it_waits() {
    let forge = Forge::set_up(Some("ci.yml"), "", &[ID]);
    let root = forge.scratch.root();
    // The forge refuses every push of the branch until the hook goes.
    let hook = forge.remote.join("hooks/pre-receive");
    fs::write(
        &hook,
        format!("#!/bin/sh\nwhile read old new ref; do [ \"$ref\" = refs/heads/{ID} ] && exit 1; done\nexit 0\n"),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
    forge.start(&root, ID);
    let run = forge.submit(&root, ID);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stdout.starts_with(&format!(
            "Submitted {ID} for review.\nWarning: pushing branch {ID} to origin failed, so CI has not started: "
        )),
        "{}",
        run.stdout
    );
    assert!(
        run.stdout.contains(
            "The item is in review all the same; `atelier merge` will push the branch.\nStart a reviewer with: "
        ),
        "{}",
        run.stdout
    );
    assert_eq!(field(&root, ID, "status"), "in_review");
    assert_eq!(forge.remote(ID), None);
    assert_eq!(forge.opened(), 0);

    fs::remove_file(&hook).unwrap();
    forge.approve(&root, ID);
    let run = forge.merge_waiting(300);
    assert_eq!(run.code, 3, "{}", run.stderr);
    assert!(
        run.stdout.starts_with(&format!(
            "Pushed branch {ID} to origin. {PULL}Waiting up to "
        )),
        "{}",
        run.stdout
    );
    assert_eq!(forge.remote(ID), Some(head(&root, ID)));
    assert_eq!(forge.opened(), 1);
}

#[test]
fn a_pull_request_that_fails_at_submit_is_a_warning_and_merge_opens_it_without_pushing() {
    let forge = Forge::set_up(Some("ci.yml"), "", &[ID]);
    let root = forge.scratch.root();
    forge.flag("no-pulls", true);
    forge.start(&root, ID);
    let run = forge.submit(&root, ID);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stdout.contains(&format!(
            "Warning: Pushed branch {ID}, but opening its pull request failed, so CI has not started: gh failed: HTTP 422: Validation Failed. The item is in review all the same; `atelier merge` will open the pull request.\n"
        )),
        "{}",
        run.stdout
    );
    assert_eq!(field(&root, ID, "status"), "in_review");
    let submitted = head(&root, ID);
    assert_eq!(forge.remote(ID), Some(submitted.clone()));
    assert_eq!(forge.opened(), 0);

    forge.flag("no-pulls", false);
    forge.approve(&root, ID);
    let run = forge.merge_waiting(300);
    assert_eq!(run.code, 3, "{}", run.stderr);
    assert!(
        run.stdout.starts_with(&format!("{PULL}Waiting up to ")),
        "{}",
        run.stdout
    );
    assert_eq!(forge.remote(ID), Some(submitted));
    assert_eq!(forge.opened(), 1);
}

#[test]
fn a_run_that_passed_on_the_submit_commit_counts_after_the_approval() {
    let forge = Forge::new(Some("ci.yml"));
    let root = forge.scratch.root();
    let (submitted, approved) = (forge.submitted(), head(&root, ID));
    // A newer run, on the approval's commit, is still going.
    forge.ci(
        ID,
        &[
            (&submitted, "completed", Some("success")),
            (&approved, "in_progress", None),
        ],
    );
    let status = forge.atelier(&root, "t1", &["status"]).stdout;
    assert!(
        status.ends_with(
            "It pushes the branch to origin and opens its pull request where `atelier submit` did not, waits up to 9 minutes for ci.yml to pass on the branch's latest code, then merges and pushes the base.\nThen: atelier merge\n"
        ),
        "{status}"
    );
    let run = forge.merge();
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(run.stdout, format!("{PULL}{}", merged_and_pushed(&root)));
    assert_eq!(forge.polls(ID), 2, "one before the lock, one under it");
    // The branch was not pushed again.
    assert_eq!(forge.remote(ID), Some(submitted));
    assert_eq!(forge.remote("main"), Some(head(&root, "main")));
    assert_eq!(head(&root, "main^2"), approved);
}

#[test]
fn a_run_on_a_commit_before_a_change_outside_the_items_directory_does_not_count() {
    let forge = Forge::new(Some("ci.yml"));
    let root = forge.scratch.root();
    let before = state(&root);
    // The start commit, before `Work` changed a file outside the item's
    // directory, and a commit this clone does not have.
    let started = head(&root, &format!("{ID}~3"));
    forge.ci(
        ID,
        &[
            (&started, "completed", Some("success")),
            (
                "0123456789abcdef0123456789abcdef01234567",
                "completed",
                Some("success"),
            ),
        ],
    );
    let run = forge.merge_waiting(300);
    assert_eq!(run.code, 3, "{}", run.stderr);
    assert!(
        run.stdout.ends_with(&format!(
            "CI has not finished on {}, the branch's head: no run of it has started. Nothing else changed.\nRun `atelier merge` again once ci.yml finishes.\n",
            &head(&root, ID)[..7]
        )),
        "{}",
        run.stdout
    );
    assert_eq!(state(&root), before);
}

#[test]
fn when_the_latest_finished_run_failed_and_none_is_running_merge_refuses() {
    let forge = Forge::new(Some("ci.yml"));
    let root = forge.scratch.root();
    let (submitted, approved) = (forge.submitted(), head(&root, ID));
    let (before, base) = (state(&root), forge.remote("main"));
    forge.ci(
        ID,
        &[
            (&submitted, "completed", Some("success")),
            (&approved, "completed", Some("failure")),
        ],
    );
    let run = forge.merge();
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert_eq!(run.stdout, PULL);
    assert!(
        run.stderr.starts_with(&format!(
            "`atelier merge` refused: CI failed: ci.yml finished failure on {} (https://github.com/owner/repo/actions/runs/2). Nothing was merged.\n",
            &approved[..7]
        )),
        "{}",
        run.stderr
    );
    assert_eq!(state(&root), before);
    assert_eq!(forge.remote("main"), base);
}

#[test]
fn merge_waits_while_a_newer_run_is_going_and_merges_as_soon_as_it_passes() {
    let forge = Forge::new(Some("ci.yml"));
    let root = forge.scratch.root();
    let (submitted, approved) = (forge.submitted(), head(&root, ID));
    let fails = (approved.as_str(), "completed", Some("failure"));
    forge.ci(ID, &[fails, (&approved, "in_progress", None)]);
    let since = Instant::now();
    let child = forge.waiting(&root, ID);
    // Waiting, it leaves the forge's branch where `submit` put it.
    assert_eq!(forge.remote(ID), Some(submitted.clone()));
    assert_eq!(forge.opened(), 1);
    forge.ci(ID, &[fails, (&approved, "completed", Some("success"))]);
    let run = run(child.wait_with_output().unwrap());
    assert!(since.elapsed() < Duration::from_secs(15));
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        format!(
            "{PULL}Waiting up to 20 seconds for ci.yml to finish on pull request #7. Nothing is written while it waits, so this may be interrupted and `atelier merge` run again.\n{}",
            merged_and_pushed(&root)
        )
    );
    assert_eq!(forge.remote(ID), Some(submitted));
}

#[test]
fn merge_refuses_as_soon_as_the_run_it_waits_for_fails() {
    let forge = Forge::new(Some("ci.yml"));
    let root = forge.scratch.root();
    let approved = head(&root, ID);
    let before = state(&root);
    forge.ci(ID, &[(&approved, "queued", None)]);
    let since = Instant::now();
    let child = forge.waiting(&root, ID);
    forge.ci(ID, &[(&approved, "completed", Some("timed_out"))]);
    let run = run(child.wait_with_output().unwrap());
    assert!(since.elapsed() < Duration::from_secs(15));
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(
        run.stderr
            .starts_with("`atelier merge` refused: CI failed: ci.yml finished timed_out on "),
        "{}",
        run.stderr
    );
    assert_eq!(state(&root), before);
}

#[test]
fn at_the_bound_merge_exits_not_yet_and_changes_nothing() {
    let forge = Forge::new(Some("ci.yml"));
    let root = forge.scratch.root();
    let approved = head(&root, ID);
    // A cancelled run is not a finished one.
    forge.ci(
        ID,
        &[
            (&approved, "completed", Some("cancelled")),
            (&approved, "in_progress", None),
        ],
    );
    let (before, branch, base) = (state(&root), forge.remote(ID), forge.remote("main"));
    let since = Instant::now();
    let run = forge.merge_waiting(2000);
    assert!(since.elapsed() >= Duration::from_secs(2));
    assert_eq!(run.code, 3, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        format!(
            "{PULL}Waiting up to 2 seconds for ci.yml to finish on pull request #7. Nothing is written while it waits, so this may be interrupted and `atelier merge` run again.\n\
             CI has not finished on {}, the branch's head: it is in progress (https://github.com/owner/repo/actions/runs/2). Nothing else changed.\nRun `atelier merge` again once ci.yml finishes.\n",
            &approved[..7]
        )
    );
    assert_eq!(run.stderr, "");
    assert_eq!(state(&root), before);
    assert_eq!((forge.remote(ID), forge.remote("main")), (branch, base));
}

#[test]
fn a_cancelled_run_is_not_a_finished_one() {
    let forge = Forge::new(Some("ci.yml"));
    let root = forge.scratch.root();
    forge.ci(ID, &[(&head(&root, ID), "completed", Some("cancelled"))]);
    let run = forge.merge_waiting(300);
    assert_eq!(run.code, 3, "{}", run.stderr);
    assert!(
        run.stdout.contains(
            "the branch's head: the latest was cancelled (https://github.com/owner/repo/actions/runs/1)."
        ),
        "{}",
        run.stdout
    );
}

#[test]
fn a_poll_that_fails_counts_as_not_finished() {
    let forge = Forge::new(Some("ci.yml"));
    let root = forge.scratch.root();
    let before = state(&root);
    forge.flag("down", true);
    let run = forge.merge_waiting(1000);
    assert_eq!(run.code, 3, "{}", run.stderr);
    assert!(
        run.stdout.ends_with(
            "the branch's head: the forge did not answer, as gh failed: HTTP 502: Bad Gateway. Nothing else changed.\nRun `atelier merge` again once ci.yml finishes.\n"
        ),
        "{}",
        run.stdout
    );
    assert!(forge.polls(ID) > 2, "it kept polling");
    assert_eq!(state(&root), before);
}

#[test]
fn a_gh_that_never_answers_is_ended_at_the_bound() {
    let forge = Forge::new(Some("ci.yml"));
    let root = forge.scratch.root();
    let before = state(&root);
    forge.flag("stall", true);
    let since = Instant::now();
    let run = forge.merge_waiting(1500);
    assert!(since.elapsed() < Duration::from_secs(10));
    assert_eq!(run.code, 3, "{}", run.stderr);
    assert!(
        run.stdout.contains("the forge did not answer, as gh failed: gh api repos/owner/repo/actions/workflows/ci.yml/runs did not answer in time."),
        "{}",
        run.stdout
    );
    let stalled = fs::read_to_string(forge.gh.join("stalled")).unwrap();
    let proc = PathBuf::from(format!("/proc/{}", stalled.trim()));
    assert!(!proc.exists(), "the stalled gh is still running");
    assert_eq!(state(&root), before);
}

#[test]
fn a_merge_killed_during_the_wait_leaves_everything_as_it_was() {
    let forge = Forge::new(Some("ci.yml"));
    let root = forge.scratch.root();
    forge.ci(ID, &[(&head(&root, ID), "in_progress", None)]);
    let (before, branch, base) = (state(&root), forge.remote(ID), forge.remote("main"));
    let mut child = forge.waiting(&root, ID);
    child.kill().unwrap();
    child.wait().unwrap();
    assert_eq!(state(&root), before);
    assert_eq!((forge.remote(ID), forge.remote("main")), (branch, base));
    assert_eq!(git(&root, &["worktree", "list"]).lines().count(), 1);
}

#[test]
fn another_items_merge_completes_while_one_waits() {
    const OTHER: &str = "demo-cd34";
    let forge = Forge::set_up(Some("ci.yml"), "", &[ID, OTHER]);
    let root = forge.scratch.root();
    let other = forge.scratch.worktree("other");
    for (at, id) in [(&root, ID), (&other, OTHER)] {
        forge.start(at, id);
        assert_eq!(forge.submit(at, id).code, 0);
        forge.approve(at, id);
    }
    forge.ci(ID, &[(&head(&root, ID), "in_progress", None)]);
    forge.ci(
        OTHER,
        &[(&head(&root, OTHER), "completed", Some("success"))],
    );
    let waiting = forge.waiting(&root, ID);
    let second = forge.atelier(&other, "t1", &["merge"]);
    assert_eq!(second.code, 0, "{}", second.stderr);
    assert_eq!(head(&root, "main^2"), head(&root, OTHER));
    let first = head(&root, "main");

    // Once its CI passes, the first merges onto the base the second left.
    forge.ci(ID, &[(&head(&root, ID), "completed", Some("success"))]);
    let run = run(waiting.wait_with_output().unwrap());
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(head(&root, "main^1"), first);
    assert_eq!(head(&root, "main^2"), head(&root, ID));
}

/// Starts `merge` waiting for a run in progress on the head, changes the
/// clone with `change`, then lets the run pass, and returns what `merge`
/// said.
fn changed_during_the_wait(forge: &Forge, change: impl Fn(&Path)) -> Run {
    let root = forge.scratch.root();
    let approved = head(&root, ID);
    forge.ci(ID, &[(&approved, "in_progress", None)]);
    let waiting = forge.waiting(&root, ID);
    change(&root);
    forge.ci(ID, &[(&approved, "completed", Some("success"))]);
    run(waiting.wait_with_output().unwrap())
}

#[test]
fn a_base_that_gains_a_commit_during_the_wait_is_merged_onto() {
    let forge = Forge::new(Some("ci.yml"));
    let base = forge.scratch.worktree("base");
    let run = changed_during_the_wait(&forge, |_| {
        git(&base, &["switch", "-q", "main"]);
        fs::write(base.join("other.txt"), "other\n").unwrap();
        git(&base, &["add", "other.txt"]);
        git(&base, &["commit", "-qm", "Meanwhile"]);
    });
    let root = forge.scratch.root();
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(head(&root, "main^1"), head(&base, "HEAD^"));
    assert_eq!(
        git(&root, &["show", "main^1", "--format=%s", "-s"]),
        "Meanwhile"
    );
    assert_eq!(forge.remote("main"), Some(head(&root, "main")));
}

#[test]
fn a_merge_check_the_base_gains_during_the_wait_is_run() {
    let forge = Forge::new(Some("ci.yml"));
    let base = forge.scratch.worktree("base");
    let run = changed_during_the_wait(&forge, |_| {
        git(&base, &["switch", "-q", "main"]);
        append(
            &base,
            ".atelier/config.toml",
            "\n[[merge_check]]\nrun = \"echo broken; false\"\n",
        );
        git(&base, &["commit", "-qam", "Check"]);
    });
    let root = forge.scratch.root();
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(
        run.stderr.starts_with(&format!(
            "`atelier merge` refused: the merge check `echo broken; false` failed (exit 1) on {ID} merged into main. Nothing changed.\nThe end of its output:\n  broken\n"
        )),
        "{}",
        run.stderr
    );
    assert_eq!(field(&root, "main", "status"), "ready");
}

#[test]
fn a_branch_that_gains_a_commit_outside_the_item_during_the_wait_is_stale() {
    let forge = Forge::new(Some("ci.yml"));
    let run = changed_during_the_wait(&forge, |root| {
        fs::write(root.join("other.txt"), "unreviewed\n").unwrap();
        git(root, &["add", "other.txt"]);
        git(root, &["commit", "-qm", "Unreviewed"]);
    });
    let root = forge.scratch.root();
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(
        run.stderr.contains("(changed: other.txt)")
            && run.stderr.ends_with("Then: atelier submit\n"),
        "{}",
        run.stderr
    );
    assert_eq!(field(&root, "main", "status"), "ready");
}

#[test]
fn with_no_ci_workflow_submit_pushes_and_opens_and_merge_waits_for_nothing() {
    let forge = Forge::new(None);
    let root = forge.scratch.root();
    let submitted = forge.submitted();
    assert_eq!(forge.remote(ID), Some(submitted.clone()));
    assert_eq!(forge.opened(), 1);
    let run = forge.merge();
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(run.stdout, format!("{PULL}{}", merged_and_pushed(&root)));
    assert_eq!(forge.polls(ID), 0);
    assert_eq!(forge.remote(ID), Some(submitted));
    assert_eq!(forge.remote("main"), Some(head(&root, "main")));
    assert_eq!(field(&root, "main", "status"), "done");
}

#[test]
fn a_merge_whose_base_push_failed_pushes_it_when_run_again() {
    let forge = Forge::new(None);
    let root = forge.scratch.root();
    // The forge refuses every push of main until the hook goes.
    let hook = forge.remote.join("hooks/pre-receive");
    fs::write(
        &hook,
        "#!/bin/sh\nwhile read old new ref; do [ \"$ref\" = refs/heads/main ] && exit 1; done\nexit 0\n",
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
    let base = forge.remote("main");

    let run = forge.merge();
    assert_eq!(run.code, 1, "{}", run.stdout);
    let merged = head(&root, "main");
    assert!(
        run.stderr.contains(&format!(
            "Merged {ID} into main with merge commit {}, which marks it done.\nPushing main to origin failed: ",
            &merged[..7]
        )),
        "{}",
        run.stderr
    );
    assert!(run
        .stderr
        .ends_with("Then run `atelier merge` again on this branch to push it.\n"));
    assert_eq!(field(&root, "main", "status"), "done");
    assert_eq!(forge.remote("main"), base);

    fs::remove_file(&hook).unwrap();
    let run = forge.merge();
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        format!(
            "{ID} is already merged into main; nothing is left to do on it.\nPushed main to origin.\n"
        )
    );
    assert_eq!(forge.remote("main"), Some(merged));
}

#[test]
fn a_forge_merge_whose_branch_moves_during_its_checks_pushes_nothing() {
    let forge = Forge::with(Some("ci.yml"), &waiting_check());
    let root = forge.scratch.root();
    let tip = head(&root, "main");
    let (branch, base) = (forge.remote(ID), forge.remote("main"));
    forge.ci(ID, &[(&head(&root, ID), "completed", Some("success"))]);
    let command = forge.command(&root, "t1", &["merge"]);
    let waiting = Waiting::spawn(command, forge.scratch.dir.path(), "merge");
    append(&root, &item_path(ID), "\nMore report.\n");
    git(&root, &["commit", "-qam", "Meanwhile"]);
    let run = waiting.finish();
    assert!(run.stdout.starts_with(PULL), "{}", run.stdout);
    wrote_nothing(&forge.scratch, run, &format!("branch {ID}"), &tip);
    assert_eq!((forge.remote(ID), forge.remote("main")), (branch, base));
    assert_eq!(forge.opened(), 1);
}

#[test]
fn an_untracked_file_in_the_items_directory_stops_the_merge() {
    // A review file nobody committed still means the item was reviewed.
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    let path = root.join(item_path(ID));
    let text = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, text + "It prints hello.\n").unwrap();
    git(&root, &["commit", "-qam", "Report"]);
    fs::create_dir_all(root.join(REVIEWS)).unwrap();
    fs::write(root.join(REVIEWS).join("1-aaaa.md"), UNDECIDED).unwrap();
    let before = state(&root);
    let refused = |run: Run| {
        assert_eq!(run.code, 1);
        let named = format!("(these are not: {REVIEWS}/1-aaaa.md)\n");
        assert!(run.stderr.contains(&named), "{}", run.stderr);
    };
    refused(merge(&root, "t1", &["--trivial", "a typo"]));
    assert_eq!(state(&root), before);
    // Ignored, it still counts.
    let exclude = git(&root, &["rev-parse", "--git-path", "info/exclude"]);
    fs::write(root.join(exclude), format!("{REVIEWS}/\n")).unwrap();
    refused(merge(&root, "t1", &["--trivial", "a typo"]));
    assert_eq!(field(&root, "main", "status"), "ready");
}

#[test]
fn an_ignored_file_in_the_base_checkout_is_not_overwritten() {
    let scratch = approved();
    let root = scratch.root();
    let base = scratch.worktree("base");
    git(&base, &["switch", "-q", "main"]);
    let tip = head(&root, "main");
    // The base's checkout holds an ignored hello.txt, which the branch adds.
    let exclude = git(&root, &["rev-parse", "--git-common-dir"]);
    fs::write(root.join(exclude).join("info/exclude"), "hello.txt\n").unwrap();
    fs::write(base.join("hello.txt"), "local\n").unwrap();
    let run = merge(&root, "t1", &[]);
    assert_eq!(run.code, 2, "{}", run.stdout);
    assert!(
        run.stderr
            .starts_with("atelier: Moving main to the merge commit failed, so main is as it was."),
        "{}",
        run.stderr
    );
    assert_eq!(read(&base, "hello.txt"), "local\n");
    assert_eq!(head(&root, "main"), tip);
    assert_eq!(head(&base, "HEAD"), tip);
}
