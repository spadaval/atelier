//! What ends a branch, and what stays safe once it is gone: a started item
//! dropped on one backlog branch, `merge`'s deletion of the branch it
//! merged, the claim checked against the base now that a merged branch no
//! longer refuses it, and a detached HEAD that loses no commit. Driven in
//! clones of one bare remote with a fake `gh`.

mod common;

use common::hub::*;
use common::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

const OTHER: &str = "demo-cd34";

fn head(at: &Path) -> String {
    git(at, &["rev-parse", "HEAD"])
}

fn current(at: &Path) -> String {
    git(at, &["branch", "--show-current"])
}

fn tracking(at: &Path, branch: &str) -> Option<String> {
    let reference = format!("refs/remotes/origin/{branch}");
    let output = git_output(at, &["rev-parse", "--verify", "--quiet", &reference]);
    output
        .status
        .success()
        .then(|| String::from_utf8(output.stdout).unwrap().trim().to_string())
}

fn skipping(hub: &Hub, at: &Path, thread: &str, args: &[&str]) -> Run {
    let mut command = hub.command(at, thread, args);
    let output = command.env("ATELIER_SKIP", "Ada said to").output().unwrap();
    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

/// `t1` starts `ID` in `at` and pushes a commit of work.
fn working(hub: &Hub, at: &Path) -> String {
    ok(hub.atelier(at, "t1", &["start", ID]));
    fs::write(at.join("hello.txt"), "hello\n").unwrap();
    git(at, &["add", "hello.txt"]);
    git(at, &["commit", "-qm", "Say hello"]);
    git(at, &["push", "-q", "origin", ID]);
    head(at)
}

/// Ticks `ID`'s Done when lines and writes its Report in `at`.
fn finished(at: &Path) {
    let path = at.join(item_path(ID));
    let ticked = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, ticked + "It prints hello.\n").unwrap();
}

/// A backlog branch in `at` that drops `ID`, committed.
fn dropping(hub: &Hub, at: &Path) -> Run {
    ok(hub.atelier(at, "p1", &["start", "--backlog", "drop-it"]));
    let run = hub.atelier(at, "p1", &["drop", ID, "--reason", "Not needed"]);
    ok(run.clone());
    run_then(at, &run.stdout);
    run
}

/// `ID` dropped on the remote's base by a backlog branch merged from a
/// clone of its own.
fn dropped(hub: &Hub) {
    let planner = hub.clone("dropper");
    dropping(hub, &planner);
    ok(hub.atelier(&planner, "p1", &["merge"]));
    assert_eq!(hub.field("main", "status"), "dropped");
}

/// Makes the next push from `clone` find that the remote's base has
/// `status` for `ID`, as when another clone's merge landed between this
/// one's fetch and its push.
fn base_moves_at_push(hub: &Hub, clone: &Path, status: &str) {
    let at = hub.clone("mover");
    write_item(&at, ID, status, &[], &[], "Dropped: Not needed\n");
    git(&at, &["commit", "-qam", "End it"]);
    git(&at, &["push", "-q", "origin", "HEAD:refs/prepared/end"]);
    at_push(
        clone,
        &format!(
            "git --git-dir='{}' update-ref refs/heads/main {}",
            hub.remote.display(),
            head(&at)
        ),
    );
}

/// Sends `clone`'s `origin` to a repository that is not there.
fn cut_off(hub: &Hub, clone: &Path) {
    let reached = format!("url.{}.insteadOf", hub.remote.display());
    git(clone, &["config", "--unset", &reached]);
    let gone = hub.dir.path().join("gone.git");
    git(
        clone,
        &["config", &format!("url.{}.insteadOf", gone.display()), URL],
    );
}

/// Runs the shell lines `line` in `clone`'s pre-push hook, once.
fn at_push(clone: &Path, line: &str) {
    let hook = clone.join(".git/hooks/pre-push");
    fs::write(&hook, format!("#!/bin/sh\n{line}\nrm -f \"$0\"\n")).unwrap();
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
}

/// Runs the shell text `script` when the forge has merged, before `merge`
/// hears back.
fn after_merge(hub: &Hub, script: &str) {
    fs::write(hub.gh.join("after-merge"), script).unwrap();
}

