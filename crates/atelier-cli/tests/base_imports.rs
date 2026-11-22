//! Real Git conflict-resolution commits, with the pre-commit hook enabled.
mod common;
use common::*;
use std::{fs, os::unix::fs::PermissionsExt, path::Path, process::Command};

fn commit(root: &Path) -> Run {
    let output = identity(
        Command::new("git")
            .current_dir(root)
            .args(["commit", "-qm", "Resolve catch-up"])
            .env_remove("ATELIER_SKIP"),
    )
    .output()
    .unwrap();
    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

// Seed the two committed histories before installing hooks. No commit under
// test runs without the hook, and no fixture uses a guard skip or commit-tree.
fn merging() -> Scratch {
    merging_record("approved", &[])
}

fn merging_record(status: &str, blockers: &[&str]) -> Scratch {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    fs::write(root.join("conflict"), "base\n").unwrap();
    on_base(&root, "Common ancestor");
    git(&root, &["checkout", "-qb", "topic"]);
    fs::write(root.join("conflict"), "topic\n").unwrap();
    git(&root, &["add", "."]);
    git(&root, &["commit", "-qm", "Topic"]);
    off_base(&root);
    write_item(
        &root,
        ID,
        status,
        &["t1"],
        blockers,
        "Already approved on base.",
    );
    fs::write(root.join("conflict"), "new base\n").unwrap();
    on_base(&root, "Base record already approved");
    git(&root, &["checkout", "-q", "topic"]);
    install_import_hook(&root);
    let merge = git_output(&root, &["merge", "--no-commit", "main"]);
    assert!(!merge.status.success());
    assert!(root.join(".git/MERGE_HEAD").exists());
    fs::write(root.join("conflict"), "resolved\n").unwrap();
    git(&root, &["add", "."]);
    scratch
}

#[test]
fn unchanged_verified_base_import_commits_through_hook() {
    let scratch = merging();
    let root = scratch.root();
    assert_eq!(
        read(&root, &item_path(ID)),
        git(&root, &["show", &format!("main:{}", item_path(ID))])
    );
    let run = commit(&root);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        git(&root, &["rev-list", "--parents", "-n1", "HEAD"])
            .split_whitespace()
            .count(),
        3
    );
}

#[test]
fn unverified_or_edited_imports_still_refuse_through_hook() {
    for case in [
        "edited",
        "forged-status",
        "non-base",
        "multiple-parents",
        "rev-expression",
        "unstaged-config",
        "no-merge",
        "staged-config",
        "redirect-config",
        "malformed",
        "invalid",
        "on-base",
        "old-base",
    ] {
        let scratch = merging();
        let root = scratch.root();
        let merge_head = root.join(".git/MERGE_HEAD");
        match case {
            "edited" => append(&root, &item_path(ID), "Extra authored evidence.\n"),
            "forged-status" => {
                let text = read(&root, &item_path(ID))
                    .replace("status: approved", "status: changes-requested");
                fs::write(root.join(item_path(ID)), text).unwrap();
            }
            "non-base" => fs::write(&merge_head, git(&root, &["rev-parse", "HEAD"])).unwrap(),
            "multiple-parents" => {
                let text = fs::read_to_string(&merge_head).unwrap();
                fs::write(&merge_head, text.repeat(2)).unwrap();
            }
            "rev-expression" => fs::write(&merge_head, "refs/heads/main\n").unwrap(),
            "unstaged-config" => {
                fs::write(&merge_head, git(&root, &["rev-parse", "HEAD"]) + "\n").unwrap();
            }
            "no-merge" => fs::remove_file(&merge_head).unwrap(),
            "staged-config" => append(&root, ".atelier/config.toml", "\n# changed\n"),
            "redirect-config" => {
                let text = read(&root, ".atelier/config.toml").replace("main", "topic");
                fs::write(root.join(".atelier/config.toml"), text).unwrap();
            }
            "malformed" => fs::write(root.join(item_path(ID)), "not a record").unwrap(),
            "invalid" => {
                let text = read(&root, &item_path(ID))
                    .replace("blocked_by: []", "blocked_by: [demo-zz99]");
                fs::write(root.join(item_path(ID)), text).unwrap();
            }
            "on-base" => {
                git(&root, &["symbolic-ref", "HEAD", "refs/heads/main"]);
                fs::write(
                    root.join(".atelier/new-record-note"),
                    "base writes still refused",
                )
                .unwrap();
            }
            "old-base" => {
                git(&root, &["branch", "-f", "main", "HEAD"]);
            }
            _ => unreachable!(),
        }
        git(&root, &["add", "."]);
        if case == "unstaged-config" {
            let text = read(&root, ".atelier/config.toml").replace("main", "topic");
            fs::write(root.join(".atelier/config.toml"), text).unwrap();
        }
        let before = git(&root, &["rev-parse", "HEAD"]);
        let run = commit(&root);
        assert_ne!(run.code, 0, "{case}: accepted");
        assert!(
            run.stderr.contains("pre-commit hook refused"),
            "{case}: {}",
            run.stderr
        );
        assert_eq!(git(&root, &["rev-parse", "HEAD"]), before, "{case}");
    }
}

