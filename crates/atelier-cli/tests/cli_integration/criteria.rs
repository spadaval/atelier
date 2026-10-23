//! Mission success criteria and the receipts that judge them (atelier-ho7o,
//! atelier-stsa, decision atelier-et6q).

use super::*;

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

/// A mission whose own Success criteria section defines three criteria,
/// one work item advancing it, and one item outside every mission.
fn fixture(dir: &Path) -> (String, String, String) {
    init_atelier(dir);
    let mission = ok(
        dir,
        &[
            "issue", "create", "-q", "--issue-type", "mission", "--body",
            "## Outcome\n\nOutput is safe to pipe.\n\n## Success criteria\n\n- pipe-safe: Redirected output has no styling. Shown by a test.\n- quiet-tokens: Quiet output prints one token\n  per command.\n- S3: Legacy positional criterion.",
            "Mission",
        ],
    )
    .trim()
    .to_string();
    let item = |title: &str| {
        ok(
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
        .to_string()
    };
    let (work, outside) = (item("Work"), item("Outside"));
    ok(dir, &["issue", "link", "-q", &work, &mission]);
    (mission, work, outside)
}

fn record(dir: &Path, target: &str, result: &str, criteria: &[&str], summary: &str) -> String {
    let target = format!("issue/{target}");
    let mut args = vec![
        "evidence",
        "record",
        "-q",
        "--target",
        &target,
        "--kind",
        "validation",
        "--result",
        result,
    ];
    for criterion in criteria {
        args.extend(["--criterion", criterion]);
    }
    args.push(summary);
    ok(dir, &args).trim().to_string()
}

#[test]
fn receipts_record_and_show_a_result_per_criterion() {
    let dir = tempdir().unwrap();
    let (_, work, _) = fixture(dir.path());
    let id = record(
        dir.path(),
        &work,
        "fail",
        &["pipe-safe=pass", "quiet-tokens=fail"],
        "One run judged both",
    );
    let text = read_canonical_record(dir.path(), "evidence", &id);
    assert!(
        text.contains("criteria:\n- slug: \"pipe-safe\"\n  result: \"pass\"\n- slug: \"quiet-tokens\"\n  result: \"fail\"\ncriteria_mission: \""),
        "{text}"
    );
    let show = ok(dir.path(), &["evidence", "show", &id]);
    assert!(
        show.contains("Criteria: pipe-safe pass, quiet-tokens fail"),
        "{show}"
    );
    ok(dir.path(), &["check"]);
}

#[test]
fn receipts_refuse_criteria_their_mission_does_not_define() {
    let dir = tempdir().unwrap();
    let (mission, work, outside) = fixture(dir.path());
    let output = refused(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{work}"),
            "--kind",
            "validation",
            "--result",
            "pass",
            "--criterion",
            "pipe-saef=pass",
            "Typo",
        ],
    );
    assert!(
        output.contains(&format!(
            "names criterion pipe-saef, which mission {mission} does not define"
        )),
        "{output}"
    );
    let output = refused(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{outside}"),
            "--kind",
            "validation",
            "--result",
            "pass",
            "--criterion",
            "pipe-safe=pass",
            "Outside",
        ],
    );
    assert!(
        output.contains(&format!("its target {outside} is in no mission")),
        "{output}"
    );
    let output = refused(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "--criterion",
            "pipe-safe=pass",
            "No target",
        ],
    );
    assert!(
        output.contains("--criterion needs an issue --target"),
        "{output}"
    );
    for (criterion, expected) in [
        ("pipe-safe", "needs a result: `<slug>=<result>`"),
        ("Pipe=pass", "is not a criterion slug"),
        (
            "pipe-safe=maybe",
            "result must be pass, fail, blocked, or not-applicable",
        ),
    ] {
        let output = refused(
            dir.path(),
            &[
                "evidence",
                "record",
                "--target",
                &format!("issue/{work}"),
                "--kind",
                "validation",
                "--result",
                "pass",
                "--criterion",
                criterion,
                "Bad",
            ],
        );
        assert!(output.contains(expected), "{criterion}: {output}");
    }
    let output = refused(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{work}"),
            "--kind",
            "validation",
            "--result",
            "pass",
            "--criterion",
            "S3=pass",
            "--criterion",
            "S3=fail",
            "Twice",
        ],
    );
    assert!(output.contains("names S3 more than once"), "{output}");
    // A mission names its own criteria too.
    record(
        dir.path(),
        &mission,
        "pass",
        &["S3=pass"],
        "On the mission itself",
    );
    assert!(!evidence_tree_contains_summary(dir.path(), "Typo"));
}

