//! T3 Code's threads in the Desk, read from a stand-in MCP server that
//! answers in the shapes the real one gives (`fixtures/t3_thread_list.json`
//! is `structuredContent` of a real `t3_thread_list` call), with tokens from
//! a stand-in `t3` command.

use atelier_desk::config::T3Config;
use atelier_desk::{router_for, Config, Desk};
use axum::body::Body;
use axum::http::{HeaderMap, Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use tempfile::TempDir;
use tokio::task::JoinHandle;
use tower::ServiceExt;

const MISSION: &str = "demo-mmmm";
const LED: &str = "demo-llll";
const FIRST: &str = "demo-a001";
const SECOND: &str = "demo-a002";
/// An identity holding `/`, `:` and `%`, as T3 Code's and Codex's do.
const ODD: &str = "thread:delegated-task:command%3Amcp%3Ad4040a34%3Adelegate-task%3A6b85/sub:1";
const SECOND_IMPLEMENTER: &str = "5bcb08e9-f2d3-4f91-8690-1f2c2e9401a5";
const REVIEWER: &str = "019a1b2c-reviewer-not-live";
const LEAD: &str = "thread:lead";
const NOBODY: &str = "thread:unnamed";

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

fn mission(root: &Path, id: &str, lead: &str) {
    let text = format!(
        "---\nschema: atelier.mission\nschema_version: 3\nid: {id}\ntitle: Mission {id}\n\
         status: ready\nlabels: []\nlead: {lead}\nready: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n## Why\n\nBecause.\n"
    );
    write(root, &format!(".atelier/missions/{id}/mission.md"), &text);
}

fn item(root: &Path, id: &str, mission: &str, implementer: &str, body: &str) {
    let text = format!(
        "---\nschema: atelier.item\nschema_version: 3\nid: {id}\ntitle: Item {id}\n\
         status: building\nmission: {mission}\nlabels: []\npriority: high\nblocked_by: []\n\
         implementers:\n- {implementer}\nready: null\ntrivial: null\ncaused_by: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Goal\n\nDo {id}.\n\n\
         ## Done when\n\n- [ ] It is done.\n{body}"
    );
    write(root, &format!(".atelier/items/{id}/item.md"), &text);
}

/// A repository whose mission `MISSION` has no lead and two items held by
/// different threads, the second also reviewed by `REVIEWER` and with a
/// question answered by `sumeet`; and whose mission `LED` has `LEAD`.
fn scratch(root: &Path) {
    fs::create_dir_all(root).unwrap();
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["config", "user.name", "b"]);
    git(root, &["config", "user.email", "b@b"]);
    let config = "schema_version = 3\nslug = \"demo\"\nbase_branch = \"main\"\n";
    write(root, ".atelier/config.toml", config);
    mission(root, MISSION, "null");
    mission(root, LED, LEAD);
    item(root, FIRST, MISSION, ODD, "");
    let answered = "\n## Decisions\n\n### 2026-11-10: Which way?\n\nAnswer by sumeet: This way.\n";
    item(root, SECOND, MISSION, SECOND_IMPLEMENTER, answered);
    let review = format!(
        "---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: 0123abc\n\
         reviewers:\n- {REVIEWER}\nmodel: null\nverdict: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Summary\n\nFine.\n"
    );
    write(
        root,
        &format!(".atelier/items/{SECOND}/reviews/1-x7qd.md"),
        &review,
    );
    git(root, &["add", "-A"]);
    git(root, &["commit", "-qm", "Plan"]);
}

/// A stand-in `t3` command: each `issue` prints a new token, `secret-<n>`,
/// for session `session-<n>`, expiring at the time in `expires` if that
/// file exists; every run is logged in `calls`.
const T3: &str = r#"#!/bin/sh
dir=$(dirname "$0")
echo "$*" >> "$dir/calls"
if [ "$1 $2 $3" = "auth session issue" ]; then
  n=$(grep -c ' issue ' "$dir/calls")
  expires=$(cat "$dir/expires" 2>/dev/null || echo 2099-01-01T00:00:00.000Z)
  printf '{"sessionId":"session-%s","token":"secret-%s","method":"bearer-access-token","scopes":["orchestration:read"],"subject":"mcp-client","client":{"label":"atelier-desk","deviceType":"bot"},"expiresAt":"%s"}\n' "$n" "$n" "$expires"
elif [ "$1 $2 $3" = "auth session revoke" ]; then
  echo "Revoked session $4."
fi
"#;

