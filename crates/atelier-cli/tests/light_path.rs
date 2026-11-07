//! The light path (ADR 0028 decision 6, atelier-4nsl): a branch no issue
//! owns, such as a release, docs or a mockup, is reviewed and merged on an
//! independent approval of its head, under the same merge checks, with no
//! issue, start or Report around it. Its review records name the branch and
//! count only for its current run; an owned branch keeps the issue path; an
//! approval of a branch covers no done claim.
use std::{fs, path::Path};
use tempfile::tempdir;
#[path = "support/cli.rs"]
mod support_cli;
use support_cli::{cli, commit, git, ok};

const AUTHOR: &str = "Author";
const REVIEWER: &str = "Reviewer";

/// A repository with the starter policy and a pushed base, so `land` on
/// the base judges every later base commit.
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

/// Writes `text` to `file` and commits it as `message`.
fn change(repo: &Path, file: &str, text: &str, message: &str) {
    fs::write(repo.join(file), text).unwrap();
    commit(repo, message);
}

/// Records an approval of the checked-out `branch` at HEAD and commits it.
fn approve(repo: &Path, branch: &str) -> String {
    ok(
        repo,
        &[
            "-q",
            "review",
            "approve",
            "--branch",
            branch,
            "--identity",
            REVIEWER,
            "--commit-records",
            "Read every commit on the branch",
        ],
    )
    .lines()
    .next()
    .unwrap()
    .to_owned()
}

/// Records a finding about the checked-out `branch` at HEAD and commits it.
fn finding(repo: &Path, branch: &str, recommendation: &str, summary: &str) -> String {
    ok(
        repo,
        &[
            "-q",
            "review",
            "finding",
            "--branch",
            branch,
            "--identity",
            REVIEWER,
            "--recommendation",
            recommendation,
            "--commit-records",
            summary,
        ],
    )
    .lines()
    .next()
    .unwrap()
    .to_owned()
}

fn review_text(repo: &Path, id: &str) -> String {
    fs::read_to_string(repo.join(format!(".atelier/reviews/{id}.md"))).unwrap()
}

fn issue_path(repo: &Path, id: &str) -> std::path::PathBuf {
    repo.join(format!(".atelier/issues/{id}.md"))
}

/// Ticks `id`'s Done when and writes its Report; the caller commits.
fn finish(repo: &Path, id: &str) {
    let path = issue_path(repo, id);
    let text = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, format!("{text}\n## Report\n\nDone.\n")).unwrap();
}

/// Creates an issue with one Done when line and returns its ID.
fn issue(repo: &Path, title: &str) -> String {
    ok(
        repo,
        &[
            "-q",
            "issue",
            "create",
            title,
            "--body",
            "## Goal\n\nShip it.\n\n## Done when\n\n- [ ] It ships.",
        ],
    )
}

/// The status of `id` on `main`.
fn status_on_main(repo: &Path, id: &str) -> String {
    let text = git(repo, &["show", &format!("main:.atelier/issues/{id}.md")]);
    atelier_records::parse_issue_record(&text, Path::new(&format!("issues/{id}.md")))
        .unwrap()
        .issue
        .status
}

