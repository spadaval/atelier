//! Review events read without opening record files (atelier-phfo): the
//! rows, state and finding tree of `review list`, the brief `review show`,
//! the confirmation each recording prints, generated titles, and the shared
//! invalid-value form for commits and numbers.

use super::*;

const AUTHOR: &str = "Atelier Test";

fn ok(dir: &Path, args: &[&str]) -> String {
    let (success, stdout, stderr) = run_atelier_raw(dir, args);
    assert!(success, "atelier {args:?} failed: {stderr}");
    stdout
}

fn git(dir: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success(), "git {args:?} failed");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn short(commit: &str) -> &str {
    &commit[..8]
}

/// Records an event quietly and returns its ID.
fn event(dir: &Path, args: &[&str]) -> String {
    let mut quiet = vec!["-q", "review"];
    quiet.extend_from_slice(args);
    ok(dir, &quiet).trim().to_owned()
}

/// A started owner with one child and one code commit after the start.
fn owner(dir: &Path) -> (String, String) {
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
    (id, child)
}

/// The value columns of a list row, split on its two-space separators.
fn columns(row: &str) -> Vec<&str> {
    row.split("  ").collect()
}

#[test]
fn review_list_shows_rows_in_time_order_then_approval_state_and_finding_tree() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    let (id, child) = owner(dir);
    let pin = git(dir, &["rev-parse", "HEAD"]);

    let fixed = event(
        dir,
        &[
            "finding",
            &id,
            "--identity",
            "Reviewer R",
            "--recommendation",
            "fix_now",
            "parse() ignores input\n\nIt should read the tokens.",
        ],
    );
    let named = event(
        dir,
        &[
            "finding",
            &child,
            "--identity",
            "Reviewer R",
            "--recommendation",
            "no_action",
            "Name could be better",
        ],
    );
    let open = event(
        dir,
        &[
            "finding",
            &id,
            "--identity",
            "Reviewer R",
            "--recommendation",
            "defer",
            "No error type",
        ],
    );
    commit_all(dir, "findings");
    fs::write(dir.join("parser.rs"), "fn parse(_: &str) {}\n").unwrap();
    commit_all(dir, "read the input");
    let resolution = git(dir, &["rev-parse", "HEAD"]);
    let fix = event(
        dir,
        &[
            "disposition",
            &fixed,
            "--identity",
            "Reviewer R",
            "--decision",
            "fix_now",
            "--resolution-commit",
            &resolution,
            "Reads the input now",
        ],
    );
    let first = event(
        dir,
        &[
            "disposition",
            &named,
            "--identity",
            "Reviewer R",
            "--decision",
            "no_action",
            "Fine",
        ],
    );
    let second = event(
        dir,
        &[
            "disposition",
            &named,
            "--identity",
            "Reviewer R",
            "--decision",
            "no_action",
            "Fine, the name is the domain term",
        ],
    );
    let approval = event(
        dir,
        &["approve", &id, "--identity", "Reviewer R", "Reviewed all"],
    );
    commit_all(dir, "review");

    let listed = ok(dir, &["review", "list", &id]);
    for debug in ["Some(", "None", "{"] {
        assert!(!listed.contains(debug), "{debug}: {listed}");
    }
    let lines = listed.lines().collect::<Vec<_>>();
    let order = [&fixed, &named, &open, &fix, &first, &second, &approval];
    for (line, event) in lines.iter().zip(order) {
        assert!(line.starts_with(&format!("{event}  ")), "{listed}");
    }
    let row = columns(lines[0]);
    assert_eq!(
        (row[1], row[2], row[3], row[5]),
        (
            "finding fix_now",
            &*format!("@{}", short(&pin)),
            "Reviewer R",
            "parse() ignores input"
        ),
        "the owner column is dropped and the summary cut to its first line: {listed}"
    );
    assert_eq!(row[4].len(), "2026-10-23".len(), "{listed}");
    assert_eq!(
        columns(lines[1])[1..3],
        ["finding no_action", &*format!("on {child}")],
        "an event on a child names it: {listed}"
    );
    assert_eq!(
        columns(lines[3])[1..3],
        [
            &*format!("disposition fix_now → {fixed}"),
            &*format!("@{}", short(&resolution))
        ],
        "{listed}"
    );
    assert_eq!(lines[7], "", "{listed}");
    assert_eq!(
        lines[8..],
        [
            format!(
                "Approval: ✓ current — {approval} @{} by Reviewer R",
                short(&resolution)
            ),
            "Findings: 2 disposed, 1 open".to_owned(),
            format!("  ⚠ {open} defer, no disposition  No error type"),
            format!(
                "  ✓ {fixed} fix_now → fix_now {fix}, fixed at {}  parse() ignores input",
                short(&resolution)
            ),
            format!(
                "  ✓ {named} no_action → no_action {first} → no_action {second}  Name could be better"
            ),
        ],
        "{listed}"
    );

    // A short ID and the child both show the owner's view; quiet prints IDs.
    assert_eq!(ok(dir, &["review", "list", issue_key(&id)]), listed);
    assert_eq!(
        ok(dir, &["review", "list", &child]),
        format!("{child} is reviewed with its branch owner {id}.\n{listed}")
    );
    assert_eq!(
        ok(dir, &["-q", "review", "list", &id]),
        order.map(|id| format!("{id}\n")).concat()
    );

    // A later code commit makes the approval stale, and names the commit.
    fs::write(dir.join("extra.rs"), "fn extra() {}\n").unwrap();
    commit_all(dir, "add extra");
    let extra = git(dir, &["rev-parse", "HEAD"]);
    let listed = ok(dir, &["review", "list", &id]);
    assert!(
        listed.contains(&format!(
            "\nApproval: ⚠ stale — {approval} @{} by Reviewer R; changed since by 1 commit:\n  {} add extra: extra.rs\nFindings:",
            short(&resolution),
            short(&extra)
        )),
        "{listed}"
    );
}

