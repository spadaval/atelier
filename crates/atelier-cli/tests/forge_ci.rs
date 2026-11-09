//! `ci.passed` and `merge --push` (ADR 0031; docs/product/reviews-and-merge.md,
//! CI on a forge) against a fake `gh` on `PATH` that answers `gh api` from
//! files: `runs/<sha>` for a commit's pull request runs, `base` for the
//! base's newest push run, `pulls` for the pull requests that have a commit,
//! `pull` for one pull request's state, and `signed-out` to answer as a
//! signed-out `gh` does. `on-runs`, when present, is a script run once at
//! the first runs lookup. Every call is logged to `calls`.
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
for arg in "$@"; do
  case "$arg" in
    head_sha=*) answer="runs/${arg#head_sha=}" ;;
    branch=*) answer="base" ;;
    */pulls) answer="pulls" ;;
    */pulls/*) answer="pull" ;;
  esac
done
case "$answer" in
  runs/*)
    if [ -f "$FAKE_GH/on-runs" ]; then
      sh "$FAKE_GH/on-runs"
      rm -f "$FAKE_GH/on-runs"
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
        fs::write(work.join(file), "from another checkout\n").unwrap();
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
            r#"[{"number":7,"state":"open","base":{"ref":"main"}}]"#,
        )
        .unwrap();
        fs::write(
            self.gh.path().join("pull"),
            format!(r#"{{"number":7,"merged":{merged}}}"#),
        )
        .unwrap();
    }
    /// `merge --push` refuses with `expected`, ending with a verdict that
    /// says nothing was pushed, and the forge's base is unchanged.
    fn push_refuses(&self, args: &[&str], expected: &[&str], gh: bool) -> String {
        let before = self.remote_main();
        let (passed, output) = self.atelier(args, gh);
        assert!(!passed, "{output}");
        for text in expected {
            assert!(output.contains(text), "missing {text:?} in:\n{output}");
        }
        assert!(
            verdict(&output).starts_with("merge: refused")
                && verdict(&output).ends_with(", nothing pushed"),
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
        &[&format!("⚠ {id} ci.passed: ci.yml is in progress on pull request #7 for {}: {url}. Next: `gh run watch 21`, then `atelier merge {id}`", short(&head))],
        true,
    );
    forge.run(&head, 21, "queued", None);
    forge.refuses(&["ci.yml is queued on pull request #7"], true);
    forge.run(&head, 21, "completed", Some("failure"));
    forge.refuses(
        &[&format!("⚠ {id} ci.passed: ci.yml failed on pull request #7 for {}: {url}. Next: `gh run view 21 --log-failed`; fix it on the branch, commit and push, then `atelier merge {id}` once CI passes", short(&head))],
        true,
    );
    forge.run(&head, 21, "completed", Some("cancelled"));
    forge.refuses(
        &[
            "ci.yml was cancelled on pull request #7",
            "Next: `gh run rerun 21`",
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
        &[&format!("⚠ {id} ci.passed: no open pull request into main has {}, the branch head, so CI has not run on it. Next: push the branch and open a pull request into main (`gh pr create --base main`), then `atelier merge {id}` once CI passes", short(&head))],
        true,
    );
    fs::write(
        forge.gh.path().join("pulls"),
        r#"[{"number":7,"state":"open","base":{"ref":"main"}}]"#,
    )
    .unwrap();
    forge.refuses(
        &[&format!(
            "⚠ {id} ci.passed: no pull_request run of ci.yml exists for {}, the branch head, or an earlier branch commit with its code",
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
        forge.calls().contains("repos/owner/repo/pulls/7"),
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

#[test]
fn merge_push_pushes_nothing_when_a_gate_refuses_or_gh_is_missing_or_signed_out() {
    let forge = Forge::new("  - ci.passed\n");
    let id = forge.id.as_str();
    let head = forge.head();
    forge.pull_request(true);
    forge.run(&head, 51, "in_progress", None);
    forge.push_refuses(
        &["merge", id, "--push"],
        &[
            "ci.yml is in progress",
            "merge: refused — 1 check (ci.passed), nothing pushed",
        ],
        true,
    );
    forge.run(&head, 51, "completed", Some("success"));
    let calls = forge.calls().lines().count();
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
    assert_eq!(
        forge.calls().lines().skip(calls).collect::<Vec<_>>(),
        ["auth status --hostname github.com"]
    );
    assert_eq!(
        git(forge.path(), &["rev-parse", "HEAD"]),
        head,
        "nothing merged"
    );
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
            "merge: refused — 1 check (merge_push_base_diverged), nothing pushed",
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
        "merge: refused — 1 check (merge_push_base_rejected), nothing pushed",
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
            "Nothing was pushed; merge commit",
            "merge: refused — 1 check (land_base_unreviewed), nothing pushed",
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
