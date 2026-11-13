//! The branch model: `start` makes every kind of branch, names it by the
//! convention and its configured prefix, and claims it by pushing it to the
//! remote; a backlog branch merges its record changes on `atelier check`;
//! and a started item is dropped on its own branch. Driven in scratch
//! repositories, several clones of one bare remote where a claim or a drop
//! crosses clones.

mod common;

use common::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// The URL the clones' `origin` names, which `insteadOf` sends to the bare
/// repository, so the forge reads a GitHub repository from it.
const URL: &str = "git@github.com:owner/repo.git";

/// Clones of one bare remote, whose base config names it as the forge's
/// remote, with a fake `gh` that every command finds first on `PATH`.
struct Hub {
    dir: TempDir,
    remote: PathBuf,
    gh: PathBuf,
}

impl Hub {
    /// A remote whose `main` holds `items`, each an ID with its status.
    fn new(items: &[(&str, &str)]) -> Hub {
        let dir = TempDir::new().unwrap();
        let seed = dir.path().join("seed");
        git(&seed, &["init", "-q", "-b", "main"]);
        fs::create_dir_all(seed.join(".atelier")).unwrap();
        fs::write(
            seed.join(".atelier/config.toml"),
            "schema_version = 3\nslug = \"demo\"\nbase_branch = \"main\"\n\n\
             [forge]\nprovider = \"github\"\nremote = \"origin\"\n",
        )
        .unwrap();
        for (id, status) in items {
            write_item(&seed, id, status, &[], &[], "");
        }
        git(&seed, &["add", "-A"]);
        git(&seed, &["commit", "-qm", "Plan"]);
        let remote = dir.path().join("remote.git");
        git(
            dir.path(),
            &[
                "init",
                "-q",
                "--bare",
                "-b",
                "main",
                remote.to_str().unwrap(),
            ],
        );
        git(&seed, &["push", "-q", remote.to_str().unwrap(), "main"]);
        let gh = dir.path().join("gh");
        fs::create_dir_all(gh.join("bin")).unwrap();
        fs::write(gh.join("bin/gh"), FAKE_GH).unwrap();
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(gh.join("bin/gh"), fs::Permissions::from_mode(0o755)).unwrap();
        Hub { dir, remote, gh }
    }

    /// A clone named `name`, detached at the remote's `main`.
    fn clone(&self, name: &str) -> PathBuf {
        let at = self.dir.path().join(name);
        let from = self.remote.to_str().unwrap();
        git(
            self.dir.path(),
            &["clone", "-q", from, at.to_str().unwrap()],
        );
        git(&at, &["remote", "set-url", "origin", URL]);
        git(&at, &["config", &format!("url.{from}.insteadOf"), URL]);
        git(&at, &["checkout", "-q", "--detach", "origin/main"]);
        git(&at, &["branch", "-qD", "main"]);
        git(&at, &["branch", "-q", "--track", "main", "origin/main"]);
        at
    }

    fn command(&self, at: &Path, thread: &str, args: &[&str]) -> Command {
        let path = format!(
            "{}:{}",
            self.gh.join("bin").display(),
            std::env::var("PATH").unwrap()
        );
        let mut command = atelier_command(at, thread, args);
        command.env("PATH", path).env("FAKE_GH", &self.gh);
        command
    }

    fn atelier(&self, at: &Path, thread: &str, args: &[&str]) -> Run {
        let output = self.command(at, thread, args).output().unwrap();
        Run {
            code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8(output.stdout).unwrap(),
            stderr: String::from_utf8(output.stderr).unwrap(),
        }
    }