#[test]
fn review_list_without_events_or_approval_says_so_and_unknown_ids_fail() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    let (id, _) = owner(dir);
    assert_eq!(
        ok(dir, &["review", "list", &id]),
        format!("No review events for {id}.\n")
    );
    event(
        dir,
        &["record", &id, "--identity", "Reviewer R", "Read the parser"],
    );
    let listed = ok(dir, &["review", "list", &id]);
    assert!(
        listed
            .ends_with("\n\nApproval: ⚠ missing — no independent approval or trivial exemption\n"),
        "an account is no approval, and no finding prints no Findings line: {listed}"
    );
    let (success, stdout, stderr) = run_atelier_raw(dir, &["review", "list", "doesnotexist"]);
    assert!(
        !success && stdout.is_empty() && stderr.contains("issue doesnotexist not found"),
        "{stderr}"
    );
}

#[test]
fn review_show_is_a_brief_view_with_linked_events() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    let (id, _) = owner(dir);
    let pin = git(dir, &["rev-parse", "HEAD"]);
    let finding = event(
        dir,
        &[
            "finding",
            &id,
            "--identity",
            "Reviewer R",
            "--recommendation",
            "no_action",
            "Name could be better\n\nparse is vague.",
        ],
    );
    let first = event(
        dir,
        &[
            "disposition",
            issue_key(&finding),
            "--identity",
            "Reviewer R",
            "--decision",
            "no_action",
            "Fine",
        ],
    );
    let second = event(
        dir,
        &[
            "disposition",
            &finding,
            "--identity",
            "Reviewer S",
            "--decision",
            "no_action",
            "Domain term",
        ],
    );
    let approval = event(
        dir,
        &["approve", &id, "--identity", "Reviewer R", "Looks right"],
    );
    commit_all(dir, "review");

    let shown = ok(dir, &["review", "show", issue_key(&finding)]);
    let header = format!(
        "{finding} finding no_action on {id} @{} by Reviewer R | ",
        short(&pin)
    );
    assert!(shown.starts_with(&header), "{shown}");
    assert!(
        shown.ends_with(&format!(
            "\nDispositions:\n  → no_action {first} by Reviewer R: Fine\n  → no_action {second} by Reviewer S: Domain term\n\nName could be better\n\nparse is vague.\n"
        )),
        "{shown}"
    );
    assert_eq!(shown.matches("Name could be better").count(), 1, "{shown}");
    for raw in ["{", "\"labels\"", "created_at"] {
        assert!(!shown.contains(raw), "{raw}: {shown}");
    }

    let shown = ok(dir, &["review", "show", &second]);
    assert!(
        shown.starts_with(&format!(
            "{second} disposition no_action → {finding} on {id} @{} by Reviewer S | ",
            short(&pin)
        )) && shown.ends_with(&format!(
            "\nFinding: {finding} no_action by Reviewer R: Name could be better\nSupersedes: {first}\n\nDomain term\n"
        )),
        "{shown}"
    );

    let shown = ok(dir, &["review", "show", &approval]);
    assert!(
        shown.starts_with(&format!(
            "{approval} approval on {id} @{} by Reviewer R | ",
            short(&pin)
        )) && shown.ends_with(" | ✓ current\n\nLooks right\n"),
        "{shown}"
    );
    assert_eq!(
        ok(dir, &["-q", "review", "show", &approval]),
        format!("{approval} approval\n")
    );
}