#[test]
fn review_request_briefs_the_commits_of_a_branch_no_issue_owns() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());

    // The base is never a light-path branch.
    let (passed, output) = cli(repo, &["review", "request"]);
    assert!(
        !passed && output.contains("review_request_base: main is the base branch"),
        "{output}"
    );

    git(repo, &["switch", "-qc", "docs"]);
    let (passed, output) = cli(repo, &["review", "request"]);
    assert!(
        !passed && output.contains("review_request_empty: docs has no commits since it left main"),
        "{output}"
    );
    let fork = git(repo, &["rev-parse", "HEAD"]);
    change(
        repo,
        "GUIDE.md",
        "# Guide\n\nHow to use it.\n",
        "Write the guide",
    );
    change(repo, "FAQ.md", "# FAQ\n", "Start the FAQ");
    let head = git(repo, &["rev-parse", "HEAD"]);

    let brief = ok(repo, &["review", "request", "--identity", REVIEWER]);
    for expected in [
        "# Review request: branch docs",
        "## Range",
        "on `docs`, forked from `main`",
        "(2 commits)",
        &format!("- Head, to pin records to: `{head}`"),
        "## Commits",
        "Write the guide",
        "Start the FAQ",
        "## Diff stat (whole branch)",
        "GUIDE.md | 3 +++",
        "2 files changed",
        "## Findings so far\n\nNone.",
        "## Working rules",
        "Stay read-only on code",
        &format!("atelier review finding --branch docs --identity \"{REVIEWER}\" --commit {head}"),
        &format!("atelier review approve --branch docs --identity \"{REVIEWER}\" --commit {head}"),
    ] {
        assert!(brief.contains(expected), "missing {expected:?}:\n{brief}");
    }
    // No issue states requirements for the branch.
    assert!(
        !brief.contains("## Goal") && !brief.contains("## Done when"),
        "{brief}"
    );
    assert_eq!(
        ok(repo, &["-q", "review", "request"]),
        format!("{fork}..{head}")
    );

    // A finding shows in the next brief, and the range starts at the last
    // approval once there is one.
    let found = finding(repo, "docs", "fix_now", "high: GUIDE.md is vague");
    let brief = ok(repo, &["review", "request"]);
    assert!(
        brief.contains(&format!(
            "{found} fix_now | open until an approval after its fix"
        )),
        "{brief}"
    );
    change(
        repo,
        "GUIDE.md",
        "# Guide\n\nRun `tool`.\n",
        "Make the guide concrete",
    );
    let approval = approve(repo, "docs");
    change(repo, "FAQ.md", "# FAQ\n\nQ: why?\n", "Answer a question");
    let brief = ok(repo, &["review", "request"]);
    assert!(
        brief.contains(&format!(
            "since the last approval by {REVIEWER} ({approval})"
        )) && brief.contains(&format!("{found} fix_now | approved since by {approval}"))
            && brief.contains(&format!("{approval} approval @")),
        "{brief}"
    );

    // On a branch an open issue owns, the request with no ID is the owner's.
    git(repo, &["switch", "-q", "main"]);
    let owner = issue(repo, "Owned work");
    commit(repo, "Plan the owned work");
    ok(repo, &["start", &owner]);
    change(
        repo,
        "owned.rs",
        "fn owned() {}\n",
        "Implement the owned work",
    );
    finish(repo, &owner);
    commit(repo, "Tick and report");
    let brief = ok(repo, &["review", "request"]);
    assert!(
        brief.contains(&format!("# Review request: {owner} "))
            && brief.contains("## Goal")
            && brief.contains("## Done when"),
        "{brief}"
    );
}

#[test]
fn an_owner_done_on_the_base_no_longer_owns_its_branch_name() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());
    let owner = issue(repo, "Earlier work");
    commit(repo, "Plan the earlier work");
    ok(repo, &["start", &owner]);
    change(
        repo,
        "earlier.rs",
        "fn earlier() {}\n",
        "Implement the earlier work",
    );
    finish(repo, &owner);
    commit(repo, "Tick and report");
    ok(
        repo,
        &[
            "review",
            "approve",
            &owner,
            "--identity",
            REVIEWER,
            "--commit-records",
            "Reviewed",
        ],
    );
    ok(repo, &["merge", &owner]);
    assert_eq!(status_on_main(repo, &owner), "done");

    // The same name again, from the base: the record that names it is done
    // there, so the branch is on the light path.
    git(repo, &["switch", "-qc", &owner]);
    change(
        repo,
        "later.rs",
        "fn later() {}\n",
        "Follow up on the earlier work",
    );
    let brief = ok(repo, &["review", "request"]);
    assert!(
        brief.contains(&format!("# Review request: branch {owner}")),
        "{brief}"
    );
}

