//! `atelier review defer` (atelier-stnv): every finding on an item that was
//! recommended defer with no decision gets a backlog follow-up under one
//! parent per item and a defer disposition, in one step; a dry run writes
//! nothing, and a later run reuses the parent and files nothing twice.

use super::*;

fn ok(dir: &Path, args: &[&str]) -> String {
    let (success, stdout, stderr) = run_atelier_raw(dir, args);
    assert!(success, "atelier {args:?} failed: {stdout}{stderr}");
    stdout
}

fn finding(dir: &Path, id: &str, recommendation: &str, text: &str) -> String {
    ok(
        dir,
        &[
            "-q",
            "review",
            "finding",
            id,
            "--identity",
            "Reviewer",
            "--recommendation",
            recommendation,
            text,
        ],
    )
    .trim()
    .to_owned()
}

fn disposition_of(dir: &Path, finding: &str) -> String {
    ok(dir, &["review", "show", finding])
        .lines()
        .find(|line| line.starts_with("Disposition:"))
        .unwrap_or_default()
        .to_owned()
}

/// A started item with a code commit, a child, and findings on both.
fn reviewed(dir: &Path) -> (String, Vec<String>, String, String) {
    init_atelier(dir);
    init_git_repo(dir);
    let id = ok(
        dir,
        &["issue", "create", "-q", "--issue-type", "feature", "Parser"],
    )
    .trim()
    .to_string();
    let child = ok(dir, &["issue", "create", "-q", "--parent", &id, "Lexer"])
        .trim()
        .to_string();
    commit_all(dir, "plan");
    ok(dir, &["start", &id]);
    fs::write(dir.join("parser.rs"), "fn parse() {}\n").unwrap();
    commit_all(dir, "add the parser");
    let deferred = vec![
        finding(dir, &id, "defer", "high: parser.rs:1 has no error type"),
        finding(dir, &child, "defer", "the lexer allocates per token"),
    ];
    let fix_now = finding(dir, &id, "fix_now", "high: parse panics on empty input");
    let no_action = finding(dir, &id, "no_action", "low: rename parse");
    commit_all(dir, "findings");
    (id, deferred, fix_now, no_action)
}

#[test]
fn review_defer_dry_run_lists_the_plan_and_writes_nothing() {
    let dir = tempdir().unwrap();
    let (id, deferred, fix_now, no_action) = reviewed(dir.path());
    let output = ok(
        dir.path(),
        &["review", "defer", &id, "--identity", "Lead", "--dry-run"],
    );
    assert!(
        output.contains(&format!(
            "Would defer 2 findings on {id} under a new backlog parent \"Deferred review findings on {id}\":"
        )) && output.contains(&format!(
            "  {} → new backlog follow-up \"parser.rs:1 has no error type\"",
            deferred[0]
        )) && output.contains(&format!(
            "  {} → new backlog follow-up \"the lexer allocates per token\"",
            deferred[1]
        )) && !output.contains(&fix_now)
            && !output.contains(&no_action)
            && output.contains("Dry run: nothing was written."),
        "{output}"
    );
    assert_eq!(git_status_short(dir.path()), "");
    assert_eq!(
        disposition_of(dir.path(), &deferred[0]),
        "Disposition: ⚠ none"
    );
}

#[test]
fn review_defer_files_one_follow_up_per_finding_and_a_second_run_files_nothing() {
    let dir = tempdir().unwrap();
    let (id, deferred, fix_now, no_action) = reviewed(dir.path());
    let output = ok(
        dir.path(),
        &[
            "review",
            "defer",
            &id,
            "--identity",
            "Lead",
            "--commit-records",
        ],
    );
    // Its records are one commit, and the tree is clean after it.
    assert_eq!(git_status_short(dir.path()), "");
    let subject = Command::new("git")
        .current_dir(dir.path())
        .args(["log", "-1", "--format=%s"])
        .output()
        .unwrap();
    let subject = String::from_utf8_lossy(&subject.stdout).to_string();
    assert!(subject.starts_with("Record defer "), "{subject}");

    let parent = output
        .lines()
        .next()
        .and_then(|line| line.split(" under ").nth(1))
        .and_then(|rest| rest.split(' ').next())
        .unwrap()
        .to_owned();
    assert!(
        output.starts_with(&format!(
            "Deferred 2 findings on {id} under {parent} (new) @"
        )),
        "{output}"
    );
    let parent_record = read_canonical_record(dir.path(), "issues", &parent);
    assert!(
        parent_record.contains("deferred-findings")
            && parent_record.contains("discovered_from")
            && parent_record.contains(&id)
            && parent_record.contains("status: \"backlog\""),
        "{parent_record}"
    );
    let mut follow_ups = Vec::new();
    for finding in &deferred {
        let line = disposition_of(dir.path(), finding);
        assert!(
            line.starts_with("Disposition: defer by Lead @") && line.contains(", follow-up "),
            "{line}"
        );
        let follow_up = line
            .split(", follow-up ")
            .nth(1)
            .and_then(|rest| rest.split([' ', ':']).next())
            .unwrap()
            .to_owned();
        let record = read_canonical_record(dir.path(), "issues", &follow_up);
        assert!(
            record.contains("status: \"backlog\"")
                && record.contains(finding)
                && record.contains("discovered_from"),
            "{record}"
        );
        assert!(parent_record.contains(&follow_up), "{parent_record}");
        follow_ups.push(follow_up);
    }
    // The finding's severity sets the follow-up's priority.
    assert!(
        read_canonical_record(dir.path(), "issues", &follow_ups[0]).contains("priority: \"P1\"")
    );
    // Findings recommended otherwise are left alone.
    assert_eq!(disposition_of(dir.path(), &fix_now), "Disposition: ⚠ none");
    assert_eq!(
        disposition_of(dir.path(), &no_action),
        "Disposition: ⚠ none"
    );
    ok(dir.path(), &["check"]);

    // A second run finds nothing to defer and files nothing.
    let again = ok(dir.path(), &["review", "defer", &id, "--identity", "Lead"]);
    assert!(again.contains("nothing to defer"), "{again}");
    assert_eq!(git_status_short(dir.path()), "");

    // A later deferred finding goes under the same parent.
    let later = finding(dir.path(), &id, "defer", "medium: add a fuzz test");
    commit_all(dir.path(), "a later finding");
    let output = ok(dir.path(), &["review", "defer", &id, "--identity", "Lead"]);
    assert!(
        output.starts_with(&format!("Deferred 1 finding on {id} under {parent} @")),
        "{output}"
    );
    let issues = fs::read_dir(dir.path().join(".atelier/issues"))
        .unwrap()
        .filter_map(|entry| {
            let text = fs::read_to_string(entry.unwrap().path()).unwrap();
            text.contains("deferred-findings").then_some(text)
        })
        .count();
    assert_eq!(issues, 1, "one parent per item");
    let parent_record = read_canonical_record(dir.path(), "issues", &parent);
    let line = disposition_of(dir.path(), &later);
    let follow_up = line
        .split(", follow-up ")
        .nth(1)
        .and_then(|rest| rest.split([' ', ':']).next())
        .unwrap();
    assert!(parent_record.contains(follow_up), "{parent_record}");
}