#[test]
fn recording_confirms_what_was_recorded_and_warns_on_an_author_identity() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    let (id, _) = owner(dir);
    let pin = git(dir, &["rev-parse", "HEAD"]);
    let at = format!("on {id} @{}", short(&pin));
    let record = |args: &[&str]| {
        let mut full = vec!["review"];
        full.extend_from_slice(args);
        let stdout = ok(dir, &full);
        let new = stdout
            .split_whitespace()
            .find(|word| word.starts_with(&id[..id.find('-').unwrap() + 1]) && *word != id)
            .unwrap()
            .trim_end_matches([':', ',', '.'])
            .to_owned();
        (stdout, new)
    };

    let (out, account) = record(&["record", &id, "--identity", "Reviewer R", "Read it"]);
    assert_eq!(out, format!("Review account {account} {at}\n"));
    let (out, finding) = record(&[
        "finding",
        &id,
        "--identity",
        "Reviewer R",
        "--recommendation",
        "fix_now",
        "parse() ignores input\nmore",
    ]);
    assert_eq!(
        out,
        format!(
            "Finding {finding} fix_now {at}. Dispose: atelier review disposition {finding} --identity \"...\" --decision fix_now --resolution-commit ... \"...\"\n"
        )
    );
    let (out, disposition) = record(&[
        "disposition",
        &finding,
        "--identity",
        AUTHOR,
        "--decision",
        "no_action",
        "Not a problem",
    ]);
    assert_eq!(
        out,
        format!(
            "Disposition {disposition} no_action {at}: finding {finding} \"parse() ignores input\"\n⚠ identity matches a Git author; this will not satisfy findings.disposed\n"
        )
    );
    let (out, successor) = record(&[
        "disposition",
        &finding,
        "--identity",
        "Reviewer R",
        "--decision",
        "no_action",
        "Accepted",
    ]);
    assert_eq!(
        out,
        format!(
            "Disposition {successor} no_action {at}: finding {finding} \"parse() ignores input\", supersedes {disposition}\n"
        )
    );
    let (out, approval) = record(&[
        "approve",
        &id,
        "--identity",
        "atelier-test@example.com",
        "Mine",
    ]);
    assert_eq!(
        out,
        format!(
            "Approval {approval} {at}\n⚠ identity matches a Git author; this will not satisfy review.current\n"
        )
    );
    let (out, approval) = record(&["approve", &id, "--identity", "Reviewer R", "Good"]);
    assert_eq!(out, format!("Approval {approval} {at}\n"));
    let (out, exemption) = record(&["trivial", &id, "--identity", AUTHOR, "Evident"]);
    assert_eq!(out, format!("Trivial exemption {exemption} {at}\n"));

    // Quiet prints the ID alone; the warning goes to stderr.
    let (success, stdout, stderr) = run_atelier_raw(
        dir,
        &["-q", "review", "approve", &id, "--identity", AUTHOR, "Mine"],
    );
    assert!(success && is_record_id(stdout.trim()), "{stdout}");
    assert!(
        stderr.contains("⚠ identity matches a Git author"),
        "{stderr}"
    );

    let (success, _, stderr) = run_atelier_raw(
        dir,
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer R",
            "--decision",
            "defer",
            "Later",
        ],
    );
    assert!(
        !success && stderr.contains("DEFER requires --follow-up <issue-id>"),
        "{stderr}"
    );
}

