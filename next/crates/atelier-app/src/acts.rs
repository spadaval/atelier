//! The act table: each lifecycle act, the role that runs it, the statuses
//! it runs from, the status it sets, and its conditions. The commands check
//! an act with [`check`], and `status` lists what [`unmet`] gives for the
//! act it names next, so the two cannot disagree.

use crate::review::chain;
use atelier_core::{
    is_open, Decision, Finding, FindingName, MissionStatus, Outcome, Response, Status, Thread,
    Verdict,
};
use atelier_records::{validate, ItemDir, MissionDir, Problem, ReviewFile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Act {
    /// `ready` on an item.
    Ready,
    /// `ready` on a mission.
    ReadyMission,
    Start,
    /// `start --new`, which creates the item it starts.
    StartNew,
    TakeOver,
    Submit,
    Review,
    VerdictChanges,
    VerdictApprove,
    Escalate,
    Decide,
    Merge,
    MergeTrivial,
    /// `drop` on an item or a mission.
    Drop,
    /// `close` on an item, which ends it on the base with a result.
    Close,
    /// `close` on a mission.
    CloseMission,
    /// `verdict changes` on a plan review.
    PlanVerdictChanges,
    /// `verdict approve` on a plan review.
    PlanVerdictApprove,
    /// `lead` on a mission.
    Lead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Planner,
    Implementer,
    Reviewer,
    /// Answers an item's question: a thread that never implemented the
    /// item nor reviewed the finding the question is about.
    Decider,
    Lead,
}

pub struct Row {
    pub act: Act,
    /// The command, after `atelier`.
    pub command: &'static str,
    pub role: Role,
    /// Whether the act binds the caller to its role. An act that does not
    /// needs the caller to hold the role already, unless it is a planner's,
    /// which no thread holds.
    pub binds: bool,
    /// The statuses it runs from; none for an act that creates its record.
    pub from: &'static [Status],
    /// The status the act sets; `None` leaves it unchanged.
    pub to: Option<Status>,
}

/// The statuses of an item someone holds.
pub const STARTED: &[Status] = &[
    Status::Building,
    Status::InReview,
    Status::ChangesRequested,
    Status::Approved,
];

/// The statuses of an item or mission whose front matter a planner may
/// still change with `set`.
pub const PLANNED: &[Status] = &[Status::Draft, Status::Ready];

/// Every status a mission can have.
const MISSION: &[Status] = &[Status::Draft, Status::Ready, Status::Done, Status::Dropped];

pub const TABLE: [Row; 19] = [
    Row {
        act: Act::Ready,
        command: "ready",
        role: Role::Planner,
        binds: false,
        from: &[Status::Draft],
        to: Some(Status::Ready),
    },
    Row {
        act: Act::ReadyMission,
        command: "ready",
        role: Role::Planner,
        binds: false,
        from: &[Status::Draft],
        to: Some(Status::Ready),
    },
    Row {
        act: Act::Start,
        command: "start",
        role: Role::Implementer,
        binds: true,
        from: &[Status::Ready],
        to: Some(Status::Building),
    },
    Row {
        act: Act::StartNew,
        command: "start --new",
        role: Role::Implementer,
        binds: true,
        from: &[],
        to: Some(Status::Building),
    },
    Row {
        act: Act::TakeOver,
        command: "start",
        role: Role::Implementer,
        binds: true,
        from: STARTED,
        to: None,
    },
    Row {
        act: Act::Submit,
        command: "submit",
        role: Role::Implementer,
        binds: false,
        from: &[Status::Building, Status::ChangesRequested, Status::Approved],
        to: Some(Status::InReview),
    },
    Row {
        act: Act::Review,
        command: "review",
        role: Role::Reviewer,
        binds: true,
        from: &[Status::InReview],
        to: None,
    },
    Row {
        act: Act::VerdictChanges,
        command: "verdict changes",
        role: Role::Reviewer,
        binds: false,
        from: &[Status::InReview],
        to: Some(Status::ChangesRequested),
    },
    Row {
        act: Act::VerdictApprove,
        command: "verdict approve",
        role: Role::Reviewer,
        binds: false,
        from: &[Status::InReview],
        to: Some(Status::Approved),
    },
    Row {
        act: Act::Escalate,
        command: "escalate",
        role: Role::Implementer,
        binds: false,
        from: &[Status::Building, Status::ChangesRequested, Status::Approved],
        to: None,
    },
    // The answer stores the decider's thread, as an entry command does.
    Row {
        act: Act::Decide,
        command: "decide",
        role: Role::Decider,
        binds: true,
        from: STARTED,
        to: None,
    },
    Row {
        act: Act::Merge,
        command: "merge",
        role: Role::Implementer,
        binds: false,
        from: &[Status::Approved],
        to: Some(Status::Done),
    },
    Row {
        act: Act::MergeTrivial,
        command: "merge --trivial",
        role: Role::Implementer,
        binds: false,
        from: &[Status::Building],
        to: Some(Status::Done),
    },
    Row {
        act: Act::Drop,
        command: "drop",
        role: Role::Planner,
        binds: false,
        from: &[
            Status::Draft,
            Status::Ready,
            Status::Building,
            Status::InReview,
            Status::ChangesRequested,
            Status::Approved,
        ],
        to: Some(Status::Dropped),
    },
    // Anyone may close an item; like a planner's act, it needs no role.
    Row {
        act: Act::Close,
        command: "close",
        role: Role::Planner,
        binds: false,
        from: PLANNED,
        to: Some(Status::Done),
    },
    // Its lead or a planner; no role is checked.
    Row {
        act: Act::CloseMission,
        command: "close",
        role: Role::Planner,
        binds: false,
        from: &[Status::Ready],
        to: Some(Status::Done),
    },
    Row {
        act: Act::PlanVerdictChanges,
        command: "verdict changes",
        role: Role::Reviewer,
        binds: false,
        from: MISSION,
        to: None,
    },
    Row {
        act: Act::PlanVerdictApprove,
        command: "verdict approve",
        role: Role::Reviewer,
        binds: false,
        from: MISSION,
        to: None,
    },
    Row {
        act: Act::Lead,
        command: "lead",
        role: Role::Lead,
        binds: true,
        from: &[Status::Ready],
        to: None,
    },
];

pub fn row(act: Act) -> &'static Row {
    TABLE
        .iter()
        .find(|row| row.act == act)
        .expect("every act has a row")
}

