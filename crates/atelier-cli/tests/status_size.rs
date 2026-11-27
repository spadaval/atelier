//! What `status` prints first, and how much, for a mission's lead and for
//! a thread holding an item: what waits on the reader comes before what
//! does not, the backlog comes last, and a lead's whole status fits what a
//! harness shows of it.

mod common;

use common::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const LEAD: &str = "lead1";

/// A thread ID as long as the ones T3 Code gives a delegated thread.
fn holder(n: usize) -> String {
    format!(
        "thread:delegated-task:command%3Amcp%3A634c0de9-5797-4092-bc79-ae7c0a91aed5%3Adelegate-task%3A{n:08}-71eb-44c7-900e-242ea74e5fe8"
    )
}

/// One item of a fixture: its ID, mission, status, holder, blockers,
/// number of Done when lines and title.
struct Spec<'a> {
    id: &'a str,
    mission: Option<&'a str>,
    status: &'a str,
    holder: Option<String>,
    blocked_by: &'a [&'a str],
    done_when: usize,
    title: &'a str,
    labels: &'a [&'a str],
}

#[allow(clippy::too_many_arguments)]
fn spec<'a>(
    id: &'a str,
    mission: Option<&'a str>,
    status: &'a str,
    holder: Option<usize>,
    blocked_by: &'a [&'a str],
    done_when: usize,
    title: &'a str,
) -> Spec<'a> {
    Spec {
        id,
        mission,
        status,
        holder: holder.map(self::holder),
        blocked_by,
        done_when,
        title,
        labels: &[],
    }
}

fn item_text(spec: &Spec, status: &str) -> String {
    let list = |values: &[String]| -> String {
        match values.is_empty() {
            true => " []".into(),
            false => values.iter().map(|v| format!("\n- {v}")).collect(),
        }
    };
    let blocked: Vec<String> = spec.blocked_by.iter().map(|s| s.to_string()).collect();
    let implementers: Vec<String> = match status {
        "open" => Vec::new(),
        _ => spec.holder.iter().cloned().collect(),
    };
    let labels: Vec<String> = spec.labels.iter().map(|s| s.to_string()).collect();
    let mut done_when = String::new();
    for n in 0..spec.done_when {
        done_when += &format!(
            "- [ ] Done when: Line {n} of what the change must do, written out in full as a \
             plan writes it, naming the routes, the commands, the files and the tests that \
             prove each part of it, so a reviewer can judge it line by line.\n"
        );
    }
    format!(
        "---\nschema: atelier.item\nschema_version: 3\nid: {}\ntitle: {}\nstatus: {status}\n\
         mission: {}\nlabels:{}\npriority: medium\nblocked_by:{}\nimplementers:{}\nready: null\n\
         trivial: null\ncaused_by: null\ncreated_at: 2026-11-20T09:00:00Z\n---\n\n## Goal\n\n\
         Do it.\n\n## Done when\n\n{done_when}\n## Scope\n\nOnly it.\n\n## Report\n\n",
        spec.id,
        quoted(spec.title),
        spec.mission.unwrap_or("null"),
        list(&labels),
        list(&blocked),
        list(&implementers),
    )
}

/// `text` as a YAML string in single quotes.
fn quoted(text: &str) -> String {
    format!("'{}'", text.replace('\'', "''"))
}