/// The stand-in MCP server's state: the threads it lists, the tokens it
/// refuses, and each request it was sent with its bearer token.
#[derive(Default)]
struct StandIn {
    threads: Mutex<Vec<Value>>,
    refused: Mutex<Vec<String>>,
    requests: Mutex<Vec<(String, Value)>>,
}

/// Answers as T3 Code's MCP server does: `initialize` opens a session,
/// every other request needs it, and `t3_thread_list` gives one thread per
/// page so paging is exercised.
async fn mcp(
    axum::extract::State(standin): axum::extract::State<Arc<StandIn>>,
    headers: HeaderMap,
    Json(request): Json<Value>,
) -> Response {
    let header = |name: &str| headers.get(name).and_then(|v| v.to_str().ok());
    let bearer = header("authorization").unwrap_or("").replace("Bearer ", "");
    standin
        .requests
        .lock()
        .unwrap()
        .push((bearer.clone(), request.clone()));
    if standin.refused.lock().unwrap().contains(&bearer) {
        let error = json!({"error": "invalid_mcp_credential"});
        return (StatusCode::UNAUTHORIZED, Json(error)).into_response();
    }
    let id = request["id"].clone();
    match request["method"].as_str().unwrap() {
        "initialize" => {
            let result = json!({"jsonrpc": "2.0", "id": id, "result": {
                "protocolVersion": "2025-06-18",
                "capabilities": {"logging": {}, "tools": {"listChanged": true}, "completions": {}},
                "serverInfo": {"name": "T3 Code", "version": "0.1.9-nightly.20261110.4478"}}});
            ([("mcp-session-id", "stand-in-session")], Json(result)).into_response()
        }
        "notifications/initialized" => StatusCode::ACCEPTED.into_response(),
        _ if header("mcp-session-id") != Some("stand-in-session") => {
            let error = json!({"jsonrpc": "2.0", "id": id, "error": {"code": -32600,
                "message": "MCP-Session-Id header is required; send initialize to start a session"}});
            (StatusCode::BAD_REQUEST, Json(error)).into_response()
        }
        "tools/call" => {
            assert_eq!(request["params"]["name"], "t3_thread_list");
            let threads = standin.threads.lock().unwrap().clone();
            let cursor = request["params"]["arguments"]["cursor"]
                .as_u64()
                .unwrap_or(0) as usize;
            let page: Vec<Value> = threads.iter().skip(cursor).take(1).cloned().collect();
            let next = (cursor + 1 < threads.len()).then_some(cursor + 1);
            let content = json!({"projectId": null, "currentThreadId": null,
                "threads": page, "nextCursor": next, "total": threads.len()});
            let result = json!({"content": [{"type": "text", "text": content.to_string()}],
                "structuredContent": content, "isError": false});
            Json(json!({"jsonrpc": "2.0", "id": id, "result": result})).into_response()
        }
        other => panic!("the stand-in does not take {other}"),
    }
}

/// A thread in the real list's shape, with `id`, `status` and `title`.
fn thread(id: &str, status: &str, title: &str) -> Value {
    let fixture = include_str!("fixtures/t3_thread_list.json");
    let list: Value = serde_json::from_str(fixture).unwrap();
    let mut thread = list["threads"][1].clone();
    thread["threadId"] = json!(id);
    thread["status"] = json!(status);
    thread["title"] = json!(title);
    thread["link"] = json!(format!("https://app.t3.codes/threads/env/{id}"));
    thread
}

struct World {
    dir: TempDir,
    standin: Arc<StandIn>,
    server: Option<JoinHandle<()>>,
    desk: Arc<Desk>,
    app: Router,
}

impl World {
    /// The scratch repository served by a Desk whose `[t3]` names the
    /// stand-in command and the MCP server at `url`, or the stand-in
    /// server when `url` is `None`; or with no `[t3]` when `configured` is
    /// false.
    async fn new(url: Option<String>, configured: bool) -> World {
        let dir = TempDir::new().unwrap();
        scratch(&dir.path().join("repos/demo"));
        let command = dir.path().join("t3");
        fs::write(&command, T3).unwrap();
        Command::new("chmod")
            .arg("+x")
            .arg(&command)
            .status()
            .unwrap();
        let standin = Arc::new(StandIn::default());
        let mcp = Router::new()
            .route("/mcp", post(mcp))
            .with_state(standin.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move { axum::serve(listener, mcp).await.unwrap() });
        let text = "caller = \"sumeet\"\nroots = []\n";
        let mut config: Config = toml::from_str(text).unwrap();
        config.roots = vec![dir.path().join("repos")];
        config.t3 = configured.then(|| T3Config {
            command,
            url: url.unwrap_or(format!("http://{address}/mcp")),
            session_message: atelier_desk::config::SESSION_MESSAGE.into(),
        });
        let desk = Desk::new(config);
        World {
            app: router_for(desk.clone()),
            desk,
            standin,
            server: Some(server),
            dir,
        }
    }