/// What an act is checked against: the item's record, found by the
/// cross-branch rule, and what lies outside it.
pub struct Facts<'a> {
    pub item: &'a ItemDir,
    /// Whether a local branch named for the item exists.
    pub branch: bool,
    /// Each item the item is blocked by, with its status; `None` when no
    /// such item is found.
    pub blockers: &'a [(String, Option<Status>)],
    /// The item's mission, if it names one, with the mission's status on
    /// the base; `None` when no such mission is found. See [`mission_of`].
    pub mission: Option<(String, Option<Status>)>,
    pub caller: &'a Thread,
    /// The checkout on the item's branch, which `merge` reads. The CLI
    /// gathers it where `merge` runs and `status` lists its conditions: on
    /// that branch. Without it, the conditions it bears on are not listed.
    pub checkout: Option<&'a Checkout>,
}

/// What `merge` reads from Git besides the item's record.
#[derive(Debug, Clone, Default)]
pub struct Checkout {
    /// Tracked files whose changes are not committed, and files in the
    /// item's directory that are not tracked.
    pub uncommitted: Vec<String>,
    /// On an approved item: what differs between the commit its approval
    /// reviewed and the branch's head.
    pub since_approval: Option<SinceApproval>,
}

#[derive(Debug, Clone)]
pub struct SinceApproval {
    /// Every path that differs.
    pub changed: Vec<String>,
    /// The item's record at the reviewed commit, if it had one.
    pub reviewed: Option<ItemDir>,
    /// The item's record at the head, which is what merges, whatever the
    /// checkout holds.
    pub head: Option<ItemDir>,
}

/// What a mission's act is checked against.
pub struct MissionFacts<'a> {
    pub mission: &'a MissionDir,
    /// The items that name the mission, each with its status as
    /// [`crate::Located::record`] finds it; what `close` waits for.
    pub items: &'a [(String, Status)],
    /// The plan review a verdict is recorded in: the caller's.
    pub review: Option<&'a ReviewFile>,
}

