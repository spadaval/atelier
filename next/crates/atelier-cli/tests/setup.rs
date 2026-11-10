//! `setup` in scratch repositories: the Git hooks it installs, a hook of
//! the clone's own kept and still run, and the harness hooks merged into a
//! settings file. Every file it writes is under the scratch directory.

mod common;

use common::*;
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// `PATH` with the new binary's directory first, as the hook scripts run
/// `atelier` from `PATH`.
fn path() -> String {
    let binary = Path::new(env!("CARGO_BIN_EXE_atelier"));
    let rest = std::env::var("PATH").unwrap_or_default();
    format!("{}:{rest}", binary.parent().unwrap().display())
}

fn ran(command: &mut Command) -> Run {
    let output = command.stdin(Stdio::null()).output().unwrap();
    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

/// Git run as thread t1, with the hooks finding the new binary.
fn git_hooked(at: &Path, args: &[&str]) -> Run {
    let mut command = Command::new("git");
    command.current_dir(at).args(args).env("PATH", path());
    command.env("T3CODE_THREAD_ID", "t1");
    ran(identity(&mut command))
}

/// [`git_hooked`] that must succeed. Once `setup` has run, every Git
/// command that runs a hook goes through it, or the hook would run the
/// `atelier` installed on the host.
fn passes(at: &Path, args: &[&str]) {
    let run = git_hooked(at, args);
    assert_eq!(run.code, 0, "git {args:?}: {}", run.stderr);
}

/// A line printed for an agent to paste, run as written by a shell.
fn shell(at: &Path, line: &str) -> Run {
    let mut command = Command::new("sh");
    command
        .current_dir(at)
        .args(["-c", line])
        .env("PATH", path());
    command.env("T3CODE_THREAD_ID", "t1");
    ran(identity(&mut command))
}

fn hooks_dir(root: &Path) -> PathBuf {
    root.join(".git/hooks")
}

fn script(name: &str) -> String {
    format!("#!/bin/sh\nexec atelier check --hook {name} \"$@\"\n")
}

fn write_hook(path: &Path, text: &str, mode: u32) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).unwrap();
}

/// Every file in the hooks directory with its bytes, or its link target.
fn snapshot(dir: &Path) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = fs::read_dir(dir)
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            let content = match fs::read_link(&path) {
                Ok(target) => format!("-> {}", target.display()),
                Err(_) => fs::read_to_string(&path).unwrap(),
            };
            (name, content)
        })
        .collect();
    files.sort();
    files
}

/// Stages an item record that does not read, which Atelier's pre-commit
/// hook refuses.
fn stage_broken_record(checkout: &Path) {
    let path = checkout.join(".atelier/items/demo-zz99/item.md");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, "---\nnot: a record\n---\n").unwrap();
    git(checkout, &["add", "-A"]);
}