    fn list(&self, threads: Vec<Value>) {
        *self.standin.threads.lock().unwrap() = threads;
    }

    async fn poll(&self) {
        let threads = self.desk.threads.clone();
        tokio::task::spawn_blocking(move || threads.poll())
            .await
            .unwrap();
    }

    fn calls(&self) -> Vec<String> {
        let calls = fs::read_to_string(self.dir.path().join("calls")).unwrap_or_default();
        calls.lines().map(str::to_string).collect()
    }

    async fn get(&self, uri: &str) -> Value {
        let request = Request::get(uri).header("host", "localhost");
        let response = self
            .app
            .clone()
            .oneshot(request.body(Body::empty()).unwrap());
        let response = response.await.unwrap();
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await;
        let text = String::from_utf8(body.unwrap().to_vec()).unwrap();
        assert_eq!(status, StatusCode::OK, "{uri}: {text}");
        assert!(!text.contains("secret-"), "{uri} gave a token: {text}");
        serde_json::from_str(&text).unwrap()
    }
}

/// The four live threads: the two implementers, the lead, and one no
/// record names; the second implementer and the unnamed one are waiting.
fn four() -> Vec<Value> {
    vec![
        thread(ODD, "running", "Implement demo-a001"),
        thread(SECOND_IMPLEMENTER, "waiting", "Implement demo-a002"),
        thread(LEAD, "running", "Lead demo-llll"),
        thread(NOBODY, "waiting", "A chat about nothing tracked"),
    ]
}

fn hold(kind: &str, id: &str, role: &str) -> Value {
    json!({"repo": "demo", "kind": kind, "id": id, "role": role})
}