/// The round of a mission's plan reviews, and the caller's file in it. A
/// round is over once every file of it has a verdict, since its reviewers
/// work blind in checkouts of their own; until then it is the latest round,
/// and its file is the caller's of those with no verdict, else the first by
/// name, which the caller takes over. Once it is over, the round is the
/// next, which has no file until `review` creates one.
pub fn plan_round<'a>(mission: &'a MissionDir, caller: &Thread) -> Round<'a> {
    let latest = mission
        .reviews
        .iter()
        .map(|r| r.name.round)
        .max()
        .unwrap_or(0);
    let open: Vec<&ReviewFile> = mission
        .reviews
        .iter()
        .filter(|r| r.name.round == latest && r.review.verdict.is_none())
        .collect();
    let mine = open
        .iter()
        .find(|r| r.review.reviewers.holder() == Some(caller))
        .or(open.first());
    match mine {
        Some(file) => Round {
            number: latest,
            review: Some(file),
        },
        None => Round {
            number: latest + 1,
            review: None,
        },
    }
}

/// The mission `item` names, if any, with its status among `missions`.
pub fn mission_of(item: &ItemDir, missions: &[MissionDir]) -> Option<(String, Option<Status>)> {
    let id = item.item.mission.clone()?;
    let found = missions.iter().find(|dir| dir.mission.id == id);
    let status = found.map(|dir| dir.mission.status.status());
    Some((id, status))
}

/// The review whose verdict approved the item: one of the latest round's
/// with an approve verdict.
pub fn approval(item: &ItemDir) -> Option<&ReviewFile> {
    let latest = item.reviews.iter().map(|r| r.name.round).max()?;
    item.reviews
        .iter()
        .find(|r| r.name.round == latest && r.review.verdict == Some(Verdict::Approve))
}

/// The sections of an item that a review approves along with the code.
const REQUIREMENTS: [&str; 3] = ["Goal", "Done when", "Scope"];

/// The round of review an item is in, or a mission's plan (see [`plan_round`]).
pub struct Round<'a> {
    pub number: u32,
    /// The round's review file: a review of the latest round, the
    /// caller's when it holds one. `None` once a review of the latest round
    /// has a verdict, since that verdict ended the round; `number` is then
    /// the next round, whose file `review` creates.
    pub review: Option<&'a ReviewFile>,
}

impl<'a> Facts<'a> {
    pub fn status(&self) -> Status {
        self.item.item.status
    }

    pub fn holder(&self) -> Option<&Thread> {
        self.item.item.implementers.holder()
    }

    pub fn holds(&self) -> bool {
        self.holder() == Some(self.caller)
    }

    pub fn round(&self) -> Round<'a> {
        let reviews = &self.item.reviews;
        let latest = reviews.iter().map(|r| r.name.round).max().unwrap_or(0);
        let open: Vec<&ReviewFile> = reviews.iter().filter(|r| r.name.round == latest).collect();
        let ended = open.iter().any(|r| r.review.verdict.is_some());
        let Some(first) = open.first().filter(|_| !ended) else {
            return Round {
                number: latest + 1,
                review: None,
            };
        };
        let review = open
            .iter()
            .find(|r| r.review.reviewers.holder() == Some(self.caller))
            .unwrap_or(first);
        Round {
            number: latest,
            review: Some(review),
        }
    }

    /// The round's review file, while the item is in review and the caller
    /// holds that file.
    pub fn reviewing(&self) -> Option<&'a ReviewFile> {
        if self.status() != Status::InReview {
            return None;
        }
        let review = self.round().review?;
        (review.review.reviewers.holder() == Some(self.caller)).then_some(review)
    }

    /// The last review of the item the caller held, if any.
    pub fn reviewed(&self) -> Option<&'a ReviewFile> {
        let item: &'a ItemDir = self.item;
        item.reviews
            .iter()
            .filter(|file| file.review.reviewers.0.contains(self.caller))
            .max_by_key(|file| file.name.round)
    }

    /// The role the caller holds on the item, if any.
    pub fn role(&self) -> Option<Role> {
        if self.holds() {
            Some(Role::Implementer)
        } else if self.reviewing().is_some() {
            Some(Role::Reviewer)
        } else {
            None
        }
    }
}

