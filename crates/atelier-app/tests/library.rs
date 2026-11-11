//! The acts a program calls as functions, as the Desk does: a repository
//! opened by path, from a working directory outside it, with the caller
//! naming who acts, and each result read as data.

use atelier_app::{decide, ready, Repo, Skip};
use atelier_core::Thread;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

const ID: &str = "demo-ab12";

fn git(at: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(at)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn item_path(id: &str) -> String {
    format!(".atelier/items/{id}/item.md")
}

/// Writes item `id` with `status`, `implementers` and `body` after its
/// front matter.
fn write_item(root: &Path, id: &str, status: &str, implementers: &str, body: &str) {
    let path = root.join(item_path(id));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        format!(
            "---\nschema: atelier.item\nschema_version: 3\nid: {id}\ntitle: Say hello\n\
             status: {status}\nmission: null\nlabels: []\npriority: high\nblocked_by: []\n\
             implementers: {implementers}\nready: null\ntrivial: null\ncaused_by: null\n\
             created_at: 2026-11-10T09:00:00Z\n---\n\n## Goal\n\nPrint hello.\n\n\
             ## Done when\n\n- [ ] It prints hello.\n{body}"
        ),
    )
    .unwrap();
}

/// A repository on `main` whose one commit holds item `ID` with `status`.
fn scratch(status: &str) -> (TempDir, PathBuf) {
    let dir = TempDir::new().unwrap();
    let root = dir.path().join("repo");
    fs::create_dir_all(root.join(".atelier")).unwrap();
    git(&root, &["init", "-q", "-b", "main"]);
    git(&root, &["config", "user.name", "b"]);
    git(&root, &["config", "user.email", "b@b"]);
    fs::write(
        root.join(".atelier/config.toml"),
        "schema_version = 3\nslug = \"demo\"\nbase_branch = \"main\"\n",
    )
    .unwrap();
    write_item(&root, ID, status, "[]", "");
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Plan"]);
    (dir, root)
}

/// The scratch repository's working directory is not this process's.
fn outside(root: &Path) {
    let here = std::env::current_dir().unwrap().canonicalize().unwrap();
    assert!(!here.starts_with(root.canonicalize().unwrap()));
}

#[test]
fn ready_returns_what_it_wrote_and_committed_or_its_refusal() {
    let (_dir, root) = scratch("draft");
    outside(&root);
    let skip = Skip::new(None);
    let mut repo = Repo::open(&root, Thread("sumeet".into()), &skip).unwrap();

    let done = ready(&mut repo, &[ID.into()], "sumeet").unwrap().unwrap();
    assert_eq!(done.paths, [item_path(ID)]);
    assert_eq!(done.commit, Some(git(&root, &["rev-parse", "HEAD"])));
    assert!(done
        .text
        .starts_with(&format!("Made ready, as sumeet said: {ID}.\n")));
    assert_eq!(
        git(&root, &["log", "-1", "--format=%B"]),
        format!("Ready {ID}")
    );

    // Again: it is no draft now, and nothing is written.
    let refused = ready(&mut repo, &[ID.into()], "sumeet")
        .unwrap()
        .unwrap_err();
    assert_eq!(refused.rules, [format!("{ID} is draft (it is ready)")]);
    assert_eq!(
        refused.text,
        format!(
            "`atelier ready` refused; nothing changed.\n  {ID}: it is ready, and `atelier ready` \
             readies a draft\n"
        )
    );
    assert_eq!(
        refused.next.as_deref(),
        Some("Run it again with only the drafts to make ready.")
    );
    assert_eq!(
        git(&root, &["log", "-1", "--format=%s"]),
        format!("Ready {ID}")
    );
    assert_eq!(git(&root, &["status", "--porcelain"]), "");

    // Off the base, the way out names the checkout that has it.
    git(&root, &["switch", "-q", "--detach"]);
    let mut repo = Repo::open(&root, Thread("sumeet".into()), &skip).unwrap();
    let refused = ready(&mut repo, &[ID.into()], "sumeet")
        .unwrap()
        .unwrap_err();
    assert_eq!(
        refused.next.as_deref(),
        Some("Run it in a checkout of main, as after `git switch main`.")
    );
}