#[tokio::test(flavor = "multi_thread")]
async fn live_threads_are_listed_with_what_they_hold() {
    let world = World::new(None, true).await;
    world.list(four());
    // A request never calls T3 Code: only the poll does.
    let before = world.get("/api/threads").await;
    assert_eq!(before["t3"]["error"], "T3 Code has not been polled yet.");
    assert!(world.calls().is_empty());
    assert!(world.standin.requests.lock().unwrap().is_empty());
    world.poll().await;
    let body = world.get("/api/threads").await;
    assert_eq!(
        body["t3"],
        json!({"reachable": true, "age_seconds": 0, "error": null})
    );
    let threads = body["threads"].as_array().unwrap();
    let ids: Vec<&str> = threads.iter().map(|t| t["id"].as_str().unwrap()).collect();
    assert_eq!(ids, [ODD, SECOND_IMPLEMENTER, LEAD, NOBODY]);
    assert_eq!(
        threads[0]["holds"],
        json!([hold("item", FIRST, "implementer")])
    );
    assert_eq!(
        threads[1]["holds"],
        json!([hold("item", SECOND, "implementer")])
    );
    assert_eq!(threads[2]["holds"], json!([hold("mission", LED, "lead")]));
    // A thread no record names is tied to no repository.
    assert_eq!(threads[3]["holds"], json!([]));
    let first = &threads[1];
    assert_eq!(first["title"], "Implement demo-a002");
    assert_eq!(first["status"], "waiting");
    assert_eq!(first["provider"], "claudeAgent");
    assert_eq!(first["model"], "claude-fable-5-2");
    assert_eq!(first["parent"], Value::Null);
    assert_eq!(first["updated_at"], "2026-11-11T06:28:22.762Z");
    let link = format!("https://app.t3.codes/threads/env/{SECOND_IMPLEMENTER}");
    assert_eq!(first["link"], link);
    assert_eq!(
        first["pull_requests"][0],
        json!({"repository": "spadaval/atelier", "number": 88,
               "url": "https://github.com/spadaval/atelier/pull/88", "state": "merged"})
    );

    // One token, read-only, from the configured command; and the list is
    // asked for live, unsettled threads in every project, page by page.
    let issue = "auth session issue --subject mcp-client --scope orchestration:read \
                 --ttl 1h --label atelier-desk --json";
    assert_eq!(world.calls(), [issue]);
    let requests = world.standin.requests.lock().unwrap().clone();
    assert!(requests.iter().all(|(bearer, _)| bearer == "secret-1"));
    let calls: Vec<&Value> = (requests.iter())
        .filter(|(_, r)| r["method"] == "tools/call")
        .map(|(_, r)| &r["params"]["arguments"])
        .collect();
    assert_eq!(
        calls.len(),
        5,
        "one page per thread, and the recent threads"
    );
    assert_eq!(calls[0]["allProjects"], true);
    assert_eq!(calls[0]["includeSubagents"], true);
    assert_eq!(calls[0]["settled"], false);
    let live = ["preparing", "queued", "starting", "running", "waiting"];
    assert_eq!(calls[0]["statuses"], json!(live));
    assert_eq!(calls[3]["cursor"], 3);
    // The first poll also reads the recent threads, ended ones too, for
    // the models of threads no longer live.
    assert_eq!(calls[4]["statuses"], Value::Null);
    assert_eq!(calls[4]["settled"], Value::Null);
    assert_eq!(calls[4]["limit"], 100);
    let mut side = atelier_desk::api::Side {
        identity: SECOND_IMPLEMENTER.into(),
        model: None,
        provider: None,
        link: None,
    };
    world.desk.threads.describe(&mut side);
    assert_eq!(side.model.as_deref(), Some("claude-fable-5-2"));
    assert_eq!(side.provider.as_deref(), Some("claudeAgent"));

    // Stopping revokes the token's session, and no new one is issued.
    world.desk.threads.close();
    world.poll().await;
    assert_eq!(world.calls()[1..], ["auth session revoke session-1"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_mission_and_its_items_show_their_agents() {
    let world = World::new(None, true).await;
    world.list(four());
    world.poll().await;
    let mark = |title: &str, status: &str, id: &str| {
        json!({"title": title, "status": status, "provider": "claudeAgent",
               "model": "claude-fable-5-2",
               "link": format!("https://app.t3.codes/threads/env/{id}")})
    };
    let page = world
        .get(&format!("/api/repos/demo/missions/{MISSION}"))
        .await;
    assert_eq!(page["mission"]["lead"], Value::Null);
    assert_eq!(page["t3"]["reachable"], true);
    assert_eq!(
        page["threads"],
        json!([
            {"identity": ODD, "role": "implementer", "item": FIRST,
             "thread": mark("Implement demo-a001", "running", ODD)},
            {"identity": SECOND_IMPLEMENTER, "role": "implementer", "item": SECOND,
             "thread": mark("Implement demo-a002", "waiting", SECOND_IMPLEMENTER)},
            {"identity": REVIEWER, "role": "reviewer", "item": SECOND, "thread": null},
            {"identity": "sumeet", "role": "decider", "item": SECOND, "thread": null},
        ])
    );
    let led = world.get(&format!("/api/repos/demo/missions/{LED}")).await;
    assert_eq!(
        led["threads"],
        json!([{"identity": LEAD, "role": "lead", "item": null,
                "thread": mark("Lead demo-llll", "running", LEAD)}])
    );
    let item = world.get(&format!("/api/repos/demo/items/{FIRST}")).await;
    assert_eq!(item["implementers"], json!([ODD]));
    assert_eq!(
        item["threads"],
        json!([{"identity": ODD, "role": "implementer", "item": null,
                "thread": mark("Implement demo-a001", "running", ODD)}])
    );

    // A thread that stops being live loses its mark; the identity stays.
    world.list(vec![thread(LEAD, "running", "Lead demo-llll")]);
    world.poll().await;
    let item = world.get(&format!("/api/repos/demo/items/{FIRST}")).await;
    assert_eq!(
        item["threads"],
        json!([{"identity": ODD, "role": "implementer", "item": null, "thread": null}])
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn the_inbox_lists_waiting_threads_that_hold_work_in_the_repository() {
    let world = World::new(None, true).await;
    world.list(four());
    world.poll().await;
    let inbox = world.get("/api/inbox").await;
    assert_eq!(inbox["t3"]["reachable"], true);
    let threads = inbox["repos"][0]["threads"].as_array().unwrap();
    let ids: Vec<&str> = threads.iter().map(|t| t["id"].as_str().unwrap()).collect();
    // The running ones are not waiting, and the waiting one no record
    // names is in no repository.
    assert_eq!(ids, [SECOND_IMPLEMENTER]);
    assert_eq!(
        threads[0]["holds"],
        json!([hold("item", SECOND, "implementer")])
    );
}

/// Every route answers 200 with its records, its threads marked
/// unavailable.
async fn unavailable(world: &World, why: &str) {
    let threads = world.get("/api/threads").await;
    assert_eq!(threads["t3"]["reachable"], false);
    let error = threads["t3"]["error"].as_str().unwrap();
    assert!(error.contains(why), "{error}");
    assert_eq!(threads["threads"], json!([]));
    let page = world
        .get(&format!("/api/repos/demo/missions/{MISSION}"))
        .await;
    assert_eq!(page["t3"], threads["t3"]);
    assert_eq!(page["items"].as_array().unwrap().len(), 2);
    assert_eq!(page["threads"][0]["identity"], ODD);
    assert_eq!(page["threads"][0]["thread"], Value::Null);
    let item = world.get(&format!("/api/repos/demo/items/{FIRST}")).await;
    assert_eq!(item["t3"], threads["t3"]);
    assert_eq!(item["threads"][0]["thread"], Value::Null);
    let inbox = world.get("/api/inbox").await;
    assert_eq!(inbox["t3"], threads["t3"]);
    assert_eq!(inbox["repos"][0]["threads"], json!([]));
    world.get("/api/repos").await;
}

#[tokio::test(flavor = "multi_thread")]
async fn with_t3_code_stopped_every_route_still_answers() {
    // A port that refuses connections: bound, then released.
    let closed = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/mcp", closed.local_addr().unwrap());
    drop(closed);
    let world = World::new(Some(url), true).await;
    world.poll().await;
    unavailable(&world, "cannot reach T3 Code's MCP server").await;
}

#[tokio::test(flavor = "multi_thread")]
async fn without_a_t3_table_no_call_is_made() {
    let world = World::new(None, false).await;
    world.list(four());
    world.poll().await;
    unavailable(&world, "not configured").await;
    assert_eq!(world.calls(), Vec::<String>::new());
    assert!(world.standin.requests.lock().unwrap().is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn a_failed_poll_keeps_the_last_list() {
    let mut world = World::new(None, true).await;
    world.list(four());
    world.poll().await;
    world.server.take().unwrap().abort();
    // The aborted server's open connection may answer once more.
    for _ in 0..3 {
        world.poll().await;
    }
    let body = world.get("/api/threads").await;
    assert_eq!(body["t3"]["reachable"], false);
    assert_eq!(body["t3"]["age_seconds"], 0);
    let error = body["t3"]["error"].as_str().unwrap();
    assert!(error.contains("cannot reach"), "{error}");
    assert_eq!(body["threads"].as_array().unwrap().len(), 4);
    let page = world.get(&format!("/api/repos/demo/items/{FIRST}")).await;
    assert_eq!(page["threads"][0]["thread"]["status"], "running");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_refused_or_expired_token_is_replaced_and_revoked() {
    let world = World::new(None, true).await;
    // One thread, so each poll is one call.
    world.list(vec![thread(LEAD, "running", "Lead demo-llll")]);
    world
        .standin
        .refused
        .lock()
        .unwrap()
        .push("secret-1".into());
    world.poll().await;
    let body = world.get("/api/threads").await;
    assert_eq!(body["t3"]["reachable"], true, "{body}");
    let calls = world.calls();
    assert_eq!(calls.len(), 3, "{calls:?}");
    assert_eq!(calls[1], "auth session revoke session-1");
    assert!(calls[2].starts_with("auth session issue "));

    // A token at its expiry is replaced before it is used again.
    fs::write(world.dir.path().join("expires"), "2026-01-01T00:00:00.000Z").unwrap();
    world
        .standin
        .refused
        .lock()
        .unwrap()
        .push("secret-2".into());
    world.poll().await; // secret-2 is refused; secret-3 is issued, already expired
    world.poll().await; // secret-3 is replaced by secret-4 before use
    let calls = world.calls();
    assert_eq!(calls[3], "auth session revoke session-2");
    assert!(calls[4].starts_with("auth session issue "));
    assert_eq!(calls[5], "auth session revoke session-3");
    assert!(calls[6].starts_with("auth session issue "));
    assert_eq!(calls.len(), 7, "{calls:?}");
    let last = world
        .standin
        .requests
        .lock()
        .unwrap()
        .last()
        .unwrap()
        .0
        .clone();
    assert_eq!(last, "secret-4");

    // Every token refused: the error says so and quotes none.
    let all = (1..10).map(|n| format!("secret-{n}"));
    world.standin.refused.lock().unwrap().extend(all);
    world.poll().await;
    let body = world.get("/api/threads").await;
    assert_eq!(body["t3"]["reachable"], false);
    let error = body["t3"]["error"].as_str().unwrap();
    assert!(error.contains("refused the Desk's token"), "{error}");
}
