//! What the scenario tests share: scratch repositories and the new binary
//! driven in them, one thread at a time through `T3CODE_THREAD_ID`.

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

pub const ID: &str = "demo-ab12";

/// What ends a refusal of anything else while `ATELIER_SKIP` is set.
pub const NOT_SKIPPABLE: &str =
    "ATELIER_SKIP passes only a rule, and what refused this is not one, so it cannot be skipped.\n";

/// What ends every refusal of a rule, after its way out.
pub const SKIP: &str = "If the rule is wrong for this case, go past it on purpose: run the same command again with ATELIER_SKIP=\"<why>\" in front of it.\n";

/// The line `status` begins with in a clone with none of Atelier's Git
/// hooks, as every scratch repository is until `setup` runs in it.
pub const NO_HOOKS: &str = "This clone has none of Atelier's Git hooks, so nothing checks its commits and pushes. Install them with: atelier setup\n";

/// What a handoff line says between who must be another thread and the
/// one task it gives.
pub const START_ONE: &str = "must be a thread other than yours. Starting it is part of the work you were given, so start one yourself now, without waiting to be asked. Under Codex a subagent you start is one; under Claude Code a subagent runs as your thread and is refused, so use your harness's tool for a new agent thread. Give it fresh context, not a fork of yours, and give it this task:\n";

pub struct Scratch {
    pub dir: TempDir,
}

impl Scratch {
    /// A repository on `main` whose one commit holds `items`, each an ID
    /// with its status.
    pub fn new(items: &[(&str, &str)]) -> Scratch {
        let scratch = Scratch {
            dir: TempDir::new().unwrap(),
        };
        let root = scratch.root();
        git(&root, &["init", "-q", "-b", "main"]);
        fs::create_dir_all(root.join(".atelier")).unwrap();
        fs::write(
            root.join(".atelier/config.toml"),
            "schema_version = 3\nslug = \"demo\"\nbase_branch = \"main\"\n",
        )
        .unwrap();
        for (id, status) in items {
            write_item(&root, id, status, &[], &[], "");
        }
        git(&root, &["add", "-A"]);
        git(&root, &["commit", "-qm", "Plan"]);
        scratch
    }

    pub fn root(&self) -> PathBuf {
        self.dir.path().join("repo")
    }

    /// A second worktree of the repository, detached at `main`.
    pub fn worktree(&self, name: &str) -> PathBuf {
        let path = self.dir.path().join(name);
        git(
            &self.root(),
            &[
                "worktree",
                "add",
                "-q",
                "--detach",
                path.to_str().unwrap(),
                "main",
            ],
        );
        path
    }
}

pub fn item_path(id: &str) -> String {
    format!(".atelier/items/{id}/item.md")
}

pub fn write_item(
    checkout: &Path,
    id: &str,
    status: &str,
    implementers: &[&str],
    blocked_by: &[&str],
    report: &str,
) {
    let list = |values: &[&str]| -> String {
        if values.is_empty() {
            " []".into()
        } else {
            values.iter().map(|v| format!("\n- {v}")).collect()
        }
    };
    let path = checkout.join(item_path(id));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        format!(
            "---\nschema: atelier.item\nschema_version: 3\nid: {id}\ntitle: Say hello\n\
             status: {status}\nmission: null\nlabels: []\npriority: high\nblocked_by:{}\n\
             implementers:{}\nready: null\ntrivial: null\ncaused_by: null\n\
             created_at: 2026-11-10T09:00:00Z\n---\n\n## Goal\n\nPrint hello.\n\n## Done when\n\n\
             - [ ] `hello` prints hello.\n- [ ] A test covers it.\n\n## Scope\n\nOnly hello.\n\n\
             ## Report\n\n{report}",
            list(blocked_by),
            list(implementers)
        ),
    )
    .unwrap();
}

