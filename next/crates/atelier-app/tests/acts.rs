use atelier_app::acts::{
    check, check_mission, next, row, Act, Condition, Facts, Refusal, Role, TABLE,
};
use atelier_app::{carried, entry};
use atelier_core::{MissionStatus, Status, Thread};
use atelier_records::{parse_item, parse_review, ItemDir, ReviewFile};
use std::path::PathBuf;

const ALL: [Status; 8] = [
    Status::Draft,
    Status::Ready,
    Status::Building,
    Status::InReview,
    Status::ChangesRequested,
    Status::Approved,
    Status::Done,
    Status::Dropped,
];

const DONE: &str = "## Goal\ng\n\n## Done when\n\n- [x] One.\n\n## Report\n\nDone.\n";
const UNDONE: &str = "## Goal\ng\n\n## Done when\n\n- [x] One.\n- [ ] Two.\n\n## Report\n\n";

fn status_text(status: Status) -> &'static str {
    match status {
        Status::Draft => "draft",
        Status::Ready => "ready",
        Status::Building => "building",
        Status::InReview => "in_review",
        Status::ChangesRequested => "changes_requested",
        Status::Approved => "approved",
        Status::Done => "done",
        Status::Dropped => "dropped",
    }
}

fn record(status: Status, implementers: &[&str], body: &str) -> ItemDir {
    let implementers: String = implementers.iter().map(|t| format!("\n- {t}")).collect();
    let implementers = if implementers.is_empty() {
        " []".to_string()
    } else {
        implementers
    };
    let text = format!(
        "---\nschema: atelier.item\nschema_version: 3\nid: demo-ab12\ntitle: T\nstatus: {}\n\
         mission: null\nlabels: []\npriority: high\nblocked_by: []\nimplementers:{implementers}\n\
         ready: null\ntrivial: null\ncaused_by: null\ncreated_at: 2026-11-10T09:00:00Z\n---\n{body}",
        status_text(status)
    );
    let path = PathBuf::from("demo-ab12/item.md");
    ItemDir {
        item: parse_item(&path, &text).unwrap(),
        path,
        reviews: Vec::new(),
    }
}

fn thread(id: &str) -> Thread {
    Thread(id.to_string())
}

/// A review file named `name`, held by the last of `reviewers`, with
/// `verdict` (`null` for none) and the findings in `findings`.
fn review(name: &str, reviewers: &[&str], verdict: &str, findings: &str) -> ReviewFile {
    let round = name.split('-').next().unwrap();
    let reviewers: String = reviewers.iter().map(|t| format!("\n- {t}")).collect();
    let text = format!(
        "---\nschema: atelier.review\nschema_version: 3\nround: {round}\ncommit: abc1234\n\
         reviewers:{reviewers}\nmodel: null\nverdict: {verdict}\ncreated_at: 2026-11-10T09:00:00Z\n---\n\
         ## Summary\ns\n{findings}"
    );
    let path = PathBuf::from(format!("{name}.md"));
    ReviewFile {
        name: name.parse().unwrap(),
        review: parse_review(&path, &text).unwrap(),
        path,
    }
}

const FIX_NOW: &str = "\n## Finding 1: Wrong\nRecommendation: fix now\n";

fn with_reviews(mut item: ItemDir, reviews: &[ReviewFile]) -> ItemDir {
    item.reviews = reviews.to_vec();
    item
}

fn facts<'a>(item: &'a ItemDir, caller: &'a Thread) -> Facts<'a> {
    Facts {
        item,
        branch: true,
        blockers: &[],
        mission: None,
        caller,
        checkout: None,
    }
}

/// The architecture specification's act table, for the acts this step
/// builds, written out here so the code's table is checked against it.
const SPECIFIED: [(Act, &[Status], Option<Status>); 10] = [
    (Act::Ready, &[Status::Draft], Some(Status::Ready)),
    (Act::Start, &[Status::Ready], Some(Status::Building)),
    // Created by the act, so it runs from no status at all.
    (Act::StartNew, &[], Some(Status::Building)),
    (
        Act::TakeOver,
        &[
            Status::Building,
            Status::InReview,
            Status::ChangesRequested,
            Status::Approved,
        ],
        None,
    ),
    (
        Act::Submit,
        &[Status::Building, Status::ChangesRequested, Status::Approved],
        Some(Status::InReview),
    ),
    (Act::Review, &[Status::InReview], None),
    (
        Act::VerdictChanges,
        &[Status::InReview],
        Some(Status::ChangesRequested),
    ),
    (
        Act::VerdictApprove,
        &[Status::InReview],
        Some(Status::Approved),
    ),
    (Act::Merge, &[Status::Approved], Some(Status::Done)),
    (Act::MergeTrivial, &[Status::Building], Some(Status::Done)),
];