/// The repository of [`scratch`], with item `ID` started by `t1` on its
/// branch, checked out, holding a question nobody answered.
fn asked() -> (TempDir, PathBuf) {
    let (dir, root) = scratch("ready");
    git(&root, &["switch", "-q", "-c", ID]);
    let question = "\n## Decisions\n\n### 2026-11-10: Which greeting?\n";
    write_item(&root, ID, "building", "[t1]", question);
    git(&root, &["commit", "-qam", "Ask"]);
    (dir, root)
}

#[test]
fn decide_writes_the_answer_by_the_caller_named_or_returns_its_refusal() {
    let (_dir, root) = asked();
    outside(&root);
    let skip = Skip::new(None);

    // Its implementer does not decide it, and is pointed at who does.
    let mut repo = Repo::open(&root, Thread("t1".into()), &skip).unwrap();
    let refused = decide(&mut repo, ID, "Hello", None).unwrap().unwrap_err();
    assert_eq!(refused.rules, ["You have never implemented demo-ab12"]);
    assert_eq!(
        refused.text,
        format!(
            "`atelier decide {ID}` refused; nothing changed.\nYou (thread t1) implemented {ID}, \
             and none of its implementers, past or present, decides its questions.\n"
        )
    );
    let next = refused.next.unwrap();
    assert!(
        next.starts_with(
            "A decider must be a thread other than yours, and starting it is your job:"
        ),
        "{next}"
    );
    assert!(
        next.ends_with("Then wait for its answer, and run `atelier status`."),
        "{next}"
    );
    assert_eq!(git(&root, &["status", "--porcelain"]), "");

    // The caller the program names is the one the record gets.
    let mut repo = Repo::open(&root, Thread("sumeet".into()), &skip).unwrap();
    let done = decide(&mut repo, ID, "Hello", None).unwrap().unwrap();
    assert_eq!(done.paths, [item_path(ID)]);
    assert_eq!(done.commit, None);
    assert!(
        done.text.starts_with(&format!(
            "Answered a question on {ID}, as thread sumeet:\n  2026-11-10: Which greeting?\n  \
             Answer: Hello\n"
        )),
        "{}",
        done.text
    );
    let text = fs::read_to_string(root.join(item_path(ID))).unwrap();
    assert!(
        text.ends_with("### 2026-11-10: Which greeting?\n\nAnswer by sumeet: Hello\n"),
        "{text}"
    );
    assert!(skip.passed().is_empty());
}

#[test]
fn a_caller_the_records_cannot_name_and_read_back_is_refused_before_anything_is_written() {
    let (_dir, root) = asked();
    let skip = Skip::new(None);
    for name in ["Sumeet: the owner", "two\nlines"] {
        let opened = Repo::open(&root, Thread(name.into()), &skip);
        let error = opened.err().expect("refused").to_string();
        assert!(
            error.contains("cannot be written into a record and read back as it is"),
            "{error}"
        );
    }
    assert_eq!(git(&root, &["status", "--porcelain"]), "");
    // A slash is read back whole, as a Codex subagent's identity has one.
    let mut repo = Repo::open(&root, Thread("t3/codex".into()), &skip).unwrap();
    assert!(decide(&mut repo, ID, "Hello", None).unwrap().is_ok());
}