/// A condition of an act that does not hold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    /// A blocker is not done.
    Blocked {
        id: String,
        status: Option<Status>,
    },
    /// The item's mission is not ready; `None` when no such mission is
    /// found.
    MissionNotReady {
        id: String,
        status: Option<Status>,
    },
    /// A branch named for the item exists, so `start` cannot create it.
    BranchExists,
    /// No branch named for the item exists to take over.
    NoBranch,
    /// A Done when line is not ticked.
    Unticked(String),
    NoReport,
    /// An open fix-now finding has no line in Responses and no question
    /// waiting on it. `rebutted` when a rebuttal of it was kept, so it has
    /// had its one try.
    NoResponse {
        finding: FindingName,
        title: String,
        rebutted: bool,
    },
    /// A Decisions entry has no answer.
    Unanswered(String),
    /// The caller has been the item's implementer, so cannot review it or
    /// decide its question.
    FormerImplementer,
    /// The caller reviewed the finding the question is about, here or in
    /// a round it was carried from, so cannot decide it.
    ReviewedFinding(FindingName),
    /// `decide` needs an unanswered question.
    NoQuestion,
    /// A review file does not carry this finding, which a decision settled,
    /// with the `Decided:` line the tool wrote.
    DecisionDropped {
        finding: FindingName,
        outcome: Outcome,
    },
    /// This finding of a review file has a `Decided:` line no decision
    /// wrote, or carries a finding a decision settled other than as the
    /// tool wrote it.
    DecisionChanged(FindingName),
    /// `verdict changes` needs an open fix-now finding in the caller's file.
    NoOpenFinding,
    /// `verdict approve` needs no fix-now finding of the item open.
    OpenFinding {
        finding: FindingName,
        title: String,
    },
    /// No review of the latest round approves the item.
    NoApproval,
    /// The approval no longer covers the head: since the commit it
    /// reviewed, these paths outside the item's directory, or these of its
    /// requirement sections, changed.
    Stale {
        round: u32,
        commit: String,
        changed: Vec<String>,
    },
    /// The checkout holds changes no commit has: to tracked files, or new
    /// files in the item's directory.
    Uncommitted(Vec<String>),
    /// `merge --trivial` needs an item nobody reviewed.
    Reviewed,
    /// `close` on a mission needs each of its items done or dropped.
    ItemOpen {
        id: String,
        status: Status,
    },
    /// `close` on a mission needs its Result written.
    NoResult,
}

#[derive(Debug)]
pub enum Refusal {
    /// The caller holds a role on the item, and the act is another role's.
    Role(Role),
    /// The act does not run from the item's status.
    From {
        act: Act,
        status: Status,
    },
    /// The act needs the caller to hold its role on the item, the item
    /// itself or this round's review, and it does not.
    NotHolder {
        role: Role,
        holder: Option<Thread>,
    },
    Unmet(Vec<Condition>),
    Record(Problem),
}

/// The status `act` sets on the item, `None` for unchanged, or why it is
/// refused.
pub fn check(act: Act, facts: &Facts) -> Result<Option<Status>, Refusal> {
    let row = row(act);
    // A thread that holds one role and runs another's command is told its
    // role, whatever the status. `review` and `decide` by an implementer
    // are refused by their own conditions instead, which name every thread
    // that may not run them, and a planner's act on an item nobody holds
    // yet needs no role on it.
    if let Some(role) = facts.role().filter(|role| *role != row.role) {
        if !matches!(act, Act::Review | Act::Decide) && row.role != Role::Planner {
            return Err(Refusal::Role(role));
        }
    }
    if !row.from.contains(&facts.status()) {
        return Err(Refusal::From {
            act,
            status: facts.status(),
        });
    }
    if !row.binds {
        let (holds, holder) = match row.role {
            // A mission's role and a decider's bind, so their acts never
            // come here.
            Role::Planner | Role::Lead | Role::Decider => (true, None),
            Role::Implementer => (facts.holds(), facts.holder()),
            Role::Reviewer => (
                facts.reviewing().is_some(),
                facts
                    .round()
                    .review
                    .and_then(|r| r.review.reviewers.holder()),
            ),
        };
        if !holds {
            return Err(Refusal::NotHolder {
                role: row.role,
                holder: holder.cloned(),
            });
        }
    }
    let unmet = unmet(act, facts).map_err(Refusal::Record)?;
    if !unmet.is_empty() {
        return Err(Refusal::Unmet(unmet));
    }
    Ok(row.to)
}