#[test]
fn setup_installs_three_hooks_once_for_every_worktree_and_status_says_when_there_are_none() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let second = scratch.worktree("second");
    fs::remove_dir_all(hooks_dir(&root)).unwrap();
    assert!(atelier(&root, "t1", &["status"])
        .stdout
        .starts_with(NO_HOOKS));

    // Run in the second worktree, it writes into the hooks directory both
    // share, creating it.
    let run = atelier(&second, "t1", &["setup"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let dir = hooks_dir(&root).canonicalize().unwrap();
    assert_eq!(
        run.stdout,
        format!(
            "Installed Atelier's Git hooks in {}, which every worktree of this clone shares:\n  \
             pre-commit: written\n  pre-push: written\n  post-checkout: written\n",
            dir.display()
        )
    );
    for name in ["pre-commit", "pre-push", "post-checkout"] {
        let hook = dir.join(name);
        assert_eq!(fs::read_to_string(&hook).unwrap(), script(name));
        assert_eq!(
            fs::metadata(&hook).unwrap().permissions().mode() & 0o777,
            0o755
        );
    }
    let installed = snapshot(&dir);
    for at in [&root, &second] {
        let run = atelier(at, "t1", &["setup"]);
        assert_eq!(run.code, 0, "{}", run.stderr);
        assert_eq!(
            run.stdout,
            format!(
                "Atelier's Git hooks are in {} already; nothing changed.\n",
                dir.display()
            )
        );
        assert!(!atelier(at, "t1", &["status"]).stdout.contains(NO_HOOKS));
    }
    assert_eq!(snapshot(&dir), installed);

    // The pre-commit hook refuses a broken record in each worktree.
    for at in [&root, &second] {
        stage_broken_record(at);
        let run = git_hooked(at, &["commit", "-qm", "Broken"]);
        assert_eq!(run.code, 1, "{}", run.stderr);
        assert!(
            run.stderr.starts_with(
                "Atelier's pre-commit hook refused this commit; nothing was committed.\n"
            ),
            "{}",
            run.stderr
        );
    }

    // A hook Atelier wrote that has lost its mode does not count, and setup
    // writes it again.
    fs::set_permissions(dir.join("pre-push"), fs::Permissions::from_mode(0o644)).unwrap();
    let run = atelier(&root, "t1", &["setup"]);
    assert!(
        run.stdout.contains("  pre-push: written\n"),
        "{}",
        run.stdout
    );
    assert_eq!(snapshot(&dir), installed);
    assert_eq!(
        fs::metadata(dir.join("pre-push"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o755
    );
}

#[test]
fn a_kept_pre_push_hook_gets_gits_arguments_and_input_and_can_still_refuse_after_atelier_passes() {
    let scratch = Scratch::new(&[(ID, "ready")]);
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

    // The clone's own hook records what it was given, and refuses while a
    // file named stop is there.
    let seen = scratch.dir.path().join("seen");
    let stop = scratch.dir.path().join("stop");
    let own = format!(
        "#!/bin/sh\necho \"$@\" > '{0}'\ncat >> '{0}'\nif [ -e '{1}' ]; then echo 'own hook says no' >&2; exit 3; fi\n",
        seen.display(),
        stop.display()
    );
    let dir = hooks_dir(&root);
    write_hook(&dir.join("pre-push"), &own, 0o755);
    let run = atelier(&root, "t1", &["setup"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(run.stdout.contains(
        "  pre-push: written; this clone's own pre-push hook was moved to pre-push.local, and runs after Atelier's rule passes\n"
    ), "{}", run.stdout);
    assert_eq!(fs::read_to_string(dir.join("pre-push.local")).unwrap(), own);
    assert_eq!(
        fs::read_to_string(dir.join("pre-push")).unwrap(),
        script("pre-push")
    );

    // Atelier passes a push of another branch, and the kept hook gets
    // Git's arguments and its line of input.
    passes(&root, &["switch", "-qc", "topic"]);
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    git(&root, &["add", "hello.txt"]);
    passes(&root, &["commit", "-qm", "Say hello"]);
    let tip = git(&root, &["rev-parse", "HEAD"]);
    let run = git_hooked(&root, &["push", "-q", "origin", "topic"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let zeros = "0".repeat(tip.len());
    assert_eq!(
        fs::read_to_string(&seen).unwrap(),
        format!(
            "origin {0}\nrefs/heads/topic {tip} refs/heads/topic {zeros}\n",
            remote.display()
        )
    );

    // It still refuses a push, which then does not happen.
    fs::write(&stop, "").unwrap();
    passes(&root, &["commit", "-qm", "Again", "--allow-empty"]);
    let run = git_hooked(&root, &["push", "-q", "origin", "topic"]);
    assert_eq!(run.code, 1, "{}", run.stderr);
    assert!(run.stderr.contains("own hook says no\n"), "{}", run.stderr);
    assert!(run.stderr.contains(
        "This clone's own pre-push hook, which atelier setup kept as pre-push.local in its hooks directory, refused: it exited with 3. Nothing was pushed.\n"
    ), "{}", run.stderr);
    assert!(git(&root, &["ls-remote", "origin", "refs/heads/topic"]).starts_with(&tip));

    // When Atelier's rule refuses, the kept hook does not run.
    fs::remove_file(&seen).unwrap();
    passes(&root, &["switch", "-q", "main"]);
    passes(&root, &["merge", "-q", "--ff-only", "topic"]);
    let run = git_hooked(&root, &["push", "-q", "origin", "main"]);
    assert_eq!(run.code, 1, "{}", run.stderr);
    assert!(
        run.stderr
            .starts_with("Atelier's pre-push hook refused this push"),
        "{}",
        run.stderr
    );
    assert!(!seen.exists());
}

#[test]
fn a_hook_beside_a_local_one_is_left_byte_for_byte_and_the_way_out_works() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let dir = hooks_dir(&root);
    write_hook(&dir.join("pre-commit"), "#!/bin/sh\necho mine\n", 0o755);
    write_hook(
        &dir.join("pre-commit.local"),
        "#!/bin/sh\necho older\n",
        0o700,
    );
    let before = snapshot(&dir);

    let run = atelier(&root, "t1", &["setup"]);
    assert_eq!(run.code, 1);
    assert_eq!(run.stdout, "");
    let dir = dir.canonicalize().unwrap();
    assert_eq!(
        run.stderr,
        format!(
            "`atelier setup` refused; nothing changed.\nsetup keeps a hook of this clone's own by \
             moving it to <name>.local, and in {} that name is taken:\n  pre-commit, and \
             pre-commit.local\nFor each, put what both files do into <name>.local and remove \
             <name>, then run: atelier setup\n",
            dir.display()
        )
    );
    // Neither file, nor any other hook, changed.
    assert_eq!(snapshot(&dir), before);
    assert_eq!(
        fs::metadata(dir.join("pre-commit.local"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o700
    );

    // The way out, done as it says.
    write_hook(
        &dir.join("pre-commit.local"),
        "#!/bin/sh\necho older\necho mine\n",
        0o755,
    );
    fs::remove_file(dir.join("pre-commit")).unwrap();
    let way_out = run.stderr.lines().last().unwrap();
    let run = shell(&root, way_out.split("then run: ").nth(1).unwrap());
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stdout.contains("  pre-commit: written\n"),
        "{}",
        run.stdout
    );

    // The kept pre-commit hook runs after Atelier's rule, on a commit.
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    git(&root, &["add", "hello.txt"]);
    let run = git_hooked(&root, &["commit", "-qm", "Say hello"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    // Git puts a hook's output on its own stderr.
    assert!(run.stderr.contains("older\nmine\n"), "{}", run.stderr);
}

#[test]
fn a_symbolic_link_or_a_hook_that_is_not_executable_is_kept_as_it_was_and_runs_only_as_git_would() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let dir = hooks_dir(&root);
    // A hook linked to a tracked script, by a path relative to the hooks
    // directory, which refuses every commit.
    write_hook(
        &root.join("refuse.sh"),
        "#!/bin/sh\necho linked hook refuses >&2\nexit 1\n",
        0o755,
    );
    git(&root, &["add", "refuse.sh"]);
    git(&root, &["commit", "-qm", "Add a hook script"]);
    symlink("../../refuse.sh", dir.join("pre-commit")).unwrap();
    // A post-checkout hook that is not executable, which Git does not run.
    write_hook(
        &dir.join("post-checkout"),
        "#!/bin/sh\necho should not run\n",
        0o644,
    );

    let run = atelier(&root, "t1", &["setup"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(run.stdout.contains(
        "  post-checkout: written; this clone's own post-checkout hook, which is not executable and so did not run, was moved to post-checkout.local\n"
    ), "{}", run.stdout);
    assert_eq!(
        fs::read_link(dir.join("pre-commit.local")).unwrap(),
        PathBuf::from("../../refuse.sh")
    );

    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    git(&root, &["add", "hello.txt"]);
    let run = git_hooked(&root, &["commit", "-qm", "Say hello"]);
    assert_eq!(run.code, 1, "{}", run.stderr);
    assert!(
        run.stderr.contains("linked hook refuses\n"),
        "{}",
        run.stderr
    );
    assert!(
        run.stderr.contains("Nothing was committed.\n"),
        "{}",
        run.stderr
    );

    let run = git_hooked(&root, &["switch", "-qc", "topic"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    // The kept hook is not run, so neither runs nor fails.
    assert!(!run.stderr.contains("should not run"), "{}", run.stderr);
    assert!(!run.stderr.contains("post-checkout.local"), "{}", run.stderr);
    assert!(run.stderr.contains("You are thread t1"), "{}", run.stderr);
}

#[test]
fn a_kept_post_checkout_hook_that_fails_does_not_fail_the_checkout() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let dir = hooks_dir(&root);
    let seen = scratch.dir.path().join("seen");
    write_hook(
        &dir.join("post-checkout"),
        &format!(
            "#!/bin/sh\necho \"$@\" > '{}'\necho own hook failed >&2\nexit 4\n",
            seen.display()
        ),
        0o755,
    );
    assert_eq!(atelier(&root, "t1", &["setup"]).code, 0);
    let before = git(&root, &["rev-parse", "HEAD"]);

    let run = git_hooked(&root, &["switch", "-qc", "topic"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    // Status first, then the kept hook, with Git's three arguments; Git
    // puts a hook's output on its own stderr.
    let said = run.stderr;
    assert!(
        said.starts_with("You are thread t1, on branch topic"),
        "{said}"
    );
    assert!(said.ends_with(
        "own hook failed\nSwitched; this clone's own post-checkout hook, kept as post-checkout.local, failed: it exited with 4.\n"
    ), "{said}");
    assert_eq!(
        fs::read_to_string(&seen).unwrap(),
        format!("{before} {before} 1\n")
    );
    assert_eq!(git(&root, &["symbolic-ref", "--short", "HEAD"]), "topic");
}

#[test]
fn setup_stopped_after_moving_a_hook_finishes_when_run_again() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let dir = hooks_dir(&root);
    // What a run stopped between the move and the write leaves.
    write_hook(&dir.join("pre-push.local"), "#!/bin/sh\nexit 0\n", 0o755);
    let run = atelier(&root, "t1", &["setup"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stdout.contains("  pre-push: written\n"),
        "{}",
        run.stdout
    );
    assert_eq!(
        fs::read_to_string(dir.join("pre-push.local")).unwrap(),
        "#!/bin/sh\nexit 0\n"
    );
    assert_eq!(
        fs::read_to_string(dir.join("pre-push")).unwrap(),
        script("pre-push")
    );
}

#[test]
fn a_hooks_path_outside_the_shared_git_directory_is_refused_and_the_way_out_works() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    // A directory of each worktree's own, as husky sets it.
    git(&root, &["config", "core.hooksPath", ".githooks"]);
    write_hook(
        &root.join(".githooks/pre-commit"),
        "#!/bin/sh\nexit 0\n",
        0o755,
    );
    let before = snapshot(&root.join(".githooks"));
    let shared = hooks_dir(&root).canonicalize().unwrap();
    let shared_before = snapshot(&shared);

    let run = atelier(&root, "t1", &["setup"]);
    assert_eq!(run.code, 1);
    let own = root.canonicalize().unwrap().join(".githooks");
    assert_eq!(
        run.stderr,
        format!(
            "`atelier setup` refused; nothing changed.\nGit runs this clone's hooks from {}, as \
             core.hooksPath says, and setup installs hooks only in the Git directory every \
             worktree of the clone shares.\nCopy any hook this clone needs from there into {1}, \
             then run: git config core.hooksPath {1} && atelier setup\n",
            own.display(),
            shared.display()
        )
    );
    assert_eq!(snapshot(&root.join(".githooks")), before);
    assert_eq!(snapshot(&shared), shared_before);
    assert!(atelier(&root, "t1", &["status"])
        .stdout
        .starts_with(NO_HOOKS));

    let line = run.stderr.lines().last().unwrap();
    let run = shell(&root, line.split("then run: ").nth(1).unwrap());
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(fs::read_to_string(shared.join("pre-commit")).unwrap() == script("pre-commit"));
    assert!(!atelier(&root, "t1", &["status"]).stdout.contains(NO_HOOKS));
}

const SETTINGS: &str = ".claude/settings.json";

fn json(root: &Path, file: &str) -> serde_json::Value {
    serde_json::from_str(&read(root, file)).unwrap()
}

#[test]
fn setup_claude_merges_its_hooks_into_the_settings_keeping_others_and_changes_nothing_again() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    // Another tool's hooks, and the old Atelier's, in the settings already.
    let before = r#"{
  "permissions": {"allow": ["Bash(ls)"]},
  "hooks": {
    "SessionStart": [
      {"matcher": "startup", "hooks": [{"type": "command", "command": "other-tool start"}]},
      {"hooks": [{"type": "command", "command": "atelier status --hook claude # atelier-managed:SessionStart", "timeout": 30}]}
    ],
    "Stop": [
      {"hooks": [
        {"type": "command", "command": "atelier land --hook claude # atelier-managed:Stop", "timeout": 30},
        {"type": "command", "command": "other-tool stop"}
      ]}
    ],
    "PreToolUse": [{"matcher": "Bash", "hooks": [{"type": "command", "command": "other-tool guard"}]}]
  }
}
"#;
    fs::create_dir_all(root.join(".claude")).unwrap();
    fs::write(root.join(SETTINGS), before).unwrap();

    let run = atelier(&root, "t1", &["setup", "claude"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        "Wrote Atelier's hooks into .claude/settings.json, keeping every other hook there.\n\
         A Claude Code session started in this repository begins with atelier status, and each \
         turn ends with atelier check.\nCommit .claude/settings.json so that every clone has it.\n"
    );
    let after = json(&root, SETTINGS);
    let expected: serde_json::Value = serde_json::from_str(
        r#"{
  "permissions": {"allow": ["Bash(ls)"]},
  "hooks": {
    "SessionStart": [
      {"matcher": "startup", "hooks": [{"type": "command", "command": "other-tool start"}]},
      {"hooks": [{"type": "command", "command": "atelier status # atelier-managed:SessionStart", "timeout": 30}]}
    ],
    "Stop": [
      {"hooks": [{"type": "command", "command": "other-tool stop"}]},
      {"hooks": [{"type": "command", "command": "atelier check || exit 1 # atelier-managed:Stop", "timeout": 30}]}
    ],
    "PreToolUse": [{"matcher": "Bash", "hooks": [{"type": "command", "command": "other-tool guard"}]}]
  }
}"#,
    )
    .unwrap();
    assert_eq!(after, expected);

    // Run again, and with another tool's hook added after Atelier's, it
    // changes nothing.
    let mut edited = after.clone();
    edited["hooks"]["SessionStart"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({"hooks": [{"type": "command", "command": "later-tool"}]}));
    let text = serde_json::to_string_pretty(&edited).unwrap() + "\n";
    fs::write(root.join(SETTINGS), &text).unwrap();
    let run = atelier(&root, "t1", &["setup", "claude"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(run
        .stdout
        .starts_with("Atelier's hooks are in .claude/settings.json already; nothing changed.\n"));
    assert_eq!(read(&root, SETTINGS), text);
}

#[test]
fn setup_claude_refuses_settings_it_cannot_read_and_writes_a_new_file_where_there_is_none() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    fs::create_dir_all(root.join(".claude")).unwrap();
    fs::write(root.join(SETTINGS), "{\"hooks\": {,}\n").unwrap();
    let run = atelier(&root, "t1", &["setup", "claude"]);
    assert_eq!(run.code, 1);
    assert!(run.stderr.starts_with(
        "`atelier setup claude` refused; nothing changed.\n.claude/settings.json is not valid JSON: "
    ), "{}", run.stderr);
    assert!(run
        .stderr
        .ends_with("Fix .claude/settings.json, then run: atelier setup claude\n"));
    assert_eq!(read(&root, SETTINGS), "{\"hooks\": {,}\n");

    fs::write(root.join(SETTINGS), "{\"hooks\": {\"Stop\": {}}}\n").unwrap();
    let run = atelier(&root, "t1", &["setup", "claude"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr
            .contains("hooks.Stop in .claude/settings.json is not a list.\n"),
        "{}",
        run.stderr
    );

    // A settings file that is a link is not replaced.
    fs::remove_file(root.join(SETTINGS)).unwrap();
    fs::write(root.join("shared.json"), "{}\n").unwrap();
    symlink("../shared.json", root.join(SETTINGS)).unwrap();
    let run = atelier(&root, "t1", &["setup", "claude"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr
            .contains(".claude/settings.json is a symbolic link"),
        "{}",
        run.stderr
    );
    assert_eq!(read(&root, "shared.json"), "{}\n");

    fs::remove_dir_all(root.join(".claude")).unwrap();
    let run = atelier(&root, "t1", &["setup", "claude"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let settings = json(&root, SETTINGS);
    assert_eq!(
        settings["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        "atelier status # atelier-managed:SessionStart"
    );
    assert_eq!(
        fs::metadata(root.join(SETTINGS))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o644
    );
}

#[test]
fn setup_codex_starts_a_session_with_status_and_changes_nothing_again() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    let run = atelier(&root, "t1", &["setup", "codex"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        "Wrote Atelier's hooks into .codex/hooks.json, keeping every other hook there.\n\
         A Codex session started in this repository begins with atelier status, once Codex \
         trusts the project and you have trusted the hook in its /hooks list.\n\
         Commit .codex/hooks.json so that every clone has it.\n"
    );
    let written = read(&root, ".codex/hooks.json");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&written).unwrap(),
        serde_json::json!({"hooks": {"SessionStart": [{"hooks": [
            {"type": "command", "command": "atelier status # atelier-managed:SessionStart", "timeout": 30}
        ]}]}})
    );
    let run = atelier(&root, "t1", &["setup", "codex"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(run
        .stdout
        .starts_with("Atelier's hooks are in .codex/hooks.json already; nothing changed.\n"));
    assert_eq!(read(&root, ".codex/hooks.json"), written);
    assert!(!root.join(".claude").exists());
}