#[test]
fn a_skip_reaches_its_own_acts_commit_and_result_and_no_other_acts() {
    // One act with a reason: `ready` on an item that is no draft passes.
    let (_first_dir, first) = scratch("ready");
    let skip = Skip::new(Some("  Sumeet\n said to "));
    let mut repo = Repo::open(&first, Thread("p1".into()), &skip).unwrap();
    let done = ready(&mut repo, &[ID.into()], "sumeet").unwrap().unwrap();
    let rule = format!("{ID} is draft (it is ready)");
    assert_eq!(skip.passed(), [format!("{rule}: Sumeet said to")]);
    assert!(done.commit.is_some());
    assert_eq!(
        git(&first, &["log", "-1", "--format=%B"]),
        format!("Ready {ID}\n\nAtelier-Skip: {rule}: Sumeet said to")
    );

    // Then another, in the same process, with no reason, on another
    // repository: nothing of the first reaches its commit or its result.
    let (_second_dir, second) = scratch("draft");
    let skip = Skip::new(None);
    let mut repo = Repo::open(&second, Thread("p1".into()), &skip).unwrap();
    let done = ready(&mut repo, &[ID.into()], "sumeet").unwrap().unwrap();
    assert!(skip.passed().is_empty());
    assert!(!done.text.contains("Sumeet said to"), "{}", done.text);
    assert_eq!(
        git(&second, &["log", "-1", "--format=%B"]),
        format!("Ready {ID}")
    );
    // And its refusal of the same rule is refused, offering no pass.
    let refused = ready(&mut repo, &[ID.into()], "sumeet")
        .unwrap()
        .unwrap_err();
    assert_eq!(refused.rules, [rule]);
}

#[test]
fn ready_given_no_id_refuses_and_commits_nothing() {
    let (_dir, root) = scratch("draft");
    fs::write(root.join("unrelated.txt"), "staged\n").unwrap();
    git(&root, &["add", "unrelated.txt"]);
    let before = (
        git(&root, &["rev-parse", "HEAD"]),
        git(&root, &["status", "--porcelain", "-uall"]),
    );
    let skip = Skip::new(None);
    let mut repo = Repo::open(&root, Thread("p1".into()), &skip).unwrap();
    let refused = ready(&mut repo, &[], "sumeet").unwrap().unwrap_err();
    assert!(refused.rules.is_empty());
    assert_eq!(
        refused.next.as_deref(),
        Some("Then: atelier ready <id>... --by <name>")
    );
    let after = (
        git(&root, &["rev-parse", "HEAD"]),
        git(&root, &["status", "--porcelain", "-uall"]),
    );
    assert_eq!(after, before);
}

#[test]
fn the_hooks_a_commit_runs_are_given_the_invocations_reason_and_not_the_processs() {
    let (_dir, root) = scratch("draft");
    for id in ["demo-cd34", "demo-ef56"] {
        write_item(&root, id, "draft", "[]", "");
    }
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Plan more"]);
    // A pre-commit hook that says what ATELIER_SKIP it was given.
    let seen = root.join(".git/skip-seen");
    let hook = root.join(".git/hooks/pre-commit");
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\nprintf '%s' \"${{ATELIER_SKIP-unset}}\" > '{}'\n",
            seen.display()
        ),
    )
    .unwrap();
    let mut permissions = fs::metadata(&hook).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
    fs::set_permissions(&hook, permissions).unwrap();
    let ready_with = |id: &str, reason: Option<&str>| {
        let skip = Skip::new(reason);
        let mut repo = Repo::open(&root, Thread("p1".into()), &skip).unwrap();
        ready(&mut repo, &[id.into()], "sumeet").unwrap().unwrap();
        fs::read_to_string(&seen).unwrap()
    };
    // Nextest runs each test in a process of its own, so the variable set
    // here reaches no other test.
    std::env::set_var("ATELIER_SKIP", "the process's");
    assert_eq!(ready_with(ID, Some("the invocation's")), "the invocation's");
    assert_eq!(ready_with("demo-cd34", None), "unset");
    std::env::remove_var("ATELIER_SKIP");
    assert_eq!(
        ready_with("demo-ef56", Some("only this one's")),
        "only this one's"
    );
}
