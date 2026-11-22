//! What keeps rounds of review converging: a later round's diff, the exit
//! after two split rounds of plan review, and the cap on rounds of an
//! item's review, driven in scratch repositories.

mod common;

use common::*;
use std::fs;
use std::path::Path;
use std::process::Command;

const MISSION: &str = "demo-m1ss";
const SPLIT_RULE: &str =
    "Plan review of a change to mission demo-m1ss stops after 2 split rounds in a row";

fn mission_file(name: &str) -> String {
    format!(".atelier/missions/{MISSION}/{name}")
}

fn write_mission(at: &Path) {
    let path = at.join(mission_file("mission.md"));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        format!(
            "---\nschema: atelier.mission\nschema_version: 3\nid: {MISSION}\ntitle: Greet\n\
             status: open\nlabels: []\nlead: lead1\nplanners:\n- p1\nready: null\n\
             created_at: 2026-11-22T09:00:00Z\n---\n\n## Why\n\nAsked for.\n\n\
             ## Outcome\n\nGreetings.\n"
        ),
    )
    .unwrap();
}

/// Writes draft item `id` in the mission.
fn write_mission_item(at: &Path, id: &str) {
    write_item(at, id, "draft", &[], &[], "");
    edit(
        at,
        &item_path(id),
        "mission: null",
        &format!("mission: {MISSION}"),
    );
}

fn write(at: &Path, path: &str, text: &str) {
    fs::create_dir_all(at.join(path).parent().unwrap()).unwrap();
    fs::write(at.join(path), text).unwrap();
}

fn edit(at: &Path, path: &str, from: &str, to: &str) {
    let text = read(at, path);
    assert!(text.contains(from), "{from} in {path}");
    write(at, path, &text.replace(from, to));
}

fn commit(at: &Path, message: &str) -> String {
    git(at, &["add", "-A"]);
    git(at, &["commit", "-qm", message]);
    git(at, &["rev-parse", "HEAD"])
}

fn review_text(round: &str, by: &str, verdict: &str, commit: &str) -> String {
    let finding = match verdict {
        "changes" => "\n## Finding 1: Vague\nRecommendation: fix now\nSay who is greeted.\n",
        _ => "",
    };
    format!(
        "---\nschema: atelier.review\nschema_version: 3\nround: {round}\ncommit: {commit}\n\
         reviewers:\n- {by}\nmodel: null\nverdict: {verdict}\n\
         created_at: 2026-11-22T09:00:00Z\n---\n\n## Summary\n\nRead it.\n{finding}"
    )
}

/// Writes a plan review `name` by a thread of its own, of `commit`.
fn plan_review(at: &Path, name: &str, verdict: &str, commit: &str) {
    let round = name.split('-').next().unwrap();
    let text = review_text(round, &format!("r-{name}"), verdict, commit);
    write(at, &mission_file(&format!("reviews/{name}.md")), &text);
}

/// A round of plan review of the checkout's HEAD, one review for each of
/// `verdicts`, committed as its reviewers' pushes leave the branch.
fn plan_round(at: &Path, round: u32, verdicts: &[&str]) -> String {
    let reviewed = git(at, &["rev-parse", "HEAD"]);
    for (number, verdict) in verdicts.iter().enumerate() {
        plan_review(at, &format!("{round}-own{number}"), verdict, &reviewed);
    }
    commit(at, &format!("Review round {round}"));
    reviewed
}

/// A scratch repository whose checkout is on the mission's branch, holding
/// its proposed plan, committed.
fn proposed() -> Scratch {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    git(&root, &["switch", "-q", "-c", MISSION, "main"]);
    write_mission(&root);
    commit(&root, "Plan");
    scratch
}

/// A scratch repository whose base holds the mission agreed, with `items`
/// in it.
fn agreed(items: &[&str]) -> Scratch {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    write_mission(&root);
    for id in items {
        write_mission_item(&root, id);
    }
    on_base(&root, "Agree");
    scratch
}

