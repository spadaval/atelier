//! The views a program reads as data, as the Desk does: a repository
//! opened for reading from commits only, a mission and an item as `show`
//! shows them, and what changed in a mission's plan between two commits.

use atelier_app::plan::plan_changed_at;
use atelier_app::{item_view, mission_view, ReadFrom, Repo, Skip};
use atelier_core::{Status, Thread};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

const MISSION: &str = "demo-m1ss";
const STARTED: &str = "demo-ab12";
const PLANNED: &str = "demo-cd34";

fn git(at: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(at)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn write(root: &Path, path: &str, text: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn item_path(id: &str) -> String {
    format!(".atelier/items/{id}/item.md")
}

/// Item `id` of the mission, titled `title`, with `status` and
/// `implementers`, and `body` after its Done when.
fn item(id: &str, title: &str, status: &str, implementers: &str, body: &str) -> String {
    format!(
        "---\nschema: atelier.item\nschema_version: 3\nid: {id}\ntitle: {title}\n\
         status: {status}\nmission: {MISSION}\nlabels: []\npriority: high\nblocked_by: []\n\
         implementers: {implementers}\nready: null\ntrivial: null\ncaused_by: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Goal\n\nPrint hello.\n\n\
         ## Done when\n\n- [ ] It prints hello.\n{body}"
    )
}

const PLAN_REVIEW: &str = "---\nschema: atelier.review\nschema_version: 3\nround: 1\n\
                           commit: abc1234\nreviewers:\n- r1\nmodel: m\nverdict: approve\n\
                           created_at: 2026-11-10T09:00:00Z\n---\n## Summary\ns\n";

/// A repository on `main` holding a ready mission with a design, notes
/// and a plan review, its item `PLANNED` ready with an open question, and
/// its item `STARTED`, which the branch named for it holds building. The
/// checkout has that branch out.
fn scratch() -> (TempDir, PathBuf) {
    let dir = TempDir::new().unwrap();
    let root = dir.path().join("repo");
    fs::create_dir_all(&root).unwrap();
    git(&root, &["init", "-q", "-b", "main"]);
    git(&root, &["config", "user.name", "b"]);
    git(&root, &["config", "user.email", "b@b"]);
    write(
        &root,
        ".atelier/config.toml",
        "schema_version = 3\nslug = \"demo\"\nbase_branch = \"main\"\n",
    );
    let mission = format!(".atelier/missions/{MISSION}");
    write(
        &root,
        &format!("{mission}/mission.md"),
        &format!(
            "---\nschema: atelier.mission\nschema_version: 3\nid: {MISSION}\ntitle: M\n\
             status: ready\nlabels: []\nlead: null\nready: null\n\
             created_at: 2026-11-10T09:00:00Z\n---\n## Outcome\no\n"
        ),
    );
    write(&root, &format!("{mission}/design.md"), "# Design\n");
    write(&root, &format!("{mission}/notes.md"), "# Notes\n");
    write(&root, &format!("{mission}/reviews/1-aaaa.md"), PLAN_REVIEW);
    let question = "\n## Decisions\n\n### 2026-11-10: Which way?\n";
    write(
        &root,
        &item_path(PLANNED),
        &item(PLANNED, "Planned", "ready", "[]", question),
    );
    write(
        &root,
        &item_path(STARTED),
        &item(STARTED, "Committed", "ready", "[]", ""),
    );
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Plan"]);
    git(&root, &["switch", "-qc", STARTED]);
    write(
        &root,
        &item_path(STARTED),
        &item(STARTED, "Committed", "building", "[t1]", ""),
    );
    git(&root, &["commit", "-qam", "Start"]);
    (dir, root)
}

fn caller() -> Thread {
    Thread("sumeet".into())
}

#[test]
fn a_repository_opened_from_commits_reads_no_uncommitted_record() {
    let (_dir, root) = scratch();
    write(
        &root,
        ".atelier/config.toml",
        "schema_version = 3\nslug = \"edited\"\nbase_branch = \"main\"\n",
    );
    write(
        &root,
        &item_path(STARTED),
        &item(STARTED, "Uncommitted", "building", "[t1]", ""),
    );
    let skip = Skip::new(None);
    let title = |repo: &mut Repo| {
        let located = repo.locate(STARTED).unwrap();
        located.record().unwrap().item.title.clone()
    };

    // Opened as the commands open it, the checkout's files are read.
    let mut checkout = Repo::open(&root, caller(), &skip).unwrap();
    assert_eq!(checkout.slug, "edited");
    assert_eq!(title(&mut checkout), "Uncommitted");

    let mut repo = Repo::open_committed(&root, caller(), &skip).unwrap();
    assert_eq!((repo.slug.as_str(), repo.base.as_str()), ("demo", "main"));
    assert_eq!(title(&mut repo), "Committed");
    let everything = repo.everything().unwrap();
    let started = everything.iter().find(|l| l.id == STARTED).unwrap();
    assert_eq!(started.record().unwrap().item.title, "Committed");
    let current = repo.current().unwrap().unwrap();
    assert_eq!(current.record().unwrap().item.title, "Committed");
    assert_eq!(repo.branch.as_deref(), Some(STARTED));
}

#[test]
fn the_config_is_read_from_the_commit_head_names() {
    let (_dir, root) = scratch();
    git(&root, &["switch", "-q", "main"]);
    write(
        &root,
        ".atelier/config.toml",
        "schema_version = 3\nslug = \"later\"\nbase_branch = \"main\"\n",
    );
    git(&root, &["commit", "-qam", "Rename"]);
    git(&root, &["switch", "-q", "--detach", "HEAD~1"]);
    let skip = Skip::new(None);
    let repo = Repo::open_committed(&root, caller(), &skip).unwrap();
    assert_eq!(repo.slug, "demo");
}

#[test]
fn the_mission_view_holds_what_show_prints() {
    let (_dir, root) = scratch();
    let skip = Skip::new(None);
    let mut repo = Repo::open_committed(&root, caller(), &skip).unwrap();

    let view = mission_view(&mut repo, MISSION).unwrap().unwrap();
    assert_eq!(view.dir.mission.id, MISSION);
    assert!(view.in_progress);
    let items: Vec<(&str, Status)> = view
        .items
        .iter()
        .map(|e| (e.id.as_str(), e.status))
        .collect();
    assert_eq!(
        items,
        [(PLANNED, Status::Ready), (STARTED, Status::Building)]
    );
    let reviews: Vec<String> = view
        .dir
        .reviews
        .iter()
        .map(|r| r.name.to_string())
        .collect();
    assert_eq!(reviews, ["1-aaaa"]);

    assert!(mission_view(&mut repo, PLANNED).unwrap().is_none());
}

#[test]
fn the_item_view_says_which_record_it_read() {
    let (_dir, root) = scratch();
    let skip = Skip::new(None);
    let mut repo = Repo::open_committed(&root, caller(), &skip).unwrap();

    let started = item_view(&mut repo, STARTED).unwrap().unwrap();
    assert_eq!(started.read_from, ReadFrom::Branch);
    assert_eq!(started.record().item.status, Status::Building);
    assert_eq!(started.entry.status, Status::Building);
    assert_eq!(
        started.links.mission,
        Some((MISSION.to_string(), Some(Status::Ready)))
    );

    let planned = item_view(&mut repo, PLANNED).unwrap().unwrap();
    assert_eq!(planned.read_from, ReadFrom::Base);

    assert!(item_view(&mut repo, "demo-zz99").unwrap().is_none());
}

#[test]
fn plan_changes_are_read_between_two_commits() {
    let (_dir, root) = scratch();
    git(&root, &["switch", "-q", "main"]);
    let reviewed = git(&root, &["rev-parse", "HEAD"]);
    let mission = format!(".atelier/missions/{MISSION}");
    write(
        &root,
        &format!("{mission}/design.md"),
        "# Design\n\nChanged.\n",
    );
    write(&root, &format!("{mission}/reviews/2-bbbb.md"), PLAN_REVIEW);
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Change the design"]);
    let tip = git(&root, &["rev-parse", "HEAD"]);
    // Uncommitted edits to the notes and to an item are not read.
    write(
        &root,
        &format!("{mission}/notes.md"),
        "# Notes\n\nEdited.\n",
    );
    write(
        &root,
        &item_path(PLANNED),
        &item(PLANNED, "Edited", "ready", "[]", ""),
    );
    let skip = Skip::new(None);
    let mut repo = Repo::open_committed(&root, caller(), &skip).unwrap();

    let changed = plan_changed_at(&mut repo, MISSION, &reviewed, &tip).unwrap();
    assert_eq!(changed, Some(vec![format!("{mission}/design.md")]));
    let unchanged = plan_changed_at(&mut repo, MISSION, &tip, &tip).unwrap();
    assert_eq!(unchanged, Some(Vec::new()));
    let unknown = "0123456789abcdef0123456789abcdef01234567";
    assert_eq!(
        plan_changed_at(&mut repo, MISSION, unknown, &tip).unwrap(),
        None
    );
}
