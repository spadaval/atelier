//! The Report section (atelier-jnrh): what a transition that requires
//! `close_reason` records, read from the record body or a flag. Atelier
//! requires only a non-empty Report; the implementer report's parts are
//! Agent Factory guidance.

use super::*;

const FULL_REPORT: &str = "Shipped the view.\n\n### Deviations\nNone.\n\n### Decisions\nKept the parser in core.\n\n### Surprises\nNone.\n\n### Open items\n- atelier-x follow-up";

fn ok(dir: &Path, args: &[&str]) -> String {
    let (success, stdout, stderr) = run_atelier(dir, args);
    assert!(success, "atelier {args:?} failed: {stderr}");
    stdout
}

fn refused(dir: &Path, args: &[&str]) -> String {
    let (success, stdout, stderr) = run_atelier(dir, args);
    assert!(!success, "atelier {args:?} should fail: {stdout}");
    format!("{stdout}{stderr}")
}

/// A started work item under the starter policy.
fn started_item(dir: &Path, title: &str) -> String {
    if !dir.join(".atelier").exists() {
        init_atelier(dir);
    }
    let id = ok(
        dir,
        &[
            "issue",
            "create",
            "-q",
            "--body",
            "## Goal\n\nG\n\n## Done when\n\nD",
            title,
        ],
    )
    .trim()
    .to_string();
    ok(dir, &["issue", "transition", &id, "start"]);
    id
}

fn report(dir: &Path, id: &str) -> String {
    let record = read_canonical_record(dir, "issues", id);
    record
        .split("## Report\n\n")
        .nth(1)
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn with_report(dir: &Path, id: &str, heading: &str) {
    edit_canonical_issue(dir, id, |markdown| {
        format!("{}\n\n## {heading}\n\n{FULL_REPORT}\n", markdown.trim_end())
    });
}

#[test]
fn supersede_refuses_without_a_report_and_names_the_flag() {
    let dir = tempdir().unwrap();
    let id = started_item(dir.path(), "Work");
    let output = refused(dir.path(), &["issue", "transition", &id, "supersede"]);
    assert!(
        output.contains("missing required field close_reason; rerun with `--reason \"...\"`, or write it in the record's `## Report` section"),
        "{output}"
    );
    let options = ok(dir.path(), &["issue", "transition", &id]);
    assert!(
        options.contains(&format!(
            "atelier issue transition {id} supersede --reason \"...\""
        )),
        "{options}"
    );
    // The work workflow has no close: merge marks work done.
    assert!(!options.contains("close"), "{options}");
    assert!(read_canonical_record(dir.path(), "issues", &id).contains("status: \"in_progress\""));
}

#[test]
fn supersede_reads_the_report_from_a_file_or_the_record_body() {
    let dir = tempdir().unwrap();
    let id = started_item(dir.path(), "Work");
    let file = dir.path().join("report.md");
    fs::write(&file, format!("\n{FULL_REPORT}\n\n")).unwrap();
    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &id,
            "supersede",
            "--reason-file",
            file.to_str().unwrap(),
        ],
    );
    assert_eq!(report(dir.path(), &id), FULL_REPORT);
    let show = ok(dir.path(), &["issue", "show", &id]);
    assert!(
        show.contains("Closed ") && show.contains(": Shipped the view."),
        "{show}"
    );

    // A Report written in the record body needs no flag.
    let second = started_item(dir.path(), "Second");
    with_report(dir.path(), &second, "Report");
    ok(dir.path(), &["check"]);
    let options = ok(dir.path(), &["issue", "transition", &second]);
    assert!(
        options.contains(&format!(
            "Command: atelier issue transition {second} supersede\n"
        )),
        "{options}"
    );
    ok(dir.path(), &["issue", "transition", &second, "supersede"]);
    assert_eq!(report(dir.path(), &second), FULL_REPORT);
}