#[test]
fn every_act_in_the_table_from_every_status_sets_the_specified_status_or_is_refused() {
    // Each item's act, and the mission's `ready` and `lead`.
    assert_eq!(TABLE.len(), SPECIFIED.len() + 2);
    for (act, from, to) in SPECIFIED {
        for status in ALL {
            // Every condition holds: the caller holds the item, except a
            // reviewer, who never implemented it and, for a verdict, holds
            // the round's review, which has an open fix-now finding for
            // `changes` and none for `approve`; a take-over has its branch
            // and a start has none.
            let findings = if act == Act::VerdictChanges {
                FIX_NOW
            } else {
                ""
            };
            // A merge needs a review that approves, and a trivial merge
            // an item nobody reviewed.
            let reviews = match act {
                Act::MergeTrivial => vec![],
                Act::Merge => vec![review("1-aaaa", &["r1"], "approve", "")],
                _ => vec![review("1-aaaa", &["r1"], "null", findings)],
            };
            let item = with_reviews(record(status, &["t1"], DONE), &reviews);
            let reviewer = matches!(act, Act::Review | Act::VerdictChanges | Act::VerdictApprove);
            // A planner holds nothing; the item's holder readying it is
            // refused by its status too, never by its role.
            let caller = thread(match act {
                _ if reviewer => "r1",
                Act::Ready if status == Status::Draft => "p1",
                _ => "t1",
            });
            let facts = Facts {
                item: &item,
                branch: act != Act::Start,
                blockers: &[],
                mission: None,
                caller: &caller,
                checkout: None,
            };
            match check(act, &facts) {
                Ok(set) => {
                    assert!(from.contains(&status), "{act:?} from {status:?}");
                    assert_eq!(set, to, "{act:?} from {status:?}");
                }
                Err(Refusal::From {
                    act: refused,
                    status: at,
                }) => {
                    assert!(!from.contains(&status), "{act:?} from {status:?}");
                    assert_eq!((refused, at), (act, status));
                }
                Err(other) => panic!("{act:?} from {status:?}: {other:?}"),
            }
        }
        assert_eq!(row(act).from, from);
    }
    // A mission's `ready`, from each of a mission's statuses.
    for status in MissionStatus::ALL {
        match check_mission(Act::ReadyMission, status) {
            Ok(set) => {
                assert_eq!(status, MissionStatus::Draft);
                assert_eq!(set, Some(MissionStatus::Ready));
            }
            Err(Refusal::From { act, status: at }) => {
                assert_ne!(status, MissionStatus::Draft);
                assert_eq!((act, at), (Act::ReadyMission, status.status()));
            }
            Err(other) => panic!("ready from {status:?}: {other:?}"),
        }
    }
    let ready = row(Act::ReadyMission);
    assert_eq!(
        (ready.role, ready.from, ready.to),
        (Role::Planner, &[Status::Draft][..], Some(Status::Ready))
    );
    // `lead`, from each of a mission's statuses: only a ready mission, and
    // its status stays.
    for status in MissionStatus::ALL {
        match check_mission(Act::Lead, status) {
            Ok(set) => assert_eq!((status, set), (MissionStatus::Ready, None)),
            Err(Refusal::From { act, status: at }) => {
                assert_ne!(status, MissionStatus::Ready);
                assert_eq!((act, at), (Act::Lead, status.status()));
            }
            Err(other) => panic!("lead from {status:?}: {other:?}"),
        }
    }
    let lead = row(Act::Lead);
    assert_eq!(
        (lead.role, lead.binds, lead.from, lead.to),
        (Role::Lead, true, &[Status::Ready][..], None)
    );
}

