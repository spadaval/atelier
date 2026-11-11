//! `POST /api/act`: `ready` and `decide` made through the Desk leave what
//! the core's own functions, the ones the `atelier` command calls, leave.
//! Each test acts in clones of a scratch repository made here.

use atelier_app::{Repo, Skip};
use atelier_core::Thread;
use atelier_desk::{router_for, Config, Desk};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use tower::ServiceExt;

/// A caller holding a slash, as a Codex subagent's identity does.
const CALLER: &str = "sumeet/desk";
const ALPHA: &str = "demo-aaaa";
const DRAFT: &str = "demo-bbbb";
const STARTED: &str = "demo-a003";
const QUESTION: &str = "2026-11-10: Which way?";

fn git(at: &Path, args: &[&str]) -> String {
    let output = Command::new("git").current_dir(at).args(args).output();
    let output = output.unwrap();
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
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

fn item(root: &Path, id: &str, status: &str, mission: &str, body: &str) {
    let text = format!(
        "---\nschema: atelier.item\nschema_version: 3\nid: {id}\ntitle: Item {id}\n\
         status: {status}\nmission: {mission}\nlabels: []\npriority: high\nblocked_by: []\n\
         implementers: []\nready: null\ntrivial: null\ncaused_by: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Goal\n\nDo {id}.\n\n\
         ## Done when\n\n- [ ] It is done.\n{body}"
    );
    write(root, &format!(".atelier/items/{id}/item.md"), &text);
}

fn identify(root: &Path) {
    git(root, &["config", "user.name", "b"]);
    git(root, &["config", "user.email", "b@b"]);
}

/// A repository on `main` with a ready mission ALPHA, whose item STARTED
/// is building on its own branch with a question, and a draft mission
/// DRAFT that may be made ready.
fn scratch(root: &Path) {
    fs::create_dir_all(root).unwrap();
    git(root, &["init", "-q", "-b", "main"]);
    identify(root);
    let config = "schema_version = 3\nslug = \"demo\"\nbase_branch = \"main\"\n";
    write(root, ".atelier/config.toml", config);
    mission(root, ALPHA, "ready");
    mission(root, DRAFT, "draft");
    item(root, STARTED, "ready", ALPHA, "");
    item(root, "demo-b001", "draft", DRAFT, "");
    git(root, &["add", "-A"]);
    git(root, &["commit", "-qm", "Plan"]);
    git(root, &["switch", "-qc", STARTED]);
    let question = format!("\n## Decisions\n\n### {QUESTION}\n");
    item(root, STARTED, "building", ALPHA, &question);
    git(root, &["commit", "-qam", "Start"]);
    git(root, &["switch", "-q", "main"]);
}

/// A clone of `origin` at `to`, with each of its branches local.
fn clone(origin: &Path, to: &Path) {
    let (from, to_text) = (origin.to_str().unwrap(), to.to_str().unwrap());
    git(origin, &["clone", "-q", from, to_text]);
    identify(to);
    git(to, &["branch", "-q", STARTED, &format!("origin/{STARTED}")]);
}

struct World {
    dir: TempDir,
    /// The repository the Desk serves as `demo`.
    served: PathBuf,
    app: Router,
}

/// A Desk acting as `CALLER` on a clone of a scratch repository.
fn world() -> World {
    let dir = TempDir::new().unwrap();
    scratch(&dir.path().join("origin"));
    let served = dir.path().join("repos/demo");
    clone(&dir.path().join("origin"), &served);
    let text = format!("caller = \"{CALLER}\"\nroots = []\n");
    let mut config: Config = toml::from_str(&text).unwrap();
    config.roots = vec![dir.path().join("repos")];
    let app = router_for(Desk::new(config));
    World { dir, served, app }
}

async fn send(
    app: &Router,
    method: &str,
    uri: &str,
    headers: &[(&str, &str)],
    body: Value,
) -> (StatusCode, Value) {
    let mut request = Request::builder().method(method).uri(uri);
    request = request.header("host", "localhost:3780");
    request = request.header("content-type", "application/json");
    for (name, value) in headers {
        request = request.header(*name, *value);
    }
    let body = match body {
        Value::Null => Body::empty(),
        body => Body::from(body.to_string()),
    };
    let response = app
        .clone()
        .oneshot(request.body(body).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await;
    (
        status,
        serde_json::from_slice(&bytes.unwrap()).unwrap_or(Value::Null),
    )
}

const FROM_APP: &[(&str, &str)] = &[("x-atelier-desk", "1")];

async fn act(app: &Router, body: Value) -> Value {
    let (status, answer) = send(app, "POST", "/api/act", FROM_APP, body).await;
    assert_eq!(status, StatusCode::OK, "{answer}");
    answer
}

async fn get(app: &Router, uri: &str) -> Value {
    let (status, page) = send(app, "GET", uri, &[], Value::Null).await;
    assert_eq!(status, StatusCode::OK, "{uri}: {page}");
    page
}

/// What the CLI's `atelier ready <ids> --by <by>` calls, in `checkout`.
fn ready_as_the_cli_does(checkout: &Path, ids: &[&str], by: &str) -> Value {
    let skip = Skip::new(None);
    let mut repo = Repo::open(checkout, Thread(CALLER.into()), &skip).unwrap();
    let ids: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
    match atelier_app::ready(&mut repo, &ids, by).unwrap() {
        Ok(done) => {
            json!({"result": "done", "text": done.text, "paths": done.paths, "commit": done.commit})
        }
        Err(refused) => {
            json!({"result": "refused", "rules": refused.rules, "text": refused.text, "next": refused.next})
        }
    }
}

/// The last commit's message and the paths it changed.
fn last_commit(at: &Path) -> (String, String) {
    let message = git(at, &["log", "-1", "--format=%B"]);
    let paths = git(
        at,
        &["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"],
    );
    (message, paths)
}

/// A mission's record, with the time `ready` recorded set aside.
fn without_time(at: &Path, id: &str) -> String {
    let text = fs::read_to_string(at.join(format!(".atelier/missions/{id}/mission.md"))).unwrap();
    let lines = text
        .lines()
        .filter(|line| !line.trim_start().starts_with("at: "));
    lines.collect::<Vec<_>>().join("\n")
}

#[tokio::test]
async fn saying_go_from_the_desk_leaves_the_commit_the_cli_leaves() {
    let world = world();
    let origin = world.dir.path().join("origin");
    let by_cli = world.dir.path().join("by-cli");
    clone(&origin, &by_cli);
    let before = git(&world.served, &["rev-parse", "HEAD"]);
    assert_eq!(before, git(&by_cli, &["rev-parse", "HEAD"]));

    let done = act(
        &world.app,
        json!({"act": "ready", "repo": "demo", "ids": [DRAFT]}),
    )
    .await;
    let cli = ready_as_the_cli_does(&by_cli, &[DRAFT], CALLER);

    assert_eq!(done["result"], "done", "{done}");
    assert_eq!(done["paths"], cli["paths"]);
    // Each says the commit it made, which differ in their time.
    let said = |at: &Path, text: &Value| {
        let short = git(at, &["rev-parse", "--short", "HEAD"]);
        text.as_str().unwrap().replace(&short, "<commit>")
    };
    assert_eq!(
        said(&world.served, &done["text"]),
        said(&by_cli, &cli["text"])
    );
    for at in [&world.served, &by_cli] {
        let count = git(at, &["rev-list", "--count", &format!("{before}..HEAD")]);
        assert_eq!(count, "1", "one new commit in {}", at.display());
    }
    assert_eq!(done["commit"], git(&world.served, &["rev-parse", "HEAD"]));
    assert_eq!(last_commit(&world.served), last_commit(&by_cli));
    let paths = last_commit(&world.served).1;
    assert_eq!(paths, format!(".atelier/missions/{DRAFT}/mission.md"));
    assert_eq!(
        without_time(&world.served, DRAFT),
        without_time(&by_cli, DRAFT)
    );
    assert!(without_time(&world.served, DRAFT).contains(&format!("by: {CALLER}\n")));

    // The comparison can fail: the same act under another name differs.
    let other = world.dir.path().join("other");
    clone(&origin, &other);
    ready_as_the_cli_does(&other, &[DRAFT], "someone-else");
    assert_ne!(
        without_time(&world.served, DRAFT),
        without_time(&other, DRAFT)
    );

    let page = get(&world.app, &format!("/api/repos/demo/missions/{DRAFT}")).await;
    assert_eq!(page["mission"]["status"], "ready");
    assert_eq!(page["mission"]["ready"]["by"], CALLER);
}

#[tokio::test]
async fn saying_go_leaves_an_unrelated_staged_file_staged() {
    let world = world();
    write(&world.served, "notes.txt", "mine\n");
    git(&world.served, &["add", "notes.txt"]);
    let done = act(
        &world.app,
        json!({"act": "ready", "repo": "demo", "ids": [DRAFT]}),
    )
    .await;
    assert_eq!(done["result"], "done", "{done}");
    assert_eq!(
        git(&world.served, &["diff", "--cached", "--name-only"]),
        "notes.txt"
    );
    assert!(!last_commit(&world.served).1.contains("notes.txt"));
}

#[tokio::test]
async fn an_answer_waits_in_memory_until_a_commit_on_the_branch_holds_it() {
    let world = world();
    let checkout = world.dir.path().join("started");
    let path = checkout.to_str().unwrap();
    git(&world.served, &["worktree", "add", "-q", path, STARTED]);
    let tips = || git(&world.served, &["rev-parse", "main", STARTED]);
    let before = tips();
    let item_uri = format!("/api/repos/demo/items/{STARTED}");
    let mission_uri = format!("/api/repos/demo/missions/{ALPHA}");
    let page = get(&world.app, &item_uri).await;
    assert_eq!(page["can_decide"], true);
    let found = fs::canonicalize(page["decide_at"].as_str().unwrap()).unwrap();
    assert_eq!(found, fs::canonicalize(&checkout).unwrap());
    let inbox = get(&world.app, "/api/inbox").await;
    assert_eq!(inbox["repos"][0]["questions"][0]["item"], STARTED);

    let body = json!({"act": "decide", "repo": "demo", "id": STARTED, "answer": "This way.", "settle": null});
    let done = act(&world.app, body).await;
    assert_eq!(done["result"], "done", "{done}");
    assert_eq!(done["commit"], Value::Null);
    let written = format!(".atelier/items/{STARTED}/item.md");
    assert_eq!(done["paths"], json!([written]));
    assert_eq!(tips(), before, "decide commits nothing");
    assert_eq!(
        git(&checkout, &["status", "--porcelain"]),
        format!("M {written}")
    );
    let record = fs::read_to_string(checkout.join(&written)).unwrap();
    assert!(
        record.contains(&format!("Answer by {CALLER}: This way.")),
        "{record}"
    );

    let pending = json!([{"item": STARTED, "question": QUESTION, "finding": null,
        "answer": {"by": CALLER, "text": "This way."}, "outcome": null}]);
    assert_eq!(
        get(&world.app, &mission_uri).await["pending_answers"],
        pending
    );
    let page = get(&world.app, &item_uri).await;
    assert_eq!(page["pending_answers"], pending);
    assert_eq!(page["decisions"][0]["answer"], Value::Null);
    let inbox = get(&world.app, "/api/inbox").await;
    assert_eq!(inbox["repos"][0]["questions"], json!([]));

    git(&checkout, &["commit", "-qam", "Submit"]);
    let page = get(&world.app, &item_uri).await;
    assert_eq!(page["pending_answers"], json!([]));
    assert_eq!(page["decisions"][0]["answer"]["by"], CALLER);
    assert_eq!(
        get(&world.app, &mission_uri).await["pending_answers"],
        json!([])
    );
}

#[tokio::test]
async fn an_answer_is_told_from_an_earlier_one_under_the_same_heading() {
    let world = world();
    let checkout = world.dir.path().join("started");
    let path = checkout.to_str().unwrap();
    git(&world.served, &["worktree", "add", "-q", path, STARTED]);
    // The question asked again under the same heading, the first answered.
    let asked = format!(
        "\n## Decisions\n\n### {QUESTION}\n\nAnswer by someone: That way.\n\n### {QUESTION}\n"
    );
    item(&checkout, STARTED, "building", ALPHA, &asked);
    git(&checkout, &["commit", "-qam", "Ask again"]);
    let item_uri = format!("/api/repos/demo/items/{STARTED}");
    let mission_uri = format!("/api/repos/demo/missions/{ALPHA}");
    let inbox = get(&world.app, "/api/inbox").await;
    assert_eq!(inbox["repos"][0]["questions"][0]["item"], STARTED);

    let body = json!({"act": "decide", "repo": "demo", "id": STARTED, "answer": "This way."});
    assert_eq!(act(&world.app, body).await["result"], "done");
    let pending = json!([{"item": STARTED, "question": QUESTION, "finding": null,
        "answer": {"by": CALLER, "text": "This way."}, "outcome": null}]);
    assert_eq!(
        get(&world.app, &mission_uri).await["pending_answers"],
        pending
    );
    assert_eq!(get(&world.app, &item_uri).await["pending_answers"], pending);
    let inbox = get(&world.app, "/api/inbox").await;
    assert_eq!(inbox["repos"][0]["questions"], json!([]));

    git(&checkout, &["commit", "-qam", "Submit"]);
    assert_eq!(
        get(&world.app, &mission_uri).await["pending_answers"],
        json!([])
    );
    let page = get(&world.app, &item_uri).await;
    assert_eq!(page["pending_answers"], json!([]));
    assert_eq!(page["decisions"][1]["answer"]["by"], CALLER);
}

#[tokio::test]
async fn where_no_checkout_has_the_branch_the_function_refuses() {
    let world = world();
    // `decide` with no checkout of the item's branch, from the checkout
    // of the base, as the function refuses there.
    let body = json!({"act": "decide", "repo": "demo", "id": STARTED, "answer": "This way."});
    let refused = act(&world.app, body).await;
    let skip = Skip::new(None);
    let mut repo = Repo::open(&world.served, Thread(CALLER.into()), &skip).unwrap();
    let own = atelier_app::decide(&mut repo, STARTED, "This way.", None).unwrap();
    let own = own.unwrap_err();
    assert_eq!(
        refused,
        json!({"result": "refused", "rules": own.rules, "text": own.text, "next": own.next})
    );
    assert!(own.next.is_some());

    // `ready` with no checkout of the base.
    git(&world.served, &["switch", "-qc", "elsewhere"]);
    let refused = act(
        &world.app,
        json!({"act": "ready", "repo": "demo", "ids": [DRAFT]}),
    )
    .await;
    let own = ready_as_the_cli_does(&world.served, &[DRAFT], CALLER);
    assert_eq!(refused, own);
    assert_eq!(refused["result"], "refused");
    assert!(refused["next"].is_string());
    assert_eq!(git(&world.served, &["rev-list", "--count", "main"]), "1");
}

#[tokio::test]
async fn only_the_desks_own_app_may_act() {
    let world = world();
    let head = || git(&world.served, &["rev-parse", "HEAD"]);
    let before = head();
    let go = json!({"act": "ready", "repo": "demo", "ids": [DRAFT]});
    let session = json!({"repo": "demo", "mission": DRAFT});
    for uri in ["/api/act", "/api/sessions"] {
        let body = if uri == "/api/act" { &go } else { &session };
        for headers in [
            &[][..],
            &[("x-atelier-desk", "0")][..],
            &[
                ("x-atelier-desk", "1"),
                ("origin", "http://attacker.example"),
            ][..],
            &[("x-atelier-desk", "1"), ("origin", "null")][..],
        ] {
            let (status, answer) = send(&world.app, "POST", uri, headers, body.clone()).await;
            assert_eq!(status, StatusCode::FORBIDDEN, "{uri} {headers:?}: {answer}");
            assert!(answer["error"].is_string());
        }
    }
    assert_eq!(head(), before, "a refused act writes nothing");

    // From the app, at a host it answers to.
    let headers = [("x-atelier-desk", "1"), ("origin", "http://localhost:5173")];
    let (status, done) = send(&world.app, "POST", "/api/act", &headers, go).await;
    assert_eq!(
        (status, &done["result"]),
        (StatusCode::OK, &json!("done")),
        "{done}"
    );

    // A request the app could not have meant.
    for (body, status) in [
        (
            json!({"act": "close", "repo": "demo", "ids": [DRAFT]}),
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            json!({"act": "ready", "repo": "demo", "ids": [DRAFT], "by": "x"}),
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            json!({"act": "decide", "repo": "demo", "id": "../x", "answer": "a"}),
            StatusCode::BAD_REQUEST,
        ),
        (
            json!({"act": "ready", "repo": "nowhere", "ids": [DRAFT]}),
            StatusCode::NOT_FOUND,
        ),
    ] {
        let (got, answer) = send(&world.app, "POST", "/api/act", FROM_APP, body.clone()).await;
        assert_eq!(got, status, "{body}: {answer}");
        assert!(answer["error"].is_string(), "{body}: {answer}");
    }
}