fn evidence_tree_contains_summary(dir: &Path, summary: &str) -> bool {
    fs::read_dir(dir.join(".atelier/evidence"))
        .map(|entries| {
            entries.flatten().any(|entry| {
                fs::read_to_string(entry.path()).is_ok_and(|text| text.contains(summary))
            })
        })
        .unwrap_or(false)
}

#[test]
fn captured_receipts_give_each_criterion_the_command_result() {
    let dir = tempdir().unwrap();
    let (_, work, _) = fixture(dir.path());
    let target = format!("issue/{work}");
    let id = ok(
        dir.path(),
        &[
            "evidence",
            "record",
            "-q",
            "--target",
            &target,
            "--kind",
            "test",
            "--criterion",
            "pipe-safe",
            "--",
            "false",
        ],
    )
    .trim()
    .to_string();
    assert!(ok(dir.path(), &["evidence", "show", &id]).contains("Criteria: pipe-safe fail"));
    let output = refused(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &target,
            "--kind",
            "test",
            "--criterion",
            "pipe-safe=pass",
            "--",
            "true",
        ],
    );
    assert!(
        output.contains("derives each criterion's result from the exit status"),
        "{output}"
    );
    let output = refused(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &target,
            "--kind",
            "test",
            "--criterion",
            "nope",
            "--",
            "true",
        ],
    );
    assert!(output.contains("the command was not executed"), "{output}");
}

#[test]
fn check_refuses_receipts_naming_undefined_or_renamed_criteria() {
    let dir = tempdir().unwrap();
    let (mission, work, _) = fixture(dir.path());
    let id = record(
        dir.path(),
        &work,
        "pass",
        &["quiet-tokens=pass"],
        "Quiet tokens shown",
    );
    ok(dir.path(), &["check"]);
    // Renaming the slug after proof exists orphans the proof, visibly.
    edit_canonical_issue(dir.path(), &mission, |markdown| {
        markdown.replace("- quiet-tokens:", "- quiet-output:")
    });
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    assert!(!success, "{stdout}");
    assert!(
        format!("{stdout}{stderr}").contains(&format!(
            "{id}: Evidence names criterion quiet-tokens, which mission {mission} does not define"
        )),
        "{stdout}{stderr}"
    );
    let view = ok(dir.path(), &["work", "mission", &mission]);
    assert!(
        view.contains(&format!(
            "⚠ {id} names criterion quiet-tokens, which this mission does not define"
        )),
        "{view}"
    );
}

#[test]
fn work_mission_marks_each_criterion_from_its_newest_receipt() {
    let dir = tempdir().unwrap();
    let (mission, work, _) = fixture(dir.path());
    record(
        dir.path(),
        &work,
        "pass",
        &["pipe-safe=pass", "quiet-tokens=pass"],
        "First run",
    );
    std::thread::sleep(std::time::Duration::from_millis(20));
    let failing = record(
        dir.path(),
        &work,
        "fail",
        &["quiet-tokens=fail"],
        "Second run",
    );
    let view = ok(dir.path(), &["work", "mission", &mission]);
    assert!(
        view.contains("Criteria: 3 criteria: 1 proven, 1 failing, 1 unproven"),
        "{view}"
    );
    let section = view
        .split("Success Criteria\n----------------\n")
        .nth(1)
        .expect(&view);
    let rows = section
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 3, "{view}");
    assert!(
        rows[0].starts_with("pipe-safe ✓ proven Redirected output has no styling. | atelier-"),
        "{view}"
    );
    assert!(
        !rows[0].contains("Shown by a test"),
        "only the first sentence: {view}"
    );
    assert!(
        rows[1].starts_with(&format!("quiet-tokens ⚠ failing Quiet output prints one token per command. | {failing} fail at ")),
        "{view}"
    );
    assert_eq!(
        rows[2], "S3 unproven Legacy positional criterion.",
        "{view}"
    );

    // A newer pass proves it again, and a code commit since shows.
    std::thread::sleep(std::time::Duration::from_millis(20));
    let passing = record(
        dir.path(),
        &work,
        "pass",
        &["quiet-tokens=pass"],
        "Third run",
    );
    fs::write(dir.path().join("code.txt"), "change\n").unwrap();
    commit_all(dir.path(), "code after the proof");
    let view = ok(dir.path(), &["work", "mission", &mission]);
    assert!(
        view.contains(&format!(
            "quiet-tokens ✓ proven Quiet output prints one token per command. | {passing} pass at "
        )),
        "{view}"
    );
    assert!(view.contains("| 1 code commit since"), "{view}");
    assert!(
        view.contains("Criteria: 3 criteria: 2 proven (2 before later code commits), 1 unproven"),
        "{view}"
    );
    // Filtered views leave the criteria out.
    let ready = ok(dir.path(), &["work", "mission", &mission, "--ready"]);
    assert!(!ready.contains("Success Criteria"), "{ready}");
}