/// A record an older Atelier wrote keeps its `## Close Reason` heading until
/// it is next written, and is read as the Report.
#[test]
fn a_close_reason_heading_is_read_as_the_report_and_rewritten_as_report() {
    let dir = tempdir().unwrap();
    let id = started_item(dir.path(), "Work");
    with_report(dir.path(), &id, "Close Reason");
    ok(dir.path(), &["check"]);
    ok(dir.path(), &["issue", "transition", &id, "supersede"]);
    let record = read_canonical_record(dir.path(), "issues", &id);
    assert!(!record.contains("Close Reason"), "{record}");
    assert_eq!(report(dir.path(), &id), FULL_REPORT);
}

#[test]
fn reason_and_reason_file_conflict_and_reason_replaces_the_whole_report() {
    let dir = tempdir().unwrap();
    let id = started_item(dir.path(), "Work");
    let file = dir.path().join("report.md");
    fs::write(&file, FULL_REPORT).unwrap();
    let output = refused(
        dir.path(),
        &[
            "issue",
            "transition",
            &id,
            "supersede",
            "--reason",
            "x",
            "--reason-file",
            file.to_str().unwrap(),
        ],
    );
    assert!(output.contains("cannot be used with"), "{output}");
    let output = refused(
        dir.path(),
        &[
            "issue",
            "transition",
            &id,
            "supersede",
            "--reason-file",
            "missing.md",
        ],
    );
    assert!(
        output.contains("cannot read --reason-file missing.md"),
        "{output}"
    );
    // A report written for review gives way to the supersede reason.
    with_report(dir.path(), &id, "Report");
    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &id,
            "supersede",
            "--reason",
            "Not needed after all.",
        ],
    );
    assert_eq!(report(dir.path(), &id), "Not needed after all.");
}

#[test]
fn a_retransition_with_no_flag_says_which_report_it_kept() {
    let dir = tempdir().unwrap();
    let id = started_item(dir.path(), "Work");
    let file = dir.path().join("report.md");
    fs::write(&file, FULL_REPORT).unwrap();
    let output = ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &id,
            "supersede",
            "--reason-file",
            file.to_str().unwrap(),
        ],
    );
    assert!(!output.contains("Report:"), "{output}");

    // No transition leaves done, so an item reopens only by a hand edit that
    // keeps the old Report; the next transition names the one it kept.
    edit_canonical_issue(dir.path(), &id, |markdown| {
        replace_front_matter_scalar(&markdown, "status", "in_progress")
            .lines()
            .filter(|line| !line.starts_with("closed_at: "))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    });
    let output = ok(dir.path(), &["issue", "transition", &id, "supersede"]);
    assert!(
        output.contains("Report: kept from the record: Shipped the view.\n"),
        "{output}"
    );
    assert_eq!(report(dir.path(), &id), FULL_REPORT);
}

/// A policy from before atelier-jnrh that required each report part on its
/// work `close` still loads, and its parts are read as a non-empty Report.
#[test]
fn an_older_policys_report_parts_are_read_as_a_non_empty_report() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let policy = starter_with_work_close().replace(
        "        required_fields: [close_reason]\n        description: Done claim;",
        "        required_fields: [close_reason, report.deviations, report.decisions, report.surprises, report.open_items]\n        description: Done claim;",
    );
    assert_ne!(policy, starter_with_work_close());
    fs::write(dir.path().join(".atelier/workflow.yaml"), policy).unwrap();
    let id = started_item(dir.path(), "Work");
    let output = refused(dir.path(), &["issue", "transition", &id, "close"]);
    assert!(
        output.contains("missing required field close_reason"),
        "{output}"
    );
    ok(
        dir.path(),
        &["issue", "transition", &id, "close", "--reason", "Done."],
    );
    assert_eq!(report(dir.path(), &id), "Done.");
    assert!(read_canonical_record(dir.path(), "issues", &id).contains("status: \"done\""));
}
