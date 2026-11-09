//! `ci.passed` and `merge --push` (ADR 0031; docs/product/reviews-and-merge.md,
//! CI on a forge) against a fake `gh` on `PATH` that answers `gh api` from
//! files: `runs/<sha>` for a commit's pull request runs, `base` for the
//! base's newest push run, `pulls` for the pull requests that have a commit,
//! `pull` for one pull request's state, and `signed-out` to answer as a
//! signed-out `gh` does. With `pushed`, a list of commits, it refuses a
//! commit's pull requests for any other commit, as GitHub does for one it
//! has never seen; with `list-fails`, listing pull requests fails. A POST
//! to the pull requests opens #7, writing `pulls` and `pull`, unless
//! `create-fails` exists. `on-runs`, when present, is a script run once at
//! the first runs lookup. `sequence/<sha>/<n>` files are a commit's runs
//! answers in turn: each lookup of the commit moves the lowest-numbered one
//! to `runs/<sha>`. Every call is logged to `calls`.
//!
//! `merge --push` reaches `git@github.com:owner/repo.git` through a fake ssh
//! that serves a bare repository instead, so nothing leaves the machine.
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use tempfile::{tempdir, TempDir};
#[path = "support/cli.rs"]
mod support_cli;
use support_cli::{commit, git, ok};

const FAKE_GH: &str = r#"#!/bin/sh
echo "$*" >> "$FAKE_GH/calls"
if [ -f "$FAKE_GH/signed-out" ]; then
  echo "To get started with GitHub CLI, please run:  gh auth login" >&2
  exit 4
fi
if [ "$1" = auth ]; then
  exit 0
