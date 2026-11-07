//! One issue's whole lifecycle under the `atelier init` starter policy: the
//! scenario of mission atelier-mmpv's `lifecycle` success criterion
//! (docs/strategies/simpler-lifecycle.md).
//!
//! Start; a note, a link and the ticked Done when; a review with one
//! `fix_now` and one `defer` finding; a fix commit; the fix's re-review and
//! approval; a note and a link edited after approval; merge; then `land` on
//! the base and `doctor`. Every `atelier` command passes, the one approval
//! is the fix's re-review, and the merge commit marks the issue done. The
//! test prints how many `atelier` commands the author and the reviewer ran
//! (run it with `--no-capture` to see them); it pins no count, so a change
//! that adds a step shows in that line and in this scenario's diff.
//!
//! The repository starts from `git init` on `main` or on `master`, and its
//! setup follows the steps `atelier init` prints, with no edit to the policy
//! it writes (atelier-l457).
use std::{cell::Cell, fs, path::Path};
use tempfile::tempdir;
#[path = "support/cli.rs"]
mod support_cli;
use support_cli::{commit, git, ok};

const AUTHOR: &str = "Author";
const REVIEWER: &str = "Reviewer";
const WORK: &str = "## Goal\n\nShip the change.\n\n## Done when\n\n- [ ] The scenario passes.";

/// Who runs an `atelier` command, and how many each has run.
struct Actors<'r> {
    repo: &'r Path,
    planner: Cell<usize>,
    author: Cell<usize>,
    reviewer: Cell<usize>,
}
impl Actors<'_> {
    fn run(&self, count: &Cell<usize>, args: &[&str]) -> String {
        count.set(count.get() + 1);
        ok(self.repo, args)
    }
    fn planner(&self, args: &[&str]) -> String {
        self.run(&self.planner, args)
    }
    fn author(&self, args: &[&str]) -> String {
        self.run(&self.author, args)
    }
    fn reviewer(&self, args: &[&str]) -> String {
        self.run(&self.reviewer, args)
    }
}

#[test]
fn one_issue_goes_from_start_to_merge_with_no_refusal_and_one_approval() {
    lifecycle("main");
}

/// The same scenario from `git init` on `master`: init detects the branch,
/// so the starter policy merges as written.
#[test]
fn the_lifecycle_merges_from_a_git_init_on_master_with_no_policy_edit() {
    lifecycle("master");
}

/// The next steps a command's footer prints, as `(label, command)`.
fn next_steps(output: &str) -> Vec<(String, String)> {
    output
        .split_once("\nNext\n----\n")
        .map(|(_, footer)| footer)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| line.trim().split_once(": "))
        .map(|(label, command)| (label.to_owned(), command.to_owned()))
        .collect()
}

