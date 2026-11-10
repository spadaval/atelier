//! `show`, `list`, `lead` and the base's `status`, over one fixture: a
//! ready mission with items in several statuses, one started in a second
//! worktree, and a done mission with an answered question and a plan
//! review.

mod common;

use common::*;
use std::fs;
use std::path::{Path, PathBuf};

const READY_BY: &str = "\n  by: sumeet\n  at: 2026-11-10T09:00:00Z";

fn write(root: &Path, path: &str, text: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn mission(root: &Path, id: &str, status: &str, title: &str, body: &str) {
    let ready = if status == "draft" { " null" } else { READY_BY };
    write(
        root,
        &format!(".atelier/missions/{id}/mission.md"),
        &format!(
            "---\nschema: atelier.mission\nschema_version: 3\nid: {id}\ntitle: {title}\n\
             status: {status}\nlabels: []\nlead: null\nready:{ready}\n\
             created_at: 2026-11-10T09:00:00Z\n---\n{body}"
        ),
    );
}

/// An item; `fields` are its mission, labels, priority and blockers as
/// front-matter values.
fn item(root: &Path, id: &str, status: &str, title: &str, fields: [&str; 4], body: &str) {
    let [mission, labels, priority, blocked_by] = fields;
    let ready = if status == "draft" { " null" } else { READY_BY };
    write(
        root,
        &item_path(id),
        &format!(
            "---\nschema: atelier.item\nschema_version: 3\nid: {id}\ntitle: {title}\n\
             status: {status}\nmission: {mission}\nlabels: {labels}\npriority: {priority}\n\
             blocked_by: {blocked_by}\nimplementers: []\nready:{ready}\ntrivial: null\n\
             caused_by: null\ncreated_at: 2026-11-10T09:00:00Z\n---\n\n## Goal\n\nDo it.\n\n\
             ## Done when\n\n- [ ] It is done.\n{body}"
        ),
    );
}

/// The fixture, with `demo-d4dd` started by t1 in the worktree `second`.
fn fixture() -> (Scratch, PathBuf) {
    let scratch = Scratch::new(&[]);
    let root = scratch.root();
    mission(
        &root,
        "demo-m1ss",
        "ready",
        "Greet everyone",
        "\n## Outcome\n\nEvery greeting works.\n\n## Success criteria\n\n- hello: `hello` prints hello.\n",
    );
    mission(
        &root,
        "demo-m2dn",
        "done",
        "Set up the project",
        "\n## Outcome\n\nA project exists.\n\n## Result\n\nIt exists.\n",
    );
    mission(
        &root,
        "demo-m3dr",
        "draft",
        "Say more",
        "\n## Outcome\n\nMore.\n",
    );
    // Ready, and with no started item, so not in progress.
    mission(
        &root,
        "demo-m4rd",
        "ready",
        "Tidy up",
        "\n## Outcome\n\nTidy.\n",
    );
    write(
        &root,
        ".atelier/missions/demo-m2dn/reviews/1-k3fa.md",
        "---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: 0123456789abcdef\n\
         reviewers:\n- r9\nmodel: gpt-6.1-sol\nverdict: approve\ncreated_at: 2026-11-10T09:00:00Z\n\
         ---\n\n## Summary\n\nFine.\n",
    );
    let m1 = "demo-m1ss";
    let settled =
        "\n## Decisions\n\n### 2026-11-08: Which greeting first?\n\nAnswer by lead1: hello.\n";
    let a1 = [m1, "[ui]", "high", "[]"];
    item(&root, "demo-a1aa", "ready", "Say hello", a1, settled);
    let blocked = [m1, "[]", "medium", "[demo-a1aa]"];
    item(&root, "demo-b2bb", "ready", "Say goodbye", blocked, "");
    item(
        &root,
        "demo-c3cc",
        "draft",
        "Say hi",
        [m1, "[]", "medium", "[]"],
        "",
    );
    let question = "\n## Decisions\n\n### 2026-11-10: Should waving be animated?\n";
    item(
        &root,
        "demo-d4dd",
        "ready",
        "Wave",
        [m1, "[ui]", "medium", "[]"],
        question,
    );
    let answered = "\n## Report\n\nCreated.\n\n## Decisions\n\n### 2026-11-09: Which license?\n\nAnswer by lead1: MIT.\n";
    let done = ["demo-m2dn", "[]", "low", "[]"];
    item(
        &root,
        "demo-e5ee",
        "done",
        "Create the repository",
        done,
        answered,
    );
    let human = ["null", "[needs-human]", "medium", "[]"];
    item(&root, "demo-f6ff", "ready", "Choose a name", human, "");
    let cli = ["null", "[cli]", "medium", "[]"];
    item(&root, "demo-g7gg", "ready", "Add a version flag", cli, "");
    let dropped = "\n## Report\n\nDropped: nobody uses it.\n";
    let none = ["null", "[]", "medium", "[]"];
    item(&root, "demo-h8hh", "dropped", "Port it", none, dropped);
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "Plan"]);
    let second = scratch.worktree("second");
    let run = atelier(&second, "t1", &["start", "demo-d4dd"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    (scratch, second)
}

/// What Git prints, without trimming its first line's leading spaces.
fn git_output_text(at: &Path, args: &[&str]) -> String {
    let output = git_output(at, args);
    String::from_utf8(output.stdout)
        .unwrap()
        .trim_end()
        .to_string()
}

fn stdout(at: &Path, thread: &str, args: &[&str]) -> String {
    let run = atelier(at, thread, args);
    assert_eq!(run.code, 0, "{args:?}: {}", run.stderr);
    run.stdout
}

const M1: &str = "Mission demo-m1ss (ready, in progress): Greet everyone\n";
const A1: &str = "    demo-a1aa (ready, high, can start now): Say hello\n";
const B2: &str =
    "    demo-b2bb (ready, medium, blocked by demo-a1aa, which is ready): Say goodbye\n";
const C3: &str = "    demo-c3cc (draft, medium): Say hi\n";
const D4: &str = "    demo-d4dd (building, medium, held by t1): Wave\n";
const F6: &str = "    demo-f6ff (ready, medium, waits on a human): Choose a name\n";
const G7: &str = "    demo-g7gg (ready, medium, can start now): Add a version flag\n";

#[test]
fn list_shows_open_work_by_mission_and_label_and_each_flag_narrows_it() {
    let (scratch, second) = fixture();
    let root = scratch.root();
    let list = |args: &[&str]| stdout(&root, "p1", &[&["list"], args].concat());

    // A started item is read from its branch, wherever `list` runs.
    let all = format!(
        "{M1}  No label:\n{B2}{C3}  Labelled ui:\n{A1}{D4}Mission demo-m3dr (draft): Say more\nMission demo-m4rd (ready): Tidy up\n\
         In no mission:\n  Labelled cli:\n{G7}  Labelled needs-human:\n{F6}"
    );
    assert_eq!(list(&[]), all);
    assert_eq!(stdout(&second, "p1", &["list"]), all);

    assert_eq!(
        list(&["--ready"]),
        format!("{M1}  Labelled ui:\n{A1}In no mission:\n  Labelled cli:\n{G7}  Labelled needs-human:\n{F6}")
    );
    assert_eq!(list(&["--blocked"]), format!("{M1}  No label:\n{B2}"));
    assert_eq!(
        list(&["--needs-human"]),
        format!("In no mission:\n  Labelled needs-human:\n{F6}")
    );
    assert_eq!(
        list(&["--mission", "demo-m1ss"]),
        format!("{M1}  No label:\n{B2}{C3}  Labelled ui:\n{A1}{D4}")
    );
    assert_eq!(
        list(&["--label", "ui"]),
        format!("{M1}  Labelled ui:\n{A1}{D4}")
    );
    // Flags narrow together.
    assert_eq!(
        list(&["--ready", "--label", "ui"]),
        format!("{M1}  Labelled ui:\n{A1}")
    );
    assert_eq!(list(&["--ready", "--blocked"]), "No open work matches.\n");
    // A finished mission and its items, and a dropped item.
    let finished = "Mission demo-m2dn (done): Set up the project\n  No label:\n    \
                    demo-e5ee (done, low): Create the repository\n\
                    In no mission:\n  No label:\n    demo-h8hh (dropped, medium): Port it\n";
    assert_eq!(list(&["--done"]), finished);
    assert_eq!(
        list(&["--done", "--mission", "demo-m1ss"]),
        "Nothing finished matches.\n"
    );

    // Mine: what the thread holds as implementer, then as lead.
    let mine = format!("{M1}  Labelled ui:\n{D4}");
    assert_eq!(stdout(&root, "t1", &["list", "--mine"]), mine);
    assert_eq!(list(&["--mine"]), "No open work matches.\n");
    let run = atelier(&root, "lead1", &["lead", "demo-m1ss"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    git(&root, &["commit", "-qam", "Lead demo-m1ss"]);
    let led = M1.replace("in progress)", "in progress, led by lead1)");
    assert_eq!(
        stdout(&root, "lead1", &["list", "--mine"]),
        format!("{led}  No label:\n{B2}{C3}  Labelled ui:\n{A1}{D4}")
    );
}

#[test]
fn list_mine_keeps_the_review_a_thread_holds() {
    let scratch = submitted();
    let root = scratch.root();
    assert_eq!(
        stdout(&root, "r1", &["list", "--mine"]),
        "No open work matches.\n"
    );
    assert_eq!(atelier(&root, "r1", &["review", ID]).code, 0);
    assert_eq!(
        stdout(&root, "r1", &["list", "--mine"]),
        "In no mission:\n  No label:\n    demo-ab12 (in review, high, held by t1, review held by r1): Say hello\n"
    );
}

#[test]
fn status_on_the_base_shows_what_can_start_the_missions_in_progress_and_what_waits_on_a_human() {
    let (scratch, _second) = fixture();
    let root = scratch.root();
    assert_eq!(
        stdout(&root, "p1", &["status"]),
        "You are thread p1, on the base branch main.\n\
         Ready to start:\n  demo-a1aa (high): Say hello\n  demo-g7gg (medium): Add a version flag\n  \
         demo-b2bb (medium): Say goodbye\n    [ ] demo-a1aa is done (it is ready)\n\
         Held by other threads:\n  demo-d4dd (building, held by t1): Wave\n\
         Missions in progress:\n  demo-m1ss (ready, in progress): Greet everyone\n\
         Waiting on a human (labelled needs-human):\n  \
         demo-f6ff (ready, medium, waits on a human): Choose a name\n\
         Then: atelier start demo-a1aa\n"
    );
}

#[test]
fn lead_takes_a_ready_mission_and_hands_each_item_that_can_start_to_another_thread() {
    let (scratch, second) = fixture();
    let root = scratch.root();
    let path = ".atelier/missions/demo-m1ss/mission.md";

    // Not a ready mission: refused, naming `ready`, and nothing written.
    for (id, said) in [
        ("demo-m3dr", "Mission demo-m3dr is draft, and `atelier lead` runs on a mission that is ready. A planner makes it ready with: atelier ready demo-m3dr --by <name>\n"),
        ("demo-m2dn", "Mission demo-m2dn is done, and `atelier lead` runs on a mission that is ready. Nothing is left to lead on it.\n"),
        ("demo-a1aa", "No mission demo-a1aa is in this checkout. `atelier list` shows the missions.\n"),
    ] {
        let run = atelier(&root, "lead1", &["lead", id]);
        assert_eq!(run.code, 1, "{id}");
        assert_eq!(run.stderr, format!("`atelier lead {id}` refused; nothing changed.\n{said}"));
    }
    // Planning happens on the base.
    let run = atelier(&second, "lead1", &["lead", "demo-m1ss"]);
    assert_eq!(run.code, 1);
    assert!(
        run.stderr.starts_with("`atelier lead` writes planning"),
        "{}",
        run.stderr
    );
    assert_eq!(git(&root, &["status", "--porcelain"]), "");

    // An item made ready and not committed is not handed out: `start`
    // reads the base's last commit, and would refuse it.
    let run = atelier(&root, "p1", &["ready", "demo-c3cc", "--by", "sumeet"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let items = "Its items:\n  Draft:\n    demo-c3cc (medium): Say hi\n  Ready:\n    \
                 demo-a1aa (high, can start now): Say hello\n    \
                 demo-b2bb (medium, blocked by demo-a1aa, which is ready): Say goodbye\n  \
                 Building:\n    demo-d4dd (medium, held by t1): Wave\n";
    let handoff = "An implementer must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one with your harness's tool for a new agent thread, and give it this task:\n  \
                   Run atelier start demo-a1aa and follow what it prints.\n\
                   Then wait for it to finish, and run `atelier status`.\n";
    let then = format!(
        "Wrote, and did not commit:\n  {path}\nPlanning counts once it is committed on the base.\n\
         Then: git add -- {path} && git commit -m \"Lead demo-m1ss\" -- {path}\n"
    );
    assert_eq!(
        stdout(&root, "lead1", &["lead", "demo-m1ss"]),
        format!("You now lead mission demo-m1ss: Greet everyone\nNobody led it before.\n{items}{handoff}{then}")
    );
    assert_eq!(front(&read(&root, path), "lead"), "lead1");
    // It commits nothing.
    let changed = format!(" M {}\n M {path}", item_path("demo-c3cc"));
    assert_eq!(git_output_text(&root, &["status", "--porcelain"]), changed);
    git(&root, &["commit", "-qm", "Lead demo-m1ss", "--", path]);
    git(&root, &["checkout", "--", &item_path("demo-c3cc")]);

    // The lead's status: its mission and what each item needs.
    let status = stdout(&root, "lead1", &["status"]);
    let tail = format!(
        "You lead mission demo-m1ss: Greet everyone\n{}{handoff}",
        &items[11..]
    );
    assert!(status.ends_with(&tail), "{status}");
    assert!(
        status.contains("  demo-m1ss (ready, in progress, led by lead1): Greet everyone\n"),
        "{status}"
    );

    // Another thread takes it over, and is told who led it.
    let taken = stdout(&root, "lead2", &["lead", "demo-m1ss"]);
    assert!(
        taken.starts_with(
            "You now lead mission demo-m1ss: Greet everyone\nThread lead1 led it before.\n"
        ),
        "{taken}"
    );
    // The lead running it again changes nothing, so has nothing to commit.
    git(&root, &["commit", "-qam", "Lead demo-m1ss"]);
    let again = stdout(&root, "lead2", &["lead", "demo-m1ss"]);
    assert!(
        again.starts_with("You already lead mission demo-m1ss: Greet everyone\n"),
        "{again}"
    );
    assert!(!again.contains("Wrote"), "{again}");
    assert_eq!(git(&root, &["status", "--porcelain"]), "");
}

#[test]
fn show_prints_a_mission_on_one_page_with_its_items_questions_and_plan_reviews() {
    let (scratch, _second) = fixture();
    let root = scratch.root();
    assert_eq!(
        stdout(&root, "p1", &["show", "demo-m1ss"]),
        "Mission demo-m1ss: Greet everyone\nStatus: ready, in progress\nLead: nobody\nLabels: none\n\
         Made ready by sumeet at 2026-11-10 09:00:00 +00:00\n\n\
         ## Outcome\n\nEvery greeting works.\n\n## Success criteria\n\n- hello: `hello` prints hello.\n\n\
         Items:\n  Draft:\n    demo-c3cc (medium): Say hi\n  Ready:\n    \
         demo-a1aa (high, can start now): Say hello\n    \
         demo-b2bb (medium, blocked by demo-a1aa, which is ready): Say goodbye\n  \
         Building:\n    demo-d4dd (medium, held by t1): Wave\n\n\
         Questions:\n  demo-d4dd: 2026-11-10: Should waving be animated? (unanswered)\n  \
         demo-a1aa: 2026-11-08: Which greeting first? Answer by lead1: hello.\n\n\
         Plan reviews:\n  None.\n"
    );
    // A done mission, with its Result, its answered question and its plan
    // review.
    assert_eq!(
        stdout(&root, "p1", &["show", "demo-m2dn"]),
        "Mission demo-m2dn: Set up the project\nStatus: done\nLead: nobody\nLabels: none\n\
         Made ready by sumeet at 2026-11-10 09:00:00 +00:00\n\n\
         ## Outcome\n\nA project exists.\n\n## Result\n\nIt exists.\n\n\
         Items:\n  Done:\n    demo-e5ee (low): Create the repository\n\n\
         Questions:\n  demo-e5ee: 2026-11-09: Which license? Answer by lead1: MIT.\n\n\
         Plan reviews:\n  Round 1, review 1-k3fa by r9, gpt-6.1-sol, of commit 0123456: approved\n"
    );
    let run = atelier(&root, "p1", &["show", "demo-zz99"]);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stderr,
        "No item or mission demo-zz99 is in main's last commit or on a branch named demo-zz99. Nothing changed.\n`atelier list` lists the open work.\n"
    );
}

#[test]
fn show_prints_a_started_item_from_its_branch_wherever_it_runs_and_a_done_one() {
    let (scratch, second) = fixture();
    let root = scratch.root();
    let header = |done_when: &str| {
        format!(
            "Item demo-d4dd: Wave\nStatus: building, held by t1\nMission: demo-m1ss (ready)\n\
             Labels: ui\nPriority: medium\nBlocked by: none\n\
             Made ready by sumeet at 2026-11-10 09:00:00 +00:00\n\n\
             ## Goal\n\nDo it.\n\n## Done when\n\n- [{done_when}] It is done.\n\n\
             ## Decisions\n\n### 2026-11-10: Should waving be animated?\n\nReviews: none yet.\n"
        )
    };
    assert_eq!(stdout(&root, "p1", &["show", "demo-d4dd"]), header(" "));
    // On its own branch the checkout counts, so an uncommitted tick shows
    // there; elsewhere the branch's tip does.
    let path = second.join(item_path("demo-d4dd"));
    let ticked = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
    fs::write(&path, ticked).unwrap();
    assert_eq!(stdout(&second, "p1", &["show", "demo-d4dd"]), header("x"));
    assert_eq!(stdout(&root, "p1", &["show", "demo-d4dd"]), header(" "));

    let done = stdout(&root, "p1", &["show", "demo-e5ee"]);
    assert!(
        done.starts_with(
            "Item demo-e5ee: Create the repository\nStatus: done\nMission: demo-m2dn (done)\n"
        ),
        "{done}"
    );
    assert!(done.contains("## Report\n\nCreated.\n"), "{done}");
    let blocked = stdout(&root, "p1", &["show", "demo-b2bb"]);
    assert!(
        blocked.contains("\nBlocked by: demo-a1aa (ready)\n"),
        "{blocked}"
    );
}

#[test]
fn show_prints_each_review_by_round_with_its_findings_and_whether_each_is_open() {
    let scratch = submitted();
    let root = scratch.root();
    assert_eq!(atelier(&root, "r1", &["review", ID]).code, 0);
    let (stem, path) = review_of(&root, 1);
    append(
        &root,
        &path,
        "\n## Finding 1: No test\nRecommendation: fix now\n\n## Finding 2: Later\nRecommendation: defer\n",
    );
    let run = atelier(
        &root,
        "r1",
        &["verdict", "changes", "--model", "gpt-6.1-sol"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    let commit = front(&read(&root, &path), "commit");
    let show = stdout(&root, "p1", &["show", ID]);
    assert!(
        show.starts_with("Item demo-ab12: Say hello\nStatus: changes requested, held by t1\n"),
        "{show}"
    );
    assert!(
        show.ends_with(&format!(
            "\nReviews:\n  Round 1, review {stem} by r1, gpt-6.1-sol, of commit {}: changes requested\n    \
             {stem}.1 (fix now, open): No test\n    {stem}.2 (defer): Later\n",
            &commit[..7]
        )),
        "{show}"
    );
}
