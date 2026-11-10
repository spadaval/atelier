use atelier_core::{
    is_deferred, ForgeProvider, MissionStatus, Now, Outcome, Recommendation, Response, Settlement,
    Status, Thread, Verdict,
};
use atelier_records::create_review;
use atelier_records::{
    parse_config, parse_item, parse_mission, parse_review, render_config, render_item,
    render_mission, render_review, validate, ItemDir, Problem, Store,
};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

const ITEM: &str = "---
schema: atelier.item
schema_version: 3
id: atelier-06jc
title: 'Put open findings in full: the review brief'
status: changes_requested
mission: atelier-icv4
labels:
- review
priority: high
blocked_by:
- atelier-b10c
implementers:
- t3-implementer-1
- t3-implementer-2
ready:
  by: sumeet
  at: 2026-11-09T19:56:00-05:00
trivial: null
caused_by: atelier-x1y2
created_at: 2026-11-09T19:00:00.123456789Z
---

## Goal

Print open findings in full.

## Done when

- [x] The brief prints them.

## Report

It prints them.

### Responses

2-m2bn.1: fixed in 0123abc
- 2-m2bn.2: rebutted: the Scope is printed by `show`

## Decisions

### 2026-11-10: Should the brief carry the parent's Scope?

Finding: 2-m2bn.2

Answer by t3-decider: No; the item's own Scope is enough.
Outcome: no action
";

const BLOCKER: &str = "---
schema: atelier.item
schema_version: 3
id: atelier-b10c
title: Blocker
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-09T19:00:00-05:00
---
## Goal
g
## Done when
d
";

const MISSION: &str = "---
schema: atelier.mission
schema_version: 3
id: atelier-icv4
title: Review briefs that carry what a reviewer needs
status: ready
labels:
- review
lead: t3-lead
ready:
  by: sumeet
  at: 2026-11-09T19:56:00-05:00
created_at: 2026-11-09T18:00:00Z
---

# Why

Reviewers fetch findings one by one.

## Outcome

The brief carries them.

## Result
";

const REVIEW_1: &str = "---
schema: atelier.review
schema_version: 3
round: 1
commit: 0123456789abcdef0123456789abcdef01234567
reviewers:
- t3-reviewer-1
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-10T09:00:00-05:00
---

## Summary

Two problems.

## Finding 1: The brief omits Scope
Recommendation: fix now

## Finding 2: Old wording
Recommendation: defer

Taken: atelier-n3w1 by t3-gardener
";

const REVIEW_2: &str = "---
schema: atelier.review
schema_version: 3
round: 2
commit: 89abcdef0123456789abcdef0123456789abcdef
reviewers:
- t3-reviewer-2
- t3-reviewer-3
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-10T11:00:00-05:00
---
## Summary

## Finding 1: The brief omits Scope
Recommendation: fix now
Was: 1-k3fa.1
Now: kept

## Finding 2: A rebuttal
Recommendation: fix now

## Finding 3: Not worth it
Recommendation: defer

Dropped by t3-gardener: the code it names is gone
";

const CONFIG: &str = r#"schema_version = 3
slug = "atelier"
base_branch = "master"

[[merge_check]]
run = "cargo fmt -- --check"

[[merge_check]]
run = "cargo nextest run"

[forge]
provider = "github"
remote = "origin"
ci_workflow = "ci.yml"
"#;

/// A `.atelier/` directory holding `files`, each a path and its text.
fn atelier(files: &[(&str, &str)]) -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    for (path, text) in files {
        let path = dir.path().join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    dir
}

/// The example tree: a mission and two items, the first in the mission,
/// with two reviews.
fn example() -> Vec<(&'static str, String)> {
    vec![
        ("missions/atelier-icv4/mission.md", MISSION.to_string()),
        ("items/atelier-06jc/item.md", ITEM.to_string()),
        ("items/atelier-06jc/reviews/1-k3fa.md", REVIEW_1.to_string()),
        ("items/atelier-06jc/reviews/2-m2bn.md", REVIEW_2.to_string()),
        ("items/atelier-b10c/item.md", BLOCKER.to_string()),
    ]
}

/// The problems `check` would report for the example tree after `edit`
/// replaces one file's text: `(path, old, new)`.
fn problems_after(edits: &[(&str, &str, &str)]) -> Vec<Problem> {
    let mut files = example();
    for (path, old, new) in edits {
        let (_, text) = files.iter_mut().find(|(p, _)| p == path).unwrap();
        assert!(text.contains(old), "{path} has no `{old}`");
        *text = text.replacen(old, new, 1);
    }
    let files: Vec<(&str, &str)> = files.iter().map(|(p, t)| (*p, t.as_str())).collect();
    let dir = atelier(&files);
    let store = Store::new(dir.path());
    validate(&store.items().unwrap(), &store.missions().unwrap())
}