fn lifecycle(base: &str) {
    let dir = tempdir().unwrap();
    let repo = dir.path();
    let actors = Actors {
        repo,
        planner: Cell::new(0),
        author: Cell::new(0),
        reviewer: Cell::new(0),
    };

    // A repository with the starter policy, set up as `init` says: review
    // the policy, which names the detected base, commit the install on the
    // base, then create the first issue on it.
    git(repo, &["init", "-qb", base]);
    git(repo, &["config", "user.name", AUTHOR]);
    git(repo, &["config", "user.email", "author@example.test"]);
    let init = actors.planner(&["init", "--slug", "atelier"]);
    assert!(
        init.contains(&format!("Base branch: {base} (from the current branch)")),
        "{init}"
    );
    let steps = next_steps(&init);
    let [(review, policy), (commit_label, install), (create, first_issue)] = steps.as_slice()
    else {
        panic!("init prints three next steps: {init}");
    };
    assert!(review.starts_with("Review the generated policy"), "{init}");
    let written = fs::read_to_string(repo.join(policy)).unwrap();
    assert!(
        written.contains(&format!("branch_policy:\n  base_branch: {base}\n")),
        "{written}"
    );
    assert!(
        commit_label.starts_with(&format!("Commit the install on {base}")),
        "{init}"
    );
    let committed = std::process::Command::new("sh")
        .current_dir(repo)
        .args(["-c", install])
        .output()
        .unwrap();
    assert!(committed.status.success(), "{install}: {committed:?}");
    assert_eq!(git(repo, &["status", "--porcelain"]), "");
    assert_eq!(create, "Create the first issue");
    assert!(first_issue.starts_with("atelier issue create "), "{init}");
    let id = actors.planner(&["-q", "issue", "create", "Change", "--body", WORK]);
    let context = actors.planner(&["-q", "issue", "create", "Context", "--body", WORK]);
    commit(repo, "Plan the change");

    // The author starts the issue on the base, which creates its branch,
    // implements it, notes and links it, ticks its Done when and writes the
    // implementer report.
    actors.author(&["start", &id]);
    assert_eq!(git(repo, &["branch", "--show-current"]), id);
    fs::write(repo.join("src.rs"), "first version\n").unwrap();
    commit(repo, "Implement the change");
    actors.author(&["issue", "note", &id, "Implemented in src.rs"]);
    actors.author(&["issue", "link", &id, &context, "--role", "related"]);
    let path = repo.join(format!(".atelier/issues/{id}.md"));
    let record = fs::read_to_string(&path).unwrap();
    assert!(record.contains("- [ ] The scenario passes."), "{record}");
    fs::write(
        &path,
        format!(
            "{}\n## Report\n\nShipped the change.\n",
            record.replace("- [ ] The scenario passes.", "- [x] The scenario passes.")
        ),
    )
    .unwrap();
    commit(repo, "Tick the Done when and report");
    actors.author(&["review", "request", &id]);

    // The reviewer raises one fix_now and one defer finding, and does not
    // approve.
    let finding = |recommendation: &str, summary: &str| {
        actors.reviewer(&[
            "-q",
            "review",
            "finding",
            &id,
            "--identity",
            REVIEWER,
            "--recommendation",
            recommendation,
            "--commit-records",
            summary,
        ])
    };
    let fix_now = finding("fix_now", "src.rs:1 reads the wrong input");
    let defer = finding("defer", "src.rs:1 could share a helper");

    // The author fixes the fix_now finding, which needs no disposition, and
    // defers the other to a follow-up.
    fs::write(repo.join("src.rs"), "fixed version\n").unwrap();
    commit(repo, "Fix the finding");
    let follow_up = actors.author(&["-q", "issue", "create", "Share a helper", "--body", WORK]);
    commit(repo, "File the follow-up");
    actors.author(&[
        "review",
        "disposition",
        &defer,
        "--identity",
        AUTHOR,
        "--decision",
        "defer",
        "--follow-up",
        &follow_up,
        "--commit-records",
        "A helper is its own change",
    ]);
    actors.author(&["review", "request", &id]);

    // The reviewer re-reviews the fix and approves.
    actors.reviewer(&[
        "review",
        "approve",
        &id,
        "--identity",
        REVIEWER,
        "--commit-records",
        "Reviewed the fix and the change",
    ]);

    // A note and a link edited after approval keep it current.
    actors.author(&["issue", "note", &id, "Handoff after approval"]);
    actors.author(&["issue", "link", &id, &follow_up, "--role", "related"]);
    commit(repo, "Note and link after approval");
    let tip = git(repo, &["rev-parse", "HEAD"]);

    let merged = actors.author(&["merge", &id]);
    assert!(merged.ends_with("merge: passed"), "{merged}");
    assert_eq!(git(repo, &["branch", "--show-current"]), base);
    let landed = actors.author(&["land"]);
    assert!(landed.ends_with("land: passed"), "{landed}");
    let doctor = actors.author(&["doctor"]);
    assert!(doctor.contains("Doctor: every done claim"), "{doctor}");

    // The merge commit marks the issue done; the branch it merged did not.
    let status_at = |commit: &str| {
        let text = git(
            repo,
            &["show", &format!("{commit}:.atelier/issues/{id}.md")],
        );
        atelier_records::parse_issue_record(&text, Path::new(&format!("issues/{id}.md")))
            .unwrap()
            .issue
            .status
    };
    assert_eq!(git(repo, &["rev-parse", &format!("{base}^2")]), tip);
    assert_eq!(status_at(&tip), "in_progress");
    assert_eq!(status_at(base), "done");
    // The one approval is the fix's re-review, and the fix_now finding
    // carries no disposition.
    let reviews = repo.join(".atelier/reviews");
    let approvals = fs::read_dir(&reviews)
        .unwrap()
        .map(|entry| fs::read_to_string(entry.unwrap().path()).unwrap())
        .filter(|text| text.contains("\n  event: approval\n"))
        .count();
    assert_eq!(approvals, 1);
    let fixed = fs::read_to_string(reviews.join(format!("{fix_now}.md"))).unwrap();
    assert!(!fixed.contains("disposition:"), "{fixed}");

    println!(
        "lifecycle on {base}: the author ran {} atelier commands and the reviewer {} ({} together); planning ran {}",
        actors.author.get(),
        actors.reviewer.get(),
        actors.author.get() + actors.reviewer.get(),
        actors.planner.get()
    );
}
