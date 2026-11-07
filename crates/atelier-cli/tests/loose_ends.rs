//! Loose ends (ADR 0028, decision 3; atelier-8bh4): work that changes no
//! code and can only happen after a merge, such as pushing a tag, waits on
//! what it follows through `blocked_by`, is never part of a mission, and is
//! done on the base with a one-line result. Validations share its workflow.
use std::{fs, path::Path};
use tempfile::tempdir;
#[path = "support/cli.rs"]
mod support_cli;
use support_cli::{cli, commit, git, ok};

const AUTHOR: &str = "Author";
const REVIEWER: &str = "Reviewer";
const ITEM: &str =
    "## Goal\n\nShip the launcher.\n\n## Done when\n\n- [ ] The launcher test passes.";
const TAG: &str =
    "## Goal\n\nTag the release once the launcher merges.\n\n## Done when\n\n- [ ] v1 is pushed.";
/// The policy an older `atelier init` wrote: `validation` in `work`, and no
/// `loose_end`.
const OLD_POLICY: &str =
    include_str!("../../atelier-workflow/fixtures/starter-policy-with-blocked-and-review.yaml");

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

fn record(repo: &Path, id: &str) -> String {
    fs::read_to_string(record_path(repo, id)).unwrap()
}

fn edit(repo: &Path, id: &str, from: &str, to: &str) {
    let text = record(repo, id);
    assert!(text.contains(from), "{id} lacks {from:?}:\n{text}");
    fs::write(record_path(repo, id), text.replacen(from, to, 1)).unwrap();
}

