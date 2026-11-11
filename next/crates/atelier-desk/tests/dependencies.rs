//! The Desk stays out of what the tracker ships: nothing depends on it, and
//! its server's crates are in no tracker crate's dependencies.

use serde_json::Value;
use std::collections::{BTreeSet, HashMap};
use std::process::Command;

const TRACKER: [&str; 4] = [
    "atelier-cli",
    "atelier-app",
    "atelier-records",
    "atelier-core",
];
const SERVER: [&str; 5] = ["axum", "tokio", "ts-rs", "tower-http", "ureq"];

/// Each package's dependencies by ID, with whether each is only for its
/// tests.
type Edges = HashMap<String, Vec<(String, bool)>>;

/// `cargo metadata` of this workspace: each package's name by ID, and its
/// dependencies.
fn graph() -> (HashMap<String, String>, Edges) {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let output = Command::new(cargo)
        .args(["metadata", "--format-version", "1", "--locked"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
    assert!(output.status.success(), "cargo metadata: {output:?}");
    let metadata: Value = serde_json::from_slice(&output.stdout).unwrap();
    let names = metadata["packages"].as_array().unwrap().iter();
    let names = names.map(|p| {
        (
            p["id"].as_str().unwrap().into(),
            p["name"].as_str().unwrap().into(),
        )
    });
    let mut deps = HashMap::new();
    for node in metadata["resolve"]["nodes"].as_array().unwrap() {
        let edges = node["deps"].as_array().unwrap().iter().map(|dep| {
            let kinds = dep["dep_kinds"].as_array().unwrap();
            let dev_only = kinds.iter().all(|kind| kind["kind"] == "dev");
            (dep["pkg"].as_str().unwrap().to_string(), dev_only)
        });
        deps.insert(node["id"].as_str().unwrap().to_string(), edges.collect());
    }
    (names.collect(), deps)
}

/// The names of every package `name` is built with, its tests' own
/// dependencies left out.
fn shipped_with(name: &str) -> BTreeSet<String> {
    let (names, deps) = graph();
    let start = names.iter().find(|(_, n)| *n == name).unwrap().0.clone();
    let mut seen = BTreeSet::new();
    let mut pending = vec![start];
    while let Some(id) = pending.pop() {
        if !seen.insert(id.clone()) {
            continue;
        }
        let edges = deps.get(&id).into_iter().flatten();
        pending.extend(
            edges
                .filter(|(_, dev_only)| !dev_only)
                .map(|(dep, _)| dep.clone()),
        );
    }
    seen.iter().map(|id| names[id].clone()).collect()
}

#[test]
fn no_crate_depends_on_the_desk() {
    let (names, deps) = graph();
    for (id, edges) in &deps {
        let on_desk = edges.iter().any(|(dep, _)| names[dep] == "atelier-desk");
        assert!(!on_desk, "{} depends on atelier-desk", names[id]);
    }
}

#[test]
fn the_tracker_is_built_without_the_server_crates() {
    // The walk finds them where they are.
    let desk = shipped_with("atelier-desk");
    assert!(SERVER.iter().all(|name| desk.contains(*name)), "{desk:?}");
    for crate_name in TRACKER {
        let found = shipped_with(crate_name);
        let server: Vec<&&str> = SERVER
            .iter()
            .filter(|name| found.contains(**name))
            .collect();
        assert!(server.is_empty(), "{crate_name} is built with {server:?}");
    }
}
