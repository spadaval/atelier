//! The reads behind the views a host shows beside its threads: the
//! sidebar's missions, a project's strip, the records' history from commit
//! subjects, CI on a pull request, the items in no mission, and the fields
//! they added to `/api/motion` and an item's page. `/api/thread` is tested
//! with T3 Code's stand-in, in `threads.rs`.

use atelier_desk::{router_for, Config, Desk};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tempfile::TempDir;
use tower::ServiceExt;

const ALPHA: &str = "demo-aaaa";
const BETA: &str = "demo-bbbb";
const STARTED: &str = "demo-a003";

fn git_at(at: &Path, args: &[&str], date: Option<&str>) -> String {
    let mut command = Command::new("git");
    command.current_dir(at).args(args);
    if let Some(date) = date {
        command
            .env("GIT_COMMITTER_DATE", date)
            .env("GIT_AUTHOR_DATE", date);
    }
    let output = command.output().unwrap();
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn git(at: &Path, args: &[&str]) -> String {
    git_at(at, args, None)
}

fn write(root: &Path, path: &str, text: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn mission(root: &Path, id: &str, status: &str) {
    let text = format!(
        "---\nschema: atelier.mission\nschema_version: 3\nid: {id}\ntitle: Mission {id}\n\
         status: {status}\nlabels: []\nlead: null\nready: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n## Why\n\nBecause.\n\n## Outcome\n\nIt works.\n"
    );
    write(root, &format!(".atelier/missions/{id}/mission.md"), &text);
}

/// Item `id` with `status`, in `mission`, labelled `labels` (YAML), held
/// by `implementers` (YAML), with `priority`.
fn item(
    root: &Path,
    id: &str,
    status: &str,
    mission: &str,
    labels: &str,
    implementers: &str,
    priority: &str,
) {
    let text = format!(
        "---\nschema: atelier.item\nschema_version: 3\nid: {id}\ntitle: Item {id}\n\
         status: {status}\nmission: {mission}\nlabels: {labels}\npriority: {priority}\nblocked_by: []\n\
         implementers: {implementers}\nready: null\ntrivial: null\ncaused_by: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Goal\n\nDo {id}.\n\n\
         ## Done when\n\n- [ ] It is done.\n"
    );
    write(root, &format!(".atelier/items/{id}/item.md"), &text);
}

/// The time `minutes` before now, as Git takes a date.
fn before(minutes: i64) -> String {
    (chrono::Utc::now() - chrono::Duration::minutes(minutes))
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

/// A repository on `main`: mission ALPHA agreed, with items draft, ready
/// (needs-human), started on its own branch by `t1`, and done; mission
/// BETA proposed; items in no mission that are ready (needs-human), ready
/// at low priority, ready at high priority, and done. Its history has a
/// commit for each step Atelier's commands name, one 30 days old.
fn scratch(root: &Path) {
    fs::create_dir_all(root).unwrap();
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["config", "user.name", "b"]);
    git(root, &["config", "user.email", "b@b"]);
    write(
        root,
        ".atelier/config.toml",
        "schema_version = 3\nslug = \"demo\"\nbase_branch = \"main\"\n",
    );
    mission(root, ALPHA, "ready");
    mission(root, BETA, "draft");
    item(root, "demo-a001", "draft", ALPHA, "[]", "[]", "high");
    item(
        root,
        "demo-a002",
        "ready",
        ALPHA,
        "[needs-human]",
        "[]",
        "high",
    );
    item(root, STARTED, "ready", ALPHA, "[]", "[]", "high");
    item(root, "demo-a004", "done", ALPHA, "[]", "[]", "high");
    item(
        root,
        "demo-c001",
        "ready",
        "null",
        "[needs-human]",
        "[]",
        "critical",
    );
    item(root, "demo-c002", "ready", "null", "[]", "[]", "low");
    item(root, "demo-c003", "ready", "null", "[]", "[]", "high");
    item(root, "demo-c004", "done", "null", "[]", "[]", "high");
    git(root, &["add", "-A"]);
    let old = Some("2026-01-01T09:00:00Z");
    git_at(root, &["commit", "-qm", "Plan mission demo-aaaa"], old);
    let at = |minutes| Some(before(minutes));
    let empty = |message: &str, minutes| {
        let args = ["commit", "-q", "--allow-empty", "-m", message];
        git_at(root, &args, at(minutes).as_deref());
    };
    empty("Merge demo-a004: Item demo-a004", 50);
    empty("Merge demo-zzzz: No such record", 40);
    git(root, &["switch", "-qc", STARTED]);
    item(root, STARTED, "building", ALPHA, "[]", "[t1]", "high");
    git_at(
        root,
        &["commit", "-qam", "Start demo-a003"],
        at(30).as_deref(),
    );
    empty("Submit demo-a003 for review", 20);
    empty("Review demo-a003, round 1: changes requested", 10);
    git(root, &["switch", "-q", "main"]);
}

struct Served {
    _dir: TempDir,
    root: PathBuf,
    desk: Arc<Desk>,
    app: Router,
}

/// The scratch repository served, with `gh` the stand-in at `gh` where
/// one is given.
fn serve(gh: Option<PathBuf>) -> Served {
    let dir = TempDir::new().unwrap();
    let root = dir.path().join("repos/demo");
    scratch(&root);
    let mut config: Config = toml::from_str("caller = \"ada\"\nroots = []\n").unwrap();
    config.roots = vec![dir.path().join("repos")];
    if let Some(gh) = gh {
        config.gh = gh;
    }
    let desk = Desk::new(config);
    Served {
        root,
        app: router_for(desk.clone()),
        desk,
        _dir: dir,
    }
}

async fn get(app: &Router, uri: &str) -> (StatusCode, Value) {
    let request = Request::get(uri).header("host", "localhost:3780");
    let response = (app.clone().oneshot(request.body(Body::empty()).unwrap()))
        .await
        .unwrap();
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    (status, serde_json::from_slice(&body).unwrap_or(Value::Null))
}

async fn ok(app: &Router, uri: &str) -> Value {
    let (status, body) = get(app, uri).await;
    assert_eq!(status, StatusCode::OK, "{uri}: {body}");
    body
}

fn strings(value: &Value, field: &str) -> Vec<String> {
    let list = value.as_array().unwrap().iter();
    list.map(|v| v[field].as_str().unwrap().to_string())
        .collect()
}

#[tokio::test]
async fn the_sidebar_lists_each_open_mission_with_its_threads() {
    let served = serve(None);
    let sidebar = ok(&served.app, "/api/sidebar").await;
    let repo = &sidebar["repos"][0];
    assert_eq!(repo["repo"], "demo");
    assert!(
        repo.get("error").is_none(),
        "no error field when it reads: {repo}"
    );
    assert_eq!(strings(&repo["missions"], "id"), [ALPHA, BETA]);
    let alpha = &repo["missions"][0];
    assert_eq!(alpha["state"], "in_progress", "{alpha}");
    assert_eq!(
        (&alpha["done"], &alpha["total"]),
        (&Value::from(1), &Value::from(4))
    );
    assert_eq!(alpha["waiting"], 1, "its needs-human item");
    let started = (alpha["items"].as_array().unwrap().iter())
        .find(|i| i["id"] == STARTED)
        .unwrap();
    assert_eq!(started["status"], "building");
    assert_eq!(started["threads"][0]["identity"], "t1");
    assert_eq!(started["threads"][0]["role"], "implementer");
    assert_eq!(repo["missions"][1]["state"], "proposed");

    // A repository that does not read is listed with why.
    let broken = served.root.parent().unwrap().join("broken");
    scratch(&broken);
    write(
        &broken,
        ".atelier/items/demo-a001/item.md",
        "not a record\n",
    );
    git(&broken, &["commit", "-qam", "Break"]);
    let sidebar = ok(&served.app, "/api/sidebar").await;
    let broken = &sidebar["repos"][0];
    assert_eq!(broken["repo"], "broken", "{sidebar}");
    assert!(broken["error"].is_string());
    assert_eq!(broken["missions"], Value::Array(vec![]));
}

#[tokio::test]
async fn the_project_strip_says_what_can_start_what_waits_and_what_moves() {
    let served = serve(None);
    let launch = ok(&served.app, "/api/launch?repo=demo").await;
    // Ready and startable, needing no human, highest priority first.
    assert_eq!(
        strings(&launch["ready"], "id"),
        ["demo-c003", "demo-c002"],
        "{launch}"
    );
    assert_eq!(launch["ready"][0]["priority"], "high");
    // Two needs-human items and one proposed plan; T3 Code is not configured.
    assert_eq!(launch["waiting"], 3);
    assert_eq!(launch["threads_waiting"], Value::Array(vec![]));
    assert_eq!(
        strings(&launch["waiting_items"], "id"),
        [BETA, "demo-c001", "demo-a002"]
    );
    let moving = &launch["in_motion"][0];
    assert_eq!(moving["id"], STARTED, "{launch}");
    assert_eq!(moving["stage"], "building");
    assert_eq!(moving["holder"]["identity"], "t1");
    assert_eq!(moving["holder"]["title"], Value::Null);
    assert_eq!(moving["live"], false);
    let age = moving["age_seconds"].as_i64().unwrap();
    assert!(
        (600..1200).contains(&age),
        "its review, ten minutes ago: {age}"
    );

    let (status, body) = get(&served.app, "/api/launch?repo=elsewhere").await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
}

#[tokio::test]
async fn motion_and_an_item_carry_the_fields_the_views_read() {
    let served = serve(None);
    let motion = ok(&served.app, "/api/motion").await;
    let repo = &motion["repos"][0];
    let changed = repo["items"][0]["changed_at"].as_str().unwrap();
    let tip = git(&served.root, &["log", "-1", "--format=%cI", STARTED]);
    assert_eq!(changed, tip);
    assert_eq!(
        strings(&repo["ready"], "id"),
        ["demo-c001", "demo-a002", "demo-c003", "demo-c002"]
    );

    // Read again with nothing changed, motion and the project strip run no
    // command for the branches' times, or anything else they keep.
    ok(&served.app, "/api/launch?repo=demo").await;
    let built = served.desk.built.load(Ordering::Relaxed);
    ok(&served.app, "/api/motion").await;
    ok(&served.app, "/api/launch?repo=demo").await;
    assert_eq!(served.desk.built.load(Ordering::Relaxed), built);
    // A new commit on the branch is read.
    git(&served.root, &["switch", "-q", STARTED]);
    git_at(
        &served.root,
        &["commit", "-q", "--allow-empty", "-m", "More"],
        Some("2030-01-01T00:00:00Z"),
    );
    git(&served.root, &["switch", "-q", "main"]);
    let motion = ok(&served.app, "/api/motion").await;
    assert_eq!(
        motion["repos"][0]["items"][0]["changed_at"],
        "2030-01-01T00:00:00Z"
    );

    // No forge, so no pull request is known.
    let page = ok(&served.app, &format!("/api/repos/demo/items/{STARTED}")).await;
    assert_eq!(page["pull_request"], Value::Null);
}

#[tokio::test]
async fn history_is_read_from_commit_subjects() {
    let served = serve(None);
    let history = ok(&served.app, "/api/history?repo=demo").await;
    let events = history["events"].as_array().unwrap();
    let kinds: Vec<(&str, &str)> = (events.iter())
        .map(|e| (e["kind"].as_str().unwrap(), e["id"].as_str().unwrap()))
        .collect();
    assert_eq!(
        kinds,
        [
            ("review", STARTED),
            ("submitted", STARTED),
            ("started", STARTED),
            ("merged", "demo-zzzz"),
            ("merged", "demo-a004"),
            ("planned", ALPHA),
        ],
        "{history}"
    );
    let review = &events[0];
    assert_eq!(
        (&review["round"], &review["verdict"]),
        (&Value::from(1), &Value::from("changes requested"))
    );
    assert_eq!(review["record"], "item");
    assert_eq!(review["title"], "Item demo-a003");
    assert_eq!(events[3]["record"], Value::Null, "no such record");
    assert_eq!(events[5]["record"], "mission");
    assert_eq!(events[5]["at"], "2026-01-01T09:00:00Z");

    let week = ok(&served.app, "/api/history?repo=demo&days=7").await;
    assert_eq!(
        week["events"].as_array().unwrap().len(),
        5,
        "the old plan is left out"
    );
    let some = ok(
        &served.app,
        "/api/history?repo=demo&ids=demo-a004,demo-aaaa",
    )
    .await;
    assert_eq!(strings(&some["events"], "id"), ["demo-a004", ALPHA]);

    let (status, _) = get(&served.app, "/api/history?repo=elsewhere").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = get(&served.app, "/api/history?repo=demo&days=soon").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn the_roadmap_lists_the_open_items_in_no_mission() {
    let served = serve(None);
    let roadmap = ok(&served.app, "/api/roadmap?repo=demo").await;
    assert_eq!(roadmap["repo"], "demo");
    assert_eq!(
        strings(&roadmap["loose"], "id"),
        ["demo-c001", "demo-c002", "demo-c003"]
    );
    let (status, _) = get(&served.app, "/api/roadmap?repo=elsewhere").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

/// A `gh` stand-in that logs each call to `calls` beside it and answers
/// `pr view <n>` with CI that passes for 41, runs for 42, and fails as a
/// command for any other.
fn gh(dir: &Path) -> PathBuf {
    let path = dir.join("gh");
    let calls = dir.join("calls");
    let script = format!(
        r#"#!/bin/sh
echo "$*" >> '{}'
case "$3" in
  41) echo '{{"statusCheckRollup":[{{"name":"rust","conclusion":"SUCCESS","status":"COMPLETED"}},{{"name":"desk","conclusion":"SKIPPED"}}]}}' ;;
  42) echo '{{"statusCheckRollup":[{{"name":"rust","conclusion":"","status":"IN_PROGRESS"}}]}}' ;;
  *) echo 'no pull request found' >&2; exit 1 ;;
esac
"#,
        calls.display()
    );
    fs::write(&path, script).unwrap();
    Command::new("chmod").arg("+x").arg(&path).status().unwrap();
    path
}

#[tokio::test]
async fn ci_is_read_with_gh_and_kept_a_minute() {
    let stand_in = TempDir::new().unwrap();
    let served = serve(Some(gh(stand_in.path())));
    let passing = ok(&served.app, "/api/checks?repo=demo&pr=41").await;
    assert_eq!(passing["state"], "passing", "{passing}");
    assert_eq!(strings(&passing["checks"], "state"), ["SUCCESS", "SKIPPED"]);
    let again = ok(&served.app, "/api/checks?repo=demo&pr=41").await;
    assert_eq!(again, passing);
    let pending = ok(&served.app, "/api/checks?repo=demo&pr=42").await;
    assert_eq!(pending["state"], "pending");
    let calls = fs::read_to_string(stand_in.path().join("calls")).unwrap();
    assert_eq!(
        calls.lines().collect::<Vec<_>>(),
        [
            "pr view 41 --json statusCheckRollup",
            "pr view 42 --json statusCheckRollup"
        ]
    );

    // A failed read is an error the page leaves out; a number that is not
    // one never reaches gh.
    let (status, body) = get(&served.app, "/api/checks?repo=demo&pr=43").await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert!(body["error"]
        .as_str()
        .unwrap()
        .contains("no pull request found"));
    let (status, _) = get(&served.app, "/api/checks?repo=demo&pr=--web").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let (status, _) = get(&served.app, "/api/checks?repo=elsewhere&pr=41").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let calls = fs::read_to_string(stand_in.path().join("calls")).unwrap();
    assert_eq!(calls.lines().count(), 3);
}

#[tokio::test]
async fn a_thread_is_looked_up_only_by_an_identity_that_fits() {
    let served = serve(None);
    let (status, _) = get(&served.app, "/api/thread?id=thread%3Aany").await;
    assert_eq!(status, StatusCode::NOT_FOUND, "T3 Code is not configured");
    for bad in ["", "a%0Ab", &"x".repeat(513)] {
        let (status, _) = get(&served.app, &format!("/api/thread?id={bad}")).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{bad}");
    }
}
