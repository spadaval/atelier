//! The Desk's routes, served from scratch repositories on the new records.

use atelier_app::{list, mission_view, Filter, Repo, Skip, LIFECYCLE};
use atelier_core::Thread;
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

fn mission(id: &str, title: &str, status: &str) -> (String, String) {
    let text = format!(
        "---\nschema: atelier.mission\nschema_version: 3\nid: {id}\ntitle: {title}\n\
         status: {status}\nlabels: []\nlead: null\nready: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n## Why\n\nBecause.\n\n## Outcome\n\nIt works.\n"
    );
    (format!(".atelier/missions/{id}/mission.md"), text)
}

/// Item `id` with `status`, in `mission`, labelled `labels` (YAML), and
/// `body` after its Done when.
fn item(id: &str, status: &str, mission: &str, labels: &str, body: &str) -> (String, String) {
    let text = format!(
        "---\nschema: atelier.item\nschema_version: 3\nid: {id}\ntitle: Item {id}\n\
         status: {status}\nmission: {mission}\nlabels: {labels}\npriority: high\nblocked_by: []\n\
         implementers: []\nready: null\ntrivial: null\ncaused_by: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Goal\n\nDo {id}.\n\n\
         ## Done when\n\n- [ ] It is done.\n{body}"
    );
    (format!(".atelier/items/{id}/item.md"), text)
}

fn put(root: &Path, (path, text): (String, String)) {
    write(root, &path, &text);
}

/// A repository on `main` whose mission ALPHA is ready, with items draft,
/// ready (needs-human), building on its own branch with a question, and
/// done; mission BETA is a draft with one ready item; and one item, which
/// needs a human, is in no mission. The checkout has `main` out.
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
    put(root, mission(ALPHA, "Alpha", "ready"));
    put(root, mission(BETA, "Beta", "draft"));
    write(
        root,
        &format!(".atelier/missions/{ALPHA}/design.md"),
        "# The design\n",
    );
    put(root, item("demo-a001", "draft", ALPHA, "[]", ""));
    put(root, item("demo-a002", "ready", ALPHA, "[needs-human]", ""));
    put(root, item(STARTED, "ready", ALPHA, "[]", ""));
    put(
        root,
        item("demo-a004", "done", ALPHA, "[]", "\n## Report\n\nDone.\n"),
    );
    put(root, item("demo-b001", "ready", BETA, "[]", ""));
    put(
        root,
        item("demo-c001", "ready", "null", "[needs-human]", ""),
    );
    git(root, &["add", "-A"]);
    git(root, &["commit", "-qm", "Plan"]);
    git(root, &["switch", "-qc", STARTED]);
    let question = "\n## Decisions\n\n### 2026-11-10: Which way?\n";
    put(root, item(STARTED, "building", ALPHA, "[]", question));
    git(root, &["commit", "-qam", "Start"]);
    git(root, &["switch", "-q", "main"]);
}

struct Served {
    _dir: TempDir,
    root: PathBuf,
    desk: Arc<Desk>,
    app: Router,
}

fn config(roots: Vec<PathBuf>, app: Option<PathBuf>) -> Config {
    let text = "caller = \"sumeet\"\nroots = []\n";
    let mut config: Config = toml::from_str(text).unwrap();
    config.roots = roots;
    config.app = app;
    config
}

fn serve() -> Served {
    let dir = TempDir::new().unwrap();
    let root = dir.path().join("repos/demo");
    scratch(&root);
    let desk = Desk::new(config(vec![dir.path().join("repos")], None));
    Served {
        root,
        app: router_for(desk.clone()),
        desk,
        _dir: dir,
    }
}

async fn get_from(app: &Router, uri: &str, host: &str) -> (StatusCode, Vec<u8>) {
    send(app, "GET", uri, host).await
}

