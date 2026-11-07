//! Proof has a time (ADR 0027): the scenario that mission atelier-n4mj's
//! `proof-timing` criterion names (docs/strategies/lighter-workflow.md). An
//! item with one Done when check before merge and one after takes the
//! after-merge path, and nothing on it refuses.
use std::{fs, path::Path};
use tempfile::tempdir;
#[path = "support/cli.rs"]
mod support_cli;
use support_cli::{cli, commit, git, ok};

const AUTHOR: &str = "Author";
const REVIEWER: &str = "Reviewer";
const LATE: &str = "- [ ] After merge (device): the launcher opens on the phone. Stand-in: the emulator test passes.";
const BODY: &str = "## Goal\n\nShip the launcher.\n\n## Done when\n\n- The launcher test passes.\n- [ ] After merge (device): the launcher opens on the phone. Stand-in: the emulator test passes.";

/// A repository with the starter policy and a pushed base, so `land` judges
/// every later base commit.
fn repository(repo: &Path, upstream: &Path) {
    git(repo, &["init", "-qb", "main"]);
    git(repo, &["config", "user.name", AUTHOR]);
    git(repo, &["config", "user.email", "author@example.test"]);
    ok(repo, &["init", "--slug", "atelier"]);
    commit(repo, "Initialize Atelier");
    git(upstream, &["init", "-q", "--bare"]);
    git(
        repo,
        &["remote", "add", "origin", upstream.to_str().unwrap()],
    );
    git(repo, &["push", "-q", "-u", "origin", "main"]);
}

fn record_path(repo: &Path, id: &str) -> std::path::PathBuf {
    repo.join(".atelier/issues").join(format!("{id}.md"))
}

fn edit(repo: &Path, id: &str, from: &str, to: &str) {
    let path = record_path(repo, id);
    let text = fs::read_to_string(&path).unwrap();
    assert!(text.contains(from), "{id} lacks {from:?}:\n{text}");
    fs::write(path, text.replacen(from, to, 1)).unwrap();
}

/// The lines of `page`'s section headed `title`, up to the blank line that
/// ends it.
fn section<'a>(page: &'a str, title: &str) -> Vec<&'a str> {
    page.lines()
        .skip_while(|line| line.trim() != title)
        .skip(1)
        .take_while(|line| !line.trim().is_empty())
        .collect()
}

/// Closes `mission` in the working tree, returns the output, and puts the
/// records back.
fn close_and_restore(repo: &Path, mission: &str) -> String {
    let output = ok(
        repo,
        &["issue", "transition", mission, "close", "--reason", "Done."],
    );
    git(repo, &["checkout", "-q", "--", ".atelier"]);
    git(repo, &["clean", "-qfd", ".atelier"]);
    output
}