#[test]
fn review_records_about_a_branch_count_only_for_its_current_run() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());
    git(repo, &["switch", "-qc", "release"]);
    let fork = git(repo, &["rev-parse", "HEAD"]);
    change(repo, "VERSION", "1\n", "Bump the version to 1");

    // A record about a branch is written on that branch, after it left the
    // base.
    for (args, expected) in [
        (
            vec!["--branch", "other"],
            "review_branch_not_checked_out: this checkout is on release",
        ),
        (
            vec!["--branch", "release", "--commit", fork.as_str()],
            "review_branch_pin:",
        ),
        (vec!["--branch", "two words"], "is not a branch name"),
    ] {
        let mut all = vec!["review", "approve", "--identity", REVIEWER];
        all.extend(args);
        all.push("Approved");
        let (passed, output) = cli(repo, &all);
        assert!(!passed && output.contains(expected), "{output}");
    }
    // There is no trivial exemption of a branch.
    let (passed, _) = cli(
        repo,
        &[
            "review",
            "trivial",
            "--branch",
            "release",
            "--identity",
            REVIEWER,
            "Trivial",
        ],
    );
    assert!(!passed);

    let found = finding(
        repo,
        "release",
        "no_action",
        "low: VERSION has no newline policy",
    );
    ok(
        repo,
        &[
            "review",
            "disposition",
            &found,
            "--identity",
            AUTHOR,
            "--decision",
            "no_action",
            "--commit-records",
            "Fine as it is",
        ],
    );
    let approval = approve(repo, "release");
    // The records name the branch, at the newer schema version.
    for id in [&found, &approval] {
        let text = review_text(repo, id);
        assert!(
            text.contains("schema_version: 3\n")
                && text.contains("branch: release\n")
                && !text.contains("target:"),
            "{text}"
        );
    }
    assert!(review_text(repo, &found).contains("decision: no_action"));
    let listed = ok(repo, &["review", "list"]);
    assert!(
        listed.contains(&format!("{approval}  approval  on branch release")),
        "{listed}"
    );
    let shown = ok(repo, &["review", "show", &approval]);
    assert!(shown.contains("approval on branch release"), "{shown}");
    ok(repo, &["merge"]);
    ok(repo, &["check"]);

    // A later branch with the same name inherits neither the approval nor
    // the finding: they are behind the point where it left the base.
    git(repo, &["switch", "-qc", "release"]);
    change(repo, "VERSION", "2\n", "Bump the version to 2");
    let brief = ok(repo, &["review", "request"]);
    assert!(
        brief.contains("## Findings so far\n\nNone.")
            && !brief.contains(&approval)
            && !brief.contains(&found),
        "{brief}"
    );
    let (passed, output) = cli(repo, &["merge"]);
    assert!(
        !passed && output.contains("review.current: no independent approval of this branch"),
        "{output}"
    );

    // An approval goes stale only on a code change; a record change keeps it.
    let approval = approve(repo, "release");
    ok(
        repo,
        &[
            "review",
            "record",
            "--branch",
            "release",
            "--identity",
            REVIEWER,
            "--commit-records",
            "Read the notes again",
        ],
    );
    ok(repo, &["merge", "--check"]);
    change(repo, "VERSION", "2.0\n", "Spell out the version");
    let (passed, output) = cli(repo, &["merge", "--check"]);
    assert!(
        !passed && output.contains(&format!("approval {approval}")) && output.contains("is stale"),
        "{output}"
    );

    // Approvals stay as first committed.
    let fresh = approve(repo, "release");
    let path = repo.join(format!(".atelier/reviews/{fresh}.md"));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text.replace(REVIEWER, "Someone else")).unwrap();
    commit(repo, "Rewrite the approval");
    let (passed, output) = cli(repo, &["merge", "--check"]);
    assert!(!passed && output.contains("review_append_only"), "{output}");
}

