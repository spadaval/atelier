//! Each route against the fixture repositories: the checked-in fixture JSON
//! the app is built on, agreement with `atelier brief -q` and
//! `atelier debrief -q`, the review a mission opens on, errors and the
//! cache.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use super::fixture::{self, Repo};
use super::{client, get_json, scratch};

/// The fixture JSON beside the app: `crates/atelier-site/web/src/fixtures`.
fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../atelier-site/web/src/fixtures")
}

/// Whether to rewrite checked-in output instead of comparing with it.
fn updating() -> bool {
    std::env::var_os("ATELIER_UPDATE_FIXTURES").is_some_and(|value| value == "1")
}

/// `alpha` and `beta` under one root.
pub(super) fn fleet(root: &Path) -> (Repo, Repo) {
    let (alpha, _) = fixture::alpha(&root.join("alpha"));
    let beta = fixture::beta(&root.join("beta"));
    (alpha, beta)
}

/// Every route of the fixture fleet with the file its response is kept in.
pub(super) fn routes() -> Vec<(String, String)> {
    let mut routes = vec![
        ("/api/repos".to_owned(), "repos.json".to_owned()),
        ("/api/inbox".to_owned(), "inbox.json".to_owned()),
    ];
    for (repo, missions) in [
        ("alpha", &["alpha-m1", "alpha-m2"]),
        ("beta", &["beta-m1", "beta-m2"]),
    ] {
        for mission in missions {
            let base = format!("/api/{repo}/mission/{mission}");
            routes.push((base.clone(), format!("{repo}/{mission}.json")));
            for view in ["brief", "debrief", "strategy"] {
                routes.push((
                    format!("{base}/{view}"),
                    format!("{repo}/{mission}.{view}.json"),
                ));
            }
        }
    }
    for repo in ["alpha", "beta"] {
        routes.push((format!("/api/{repo}/board"), format!("{repo}/board.json")));
    }
    // A task, a decision, a loose end, a needs-human loose end and a done
    // task with an approval.
    for item in ["alpha-a1", "alpha-a2", "alpha-l1", "alpha-l2", "alpha-b1"] {
        routes.push((
            format!("/api/alpha/item/{item}"),
            format!("alpha/item/{item}.json"),
        ));
    }
    routes.push((
        "/api/alpha/item/alpha-zzzz".to_owned(),
        "errors/unknown-item.json".to_owned(),
    ));
    routes.push((
        "/api/nowhere/mission/x".to_owned(),
        "errors/unknown-repo.json".to_owned(),
    ));
    routes.push((
        "/api/alpha/mission/alpha-zzzz".to_owned(),
        "errors/unknown-mission.json".to_owned(),
    ));
    routes
}

/// Writes the fixture fleet under `$ATELIER_FIXTURE_FLEET`, for the web
/// app's Playwright suite (`crates/atelier-site/web/e2e/serve.sh`) to serve.
#[test]
#[ignore = "run by the Playwright suite to build its fixture repositories"]
fn write_fixture_fleet() {
    let root = std::env::var_os("ATELIER_FIXTURE_FLEET")
        .expect("ATELIER_FIXTURE_FLEET names an empty directory to write the fleet into");
    fleet(Path::new(&root));
}

#[test]
fn fixture_responses_match_the_checked_in_json() {
    let root = scratch();
    fleet(root.path());
    let client = client(root.path());
    let dir = fixtures_dir();
    let mut stale = Vec::new();
    let mut expected = BTreeSet::new();
    for (route, file) in routes() {
        let (status, body) = client.get(&route);
        let value: Value = serde_json::from_str(&body).unwrap();
        let text = format!(
            "{}\n",
            serde_json::to_string_pretty(&serde_json::json!({
                "route": route,
                "status": status,
                "body": value,
            }))
            .unwrap()
        );
        let path = dir.join(&file);
        expected.insert(PathBuf::from(&file));
        if updating() {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, &text).unwrap();
        } else if fs::read_to_string(&path).ok().as_deref() != Some(text.as_str()) {
            stale.push(file);
        }
    }
    let mut extra = Vec::new();
    collect(&dir, &dir, &mut extra);
    extra.retain(|path| !expected.contains(path));
    assert!(
        stale.is_empty() && extra.is_empty(),
        "fixture JSON under {} is stale: {stale:?} differ, {extra:?} are not written by any route. Rewrite it with ATELIER_UPDATE_FIXTURES=1 cargo nextest run --test serve fixture_responses",
        dir.display()
    );
}