fn write(root: &Path, path: &str, text: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn mission_text(id: &str, lead: &str, title: &str) -> String {
    format!(
        "---\nschema: atelier.mission\nschema_version: 3\nid: {id}\ntitle: {title}\n\
         status: open\nlabels: []\nlead: {lead}\nplanners:\n- p1\n\
         created_at: 2026-11-20T09:00:00Z\n---\n\n## Outcome\n\nIt is done.\n",
        title = quoted(title)
    )
}

/// Commits everything in `root` at committer time `at`, and moves `main`
/// to it.
fn commit_at(root: &Path, message: &str, at: &str) {
    git(root, &["add", "-A"]);
    let run = Command::new("git")
        .args(["commit", "-qm", message])
        .env("GIT_COMMITTER_DATE", at)
        .env("GIT_AUTHOR_DATE", at)
        .current_dir(root)
        .output()
        .unwrap();
    assert!(run.status.success(), "{run:?}");
    git(root, &["branch", "-f", "main", "HEAD"]);
}

/// Writes `missions` and `items` on the base, open where started, then
/// starts each started one on a branch of its name, as `start` leaves it.
/// Returns a checkout of the base made after.
fn build(scratch: &Scratch, missions: &[(&str, &str, &str)], items: &[Spec]) -> PathBuf {
    let root = scratch.root();
    for (id, lead, title) in missions {
        write(
            &root,
            &format!(".atelier/missions/{id}/mission.md"),
            &mission_text(id, lead, title),
        );
    }
    for spec in items {
        let status = match spec.status {
            "done" | "open" => spec.status,
            _ => "open",
        };
        write(&root, &item_path(spec.id), &item_text(spec, status));
    }
    commit_at(&root, "Plan", "2026-11-20T09:00:00Z");
    for spec in items
        .iter()
        .filter(|s| !matches!(s.status, "done" | "open"))
    {
        git(&root, &["checkout", "-q", "-b", spec.id, "main"]);
        write(&root, &item_path(spec.id), &item_text(spec, spec.status));
        git(&root, &["add", "-A"]);
        git(&root, &["commit", "-qm", &format!("Start {}", spec.id)]);
        git(&root, &["checkout", "-q", SCRATCH]);
    }
    scratch.worktree("base")
}

/// A review of round `round` of item `id`, opened at `created` and holding
/// `findings`, each a title, a recommendation, and the finding it carries.
fn review(
    root: &Path,
    id: &str,
    name: &str,
    verdict: &str,
    created: &str,
    findings: &[(&str, &str, Option<&str>)],
) {
    let round = name.split('-').next().unwrap();
    let commit = git(root, &["rev-parse", "HEAD"]);
    let mut body = String::new();
    for (n, (title, recommendation, was)) in findings.iter().enumerate() {
        body += &format!(
            "\n## Finding {}: {title}\nRecommendation: {recommendation}\n",
            n + 1
        );
        if let Some(was) = was {
            body += &format!(
                "Was: {was}\nResponse: fixed in {}\nNow: fixed\n",
                &commit[..8]
            );
        }
        body += "\nWhat it is.\n";
    }
    write(
        root,
        &format!(".atelier/items/{id}/reviews/{name}.md"),
        &format!(
            "---\nschema: atelier.review\nschema_version: 3\nround: {round}\ncommit: {commit}\n\
             reviewers:\n- r{round}\nmodel: null\nverdict: {verdict}\ncreated_at: {created}\n---\n\n\
             ## Summary\n\nRead it.\n{body}"
        ),
    );
}

const TITLES: [&str; 12] = [
    "Record acceptance against an exact asset version or experience build, apart from review",
    "Make If-Match optional on every route and --revision optional on every command",
    "Hash a file once on upload and once on download, and trust a stored blob of the right size",
    "Attach reviews only to asset versions and builds, and have the server compute the verdict",
    "Delete asset-to-asset dependency pins, which nothing loads, and the table that holds them",
    "Give each asset version runtime, source and preview representations",
    "Record a repository and commit on builds and asset versions, and allow a build with no files",
    "Replace task candidates with one delivered result on the task, and start new tasks as ready",
    "Show a preview in the dashboard when a model is too large for the 3D viewer",
    "Write every record reference one way across the hub CLI, by name or by id",
    "Split the hub CLI into one module per command group so parallel work stops colliding",
    "Rename asset revisions to versions so that revision only means a record's edit counter",
];

/// A twelve-item mission led by [`LEAD`]: two items done after timed
/// review rounds, three building and two in review in other threads, and
/// five open, each blocked. One other item is open in the backlog, and
/// `t9` holds another.
fn twelve() -> (Scratch, PathBuf) {
    let scratch = Scratch::new(&[]);
    let m = Some("demo-m1ss");
    let spec =
        |id, status, holder, blocked_by, title| spec(id, m, status, holder, blocked_by, 7, title);
    let items = [
        spec("demo-d001", "done", None, &[], TITLES[10]),
        spec("demo-d002", "done", None, &[], TITLES[11]),
        spec("demo-b001", "building", Some(1), &["demo-d001"], TITLES[0]),
        spec("demo-b002", "building", Some(2), &["demo-d001"], TITLES[1]),
        spec("demo-b003", "building", Some(3), &["demo-d002"], TITLES[2]),
        spec("demo-r001", "in_review", Some(4), &[], TITLES[3]),
        spec("demo-r002", "in_review", Some(5), &[], TITLES[4]),
        spec("demo-o001", "open", None, &["demo-b001"], TITLES[5]),
        spec("demo-o002", "open", None, &["demo-o001"], TITLES[6]),
        spec(
            "demo-o003",
            "open",
            None,
            &["demo-b002", "demo-b003", "demo-r001", "demo-o002"],
            TITLES[7],
        ),
        spec(
            "demo-o004",
            "open",
            None,
            &["demo-o001", "demo-o002"],
            TITLES[8],
        ),
        spec("demo-o005", "open", None, &["demo-r002"], TITLES[9]),
        Spec {
            id: "demo-x001",
            mission: None,
            status: "open",
            holder: None,
            blocked_by: &[],
            done_when: 2,
            title: "Say goodbye when the last item of a mission merges, and not before",
            labels: &[],
        },
        Spec {
            id: "demo-x002",
            mission: None,
            status: "building",
            holder: Some("t9".into()),
            blocked_by: &[],
            done_when: 2,
            title: "Say hello",
            labels: &[],
        },
    ];
    let root = scratch.root();
    // The done items' rounds: 40 and 15 minutes for one, 25 for the other.
    review(
        &root,
        "demo-d001",
        "1-aaaa",
        "changes",
        "2026-11-20T10:00:00Z",
        &[
            ("No test", "fix now", None),
            ("Wrong name", "fix now", None),
        ],
    );
    commit_at(&root, "Review round 1", "2026-11-20T10:40:00Z");
    review(
        &root,
        "demo-d001",
        "2-bbbb",
        "approve",
        "2026-11-20T11:00:00Z",
        &[
            ("No test", "fix now", Some("1-aaaa.1")),
            ("Wrong name", "fix now", Some("1-aaaa.2")),
            ("A comment could be shorter", "defer", None),
        ],
    );
    commit_at(&root, "Review round 2", "2026-11-20T11:15:00Z");
    // demo-d002's review opens at 12:00, and its verdict commits at 12:25.
    review(
        &root,
        "demo-d002",
        "1-cccc",
        "null",
        "2026-11-20T12:00:00Z",
        &[],
    );
    commit_at(&root, "Open demo-d002's review", "2026-11-20T12:00:00Z");
    let path = root.join(".atelier/items/demo-d002/reviews/1-cccc.md");
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text.replace("verdict: null", "verdict: approve")).unwrap();
    commit_at(&root, "Approve demo-d002", "2026-11-20T12:25:00Z");
    let base = build(
        &scratch,
        &[("demo-m1ss", LEAD, "Give the Hub one model in studio terms")],
        &items,
    );
    (scratch, base)
}

