//! `discovered_from` (atelier-tq1j): new work records where it was found, and
//! the debrief places it by that source.

use super::*;

fn ok(dir: &Path, args: &[&str]) -> String {
    let (success, stdout, stderr) = run_atelier_raw(dir, args);
    assert!(success, "atelier {args:?} failed: {stderr}");
    stdout
}

fn create(dir: &Path, title: &str, extra: &[&str]) -> String {
    let mut args = vec![
        "issue",
        "create",
        "-q",
        "--body",
        "## Goal\n\nG\n\n## Done when\n\nD",
        title,
    ];
    args.extend_from_slice(extra);
    ok(dir, &args).trim().to_string()
}

fn repo() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    dir
}

#[test]
fn create_stores_the_link_and_show_prints_the_source() {
    let dir = repo();
    let dir = dir.path();
    let item = create(dir, "Worked item", &[]);
    commit_all(dir, "item");
    let found = create(dir, "Found work", &["--discovered-from", &item]);

    let record = std::fs::read_to_string(dir.join(format!(".atelier/issues/{found}.md"))).unwrap();
    assert!(
        record.contains("type: \"discovered_from\"") && record.contains(&item),
        "{record}"
    );
    let shown = ok(dir, &["issue", "show", &found]);
    assert!(
        shown.contains(&format!("Discovered from: issue {item}")),
        "{shown}"
    );
    let full = ok(dir, &["issue", "show", &found, "--full"]);
    assert!(
        full.contains(&format!("Discovered From\n---------------\n  issue {item}")),
        "{full}"
    );
    ok(dir, &["check"]);
}

#[test]
fn a_finding_or_a_receipt_can_be_the_source() {
    let dir = repo();
    let dir = dir.path();
    let item = create(dir, "Worked item", &[]);
    commit_all(dir, "item");
    let finding = ok(
        dir,
        &[
            "review",
            "finding",
            &item,
            "-q",
            "--identity",
            "Reviewer",
            "--recommendation",
            "defer",
            "Edge case",
        ],
    )
    .trim()
    .to_string();
    let receipt = ok(
        dir,
        &[
            "evidence",
            "record",
            "-q",
            "--target",
            &format!("issue/{item}"),
            "--kind",
            "validation",
            "--result",
            "pass",
            "Checked",
        ],
    )
    .trim()
    .to_string();
    let from_finding = create(dir, "From finding", &["--discovered-from", &finding]);
    let from_receipt = create(dir, "From receipt", &["--discovered-from", &receipt]);

    let shown = ok(dir, &["issue", "show", &from_finding]);
    assert!(
        shown.contains(&format!("Discovered from: review {finding} (about {item})")),
        "{shown}"
    );
    let shown = ok(dir, &["issue", "show", &from_receipt]);
    assert!(
        shown.contains(&format!(
            "Discovered from: evidence {receipt} (about {item})"
        )),
        "{shown}"
    );
    ok(dir, &["check"]);
}

#[test]
fn a_missing_source_is_refused_at_create_and_by_check() {
    let dir = repo();
    let dir = dir.path();
    let (success, _, stderr) = run_atelier_raw(
        dir,
        &[
            "issue",
            "create",
            "Orphan",
            "--body",
            "## Goal\n\nG\n\n## Done when\n\nD",
            "--discovered-from",
            "atelier-nope",
        ],
    );
    assert!(!success && stderr.contains("atelier-nope"), "{stderr}");

    let item = create(dir, "Worked item", &[]);
    let found = create(dir, "Found work", &["--discovered-from", &item]);
    std::fs::remove_file(dir.join(format!(".atelier/issues/{item}.md"))).unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir, &["check"]);
    let stderr = format!("{stdout}{stderr}");
    assert!(
        !success && stderr.contains(&item) && stderr.contains("discovered_from"),
        "{stderr}"
    );
}

#[test]
fn a_deferral_must_name_the_source_its_follow_up_declares() {
    let dir = repo();
    let dir = dir.path();
    let item = create(dir, "Worked item", &[]);
    let other = create(dir, "Other item", &[]);
    commit_all(dir, "items");
    let finding = ok(
        dir,
        &[
            "review",
            "finding",
            &item,
            "-q",
            "--identity",
            "Reviewer",
            "--recommendation",
            "defer",
            "Edge case",
        ],
    )
    .trim()
    .to_string();
    let elsewhere = create(dir, "Found elsewhere", &["--discovered-from", &other]);
    ok(
        dir,
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "defer",
            "--follow-up",
            &elsewhere,
            "Deferred",
        ],
    );
    let (success, stdout, stderr) = run_atelier_raw(dir, &["check"]);
    let stderr = format!("{stdout}{stderr}");
    assert!(
        !success && stderr.contains("review_follow_up_source_mismatch") && stderr.contains(&other),
        "{stderr}"
    );
}

#[test]
fn a_deferral_to_a_follow_up_found_in_the_same_item_passes_check() {
    let dir = repo();
    let dir = dir.path();
    let item = create(dir, "Worked item", &[]);
    commit_all(dir, "item");
    let finding = ok(
        dir,
        &[
            "review",
            "finding",
            &item,
            "-q",
            "--identity",
            "Reviewer",
            "--recommendation",
            "defer",
            "Edge case",
        ],
    )
    .trim()
    .to_string();
    let follow_up = create(dir, "Found here", &["--discovered-from", &finding]);
    ok(
        dir,
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "defer",
            "--follow-up",
            &follow_up,
            "Deferred",
        ],
    );
    ok(dir, &["check"]);
}

#[test]
fn the_debrief_lists_discovered_work_by_source_without_an_advances_link() {
    let dir = repo();
    let dir = dir.path();
    let mission = ok(
        dir,
        &[
            "issue",
            "create",
            "-q",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nDone.\n\n## Success criteria\n\n- shown: It is shown.",
            "Mission",
        ],
    )
    .trim()
    .to_string();
    let item = create(dir, "Worked item", &[]);
    ok(dir, &["issue", "link", "-q", &item, &mission]);
    commit_all(dir, "plan");
    ok(dir, &["issue", "transition", &mission, "ready"]);
    commit_all(dir, "start");
    let finding = ok(
        dir,
        &[
            "review",
            "finding",
            &item,
            "-q",
            "--identity",
            "Reviewer",
            "--recommendation",
            "defer",
            "Edge case",
        ],
    )
    .trim()
    .to_string();
    let from_item = create(dir, "Noticed while working", &["--discovered-from", &item]);
    let from_finding = create(dir, "Raised by review", &["--discovered-from", &finding]);
    let elsewhere = create(dir, "Unrelated work", &[]);
    commit_all(dir, "discover work");

    let page = ok(dir, &["debrief", &mission]);
    let heading = "Found Along The Way\n-------------------\n";
    let found = page
        .split(heading)
        .nth(1)
        .expect(&page)
        .split("\n\n")
        .next()
        .unwrap();
    assert!(
        found.contains(&format!(
            "{from_item} task todo Noticed while working | found in {item}"
        )),
        "{page}"
    );
    assert!(
        found.contains(&format!(
            "{from_finding} task todo Raised by review | found in {item}"
        )),
        "{page}"
    );
    assert!(!page.contains(&elsewhere), "{page}");
    let quiet = ok(dir, &["debrief", &mission, "-q"]);
    assert!(quiet.contains("follow_ups=2"), "{quiet}");
}