/// One problem, in `file`, naming `field`, whose message has `words`.
fn assert_one_problem(problems: &[Problem], file: &str, field: &str, words: &str) {
    assert_eq!(problems.len(), 1, "{problems:#?}");
    let shown = problems[0].to_string();
    assert!(problems[0].file.ends_with(file), "{shown}");
    assert_eq!(problems[0].field, field, "{shown}");
    assert!(
        shown.contains(file) && shown.contains(field) && shown.contains(words),
        "{shown}"
    );
}

fn load(files: &[(&str, &str)], id: &str) -> ItemDir {
    Store::new(atelier(files).path()).item(id).unwrap()
}

#[test]
fn each_record_kind_reads_and_writes_back_the_same_file() {
    let path = Path::new("item.md");
    assert_eq!(render_item(&parse_item(path, ITEM).unwrap()), ITEM);
    assert_eq!(render_item(&parse_item(path, BLOCKER).unwrap()), BLOCKER);
    assert_eq!(
        render_mission(&parse_mission(path, MISSION).unwrap()),
        MISSION
    );
    for review in [REVIEW_1, REVIEW_2] {
        assert_eq!(render_review(&parse_review(path, review).unwrap()), review);
    }
    assert_eq!(render_config(&parse_config(path, CONFIG).unwrap()), CONFIG);
}

#[test]
fn a_review_with_no_model_reads_and_writes_back_as_null() {
    let unknown = REVIEW_1.replace("model: gpt-6.1-sol", "model: null");
    let review = parse_review(Path::new("x"), &unknown).unwrap();
    assert_eq!(review.model, None);
    assert_eq!(render_review(&review), unknown);
    assert_eq!(
        parse_review(Path::new("x"), REVIEW_1)
            .unwrap()
            .model
            .as_deref(),
        Some("gpt-6.1-sol")
    );
}

#[test]
fn an_item_gives_its_sections_and_a_review_each_finding_as_written() {
    let dir = load(
        &example()
            .iter()
            .map(|(p, t)| (*p, t.as_str()))
            .collect::<Vec<_>>(),
        "atelier-06jc",
    );
    assert_eq!(
        dir.section("Goal"),
        Some("\nPrint open findings in full.\n\n")
    );
    assert_eq!(dir.section("Scope"), None);
    assert!(dir.section("Report").unwrap().contains("### Responses"));
    let second = &dir.reviews[1];
    assert_eq!(
        second.finding_text(1),
        Some("Recommendation: fix now\nWas: 1-k3fa.1\nNow: kept\n\n")
    );
    assert_eq!(
        second.finding_text(3),
        Some("Recommendation: defer\n\nDropped by t3-gardener: the code it names is gone\n")
    );
    assert_eq!(second.finding_text(4), None);
}

#[test]
fn a_body_keeps_its_leading_and_trailing_newlines() {
    let path = Path::new("item.md");
    let tight = BLOCKER.replace("---\n## Goal", "---\n\n## Goal") + "\n";
    let loose = parse_item(path, &tight).unwrap();
    assert!(loose.body.starts_with("\n## Goal") && loose.body.ends_with("d\n\n"));
    assert_ne!(loose.body, parse_item(path, BLOCKER).unwrap().body);
    assert_eq!(render_item(&loose), tight);
}

#[test]
fn the_store_writes_records_where_it_reads_them() {
    let dir = atelier(&[]);
    let store = Store::new(dir.path());
    let mut item = parse_item(Path::new("x"), ITEM).unwrap();
    item.status = Status::InReview;
    store.write_item(&item).unwrap();
    assert_eq!(store.item("atelier-06jc").unwrap().item, item);

    let review = parse_review(Path::new("x"), REVIEW_2).unwrap();
    let name = create_review(&dir.path().join("items/atelier-06jc"), &review).unwrap();
    let mut file = store.item("atelier-06jc").unwrap().reviews.remove(0);
    file.review.verdict = Some(Verdict::Approve);
    file.write().unwrap();
    let read = store.item("atelier-06jc").unwrap();
    assert_eq!(read.reviews.len(), 1);
    assert_eq!(read.reviews[0].name, name);
    assert_eq!(read.reviews[0].review, file.review);
}