async fn send(app: &Router, method: &str, uri: &str, host: &str) -> (StatusCode, Vec<u8>) {
    let request = Request::builder()
        .method(method)
        .uri(uri)
        .header("host", host);
    let response = app
        .clone()
        .oneshot(request.body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    (status, body.to_vec())
}

async fn get(app: &Router, uri: &str) -> (StatusCode, Value) {
    let (status, body) = get_from(app, uri, "localhost:3780").await;
    let value = serde_json::from_slice(&body).unwrap_or(Value::Null);
    (status, value)
}

async fn ok(app: &Router, uri: &str) -> Value {
    let (status, body) = get(app, uri).await;
    assert_eq!(status, StatusCode::OK, "{uri}: {body}");
    body
}

fn ids(items: &Value) -> Vec<String> {
    let items = items.as_array().unwrap().iter();
    items
        .map(|item| item["id"].as_str().unwrap().to_string())
        .collect()
}

#[tokio::test]
async fn each_mission_agrees_with_show_and_the_inbox_with_list() {
    let served = serve();
    let skip = Skip::new(None);
    let mut repo = Repo::open_committed(&served.root, Thread("sumeet".into()), &skip).unwrap();
    for id in [ALPHA, BETA] {
        let page = ok(&served.app, &format!("/api/repos/demo/missions/{id}")).await;
        let view = mission_view(&mut repo, id).unwrap().unwrap();
        let mission = &page["mission"];
        assert_eq!(mission["title"], view.dir.mission.title.as_str());
        assert_eq!(
            mission["status"],
            serde_json::to_value(view.dir.mission.status).unwrap()
        );
        let counts: Vec<Value> = (LIFECYCLE.iter())
            .map(|s| (s, view.items.iter().filter(|e| e.status == *s).count()))
            .filter(|(_, count)| *count > 0)
            .map(|(s, count)| serde_json::json!({"status": s, "count": count}))
            .collect();
        assert_eq!(mission["counts"], Value::Array(counts), "{id}");
        assert_eq!(page["commit"], repo.base_tip.as_str());
    }
    // The started item counts as its branch's record says, building.
    let alpha = ok(&served.app, &format!("/api/repos/demo/missions/{ALPHA}")).await;
    let expected = serde_json::json!([
        {"status": "draft", "count": 1}, {"status": "ready", "count": 1},
        {"status": "building", "count": 1}, {"status": "done", "count": 1}
    ]);
    assert_eq!(alpha["mission"]["counts"], expected);
    assert_eq!(
        alpha["mission"]["waiting"],
        serde_json::json!(["demo-a002", STARTED])
    );
    assert_eq!(alpha["documents"][0]["title"], "The design");
    assert_eq!(alpha["sections"][0]["heading"], "Why");

    let inbox = ok(&served.app, "/api/inbox").await;
    let groups = inbox["repos"][0]["needs_human"].as_array().unwrap();
    let served_ids: Vec<String> = groups.iter().flat_map(|g| ids(&g["items"])).collect();
    let items = repo.everything().unwrap();
    let missions = repo.missions().unwrap();
    let entries = atelier_app::entries(&items, &missions, &repo.caller).unwrap();
    let filter = Filter {
        needs_human: true,
        ..Filter::default()
    };
    let listed: Vec<String> = list(&entries, &missions, &repo.caller, &filter)
        .iter()
        .flat_map(|group| group.items.iter().map(|e| e.id.clone()))
        .collect();
    assert_eq!(served_ids, listed);
    assert_eq!(listed, ["demo-a002", "demo-c001"]);
    assert_eq!(inbox["repos"][0]["questions"][0]["item"], STARTED);
    assert_eq!(ids(&inbox["repos"][0]["drafts"]), [BETA]);

    let rail = ok(&served.app, "/api/repos/demo/missions").await;
    assert_eq!(ids(&rail["missions"]), [ALPHA, BETA]);
    assert_eq!(rail["loose"], 1);
    let done = ok(&served.app, "/api/repos/demo/missions?done=true").await;
    assert_eq!(
        (done["missions"].as_array().unwrap().len(), &done["loose"]),
        (0, &Value::from(0))
    );
}

#[tokio::test]
async fn records_are_read_from_commits_not_the_checkouts() {
    let served = serve();
    let root = &served.root;
    // Uncommitted edits: the base's config and a mission in the checkout of
    // the base, and the started item in a checkout of its branch.
    write(
        root,
        ".atelier/config.toml",
        "schema_version = 3\nslug = \"demo\"\nbase_branch = \"elsewhere\"\n",
    );
    put(root, mission(ALPHA, "Edited", "ready"));
    let branch = root.parent().unwrap().parent().unwrap().join("started");
    git(
        root,
        &["worktree", "add", "-q", branch.to_str().unwrap(), STARTED],
    );
    let edited = "\n## Report\n\nNot committed.\n";
    put(&branch, item(STARTED, "in_review", ALPHA, "[]", edited));

    let page = ok(&served.app, &format!("/api/repos/demo/missions/{ALPHA}")).await;
    assert_eq!(page["mission"]["title"], "Alpha");
    let item = ok(&served.app, &format!("/api/repos/demo/items/{STARTED}")).await;
    assert_eq!(item["row"]["status"], "building");
    assert_eq!(item["read_from"], "branch");
    assert_eq!(item["row"]["question"], "2026-11-10: Which way?");
    assert_eq!(item["report"], Value::Null);
}

#[tokio::test]
async fn a_new_commit_on_the_base_changes_the_response_without_a_restart() {
    let served = serve();
    let uri = format!("/api/repos/demo/missions/{ALPHA}");
    let built = || served.desk.built.load(Ordering::Relaxed);
    assert_eq!(ok(&served.app, &uri).await["mission"]["title"], "Alpha");
    let after_first = built();
    assert_eq!(ok(&served.app, &uri).await["mission"]["title"], "Alpha");
    assert_eq!(
        built(),
        after_first,
        "an unchanged repository is answered from the cache"
    );

    put(&served.root, mission(ALPHA, "Renamed", "ready"));
    git(&served.root, &["commit", "-qam", "Rename"]);
    assert_eq!(ok(&served.app, &uri).await["mission"]["title"], "Renamed");
    assert_eq!(built(), after_first + 1);

    // A new commit on an item's branch is a new read too.
    let worktree = served.root.parent().unwrap().parent().unwrap().join("b");
    git(
        &served.root,
        &["worktree", "add", "-q", worktree.to_str().unwrap(), STARTED],
    );
    put(
        &worktree,
        item(STARTED, "building", ALPHA, "[needs-human]", ""),
    );
    git(&worktree, &["commit", "-qam", "Label"]);
    let page = ok(&served.app, &uri).await;
    assert_eq!(
        page["mission"]["waiting"],
        serde_json::json!(["demo-a002", STARTED])
    );
    assert_eq!(page["questions"], serde_json::json!([]));
}

#[tokio::test]
async fn the_item_and_document_routes() {
    let served = serve();
    let item = ok(&served.app, "/api/repos/demo/items/demo-a004").await;
    assert_eq!(item["row"]["status"], "done");
    assert_eq!(item["report"], "Done.");
    assert_eq!(item["done_when"][0]["text"], "It is done.");
    let path = format!(".atelier/missions/{ALPHA}/./design.md");
    let doc = ok(&served.app, &format!("/api/repos/demo/doc?path={path}")).await;
    assert_eq!(doc["markdown"], "# The design\n");
    assert_eq!(doc["path"], format!(".atelier/missions/{ALPHA}/design.md"));

    for (uri, status) in [
        ("/api/repos/demo/items/demo-zzzz", StatusCode::NOT_FOUND),
        ("/api/repos/demo/items/..%2Fx", StatusCode::BAD_REQUEST),
        (
            "/api/repos/demo/missions/Not-An-Id",
            StatusCode::BAD_REQUEST,
        ),
        (
            "/api/repos/demo/doc?path=../../etc/passwd.md",
            StatusCode::BAD_REQUEST,
        ),
        (
            "/api/repos/demo/doc?path=.atelier/config.toml",
            StatusCode::BAD_REQUEST,
        ),
        ("/api/repos/demo/doc?path=missing.md", StatusCode::NOT_FOUND),
        ("/api/repos/nowhere/missions", StatusCode::NOT_FOUND),
        // Not served by this item.
        (
            "/api/repos/demo/missions/demo-aaaa/history",
            StatusCode::NOT_FOUND,
        ),
        ("/api/act", StatusCode::NOT_FOUND),
        // Rejected by axum's extractors.
        ("/api/repos/demo/doc", StatusCode::BAD_REQUEST),
        (
            "/api/repos/demo/missions?done=maybe",
            StatusCode::BAD_REQUEST,
        ),
    ] {
        let (got, body) = get(&served.app, uri).await;
        assert_eq!(got, status, "{uri}: {body}");
        assert!(body["error"].is_string(), "{uri}: {body}");
    }
    for (method, uri, status) in [
        ("POST", "/api/repos", StatusCode::METHOD_NOT_ALLOWED),
        (
            "DELETE",
            "/api/repos/demo/items/demo-a004",
            StatusCode::METHOD_NOT_ALLOWED,
        ),
        ("POST", "/api/act", StatusCode::NOT_FOUND),
    ] {
        let (got, body) = send(&served.app, method, uri, "localhost").await;
        let body: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);
        assert_eq!(got, status, "{method} {uri}: {body}");
        assert!(body["error"].is_string(), "{method} {uri}: {body}");
    }
}

