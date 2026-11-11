//! `POST /api/sessions`: a planning session starts a T3 Code thread in the
//! repository's T3 project, under a token that may operate and is issued
//! for that and revoked after. T3 Code is a stand-in here that answers in
//! the real server's shapes (`t3_project_list`, `t3_thread_launch` in
//! T3 Code's `apps/server/src/mcp/toolkits/project/`) and refuses a launch
//! under a token that only reads, as the real one does.

use atelier_desk::config::{T3Config, SESSION_MESSAGE};
use atelier_desk::{router_for, Config, Desk};
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderMap, Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use tempfile::TempDir;
use tower::ServiceExt;

const MISSION: &str = "demo-mmmm";

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

fn scratch(root: &Path) {
    fs::create_dir_all(root).unwrap();
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["config", "user.name", "b"]);
    git(root, &["config", "user.email", "b@b"]);
    let config = "schema_version = 3\nslug = \"demo\"\nbase_branch = \"main\"\n";
    write(root, ".atelier/config.toml", config);
    let mission = format!(
        "---\nschema: atelier.mission\nschema_version: 3\nid: {MISSION}\ntitle: The plan\n\
         status: draft\nlabels: []\nlead: null\nready: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n## Outcome\n\nIt works.\n"
    );
    write(
        root,
        &format!(".atelier/missions/{MISSION}/mission.md"),
        &mission,
    );
    git(root, &["add", "-A"]);
    git(root, &["commit", "-qm", "Plan"]);
}

/// A stand-in `t3` command: each `issue` prints a token naming its number
/// and its scope, `secret-<n>-<scope>`; every run is logged in `calls`.
const T3: &str = r#"#!/bin/sh
dir=$(dirname "$0")
echo "$*" >> "$dir/calls"
if [ "$1 $2 $3" = "auth session issue" ]; then
  n=$(grep -c ' issue ' "$dir/calls")
  printf '{"sessionId":"session-%s","token":"secret-%s-%s","expiresAt":"2099-01-01T00:00:00.000Z"}\n' "$n" "$n" "$7"
fi
"#;

#[derive(Default)]
struct StandIn {
    projects: Mutex<Vec<Value>>,
    /// Each tool called, with the token it was called with.
    calls: Mutex<Vec<(String, String, Value)>>,
}

fn tool_result(id: Value, content: Value, error: bool) -> Response {
    let result = json!({"content": [{"type": "text", "text": content.to_string()}],
        "structuredContent": content, "isError": error});
    Json(json!({"jsonrpc": "2.0", "id": id, "result": result})).into_response()
}

async fn mcp(
    State(standin): State<Arc<StandIn>>,
    headers: HeaderMap,
    Json(request): Json<Value>,
) -> Response {
    let bearer = headers.get("authorization").and_then(|v| v.to_str().ok());
    let bearer = bearer.unwrap_or("").replace("Bearer ", "");
    let id = request["id"].clone();
    match request["method"].as_str().unwrap() {
        "initialize" => {
            let result =
                json!({"jsonrpc": "2.0", "id": id, "result": {"protocolVersion": "2025-06-18"}});
            ([("mcp-session-id", "stand-in")], Json(result)).into_response()
        }
        "notifications/initialized" => StatusCode::ACCEPTED.into_response(),
        "tools/call" => {
            let tool = request["params"]["name"].as_str().unwrap().to_string();
            let arguments = request["params"]["arguments"].clone();
            let call = (tool.clone(), bearer.clone(), arguments.clone());
            standin.calls.lock().unwrap().push(call);
            match tool.as_str() {
                "t3_project_list" => {
                    // One project per page, so paging is exercised.
                    let projects = standin.projects.lock().unwrap().clone();
                    let cursor = arguments["cursor"].as_u64().unwrap() as usize;
                    let page: Vec<Value> = projects.iter().skip(cursor).take(1).cloned().collect();
                    let next = (cursor + 1 < projects.len()).then_some(cursor + 1);
                    tool_result(id, json!({"projects": page, "nextCursor": next}), false)
                }
                "t3_thread_launch" if !bearer.ends_with("orchestration:operate") => {
                    let refusal = json!({"code": "capability_denied"});
                    tool_result(id, refusal, true)
                }
                "t3_thread_launch" => {
                    let thread = "mcp:launch:0f1e";
                    tool_result(
                        id,
                        json!({"threadId": thread,
                        "link": format!("https://app.t3.codes/threads/env/{thread}"),
                        "projectId": arguments["projectId"], "runId": "run-1", "status": "running",
                        "modelSelection": {"model": "m"}}),
                        false,
                    )
                }
                other => panic!("the stand-in does not take {other}"),
            }
        }
        other => panic!("the stand-in does not take {other}"),
    }
}

fn project(id: &str, root: &Path) -> Value {
    json!({"id": id, "title": id, "workspaceRoot": root, "deletedAt": null, "archivedAt": null})
}

struct World {
    dir: TempDir,
    served: PathBuf,
    standin: Arc<StandIn>,
    app: Router,
}

