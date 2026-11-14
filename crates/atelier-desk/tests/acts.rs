//! `POST /api/act`: the human's stop of a proposed plan, carried through
//! by the Desk in a clone of its own, writes the record `atelier drop`
//! writes, through the same function, and publishes it as atelier-2vdq
//! defines a stop: pushed to the plan's branch and its pull request
//! closed. An open mission's stop, which `submit` and `merge` publish, is
//! shown pending until its branch merges, and then stopped. Each test works
//! against a bare remote with a forge, and a `gh` stand-in.

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
use std::sync::Arc;
use tempfile::TempDir;
use tower::ServiceExt;

const CALLER: &str = "sumeet";
/// An agreed mission, open on the base.
const OPEN: &str = "demo-aaaa";
/// A proposed plan, on its own branch.
const PLAN: &str = "demo-pppp";
const REASON: &str = "Not worth it this month";

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

fn mission(root: &Path, id: &str) {
    let text = format!(
        "---\nschema: atelier.mission\nschema_version: 3\nid: {id}\ntitle: Mission {id}\n\
         status: open\nlabels: []\nlead: null\nplanners:\n- p1\n\
         created_at: 2026-11-10T09:00:00Z\n---\n## Why\n\nSumeet asked for it.\n\n\
         ## Outcome\n\nIt works.\n\n## Result\n"
    );
    write(root, &format!(".atelier/missions/{id}/mission.md"), &text);
}

fn identify(root: &Path) {
    git(root, &["config", "user.name", "b"]);
    git(root, &["config", "user.email", "b@b"]);
}

struct World {
    dir: TempDir,
    /// The bare remote, the forge's.
    origin: PathBuf,
    /// The planner's clone, which pushes.
    planner: PathBuf,
    /// The repository the Desk serves as `demo`.
    served: PathBuf,
    /// What the `gh` stand-in says the plan's pull request is.
    pr_state: PathBuf,
    /// Each `gh pr close` the stand-in was given.
    closed: PathBuf,
    desk: Arc<Desk>,
    app: Router,
}

/// A bare remote whose `main` holds OPEN, agreed, and whose branch PLAN
/// proposes mission PLAN; a planner's clone; and the served clone, which
/// has `main` out.
fn world() -> World {
    let dir = TempDir::new().unwrap();
    let origin = dir.path().join("origin.git");
    git(
        dir.path(),
        &["init", "-q", "--bare", "-b", "main", "origin.git"],
    );
    let planner = dir.path().join("planner");
    git(
        dir.path(),
        &["clone", "-q", origin.to_str().unwrap(), "planner"],
    );
    identify(&planner);
    git(&planner, &["switch", "-qc", "main"]);
    let config = "schema_version = 3\nslug = \"demo\"\nbase_branch = \"main\"\n\n\
                  [forge]\nprovider = \"github\"\nremote = \"origin\"\n";
    write(&planner, ".atelier/config.toml", config);
    mission(&planner, OPEN);
    git(&planner, &["add", "-A"]);
    git(&planner, &["commit", "-qm", "Plan"]);
    git(&planner, &["push", "-q", "origin", "main"]);
    git(&planner, &["switch", "-qc", PLAN]);
    mission(&planner, PLAN);
    git(&planner, &["add", "-A"]);
    git(&planner, &["commit", "-qm", "Plan the plan"]);
    git(&planner, &["push", "-q", "origin", PLAN]);
    git(&planner, &["switch", "-q", "main"]);

    let served = dir.path().join("repos/demo");
    fs::create_dir_all(served.parent().unwrap()).unwrap();
    git(
        dir.path(),
        &["clone", "-q", origin.to_str().unwrap(), "repos/demo"],
    );
    identify(&served);

    let pr_state = dir.path().join("pr-state");
    let closed = dir.path().join("closed");
    fs::write(&pr_state, "OPEN").unwrap();
    let gh = dir.path().join("gh");
    let script = format!(
        "#!/bin/sh\n[ \"$(cat {state})\" = FAIL ] && {{ echo 'gh: the forge did not answer' >&2; exit 1; }}\n\
         [ \"$1 $2 $(cat {state})\" = 'pr list NONE' ] && {{ echo '[]'; exit 0; }}\ncase \"$1 $2\" in\n\
         'pr list') printf '[{{\"number\":7,\"url\":\"https://forge.example/pull/7\",\"state\":\"%s\",\"headRefName\":\"{PLAN}\"}}]' \"$(cat {state})\" ;;\n\
         'pr close') printf '%s\\n' \"$*\" >> {closed}; printf CLOSED > {state} ;;\n\
         *) exit 1 ;;\nesac\n",
        state = pr_state.display(),
        closed = closed.display(),
    );
    fs::write(&gh, script).unwrap();
    chmod(&gh);

    let text = format!("caller = \"{CALLER}\"\nrepos = []\n");
    let mut config: Config = toml::from_str(&text).unwrap();
    config.repos = vec![served.clone()];
    config.state = Some(dir.path().join("state"));
    config.gh = gh;
    let desk = Desk::new(config);
    World {
        app: router_for(desk.clone()),
        desk,
        dir,
        origin,
        planner,
        served,
        pr_state,
        closed,
    }
}