#[tokio::test]
async fn a_repository_that_does_not_read_is_listed_with_its_error() {
    let dir = TempDir::new().unwrap();
    let repos = dir.path().join("repos");
    scratch(&repos.join("demo"));
    // One whose record does not read, and one with no Atelier records,
    // which is not served.
    let broken = repos.join("broken");
    scratch(&broken);
    write(
        &broken,
        ".atelier/items/demo-a001/item.md",
        "not a record\n",
    );
    git(&broken, &["commit", "-qam", "Break"]);
    let plain = repos.join("plain");
    fs::create_dir_all(&plain).unwrap();
    git(&plain, &["init", "-q"]);
    // A second root repeating a name.
    let other = dir.path().join("other");
    scratch(&other.join("demo"));
    let app = router_for(Desk::new(config(vec![repos, other], None)));

    let listed = ok(&app, "/api/repos").await;
    let repos = listed["repos"].as_array().unwrap();
    let names: Vec<&str> = repos.iter().map(|r| r["name"].as_str().unwrap()).collect();
    assert_eq!(names, ["broken", "demo", "demo"]);
    assert!(
        repos[0]["error"].as_str().unwrap().contains("item.md"),
        "{}",
        repos[0]
    );
    assert_eq!(repos[1]["error"], Value::Null);
    assert_eq!(
        (&repos[1]["drafts"], &repos[1]["waiting"]),
        (&Value::from(1), &Value::from(2))
    );
    assert_eq!(repos[1]["in_progress"], 1);
    assert!(repos[2]["error"]
        .as_str()
        .unwrap()
        .contains("already serves"));
    let inbox = ok(&app, "/api/inbox").await;
    assert!(inbox["repos"][0]["error"].is_string());
    assert_eq!(
        inbox["repos"][1]["needs_human"].as_array().unwrap().len(),
        2
    );
}