#[test]
fn a_started_item_is_dropped_on_one_backlog_branch_whose_merge_closes_its_pull_request() {
    let hub = Hub::new(&[(ID, "ready")]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    let work = working(&hub, &a);

    git(&b, &["fetch", "-q", "origin"]);
    let run = dropping(&hub, &b);
    assert!(
        run.stdout.starts_with(&format!(
            "Dropped started item {ID}: Not needed\nOnce this branch merges, {ID}'s `submit`, `merge` and take-over are refused. Branch {ID} is left as it is. That merge closes its pull request, if one is open.\n"
        )),
        "{}",
        run.stdout
    );
    assert!(!run.stdout.contains("also adds items"), "{}", run.stdout);
    let text = read(&b, &item_path(ID));
    assert_eq!(front(&text, "status"), "dropped");
    assert!(text.contains("Dropped: Not needed\n"), "{text}");

    fs::create_dir_all(hub.gh.join("open")).unwrap();
    fs::write(
        hub.gh.join("open").join(ID),
        r#"[{"number":9,"html_url":"https://github.com/owner/repo/pull/9"}]"#,
    )
    .unwrap();
    ok(hub.atelier(&b, "p1", &["submit"]));
    let run = hub.atelier(&b, "p1", &["merge"]);
    ok(run.clone());
    assert!(
        run.stdout
            .contains(&format!("Closed pull request #9 of dropped {ID}.\n")),
        "{}",
        run.stdout
    );
    let closed = fs::read_to_string(hub.gh.join("closed")).unwrap();
    assert_eq!(
        closed,
        format!(
            "pr close 9 --repo github.com/owner/repo --comment {ID} is dropped on main, by the merge of backlog/drop-it.\n"
        )
    );
    // The one branch was enough, and the item's is left as it was.
    assert_eq!(hub.field("main", "status"), "dropped");
    assert_eq!(hub.tip(ID), Some(work));
    let status = hub.atelier(&b, "p1", &["status"]).stdout;
    assert!(!status.contains("has not reached the base"), "{status}");
    assert!(
        status.contains(&format!(
            "nothing is left to do on them:\n  {ID}: Not needed\n"
        )),
        "{status}"
    );
}

#[test]
fn a_pull_request_that_cannot_be_closed_is_a_warning_and_the_drop_merges() {
    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    working(&hub, &a);
    fs::create_dir_all(hub.gh.join("open")).unwrap();
    fs::write(
        hub.gh.join("open").join(ID),
        r#"[{"number":9,"html_url":"https://github.com/owner/repo/pull/9"}]"#,
    )
    .unwrap();
    fs::write(hub.gh.join("close-refused"), "").unwrap();
    let b = hub.clone("b");
    dropping(&hub, &b);
    let run = hub.atelier(&b, "p1", &["merge"]);
    ok(run.clone());
    assert!(
        run.stdout.contains(&format!(
            "Warning: could not close an open pull request of dropped {ID}: "
        )) && run.stdout.contains("HTTP 502: Bad Gateway"),
        "{}",
        run.stdout
    );
    assert_eq!(hub.field("main", "status"), "dropped");
    assert_eq!(hub.tip("backlog/drop-it"), None);
}

#[test]
fn a_drop_names_each_item_the_branch_added_and_the_command_that_brings_it() {
    let hub = Hub::new(&[(ID, "ready"), ("demo-kn0w", "draft")]);
    let a = hub.clone("a");
    working(&hub, &a);
    // One it filed is pushed, and one is only in this clone.
    for (id, push) in [("demo-f1ed", true), ("demo-l0ca", false)] {
        write_item(&a, id, "draft", &[], &[], "");
        git(&a, &["add", "-A"]);
        git(&a, &["commit", "-qm", "File"]);
        if push {
            git(&a, &["push", "-q", "origin", ID]);
        }
    }
    // A change to an item the base holds is no item it adds.
    append(&a, &item_path("demo-kn0w"), "More.\n");
    git(&a, &["commit", "-qam", "Say more"]);

    let b = hub.clone("b");
    let run = dropping(&hub, &b);
    let there = format!(
        "Branch {ID} also adds items that are not on main, and dropping {ID} leaves them behind. For each still wanted, run its command on a backlog branch and commit it:\n  demo-f1ed: Say hello\n    git checkout origin/{ID} -- .atelier/items/demo-f1ed\n"
    );
    assert!(run.stdout.contains(&there), "{}", run.stdout);
    assert!(!run.stdout.contains("demo-l0ca"), "{}", run.stdout);
    git(
        &b,
        &[
            "checkout",
            &format!("origin/{ID}"),
            "--",
            ".atelier/items/demo-f1ed",
        ],
    );
    assert!(b.join(item_path("demo-f1ed")).exists());

    // Where the branch is local, what was never pushed is named too, and so
    // it is by a drop on the item's own branch.
    let on_branch = hub.atelier(&a, "t1", &["drop", ID, "--reason", "Not needed"]);
    ok(on_branch.clone());
    for id in ["demo-f1ed", "demo-l0ca"] {
        let line = format!("    git checkout {ID} -- .atelier/items/{id}\n");
        assert!(on_branch.stdout.contains(&line), "{}", on_branch.stdout);
    }
    assert!(
        !on_branch.stdout.contains("demo-kn0w:"),
        "{}",
        on_branch.stdout
    );
    // And by the drop that carries that one to the base.
    run_then(&a, &on_branch.stdout);
    git(&a, &["checkout", "-q", "--detach", "origin/main"]);
    ok(hub.atelier(&a, "p1", &["start", "--backlog", "carry"]));
    let carried = hub.atelier(&a, "p1", &["drop", ID, "--reason", "Not needed"]);
    ok(carried.clone());
    assert!(
        carried.stdout.contains("  demo-l0ca: "),
        "{}",
        carried.stdout
    );
}

#[test]
fn an_item_only_on_its_branch_is_dropped_on_a_backlog_branch_with_its_definition() {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    let work = scratch.worktree("work");
    ok(atelier(&work, "t1", &["start", "--new", "Sneeze"]));
    let id = current(&work);
    let path = item_path(&id);
    let text = read(&work, &path)
        .replace("## Goal\n", "## Goal\n\nSneeze.\n")
        .replace("## Done when\n", "## Done when\n\n- [x] It sneezes.\n")
        .replace("## Scope\n", "## Scope\n\nOne sneeze.\n");
    fs::write(work.join(&path), text + "\n## Report\n\nIt sneezes.\n").unwrap();
    git(&work, &["commit", "-qam", "Sneeze"]);
    let branch = head(&work);

    ok(atelier(&root, "p1", &["start", "--backlog", "drop"]));
    let run = atelier(&root, "p1", &["drop", &id, "--reason", "Nobody asked"]);
    ok(run.clone());
    run_then(&root, &run.stdout);
    let text = read(&root, &path);
    assert_eq!(front(&text, "status"), "dropped");
    assert_eq!(front(&text, "implementers"), "t1");
    assert!(
        text.ends_with(
            "\n## Goal\n\nSneeze.\n\n## Done when\n\n- [x] It sneezes.\n\n\
             ## Scope\n\nOne sneeze.\n\n## Report\n\nDropped: Nobody asked\n"
        ),
        "{text}"
    );
    ok(atelier(&root, "p1", &["merge"]));
    assert_eq!(
        front(&git(&root, &["show", &format!("main:{path}")]), "status"),
        "dropped"
    );
    assert_eq!(head(&work), branch);
    let run = atelier(&work, "t1", &["submit"]);
    assert!(
        run.stderr
            .starts_with(&format!("{id} is dropped, and `atelier submit` runs on")),
        "{}",
        run.stderr
    );
}

#[test]
fn a_stale_clones_submit_and_merge_stop_at_a_drop_made_on_a_backlog_branch() {
    let hub = Hub::new(&[(ID, "ready")]);
    let b = hub.clone("b");
    hub.approved(&b, "t1");
    dropped(&hub);

    // The one that never fetched the drop: its merge conflicts on status.
    let base = hub.tip("main");
    let run = hub.atelier(&b, "t1", &["merge"]);
    assert_ne!(run.code, 0, "{}", run.stdout);
    assert_eq!(hub.tip("main"), base);

    // Its submit fetches the base, and opens no review.
    let before = head(&b);
    let run = hub.atelier(&b, "t1", &["submit"]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(
        run.stderr
            .starts_with(&format!("{ID} is dropped, and `atelier submit` runs on")),
        "{}",
        run.stderr
    );
    assert_eq!(head(&b), before);
}

#[test]
fn submit_refuses_an_item_done_or_dropped_on_the_remotes_base_and_warns_when_it_cannot_read_it() {
    for status in ["done", "dropped"] {
        let hub = Hub::new(&[(ID, "ready")]);
        let a = hub.clone("a");
        working(&hub, &a);
        finished(&a);
        let seed = hub.clone("seed-end");
        write_item(&seed, ID, status, &[], &[], "Ended.\n");
        git(&seed, &["commit", "-qam", "End it"]);
        git(&seed, &["push", "-q", "origin", "HEAD:main"]);
        let before = head(&a);
        let run = hub.atelier(&a, "t1", &["submit"]);
        assert_eq!(run.code, 1, "{}", run.stdout);
        assert!(
            run.stderr
                .starts_with(&format!("{ID} is {status}, and `atelier submit` runs on")),
            "{}",
            run.stderr
        );
        assert_eq!(head(&a), before);
        assert!(!hub.gh.join("opened").exists());
    }

    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    working(&hub, &a);
    finished(&a);
    cut_off(&hub, &a);
    let run = hub.atelier(&a, "t1", &["submit"]);
    ok(run.clone());
    assert!(
        run.stdout.contains(&format!(
            "Warning: fetching main from origin failed, so whether {ID} is done or dropped was judged by the last fetch: "
        )),
        "{}",
        run.stdout
    );
}

#[test]
fn a_stale_clone_cannot_restart_or_take_over_a_merged_or_dropped_item_whose_branch_is_gone() {
    for end in ["merged", "dropped"] {
        let hub = Hub::new(&[(ID, "ready")]);
        let (a, fresh, held) = (hub.clone("a"), hub.clone("fresh"), hub.clone("held"));
        working(&hub, &a);
        // `held` saw it building, and `fresh` only ready.
        git(&held, &["fetch", "-q", "origin"]);
        if end == "merged" {
            finished(&a);
            ok(hub.atelier(&a, "t1", &["submit"]));
            ok(hub.atelier(&a, "r1", &["review", ID]));
            let run = hub.atelier(&a, "r1", &["verdict", "approve"]);
            ok(run.clone());
            run_then(&a, &run.stdout);
            ok(hub.atelier(&a, "t1", &["merge"]));
        } else {
            dropped(&hub);
            // As the human or the forge may.
            git(
                &hub.remote,
                &["update-ref", "-d", &format!("refs/heads/{ID}")],
            );
        }
        assert_eq!(hub.tip(ID), None, "{end}");
        let status = if end == "merged" { "done" } else { "dropped" };
        for at in [&fresh, &held] {
            let before = head(at);
            let run = hub.atelier(at, "t2", &["start", ID]);
            assert_eq!(run.code, 1, "{end}: {}", run.stdout);
            assert!(
                run.stderr
                    .starts_with(&format!("{ID} is {status}, and `atelier start` runs on")),
                "{end}: {}",
                run.stderr
            );
            assert_eq!(hub.tip(ID), None, "{end}: a stale clone made it again");
            assert!(!branch_exists(at, ID));
            assert_eq!((head(at), current(at)), (before, String::new()));
        }
    }
}

#[test]
fn start_and_take_over_change_nothing_when_the_remotes_base_cannot_be_fetched() {
    let hub = Hub::new(&[(ID, "ready"), (OTHER, "ready")]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    let work = working(&hub, &a);
    git(&b, &["fetch", "-q", "origin"]);
    cut_off(&hub, &b);
    let before = head(&b);
    for id in [ID, OTHER] {
        let run = hub.atelier(&b, "t2", &["start", id]);
        assert_eq!(run.code, 1, "{}", run.stdout);
        assert!(
            run.stderr.starts_with(&format!(
                "`atelier start` refused; nothing changed.\norigin cannot be reached to fetch main, which says whether {id} is already done or dropped: "
            )),
            "{}",
            run.stderr
        );
        assert!(!branch_exists(&b, id));
        assert_eq!(head(&b), before);
    }
    assert_eq!(hub.tip(ID), Some(work));
    assert_eq!(hub.tip(OTHER), None);
}

#[test]
fn a_failed_fetch_is_no_failure_only_where_the_clone_already_holds_the_remotes_base() {
    // As when another fetch in the clone moved the tracking ref first.
    let hub = Hub::new(&[(ID, "ready"), (OTHER, "ready")]);
    let a = hub.clone("a");
    git(&a, &["config", "remote.origin.uploadpack", "false"]);
    ok(hub.atelier(&a, "t1", &["start", ID]));

    let b = hub.clone("b");
    fs::write(b.join("other.txt"), "other\n").unwrap();
    git(&b, &["add", "other.txt"]);
    git(&b, &["commit", "-qm", "Other"]);
    git(&b, &["push", "-q", "origin", "HEAD:main"]);
    let run = hub.atelier(&a, "t1", &["start", OTHER]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(
        run.stderr.contains("cannot be reached to fetch main"),
        "{}",
        run.stderr
    );
    assert_eq!(hub.tip(OTHER), None);
}

#[test]
fn a_fresh_claim_racing_a_merge_refuses_without_switching_and_keeps_the_pushed_branch() {
    // The base of a merged item holds a started status, which reads done.
    for (there, reads) in [("approved", "done"), ("dropped", "dropped")] {
        let hub = Hub::new(&[(ID, "ready")]);
        let a = hub.clone("a");
        let before = head(&a);
        base_moves_at_push(&hub, &a, there);
        let run = hub.atelier(&a, "t1", &["start", ID]);
        assert_eq!(run.code, 1, "{}", run.stdout);
        assert_eq!(
            run.stderr,
            format!(
                "`atelier start {ID}` refused: {ID} became {reads} on main while it started. The local branch is removed, and this checkout is as it was.\nWarning: the branch {ID} it pushed stays on origin, where it counts for nothing, since main's record wins.\n"
            )
        );
        assert_eq!((head(&a), current(&a)), (before, String::new()));
        assert!(!branch_exists(&a, ID));
        assert!(hub.tip(ID).is_some(), "only merge deletes on the remote");
        let shown = hub.atelier(&a, "t9", &["show", ID]).stdout;
        assert!(shown.contains(&format!("Status: {reads}\n")), "{shown}");
    }
}

#[test]
fn a_take_over_racing_a_drop_refuses_and_keeps_the_earlier_implementers_work() {
    for in_bases_checkout in [false, true] {
        let hub = Hub::new(&[(ID, "ready")]);
        let (a, b) = (hub.clone("a"), hub.clone("b"));
        let work = working(&hub, &a);
        git(&b, &["fetch", "-q", "origin"]);
        if in_bases_checkout {
            git(&b, &["switch", "-q", "main"]);
        }
        base_moves_at_push(&hub, &b, "dropped");
        let run = hub.atelier(&b, "t2", &["start", ID]);
        assert_eq!(run.code, 1, "{}", run.stdout);
        let here = match in_bases_checkout {
            true => String::new(),
            false => format!(" This checkout is on {ID}."),
        };
        assert_eq!(
            run.stderr,
            format!(
                "`atelier start {ID}` refused: {ID} became dropped on main while it was taken over, so nothing is left to do on it.\nBranch {ID} is kept here and on origin, with the work already on it.{here}\n"
            )
        );
        // Both refs keep the earlier work, and HEAD is valid.
        let pushed = hub.tip(ID).expect("the remote branch is kept");
        git(
            &hub.remote,
            &["merge-base", "--is-ancestor", &work, &pushed],
        );
        assert_eq!(git(&b, &["rev-parse", ID]), pushed);
        assert_eq!(git(&b, &["show", &format!("{ID}:hello.txt")]), "hello");
        let on = if in_bases_checkout { "main" } else { ID };
        assert_eq!(current(&b), on);
        git(&b, &["rev-parse", "--verify", "HEAD^{commit}"]);
    }
}

#[test]
fn a_claim_whose_check_after_the_push_fails_is_confirmed_by_running_start_again() {
    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    // The base moves with the claim's push, and the fetch of it then fails.
    let seed = hub.clone("seed-moves");
    fs::write(seed.join("other.txt"), "other\n").unwrap();
    git(&seed, &["add", "other.txt"]);
    git(&seed, &["commit", "-qm", "Other"]);
    git(&seed, &["push", "-q", "origin", "HEAD:refs/prepared/other"]);
    at_push(
        &a,
        &format!(
            "git --git-dir='{}' update-ref refs/heads/main {}\ngit config remote.origin.uploadpack false",
            hub.remote.display(),
            head(&seed)
        ),
    );
    let run = hub.atelier(&a, "t1", &["start", ID]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(
        run.stderr.starts_with(&format!(
            "`atelier start {ID}` pushed branch {ID}, then could not fetch main from origin"
        )),
        "{}",
        run.stderr
    );
    assert_eq!(current(&a), "");
    git(&a, &["config", "--unset", "remote.origin.uploadpack"]);
    let run = hub.atelier(&a, "t1", &["start", ID]);
    ok(run.clone());
    assert!(
        run.stdout.starts_with(&format!("You already hold {ID}")),
        "{}",
        run.stdout
    );
    assert_eq!(current(&a), ID);
}

#[test]
fn merge_deletes_the_merged_branch_on_the_remote_and_here_and_detaches_at_its_head() {
    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    hub.approved(&a, "t1");
    let merged = head(&a);
    let run = hub.atelier(&a, "t1", &["merge"]);
    ok(run.clone());
    assert!(
        run.stdout.ends_with(&format!(
            "Deleted branch {ID} on origin.\n{}",
            deleted(ID, &merged)
        )),
        "{}",
        run.stdout
    );
    assert_eq!((hub.tip(ID), tracking(&a, ID)), (None, None));
    assert!(!branch_exists(&a, ID));
    assert_eq!((current(&a), head(&a)), (String::new(), merged.clone()));
    assert_eq!(git(&a, &["status", "--porcelain"]), "");
    // Never `gh pr merge --delete-branch`, which switches the worktree.
    let calls = fs::read_to_string(hub.gh.join("calls")).unwrap();
    assert!(!calls.contains("--delete-branch"), "{calls}");
}

#[test]
fn a_backlog_and_a_mission_branch_are_deleted_once_merged() {
    let hub = Hub::new(&[(ID, "draft")]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    ok(hub.atelier(&a, "p1", &["start", "--backlog", "plan"]));
    let run = hub.atelier(&a, "p1", &["ready", ID, "--by", "ada"]);
    ok(run.clone());
    run_then(&a, &run.stdout);
    ok(hub.atelier(&b, "p2", &["start", "--mission", "Greet"]));
    let mission = current(&b);

    let cases = [(&a, "backlog/plan", "p1"), (&b, mission.as_str(), "p2")];
    for (at, branch, thread) in cases {
        let merged = head(at);
        // The plan's reviews are not what is tested.
        let run = skipping(&hub, at, thread, &["merge"]);
        ok(run.clone());
        assert!(
            run.stdout.ends_with(&format!(
                "Deleted branch {branch} on origin.\n{}",
                deleted(branch, &merged)
            )),
            "{}",
            run.stdout
        );
        assert_eq!((hub.tip(branch), tracking(at, branch)), (None, None));
        assert!(!branch_exists(at, branch));
        assert_eq!((current(at), head(at)), (String::new(), merged));
    }
    // The slug is free again.
    ok(hub.atelier(&a, "p1", &["start", "--backlog", "plan"]));
}

#[test]
fn a_failed_catch_up_leaves_both_refs_untouched_until_merge_runs_again() {
    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    hub.approved(&a, "t1");
    let merged = head(&a);
    after_merge(
        &hub,
        &format!(
            "git -C '{}' config remote.origin.uploadpack false\n",
            a.display()
        ),
    );
    let run = hub.atelier(&a, "t1", &["merge"]);
    ok(run.clone());
    assert!(
        run.stdout.ends_with(&format!(
            "Kept branch {ID} here and on origin, since main as this clone has it does not hold the merge yet. Fetch, then run `atelier merge` again to delete it.\n"
        )),
        "{}",
        run.stdout
    );
    assert_eq!(hub.tip(ID), Some(merged.clone()));
    assert_eq!(tracking(&a, ID), Some(merged.clone()));
    assert_eq!((current(&a), head(&a)), (ID.to_string(), merged.clone()));
    // So nobody is offered the merged item to start.
    let at = hub.dir.path().join("looking");
    git(
        &a,
        &[
            "worktree",
            "add",
            "-q",
            "--detach",
            at.to_str().unwrap(),
            "main",
        ],
    );
    let status = hub.atelier(&at, "t2", &["status"]).stdout;
    assert!(!status.contains(&format!("atelier start {ID}")), "{status}");

    git(&a, &["config", "--unset", "remote.origin.uploadpack"]);
    git(&a, &["fetch", "-q", "origin"]);
    let again = hub.atelier(&a, "t1", &["merge"]);
    ok(again.clone());
    assert_eq!(
        again.stdout,
        format!(
            "{ID} is already merged into main; nothing is left to do on it.\nDeleted branch {ID} on origin.\n{}",
            deleted(ID, &merged)
        )
    );
    assert_eq!(hub.tip(ID), None);
    assert!(!branch_exists(&a, ID));
}

#[test]
fn merge_run_again_keeps_a_merged_branch_someone_is_still_writing_on() {
    let hub = Hub::new(&[(ID, "draft")]);
    let a = hub.clone("a");
    ok(hub.atelier(&a, "p1", &["start", "--backlog", "plan"]));
    ok(hub.atelier(&a, "p1", &["ready", ID, "--by", "ada"]));
    // Not committed, so the branch holds nothing the base lacks.
    let run = hub.atelier(&a, "p1", &["merge"]);
    ok(run.clone());
    assert!(
        run.stdout.contains("Left branch backlog/plan checked out"),
        "{}",
        run.stdout
    );
    assert!(hub.tip("backlog/plan").is_some());
    assert_eq!(current(&a), "backlog/plan");
}

#[test]
fn a_late_push_refuses_the_remote_deletion_and_is_not_lost() {
    let hub = Hub::new(&[(ID, "ready")]);
    let (a, late) = (hub.clone("a"), hub.clone("late"));
    hub.approved(&a, "t1");
    let merged = head(&a);
    git(&a, &["push", "-q", "origin", ID]);
    git(&late, &["fetch", "-q", "origin"]);
    git(&late, &["checkout", "-q", "--detach", &merged]);
    fs::write(late.join("late.txt"), "late\n").unwrap();
    git(&late, &["add", "late.txt"]);
    git(&late, &["commit", "-qm", "Late"]);
    let pushed = head(&late);
    git(&late, &["push", "-q", "origin", "HEAD:refs/prepared/late"]);
    after_merge(
        &hub,
        &format!(
            "git --git-dir='{}' update-ref refs/heads/{ID} {pushed}\n",
            hub.remote.display()
        ),
    );
    let run = hub.atelier(&a, "t1", &["merge"]);
    ok(run.clone());
    assert!(
        run.stdout.contains(&format!(
            "Warning: branch {ID} stays on origin, which refused its deletion at the merged commit, as after a later push to it: "
        )),
        "{}",
        run.stdout
    );
    assert_eq!(hub.tip(ID), Some(pushed));
    // The local branch held the merged commit alone.
    assert!(!branch_exists(&a, ID));
}

#[test]
fn a_branch_the_remote_already_deleted_is_a_warning() {
    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    hub.approved(&a, "t1");
    after_merge(
        &hub,
        &format!(
            "git --git-dir='{}' update-ref -d refs/heads/{ID}\n",
            hub.remote.display()
        ),
    );
    let run = hub.atelier(&a, "t1", &["merge"]);
    ok(run.clone());
    assert!(
        run.stdout.contains(&format!(
            "Warning: origin had already deleted branch {ID}.\n"
        )),
        "{}",
        run.stdout
    );
    assert!(!branch_exists(&a, ID));
}

#[test]
fn changes_a_skip_left_uncommitted_keep_the_local_branch_and_not_the_remotes() {
    let hub = Hub::new(&[(ID, "ready")]);
    let a = hub.clone("a");
    hub.approved(&a, "t1");
    fs::write(a.join("hello.txt"), "hello?\n").unwrap();
    let run = skipping(&hub, &a, "t1", &["merge"]);
    ok(run.clone());
    assert!(
        run.stdout.contains(&format!(
            "Deleted branch {ID} on origin.\nLeft branch {ID} checked out"
        )),
        "{}",
        run.stdout
    );
    assert_eq!(hub.tip(ID), None);
    assert_eq!(current(&a), ID);
    assert_eq!(read(&a, "hello.txt"), "hello?\n");
}

#[test]
fn after_a_merge_planning_is_refused_and_a_commit_made_detached_is_not_orphaned() {
    let hub = Hub::new(&[(ID, "ready"), (OTHER, "draft")]);
    let a = hub.clone("a");
    hub.approved(&a, "t1");
    ok(hub.atelier(&a, "t1", &["merge"]));
    assert_eq!(current(&a), "");
    let status = hub.atelier(&a, "t1", &["status"]).stdout;
    assert!(
        !status.contains("Commits of this detached HEAD"),
        "{status}"
    );

    let finding = format!("{ID}/1-aaaa.1");
    let commands = [
        &["new", "Wave"][..],
        &["new", "Wave", "--from", &finding],
        &["set", OTHER, "--title", "Hi"],
        &["ready", OTHER, "--by", "ada"],
        &["drop", OTHER, "--reason", "Not needed"],
        &["drop", &finding, "--reason", "Not needed"],
        &["close", OTHER, "Not needed"],
        &["lead", "demo-zzzz"],
    ];
    for args in commands {
        let run = hub.atelier(&a, "t1", args);
        assert_eq!(run.code, 1, "{args:?}: {}", run.stdout);
        assert!(
            run.stderr.contains("refused; nothing written.\nThis checkout is on a detached HEAD, and a commit made there is on no branch, so it would never reach main.\n")
                && run.stderr.contains("`atelier start --backlog <slug>`")
                && run.stderr.contains("`atelier start <id>`")
                && run.stderr.ends_with(SKIP),
            "{args:?}: {}",
            run.stderr
        );
        assert_eq!(git(&a, &["status", "--porcelain", "-uall"]), "");
    }

    // A commit made by hand, with a harness's checkpoints at and past it.
    fs::write(a.join("later.txt"), "later\n").unwrap();
    git(&a, &["add", "later.txt"]);
    git(&a, &["commit", "-qm", "Later"]);
    let loose = head(&a);
    git(&a, &["update-ref", "refs/t3/checkpoints/at", &loose]);
    let tree = git(&a, &["rev-parse", "HEAD^{tree}"]);
    let past = git(
        &a,
        &["commit-tree", &tree, "-p", &loose, "-m", "Checkpoint"],
    );
    git(&a, &["update-ref", "refs/t3/checkpoints/past", &past]);

    let status = hub.atelier(&a, "t1", &["status"]).stdout;
    let listed = format!(
        "Commits of this detached HEAD on no branch, remote-tracking branch or tag, which a switch away would lose:\n  {} Later\n",
        git(&a, &["rev-parse", "--short", &loose])
    );
    assert!(status.contains(&listed), "{status}");
    let starts = [
        &["start", "--backlog", "next"][..],
        &["start", "--mission", "Next"],
        &["start", "--new", "Next"],
    ];
    for args in starts {
        let run = hub.atelier(&a, "t1", args);
        assert_eq!(run.code, 1, "{args:?}: {}", run.stdout);
        assert!(
            run.stderr.starts_with("`atelier start` refused; nothing changed.\nThis detached HEAD holds commits on no branch")
                && run.stderr.ends_with(SKIP),
            "{args:?}: {}",
            run.stderr
        );
        assert_eq!((current(&a), head(&a)), (String::new(), loose.clone()));
    }
    assert_eq!(hub.tip("backlog/next"), None);

    // A tag, a branch or a remote-tracking branch that holds it keeps it.
    for reference in [
        "refs/tags/kept",
        "refs/heads/kept",
        "refs/remotes/origin/kept",
    ] {
        git(&a, &["update-ref", reference, &loose]);
        let status = hub.atelier(&a, "t1", &["status"]).stdout;
        assert!(
            !status.contains("Commits of this detached HEAD"),
            "{reference}: {status}"
        );
        git(&a, &["update-ref", "-d", reference]);
    }
    // And on purpose it switches away.
    ok(skipping(&hub, &a, "t1", &["start", "--backlog", "next"]));
    assert_eq!(current(&a), "backlog/next");
}

#[test]
fn start_of_a_ready_or_started_item_does_not_switch_away_from_a_loose_commit() {
    let hub = Hub::new(&[(ID, "ready"), (OTHER, "ready")]);
    let (a, b) = (hub.clone("a"), hub.clone("b"));
    working(&hub, &a);
    git(&b, &["fetch", "-q", "origin"]);
    fs::write(b.join("loose.txt"), "loose\n").unwrap();
    git(&b, &["add", "loose.txt"]);
    git(&b, &["commit", "-qm", "Loose"]);
    let loose = head(&b);
    for id in [ID, OTHER] {
        let run = hub.atelier(&b, "t2", &["start", id]);
        assert_eq!(run.code, 1, "{}", run.stdout);
        assert!(
            run.stderr.contains("switching away would lose"),
            "{}",
            run.stderr
        );
        assert_eq!(head(&b), loose);
        assert!(!branch_exists(&b, id));
    }
}

#[test]
fn status_counts_merged_branches_and_detached_worktrees_at_their_thresholds_and_deletes_nothing() {
    let line = |merged: usize, detached: usize| {
        format!(
            "Piled up in this clone: {merged} local branches merged into main, {detached} detached worktrees. See them with `git branch --merged main` and `git worktree list`.\n"
        )
    };
    // Every merged branch counts, whatever its name; the fixture's is one.
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    for n in 2..20 {
        git(&root, &["branch", &format!("t3/merged-{n}"), "main"]);
    }
    let status = atelier(&root, "t9", &["status"]).stdout;
    assert!(!status.contains("Piled up"), "{status}");
    git(&root, &["branch", "t3/merged-20", "main"]);
    let status = atelier(&root, "t9", &["status"]).stdout;
    assert!(status.contains(&line(20, 0)), "{status}");
    // It only says: the command suggested deletes nothing.
    for word in [
        "branch -d",
        "branch -D",
        "worktree remove",
        "worktree prune",
        "--delete",
    ] {
        assert!(!status.contains(word), "{status}");
    }
    // An unmerged branch is not counted, and an item's branch says none.
    ok(atelier(&root, "t1", &["start", ID]));
    let status = atelier(&root, "t1", &["status"]).stdout;
    assert!(!status.contains("Piled up"), "{status}");

    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    for n in 1..10 {
        scratch.worktree(&format!("detached-{n}"));
    }
    let status = atelier(&root, "t9", &["status"]).stdout;
    assert!(!status.contains("Piled up"), "{status}");
    scratch.worktree("detached-10");
    let status = atelier(&root, "t9", &["status"]).stdout;
    assert!(status.contains(&line(1, 10)), "{status}");
}