fn create(repo: &Path, args: &[&str]) -> String {
    let mut all = vec!["-q", "issue", "create"];
    all.extend_from_slice(args);
    ok(repo, &all)
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

fn lists(page: &str, title: &str, id: &str) -> bool {
    section(page, title).iter().any(|line| line.contains(id))
}

/// Starts `item`, implements it, ticks its Done when and writes its report,
/// then approves it as the reviewer, merges it and pushes the base.
fn implement_and_merge(repo: &Path, item: &str) {
    ok(repo, &["start", item]);
    commit(repo, &format!("Start {item}"));
    fs::write(repo.join("launcher.rs"), "fn main() {}\n").unwrap();
    edit(
        repo,
        item,
        "- [ ] The launcher test passes.",
        "- [x] The launcher test passes.",
    );
    let text = record(repo, item);
    fs::write(
        record_path(repo, item),
        format!("{text}\n## Report\n\nShipped.\n"),
    )
    .unwrap();
    commit(repo, &format!("Implement {item}"));
    git(repo, &["config", "user.name", REVIEWER]);
    let head = git(repo, &["rev-parse", "HEAD"]);
    ok(
        repo,
        &[
            "review",
            "approve",
            item,
            "--identity",
            REVIEWER,
            "--commit",
            &head,
            "--commit-records",
            "Reviewed the launcher",
        ],
    );
    git(repo, &["config", "user.name", AUTHOR]);
    let output = ok(repo, &["merge", item]);
    assert!(output.contains("merge: passed"), "{output}");
    git(repo, &["switch", "-q", "main"]);
    git(repo, &["push", "-q"]);
}

#[test]
fn a_loose_end_waits_for_its_merge_and_is_done_on_the_base() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());
    // Mission close also asks that its work be done, so a loose end in its
    // scope would hold it open.
    let policy = repo.join(".atelier/workflow.yaml");
    let close =
        "        - blockers.none_open\n      supersede:\n        from: [draft, todo, in_progress]";
    let text = fs::read_to_string(&policy).unwrap();
    assert!(text.contains(close), "{text}");
    fs::write(
        &policy,
        text.replacen(
            close,
            &close.replacen("none_open\n", "none_open\n        - work.terminal\n", 1),
            1,
        ),
    )
    .unwrap();
    commit(repo, "Close missions once their work is done");

    let mission = create(
        repo,
        &[
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nThe launcher ships.",
            "Launcher",
        ],
    );
    let item = create(repo, &["Launcher", "--body", ITEM]);
    ok(repo, &["issue", "link", "-q", &item, &mission]);
    let tag = create(
        repo,
        &[
            "Push the v1 tag",
            "--issue-type",
            "loose_end",
            "--body",
            TAG,
        ],
    );
    ok(
        repo,
        &["issue", "link", "-q", &tag, &item, "--role", "blocked_by"],
    );
    commit(repo, "Plan the launcher and its tag");
    ok(repo, &["issue", "transition", &mission, "ready"]);
    ok(repo, &["issue", "transition", &mission, "start"]);
    commit(repo, "Start the mission");
    git(repo, &["push", "-q"]);

    // Before the launcher merges, the tag waits on it.
    let page = ok(repo, &["work", "blocked"]);
    assert!(lists(&page, "Blocked Work", &tag), "{page}");
    let page = ok(repo, &["work", "ready"]);
    assert!(!page.contains(&tag), "{page}");

    // A loose end takes no branch: start and merge refuse it, naming the
    // transition that does it.
    for command in ["start", "merge"] {
        let (started, output) = cli(repo, &[command, &tag]);
        assert!(
            !started
                && output.contains(&format!(
                    "{command}_done_on_base: {tag} is a loose_end, whose workflow has no start transition"
                ))
                && output.contains(&format!(
                    "by its `done` transition once nothing it waits on is open. Next: atelier issue transition {tag} done --reason \"...\""
                )),
            "{command}: {output}"
        );
    }
    assert_eq!(git(repo, &["branch", "--show-current"]), "main");

    implement_and_merge(repo, &item);

    // Once it merges, the tag is ready, and nothing suggests starting it.
    let page = ok(repo, &["work", "ready"]);
    assert!(lists(&page, "Ready Work", &tag), "{page}");
    for args in [vec!["status"], vec!["work", "ready"], vec!["work", "all"]] {
        let page = ok(repo, &args);
        assert!(
            !page.contains(&format!("atelier start {tag}")),
            "{args:?}:\n{page}"
        );
    }
    // It is the human's when labelled so.
    ok(repo, &["issue", "update", &tag, "-l", "needs-human"]);
    let page = ok(repo, &["work", "human"]);
    assert!(lists(&page, "Needs a Human", &tag), "{page}");
    let page = ok(repo, &["work", "ready"]);
    assert!(!lists(&page, "Ready Work", &tag), "{page}");
    git(repo, &["checkout", "-q", "--", ".atelier"]);

    // It is no part of the mission, which closes while it is open, even when
    // a hand edit `atelier check` reports links it there.
    let page = ok(repo, &["work", "mission", &mission, "--all"]);
    assert!(!page.contains(&tag), "{page}");
    edit(
        repo,
        &tag,
        "    type: \"blocked_by\"\n",
        &format!("    type: \"blocked_by\"\n  - kind: \"issue\"\n    id: \"{mission}\"\n    type: \"advances\"\n"),
    );
    let (checked, output) = cli(repo, &["check"]);
    assert!(
        !checked && output.contains(&format!("Loose end {tag} advances {mission}")),
        "{output}"
    );
    let page = ok(repo, &["work", "mission", &mission, "--all"]);
    assert!(!page.contains(&tag), "{page}");
    let output = ok(
        repo,
        &[
            "issue",
            "transition",
            &mission,
            "close",
            "--reason",
            "Shipped.",
        ],
    );
    assert!(
        output.contains("To:       done") && !output.contains("Warning"),
        "{output}"
    );
    git(repo, &["checkout", "-q", "--", ".atelier"]);

    // Done on the base with a one-line result, which `land` accepts.
    let (done, output) = cli(repo, &["issue", "transition", &tag, "done"]);
    assert!(!done && output.contains("--reason"), "{output}");
    ok(
        repo,
        &[
            "issue",
            "transition",
            &tag,
            "done",
            "--reason",
            "Pushed v1.",
        ],
    );
    commit(repo, &format!("Push the v1 tag ({tag})"));
    let (landed, output) = cli(repo, &["land"]);
    assert!(landed, "{output}");
    git(repo, &["push", "-q"]);

    // So does the mission, closed on the base.
    ok(
        repo,
        &[
            "issue",
            "transition",
            &mission,
            "close",
            "--reason",
            "Shipped.",
        ],
    );
    commit(repo, &format!("Close {mission}"));
    let (landed, output) = cli(repo, &["land"]);
    assert!(landed, "{output}");
}

