use super::*;

/// Words that name internals rather than CONTEXT.md terms. They may appear
/// only inside a code or configuration key, joined to it by `_` or `.`.
const INTERNAL_WORDS: [&str; 2] = ["lint", "canonical"];

/// Each standalone word of `text`, lowercased. A code such as
/// `canonical_snapshot_uncommitted` or a key such as
/// `prune.canonical_retention_days` stays one word, so it never matches; a
/// hyphen separates words, so `non-canonical` yields `canonical`.
fn words(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '.'))
        .map(|word| word.trim_matches('.').to_lowercase())
}

fn is_snake_code(token: &str) -> bool {
    token.contains('_')
        && token
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

/// Every code in `text`: a snake_case token that leads a clause as
/// `<code>:`, or the `<code>` of a workflow policy error's `error: <code>`
/// line. Counted across all lines, so a code repeated in a `Caused by:` line
/// counts too.
fn codes(text: &str) -> Vec<&str> {
    let tokens = text.split_whitespace().collect::<Vec<_>>();
    tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| match token.strip_suffix(':') {
            Some(code) if is_snake_code(code) => Some(code),
            _ if index > 0 && tokens[index - 1] == "error:" && is_snake_code(token) => Some(*token),
            _ => None,
        })
        .collect()
}

/// The error a failed command printed: stderr from `Error:` to the end,
/// including any `Caused by:` chain. Warnings before it are not the error.
fn error_text(stderr: &str) -> Option<&str> {
    stderr
        .find("Error: ")
        .filter(|start| *start == 0 || stderr[..*start].ends_with('\n'))
        .map(|start| &stderr[start..])
}

/// Asserts the output contract's wording rules (docs/product/human-cli-output.md,
/// Errors) over one command's rendered output.
fn assert_contract_terms(command: &str, stdout: &str, stderr: &str) {
    let transcript = format!("{stdout}{stderr}");
    for word in words(&transcript) {
        assert!(
            !INTERNAL_WORDS.contains(&word.as_str()),
            "`{command}` printed internal word {word:?}:\n{transcript}"
        );
    }
    let Some(error) = error_text(stderr) else {
        return;
    };
    let codes = codes(error);
    assert!(
        codes.len() <= 1,
        "`{command}` stacked codes {codes:?}:\n{error}"
    );
    let steps = error.matches("Next:").count();
    assert!(
        steps <= 1,
        "`{command}` gave more than one recovery step:\n{error}"
    );
    assert!(
        codes.is_empty() || steps == 1,
        "`{command}` refused with code {codes:?} but no `Next:` step:\n{error}"
    );
}

/// Runs each command in `dir` and asserts the contract over its output.
fn render(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let (success, stdout, stderr) = run_atelier_raw(dir, args);
    assert_contract_terms(&args.join(" "), &stdout, &stderr);
    (success, stdout, stderr)
}

fn create(dir: &Path, args: &[&str]) -> String {
    let mut full = vec!["--quiet", "issue", "create"];
    full.extend_from_slice(args);
    let (success, stdout, stderr) = run_atelier(dir, &full);
    assert!(success, "issue create failed: {stderr}");
    stdout.trim().to_string()
}

/// atelier-91w1: rendered views, help and errors use CONTEXT.md terms,
/// and each error has at most one code and one recovery step.
#[test]
fn rendered_output_uses_context_terms_and_one_code_per_error() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let work_body = "## Goal\n\nShip it.\n\n## Done when\n\nIt ships.";
    let mission = create(
        dir.path(),
        &[
            "Ship the release",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nShipped.\n\n## Success criteria\n\nUsers have it.",
        ],
    );
    let scoped = create(dir.path(), &["Write the notes", "--body", work_body]);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", &mission, &scoped, "--role", "advances"],
    );
    assert!(success, "link failed: {stderr}");
    let parent = create(dir.path(), &["Group the work", "--body", work_body]);
    let child = create(
        dir.path(),
        &["Do one part", "--parent", &parent, "--body", work_body],
    );
    let other = create(dir.path(), &["Unrelated work", "--body", work_body]);
    init_git_repo(dir.path());
    commit_all(dir.path(), "fixture records");

    let render = |args: &[&str]| render(dir.path(), args);

    let (success, stdout, stderr) = render(&["check"]);
    assert!(
        success && stdout.contains("Check passed."),
        "{stdout}{stderr}"
    );
    let (_, stdout, _) = render(&["issue", "show", &mission, "--full"]);
    assert!(stdout.contains("Mission Rollup\n"), "{stdout}");
    assert!(stdout.contains("Recently Updated\n"), "{stdout}");
    let (_, stdout, _) = render(&["issue", "show", &parent, "--full"]);
    assert!(stdout.contains("Parent Rollup\n"), "{stdout}");
    render(&["issue", "show", &child]);
    render(&["prune"]);

    // A record that differs from HEAD: land reports its own check once.
    let other_path = canonical_issue_path(dir.path(), &other);
    let committed = fs::read_to_string(&other_path).unwrap();
    fs::write(&other_path, format!("{committed}\nUncommitted line\n")).unwrap();
    let (success, _, stderr) = render(&["land"]);
    assert!(
        !success
            && stderr.contains(&format!(
                "land_records_uncommitted: 1 tracker record differs from HEAD: .atelier/issues/{other}.md (modified)"
            )),
        "{stderr}"
    );

    // Another record's failure in a scoped check is labelled `records`.
    corrupt_issue_title_yaml(dir.path(), &other, "Unrelated work");
    let (success, stdout, stderr) = render(&["check", &child]);
    assert!(!success, "{stdout}");
    assert!(stdout.contains("\n  records: "), "{stdout}");
    assert!(
        stderr.contains(&format!(
            "check_findings: check failed with 1 finding. Next: fix each finding above, then run `atelier check {child}`"
        )),
        "{stderr}"
    );
    let (success, stdout, stderr) = render(&["check"]);
    assert!(
        !success && stdout.contains("Check found 1 finding:"),
        "{stdout}"
    );
}

