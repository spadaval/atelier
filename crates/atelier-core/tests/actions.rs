use atelier_core::actions::{
    check, check_mission, next, row, settle_refusals, unmet, unsettled, Action, Condition, Facts,
    MissionFacts, Refusal, Role, TABLE,
};
use atelier_core::model::{FindingName, MissionStatus, Settlement, Status, Thread};
use atelier_core::records::{
    parse_item, parse_mission, parse_review, ItemDir, MissionDir, ReviewFile,
};
use atelier_core::{carried, carried_plan, entry};
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

/// The architecture specification's action table, for the actions this step
/// builds, written out here so the code's table is checked against it.
const SPECIFIED: [(Action, &[Status], Option<Status>); 13] = [
    (Action::Ready, &[Status::Draft], Some(Status::Ready)),
    (Action::Start, &[Status::Ready], Some(Status::Building)),
    // Created by the action, so it runs from no status at all.
    (Action::StartNew, &[], Some(Status::Building)),
    (
        Action::TakeOver,
        &[
            Status::Building,
            Status::InReview,
            Status::ChangesRequested,
            Status::Approved,
        ],
        None,
    ),
    (
        Action::Submit,
        &[Status::Building, Status::ChangesRequested, Status::Approved],
        Some(Status::InReview),
    ),
    (Action::Review, &[Status::InReview], None),
    (
        Action::VerdictChanges,
        &[Status::InReview],
        Some(Status::ChangesRequested),
    ),
    (
        Action::VerdictApprove,
        &[Status::InReview],
        Some(Status::Approved),
    ),
    (Action::Merge, &[Status::Approved], Some(Status::Done)),
    (
        Action::MergeTrivial,
        &[Status::Building],
        Some(Status::Done),
    ),
    (
        Action::Drop,
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
        Action::Close,
        &[Status::Draft, Status::Ready],
        Some(Status::Done),
    ),
    // `new --from` and `drop` on a finding change the finding, not its item.
    (Action::Settle, &[Status::Done], None),
];

/// The actions on a mission, from the same table, where a mission's `open` is
/// written `ready` and the old `draft` is `draft`.
const MISSION_SPECIFIED: [(Action, &[Status], Option<Status>); 7] = [
    // `lead` binds its lead to an open mission, and its status stays.
    (Action::Lead, &[Status::Ready], None),
    // A proposed plan's branch: `submit` writes an old draft open, and
    // `merge` changes no status; a plan dropped there does neither.
    (
        Action::SubmitMission,
        &[Status::Draft, Status::Ready],
        Some(Status::Ready),
    ),
    (Action::MergeMission, &[Status::Ready], None),
    (
        Action::Drop,
        &[Status::Draft, Status::Ready],
        Some(Status::Dropped),
    ),
    (Action::CloseMission, &[Status::Ready], Some(Status::Done)),
    // A plan review's verdict changes no status, from any.
    (
        Action::PlanVerdictChanges,
        &[Status::Draft, Status::Ready, Status::Done, Status::Dropped],
        None,
    ),
    (
        Action::PlanVerdictApprove,
        &[Status::Draft, Status::Ready, Status::Done, Status::Dropped],
        None,
    ),
];

/// The actions on no record a status is taken from: a mission's creation on
/// its branch, and a backlog branch's start, submit and merge.
const UNRECORDED_SPECIFIED: [(Action, Option<Status>); 4] = [
    (Action::StartMission, Some(Status::Ready)),
    (Action::StartBacklog, None),
    (Action::SubmitBacklog, None),
    (Action::MergeBacklog, None),
];