fn status(at: &Path, thread: &str) -> String {
    let run = atelier(at, thread, &["status"]);
    assert_eq!(run.code, 0, "{}{}", run.stdout, run.stderr);
    run.stdout
}

/// The byte offset of `part` in `text`, which must hold it.
fn at(text: &str, part: &str) -> usize {
    text.find(part)
        .unwrap_or_else(|| panic!("no {part:?} in:\n{text}"))
}

#[test]
fn a_leads_status_of_a_twelve_item_mission_is_under_2_kb_with_what_waits_on_it_first() {
    let (_scratch, base) = twelve();
    let shown = status(&base, LEAD);
    assert!(shown.len() < 2048, "{} bytes:\n{shown}", shown.len());
    // What waits on the lead, then its mission, then the backlog.
    let waits = at(
        &shown,
        "Nothing in the missions you lead waits on you now.\n",
    );
    let mission = at(&shown, "You lead mission demo-m1ss: ");
    let backlog = at(
        &shown,
        "The backlog, which waits on nobody in particular:\n",
    );
    assert!(waits < mission && mission < backlog, "{shown}");
    assert!(at(&shown, "  demo-x001 (medium): ") > backlog, "{shown}");
    // One line an item not done, with neither its holder nor its Done when
    // lines, and the done ones counted.
    for id in [
        "demo-b001",
        "demo-b002",
        "demo-b003",
        "demo-r001",
        "demo-o005",
    ] {
        assert_eq!(shown.matches(&format!("  {id} (")).count(), 1, "{shown}");
    }
    assert!(!shown.contains("Done when: Line"), "{shown}");
    assert!(!shown.contains("%3Adelegate-task"), "{shown}");
    assert!(shown.contains("  And 2 done.\n"), "{shown}");
    assert!(
        shown.contains("  demo-o003 (open, medium, blocked by 4 items): "),
        "{shown}"
    );
    assert!(
        shown.contains("  demo-o001 (open, medium, blocked by demo-b001): "),
        "{shown}"
    );
    // The mission's items are not shown again in the backlog, which holds
    // the rest.
    let rest = &shown[backlog..];
    assert!(
        !rest.contains("demo-b00") && !rest.contains("demo-o00"),
        "{shown}"
    );
    assert!(!rest.contains("cannot start yet"), "{shown}");
    assert!(
        rest.contains("Held by other threads:\n  demo-x002 (building, held by t9): Say hello\n"),
        "{shown}"
    );
}