#[test]
fn configured_remote_base_is_trusted_but_an_unconfigured_remote_is_not() {
    for trusted in [false, true] {
        let scratch = Scratch::new(&[(ID, "ready")]);
        let root = scratch.root();
        if trusted {
            append(&root, ".atelier/config.toml", "\n[forge]\nprovider = \"github\"\nremote = \"upstream\"\nci_workflow = \"ci.yml\"\n");
        }
        fs::write(root.join("config-seed"), "seed").unwrap();
        on_base(&root, "Committed config");
        git(&root, &["checkout", "-qb", "topic"]);
        fs::write(root.join("topic"), "topic").unwrap();
        git(&root, &["add", "."]);
        git(&root, &["commit", "-qm", "Topic"]);
        off_base(&root);
        write_item(&root, ID, "approved", &["t1"], &[], "Already reviewed");
        git(&root, &["add", "."]);
        git(&root, &["commit", "-qm", "Remote base"]);
        git(&root, &["update-ref", "refs/remotes/upstream/main", "HEAD"]);
        git(&root, &["checkout", "-q", "topic"]);
        install_import_hook(&root);
        git(&root, &["merge", "--no-commit", "upstream/main"]);
        let run = commit(&root);
        assert_eq!(run.code == 0, trusted, "{}", run.stderr);
    }
}

fn install_import_hook(root: &Path) {
    let binary = std::env::var("ATELIER_TEST_IMPORT_BINARY")
        .unwrap_or_else(|_| env!("CARGO_BIN_EXE_atelier").into());
    let hooks = git(
        root,
        &["rev-parse", "--path-format=absolute", "--git-path", "hooks"],
    );
    let hook = Path::new(&hooks).join("pre-commit");
    fs::create_dir_all(&hooks).unwrap();
    fs::write(
        &hook,
        format!("#!/bin/sh\nexec '{binary}' check --hook pre-commit\n"),
    )
    .unwrap();
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
}

#[test]
fn merge_state_is_read_from_the_linked_worktree() {
    let scratch = merging();
    let root = scratch.root();
    git(&root, &["merge", "--abort"]);
    let linked = scratch.dir.path().join("linked");
    git(
        &root,
        &[
            "worktree",
            "add",
            "-qb",
            "linked-topic",
            linked.to_str().unwrap(),
            "topic",
        ],
    );
    assert!(!git_output(&linked, &["merge", "--no-commit", "main"])
        .status
        .success());
    fs::write(linked.join("conflict"), "resolved\n").unwrap();
    git(&linked, &["add", "."]);
    let run = commit(&linked);
    assert_eq!(run.code, 0, "{}", run.stderr);
}

#[test]
fn even_byte_identical_imports_must_parse_and_validate() {
    for (status, blockers) in [("finished", &[][..]), ("approved", &["demo-zz99"][..])] {
        let scratch = merging_record(status, blockers);
        let root = scratch.root();
        let run = commit(&root);
        assert_ne!(run.code, 0, "invalid base record accepted");
        assert!(
            run.stderr.contains("pre-commit hook refused"),
            "{}",
            run.stderr
        );
    }
}