/// A started work item on its own branch, with every record committed.
fn started_fixture() -> (tempfile::TempDir, String, String) {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let work_body = "## Goal\n\nShip it.\n\n## Done when\n\nIt ships.";
    let mission = create(
        dir.path(),
        &[
            "Ship the release",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nShipped.\n\n## Success criteria\n\nUsers have it.",
        ],
    );
    let work = create(dir.path(), &["Write the notes", "--body", work_body]);
    init_git_repo(dir.path());
    commit_all(dir.path(), "fixture records");
    let (success, _, stderr) = run_atelier_raw(dir.path(), &["start", &work]);
    assert!(success, "start failed: {stderr}");
    commit_all(dir.path(), "start records");
    (dir, mission, work)
}

/// atelier-91w1: merge refusals and an invalid candidate workflow policy
/// each carry one code, counting the `Caused by:` chain, and one `Next:`.
#[test]
fn merge_refusals_and_invalid_policy_carry_one_code_and_one_step() {
    let (dir, mission, work) = started_fixture();
    let render = |args: &[&str]| render(dir.path(), args);

    let (success, stdout, _) = render(&["merge", "--check", &mission]);
    assert!(!success && stdout.contains("merge_mission: "), "{stdout}");
    let unstarted = create(dir.path(), &["Unstarted work"]);
    commit_all(dir.path(), "unstarted record");
    let (success, stdout, _) = render(&["merge", "--check", &unstarted]);
    assert!(
        !success
            && stdout.contains(&format!(
                "merge_owner_unstarted: {unstarted} has no work branch. Next: atelier start {unstarted}"
            )),
        "{stdout}"
    );

    let policy = dir.path().join(".atelier/workflow.yaml");
    let valid = fs::read_to_string(&policy).unwrap();
    fs::write(&policy, format!("{valid}bogus_key: [\n")).unwrap();
    commit_all(dir.path(), "invalid candidate policy");
    // The invalid policy is one failed check: one line, one code, one step
    // (atelier-y0en).
    for args in [vec!["land"], vec!["merge", "--check", work.as_str()]] {
        let (success, stdout, stderr) = render(&args);
        let transcript = format!("{stdout}{stderr}");
        assert!(
            !success && codes(&transcript) == ["workflow_config_parse_error"],
            "{transcript}"
        );
        assert_eq!(
            transcript
                .matches("did not find expected node content")
                .count(),
            1,
            "{transcript}"
        );
        assert!(!transcript.contains("before merging"), "{transcript}");
    }
}

#[test]
fn contract_term_scan_catches_jargon_and_stacked_codes() {
    assert!(words("Lint passed.").any(|word| word == "lint"));
    assert!(words("(canonical): x").any(|word| word == "canonical"));
    assert!(words("a non-canonical record").any(|word| word == "canonical"));
    assert!(words("Canonical-foo").any(|word| word == "canonical"));
    assert!(!words("canonical_snapshot_uncommitted: x").any(|word| word == "canonical"));
    assert!(!words("prune.canonical_retention_days").any(|word| word == "canonical"));
    assert_eq!(
        codes("Error: land_records_uncommitted: canonical_snapshot_uncommitted: x"),
        ["land_records_uncommitted", "canonical_snapshot_uncommitted"]
    );
    let chained = "Error: land_policy_invalid: invalid\n\nCaused by:\n    error: workflow_config_parse_error\n    path: .atelier/workflow.yaml\n";
    assert_eq!(
        codes(chained),
        ["land_policy_invalid", "workflow_config_parse_error"]
    );
    assert_eq!(codes("Health        : ready"), Vec::<&str>::new());
    assert_eq!(
        error_text("warning: x\nError: boom\n\nCaused by:\n    y\n"),
        Some("Error: boom\n\nCaused by:\n    y\n")
    );
}