#[test]
fn a_leads_status_names_the_commands_that_show_what_it_leaves_out() {
    let (_scratch, base) = twelve();
    let shown = status(&base, LEAD);
    assert!(
        shown.contains(
            "Each item's holder, blockers and Done when lines: atelier show <item>.\n\
             You lead mission demo-m1ss: "
        ),
        "{shown}"
    );
    // It has what `status` leaves out: the holder, each blocker with its
    // status, and the Done when lines.
    let item = atelier(&base, LEAD, &["show", "demo-b001"]).stdout;
    assert!(item.contains(&holder(1)), "{item}");
    assert!(item.contains("Done when: Line 6 of what"), "{item}");
    let item = atelier(&base, LEAD, &["show", "demo-o003"]).stdout;
    assert!(
        item.contains("demo-b002") && item.contains("building"),
        "{item}"
    );
}

#[test]
fn a_leads_status_says_its_review_pace_as_a_count_of_the_records_gives_it() {
    let (_scratch, base) = twelve();
    let shown = status(&base, LEAD);
    // Two items in three rounds; rounds of 40, 15 and 25 minutes, from each
    // review's opening to the commit of its verdict; round 2 of demo-d001
    // carried two findings and raised one, to defer.
    assert!(
        shown.contains(
            "\n  Review pace: 3 rounds for 2 items, 1.5 each; a round took 25m at the median, \
             40m at most; later rounds found 1 new finding, 0 to fix now.\n"
        ),
        "{shown}"
    );
}

#[test]
fn a_thread_holding_an_item_reads_it_and_its_way_there_before_the_backlog() {
    let (_scratch, base) = twelve();
    let shown = status(&base, "t9");
    let held = at(&shown, "You hold:\n  demo-x002 (building): Say hello\n");
    let then = at(&shown, "\nThen: ");
    let backlog = at(
        &shown,
        "The backlog, which waits on nobody in particular:\n",
    );
    assert!(held < then && then < backlog, "{shown}");
    // It leads nothing, so the lead's mission's items are its backlog.
    assert!(at(&shown, "Held by other threads:\n") > backlog, "{shown}");
    assert!(!shown.contains("You lead"), "{shown}");
}

#[test]
fn a_lead_with_items_that_can_start_is_handed_them_before_its_missions() {
    let (scratch, _base) = twelve();
    let root = scratch.root();
    // demo-o005 waited on demo-r002; once that is done, it can start.
    let path = root.join(item_path("demo-r002"));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text.replace("status: open", "status: done")).unwrap();
    commit_at(&root, "demo-r002 done", "2026-11-20T13:00:00Z");
    git(&root, &["branch", "-D", "demo-r002"]);
    let base = scratch.worktree("base2");
    let shown = status(&base, LEAD);
    let handoff = at(
        &shown,
        "  Run atelier start demo-o005 and follow what it prints.\n",
    );
    assert!(handoff < at(&shown, "You lead mission"), "{shown}");
    assert!(
        shown.contains("  demo-o005 (open, medium, can start now): "),
        "{shown}"
    );
}