fi
answer=""
method=GET
previous=""
for arg in "$@"; do
  if [ "$previous" = -X ]; then method="$arg"; fi
  previous="$arg"
  case "$arg" in
    head_sha=*) answer="runs/${arg#head_sha=}" ;;
    branch=*) answer="base" ;;
    */commits/*/pulls)
      answer="pulls"
      sha="${arg%/pulls}"
      sha="${sha##*/}"
      if [ -f "$FAKE_GH/pushed" ] && ! grep -qx "$sha" "$FAKE_GH/pushed"; then
        echo "gh: No commit found for SHA: $sha (HTTP 422)" >&2
        exit 1
      fi ;;
    */pulls)
      answer="pulls"
      if [ -f "$FAKE_GH/list-fails" ]; then
        echo "gh: Server Error (HTTP 502)" >&2
        exit 1
      fi ;;
    */pulls/*) answer="pull" ;;
  esac
done
case "$answer" in
  runs/*)
    if [ -f "$FAKE_GH/on-runs" ]; then
      sh "$FAKE_GH/on-runs"
      rm -f "$FAKE_GH/on-runs"
    fi
    sequence="$FAKE_GH/sequence/${answer#runs/}"
    if [ -d "$sequence" ]; then
      next=$(ls "$sequence" | sort -n | head -n 1)
      if [ -n "$next" ]; then mv "$sequence/$next" "$FAKE_GH/$answer"; fi
    fi ;;
  pulls)
    if [ "$method" = POST ]; then
      if [ -f "$FAKE_GH/create-fails" ]; then
        echo "gh: Validation Failed (HTTP 422)" >&2
        exit 1
      fi
      echo '[{"number":7,"state":"open","base":{"ref":"main"},"html_url":"https://github.com/owner/repo/pull/7"}]' > "$FAKE_GH/pulls"
      echo '{"number":7,"merged":true}' > "$FAKE_GH/pull"
      echo '{"number":7,"html_url":"https://github.com/owner/repo/pull/7"}'
      exit 0
    fi ;;
esac
if [ -f "$FAKE_GH/$answer" ]; then
  cat "$FAKE_GH/$answer"
elif [ "$answer" = pulls ]; then
  echo '[]'
elif [ "$answer" = pull ]; then
  echo '{"merged":false}'
else
  echo '{"total_count":0,"workflow_runs":[]}'
fi
"#;

/// Runs the git command ssh was asked to run on the remote, such as
/// `git-upload-pack 'owner/repo.git'`, on the bare repository `$FAKE_REMOTE`.
const FAKE_SSH: &str = r#"#!/bin/sh
for last; do :; done
command="${last%% *}"
exec git "${command#git-}" "$FAKE_REMOTE"
"#;

const FORGE: &str = "forge:\n  provider: github\n  ci_workflow: ci.yml\n";

/// A repository whose base policy names a forge and runs `checks`, with an
/// owner started, implemented, reported and approved on its branch.
struct Forge {
    dir: TempDir,
    /// The fake gh's directory.
    gh: TempDir,
    id: String,
    branch: String,
}

impl Forge {
    fn new(checks: &str) -> Self {
        Self::with_forge(FORGE, checks)
    }
    fn with_forge(forge: &str, checks: &str) -> Self {
        let dir = tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "-qb", "main"]);
        git(repo, &["config", "user.name", "Author"]);
        git(repo, &["config", "user.email", "author@example.test"]);
        git(
            repo,
            &["remote", "add", "origin", "git@github.com:owner/repo.git"],
        );
        ok(repo, &["init", "--slug", "atelier"]);
        append_policy(repo, &format!("{forge}merge_checks:\n{checks}"));
        commit(repo, "base policy with a forge");
        let id = owner(repo, "Owner");
        ok(repo, &["start", &id]);
        fs::write(repo.join("src.rs"), "tested code\n").unwrap();
        commit(repo, "implementation");
        report(repo, &id);
        commit(repo, "report");
        approve(repo, &id);
        let gh = tempdir().unwrap();
        fs::create_dir(gh.path().join("runs")).unwrap();
        fs::create_dir(gh.path().join("bin")).unwrap();
        let fake = gh.path().join("bin/gh");
        fs::write(&fake, FAKE_GH).unwrap();
        make_executable(&fake);
        let ssh = gh.path().join("ssh");
        fs::write(&ssh, FAKE_SSH).unwrap();
        make_executable(&ssh);
        // The forge's copy of the repository, which origin reaches.
        git(
            repo,
            &[
                "init",
                "-q",
                "--bare",
                "-b",
                "main",
                &gh.path().join("remote.git").to_string_lossy(),
            ],
        );
        git(
            repo,
            &[
                "push",
                "-q",
                &gh.path().join("remote.git").to_string_lossy(),
                "main",
            ],
        );
        let branch = git(repo, &["branch", "--show-current"]);
        Self {
            dir,
            gh,
            id,
            branch,
        }
    }
    fn path(&self) -> &Path {
        self.dir.path()
    }
    fn head(&self) -> String {
        git(self.path(), &["rev-parse", "HEAD"])
    }
    /// Records a pull request run of ci.yml for `commit`.
    fn run(&self, commit: &str, id: u64, status: &str, conclusion: Option<&str>) {
        fs::write(
            self.gh.path().join("runs").join(commit),
            runs(&[(id, commit, status, conclusion, Some(7))]),
        )
        .unwrap();
    }
    fn calls(&self) -> String {
        fs::read_to_string(self.gh.path().join("calls")).unwrap_or_default()
    }
    /// `PATH` with the fake gh first.
    fn with_gh(&self) -> String {
        format!(
            "{}:{}",
            self.gh.path().join("bin").display(),
            std::env::var("PATH").unwrap()
        )
    }
    /// atelier with the fake gh, or with no gh on `PATH` at all.
    fn atelier(&self, args: &[&str], gh: bool) -> (bool, String) {
        self.atelier_env(args, gh, &[])
    }
    /// atelier with the fake gh and `env` besides.
    fn atelier_env(&self, args: &[&str], gh: bool, env: &[(&str, &str)]) -> (bool, String) {
        let path = if gh {
            self.with_gh()
        } else {
            without_gh(self.gh.path()).display().to_string()
        };
        let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
            .current_dir(self.path())
            .env("PATH", path)
            .env("FAKE_GH", self.gh.path())
            .env("FAKE_REMOTE", self.remote())
            .env("GIT_SSH_COMMAND", self.gh.path().join("ssh"))
            .env("ATELIER_TEST_MERGE_PR_WAIT_SECS", "0")
            .env("ATELIER_TEST_MERGE_CI_POLL_MS", "10")
            .envs(env.iter().copied())
            .args(args)
            .output()
            .unwrap();
        (
            output.status.success(),
            format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ),
        )
    }
    /// The forge's copy of the work branch, if it has one.
    fn remote_branch(&self) -> Option<String> {
        let output = Command::new("git")
            .args(["--git-dir", &self.remote().to_string_lossy()])
            .args(["rev-parse", "--verify", "--quiet"])
            .arg(format!("refs/heads/{}", self.branch))
            .output()
            .unwrap();
        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
    }
    /// The pull requests `merge --push` opened, as the fake gh logged them.
    fn opened(&self) -> Vec<String> {
        self.calls()
            .lines()
            .filter(|call| call.contains("-X POST"))
            .map(str::to_owned)
            .collect()
    }
    /// Answers the runs lookups of `commit` with `states` in turn, the last
    /// one for good.
    #[cfg(feature = "test-hooks")]
    fn run_sequence(&self, commit: &str, id: u64, states: &[(&str, Option<&str>)]) {
        let dir = self.gh.path().join("sequence").join(commit);
        fs::create_dir_all(&dir).unwrap();
        for (n, (status, conclusion)) in states.iter().enumerate() {
            fs::write(
                dir.join(n.to_string()),
                runs(&[(id, commit, status, *conclusion, Some(7))]),
            )
            .unwrap();
        }
    }
    /// The bare repository the forge holds.
    fn remote(&self) -> PathBuf {
        self.gh.path().join("remote.git")
    }
    /// The forge's base head.
    fn remote_main(&self) -> String {
        git(
            self.path(),
            &[
                "--git-dir",
                &self.remote().to_string_lossy(),
                "rev-parse",
                "main",
            ],
        )
    }
    /// Commits `file` on the forge's base from a scratch clone, as another
    /// agent's push would, and returns the commit.
    fn push_to_remote_main(&self, file: &str) -> String {
        self.push_to_remote_main_with(file, "from another checkout\n")
    }
    /// Commits `file` with `content` on the forge's base.
    fn push_to_remote_main_with(&self, file: &str, content: &str) -> String {
        let clone = tempdir().unwrap();
        let work = clone.path().join("work");
        git(
            self.path(),
            &[
                "clone",
                "-q",
                &self.remote().to_string_lossy(),
                &work.to_string_lossy(),
            ],
        );
        fs::write(work.join(file), content).unwrap();
        git(&work, &["add", "."]);
        git(
            &work,
            &[
                "-c",
                "user.name=Other",
                "-c",
                "user.email=other@example.test",
                "commit",
                "-qm",
                "another change on the forge",
            ],
        );
        git(&work, &["push", "-q", "origin", "main"]);
        git(&work, &["rev-parse", "HEAD"])
    }
    /// Says the branch head is in open pull request #7 into main, and
    /// whether GitHub has marked it merged.
    fn pull_request(&self, merged: bool) {
        fs::write(
            self.gh.path().join("pulls"),
            r#"[{"number":7,"state":"open","base":{"ref":"main"},"html_url":"https://github.com/owner/repo/pull/7"}]"#,
        )
        .unwrap();
        fs::write(
            self.gh.path().join("pull"),
            format!(r#"{{"number":7,"merged":{merged}}}"#),
        )
        .unwrap();
    }
    /// `merge --push` refuses with `expected`, ending with a verdict that
    /// says the base was not pushed, and the forge's base is unchanged.
    fn push_refuses(&self, args: &[&str], expected: &[&str], gh: bool) -> String {
        let before = self.remote_main();
        let (passed, output) = self.atelier(args, gh);
        assert!(!passed, "{output}");
        for text in expected {
            assert!(output.contains(text), "missing {text:?} in:\n{output}");
        }
        assert!(
            verdict(&output).starts_with("merge: refused")
                && (verdict(&output).ends_with(", nothing pushed")
                    || verdict(&output).ends_with(", main not pushed")),
            "{output}"
        );
        assert_eq!(self.remote_main(), before, "the forge's base moved");
        output
    }
    fn passes(&self, args: &[&str]) -> String {
        let (passed, output) = self.atelier(args, true);
        assert!(passed, "atelier {args:?}: {output}");
        output
    }
    /// `merge` refuses with `expected` and leaves the base alone.
    fn refuses(&self, expected: &[&str], gh: bool) -> String {
        let before = git(self.path(), &["rev-parse", "main"]);
        let (passed, output) = self.atelier(&["merge", &self.id], gh);
        assert!(!passed, "{output}");
        for text in expected {
            assert!(output.contains(text), "missing {text:?} in:\n{output}");
        }
        assert_eq!(git(self.path(), &["rev-parse", "main"]), before);
        output
    }
}

/// The verdict line `merge` prints last on standard output; `atelier()`
/// puts standard error after it.
fn verdict(output: &str) -> &str {
    output
        .lines()
        .rfind(|line| line.starts_with("merge: "))
        .unwrap_or_default()
}

/// A run: its ID, head commit, status, conclusion and pull request.
type Run<'a> = (u64, &'a str, &'a str, Option<&'a str>, Option<u64>);

fn runs(runs: &[Run<'_>]) -> String {
    let runs = runs
        .iter()
        .map(|(id, sha, status, conclusion, pull)| {
            serde_json::json!({
                "id": id,
                "status": status,
                "conclusion": conclusion,
                "html_url": format!("https://github.com/owner/repo/actions/runs/{id}"),
                "head_sha": sha,
                "event": "pull_request",
                "created_at": format!("2026-11-08T10:00:{:02}Z", id % 60),
                "pull_requests": pull.map(|number| vec![serde_json::json!({"number": number})]).unwrap_or_default(),
            })
        })
        .collect::<Vec<_>>();
    serde_json::json!({ "total_count": runs.len(), "workflow_runs": runs }).to_string()
}

/// A `PATH` holding only the tools atelier runs besides gh (`git`, `sh` and
/// `tar`), so no gh is found.
fn without_gh(scratch: &Path) -> PathBuf {
    let bin = scratch.join("no-gh");
    if !bin.exists() {
        fs::create_dir(&bin).unwrap();
        for tool in ["git", "sh", "tar"] {
            let found = String::from_utf8(
                Command::new("sh")
                    .args(["-c", &format!("command -v {tool}")])
                    .output()
                    .unwrap()
                    .stdout,
            )
            .unwrap();
            std::os::unix::fs::symlink(found.trim(), bin.join(tool)).unwrap();
        }
    }
    bin
}

fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

fn append_policy(repo: &Path, text: &str) {
    let path = repo.join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!("{policy}\n{text}")).unwrap();
}

fn owner(repo: &Path, title: &str) -> String {
    let id = ok(
        repo,
        &[
            "-q",
            "issue",
            "create",
            title,
            "--body",
            "## Goal\n\nShip change.\n\n## Done when\n\nScenario passes.",
        ],
    );
    commit(repo, "plan");
    id
}

fn report(repo: &Path, id: &str) {
    let store = atelier_records::RecordStore::new(repo.join(".atelier"));
    let mut record = store.load_issue_by_id(id).unwrap();
    record.sections.report = Some("complete".to_owned());
    store.write_issue_atomic(&record).unwrap();
}

fn approve(repo: &Path, id: &str) {
    ok(
        repo,
        &[
            "-q",
            "review",
            "approve",
            id,
            "--identity",
            "Reviewer",
            "Reviewed the committed change",
        ],
    );
    commit(repo, "approval");
}

fn short(commit: &str) -> &str {
    &commit[..8]
}

#[test]
fn ci_passed_passes_on_the_head_or_an_earlier_commit_with_its_code() {
    let forge = Forge::new("  - ci.passed\n");
    let head = forge.head();
    forge.run(&head, 11, "completed", Some("success"));
    let output = forge.passes(&["merge", &forge.id, "--check"]);
    assert!(
        output.contains(&format!("✓ {} ci.passed: ci.yml passed on pull request #7 for {}, which has the branch head's code, and main has gained no code since the branch left it: https://github.com/owner/repo/actions/runs/11\n", forge.id, short(&head))),
        "{output}"
    );
    assert!(
        forge.calls().contains("event=pull_request"),
        "{}",
        forge.calls()
    );

    // Only the implementation commit has a run; the report and approval
    // after it change records alone, so its run stands for the head.
    fs::remove_file(forge.gh.path().join("runs").join(&head)).unwrap();
    let implementation = git(forge.path(), &["rev-parse", "HEAD~2"]);
    forge.run(&implementation, 12, "completed", Some("success"));
    let output = forge.passes(&["merge", &forge.id]);
    let line = format!(
        "✓ {} ci.passed: ci.yml passed on pull request #7 for {}, which has the branch head's code",
        forge.id,
        short(&implementation)
    );
    assert!(output.contains(&line), "{output}");
    let message = git(forge.path(), &["log", "-1", "--format=%B", "main"]);
    assert!(message.contains(&line), "{message}");
    assert_eq!(
        git(forge.path(), &["rev-parse", "main^2"]),
        head,
        "the merge commit's parents are unchanged"
    );
}

#[test]
fn ci_passed_refuses_with_the_runs_state_and_link_or_when_no_run_exists() {
    let forge = Forge::new("  - ci.passed\n");
    let id = &forge.id;
    let url = "https://github.com/owner/repo/actions/runs/21";

    // A code change after a passing run leaves it behind: the walk stops at
    // the first commit whose code differs from the head's.
    let implementation = git(forge.path(), &["rev-parse", "HEAD~2"]);
    forge.run(&implementation, 20, "completed", Some("success"));
    fs::write(forge.path().join("src.rs"), "changed after the run\n").unwrap();
    commit(forge.path(), "code change after the run");
    approve(forge.path(), id);
    let head = forge.head();

    forge.run(&head, 21, "in_progress", None);
    forge.refuses(
        &[&format!("⚠ {id} ci.passed: ci.yml is in progress on pull request #7 for {}: {url}. Next: atelier merge {id} --wait", short(&head))],
        true,
    );
    forge.run(&head, 21, "queued", None);
    forge.refuses(&["ci.yml is queued on pull request #7"], true);
    forge.run(&head, 21, "completed", Some("failure"));
    forge.refuses(
        &[&format!("⚠ {id} ci.passed: ci.yml failed on pull request #7 for {}: {url}. Next: `gh run view 21 --log-failed`; fix it on the branch, commit, then `atelier merge {id} --push --wait`", short(&head))],
        true,
    );
    forge.run(&head, 21, "completed", Some("cancelled"));
    forge.refuses(
        &[
            "ci.yml was cancelled on pull request #7",
            &format!("Next: `gh run rerun 21`, then `atelier merge {id} --wait`"),
        ],
        true,
    );
    // The newest run decides when none passed.
    let older = git(forge.path(), &["rev-parse", "HEAD~1"]);
    fs::write(
        forge.gh.path().join("runs").join(&older),
        runs(&[(5, &older, "completed", Some("failure"), Some(7))]),
    )
    .unwrap();
    forge.run(&head, 21, "in_progress", None);
    forge.refuses(&["ci.yml is in progress"], true);
    fs::remove_file(forge.gh.path().join("runs").join(&older)).unwrap();

    fs::remove_file(forge.gh.path().join("runs").join(&head)).unwrap();
    forge.refuses(
        &[&format!("⚠ {id} ci.passed: no open pull request into main has {}, the branch head, so CI has not run on it. Next: atelier merge {id} --push --wait", short(&head))],
        true,
    );
    fs::write(
        forge.gh.path().join("pulls"),
        r#"[{"number":7,"state":"open","base":{"ref":"main"}}]"#,
    )
    .unwrap();
    forge.refuses(
        &[&format!(
            "⚠ {id} ci.passed: no pull_request run of ci.yml exists for {}, the branch head, or an earlier branch commit with its code. Next: atelier merge {id} --push --wait",
            short(&head)
        )],
        true,
    );
}

#[test]
fn ci_passed_names_gh_auth_login_when_gh_is_missing_or_signed_out() {
    let forge = Forge::new("  - ci.passed\n");
    let id = &forge.id;
    forge.refuses(
        &[&format!("⚠ {id} ci.passed: gh, the GitHub CLI, is not on PATH, so Atelier cannot read CI. Next: install the GitHub CLI (https://cli.github.com), then `gh auth login`")],
        false,
    );
    fs::write(forge.gh.path().join("signed-out"), "").unwrap();
    forge.refuses(
        &[&format!("⚠ {id} ci.passed: gh is not signed in to github.com, so Atelier cannot read CI. Next: `gh auth login`")],
        true,
    );
}

#[test]
fn a_branch_that_changes_only_records_needs_no_run_and_no_gh() {
    let dir = tempdir().unwrap();
    let repo = dir.path();
    git(repo, &["init", "-qb", "main"]);
    git(repo, &["config", "user.name", "Author"]);
    git(repo, &["config", "user.email", "author@example.test"]);
    ok(repo, &["init", "--slug", "atelier"]);
    append_policy(repo, &format!("{FORGE}merge_checks:\n  - ci.passed\n"));
    commit(repo, "base policy with a forge");
    let id = owner(repo, "Owner");
    ok(repo, &["start", &id]);
    report(repo, &id);
    commit(repo, "report");
    approve(repo, &id);
    let scratch = tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(repo)
        .env("PATH", without_gh(scratch.path()))
        .args(["merge", &id])
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "{text}{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        text.contains(&format!(
            "✓ {id} ci.passed: the branch changes only records, so it needs no CI run.\n"
        )),
        "{text}"
    );
}

#[test]
fn a_docs_only_base_change_after_the_run_needs_no_new_run_and_shell_checks_run_on_the_merged_tree()
{
    // Only the merged tree has both the base's note and the branch's code.
    let forge = Forge::new(
        "  - shell.check: { command: 'test -f docs/note.md && test -f src.rs' }\n  - ci.passed\n",
    );
    let head = forge.head();
    forge.run(&head, 31, "completed", Some("success"));
    git(forge.path(), &["switch", "-q", "main"]);
    fs::create_dir(forge.path().join("docs")).unwrap();
    fs::write(forge.path().join("docs/note.md"), "a note\n").unwrap();
    commit(forge.path(), "docs on the base after the run");
    git(forge.path(), &["switch", "-q", &forge.branch]);

    let output = forge.passes(&["merge", &forge.id, "--check"]);
    assert!(
        output.contains("shell.check, merge.merged_tree")
            && output.contains("ci.passed: ci.yml passed"),
        "{output}"
    );
    forge.passes(&["merge", &forge.id]);
    let message = git(forge.path(), &["log", "-1", "--format=%B", "main"]);
    assert!(
        message.contains("- shell.check: test -f docs/note.md && test -f src.rs")
            && message.contains("ci.passed: ci.yml passed on pull request #7"),
        "{message}"
    );
    // Only the branch's commits were asked about: no new run was needed.
    assert!(
        forge
            .calls()
            .lines()
            .filter(|call| call.contains("head_sha="))
            .all(|call| call.contains(&head)),
        "{}",
        forge.calls()
    );
}

#[test]
fn a_code_base_change_after_the_run_still_passes_and_says_base_ci_covers_the_combination() {
    let forge = Forge::new("  - ci.passed\n");
    let head = forge.head();
    forge.run(&head, 41, "completed", Some("success"));
    git(forge.path(), &["switch", "-q", "main"]);
    fs::write(forge.path().join("other.rs"), "landed on the base\n").unwrap();
    commit(forge.path(), "code on the base after the run");
    let base = git(forge.path(), &["rev-parse", "HEAD"]);
    git(forge.path(), &["switch", "-q", &forge.branch]);
    // The base's own CI failed; that is shown and refuses nothing.
    fs::write(
        forge.gh.path().join("base"),
        runs(&[(40, &base, "completed", Some("failure"), None)]),
    )
    .unwrap();

    let output = forge.passes(&["merge", &forge.id]);
    let line = format!(
        "✓ {} ci.passed: ci.yml passed on pull request #7 for {}, which has the branch head's code: https://github.com/owner/repo/actions/runs/41. That run tested the head merged with main as it was when the run started, not the tree this merge makes. main has gained code since the branch left it, and CI on main after the push covers the combination.",
        forge.id,
        short(&head)
    );
    assert!(output.contains(&line), "{output}");
    assert!(
        output.contains(&format!(
            "Note: ci.yml failed on main at {}: https://github.com/owner/repo/actions/runs/40. It refuses nothing.",
            short(&base)
        )),
        "{output}"
    );
    let message = git(forge.path(), &["log", "-1", "--format=%B", "main"]);
    assert!(message.contains(&line), "{message}");
}

#[test]
fn atelier_check_refuses_a_malformed_forge_naming_the_key() {
    let dir = tempdir().unwrap();
    let repo = dir.path();
    git(repo, &["init", "-qb", "main"]);
    ok(repo, &["init", "--slug", "atelier"]);
    let path = repo.join(".atelier/workflow.yaml");
    let starter = fs::read_to_string(&path).unwrap();
    for (policy, code, field) in [
        (
            "forge:\n  ci_workflow: ci.yml\n",
            "workflow_config_invalid_forge",
            "forge.provider",
        ),
        (
            "forge:\n  provider: gitlab\n  ci_workflow: ci.yml\n",
            "workflow_config_invalid_forge",
            "forge.provider",
        ),
        (
            "forge:\n  provider: github\n",
            "workflow_config_invalid_forge",
            "forge.ci_workflow",
        ),
        (
            "forge:\n  provider: github\n  ci_workflow: ''\n",
            "workflow_config_invalid_forge",
            "forge.ci_workflow",
        ),
        (
            "forge:\n  provider: github\n  ci_workflow: ci.yml\n  remote: ''\n",
            "workflow_config_invalid_forge",
            "forge.remote",
        ),
        (
            "forge:\n  provider: github\n  ci_workflow: ci.yml\n  token: x\n",
            "workflow_config_invalid_forge",
            "forge.token",
        ),
        (
            "merge_checks:\n  - ci.passed\n",
            "workflow_config_invalid_validator",
            "merge_checks",
        ),
    ] {
        fs::write(&path, format!("{starter}\n{policy}")).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
            .current_dir(repo)
            .arg("check")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(!output.status.success(), "{policy}: {text}");
        assert!(
            text.contains(&format!("error: {code}")) && text.contains(&format!("field: {field}\n")),
            "{policy}: {text}"
        );
    }
    fs::write(
        &path,
        format!("{starter}\n{FORGE}merge_checks:\n  - ci.passed\n"),
    )
    .unwrap();
    ok(repo, &["check"]);
}

#[test]
fn switching_the_policy_to_ci_passed_takes_effect_on_the_next_merge() {
    // The base runs a CI script, as this repository did with
    // scripts/merge-ci-passed.sh.
    let dir = tempdir().unwrap();
    let repo = dir.path();
    git(repo, &["init", "-qb", "main"]);
    git(repo, &["config", "user.name", "Author"]);
    git(repo, &["config", "user.email", "author@example.test"]);
    git(
        repo,
        &["remote", "add", "origin", "git@github.com:owner/repo.git"],
    );
    ok(repo, &["init", "--slug", "atelier"]);
    fs::write(repo.join("ci-script.sh"), "test -f src.rs\n").unwrap();
    append_policy(
        repo,
        "merge_checks:\n  - shell.check: { command: 'sh ci-script.sh' }\n",
    );
    commit(repo, "base runs a CI script");
    let switch = owner(repo, "Switch to ci.passed");
    ok(repo, &["start", &switch]);
    fs::write(repo.join("src.rs"), "code\n").unwrap();
    let path = repo.join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap().replace(
        "merge_checks:\n  - shell.check: { command: 'sh ci-script.sh' }\n",
        &format!("{FORGE}merge_checks:\n  - ci.passed\n"),
    );
    fs::write(&path, policy).unwrap();
    commit(repo, "switch the policy");
    report(repo, &switch);
    commit(repo, "report");
    approve(repo, &switch);

    let forge = Forge {
        dir,
        gh: tempdir().unwrap(),
        id: switch.clone(),
        branch: String::new(),
    };
    fs::create_dir_all(forge.gh.path().join("bin")).unwrap();
    fs::create_dir_all(forge.gh.path().join("runs")).unwrap();
    fs::write(forge.gh.path().join("bin/gh"), FAKE_GH).unwrap();
    make_executable(&forge.gh.path().join("bin/gh"));

    // The switch merges under the base's policy: its script, not ci.passed.
    let output = forge.passes(&["merge", &switch, "--check"]);
    assert!(
        output.contains("shell.check") && !output.contains("ci.passed"),
        "{output}"
    );
    forge.passes(&["merge", &switch]);
    assert_eq!(
        forge.calls(),
        "",
        "the switch's merge asked the forge nothing"
    );
    let message = git(forge.path(), &["log", "-1", "--format=%B", "main"]);
    assert!(
        message.contains("- shell.check: sh ci-script.sh"),
        "{message}"
    );

    // The next branch merges onto the new base, under ci.passed.
    let next = owner(forge.path(), "Next change");
    ok(forge.path(), &["start", &next]);
    fs::write(forge.path().join("src.rs"), "more code\n").unwrap();
    commit(forge.path(), "next implementation");
    report(forge.path(), &next);
    commit(forge.path(), "report");
    approve(forge.path(), &next);
    let head = forge.head();
    forge.run(&head, 51, "completed", Some("success"));
    let output = forge.passes(&["merge", &next]);
    assert!(
        output.contains(&format!(
            "✓ {next} ci.passed: ci.yml passed on pull request #7 for {}",
            short(&head)
        )),
        "{output}"
    );
    assert!(
        forge.calls().contains(&format!("head_sha={head}")),
        "{}",
        forge.calls()
    );
}

#[test]
fn a_remote_name_with_a_slash_is_read_as_the_remote_not_a_url() {
    let forge = Forge::with_forge(
        "forge:\n  provider: github\n  remote: upstream/github\n  ci_workflow: ci.yml\n",
        "  - ci.passed\n",
    );
    git(
        forge.path(),
        &[
            "remote",
            "add",
            "upstream/github",
            "git@github.com:other/repo.git",
        ],
    );
    let head = forge.head();
    forge.run(&head, 61, "completed", Some("success"));
    let output = forge.passes(&["merge", &forge.id, "--check"]);
    assert!(output.contains("ci.passed: ci.yml passed"), "{output}");
    assert!(
        forge
            .calls()
            .contains("repos/other/repo/actions/workflows/ci.yml/runs"),
        "{}",
        forge.calls()
    );
}

/// A work branch an older Atelier marked done is landed with the merge
/// checks run in its own checkout. Its HEAD can be a merge of the base, and
/// ci.passed still judges the branch's code, not the merged-in base's
/// (finding atelier-oqrm).
#[test]
fn land_on_a_done_branch_whose_head_merges_the_base_still_judges_the_branch_code() {
    let forge = Forge::new("  - ci.passed\n");
    let store = atelier_records::RecordStore::new(forge.path().join(".atelier"));
    let mut record = store.load_issue_by_id(&forge.id).unwrap();
    record.issue.status = "done".into();
    record.issue.closed_at = Some(chrono::Utc::now());
    store.write_issue_atomic(&record).unwrap();
    commit(
        forge.path(),
        "an older Atelier closes the work on its branch",
    );
    approve(forge.path(), &forge.id);
    let refused = |output: &str| {
        output.contains(&format!("⚠ {} ci.passed: no open pull request", forge.id))
            && !output.contains("changes only records")
            && output.contains("land: work in progress")
    };
    // land reports work in progress, exiting zero, while a check is open.
    let (_, output) = forge.atelier(&["land"], true);
    assert!(refused(&output), "{output}");

    // A record-only commit on the base, merged into the branch.
    git(forge.path(), &["switch", "-q", "main"]);
    owner(forge.path(), "Unrelated plan");
    git(forge.path(), &["switch", "-q", &forge.branch]);
    git(forge.path(), &["merge", "-q", "--no-edit", "main"]);
    let (_, output) = forge.atelier(&["land"], true);
    assert!(refused(&output), "{output}");
}

/// `merge --push` fetches the base the forge has, fast-forwards the local
/// base onto it, merges, judges the base as `land` does, pushes it and
/// confirms the pull request merged.
#[test]
fn merge_push_publishes_the_merge_and_confirms_the_pull_request_merged() {
    let forge = Forge::new("  - ci.passed\n");
    let head = forge.head();
    forge.run(&head, 31, "completed", Some("success"));
    forge.pull_request(true);
    let elsewhere = forge.push_to_remote_main("other.txt");

    let output = forge.passes(&["merge", &forge.id, "--push"]);
    assert!(
        output.contains(&format!(
            "Fast-forwarded main to {} from origin.",
            short(&elsewhere)
        )),
        "{output}"
    );
    // The approval commit is the one record-only commit no review covers,
    // as `land` on the base would count it.
    assert!(
        output.contains("Base commits: 1 unpushed record-only commit outside review.\n"),
        "{output}"
    );
    assert_eq!(
        verdict(&output),
        "merge: passed — pushed main to origin, pull request #7 merged",
        "{output}"
    );
    let main = git(forge.path(), &["rev-parse", "main"]);
    assert_eq!(forge.remote_main(), main, "the forge has the merge commit");
    assert_eq!(git(forge.path(), &["rev-parse", "main^1"]), elsewhere);
    assert_eq!(git(forge.path(), &["rev-parse", "main^2"]), head);
    assert!(
        forge.calls().starts_with("auth status"),
        "{}",
        forge.calls()
    );
    assert!(
        forge.calls().contains(&format!(
            "repos/owner/repo/pulls -f head=owner:{} -f base=main -f state=open",
            forge.branch
        )) && forge.calls().contains("repos/owner/repo/pulls/7"),
        "{}",
        forge.calls()
    );
}

/// With no ID, `merge --push` publishes an approved branch no issue owns,
/// here through a `forge.remote` written as a URL.
#[test]
fn merge_push_publishes_an_approved_light_path_branch_to_a_url_remote() {
    let forge = Forge::with_forge(
        "forge:\n  provider: github\n  remote: git@github.com:owner/repo.git\n  ci_workflow: ci.yml\n",
        "  - ci.passed\n",
    );
    let repo = forge.path();
    git(repo, &["switch", "-qc", "docs", "main"]);
    fs::write(repo.join("guide.md"), "a guide\n").unwrap();
    commit(repo, "write a guide");
    forge.run(&forge.head(), 41, "completed", Some("success"));
    ok(
        repo,
        &[
            "-q",
            "review",
            "approve",
            "--branch",
            "docs",
            "--identity",
            "Reviewer",
            "--commit-records",
            "Read the branch",
        ],
    );
    let head = forge.head();
    forge.pull_request(true);

    let output = forge.passes(&["merge", "--push"]);
    assert_eq!(
        verdict(&output),
        "merge: passed — pushed main to git@github.com:owner/repo.git, pull request #7 merged",
        "{output}"
    );
    assert_eq!(forge.remote_main(), git(repo, &["rev-parse", "main"]));
    assert_eq!(git(repo, &["rev-parse", "main^2"]), head);
}

/// A missing or signed-out gh pushes nothing at all; a gate that refuses
/// leaves the branch and its pull request pushed, and the base unpushed.
#[test]
fn merge_push_pushes_nothing_without_gh_and_no_base_when_a_gate_refuses() {
    let forge = Forge::new("  - ci.passed\n");
    let id = forge.id.as_str();
    let head = forge.head();
    forge.pull_request(true);
    forge.push_refuses(
        &["merge", id, "--push"],
        &[
            "forge_gh_missing: gh, the GitHub CLI, is not on PATH",
            "merge: refused — 1 check (forge_gh_missing), nothing pushed",
        ],
        false,
    );
    fs::write(forge.gh.path().join("signed-out"), "").unwrap();
    forge.push_refuses(
        &["merge", id, "--push"],
        &[
            "forge_gh_signed_out: gh is not signed in to github.com",
            "Next: `gh auth login`",
            "merge: refused — 1 check (forge_gh_signed_out), nothing pushed",
        ],
        true,
    );
    // The signed-out gh was asked only whether it is signed in.
    assert_eq!(forge.calls(), "auth status --hostname github.com\n");
    assert_eq!(forge.remote_branch(), None, "the branch was pushed");

    fs::remove_file(forge.gh.path().join("signed-out")).unwrap();
    forge.run(&head, 51, "in_progress", None);
    forge.push_refuses(
        &["merge", id, "--push"],
        &[
            "Pull request #7: https://github.com/owner/repo/pull/7 (already open)",
            "ci.yml is in progress",
            "merge: refused — 1 check (ci.passed), main not pushed",
        ],
        true,
    );
    assert_eq!(forge.remote_branch(), Some(head.clone()));
    assert_eq!(
        git(forge.path(), &["rev-parse", "HEAD"]),
        head,
        "nothing merged"
    );
}

/// `merge --push` on an approved branch the forge has never seen pushes it
/// and opens its pull request, refuses while CI is pending, and on a rerun
/// reuses the pull request and publishes once CI passes.
#[test]
fn merge_push_pushes_the_branch_and_opens_its_pull_request_then_publishes_once_ci_passes() {
    let forge = Forge::new("  - ci.passed\n");
    let id = forge.id.as_str();
    let head = forge.head();
    assert_eq!(forge.remote_branch(), None);
    forge.run(&head, 141, "in_progress", None);

    let output = forge.push_refuses(
        &["merge", id, "--push"],
        &[
            "Pull request #7: https://github.com/owner/repo/pull/7 (opened now)\n",
            &format!("⚠ {id} ci.passed: ci.yml is in progress on pull request #7 for {}: https://github.com/owner/repo/actions/runs/141. Next: atelier merge {id} --push --wait", short(&head)),
            "merge: refused — 1 check (ci.passed), main not pushed",
        ],
        true,
    );
    assert_eq!(forge.remote_branch(), Some(head.clone()), "{output}");
    // Never forced, and the branch tracks the forge's copy.
    assert!(!forge.calls().contains("force"), "{}", forge.calls());
    assert_eq!(
        git(forge.path(), &["rev-parse", "--abbrev-ref", "@{upstream}"]),
        format!("origin/{}", forge.branch)
    );
    let opened = forge.opened();
    assert_eq!(opened.len(), 1, "{opened:?}");
    assert!(
        opened[0].contains("repos/owner/repo/pulls")
            && opened[0].contains(&format!(
                "-f title={id}: Owner -f head={} -f base=main -f body=Atelier item {id}.",
                forge.branch
            ))
            && forge.calls().contains(
                "Review is recorded in `.atelier/reviews` on the branch, not on this pull request."
            ),
        "{opened:?}"
    );

    forge.run(&head, 141, "completed", Some("success"));
    let output = forge.passes(&["merge", id, "--push"]);
    assert!(
        output.contains("Pull request #7: https://github.com/owner/repo/pull/7 (already open)\n"),
        "{output}"
    );
    assert_eq!(forge.opened().len(), 1, "a second pull request was opened");
    assert_eq!(
        verdict(&output),
        "merge: passed — pushed main to origin, pull request #7 merged",
        "{output}"
    );
    assert_eq!(
        forge.remote_main(),
        git(forge.path(), &["rev-parse", "main"])
    );
    assert_eq!(git(forge.path(), &["rev-parse", "main^2"]), head);
}

/// A light-path branch's pull request takes the subject of its newest
/// commit that changes code, not the approval after it.
#[test]
fn merge_push_titles_a_light_path_pull_request_by_its_newest_code_commit() {
    let forge = Forge::new("  - ci.passed\n");
    let repo = forge.path();
    git(repo, &["switch", "-qc", "docs", "main"]);
    fs::write(repo.join("guide.md"), "a guide\n").unwrap();
    commit(repo, "Write a guide");
    forge.run(&forge.head(), 151, "completed", Some("success"));
    ok(
        repo,
        &[
            "-q",
            "review",
            "approve",
            "--branch",
            "docs",
            "--identity",
            "Reviewer",
            "--commit-records",
            "Read the branch",
        ],
    );
    let output = forge.passes(&["merge", "--push"]);
    let opened = forge.opened();
    assert!(
        opened.len() == 1
            && opened[0].contains("-f title=Write a guide -f head=docs -f base=main -f body=Branch docs, which no Atelier item owns."),
        "{opened:?}\n{output}"
    );
}

/// Steps 2 and 3 run before the gates, so a branch whose approval is stale
/// is still pushed with a pull request, and the base is not.
#[test]
fn merge_push_pushes_the_branch_and_opens_its_pull_request_when_a_gate_refuses() {
    let forge = Forge::new("  - ci.passed\n");
    fs::write(forge.path().join("src.rs"), "changed after the approval\n").unwrap();
    commit(forge.path(), "code change after the approval");
    let head = forge.head();
    forge.run(&head, 161, "completed", Some("success"));

    let output = forge.push_refuses(
        &["merge", &forge.id, "--push"],
        &[
            "Pull request #7: https://github.com/owner/repo/pull/7 (opened now)",
            "review.current",
            "main not pushed",
        ],
        true,
    );
    assert_eq!(forge.remote_branch(), Some(head), "{output}");
    assert_eq!(forge.opened().len(), 1, "{output}");
}

/// A forge copy of the branch with commits this one lacks rejects the
/// push: merge refuses before the pull request and the base, never forcing.
#[test]
fn merge_push_refuses_a_rejected_branch_push_and_pushes_no_base() {
    let forge = Forge::new("  - ci.passed\n");
    let head = forge.head();
    forge.run(&head, 171, "completed", Some("success"));
    // Another commit on the forge's copy of the branch.
    let bare = forge.remote().to_string_lossy().into_owned();
    git(
        forge.path(),
        &["push", "-q", &bare, "HEAD~1:refs/heads/elsewhere"],
    );
    let clone = tempdir().unwrap();
    let work = clone.path().join("work");
    git(
        forge.path(),
        &[
            "clone",
            "-q",
            "-b",
            "elsewhere",
            &bare,
            &work.to_string_lossy(),
        ],
    );
    fs::write(work.join("other.rs"), "another agent's change\n").unwrap();
    git(&work, &["add", "."]);
    git(
        &work,
        &[
            "-c",
            "user.name=Other",
            "-c",
            "user.email=other@example.test",
            "commit",
            "-qm",
            "theirs",
        ],
    );
    git(
        &work,
        &[
            "push",
            "-q",
            "origin",
            &format!("HEAD:refs/heads/{}", forge.branch),
        ],
    );
    let theirs = forge.remote_branch().unwrap();

    let id = forge.id.as_str();
    let branch = forge.branch.as_str();
    forge.push_refuses(
        &["merge", id, "--push"],
        &[
            &format!("merge_push_branch_rejected: git push to origin refused {branch}: "),
            &format!("Next: `git fetch origin {branch} && git merge --no-edit FETCH_HEAD`, then `atelier merge {id} --push`"),
            "The remote branch has commits this one lacks; nothing was pushed.",
            "merge: refused — 1 check (merge_push_branch_rejected), nothing pushed",
        ],
        true,
    );
    assert_eq!(forge.remote_branch(), Some(theirs), "the push was forced");
    assert!(forge.opened().is_empty(), "{:?}", forge.opened());
    assert_eq!(
        git(forge.path(), &["rev-parse", "HEAD"]),
        head,
        "nothing merged"
    );
}

/// A branch push that fails for another reason, such as a remote that
/// wants credentials, is not called a rejection and names no merge.
#[test]
fn merge_push_names_a_failed_branch_push_apart_from_a_rejected_one() {
    let forge = Forge::new("  - ci.passed\n");
    let id = forge.id.as_str();
    let branch = forge.branch.as_str();
    let before = forge.remote_main();
    let (passed, output) = forge.atelier_env(
        &["merge", id, "--push"],
        true,
        &[("GIT_SSH_COMMAND", "false")],
    );
    assert!(!passed, "{output}");
    assert!(
        output.contains(&format!("merge_push_branch_failed: git push to origin could not push {branch}: "))
            && output.contains(&format!("; nothing was pushed. Next: check that `git push origin {branch}` works, then `atelier merge {id} --push`")),
        "{output}"
    );
    assert!(!output.contains("FETCH_HEAD"), "{output}");
    assert_eq!(
        verdict(&output),
        "merge: refused — 1 check (merge_push_branch_failed), nothing pushed",
        "{output}"
    );
    assert_eq!(forge.remote_main(), before);
    assert!(forge.opened().is_empty(), "{:?}", forge.opened());
}

/// `--push --wait` polls a pending run, printing each state it sees, and
/// merges and publishes when it passes.
#[cfg(feature = "test-hooks")]
#[test]
fn merge_push_wait_merges_and_publishes_once_pending_ci_passes() {
    let forge = Forge::new("  - ci.passed\n");
    let head = forge.head();
    forge.run_sequence(
        &head,
        181,
        &[
            ("queued", None),
            ("in_progress", None),
            ("completed", Some("success")),
        ],
    );
    let output = forge.passes(&["merge", &forge.id, "--push", "--wait"]);
    let url = "https://github.com/owner/repo/actions/runs/181";
    for line in [
        format!("Waiting up to 30 minutes for CI: ci.yml is in progress on pull request #7 for {}: {url}\n", short(&head)),
        format!("CI: ci.yml passed on pull request #7 for {}", short(&head)),
    ] {
        assert!(output.contains(&line), "missing {line:?} in:\n{output}");
    }
    assert_eq!(
        verdict(&output),
        "merge: passed — pushed main to origin, pull request #7 merged",
        "{output}"
    );
    assert_eq!(
        forge.remote_main(),
        git(forge.path(), &["rev-parse", "main"])
    );
}

/// `--wait` ends on a failed run, or at its timeout naming how long it
/// waited and keeping `--push`, and never waits on a run that finished.
#[test]
fn merge_wait_refuses_on_a_failed_run_or_at_its_timeout() {
    let forge = Forge::new("  - ci.passed\n");
    let id = forge.id.as_str();
    let head = forge.head();
    let url = "https://github.com/owner/repo/actions/runs/191";
    forge.run(&head, 191, "completed", Some("failure"));
    let output = forge.push_refuses(
        &["merge", id, "--push", "--wait"],
        &[&format!("⚠ {id} ci.passed: ci.yml failed on pull request #7 for {}: {url}. Next: `gh run view 191 --log-failed`; fix it on the branch, commit, then `atelier merge {id} --push --wait`", short(&head))],
        true,
    );
    assert!(!output.contains("Waiting"), "{output}");

    forge.run(&head, 191, "in_progress", None);
    forge.push_refuses(
        &["merge", id, "--push", "--wait", "--timeout", "0"],
        &[
            "Waiting up to 0 seconds for CI: ci.yml is in progress",
            &format!("⚠ {id} ci.passed: ci.yml is in progress on pull request #7 for {}: {url}; merge waited 0 seconds for it. Next: atelier merge {id} --push --wait", short(&head)),
            "merge: refused — 1 check (ci.passed), main not pushed",
        ],
        true,
    );
}

/// A run that never starts is pending only for the grace period, then
/// refuses as no run.
#[cfg(feature = "test-hooks")]
#[test]
fn merge_push_wait_refuses_a_run_that_never_starts_after_its_grace_period() {
    let forge = Forge::new("  - ci.passed\n");
    let id = forge.id.as_str();
    let before = forge.remote_main();
    let (passed, output) = forge.atelier_env(
        &["merge", id, "--push", "--wait"],
        true,
        &[("ATELIER_TEST_MERGE_CI_GRACE_MS", "50")],
    );
    assert!(!passed, "{output}");
    assert!(
        output.contains(&format!("⚠ {id} ci.passed: no pull_request run of ci.yml exists for {}, the branch head, or an earlier branch commit with its code, and none started within 0 seconds of waiting. Next: atelier merge {id} --push --wait", short(&forge.head()))),
        "{output}"
    );
    assert_eq!(forge.remote_main(), before);
}

/// `--wait` without `--push` waits for CI on a branch already pushed, then
/// merges without pushing anything.
#[cfg(feature = "test-hooks")]
#[test]
fn merge_wait_without_push_waits_then_merges_without_pushing() {
    let forge = Forge::new("  - ci.passed\n");
    let head = forge.head();
    forge.run_sequence(
        &head,
        201,
        &[
            ("in_progress", None),
            ("in_progress", None),
            ("completed", Some("success")),
        ],
    );
    let before = forge.remote_main();
    let output = forge.passes(&["merge", &forge.id, "--wait"]);
    assert!(
        output.contains("Waiting up to 30 minutes for CI: ci.yml is in progress"),
        "{output}"
    );
    assert_eq!(verdict(&output), "merge: passed", "{output}");
    assert_eq!(git(forge.path(), &["rev-parse", "main^2"]), head);
    assert_eq!(forge.remote_main(), before, "the base was pushed");
    assert_eq!(forge.remote_branch(), None, "the branch was pushed");
    assert!(forge.opened().is_empty(), "{:?}", forge.opened());
}

/// `--push`, `--wait` and `--timeout` do not combine with `--check`,
/// `--json` or `--sync`, and `--timeout` needs `--wait`.
#[test]
fn push_wait_and_timeout_refuse_check_json_sync_and_a_timeout_without_wait() {
    let forge = Forge::new("  - ci.passed\n");
    let id = forge.id.as_str();
    for flag in ["--push", "--wait", "--timeout"] {
        for other in ["--check", "--json", "--sync"] {
            let mut args = vec!["merge", id, flag];
            if flag == "--timeout" {
                args.extend(["5", "--wait"]);
            }
            args.push(other);
            let (passed, output) = forge.atelier(&args, true);
            assert!(
                !passed && output.contains("cannot be used with"),
                "{args:?}: {output}"
            );
        }
    }
    let (passed, output) = forge.atelier(&["merge", id, "--timeout", "5"], true);
    assert!(
        !passed && output.contains("the following required arguments were not provided:\n  --wait"),
        "{output}"
    );
    assert_eq!(forge.calls(), "", "a refused flag reached gh");
}

#[test]
fn merge_push_refuses_a_local_base_that_diverged_from_the_forge_before_merging() {
    let forge = Forge::new("  - ci.passed\n");
    let head = forge.head();
    forge.run(&head, 61, "completed", Some("success"));
    forge.pull_request(true);
    let remote = forge.push_to_remote_main("other.txt");
    // A record-only commit on the local base that the forge lacks.
    let repo = forge.path();
    git(repo, &["switch", "-q", "main"]);
    owner(repo, "A plan on the local base");
    let local = git(repo, &["rev-parse", "main"]);
    git(repo, &["switch", "-q", &forge.branch]);

    forge.push_refuses(
        &["merge", &forge.id, "--push"],
        &[
            &format!(
                "merge_push_base_diverged: local main at {} and main on origin at {} have diverged",
                short(&local),
                short(&remote)
            ),
            "Next: in the main checkout, `git fetch origin main && git merge --no-edit FETCH_HEAD` and `atelier land`, then `git push origin main`",
            "merge: refused — 1 check (merge_push_base_diverged), main not pushed",
        ],
        true,
    );
    assert_eq!(git(repo, &["rev-parse", "main"]), local, "nothing merged");
    assert_eq!(git(repo, &["rev-parse", "HEAD"]), head);
}

/// The forge's base moves while the gates run, so its push is rejected: the
/// merge commit stays on the local base and the Next says how to publish it.
#[test]
fn merge_push_keeps_the_merge_when_the_forge_rejects_the_base_push() {
    let forge = Forge::new("  - ci.passed\n");
    let head = forge.head();
    forge.run(&head, 71, "completed", Some("success"));
    forge.pull_request(true);
    let moved = forge.push_to_remote_main("other.txt");
    // Undo it on the forge, and have the CI lookup redo it mid-merge.
    let bare = forge.remote().to_string_lossy().into_owned();
    let before = git(forge.path(), &["--git-dir", &bare, "rev-parse", "main^"]);
    git(
        forge.path(),
        &["--git-dir", &bare, "update-ref", "refs/heads/main", &before],
    );
    fs::write(
        forge.gh.path().join("on-runs"),
        format!("git --git-dir '{bare}' update-ref refs/heads/main {moved}\n"),
    )
    .unwrap();

    let (passed, output) = forge.atelier(&["merge", &forge.id, "--push"], true);
    assert!(!passed, "{output}");
    assert!(
        output.contains("merge_push_base_rejected: git push to origin refused main"),
        "{output}"
    );
    assert!(
        output.contains("Next: in the main checkout, `git fetch origin main && git merge --no-edit FETCH_HEAD`, then `atelier land`, then `git push origin main`"),
        "{output}"
    );
    assert!(!output.contains("land --push"), "{output}");
    assert_eq!(
        verdict(&output),
        "merge: refused — 1 check (merge_push_base_rejected), main not pushed",
        "{output}"
    );
    assert_eq!(forge.remote_main(), moved);
    assert_eq!(
        git(forge.path(), &["rev-parse", "main^2"]),
        head,
        "the merge stays"
    );
}

/// The base judgment `land` makes runs before the push: an unreviewed code
/// commit on the local base refuses, and the merge commit stays local.
#[test]
fn merge_push_judges_the_unpushed_base_commits_as_land_does() {
    let forge = Forge::new("  - ci.passed\n");
    let head = forge.head();
    forge.run(&head, 81, "completed", Some("success"));
    forge.pull_request(true);
    let repo = forge.path();
    git(repo, &["switch", "-q", "main"]);
    fs::write(repo.join("unreviewed.rs"), "code\n").unwrap();
    commit(repo, "code committed on the base directly");
    git(repo, &["switch", "-q", &forge.branch]);

    forge.push_refuses(
        &["merge", &forge.id, "--push"],
        &[
            "land_base_unreviewed: commit",
            "main was not pushed; merge commit",
            "merge: refused — 1 check (land_base_unreviewed), main not pushed",
        ],
        true,
    );
    assert_eq!(git(repo, &["rev-parse", "main^2"]), head, "the merge stays");
}

/// When GitHub has not marked the pull request merged within the wait, the
/// base is still published, so merge passes and says so, closing nothing.
#[cfg(feature = "test-hooks")]
#[test]
fn merge_push_passes_with_a_note_when_the_pull_request_is_still_open() {
    let forge = Forge::new("  - ci.passed\n");
    let head = forge.head();
    forge.run(&head, 91, "completed", Some("success"));
    forge.pull_request(false);

    let output = forge.passes(&["merge", &forge.id, "--push"]);
    let main = git(forge.path(), &["rev-parse", "main"]);
    assert!(
        output.contains(&format!("Note: pushed main to origin at {}; pull request #7 is still open because GitHub has not marked it merged. Atelier does not close it, since closed is not merged. Next: gh pr view 7\n", short(&main))),
        "{output}"
    );
    assert_eq!(
        verdict(&output),
        "merge: passed — pushed main to origin",
        "{output}"
    );
    assert_eq!(forge.remote_main(), main);
    assert!(!forge.calls().contains("close"), "{}", forge.calls());
}

/// Without a forge, `--push` refuses and pushes nothing; plain `merge`
/// names `land --push` as its next step.
#[test]
fn merge_push_needs_a_forge_and_plain_merge_names_land_push() {
    let forge = Forge::with_forge("", "  - blockers.none_open\n");
    forge.push_refuses(
        &["merge", &forge.id, "--push"],
        &[&format!("merge_push_no_forge: the main policy names no forge, so merge --push has nowhere to publish; nothing was pushed. Next: `atelier merge {}`, then `atelier land --push` from the main checkout", forge.id)],
        true,
    );
    let output = forge.passes(&["merge", &forge.id]);
    assert!(
        output.contains("; branch deleted. Next: atelier land --push from the main checkout.\n"),
        "{output}"
    );
    assert_eq!(verdict(&output), "merge: passed", "{output}");
}

#[test]
fn plain_merge_in_a_forge_policy_names_merge_push_for_next_time() {
    let forge = Forge::new("  - ci.passed\n");
    forge.run(&forge.head(), 95, "completed", Some("success"));
    let output = forge.passes(&["merge", &forge.id]);
    assert!(
        output.contains(&format!("; branch deleted. Next: atelier land --push from the main checkout; next time, `atelier merge {} --push` publishes it in one step.\n", forge.id)),
        "{output}"
    );
    assert_eq!(
        forge.remote_main(),
        git(forge.path(), &["rev-parse", "main^1"]),
        "nothing pushed"
    );
}

/// A base fast-forward never overwrites what `merge` would refuse to: an
/// ignored, uncommitted tracker record in a separate base checkout, where
/// the forge's base commits a record at the same path (finding atelier-kl16).
#[test]
fn merge_push_refuses_to_fast_forward_a_base_checkout_with_an_uncommitted_record() {
    let forge = Forge::new("  - ci.passed\n");
    let head = forge.head();
    forge.run(&head, 101, "completed", Some("success"));
    forge.pull_request(true);
    let repo = forge.path();
    let checkouts = tempdir().unwrap();
    let base = checkouts.path().join("base");
    git(
        repo,
        &["worktree", "add", "-q", &base.to_string_lossy(), "main"],
    );
    let record = ".atelier/issues/atelier-zz00.md";
    let common = git(
        repo,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    );
    fs::create_dir_all(Path::new(&common).join("info")).unwrap();
    fs::write(
        Path::new(&common).join("info/exclude"),
        format!("/{record}\n"),
    )
    .unwrap();
    fs::write(base.join(record), "an unsaved local plan\n").unwrap();
    forge.push_to_remote_main_with(record, "the forge's record\n");
    let local = git(repo, &["rev-parse", "main"]);

    forge.push_refuses(
        &["merge", &forge.id, "--push"],
        &[
            "canonical_snapshot_uncommitted",
            "must be clean before merge --push fast-forwards it; nothing was merged, and main was not pushed.",
        ],
        true,
    );
    assert_eq!(
        fs::read_to_string(base.join(record)).unwrap(),
        "an unsaved local plan\n"
    );
    assert_eq!(
        git(repo, &["rev-parse", "main"]),
        local,
        "main did not move"
    );
    assert_eq!(git(repo, &["rev-parse", "HEAD"]), head, "nothing merged");
}

/// The base the merge fetches decides the forge: one that dropped the
/// forge, or names another, refuses before merging (finding atelier-gfwz).
#[test]
fn merge_push_reads_the_forge_from_the_fetched_base() {
    for (forge_block, expected) in [
        ("", "merge_push_no_forge: the main policy names no forge"),
        (
            "forge:\n  provider: github\n  ci_workflow: other.yml\n",
            "merge_push_forge_changed: main at ",
        ),
    ] {
        let forge = Forge::new("  - ci.passed\n");
        let head = forge.head();
        forge.run(&head, 111, "completed", Some("success"));
        forge.pull_request(true);
        let policy = git(forge.path(), &["show", "main:.atelier/workflow.yaml"]);
        let policy = policy.replace(
            &format!("{FORGE}merge_checks:\n  - ci.passed"),
            &format!("{forge_block}merge_checks:\n  - blockers.none_open"),
        );
        assert!(!policy.contains("ci.passed"), "{policy}");
        forge.push_to_remote_main_with(".atelier/workflow.yaml", &format!("{policy}\n"));

        let output = forge.push_refuses(&["merge", &forge.id, "--push"], &[expected], true);
        let unpushed = if forge_block.is_empty() {
            format!("{} was pushed, and main was not.", forge.branch)
        } else {
            "main was not pushed.".to_owned()
        };
        assert!(output.contains(&unpushed), "{output}");
        assert_eq!(
            git(forge.path(), &["rev-parse", "HEAD"]),
            head,
            "nothing merged"
        );
        assert!(!output.contains("✓ Merged"), "{output}");
    }
}

/// The approval is recorded after the last push, so GitHub has never seen
/// the branch head: the pull request is found by its branch, and CI on the
/// pushed code commit stands for the head (finding atelier-sihf).
#[test]
fn merge_push_confirms_the_pull_request_when_the_approval_was_never_pushed() {
    let forge = Forge::new("  - ci.passed\n");
    let implementation = git(forge.path(), &["rev-parse", "HEAD~2"]);
    forge.run(&implementation, 121, "completed", Some("success"));
    fs::write(
        forge.gh.path().join("pushed"),
        format!("{implementation}\n"),
    )
    .unwrap();
    forge.pull_request(true);

    let output = forge.passes(&["merge", &forge.id, "--push"]);
    assert_eq!(
        verdict(&output),
        "merge: passed — pushed main to origin, pull request #7 merged",
        "{output}"
    );
}

/// A pull request that cannot be looked up or opened refuses after the
/// branch push, before the gates, and leaves the base unpushed.
#[test]
fn merge_push_refuses_when_the_pull_request_cannot_be_looked_up_or_opened() {
    for (file, doing) in [("list-fails", "look up"), ("create-fails", "open")] {
        let forge = Forge::new("  - ci.passed\n");
        let head = forge.head();
        forge.run(&head, 131, "completed", Some("success"));
        fs::write(forge.gh.path().join(file), "").unwrap();
        let branch = &forge.branch;
        forge.push_refuses(
            &["merge", &forge.id, "--push"],
            &[&format!("forge_unreachable: gh could not {doing} the pull request from {branch} into main: gh: "),
              &format!("; {branch} was pushed, and main was not. Next: retry; `gh auth status` shows the sign-in"),
              "merge: refused — 1 check (forge_unreachable), main not pushed"],
            true,
        );
        assert_eq!(forge.remote_branch(), Some(head));
        assert_eq!(
            git(forge.path(), &["rev-parse", "HEAD"]),
            forge.head(),
            "nothing merged"
        );
    }
}
