use super::*;

/// Words that name internals rather than CONTEXT.md terms. They may appear
/// only inside a code or configuration key, joined to it by `_` or `.`.
const INTERNAL_WORDS: [&str; 2] = ["lint", "canonical"];

/// Each standalone word of `text`, lowercased: a code such as
/// `canonical_snapshot_uncommitted` or a key such as
/// `prune.canonical_retention_days` stays one word, so it never matches.
fn words(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '.' || c == '-'))
        .map(|word| word.trim_matches('.').to_lowercase())
}

/// Leading snake_case codes on one line, such as `land_records_uncommitted:`.
fn codes(line: &str) -> Vec<&str> {
    line.split_whitespace()
        .filter_map(|token| token.strip_suffix(':'))
        .filter(|token| {
            token.contains('_')
                && token
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        })
        .collect()
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
    for line in transcript.lines() {
        assert!(
            codes(line).len() <= 1,
            "`{command}` stacked codes on one line: {line}"
        );
    }
    assert!(
        stderr.matches("Next:").count() <= 1,
        "`{command}` gave more than one recovery step:\n{stderr}"
    );
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

    let mut rendered = Vec::new();
    let mut render = |args: &[&str]| -> (bool, String, String) {
        let (success, stdout, stderr) = run_atelier_raw(dir.path(), args);
        rendered.push((args.join(" "), stdout.clone(), stderr.clone()));
        (success, stdout, stderr)
    };

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
    render(&["prune", "--help"]);
    render(&["issue", "create", "--help"]);
    render(&["check", "--help"]);

    // A record that differs from HEAD: land reports its own check once.
    let other_path = canonical_issue_path(dir.path(), &other);
    let committed = fs::read_to_string(&other_path).unwrap();
    fs::write(&other_path, format!("{committed}\nUncommitted line\n")).unwrap();
    let (success, _, stderr) = render(&["land"]);
    assert!(
        !success && stderr.contains("land_records_uncommitted: .atelier/issues/"),
        "{stderr}"
    );

    // Another record's failure in a scoped check is labelled `records`.
    corrupt_issue_title_yaml(dir.path(), &other, "Unrelated work");
    let (success, stdout, stderr) = render(&["check", &child]);
    assert!(!success, "{stdout}");
    assert!(stdout.contains("\n  records: "), "{stdout}");
    assert!(
        stderr.contains(&format!(
            "Check failed with 1 finding(s). Next: fix each finding above, then run `atelier check {child}`"
        )),
        "{stderr}"
    );
    let (success, stdout, stderr) = render(&["check"]);
    assert!(
        !success && stdout.contains("Check found 1 finding(s):"),
        "{stdout}"
    );

    for (command, stdout, stderr) in &rendered {
        assert_contract_terms(command, stdout, stderr);
    }
}

#[test]
fn contract_term_scan_catches_jargon_and_stacked_codes() {
    assert!(words("Lint passed.").any(|word| word == "lint"));
    assert!(words("(canonical): x").any(|word| word == "canonical"));
    assert!(!words("canonical_snapshot_uncommitted: x").any(|word| word == "canonical"));
    assert!(!words("prune.canonical_retention_days").any(|word| word == "canonical"));
    assert_eq!(
        codes("Error: land_records_uncommitted: canonical_snapshot_uncommitted: x"),
        ["land_records_uncommitted", "canonical_snapshot_uncommitted"]
    );
    assert_eq!(codes("Health        : ready"), Vec::<&str>::new());
}