#[test]
fn a_check_after_merge_is_briefed_shown_until_validated_and_refuses_nothing() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());

    // A started mission and one planned item with a check after merge.
    let mission = ok(
        repo,
        &[
            "-q",
            "issue",
            "create",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nThe launcher ships.",
            "Launcher",
        ],
    );
    let item = ok(repo, &["-q", "issue", "create", "Launcher", "--body", BODY]);
    ok(repo, &["issue", "link", "-q", &item, &mission]);
    commit(repo, "Plan the launcher");
    ok(repo, &["issue", "transition", &mission, "ready"]);
    ok(repo, &["issue", "transition", &mission, "start"]);
    commit(repo, "Start the mission");
    git(repo, &["push", "-q"]);

    ok(repo, &["start", &item]);
    commit(repo, "Start the launcher");
    fs::write(repo.join("launcher.rs"), "fn main() {}\n").unwrap();
    commit(repo, "Implement the launcher");
    ok(
        repo,
        &[
            "issue",
            "transition",
            &item,
            "close",
            "--reason",
            "Shipped; the phone check waits for the merge.",
            "--commit-records",
        ],
    );

    // The brief lists the check with its reason and stand-in, and says when
    // the reason is missing.
    let brief = ok(repo, &["review", "request", &item]);
    let listed = |brief: &str| {
        brief
            .lines()
            .filter(|line| line.starts_with(&format!("- {item}: ")))
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    assert!(brief.contains("\n## After merge\n"), "{brief}");
    assert_eq!(
        listed(&brief),
        [format!(
            "- {item}: the launcher opens on the phone. | reason: device | stand-in: the emulator test passes."
        )],
        "{brief}"
    );
    assert!(
        brief.contains(
            "Record a finding for any that could be checked before merge, or that lacks a reason or a stand-in."
        ),
        "{brief}"
    );
    edit(repo, &item, "After merge (device):", "After merge:");
    let brief = ok(repo, &["review", "request", &item]);
    assert_eq!(
        listed(&brief),
        [format!(
            "- {item}: the launcher opens on the phone. | reason missing | stand-in: the emulator test passes."
        )],
        "{brief}"
    );
    git(repo, &["checkout", "-q", "--", ".atelier"]);

    git(repo, &["config", "user.name", REVIEWER]);
    let head = git(repo, &["rev-parse", "HEAD"]);
    ok(
        repo,
        &[
            "review",
            "approve",
            &item,
            "--identity",
            REVIEWER,
            "--commit",
            &head,
            "--commit-records",
            "Reviewed the launcher and its after-merge check",
        ],
    );
    git(repo, &["config", "user.name", AUTHOR]);
    let output = ok(repo, &["merge", &item]);
    assert!(output.contains("merge: passed"), "{output}");
    git(repo, &["switch", "-q", "main"]);
    git(repo, &["push", "-q"]);

    // Merged, it awaits validation in status and the work views.
    let awaiting = |page: &str| {
        section(page, "Awaiting Validation")
            .iter()
            .any(|line| line.contains(&item))
    };
    for args in [
        vec!["status"],
        vec!["work", "all"],
        vec!["work", "mission", &mission],
    ] {
        let page = ok(repo, &args);
        assert!(awaiting(&page), "{args:?}:\n{page}");
    }

    // Closing the mission now warns and names it, and still closes.
    let output = close_and_restore(repo, &mission);
    assert!(
        output.contains(&format!(
            "Warning: {mission} closes with validation still open: {item} (1 open after-merge check)."
        )) && output.contains("To:       done"),
        "{output}"
    );

    // A failed validation files a follow-up bug and carries the check to it;
    // the mission then closes without the warning.
    let bug = ok(
        repo,
        &[
            "-q",
            "issue",
            "create",
            "The launcher does not open on the phone",
            "--issue-type",
            "bug",
            "--discovered-from",
            &item,
            "--body",
            "## Goal\n\nOpen on the phone.\n\n## Done when\n\n- The fix's test passes.",
        ],
    );
    edit(repo, &item, LATE, &format!("{LATE} Carried to {bug}."));
    let output = close_and_restore(repo, &mission);
    assert!(!output.contains("Warning"), "{output}");

    // "carried to" naming no issue carries nothing: the check stays open.
    edit(repo, &item, LATE, &format!("{LATE} Carried to the phone."));
    let output = close_and_restore(repo, &mission);
    assert!(output.contains("Warning"), "{output}");

    // Validation passes: ticked on the base as a planning change, with no
    // reviewer identity. `land` accepts it and the item stops awaiting.
    edit(repo, &item, "- [ ] After merge", "- [x] After merge");
    commit(repo, &format!("Validate {item} on the phone"));
    let (landed, output) = cli(repo, &["land"]);
    assert!(landed, "{output}");
    for args in [vec!["status"], vec!["work", "all"]] {
        let page = ok(repo, &args);
        assert!(!awaiting(&page), "{args:?}:\n{page}");
    }
    let debrief = ok(repo, &["debrief", &mission]);
    assert!(
        !debrief.contains("Done when edited") && !debrief.contains("Awaiting validation"),
        "{debrief}"
    );
    let output = close_and_restore(repo, &mission);
    assert!(!output.contains("Warning"), "{output}");
}

#[test]
fn a_check_moved_to_after_merge_once_started_is_shown_to_the_reviewer() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());
    let planned = "## Goal\n\nShip the launcher.\n\n## Done when\n\n- The launcher test passes.\n- The launcher opens on the phone.";
    let item = ok(
        repo,
        &["-q", "issue", "create", "Launcher", "--body", planned],
    );
    commit(repo, "Plan the launcher");
    ok(repo, &["start", &item]);
    commit(repo, "Start the launcher");

    // Before the move nothing is shown.
    let brief = ok(repo, &["review", "request", &item]);
    assert!(!brief.contains("Done when edited"), "{brief}");

    edit(
        repo,
        &item,
        "- The launcher opens on the phone.",
        "- [ ] After merge (device): the launcher opens on the phone.",
    );
    commit(repo, "Move the phone check after merge");
    let moved = git(repo, &["rev-parse", "--short=8", "HEAD"]);
    let line = format!("{item} Done when edited at {moved} after the item was started.");

    let brief = ok(repo, &["review", "request", &item]);
    let edits = brief
        .lines()
        .filter(|text| text.contains("Done when edited at"))
        .collect::<Vec<_>>();
    assert!(
        brief.contains("\n## Done when edited after start\n")
            && edits == [format!("- {line}")]
            && brief.contains("| reason: device | stand-in missing"),
        "{brief}"
    );
    let (_, output) = cli(repo, &["merge", &item, "--check"]);
    assert!(output.contains(&format!("Note: {line}")), "{output}");

    // Ticking the check while rewriting it is an edit too, and the brief
    // marks a check ticked before the merge.
    edit(
        repo,
        &item,
        "- [ ] After merge (device): the launcher opens on the phone.",
        "- [x] After merge (device): the launcher compiles.",
    );
    commit(repo, "Tick the phone check and loosen it");
    let loosened = git(repo, &["rev-parse", "--short=8", "HEAD"]);
    let brief = ok(repo, &["review", "request", &item]);
    assert!(
        brief.contains(&format!(
            "- {item} Done when edited at {loosened} after the item was started."
        )) && brief.contains("| reason: device | stand-in missing | ticked before merge"),
        "{brief}"
    );
}