fn collect(root: &Path, dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(root, &path, files);
        } else {
            files.push(path.strip_prefix(root).unwrap().to_path_buf());
        }
    }
}

/// `key=value` pairs of a `-q` line.
fn counts(line: &str) -> BTreeMap<String, usize> {
    line.split_whitespace()
        .filter_map(|pair| pair.split_once('='))
        .map(|(key, value)| (key.to_owned(), value.parse().unwrap()))
        .collect()
}

fn atelier(dir: &Path, args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir)
        .args(args)
        .env("ATELIER_NO_UPDATE_CHECK", "1")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "atelier {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn length(value: &Value, pointer: &str) -> usize {
    value
        .pointer(pointer)
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("{pointer} is not an array"))
        .len()
}

/// The counts `atelier brief -q` prints, read from the brief response.
fn brief_counts(brief: &Value) -> BTreeMap<String, usize> {
    let items = brief["items"].as_array().unwrap();
    BTreeMap::from([
        ("criteria".into(), length(brief, "/criteria/parse/criteria")),
        ("items".into(), items.len()),
        (
            "blocked".into(),
            items
                .iter()
                .filter(|item| !item["open_blockers"].as_array().unwrap().is_empty())
                .count(),
        ),
        (
            "unknowns".into(),
            items
                .iter()
                .filter(|item| !item["unknowns"].is_null())
                .count(),
        ),
        ("human_decisions".into(), length(brief, "/human_decisions")),
        (
            "approvals".into(),
            brief["mission_events"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|event| event["event"] == "approval")
                .count(),
        ),
    ])
}

/// The criteria and item counts `atelier debrief -q` prints, read from the
/// debrief response.
fn debrief_counts(debrief: &Value) -> BTreeMap<String, usize> {
    let marks = |mark: &str| {
        debrief["criteria"]["proofs"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|proof| proof["mark"] == mark)
            .count()
    };
    let mut items = BTreeSet::new();
    for entry in debrief["work"].as_array().unwrap() {
        items.insert(entry["id"].as_str().unwrap().to_owned());
        for child in entry["children"].as_array().unwrap() {
            items.insert(child["id"].as_str().unwrap().to_owned());
        }
    }
    for list in ["left", "found", "dropped"] {
        for entry in debrief[list].as_array().unwrap() {
            items.insert(entry["id"].as_str().unwrap().to_owned());
        }
    }
    for ask in debrief["asks"].as_array().unwrap() {
        if matches!(ask["kind"].as_str(), Some("decision" | "needs_human")) {
            items.insert(ask["value"].as_str().unwrap().to_owned());
        }
    }
    BTreeMap::from([
        ("criteria".into(), length(debrief, "/criteria/proofs")),
        ("proven".into(), marks("proven")),
        ("failing".into(), marks("failing")),
        ("unproven".into(), marks("unproven")),
        ("items".into(), items.len()),
        (
            "decisions".into(),
            length(debrief, "/decisions/notes") + length(debrief, "/decisions/records"),
        ),
        ("follow_ups".into(), length(debrief, "/found")),
    ])
}

