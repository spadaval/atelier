//! Discovery: each repository once, however many worktrees it has, read
//! from its base branch found through refs and committed policy alone.

use std::path::Path;

use serde_json::Value;

use super::fixture::{self, Issue, Repo};
use super::{canonical, client, get_json, scratch};

/// The `/api/repos` entries under `root`.
fn repos(root: &Path) -> Vec<Value> {
    get_json(&client(root), "/api/repos", 200)["repos"]
        .as_array()
        .unwrap()
        .clone()
}

fn mission_ids(repo: &Value) -> Vec<String> {
    repo["missions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|mission| mission["id"].as_str().unwrap().to_owned())
        .collect()
}

fn mission(id: &str) -> Issue<'_> {
    Issue {
        id,
        issue_type: "mission",
        status: "draft",
        title: "A mission",
        body: "## Outcome\n\nDone.",
        ..Issue::default()
    }
}

#[test]
fn a_repository_with_two_worktrees_shows_once_with_the_base_branchs_records() {
    let root = scratch();
    let repo = Repo::init(
        &root.path().join("main"),
        "solo",
        "main",
        &fixture::policy("main"),
    );
    mission("solo-m1").write(&repo);
    let old = repo.commit("Plan solo-m1");
    // One worktree detached at an older base commit, one on a work branch
    // with a mission of its own.
    repo.git(&["worktree", "add", "-q", "--detach", "../behind", &old]);
    repo.git(&["worktree", "add", "-q", "-b", "work", "../ahead"]);
    let ahead = Repo::open(&root.path().join("ahead"));
    mission("solo-m3").write(&ahead);
    ahead.commit("Plan solo-m3 on work");
    mission("solo-m2").write(&repo);
    let tip = repo.commit("Plan solo-m2");
    // All three checkouts hold `.atelier/`; discovery finds each.
    let found = atelier_site::discovery::discover(&[root.path().to_path_buf()]);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(
        found[0].checkouts,
        ["ahead", "behind", "main"].map(|name| canonical(&root.path().join(name)))
    );
    assert_eq!(
        found[0].common_dir.as_ref().unwrap(),
        &canonical(&repo.dir.join(".git"))
    );
    let listed = repos(root.path());
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0]["repo"], "solo");
    assert_eq!(listed[0]["base_branch"], "main");
    assert_eq!(listed[0]["base_commit"], tip);
    assert_eq!(mission_ids(&listed[0]), ["solo-m1", "solo-m2"]);
}

#[test]
fn a_default_branch_named_by_origin_head_is_found_whatever_its_name() {
    let root = scratch();
    let repo = Repo::init(
        &root.path().join("t"),
        "trunky",
        "trunk",
        &fixture::policy("trunk"),
    );
    mission("trunky-m1").write(&repo);
    let tip = repo.commit("Plan");
    repo.git(&["update-ref", "refs/remotes/origin/trunk", &tip]);
    repo.git(&[
        "symbolic-ref",
        "refs/remotes/origin/HEAD",
        "refs/remotes/origin/trunk",
    ]);
    // Neither main nor master exists; a stray branch does not matter.
    repo.git(&["branch", "other"]);
    let listed = repos(root.path());
    assert_eq!(listed[0]["error"], Value::Null, "{}", listed[0]["error"]);
    assert_eq!(listed[0]["base_branch"], "trunk");
    assert_eq!(listed[0]["base_commit"], tip);
    assert_eq!(mission_ids(&listed[0]), ["trunky-m1"]);
}

#[test]
fn a_main_checkout_on_a_work_branch_does_not_change_the_base() {
    let root = scratch();
    let repo = Repo::init(
        &root.path().join("r"),
        "steady",
        "main",
        &fixture::policy("main"),
    );
    mission("steady-m1").write(&repo);
    let tip = repo.commit("Plan");
    repo.git(&["switch", "-q", "-c", "work"]);
    repo.write(".atelier/workflow.yaml", &fixture::policy("work"));
    mission("steady-m2").write(&repo);
    repo.commit("Name work as the base, on work");
    let listed = repos(root.path());
    assert_eq!(listed[0]["base_branch"], "main");
    assert_eq!(listed[0]["base_commit"], tip);
    assert_eq!(mission_ids(&listed[0]), ["steady-m1"]);
}

/// The error a lone repository under `root` shows, also from its routes.
fn error_of(root: &Path, repo: &str) -> String {
    let listed = repos(root);
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0]["repo"], repo);
    assert!(listed[0]["missions"].as_array().unwrap().is_empty());
    assert_eq!(listed[0]["base_commit"], Value::Null);
    let error = listed[0]["error"].as_str().unwrap().to_owned();
    let route = get_json(&client(root), &format!("/api/{repo}/mission/x/brief"), 500);
    assert_eq!(route["error"], error);
    error
}

#[test]
fn a_repository_without_a_default_branch_shows_why() {
    let root = scratch();
    let repo = Repo::init(
        &root.path().join("nodefault"),
        "x",
        "trunk",
        &fixture::policy("trunk"),
    );
    mission("x-m1").write(&repo);
    repo.commit("Plan");
    let error = error_of(root.path(), "nodefault");
    assert!(error.contains("no default branch"), "{error}");
    assert!(error.contains("neither main nor master"), "{error}");
}

#[test]
fn both_main_and_master_without_origin_head_is_an_error() {
    let root = scratch();
    let repo = Repo::init(
        &root.path().join("both"),
        "x",
        "main",
        &fixture::policy("main"),
    );
    repo.git(&["branch", "master"]);
    let error = error_of(root.path(), "both");
    assert!(error.contains("both main and master exist"), "{error}");
}

#[test]
fn a_base_whose_own_policy_names_another_branch_is_an_error() {
    let root = scratch();
    let repo = Repo::init(
        &root.path().join("loop"),
        "x",
        "main",
        &fixture::policy("release"),
    );
    repo.git(&["switch", "-q", "-c", "release"]);
    repo.write(".atelier/workflow.yaml", &fixture::policy("main"));
    repo.commit("Name main on release");
    repo.git(&["switch", "-q", "main"]);
    let error = error_of(root.path(), "loop");
    assert!(
        error.contains("names base branch release, but the policy on release names main"),
        "{error}"
    );
}

#[test]
fn a_base_branch_with_no_local_branch_is_an_error() {
    let root = scratch();
    Repo::init(
        &root.path().join("gone"),
        "x",
        "main",
        &fixture::policy("release"),
    );
    let error = error_of(root.path(), "gone");
    assert!(
        error.contains("names base branch release, which has no local branch"),
        "{error}"
    );
}