#[test]
fn creating_reviews_of_a_round_gives_each_its_own_name() {
    let dir = atelier(&[]);
    let review = parse_review(Path::new("x"), REVIEW_2).unwrap();
    let item = dir.path().join("items/atelier-06jc");
    let first = create_review(&item, &review).unwrap();
    let second = create_review(&item, &review).unwrap();
    assert_ne!(first, second);
    for name in [&first, &second] {
        assert_eq!(name.round, 2);
        assert_eq!(name.suffix.len(), 4);
        assert!(name
            .suffix
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
        let path = dir
            .path()
            .join(format!("items/atelier-06jc/reviews/2-{}.md", name.suffix));
        assert_eq!(fs::read_to_string(path).unwrap(), REVIEW_2);
    }
}

#[test]
fn the_example_tree_is_valid() {
    assert_eq!(problems_after(&[]), []);
}

#[test]
fn an_unknown_status_is_refused_naming_file_and_field() {
    let dir = atelier(&[(
        "items/atelier-b10c/item.md",
        &BLOCKER.replace("status: draft", "status: blocked"),
    )]);
    let error = Store::new(dir.path()).items().unwrap_err();
    let shown = error.to_string();
    assert!(
        error.file.ends_with("items/atelier-b10c/item.md"),
        "{shown}"
    );
    assert!(
        shown.contains("status: unknown variant `blocked`"),
        "{shown}"
    );
}

#[test]
fn a_mission_reads_its_fields_and_keeps_its_body() {
    let dir = atelier(&[("missions/atelier-icv4/mission.md", MISSION)]);
    let read = Store::new(dir.path()).mission("atelier-icv4").unwrap();
    assert!(read.path.ends_with("missions/atelier-icv4/mission.md"));
    let mission = read.mission;
    assert_eq!(mission.status, MissionStatus::Ready);
    assert_eq!(mission.labels, ["review"]);
    assert_eq!(mission.lead, Some(Thread("t3-lead".into())));
    assert_eq!(mission.ready.unwrap().by, "sumeet");
    assert_eq!(
        mission.body,
        MISSION
            .split_once("created_at: 2026-11-09T18:00:00Z\n---\n")
            .unwrap()
            .1
    );
}

#[test]
fn a_mission_with_an_unknown_status_is_refused_naming_file_and_field() {
    // `building` is an item's status, and no mission's.
    for status in ["blocked", "building"] {
        let text = MISSION.replace("status: ready", &format!("status: {status}"));
        let dir = atelier(&[("missions/atelier-icv4/mission.md", &text)]);
        let error = Store::new(dir.path()).missions().unwrap_err();
        let shown = error.to_string();
        assert!(
            error.file.ends_with("missions/atelier-icv4/mission.md"),
            "{shown}"
        );
        assert!(
            shown.contains(&format!("status: unknown variant `{status}`")),
            "{shown}"
        );
    }
}

#[test]
fn a_mission_needs_an_outcome_section() {
    let file = "missions/atelier-icv4/mission.md";
    let problems = problems_after(&[(file, "## Outcome\n", "## Outcomes\n")]);
    assert_one_problem(&problems, file, "Outcome", "no `## Outcome` section");
    // A heading in a fenced example is no section.
    let fenced = problems_after(&[(file, "## Outcome\n", "```\n## Outcome\n```\n")]);
    assert_one_problem(&fenced, file, "Outcome", "no `## Outcome` section");
}

#[test]
fn a_missions_other_files_are_not_examined() {
    let mut files = example();
    files.push((
        "missions/atelier-icv4/design.md",
        "# Design\nNo front matter.\n".into(),
    ));
    let files: Vec<(&str, &str)> = files.iter().map(|(p, t)| (*p, t.as_str())).collect();
    let dir = atelier(&files);
    let binary = dir.path().join("missions/atelier-icv4/sketch.png");
    fs::write(&binary, [0x89, b'P', b'N', b'G', 0xff, 0x00, 0xfe]).unwrap();
    fs::create_dir_all(dir.path().join("missions/atelier-icv4/spikes")).unwrap();
    let store = Store::new(dir.path());
    let missions = store.missions().unwrap();
    assert_eq!(missions.len(), 1);
    assert_eq!(validate(&store.items().unwrap(), &missions), []);
}

#[test]
fn a_mission_id_must_be_an_id_and_its_directorys_name() {
    let file = "missions/atelier-icv4/mission.md";
    let problems = problems_after(&[(file, "id: atelier-icv4", "id: atelier-icv5")]);
    // The item in it now names no mission, too.
    let (mission, item): (Vec<Problem>, Vec<Problem>) =
        problems.into_iter().partition(|p| p.file.ends_with(file));
    assert_one_problem(&mission, file, "id", "not its directory's name");
    assert_one_problem(
        &item,
        "items/atelier-06jc/item.md",
        "mission",
        "names no mission",
    );
}

#[test]
fn an_items_mission_must_name_a_mission() {
    let file = "items/atelier-06jc/item.md";
    let problems = problems_after(&[(file, "mission: atelier-icv4", "mission: atelier-g0ne")]);
    assert_one_problem(
        &problems,
        file,
        "mission",
        "`atelier-g0ne` names no mission",
    );
    // An item is not a mission.
    let problems = problems_after(&[(file, "mission: atelier-icv4", "mission: atelier-b10c")]);
    assert_one_problem(
        &problems,
        file,
        "mission",
        "`atelier-b10c` names no mission",
    );
}

#[test]
fn the_store_writes_a_mission_where_it_reads_it() {
    let dir = atelier(&[]);
    let store = Store::new(dir.path());
    let mut mission = parse_mission(Path::new("x"), MISSION).unwrap();
    mission.status = MissionStatus::Done;
    store.write_mission(&mission).unwrap();
    assert_eq!(store.mission("atelier-icv4").unwrap().mission, mission);
    assert_eq!(store.missions().unwrap().len(), 1);
}

#[test]
fn an_unknown_front_matter_key_is_refused_naming_it() {
    let text = BLOCKER.replace("trivial: null\n", "trivial: null\nowner: me\n");
    let shown = parse_item(Path::new("item.md"), &text)
        .unwrap_err()
        .to_string();
    assert!(
        shown.contains("item.md") && shown.contains("unknown field `owner`"),
        "{shown}"
    );
}

#[test]
fn an_item_needs_goal_and_done_when() {
    let file = "items/atelier-b10c/item.md";
    let problems = problems_after(&[(file, "## Goal\n", "## Aim\n")]);
    assert_one_problem(&problems, file, "Goal", "no `## Goal` section");
    let problems = problems_after(&[(file, "## Done when\n", "## Done\n")]);
    assert_one_problem(&problems, file, "Done when", "no `## Done when` section");
}

#[test]
fn a_finding_needs_a_recommendation_as_its_first_line() {
    let file = "items/atelier-06jc/reviews/1-k3fa.md";
    for (old, new) in [
        ("Recommendation: defer\n", ""),
        (
            "Recommendation: defer\n",
            "Some text.\nRecommendation: defer\n",
        ),
        ("Recommendation: defer\n", "Recommendation: later\n"),
    ] {
        let problems = problems_after(&[(file, old, new)]);
        assert_one_problem(
            &problems,
            file,
            "## Finding 2: Old wording",
            "Recommendation",
        );
    }
}

#[test]
fn an_approve_verdict_is_refused_while_its_file_holds_an_open_fix_now_finding() {
    let file = "items/atelier-06jc/reviews/2-m2bn.md";
    let problems = problems_after(&[(file, "verdict: changes", "verdict: approve")]);
    assert_one_problem(
        &problems,
        file,
        "verdict",
        "fix-now finding 2-m2bn.1 is open",
    );

    // Keeping it while weakening its recommendation is refused, approve or not.
    let weakened = problems_after(&[
        (file, "verdict: changes", "verdict: approve"),
        (file, "fix now\nWas: 1-k3fa.1", "defer\nWas: 1-k3fa.1"),
    ]);
    assert_one_problem(
        &weakened,
        file,
        "## Finding 1: The brief omits Scope",
        "`Now: kept`",
    );

    // Approving round 1 is wrong too, until round 2 carries its finding.
    let file = "items/atelier-06jc/reviews/1-k3fa.md";
    let problems = problems_after(&[
        (file, "verdict: changes", "verdict: approve"),
        (
            "items/atelier-06jc/reviews/2-m2bn.md",
            "Was: 1-k3fa.1\nNow: kept\n",
            "",
        ),
    ]);
    assert_one_problem(
        &problems,
        file,
        "verdict",
        "fix-now finding 1-k3fa.1 is open",
    );
}

#[test]
fn a_missions_plan_reviews_are_read_with_it_and_checked_as_an_items_are() {
    const PLAN: &str = "missions/atelier-icv4/reviews/1-p1an.md";
    const OTHER: &str = "missions/atelier-icv4/reviews/1-b1nd.md";
    let review = REVIEW_1.replace("Taken: atelier-n3w1 by t3-gardener\n", "");
    let blind = review.replace("verdict: changes", "verdict: approve");
    let blind = blind.replace("Recommendation: fix now", "Recommendation: defer");
    let mut files = example();
    files.push((PLAN, review.clone()));
    files.push((OTHER, blind));
    let files: Vec<(&str, &str)> = files.iter().map(|(p, t)| (*p, t.as_str())).collect();
    let dir = atelier(&files);
    let store = Store::new(dir.path());
    let mission = store.mission("atelier-icv4").unwrap();
    let names: Vec<String> = mission.reviews.iter().map(|r| r.name.to_string()).collect();
    assert_eq!(names, ["1-b1nd", "1-p1an"]);
    assert_eq!(mission.findings().unwrap().len(), 4);
    // Two blind reviews of one round are both valid, one approving while
    // the other's fix-now finding is open.
    assert_eq!(
        validate(&store.items().unwrap(), std::slice::from_ref(&mission)),
        []
    );

    // Its own open finding refuses an approve; a round must match the
    // file's name; a `Was:` must name an earlier round's finding.
    let cases = [
        (
            "verdict: changes",
            "verdict: approve",
            "verdict",
            "1-p1an.1 is open",
        ),
        ("round: 1", "round: 2", "round", "not the file name's round"),
        (
            "Recommendation: fix now\n",
            "Recommendation: fix now\nWas: 1-b1nd.1\nNow: kept\n",
            "## Finding 1: The brief omits Scope",
            "names no finding of an earlier round",
        ),
    ];
    for (old, new, field, words) in cases {
        let path = dir.path().join(PLAN);
        fs::write(&path, review.replacen(old, new, 1)).unwrap();
        let mission = store.mission("atelier-icv4").unwrap();
        assert_one_problem(&validate(&[], &[mission]), PLAN, field, words);
    }
}

#[test]
fn a_dropped_record_gives_the_reason_on_the_first_line_of_its_report_or_result() {
    let dropped = atelier_records::put_first(
        &ITEM.replace("status: changes_requested", "status: dropped"),
        "Report",
        "Dropped: superseded",
    );
    let dir = atelier(&[
        ("items/atelier-06jc/item.md", &dropped),
        (
            "missions/atelier-icv4/mission.md",
            &atelier_records::put_first(MISSION, "Result", "Dropped: no time"),
        ),
    ]);
    let store = Store::new(dir.path());
    let item = store.item("atelier-06jc").unwrap();
    assert_eq!(item.dropped(), Some("superseded"));
    // The Report keeps its text and its Responses, which still read.
    assert!(item
        .section("Report")
        .unwrap()
        .contains("It prints them.\n\n### Responses"));
    assert_eq!(item.responses().unwrap().len(), 2);
    assert_eq!(
        store.mission("atelier-icv4").unwrap().dropped(),
        Some("no time")
    );
    assert_eq!(
        load(&[("items/atelier-06jc/item.md", ITEM)], "atelier-06jc").dropped(),
        None
    );
}

#[test]
fn a_blocked_by_must_name_an_item() {
    let file = "items/atelier-06jc/item.md";
    let problems = problems_after(&[(file, "- atelier-b10c\n", "- atelier-g0ne\n")]);
    assert_one_problem(
        &problems,
        file,
        "blocked_by",
        "`atelier-g0ne` names no item",
    );
}

#[test]
fn implementers_and_reviewers_are_lists_whose_last_entry_holds_the_role() {
    let dir = load(
        &example()
            .iter()
            .map(|(p, t)| (*p, t.as_str()))
            .collect::<Vec<_>>(),
        "atelier-06jc",
    );
    let mut item = dir.item;
    assert_eq!(
        item.implementers.holder(),
        Some(&Thread("t3-implementer-2".into()))
    );
    item.implementers.take(Thread("t3-implementer-3".into()));
    let reread = parse_item(Path::new("x"), &render_item(&item)).unwrap();
    assert_eq!(reread.implementers.0.len(), 3);
    assert_eq!(
        reread.implementers.holder(),
        Some(&Thread("t3-implementer-3".into()))
    );

    let mut review = dir.reviews[1].review.clone();
    assert_eq!(
        review.reviewers.holder(),
        Some(&Thread("t3-reviewer-3".into()))
    );
    review.reviewers.take(Thread("t3-reviewer-4".into()));
    let reread = parse_review(Path::new("x"), &render_review(&review)).unwrap();
    assert_eq!(
        reread.reviewers.holder(),
        Some(&Thread("t3-reviewer-4".into()))
    );
}

#[test]
fn findings_are_named_by_review_stem_and_number() {
    let files = example();
    let dir = load(
        &files
            .iter()
            .map(|(p, t)| (*p, t.as_str()))
            .collect::<Vec<_>>(),
        "atelier-06jc",
    );
    let findings = dir.findings().unwrap();
    let names: Vec<String> = findings.iter().map(|f| f.name.to_string()).collect();
    assert_eq!(
        names,
        ["1-k3fa.1", "1-k3fa.2", "2-m2bn.1", "2-m2bn.2", "2-m2bn.3"]
    );
    assert_eq!(findings[0].title, "The brief omits Scope");
    assert_eq!(findings[1].recommendation, Recommendation::Defer);
    assert_eq!(findings[2].was.as_ref().unwrap().to_string(), "1-k3fa.1");
    assert_eq!(findings[2].now, Some(Now::Kept));

    let responses = dir.responses().unwrap();
    assert_eq!(responses.len(), 2);
    assert!(matches!(&responses[0], Response::Fixed { finding, commit }
        if finding.to_string() == "2-m2bn.1" && commit == "0123abc"));
    assert!(matches!(&responses[1], Response::Rebutted { finding, fact }
        if finding.to_string() == "2-m2bn.2" && fact == "the Scope is printed by `show`"));

    let decisions = dir.decisions().unwrap();
    assert_eq!(decisions.len(), 1);
    assert_eq!(
        decisions[0].question,
        "2026-11-10: Should the brief carry the parent's Scope?"
    );
    assert_eq!(
        decisions[0].finding.as_ref().unwrap().to_string(),
        "2-m2bn.2"
    );
    assert_eq!(
        decisions[0].answer.as_ref().unwrap().by,
        Thread("t3-decider".into())
    );
    assert_eq!(decisions[0].outcome, Some(Outcome::NoAction));
}

#[test]
fn a_finding_name_no_review_has_is_refused() {
    let item = "items/atelier-06jc/item.md";
    let review = "items/atelier-06jc/reviews/2-m2bn.md";
    let problems = problems_after(&[(review, "Was: 1-k3fa.1", "Was: 1-zzzz.1")]);
    assert_one_problem(
        &problems,
        review,
        "## Finding 1: The brief omits Scope",
        "`Was: 1-zzzz.1` names no finding of an earlier round",
    );
    let problems = problems_after(&[(review, "Was: 1-k3fa.1", "Was: 1-k3fa.9")]);
    assert_one_problem(
        &problems,
        review,
        "## Finding 1: The brief omits Scope",
        "`Was: 1-k3fa.9`",
    );
    let problems = problems_after(&[(item, "2-m2bn.1: fixed", "3-m2bn.1: fixed")]);
    assert_one_problem(&problems, item, "Responses", "`3-m2bn.1` names no finding");
    let problems = problems_after(&[(item, "Finding: 2-m2bn.2", "Finding: 2-m2bn.7")]);
    assert_one_problem(
        &problems,
        item,
        "Decisions: 2026-11-10: Should the brief carry the parent's Scope?",
        "`Finding: 2-m2bn.7` names no finding",
    );
}

#[test]
fn a_finding_name_in_another_form_is_refused() {
    let item = "items/atelier-06jc/item.md";
    let review = "items/atelier-06jc/reviews/2-m2bn.md";
    let cases = [
        (review, "Was: 1-k3fa.1", "Was: k3fa-1"),
        (item, "2-m2bn.1: fixed", "m2bn.1: fixed"),
        (item, "Finding: 2-m2bn.2", "Finding: 2-m2bn"),
    ];
    for (file, old, new) in cases {
        let files = example();
        let mut files: Vec<(&str, String)> = files;
        let text = &mut files.iter_mut().find(|(p, _)| *p == file).unwrap().1;
        *text = text.replacen(old, new, 1);
        let dir = load(
            &files
                .iter()
                .map(|(p, t)| (*p, t.as_str()))
                .collect::<Vec<_>>(),
            "atelier-06jc",
        );
        let problems = validate(&[dir], &[]);
        assert!(
            problems
                .iter()
                .any(|p| p.file.ends_with(file) && p.message.contains("is not a finding name")),
            "{new}: {problems:#?}"
        );
    }
}

#[test]
fn records_whose_names_or_lines_are_ambiguous_are_refused() {
    let item = "items/atelier-06jc/item.md";
    let review = "items/atelier-06jc/reviews/2-m2bn.md";
    let finding_1 = "## Finding 1: The brief omits Scope";
    let decision = "Decisions: 2026-11-10: Should the brief carry the parent's Scope?";
    let answer = "Answer by t3-decider: No; the item's own Scope is enough.\n";
    let cases = [
        (
            "items/atelier-b10c/item.md",
            "id: atelier-b10c",
            "id: atelier-b10d",
            "id",
            "not its directory's name",
        ),
        (
            review,
            "round: 2",
            "round: 3",
            "round",
            "round 3 is not the file name's round",
        ),
        (
            review,
            "Finding 3: Not",
            "Finding 2: Not",
            "## Finding 2: Not worth it",
            "appears twice",
        ),
        (
            review,
            "Now: kept\n",
            "",
            finding_1,
            "a carried finding has a `Was:` line",
        ),
        (
            review,
            "Was: 1-k3fa.1\n",
            "",
            finding_1,
            "a carried finding has a `Was:` line",
        ),
        (
            review,
            "Now: kept\n",
            "Now: kept\nDecided: defer\n",
            finding_1,
            "a carried finding",
        ),
        (
            review,
            "fix now\nWas: 1-k3fa.1",
            "defer\nWas: 1-k3fa.1",
            finding_1,
            "carried as `Now: kept`",
        ),
        (
            review,
            "fix now\nWas: 1-k3fa.1",
            "no action\nWas: 1-k3fa.1",
            finding_1,
            "carried as `Now: kept`",
        ),
        (
            item,
            "Finding: 2-m2bn.2\n",
            "",
            decision,
            "an `Outcome:` needs",
        ),
        (item, answer, "", decision, "an `Outcome:` needs"),
    ];
    for (file, old, new, field, words) in cases {
        let problems = problems_after(&[(file, old, new)]);
        assert!(
            problems
                .iter()
                .any(|p| p.file.ends_with(file) && p.field == field && p.message.contains(words)),
            "{new}: {problems:#?}"
        );
    }
}

#[test]
fn caused_by_holds_an_item_id_or_a_commit_hash_checked_for_form_only() {
    let file = "items/atelier-06jc/item.md";
    for cause in [
        "atelier-g0ne",
        "0123abc",
        "0123456789abcdef0123456789abcdef01234567",
    ] {
        let problems = problems_after(&[(
            file,
            "caused_by: atelier-x1y2",
            &format!("caused_by: {cause}"),
        )]);
        assert_eq!(problems, [], "{cause}");
    }
    for cause in ["the login change", "0123ab", "ATELIER-X1Y2"] {
        let problems = problems_after(&[(
            file,
            "caused_by: atelier-x1y2",
            &format!("caused_by: '{cause}'"),
        )]);
        assert_one_problem(
            &problems,
            file,
            "caused_by",
            "neither an item ID nor a commit hash",
        );
    }
}

#[test]
fn a_deferred_finding_is_settled_by_a_taken_or_dropped_line() {
    let files = example();
    let dir = load(
        &files
            .iter()
            .map(|(p, t)| (*p, t.as_str()))
            .collect::<Vec<_>>(),
        "atelier-06jc",
    );
    let findings = dir.findings().unwrap();
    let taken = &findings[1];
    let decisions = dir.decisions().unwrap();
    assert!(is_deferred(taken, &findings, &decisions) && taken.is_settled());
    assert_eq!(
        taken.settlement,
        Some(Settlement::Taken {
            item: "atelier-n3w1".into(),
            by: Thread("t3-gardener".into())
        })
    );
    let dropped = &findings[4];
    assert!(is_deferred(dropped, &findings, &decisions) && dropped.is_settled());
    assert_eq!(
        dropped.settlement,
        Some(Settlement::Dropped {
            by: Thread("t3-gardener".into()),
            reason: "the code it names is gone".into()
        })
    );
    // Both write back unchanged.
    assert_eq!(render_review(&dir.reviews[0].review), REVIEW_1);
    assert_eq!(render_review(&dir.reviews[1].review), REVIEW_2);

    let unsettled = REVIEW_1.replace("\nTaken: atelier-n3w1 by t3-gardener\n", "");
    let files = [
        (
            "items/atelier-06jc/item.md",
            BLOCKER.replace("atelier-b10c", "atelier-06jc"),
        ),
        ("items/atelier-06jc/reviews/1-k3fa.md", unsettled),
    ];
    let dir = load(
        &files
            .iter()
            .map(|(p, t)| (*p, t.as_str()))
            .collect::<Vec<_>>(),
        "atelier-06jc",
    );
    let findings = dir.findings().unwrap();
    assert!(is_deferred(&findings[1], &findings, &[]) && !findings[1].is_settled());
}

#[test]
fn settling_lines_in_another_form_are_refused() {
    let file = "items/atelier-06jc/reviews/1-k3fa.md";
    for line in [
        "Taken: atelier-n3w1",
        "Taken: not-an-id! by t3",
        "Dropped by t3-gardener",
    ] {
        let problems = problems_after(&[(file, "Taken: atelier-n3w1 by t3-gardener", line)]);
        assert_one_problem(&problems, file, "## Finding 2: Old wording", "");
    }
    let both = "Taken: atelier-n3w1 by t3-gardener\nDropped by t3: no";
    let problems = problems_after(&[(file, "Taken: atelier-n3w1 by t3-gardener", both)]);
    assert_one_problem(
        &problems,
        file,
        "## Finding 2: Old wording",
        "two Taken or Dropped lines",
    );
}

#[test]
fn field_lines_in_a_fenced_example_are_text() {
    let review = "items/atelier-06jc/reviews/1-k3fa.md";
    let item = "items/atelier-06jc/item.md";
    let fenced_taken = "````text\n```\nTaken: atelier-bbbb by example-thread\n```\n````";
    let fenced_entry = "~~~~\n~~~\nFinding: 2-m2bn.1\nAnswer by example-thread: Example only\nOutcome: no action\n~~~\n~~~~";
    let mut files = example();
    for (path, old, new) in [
        (review, "Taken: atelier-n3w1 by t3-gardener", fenced_taken),
        (item, "Finding: 2-m2bn.2\n\nAnswer by t3-decider: No; the item's own Scope is enough.\nOutcome: no action", fenced_entry),
    ] {
        let text = &mut files.iter_mut().find(|(p, _)| *p == path).unwrap().1;
        assert!(text.contains(old), "{old}");
        *text = text.replacen(old, new, 1);
    }
    let rendered = |path: &str| files.iter().find(|(p, _)| *p == path).unwrap().1.clone();
    let dir = load(
        &files
            .iter()
            .map(|(p, t)| (*p, t.as_str()))
            .collect::<Vec<_>>(),
        "atelier-06jc",
    );
    assert!(!dir.findings().unwrap()[1].is_settled());
    let decision = &dir.decisions().unwrap()[0];
    assert_eq!(
        (&decision.finding, &decision.answer, decision.outcome),
        (&None, &None, None)
    );
    let open: Vec<String> = dir
        .open_findings()
        .unwrap()
        .iter()
        .map(|f| f.name.to_string())
        .collect();
    assert_eq!(open, ["2-m2bn.1", "2-m2bn.2"]);
    assert_eq!(render_item(&dir.item), rendered(item));
    assert_eq!(render_review(&dir.reviews[0].review), rendered(review));
}

#[test]
fn inline_code_on_a_line_of_its_own_opens_no_fence() {
    let review = "items/atelier-06jc/reviews/2-m2bn.md";
    let inline = (review, "## Summary\n", "## Summary\n\n```inline code```\n");
    let problems = problems_after(&[inline, (review, "verdict: changes", "verdict: approve")]);
    assert_one_problem(
        &problems,
        review,
        "verdict",
        "fix-now finding 2-m2bn.1 is open",
    );
    let problems = problems_after(&[inline, (review, "Recommendation: defer\n", "")]);
    assert_one_problem(
        &problems,
        review,
        "## Finding 3: Not worth it",
        "Recommendation",
    );
}

#[test]
fn a_fix_now_finding_stays_open_until_a_later_round_or_a_decision_closes_it() {
    let open = |edits: &[(&str, &str, &str)]| -> Vec<String> {
        let mut files = example();
        for (path, old, new) in edits {
            let text = &mut files.iter_mut().find(|(p, _)| p == path).unwrap().1;
            *text = text.replacen(old, new, 1);
        }
        let dir = load(
            &files
                .iter()
                .map(|(p, t)| (*p, t.as_str()))
                .collect::<Vec<_>>(),
            "atelier-06jc",
        );
        dir.open_findings()
            .unwrap()
            .iter()
            .map(|f| f.name.to_string())
            .collect()
    };
    let review = "items/atelier-06jc/reviews/2-m2bn.md";
    let item = "items/atelier-06jc/item.md";
    // 1-k3fa.1 is carried as kept by 2-m2bn.1; 2-m2bn.2 is decided.
    assert_eq!(open(&[]), ["2-m2bn.1"]);
    assert_eq!(
        open(&[(review, "Now: kept", "Now: fixed")]),
        Vec::<String>::new()
    );
    assert_eq!(
        open(&[(review, "Now: kept", "Now: withdrawn")]),
        Vec::<String>::new()
    );
    assert_eq!(
        open(&[(review, "Was: 1-k3fa.1\nNow: kept\n", "")]),
        ["1-k3fa.1", "2-m2bn.1"]
    );
    assert_eq!(
        open(&[(item, "Outcome: no action\n", "")]),
        ["2-m2bn.1", "2-m2bn.2"]
    );
}

#[test]
fn config_reads_its_fields() {
    let config = parse_config(Path::new("config.toml"), CONFIG).unwrap();
    assert_eq!(config.slug, "atelier");
    assert_eq!(config.base_branch, "master");
    let checks: Vec<&str> = config.merge_checks.iter().map(|c| c.run.as_str()).collect();
    assert_eq!(checks, ["cargo fmt -- --check", "cargo nextest run"]);
    let forge = config.forge.unwrap();
    assert_eq!(forge.provider, ForgeProvider::Github);
    assert_eq!(forge.remote, "origin");
    assert_eq!(forge.ci_workflow.as_deref(), Some("ci.yml"));

    let no_ci = CONFIG.replace("ci_workflow = \"ci.yml\"\n", "");
    let config = parse_config(Path::new("config.toml"), &no_ci).unwrap();
    assert_eq!(config.forge.as_ref().unwrap().ci_workflow, None);
    assert_eq!(render_config(&config), no_ci);

    let minimal = "schema_version = 3\nslug = \"x\"\nbase_branch = \"main\"\n";
    let config = parse_config(Path::new("config.toml"), minimal).unwrap();
    assert!(config.merge_checks.is_empty() && config.forge.is_none());
    assert_eq!(render_config(&config), minimal);
}

#[test]
fn a_config_error_names_the_line_of_the_key() {
    let text = CONFIG.replacen(
        "slug = \"atelier\"\n",
        "slug = \"atelier\"\nowner = \"me\"\n",
        1,
    );
    let error = parse_config(Path::new("config.toml"), &text).unwrap_err();
    assert_eq!(error.field, "line 3", "{error}");
}

#[test]
fn config_refuses_what_it_does_not_know() {
    let cases = [
        (
            "slug = \"atelier\"\n",
            "slug = \"atelier\"\nowner = \"me\"\n",
            "unknown field `owner`",
        ),
        (
            "remote = \"origin\"\n",
            "remote = \"origin\"\nbranch = \"x\"\n",
            "unknown field `branch`",
        ),
        (
            "run = \"cargo nextest run\"\n",
            "run = \"cargo nextest run\"\nshell = \"sh\"\n",
            "unknown field `shell`",
        ),
        (
            "base_branch = \"master\"\n",
            "base_branch = \"master\"\nmerge_strategy = \"merge_commit\"\n",
            "unknown field `merge_strategy`",
        ),
        ("\"github\"", "\"forgejo\"", "unknown variant `forgejo`"),
        (
            "schema_version = 3",
            "schema_version = 2",
            "schema version 2",
        ),
        (
            "base_branch = \"master\"\n",
            "",
            "missing field `base_branch`",
        ),
    ];
    for (old, new, words) in cases {
        let error =
            parse_config(Path::new("x/config.toml"), &CONFIG.replacen(old, new, 1)).unwrap_err();
        let shown = error.to_string();
        assert!(
            shown.starts_with("x/config.toml: line ") && shown.contains(words),
            "{shown}"
        );
    }
    let error = parse_config(
        Path::new("config.toml"),
        &CONFIG.replace("\"atelier\"", "\"At Elier\""),
    )
    .unwrap_err();
    assert_eq!(error.field, "slug");
}