#[test]
fn briefs_and_debriefs_match_what_the_cli_prints() {
    let root = scratch();
    let (alpha, beta) = fleet(root.path());
    let client = client(root.path());
    let mut compared = 0;
    for (repo, dir, missions) in [
        ("alpha", &alpha.dir, ["alpha-m1", "alpha-m2"]),
        ("beta", &beta.dir, ["beta-m1", "beta-m2"]),
    ] {
        for mission in missions {
            let brief = get_json(
                &client,
                &format!("/api/{repo}/mission/{mission}/brief"),
                200,
            );
            let brief = &brief["brief"];
            let title = brief["mission"]["title"].as_str().unwrap();
            assert!(
                atelier(dir, &["brief", mission]).contains(title),
                "atelier brief {mission} names {title}"
            );
            let printed = counts(&atelier(dir, &["brief", mission, "-q"]));
            assert_eq!(brief_counts(brief), printed, "brief of {mission}");

            let debrief = get_json(
                &client,
                &format!("/api/{repo}/mission/{mission}/debrief"),
                200,
            );
            let debrief = &debrief["debrief"];
            assert_eq!(debrief["mission"]["title"], title);
            assert!(
                atelier(dir, &["debrief", mission]).contains(title),
                "atelier debrief {mission} names {title}"
            );
            let mut printed = counts(&atelier(dir, &["debrief", mission, "-q"]));
            printed.remove("deviations");
            assert_eq!(debrief_counts(debrief), printed, "debrief of {mission}");
            compared += 1;
        }
    }
    assert_eq!(compared, 4);
    // The fixture exercises what the counts count.
    let done = get_json(&client, "/api/alpha/mission/alpha-m2/debrief", 200);
    assert_eq!(debrief_counts(&done["debrief"])["proven"], 1);
    let draft = get_json(&client, "/api/alpha/mission/alpha-m1/brief", 200);
    assert_eq!(brief_counts(&draft["brief"])["criteria"], 2);
    assert_eq!(brief_counts(&draft["brief"])["unknowns"], 1);
    assert_eq!(brief_counts(&draft["brief"])["human_decisions"], 1);
}