#[test]
fn a_backlog_branchs_actions_run_from_no_status_for_any_thread_and_are_not_an_implementers() {
    for (action, to) in UNRECORDED_SPECIFIED {
        let row = row(action);
        assert!(row.from.is_empty(), "{action:?}");
        assert_eq!(row.to, to, "{action:?}");
        // Any thread: a planner's action, not the implementer's action on an
        // item. Only `start --mission` binds, its caller as the planner.
        assert_eq!(row.role, Role::Planner, "{action:?}");
        assert_eq!(row.binds, action == Action::StartMission, "{action:?}");
    }
    assert_eq!(row(Action::SubmitBacklog).command, "submit");
    assert_eq!(row(Action::MergeBacklog).command, "merge");
    assert_eq!(row(Action::StartBacklog).command, "start --backlog");
    assert_eq!(row(Action::StartMission).command, "start --mission");
}

#[test]
fn every_action_in_the_table_from_every_status_sets_the_specified_status_or_is_refused() {
    // Each item's action, and each mission's, `lead` among them; `drop` is
    // both; and the actions on no record.
    assert_eq!(
        TABLE.len(),
        SPECIFIED.len() + MISSION_SPECIFIED.len() - 1 + UNRECORDED_SPECIFIED.len()
    );
    for (action, from, to) in SPECIFIED {
        for status in ALL {
            // Every condition holds: the caller holds the item, except a
            // reviewer, who never implemented it and, for a verdict, holds
            // the round's review, which has an open fix-now finding for
            // `changes` and none for `approve`; a take-over has its branch
            // and a start or a close has none.
            let findings = if action == Action::VerdictChanges {
                FIX_NOW
            } else {
                ""
            };
            // A merge needs a review that approves, and a trivial merge
            // an item nobody reviewed.
            let reviews = match action {
                Action::MergeTrivial => vec![],
                Action::Merge => vec![review("1-aaaa", &["r1"], "approve", "")],
                _ => vec![review("1-aaaa", &["r1"], "null", findings)],
            };
            let item = with_reviews(record(status, &["t1"], DONE), &reviews);
            let reviewer = matches!(
                action,
                Action::Review | Action::VerdictChanges | Action::VerdictApprove
            );
            // A planner holds nothing; the item's holder readying it is
            // refused by its status too, never by its role.
            let caller = thread(match action {
                _ if reviewer => "r1",
                Action::Ready if status == Status::Draft => "p1",
                _ => "t1",
            });
            let facts = Facts {
                item: &item,
                branch: !matches!(action, Action::Start | Action::Close),
                blockers: &[],
                mission: None,
                caller: &caller,
                checkout: None,
            };
            match check(action, &facts) {
                Ok(set) => {
                    assert!(from.contains(&status), "{action:?} from {status:?}");
                    assert_eq!(set, to, "{action:?} from {status:?}");
                }
                Err(Refusal::From {
                    action: refused,
                    status: at,
                }) => {
                    assert!(!from.contains(&status), "{action:?} from {status:?}");
                    assert_eq!((refused, at), (action, status));
                }
                Err(other) => panic!("{action:?} from {status:?}: {other:?}"),
            }
        }
        assert_eq!(row(action).from, from);
    }
    // Each mission's action, from each of a mission's statuses, with every
    // condition holding: its items done or dropped, its Result written, and
    // the caller's plan review holding an open fix-now finding for
    // `changes` and none for `approve`.
    let items = [
        ("demo-d0ne".to_string(), Status::Done),
        ("demo-dr0p".to_string(), Status::Dropped),
    ];
    for (action, from, to) in MISSION_SPECIFIED {
        for status in MissionStatus::ALL {
            let findings = if action == Action::PlanVerdictChanges {
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
            match check_mission(action, &facts) {
                Ok(set) => {
                    assert!(
                        from.contains(&status.status()),
                        "{action:?} from {status:?}"
                    );
                    assert_eq!(
                        set.map(MissionStatus::status),
                        to,
                        "{action:?} from {status:?}"
                    );
                }
                Err(Refusal::From {
                    action: refused,
                    status: at,
                }) => {
                    assert!(
                        !from.contains(&status.status()),
                        "{action:?} from {status:?}"
                    );
                    assert_eq!((refused, at), (action, status.status()));
                }
                Err(other) => panic!("{action:?} from {status:?}: {other:?}"),
            }
        }
        // Drop's row also holds an item's started statuses.
        assert!(from.iter().all(|status| row(action).from.contains(status)));
    }
    // Anyone plans a mission, and is bound as its planner.
    let plan = row(Action::StartMission);
    assert_eq!(
        (plan.role, plan.binds, plan.from, plan.to),
        (Role::Planner, true, &[][..], Some(Status::Ready))
    );
    let lead = row(Action::Lead);
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
    let Err(Refusal::Unmet(unmet)) = check_mission(Action::CloseMission, &facts) else {
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
        check_mission(Action::CloseMission, &facts).unwrap(),
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
            [Refusal::From { action, status: at }] => {
                assert_eq!((*action, *at), (Action::Settle, status))
            }
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
    match check(Action::Close, &facts) {
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
    let verdict = |dir: &MissionDir, file: &ReviewFile, action| {
        let facts = MissionFacts {
            mission: dir,
            caller: &caller,
            items: &[],
            review: Some(file),
        };
        check_mission(action, &facts)
    };
    // The blind reviewer approves its own clean file, whatever the other holds.
    assert!(verdict(&dir, &blind, Action::PlanVerdictApprove).is_ok());
    assert!(matches!(
        verdict(&dir, &blind, Action::PlanVerdictChanges),
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
        verdict(&dir2, &missing, Action::PlanVerdictApprove),
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
    assert!(verdict(&dir2, &fixed, Action::PlanVerdictApprove).is_ok());
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
        verdict(&both, &missing, Action::PlanVerdictApprove),
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
        verdict(&dir3, &wrong_round, Action::PlanVerdictApprove),
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
        match check(Action::Start, &facts) {
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
    for (action, item) in [(Action::TakeOver, &building), (Action::Merge, &approved)] {
        for status in [Status::Ready, Status::Done, Status::Dropped] {
            let facts = Facts {
                mission: Some(("demo-m1ss".to_string(), Some(status))),
                ..facts(item, &caller)
            };
            let checked = check(action, &facts);
            match status {
                Status::Dropped => assert!(
                    matches!(
                        &checked,
                        Err(Refusal::Unmet(unmet)) if *unmet == [Condition::MissionNotOpen {
                            id: "demo-m1ss".into(),
                            status: Some(Status::Dropped),
                        }]
                    ),
                    "{action:?}: {checked:?}"
                ),
                _ => assert!(checked.is_ok(), "{action:?} {status:?}: {checked:?}"),
            }
        }
    }
}

#[test]
fn only_a_planner_merges_a_proposed_plan_and_none_merges_a_dropped_one() {
    let dir = mission(MissionStatus::Open, "", &[]);
    let facts = |caller: &Thread| {
        check_mission(
            Action::MergeMission,
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
    for action in [Action::SubmitMission, Action::MergeMission] {
        let checked = check_mission(
            action,
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
            "{action:?}: {checked:?}"
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
                let Some(action) = shown.next else {
                    assert!(shown.unmet.is_empty());
                    continue;
                };
                // The action status names next is never refused for its
                // status or its holder, only for its conditions.
                let checked = match check(action, &facts) {
                    Ok(_) => Vec::new(),
                    Err(Refusal::Unmet(conditions)) => conditions,
                    Err(other) => panic!("{action:?} from {status:?}: {other:?}"),
                };
                assert_eq!(shown.unmet, checked, "{action:?} from {status:?}");
                reported += checked.len();
            }
        }
    }
    assert!(reported > 0);
}

#[test]
fn the_next_action_follows_the_status_and_who_is_asking() {
    let cases = [
        (Status::Draft, "t1", None),
        (Status::Ready, "r1", Some(Action::Start)),
        (Status::Building, "t1", Some(Action::Submit)),
        (Status::Building, "t0", Some(Action::TakeOver)),
        (Status::ChangesRequested, "t1", Some(Action::Submit)),
        (Status::InReview, "t1", None),
        (Status::InReview, "t0", Some(Action::TakeOver)),
        (Status::InReview, "r1", Some(Action::Review)),
        (Status::Approved, "t1", Some(Action::Merge)),
        (Status::Approved, "r1", Some(Action::TakeOver)),
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
    for (findings, expected) in [
        ("", Action::VerdictApprove),
        (FIX_NOW, Action::VerdictChanges),
    ] {
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
        match check(Action::Submit, &facts) {
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
        check(Action::Submit, &facts),
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
        match check(Action::Submit, &facts) {
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
        unmet(Action::Submit, &facts(&item, &caller)).unwrap(),
        [Condition::NoResponse {
            finding: "2-bbbb.1".parse().unwrap(),
            title: "Wrong".into(),
            rebutted: true,
        }]
    );
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
    for action in [
        Action::Submit,
        Action::TakeOver,
        Action::Merge,
        Action::Start,
    ] {
        assert!(
            matches!(
                check(action, &facts(&item, &r1)),
                Err(Refusal::Role(Role::Reviewer))
            ),
            "{action:?}"
        );
    }
    let t1 = thread("t1");
    for action in [Action::VerdictApprove, Action::VerdictChanges] {
        assert!(
            matches!(
                check(action, &facts(&item, &t1)),
                Err(Refusal::Role(Role::Implementer))
            ),
            "{action:?}"
        );
    }
    // An author asking to review is refused by the condition, current
    // holder or not.
    for author in ["t0", "t1"] {
        let author = thread(author);
        match check(Action::Review, &facts(&item, &author)) {
            Err(Refusal::Unmet(unmet)) => assert_eq!(unmet, [Condition::FormerImplementer]),
            other => panic!("{other:?}"),
        }
    }
    // A former reviewer, and anyone else, is told who holds the review.
    for other in ["r0", "x9"] {
        let other = thread(other);
        match check(Action::VerdictApprove, &facts(&item, &other)) {
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
        check(Action::Submit, &facts(&decided, &r1)),
        Err(Refusal::NotHolder {
            role: Role::Implementer,
            ..
        })
    ));
    let unopened = record(Status::InReview, &["t1"], DONE);
    assert!(matches!(
        check(Action::VerdictApprove, &facts(&unopened, &r1)),
        Err(Refusal::NotHolder {
            role: Role::Reviewer,
            holder: None
        })
    ));
}

#[test]
fn verdicts_count_open_findings_across_rounds_and_refuse_an_invalid_file() {
    let r1 = thread("r1");
    let verdict = |reviews: &[ReviewFile], action: Action| {
        let item = with_reviews(record(Status::InReview, &["t1"], DONE), reviews);
        check(action, &facts(&item, &r1))
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
        verdict(&kept, Action::VerdictChanges).unwrap(),
        Some(Status::ChangesRequested)
    );
    match verdict(&kept, Action::VerdictApprove) {
        Err(Refusal::Unmet(unmet)) => assert_eq!(unmet, [open_as("2-bbbb.1")]),
        other => panic!("{other:?}"),
    }
    // Withdrawn, nothing is open: approve, not changes.
    let withdrawn = [round_1.clone(), carry("withdrawn")];
    assert_eq!(
        verdict(&withdrawn, Action::VerdictApprove).unwrap(),
        Some(Status::Approved)
    );
    match verdict(&withdrawn, Action::VerdictChanges) {
        Err(Refusal::Unmet(unmet)) => assert_eq!(unmet, [Condition::NoOpenFinding]),
        other => panic!("{other:?}"),
    }
    // Not carried at all, round 1's finding stays open and blocks approve.
    let dropped = [round_1.clone(), review("2-bbbb", &["r1"], "null", "")];
    match verdict(&dropped, Action::VerdictApprove) {
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
    match verdict(&[round_1, bad], Action::VerdictApprove) {
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
    let verdict = |body: &str, earlier: &[ReviewFile], file: &str, action: Action| {
        let round = earlier.len() + 1;
        let mut reviews = earlier.to_vec();
        reviews.push(review(&format!("{round}-cccc"), &["r2"], "null", file));
        let item = with_reviews(record(Status::InReview, &["t1"], body), &reviews);
        match check(action, &facts(&item, &caller)) {
            Ok(_) => Vec::new(),
            Err(Refusal::Unmet(unmet)) => unmet,
            Err(other) => panic!("{other:?}"),
        }
    };
    let round_1 = std::slice::from_ref(&first);
    let named = |name: &str| name.parse().unwrap();
    // The Decisions entry closed the finding: a new round need not carry
    // it, and approves.
    assert_eq!(verdict(&decided, round_1, "", Action::VerdictApprove), []);
    // A new review neither reopens it nor writes `Decided:` of its own.
    assert_eq!(
        verdict(
            &decided,
            round_1,
            &carry("1-aaaa.1", "Now: kept"),
            Action::VerdictChanges
        ),
        [Condition::Decided(named("2-cccc.1"))]
    );
    for body in [decided.as_str(), DONE] {
        assert_eq!(
            verdict(
                body,
                round_1,
                &carry("1-aaaa.1", "Decided: defer"),
                Action::VerdictApprove
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
    assert_eq!(verdict(&decided, &rounds, "", Action::VerdictApprove), []);
    for was in ["1-aaaa.1", "2-bbbb.1"] {
        assert_eq!(
            verdict(
                &decided,
                &rounds,
                &carry(was, "Now: kept"),
                Action::VerdictChanges
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
        match check(Action::VerdictApprove, &facts(&item, &caller)) {
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

/// Each open mission record of the five repositories this tool serves, as
/// their bases held them on 2026-11-13, with each one's title, labels,
/// threads and body replaced, since some are private: the front matter
/// they were written in, `status: ready` with a `ready` field, or `status:
/// draft` with none.
#[test]
fn every_open_mission_record_of_the_five_repositories_reads_in_the_old_form() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/open-missions");
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    paths.sort();
    assert_eq!(paths.len(), 16);
    let (mut open, mut drafts) = (0, 0);
    let caller = thread("l1");
    for path in &paths {
        let text = std::fs::read_to_string(path).unwrap();
        let mission = parse_mission(path, &text).unwrap_or_else(|e| panic!("{e}"));
        let stored = text
            .lines()
            .find_map(|l| l.strip_prefix("status: "))
            .unwrap();
        let dir = MissionDir {
            mission: mission.clone(),
            path: path.clone(),
            reviews: Vec::new(),
        };
        let lead = check_mission(
            Action::Lead,
            &MissionFacts {
                mission: &dir,
                caller: &caller,
                items: &[],
                review: None,
            },
        );
        // An item of it, ready, as `start` sees it on the base.
        let item = record(Status::Ready, &[], DONE);
        let facts = Facts {
            mission: Some((mission.id.clone(), Some(mission.status.status()))),
            branch: false,
            ..facts(&item, &caller)
        };
        let start = check(Action::Start, &facts);
        match stored {
            // `ready` is open: led, and its items start.
            "ready" => {
                open += 1;
                assert_eq!(mission.status, MissionStatus::Open, "{}", path.display());
                assert!(lead.is_ok() && start.is_ok(), "{}", path.display());
            }
            // The old draft is proposed: neither led nor started.
            "draft" => {
                drafts += 1;
                assert_eq!(mission.status, MissionStatus::Draft, "{}", path.display());
                assert!(lead.is_err() && start.is_err(), "{}", path.display());
            }
            other => panic!("{}: {other}", path.display()),
        }
        // Its `ready` field is read and never written back.
        let written = atelier_core::records::render_mission(&mission);
        assert!(!written.contains("ready"), "{written}");
        assert!(written.contains("\nplanners: []\n"), "{written}");
    }
    assert_eq!((open, drafts), (8, 8));
}