#[test]
fn work_mission_reads_criteria_from_the_linked_strategy() {
    let dir = tempdir().unwrap();
    let (mission, work, _) = fixture(dir.path());
    fs::create_dir_all(dir.path().join("docs/strategies")).unwrap();
    fs::write(
        dir.path().join("docs/strategies/s.md"),
        "# Why\n\nBecause.\n\n# Success Criteria\n\n- fast-start: Starts in a second.\n- a loose bullet\n\n# Shape\n\n- not-a-criterion: here\n",
    )
    .unwrap();
    ok(
        dir.path(),
        &[
            "issue",
            "update",
            &mission,
            "--strategy",
            "docs/strategies/s.md",
        ],
    );
    let view = ok(dir.path(), &["work", "mission", &mission]);
    assert!(view.contains("Criteria: 1 criterion: 1 unproven"), "{view}");
    assert!(
        view.contains("fast-start unproven Starts in a second."),
        "{view}"
    );
    assert!(
        !view.contains("pipe-safe unproven"),
        "the strategy replaces the mission's section: {view}"
    );
    assert!(
        view.contains("⚠ bullet `- a loose bullet` does not start with a criterion slug"),
        "{view}"
    );
    assert!(!view.contains("not-a-criterion"), "{view}");
    record(dir.path(), &work, "pass", &["fast-start=pass"], "Fast");
    assert!(ok(dir.path(), &["work", "mission", &mission]).contains("fast-start ✓ proven"));
}

#[test]
fn proof_stays_with_the_mission_it_was_recorded_against() {
    let dir = tempdir().unwrap();
    let (mission, work, _) = fixture(dir.path());
    let early = record(
        dir.path(),
        &work,
        "pass",
        &["S3=pass"],
        "Before the second mission",
    );
    assert!(read_canonical_record(dir.path(), "evidence", &early)
        .contains(&format!("criteria_mission: \"{mission}\"\n")));
    let other = ok(
        dir.path(),
        &[
            "issue",
            "create",
            "-q",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nOther.\n\n## Success criteria\n\n- S3: Another mission's S3.",
            "Other mission",
        ],
    )
    .trim()
    .to_string();
    // While the item is in both missions, new proof on it is ambiguous.
    ok(dir.path(), &["issue", "link", "-q", &work, &other]);
    let output = refused(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{work}"),
            "--kind",
            "validation",
            "--result",
            "pass",
            "--criterion",
            "S3=pass",
            "Ambiguous",
        ],
    );
    assert!(
        output.contains("target the mission whose criteria it judges"),
        "{output}"
    );
    // Moved to the other mission, the item takes none of its old proof along.
    ok(dir.path(), &["issue", "unlink", "-q", &work, &mission]);
    let first = ok(dir.path(), &["work", "mission", &mission]);
    assert!(
        first.contains(&format!(
            "S3 ✓ proven Legacy positional criterion. | {early} pass at "
        )),
        "{first}"
    );
    let second = ok(dir.path(), &["work", "mission", &other]);
    assert!(
        second.contains("S3 unproven Another mission's S3."),
        "{second}"
    );
    assert!(!second.contains(&early), "{second}");
    ok(dir.path(), &["check"]);
    // Proof recorded on the mission itself is unambiguous.
    record(
        dir.path(),
        &other,
        "pass",
        &["S3=pass"],
        "On the other mission",
    );
    assert!(ok(dir.path(), &["work", "mission", &other]).contains("S3 ✓ proven"));
}