#[test]
fn start_needs_the_items_mission_ready_on_the_base() {
    let item = record(Status::Ready, &[], DONE);
    let caller = thread("t1");
    let mission = |status| Some(("demo-m1ss".to_string(), status));
    // No mission; a ready one; one that is not ready; one not found.
    let cases = [
        None,
        mission(Some(Status::Ready)),
        mission(Some(Status::Draft)),
        mission(Some(Status::Done)),
        mission(None),
    ];
    for named in cases {
        let facts = Facts {
            mission: named.clone(),
            branch: false,
            ..facts(&item, &caller)
        };
        let expected = match &named {
            Some((id, status)) if *status != Some(Status::Ready) => {
                vec![Condition::MissionNotReady {
                    id: id.clone(),
                    status: *status,
                }]
            }
            _ => Vec::new(),
        };
        match check(Act::Start, &facts) {
            Ok(set) => assert!(expected.is_empty() && set == Some(Status::Building)),
            Err(Refusal::Unmet(unmet)) => assert_eq!(unmet, expected, "{named:?}"),
            Err(other) => panic!("{named:?}: {other:?}"),
        }
        // `status` lists what `start` checks.
        assert_eq!(entry(&facts).unwrap().unmet, expected);
    }
}

#[test]
fn status_lists_the_conditions_the_command_reports() {
    let blocked = [("demo-b10c".to_string(), Some(Status::Building))];
    let cases = [
        (DONE, &blocked[..0], false),
        (UNDONE, &blocked[..0], false),
        (DONE, &blocked[..], true),
        (UNDONE, &blocked[..], true),
    ];
    // No review; r1 holding the round's review, clean or with an open
    // finding; and r1 holding round 2's, which does not carry round 1's
    // open finding.
    let reviews = [
        vec![],
        vec![review("1-aaaa", &["r1"], "null", "")],
        vec![review("1-aaaa", &["r1"], "null", FIX_NOW)],
        vec![
            review("1-aaaa", &["r0"], "changes", FIX_NOW),
            review("2-bbbb", &["r1"], "null", ""),
        ],
    ];
    let mut reported = 0;
    for status in ALL {
        for caller in ["t1", "t0", "r1"] {
            for ((body, blockers, branch), reviews) in cases
                .into_iter()
                .flat_map(|case| reviews.iter().map(move |r| (case, r)))
            {
                // t1 holds the item, t0 held it before, r1 never did.
                let item = with_reviews(record(status, &["t0", "t1"], body), reviews);
                let caller = thread(caller);
                let facts = Facts {
                    item: &item,
                    branch,
                    blockers,
                    mission: None,
                    caller: &caller,
                    checkout: None,
                };
                let shown = entry(&facts).unwrap();
                assert_eq!(shown.next, next(&facts).unwrap());
                let Some(act) = shown.next else {
                    assert!(shown.unmet.is_empty());
                    continue;
                };
                // The act status names next is never refused for its
                // status or its holder, only for its conditions.
                let checked = match check(act, &facts) {
                    Ok(_) => Vec::new(),
                    Err(Refusal::Unmet(conditions)) => conditions,
                    Err(other) => panic!("{act:?} from {status:?}: {other:?}"),
                };
                assert_eq!(shown.unmet, checked, "{act:?} from {status:?}");
                reported += checked.len();
            }
        }
    }
    assert!(reported > 0);
}

#[test]
fn the_next_act_follows_the_status_and_who_is_asking() {
    let cases = [
        (Status::Draft, "t1", None),
        (Status::Ready, "r1", Some(Act::Start)),
        (Status::Building, "t1", Some(Act::Submit)),
        (Status::Building, "t0", Some(Act::TakeOver)),
        (Status::ChangesRequested, "t1", Some(Act::Submit)),
        (Status::InReview, "t1", None),
        (Status::InReview, "t0", Some(Act::TakeOver)),
        (Status::InReview, "r1", Some(Act::Review)),
        (Status::Approved, "t1", Some(Act::Merge)),
        (Status::Approved, "r1", Some(Act::TakeOver)),
        (Status::Done, "t1", None),
        (Status::Dropped, "t1", None),
    ];
    for (status, caller, expected) in cases {
        let item = record(status, &["t0", "t1"], DONE);
        let caller = thread(caller);
        let facts = Facts {
            item: &item,
            branch: true,
            blockers: &[],
            mission: None,
            caller: &caller,
            checkout: None,
        };
        assert_eq!(next(&facts).unwrap(), expected, "{status:?} for {caller}");
    }
    // The round's reviewer: changes while its file holds an open fix-now
    // finding, else approve.
    let r1 = thread("r1");
    for (findings, expected) in [("", Act::VerdictApprove), (FIX_NOW, Act::VerdictChanges)] {
        let item = with_reviews(
            record(Status::InReview, &["t1"], DONE),
            &[review("1-aaaa", &["r1"], "null", findings)],
        );
        assert_eq!(next(&facts(&item, &r1)).unwrap(), Some(expected));
    }
}

