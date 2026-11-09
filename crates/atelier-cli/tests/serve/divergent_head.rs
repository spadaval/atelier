//! The views read one captured base commit and nothing else: with the
//! common Git directory's `HEAD` on a work branch that diverges from the
//! base, every view matches a checkout of the base.

use std::path::Path;

use atelier_app::git_freshness::ReadContext;
use atelier_app::record_index::RecordIndex;
use serde_json::Value;

use super::fixture::{self, receipt};
use super::{client, get_json, scratch};

/// The brief and debrief of `mission` as the CLI builds them in `checkout`.
fn cli_views(checkout: &Path, mission: &str) -> (Value, Value) {
    let index = RecordIndex::load(&checkout.join(".atelier")).unwrap();
    let policy = atelier_app::workflow_policy::load(checkout).unwrap();
    let mut read = ReadContext::new(checkout);
    let brief =
        atelier_app::mission_review::brief(&index, Some(&policy), &mut read, mission).unwrap();
    let mut read = ReadContext::new(checkout);
    let debrief =
        atelier_app::mission_review::debrief(&index, Some(&policy), &mut read, mission).unwrap();
    (
        serde_json::to_value(brief).unwrap(),
        serde_json::to_value(debrief).unwrap(),
    )
}

#[test]
fn views_read_the_base_commit_when_head_is_a_diverging_work_branch() {
    let root = scratch();
    let (alpha, _) = fixture::alpha(&root.path().join("alpha"));
    let base = alpha.git(&["rev-parse", "main"]);
    let elsewhere = scratch();
    let base_checkout = elsewhere.path().join("base");
    alpha.git(&[
        "worktree",
        "add",
        "-q",
        "--detach",
        base_checkout.to_str().unwrap(),
        &base,
    ]);

    // The main checkout, whose HEAD is the common directory's HEAD, moves
    // to a work branch with its own strategy text, criteria, code and a
    // receipt only it holds.
    alpha.git(&["switch", "-q", "-c", "work"]);
    alpha.write(
        "docs/strategies/one.md",
        "# Why\n\nBranch why.\n\n# Outcome\n\nBranch outcome.\n\n# Success Criteria\n\n- shown: The branch brief.\n- branch-only: Only on the branch.\n",
    );
    alpha.write("src/loader.txt", "branch code\n");
    let branch_code = alpha.commit("Rewrite the plan on work");
    alpha.write(
        ".atelier/evidence/alpha-e9.md",
        &receipt(
            "alpha-e9",
            "alpha-m1",
            &branch_code,
            "fail",
            &[("shown", "fail")],
            "alpha-m1",
            90,
        ),
    );
    alpha.commit("Fail shown on work");
    assert_eq!(alpha.git(&["symbolic-ref", "HEAD"]), "refs/heads/work");

    let client = client(root.path());
    for mission in ["alpha-m1", "alpha-m2"] {
        let (brief, debrief) = cli_views(&base_checkout, mission);
        let served = get_json(&client, &format!("/api/alpha/mission/{mission}/brief"), 200);
        assert_eq!(served["base_commit"], base);
        assert_eq!(served["brief"], brief, "brief of {mission}");
        let served = get_json(
            &client,
            &format!("/api/alpha/mission/{mission}/debrief"),
            200,
        );
        assert_eq!(served["base_commit"], base);
        assert_eq!(served["debrief"], debrief, "debrief of {mission}");
        // The work branch's checkout gives other views, so the comparison
        // above could fail.
        let (branch_brief, branch_debrief) = cli_views(&alpha.dir, mission);
        assert_ne!(branch_debrief, debrief, "debrief of {mission} on work");
        if mission == "alpha-m1" {
            assert_ne!(branch_brief, brief);
        }
    }

    // What the base holds, named.
    let brief = &get_json(&client, "/api/alpha/mission/alpha-m1/brief", 200)["brief"];
    assert_eq!(brief["generated_from"], base);
    assert_eq!(brief["outcome"], "Every plan is read on the base.");
    let slugs = brief["criteria"]["parse"]["criteria"]
        .as_array()
        .unwrap()
        .iter()
        .map(|criterion| criterion["slug"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(slugs, ["shown", "current"]);
    let debrief = &get_json(&client, "/api/alpha/mission/alpha-m1/debrief", 200)["debrief"];
    assert_eq!(debrief["generated_from"], base);
    assert_eq!(debrief["why"], "Plans go unread.");
    let shown = &debrief["criteria"]["proofs"][0];
    assert_eq!(shown["criterion"]["slug"], "shown");
    assert_eq!(shown["mark"], "proven");
    let receipts = shown["receipts"].as_array().unwrap();
    assert_eq!(receipts.len(), 1, "only the base's receipt: {receipts:?}");
    assert_eq!(receipts[0]["evidence_id"], "alpha-e2");
    // Two code commits on the base since the receipt's pin (the loader's
    // speed-up and the notes); the branch's own code commit is not counted.
    assert_eq!(receipts[0]["code_commits_since"], 2);
    let strategy = get_json(&client, "/api/alpha/mission/alpha-m1/strategy", 200);
    assert!(strategy["markdown"]
        .as_str()
        .unwrap()
        .starts_with("# Why\n\nPlans go unread."));
}
