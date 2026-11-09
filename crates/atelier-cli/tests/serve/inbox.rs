//! The inbox route (atelier-uma8): what waits on the human in each fixture
//! repository, against what `atelier status` and `atelier work human`
//! print there, and the missions each entry links.

use std::path::Path;
use std::process::Command;

use serde_json::Value;

use super::api::fleet;
use super::{client, get_json, scratch};

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

/// Each item's ID with the missions it links, as `(id, review)`.
fn entries(repo: &Value) -> Vec<(String, Vec<(String, String)>)> {
    repo["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            (
                entry["item"]["id"].as_str().unwrap().to_owned(),
                entry["missions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|mission| {
                        (
                            mission["id"].as_str().unwrap().to_owned(),
                            mission["review"].as_str().unwrap().to_owned(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn pair(id: &str, review: &str) -> (String, String) {
    (id.to_owned(), review.to_owned())
}

#[test]
fn the_inbox_lists_what_status_counts_as_needing_a_human_and_approved_drafts() {
    let root = scratch();
    let (alpha, beta) = fleet(root.path());
    // `atelier status` writes runtime files a real checkout ignores.
    for repo in [&alpha, &beta] {
        std::fs::write(repo.dir.join(".git/info/exclude"), ".atelier/runtime/\n").unwrap();
    }
    // The base branch is checked out in each fixture, so the CLI reads it.
    alpha.git(&["checkout", "-q", "main"]);
    let client = client(root.path());
    let body = get_json(&client, "/api/inbox", 200);
    let repos = body["repos"].as_array().unwrap();
    let names = repos
        .iter()
        .map(|repo| repo["repo"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(names, ["alpha", "beta"]);

    for (repo, dir) in repos.iter().zip([&alpha.dir, &beta.dir]) {
        let ids = entries(repo)
            .into_iter()
            .map(|(id, _)| id)
            .collect::<Vec<_>>();
        let status = atelier(dir, &["status", "-q"]);
        assert!(
            status.contains(&format!("human={}", ids.len())),
            "{}: inbox {ids:?}, status {status}",
            repo["repo"]
        );
        let human = atelier(dir, &["work", "human", "-q"]);
        let mut listed = human.lines().map(str::to_owned).collect::<Vec<_>>();
        let mut inbox = ids.clone();
        listed.sort();
        inbox.sort();
        assert_eq!(inbox, listed, "{}", repo["repo"]);
    }

    // A decision in draft alpha-m1's scope links it, on its brief; a loose
    // end blocked by done alpha-m2 links that, on its debrief.
    assert_eq!(
        entries(&repos[0]),
        [
            ("alpha-a2".to_owned(), vec![pair("alpha-m1", "brief")]),
            ("alpha-l2".to_owned(), vec![pair("alpha-m2", "debrief")]),
        ]
    );
    // A validation related only to superseded beta-m3 links no mission.
    assert_eq!(entries(&repos[1]), [("beta-v1".to_owned(), vec![])]);

    // Draft alpha-m1 has a planning approval; beta-m1, in its initial status
    // with none, is left out.
    let drafts = |repo: &Value| {
        repo["drafts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|mission| {
                pair(
                    mission["id"].as_str().unwrap(),
                    mission["review"].as_str().unwrap(),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(drafts(&repos[0]), [pair("alpha-m1", "brief")]);
    assert_eq!(drafts(&repos[1]), []);
    assert_eq!(body["count"], 4);
}