/// The status `act`, a mission's, sets on the mission, `None` for
/// unchanged, or why it is refused.
pub fn check_mission(act: Act, facts: &MissionFacts) -> Result<Option<MissionStatus>, Refusal> {
    let row = row(act);
    let status = facts.mission.mission.status.status();
    if !row.from.contains(&status) {
        return Err(Refusal::From { act, status });
    }
    let unmet = unmet_mission(act, facts).map_err(Refusal::Record)?;
    if !unmet.is_empty() {
        return Err(Refusal::Unmet(unmet));
    }
    Ok(row.to.and_then(MissionStatus::of))
}

/// The conditions of `act`, a mission's, that do not hold.
pub fn unmet_mission(act: Act, facts: &MissionFacts) -> Result<Vec<Condition>, Problem> {
    let mission = facts.mission;
    let mut unmet = Vec::new();
    match act {
        Act::CloseMission => {
            for (id, status) in facts.items {
                if !matches!(status, Status::Done | Status::Dropped) {
                    unmet.push(Condition::ItemOpen {
                        id: id.clone(),
                        status: *status,
                    });
                }
            }
            if !mission.result_written() {
                unmet.push(Condition::NoResult);
            }
        }
        Act::PlanVerdictChanges | Act::PlanVerdictApprove => {
            let Some(file) = facts.review else {
                return Ok(unmet);
            };
            let problems = validate(&[], std::slice::from_ref(mission));
            if let Some(problem) = problems.into_iter().find(|p| p.file == file.path) {
                return Err(problem);
            }
            unmet = verdict(act, file, &mission.findings()?, &[]);
        }
        _ => {}
    }
    Ok(unmet)
}

/// What a verdict in `file` needs, given every finding and decision of its
/// item or mission: `changes` an open fix-now finding of the file's own,
/// and `approve` none open among the file's and those of earlier rounds,
/// so also none the file should carry and does not. Another file of the
/// same round is another reviewer's, working blind, so what it carries
/// does not close a finding for this one.
fn verdict(
    act: Act,
    file: &ReviewFile,
    findings: &[Finding],
    decisions: &[Decision],
) -> Vec<Condition> {
    let seen: Vec<Finding> = findings
        .iter()
        .filter(|f| f.name.review == file.name || f.name.review.round < file.name.round)
        .cloned()
        .collect();
    let mut unmet = decided(file, &seen, decisions);
    let mut open = seen.iter().filter(|f| is_open(f, &seen, decisions));
    if matches!(act, Act::VerdictChanges | Act::PlanVerdictChanges) {
        if !open.any(|f| f.name.review == file.name) {
            unmet.push(Condition::NoOpenFinding);
        }
    } else {
        unmet.extend(open.map(|f| Condition::OpenFinding {
            finding: f.name.clone(),
            title: f.title.clone(),
        }));
    }
    unmet
}

/// What a decision settled stays settled: `file` carries each finding of
/// an earlier round that a decision settled and no earlier round carries,
/// with the `Decided:` line the tool wrote, and nothing else in it has a
/// `Decided:` line or carries a settled finding. `seen` is the file's
/// findings and the earlier rounds'.
fn decided(file: &ReviewFile, seen: &[Finding], decisions: &[Decision]) -> Vec<Condition> {
    let settled = |name: &FindingName| {
        let decision = decisions.iter().find(|d| d.finding.as_ref() == Some(name));
        decision.and_then(|d| d.outcome)
    };
    let (own, earlier): (Vec<&Finding>, Vec<&Finding>) =
        seen.iter().partition(|f| f.name.review == file.name);
    let carried_before = |name: &FindingName| earlier.iter().any(|f| f.was.as_ref() == Some(name));
    let mut unmet = Vec::new();
    for finding in &earlier {
        let Some(outcome) = settled(&finding.name).filter(|_| !carried_before(&finding.name))
        else {
            continue;
        };
        let carried = own
            .iter()
            .any(|f| f.was.as_ref() == Some(&finding.name) && f.decided == Some(outcome));
        if !carried {
            unmet.push(Condition::DecisionDropped {
                finding: finding.name.clone(),
                outcome,
            });
        }
    }
    for finding in own {
        let as_written = match &finding.was {
            Some(was) if settled(was).is_some() => {
                finding.decided == settled(was) && !carried_before(was)
            }
            // A finding carried with a `Decided:` line is settled too.
            Some(was)
                if earlier
                    .iter()
                    .any(|f| f.name == *was && f.decided.is_some()) =>
            {
                false
            }
            _ => finding.decided.is_none(),
        };
        if !as_written {
            unmet.push(Condition::DecisionChanged(finding.name.clone()));
        }
    }
    unmet
}