#[test]
fn only_the_holder_submits() {
    let item = record(Status::Building, &["t0", "t1"], DONE);
    for (caller, holder) in [("t0", Some("t1")), ("r1", Some("t1"))] {
        let caller = thread(caller);
        let facts = Facts {
            item: &item,
            branch: true,
            blockers: &[],
            mission: None,
            caller: &caller,
            checkout: None,
        };
        match check(Act::Submit, &facts) {
            Err(Refusal::NotHolder {
                role: Role::Implementer,
                holder: found,
            }) => {
                assert_eq!(found, holder.map(thread));
            }
            other => panic!("{other:?}"),
        }
    }
    let unheld = record(Status::Building, &[], DONE);
    let caller = thread("t1");
    let facts = Facts {
        item: &unheld,
        branch: true,
        blockers: &[],
        mission: None,
        caller: &caller,
        checkout: None,
    };
    assert!(matches!(
        check(Act::Submit, &facts),
        Err(Refusal::NotHolder { holder: None, .. })
    ));
}

#[test]
fn submit_needs_each_open_finding_answered_and_each_question_decided() {
    let review = "---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: abc1234\n\
                  reviewers:\n- r1\nmodel: m\nverdict: changes\ncreated_at: 2026-11-10T09:00:00Z\n---\n\
                  ## Summary\ns\n\n## Finding 1: Wrong\nRecommendation: fix now\n\n\
                  ## Finding 2: Also wrong\nRecommendation: fix now\n\n\
                  ## Finding 3: Later\nRecommendation: defer\n";
    let path = PathBuf::from("1-aaaa.md");
    let reviews = vec![ReviewFile {
        name: "1-aaaa".parse().unwrap(),
        review: parse_review(&path, review).unwrap(),
        path,
    }];
    let conditions = |body: &str| {
        let mut item = record(Status::ChangesRequested, &["t1"], body);
        item.reviews = reviews.clone();
        let caller = thread("t1");
        let facts = Facts {
            item: &item,
            branch: true,
            blockers: &[],
            mission: None,
            caller: &caller,
            checkout: None,
        };
        match check(Act::Submit, &facts) {
            Ok(_) => Vec::new(),
            Err(Refusal::Unmet(conditions)) => conditions,
            Err(other) => panic!("{other:?}"),
        }
    };
    let open = |number: u32, title: &str| Condition::NoResponse {
        finding: format!("1-aaaa.{number}").parse().unwrap(),
        title: title.into(),
    };
    assert_eq!(conditions(DONE), [open(1, "Wrong"), open(2, "Also wrong")]);

    let answered = format!(
        "{DONE}\n### Responses\n\n- 1-aaaa.1: fixed in abc1234\n\n## Decisions\n\n\
         ### 2026-11-10: Is finding 2 worth fixing?\n\nFinding: 1-aaaa.2\n"
    );
    // Finding 2 has a decision, still unanswered: the question is what waits.
    assert_eq!(
        conditions(&answered),
        [Condition::Unanswered(
            "2026-11-10: Is finding 2 worth fixing?".into()
        )]
    );
    let decided = format!("{answered}\nAnswer by d1: No.\nOutcome: no action\n");
    assert_eq!(conditions(&decided), []);
}