pub fn git(at: &Path, args: &[&str]) -> String {
    let output = git_output(at, args);
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

pub fn git_output(at: &Path, args: &[&str]) -> Output {
    if !at.exists() {
        fs::create_dir_all(at).unwrap();
    }
    identity(Command::new("git").current_dir(at).args(args))
        .output()
        .unwrap()
}

pub fn identity(command: &mut Command) -> &mut Command {
    command
        .env("GIT_AUTHOR_NAME", "b")
        .env("GIT_AUTHOR_EMAIL", "b@b")
        .env("GIT_COMMITTER_NAME", "b")
        .env("GIT_COMMITTER_EMAIL", "b@b")
}

pub fn atelier_command(at: &Path, thread: &str, args: &[&str]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_atelier"));
    command.current_dir(at).args(args);
    for variable in [
        "CLAUDE_CODE_SESSION_ID",
        "CODEX_SESSION_ID",
        "CODEX_THREAD_ID",
        "GROK_SESSION_ID",
    ] {
        command.env_remove(variable);
    }
    command.env("T3CODE_THREAD_ID", thread);
    command.env_remove("ATELIER_TEST_HOLD_BEFORE_BRANCH");
    command.env_remove("ATELIER_SKIP");
    identity(&mut command);
    command
}

pub struct Run {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn atelier(at: &Path, thread: &str, args: &[&str]) -> Run {
    let output = atelier_command(at, thread, args).output().unwrap();
    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

/// The item's front-matter field `key` as `rev` holds it; a list's entries
/// are joined by spaces.
pub fn field(at: &Path, rev: &str, key: &str) -> String {
    front(
        &git(at, &["show", &format!("{rev}:{}", item_path(ID))]),
        key,
    )
}

/// The front-matter field `key` of the record `text`, as [`field`] gives it.
pub fn front(text: &str, key: &str) -> String {
    let mut lines = text
        .lines()
        .skip_while(|line| !line.starts_with(&format!("{key}:")));
    let value = lines.next().unwrap()[key.len() + 1..].trim().to_string();
    if !value.is_empty() {
        return value;
    }
    let entries: Vec<&str> = lines.map_while(|line| line.strip_prefix("- ")).collect();
    entries.join(" ")
}

pub fn branch_exists(at: &Path, name: &str) -> bool {
    git_output(
        at,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/heads/{name}"),
        ],
    )
    .status
    .success()
}

pub const REVIEWS: &str = ".atelier/items/demo-ab12/reviews";

/// The review files of the item in the checkout, by name.
pub fn reviews(root: &Path) -> Vec<String> {
    let Ok(listing) = fs::read_dir(root.join(REVIEWS)) else {
        return Vec::new();
    };
    let mut names: Vec<String> = listing
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    names
}

/// The one review file of `round`, as its stem and its path from the root.
pub fn review_of(root: &Path, round: u32) -> (String, String) {
    let found: Vec<String> = reviews(root)
        .into_iter()
        .filter(|name| name.starts_with(&format!("{round}-")))
        .collect();
    assert_eq!(found.len(), 1, "{found:?}");
    let stem = found[0].strip_suffix(".md").unwrap().to_string();
    assert_eq!(stem.len(), format!("{round}-").len() + 4, "{stem}");
    let path = format!("{REVIEWS}/{stem}.md");
    (stem, path)
}

pub fn read(root: &Path, path: &str) -> String {
    fs::read_to_string(root.join(path)).unwrap()
}

pub fn append(root: &Path, path: &str, text: &str) {
    let path = root.join(path);
    let before = fs::read_to_string(&path).unwrap();
    fs::write(path, before + text).unwrap();
}

/// Ticks the item's Done when lines, writes its Report and submits it as
/// `thread`.
pub fn finish_and_submit(root: &Path, thread: &str) {
    let path = root.join(item_path(ID));
    let ticked = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, ticked + "It prints hello.\n").unwrap();
    let run = atelier(root, thread, &["submit"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
}

/// A scratch repository whose item t1 started, changed and submitted.
pub fn submitted() -> Scratch {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    fs::write(root.join("hello.txt"), "hello\n").unwrap();
    git(&root, &["add", "hello.txt"]);
    git(&root, &["commit", "-qm", "Say hello"]);
    finish_and_submit(&root, "t1");
    scratch
}
