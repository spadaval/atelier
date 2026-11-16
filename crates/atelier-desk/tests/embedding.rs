//! What a page that frames the Desk asks it: which repository each one
//! served is, what an ID names, and what a thread identity is tied to.

use atelier_desk::{router_for, Config, Desk};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;
use tower::ServiceExt;

/// A thread that implemented an item now done and reviews another's open
/// round, as a T3 Code identity spells it.
const BOTH: &str = "thread:delegated-task:command%3Amcp%3Ad4040a34/sub:1";
const IMPLEMENTER: &str = "thread:implementer";
const REPLACED: &str = "thread:replaced";
const TAKER: &str = "thread:taker";
const LEAD: &str = "thread:lead";
const PLANNER: &str = "thread:planner";
const EARLY: &str = "thread:early-plan-reviewer";
const LATE: &str = "thread:late-plan-reviewer";

fn git(at: &Path, args: &[&str]) {
    let output = Command::new("git").current_dir(at).args(args).output();
    let output = output.unwrap();
    assert!(output.status.success(), "git {args:?}: {output:?}");
}

fn write(root: &Path, path: &str, text: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn threads(threads: &[&str]) -> String {
    let lines: Vec<String> = threads.iter().map(|t| format!("\n- {t}")).collect();
    match lines.is_empty() {
        true => " []".into(),
        false => lines.concat(),
    }
}

fn mission(root: &Path, id: &str, status: &str, lead: &str, planners: &[&str]) {
    let text = format!(
        "---\nschema: atelier.mission\nschema_version: 3\nid: {id}\ntitle: Mission {id}\n\
         status: {status}\nlabels: []\nlead: {lead}\nplanners:{}\nready: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n## Why\n\nBecause.\n",
        threads(planners)
    );
    write(root, &format!(".atelier/missions/{id}/mission.md"), &text);
}

fn item(root: &Path, id: &str, status: &str, implementers: &[&str]) {
    let text = format!(
        "---\nschema: atelier.item\nschema_version: 3\nid: {id}\ntitle: Item {id}\n\
         status: {status}\nmission: demo-mmmm\nlabels: []\npriority: high\nblocked_by: []\n\
         implementers:{}\nready: null\ntrivial: null\ncaused_by: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Goal\n\nDo {id}.\n\n\
         ## Done when\n\n- [x] It is done.\n\n## Report\n\nDone.\n",
        threads(implementers)
    );
    write(root, &format!(".atelier/items/{id}/item.md"), &text);
}

fn review(root: &Path, at: &str, round: u32, reviewer: &str, verdict: &str) {
    let text = format!(
        "---\nschema: atelier.review\nschema_version: 3\nround: {round}\n\
         commit: 0000000000000000000000000000000000000000\nreviewers:\n- {reviewer}\n\
         model: null\nverdict: {verdict}\ncreated_at: 2026-11-10T10:00:00Z\n---\n\n\
         ## Summary\n\nRead.\n"
    );
    write(root, at, &text);
}

/// Repository `demo`, whose `origin` is GitHub's over SSH: mission
/// `demo-mmmm` is open, led by `LEAD` and planned by `PLANNER`, with a
/// closed first round of plan review by `EARLY` and an open second by
/// `LATE`; mission `demo-dddd`, done, was led by `LEAD` too. Item
/// `demo-a001` is done, implemented by `BOTH`. On their own branches,
/// `demo-a002` is in review, its open round reviewed by `BOTH` after a
/// closed round by `EARLY`, and `demo-a003` is building, taken over from
/// `REPLACED` by `TAKER`.
fn demo(root: &Path) {
    fs::create_dir_all(root).unwrap();
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["config", "user.name", "b"]);
    git(root, &["config", "user.email", "b@b"]);
    let origin = "git@github.com:Spadaval/Demo.git";
    git(root, &["remote", "add", "origin", origin]);
    let config = "schema_version = 3\nslug = \"demo\"\nbase_branch = \"main\"\n";
    write(root, ".atelier/config.toml", config);
    mission(root, "demo-mmmm", "ready", LEAD, &[PLANNER]);
    let plan = ".atelier/missions/demo-mmmm/reviews";
    review(root, &format!("{plan}/1-aaaa.md"), 1, EARLY, "approve");
    review(root, &format!("{plan}/2-bbbb.md"), 2, LATE, "null");
    mission(root, "demo-dddd", "done", LEAD, &[]);
    item(root, "demo-a001", "done", &[BOTH]);
    item(root, "demo-a002", "ready", &[]);
    item(root, "demo-a003", "ready", &[]);
    git(root, &["add", "-A"]);
    git(root, &["commit", "-qm", "Plan"]);
    // Each started item on its own branch, where its record counts.
    git(root, &["switch", "-qc", "demo-a002"]);
    item(root, "demo-a002", "in_review", &[IMPLEMENTER]);
    let reviews = ".atelier/items/demo-a002/reviews";
    review(root, &format!("{reviews}/1-cccc.md"), 1, EARLY, "changes");
    review(root, &format!("{reviews}/2-dddd.md"), 2, BOTH, "null");
    git(root, &["add", "-A"]);
    git(root, &["commit", "-qm", "Review demo-a002"]);
    git(root, &["switch", "-qc", "demo-a003", "main"]);
    item(root, "demo-a003", "building", &[REPLACED, TAKER]);
    git(root, &["commit", "-qam", "Take demo-a003 over"]);
    git(root, &["switch", "-q", "main"]);
}