#[test]
fn blocked_and_not_applicable_receipts_leave_a_criterion_unproven() {
    let dir = tempdir().unwrap();
    let (mission, work, _) = fixture(dir.path());
    let blocked = record(
        dir.path(),
        &work,
        "blocked",
        &["pipe-safe=blocked", "S3=not-applicable"],
        "Could not run",
    );
    let view = ok(dir.path(), &["work", "mission", &mission]);
    assert!(
        view.contains(&format!(
            "pipe-safe unproven Redirected output has no styling. | {blocked} blocked at "
        )),
        "{view}"
    );
    assert!(
        view.contains(&format!(
            "S3 unproven Legacy positional criterion. | {blocked} not-applicable at "
        )),
        "{view}"
    );
    assert!(view.contains("Criteria: 3 criteria: 3 unproven"), "{view}");
}

#[test]
fn a_receipt_pinned_outside_head_history_does_not_decide() {
    let dir = tempdir().unwrap();
    let (mission, work, _) = fixture(dir.path());
    let kept = record(dir.path(), &work, "pass", &["pipe-safe=pass"], "On main");
    commit_all(dir.path(), "record the receipt on main");
    let git = |args: &[&str]| {
        let status = Command::new("git")
            .current_dir(dir.path())
            .args(args)
            .status()
            .unwrap();
        assert!(status.success(), "git {args:?}");
    };
    let main = String::from_utf8(
        Command::new("git")
            .current_dir(dir.path())
            .args(["branch", "--show-current"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim()
    .to_string();
    git(&["switch", "-q", "-c", "side"]);
    fs::write(dir.path().join("side.txt"), "side\n").unwrap();
    commit_all(dir.path(), "side code");
    std::thread::sleep(std::time::Duration::from_millis(20));
    let stray = record(
        dir.path(),
        &work,
        "fail",
        &["pipe-safe=fail"],
        "On the side branch",
    );
    let path = dir.path().join(format!(".atelier/evidence/{stray}.md"));
    let text = fs::read_to_string(&path).unwrap();
    git(&["switch", "-q", &main]);
    fs::write(&path, text).unwrap();
    commit_all(dir.path(), "bring a receipt pinned on the side branch");
    let view = ok(dir.path(), &["work", "mission", &mission]);
    assert!(
        view.contains(&format!(
            "pipe-safe ✓ proven Redirected output has no styling. | {kept} pass at "
        )),
        "{view}"
    );
}

#[test]
fn scoped_check_of_a_mission_covers_receipts_in_its_scope() {
    let dir = tempdir().unwrap();
    let (mission, work, _) = fixture(dir.path());
    let id = record(dir.path(), &work, "pass", &["S3=pass"], "S3 shown");
    edit_canonical_issue(dir.path(), &mission, |markdown| {
        markdown.replace("- S3:", "- legacy:")
    });
    // An unrelated issue finding does not hide the receipt's.
    edit_canonical_issue(dir.path(), &work, |markdown| {
        markdown.replace("labels: []", "labels:\n- \"size-q\"")
    });
    let (success, stdout, stderr) = run_atelier(dir.path(), &["check", &mission]);
    assert!(!success, "{stdout}");
    let output = format!("{stdout}{stderr}");
    assert!(
        output.contains(&format!(
            "{id}: Evidence names criterion S3, which mission {mission} does not define"
        )),
        "{output}"
    );
    let (_, stdout, stderr) = run_atelier(dir.path(), &["check"]);
    let output = format!("{stdout}{stderr}");
    assert!(
        output.contains("size-q") && output.contains(&format!("{id}: Evidence names criterion S3")),
        "{output}"
    );
}

#[test]
fn work_mission_needs_no_git_history() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let mission = ok(
        dir.path(),
        &[
            "issue",
            "create",
            "-q",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nO\n\n## Success criteria\n\n- fast: Fast.",
            "Mission",
        ],
    )
    .trim()
    .to_string();
    assert!(!dir.path().join(".git").exists());
    let view = ok(dir.path(), &["work", "mission", &mission]);
    assert!(view.contains("fast unproven Fast."), "{view}");
}