/// The Hub's state behind its lead's `status` of 17,870 bytes: three
/// missions led, eight items held by other threads, each with its Done
/// when lines, and blockers chained across the missions.
fn hub() -> (Scratch, PathBuf) {
    let scratch = Scratch::new(&[]);
    let (a, b, c) = (
        Some("hub-v2-ljxh"),
        Some("hub-v2-n8no"),
        Some("hub-v2-rqpi"),
    );
    let items = [
        spec("hub-v2-5lw4", a, "open", None, &["hub-v2-l0q8"], 5, "Give each asset version runtime, source and preview representations"),
        spec("hub-v2-rv5m", a, "open", None, &["hub-v2-wcx8", "hub-v2-uh2t", "hub-v2-w7sd", "hub-v2-vkx4"], 5, "Replace task candidates with one delivered result on the task, and start new tasks as ready"),
        spec("hub-v2-uh2t", a, "open", None, &["hub-v2-5lw4"], 5, "Record a repository and commit on builds and asset versions, and allow a build with no files"),
        spec("hub-v2-6t8y", a, "building", Some(1), &["hub-v2-3wvd"], 7, "Make asset and experience names unique per project and accept a name wherever an id is taken"),
        spec("hub-v2-l0q8", a, "building", Some(2), &["hub-v2-3wvd"], 6, "Delete asset-to-asset dependency pins, which nothing loads"),
        spec("hub-v2-vkx4", a, "building", Some(3), &["hub-v2-3wvd"], 6, "Make If-Match optional on every route and --revision optional on every command"),
        spec("hub-v2-w7sd", a, "building", Some(4), &["hub-v2-3wvd"], 7, "Record acceptance against an exact asset version or experience build"),
        spec("hub-v2-wcx8", a, "building", Some(5), &["hub-v2-3wvd"], 8, "Attach reviews only to asset versions and builds, and have the server compute the one review verdict"),
        spec("hub-v2-3wvd", a, "in_review", Some(6), &[], 6, "Rename asset revisions to versions so that revision only means a record's edit counter"),
        spec("hub-v2-3c0x", a, "done", None, &[], 4, "Split the hub CLI into one module per command group so parallel work stops colliding in cli/index.ts"),
        spec("hub-v2-34sr", b, "open", None, &["hub-v2-7stp", "hub-v2-uh2t", "hub-v2-wcx8", "hub-v2-w7sd"], 5, "Show each pin's state with hub inventory, and record a build from the pin file"),
        spec("hub-v2-57r5", b, "open", None, &["hub-v2-5lw4", "hub-v2-6t8y", "hub-v2-uh2t", "hub-v2-vkx4"], 5, "Publish an asset version straight from any folder with hub publish"),
        spec("hub-v2-77tp", b, "open", None, &["hub-v2-57r5", "hub-v2-7stp", "hub-v2-rvyy", "hub-v2-34sr"], 5, "Remove hub workspace and document what lives in Git and what lives in the Hub"),
        spec("hub-v2-7stp", b, "open", None, &["hub-v2-524p", "hub-v2-5lw4", "hub-v2-6t8y"], 5, "Pin asset versions in a file in Git and load them into a checkout with hub load"),
        spec("hub-v2-rvyy", b, "open", None, &["hub-v2-57r5"], 5, "Give an agent a writable copy of an asset's source with hub work"),
        spec("hub-v2-q9ax", b, "building", Some(7), &[], 5, "Hash a file once on upload and once on download"),
        spec("hub-v2-524p", b, "in_review", Some(8), &[], 6, "Add a machine-local read-only blob cache that hard-links files into checkouts"),
        spec("hub-v2-17b7", c, "open", None, &["hub-v2-5lw4", "hub-v2-uh2t"], 5, "Show a preview in the dashboard when a model is too large for the 3D viewer"),
        spec("hub-v2-nrub", c, "open", None, &["hub-v2-6t8y", "hub-v2-w7sd", "hub-v2-rv5m"], 5, "Write every record reference one way across the hub CLI"),
        spec("hub-v2-on9g", c, "open", None, &["hub-v2-17b7", "hub-v2-5lw4", "hub-v2-wcx8", "hub-v2-w7sd", "hub-v2-uh2t", "hub-v2-rv5m"], 5, "Lead asset and build pages with version, review verdict and acceptance"),
        spec("hub-v2-rpxu", c, "open", None, &["hub-v2-nrub", "hub-v2-rv5m", "hub-v2-wcx8", "hub-v2-vkx4", "hub-v2-57r5"], 5, "Add the everyday verbs so a ready task reaches done in six commands"),
        spec("hub-v2-upo4", c, "open", None, &["hub-v2-rpxu", "hub-v2-6t8y", "hub-v2-wcx8", "hub-v2-w7sd", "hub-v2-rv5m", "hub-v2-7stp", "hub-v2-rvyy", "hub-v2-34sr"], 5, "Select the only project automatically and print names in place of UUIDs"),
        spec("hub-v2-cram", None, "open", None, &[], 4, "Bound what a model can make the 3D viewer decode"),
        spec("hub-v2-udwb", None, "open", None, &[], 4, "Keep a separate walkthrough screenshot for every asset route"),
        Spec {
            labels: &["needs-human"],
            ..spec("hub-v2-wn8l", None, "open", None, &[], 3, "Run the asset loader's acceptance tests on the Windows GPU machine")
        },
    ];
    let lead = "mcp:0571dde4-9f4f-4810-bbdd-fccef1c2604b";
    let base = build(
        &scratch,
        &[
            ("hub-v2-ljxh", lead, "Give the Hub one model in studio terms: versions, representations, acceptance and one review verdict"),
            ("hub-v2-n8no", lead, "Publish, load and edit assets from any folder, with one local copy shared by every checkout"),
            ("hub-v2-rqpi", lead, "Make everyday Hub work take six commands, and give large models something to look at in the dashboard"),
        ],
        &items,
    );
    (scratch, base)
}