#[test]
fn merge_takes_a_branch_no_issue_owns_on_an_approval_of_its_head() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());
    // The same merge checks as for an issue run on the merged tree.
    let policy = repo.join(".atelier/workflow.yaml");
    let text = fs::read_to_string(&policy).unwrap();
    fs::write(
        &policy,
        format!("{text}merge_checks:\n  - shell.check:\n      command: \"test ! -f BROKEN\"\n"),
    )
    .unwrap();
    commit(repo, "Check that nothing is broken");
    git(repo, &["push", "-q"]);

    git(repo, &["switch", "-qc", "mockup"]);
    change(repo, "mockup.html", "<p>draft</p>\n", "Draft the mockup");
    let base = git(repo, &["rev-parse", "main"]);
    let (passed, output) = cli(repo, &["merge"]);
    assert!(
        !passed
            && output.contains("review.current")
            && output.contains("Next: atelier review request"),
        "{output}"
    );
    assert_eq!(git(repo, &["rev-parse", "main"]), base);

    // Findings are judged as for an issue.
    let fix = finding(repo, "mockup", "fix_now", "high: mockup.html has no title");
    let later = finding(repo, "mockup", "defer", "low: no dark theme");
    change(
        repo,
        "mockup.html",
        "<title>Mockup</title>\n<p>draft</p>\n",
        "Title the mockup",
    );
    approve(repo, "mockup");
    let (passed, output) = cli(repo, &["merge", "--check"]);
    assert!(
        !passed
            && output.contains("findings.disposed")
            && output.contains(&format!("{later} defer finding"))
            && !output.contains(&format!("{fix} fix_now")),
        "{output}"
    );
    let follow_up = issue(repo, "Dark theme");
    ok(
        repo,
        &[
            "review",
            "disposition",
            &later,
            "--identity",
            AUTHOR,
            "--decision",
            "defer",
            "--follow-up",
            &follow_up,
            "--commit-records",
            "Later",
        ],
    );
    git(repo, &["add", "."]);
    git(repo, &["commit", "-qm", "File the follow-up"]);

    // The merge checks hold the light path too.
    change(repo, "BROKEN", "", "Break the build");
    approve(repo, "mockup");
    let (passed, output) = cli(repo, &["merge", "--check"]);
    assert!(!passed && output.contains("shell.check"), "{output}");
    git(repo, &["rm", "-q", "BROKEN"]);
    git(repo, &["commit", "-qm", "Unbreak the build"]);
    approve(repo, "mockup");

    // A sync works with no ID, and keeps the approval across a clean merge
    // of the base.
    git(repo, &["switch", "-q", "main"]);
    issue(repo, "Unrelated plan");
    commit(repo, "Plan something unrelated");
    git(repo, &["push", "-q"]);
    git(repo, &["switch", "-q", "mockup"]);
    let output = ok(repo, &["merge", "--sync"]);
    assert!(
        output.contains("Merging main into mockup.")
            && output.contains("still covers the branch")
            && output.contains("Next: atelier merge --check"),
        "{output}"
    );

    let head = git(repo, &["rev-parse", "HEAD"]);
    let base = git(repo, &["rev-parse", "main"]);
    let output = ok(repo, &["merge"]);
    assert!(
        output.contains("Merged mockup into main with merge commit") && !output.contains("marks"),
        "{output}"
    );
    // A merge commit of the base and the branch head that writes no issue
    // state: its tree is Git's own merge of the two.
    assert_eq!(git(repo, &["rev-parse", "main^1"]), base);
    assert_eq!(git(repo, &["rev-parse", "main^2"]), head);
    let merged = git(repo, &["merge-tree", "--write-tree", &base, &head]);
    assert_eq!(git(repo, &["rev-parse", "main^{tree}"]), merged);
    assert!(!git(repo, &["log", "-1", "--format=%B", "main"]).contains("Marks"));
    // land on the base counts the branch's commits as reviewed, and check
    // and doctor accept its records.
    let output = ok(repo, &["land"]);
    assert!(
        output.contains("land: passed") && !output.contains("land_base_unreviewed"),
        "{output}"
    );
    ok(repo, &["check"]);
    ok(repo, &["doctor"]);
}

