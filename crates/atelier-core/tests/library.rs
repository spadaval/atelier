//! The actions a program calls as functions, as the Desk does: a repository
//! opened by path, from a working directory outside it, with the caller
//! naming who acts, and each result read as data.

use atelier_core::model::Thread;
use atelier_core::plan::drop_mission;
use atelier_core::{history, Happened, Move, Repo, Skip};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

const ID: &str = "demo-ab12";
const MISSION: &str = "demo-m1ss";

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

/// The repository of [`scratch`], whose base also holds `MISSION`, which
/// is `status`.
fn with_mission(status: &str) -> (TempDir, PathBuf) {
    let (dir, root) = scratch("open");
    let path = root.join(mission_path());
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let text = format!(
        "---\nschema: atelier.mission\nschema_version: 3\nid: {MISSION}\ntitle: M\n\
         status: {status}\nlabels: []\nlead: null\ncreated_at: 2026-11-10T09:00:00Z\n---\n\
         ## Outcome\no\n\n## Result\n"
    );
    fs::write(path, text).unwrap();
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Plan the mission"]);
    (dir, root)
}

fn mission_path() -> String {
    format!(".atelier/missions/{MISSION}/mission.md")
}

#[test]
fn a_drop_returns_what_it_wrote_without_committing_or_its_refusal() {
    let (_dir, root) = with_mission("open");
    outside(&root);
    let skip = Skip::new(None);

    // In the base's own checkout nothing is written.
    let mut repo = Repo::open(&root, Thread("ada".into()), &skip).unwrap();
    let refused = drop_mission(&mut repo, MISSION, "no need")
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
    let mut repo = Repo::open(&root, Thread("ada".into()), &skip).unwrap();
    let done = drop_mission(&mut repo, MISSION, "no need")
        .unwrap()
        .unwrap();
    assert_eq!(done.paths, [mission_path()]);
    assert_eq!(done.commit, None);
    assert!(done
        .text
        .starts_with(&format!("Dropped mission {MISSION}: no need\n")));
    assert_eq!(git(&root, &["rev-parse", "HEAD"]), head);
    assert_eq!(git(&root, &["diff", "--name-only"]), mission_path());

    // Again: it is dropped now, and nothing is written.
    let written = fs::read_to_string(root.join(mission_path())).unwrap();
    let refused = drop_mission(&mut repo, MISSION, "again")
        .unwrap()
        .unwrap_err();
    assert_eq!(
        refused.rules,
        [format!("{MISSION} is draft or open (it is dropped)")]
    );
    assert_eq!(
        fs::read_to_string(root.join(mission_path())).unwrap(),
        written
    );
}

/// The repository of [`scratch`], with item `ID` started by `t1` on its
/// branch, checked out, holding a question nobody answered.
fn asked() -> (TempDir, PathBuf) {
    let (dir, root) = scratch("open");
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
    for name in ["Ada: the owner", "two\nlines"] {
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
    // One action with a reason: `drop` on a mission dropped already passes.
    let (_first_dir, first) = with_mission("dropped");
    git(&first, &["switch", "-qc", "plan"]);
    let skip = Skip::new(Some("  Ada\n said to "));
    let mut repo = Repo::open(&first, Thread("p1".into()), &skip).unwrap();
    let done = drop_mission(&mut repo, MISSION, "no need")
        .unwrap()
        .unwrap();
    let rule = format!("{MISSION} is draft or open (it is dropped)");
    assert_eq!(skip.passed(), [format!("{rule}: Ada said to")]);
    // `drop` makes no commit, so the skip is in its result alone.
    assert!(done.commit.is_none());
    assert!(
        done.text.contains("Wrote, and did not commit:"),
        "{}",
        done.text
    );

    // Then another, in the same process, with no reason, on another
    // repository: nothing of the first reaches its result.
    let (_second_dir, second) = with_mission("open");
    git(&second, &["switch", "-qc", "plan"]);
    let skip = Skip::new(None);
    let mut repo = Repo::open(&second, Thread("p1".into()), &skip).unwrap();
    let done = drop_mission(&mut repo, MISSION, "no need")
        .unwrap()
        .unwrap();
    assert!(skip.passed().is_empty());
    assert!(!done.text.contains("Ada said to"), "{}", done.text);
    // And its refusal of the same rule is refused, offering no pass.
    let refused = drop_mission(&mut repo, MISSION, "again")
        .unwrap()
        .unwrap_err();
    assert_eq!(refused.rules, [rule]);
}

#[test]
fn history_returns_the_actions_as_data_to_a_repository_opened_as_the_desk_opens_it() {
    let (_dir, root) = scratch("draft");
    outside(&root);
    let skip = Skip::new(None);
    git(&root, &["switch", "-q", "-c", "backlog/go"]);
    // As a go-ahead was recorded before items were open.
    let path = root.join(item_path(ID));
    let readied = fs::read_to_string(&path)
        .unwrap()
        .replace("status: draft", "status: ready")
        .replace(
            "ready: null",
            "ready:\n  by: ada\n  at: 2026-11-10T09:30:00Z",
        );
    fs::write(&path, readied).unwrap();
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
            by: Some("ada".into()),
            status: Move {
                from: Some("draft"),
                to: "ready",
            }
        }
    );
    assert!(history(&mut repo, Some("demo-zz99"), None)
        .unwrap()
        .is_none());
}

#[test]
fn history_past_its_deadline_is_an_error_and_before_it_is_the_whole_history() {
    // An old record, whose first commit is one line of history.
    let (_dir, root) = scratch("ready");
    let skip = Skip::new(None);
    let mut repo = Repo::open_committed(&root, Thread("desk".into()), &skip).unwrap();
    let mut until = |deadline| atelier_core::history_until(&mut repo, None, None, Some(deadline));
    let now = std::time::Instant::now();
    let error = until(now).unwrap_err();
    assert!(error.to_string().contains("deadline"), "{error}");
    let found = until(now + std::time::Duration::from_secs(60))
        .unwrap()
        .unwrap();
    assert_eq!(found.actions.len(), 1, "{:?}", found.actions);
}