#[test]
fn the_round_is_the_latest_until_a_verdict_ends_it_and_then_the_next() {
    let caller = thread("r2");
    let round = |reviews: &[ReviewFile]| {
        let item = with_reviews(record(Status::InReview, &["t1"], DONE), reviews);
        let round = facts(&item, &caller).round();
        (round.number, round.review.map(|r| r.name.to_string()))
    };
    assert_eq!(round(&[]), (1, None));
    let open = review("1-aaaa", &["r1"], "null", "");
    assert_eq!(
        round(std::slice::from_ref(&open)),
        (1, Some("1-aaaa".into()))
    );
    let changes = review("1-aaaa", &["r1"], "changes", FIX_NOW);
    assert_eq!(round(std::slice::from_ref(&changes)), (2, None));
    let approved = review("2-bbbb", &["r1"], "approve", "");
    assert_eq!(round(&[changes.clone(), approved.clone()]), (3, None));
    // An open review of an earlier round is no longer this round's.
    assert_eq!(round(&[open, approved]), (3, None));
    // Of two open in the latest round, the caller's; else the first.
    let first = review("2-cccc", &["r1"], "null", "");
    let mine = review("2-dddd", &["r2"], "null", "");
    assert_eq!(
        round(&[changes.clone(), first.clone(), mine]),
        (2, Some("2-dddd".into()))
    );
    let other = review("2-eeee", &["r3"], "null", "");
    assert_eq!(
        round(&[changes.clone(), first.clone(), other]),
        (2, Some("2-cccc".into()))
    );
    // The first verdict in a round ends it, and a file left open in it by
    // a second reviewer is not the next round's.
    let decided = review("2-ffff", &["r3"], "changes", FIX_NOW);
    assert_eq!(round(&[changes, first, decided]), (3, None));
}

#[test]
fn a_thread_holding_one_role_is_told_it_when_it_runs_another_roles_command() {
    let item = with_reviews(
        record(Status::InReview, &["t0", "t1"], DONE),
        &[review("1-aaaa", &["r0", "r1"], "null", "")],
    );
    let r1 = thread("r1");
    for act in [Act::Submit, Act::TakeOver, Act::Merge, Act::Start] {
        assert!(
            matches!(
                check(act, &facts(&item, &r1)),
                Err(Refusal::Role(Role::Reviewer))
            ),
            "{act:?}"
        );
    }
    let t1 = thread("t1");
    for act in [Act::VerdictApprove, Act::VerdictChanges] {
        assert!(
            matches!(
                check(act, &facts(&item, &t1)),
                Err(Refusal::Role(Role::Implementer))
            ),
            "{act:?}"
        );
    }
    // An author asking to review is refused by the condition, current
    // holder or not.
    for author in ["t0", "t1"] {
        let author = thread(author);
        match check(Act::Review, &facts(&item, &author)) {
            Err(Refusal::Unmet(unmet)) => assert_eq!(unmet, [Condition::FormerImplementer]),
            other => panic!("{other:?}"),
        }
    }
    // A former reviewer, and anyone else, is told who holds the review.
    for other in ["r0", "x9"] {
        let other = thread(other);
        match check(Act::VerdictApprove, &facts(&item, &other)) {
            Err(Refusal::NotHolder {
                role: Role::Reviewer,
                holder,
            }) => assert_eq!(holder, Some(thread("r1"))),
            refused => panic!("{refused:?}"),
        }
    }
    // Once the verdict is in, the reviewer holds no role, and no open file.
    let decided = with_reviews(
        record(Status::ChangesRequested, &["t1"], DONE),
        &[review("1-aaaa", &["r1"], "changes", FIX_NOW)],
    );
    assert!(matches!(
        check(Act::Submit, &facts(&decided, &r1)),
        Err(Refusal::NotHolder {
            role: Role::Implementer,
            ..
        })
    ));
    let unopened = record(Status::InReview, &["t1"], DONE);
    assert!(matches!(
        check(Act::VerdictApprove, &facts(&unopened, &r1)),
        Err(Refusal::NotHolder {
            role: Role::Reviewer,
            holder: None
        })
    ));
}