#[test]
fn a_mission_opens_on_the_brief_only_in_its_workflows_initial_status() {
    let root = scratch();
    fleet(root.path());
    let client = client(root.path());
    let repos = get_json(&client, "/api/repos", 200);
    let reviews = repos["repos"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|repo| repo["missions"].as_array().unwrap())
        .map(|mission| {
            (
                mission["id"].as_str().unwrap().to_owned(),
                (
                    mission["status"].as_str().unwrap().to_owned(),
                    mission["status_category"].as_str().unwrap().to_owned(),
                    mission["review"].as_str().unwrap().to_owned(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let expect = |status: &str, category: &str, review: &str| {
        (status.to_owned(), category.to_owned(), review.to_owned())
    };
    assert_eq!(
        reviews,
        BTreeMap::from([
            ("alpha-m1".to_owned(), expect("draft", "todo", "brief")),
            ("alpha-m2".to_owned(), expect("done", "done", "debrief")),
            // `ready` shares `proposed`'s category, but is past the initial
            // status, so its debrief is shown.
            ("beta-m1".to_owned(), expect("proposed", "todo", "brief")),
            ("beta-m2".to_owned(), expect("ready", "todo", "debrief")),
            (
                "beta-m3".to_owned(),
                expect("superseded", "done", "debrief")
            ),
        ])
    );
    let one = get_json(&client, "/api/beta/mission/beta-m2", 200);
    assert_eq!(one["mission"]["review"], "debrief");
    assert_eq!(one["repo"], "beta");
    assert_eq!(
        one["mission"]["links"]["debrief"],
        "/api/beta/mission/beta-m2/debrief"
    );
}

#[test]
fn unknown_repositories_missions_and_routes_are_json_404s() {
    let root = scratch();
    fleet(root.path());
    let client = client(root.path());
    for (path, says) in [
        (
            "/api/nowhere/mission/alpha-m1",
            "no Atelier repository named nowhere",
        ),
        (
            "/api/alpha/mission/alpha-zzzz/brief",
            "no mission alpha-zzzz in alpha",
        ),
        (
            "/api/alpha/mission/alpha-a1/debrief",
            "alpha-a1 is a task, not a mission",
        ),
        (
            "/api/alpha/mission/alpha-m2/strategy",
            "alpha-m2 links no strategy",
        ),
        ("/api/alpha", "no such route"),
        ("/api/no/such/route", "no such route"),
    ] {
        let error = get_json(&client, path, 404);
        let text = error["error"].as_str().unwrap();
        assert!(text.contains(says), "GET {path}: {text}");
    }
    // A short id resolves as the CLI resolves it.
    let short = get_json(&client, "/api/alpha/mission/m1", 200);
    assert_eq!(short["mission"]["id"], "alpha-m1");
}

/// Every path outside `/api/` belongs to the app (atelier-yn69): a page
/// path gets its `index.html`, built or not, and a missing asset is a 404.
#[test]
fn paths_outside_the_api_get_the_app_and_a_missing_asset_is_a_404() {
    let root = scratch();
    fleet(root.path());
    let client = client(root.path());
    for path in [
        "/",
        "/alpha/mission/alpha-m1",
        "/alpha/mission/alpha-m1/debrief",
    ] {
        let (status, body) = client.get(path);
        assert_eq!(status, 200, "GET {path}");
        assert!(
            body.to_ascii_lowercase().starts_with("<!doctype html>"),
            "GET {path}: {body}"
        );
    }
    let (status, body) = client.get("/assets/index-missing.js");
    assert_eq!(status, 404, "{body}");
    assert!(
        !body.to_ascii_lowercase().contains("<!doctype html>"),
        "{body}"
    );
}

#[test]
fn a_repository_whose_records_fail_to_load_answers_with_its_error_while_others_answer() {
    let root = scratch();
    let (alpha, _) = fleet(root.path());
    alpha.write(".atelier/issues/alpha-bad.md", "not a record\n");
    alpha.commit("Break a record");
    let client = client(root.path());
    let repos = get_json(&client, "/api/repos", 200);
    let by_name = |name: &str| {
        repos["repos"]
            .as_array()
            .unwrap()
            .iter()
            .find(|repo| repo["repo"] == name)
            .unwrap()
            .clone()
    };
    let broken = by_name("alpha");
    let error = broken["error"].as_str().unwrap();
    assert!(error.contains("alpha-bad.md"), "{error}");
    assert!(broken["missions"].as_array().unwrap().is_empty());
    assert!(by_name("beta")["error"].is_null());
    let failed = get_json(&client, "/api/alpha/mission/alpha-m1/brief", 500);
    assert!(failed["error"].as_str().unwrap().contains("alpha-bad.md"));
    get_json(&client, "/api/beta/mission/beta-m1/brief", 200);
}

#[test]
fn a_view_is_built_once_per_base_commit() {
    let root = scratch();
    let (alpha, _) = fleet(root.path());
    let client = client(root.path());
    let path = "/api/alpha/mission/alpha-m2/debrief";
    let first = get_json(&client, path, 200);
    let built = client.site().builds_run();
    assert_eq!(get_json(&client, path, 200), first);
    assert_eq!(
        client.site().builds_run(),
        built,
        "a warm request builds nothing"
    );
    // Twenty requests at once for a view not yet built build it at most
    // twice: one per build slot, each checking the cache once it has one.
    let paths = vec!["/api/alpha/mission/alpha-m1/debrief".to_owned(); 20];
    for (status, _) in client.get_all(&paths) {
        assert_eq!(status, 200);
    }
    assert!(client.site().builds_run() <= built + 2);
    // A new base commit is read and built afresh.
    let built = client.site().builds_run();
    alpha.write("src/more.txt", "more\n");
    let moved = alpha.commit("Move the base");
    let after = get_json(&client, path, 200);
    assert_eq!(after["base_commit"], moved);
    assert_ne!(after["base_commit"], first["base_commit"]);
    assert!(client.site().builds_run() > built);
}

/// Makes every loose-object directory of `objects` unreadable, or readable
/// again, so Git still opens the repository and reads its refs but cannot
/// read a commit.
fn readable(objects: &Path, yes: bool) {
    let mode = if yes { "u+rwx" } else { "000" };
    for entry in fs::read_dir(objects).unwrap().flatten() {
        let name = entry.file_name();
        if name.len() == 2 {
            let status = Command::new("chmod")
                .arg(mode)
                .arg(entry.path())
                .status()
                .unwrap();
            assert!(status.success());
        }
    }
}

#[test]
fn a_failure_reading_git_is_not_kept_once_git_reads_again() {
    let root = scratch();
    let (alpha, _) = fleet(root.path());
    let objects = alpha.dir.join(".git/objects");
    let client = client(root.path());
    // The records are loaded; then a view is built while Git cannot read
    // objects.
    get_json(&client, "/api/alpha/mission/alpha-m1", 200);
    readable(&objects, false);
    let (status, body) = client.get("/api/alpha/mission/alpha-m2/debrief");
    readable(&objects, true);
    assert_eq!(status, 500, "{body}");
    get_json(&client, "/api/alpha/mission/alpha-m2/debrief", 200);

    // A server that first meets the failure reading the base's policy.
    let fresh = super::client(root.path());
    readable(&objects, false);
    let repos = get_json(&fresh, "/api/repos", 200);
    let (status, body) = fresh.get("/api/alpha/mission/alpha-m1/brief");
    readable(&objects, true);
    let alpha_entry = repos["repos"]
        .as_array()
        .unwrap()
        .iter()
        .find(|repo| repo["base_branch"].is_null() || repo["repo"] == "alpha")
        .unwrap()
        .clone();
    assert!(alpha_entry["error"].is_string(), "{alpha_entry}");
    assert_ne!(status, 200, "{body}");
    get_json(&fresh, "/api/alpha/mission/alpha-m1/brief", 200);
    let repos = get_json(&fresh, "/api/repos", 200);
    assert!(repos["repos"][0]["error"].is_null(), "{repos}");
}

/// Makes the loose object holding `path` at the base unreadable, or readable
/// again.
fn object_readable(repo: &Repo, path: &str, yes: bool) {
    let oid = repo.git(&["rev-parse", &format!("main:{path}")]);
    let file = repo
        .dir
        .join(".git/objects")
        .join(&oid[..2])
        .join(&oid[2..]);
    let mode = if yes { "u+r" } else { "000" };
    let status = Command::new("chmod").arg(mode).arg(file).status().unwrap();
    assert!(status.success());
}

#[test]
fn a_strategy_git_cannot_read_for_a_moment_is_never_served_or_kept_as_missing() {
    let root = scratch();
    let (alpha, _) = fleet(root.path());
    let client = client(root.path());
    get_json(&client, "/api/alpha/mission/alpha-m1", 200);
    object_readable(&alpha, "docs/strategies/one.md", false);
    let answers = ["brief", "debrief", "strategy"]
        .map(|view| client.get(&format!("/api/alpha/mission/alpha-m1/{view}")));
    object_readable(&alpha, "docs/strategies/one.md", true);
    for (status, body) in &answers {
        assert_eq!(*status, 500, "{body}");
        assert!(body.contains("docs/strategies/one.md"), "{body}");
    }
    let brief = get_json(&client, "/api/alpha/mission/alpha-m1/brief", 200);
    assert_eq!(brief["brief"]["outcome"], "Every plan is read on the base.");
    assert_eq!(length(&brief, "/brief/criteria/parse/criteria"), 2);
    let debrief = get_json(&client, "/api/alpha/mission/alpha-m1/debrief", 200);
    assert_eq!(debrief["debrief"]["why"], "Plans go unread.");
    get_json(&client, "/api/alpha/mission/alpha-m1/strategy", 200);
}

#[test]
fn a_config_git_cannot_read_for_a_moment_does_not_keep_the_wrong_name() {
    let root = scratch();
    let repo = Repo::init(
        &root.path().join("checkout"),
        "gamma",
        "main",
        &fixture::policy("main"),
    );
    let client = client(root.path());
    object_readable(&repo, ".atelier/config.toml", false);
    let during = get_json(&client, "/api/repos", 200);
    object_readable(&repo, ".atelier/config.toml", true);
    assert!(during["repos"][0]["error"].is_string(), "{during}");
    let after = get_json(&client, "/api/repos", 200);
    assert_eq!(after["repos"][0]["repo"], "gamma", "{after}");
    assert!(after["repos"][0]["error"].is_null(), "{after}");
}
