//! `atelier check` warns about mechanical plan problems (atelier-edaj): an
//! open work item with no Done when lines or an empty one, and a
//! repository-relative link to a missing file in an issue record or a
//! mission's strategy. Warnings never change the exit code.

use super::*;

fn create(dir: &Path, args: &[&str]) -> String {
    let mut args = args.to_vec();
    args.splice(0..0, ["issue", "create", "-q"]);
    let (success, stdout, stderr) = run_atelier(dir, &args);
    assert!(success, "atelier {args:?} failed: {stderr}");
    stdout.trim().to_string()
}

fn item(dir: &Path, title: &str, body: &str) -> String {
    create(dir, &[title, "--body", body])
}

/// `.atelier/issues/<id>.md:<line>` for the first line of the record
/// equal to `line`.
fn at(dir: &Path, id: &str, line: &str) -> String {
    let path = format!(".atelier/issues/{id}.md");
    let text = fs::read_to_string(dir.join(&path)).unwrap();
    let number = text
        .lines()
        .position(|candidate| candidate == line)
        .unwrap_or_else(|| panic!("{line:?} is not in {path}:\n{text}"));
    format!("{path}:{}", number + 1)
}

fn check(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let mut args = args.to_vec();
    args.insert(0, "check");
    run_atelier(dir, &args)
}

#[test]
fn check_warns_about_plan_problems_without_changing_its_exit_code() {
    let dir = tempdir().unwrap();
    let dir = dir.path();
    init_atelier(dir);
    fs::create_dir_all(dir.join("docs/strategies")).unwrap();
    fs::write(dir.join("docs/design.md"), "# Design\n").unwrap();

    // Records with boxed Done when lines and working links get no warning,
    // and the output is what it was before the checks existed.
    let sound = item(
        dir,
        "Sound",
        "## Goal\n\nShip it.\n\n## Done when\n\n- [ ] The test passes.\n\n## Context\n\nSee [design](docs/design.md#shape), [the record](../../docs/design.md) and [the site](https://example.com/missing.md).",
    );
    fs::write(
        dir.join("docs/strategies/good.md"),
        "---\nmission:\n---\n\n# Good\n\nSee [design](../design.md).\n",
    )
    .unwrap();
    create(
        dir,
        &[
            "Good mission",
            "--issue-type",
            "mission",
            "--strategy",
            "docs/strategies/good.md",
            "--body",
            "## Outcome\n\nIt works.",
        ],
    );
    let (passed, stdout, stderr) = check(dir, &[]);
    assert!(passed, "{stdout}{stderr}");
    assert_eq!(stdout, "Check passed.\n");
    assert_eq!(stderr, "");

    // A Done when without lines, an empty Done when line, a broken link in
    // a record, and one in a strategy a mission links.
    let prose = item(
        dir,
        "Prose",
        "## Goal\n\nShip it.\n\n## Done when\n\nThe tests pass.",
    );
    let empty = item(
        dir,
        "Empty",
        "## Goal\n\nShip it.\n\n## Done when\n\n- [ ] The test passes.\n- [ ]",
    );
    let linked = item(
        dir,
        "Linked",
        "## Goal\n\nShip it.\n\n## Done when\n\n- [ ] The test passes.\n\n## Context\n\nRead [the plan](docs/plan.md) first.",
    );
    fs::write(
        dir.join("docs/strategies/broken.md"),
        "---\nmission:\n---\n\n# Broken\n\nSee [notes](notes.md).\n",
    )
    .unwrap();
    create(
        dir,
        &[
            "Broken mission",
            "--issue-type",
            "mission",
            "--strategy",
            "docs/strategies/broken.md",
            "--body",
            "## Outcome\n\nIt works.",
        ],
    );
    let prose_warning = format!(
        "Warning: {}: Done when has no `- [ ]` lines, so `atelier review request` has nothing to hold the work to. Next: write each result that finishes the work as its own `- [ ] ` line under this heading.",
        at(dir, &prose, "## Done when")
    );
    let empty_warning = format!(
        "Warning: {}: This Done when line has no text. Next: write the observable result that finishes the work after the box, or delete the line.",
        at(dir, &empty, "- [ ]")
    );
    let link_warning = format!(
        "Warning: {}: The link to docs/plan.md points at no file, beside this file or from the repository root. Next: correct the path, or remove the link.",
        at(dir, &linked, "Read [the plan](docs/plan.md) first.")
    );
    let strategy_warning = "Warning: docs/strategies/broken.md:7: The link to notes.md points at no file, beside this file or from the repository root. Next: correct the path, or remove the link.";
    let (passed, stdout, stderr) = check(dir, &[]);
    assert!(passed, "warnings do not fail the check: {stdout}{stderr}");
    let mut lines = stdout.lines().collect::<Vec<_>>();
    assert_eq!(lines.remove(0), "Check passed.");
    lines.sort();
    let mut expected = vec![
        prose_warning.as_str(),
        empty_warning.as_str(),
        link_warning.as_str(),
        strategy_warning,
    ];
    expected.sort();
    assert_eq!(lines, expected, "{stdout}");
    assert!(!stdout.contains(&sound), "{stdout}");

    // Scoped to one item, only its own warnings print.
    let (passed, stdout, _) = check(dir, &[&prose]);
    assert!(passed);
    assert_eq!(stdout, format!("Check passed.\n{prose_warning}\n"));

    // Quiet output keeps stdout to the token; the warnings go to stderr.
    let (passed, stdout, stderr) = check(dir, &["-q"]);
    assert!(passed);
    assert_eq!(stdout, "passed\n");
    assert!(
        stderr.contains(&prose_warning) && stderr.contains(strategy_warning),
        "{stderr}"
    );

    // Closed work items get no Done when warning; their links still count.
    let path = dir.join(format!(".atelier/issues/{prose}.md"));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text.replace("status: \"todo\"", "status: \"done\"")).unwrap();
    let (_, stdout, _) = check(dir, &[]);
    assert!(!stdout.contains(&prose), "{stdout}");
    fs::write(&path, text).unwrap();

    // When check fails for another reason, the warnings still print and
    // the failure is the same.
    fs::remove_file(dir.join("docs/strategies/good.md")).unwrap();
    let (passed, stdout, stderr) = check(dir, &[]);
    assert!(!passed, "{stdout}");
    assert!(
        stdout.starts_with("Check found 1 finding:")
            && stdout.contains("Mission strategy docs/strategies/good.md does not exist"),
        "{stdout}"
    );
    for warning in &expected {
        assert!(stdout.contains(warning), "{warning} missing from {stdout}");
    }
    assert!(
        stderr.contains("check_findings: check failed with 1 finding"),
        "{stderr}"
    );
}
