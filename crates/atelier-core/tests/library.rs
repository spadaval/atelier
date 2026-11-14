//! The actions a program calls as functions, as the Desk does: a repository
//! opened by path, from a working directory outside it, with the caller
//! naming who acts, and each result read as data.

use atelier_core::model::{Status, Thread};
use atelier_core::{history, ready, Happened, Move, Repo, Skip};
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
fn ready_returns_what_it_wrote_without_committing_or_its_refusal() {
    let (_dir, root) = scratch("draft");
    outside(&root);
    let skip = Skip::new(None);

    // In the base's own checkout nothing is written.
    let mut repo = Repo::open(&root, Thread("sumeet".into()), &skip).unwrap();
    let refused = ready(&mut repo, &[ID.into()], "sumeet")
        .unwrap()
        .unwrap_err();
    assert_eq!(
        refused.rules,
        ["Nothing is written in the checkout of the base branch main"]
    );
    assert!(refused
        .next
        .as_deref()
        .unwrap()
        .contains("atelier start --backlog <slug>"));
    assert_eq!(git(&root, &["status", "--porcelain"]), "");

    git(&root, &["switch", "-q", "-c", "backlog/go"]);
    let head = git(&root, &["rev-parse", "HEAD"]);
    let mut repo = Repo::open(&root, Thread("sumeet".into()), &skip).unwrap();
    let done = ready(&mut repo, &[ID.into()], "sumeet").unwrap().unwrap();
    assert_eq!(done.paths, [item_path(ID)]);
    assert_eq!(done.commit, None);
    assert!(done
        .text
        .starts_with(&format!("Made ready, as sumeet said: {ID}.\n")));
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    assert_eq!(git(&root, &["diff", "--name-only"]), item_path(ID));

    // Again: it is no draft now, and nothing is written.
    let written = fs::read_to_string(root.join(item_path(ID))).unwrap();
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
        fs::read_to_string(root.join(item_path(ID))).unwrap(),
        written
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
    let repo = Repo::open(&root, Thread("t3/codex".into()), &skip).unwrap();
    assert_eq!(repo.caller, Thread("t3/codex".into()));
}

#[test]
fn a_skip_reaches_its_own_actions_commit_and_result_and_no_other_actions() {
    // One action with a reason: `ready` on an item that is no draft passes.
    let (_first_dir, first) = scratch("ready");
    git(&first, &["switch", "-q", "--detach"]);
    let skip = Skip::new(Some("  Sumeet\n said to "));
    let mut repo = Repo::open(&first, Thread("p1".into()), &skip).unwrap();
    let done = ready(&mut repo, &[ID.into()], "sumeet").unwrap().unwrap();
    let rule = format!("{ID} is draft (it is ready)");
    assert_eq!(skip.passed(), [format!("{rule}: Sumeet said to")]);
    // `ready` makes no commit, so the skip is in its result alone.
    assert!(done.commit.is_none());
    assert!(
        done.text.contains("Wrote, and did not commit:"),
        "{}",
        done.text
    );

    // Then another, in the same process, with no reason, on another
    // repository: nothing of the first reaches its result.
    let (_second_dir, second) = scratch("draft");
    git(&second, &["switch", "-q", "--detach"]);
    let skip = Skip::new(None);
    let mut repo = Repo::open(&second, Thread("p1".into()), &skip).unwrap();
    let done = ready(&mut repo, &[ID.into()], "sumeet").unwrap().unwrap();
    assert!(skip.passed().is_empty());
    assert!(!done.text.contains("Sumeet said to"), "{}", done.text);
    // And its refusal of the same rule is refused, offering no pass.
    let refused = ready(&mut repo, &[ID.into()], "sumeet")
        .unwrap()
        .unwrap_err();
    assert_eq!(refused.rules, [rule]);
}

#[test]
fn ready_given_no_id_refuses_and_commits_nothing() {
    let (_dir, root) = scratch("draft");
    git(&root, &["switch", "-q", "--detach"]);
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
fn history_returns_the_actions_as_data_to_a_repository_opened_as_the_desk_opens_it() {
    let (_dir, root) = scratch("draft");
    outside(&root);
    let skip = Skip::new(None);
    git(&root, &["switch", "-q", "-c", "backlog/go"]);
    let mut repo = Repo::open(&root, Thread("sumeet".into()), &skip).unwrap();
    ready(&mut repo, &[ID.into()], "sumeet").unwrap().unwrap();
    git(&root, &["commit", "-qam", "Ready"]);
    // Its backlog branch merged, as a fast-forward here.
    git(&root, &["branch", "-f", "main", "HEAD"]);

    let mut repo = Repo::open_committed(&root, Thread("desk".into()), &skip).unwrap();
    let found = history(&mut repo, Some(ID), None).unwrap().unwrap();
    assert!(found.passed_over.is_empty());
    let [logged] = found.actions.as_slice() else {
        panic!("{:?}", found.actions)
    };
    assert_eq!(logged.id, ID);
    assert_eq!(logged.thread, None);
    assert_eq!(logged.commit, git(&root, &["rev-parse", "--short", "HEAD"]));
    assert_eq!(
        logged.action,
        Happened::Ready {
            by: Some("sumeet".into()),
            status: Move {
                from: Some(Status::Draft),
                to: Status::Ready,
                mission: false,
            }
        }
    );
    assert!(history(&mut repo, Some("demo-zz99"), None)
        .unwrap()
        .is_none());
}