fn chmod(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
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

/// The served repository's state that the Desk must never change: its
/// checkout's `HEAD`, its files, and its local branches.
fn untouched(served: &Path) -> (String, String, String) {
    (
        git(served, &["rev-parse", "HEAD"]),
        git(served, &["status", "--porcelain"]),
        git(
            served,
            &[
                "for-each-ref",
                "--format=%(refname) %(objectname)",
                "refs/heads",
            ],
        ),
    )
}

/// What `atelier drop <id> --reason <reason>` writes, in a fresh clone of
/// `origin` on `branch` as it was at `at`: the command calls the same
/// function.
fn drop_as_the_cli_does(dir: &Path, origin: &Path, branch: &str, at: &str, id: &str) -> String {
    let clone = dir.join("cli");
    git(dir, &["clone", "-q", origin.to_str().unwrap(), "cli"]);
    identify(&clone);
    git(&clone, &["switch", "-qC", branch, at]);
    let skip = Skip::new(None);
    let mut repo = Repo::open(&clone, Thread(CALLER.into()), &skip).unwrap();
    let done = atelier_app::plan::drop_mission(&mut repo, id, REASON)
        .unwrap()
        .unwrap();
    assert_eq!(done.paths, [format!(".atelier/missions/{id}/mission.md")]);
    fs::read_to_string(clone.join(&done.paths[0])).unwrap()
}

#[tokio::test]
async fn a_proposed_plan_is_stopped_from_its_page_and_shown_stopped_once_published() {
    let world = world();
    world.desk.fetch_all();
    let before = untouched(&world.served);
    let uri = format!("/api/repos/demo/missions/{PLAN}");

    let page = get(&world.app, &uri).await;
    assert_eq!(page["mission"]["state"], "proposed", "{page}");
    assert_eq!(page["mission"]["branch"], format!("origin/{PLAN}"));
    assert_eq!(page["pull_request"]["number"], 7);
    assert_eq!(page["can_stop"], true);
    let proposed = get(&world.app, "/api/proposed").await;
    let plans = &proposed["repos"][0]["plans"];
    assert_eq!(plans[0]["mission"]["id"], PLAN, "{proposed}");
    assert_eq!(
        plans[0]["pull_request"]["url"],
        "https://forge.example/pull/7"
    );

    // A reason is needed, on one line.
    let empty = act(
        &world.app,
        json!({"act": "stop", "repo": "demo", "id": PLAN, "reason": " "}),
    )
    .await;
    assert_eq!(empty["result"], "refused", "{empty}");

    let done = act(
        &world.app,
        json!({"act": "stop", "repo": "demo", "id": PLAN, "reason": REASON}),
    )
    .await;
    assert_eq!(done["result"], "done", "{done}");
    assert_eq!(
        done["paths"],
        json!([format!(".atelier/missions/{PLAN}/mission.md")])
    );
    let pushed = git(&world.origin, &["rev-parse", &format!("refs/heads/{PLAN}")]);
    assert_eq!(
        done["commit"], pushed,
        "the drop is pushed to the plan's branch"
    );
    let closed = fs::read_to_string(&world.closed).unwrap();
    assert_eq!(
        closed.trim(),
        format!("pr close {PLAN} --comment Stopped from the Desk by {CALLER}: {REASON}")
    );
    assert_eq!(
        untouched(&world.served),
        before,
        "the served repository's checkout is not written"
    );

    // The record is the one `atelier drop` writes.
    let path = format!("{PLAN}:.atelier/missions/{PLAN}/mission.md");
    let stopped = git(&world.origin, &["show", &path]);
    let cli = drop_as_the_cli_does(
        world.dir.path(),
        &world.origin,
        PLAN,
        &format!("origin/{PLAN}~1"),
        PLAN,
    );
    assert_eq!(format!("{stopped}\n"), cli);
    assert!(
        stopped.contains(&format!("## Result\n\nDropped: {REASON}")),
        "{stopped}"
    );

    // Pushed and the pull request closed: stopped, and no longer proposed.
    let page = get(&world.app, &uri).await;
    assert_eq!(page["mission"]["state"], "stopped", "{page}");
    assert_eq!(page["dropped"], REASON);
    let proposed = get(&world.app, "/api/proposed").await;
    assert_eq!(proposed["repos"][0]["plans"], json!([]), "{proposed}");
    // While its pull request is open, the stop is not published.
    fs::write(&world.pr_state, "OPEN").unwrap();
    world.desk.fetch_all();
    let page = get(&world.app, &uri).await;
    assert_eq!(page["mission"]["state"], "stop_pending", "{page}");
    // A second stop is refused: it is stopped already.
    let again = act(
        &world.app,
        json!({"act": "stop", "repo": "demo", "id": PLAN, "reason": REASON}),
    )
    .await;
    assert_eq!(again["result"], "refused", "{again}");
}

#[tokio::test]
async fn an_open_missions_stop_is_pending_until_its_branch_merges_and_the_desk_does_not_make_it() {
    let world = world();
    world.desk.fetch_all();
    let uri = format!("/api/repos/demo/missions/{OPEN}");
    let page = get(&world.app, &uri).await;
    assert_eq!(page["mission"]["state"], "open", "{page}");
    assert_eq!(page["can_stop"], false);
    assert_eq!(
        page["stop_commands"][1],
        format!("atelier drop {OPEN} --reason \"<why>\"")
    );
    let refused = act(
        &world.app,
        json!({"act": "stop", "repo": "demo", "id": OPEN, "reason": REASON}),
    )
    .await;
    assert_eq!(refused["result"], "refused", "{refused}");

    // Someone writes the stop on a backlog branch and pushes it.
    let branch = format!("backlog/stop-{OPEN}");
    git(&world.planner, &["switch", "-qc", &branch, "main"]);
    let skip = Skip::new(None);
    let mut repo = Repo::open(&world.planner, Thread("p1".into()), &skip).unwrap();
    atelier_app::plan::drop_mission(&mut repo, OPEN, REASON)
        .unwrap()
        .unwrap();
    git(&world.planner, &["commit", "-qam", &format!("Drop {OPEN}")]);
    git(&world.planner, &["push", "-q", "origin", &branch]);
    world.desk.fetch_all();
    let page = get(&world.app, &uri).await;
    assert_eq!(page["mission"]["state"], "stop_pending", "{page}");
    assert_eq!(page["stop_on"], format!("origin/{branch}"));
    let proposed = get(&world.app, "/api/proposed").await;
    let changes = &proposed["repos"][0]["changes"];
    assert_eq!(
        changes[0]["branch"],
        format!("origin/{branch}"),
        "{proposed}"
    );
    assert_eq!(changes[0]["records"][0]["change"], "dropped");

    // It merges.
    git(&world.planner, &["switch", "-q", "main"]);
    git(
        &world.planner,
        &["merge", "-q", "--no-ff", "-m", "Merge the stop", &branch],
    );
    git(&world.planner, &["push", "-q", "origin", "main"]);
    world.desk.fetch_all();
    let page = get(&world.app, &uri).await;
    assert_eq!(page["mission"]["state"], "stopped", "{page}");
}

#[tokio::test]
async fn only_the_desks_own_app_may_act() {
    let world = world();
    let before = untouched(&world.served);
    let stop = json!({"act": "stop", "repo": "demo", "id": PLAN, "reason": REASON});
    let session = json!({"repo": "demo", "mission": PLAN});
    for uri in ["/api/act", "/api/sessions"] {
        let body = if uri == "/api/act" { &stop } else { &session };
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
    assert_eq!(
        untouched(&world.served),
        before,
        "a refused act writes nothing"
    );
    assert!(!world.closed.exists(), "a refused act closes nothing");

    // A request the app could not have meant.
    for (body, status) in [
        (
            json!({"act": "ready", "repo": "demo", "ids": [PLAN]}),
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            json!({"act": "stop", "repo": "demo", "id": PLAN, "reason": "r", "by": "x"}),
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            json!({"act": "stop", "repo": "demo", "id": "../x", "reason": "r"}),
            StatusCode::BAD_REQUEST,
        ),
        (
            json!({"act": "stop", "repo": "nowhere", "id": PLAN, "reason": "r"}),
            StatusCode::NOT_FOUND,
        ),
    ] {
        let (got, answer) = send(&world.app, "POST", "/api/act", FROM_APP, body.clone()).await;
        assert_eq!(got, status, "{body}: {answer}");
        assert!(answer["error"].is_string(), "{body}: {answer}");
    }
}

/// The page and both lists of missions, for `id`: its state on its page,
/// and whether the unfinished list and the finished list hold it.
async fn shown(app: &Router, id: &str) -> (Value, bool, bool) {
    let page = get(app, &format!("/api/repos/demo/missions/{id}")).await;
    let has = |list: &Value| (list["missions"].as_array().unwrap().iter()).any(|m| m["id"] == id);
    let open = has(&get(app, "/api/repos/demo/missions").await);
    let done = has(&get(app, "/api/repos/demo/missions?done=true").await);
    (page["mission"]["state"].clone(), open, done)
}

#[tokio::test]
async fn a_plan_merged_since_the_last_fetch_is_not_stopped_as_a_plan() {
    let world = world();
    world.desk.fetch_all();
    // The plan merges on the remote after the Desk's fetch.
    git(
        &world.planner,
        &["merge", "-q", "--no-ff", "-m", "Merge the plan", PLAN],
    );
    git(&world.planner, &["push", "-q", "origin", "main"]);
    let before = git(&world.origin, &["rev-parse", &format!("refs/heads/{PLAN}")]);
    let refused = act(
        &world.app,
        json!({"act": "stop", "repo": "demo", "id": PLAN, "reason": REASON}),
    )
    .await;
    assert_eq!(refused["result"], "refused", "{refused}");
    assert!(refused["text"]
        .as_str()
        .unwrap()
        .contains("merged since the Desk last fetched"));
    let after = git(&world.origin, &["rev-parse", &format!("refs/heads/{PLAN}")]);
    assert_eq!(before, after, "nothing is pushed");
    assert!(!world.closed.exists(), "no pull request is closed");
    assert_eq!(shown(&world.app, PLAN).await, (json!("open"), true, false));
}

#[tokio::test]
async fn an_agreed_missions_drop_on_its_own_branch_is_pending_in_both_lists_until_it_merges() {
    let world = world();
    git(&world.planner, &["switch", "-qc", OPEN, "main"]);
    let skip = Skip::new(None);
    let mut repo = Repo::open(&world.planner, Thread("p1".into()), &skip).unwrap();
    atelier_app::plan::drop_mission(&mut repo, OPEN, REASON)
        .unwrap()
        .unwrap();
    git(&world.planner, &["commit", "-qam", &format!("Drop {OPEN}")]);
    git(&world.planner, &["push", "-q", "origin", OPEN]);
    world.desk.fetch_all();
    let page = get(&world.app, &format!("/api/repos/demo/missions/{OPEN}")).await;
    assert_eq!(page["stop_on"], format!("origin/{OPEN}"), "{page}");
    assert_eq!(
        shown(&world.app, OPEN).await,
        (json!("stop_pending"), true, false)
    );
    // The drop is a proposed change of its branch until it merges.
    let dropping = |proposed: &Value| {
        let changes = proposed["repos"][0]["changes"].as_array().unwrap().clone();
        changes
            .into_iter()
            .find(|c| c["branch"] == format!("origin/{OPEN}"))
    };
    let proposed = get(&world.app, "/api/proposed").await;
    let change = dropping(&proposed).expect("the drop is proposed");
    assert_eq!(change["records"][0]["id"], OPEN, "{proposed}");
    assert_eq!(change["records"][0]["change"], "dropped");
    git(&world.planner, &["switch", "-q", "main"]);
    git(
        &world.planner,
        &["merge", "-q", "--no-ff", "-m", "Merge the stop", OPEN],
    );
    git(&world.planner, &["push", "-q", "origin", "main"]);
    world.desk.fetch_all();
    assert_eq!(
        shown(&world.app, OPEN).await,
        (json!("stopped"), false, true)
    );
    assert!(dropping(&get(&world.app, "/api/proposed").await).is_none());
}

#[tokio::test]
async fn a_stop_whose_pull_request_cannot_be_read_stays_pending_and_says_why() {
    let world = world();
    // The forge does not answer from the start.
    fs::write(&world.pr_state, "FAIL").unwrap();
    world.desk.fetch_all();
    let done = act(
        &world.app,
        json!({"act": "stop", "repo": "demo", "id": PLAN, "reason": REASON}),
    )
    .await;
    assert_eq!(done["result"], "done", "{done}");
    assert!(
        done["text"]
            .as_str()
            .unwrap()
            .contains("its pull request was not closed"),
        "{done}"
    );
    let page = get(&world.app, &format!("/api/repos/demo/missions/{PLAN}")).await;
    assert!(
        page["fetched"]["error"]
            .as_str()
            .unwrap()
            .contains("the forge did not answer"),
        "{page}"
    );
    assert_eq!(
        shown(&world.app, PLAN).await,
        (json!("stop_pending"), true, false)
    );
    // The forge answers: the pull request was open, and is closed now.
    fs::write(&world.pr_state, "CLOSED").unwrap();
    world.desk.fetch_all();
    assert_eq!(
        shown(&world.app, PLAN).await,
        (json!("stopped"), false, true)
    );
    // A failure after a list was read keeps that list, and says so.
    fs::write(&world.pr_state, "FAIL").unwrap();
    world.desk.fetch_all();
    assert_eq!(
        shown(&world.app, PLAN).await,
        (json!("stopped"), false, true)
    );
    let page = get(&world.app, &format!("/api/repos/demo/missions/{PLAN}")).await;
    assert!(page["fetched"]["error"].is_string(), "{page}");
}

#[tokio::test]
async fn an_agreed_missions_amendment_on_its_own_branch_is_proposed_with_its_other_records() {
    let world = world();
    git(&world.planner, &["switch", "-qc", OPEN, "main"]);
    let path = world
        .planner
        .join(format!(".atelier/missions/{OPEN}/mission.md"));
    let text = fs::read_to_string(&path)
        .unwrap()
        .replace("Sumeet asked for it.", "Sumeet asked for more.");
    fs::write(&path, text).unwrap();
    let item = format!(
        "---\nschema: atelier.item\nschema_version: 3\nid: demo-i001\ntitle: A new item\n\
         status: draft\nmission: {OPEN}\nlabels: []\npriority: medium\nblocked_by: []\n\
         implementers: []\nready: null\ntrivial: null\ncaused_by: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Goal\n\nMore.\n\n## Done when\n\n## Scope\n"
    );
    fs::create_dir_all(world.planner.join(".atelier/items/demo-i001")).unwrap();
    fs::write(world.planner.join(".atelier/items/demo-i001/item.md"), item).unwrap();
    git(&world.planner, &["add", "-A"]);
    git(&world.planner, &["commit", "-qm", "Amend the plan"]);
    git(&world.planner, &["push", "-q", "origin", OPEN]);
    world.desk.fetch_all();
    let proposed = get(&world.app, "/api/proposed").await;
    let repo = &proposed["repos"][0];
    let plans = repo["plans"].as_array().unwrap();
    let amended = plans
        .iter()
        .find(|p| p["mission"]["id"] == OPEN)
        .expect("the amendment is listed");
    assert_eq!(amended["amends"], true, "{repo}");
    assert_eq!(amended["mission"]["branch"], format!("origin/{OPEN}"));
    let changes = repo["changes"].as_array().unwrap();
    let on = changes
        .iter()
        .find(|c| c["branch"] == format!("origin/{OPEN}"))
        .expect("its other records are changes");
    assert_eq!(on["records"][0]["id"], "demo-i001", "{repo}");
    assert_eq!(on["records"][0]["change"], "added");
}

#[tokio::test]
async fn a_stop_after_a_list_without_its_pull_request_stays_pending_while_the_forge_fails() {
    let world = world();
    // The last good list has no pull request for the plan's branch.
    fs::write(&world.pr_state, "NONE").unwrap();
    world.desk.fetch_all();
    // Then the forge stops answering, and the stop cannot close one.
    fs::write(&world.pr_state, "FAIL").unwrap();
    let done = act(
        &world.app,
        json!({"act": "stop", "repo": "demo", "id": PLAN, "reason": REASON}),
    )
    .await;
    assert!(
        done["text"]
            .as_str()
            .unwrap()
            .contains("its pull request was not closed"),
        "{done}"
    );
    assert_eq!(
        shown(&world.app, PLAN).await,
        (json!("stop_pending"), true, false)
    );
    // A list read now that has none confirms it.
    fs::write(&world.pr_state, "NONE").unwrap();
    world.desk.fetch_all();
    assert_eq!(
        shown(&world.app, PLAN).await,
        (json!("stopped"), false, true)
    );
}

#[tokio::test]
async fn a_stopped_plans_items_are_not_proposed() {
    let world = world();
    git(&world.planner, &["switch", "-q", PLAN]);
    let item = format!(
        "---\nschema: atelier.item\nschema_version: 3\nid: demo-i002\ntitle: The plan's item\n\
         status: draft\nmission: {PLAN}\nlabels: []\npriority: medium\nblocked_by: []\n\
         implementers: []\nready: null\ntrivial: null\ncaused_by: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Goal\n\nIt.\n\n## Done when\n\n## Scope\n"
    );
    fs::create_dir_all(world.planner.join(".atelier/items/demo-i002")).unwrap();
    fs::write(world.planner.join(".atelier/items/demo-i002/item.md"), item).unwrap();
    git(&world.planner, &["add", "-A"]);
    git(&world.planner, &["commit", "-qm", "Plan an item"]);
    git(&world.planner, &["push", "-q", "origin", PLAN]);
    world.desk.fetch_all();
    let before = get(&world.app, "/api/proposed").await;
    assert_eq!(
        before["repos"][0]["plans"][0]["mission"]["id"], PLAN,
        "{before}"
    );
    assert_eq!(
        before["repos"][0]["changes"],
        json!([]),
        "a plan's items are its plan"
    );
    let done = act(
        &world.app,
        json!({"act": "stop", "repo": "demo", "id": PLAN, "reason": REASON}),
    )
    .await;
    assert_eq!(done["result"], "done", "{done}");
    let after = get(&world.app, "/api/proposed").await;
    assert_eq!(after["repos"][0]["plans"], json!([]), "{after}");
    assert_eq!(after["repos"][0]["changes"], json!([]), "{after}");
}

#[tokio::test]
async fn a_pull_request_closed_before_the_stop_does_not_confirm_it() {
    let world = world();
    // The last list showed the plan's pull request closed; it is reopened
    // since, and the forge stops answering before the stop.
    fs::write(&world.pr_state, "CLOSED").unwrap();
    world.desk.fetch_all();
    fs::write(&world.pr_state, "FAIL").unwrap();
    let done = act(
        &world.app,
        json!({"act": "stop", "repo": "demo", "id": PLAN, "reason": REASON}),
    )
    .await;
    assert!(
        done["text"]
            .as_str()
            .unwrap()
            .contains("its pull request was not closed"),
        "{done}"
    );
    assert_eq!(
        shown(&world.app, PLAN).await,
        (json!("stop_pending"), true, false)
    );
    // A list read after the stop confirms it.
    fs::write(&world.pr_state, "CLOSED").unwrap();
    world.desk.fetch_all();
    assert_eq!(
        shown(&world.app, PLAN).await,
        (json!("stopped"), false, true)
    );
}

#[tokio::test]
async fn an_amendment_and_a_stop_on_another_branch_are_each_shown_as_they_are() {
    let world = world();
    // An amendment of the agreed mission on its own branch.
    git(&world.planner, &["switch", "-qc", OPEN, "main"]);
    let path = world
        .planner
        .join(format!(".atelier/missions/{OPEN}/mission.md"));
    let text = fs::read_to_string(&path)
        .unwrap()
        .replace("Sumeet asked for it.", "Sumeet asked for more.");
    fs::write(&path, text).unwrap();
    git(&world.planner, &["commit", "-qam", "Amend the plan"]);
    git(&world.planner, &["push", "-q", "origin", OPEN]);
    // Its stop, on a backlog branch.
    let stop = format!("backlog/stop-{OPEN}");
    git(&world.planner, &["switch", "-qc", &stop, "main"]);
    let skip = Skip::new(None);
    let mut repo = Repo::open(&world.planner, Thread("p1".into()), &skip).unwrap();
    atelier_app::plan::drop_mission(&mut repo, OPEN, REASON)
        .unwrap()
        .unwrap();
    git(&world.planner, &["commit", "-qam", &format!("Drop {OPEN}")]);
    git(&world.planner, &["push", "-q", "origin", &stop]);
    world.desk.fetch_all();
    let page = get(&world.app, &format!("/api/repos/demo/missions/{OPEN}")).await;
    assert_eq!(page["stop_on"], format!("origin/{stop}"), "{page}");
    let proposed = get(&world.app, "/api/proposed").await;
    let repo = &proposed["repos"][0];
    let amended = (repo["plans"].as_array().unwrap().iter())
        .find(|p| p["mission"]["id"] == OPEN)
        .expect("the amendment is listed");
    assert_eq!(amended["amends"], true, "{repo}");
    let drops: Vec<&Value> = (repo["changes"].as_array().unwrap().iter())
        .filter(|c| {
            (c["records"].as_array().unwrap().iter())
                .any(|r| r["id"] == OPEN && r["change"] == "dropped")
        })
        .collect();
    assert_eq!(drops.len(), 1, "{repo}");
    assert_eq!(drops[0]["branch"], format!("origin/{stop}"));
}