fn start_amendment(root: &Path) {
    let run = atelier(root, "p1", &["start", "--backlog", "amend"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
}

/// What `atelier review` prints to a new plan reviewer in a detached
/// checkout of the branch's head, and that checkout.
fn plan_brief(scratch: &Scratch) -> (String, std::path::PathBuf) {
    let root = scratch.root();
    let at = scratch.dir.path().join("reviewer");
    let (to, head) = (at.to_str().unwrap(), git(&root, &["rev-parse", "HEAD"]));
    git(&root, &["worktree", "add", "-q", "--detach", to, &head]);
    let run = atelier(&at, "reviewer", &["review", MISSION]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    (run.stdout, at)
}

/// The paths the first diff command in `brief` shows, run at `at` as a
/// reviewer pasting it would.
fn delta_paths(at: &Path, brief: &str) -> Vec<String> {
    let line = brief
        .lines()
        .find(|line| line.starts_with("  git") && line.contains(" diff "))
        .unwrap_or_else(|| panic!("no diff in {brief}"));
    let line = line
        .trim()
        .replacen(" diff ", " diff --no-renames --name-only ", 1);
    let output = identity(Command::new("sh").current_dir(at).args(["-c", &line]))
        .output()
        .unwrap();
    assert!(output.status.success(), "{line}: {output:?}");
    let listed = String::from_utf8(output.stdout).unwrap();
    listed.lines().map(str::to_string).collect()
}

#[test]
fn a_later_plan_round_puts_the_diff_of_the_plans_own_paths_first() {
    let scratch = proposed();
    let root = scratch.root();
    for id in ["demo-edit", "demo-left"] {
        write_mission_item(&root, id);
    }
    write_item(&root, "demo-othr", "draft", &[], &[], "");
    write(&root, &mission_file("the shape.md"), "Round\n");
    write(&root, &mission_file("notes.md"), "One\n");
    commit(&root, "Items");
    let reviewed = plan_round(&root, 1, &["approve", "changes"]);

    edit(&root, &item_path("demo-edit"), "Print hello.", "Print hi.");
    // An item that left the mission is of it at the commit reviewed, and one
    // that joined it is at the head.
    edit(
        &root,
        &item_path("demo-left"),
        &format!("mission: {MISSION}"),
        "mission: null",
    );
    write_mission_item(&root, "demo-join");
    edit(&root, &item_path("demo-othr"), "Print hello.", "Print bye.");
    write(&root, &mission_file("the shape.md"), "Square\n");
    write(&root, &mission_file("notes.md"), "Two\n");
    write(
        &root,
        ".atelier/items/demo-edit/reviews/1-item.md",
        &review_text("1", "r9", "approve", &reviewed),
    );
    commit(&root, "Answer round 1");

    let (brief, at) = plan_brief(&scratch);
    let first = format!(
        "You are a reviewer of the plan of mission {MISSION} (thread reviewer), round 2.\n\
         What changed in the plan since round 1 reviewed it at commit {}:\n  git diff {reviewed} ",
        &reviewed[..7]
    );
    assert!(brief.starts_with(&first), "{brief}");
    assert!(
        brief.contains("This round checks those changes and judges each open finding carried from earlier rounds, below. A new finding in a part of the plan that did not change blocks only if it is a serious defect the earlier rounds missed: record that one as `fix now`, and any other as `defer`."),
        "{brief}"
    );
    assert!(!brief.contains("Review the plan as commit"), "{brief}");
    assert!(brief.contains("carried as Finding 1: Vague"), "{brief}");
    assert_eq!(
        delta_paths(&at, &brief),
        [
            item_path("demo-edit"),
            item_path("demo-join"),
            item_path("demo-left"),
            mission_file("the shape.md"),
        ]
    );

    // From below the root the command names it, since the paths are from it.
    let below = at.join(".atelier");
    let run = atelier(&below, "reviewer", &["review", MISSION]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(run.stdout.contains("  git -C "), "{}", run.stdout);
    assert_eq!(delta_paths(&below, &run.stdout), delta_paths(&at, &brief));
}

#[test]
fn an_unchanged_plan_says_so_and_prints_no_diff() {
    let scratch = proposed();
    let root = scratch.root();
    let reviewed = plan_round(&root, 1, &["approve", "changes"]);
    write(&root, &mission_file("notes.md"), "A note.\n");
    commit(&root, "Note");

    let (brief, _) = plan_brief(&scratch);
    assert!(
        brief.contains(&format!(
            "The plan is unchanged since round 1 reviewed it at commit {}, so there is no diff to read.\nThis round judges each open finding carried from earlier rounds, below.",
            &reviewed[..7]
        )),
        "{brief}"
    );
    assert!(!brief.contains("git diff"), "{brief}");
    assert!(brief.contains("carried as Finding 1: Vague"), "{brief}");
}

#[test]
fn a_plan_round_with_no_one_earlier_commit_in_its_history_reviews_the_whole_plan() {
    for case in ["missing", "two commits", "not an ancestor"] {
        let scratch = proposed();
        let root = scratch.root();
        let planned = git(&root, &["rev-parse", "HEAD"]);
        let (first, second) = match case {
            "missing" => ("1".repeat(40), "1".repeat(40)),
            "two commits" => (planned.clone(), git(&root, &["rev-parse", "main"])),
            _ => {
                let aside = git(&root, &["commit-tree", "HEAD^{tree}", "-m", "Rebased away"]);
                (aside.clone(), aside)
            }
        };
        plan_review(&root, "1-aaaa", "approve", &first);
        plan_review(&root, "1-bbbb", "changes", &second);
        commit(&root, "Review");

        let (brief, _) = plan_brief(&scratch);
        assert!(
            brief.starts_with(&format!(
                "You are a reviewer of the plan of mission {MISSION} (thread reviewer), round 2.\n\
                 Review the plan as commit "
            )),
            "{case}: {brief}"
        );
        assert!(!brief.contains("git diff"), "{case}: {brief}");
        assert!(
            brief.contains("carried as Finding 1: Vague"),
            "{case}: {brief}"
        );
    }
}

#[test]
fn an_amendments_delta_keeps_its_item_edits_and_leaves_out_what_the_base_merged() {
    let scratch = agreed(&["demo-mine", "demo-base"]);
    let root = scratch.root();
    start_amendment(&root);
    edit(
        &root,
        &mission_file("mission.md"),
        "Greetings.",
        "Two greetings.",
    );
    commit(&root, "Amend");
    let reviewed = plan_round(&root, 1, &["approve", "changes"]);

    let base = scratch.worktree("base");
    append(&base, &item_path("demo-base"), "Merged: it prints hello.\n");
    write(&base, ".atelier/items/demo-base/proof.txt", "hello\n");
    write(&base, &mission_file("notes.md"), "Merged demo-base.\n");
    for id in ["demo-base", "demo-mine"] {
        write(
            &base,
            &format!(".atelier/items/{id}/reviews/1-base.md"),
            &review_text("1", "r9", "approve", &reviewed),
        );
    }
    on_base(&base, "Merge demo-base");
    git(&root, &["merge", "-q", "--no-edit", "main"]);
    edit(&root, &item_path("demo-mine"), "Print hello.", "Print hi.");
    edit(
        &root,
        &mission_file("mission.md"),
        "Two greetings.",
        "Three greetings.",
    );
    commit(&root, "Answer round 1");

    let (brief, at) = plan_brief(&scratch);
    assert_eq!(
        delta_paths(&at, &brief),
        [item_path("demo-mine"), mission_file("mission.md")]
    );
}

#[test]
fn an_item_edit_an_amendment_took_back_after_a_round_shows_in_the_delta() {
    let scratch = agreed(&["demo-back", "demo-base"]);
    let root = scratch.root();
    start_amendment(&root);
    edit(
        &root,
        &mission_file("mission.md"),
        "Greetings.",
        "Two greetings.",
    );
    edit(
        &root,
        &item_path("demo-back"),
        "Print hello.",
        "Print hello twice.",
    );
    write_mission_item(&root, "demo-gone");
    commit(&root, "Amend");
    let reviewed = plan_round(&root, 1, &["approve", "changes"]);

    let base = scratch.worktree("base");
    append(&base, &item_path("demo-base"), "Merged: it prints hello.\n");
    on_base(&base, "Merge demo-base");
    git(&root, &["merge", "-q", "--no-edit", "main"]);
    edit(
        &root,
        &item_path("demo-back"),
        "Print hello twice.",
        "Print hello.",
    );
    fs::remove_dir_all(root.join(".atelier/items/demo-gone")).unwrap();
    commit(&root, "Take both back");

    // The head holds them as the base does, so only what the branch changed
    // at the commit reviewed, `git diff main...<prev>`, names them.
    let at_head = git(&root, &["diff", "--name-only", "main...HEAD"]);
    assert!(
        !at_head.contains("demo-back") && !at_head.contains("demo-gone"),
        "{at_head}"
    );

    let (brief, at) = plan_brief(&scratch);
    assert_eq!(
        delta_paths(&at, &brief),
        [item_path("demo-back"), item_path("demo-gone")]
    );
    let shown = git(
        &at,
        &["diff", &reviewed, "HEAD", "--", &item_path("demo-back")],
    );
    assert!(shown.contains("-Print hello twice."), "{shown}");
}

/// Ends round `round` of the item's review asking for changes, as a
/// reviewer of its own would, and gives the finding left open.
fn request_changes(root: &Path, round: u32) -> String {
    let reviewer = format!("r{round}");
    let run = atelier(root, &reviewer, &["review", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let (stem, path) = review_of(root, round);
    if round == 1 {
        append(
            root,
            &path,
            "\n## Finding 1: No period\nRecommendation: fix now\nNone.\n",
        );
    }
    let run = verdict(root, &reviewer, &["verdict", "changes"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    format!("{stem}.1")
}

/// t1 answers `finding` as fixed.
fn answer(root: &Path, finding: &str) {
    if !read(root, &item_path(ID)).contains("### Responses") {
        append(root, &item_path(ID), "\n### Responses\n\n");
    }
    let head = git(root, &["rev-parse", "--short", "HEAD"]);
    append(
        root,
        &item_path(ID),
        &format!("- {finding}: fixed in {head}\n"),
    );
}

fn submit(root: &Path) {
    let run = atelier(root, "t1", &["submit"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
}

fn item_brief(root: &Path) -> String {
    let run = atelier(root, "r2", &["review", ID]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    run.stdout
}

#[test]
fn a_code_round_whose_earlier_commit_is_gone_reviews_the_whole_change() {
    for case in ["missing", "not an ancestor"] {
        let scratch = submitted();
        let root = scratch.root();
        let reviewed = git(&root, &["rev-parse", "HEAD"]);
        let finding = request_changes(&root, 1);
        let gone = match case {
            "missing" => {
                let (_, path) = review_of(&root, 1);
                edit(&root, &path, &reviewed, &"1".repeat(40));
                commit(&root, "Point at a commit this clone lacks");
                "1".repeat(40)
            }
            _ => {
                let base = scratch.worktree("base");
                write(&base, "upstream.txt", "new\n");
                on_base(&base, "Upstream");
                git(&root, &["rebase", "-q", "main"]);
                reviewed.clone()
            }
        };
        answer(&root, &finding);
        submit(&root);
        let head = git(&root, &["rev-parse", "HEAD"]);

        let brief = item_brief(&root);
        assert!(
            brief.contains(&format!(
                "round 2.\nThe last round reviewed commit {}, which is not in this commit's history, so there is no diff since then. Review the change on branch {ID} ",
                &gone[..7]
            )),
            "{case}: {brief}"
        );
        assert!(
            brief.contains(&format!(
                "\nThe change, at commit {head}:\n  git diff main...{head}\n"
            )),
            "{case}: {brief}"
        );
        assert!(
            !brief.contains(&format!("git diff {gone}")),
            "{case}: {brief}"
        );
        assert!(!brief.contains("What changed since"), "{case}: {brief}");
        assert!(
            brief.contains("carried as Finding 1: No period"),
            "{case}: {brief}"
        );
    }
}

#[test]
fn a_code_delta_across_a_merge_of_the_base_names_only_the_branchs_paths() {
    let scratch = Scratch::new(&[(ID, "ready")]);
    let root = scratch.root();
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    write(&root, "hello.txt", "hello\n");
    write(&root, "taken back.txt", "extra\n");
    commit(&root, "Say hello");
    finish_and_submit(&root, "t1");
    let reviewed = git(&root, &["rev-parse", "HEAD"]);
    let finding = request_changes(&root, 1);

    let base = scratch.worktree("base");
    write(&base, "upstream.txt", "new\n");
    on_base(&base, "Upstream");
    git(&root, &["merge", "-q", "--no-edit", "main"]);
    write(&root, "hello.txt", "hello.\n");
    fs::remove_file(root.join("taken back.txt")).unwrap();
    commit(&root, "Fix it");
    answer(&root, &finding);
    submit(&root);
    let head = git(&root, &["rev-parse", "HEAD"]);

    let brief = item_brief(&root);
    let (_, review) = review_of(&root, 1);
    assert!(
        brief.contains(&format!(
            "What changed since the last round's review, at commit {head}, in the paths this branch changes, since main was merged in between:\n  git diff {reviewed} {head} -- "
        )),
        "{brief}"
    );
    assert_eq!(
        delta_paths(&root, &brief),
        [
            item_path(ID),
            review,
            "hello.txt".into(),
            "taken back.txt".into()
        ]
    );
}

/// `atelier status` as p1.
fn status(root: &Path) -> String {
    let run = atelier(root, "p1", &["status"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    run.stdout
}

fn exit_after(splits: u32) -> String {
    format!("Plan review of mission {MISSION} stops: its last {splits} rounds on this change split, one approving and one asking for changes. Bring the open findings to the human to rule.")
}

/// `submit` as `thread`, refused for the split rounds, changing nothing.
fn submit_refused(root: &Path, thread: &str, splits: u32) {
    let head = git(root, &["rev-parse", "HEAD"]);
    let run = atelier(root, thread, &["submit"]);
    assert_eq!(run.code, 1, "{}", run.stdout);
    assert!(
        run.stderr.starts_with(&format!(
            "`atelier submit` refused; nothing changed.\n{}",
            exit_after(splits)
        )),
        "{}",
        run.stderr
    );
    assert!(run.stderr.ends_with(SKIP), "{}", run.stderr);
    assert!(!run.stderr.contains("run atelier review"), "{}", run.stderr);
    assert_eq!(git(root, &["rev-parse", "HEAD"]), head);
    assert_eq!(git(root, &["status", "--porcelain"]), "");
}

fn skipping(root: &Path, command: &str) -> Run {
    let output = atelier_command(root, "p1", &[command])
        .env("ATELIER_SKIP", "The human ruled")
        .output()
        .unwrap();
    Run {
        code: output.status.code().unwrap(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

#[test]
fn two_split_rounds_send_a_proposed_plan_to_the_human() {
    let scratch = proposed();
    let root = scratch.root();
    let none = format!("Plan review of mission {MISSION}: no review of this change here yet.\n");
    assert!(status(&root).contains(&none), "{}", status(&root));

    plan_round(&root, 1, &["approve", "changes"]);
    // One split: another round, and the planner is told where that ends.
    let shown = status(&root);
    assert!(
        shown.contains(&format!(
            "Plan review of mission {MISSION}, round 1: 1 of 2 approvals count.\n"
        )) && !shown.contains("stops: its last"),
        "{shown}"
    );
    let merge = atelier(&root, "p1", &["merge"]);
    assert!(
        merge.stderr.contains("Run `atelier submit` here"),
        "{}",
        merge.stderr
    );
    let run = atelier(&root, "p1", &["submit"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stdout.contains("for round 2 of plan review")
            && run.stdout.contains("unless that makes 2 rounds in a row on this change where one approves and the other asks for changes: review stops there, and you bring the open findings to the human to rule."),
        "{}",
        run.stdout
    );

    plan_round(&root, 2, &["changes", "approve"]);
    let shown = status(&root);
    assert!(shown.contains(&exit_after(2)), "{shown}");
    assert!(!shown.contains("Then: atelier"), "{shown}");
    submit_refused(&root, "p1", 2);
    // Nobody takes the plan over past the exit by submitting it.
    submit_refused(&root, "p2", 2);
    let merge = atelier(&root, "p1", &["merge"]);
    assert_eq!(merge.code, 1);
    assert!(merge.stderr.contains(&exit_after(2)), "{}", merge.stderr);
    assert!(
        !merge.stderr.contains("Run `atelier submit` here"),
        "{}",
        merge.stderr
    );
    assert!(merge.stderr.ends_with(SKIP), "{}", merge.stderr);

    // The human has it amended and reviewed once more.
    edit(
        &root,
        &mission_file("mission.md"),
        "Greetings.",
        "Greetings to all.",
    );
    commit(&root, "Amend as ruled");
    let run = skipping(&root, "submit");
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stdout.contains("for round 3 of plan review"),
        "{}",
        run.stdout
    );
    assert!(
        run.stdout.contains(&format!(
            "Skipped with ATELIER_SKIP: {SPLIT_RULE}: The human ruled\n"
        )),
        "{}",
        run.stdout
    );
    plan_round(&root, 3, &["approve", "changes"]);
    submit_refused(&root, "p1", 3);

    // Or has it merge as it stands.
    let run = skipping(&root, "merge");
    assert_eq!(run.code, 0, "{}{}", run.stdout, run.stderr);
}

#[test]
fn status_still_says_how_to_commit_the_fix_of_a_committed_record_that_does_not_read() {
    let scratch = proposed();
    let root = scratch.root();
    let path = mission_file("mission.md");
    edit(&root, &path, "status: open", "status: bogus");
    commit(&root, "Break it");
    edit(&root, &path, "status: bogus", "status: open");
    let shown = status(&root);
    assert!(
        shown.contains("Plan review not shown, since a committed record does not read: ")
            && shown.contains(&format!("Commit them with: git add -- {path}")),
        "{shown}"
    );
}

#[test]
fn a_round_that_does_not_split_breaks_the_row() {
    let scratch = proposed();
    let root = scratch.root();
    plan_round(&root, 1, &["approve", "changes"]);
    plan_round(&root, 2, &["changes", "changes"]);
    plan_round(&root, 3, &["approve", "changes"]);
    assert!(
        !status(&root).contains("stops: its last"),
        "{}",
        status(&root)
    );
    let run = atelier(&root, "p1", &["submit"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
}

#[test]
fn an_amendment_starts_at_no_splits_and_two_of_its_own_send_it_to_the_human() {
    // The plan merged on the human's ruling after two split rounds.
    let scratch = agreed(&[]);
    let root = scratch.root();
    let planned = git(&root, &["rev-parse", "HEAD"]);
    for (name, verdict) in [
        ("1-aaaa", "approve"),
        ("1-bbbb", "changes"),
        ("2-cccc", "approve"),
        ("2-dddd", "changes"),
    ] {
        plan_review(&root, name, verdict, &planned);
    }
    on_base(&root, "Merge the plan");

    start_amendment(&root);
    edit(
        &root,
        &mission_file("mission.md"),
        "Greetings.",
        "Two greetings.",
    );
    commit(&root, "Amend");
    let shown = status(&root);
    assert!(
        shown.contains(&format!(
            "Plan review of mission {MISSION}: no review of this change here yet.\n"
        )) && !shown.contains("stops: its last"),
        "{shown}"
    );
    let merge = atelier(&root, "p1", &["merge"]);
    assert!(
        merge.stderr.contains("Run `atelier submit` here"),
        "{}",
        merge.stderr
    );
    let run = atelier(&root, "p1", &["submit"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stdout.contains("for round 3 of plan review"),
        "{}",
        run.stdout
    );

    plan_round(&root, 3, &["approve", "changes"]);
    assert!(
        !status(&root).contains("stops: its last"),
        "{}",
        status(&root)
    );
    let run = atelier(&root, "p1", &["submit"]);
    assert_eq!(run.code, 0, "{}", run.stderr);

    plan_round(&root, 4, &["approve", "changes"]);
    // Another amendment's round 3, merged meanwhile, is not this one's.
    let base = scratch.worktree("base");
    plan_review(&base, "3-zzzz", "approve", &planned);
    on_base(&base, "Merge another amendment");
    git(&root, &["merge", "-q", "--no-edit", "main"]);

    let shown = status(&root);
    assert!(shown.contains(&exit_after(2)), "{shown}");
    assert!(!shown.contains("Then: atelier"), "{shown}");
    submit_refused(&root, "p1", 2);
    let merge = atelier(&root, "p1", &["merge"]);
    assert!(merge.stderr.contains(&exit_after(2)), "{}", merge.stderr);
    assert!(
        !merge.stderr.contains("Run `atelier submit` here"),
        "{}",
        merge.stderr
    );
}

#[test]
fn round_3_asking_for_changes_takes_the_item_to_the_missions_lead() {
    let scratch = agreed(&[]);
    let root = scratch.root();
    write_mission_item(&root, ID);
    edit(&root, &item_path(ID), "status: draft", "status: ready");
    on_base(&root, "Ready it");
    assert_eq!(atelier(&root, "t1", &["start", ID]).code, 0);
    write(&root, "hello.txt", "hello\n");
    commit(&root, "Say hello");
    finish_and_submit(&root, "t1");
    for round in 1..3 {
        let finding = request_changes(&root, round);
        let shown = atelier(&root, "t1", &["status"]).stdout;
        assert!(shown.ends_with("Then: atelier submit\n"), "{shown}");
        answer(&root, &finding);
        submit(&root);
    }
    let finding = request_changes(&root, 3);

    let reached =
        format!("Automated review of {ID} reached its 3-round cap: round 3 asked for changes.\n");
    let rulings = format!(
        "The lead rules one of: merge it as it stands with `ATELIER_SKIP=\"<ruling>\" atelier merge` on its branch; adjust its Scope or Done when; drop it with `atelier drop {ID} --reason \"<why>\"`; or fix it. Once the lead rules to fix it, the implementer fixes and commits, and `atelier submit` opens round 4.\n"
    );
    let waits = format!(
        "{reached}Take it to your mission's lead, or with no mission to the thread that started you, and wait for the ruling.\n{rulings}"
    );
    let for_lead = format!("{reached}Settling it is yours.\n{rulings}");
    let shown = atelier(&root, "t1", &["status"]).stdout;
    assert!(shown.ends_with(&waits), "{shown}");
    assert!(!shown.contains("Then: atelier submit"), "{shown}");
    assert!(shown.contains(&format!("Finding {finding}")), "{shown}");
    // A submit still lacking a response says the same.
    let refused = atelier(&root, "t1", &["submit"]);
    assert!(refused.stderr.contains(&waits), "{}", refused.stderr);
    assert!(
        !refused.stderr.contains("Then: atelier submit"),
        "{}",
        refused.stderr
    );
    // Code not committed is still named.
    write(&root, "hello.txt", "hello.\n");
    let shown = atelier(&root, "t1", &["status"]);
    assert_eq!(shown.code, 0, "{}", shown.stderr);
    assert!(
        shown.stdout.contains("Not committed: hello.txt"),
        "{}",
        shown.stdout
    );
    git(&root, &["checkout", "--", "hello.txt"]);

    let base = scratch.worktree("base");
    for at in [&root, &base] {
        let shown = atelier(at, "lead1", &["status"]).stdout;
        assert!(shown.contains(&for_lead), "{shown}");
    }
    let shown = atelier(&root, "t9", &["status"]).stdout;
    assert!(!shown.contains("cap"), "{shown}");

    // The lead rules to fix it: submit opens round 4, and nothing is capped.
    answer(&root, &finding);
    submit(&root);
    let shown = atelier(&root, "t1", &["status"]).stdout;
    assert!(
        shown.contains("in review, round 4") && !shown.contains("cap"),
        "{shown}"
    );
}