#[test]
fn a_loose_end_is_no_issue_s_child_and_advances_no_mission() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());
    let mission = create(
        repo,
        &[
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nThe launcher ships.",
            "Launcher",
        ],
    );
    let item = create(repo, &["Launcher", "--body", ITEM]);
    let tag = create(
        repo,
        &[
            "Push the v1 tag",
            "--issue-type",
            "loose_end",
            "--body",
            TAG,
        ],
    );
    commit(repo, "Plan");

    let refused = |args: &[&str], what: &str| {
        let (written, output) = cli(repo, args);
        assert!(
            !written && output.contains("record_invalid") && output.contains(what),
            "{args:?}: {output}"
        );
        assert_eq!(git(repo, &["status", "--porcelain"]), "", "{args:?}");
    };
    let advances = format!("Loose end {tag} advances {mission}");
    refused(
        &["issue", "link", &tag, &mission, "--role", "advances"],
        &advances,
    );
    refused(
        &["issue", "link", &mission, &tag, "--role", "advances"],
        &advances,
    );
    let child = format!("Loose end {tag} is a child of {item}");
    refused(&["issue", "update", &tag, "--parent", &item], &child);
    let (created, output) = cli(
        repo,
        &[
            "issue",
            "create",
            "Push the v2 tag",
            "--issue-type",
            "loose_end",
            "--body",
            TAG,
            "--parent",
            &item,
        ],
    );
    assert!(
        !created && output.contains("record_invalid") && output.contains("is a child of"),
        "{output}"
    );
    // Waiting on the item is the link a loose end takes.
    ok(
        repo,
        &["issue", "link", &tag, &item, "--role", "blocked_by"],
    );
    ok(repo, &["check"]);
}

#[test]
fn validation_is_done_on_the_base_unless_the_policy_keeps_it_in_work() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());
    let mission = create(
        repo,
        &[
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nThe launcher ships.",
            "Launcher",
        ],
    );
    let check = create(
        repo,
        &[
            "Validate the launcher against the outcome",
            "--issue-type",
            "validation",
            "--body",
            "## Goal\n\nJudge the outcome.\n\n## Done when\n\n- [ ] The outcome holds.",
        ],
    );
    ok(repo, &["issue", "link", "-q", &check, &mission]);
    commit(repo, "Plan the mission and its validation");
    ok(repo, &["issue", "transition", &mission, "ready"]);
    ok(repo, &["issue", "transition", &mission, "start"]);
    commit(repo, "Start the mission");

    // The validation is ready mission work, but not work to start.
    let page = ok(repo, &["work", "mission", &mission]);
    assert!(
        lists(&page, "Ready Work", &check) && !page.contains(&format!("atelier start {check}")),
        "{page}"
    );
    let (started, output) = cli(repo, &["start", &check]);
    assert!(
        !started
            && output.contains("start_done_on_base")
            && output.contains(&format!("atelier issue transition {check} done")),
        "{output}"
    );
    ok(
        repo,
        &[
            "issue",
            "transition",
            &check,
            "done",
            "--reason",
            "The outcome holds.",
        ],
    );
    commit(repo, "Validate the mission");
    let (landed, output) = cli(repo, &["land"]);
    assert!(landed, "{output}");

    // A policy that keeps `validation` in `work` starts it as before.
    fs::write(repo.join(".atelier/workflow.yaml"), OLD_POLICY).unwrap();
    let other = create(repo, &["Validate again", "--issue-type", "validation"]);
    commit(repo, "Keep validation in work");
    git(repo, &["push", "-q"]);
    let output = ok(repo, &["start", &other]);
    assert!(output.contains("Applied transition start"), "{output}");
}