/// The item's oldest unanswered question, which `decide` answers, with
/// its place among the Decisions entries.
pub fn question(item: &ItemDir) -> Result<Option<(usize, Decision)>, Problem> {
    let mut decisions = item.decisions()?.into_iter().enumerate();
    Ok(decisions.find(|(_, d)| d.answer.is_none()))
}

/// The conditions of `act` that do not hold.
pub fn unmet(act: Act, facts: &Facts) -> Result<Vec<Condition>, Problem> {
    let item = facts.item;
    let mut unmet = Vec::new();
    match act {
        Act::Ready | Act::ReadyMission | Act::StartNew | Act::Drop | Act::Lead | Act::Escalate => {}
        // A mission's act is checked with `unmet_mission`.
        Act::CloseMission | Act::PlanVerdictChanges | Act::PlanVerdictApprove => {}
        Act::Start => {
            for (id, status) in facts.blockers {
                if *status != Some(Status::Done) {
                    unmet.push(Condition::Blocked {
                        id: id.clone(),
                        status: *status,
                    });
                }
            }
            if let Some((id, status)) = &facts.mission {
                if *status != Some(Status::Ready) {
                    unmet.push(Condition::MissionNotReady {
                        id: id.clone(),
                        status: *status,
                    });
                }
            }
            if facts.branch {
                unmet.push(Condition::BranchExists);
            }
        }
        Act::TakeOver => {
            if !facts.branch {
                unmet.push(Condition::NoBranch);
            }
            // A dropped mission's items start no more, taken over or not.
            if let Some((id, Some(Status::Dropped))) = &facts.mission {
                unmet.push(Condition::MissionNotReady {
                    id: id.clone(),
                    status: Some(Status::Dropped),
                });
            }
        }
        Act::Submit => {
            for (ticked, text) in item.done_when() {
                if !ticked {
                    unmet.push(Condition::Unticked(text));
                }
            }
            if !item.report_written() {
                unmet.push(Condition::NoReport);
            }
            let responses = item.responses()?;
            let decisions = item.decisions()?;
            let findings = item.findings()?;
            let rebutted = |name: &FindingName| {
                let rebuttal = |r: &Response| matches!(r, Response::Rebutted { finding, .. } if finding == name);
                responses.iter().any(rebuttal)
            };
            for finding in item.open_findings()? {
                // A question about it waits, and is listed below.
                let answered = responses.iter().any(|r| *r.finding() == finding.name)
                    || decisions
                        .iter()
                        .any(|d| d.finding.as_ref() == Some(&finding.name) && d.answer.is_none());
                if !answered {
                    let carried = chain(&finding.name, &findings).into_iter().skip(1);
                    unmet.push(Condition::NoResponse {
                        rebutted: carried.into_iter().any(|f| rebutted(&f.name)),
                        finding: finding.name,
                        title: finding.title,
                    });
                }
            }
            for decision in decisions.into_iter().filter(|d| d.answer.is_none()) {
                unmet.push(Condition::Unanswered(decision.question));
            }
        }
        Act::Review => {
            if item.item.implementers.0.contains(facts.caller) {
                unmet.push(Condition::FormerImplementer);
            }
        }
        Act::Decide => {
            if item.item.implementers.0.contains(facts.caller) {
                unmet.push(Condition::FormerImplementer);
            }
            let Some((_, question)) = question(item)? else {
                unmet.push(Condition::NoQuestion);
                return Ok(unmet);
            };
            // Every reviewer of the finding, and of each it carries.
            let findings = item.findings()?;
            let about = question.finding.as_ref();
            let reviewed = about.is_some_and(|name| {
                chain(name, &findings).iter().any(|f| {
                    let review = item.reviews.iter().find(|r| r.name == f.name.review);
                    review.is_some_and(|r| r.review.reviewers.0.contains(facts.caller))
                })
            });
            if let Some(name) = about.filter(|_| reviewed) {
                unmet.push(Condition::ReviewedFinding(name.clone()));
            }
        }
        Act::VerdictChanges | Act::VerdictApprove => {
            let Some(file) = facts.reviewing() else {
                return Ok(unmet);
            };
            let problems = validate(std::slice::from_ref(item), &[]);
            if let Some(problem) = problems.into_iter().find(|p| p.file == file.path) {
                return Err(problem);
            }
            unmet = verdict(act, file, &item.findings()?, &item.decisions()?);
        }
        Act::Close => {
            if facts.branch {
                unmet.push(Condition::BranchExists);
            }
        }
        Act::Merge | Act::MergeTrivial => {
            if act == Act::MergeTrivial {
                for (ticked, text) in item.done_when() {
                    if !ticked {
                        unmet.push(Condition::Unticked(text));
                    }
                }
                if !item.report_written() {
                    unmet.push(Condition::NoReport);
                }
                if !item.reviews.is_empty() {
                    unmet.push(Condition::Reviewed);
                }
            }
            let checkout = facts.checkout;
            if act == Act::Merge {
                match approval(item) {
                    None => unmet.push(Condition::NoApproval),
                    Some(review) => {
                        if let Some(since) = checkout.and_then(|c| c.since_approval.as_ref()) {
                            unmet.extend(stale(item, review, since));
                        }
                    }
                }
            }
            for decision in item.decisions()?.into_iter() {
                if decision.answer.is_none() {
                    unmet.push(Condition::Unanswered(decision.question));
                }
            }
            if let Some(checkout) = checkout.filter(|c| !c.uncommitted.is_empty()) {
                unmet.push(Condition::Uncommitted(checkout.uncommitted.clone()));
            }
        }
    }
    Ok(unmet)
}

