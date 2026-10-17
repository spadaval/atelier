//! Decision capture (atelier-fip8): decision notes inside work items, and
//! decision records with their ADR and supersession.

use super::*;

fn ok(dir: &Path, args: &[&str]) -> String {
    let (success, stdout, stderr) = run_atelier(dir, args);
    assert!(success, "atelier {args:?} failed: {stderr}");
    stdout
}

fn refused(dir: &Path, args: &[&str]) -> String {
    let (success, stdout, stderr) = run_atelier(dir, args);
    assert!(!success, "atelier {args:?} should fail: {stdout}");
    stderr
}

/// A mission with one work item and two decision records in its scope.
fn mission_fixture(dir: &Path) -> (String, String, String, String) {
    init_atelier(dir);
    let mission = ok(
        dir,
        &[
            "issue",
            "create",
            "-q",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nDecisions are reviewable.\n\n## Success criteria\n\n- S1: shown",
            "Mission",
        ],
    )
    .trim()
    .to_string();
    let work = ok(
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
    let decision = |title: &str| {
        ok(
            dir,
            &[
                "issue",
                "create",
                "-q",
                "--issue-type",
                "decision",
                "--body",
                "## Question\n\nWhich?\n\n## Decision\n\nThis one.",
                title,
            ],
        )
        .trim()
        .to_string()
    };
    let (old, new) = (decision("Old decision"), decision("New decision"));
    for id in [&work, &old, &new] {
        ok(dir, &["issue", "link", "-q", id, &mission]);
    }
    (mission, work, old, new)
}

#[test]
fn decision_notes_record_choice_reason_rejected_and_one_way() {
    let dir = tempdir().unwrap();
    let (mission, work, _, _) = mission_fixture(dir.path());
    ok(
        dir.path(),
        &[
            "issue",
            "note",
            &work,
            "--kind",
            "decision",
            "Keep the report in Close Reason",
            "--reason",
            "It is already exempt\nfrom approval freshness",
            "--rejected",
            "a Report section",
            "--rejected",
            "a sidecar file",
            "--one-way",
        ],
    );
    ok(
        dir.path(),
        &[
            "issue",
            "note",
            &work,
            "--kind",
            "decision",
            "Name parts with ###",
            "--reason",
            "Records allow it",
        ],
    );
    let record = read_canonical_record(dir.path(), "issues", &work);
    assert!(
        record.contains("[decision one-way] Keep the report in Close Reason\n  Reason: It is already exempt from approval freshness\n  Rejected: a Report section\n  Rejected: a sidecar file\n"),
        "{record}"
    );
    assert!(
        record.contains("[decision] Name parts with ###\n  Reason: Records allow it"),
        "{record}"
    );
    ok(dir.path(), &["check"]);

    let view = ok(dir.path(), &["work", "decisions", &mission]);
    let one_way = view.find("One-way Decisions").expect(&view);
    let reversible = view.find("Reversible Decisions").expect(&view);
    assert!(one_way < reversible, "{view}");
    assert!(
        view.contains(&format!(
            "{work} {} Keep the report in Close Reason | because It is already exempt from approval freshness | rejected a Report section; a sidecar file",
            &record[record.find("- 20").unwrap() + 2..record.find("- 20").unwrap() + 12]
        )),
        "{view}"
    );
    assert!(
        view[reversible..].contains("Name parts with ### | because Records allow it"),
        "{view}"
    );
}

#[test]
fn decision_notes_refuse_missing_reasons_and_stray_flags() {
    let dir = tempdir().unwrap();
    let (_, work, _, _) = mission_fixture(dir.path());
    let stderr = refused(
        dir.path(),
        &["issue", "note", &work, "--kind", "decision", "No reason"],
    );
    assert!(
        stderr.contains("needs its reason; rerun with `--reason"),
        "{stderr}"
    );
    let stderr = refused(
        dir.path(),
        &[
            "issue", "note", &work, "--kind", "decision", "Blank", "--reason", "  ",
        ],
    );
    assert!(stderr.contains("needs its reason"), "{stderr}");
    for flags in [
        &["--reason", "why"][..],
        &["--rejected", "x"],
        &["--one-way"],
    ] {
        let mut args = vec!["issue", "note", &work, "plain"];
        args.extend_from_slice(flags);
        let stderr = refused(dir.path(), &args);
        assert!(
            stderr.contains("apply only to `--kind decision`"),
            "{stderr}"
        );
    }
    assert!(!read_canonical_record(dir.path(), "issues", &work).contains("## Notes"));
}

#[test]
fn check_refuses_malformed_decision_notes() {
    let dir = tempdir().unwrap();
    let (_, work, _, _) = mission_fixture(dir.path());
    ok(
        dir.path(),
        &[
            "issue", "note", &work, "--kind", "decision", "Fine", "--reason", "ok",
        ],
    );
    edit_canonical_issue(dir.path(), &work, |markdown| {
        format!("{}\n- 2026-10-18 [decision] Unreasoned\n- 2026-10-18 [decision maybe] Odd\n  Reason: x\n", markdown.trim_end())
    });
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(!success, "{stdout}");
    let output = format!("{stdout}{stderr}");
    assert!(
        output.contains(&format!(
            "{work}: Decision note 2 (2026-10-18) has no reason"
        )),
        "{output}"
    );
    assert!(
        output.contains(&format!("{work}: Decision note 3 (2026-10-18) tag [decision maybe] is not [decision] or [decision one-way]")),
        "{output}"
    );
    let mission = issue_id_by_title(dir.path(), "Mission");
    let view = ok(dir.path(), &["work", "decisions", &mission]);
    assert!(
        view.contains(&format!(
            "⚠ {work} decision note 2 (2026-10-18) has no reason"
        )),
        "{view}"
    );
}

#[test]
fn decision_notes_never_store_what_check_refuses() {
    let dir = tempdir().unwrap();
    let (_, work, _, _) = mission_fixture(dir.path());
    // A label inside a multi-line choice is folded into the choice line.
    ok(
        dir.path(),
        &[
            "issue",
            "note",
            &work,
            "--kind",
            "decision",
            "Use V\nReason: injected",
            "--reason",
            "R",
        ],
    );
    assert!(read_canonical_record(dir.path(), "issues", &work)
        .contains("[decision] Use V Reason: injected\n  Reason: R\n"));
    for args in [
        &["Reason: leading label", "--reason", "R"][..],
        &["Choice", "--reason", "R", "--rejected", "Rejected: nested"],
    ] {
        let mut full = vec!["issue", "note", &work, "--kind", "decision"];
        full.extend_from_slice(args);
        let stderr = refused(dir.path(), &full);
        assert!(
            stderr.contains("must not start with `Reason:` or `Rejected:`"),
            "{stderr}"
        );
    }
    ok(dir.path(), &["check"]);
}

#[test]
fn long_decision_rows_continue_past_the_rows() {
    let dir = tempdir().unwrap();
    let (mission, work, _, _) = mission_fixture(dir.path());
    ok(
        dir.path(),
        &[
            "issue",
            "note",
            &work,
            "--kind",
            "decision",
            "Choose the long option that needs several words to say",
            "--reason",
            "because the explanation is also long enough to wrap",
        ],
    );
    let (success, view, stderr) = run_atelier_with_env(
        dir.path(),
        &["work", "decisions", &mission],
        &[("COLUMNS", "50")],
    );
    assert!(success, "{stderr}");
    let reversible = view.find("Reversible Decisions").expect(&view);
    let rows = view[reversible..]
        .lines()
        .skip(2)
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();
    assert!(rows.len() > 1, "{view}");
    assert!(rows[0].starts_with(&work), "{view}");
    assert!(
        rows[1..]
            .iter()
            .all(|line| line.starts_with("    ") && line.len() <= 50),
        "{view}"
    );
}

#[test]
fn decision_records_link_an_adr_and_show_supersession() {
    let dir = tempdir().unwrap();
    let (mission, work, old, new) = mission_fixture(dir.path());
    fs::create_dir_all(dir.path().join("docs/adr")).unwrap();
    fs::write(dir.path().join("docs/adr/0001-choice.md"), "# ADR\n").unwrap();

    let stderr = refused(
        dir.path(),
        &["issue", "update", &new, "--adr", "docs/adr/missing.md"],
    );
    assert!(
        stderr.contains("ADR file docs/adr/missing.md does not exist"),
        "{stderr}"
    );
    let stderr = refused(
        dir.path(),
        &["issue", "update", &work, "--adr", "docs/adr/0001-choice.md"],
    );
    assert!(
        stderr.contains("--adr applies to decision records"),
        "{stderr}"
    );
    ok(
        dir.path(),
        &["issue", "update", &new, "--adr", "docs/adr/0001-choice.md"],
    );
    assert!(read_canonical_record(dir.path(), "issues", &new)
        .contains("adr:\n  path: docs/adr/0001-choice.md\n"));
    ok(
        dir.path(),
        &["issue", "link", "-q", &new, &old, "--role", "supersedes"],
    );
    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &old,
            "decide",
            "--reason",
            "Chose the old way",
        ],
    );

    // An undecided newer record does not yet replace the old decision, and
    // is not current itself.
    let view = ok(dir.path(), &["work", "decisions", &mission]);
    assert!(!view.contains("Superseded Decisions"), "{view}");
    let records = view.find("Decision Records\n").expect(&view);
    let open = view.find("Open Decision Records").expect(&view);
    assert!(
        view[records..open].contains(&format!("{old} ✓ done medium Old decision")),
        "{view}"
    );
    assert!(
        view[open..].contains(&format!(
            "{new} todo medium New decision | ADR docs/adr/0001-choice.md | supersedes {old}"
        )),
        "{view}"
    );
    let quiet = ok(dir.path(), &["work", "decisions", &mission, "-q"]);
    assert_eq!(quiet.trim(), old, "an open record is not current");

    ok(
        dir.path(),
        &[
            "issue",
            "transition",
            &new,
            "decide",
            "--reason",
            "Chose the new way",
        ],
    );
    let view = ok(dir.path(), &["work", "decisions", &mission]);
    assert!(!view.contains("Open Decision Records"), "{view}");
    let records = view.find("Decision Records").expect(&view);
    let superseded = view.find("Superseded Decisions").expect(&view);
    assert!(
        view[records..superseded].contains(&format!(
            "{new} ✓ done medium New decision | ADR docs/adr/0001-choice.md | supersedes {old}"
        )),
        "{view}"
    );
    assert!(
        view[superseded..].contains(&format!(
            "{old} ✓ done medium Old decision | superseded by {new}"
        )),
        "{view}"
    );
    let quiet = ok(dir.path(), &["work", "decisions", &mission, "-q"]);
    assert_eq!(
        quiet.trim(),
        new,
        "only current decisions are listed quietly"
    );

    let show = ok(dir.path(), &["issue", "show", &old]);
    assert!(
        show.contains(&format!("Superseded by: {new} ✓ done medium New decision")),
        "{show}"
    );
    let show = ok(dir.path(), &["issue", "show", &new, "--full"]);
    assert!(show.contains("ADR: docs/adr/0001-choice.md"), "{show}");
    assert!(
        show.contains(&format!("Supersedes: {old} ✓ done medium Old decision")),
        "{show}"
    );

    // A deleted ADR file is a check finding, and the type cannot change
    // while the link stands.
    fs::remove_file(dir.path().join("docs/adr/0001-choice.md")).unwrap();
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(!success);
    assert!(
        format!("{stdout}{stderr}").contains(&format!(
            "{new}: Decision ADR docs/adr/0001-choice.md does not exist"
        )),
        "{stdout}{stderr}"
    );
    let stderr = refused(
        dir.path(),
        &["issue", "update", &new, "--issue-type", "task"],
    );
    assert!(stderr.contains("clear it with --no-adr"), "{stderr}");
    ok(dir.path(), &["issue", "update", &new, "--no-adr"]);
    assert!(!read_canonical_record(dir.path(), "issues", &new).contains("adr:"));
}