#[test]
fn a_leads_status_of_the_hubs_largest_state_is_under_4_kb_in_the_same_order() {
    let (_scratch, base) = hub();
    let shown = status(&base, "mcp:0571dde4-9f4f-4810-bbdd-fccef1c2604b");
    assert!(shown.len() < 4096, "{} bytes:\n{shown}", shown.len());
    assert!(shown.starts_with(&format!(
        "{NO_HOOKS}You are thread mcp:0571dde4-9f4f-4810-bbdd-fccef1c2604b, on a detached HEAD.\n\
         Nothing in the missions you lead waits on you now.\nEach item's holder, blockers and \
         Done when lines: atelier show <item>.\nYou lead mission hub-v2-ljxh: "
    )));
    let missions: Vec<usize> = ["hub-v2-ljxh", "hub-v2-n8no", "hub-v2-rqpi"]
        .iter()
        .map(|id| at(&shown, &format!("You lead mission {id}: ")))
        .collect();
    let backlog = at(
        &shown,
        "The backlog, which waits on nobody in particular:\n",
    );
    assert!(missions.windows(2).all(|w| w[0] < w[1]), "{shown}");
    assert!(missions[2] < backlog, "{shown}");
    // Every item of the three missions not done, once each.
    for id in ["hub-v2-6t8y", "hub-v2-q9ax", "hub-v2-524p", "hub-v2-upo4"] {
        assert_eq!(shown.matches(&format!("{id} (")).count(), 1, "{shown}");
    }
    assert!(!shown.contains("%3Adelegate-task"), "{shown}");
    // The backlog holds only what is outside them.
    let rest = &shown[backlog..];
    assert!(rest.contains("  hub-v2-cram (medium): "), "{shown}");
    assert!(
        rest.contains("  hub-v2-wn8l (open, medium, waits on a human): "),
        "{shown}"
    );
    assert!(!rest.contains("Missions in progress:"), "{shown}");
    assert!(!rest.contains("Held by other threads:"), "{shown}");
}

#[test]
fn drops_of_other_items_print_after_a_leads_handoff_and_its_backlog() {
    let scratch = Scratch::new(&[]);
    let m = Some("demo-m1ss");
    let ids: Vec<String> = (0..40).map(|n| format!("demo-q{n:03}")).collect();
    let mut items = vec![
        spec("demo-a1aa", m, "open", None, &[], 3, TITLES[0]),
        Spec {
            labels: &["needs-human"],
            ..spec("demo-h1um", m, "open", None, &[], 3, TITLES[1])
        },
    ];
    for (n, id) in ids.iter().enumerate() {
        items.push(spec(id, None, "building", Some(n), &[], 3, TITLES[n % 12]));
    }
    let base = build(
        &scratch,
        &[("demo-m1ss", LEAD, "Give the Hub one model in studio terms")],
        &items,
    );
    // Each is dropped on the base while its branch of building is left.
    let root = scratch.root();
    for spec in &items[2..] {
        let text = item_text(spec, "dropped").replace(
            "## Report\n\n",
            "## Report\n\nDropped: Superseded by a later item that does the same thing.\n",
        );
        write(&root, &item_path(spec.id), &text);
    }
    commit_at(&root, "Drop forty", "2026-11-20T14:00:00Z");
    git(&base, &["checkout", "-q", "--detach", "main"]);
    let shown = status(&base, LEAD);
    let handoff = at(
        &shown,
        "  Run atelier start demo-a1aa and follow what it prints.\n",
    );
    let mission = at(&shown, "You lead mission demo-m1ss: ");
    let dropped = at(&shown, "Dropped on the base while started");
    assert!(
        handoff < 2048 && handoff < mission && mission < dropped,
        "{shown}"
    );
    // The needs-human item of the mission is shown once, in the mission.
    assert_eq!(shown.matches("demo-h1um (").count(), 1, "{shown}");
    assert!(!shown.contains("Waiting on a human"), "{shown}");
}