/// A Desk serving one scratch repository, whose `[t3]` names the stand-in
/// at `url`, or the one started here when `url` is `None`, or which has no
/// `[t3]` when `configured` is false.
async fn world(url: Option<&str>, configured: bool) -> World {
    let dir = TempDir::new().unwrap();
    let served = dir.path().join("repos/demo");
    scratch(&served);
    let command = dir.path().join("t3");
    fs::write(&command, T3).unwrap();
    git(dir.path(), &["init", "-q", "unused"]);
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
    tokio::spawn(async move { axum::serve(listener, mcp).await.unwrap() });
    let mut config: Config = toml::from_str("caller = \"sumeet\"\nroots = []\n").unwrap();
    config.roots = vec![dir.path().join("repos")];
    config.t3 = configured.then(|| T3Config {
        command,
        url: url.map_or(format!("http://{address}/mcp"), str::to_string),
        session_message: SESSION_MESSAGE.into(),
    });
    let app = router_for(Desk::new(config));
    World {
        dir,
        served,
        standin,
        app,
    }
}

impl World {
    fn calls(&self) -> String {
        fs::read_to_string(self.dir.path().join("calls")).unwrap_or_default()
    }

    async fn start(&self, mission: &str) -> (StatusCode, Value) {
        let request = Request::post("/api/sessions")
            .header("host", "localhost")
            .header("content-type", "application/json")
            .header("x-atelier-desk", "1")
            .body(Body::from(
                json!({"repo": "demo", "mission": mission}).to_string(),
            ));
        let response = self.app.clone().oneshot(request.unwrap()).await.unwrap();
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await;
        let text = String::from_utf8(body.unwrap().to_vec()).unwrap();
        assert!(!text.contains("secret-"), "a response gave a token: {text}");
        (status, serde_json::from_str(&text).unwrap())
    }
}

#[tokio::test]
async fn a_session_starts_a_thread_in_the_repositorys_project() {
    let world = world(None, true).await;
    let elsewhere = world.dir.path().join("unused");
    // The served folder named through a link is still the served folder.
    let link = world.dir.path().join("link");
    std::os::unix::fs::symlink(&world.served, &link).unwrap();
    *world.standin.projects.lock().unwrap() =
        vec![project("p-other", &elsewhere), project("p-demo", &link)];

    let (status, started) = world.start(MISSION).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        started,
        json!({"result": "started", "thread_id": "mcp:launch:0f1e",
            "link": "https://app.t3.codes/threads/env/mcp:launch:0f1e", "project_id": "p-demo"})
    );

    let calls = world.standin.calls.lock().unwrap().clone();
    let tools: Vec<&str> = calls.iter().map(|(tool, _, _)| tool.as_str()).collect();
    assert_eq!(
        tools,
        ["t3_project_list", "t3_project_list", "t3_thread_launch"]
    );
    assert!(
        calls[0].1.ends_with("-orchestration:read"),
        "projects are read with the read token"
    );
    let (_, token, launch) = &calls[2];
    assert!(token.ends_with("-orchestration:operate"));
    assert_eq!(launch["projectId"], "p-demo");
    assert_eq!(launch["title"], format!("Plan {MISSION}: The plan"));
    let message = SESSION_MESSAGE
        .replace("{mission}", MISSION)
        .replace("{repo}", "demo");
    assert_eq!(launch["message"], message);
    assert!(launch["message"].as_str().unwrap().contains(MISSION));
    assert!(launch["clientRequestId"].is_string());

    // The operating token was issued for five minutes and revoked after.
    let calls = world.calls();
    let operate: Vec<&str> = calls
        .lines()
        .filter(|l| l.contains("orchestration:operate"))
        .collect();
    assert_eq!(operate.len(), 1, "{calls}");
    assert!(operate[0].contains("--ttl 5m"), "{calls}");
    let issued = calls.lines().filter(|l| l.contains(" issue ")).count();
    assert!(
        calls.lines().last().unwrap() == format!("auth session revoke session-{issued}"),
        "{calls}"
    );
}

#[tokio::test]
async fn with_no_project_for_the_repository_nothing_starts() {
    let world = world(None, true).await;
    let elsewhere = world.dir.path().join("unused");
    *world.standin.projects.lock().unwrap() = vec![project("p-other", &elsewhere)];
    let (status, refused) = world.start(MISSION).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(refused["result"], "refused");
    let text = refused["text"].as_str().unwrap();
    assert!(
        text.contains(&format!(
            "No T3 Code project has {}",
            world.served.display()
        )),
        "{text}"
    );
    let calls = world.standin.calls.lock().unwrap();
    assert!(calls.iter().all(|(tool, _, _)| tool == "t3_project_list"));
    assert!(
        !world.calls().contains("orchestration:operate"),
        "no operating token was asked for"
    );
}

#[tokio::test]
async fn with_t3_code_unreachable_or_unconfigured_nothing_starts() {
    // A port nothing listens on.
    let closed = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/mcp", closed.local_addr().unwrap());
    drop(closed);
    let world = world(Some(&url), true).await;
    let (status, refused) = world.start(MISSION).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(refused["result"], "refused");
    assert!(
        refused["text"]
            .as_str()
            .unwrap()
            .starts_with("T3 Code cannot be reached"),
        "{refused}"
    );
    assert!(!world.calls().contains("orchestration:operate"));

    let world = self::world(None, false).await;
    let (_, refused) = world.start(MISSION).await;
    assert_eq!(refused["result"], "refused");
    assert!(refused["text"]
        .as_str()
        .unwrap()
        .starts_with("T3 Code is not configured"));

    let world = self::world(None, true).await;
    let (status, missing) = world.start("demo-zzzz").await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{missing}");
    assert!(world.standin.calls.lock().unwrap().is_empty());
}