/// Repository `bare`, with no remote, and one ready item.
fn bare(root: &Path) {
    fs::create_dir_all(root).unwrap();
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["config", "user.name", "b"]);
    git(root, &["config", "user.email", "b@b"]);
    let config = "schema_version = 3\nslug = \"bare\"\nbase_branch = \"main\"\n";
    write(root, ".atelier/config.toml", config);
    item(root, "bare-b001", "ready", &[]);
    git(root, &["add", "-A"]);
    git(root, &["commit", "-qm", "Plan"]);
}

fn serve() -> (TempDir, Router) {
    let dir = TempDir::new().unwrap();
    demo(&dir.path().join("repos/demo"));
    bare(&dir.path().join("repos/bare"));
    let mut config: Config = toml::from_str("caller = \"sumeet\"\nroots = []\n").unwrap();
    config.roots = vec![dir.path().join("repos")];
    config.fetch_seconds = 0;
    (dir, router_for(Desk::new(config)))
}

async fn get(app: &Router, uri: &str) -> (StatusCode, Value) {
    let request = Request::get(uri).header("host", "localhost");
    let response = app.clone().oneshot(request.body(Body::empty()).unwrap());
    let response = response.await.unwrap();
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await;
    (status, serde_json::from_slice(&body.unwrap()).unwrap())
}

async fn ok(app: &Router, uri: &str) -> Value {
    let (status, body) = get(app, uri).await;
    assert_eq!(status, StatusCode::OK, "{uri}: {body}");
    body
}

/// `thread` percent-encoded for a query.
fn query(thread: &str) -> String {
    let encode = |b: u8| match b {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
            (b as char).to_string()
        }
        _ => format!("%{b:02X}"),
    };
    let encoded: String = thread.bytes().map(encode).collect();
    format!("/api/ties?thread={encoded}")
}

fn tie(id: &str, kind: &str, status: &str, role: &str, current: bool) -> Value {
    json!({
        "repo": "demo", "kind": kind, "id": id, "title": format!("{} {id}", match kind {
            "item" => "Item",
            _ => "Mission",
        }),
        "status": status, "role": role, "current": current,
    })
}

#[tokio::test]
async fn each_repository_is_named_by_its_origin() {
    let (_dir, app) = serve();
    let body = ok(&app, "/api/repos").await;
    let repos = body["repos"].as_array().unwrap();
    let identity = |name: &str| {
        let repo = repos.iter().find(|r| r["name"] == name).unwrap();
        repo["identity"].clone()
    };
    assert_eq!(identity("demo"), json!("github.com/spadaval/demo"));
    assert_eq!(identity("bare"), Value::Null);
}

#[tokio::test]
async fn an_id_is_named_in_whichever_repository_holds_it() {
    let (_dir, app) = serve();
    assert_eq!(
        ok(&app, "/api/records/demo-a002").await,
        json!({"repo": "demo", "kind": "item", "id": "demo-a002",
               "title": "Item demo-a002", "status": "in_review"})
    );
    assert_eq!(
        ok(&app, "/api/records/demo-mmmm").await,
        json!({"repo": "demo", "kind": "mission", "id": "demo-mmmm",
               "title": "Mission demo-mmmm", "status": "open"})
    );
    let bare = ok(&app, "/api/records/bare-b001").await;
    assert_eq!(
        (&bare["repo"], &bare["status"]),
        (&json!("bare"), &json!("ready"))
    );
    let (status, body) = get(&app, "/api/records/demo-zzzz").await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
    assert!(body["error"].as_str().unwrap().contains("demo-zzzz"));
    let (status, _) = get(&app, "/api/records/not%20an%20id").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn a_thread_is_tied_to_what_it_works_on_now_and_did_before() {
    let (_dir, app) = serve();
    // Neither thread is live: no T3 Code is configured.
    let both = ok(&app, &query(BOTH)).await;
    assert_eq!(both["thread"], BOTH);
    assert_eq!(
        both["ties"],
        json!([
            tie("demo-a001", "item", "done", "implementer", false),
            tie("demo-a002", "item", "in_review", "reviewer", true),
        ])
    );
    let ties = |body: Value| body["ties"].clone();
    assert_eq!(
        ties(ok(&app, &query(REPLACED)).await),
        json!([tie("demo-a003", "item", "building", "implementer", false)])
    );
    assert_eq!(
        ties(ok(&app, &query(TAKER)).await),
        json!([tie("demo-a003", "item", "building", "implementer", true)])
    );
    assert_eq!(
        ties(ok(&app, &query(IMPLEMENTER)).await),
        json!([tie("demo-a002", "item", "in_review", "implementer", true)])
    );
    // A reviewer of a closed round, of an item's or a plan's.
    assert_eq!(
        ties(ok(&app, &query(EARLY)).await),
        json!([
            tie("demo-a002", "item", "in_review", "reviewer", false),
            tie("demo-mmmm", "mission", "open", "reviewer", false),
        ])
    );
    assert_eq!(
        ties(ok(&app, &query(LATE)).await),
        json!([tie("demo-mmmm", "mission", "open", "reviewer", true)])
    );
    assert_eq!(
        ties(ok(&app, &query(PLANNER)).await),
        json!([tie("demo-mmmm", "mission", "open", "planner", true)])
    );
    // The lead of an open mission and of a done one.
    let mut led = ties(ok(&app, &query(LEAD)).await);
    let led = led.as_array_mut().unwrap();
    led.sort_by_key(|t| t["id"].as_str().unwrap().to_string());
    assert_eq!(
        *led,
        [
            tie("demo-dddd", "mission", "done", "lead", false),
            tie("demo-mmmm", "mission", "open", "lead", true),
        ]
    );
    // A thread tied to nothing, and a query naming none.
    assert_eq!(ties(ok(&app, &query("thread:nobody")).await), json!([]));
    let (status, _) = get(&app, "/api/ties?thread=").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let (status, body) = get(&app, "/api/ties").await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
}