/// Whether the approval still covers the head: since the commit it
/// reviewed, nothing changed outside the item's own directory, and the
/// item's requirements at the head are as it read them. A Report, a
/// response or a review file never makes it stale.
fn stale(item: &ItemDir, review: &ReviewFile, since: &SinceApproval) -> Option<Condition> {
    let own = format!(".atelier/items/{}/", item.item.id);
    let mut changed: Vec<String> = since
        .changed
        .iter()
        .filter(|path| !path.starts_with(&own))
        .cloned()
        .collect();
    for heading in REQUIREMENTS {
        let before = since.reviewed.as_ref().and_then(|r| r.section(heading));
        let now = since.head.as_ref().and_then(|r| r.section(heading));
        if before != now {
            changed.push(format!("this item's {heading}"));
        }
    }
    (!changed.is_empty()).then(|| Condition::Stale {
        round: review.name.round,
        commit: review.review.commit.clone(),
        changed,
    })
}

/// The act the caller would run next on the item, if any: start a ready
/// item; as its holder, submit it or merge it; as this round's reviewer,
/// `verdict changes` while its file holds an open fix-now finding, else
/// `verdict approve`; as anyone else, review it when it is in review and
/// they never implemented it, else take it over.
pub fn next(facts: &Facts) -> Result<Option<Act>, Problem> {
    let status = facts.status();
    if status == Status::Ready {
        return Ok(Some(Act::Start));
    }
    if !STARTED.contains(&status) {
        return Ok(None);
    }
    if let Some(file) = facts.reviewing() {
        let findings = facts.item.findings()?;
        let decisions = facts.item.decisions()?;
        let open = findings
            .iter()
            .any(|f| f.name.review == file.name && is_open(f, &findings, &decisions));
        return Ok(Some(if open {
            Act::VerdictChanges
        } else {
            Act::VerdictApprove
        }));
    }
    if !facts.holds() {
        let implemented = facts.item.item.implementers.0.contains(facts.caller);
        if status == Status::InReview && !implemented {
            return Ok(Some(Act::Review));
        }
        // A reviewer whose review is done is not offered the item itself.
        return Ok(facts.reviewed().is_none().then_some(Act::TakeOver));
    }
    Ok(match status {
        Status::Approved => Some(Act::Merge),
        Status::InReview => None,
        _ => Some(Act::Submit),
    })
}