#[test]
fn work_decisions_refuses_items_that_are_not_missions() {
    let dir = tempdir().unwrap();
    let (mission, work, _, _) = mission_fixture(dir.path());
    let stderr = refused(dir.path(), &["work", "decisions", &work]);
    assert!(stderr.contains("not a mission"), "{stderr}");
    let view = ok(dir.path(), &["work", "decisions", &mission]);
    assert!(view.contains("Decision Records"), "{view}");
    assert!(
        !view.contains("One-way Decisions"),
        "empty sections are left out: {view}"
    );
}

#[test]
fn issue_show_lists_the_current_decisions_bearing_on_an_item() {
    let dir = tempdir().unwrap();
    let (mission, work, old, new) = mission_fixture(dir.path());
    let parent = ok(
        dir.path(),
        &[
            "issue",
            "create",
            "-q",
            "--body",
            "## Goal\n\nP\n\n## Done when\n\nD",
            "Parent",
        ],
    )
    .trim()
    .to_string();
    ok(dir.path(), &["issue", "update", &work, "--parent", &parent]);
    let unrelated = ok(
        dir.path(),
        &[
            "issue",
            "create",
            "-q",
            "--body",
            "## Goal\n\nU\n\n## Done when\n\nD",
            "Sibling",
        ],
    )
    .trim()
    .to_string();
    ok(dir.path(), &["issue", "link", "-q", &unrelated, &mission]);
    let decide = |id: &str, choice: &str, one_way: bool| {
        let mut args = vec![
            "issue", "note", id, "--kind", "decision", choice, "--reason", "because",
        ];
        if one_way {
            args.push("--one-way");
        }
        ok(dir.path(), &args);
    };
    decide(&work, "Item choice", false);
    decide(&parent, "Parent choice", true);
    decide(&mission, "Mission choice", false);
    decide(&unrelated, "Sibling choice", false);
    // The old record is decided, then superseded by a decided newer one;
    // only the newer one is current.
    ok(
        dir.path(),
        &["issue", "link", "-q", &new, &old, "--role", "supersedes"],
    );
    ok(
        dir.path(),
        &["issue", "transition", &old, "decide", "--reason", "Old"],
    );
    ok(
        dir.path(),
        &["issue", "transition", &new, "decide", "--reason", "New"],
    );

    let brief = ok(dir.path(), &["issue", "show", &work]);
    assert!(
        brief.contains("Decisions: 4 current, 1 one-way:\n"),
        "{brief}"
    );
    assert!(
        brief.contains(&format!("\n  {parent} "))
            && brief.contains("Parent choice | because because"),
        "{brief}"
    );
    assert!(
        !brief.contains("Item choice"),
        "brief shows only one-way rows: {brief}"
    );

    let full = ok(dir.path(), &["issue", "show", &work, "--full"]);
    let section = full.split("Decisions\n---------\n").nth(1).expect(&full);
    let rows = section
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 4, "{full}");
    assert!(rows[0].contains("Parent choice"), "one-way first: {full}");
    assert!(
        rows[1].starts_with(&format!("{new} ✓ done medium New decision")),
        "{full}"
    );
    assert!(
        section.contains("Item choice") && section.contains("Mission choice"),
        "{full}"
    );
    assert!(
        !section.contains("Sibling choice"),
        "a sibling's notes do not bear on the item: {full}"
    );
    assert!(
        !section.contains(&format!("{old} ")),
        "superseded records are not current: {full}"
    );

    // An item with no decisions shows no Decisions line.
    let quiet = ok(dir.path(), &["issue", "show", &old]);
    assert!(
        quiet.contains("Decisions: 2 current\n"),
        "the mission's note and the newer record bear on it: {quiet}"
    );
    let lone = ok(
        dir.path(),
        &[
            "issue",
            "create",
            "-q",
            "--body",
            "## Goal\n\nL\n\n## Done when\n\nD",
            "Lone",
        ],
    )
    .trim()
    .to_string();
    assert!(!ok(dir.path(), &["issue", "show", &lone]).contains("Decisions"));
}
