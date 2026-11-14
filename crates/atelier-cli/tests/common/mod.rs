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
pub const START_ONE: &str = "must be a thread other than yours, and starting it is your job: it is part of the task you were given, even as another thread's child or subagent, and nobody else will start it. So start one yourself now, without waiting to be asked. Under Codex a subagent you start is one; under Claude Code a subagent runs as your thread and is refused, so use your harness's tool for a new agent thread. Give it fresh context, not a fork of yours, and give it this task:\n";

pub struct Scratch {
    pub dir: TempDir,
}

impl Scratch {
    /// A repository whose one commit, on `main`, holds `items`, each an
    /// ID with its status. Its checkout is detached at that commit, since
    /// nothing is written in the base's own checkout; [`on_base`] commits
    /// there.
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
        git(&root, &["checkout", "-q", "--detach"]);
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

/// Commits everything in the detached checkout `at` as `message`, and moves
/// `main` to that commit, as a merge of a backlog branch would.
pub fn on_base(at: &Path, message: &str) {
    git(at, &["add", "-A"]);
    git(at, &["commit", "-qm", message]);
    git(at, &["branch", "-f", "main", "HEAD"]);
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

#[derive(Clone)]
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

/// Runs the Git command a command printed on its last `Then: git ...`
/// line, as the agent would, in `at`.
pub fn run_then(at: &Path, stdout: &str) {
    let line = stdout
        .lines()
        .rev()
        .find_map(|line| line.strip_prefix("Then: "))
        .filter(|line| line.starts_with("git "))
        .unwrap_or_else(|| panic!("no Then: git line in {stdout}"));
    let output = identity(Command::new("sh").current_dir(at).args(["-c", line]))
        .output()
        .unwrap();
    assert!(output.status.success(), "{line}: {output:?}");
}

/// `verdict` as `thread`, and, when it wrote its files, the commit it
/// printed, as the reviewer makes it.
pub fn verdict(at: &Path, thread: &str, args: &[&str]) -> Run {
    let run = atelier(at, thread, args);
    if run.code == 0 {
        run_then(at, &run.stdout);
    }
    run
}

/// A planning command as `thread`, and, when it wrote its records, the
/// commit it printed, with the base moved to it, as the merge of a
/// backlog branch would move it. `at` is a detached checkout.
pub fn planned(at: &Path, thread: &str, args: &[&str]) -> Run {
    let run = atelier(at, thread, args);
    if run.code == 0 && run.stdout.contains("\nThen: git ") {
        run_then(at, &run.stdout);
        git(at, &["branch", "-f", "main", "HEAD"]);
    }
    run
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

/// A fake `gh` that answers from files in `$FAKE_GH`: `runs/<branch>` for
/// the CI runs of a branch's pull requests, `pulls` for the open pull
/// requests. A POST to the pull requests opens #7 and adds a line to
/// `opened`. Every call is logged to `calls`. While `down` exists a runs
/// lookup fails, and while `stall` exists it never answers, after writing
/// its process ID to `stalled`; while `once` exists it answers and then
/// creates `stall`, so every later one stalls; while `helper` exists it answers but leaves
/// a process holding its output, whose ID it writes to `helped`; while
/// `no-pulls` exists opening a pull request fails. While `pulls-down`
/// exists looking one up fails, and while `pulls-stall` exists it never
/// answers, after writing its process ID to `stalled`.
///
/// `pr merge` merges as the forge does, in the bare repository whose path
/// is in `remote`, past its hooks: a merge commit of `main` and the head
/// `--match-head-commit` names, which must be a branch's tip there, with
/// the subject and body given. Then no pull request is open, and a line is
/// added to `merged`. While `merge-refused` exists it refuses as GitHub
/// does when the base's protection is not met. `pr view` answers with
/// `view`, or an open pull request with nothing missing.
pub const FAKE_GH: &str = r#"#!/bin/sh
echo "$*" >> "$FAKE_GH/calls"
if [ "$1" = auth ]; then exit 0; fi
if [ "$1" = pr ] && [ "$2" = view ]; then
  if [ -f "$FAKE_GH/view" ]; then cat "$FAKE_GH/view"; else
    echo '{"state":"OPEN","reviewDecision":"","mergeStateStatus":"CLEAN","headRefOid":""}'
  fi
  exit 0
fi
if [ "$1" = pr ] && [ "$2" = merge ]; then
  if [ -f "$FAKE_GH/merge-refused" ]; then
    echo "X Pull request owner/repo#7 is not mergeable: the base branch policy prohibits the merge." >&2
    exit 1
  fi
  head=""; subject=""; body=""; previous=""
  for arg in "$@"; do
    case "$previous" in
      --match-head-commit) head="$arg" ;;
      --subject) subject="$arg" ;;
      --body) body="$arg" ;;
    esac
    previous="$arg"
  done
  remote=$(cat "$FAKE_GH/remote")
  export GIT_AUTHOR_NAME=forge GIT_AUTHOR_EMAIL=forge@example.com
  export GIT_COMMITTER_NAME=forge GIT_COMMITTER_EMAIL=forge@example.com
  if [ -z "$(git --git-dir="$remote" for-each-ref --points-at "$head" refs/heads/)" ]; then
    echo "GraphQL: Head branch was modified. Review and try the merge again. (mergePullRequest)" >&2
    exit 1
  fi
  tree=$(git --git-dir="$remote" merge-tree --write-tree main "$head") || {
    echo "GraphQL: Pull Request is not mergeable (mergePullRequest)" >&2; exit 1; }
  commit=$(printf '%s

%s
' "$subject" "$body" |
    git --git-dir="$remote" commit-tree "$tree" -p main -p "$head") || exit 1
  git --git-dir="$remote" update-ref refs/heads/main "$commit" || exit 1
  echo '[]' > "$FAKE_GH/pulls"
  echo "$head" >> "$FAKE_GH/merged"
  exit 0
fi
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
    if [ -f "$FAKE_GH/once" ]; then touch "$FAKE_GH/stall"; fi
    if [ -f "$FAKE_GH/helper" ]; then sleep 60 & echo $! >> "$FAKE_GH/helped"; fi
    ;;
esac
if [ "$answer" = pulls ] && [ "$method" = GET ]; then
  if [ -f "$FAKE_GH/pulls-stall" ]; then echo $$ > "$FAKE_GH/stalled"; exec sleep 600; fi
  if [ -f "$FAKE_GH/pulls-down" ]; then echo "HTTP 502: Bad Gateway" >&2; exit 1; fi
fi
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
