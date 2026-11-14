use atelier_app::acts::{
    check, check_mission, next, question, row, settle_refusals, unmet, unsettled, Act, Condition,
    Facts, MissionFacts, Refusal, Role, TABLE,
};
use atelier_app::{carried, carried_plan, entry};
use atelier_core::{FindingName, MissionStatus, Settlement, Status, Thread};
use atelier_records::{parse_item, parse_mission, parse_review, ItemDir, MissionDir, ReviewFile};
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

/// A mission that is `status`, its Result written or not, with `reviews`.
fn mission(status: MissionStatus, result: &str, reviews: &[ReviewFile]) -> MissionDir {
    let status = status_text(status.status());
    let text = format!(
        "---\nschema: atelier.mission\nschema_version: 3\nid: demo-m1ss\ntitle: M\nstatus: {status}\n\
         labels: []\nlead: null\nplanners:\n- p1\nready: null\ncreated_at: 2026-11-10T09:00:00Z\n---\n\
         ## Outcome\no\n\n## Result\n{result}"
    );
    let path = PathBuf::from("demo-m1ss/mission.md");
    MissionDir {
        mission: parse_mission(&path, &text).unwrap(),
        path,
        reviews: reviews.to_vec(),
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
const SPECIFIED: [(Act, &[Status], Option<Status>); 14] = [
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
    (
        Act::Decide,
        &[
            Status::Building,
            Status::InReview,
            Status::ChangesRequested,
            Status::Approved,
        ],
        None,
    ),
    (Act::Merge, &[Status::Approved], Some(Status::Done)),
    (Act::MergeTrivial, &[Status::Building], Some(Status::Done)),
    (
        Act::Drop,
        &[
            Status::Draft,
            Status::Ready,
            Status::Building,
            Status::InReview,
            Status::ChangesRequested,
            Status::Approved,
        ],
        Some(Status::Dropped),
    ),
    (
        Act::Close,
        &[Status::Draft, Status::Ready],
        Some(Status::Done),
    ),
    // `new --from` and `drop` on a finding change the finding, not its item.
    (Act::Settle, &[Status::Done], None),
];

/// The acts on a mission, from the same table, where a mission's `open` is
/// written `ready` and the old `draft` is `draft`.
const MISSION_SPECIFIED: [(Act, &[Status], Option<Status>); 7] = [
    // `lead` binds its lead to an open mission, and its status stays.
    (Act::Lead, &[Status::Ready], None),
    // A proposed plan's branch: `submit` writes an old draft open, and
    // `merge` changes no status; a plan dropped there does neither.
    (
        Act::SubmitMission,
        &[Status::Draft, Status::Ready],
        Some(Status::Ready),
    ),
    (Act::MergeMission, &[Status::Draft, Status::Ready], None),
    (
        Act::Drop,
        &[Status::Draft, Status::Ready],
        Some(Status::Dropped),
    ),
    (Act::CloseMission, &[Status::Ready], Some(Status::Done)),
    // A plan review's verdict changes no status, from any.
    (
        Act::PlanVerdictChanges,
        &[Status::Draft, Status::Ready, Status::Done, Status::Dropped],
        None,
    ),
    (
        Act::PlanVerdictApprove,
        &[Status::Draft, Status::Ready, Status::Done, Status::Dropped],
        None,
    ),
];

/// The acts on no record a status is taken from: a mission's creation on
/// its branch, and a backlog branch's start, submit and merge.
const UNRECORDED_SPECIFIED: [(Act, Option<Status>); 4] = [
    (Act::StartMission, Some(Status::Ready)),
    (Act::StartBacklog, None),
    (Act::SubmitBacklog, None),
    (Act::MergeBacklog, None),
];

#[test]
fn a_backlog_branchs_acts_run_from_no_status_for_any_thread_and_are_not_an_implementers() {
    for (act, to) in UNRECORDED_SPECIFIED {
        let row = row(act);
        assert!(row.from.is_empty(), "{act:?}");
        assert_eq!(row.to, to, "{act:?}");
        // Any thread: a planner's act, not the implementer's act on an
        // item. Only `start --mission` binds, its caller as the planner.
        assert_eq!(row.role, Role::Planner, "{act:?}");
        assert_eq!(row.binds, act == Act::StartMission, "{act:?}");
    }
    assert_eq!(row(Act::SubmitBacklog).command, "submit");
    assert_eq!(row(Act::MergeBacklog).command, "merge");
    assert_eq!(row(Act::StartBacklog).command, "start --backlog");
    assert_eq!(row(Act::StartMission).command, "start --mission");
}

#[test]
fn every_act_in_the_table_from_every_status_sets_the_specified_status_or_is_refused() {
    // Each item's act, and each mission's, `lead` among them; `drop` is
    // both; and the acts on no record.
    assert_eq!(
        TABLE.len(),
        SPECIFIED.len() + MISSION_SPECIFIED.len() - 1 + UNRECORDED_SPECIFIED.len()
    );
    for (act, from, to) in SPECIFIED {
        for status in ALL {
            // Every condition holds: the caller holds the item, except a
            // reviewer, who never implemented it and, for a verdict, holds
            // the round's review, which has an open fix-now finding for
            // `changes` and none for `approve`; a take-over has its branch
            // and a start or a close has none.
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
            // A decider has a question to answer.
            let body = match act {
                Act::Decide => format!("{DONE}\n## Decisions\n\n### 2026-11-10: Q?\n"),
                _ => DONE.to_string(),
            };
            let item = with_reviews(record(status, &["t1"], &body), &reviews);
            let reviewer = matches!(act, Act::Review | Act::VerdictChanges | Act::VerdictApprove);
            // A planner holds nothing; the item's holder readying it is
            // refused by its status too, never by its role. A decider is
            // neither the item's implementer nor a reviewer.
            let caller = thread(match act {
                _ if reviewer => "r1",
                Act::Decide => "d1",
                Act::Ready if status == Status::Draft => "p1",
                _ => "t1",
            });
            let facts = Facts {
                item: &item,
                branch: !matches!(act, Act::Start | Act::Close),
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
    // Each mission's act, from each of a mission's statuses, with every
    // condition holding: its items done or dropped, its Result written, and
    // the caller's plan review holding an open fix-now finding for
    // `changes` and none for `approve`.
    let items = [
        ("demo-d0ne".to_string(), Status::Done),
        ("demo-dr0p".to_string(), Status::Dropped),
    ];
    for (act, from, to) in MISSION_SPECIFIED {
        for status in MissionStatus::ALL {
            let findings = if act == Act::PlanVerdictChanges {
                FIX_NOW
            } else {
                ""
            };
            let file = review("1-aaaa", &["r1"], "null", findings);
            let dir = mission(status, "Done.\n", std::slice::from_ref(&file));
            let caller = thread("p1");
            let facts = MissionFacts {
                mission: &dir,
                caller: &caller,
                items: &items,
                review: Some(&file),
            };
            match check_mission(act, &facts) {
                Ok(set) => {
                    assert!(from.contains(&status.status()), "{act:?} from {status:?}");
                    assert_eq!(
                        set.map(MissionStatus::status),
                        to,
                        "{act:?} from {status:?}"
                    );
                }
                Err(Refusal::From {
                    act: refused,
                    status: at,
                }) => {
                    assert!(!from.contains(&status.status()), "{act:?} from {status:?}");
                    assert_eq!((refused, at), (act, status.status()));
                }
                Err(other) => panic!("{act:?} from {status:?}: {other:?}"),
            }
        }
        // Drop's row also holds an item's started statuses.
        assert!(from.iter().all(|status| row(act).from.contains(status)));
    }
    // Anyone plans a mission, and is bound as its planner.
    let plan = row(Act::StartMission);
    assert_eq!(
        (plan.role, plan.binds, plan.from, plan.to),
        (Role::Planner, true, &[][..], Some(Status::Ready))
    );
    let lead = row(Act::Lead);
    assert_eq!(
        (lead.role, lead.binds, lead.from, lead.to),
        (Role::Lead, true, &[Status::Ready][..], None)
    );
}

#[test]
fn closing_a_mission_needs_each_item_done_or_dropped_and_its_result_written() {
    let dir = mission(MissionStatus::Open, "\n### Notes\n\n", &[]);
    let items = [
        ("demo-d0ne".to_string(), Status::Done),
        ("demo-dr4f".to_string(), Status::Draft),
        ("demo-bu1d".to_string(), Status::Building),
        ("demo-dr0p".to_string(), Status::Dropped),
    ];
    let caller = thread("p1");
    let facts = MissionFacts {
        mission: &dir,
        caller: &caller,
        items: &items,
        review: None,
    };
    let Err(Refusal::Unmet(unmet)) = check_mission(Act::CloseMission, &facts) else {
        panic!("closed a mission with open items");
    };
    assert_eq!(
        unmet,
        [
            Condition::ItemOpen {
                id: "demo-dr4f".into(),
                status: Status::Draft
            },
            Condition::ItemOpen {
                id: "demo-bu1d".into(),
                status: Status::Building
            },
            Condition::NoResult,
        ]
    );
    // A mission with no items closes once its Result is written.
    let written = mission(MissionStatus::Open, "It shipped.\n", &[]);
    let facts = MissionFacts {
        mission: &written,
        caller: &caller,
        items: &[],
        review: None,
    };
    assert_eq!(
        check_mission(Act::CloseMission, &facts).unwrap(),
        Some(MissionStatus::Done)
    );
}

#[test]
fn a_finding_is_settled_only_when_its_item_is_done_and_it_is_deferred_and_unsettled() {
    let findings = "\n## Finding 1: Later\nRecommendation: defer\n\
        \n## Finding 2: Fine\nRecommendation: no action\n\
        \n## Finding 3: Gone\nRecommendation: defer\nDropped by t9: obsolete\n\
        \n## Finding 4: Slow\nRecommendation: fix now\n";
    let decided = format!(
        "{DONE}\n## Decisions\n\n### 2026-11-10: Now?\nFinding: 1-aaaa.4\nAnswer by t5: Later.\nOutcome: defer\n"
    );
    let reviews = [review("1-aaaa", &["r1"], "approve", findings)];
    let done = with_reviews(record(Status::Done, &["t1"], &decided), &reviews);
    let caller = thread("p1");
    let name = |text: &str| text.parse::<FindingName>().unwrap();
    let refused = |text| settle_refusals(&facts(&done, &caller), &name(text)).unwrap();
    let unmet = |text| match refused(text).as_slice() {
        [Refusal::Unmet(unmet)] => unmet.clone(),
        other => panic!("{text}: {other:?}"),
    };
    for settles in ["1-aaaa.1", "1-aaaa.4"] {
        assert!(refused(settles).is_empty(), "{settles}");
    }
    assert_eq!(
        unmet("1-aaaa.2"),
        [Condition::NotDeferred(name("1-aaaa.2"))]
    );
    assert_eq!(
        unmet("1-aaaa.3"),
        [Condition::Settled {
            finding: name("1-aaaa.3"),
            settlement: Settlement::Dropped {
                by: thread("t9"),
                reason: "obsolete".into()
            }
        }]
    );
    assert_eq!(unmet("1-aaaa.5"), [Condition::NoFinding(name("1-aaaa.5"))]);
    assert_eq!(unmet("2-aaaa.1"), [Condition::NoFinding(name("2-aaaa.1"))]);
    assert_eq!(names_unsettled(&done), [name("1-aaaa.1"), name("1-aaaa.4")]);

    // An item that is not done has no finding to settle yet.
    for status in [Status::Approved, Status::Dropped] {
        let open = with_reviews(record(status, &["t1"], &decided), &reviews);
        match &settle_refusals(&facts(&open, &caller), &name("1-aaaa.1")).unwrap()[..] {
            [Refusal::From { act, status: at }] => assert_eq!((*act, *at), (Act::Settle, status)),
            other => panic!("{other:?}"),
        }
        assert!(names_unsettled(&open).is_empty());
    }
}

fn names_unsettled(item: &ItemDir) -> Vec<FindingName> {
    unsettled(item)
        .unwrap()
        .into_iter()
        .map(|f| f.name)
        .collect()
}

#[test]
fn closing_an_item_needs_no_branch_named_for_it() {
    let item = record(Status::Ready, &[], DONE);
    let caller = thread("p1");
    let facts = facts(&item, &caller);
    match check(Act::Close, &facts) {
        Err(Refusal::Unmet(unmet)) => assert_eq!(unmet, [Condition::BranchExists]),
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_plan_verdict_counts_its_own_file_and_earlier_rounds_not_a_blind_one() {
    // Round 1: two blind reviews, one with an open fix-now finding. Round
    // 2's file carries it.
    let raised = review("1-aaaa", &["r1"], "changes", FIX_NOW);
    let blind = review("1-bbbb", &["r2"], "null", "");
    let dir = mission(MissionStatus::Draft, "", &[raised.clone(), blind.clone()]);
    let caller = thread("r1");
    let verdict = |dir: &MissionDir, file: &ReviewFile, act| {
        let facts = MissionFacts {
            mission: dir,
            caller: &caller,
            items: &[],
            review: Some(file),
        };
        check_mission(act, &facts)
    };
    // The blind reviewer approves its own clean file, whatever the other holds.
    assert!(verdict(&dir, &blind, Act::PlanVerdictApprove).is_ok());
    assert!(matches!(
        verdict(&dir, &blind, Act::PlanVerdictChanges),
        Err(Refusal::Unmet(unmet)) if unmet == [Condition::NoOpenFinding]
    ));
    // Round 2 carries round 1's open finding, and opens until it says it
    // was fixed.
    assert_eq!(
        carried_plan(&dir, 2)
            .unwrap()
            .iter()
            .map(|c| c.finding.name.to_string())
            .collect::<Vec<_>>(),
        ["1-aaaa.1"]
    );
    let missing = review("2-cccc", &["r3"], "null", "");
    let dir2 = mission(
        MissionStatus::Draft,
        "",
        &[raised.clone(), blind.clone(), missing.clone()],
    );
    assert!(matches!(
        verdict(&dir2, &missing, Act::PlanVerdictApprove),
        Err(Refusal::Unmet(unmet)) if unmet.len() == 1
    ));
    let fixed = review(
        "2-cccc",
        &["r3"],
        "null",
        "\n## Finding 1: Wrong\nRecommendation: fix now\nWas: 1-aaaa.1\nNow: fixed\n",
    );
    let dir2 = mission(
        MissionStatus::Draft,
        "",
        &[raised.clone(), blind, fixed.clone()],
    );
    assert!(verdict(&dir2, &fixed, Act::PlanVerdictApprove).is_ok());
    // A blind file of the same round that carries the earlier finding does
    // not close it for a file that left it out.
    let carrier = review(
        "2-eeee",
        &["r5"],
        "changes",
        "\n## Finding 1: Wrong\nRecommendation: fix now\nWas: 1-aaaa.1\nNow: kept\n",
    );
    let both = mission(
        MissionStatus::Draft,
        "",
        &[raised.clone(), missing.clone(), carrier],
    );
    assert!(matches!(
        verdict(&both, &missing, Act::PlanVerdictApprove),
        Err(Refusal::Unmet(unmet)) if unmet == [Condition::OpenFinding {
            finding: "1-aaaa.1".parse().unwrap(),
            title: "Wrong".into(),
        }]
    ));
    // A file that does not validate is refused for its problem, which is a
    // rule's: it reads.
    let mut wrong_round = review("2-dddd", &["r4"], "null", "");
    wrong_round.review.round = 3;
    let dir3 = mission(MissionStatus::Draft, "", std::slice::from_ref(&wrong_round));
    assert!(matches!(
        verdict(&dir3, &wrong_round, Act::PlanVerdictApprove),
        Err(Refusal::Unmet(unmet))
            if matches!(&unmet[..], [Condition::Invalid(problem)] if problem.field == "round")
    ));
}

#[test]
fn start_asks_of_the_items_mission_only_that_it_is_agreed_and_not_dropped() {
    let item = record(Status::Ready, &[], DONE);
    let caller = thread("t1");
    let mission = |status| Some(("demo-m1ss".to_string(), status));
    // No mission; an open one on the base; a done one; the old draft,
    // which is proposed; a dropped one; one not on the base.
    let cases = [
        None,
        mission(Some(Status::Ready)),
        mission(Some(Status::Done)),
        mission(Some(Status::Draft)),
        mission(Some(Status::Dropped)),
        mission(None),
    ];
    for named in cases {
        let facts = Facts {
            mission: named.clone(),
            branch: false,
            ..facts(&item, &caller)
        };
        let expected = match &named {
            Some((id, status @ (None | Some(Status::Draft | Status::Dropped)))) => {
                vec![Condition::MissionNotOpen {
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
fn taking_over_or_merging_an_item_needs_its_mission_not_dropped() {
    let caller = thread("t2");
    let building = record(Status::Building, &["t1"], DONE);
    let approved = with_reviews(
        record(Status::Approved, &["t2"], DONE),
        &[review("1-aaaa", &["r1"], "approve", "")],
    );
    for (act, item) in [(Act::TakeOver, &building), (Act::Merge, &approved)] {
        for status in [Status::Ready, Status::Done, Status::Dropped] {
            let facts = Facts {
                mission: Some(("demo-m1ss".to_string(), Some(status))),
                ..facts(item, &caller)
            };
            let checked = check(act, &facts);
            match status {
                Status::Dropped => assert!(
                    matches!(
                        &checked,
                        Err(Refusal::Unmet(unmet)) if *unmet == [Condition::MissionNotOpen {
                            id: "demo-m1ss".into(),
                            status: Some(Status::Dropped),
                        }]
                    ),
                    "{act:?}: {checked:?}"
                ),
                _ => assert!(checked.is_ok(), "{act:?} {status:?}: {checked:?}"),
            }
        }
    }
}

#[test]
fn only_a_planner_merges_a_proposed_plan_and_none_merges_a_dropped_one() {
    let dir = mission(MissionStatus::Open, "", &[]);
    let facts = |caller: &Thread| {
        check_mission(
            Act::MergeMission,
            &MissionFacts {
                mission: &dir,
                caller,
                items: &[],
                review: None,
            },
        )
    };
    assert!(facts(&thread("p1")).is_ok());
    assert!(matches!(
        facts(&thread("t9")),
        Err(Refusal::NotHolder { role: Role::Planner, holder: Some(last) }) if last == thread("p1")
    ));
    let dropped = mission(MissionStatus::Dropped, "Dropped: no.\n", &[]);
    let planner = thread("p1");
    for act in [Act::SubmitMission, Act::MergeMission] {
        let checked = check_mission(
            act,
            &MissionFacts {
                mission: &dropped,
                caller: &planner,
                items: &[],
                review: None,
            },
        );
        assert!(
            matches!(
                checked,
                Err(Refusal::From {
                    status: Status::Dropped,
                    ..
                })
            ),
            "{act:?}: {checked:?}"
        );
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
fn submit_needs_each_open_finding_answered_and_no_question_waits_on_it() {
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
        rebutted: false,
    };
    assert_eq!(conditions(DONE), [open(1, "Wrong"), open(2, "Also wrong")]);

    let answered = format!(
        "{DONE}\n### Responses\n\n- 1-aaaa.1: fixed in abc1234\n\n## Decisions\n\n\
         ### 2026-11-10: Is finding 2 worth fixing?\n\nFinding: 1-aaaa.2\n"
    );
    // A question written before `escalate` was removed waits on nothing:
    // unanswered, finding 2 still needs its response.
    assert_eq!(conditions(&answered), [open(2, "Also wrong")]);
    // Decided then, with an outcome, finding 2 reads as settled.
    let decided = format!("{answered}\nAnswer by d1: No.\nOutcome: no action\n");
    assert_eq!(conditions(&decided), []);
    // Answered with no outcome, finding 2 is still open and needs its
    // response.
    let answered_only = format!("{answered}\nAnswer by d1: Fix it.\n");
    assert_eq!(conditions(&answered_only), [open(2, "Also wrong")]);
}

/// A finding carried as `2-bbbb.1` from `1-aaaa.1`, whose round 1 was
/// reviewed by r1 and then r2, and round 2 by r3.
fn carried_twice() -> [ReviewFile; 2] {
    let kept = "\n## Finding 1: Wrong\nRecommendation: fix now\nWas: 1-aaaa.1\nNow: kept\n";
    [
        review("1-aaaa", &["r1", "r2"], "changes", FIX_NOW),
        review("2-bbbb", &["r3"], "changes", kept),
    ]
}

#[test]
fn a_kept_finding_whose_rebuttal_was_kept_has_had_its_one_try() {
    let body = format!("{DONE}\n### Responses\n\n- 1-aaaa.1: rebutted: it is right\n");
    let item = with_reviews(
        record(Status::ChangesRequested, &["t1"], &body),
        &carried_twice(),
    );
    let caller = thread("t1");
    assert_eq!(
        unmet(Act::Submit, &facts(&item, &caller)).unwrap(),
        [Condition::NoResponse {
            finding: "2-bbbb.1".parse().unwrap(),
            title: "Wrong".into(),
            rebutted: true,
        }]
    );
}

#[test]
fn the_oldest_question_is_decided_by_a_thread_that_never_implemented_the_item_nor_reviewed_its_finding(
) {
    let reviews = carried_twice();
    let item = |body: &str| {
        let body = format!("{DONE}\n## Decisions\n\n{body}");
        with_reviews(
            record(Status::ChangesRequested, &["t1", "t2"], &body),
            &reviews,
        )
    };
    let answered = "### 2026-11-09: First?\n\nAnswer by d0: Yes.\n\n";
    let about = "### 2026-11-10: Worth fixing?\n\nFinding: 2-bbbb.1\n\n";
    let free = "### 2026-11-10: Later?\n";
    let waiting = item(&format!("{answered}{about}{free}"));
    assert_eq!(question(&waiting).unwrap().unwrap().0, 1);
    let decide = |item: &ItemDir, caller: &str| {
        let caller = thread(caller);
        unmet(Act::Decide, &facts(item, &caller)).unwrap()
    };
    // Every implementer, the holder and the one before; every reviewer of
    // the finding and of the one it carries, a taken-over one too.
    for implementer in ["t1", "t2"] {
        assert_eq!(
            decide(&waiting, implementer),
            [Condition::FormerImplementer]
        );
    }
    let finding = "2-bbbb.1".parse().unwrap();
    for reviewer in ["r1", "r2", "r3"] {
        assert_eq!(
            decide(&waiting, reviewer),
            [Condition::ReviewedFinding(Clone::clone(&finding))]
        );
    }
    assert_eq!(decide(&waiting, "d1"), []);
    // The holder is refused by that condition, not told its role.
    let holder = thread("t2");
    assert!(matches!(
        check(Act::Decide, &facts(&waiting, &holder)),
        Err(Refusal::Unmet(unmet)) if unmet == [Condition::FormerImplementer]
    ));
    // Once it is answered, the next is about no finding: a reviewer may
    // decide that one, an implementer still not.
    let next = item(&format!("{answered}{about}Answer by d1: Fix it.\n\n{free}"));
    assert_eq!(question(&next).unwrap().unwrap().0, 2);
    assert_eq!(decide(&next, "r3"), []);
    assert_eq!(decide(&next, "t1"), [Condition::FormerImplementer]);
    // Nothing waits.
    let none = item(answered);
    assert_eq!(decide(&none, "d1"), [Condition::NoQuestion]);
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
    // A `Was:` naming no earlier finding is refused as the file's problem,
    // a rule's, since the file reads; round 1's finding stays open with it.
    let bad = review(
        "2-bbbb",
        &["r1"],
        "null",
        "\n## Finding 1: Wrong\nRecommendation: fix now\nWas: 1-aaaa.7\nNow: fixed\n",
    );
    match verdict(&[round_1, bad], Act::VerdictApprove) {
        Err(Refusal::Unmet(unmet)) => match &unmet[..] {
            [Condition::Invalid(problem), open] => {
                assert_eq!(problem.file, PathBuf::from("2-bbbb.md"));
                assert!(problem.message.contains("1-aaaa.7"), "{problem}");
                assert_eq!(*open, open_as("1-aaaa.1"));
            }
            other => panic!("{other:?}"),
        },
        other => panic!("{other:?}"),
    }
}

#[test]
fn what_a_decision_settled_before_decide_was_removed_stays_settled_and_nothing_decides_now() {
    let first = review("1-aaaa", &["r1"], "changes", FIX_NOW);
    let decided = format!(
        "{DONE}\n## Decisions\n\n### 2026-11-10: Worth it?\n\nFinding: 1-aaaa.1\n\
         Answer by d1: No.\nOutcome: defer\n"
    );
    let carry = |was: &str, line: &str| {
        format!("\n## Finding 1: Wrong\nRecommendation: fix now\nWas: {was}\n{line}\n")
    };
    let caller = thread("r2");
    let verdict = |body: &str, earlier: &[ReviewFile], file: &str, act: Act| {
        let round = earlier.len() + 1;
        let mut reviews = earlier.to_vec();
        reviews.push(review(&format!("{round}-cccc"), &["r2"], "null", file));
        let item = with_reviews(record(Status::InReview, &["t1"], body), &reviews);
        match check(act, &facts(&item, &caller)) {
            Ok(_) => Vec::new(),
            Err(Refusal::Unmet(unmet)) => unmet,
            Err(other) => panic!("{other:?}"),
        }
    };
    let round_1 = std::slice::from_ref(&first);
    let named = |name: &str| name.parse().unwrap();
    // The Decisions entry closed the finding: a new round need not carry
    // it, and approves.
    assert_eq!(verdict(&decided, round_1, "", Act::VerdictApprove), []);
    // A new review neither reopens it nor writes `Decided:` of its own.
    assert_eq!(
        verdict(
            &decided,
            round_1,
            &carry("1-aaaa.1", "Now: kept"),
            Act::VerdictChanges
        ),
        [Condition::Decided(named("2-cccc.1"))]
    );
    for body in [decided.as_str(), DONE] {
        assert_eq!(
            verdict(
                body,
                round_1,
                &carry("1-aaaa.1", "Decided: defer"),
                Act::VerdictApprove
            ),
            [Condition::Decided(named("2-cccc.1"))]
        );
    }
    // A round 2 written then with the tool's `Decided:` line still reads as
    // settled, and a round 3 cannot carry it or what it carries.
    let round_2 = review(
        "2-bbbb",
        &["r2"],
        "approve",
        &carry("1-aaaa.1", "Decided: defer"),
    );
    let rounds = [first.clone(), round_2];
    assert_eq!(verdict(&decided, &rounds, "", Act::VerdictApprove), []);
    for was in ["1-aaaa.1", "2-bbbb.1"] {
        assert_eq!(
            verdict(
                &decided,
                &rounds,
                &carry(was, "Now: kept"),
                Act::VerdictChanges
            ),
            [Condition::Decided(named("3-cccc.1"))],
            "{was}"
        );
    }
    // No round carries a settled finding, and the deferred one is the
    // carrier, which a done item's `list --deferred` would list.
    let item = with_reviews(record(Status::Done, &["t1"], &decided), &rounds);
    assert!(carried_names(&item, 2).is_empty());
    assert!(carried_names(&item, 3).is_empty());
    let unsettled: Vec<String> = (unsettled(&item).unwrap().iter())
        .map(|f| f.name.to_string())
        .collect();
    assert_eq!(unsettled, ["2-bbbb.1"]);
}

#[test]
fn a_reviewer_closes_a_carried_finding_by_withdrawing_it_or_lowering_it_with_a_reason() {
    let first = review("1-aaaa", &["r1"], "changes", FIX_NOW);
    let body = format!("{DONE}\n### Responses\n\n- 1-aaaa.1: rebutted: it is right\n");
    let caller = thread("r2");
    let approve = |file: &str| {
        let reviews = [first.clone(), review("2-cccc", &["r2"], "null", file)];
        let item = with_reviews(record(Status::InReview, &["t1"], &body), &reviews);
        match check(Act::VerdictApprove, &facts(&item, &caller)) {
            Ok(_) => Vec::new(),
            Err(Refusal::Unmet(unmet)) => unmet,
            // Its findings do not read.
            Err(Refusal::Record(problem)) => vec![Condition::Invalid(problem)],
            Err(other) => panic!("{other:?}"),
        }
    };
    let carry = |recommendation: &str, now: &str, reason: &str| {
        format!("\n## Finding 1: Wrong\nRecommendation: {recommendation}\nWas: 1-aaaa.1\nNow: {now}\n{reason}")
    };
    assert_eq!(approve(&carry("fix now", "withdrawn", "")), []);
    assert_eq!(
        approve(&carry("defer", "kept", "Right, but for the next item.\n")),
        []
    );
    assert_eq!(approve(&carry("no action", "kept", "Not worth it.\n")), []);
    // Kept at fix now, it stands.
    assert!(matches!(
        approve(&carry("fix now", "kept", "")).as_slice(),
        [Condition::OpenFinding { .. }]
    ));
    // Lowered with no reason, the file does not read.
    assert!(matches!(
        approve(&carry("defer", "kept", "")).as_slice(),
        [Condition::Invalid(_)]
    ));
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