#[test]
fn an_owned_branch_keeps_the_issue_path_and_a_branch_approval_covers_no_done_claim() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());
    let owner = issue(repo, "The feature");
    let other = issue(repo, "Another item");
    commit(repo, "Plan the work");
    git(repo, &["push", "-q"]);

    // A branch reviewed on the light path, then taken by `start`: its
    // records about the branch stop counting, and it follows the issue path.
    git(repo, &["switch", "-qc", "feature"]);
    change(
        repo,
        "feature.rs",
        "fn feature() {}\n",
        "Implement the feature",
    );
    let light = approve(repo, "feature");
    let output = ok(repo, &["start", &owner]);
    assert!(
        output.contains("1 light-path review record about the branch itself, which stop counting"),
        "{output}"
    );
    commit(repo, "Start the feature");
    let (passed, output) = cli(
        repo,
        &[
            "review",
            "approve",
            "--branch",
            "feature",
            "--identity",
            REVIEWER,
            "Approved",
        ],
    );
    assert!(
        !passed
            && output.contains(&format!(
                "review_branch_owned: feature is the work branch of {owner}"
            )),
        "{output}"
    );
    // With no ID, merge and review request act on the owner, and the
    // branch's own approval counts for nothing.
    let (passed, output) = cli(repo, &["review", "request"]);
    assert!(
        !passed && output.contains(&format!("review_request_unfinished: asking for review means the work is done to the best of your knowledge, and {owner}'s open work is not")),
        "{output}"
    );
    let (passed, output) = cli(repo, &["merge", "--check"]);
    assert!(
        !passed
            && output.contains("review.current")
            && output.contains(&format!("Next: atelier review request {owner}"))
            && !output.contains(&light),
        "{output}"
    );
    finish(repo, &owner);
    commit(repo, "Tick and report");
    let approval = ok(
        repo,
        &[
            "-q",
            "review",
            "approve",
            &owner,
            "--identity",
            REVIEWER,
            "--commit-records",
            "Reviewed the feature",
        ],
    );
    let approval = approval.lines().next().unwrap();
    // A new approval of an issue is still written at schema version 2, so
    // a binary that predates the light path reads it.
    let text = review_text(repo, approval);
    assert!(
        text.contains("schema_version: 2\n") && text.contains(&format!("target: {owner}\n")),
        "{text}"
    );
    let output = ok(repo, &["merge"]);
    assert!(
        output.contains(&format!("which marks {owner} done")),
        "{output}"
    );
    ok(repo, &["land"]);
    git(repo, &["push", "-q"]);

    // A light-path branch that marks an issue done: merge refuses the claim,
    // and so does land on the base after a plain git merge.
    git(repo, &["switch", "-qc", "sneaky"]);
    change(repo, "sneaky.rs", "fn sneaky() {}\n", "Do the other item");
    let path = issue_path(repo, &other);
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text.replace("status: \"todo\"", "status: \"done\"")).unwrap();
    assert_ne!(fs::read_to_string(&path).unwrap(), text);
    commit(repo, "Mark the other item done");
    approve(repo, "sneaky");
    let (passed, output) = cli(repo, &["merge", "--check"]);
    assert!(
        !passed && output.contains("claims.reviewed") && output.contains(&other),
        "{output}"
    );
    git(repo, &["switch", "-q", "main"]);
    git(repo, &["merge", "-q", "--no-ff", "--no-edit", "sneaky"]);
    let (passed, output) = cli(repo, &["land"]);
    assert!(
        !passed && output.contains("land_base_done_claim") && output.contains(&other),
        "{output}"
    );
}

#[test]
fn a_release_is_cut_on_the_light_path_with_its_tag_as_a_loose_end() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());

    git(repo, &["switch", "-qc", "release"]);
    change(repo, "VERSION", "0.4.0\n", "Bump the version to 0.4.0");
    change(
        repo,
        "NOTES.md",
        "# 0.4.0\n\n- The light path.\n",
        "Write the 0.4.0 notes",
    );
    // The tag is filed on the release branch with no blocker, so it reaches
    // the base with the merge and cannot be forgotten.
    let tag = ok(
        repo,
        &[
            "-q",
            "issue",
            "create",
            "Push the v0.4.0 tag",
            "--issue-type",
            "loose_end",
            "--body",
            "## Goal\n\nTag the release once it merges.\n\n## Done when\n\n- [ ] v0.4.0 is pushed.",
        ],
    );
    commit(repo, "File the tag as a loose end");
    let output = ok(repo, &["land"]);
    assert!(
        output.contains("Light path: no issue owns branch release, so its 3 commits since main merge on an independent approval of its head.")
            && output.contains("Next: atelier review request"),
        "{output}"
    );

    let brief = ok(repo, &["review", "request"]);
    assert!(
        brief.contains("# Review request: branch release") && brief.contains("NOTES.md"),
        "{brief}"
    );
    approve(repo, "release");
    let output = ok(repo, &["land"]);
    assert!(
        output.contains("Next: atelier merge") && output.contains("land: passed"),
        "{output}"
    );
    let output = ok(repo, &["merge"]);
    assert!(output.contains("Merged release into main"), "{output}");
    assert_eq!(git(repo, &["branch", "--show-current"]), "main");

    // The loose end is on the base and ready there.
    assert_eq!(status_on_main(repo, &tag), "todo");
    let ready = ok(repo, &["work", "ready"]);
    assert!(ready.contains(&tag), "{ready}");
    let output = ok(repo, &["land"]);
    assert!(output.contains("land: passed"), "{output}");
    ok(repo, &["doctor"]);
}

