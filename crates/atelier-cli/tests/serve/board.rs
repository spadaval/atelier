//! The board and item routes (atelier-ahpf): each board section on the
//! fixture repositories, review state on a work branch against what
//! `atelier status` prints, the clock that ages recently done work, and the
//! item page of a task, a decision and a loose end.

use std::process::Command;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

use serde_json::Value;

use super::api::fleet;
use super::fixture::{self, Repo};
use super::{client, client_at, get_json, scratch};

fn ids(rows: &Value) -> Vec<&str> {
    rows.as_array()
        .unwrap_or_else(|| panic!("{rows} is not an array"))
        .iter()
        .map(|row| row["id"].as_str().unwrap())
        .collect()
}

#[test]
fn the_board_shows_each_section_from_the_base_and_its_work_branch() {
    let root = scratch();
    let (alpha, _) = fleet(root.path());
    let client = client(root.path());
    let body = get_json(&client, "/api/alpha/board", 200);
    assert_eq!(body["base_branch"], "main");
    assert_eq!(body["base_commit"], alpha.git(&["rev-parse", "main"]));
    let board = &body["board"];
    assert_eq!(board["today"], "2026-01-10");

    // Open missions with progress: alpha-m2 is done, so only alpha-m1.
    let missions = board["missions"].as_array().unwrap();
    assert_eq!(missions.len(), 1, "{missions:?}");
    let mission = &missions[0];
    assert_eq!(mission["id"], "alpha-m1");
    assert_eq!(
        (mission["done"].as_u64(), mission["planned"].as_u64()),
        (Some(0), Some(2))
    );
    // alpha-e2 proves `shown` at a commit in the base's history.
    assert_eq!(
        (mission["proven"].as_u64(), mission["criteria"].as_u64()),
        (Some(1), Some(2))
    );
    assert!(mission["criteria_error"].is_null());

    // Open work by mission, then by label for work in no mission.
    let groups = board["groups"].as_array().unwrap();
    let headings = groups
        .iter()
        .map(|group| {
            (
                group["mission"].as_str(),
                group["label"].as_str(),
                ids(&group["work"]),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        headings,
        [
            (Some("alpha-m1"), None, vec!["alpha-a1", "alpha-a2"]),
            (None, Some("needs-human"), vec!["alpha-l2"]),
            (None, Some("ops"), vec!["alpha-l1"]),
        ]
    );
    let status = |group: usize, at: usize| {
        let row = &groups[group]["work"][at];
        (
            row["status"].as_str().unwrap(),
            row["state"].as_str().unwrap(),
        )
    };
    assert_eq!(status(0, 0), ("todo", "ready"));
    assert_eq!(status(0, 1), ("todo", "needs human"));
    assert_eq!(status(2, 0), ("todo", "blocked"));

    // The base marks nothing active; alpha-a1 is in progress on its branch,
    // reviewed with a finding nothing has decided, one base commit behind.
    assert_eq!(ids(&board["active"]), Vec::<&str>::new());
    let in_progress = body["in_progress"].as_array().unwrap();
    assert_eq!(in_progress.len(), 1, "{in_progress:?}");
    let work = &in_progress[0];
    assert_eq!(work["item"]["id"], "alpha-a1");
    assert_eq!(work["item"]["status"], "in_progress");
    assert_eq!(work["branch"], "alpha-a1");
    assert_eq!(
        work["on_branch"]["tip"],
        alpha.git(&["rev-parse", "alpha-a1"])
    );
    assert_eq!(work["on_branch"]["review"], "not_approved");
    assert_eq!(
        work["on_branch"]["open_findings"],
        serde_json::json!(["alpha-r2"])
    );
    assert_eq!(work["on_branch"]["behind"], 1);
    assert_eq!(body["unreadable_branches"], serde_json::json!([]));

    // Ready and needs-human work as `atelier work ready` lists it.
    assert_eq!(ids(&board["ready"]), ["alpha-a1"]);
    assert_eq!(ids(&board["human"]), ["alpha-a2", "alpha-l2"]);
    let ready = atelier(&alpha.dir, &["work", "ready", "-q"]);
    assert_eq!(ready.lines().collect::<Vec<_>>(), ["alpha-a1"]);
    let human = atelier(&alpha.dir, &["work", "human", "-q"]);
    assert_eq!(human.lines().collect::<Vec<_>>(), ["alpha-a2", "alpha-l2"]);

    // Done within 14 days of the fixture clock, newest first.
    assert_eq!(ids(&board["recently_done"]), ["alpha-m2", "alpha-b1"]);

    // beta has no branch, and its one closed record is superseded beta-m3.
    let beta = get_json(&client, "/api/beta/board", 200);
    let missions = beta["board"]["missions"].as_array().unwrap();
    assert_eq!(
        missions
            .iter()
            .map(|m| m["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["beta-m1", "beta-m2"]
    );
    assert_eq!(beta["in_progress"], serde_json::json!([]));
    assert_eq!(ids(&beta["board"]["recently_done"]), ["beta-m3"]);
}

fn atelier(dir: &std::path::Path, args: &[&str]) -> String {
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

/// The approval words `atelier status` prints on `branch`'s row.
fn status_approval(repo: &Repo, branch: &str) -> String {
    let status = atelier(&repo.dir, &["status"]);
    let row = status
        .lines()
        .find(|line| line.trim_start().starts_with(&format!("{branch}  ")))
        .unwrap_or_else(|| panic!("no row for {branch} in:\n{status}"));
    ["✓ approved", "⚠ approval stale", "⚠ not approved"]
        .into_iter()
        .find(|words| row.contains(words))
        .unwrap_or_else(|| panic!("no approval words in {row}"))
        .to_owned()
}

#[test]
fn review_state_follows_the_work_branch_on_the_next_request_as_status_says() {
    let root = scratch();
    let (_, beta) = fleet(root.path());
    // `atelier status` writes runtime files a real checkout ignores.
    std::fs::write(beta.dir.join(".git/info/exclude"), ".atelier/runtime/\n").unwrap();
    let client = client(root.path());
    let base = get_json(&client, "/api/beta/board", 200)["base_commit"].clone();
    let review = |client: &atelier_site::Client| {
        let body = get_json(client, "/api/beta/board", 200);
        assert_eq!(body["base_commit"], base, "the base has not moved");
        body["in_progress"]
            .as_array()
            .unwrap()
            .iter()
            .find(|work| work["branch"] == "beta-c1")
            .map(|work| work["on_branch"]["review"].as_str().unwrap().to_owned())
    };
    assert_eq!(review(&client), None);

    // Started, with a code commit and no review: unreviewed.
    let work = Repo::open(&beta.dir);
    work.git(&["checkout", "-q", "-b", "beta-c1"]);
    fixture::Issue {
        id: "beta-c1",
        status: "in_progress",
        title: "List the asks",
        body: "## Goal\n\nList them.\n\n## Done when\n\n- [ ] They are listed.",
        relates: &[("beta-m2", "advances")],
        branch: Some("beta-c1"),
        updated: 30,
        ..fixture::Issue::default()
    }
    .write(&work);
    work.commit("Start beta-c1");
    work.write("src/asks.txt", "the asks\n");
    let code = work.commit("List the asks");
    work.git(&["checkout", "-q", "main"]);
    assert_eq!(review(&client).as_deref(), Some("unreviewed"));
    assert_eq!(status_approval(&beta, "beta-c1"), "⚠ not approved");

    // Approved at its code commit.
    work.git(&["checkout", "-q", "beta-c1"]);
    work.write(
        ".atelier/reviews/beta-r1.md",
        &fixture::approval("beta-r1", "beta-c1", &code, 40),
    );
    work.commit("Approve beta-c1");
    work.git(&["checkout", "-q", "main"]);
    assert_eq!(status_approval(&beta, "beta-c1"), "✓ approved");
    assert_eq!(review(&client).as_deref(), Some("approved"));

    // A later code commit makes the approval stale.
    work.git(&["checkout", "-q", "beta-c1"]);
    work.write("src/asks.txt", "the asks, sorted\n");
    work.commit("Sort the asks");
    work.git(&["checkout", "-q", "main"]);
    assert_eq!(review(&client).as_deref(), Some("approval_stale"));
    assert_eq!(status_approval(&beta, "beta-c1"), "⚠ approval stale");
}

#[test]
fn a_warm_board_builds_nothing_until_a_branch_moves() {
    let root = scratch();
    let (alpha, _) = fleet(root.path());
    let client = client(root.path());
    let first = get_json(&client, "/api/alpha/board", 200);
    let built = client.site().builds_run();
    assert_eq!(get_json(&client, "/api/alpha/board", 200), first);
    assert_eq!(
        client.site().builds_run(),
        built,
        "a warm board builds nothing"
    );
    alpha.git(&["branch", "-f", "alpha-a1", "alpha-a1~1"]);
    let moved = get_json(&client, "/api/alpha/board", 200);
    assert_eq!(
        moved["in_progress"][0]["on_branch"]["open_findings"],
        serde_json::json!([])
    );
    assert_eq!(
        client.site().builds_run(),
        built + 1,
        "only the branch is read again"
    );
}

#[test]
fn recently_done_work_leaves_the_board_after_14_days() {
    let root = scratch();
    fleet(root.path());
    // alpha-b1 closed on 2026-01-01; the clock starts 14 days later.
    let now = Arc::new(AtomicI64::new(1_767_225_600 + 14 * 86_400));
    let clock = {
        let now = now.clone();
        atelier_site::Clock::new(move || {
            chrono::DateTime::from_timestamp(now.load(Ordering::SeqCst), 0).unwrap()
        })
    };
    let client = client_at(root.path(), clock);
    let done = |client: &atelier_site::Client| {
        let body = get_json(client, "/api/alpha/board", 200);
        ids(&body["board"]["recently_done"])
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    assert_eq!(done(&client), ["alpha-m2", "alpha-b1"]);
    now.fetch_add(86_400, Ordering::SeqCst);
    assert_eq!(done(&client), Vec::<String>::new());
}

#[test]
fn item_pages_show_a_task_a_decision_and_a_loose_end() {
    let root = scratch();
    fleet(root.path());
    let client = client(root.path());
    let item = |id: &str| get_json(&client, &format!("/api/alpha/item/{id}"), 200)["item"].clone();
    let links = |item: &Value| {
        item["links"]
            .as_array()
            .unwrap()
            .iter()
            .map(|link| {
                (
                    link["role"].as_str().unwrap().to_owned(),
                    link["id"].as_str().unwrap().to_owned(),
                    link["incoming"].as_bool().unwrap(),
                )
            })
            .collect::<Vec<_>>()
    };
    let link = |role: &str, id: &str, incoming: bool| (role.to_owned(), id.to_owned(), incoming);

    let task = item("alpha-b1");
    assert_eq!(task["issue"]["status"], "done");
    assert!(task["body"]
        .as_str()
        .unwrap()
        .starts_with("## Goal\n\nWrite it."));
    assert!(task["body"].as_str().unwrap().contains("## Report"));
    assert_eq!(links(&task), [link("advances", "alpha-m2", false)]);
    let reviews = task["reviews"].as_array().unwrap();
    assert_eq!(reviews.len(), 1);
    assert_eq!(reviews[0]["event"], "approval");
    assert_eq!(reviews[0]["identity"], "fixture-reviewer");

    let decision = item("alpha-a2");
    let body = decision["body"].as_str().unwrap();
    assert!(body.contains("## Question\n\nWhich server?"), "{body}");
    assert!(body.contains("## Options\n\n- axum on tokio."), "{body}");
    assert_eq!(decision["state"], "needs human");

    let loose = item("alpha-l1");
    assert_eq!(loose["issue"]["issue_type"], "loose_end");
    assert_eq!(loose["state"], "blocked");
    assert_eq!(loose["labels"], serde_json::json!(["ops"]));
    assert!(links(&loose).contains(&link("blocked_by", "alpha-a1", false)));
    assert!(loose["body"]
        .as_str()
        .unwrap()
        .ends_with("## Notes\n\n- 2026-01-01 The server is not set up yet."));

    // A short id resolves; an unknown one is a 404.
    assert_eq!(item("a1")["issue"]["id"], "alpha-a1");
    let missing = get_json(&client, "/api/alpha/item/alpha-zzzz", 404);
    assert_eq!(missing["error"], "no item alpha-zzzz in alpha");
}