#[test]
fn new_review_titles_do_not_repeat_the_body() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    let (id, _) = owner(dir);
    let pin = git(dir, &["rev-parse", "HEAD"]);
    let body = "parse() ignores input";
    let finding = event(
        dir,
        &[
            "finding",
            &id,
            "--identity",
            "Reviewer R",
            "--recommendation",
            "fix_now",
            body,
        ],
    );
    let text = read_canonical_record(dir, "reviews", &finding);
    assert!(
        text.contains(&format!("\ntitle: finding on {id} @{}\n", short(&pin))),
        "{text}"
    );
    assert_eq!(text.matches(body).count(), 1, "{text}");
    let approval = event(dir, &["approve", &id, "--identity", "Reviewer R", "Good"]);
    let text = read_canonical_record(dir, "reviews", &approval);
    assert!(
        text.contains(&format!("\ntitle: approval on {id} @{}\n", short(&pin))),
        "{text}"
    );
}

#[test]
fn invalid_commits_and_numbers_use_the_shared_invalid_value_form() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    let (id, _) = owner(dir);
    let exit = |args: &[&str]| {
        let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
            .current_dir(dir)
            .args(args)
            .output()
            .unwrap();
        (
            output.status.code(),
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        )
    };
    for (args, message) in [
        (
            vec!["review", "record", &id, "--identity", "r", "--commit", "zzz", "x"],
            "Error: Invalid commit 'zzz'. Valid values: full 40- or 64-character commit IDs",
        ),
        (
            vec![
                "review",
                "disposition",
                "zzzz",
                "--identity",
                "r",
                "--decision",
                "fix_now",
                "--resolution-commit",
                "abc",
                "x",
            ],
            "Error: Invalid resolution commit 'abc'. Valid values: full 40- or 64-character commit IDs",
        ),
        (
            vec!["prune", "--retention-days", "abc"],
            "Error: Invalid retention days 'abc'. Valid values: whole numbers of days",
        ),
        (
            vec!["prune", "--retention-days", "-3"],
            "Error: Invalid retention days '-3'. Valid values: whole numbers of days",
        ),
        (
            vec!["issue", "list", "--limit", "-3"],
            "Error: Invalid limit '-3'. Valid values: positive integers",
        ),
    ] {
        assert_eq!(exit(&args), (Some(2), message.to_owned()), "{args:?}");
    }

    let unknown = "0123456789abcdef0123456789abcdef01234567";
    let (code, stderr) = exit(&[
        "review",
        "record",
        &id,
        "--identity",
        "r",
        "--commit",
        unknown,
        "x",
    ]);
    assert_eq!(code, Some(1), "{stderr}");
    assert_eq!(
        stderr,
        format!("Error: commit {unknown} is not on this branch: Git does not know it, or HEAD does not contain it. Next: git log --format=%H")
    );
    assert!(!stderr.contains("merge-base"), "{stderr}");
}
