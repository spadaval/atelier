//! `merge` driven in scratch repositories: the merge commit, what stops
//! it, the inputs that move while its checks run, and the forge, reached
//! through a fake `gh` and a local bare repository.

mod common;

use common::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Stdio};
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
        let mark = scratch.join(format!("{name}.mark"));
        let release = scratch.join(format!("{name}.release"));
        let child = atelier_command(at, thread, &["merge"])
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
fn uncommitted_changes_in_the_checkout_stop_the_merge() {
    let scratch = approved();
    let root = scratch.root();
    fs::write(root.join("hello.txt"), "hello?\n").unwrap();
    let before = state(&root);
    let run = merge(&root, "t1", &[]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.contains(
            "  [ ] No tracked file has uncommitted changes (these do: hello.txt)\nCommit them, or set them aside with `git stash`.\n"
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

/// A fake `gh` that answers from files in `$FAKE_GH`: `runs/<sha>` for a
/// commit's pull request runs and `pulls` for the open pull requests. A
/// POST to the pull requests opens #7. Every call is logged to `calls`.
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
    head_sha=*) answer="runs/${arg#head_sha=}" ;;
    */pulls) answer=pulls ;;
  esac
done
if [ "$answer" = pulls ] && [ "$method" = POST ]; then
  echo '[{"number":7,"html_url":"https://github.com/owner/repo/pull/7"}]' > "$FAKE_GH/pulls"
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

/// An approved item in a repository whose base config names a forge,
/// with `ci_workflow` when given. Its `origin` is a GitHub URL, and pushes
/// to it go to a local bare repository.
struct Forge {
    scratch: Scratch,
    gh: PathBuf,
    remote: PathBuf,
}

impl Forge {
    fn new(ci_workflow: Option<&str>) -> Forge {
        let workflow = ci_workflow.map_or(String::new(), |w| format!("ci_workflow = \"{w}\"\n"));
        let config = format!("\n[forge]\nprovider = \"github\"\nremote = \"origin\"\n{workflow}");
        let scratch = approved_with(&config, |_| {});
        let root = scratch.root();
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

    fn merge(&self) -> Run {
        let root = self.scratch.root();
        let path = format!(
            "{}:{}",
            self.gh.join("bin").display(),
            std::env::var("PATH").unwrap()
        );
        let output = atelier_command(&root, "t1", &["merge"])
            .env("PATH", path)
            .env("FAKE_GH", &self.gh)
            .output()
            .unwrap();
        run(output)
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

    /// Answers the runs lookup of `commit` with one run in `status`.
    fn ci(&self, commit: &str, status: &str, conclusion: Option<&str>) {
        let conclusion = conclusion.map_or("null".to_string(), |c| format!("\"{c}\""));
        fs::write(
            self.gh.join("runs").join(commit),
            format!(
                "{{\"total_count\":1,\"workflow_runs\":[{{\"id\":11,\"status\":\"{status}\",\"conclusion\":{conclusion},\
                 \"html_url\":\"https://github.com/owner/repo/actions/runs/11\",\"created_at\":\"2026-11-10T10:00:00Z\"}}]}}"
            ),
        )
        .unwrap();
    }

    fn opened(&self) -> usize {
        fs::read_to_string(self.gh.join("calls"))
            .unwrap_or_default()
            .lines()
            .filter(|call| call.contains("-X POST"))
            .count()
    }
}

#[test]
fn with_a_ci_workflow_merge_stops_until_ci_passes_on_the_head() {
    let forge = Forge::new(Some("ci.yml"));
    let root = forge.scratch.root();
    let (base, branch) = (head(&root, "main"), head(&root, ID));
    let before = state(&root);
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(
        status.ends_with(
            "It pushes the branch to origin and opens its pull request, and merges once ci.yml passes on the branch's head.\nThen: atelier merge\n"
        ),
        "{status}"
    );

    // The first run pushes the branch and opens its pull request, and CI
    // has not started.
    let pushed = format!(
        "Pushed branch {ID} to origin. Pull request #7: https://github.com/owner/repo/pull/7\n"
    );
    let run = forge.merge();
    assert_eq!(run.code, 3, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        format!(
            "{pushed}CI has not finished on {}, the branch's head: no run of it has started. Nothing else changed.\nRun `atelier merge` again once ci.yml finishes.\n",
            &branch[..7]
        )
    );
    assert_eq!(forge.remote(ID), Some(branch.clone()));
    assert_eq!(forge.remote("main"), Some(base.clone()));
    assert_eq!(forge.opened(), 1);
    assert_eq!(state(&root), before);

    // Running, it is still not yet, and the pull request is not opened again.
    forge.ci(&branch, "in_progress", None);
    let run = forge.merge();
    assert_eq!(run.code, 3);
    assert!(run.stdout.contains(
        "the branch's head: it is in progress (https://github.com/owner/repo/actions/runs/11)."
    ));
    assert_eq!(forge.opened(), 1);
    assert_eq!(state(&root), before);

    // Failed, it refuses and says so.
    forge.ci(&branch, "completed", Some("failure"));
    let run = forge.merge();
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.starts_with(&format!(
            "{pushed}`atelier merge` refused: ci.yml finished failure on {}, the branch's head (https://github.com/owner/repo/actions/runs/11). Nothing was merged.\n",
            &branch[..7]
        )),
        "{}",
        run.stderr
    );
    assert_eq!(state(&root), before);
    assert_eq!(forge.remote("main"), Some(base.clone()));

    // Passed, it merges and pushes the base.
    forge.ci(&branch, "completed", Some("success"));
    let run = forge.merge();
    assert_eq!(run.code, 0, "{}", run.stderr);
    let merged = head(&root, "main");
    assert_eq!(
        run.stdout,
        format!(
            "{pushed}Merged {ID} into main with merge commit {}, which marks it done.\nPushed main to origin.\n",
            &merged[..7]
        )
    );
    assert_eq!(head(&root, "main^1"), base);
    assert_eq!(forge.remote("main"), Some(merged));
}

#[test]
fn with_no_ci_workflow_one_run_pushes_opens_merges_and_pushes_the_base() {
    let forge = Forge::new(None);
    let root = forge.scratch.root();
    let branch = head(&root, ID);
    let run = forge.merge();
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(forge.remote(ID), Some(branch));
    assert_eq!(forge.opened(), 1);
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