#[test]
fn status_shows_branches_on_the_light_path_with_their_review_state() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());
    git(repo, &["switch", "-qc", "docs"]);
    change(repo, "GUIDE.md", "# Guide\n", "Write the guide");
    let page = ok(repo, &["status"]);
    assert!(
        page.contains("Owner     : none")
            || page.contains("Owner   : none")
            || page.contains("Owner: none")
            || page
                .lines()
                .any(|line| line.starts_with("Owner") && line.ends_with("none")),
        "{page}"
    );
    assert!(
        page.lines().any(|line| line.starts_with("Light path")
            && line.ends_with("1 commit since main, ⚠ not approved")),
        "{page}"
    );
    assert!(page.contains("atelier review request"), "{page}");
    approve(repo, "docs");
    let page = ok(repo, &["status"]);
    assert!(
        page.lines().any(|line| line.starts_with("Light path")
            && line.ends_with("2 commits since main, ✓ approved")),
        "{page}"
    );

    // From another branch, an unowned branch with records about itself is
    // listed with its review state; one without is unowned.
    git(repo, &["switch", "-q", "main"]);
    git(repo, &["branch", "scratch"]);
    let page = ok(repo, &["status"]);
    assert!(
        page.contains("docs  light path, ✓ approved, behind 0, merges cleanly"),
        "{page}"
    );
    assert!(page.contains("Unowned: scratch"), "{page}");
    // The base itself has no light-path line.
    assert!(
        !page.lines().any(|line| line.starts_with("Light path")),
        "{page}"
    );
}

/// `land` on the base judges an approval of a branch as `merge` would:
/// after a plain `git merge`, which runs none of merge's gates, it counts
/// the branch's commits as reviewed only when every finding before the
/// approval is settled and no issue owned the branch when it merged.
#[test]
fn land_on_the_base_judges_an_approval_of_a_branch_as_merge_does() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());

    // A finding that needs a decision, left undecided before the approval.
    git(repo, &["switch", "-qc", "undecided"]);
    change(
        repo,
        "undecided.rs",
        "fn undecided() {}\n",
        "Add the undecided change",
    );
    let found = finding(repo, "undecided", "no_action", "low: the name is odd");
    approve(repo, "undecided");
    let (passed, output) = cli(repo, &["merge", "--check"]);
    assert!(
        !passed && output.contains("findings.disposed") && output.contains(&found),
        "{output}"
    );
    git(repo, &["switch", "-q", "main"]);
    git(repo, &["merge", "-q", "--no-ff", "--no-edit", "undecided"]);
    let (passed, output) = cli(repo, &["land"]);
    assert!(
        !passed && output.contains("land_base_unreviewed") && output.contains("undecided.rs"),
        "{output}"
    );
    // Deciding it on the base settles it, and land counts the commits.
    ok(
        repo,
        &[
            "review",
            "disposition",
            &found,
            "--identity",
            AUTHOR,
            "--decision",
            "no_action",
            "--commit-records",
            "The name is fine",
        ],
    );
    let output = ok(repo, &["land"]);
    assert!(output.contains("land: passed"), "{output}");
    git(repo, &["push", "-q"]);

    // A branch approved on the light path and then taken by `start` was
    // owned when it merged, so its approval covers nothing on the base.
    let owner = issue(repo, "Owned later");
    commit(repo, "Plan the owned work");
    git(repo, &["push", "-q"]);
    git(repo, &["switch", "-qc", "taken"]);
    change(repo, "taken.rs", "fn taken() {}\n", "Add the taken change");
    approve(repo, "taken");
    ok(repo, &["start", &owner]);
    commit(repo, "Start the owned work");
    git(repo, &["switch", "-q", "main"]);
    git(repo, &["merge", "-q", "--no-ff", "--no-edit", "taken"]);
    let (passed, output) = cli(repo, &["land"]);
    assert!(
        !passed && output.contains("land_base_unreviewed") && output.contains("taken.rs"),
        "{output}"
    );
}