#[tokio::test]
async fn a_request_for_another_host_is_refused() {
    let served = serve();
    let (status, _) = get_from(&served.app, "/api/repos", "localhost:3780").await;
    assert_eq!(status, StatusCode::OK);
    let (status, _) = get_from(&served.app, "/api/repos", "attacker.example").await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let (status, _) = get_from(&served.app, "/", "attacker.example").await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn the_app_is_served_with_its_index_for_its_own_paths() {
    let dir = TempDir::new().unwrap();
    let app_dir = dir.path().join("dist");
    write(&app_dir, "index.html", "<p>the app</p>");
    write(&app_dir, "assets/app.js", "run()");
    let with_app = router_for(Desk::new(config(vec![], Some(app_dir))));
    for (uri, body) in [
        ("/", "<p>the app</p>"),
        ("/assets/app.js", "run()"),
        ("/r/demo/m/x", "<p>the app</p>"),
    ] {
        let (status, got) = get_from(&with_app, uri, "localhost").await;
        assert_eq!(
            (status, String::from_utf8(got).unwrap().as_str()),
            (StatusCode::OK, body),
            "{uri}"
        );
    }
    // No spelling of a path under /api reaches the app's files.
    for uri in [
        "/api",
        "/api/",
        "/api/nothing",
        "/api//repos",
        "/api/repos/demo/missions/x/history",
    ] {
        let (status, body) = get(&with_app, uri).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{uri}: {body}");
        assert!(body["error"].is_string(), "{uri}: {body}");
    }

    let api_only = router_for(Desk::new(config(vec![], None)));
    assert_eq!(get(&api_only, "/").await.0, StatusCode::NOT_FOUND);
    assert_eq!(
        ok(&api_only, "/api/repos").await["repos"],
        serde_json::json!([])
    );
}
