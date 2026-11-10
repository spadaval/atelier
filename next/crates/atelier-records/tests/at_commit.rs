use atelier_core::{MissionStatus, Status, Thread};
use atelier_records::{CommitReader, ReadError};
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn git(repo: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(repo)
        .args(args)
        .env("GIT_AUTHOR_NAME", "b")
        .env("GIT_AUTHOR_EMAIL", "b@b")
        .env("GIT_COMMITTER_NAME", "b")
        .env("GIT_COMMITTER_EMAIL", "b@b")
        .output()
        .unwrap();
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn item(status: &str, implementers: &str) -> String {
    format!(
        "---\nschema: atelier.item\nschema_version: 3\nid: demo-ab12\ntitle: T\nstatus: {status}\n\
         mission: null\nlabels: []\npriority: high\nblocked_by: []\nimplementers: {implementers}\n\
         ready: null\ntrivial: null\ncaused_by: null\ncreated_at: 2026-11-10T09:00:00Z\n---\n\
         ## Goal\ng\n## Done when\n- [ ] x\n"
    )
}

const REVIEW: &str = "---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: abc1234\n\
                      reviewers:\n- r1\nmodel: m\nverdict: null\ncreated_at: 2026-11-10T09:00:00Z\n---\n\
                      ## Summary\ns\n";

/// A repository whose first commit holds the item ready, with a review,
/// and whose second holds it building; the working tree then moves on to
/// in review without a commit.
fn repository() -> (TempDir, String, String) {
    let dir = TempDir::new().unwrap();
    let repo = dir.path();
    git(repo, &["init", "-q", "-b", "main"]);
    let items = repo.join(".atelier/items/demo-ab12");
    fs::create_dir_all(items.join("reviews")).unwrap();
    fs::write(items.join("item.md"), item("ready", "[]")).unwrap();
    fs::write(items.join("reviews/1-aaaa.md"), REVIEW).unwrap();
    fs::write(items.join("reviews/notes.txt"), "not a review").unwrap();
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-qm", "ready"]);
    let older = git(repo, &["rev-parse", "HEAD"]);
    fs::write(items.join("item.md"), item("building", "[t1]")).unwrap();
    git(repo, &["commit", "-qam", "building"]);
    let newer = git(repo, &["rev-parse", "HEAD"]);
    fs::write(items.join("item.md"), item("in_review", "[t1]")).unwrap();
    (dir, older, newer)
}

#[test]
fn reads_an_item_as_an_older_commit_holds_it_while_the_checkout_moved_on() {
    let (dir, older, newer) = repository();
    let mut reader = CommitReader::new(dir.path());

    let at_older = reader.item(&older, "demo-ab12").unwrap().unwrap();
    assert_eq!(at_older.item.status, Status::Ready);
    assert!(at_older.item.implementers.0.is_empty());
    assert_eq!(at_older.reviews.len(), 1);
    assert_eq!(at_older.reviews[0].name.to_string(), "1-aaaa");
    assert_eq!(
        at_older.path.to_str().unwrap(),
        format!("{older}:.atelier/items/demo-ab12/item.md")
    );

    let at_newer = reader.item(&newer, "demo-ab12").unwrap().unwrap();
    assert_eq!(at_newer.item.status, Status::Building);
    assert_eq!(
        at_newer.item.implementers.holder(),
        Some(&Thread("t1".into()))
    );

    assert_eq!(reader.item_ids(&older).unwrap(), ["demo-ab12"]);
    assert!(reader.item(&older, "demo-zz99").unwrap().is_none());
}

#[test]
fn a_path_the_commit_lacks_is_missing() {
    let (dir, older, _) = repository();
    let mut reader = CommitReader::new(dir.path());
    for path in [".atelier/config.toml", ".atelier/nothing/here.md", "absent"] {
        match reader.text(&older, path) {
            Err(error @ ReadError::Missing { .. }) => {
                assert_eq!(
                    error.to_string(),
                    format!("{path} is not in commit {older}")
                );
            }
            other => panic!("{path}: {other:?}"),
        }
    }
}

#[test]
fn a_directory_or_a_symbolic_link_is_not_a_regular_file() {
    let (dir, _, _) = repository();
    let repo = dir.path();
    std::os::unix::fs::symlink("item.md", repo.join(".atelier/items/demo-ab12/link.md")).unwrap();
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-qm", "link"]);
    let head = git(repo, &["rev-parse", "HEAD"]);
    let mut reader = CommitReader::new(repo);
    for (path, mode) in [
        (".atelier/items/demo-ab12", "40000"),
        (".atelier/items/demo-ab12/link.md", "120000"),
    ] {
        match reader.text(&head, path) {
            Err(error @ ReadError::NotAFile { .. }) => assert_eq!(
                error.to_string(),
                format!("{path} in commit {head} is not a regular file (mode {mode})")
            ),
            other => panic!("{path}: {other:?}"),
        }
    }
}

#[test]
fn a_failing_git_says_so() {
    // Not a repository: git cat-file exits at once, with its reason.
    let empty = TempDir::new().unwrap();
    let mut reader = CommitReader::new(empty.path());
    match reader.item("HEAD", "demo-ab12") {
        Err(ReadError::Git(message)) => {
            assert!(message.contains("not a git repository"), "{message}");
        }
        other => panic!("{other:?}"),
    }

    // A commit the repository does not have, then a good read: the reader
    // recovers.
    let (dir, older, _) = repository();
    let mut reader = CommitReader::new(dir.path());
    let unknown = "0".repeat(40);
    match reader.item(&unknown, "demo-ab12") {
        Err(error @ ReadError::Git(_)) => {
            assert_eq!(
                error.to_string(),
                format!("Git failed: {unknown} names no commit")
            );
        }
        other => panic!("{other:?}"),
    }
    assert!(reader.item(&older, "demo-ab12").unwrap().is_some());
}

#[test]
fn a_record_that_does_not_parse_names_its_commit_and_path() {
    let (dir, _, _) = repository();
    let repo = dir.path();
    fs::write(
        repo.join(".atelier/items/demo-ab12/item.md"),
        "no front matter\n",
    )
    .unwrap();
    git(repo, &["commit", "-qam", "broken"]);
    let head = git(repo, &["rev-parse", "HEAD"]);
    match CommitReader::new(repo).item(&head, "demo-ab12") {
        Err(ReadError::Record(problem)) => assert_eq!(
            problem.file.to_str().unwrap(),
            format!("{head}:.atelier/items/demo-ab12/item.md")
        ),
        other => panic!("{other:?}"),
    }
}

#[test]
fn reads_a_mission_as_a_commit_holds_it_and_none_where_it_has_none() {
    let (dir, older, _) = repository();
    let repo = dir.path();
    let mission = |status: &str| {
        format!(
            "---\nschema: atelier.mission\nschema_version: 3\nid: demo-m1ss\ntitle: M\n\
             status: {status}\nlabels: []\nlead: null\nready: null\n\
             created_at: 2026-11-10T09:00:00Z\n---\n## Outcome\no\n"
        )
    };
    let path = repo.join(".atelier/missions/demo-m1ss/mission.md");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, mission("draft")).unwrap();
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-qm", "mission"]);
    let commit = git(repo, &["rev-parse", "HEAD"]);
    fs::write(&path, mission("ready")).unwrap();

    let mut reader = CommitReader::new(repo);
    let read = reader.mission(&commit, "demo-m1ss").unwrap().unwrap();
    assert_eq!(read.mission.status, MissionStatus::Draft);
    assert_eq!(
        read.path.to_str().unwrap(),
        format!("{commit}:.atelier/missions/demo-m1ss/mission.md")
    );
    assert_eq!(reader.mission_ids(&commit).unwrap(), ["demo-m1ss"]);
    assert!(reader.mission(&older, "demo-m1ss").unwrap().is_none());
    assert!(reader.mission_ids(&older).unwrap().is_empty());
}
