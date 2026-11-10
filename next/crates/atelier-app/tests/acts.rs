use atelier_app::acts::{check, next, row, Act, Condition, Facts, Refusal, TABLE};
use atelier_app::entry;
use atelier_core::{Status, Thread};
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

/// The architecture specification's act table, for the acts this step
/// builds, written out here so the code's table is checked against it.
const SPECIFIED: [(Act, &[Status], Option<Status>); 8] = [
    (Act::Start, &[Status::Ready], Some(Status::Building)),
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
    assert_eq!(TABLE.len(), SPECIFIED.len());
    for (act, from, to) in SPECIFIED {
        for status in ALL {
            // Every condition holds: the caller holds the item, except a
            // reviewer, who never implemented it; a take-over has its
            // branch and a start has none.
            let item = record(status, &["t1"], DONE);
            let caller = thread(if act == Act::Review { "r1" } else { "t1" });
            let facts = Facts {
                item: &item,
                branch: act != Act::Start,
                blockers: &[],
                caller: &caller,
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
    let mut reported = 0;
    for status in ALL {
        for caller in ["t1", "t0", "r1"] {
            for (body, blockers, branch) in cases {
                // t1 holds the item, t0 held it before, r1 never did.
                let item = record(status, &["t0", "t1"], body);
                let caller = thread(caller);
                let facts = Facts {
                    item: &item,
                    branch,
                    blockers,
                    caller: &caller,
                };
                let shown = entry(&facts).unwrap();
                assert_eq!(shown.next, next(&facts));
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
            caller: &caller,
        };
        assert_eq!(next(&facts), expected, "{status:?} for {caller}");
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
            caller: &caller,
        };
        match check(Act::Submit, &facts) {
            Err(Refusal::NotHolder { holder: found }) => {
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
        caller: &caller,
    };
    assert!(matches!(
        check(Act::Submit, &facts),
        Err(Refusal::NotHolder { holder: None })
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
            caller: &caller,
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
