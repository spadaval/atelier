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
            "{ID} (building, high, held by {}, on origin/{ID}): Say hello\n",
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
         status: open\nlabels: []\nlead: null\nplanners:\n- p1\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Outcome\n\nGreetings.\n",
    )
    .unwrap();
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Plan a mission"]);
    let run = atelier(&root, "p1", &["merge"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.contains(
            "  Mission demo-m1ss: backlog/mission adds the mission.\n    Its plan has no review.\n    0 of 2 count.\n"
        ),
        "{}",
        run.stderr
    );
    assert_eq!(git(&root, &["rev-parse", "main"]), base);

    // A started item's record reaches the base only in its own merge.
    git(&root, &["checkout", "-q", "--detach", "main"]);
    assert_eq!(
        atelier(&root, "p1", &["start", "--backlog", "start-it"]).code,
        0
    );
    write_item(&root, ID, "building", &["t1"], &[], "");
    git(&root, &["commit", "-qam", "Start by hand"]);
    let run = atelier(&root, "p1", &["merge"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.contains(&format!(
            "brings these to the base started:\n  {ID}: building\n"
        )),
        "{}",
        run.stderr
    );
    assert!(run.stderr.ends_with(SKIP), "{}", run.stderr);
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

#[test]
fn a_take_over_of_a_branch_another_worktree_has_out_moves_nothing() {
    // From the base's own checkout, with no remote.
    let scratch = Scratch::new(&[(ID, "ready")]);
    let base = scratch.worktree("base");
    git(&base, &["switch", "-q", "main"]);
    assert_eq!(atelier(&base, "t1", &["start", ID]).code, 0);
    let work = scratch.dir.path().join("work");
    git(
        &base,
        &["worktree", "add", "-q", work.to_str().unwrap(), ID],
    );
    let tip = git(&base, &["rev-parse", ID]);
    let run = atelier(&base, "t2", &["start", ID]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert_eq!(
        run.stderr,
        format!(
            "`atelier start {ID}` refused; nothing changed.\nBranch {ID} is checked out in {}, and a branch can be checked out in one worktree at a time. Run `atelier start {ID}` there.\n",
            work.display()
        )
    );
    assert_eq!(git(&base, &["rev-parse", ID]), tip);
    assert_eq!(git(&work, &["status", "--porcelain"]), "");

    // With the forge's copy ahead of a local branch another worktree has
    // out, the local branch is not moved under it either.
    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    ok(hub.atelier(&a, "t1", &["start", ID]));
    let b = hub.clone("b");
    ok(hub.atelier(&b, "t2", &["start", ID]));
    let held = git(&a, &["rev-parse", ID]);
    git(&a, &["fetch", "-q", "origin"]);
    let other = hub.dir.path().join("a-work");
    git(&a, &["checkout", "-q", "--detach", "origin/main"]);
    git(&a, &["worktree", "add", "-q", other.to_str().unwrap(), ID]);
    let run = hub.atelier(&a, "t3", &["start", ID]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(run.stderr.contains("is checked out in"), "{}", run.stderr);
    assert_eq!(git(&a, &["rev-parse", ID]), held);
    assert_eq!(git(&other, &["status", "--porcelain"]), "");
}

#[test]
fn of_two_claims_of_one_backlog_branch_at_the_same_commit_one_wins() {
    let hub = Hub::new(&[]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    let release = hub.dir.path().join("release");
    let (hold_a, hold_b) = (release.with_extension("a"), release.with_extension("b"));
    let held = |at: &Path, hold: &Path| {
        hub.command(at, "p1", &["start", "--backlog", "same"])
            .env("ATELIER_TEST_HOLD_BEFORE_BRANCH", hold)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
    };
    let (first, second) = (held(&a, &hold_a), held(&b, &hold_b));
    let since = Instant::now();
    while !(hold_a.with_extension("held").exists() && hold_b.with_extension("held").exists()) {
        assert!(since.elapsed() < Duration::from_secs(20), "never held");
        std::thread::sleep(Duration::from_millis(20));
    }
    // The first pushes, then the second, the same base commit both.
    fs::write(&hold_a, "").unwrap();
    let first = finished(first);
    ok(first);
    fs::write(&hold_b, "").unwrap();
    let second = finished(second);
    assert_eq!(second.code, 1, "{}", second.stdout);
    assert!(
        second
            .stderr
            .contains("origin already has branch backlog/same"),
        "{}",
        second.stderr
    );
    assert!(!branch_exists(&b, "backlog/same"));
}

#[test]
fn status_names_a_holders_uncommitted_drop_with_its_commit() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    let run = atelier(&root, "t1", &["drop", ID, "--reason", "Not needed"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let status = atelier(&root, "t1", &["status"]).stdout;
    let path = item_path(ID);
    assert!(
        status.contains(&format!(
            "Record changes not committed, which count only once committed and merged:\n  {path}\nCommit them with: git add -- {path} && git commit -m \"<what they change>\" -- {path}\n"
        )),
        "{status}"
    );
}

#[test]
fn review_and_verdict_are_refused_while_the_fetched_branch_is_ahead() {
    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    ok(hub.atelier(&a, "t1", &["start", ID]));
    let path = a.join(item_path(ID));
    let ticked = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, ticked + "It prints hello.\n").unwrap();
    ok(hub.atelier(&a, "t1", &["submit"]));
    let b = hub.clone("b");
    git(&b, &["switch", "-q", ID]);
    // A reviewer in the first clone opens a review, commits and pushes it.
    ok(hub.atelier(&a, "r1", &["review", ID]));
    git(&a, &["add", "-A"]);
    git(&a, &["commit", "-qm", "Open the review"]);
    git(&a, &["push", "-q", "origin", ID]);
    git(&b, &["fetch", "-q", "origin"]);
    let before = head(&b);
    for (thread, args) in [("r2", &["review", ID][..]), ("r1", &["verdict", "approve"])] {
        let run = hub.atelier(&b, thread, args);
        assert_eq!(run.code, 1, "{args:?}: {}{}", run.stdout, run.stderr);
        assert!(
            run.stderr.contains(&format!("origin/{ID} is at "))
                && run.stderr.contains("git pull --ff-only origin"),
            "{args:?}: {}",
            run.stderr
        );
        assert_eq!(head(&b), before);
        assert_eq!(git(&b, &["status", "--porcelain"]), "");
    }
}

#[test]
fn list_flags_narrow_a_drop_that_has_not_reached_the_base() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    let run = atelier(&root, "t1", &["drop", ID, "--reason", "Not needed"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    run_then(&root, &run.stdout);
    let pending = format!(
        "{ID} (dropped on its branch; the drop has not reached the base: Not needed): Say hello\n"
    );
    assert!(atelier(&root, "t9", &["list"]).stdout.contains(&pending));
    for args in [
        &["list", "--ready"][..],
        &["list", "--label", "unrelated"],
        &["list", "--mission", "demo-m1ss"],
    ] {
        let listed = atelier(&root, "t9", args).stdout;
        assert!(!listed.contains(ID), "{args:?}: {listed}");
    }
}

#[test]
fn a_record_write_says_how_its_change_reaches_the_base_for_its_branch() {
    let scratch = Scratch::new(&[(ID, "draft"), ("demo-c10s", "ready")]);
    let root = scratch.root();
    assert_eq!(
        atelier(&root, "p1", &["start", "--backlog", "plan"]).code,
        0
    );
    let run = atelier(&root, "p1", &["ready", ID, "--by", "sumeet"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let path = item_path(ID);
    assert_eq!(
        run.stdout,
        format!(
            "Made ready, as sumeet said: {ID}.\nWrote, and did not commit:\n  {path}\nIt counts once it is committed and its branch is merged: commit it, then run `atelier submit` and `atelier merge` on that branch.\nThen: git add -- {path} && git commit -m \"Ready {ID}\" -- {path}\n"
        )
    );

    // A started item's drop, on its branch, names its own next steps only.
    git(&root, &["checkout", "-q", "--detach", "main"]);
    assert_eq!(atelier(&root, "t1", &["start", "demo-c10s"]).code, 0);
    let run = atelier(
        &root,
        "p1",
        &["drop", "demo-c10s", "--reason", "Not needed"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    let path = item_path("demo-c10s");
    assert_eq!(
        run.stdout,
        format!(
            "Dropped item demo-c10s on its branch demo-c10s: Not needed\nOnce it is committed, `submit`, `merge` and a take-over of demo-c10s are refused.\nThe base's copy of demo-c10s is dropped on a backlog branch after that: `atelier start --backlog drop-demo-c10s`, then there `atelier drop demo-c10s --reason \"Not needed\"`, commit, `atelier submit` and `atelier merge`. Until then `status` and `list` say the drop has not reached the base.\nWrote, and did not commit:\n  {path}\nThen: git add -- {path} && git commit -m \"Drop demo-c10s\" -- {path}\n"
        )
    );
}

#[test]
fn a_review_finds_the_pull_request_of_the_prefixed_branch() {
    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    let config = a.join(".atelier/config.toml");
    let text = fs::read_to_string(&config).unwrap().replace(
        "base_branch = \"main\"\n",
        "base_branch = \"main\"\nbranch_prefix = \"x/\"\n",
    );
    fs::write(&config, text).unwrap();
    git(&a, &["commit", "-qam", "Configure"]);
    git(&a, &["push", "-q", "origin", "HEAD:main"]);
    git(&a, &["branch", "-f", "main", "HEAD"]);
    ok(hub.atelier(&a, "t1", &["start", ID]));
    assert_eq!(hub.tip(&format!("x/{ID}")), Some(head(&a)));
    let path = a.join(item_path(ID));
    let ticked = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, ticked + "It prints hello.\n").unwrap();
    ok(hub.atelier(&a, "t1", &["submit"]));
    let run = hub.atelier(&a, "r1", &["review", ID]);
    ok(run.clone());
    assert!(
        run.stdout.contains("Pull request #7, where CI's result on the branch can be read: https://github.com/owner/repo/pull/7\n"),
        "{}",
        run.stdout
    );
    let calls = fs::read_to_string(hub.gh.join("calls")).unwrap();
    assert!(calls.contains(&format!("head=owner:x/{ID}")), "{calls}");
    assert!(!calls.contains(&format!("head=owner:{ID}")), "{calls}");
}

#[test]
fn submit_on_a_backlog_branch_counts_an_uncommitted_config_change() {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    assert_eq!(
        atelier(&root, "p1", &["start", "--backlog", "config"]).code,
        0
    );
    append(&root, ".atelier/config.toml", "# note\n");
    let run = atelier(&root, "p1", &["submit"]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(
        run.stderr.contains(
            "are not committed, and a submit publishes only what is:\n  .atelier/config.toml\n"
        ),
        "{}",
        run.stderr
    );
}

#[test]
fn a_branch_that_changes_only_a_missions_lead_merges_after_the_base_changes_its_plan() {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    let path = root.join(".atelier/missions/demo-m1ss/mission.md");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mission = "---\nschema: atelier.mission\nschema_version: 3\nid: demo-m1ss\ntitle: Greet\n\
                   status: ready\nlabels: []\nlead: null\nready: null\n\
                   created_at: 2026-11-10T09:00:00Z\n---\n\n## Why\n\nTo greet.\n\n## Outcome\n\nGreetings.\n";
    fs::write(&path, mission).unwrap();
    on_base(&root, "Plan a mission");
    assert_eq!(
        atelier(&root, "l1", &["start", "--backlog", "lead"]).code,
        0
    );
    let run = atelier(&root, "l1", &["lead", "demo-m1ss"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    run_then(&root, &run.stdout);
    // Meanwhile the base's plan moves on.
    let other = scratch.worktree("other");
    let text = fs::read_to_string(other.join(".atelier/missions/demo-m1ss/mission.md")).unwrap();
    fs::write(
        other.join(".atelier/missions/demo-m1ss/mission.md"),
        text.replace("To greet.", "To greet everyone."),
    )
    .unwrap();
    on_base(&other, "Sharpen the why");
    let run = atelier(&root, "l1", &["merge"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let merged = git(
        &root,
        &["show", "main:.atelier/missions/demo-m1ss/mission.md"],
    );
    assert!(
        merged.contains("lead: l1\n") && merged.contains("To greet everyone."),
        "{merged}"
    );
}

/// The branch checked out in `at`.
fn current(at: &Path) -> String {
    git(at, &["symbolic-ref", "--short", "HEAD"])
}

fn mission_path(id: &str) -> String {
    format!(".atelier/missions/{id}/mission.md")
}

/// A mission planned in `at` by `t1` on its branch, with one item and an
/// approving plan review by `r1`, pushed: its ID and the item's.
fn planned_and_pushed(hub: &Hub, at: &Path) -> (String, String) {
    ok(hub.atelier(at, "t1", &["start", "--mission", "Greet"]));
    let mission = current(at);
    let made = hub.atelier(at, "t1", &["new", "Say hi", "--in", &mission]);
    ok(made.clone());
    let item = fs::read_dir(at.join(".atelier/items"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .find(|id| id != ID)
        .expect("the new item");
    let commit = head(at);
    let review = format!(".atelier/missions/{mission}/reviews/1-abcd.md");
    fs::create_dir_all(at.join(&review).parent().unwrap()).unwrap();
    fs::write(
        at.join(&review),
        format!(
            "---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: {commit}\n\
             reviewers:\n- r1\nmodel: null\nverdict: approve\ncreated_at: 2026-11-13T09:00:00Z\n---\n\n\
             ## Summary\n\nGood.\n"
        ),
    )
    .unwrap();
    git(at, &["add", "-A"]);
    git(at, &["commit", "-qm", "Plan it"]);
    git(at, &["push", "-q", "origin", &mission]);
    (mission, item)
}

#[test]
fn a_plan_pushed_from_one_clone_is_proposed_in_another_once_it_has_fetched() {
    let hub = Hub::new(&[(ID, "ready")]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    let (mission, item) = planned_and_pushed(&hub, &a);

    // No command fetches: before a fetch the plan is nowhere in b.
    let before = hub.atelier(&b, "p1", &["list", "--proposed"]);
    ok(before.clone());
    assert!(
        before
            .stdout
            .contains("Nothing matching is proposed on a branch not yet merged.\n"),
        "{}",
        before.stdout
    );
    assert!(before
        .stdout
        .contains("origin was never fetched in this clone"));
    ok(hub.atelier(&b, "p1", &["show", ID]));
    ok(hub.atelier(&b, "p1", &["status"]));
    let tracking = format!("refs/remotes/origin/{mission}");
    assert_eq!(git(&b, &["for-each-ref", &tracking]), "");

    git(&b, &["fetch", "-q", "origin"]);
    let listed = hub.atelier(&b, "p1", &["list"]);
    ok(listed.clone());
    let proposed = format!(
        "Proposed, on branches not yet merged:\nMission {mission} (proposed on origin/{mission}): Greet\n  \
         No label:\n    {item} (draft, medium, on origin/{mission}): Say hi\n"
    );
    assert!(listed.stdout.contains(&proposed), "{}", listed.stdout);
    assert!(
        listed.stdout.starts_with(
            "Read from this clone's branches, local and of origin; origin was last fetched "
        ),
        "{}",
        listed.stdout
    );
    // The base's backlog is listed apart, and holds neither.
    let base = listed.stdout.split("\nProposed").next().unwrap();
    assert!(base.contains(&format!("{ID} (ready, high, can start now)")));
    assert!(!base.contains(&mission) && !base.contains(&item), "{base}");

    let only = hub.atelier(&b, "p1", &["list", "--proposed"]);
    ok(only.clone());
    assert!(only.stdout.contains(&proposed), "{}", only.stdout);
    assert!(!only.stdout.contains(ID), "{}", only.stdout);

    // `show` reads the plan, and its reviews' verdicts, from the branch.
    let shown = hub.atelier(&b, "p1", &["show", &mission]);
    ok(shown.clone());
    let lines: Vec<&str> = shown.stdout.lines().collect();
    assert_eq!(lines[0], format!("Mission {mission}: Greet"));
    assert!(
        lines[1].starts_with(&format!(
            "Read from: origin/{mission} (origin was last fetched "
        )) && lines[1].ends_with(
            " ago), where it is proposed: it is not on the base until that branch merges"
        ),
        "{}",
        shown.stdout
    );
    assert_eq!(lines[2], "Status: proposed");
    assert!(
        shown.stdout.contains(&format!(
            "\nItems:\n  Draft:\n    {item} (medium, on origin/{mission}): Say hi\n"
        )),
        "{}",
        shown.stdout
    );
    assert!(
        shown
            .stdout
            .contains("\nPlan reviews:\n  Round 1, review 1-abcd by r1, of commit "),
        "{}",
        shown.stdout
    );
    assert!(shown.stdout.ends_with(": approved\n"), "{}", shown.stdout);
    let item_shown = hub.atelier(&b, "p1", &["show", &item]);
    ok(item_shown.clone());
    assert!(
        item_shown.stdout.contains(&format!(
            "\nRead from: origin/{mission} (origin was last fetched "
        )),
        "{}",
        item_shown.stdout
    );

    // In the clone that planned it, the local branch is read.
    let here = hub.atelier(&a, "t1", &["list", "--proposed"]);
    ok(here.clone());
    assert!(
        here.stdout.contains(&format!(
            "Mission {mission} (proposed on {mission}): Greet\n"
        )),
        "{}",
        here.stdout
    );
}

#[test]
fn a_mission_dropped_on_its_branch_is_listed_as_dropped_and_not_as_proposed() {
    let hub = Hub::new(&[(ID, "ready")]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    let (mission, _) = planned_and_pushed(&hub, &a);
    let path = a.join(mission_path(&mission));
    let text = fs::read_to_string(&path).unwrap();
    let dropped = text.replace("status: open", "status: dropped");
    let dropped = match dropped.contains("## Result\n") {
        true => dropped.replace("## Result\n", "## Result\n\nDropped: no longer wanted\n"),
        false => dropped + "\n## Result\n\nDropped: no longer wanted\n",
    };
    fs::write(&path, dropped).unwrap();
    git(&a, &["commit", "-qam", "Drop it"]);
    git(&a, &["push", "-q", "origin", &mission]);
    git(&b, &["fetch", "-q", "origin"]);

    let listed = hub.atelier(&b, "p1", &["list"]);
    ok(listed.clone());
    assert!(
        listed.stdout.contains(&format!(
            "Missions dropped on their branch, not merged:\n  {mission} (dropped on origin/{mission}: no longer wanted): Greet\n"
        )),
        "{}",
        listed.stdout
    );
    assert!(!listed.stdout.contains("Proposed, on"), "{}", listed.stdout);
    let only = hub.atelier(&b, "p1", &["list", "--proposed"]);
    ok(only.clone());
    assert!(
        !only.stdout.contains(&format!("Mission {mission}")),
        "{}",
        only.stdout
    );
    let shown = hub.atelier(&b, "p1", &["show", &mission]);
    ok(shown.clone());
    assert!(
        shown.stdout.contains("\nStatus: dropped\n"),
        "{}",
        shown.stdout
    );
    assert!(!shown.stdout.contains("proposed"), "{}", shown.stdout);
    assert!(
        shown
            .stdout
            .contains("## Result\n\nDropped: no longer wanted\n"),
        "{}",
        shown.stdout
    );
}

#[test]
fn a_backlog_branch_s_record_changes_are_listed_as_proposed_with_its_branch() {
    let hub = Hub::new(&[(ID, "ready")]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    ok(hub.atelier(&a, "t1", &["start", "--backlog", "tidy"]));
    let path = a.join(item_path(ID));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text.replace("status: ready", "status: draft")).unwrap();
    ok(hub.atelier(&a, "t1", &["new", "Say bye"]));
    git(&a, &["add", "-A"]);
    git(&a, &["commit", "-qm", "Tidy"]);
    git(&a, &["push", "-q", "origin", "backlog/tidy"]);
    git(&b, &["fetch", "-q", "origin"]);

    let listed = hub.atelier(&b, "p1", &["list"]);
    ok(listed.clone());
    assert!(
        listed.stdout.contains(&format!(
            "Record changes on branches not yet merged, which take effect when each merges:\n  \
             {ID} (on origin/backlog/tidy: ready on the base, draft there): Say hello\n"
        )),
        "{}",
        listed.stdout
    );
    assert!(
        listed
            .stdout
            .contains("(draft, medium, on origin/backlog/tidy): Say bye\n"),
        "{}",
        listed.stdout
    );
    // The change takes effect when the branch merges: the record is the
    // base's until then.
    assert!(listed
        .stdout
        .contains(&format!("{ID} (ready, high, can start now)")));
    let shown = hub.atelier(&b, "p1", &["show", ID]);
    ok(shown.clone());
    assert!(
        shown.stdout.starts_with(&format!(
            "Item {ID}: Say hello\nRead from: main\nChanged on branches not yet merged: origin/backlog/tidy (draft there)\nStatus: ready"
        )),
        "{}",
        shown.stdout
    );
}

#[test]
fn status_lists_an_item_started_in_another_clone_with_the_thread_its_branch_names() {
    let hub = Hub::new(&[(ID, "ready")]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    ok(hub.atelier(&a, "t1", &["start", ID]));
    git(&b, &["fetch", "-q", "origin"]);
    let status = hub.atelier(&b, "t2", &["status"]);
    ok(status.clone());
    assert!(
        status.stdout.contains(&format!(
            "Held by other threads:\n  {ID} (building, held by t1, on origin/{ID}): Say hello\n"
        )),
        "{}",
        status.stdout
    );
    let shown = hub.atelier(&b, "t2", &["show", ID]);
    ok(shown.clone());
    assert!(
        shown.stdout.contains(&format!(
            "\nRead from: origin/{ID} (origin was last fetched "
        )),
        "{}",
        shown.stdout
    );
}

/// The bound `list` is held to with 200 named remote branches, each an
/// item started in another clone. Ignored by default, since it times this
/// host; run with `cargo nextest run -p atelier-cli --run-ignored all
/// list_with_200`.
#[test]
#[ignore]
fn list_with_200_named_remote_branches_takes_under_two_seconds() {
    let ids: Vec<String> = (0..200).map(|n| format!("demo-p{n:03}")).collect();
    let items: Vec<(&str, &str)> = ids.iter().map(|id| (id.as_str(), "ready")).collect();
    let hub = Hub::new(&items);
    let seed = hub.dir.path().join("seed");
    for id in &ids {
        git(&seed, &["checkout", "-q", "-B", id, "main"]);
        let path = seed.join(item_path(id));
        let text = fs::read_to_string(&path).unwrap();
        let started = text
            .replace("status: ready", "status: building")
            .replace("implementers: []", "implementers:\n- t1");
        fs::write(&path, started).unwrap();
        git(&seed, &["commit", "-qam", "Start"]);
    }
    git(
        &seed,
        &["push", "-q", hub.remote.to_str().unwrap(), "--all"],
    );
    let b = hub.clone("b");
    git(&b, &["fetch", "-q", "origin"]);
    let count = git(&b, &["for-each-ref", "refs/remotes/origin/demo-p*"])
        .lines()
        .count();
    assert_eq!(count, 200);
    let since = Instant::now();
    let listed = hub.atelier(&b, "p1", &["list"]);
    let took = since.elapsed();
    ok(listed.clone());
    assert_eq!(
        listed.stdout.matches(", on origin/demo-p").count(),
        200,
        "{}",
        listed.stdout
    );
    eprintln!("list with 200 named remote branches took {took:?}");
    assert!(took < Duration::from_secs(2), "{took:?}");
}

/// The item in `at` whose ID is not one of `known`.
fn new_item(at: &Path, known: &[&str]) -> String {
    fs::read_dir(at.join(".atelier/items"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .find(|id| !known.contains(&id.as_str()))
        .expect("the new item")
}

#[test]
fn a_record_written_but_not_committed_on_the_branch_checked_out_is_found() {
    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    ok(hub.atelier(&a, "t1", &["start", "--backlog", "tidy"]));
    // The branch is at the base's tip, with no commit of its own yet.
    ok(hub.atelier(&a, "t1", &["new", "Say bye"]));
    let first = new_item(&a, &[ID]);
    let listed = hub.atelier(&a, "t1", &["list", "--proposed"]);
    ok(listed.clone());
    assert!(
        listed.stdout.contains(&format!(
            "{first} (draft, medium, on backlog/tidy): Say bye\n"
        )),
        "{}",
        listed.stdout
    );
    let shown = hub.atelier(&a, "t1", &["show", &first]);
    ok(shown.clone());
    assert!(
        shown
            .stdout
            .contains("\nRead from: backlog/tidy, where it is proposed"),
        "{}",
        shown.stdout
    );

    // With a commit on the branch, a later record not committed is found
    // beside the committed one.
    git(&a, &["add", "-A"]);
    git(&a, &["commit", "-qm", "File one"]);
    ok(hub.atelier(&a, "t1", &["new", "Say thanks"]));
    let second = new_item(&a, &[ID, &first]);
    let listed = hub.atelier(&a, "t1", &["list", "--proposed"]);
    ok(listed.clone());
    for (id, title) in [(&first, "Say bye"), (&second, "Say thanks")] {
        assert!(
            listed
                .stdout
                .contains(&format!("{id} (draft, medium, on backlog/tidy): {title}\n")),
            "{}",
            listed.stdout
        );
    }
    ok(hub.atelier(&a, "t1", &["show", &second]));
}

#[test]
fn a_ready_item_only_on_an_unmerged_branch_cannot_start_and_is_not_said_to() {
    let hub = Hub::new(&[(ID, "draft")]);
    let a = hub.clone("a");
    ok(hub.atelier(&a, "t1", &["start", "--backlog", "tidy"]));
    ok(hub.atelier(&a, "t1", &["new", "Say bye"]));
    let id = new_item(&a, &[ID]);
    let path = a.join(item_path(&id));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text.replace("status: draft", "status: ready")).unwrap();
    git(&a, &["add", "-A"]);
    git(&a, &["commit", "-qm", "File it ready"]);

    let line = format!("{id} (ready, medium, on backlog/tidy): Say bye\n");
    let proposed = hub.atelier(&a, "t1", &["list", "--proposed"]);
    ok(proposed.clone());
    assert!(proposed.stdout.contains(&line), "{}", proposed.stdout);
    for flags in [&["list", "--ready"][..], &["list", "--proposed", "--ready"]] {
        let listed = hub.atelier(&a, "t1", flags);
        ok(listed.clone());
        assert!(!listed.stdout.contains(&id), "{flags:?}: {}", listed.stdout);
    }
    let shown = hub.atelier(&a, "t1", &["show", &id]);
    ok(shown.clone());
    assert!(
        shown.stdout.contains("\nStatus: ready\n"),
        "{}",
        shown.stdout
    );
    let status = hub.atelier(&a, "t1", &["status"]);
    ok(status.clone());
    assert!(!status.stdout.contains(&id), "{}", status.stdout);
    let started = hub.atelier(&a, "t2", &["start", &id]);
    assert_eq!(started.code, 1, "{}{}", started.stdout, started.stderr);
}

#[test]
fn a_proposed_record_change_is_narrowed_by_the_flags_its_record_would_be() {
    let hub = Hub::new(&[(ID, "draft")]);
    let a = hub.clone("a");
    ok(hub.atelier(&a, "t1", &["start", "--backlog", "tidy"]));
    let path = a.join(item_path(ID));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        text.replace("title: Say hello", "title: Say hello twice"),
    )
    .unwrap();
    git(&a, &["commit", "-qam", "Retitle"]);

    let change = format!("{ID} (on backlog/tidy: changed there, still draft): Say hello\n");
    let listed = hub.atelier(&a, "t1", &["list", "--proposed"]);
    ok(listed.clone());
    assert!(listed.stdout.contains(&change), "{}", listed.stdout);
    for flag in ["--mine", "--ready", "--needs-human", "--blocked"] {
        let narrowed = hub.atelier(&a, "t1", &["list", "--proposed", flag]);
        ok(narrowed.clone());
        assert!(!narrowed.stdout.contains(ID), "{flag}: {}", narrowed.stdout);
    }
    let labelled = hub.atelier(&a, "t1", &["list", "--proposed", "--label", "none-such"]);
    ok(labelled.clone());
    assert!(!labelled.stdout.contains(ID), "{}", labelled.stdout);
}

/// The command the line of `output` that starts with `prefix` gives after
/// its last `: `.
fn command_after(output: &str, prefix: &str) -> String {
    let line = output
        .lines()
        .find(|line| line.starts_with(prefix))
        .unwrap_or_else(|| panic!("no line {prefix:?} in {output}"));
    line.rsplit_once(": ").unwrap().1.to_string()
}

fn sh(at: &Path, line: &str) -> std::process::Output {
    identity(Command::new("sh").current_dir(at).args(["-c", line]))
        .output()
        .unwrap()
}

/// An approving plan review `name` of `mission` by `thread` of `commit`,
/// written and committed in `at`.
fn approve(at: &Path, mission: &str, name: &str, thread: &str, commit: &str) {
    let path = format!(".atelier/missions/{mission}/reviews/{name}.md");
    fs::create_dir_all(at.join(&path).parent().unwrap()).unwrap();
    fs::write(
        at.join(&path),
        format!(
            "---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: {commit}\n\
             reviewers:\n- {thread}\nmodel: null\nverdict: approve\ncreated_at: 2026-11-13T09:00:00Z\n---\n\n\
             ## Summary\n\nGood.\n"
        ),
    )
    .unwrap();
    git(at, &["add", "-A"]);
    git(at, &["commit", "-qm", &format!("Review {name}")]);
}

#[test]
fn two_reviewers_in_clones_of_their_own_land_their_files_on_the_plans_branch_and_it_merges() {
    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    ok(hub.atelier(&a, "p1", &["start", "--mission", "Greet"]));
    let mission = current(&a);
    let path = a.join(mission_path(&mission));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        text.replace("## Why\n", "## Why\n\nAsked for.\n")
            .replace("## Outcome\n", "## Outcome\n\nGreetings.\n"),
    )
    .unwrap();
    git(&a, &["commit", "-qam", "Plan it"]);
    let submitted = hub.atelier(&a, "p1", &["submit"]);
    ok(submitted.clone());
    let commit = head(&a);
    assert_eq!(hub.tip(&mission).as_deref(), Some(commit.as_str()));

    // Each reviewer in a clone of its own, detached at the commit
    // submitted, so neither sees the other's file.
    let reviewers: Vec<(PathBuf, &str)> = [("r1", "c1"), ("r2", "c2")]
        .into_iter()
        .map(|(thread, name)| {
            let at = hub.clone(name);
            git(&at, &["fetch", "-q", "origin", &mission]);
            git(&at, &["checkout", "-q", "--detach", &commit]);
            let run = hub.atelier(&at, thread, &["review", &mission]);
            ok(run.clone());
            assert!(
                run.stdout.contains(&format!(
                    "Push it to {mission}: git push origin HEAD:refs/heads/{mission}\n"
                )),
                "{}",
                run.stdout
            );
            (at, thread)
        })
        .collect();
    let mut again = Vec::new();
    for (at, thread) in &reviewers {
        let run = hub.atelier(at, thread, &["verdict", "approve"]);
        ok(run.clone());
        run_then(at, &run.stdout);
        let push = command_after(&run.stdout, &format!("Push it to {mission}: "));
        if sh(at, &push).status.success() {
            again.push(false);
            continue;
        }
        let after = command_after(
            &run.stdout,
            "If that is refused because the other reviewer got there first",
        );
        let output = sh(at, &after);
        assert!(output.status.success(), "{after}: {output:?}");
        again.push(true);
    }
    assert_eq!(again, [false, true]);

    // The planner brings both files in, and the plan merges. Once it has
    // fetched them, a submit from here would leave them out.
    git(&a, &["fetch", "-q", "origin"]);
    let refused = hub.atelier(&a, "p1", &["submit"]);
    assert_eq!(refused.code, 1, "{}", refused.stdout);
    assert!(
        refused
            .stderr
            .ends_with(&format!("Then: git pull --no-rebase origin {mission}\n")),
        "{}",
        refused.stderr
    );
    git(&a, &["pull", "-q", "--no-rebase", "origin", &mission]);
    let reviews = fs::read_dir(a.join(format!(".atelier/missions/{mission}/reviews")))
        .unwrap()
        .count();
    assert_eq!(reviews, 2);
    let merged = hub.atelier(&a, "p1", &["merge"]);
    ok(merged.clone());
    let base = hub.tip("main").unwrap();
    let text = git(
        &hub.remote,
        &["show", &format!("{base}:{}", mission_path(&mission))],
    );
    assert!(text.contains("\nstatus: open\n"), "{text}");
}

#[test]
fn a_plan_dropped_on_its_branch_with_two_approvals_does_not_merge_and_the_stop_counts_once_pushed()
{
    let hub = Hub::new(&[(ID, "ready")]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    let (mission, _) = planned_and_pushed(&hub, &a);
    let reviewed = git(&a, &["rev-parse", "HEAD^"]);
    approve(&a, &mission, "1-efgh", "r2", &reviewed);
    git(&a, &["push", "-q", "origin", &mission]);

    let run = hub.atelier(&a, "t1", &["drop", &mission, "--reason", "Not now"]);
    ok(run.clone());
    assert!(
        run.stdout.contains(&format!(
            "It stops nothing until it is published: commit it, push the branch (`git push origin {mission}`) and close its pull request (`gh pr close {mission}`). Its branch is never merged.\n"
        )),
        "{}",
        run.stdout
    );
    run_then(&a, &run.stdout);
    // Committed and not pushed, it is not published.
    let unpublished = format!("Stop not published: mission {mission}'s plan is dropped here, and origin's copy of {mission} is not");
    for args in [&["status"][..], &["show", &mission]] {
        let run = hub.atelier(&a, "t1", args);
        assert!(
            run.stdout.contains(&unpublished),
            "{args:?}: {}",
            run.stdout
        );
    }
    let base = hub.tip("main");
    let run = hub.atelier(&a, "t1", &["merge"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.starts_with(&format!(
            "Mission {mission} is dropped, and `atelier merge` runs on a mission that is open."
        )),
        "{}",
        run.stderr
    );
    assert_eq!(hub.tip("main"), base);

    // Pushed, it is published: the other clone reads the plan as dropped
    // once it has fetched.
    git(&a, &["push", "-q", "origin", &mission]);
    let run = hub.atelier(&a, "t1", &["status"]);
    assert!(!run.stdout.contains("Stop not published"), "{}", run.stdout);
    git(&b, &["fetch", "-q", "origin"]);
    let shown = hub.atelier(&b, "p1", &["show", &mission]);
    ok(shown.clone());
    assert!(
        shown.stdout.contains("\nStatus: dropped\n"),
        "{}",
        shown.stdout
    );
    assert!(
        !shown.stdout.contains("Stop not published"),
        "{}",
        shown.stdout
    );
}

#[test]
fn a_stop_of_an_agreed_mission_refuses_its_ready_and_started_items_in_a_clone_that_has_fetched() {
    const READY: &str = "demo-r3dy";
    const MISSION: &str = "demo-m1ss";
    let hub = Hub::new(&[(ID, "ready"), (READY, "ready")]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    // The mission agreed on the base, with both items in it.
    git(&a, &["checkout", "-q", "main"]);
    let path = a.join(mission_path(MISSION));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        &path,
        "---\nschema: atelier.mission\nschema_version: 3\nid: demo-m1ss\ntitle: Greet\n\
         status: open\nlabels: []\nlead: null\nplanners:\n- p1\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Why\n\nAsked for.\n\n## Outcome\n\nGreetings.\n\n## Result\n",
    )
    .unwrap();
    for id in [ID, READY] {
        let item = a.join(item_path(id));
        let text = fs::read_to_string(&item).unwrap();
        fs::write(&item, text.replace("mission: null", "mission: demo-m1ss")).unwrap();
    }
    git(&a, &["add", "-A"]);
    git(&a, &["commit", "-qm", "Agree the mission"]);
    git(&a, &["push", "-q", "origin", "main"]);
    git(&a, &["checkout", "-q", "--detach", "main"]);
    git(&b, &["fetch", "-q", "origin"]);
    git(&b, &["checkout", "-q", "--detach", "origin/main"]);
    git(&b, &["branch", "-qf", "main", "origin/main"]);
    // One of its items started and approved in the other clone.
    hub.approved(&b, "t1");
    let listed = hub.atelier(&b, "t2", &["list", "--ready"]);
    assert!(listed.stdout.contains(READY), "{}", listed.stdout);

    // The stop, written on a backlog branch, is not published until that
    // branch merges.
    ok(hub.atelier(&a, "p1", &["start", "--backlog", "stop"]));
    let run = hub.atelier(&a, "p1", &["drop", MISSION, "--reason", "No time"]);
    ok(run.clone());
    run_then(&a, &run.stdout);
    let shown = hub.atelier(&a, "p1", &["show", MISSION]);
    assert!(
        shown.stdout.contains("Stop not published: mission demo-m1ss is dropped on backlog/stop, which has not merged"),
        "{}",
        shown.stdout
    );
    ok(hub.atelier(&a, "p1", &["submit"]));
    ok(hub.atelier(&a, "p1", &["merge"]));

    // The other clone fetches, and its own base has not moved: the stop
    // holds all the same.
    git(&b, &["fetch", "-q", "origin"]);
    assert_ne!(
        git(&b, &["rev-parse", "main"]),
        git(&b, &["rev-parse", "origin/main"])
    );
    let stop =
        "Mission demo-m1ss is not dropped (it was stopped, so none of its items starts or merges)";
    let run = hub.atelier(&b, "t2", &["start", READY]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(run.stderr.contains(stop), "{}", run.stderr);
    // `list` reads the stop the same way: nothing of it can start.
    let listed = hub.atelier(&b, "t2", &["list", "--ready"]);
    ok(listed.clone());
    assert!(!listed.stdout.contains(READY), "{}", listed.stdout);
    let run = hub.atelier(&b, "t1", &["merge"]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(run.stderr.contains(stop), "{}", run.stderr);
}