#[test]
fn migrate_after_merge_turns_open_lines_into_loose_ends_once() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());
    let bug = create(repo, &["The launcher crashes", "--issue-type", "bug"]);
    let open = create(
        repo,
        &[
            "Launcher",
            "--body",
            &format!(
                "## Goal\n\nShip the launcher.\n\n## Done when\n\n- [ ] The launcher test passes.\n- [ ] After merge (device): the launcher opens on the phone. Stand-in: the emulator test passes.\n- [ ] After merge (use): it runs a week without a crash.\n- [x] After merge (human): Sumeet likes the icon.\n- [ ] After merge (human): the splash looks right. Carried to {bug}.\n- [ ] After merge (deployed): the update server serves it, carried to the server."
            ),
        ],
    );
    let merged = create(
        repo,
        &[
            "Settings",
            "--body",
            "## Goal\n\nShip settings.\n\n## Done when\n\n- [x] The settings test passes.\n- [ ] After merge (Sumeet's judgement): the settings read well.",
        ],
    );
    edit(repo, &merged, "status: \"todo\"", "status: \"done\"");
    commit(repo, "Plan the launcher and settings");
    let before_merged = record(repo, &merged);

    // A policy without loose ends has nothing to turn the lines into.
    let policy = repo.join(".atelier/workflow.yaml");
    let starter = fs::read_to_string(&policy).unwrap();
    fs::write(&policy, OLD_POLICY).unwrap();
    let (migrated, output) = cli(repo, &["migrate", "after-merge"]);
    assert!(
        !migrated && output.contains("migrate_no_loose_end") && output.contains("loose_end"),
        "{output}"
    );
    fs::write(&policy, starter).unwrap();

    // The dry run shows each open line, which get needs-human, and writes
    // nothing.
    let output = ok(repo, &["migrate", "after-merge"]);
    let lines = output
        .lines()
        .filter(|line| line.starts_with("- "))
        .collect::<Vec<_>>();
    // Records are listed in ID order, which is random here.
    for expected in [
        format!("- {open}: \"the launcher opens on the phone.\" becomes a loose end blocked_by {open}, labelled needs-human (reason: device); its stand-in stays on {open} as a Done when line: the emulator test passes."),
        format!("- {open}: \"it runs a week without a crash.\" becomes a loose end blocked_by {open}, not labelled needs-human (reason: use); the line leaves {open}, which gave no stand-in"),
        format!("- {open}: \"the update server serves it, carried to the server.\" becomes a loose end blocked_by {open}, not labelled needs-human (reason: deployed); the line leaves {open}, which gave no stand-in"),
        format!("- {merged}: \"the settings read well.\" becomes a loose end blocked_by {merged}, labelled needs-human (reason: Sumeet's judgement); {merged} is done, so its record is unchanged"),
    ] {
        assert!(lines.contains(&expected.as_str()), "{expected}\n{output}");
    }
    assert_eq!(lines.len(), 4, "{output}");
    assert!(output.contains("Dry run: rerun with --apply"), "{output}");
    assert_eq!(git(repo, &["status", "--porcelain"]), "");

    let made = ok(repo, &["-q", "migrate", "after-merge", "--apply"]);
    let made = made.lines().map(str::to_owned).collect::<Vec<_>>();
    assert_eq!(made.len(), 4, "{made:?}");
    ok(repo, &["check"]);

    // The item that is not done keeps the stand-in as an ordinary line and
    // the lines that were validated or carried to an issue; the done item
    // is unchanged.
    let text = record(repo, &open);
    assert!(
        text.contains(&format!(
            "- [ ] The launcher test passes.\n- [ ] the emulator test passes.\n- [x] After merge (human): Sumeet likes the icon.\n- [ ] After merge (human): the splash looks right. Carried to {bug}.\n"
        )) && !text.contains("runs a week")
            && !text.contains("update server"),
        "{text}"
    );
    assert_eq!(record(repo, &merged), before_merged);

    // Each loose end holds its check, waits on its item, and says why.
    let phone = made
        .iter()
        .map(|id| record(repo, id))
        .find(|text| text.contains("opens on the phone"))
        .unwrap();
    assert!(
        phone.contains("issue_type: \"loose_end\"")
            && phone.contains("- \"needs-human\"")
            && phone.contains(&format!("id: \"{open}\"\n    type: \"blocked_by\""))
            && phone.contains("because of: device.")
            && phone.contains("## Done when\n\n- [ ] the launcher opens on the phone.\n"),
        "{phone}"
    );
    let settings = made
        .iter()
        .map(|id| record(repo, id))
        .find(|text| text.contains("settings read well"))
        .unwrap();
    assert!(
        settings.contains("- \"needs-human\"")
            && settings.contains(&format!("id: \"{merged}\"\n    type: \"blocked_by\"")),
        "{settings}"
    );
    commit(repo, "Migrate the After merge lines");

    // Rerunning finds nothing left, and changes nothing.
    let output = ok(repo, &["migrate", "after-merge"]);
    assert_eq!(output, "No open After merge lines to migrate.");
    ok(repo, &["migrate", "after-merge", "--apply"]);
    assert_eq!(git(repo, &["status", "--porcelain"]), "");
}