#[test]
fn verdicts_count_open_findings_across_rounds_and_refuse_an_invalid_file() {
    let r1 = thread("r1");
    let verdict = |reviews: &[ReviewFile], act: Act| {
        let item = with_reviews(record(Status::InReview, &["t1"], DONE), reviews);
        check(act, &facts(&item, &r1))
    };
    let round_1 = review("1-aaaa", &["r0"], "changes", FIX_NOW);
    let carry = |now: &str| {
        review(
            "2-bbbb",
            &["r1"],
            "null",
            &format!("\n## Finding 1: Wrong\nRecommendation: fix now\nWas: 1-aaaa.1\nNow: {now}\n"),
        )
    };
    let open_as = |name: &str| Condition::OpenFinding {
        finding: name.parse().unwrap(),
        title: "Wrong".into(),
    };
    // Kept, the carried finding is open in the file: changes, not approve.
    let kept = [round_1.clone(), carry("kept")];
    assert_eq!(
        verdict(&kept, Act::VerdictChanges).unwrap(),
        Some(Status::ChangesRequested)
    );
    match verdict(&kept, Act::VerdictApprove) {
        Err(Refusal::Unmet(unmet)) => assert_eq!(unmet, [open_as("2-bbbb.1")]),
        other => panic!("{other:?}"),
    }
    // Withdrawn, nothing is open: approve, not changes.
    let withdrawn = [round_1.clone(), carry("withdrawn")];
    assert_eq!(
        verdict(&withdrawn, Act::VerdictApprove).unwrap(),
        Some(Status::Approved)
    );
    match verdict(&withdrawn, Act::VerdictChanges) {
        Err(Refusal::Unmet(unmet)) => assert_eq!(unmet, [Condition::NoOpenFinding]),
        other => panic!("{other:?}"),
    }
    // Not carried at all, round 1's finding stays open and blocks approve.
    let dropped = [round_1.clone(), review("2-bbbb", &["r1"], "null", "")];
    match verdict(&dropped, Act::VerdictApprove) {
        Err(Refusal::Unmet(unmet)) => assert_eq!(unmet, [open_as("1-aaaa.1")]),
        other => panic!("{other:?}"),
    }
    // A `Was:` naming no earlier finding is refused as the file's problem.
    let bad = review(
        "2-bbbb",
        &["r1"],
        "null",
        "\n## Finding 1: Wrong\nRecommendation: fix now\nWas: 1-aaaa.7\nNow: fixed\n",
    );
    match verdict(&[round_1, bad], Act::VerdictApprove) {
        Err(Refusal::Record(problem)) => {
            assert_eq!(problem.file, PathBuf::from("2-bbbb.md"));
            assert!(problem.message.contains("1-aaaa.7"), "{problem}");
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_round_carries_what_earlier_rounds_left_open_with_each_text_and_response() {
    let body = format!(
        "{DONE}\n### Responses\n\n- 2-bbbb.1: rebutted: it is right\n- 1-aaaa.1: fixed in abc1234\n"
    );
    let item = with_reviews(
        record(Status::InReview, &["t1"], &body),
        &[
            review(
                "1-aaaa",
                &["r0"],
                "changes",
                "\n## Finding 1: Wrong\nRecommendation: fix now\nFirst text.\n\n\
                 ## Finding 2: Later\nRecommendation: defer\n\n\
                 ## Finding 3: Gone\nRecommendation: fix now\n",
            ),
            review(
                "2-bbbb",
                &["r1"],
                "changes",
                "\n## Finding 1: Wrong\nRecommendation: fix now\nWas: 1-aaaa.1\nNow: kept\nStill.\n\n\
                 ## Finding 2: Gone\nRecommendation: fix now\nWas: 1-aaaa.3\nNow: fixed\n",
            ),
            // This round's own file is not read.
            review("3-cccc", &["r2"], "null", "\n## Finding 1: New\nRecommendation: fix now\n"),
        ],
    );
    let carried = carried(&item, 3).unwrap();
    assert_eq!(carried.len(), 1);
    let only = &carried[0];
    assert_eq!(only.finding.name.to_string(), "2-bbbb.1");
    let texts: Vec<(String, &str)> = only
        .texts
        .iter()
        .map(|(name, text)| (name.to_string(), text.as_str()))
        .collect();
    assert_eq!(
        texts,
        [
            (
                "1-aaaa.1".to_string(),
                "Recommendation: fix now\nFirst text.\n\n"
            ),
            (
                "2-bbbb.1".to_string(),
                "Recommendation: fix now\nWas: 1-aaaa.1\nNow: kept\nStill.\n\n"
            ),
        ]
    );
    assert_eq!(
        only.response.as_ref().unwrap().finding().to_string(),
        "2-bbbb.1"
    );
    // Round 2 carries round 1's two open fix-now findings, not the deferred one.
    let names: Vec<String> = carried_names(&item, 2);
    assert_eq!(names, ["1-aaaa.1", "1-aaaa.3"]);
}

fn carried_names(item: &ItemDir, round: u32) -> Vec<String> {
    carried(item, round)
        .unwrap()
        .iter()
        .map(|c| c.finding.name.to_string())
        .collect()
}
