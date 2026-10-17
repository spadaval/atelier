//! The implementer report (atelier-sfsq): policy-required parts of Close
//! Reason, read from the record body or a file at the close transition.

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

/// A started work item under a policy whose work `close` requires every
/// report part.
fn started_item(dir: &Path) -> String {
    init_atelier(dir);
    let path = dir.join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    let close = "        required_fields: [close_reason]\n        description: Done claim;";
    assert!(policy.contains(close), "{policy}");
    fs::write(
        &path,
        policy.replace(
            close,
            "        required_fields: [close_reason, report.deviations, report.decisions, report.surprises, report.open_items]\n        description: Done claim;",
        ),
    )
    .unwrap();
    let id = ok(
        dir,
        &[
            "issue",
            "create",
            "-q",
            "--body",
            "## Goal\n\nG\n\n## Done when\n\nD",
            "Work",
        ],
    )
    .trim()
    .to_string();
    ok(dir, &["issue", "transition", &id, "start"]);
    id
}

fn close_reason(dir: &Path, id: &str) -> String {
    let record = read_canonical_record(dir, "issues", id);
    record
        .split("## Close Reason\n\n")
        .nth(1)
        .unwrap_or_default()
        .trim()
        .to_string()
}

#[test]
fn close_refuses_a_missing_report_part_and_names_the_recovery() {
    let dir = tempdir().unwrap();
    let id = started_item(dir.path());
    let output = refused(
        dir.path(),
        &["issue", "transition", &id, "close", "--reason", "Done"],
    );
    assert!(
        output.contains("missing report parts Deviations, Decisions, Surprises, Open items; add `### Deviations`, `### Decisions`, `### Surprises`, `### Open items` to the record's Close Reason (write `None.` when there is nothing to report), or pass the whole report with `--reason-file <file>`"),
        "{output}"
    );
    let options = ok(dir.path(), &["issue", "transition", &id]);
    assert!(
        options.contains(&format!(
            "atelier issue transition {id} close --reason-file <file>"
        )),
        "{options}"
    );
    let quiet = ok(dir.path(), &["issue", "transition", &id, "-q"]);
    assert!(
        quiet.lines().any(|name| name == "close"),
        "close is allowed with a flag: {quiet}"
    );

    // An empty part is a missing part.
    let file = dir.path().join("report.md");
    fs::write(&file, FULL_REPORT.replace("- atelier-x follow-up", "")).unwrap();
    let output = refused(
        dir.path(),
        &[
            "issue",
            "transition",
            &id,
            "close",
            "--reason-file",
            file.to_str().unwrap(),
        ],
    );
    assert!(
        output.contains("missing report part Open items;"),
        "{output}"
    );
    assert!(!output.contains("Deviations,"), "{output}");

    // With no Outcome, the file stands in for --reason too.
    fs::write(&file, FULL_REPORT.replace("Shipped the view.\n\n", "")).unwrap();
    let output = refused(
        dir.path(),
        &[
            "issue",
            "transition",
            &id,
            "close",
            "--reason-file",
            file.to_str().unwrap(),
        ],
    );
    assert!(
        output.contains("missing required field close_reason"),
        "{output}"
    );
    assert_eq!(
        read_canonical_record(dir.path(), "issues", &id).contains("status: \"done\""),
        false
    );
}

#[test]
fn close_reads_the_report_from_a_file_or_the_record_body() {
    let dir = tempdir().unwrap();
    let id = started_item(dir.path());
    let file = dir.path().join("report.md");
    fs::write(&file, format!("\n{FULL_REPORT}\n\n")).unwrap();
    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &id,
            "close",
            "--reason-file",
            file.to_str().unwrap(),
        ],
    );
    assert_eq!(close_reason(dir.path(), &id), FULL_REPORT);
    let show = ok(dir.path(), &["issue", "show", &id]);
    assert!(
        show.contains("Closed ") && show.contains(": Shipped the view."),
        "{show}"
    );

    // A report written in the record body needs no flag.
    let second = ok(
        dir.path(),
        &[
            "issue",
            "create",
            "-q",
            "--body",
            "## Goal\n\nG\n\n## Done when\n\nD",
            "Second",
        ],
    )
    .trim()
    .to_string();
    ok(dir.path(), &["issue", "transition", &second, "start"]);
    edit_canonical_issue(dir.path(), &second, |markdown| {
        format!(
            "{}\n\n## Close Reason\n\n{FULL_REPORT}\n",
            markdown.trim_end()
        )
    });
    ok(dir.path(), &["check"]);
    let options = ok(dir.path(), &["issue", "transition", &second]);
    assert!(
        options.contains(&format!(
            "Command: atelier issue transition {second} close\n"
        )),
        "{options}"
    );
    ok(dir.path(), &["issue", "transition", &second, "close"]);
    assert_eq!(close_reason(dir.path(), &second), FULL_REPORT);
}

#[test]
fn reason_replaces_only_the_outcome_of_a_written_report() {
    let dir = tempdir().unwrap();
    let id = started_item(dir.path());
    edit_canonical_issue(dir.path(), &id, |markdown| {
        format!(
            "{}\n\n## Close Reason\n\n{FULL_REPORT}\n",
            markdown.trim_end()
        )
    });
    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &id,
            "close",
            "--reason",
            "Shipped it differently.",
        ],
    );
    assert_eq!(
        close_reason(dir.path(), &id),
        FULL_REPORT.replace("Shipped the view.", "Shipped it differently.")
    );
}

#[test]
fn reason_and_reason_file_conflict_and_supersede_needs_only_the_outcome() {
    let dir = tempdir().unwrap();
    let id = started_item(dir.path());
    let file = dir.path().join("report.md");
    fs::write(&file, FULL_REPORT).unwrap();
    let output = refused(
        dir.path(),
        &[
            "issue",
            "transition",
            &id,
            "close",
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
            "close",
            "--reason-file",
            "missing.md",
        ],
    );
    assert!(
        output.contains("cannot read --reason-file missing.md"),
        "{output}"
    );
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
    assert_eq!(close_reason(dir.path(), &id), "Not needed after all.");
}