/// Atelier's own policy defines loose ends as the starter does: one
/// `on_base` workflow for `loose_end` and `validation`, with no `start`.
#[test]
fn atelier_s_own_policy_shares_the_starter_s_on_base_workflow() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.atelier/workflow.yaml");
    let own = atelier_workflow::parse_policy_text(
        &fs::read_to_string(path).unwrap(),
        ".atelier/workflow.yaml",
    )
    .unwrap();
    let starter = atelier_workflow::parse_policy_text(
        atelier_workflow::STARTER_POLICY_YAML,
        ".atelier/workflow.yaml",
    )
    .unwrap();
    for policy in [&own, &starter] {
        for issue_type in ["loose_end", "validation"] {
            assert_eq!(
                policy.workflow_name_for_issue_type(issue_type).unwrap(),
                "on_base"
            );
            assert!(policy.done_on_base(issue_type), "{issue_type}");
        }
        let done = policy
            .transition_for_issue_type("loose_end", "done")
            .unwrap();
        assert_eq!(
            (
                done.from.clone(),
                done.to.as_str(),
                done.required_fields.clone()
            ),
            (
                vec!["todo".to_owned()],
                "done",
                vec!["close_reason".to_owned()]
            )
        );
    }
    let transitions = |policy: &atelier_workflow::WorkflowPolicy| {
        let workflow = &policy.workflows["on_base"];
        workflow
            .transitions
            .iter()
            .map(|(name, transition)| {
                (
                    name.clone(),
                    transition.from.clone(),
                    transition.to.clone(),
                    transition.required_fields.clone(),
                    transition
                        .validators
                        .iter()
                        .map(|validator| validator.builtin.clone())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(transitions(&own), transitions(&starter));
}

/// Without `loose_end` in the policy, the advice for a needs-human child
/// that holds its owner's branch keeps it standalone instead, with commands
/// that policy accepts.
#[test]
fn the_needs_human_child_advice_fits_a_policy_without_loose_ends() {
    let dir = tempdir().unwrap();
    let upstream = tempdir().unwrap();
    let repo = dir.path();
    repository(repo, upstream.path());
    fs::write(repo.join(".atelier/workflow.yaml"), OLD_POLICY).unwrap();
    commit(repo, "Use the older starter policy");
    git(repo, &["push", "-q"]);
    let parent = create(repo, &["Launcher", "--body", ITEM]);
    let check = create(
        repo,
        &[
            "Play-test the release build",
            "--issue-type",
            "validation",
            "--parent",
            &parent,
            "-l",
            "needs-human",
        ],
    );
    commit(repo, "A feature with its play-test as a child");
    let advice = format!(
        "make it a standalone item that waits on {parent}, outside its scope: `atelier issue update {check} --no-parent`, then `atelier issue link {check} {parent} --role blocked_by`."
    );
    let output = ok(repo, &["start", &parent]);
    assert!(
        output.contains(&format!(
            "{check} labelled needs-human is a child of {parent}"
        )) && output.contains(&advice)
            && !output.contains("loose_end"),
        "{output}"
    );
    commit(repo, "Start the launcher");
    let output = ok(repo, &["status"]);
    assert!(
        output.contains(&advice) && !output.contains("loose_end"),
        "{output}"
    );
    // The advice runs under this policy.
    ok(repo, &["issue", "update", &check, "--no-parent"]);
    ok(
        repo,
        &["issue", "link", &check, &parent, "--role", "blocked_by"],
    );
    let output = ok(repo, &["status"]);
    assert!(
        !output.contains("labelled needs-human is a child"),
        "{output}"
    );
}
