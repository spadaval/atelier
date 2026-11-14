//! `check`, and the three Git hooks that run it, driven by Git in scratch
//! repositories whose hook scripts the tests write.

mod common;

use common::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

const OTHER: &str = "demo-cd34";

/// Writes the three hook scripts into the clone of `root`, each running
/// the new binary.
fn install_hooks(root: &Path) {
    let dir = git(
        root,
        &["rev-parse", "--path-format=absolute", "--git-path", "hooks"],
    );
    fs::create_dir_all(&dir).unwrap();
    for hook in ["pre-commit", "pre-push", "post-checkout"] {
        let path = Path::new(&dir).join(hook);
        let script = format!(
            "#!/bin/sh\nexec '{}' check --hook {hook} \"$@\"\n",
            env!("CARGO_BIN_EXE_atelier")
        );
        fs::write(&path, script).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    }
}

/// Git run as thread t1 would run it, with what it printed, the hooks'
/// output included.
fn git_as(at: &Path, args: &[&str]) -> Run {
    git_skipping(at, None, args)
}

/// [`git_as`], with `ATELIER_SKIP` set to `skip` when it is given.
fn git_skipping(at: &Path, skip: Option<&str>, args: &[&str]) -> Run {
    let mut command = Command::new("git");
    match skip {
        Some(reason) => command.env("ATELIER_SKIP", reason),
        None => command.env_remove("ATELIER_SKIP"),
    };
    command.current_dir(at).args(args);
    for variable in [
        "CLAUDE_CODE_SESSION_ID",
        "CODEX_SESSION_ID",
        "GROK_SESSION_ID",
    ] {
        command.env_remove(variable);
    }
    command.env("T3CODE_THREAD_ID", "t1");
    let output = identity(&mut command).output().unwrap();
    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

/// Replaces `from` with `to` in the file at `path` from the root.
fn edit(root: &Path, path: &str, from: &str, to: &str) {
    let text = read(root, path);
    assert!(text.contains(from), "{path} has no {from:?}");
    fs::write(root.join(path), text.replacen(from, to, 1)).unwrap();
}

fn commit_all(root: &Path, message: &str) -> Run {
    git(root, &["add", "-A"]);
    git_as(root, &["commit", "-qm", message])
}

/// A scratch repository with the hooks in place, whose base is on a bare
/// remote `origin`, pushed before the hooks were.
fn pushed(items: &[(&str, &str)]) -> Scratch {
    let scratch = Scratch::new(items);
    let root = scratch.root();
    let remote = scratch.dir.path().join("remote.git");
    git(
        scratch.dir.path(),
        &["init", "-q", "--bare", remote.to_str().unwrap()],
    );
    git(
        &root,
        &["remote", "add", "origin", remote.to_str().unwrap()],
    );
    git(&root, &["push", "-q", "origin", "main"]);
    install_hooks(&root);
    scratch
}

fn remote_tip(root: &Path, branch: &str) -> String {
    git(
        root,
        &["ls-remote", "origin", &format!("refs/heads/{branch}")],
    )
}

#[test]
fn check_reports_every_problem_with_its_file_and_field_and_lists_uncommitted_records() {
    let scratch = Scratch::new(&[(ID, "ready"), (OTHER, "ready")]);
    let root = scratch.root();
    let run = atelier(&root, "t1", &["check"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        "Every record is valid: 2 items, 0 missions and config.toml.\n"
    );

    // One record that does not read, one that reads and is not valid, and
    // a config with an unknown key: each is named, with its field.
    edit(
        &root,
        &item_path(OTHER),
        "status: ready",
        "status: finished",
    );
    edit(
        &root,
        &item_path(ID),
        "blocked_by: []",
        "blocked_by: [demo-zz99]",
    );
    append(&root, ".atelier/config.toml", "colour = \"red\"\n");
    let run = atelier(&root, "t1", &["check"]);
    assert_eq!(run.code, 1);
    assert_eq!(run.stdout, "");
    let said = run.stderr;
    assert!(
        said.starts_with(
            "`atelier check` found 3 problems in 1 item, 0 missions and config.toml.\n"
        ),
        "{said}"
    );
    assert!(said.contains("These do not read as records, so no rule can be checked on them:\n"));
    assert!(said.contains("  .atelier/config.toml: line 4: unknown field `colour`"));
    assert!(said.contains(&format!(
        "  {}: front matter: status: unknown variant `finished`",
        item_path(OTHER)
    )));
    assert!(said.contains(&format!(
        "Rule: every record is valid. These are not:\n  {}: blocked_by: `demo-zz99` names no item\n",
        item_path(ID)
    )));
    // Uncommitted record changes are listed, and changing no outcome; one
    // record does not read, so the skip is not offered.
    assert!(said.ends_with(&format!(
        "Record changes not committed yet: .atelier/config.toml, {}, {}\n\
         Fix each in the file named, then run atelier check again.\n",
        item_path(ID),
        item_path(OTHER)
    )));
    // Every record read, what is not valid can be skipped, and is still
    // reported.
    edit(
        &root,
        &item_path(OTHER),
        "status: finished",
        "status: ready",
    );
    git(&root, &["checkout", "--", ".atelier/config.toml"]);
    let run = atelier(&root, "t1", &["check"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.ends_with(&format!("check again.\n{SKIP}")),
        "{}",
        run.stderr
    );
    let run = atelier_command(&root, "t1", &["check"])
        .env("ATELIER_SKIP", "the blocker comes next")
        .output()
        .unwrap();
    assert_eq!(run.status.code(), Some(0));
    let said = String::from_utf8(run.stdout).unwrap();
    assert!(
        said.starts_with(&format!(
            "Skipped with ATELIER_SKIP: Every record is valid ({}: blocked_by: `demo-zz99` names no item): the blocker comes next\n`atelier check` found 1 problem in",
            item_path(ID)
        )),
        "{said}"
    );

    git(&root, &["checkout", "--", "."]);
    edit(&root, &item_path(ID), "title: Say hello", "title: Say hi");
    append(
        &root,
        ".atelier/config.toml",
        "\n[[merge_check]]\nrun = \"true\"\n",
    );
    let run = atelier(&root, "t1", &["check"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        format!(
            "Every record is valid: 2 items, 0 missions and config.toml.\n\
             Record changes not committed yet: .atelier/config.toml, {}\n",
            item_path(ID)
        )
    );

    // A record Git ignores is still read by the commands, so it is checked.
    fs::write(
        root.join(".git/info/exclude"),
        ".atelier/items/demo-ig00/\n",
    )
    .unwrap();
    let ignored = ".atelier/items/demo-ig00/item.md";
    fs::create_dir_all(root.join(ignored).parent().unwrap()).unwrap();
    fs::write(root.join(ignored), "no front matter\n").unwrap();
    let run = atelier(&root, "t1", &["check"]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(
        run.stderr.contains(&format!(
            "  {ignored}: front matter: the file does not start with front matter"
        )),
        "{}",
        run.stderr
    );
}

#[test]
fn a_commit_of_an_invalid_record_or_of_a_status_no_act_sets_is_refused_and_a_hand_edit_goes_through(
) {
    let scratch = Scratch::new(&[(ID, "ready"), (OTHER, "ready")]);
    let root = scratch.root();
    install_hooks(&root);
    let head = git(&root, &["rev-parse", "HEAD"]);
    // Refused, and with `rule` the skip offered after the way out; then,
    // with ATELIER_SKIP set, committed as a rule skipped, saying `rule`, or
    // refused again as no rule.
    let way_out = "Fix each file named, stage it with git add, and commit again.\n";
    let refused = |message: &str, lines: &[&str], rule: Option<&str>| {
        let run = commit_all(&root, message);
        assert_ne!(run.code, 0, "{}", run.stderr);
        assert!(run.stderr.starts_with(
            "Atelier's pre-commit hook refused this commit; nothing was committed.\n"
        ));
        for line in lines {
            assert!(run.stderr.contains(line), "{line}\n{}", run.stderr);
        }
        let end = match rule {
            Some(_) => format!("{way_out}{SKIP}"),
            None => way_out.to_string(),
        };
        assert!(run.stderr.ends_with(&end), "{}", run.stderr);
        assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
        let run = git_skipping(
            &root,
            Some("  Sumeet\n said so "),
            &["commit", "-qm", message],
        );
        match rule {
            Some(rule) => {
                assert_eq!(run.code, 0, "{}", run.stderr);
                // Git shows a hook's output on its own stderr.
                let said = format!("Skipped with ATELIER_SKIP: {rule}: Sumeet said so\n");
                assert_eq!(run.stderr, said);
                // A hook writes nothing: the message is Git's.
                assert_eq!(git(&root, &["log", "-1", "--format=%B"]), message);
                git(&root, &["reset", "-q", "--hard", &head]);
            }
            None => {
                assert_ne!(run.code, 0);
                assert!(run.stderr.ends_with(NOT_SKIPPABLE), "{}", run.stderr);
                assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
            }
        }
        git(&root, &["reset", "-q", "--hard"]);
    };

    edit(
        &root,
        &item_path(ID),
        "blocked_by: []",
        "blocked_by: [demo-zz99]",
    );
    let rule = format!(
        "Rule: every record is valid. These are not:\n  {}: blocked_by: `demo-zz99` names no item\n",
        item_path(ID)
    );
    let valid = format!(
        "Every record is valid ({}: blocked_by: `demo-zz99` names no item)",
        item_path(ID)
    );
    refused("Block it", &[&rule], Some(&valid));

    edit(&root, &item_path(ID), "status: ready", "status: finished");
    let parse = format!(
        "These do not read as records, so no rule can be checked on them:\n  {}: front matter: status: unknown variant `finished`",
        item_path(ID)
    );
    refused("Finish it", &[&parse], None);

    edit(&root, &item_path(ID), "status: ready", "status: approved");
    let act = format!(
        "Rule: a status changes only as an act of the lifecycle changes it. These change it as no act does:\n  {}: status: ready to approved. Put back the status HEAD has; atelier status names the command for what comes next.\n",
        item_path(ID)
    );
    let status = format!(
        "A status changes only as an act of the lifecycle changes it ({}: ready to approved)",
        item_path(ID)
    );
    refused("Approve it", &[&act], Some(&status));

    // A record that does not read, beside a rule broken, is not skipped.
    edit(&root, &item_path(ID), "status: ready", "status: finished");
    edit(
        &root,
        &item_path(OTHER),
        "blocked_by: []",
        "blocked_by: [demo-zz99]",
    );
    refused("Both", &[&parse, "Rule: every record is valid."], None);

    // A planner's own edit, a status no command would refuse included.
    edit(&root, &item_path(ID), "title: Say hello", "title: Say hi");
    edit(&root, &item_path(OTHER), "status: ready", "status: dropped");
    append(&root, &item_path(OTHER), "Dropped: not needed\n");
    let run = commit_all(&root, "Plan");
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(field(&root, "HEAD", "title"), "Say hi");

    // A record deleted leaves nothing to judge.
    git(&root, &["rm", "-rq", &format!(".atelier/items/{OTHER}")]);
    let run = git_as(&root, &["commit", "-qm", "Forget it"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
}

#[test]
fn a_rule_a_hook_skips_in_an_atelier_commit_is_printed() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    install_hooks(&root);
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    let path = root.join(item_path(ID));
    let text = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    let text = text.replace("blocked_by: []", "blocked_by: [demo-zz99]");
    fs::write(&path, text + "Done.\n").unwrap();
    // The pre-commit hook refuses `submit`'s commit; with a skip it passes
    // it, and `submit` says so.
    assert_eq!(atelier(&root, "t1", &["submit"]).code, 2);
    let run = atelier_command(&root, "t1", &["submit"])
        .env("ATELIER_SKIP", "the blocker comes next")
        .output()
        .unwrap();
    let stderr = String::from_utf8(run.stderr).unwrap();
    assert_eq!(run.status.code(), Some(0), "{stderr}");
    assert_eq!(
        stderr,
        format!(
            "Skipped with ATELIER_SKIP: Every record is valid ({}: blocked_by: `demo-zz99` names no item): the blocker comes next\n",
            item_path(ID)
        )
    );
    assert_eq!(field(&root, "HEAD", "status"), "in_review");
}

#[test]
fn a_status_change_is_judged_by_the_acts_of_its_records_kind() {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    // `start --mission` commits the mission on its branch.
    let run = atelier(&root, "t1", &["start", "--mission", "Greet"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let id = run.stdout.split_whitespace().nth(3).unwrap();
    let path = format!(".atelier/missions/{id}/mission.md");
    // An old `draft`, as a mission was planned before.
    edit(&root, &path, "status: open", "status: draft");
    assert_eq!(commit_all(&root, "Plan it as before").code, 0);
    install_hooks(&root);
    // An item's `close` goes from draft to done; no act on a mission does.
    edit(&root, &path, "status: draft", "status: done");
    let run = commit_all(&root, "Close it");
    assert_eq!(run.code, 1, "{}", run.stderr);
    assert!(
        run.stderr
            .contains(&format!("  {path}: status: draft to done.")),
        "{}",
        run.stderr
    );
    git(&root, &["reset", "-q", "--hard"]);
    // `submit` writes an old draft open, and `close` ends an open one.
    edit(&root, &path, "status: draft", "status: open");
    assert_eq!(commit_all(&root, "Submit it").code, 0);
    edit(&root, &path, "status: open", "status: done");
    let run = commit_all(&root, "Close it");
    assert_eq!(run.code, 0, "{}", run.stderr);
}

#[test]
fn the_pre_commit_hook_reads_records_as_staged_and_passes_over_other_files() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    install_hooks(&root);

    // A mission's planning document has no front matter, and an item's
    // artifact is not text: neither is read.
    let mission = ".atelier/missions/demo-mm11";
    fs::create_dir_all(root.join(mission)).unwrap();
    fs::write(
        root.join(mission).join("mission.md"),
        "---\nschema: atelier.mission\nschema_version: 3\nid: demo-mm11\ntitle: Greet\n\
         status: draft\nlabels: []\nlead: null\nready: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Outcome\n\nGreetings.\n",
    )
    .unwrap();
    fs::write(root.join(mission).join("notes.md"), "Just notes.\n").unwrap();
    let artifacts = root.join(".atelier/items").join(ID).join("artifacts");
    fs::create_dir_all(&artifacts).unwrap();
    fs::write(
        artifacts.join("shot.png"),
        [0x89, b'P', b'N', b'G', 0xff, 0xfe],
    )
    .unwrap();
    let run = commit_all(&root, "Plan a mission");
    assert_eq!(run.code, 0, "{}", run.stderr);

    // Broken as staged, and fixed only in the working tree: refused.
    edit(&root, &item_path(ID), "status: ready", "status: finished");
    git(&root, &["add", "-A"]);
    edit(&root, &item_path(ID), "status: finished", "status: ready");
    let run = git_as(&root, &["commit", "-qm", "Finish it"]);
    assert_ne!(run.code, 0);
    assert!(
        run.stderr.contains("unknown variant `finished`"),
        "{}",
        run.stderr
    );

    // A record renamed is judged at its new path, whose name is its ID's.
    git(&root, &["reset", "-q", "--hard"]);
    let renamed = ".atelier/items/demo-zz11";
    git(&root, &["mv", &format!(".atelier/items/{ID}"), renamed]);
    let run = git_as(&root, &["commit", "-qm", "Rename it"]);
    assert_ne!(run.code, 0);
    assert!(
        run.stderr.contains(&format!(
            "  {renamed}/item.md: id: `{ID}` is not its directory's name\n"
        )),
        "{}",
        run.stderr
    );
    git(&root, &["reset", "-q", "--hard"]);

    // A commit in a second worktree is judged by that worktree's index.
    let other = scratch.worktree("other");
    edit(&other, &item_path(ID), "status: ready", "status: finished");
    let run = commit_all(&other, "Finish it there");
    assert_ne!(run.code, 0);
    assert!(
        run.stderr.contains("unknown variant `finished`"),
        "{}",
        run.stderr
    );

    // A review whose item is missing, or does not read, cannot be
    // validated, so it is not committed.
    git(&root, &["reset", "-q", "--hard"]);
    let review = "---\nschema: atelier.review\nschema_version: 3\nround: 2\ncommit: 0123abc\n\
                  reviewers:\n- rev\nmodel: null\nverdict: null\n\
                  created_at: 2026-11-10T09:00:00Z\n---\n\n## Summary\n\nFine.\n";
    let orphan = ".atelier/items/demo-zz11/reviews/1-abcd.md";
    fs::create_dir_all(root.join(orphan).parent().unwrap()).unwrap();
    fs::write(root.join(orphan), review).unwrap();
    let run = commit_all(&root, "Review nothing");
    assert_ne!(run.code, 0);
    assert!(
        run.stderr.contains(&format!(
            "  {orphan}: .atelier/items/demo-zz11/item.md is missing\n"
        )),
        "{}",
        run.stderr
    );
    git(&root, &["reset", "-q", "--hard"]);
    edit(&root, &item_path(ID), "status: ready", "status: finished");
    git(&root, &["commit", "-qam", "Break it", "--no-verify"]);
    let reviewed = format!(".atelier/items/{ID}/reviews/1-abcd.md");
    fs::create_dir_all(root.join(&reviewed).parent().unwrap()).unwrap();
    fs::write(root.join(&reviewed), review).unwrap();
    let run = commit_all(&root, "Review a broken item");
    assert_ne!(run.code, 0);
    assert!(
        run.stderr.contains(&format!(
            "  {reviewed}: not validated, since {} does not read\n",
            item_path(ID)
        )),
        "{}",
        run.stderr
    );
    git(&root, &["reset", "-q", "--hard", "HEAD^"]);

    // Valid as staged, and broken only in the working tree: committed.
    git(&root, &["add", "-A"]);
    edit(&root, &item_path(ID), "title: Say hello", "title: Say hi");
    git(&root, &["add", "-A"]);
    edit(&root, &item_path(ID), "status: ready", "status: finished");
    let run = git_as(&root, &["commit", "-qm", "Retitle it"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(field(&root, "HEAD", "title"), "Say hi");
}

#[test]
fn the_first_commit_of_a_repository_is_checked() {
    let scratch = Scratch::new(&[]);
    let root = scratch.dir.path().join("fresh");
    git(&root, &["init", "-q", "-b", "trunk"]);
    install_hooks(&root);
    fs::create_dir_all(root.join(".atelier")).unwrap();
    fs::write(
        root.join(".atelier/config.toml"),
        "schema_version = 3\nslug = \"demo\"\nbase_branch = \"trunk\"\nbase = 1\n",
    )
    .unwrap();
    let run = commit_all(&root, "Start");
    assert_ne!(run.code, 0);
    assert!(run
        .stderr
        .contains("  .atelier/config.toml: line 4: unknown field `base`"));
    assert!(git_output(&root, &["rev-parse", "--verify", "HEAD"])
        .status
        .code()
        .is_some_and(|code| code != 0));

    edit(&root, ".atelier/config.toml", "base = 1\n", "");
    write_item(&root, ID, "draft", &[], &[], "");
    // Valid, and on the base, so the base's rule refuses it; the first
    // commit of a repository is the one place a skip is plainly right.
    let run = commit_all(&root, "Start");
    assert_eq!(run.code, 1, "{}", run.stderr);
    assert!(
        run.stderr
            .contains("Rule: only a merge writes the base branch trunk"),
        "{}",
        run.stderr
    );
    let run = git_skipping(
        &root,
        Some("the repository's first commit"),
        &["commit", "-qm", "Start"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
}

#[test]
fn a_push_of_the_base_with_a_commit_that_is_no_atelier_merge_is_refused() {
    let scratch = pushed(&[(ID, "ready")]);
    let root = scratch.root();
    let pushed = remote_tip(&root, "main");

    git(&root, &["switch", "-q", "main"]);
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    assert_eq!(commit_all(&root, "Say hello").code, 0);
    let commit = git(&root, &["rev-parse", "--short=7", "HEAD"]);
    let run = git_as(&root, &["push", "-q", "origin", "main"]);
    assert_ne!(run.code, 0);
    assert!(
        run.stderr.starts_with(
            "Atelier's pre-push hook refused this push; nothing was pushed.\n\
             Rule: main receives only merges made by atelier merge. Each new commit on main's first-parent line is one. These are not:\n"
        ),
        "{}",
        run.stderr
    );
    assert!(run
        .stderr
        .contains(&format!("  {commit} Say hello: not an Atelier merge\n")));
    assert!(run.stderr.contains(
        &format!("Move them to a branch: atelier start --new \"<title>\" starts an item that holds code, atelier start --backlog <slug> a branch for record changes. Then git branch -f main {} takes them off main, and atelier merge brings the branch in.\n", &pushed[..7])
    ), "{}", run.stderr);
    assert_eq!(remote_tip(&root, "main"), pushed);

    // Pushed under another name, it is no push of the base.
    let run = git_as(&root, &["push", "-q", "origin", "main:spike"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    // And another branch pushed to the base is.
    git(&root, &["branch", "-q", "feature"]);
    git(&root, &["reset", "-q", "--hard", "HEAD^"]);
    let run = git_as(&root, &["push", "-q", "origin", "feature:main"]);
    assert_ne!(run.code, 0);
    assert!(run.stderr.contains("Say hello: not an Atelier merge"));

    // A merge that is not Atelier's is refused, and so is a commit that is
    // no merge but carries the trailer.
    let run = git_as(
        &root,
        &["merge", "-q", "--no-ff", "feature", "-m", "Merge feature"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    let run = git_as(&root, &["push", "-q", "origin", "main"]);
    assert_ne!(run.code, 0);
    assert!(
        run.stderr.contains("Merge feature: not an Atelier merge"),
        "{}",
        run.stderr
    );
    git(&root, &["reset", "-q", "--hard", "HEAD^"]);
    fs::write(root.join("hello.txt"), "hi\n").unwrap();
    assert_eq!(
        commit_all(&root, "Say hi\n\nAtelier-Merge: demo-ab12").code,
        0
    );
    let run = git_as(&root, &["push", "-q", "origin", "main"]);
    assert_ne!(run.code, 0);
    assert!(
        run.stderr.contains("Say hi: not an Atelier merge"),
        "{}",
        run.stderr
    );
    git(&root, &["reset", "-q", "--hard", "HEAD^"]);

    append(
        &root,
        ".atelier/config.toml",
        "\n[[merge_check]]\nrun = \"true\"\n",
    );
    // On the base the pre-commit hook refuses it, and a skip is what gets
    // it as far as the push.
    git(&root, &["add", "-A"]);
    let run = git_skipping(
        &root,
        Some("testing the push"),
        &["commit", "-qm", "Check it"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    let run = git_as(&root, &["push", "-q", "origin", "main"]);
    assert_ne!(run.code, 0);
    assert!(
        run.stderr.contains("Check it: not an Atelier merge\n"),
        "{}",
        run.stderr
    );
    // A force push past the base's tip is judged the same way.
    let run = git_as(&root, &["push", "-q", "--force", "origin", "main"]);
    assert_ne!(run.code, 0);
    assert!(
        run.stderr
            .contains(&format!("brings the branch in.\n{SKIP}")),
        "{}",
        run.stderr
    );
    assert_eq!(remote_tip(&root, "main"), pushed);
    // Pushed with a skip, it goes through and says what it passed, in a
    // second worktree too, where Git runs the same hook.
    let commit = git(&root, &["rev-parse", "--short=7", "HEAD"]);
    git(&root, &["branch", "-q", "skipped"]);
    let second = scratch.worktree("second");
    git(&second, &["switch", "-q", "skipped"]);
    let run = git_skipping(
        &second,
        Some("CI is down"),
        &["push", "-q", "origin", "skipped:main"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout + &run.stderr,
        format!(
            "Skipped with ATELIER_SKIP: main receives only merges made by atelier merge ({commit} Check it is not one): CI is down\n"
        )
    );
    assert!(remote_tip(&root, "main").starts_with(&git(&root, &["rev-parse", "HEAD"])));
    let back = format!("{}:main", &pushed[..40]);
    git(
        &root,
        &["push", "-q", "--no-verify", "--force", "origin", &back],
    );

    // The way out it names: the commits onto an item's branch, the base
    // back to the remote's commit, and the push goes through.
    let run = atelier(&root, "t1", &["start", "--new", "Move the check"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    // Run in the base's checkout, it makes the branch and leaves the
    // checkout on the base.
    let moved = run.stdout.split_whitespace().nth(1).unwrap().to_string();
    assert_eq!(git(&root, &["symbolic-ref", "--short", "HEAD"]), "main");
    assert!(run.stdout.contains("git worktree add "), "{}", run.stdout);
    git(&root, &["reset", "-q", "--keep", &pushed[..7]]);
    let run = git_as(&root, &["push", "-q", "origin", "main", &moved]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(remote_tip(&root, "main"), pushed);
    let held = git(&root, &["log", "--format=%s", &moved]);
    assert!(held.contains("Check it\n"), "{held}");
}

#[test]
fn a_push_from_a_checkout_whose_config_does_not_read_is_refused_with_the_way_out() {
    let scratch = pushed(&[(ID, "ready")]);
    let root = scratch.root();
    append(&root, ".atelier/config.toml", "base = 1\n");
    let run = git_as(&root, &["push", "-q", "origin", "main"]);
    assert_ne!(run.code, 0);
    assert!(
        run.stderr.starts_with(
            "Atelier's pre-push hook refused this push; nothing was pushed.\n\
             These do not read as records, so no rule can be checked on them:\n  \
             .atelier/config.toml: line 4: unknown field `base`"
        ) && run
            .stderr
            .contains("Fix it in this checkout, then push again.\n"),
        "{}",
        run.stderr
    );
}

#[test]
fn a_push_of_several_refs_judges_only_the_base() {
    let scratch = pushed(&[(ID, "draft")]);
    let root = scratch.root();
    // Code on another branch, and the base as the remote has it: the base
    // is judged, and moves nothing.
    git(&root, &["switch", "-q", "-c", "spike"]);
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    assert_eq!(commit_all(&root, "Try hello").code, 0);
    git(&root, &["checkout", "-q", "--detach", "main"]);
    edit(&root, &item_path(ID), "title: Say hello", "title: Say hi");
    assert_eq!(commit_all(&root, "Retitle").code, 0);
    let run = git_as(&root, &["push", "-q", "origin", "main", "spike"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    // Code pushed to the base beside another ref refuses the whole push.
    git(&root, &["switch", "-q", "-c", "louder"]);
    fs::write(root.join("hello.txt"), "hello!\n").unwrap();
    assert_eq!(commit_all(&root, "Say it louder").code, 0);
    let run = git_as(
        &root,
        &["push", "-q", "origin", "louder:main", "main:other"],
    );
    assert_ne!(run.code, 0);
    assert!(
        run.stderr.contains("Say it louder: not an Atelier merge"),
        "{}",
        run.stderr
    );
    assert!(remote_tip(&root, "other").is_empty());
    // Deleting a branch adds nothing to the base.
    let run = git_as(&root, &["push", "-q", "origin", "--delete", "spike"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
}

#[test]
fn an_atelier_merge_of_a_branch_with_code_commits_reaches_the_base_and_its_branch_any_time() {
    let scratch = pushed(&[(ID, "ready")]);
    let root = scratch.root();
    // Every commit here goes through the pre-commit hook: the acts' too.
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    assert_eq!(commit_all(&root, "Say hello").code, 0);
    fs::write(root.join("hello.txt"), "hello!\n").unwrap();
    assert_eq!(commit_all(&root, "Say it louder").code, 0);
    let run = git_as(&root, &["push", "-q", "origin", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    finish_and_submit(&root, "t1");
    assert_eq!(atelier(&root, "t2", &["review", ID]).code, 0);
    let run = verdict(&root, "t2", &["verdict", "approve"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let run = atelier(&root, "t1", &["merge"]);
    assert_eq!(run.code, 0, "{}", run.stderr);

    // Planning on either side of the merge, pushed with it.
    git(&root, &["checkout", "-q", "--detach", "main"]);
    let run = atelier(&root, "t1", &["new", "Say goodbye"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(commit_all(&root, "Plan goodbye").code, 0);
    let run = git_as(&root, &["push", "-q", "origin", "main"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(remote_tip(&root, "main").starts_with(&git(&root, &["rev-parse", "main"])));
}

#[test]
fn a_plain_commit_of_records_alone_on_the_base_is_refused() {
    let scratch = pushed(&[(ID, "ready")]);
    let root = scratch.root();
    let pushed = remote_tip(&root, "main");
    git(&root, &["switch", "-q", "main"]);
    // On the base the commit needs a skip of the pre-commit hook's rule.
    edit(&root, &item_path(ID), "status: ready", "status: done");
    git(&root, &["add", "-A"]);
    let run = git_skipping(
        &root,
        Some("testing the push"),
        &["commit", "-qm", "Call it done"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    let commit = git(&root, &["rev-parse", "--short=7", "HEAD"]);
    let run = git_as(&root, &["push", "-q", "origin", "main"]);
    assert_ne!(run.code, 0);
    assert!(
        run.stderr
            .contains(&format!("  {commit} Call it done: not an Atelier merge\n")),
        "{}",
        run.stderr
    );
    // The way out it names.
    git(&root, &["reset", "-q", "--keep", "HEAD^"]);
    let run = git_as(&root, &["push", "-q", "origin", "main"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(remote_tip(&root, "main"), pushed);
}

#[test]
fn with_a_forge_any_push_that_moves_the_base_is_refused_and_names_merge() {
    let scratch = pushed(&[(ID, "ready")]);
    let root = scratch.root();
    let pushed = remote_tip(&root, "main");
    git(&root, &["switch", "-q", "main"]);
    append(
        &root,
        ".atelier/config.toml",
        "\n[forge]\nprovider = \"github\"\nremote = \"origin\"\n",
    );
    git(&root, &["add", "-A"]);
    let run = git_skipping(&root, Some("set up"), &["commit", "-qm", "Name the forge"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    // Even a merge with the trailer: the forge makes every merge.
    git(&root, &["switch", "-q", "-c", "side", "HEAD^"]);
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    assert_eq!(commit_all(&root, "Say hello").code, 0);
    git(&root, &["switch", "-q", "main"]);
    git(
        &root,
        &[
            "merge",
            "-q",
            "--no-ff",
            "side",
            "-m",
            "Merge side\n\nAtelier-Merge: side",
        ],
    );
    let run = git_as(&root, &["push", "-q", "origin", "main"]);
    assert_ne!(run.code, 0);
    assert_eq!(
        run.stderr,
        format!(
            "Atelier's pre-push hook refused this push; nothing was pushed.\n\
             Rule: main is moved only by the forge's merge of a pull request. With a forge configured, atelier merge has the forge merge each branch's pull request and never pushes main, so no push may move it.\n\
             Put the commits on a branch and run atelier merge there; `git branch -f main {}` puts main back.\n{SKIP}",
            &pushed[..7]
        )
        + "error: failed to push some refs to '"
        + &format!("{}'\n", scratch.dir.path().join("remote.git").display())
    );
    assert_eq!(remote_tip(&root, "main"), pushed);
    // A branch is pushed as before, and so is the base's own commit.
    let run = git_as(&root, &["push", "-q", "origin", "side", "HEAD~2:main"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    // A skip passes it, and says so.
    let run = git_skipping(&root, Some("bootstrap"), &["push", "-q", "origin", "main"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout + &run.stderr,
        "Skipped with ATELIER_SKIP: main is moved only by the forge's merge of a pull request, through atelier merge: bootstrap\n"
    );
}

#[test]
fn a_first_push_of_a_base_judges_its_whole_line() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    install_hooks(&root);
    let remote = scratch.dir.path().join("remote.git");
    git(
        scratch.dir.path(),
        &["init", "-q", "--bare", remote.to_str().unwrap()],
    );
    git(
        &root,
        &["remote", "add", "origin", remote.to_str().unwrap()],
    );
    // Its first commit is no merge.
    let run = git_as(&root, &["push", "-q", "origin", "main"]);
    assert_ne!(run.code, 0);
    assert!(run.stderr.contains("Plan: not an Atelier merge"));
    // With no commit of the remote's to name, the way out names none.
    assert!(
        run.stderr
            .contains("Then moving main back to the remote's commit takes them off main,"),
        "{}",
        run.stderr
    );
}

#[test]
fn a_branch_switch_prints_status_and_succeeds_whatever_it_finds() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    install_hooks(&root);
    let run = git_as(&root, &["switch", "-c", "spike"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let said = run.stdout + &run.stderr;
    assert!(
        said.contains("You are thread t1, on branch spike, which is no item's.\n"),
        "{said}"
    );

    // A branch named for the item, holding a record that does not read,
    // committed past the hook.
    git(&root, &["switch", "-q", "-c", ID]);
    edit(&root, &item_path(ID), "status: ready", "status: finished");
    git(&root, &["commit", "-qam", "Break it", "--no-verify"]);
    let run = git_as(&root, &["switch", "main"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let run = git_as(&root, &["switch", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(git(&root, &["symbolic-ref", "--short", "HEAD"]), ID);
    let said = run.stdout + &run.stderr;
    assert!(
        said.contains("Switched; `atelier status` cannot say more here: ")
            && said.contains("unknown variant `finished`"),
        "{said}"
    );

    // A blank skip refuses a commit and a push, and only shows on a switch,
    // which nothing may fail.
    let blank = "ATELIER_SKIP is set and blank, and a skip needs a reason: ATELIER_SKIP=\"<why>\".";
    let run = git_skipping(&root, Some(" "), &["switch", "main"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let said = run.stdout + &run.stderr;
    assert!(said.contains(&format!("{blank}\nSwitched; ")), "{said}");
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    git(&root, &["add", "hello.txt"]);
    let run = git_skipping(&root, Some(""), &["commit", "-qm", "Hello"]);
    assert_eq!(
        (run.code, run.stderr),
        (1, format!("{blank} Nothing changed.\n"))
    );
    git(&root, &["commit", "-qm", "Hello", "--no-verify"]);
    let remote = scratch.dir.path().join("remote.git");
    git(&root, &["init", "-q", "--bare", remote.to_str().unwrap()]);
    let run = git_skipping(
        &root,
        Some(""),
        &["push", "-q", remote.to_str().unwrap(), "main"],
    );
    assert_eq!(
        (run.code, run.stderr),
        (
            1,
            format!(
                "{blank} Nothing changed.\nerror: failed to push some refs to '{}'\n",
                remote.display()
            )
        )
    );
    git(&root, &["switch", "-q", ID]);

    // A file checked out is no switch, and prints nothing.
    fs::write(root.join(item_path(ID)), "changed").unwrap();
    let run = git_as(&root, &["checkout", "--", &item_path(ID)]);
    assert_eq!(run.code, 0);
    assert_eq!(run.stdout + &run.stderr, "");
}

#[test]
fn a_commit_on_the_base_that_changes_a_record_is_refused_and_a_skip_passes_it() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    install_hooks(&root);
    git(&root, &["switch", "-q", "main"]);
    // Code alone on the base is the pre-push hook's to judge, not this one's.
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    assert_eq!(commit_all(&root, "Say hello").code, 0);

    let before = git(&root, &["rev-parse", "HEAD"]);
    for change in ["change", "add", "delete"] {
        match change {
            "change" => edit(&root, &item_path(ID), "status: draft", "status: ready"),
            "add" => write_item(&root, "demo-n3w1", "draft", &[], &[], ""),
            _ => fs::remove_dir_all(root.join(".atelier/items").join(ID)).unwrap(),
        }
        let run = commit_all(&root, "Plan on the base");
        assert_eq!(run.code, 1, "{change}: {}", run.stderr);
        assert!(
            run.stderr.contains(
                "Rule: only a merge writes the base branch main, so no record changes in a commit on it. This commit changes:\n"
            ) && run.stderr.contains("`atelier start --backlog <slug>`")
                && run.stderr.contains("`atelier start --mission \"<title>\"`")
                && run.stderr.ends_with(SKIP),
            "{change}: {}",
            run.stderr
        );
        assert_eq!(git(&root, &["rev-parse", "HEAD"]), before);
        git(&root, &["reset", "-q", "--hard"]);
        git(&root, &["clean", "-qfd"]);
    }

    // A skip passes it, with its reason.
    edit(&root, &item_path(ID), "status: draft", "status: ready");
    git(&root, &["add", "-A"]);
    let run = git_skipping(&root, Some("the cutover"), &["commit", "-qm", "Ready"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stderr
            .contains("Skipped with ATELIER_SKIP: Only a merge writes the base branch main"),
        "{}",
        run.stderr
    );

    // Off the base, detached or on a branch, the same change commits.
    git(&root, &["checkout", "-q", "--detach"]);
    edit(&root, &item_path(ID), "status: ready", "status: draft");
    let run = commit_all(&root, "Back to draft");
    assert_ne!(run.code, 0, "ready to draft is no act");
    git(&root, &["reset", "-q", "--hard"]);
    write_item(&root, "demo-n3w1", "draft", &[], &[], "");
    assert_eq!(commit_all(&root, "Plan detached").code, 0);
    git(&root, &["switch", "-q", "-c", "backlog/plan"]);
    write_item(&root, "demo-n3w2", "draft", &[], &[], "");
    assert_eq!(commit_all(&root, "Plan on a branch").code, 0);
}

#[test]
fn a_base_commit_is_refused_when_it_deletes_the_config_or_the_config_does_not_read() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    install_hooks(&root);
    git(&root, &["switch", "-q", "main"]);
    let before = git(&root, &["rev-parse", "HEAD"]);
    // The base is named by HEAD's config when the commit deletes it.
    git(&root, &["rm", "-q", ".atelier/config.toml"]);
    let run = git_as(&root, &["commit", "-qm", "Delete config on base"]);
    assert_eq!(run.code, 1, "{}", run.stderr);
    assert!(
        run.stderr.contains("  .atelier/config.toml\n"),
        "{}",
        run.stderr
    );
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), before);
    git(&root, &["reset", "-q", "--hard"]);
    // And when the checkout's does not read, beside a staged record.
    edit(&root, &item_path(ID), "status: draft", "status: ready");
    git(&root, &["add", "-A"]);
    append(&root, ".atelier/config.toml", "base = 1\n");
    let run = git_as(&root, &["commit", "-qm", "Ready on base"]);
    assert_eq!(run.code, 1, "{}", run.stderr);
    assert!(
        run.stderr
            .contains("Rule: only a merge writes the base branch main"),
        "{}",
        run.stderr
    );
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), before);
}

#[test]
fn the_base_a_commit_is_judged_against_is_heads_not_the_checkouts() {
    let scratch = Scratch::new(&[(ID, "draft")]);
    let root = scratch.root();
    install_hooks(&root);
    git(&root, &["switch", "-q", "main"]);
    let before = git(&root, &["rev-parse", "HEAD"]);
    // A staged change of the base's name does not unprotect it.
    edit(
        &root,
        ".atelier/config.toml",
        "base_branch = \"main\"",
        "base_branch = \"other\"",
    );
    let run = commit_all(&root, "Move the base");
    assert_eq!(run.code, 1, "{}", run.stderr);
    assert!(
        run.stderr
            .contains("Rule: only a merge writes the base branch main"),
        "{}",
        run.stderr
    );
    // Nor does one left unstaged beside a staged record.
    git(&root, &["reset", "-q"]);
    edit(&root, &item_path(ID), "status: draft", "status: ready");
    git(&root, &["add", &item_path(ID)]);
    let run = git_as(&root, &["commit", "-qm", "Ready on base"]);
    assert_eq!(run.code, 1, "{}", run.stderr);
    assert!(
        run.stderr
            .contains("Rule: only a merge writes the base branch main"),
        "{}",
        run.stderr
    );
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), before);
}