    /// The remote's copy of `branch`, if it has one.
    fn tip(&self, branch: &str) -> Option<String> {
        let output = git_output(
            &self.remote,
            &[
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

    /// The item `ID`'s front-matter field `key` on the remote's `branch`.
    fn field(&self, branch: &str, key: &str) -> String {
        front(
            &git(
                &self.remote,
                &["show", &format!("{branch}:{}", item_path(ID))],
            ),
            key,
        )
    }

    /// `id` started by `thread` in `at`, with a change committed, submitted,
    /// and approved by `r1`, whose commit is in `at`.
    fn approved(&self, at: &Path, thread: &str) {
        ok(self.atelier(at, thread, &["start", ID]));
        fs::write(at.join("hello.txt"), "hello\n").unwrap();
        git(at, &["add", "hello.txt"]);
        git(at, &["commit", "-qm", "Say hello"]);
        let path = at.join(item_path(ID));
        let ticked = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
        fs::write(&path, ticked + "It prints hello.\n").unwrap();
        ok(self.atelier(at, thread, &["submit"]));
        ok(self.atelier(at, "r1", &["review", ID]));
        let run = self.atelier(at, "r1", &["verdict", "approve"]);
        ok(run.clone());
        run_then(at, &run.stdout);
    }
}

fn ok(run: Run) {
    assert_eq!(run.code, 0, "{}{}", run.stdout, run.stderr);
}

fn head(at: &Path) -> String {
    git(at, &["rev-parse", "HEAD"])
}

/// `start` of `ID` as `thread` in `at`, held before it makes its branch
/// until `release` exists.
fn held_start(hub: &Hub, at: &Path, thread: &str, release: &Path) -> Child {
    hub.command(at, thread, &["start", ID])
        .env("ATELIER_TEST_HOLD_BEFORE_BRANCH", release)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

fn finished(child: Child) -> Run {
    let output = child.wait_with_output().unwrap();
    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

#[test]
fn one_item_started_from_two_clones_at_once_has_one_holder_and_the_other_is_told_who() {
    let hub = Hub::new(&[(ID, "ready")]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    let (before_a, before_b) = (head(&a), head(&b));
    // Both are held after their commit and before their branch, then let
    // go together, so each makes its local branch and pushes it.
    let release = hub.dir.path().join("release");
    let (hold_a, hold_b) = (release.with_extension("a"), release.with_extension("b"));
    let first = held_start(&hub, &a, "t1", &hold_a);
    let second = held_start(&hub, &b, "t2", &hold_b);
    let since = Instant::now();
    while !(hold_a.with_extension("held").exists() && hold_b.with_extension("held").exists()) {
        assert!(since.elapsed() < Duration::from_secs(20), "never held");
        std::thread::sleep(Duration::from_millis(20));
    }
    fs::write(&hold_a, "").unwrap();
    fs::write(&hold_b, "").unwrap();
    let runs = [finished(first), finished(second)];

    let won: Vec<usize> = (0..2).filter(|i| runs[*i].code == 0).collect();
    assert_eq!(
        won.len(),
        1,
        "{:?}",
        runs.iter().map(|r| &r.stderr).collect::<Vec<_>>()
    );
    let (winner, loser) = match won[0] {
        0 => (("t1", &a, before_a), ("t2", &b, before_b)),
        _ => (("t2", &b, before_b), ("t1", &a, before_a)),
    };
    let lost = &runs[1 - won[0]];
    assert_eq!(lost.code, 1, "{}", lost.stderr);
    assert!(
        lost.stderr.starts_with(&format!(
            "`atelier start` refused; nothing changed here.\n{} holds {ID}: origin already has branch {ID}, so this clone's claim lost, and the branch it made is removed. It fetched origin/{ID}.\nTo take it over, run `atelier start {ID}` again.\n",
            winner.0
        )),
        "{}",
        lost.stderr
    );
    // The winner's claim is the remote's branch, and it is on it.
    assert_eq!(hub.field(ID, "implementers"), winner.0);
    assert_eq!(git(winner.1, &["symbolic-ref", "--short", "HEAD"]), ID);
    // The loser keeps no branch and its checkout is as it was.
    assert!(!branch_exists(loser.1, ID));
    assert_eq!(head(loser.1), loser.2);
    assert_eq!(git(loser.1, &["rev-parse", "--abbrev-ref", "HEAD"]), "HEAD");
    assert_eq!(git(loser.1, &["status", "--porcelain"]), "");

    // After a fetch, the loser's clone shows the item held by the winner.
    git(loser.1, &["fetch", "-q", "origin"]);
    let listed = hub.atelier(loser.1, loser.0, &["list"]);
    ok(listed.clone());
    assert!(
        listed.stdout.contains(&format!(
            "{ID} (building, high, held by {}): Say hello\n",
            winner.0
        )),
        "{}",
        listed.stdout
    );

    // A later start there takes it over, from the remote's branch.
    let taken = hub.atelier(loser.1, loser.0, &["start", ID]);
    ok(taken.clone());
    assert!(
        taken
            .stdout
            .starts_with(&format!("Took over {ID}; {} held it.\n", winner.0)),
        "{}",
        taken.stdout
    );
    assert_eq!(git(loser.1, &["symbolic-ref", "--short", "HEAD"]), ID);
    assert_eq!(
        hub.field(ID, "implementers"),
        format!("{} {}", winner.0, loser.0)
    );
}

#[test]
fn start_is_refused_when_the_remote_cannot_be_reached_and_keeps_no_branch() {
    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    let gone = hub.dir.path().join("gone.git");
    let reachable = format!("url.{}.insteadOf", hub.remote.display());
    git(&a, &["config", "--unset", &reachable]);
    git(
        &a,
        &["config", &format!("url.{}.insteadOf", gone.display()), URL],
    );
    let before = head(&a);
    for args in [&["start", ID][..], &["start", "--backlog", "tidy"]] {
        let run = hub.atelier(&a, "t1", args);
        assert_eq!(run.code, 1, "{args:?}: {}", run.stdout);
        assert!(
            run.stderr
                .starts_with("`atelier start` refused; nothing changed.\norigin cannot be reached"),
            "{}",
            run.stderr
        );
        assert_eq!(head(&a), before);
        assert_eq!(
            git(
                &a,
                &["for-each-ref", "--format=%(refname:short)", "refs/heads/"]
            ),
            "main"
        );
    }
    assert_eq!(hub.tip(ID), None);
}

#[test]
fn start_pushes_each_kind_of_branch_before_it_reports_success() {
    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    ok(hub.atelier(&a, "t1", &["start", ID]));
    assert_eq!(hub.tip(ID), Some(head(&a)));
    git(&a, &["checkout", "-q", "--detach", "origin/main"]);
    ok(hub.atelier(&a, "p1", &["start", "--backlog", "tidy"]));
    assert_eq!(
        hub.tip("backlog/tidy"),
        Some(git(&a, &["rev-parse", "origin/main"]))
    );
    git(&a, &["checkout", "-q", "--detach", "origin/main"]);
    let run = hub.atelier(&a, "p1", &["start", "--mission", "Greet"]);
    ok(run.clone());
    let mission = run.stdout.split_whitespace().nth(3).unwrap().to_string();
    assert_eq!(hub.tip(&mission), Some(head(&a)));

    // A backlog branch the remote has already is refused, and nothing is
    // made.
    let b = hub.clone("b");
    let run = hub.atelier(&b, "p2", &["start", "--backlog", "tidy"]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier start --backlog` refused; nothing changed.\nBranch backlog/tidy exists on origin. Pick another slug, or work on that branch.\n{SKIP}"
        )
    );
    assert!(!branch_exists(&b, "backlog/tidy"));
}

#[test]
fn start_in_the_bases_own_checkout_makes_the_branch_and_leaves_the_checkout_on_the_base() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let base = scratch.worktree("base");
    git(&base, &["switch", "-q", "main"]);
    let before = head(&base);
    for (args, branch) in [
        (&["start", ID][..], ID.to_string()),
        (&["start", "--backlog", "tidy"], "backlog/tidy".to_string()),
        (&["start", "--mission", "Greet"], String::new()),
    ] {
        let run = atelier(&base, "t1", args);
        assert_eq!(run.code, 0, "{args:?}: {}", run.stderr);
        let branch = match branch.is_empty() {
            true => run.stdout.split_whitespace().nth(3).unwrap().to_string(),
            false => branch,
        };
        assert!(branch_exists(&base, &branch), "{branch}");
        assert_eq!(git(&base, &["symbolic-ref", "--short", "HEAD"]), "main");
        assert_eq!(head(&base), before);
        assert_eq!(git(&base, &["status", "--porcelain"]), "");
        let path = scratch
            .dir
            .path()
            .join(format!("base-{}", branch.replace('/', "-")));
        let line = format!("  git worktree add {} {branch}\n", path.display());
        assert!(run.stdout.contains(&line), "{}", run.stdout);
        // The printed line gives the branch a checkout of its own.
        git(
            &base,
            &["worktree", "add", "-q", path.to_str().unwrap(), &branch],
        );
        assert_eq!(git(&path, &["symbolic-ref", "--short", "HEAD"]), branch);
    }
    let work = scratch.dir.path().join(format!("base-{ID}"));
    let status = atelier(&work, "t1", &["status"]).stdout;
    assert!(
        status.contains(&format!("You are the implementer of {ID} (thread t1).\n")),
        "{status}"
    );
}

#[test]
fn a_branch_prefix_names_every_kind_of_branch_and_the_item_loop_runs_on_it() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    append(&root, ".atelier/config.toml", "branch_prefix = \"x/\"\n");
    on_base(&root, "Configure");

    let run = atelier(&root, "t1", &["start", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stdout
            .starts_with(&format!("Started {ID} on branch x/{ID}.\n")),
        "{}",
        run.stdout
    );
    assert_eq!(
        git(&root, &["symbolic-ref", "--short", "HEAD"]),
        format!("x/{ID}")
    );
    assert!(!branch_exists(&root, ID));
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    git(&root, &["add", "hello.txt"]);
    git(&root, &["commit", "-qm", "Say hello"]);
    finish_and_submit(&root, "t1");
    assert_eq!(atelier(&root, "t2", &["review", ID]).code, 0);
    assert_eq!(verdict(&root, "t2", &["verdict", "approve"]).code, 0);
    let merged = atelier(&root, "t1", &["merge"]);
    assert_eq!(merged.code, 0, "{}", merged.stderr);
    assert_eq!(field(&root, "main", "status"), "done");

    git(&root, &["checkout", "-q", "--detach", "main"]);
    let run = atelier(&root, "p1", &["start", "--backlog", "tidy"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        git(&root, &["symbolic-ref", "--short", "HEAD"]),
        "x/backlog/tidy"
    );
    let status = atelier(&root, "p1", &["status"]).stdout;
    assert!(
        status.contains("on branch x/backlog/tidy, a backlog branch for record changes.\n"),
        "{status}"
    );
    git(&root, &["checkout", "-q", "--detach", "main"]);
    let run = atelier(&root, "p1", &["start", "--mission", "Greet"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let mission = run.stdout.split_whitespace().nth(3).unwrap();
    assert_eq!(
        git(&root, &["symbolic-ref", "--short", "HEAD"]),
        format!("x/{mission}")
    );

    // Without the prefix, a branch named for an ID is not the item's.
    git(&root, &["checkout", "-q", "--detach", "main"]);
    git(&root, &["switch", "-q", "-c", "demo-zz11"]);
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(
        status.contains("on branch demo-zz11, which is no item's.\n"),
        "{status}"
    );
}

#[test]
fn a_backlog_branch_merges_its_record_changes_once_check_passes_with_no_review() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    let run = atelier(&root, "p1", &["start", "--backlog", "plan"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    // No commit: the branch is the base's tip.
    assert_eq!(head(&root), git(&root, &["rev-parse", "main"]));
    let ready = atelier(&root, "p1", &["ready", ID, "--by", "sumeet"]);
    assert_eq!(ready.code, 0, "{}", ready.stderr);

    // Submit refuses while a record it changes is not committed.
    let run = atelier(&root, "p1", &["submit"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.starts_with(&format!(
            "`atelier submit` refused; nothing changed.\nThese record changes on backlog/plan are not committed, and a submit publishes only what is:\n  {}\n",
            item_path(ID)
        )),
        "{}",
        run.stderr
    );
    run_then(&root, &ready.stdout);
    let run = atelier(&root, "p1", &["submit"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        "Submitted backlog/plan, which changes records alone.\nNo forge is configured, so nothing was pushed.\nIt merges once `atelier check` passes on the merged tree, with no review.\nThen: atelier merge\n"
    );

    let (base, branch) = (git(&root, &["rev-parse", "main"]), head(&root));
    let run = atelier(&root, "p1", &["merge"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let merged = git(&root, &["rev-parse", "main"]);
    assert_eq!(
        run.stdout,
        format!(
            "The branch changes records alone, so it merges with no review once its records are valid: `atelier check` passed on the merged tree of main.\nMerged backlog/plan into main with merge commit {}.\n",
            &merged[..7]
        )
    );
    assert_eq!(
        git(&root, &["rev-list", "--parents", "-n1", "main"]),
        format!("{merged} {base} {branch}")
    );
    assert!(
        git(&root, &["log", "-1", "--format=%B", "main"]).ends_with("Atelier-Merge: backlog/plan")
    );
    assert_eq!(field(&root, "main", "status"), "ready");
    // Run again, it has nothing to do.
    let again = atelier(&root, "p1", &["merge"]);
    assert_eq!(again.code, 0, "{}", again.stderr);
    assert_eq!(
        again.stdout,
        "backlog/plan is already merged into main; nothing is left to do on it.\n"
    );
}

#[test]
fn a_backlog_branch_with_code_a_new_mission_or_invalid_records_is_not_merged() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    let base = git(&root, &["rev-parse", "main"]);

    // A change under crates/ needs a review: refused, naming it, and no
    // skip passes it.
    assert_eq!(
        atelier(&root, "p1", &["start", "--backlog", "sneaky"]).code,
        0
    );
    fs::create_dir_all(root.join("crates")).unwrap();
    fs::write(root.join("crates/lib.rs"), "fn main() {}\n").unwrap();
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Sneak in code"]);
    let run = atelier(&root, "p1", &["merge"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.starts_with(
            "`atelier merge` refused; nothing changed.\nbacklog/sneaky merges with no review only while it changes records alone, under `.atelier/` and not its config.toml, and it changes these against main:\n  crates/lib.rs\n"
        ),
        "{}",
        run.stderr
    );
    let skipped = atelier_command(&root, "p1", &["merge"])
        .env("ATELIER_SKIP", "just this once")
        .output()
        .unwrap();
    assert_eq!(skipped.status.code(), Some(1));
    assert!(String::from_utf8(skipped.stderr)
        .unwrap()
        .ends_with(NOT_SKIPPABLE));
    assert_eq!(git(&root, &["rev-parse", "main"]), base);

    // A new mission waits for its plan reviews (atelier-2vdq).
    git(&root, &["checkout", "-q", "--detach", "main"]);
    assert_eq!(
        atelier(&root, "p1", &["start", "--backlog", "mission"]).code,
        0
    );
    let path = root.join(".atelier/missions/demo-m1ss/mission.md");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        &path,
        "---\nschema: atelier.mission\nschema_version: 3\nid: demo-m1ss\ntitle: Greet\n\
         status: draft\nlabels: []\nlead: null\nready: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Outcome\n\nGreetings.\n",
    )
    .unwrap();
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Plan a mission"]);
    let run = atelier(&root, "p1", &["merge"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr
            .contains("  adds the mission, of mission demo-m1ss\n"),
        "{}",
        run.stderr
    );
    assert_eq!(git(&root, &["rev-parse", "main"]), base);

    // Records that do not validate on the merged tree are refused.
    git(&root, &["checkout", "-q", "--detach", "main"]);
    assert_eq!(
        atelier(&root, "p1", &["start", "--backlog", "broken"]).code,
        0
    );
    write_item(&root, ID, "draft", &[], &["demo-zz99"], "");
    git(&root, &["commit", "-qam", "Block on nothing"]);
    let run = atelier(&root, "p1", &["merge"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.starts_with(
            "`atelier merge` refused; nothing changed.\nMerged into main, backlog/broken leaves records that are not valid:\n"
        ) && run.stderr.contains("`demo-zz99` names no item"),
        "{}",
        run.stderr
    );
    assert_eq!(git(&root, &["rev-parse", "main"]), base);
}

#[test]
fn a_drop_pushed_on_the_items_branch_stops_its_merge_in_another_clone_and_then_reaches_the_base() {
    let hub = Hub::new(&[(ID, "ready")]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    hub.approved(&a, "t1");
    git(&a, &["push", "-q", "origin", ID]);

    // Another host drops it, in a checkout of its branch, and pushes it.
    git(&b, &["fetch", "-q", "origin"]);
    git(&b, &["switch", "-q", ID]);
    let reason = "The site greets instead";
    let run = hub.atelier(&b, "p1", &["drop", ID, "--reason", reason]);
    ok(run.clone());
    run_then(&b, &run.stdout);
    git(&b, &["push", "-q", "origin", ID]);
    assert_eq!(hub.field(ID, "status"), "dropped");

    // Once fetched, the holder's submit and merge are refused, and nothing
    // moves.
    git(&a, &["fetch", "-q", "origin"]);
    let (before, base) = (head(&a), hub.tip("main"));
    for command in ["submit", "merge"] {
        let run = hub.atelier(&a, "t1", &[command]);
        assert_eq!(run.code, 1, "{command}: {}", run.stdout);
        assert!(
            run.stderr
                .starts_with(&format!("{ID} is dropped, and `atelier {command}` runs on")),
            "{command}: {}",
            run.stderr
        );
        assert_eq!(head(&a), before);
        assert_eq!(hub.tip("main"), base);
    }

    // The base's copy follows on a backlog branch, then the item's branch
    // can go, and the base says dropped.
    git(&b, &["checkout", "-q", "--detach", "origin/main"]);
    ok(hub.atelier(&b, "p1", &["start", "--backlog", &format!("drop-{ID}")]));
    let run = hub.atelier(&b, "p1", &["drop", ID, "--reason", reason]);
    ok(run.clone());
    run_then(&b, &run.stdout);
    ok(hub.atelier(&b, "p1", &["submit"]));
    ok(hub.atelier(&b, "p1", &["merge"]));
    assert_eq!(hub.field("main", "status"), "dropped");
    git(&a, &["checkout", "-q", "--detach", "origin/main"]);
    git(&a, &["branch", "-qD", ID]);
    git(&b, &["push", "-q", "origin", "--delete", ID]);
    let fresh = hub.clone("fresh");
    let shown = hub.atelier(&fresh, "t9", &["show", ID]);
    ok(shown.clone());
    assert!(
        shown.stdout.contains("Status: dropped\n"),
        "{}",
        shown.stdout
    );
    assert!(
        shown.stdout.contains(&format!("Dropped: {reason}\n")),
        "{}",
        shown.stdout
    );
}

#[test]
fn a_drop_after_the_item_merged_in_another_clone_is_refused() {
    let hub = Hub::new(&[(ID, "ready")]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    hub.approved(&a, "t1");
    ok(hub.atelier(&a, "t1", &["merge"]));
    assert_eq!(hub.field("main", "status"), "done");

    // The other clone has fetched, and its own base is behind.
    git(&b, &["fetch", "-q", "origin"]);
    git(&b, &["switch", "-q", ID]);
    let before = read(&b, &item_path(ID));
    let run = hub.atelier(&b, "p1", &["drop", ID, "--reason", "Too late"]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(
        run.stderr.starts_with(&format!(
            "`atelier drop {ID}` refused; nothing written.\n{ID} is merged: its branch is in the history of the base, here or on the forge"
        )),
        "{}",
        run.stderr
    );
    assert_eq!(read(&b, &item_path(ID)), before);
    assert_eq!(git(&b, &["status", "--porcelain"]), "");
    assert_eq!(hub.field("main", "status"), "done");
}

/// What `git status` lists as changed, and that `HEAD` is still `head`.
fn changed_since(at: &Path, head: &str) -> String {
    assert_eq!(git(at, &["rev-parse", "HEAD"]), head, "a command committed");
    let listed = git_output(at, &["status", "--porcelain", "-uall"]);
    String::from_utf8(listed.stdout).unwrap()
}

#[test]
fn every_record_command_writes_its_files_prints_them_and_makes_no_commit_branch_or_push() {
    let hub = Hub::new(&[(ID, "draft"), ("demo-c10s", "ready")]);
    let a = hub.clone("a");
    // A ready mission to lead, on the base.
    let path = a.join(".atelier/missions/demo-m1ss/mission.md");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        &path,
        "---\nschema: atelier.mission\nschema_version: 3\nid: demo-m1ss\ntitle: Greet\n\
         status: ready\nlabels: []\nlead: null\nready: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Outcome\n\nGreetings.\n",
    )
    .unwrap();
    git(&a, &["add", "-A"]);
    git(&a, &["commit", "-qm", "Plan a mission"]);
    git(&a, &["push", "-q", "origin", "HEAD:main"]);
    git(&a, &["branch", "-f", "main", "HEAD"]);
    ok(hub.atelier(&a, "p1", &["start", "--backlog", "plan"]));
    let refs = || {
        let local = git(
            &a,
            &[
                "for-each-ref",
                "--format=%(refname:short) %(objectname)",
                "refs/heads/",
            ],
        );
        let remote = git(
            &hub.remote,
            &[
                "for-each-ref",
                "--format=%(refname:short) %(objectname)",
                "refs/heads/",
            ],
        );
        (local, remote)
    };
    let before = refs();
    let head = git(&a, &["rev-parse", "HEAD"]);
    for (args, wrote) in [
        (&["new", "Say goodbye"][..], ".atelier/items/"),
        (
            &["set", ID, "--priority", "low"],
            ".atelier/items/demo-ab12/item.md",
        ),
        (
            &["ready", ID, "--by", "sumeet"],
            ".atelier/items/demo-ab12/item.md",
        ),
        (
            &["drop", ID, "--reason", "Not needed"],
            ".atelier/items/demo-ab12/item.md",
        ),
        (
            &["close", "demo-c10s", "Done by hand"],
            ".atelier/items/demo-c10s/item.md",
        ),
        (
            &["lead", "demo-m1ss"],
            ".atelier/missions/demo-m1ss/mission.md",
        ),
    ] {
        let run = hub.atelier(&a, "p1", args);
        assert_eq!(run.code, 0, "{args:?}: {}", run.stderr);
        assert!(
            run.stdout
                .contains(&format!("Wrote, and did not commit:\n  {wrote}")),
            "{args:?}: {}",
            run.stdout
        );
        assert!(
            run.stdout.contains("\nThen: git add -- "),
            "{args:?}: {}",
            run.stdout
        );
        let status = changed_since(&a, &head);
        assert!(status.contains(wrote), "{args:?}: {status}");
        assert_eq!(refs(), before, "{args:?} made or pushed a branch");
    }

    // An item's review and verdict, on its branch, the same way.
    let c = hub.clone("c");
    write_item(&c, "demo-r3vw", "ready", &[], &[], "");
    git(&c, &["add", "-A"]);
    git(&c, &["commit", "-qm", "Plan"]);
    git(&c, &["push", "-q", "origin", "HEAD:main"]);
    git(&c, &["branch", "-f", "main", "HEAD"]);
    let path = c.join(item_path("demo-r3vw"));
    ok(hub.atelier(&c, "t1", &["start", "demo-r3vw"]));
    let ticked = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, ticked + "It prints hello.\n").unwrap();
    ok(hub.atelier(&c, "t1", &["submit"]));
    let head = git(&c, &["rev-parse", "HEAD"]);
    let refs = || {
        git(
            &hub.remote,
            &[
                "for-each-ref",
                "--format=%(refname:short) %(objectname)",
                "refs/heads/",
            ],
        )
    };
    let before = refs();
    let run = hub.atelier(&c, "r1", &["review", "demo-r3vw"]);
    ok(run.clone());
    assert!(
        run.stdout
            .contains("Your review file: .atelier/items/demo-r3vw/reviews/1-"),
        "{}",
        run.stdout
    );
    let status = changed_since(&c, &head);
    assert!(
        status.contains("?? .atelier/items/demo-r3vw/reviews/1-"),
        "{status}"
    );
    let run = hub.atelier(&c, "r1", &["verdict", "approve"]);
    ok(run.clone());
    assert!(run.stdout.contains("Wrote, and did not commit:\n  .atelier/items/demo-r3vw/item.md\n  .atelier/items/demo-r3vw/reviews/1-"), "{}", run.stdout);
    let status = changed_since(&c, &head);
    assert!(
        status.contains(" M .atelier/items/demo-r3vw/item.md"),
        "{status}"
    );
    assert_eq!(refs(), before);
}
