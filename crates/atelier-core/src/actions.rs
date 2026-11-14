//! The action table: each lifecycle action, the role that runs it, the statuses
//! it runs from, the status it sets, and its conditions. The commands check
//! an action with [`check`], and `status` lists what [`unmet`] gives for the
//! action it names next, so the two cannot disagree.

use crate::model::{
    is_deferred, is_open, Decision, Finding, FindingName, MissionStatus, Response, Settlement,
    Status, Thread, Verdict,
};
use crate::records::{validate, ItemDir, MissionDir, Problem, ReviewFile};
use crate::review::chain;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// `ready` on an item. A mission has no go-ahead, and `ready` refuses
    /// one.
    Ready,
    Start,
    /// `start --new`, which creates the item it starts.
    StartNew,
    /// `start --mission`, which creates a mission on its own branch.
    StartMission,
    /// `start --backlog`, which makes a backlog branch for record changes
    /// tied to no item or mission.
    StartBacklog,
    /// `submit` on a backlog branch.
    SubmitBacklog,
    /// `merge` of a backlog branch, which holds records alone.
    MergeBacklog,
    /// `submit` on a mission's branch, by a planner of it or a thread that
    /// takes the plan over.
    SubmitMission,
    /// `merge` of a mission's branch, by a planner of it.
    MergeMission,
    TakeOver,
    Submit,
    Review,
    VerdictChanges,
    VerdictApprove,
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
    /// `new --from` or `drop` on a deferred finding of a done item.
    Settle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Planner,
    Implementer,
    Reviewer,
    Lead,
}

pub struct Row {
    pub action: Action,
    /// The command, after `atelier`.
    pub command: &'static str,
    pub role: Role,
    /// Whether the action binds the caller to its role. An action that does not
    /// needs the caller to hold the role already, unless it is a planner's,
    /// which no thread holds.
    pub binds: bool,
    /// The statuses it runs from; none for an action that creates its record.
    pub from: &'static [Status],
    /// The status the action sets; `None` leaves it unchanged.
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

/// Every status a mission can have. A mission's `open` is `ready` here,
/// and the old `draft` is `draft` ([`MissionStatus::status`]).
const MISSION: &[Status] = &[Status::Draft, Status::Ready, Status::Done, Status::Dropped];

/// A mission that is proposed, read from its branch, which has not merged:
/// `open`, or the old `draft`.
pub const PROPOSED: &[Status] = &[Status::Draft, Status::Ready];

pub const TABLE: [Row; 23] = [
    Row {
        action: Action::Ready,
        command: "ready",
        role: Role::Planner,
        binds: false,
        from: &[Status::Draft],
        to: Some(Status::Ready),
    },
    Row {
        action: Action::Start,
        command: "start",
        role: Role::Implementer,
        binds: true,
        from: &[Status::Ready],
        to: Some(Status::Building),
    },
    Row {
        action: Action::StartNew,
        command: "start --new",
        role: Role::Implementer,
        binds: true,
        from: &[],
        to: Some(Status::Building),
    },
    // Anyone may plan a mission, and becomes its first planner. It is
    // `open` from the start: proposed while its branch has not merged.
    Row {
        action: Action::StartMission,
        command: "start --mission",
        role: Role::Planner,
        binds: true,
        from: &[],
        to: Some(Status::Ready),
    },
    // The actions of a backlog branch: any thread, in a checkout on that
    // branch, with no record to take a status from. None is an
    // implementer's action on an item.
    Row {
        action: Action::StartBacklog,
        command: "start --backlog",
        role: Role::Planner,
        binds: false,
        from: &[],
        to: None,
    },
    Row {
        action: Action::SubmitBacklog,
        command: "submit",
        role: Role::Planner,
        binds: false,
        from: &[],
        to: None,
    },
    Row {
        action: Action::MergeBacklog,
        command: "merge",
        role: Role::Planner,
        binds: false,
        from: &[],
        to: None,
    },
    // The actions of a mission's branch, from proposed, in a checkout on that
    // branch: `submit` binds the caller as a planner, taking the plan over
    // when it was not one, and writes an old `draft` as `open`; `merge`
    // needs a planner, and the plan `open`, as `submit` leaves it. A plan
    // dropped on its branch does neither.
    Row {
        action: Action::SubmitMission,
        command: "submit",
        role: Role::Planner,
        binds: true,
        from: PROPOSED,
        to: Some(Status::Ready),
    },
    Row {
        action: Action::MergeMission,
        command: "merge",
        role: Role::Planner,
        binds: false,
        from: &[Status::Ready],
        to: None,
    },
    Row {
        action: Action::TakeOver,
        command: "start",
        role: Role::Implementer,
        binds: true,
        from: STARTED,
        to: None,
    },
    Row {
        action: Action::Submit,
        command: "submit",
        role: Role::Implementer,
        binds: false,
        from: &[Status::Building, Status::ChangesRequested, Status::Approved],
        to: Some(Status::InReview),
    },
    Row {
        action: Action::Review,
        command: "review",
        role: Role::Reviewer,
        binds: true,
        from: &[Status::InReview],
        to: None,
    },
    Row {
        action: Action::VerdictChanges,
        command: "verdict changes",
        role: Role::Reviewer,
        binds: false,
        from: &[Status::InReview],
        to: Some(Status::ChangesRequested),
    },
    Row {
        action: Action::VerdictApprove,
        command: "verdict approve",
        role: Role::Reviewer,
        binds: false,
        from: &[Status::InReview],
        to: Some(Status::Approved),
    },
    Row {
        action: Action::Merge,
        command: "merge",
        role: Role::Implementer,
        binds: false,
        from: &[Status::Approved],
        to: Some(Status::Done),
    },
    Row {
        action: Action::MergeTrivial,
        command: "merge --trivial",
        role: Role::Implementer,
        binds: false,
        from: &[Status::Building],
        to: Some(Status::Done),
    },
    Row {
        action: Action::Drop,
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
    // Anyone may close an item; like a planner's action, it needs no role.
    Row {
        action: Action::Close,
        command: "close",
        role: Role::Planner,
        binds: false,
        from: PLANNED,
        to: Some(Status::Done),
    },
    // Its lead or a planner; no role is checked. It runs on an agreed
    // mission: open, read from the base.
    Row {
        action: Action::CloseMission,
        command: "close",
        role: Role::Planner,
        binds: false,
        from: &[Status::Ready],
        to: Some(Status::Done),
    },
    Row {
        action: Action::PlanVerdictChanges,
        command: "verdict changes",
        role: Role::Reviewer,
        binds: false,
        from: MISSION,
        to: None,
    },
    Row {
        action: Action::PlanVerdictApprove,
        command: "verdict approve",
        role: Role::Reviewer,
        binds: false,
        from: MISSION,
        to: None,
    },
    // A planner on the base, or the implementer of the item whose branch
    // it runs on; the CLI checks where it runs. It changes the finding, not
    // the status of its item, and checks the finding with `check_settle`.
    Row {
        action: Action::Settle,
        command: "new --from, drop",
        role: Role::Planner,
        binds: false,
        from: &[Status::Done],
        to: None,
    },
    Row {
        action: Action::Lead,
        command: "lead",
        role: Role::Lead,
        binds: true,
        from: &[Status::Ready],
        to: None,
    },
];

/// The actions on a mission. `drop` is also an item's; every other action is
/// only an item's.
const ON_MISSION: [Action; 8] = [
    Action::StartMission,
    Action::SubmitMission,
    Action::MergeMission,
    Action::CloseMission,
    Action::Drop,
    Action::PlanVerdictChanges,
    Action::PlanVerdictApprove,
    Action::Lead,
];

/// Whether an action on an item, or with `mission` on a mission, changes a
/// status `from` to `to`: what the pre-commit hook asks of a status
/// changed in a commit.
pub fn changes(mission: bool, from: Status, to: Status) -> bool {
    (TABLE.iter())
        .filter(|row| row.action == Action::Drop || ON_MISSION.contains(&row.action) == mission)
        .any(|row| row.from.contains(&from) && row.to == Some(to))
}

pub fn row(action: Action) -> &'static Row {
    TABLE
        .iter()
        .find(|row| row.action == action)
        .expect("every action has a row")
}

/// What an action is checked against: the item's record, found by the
/// cross-branch rule, and what lies outside it.
pub struct Facts<'a> {
    pub item: &'a ItemDir,
    /// Whether a branch named for the item exists, local or the forge's
    /// remote-tracking one.
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

/// What a mission's action is checked against.
pub struct MissionFacts<'a> {
    pub mission: &'a MissionDir,
    /// Who acts: a planner, for the actions of a mission's branch.
    pub caller: &'a Thread,
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

/// A condition of an action that does not hold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    /// A blocker is not done.
    Blocked {
        id: String,
        status: Option<Status>,
    },
    /// The item's mission is not open on the base: dropped, so that none of
    /// its items starts or merges, or the old `draft`, which is proposed;
    /// `None` when the base has no such mission.
    MissionNotOpen {
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
    /// An open fix-now finding has no line in Responses. `rebutted` when a
    /// rebuttal of it was kept, so it has had its one try.
    NoResponse {
        finding: FindingName,
        title: String,
        rebutted: bool,
    },
    /// The caller has been the item's implementer, so cannot review it.
    FormerImplementer,
    /// This finding of a review file has a `Decided:` line, which nothing
    /// writes since `decide` was removed, or carries a finding that a
    /// decision settled before then.
    Decided(FindingName),
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
    /// Settling a finding needs the item's reviews to have it.
    NoFinding(FindingName),
    /// Settling a finding needs it deferred.
    NotDeferred(FindingName),
    /// Settling a finding needs it not settled already.
    Settled {
        finding: FindingName,
        settlement: Settlement,
    },
    /// A verdict needs its review file valid; this is what is not.
    Invalid(Problem),
}

impl Condition {
    /// Whether `action` refused for this condition breaks a rule, which
    /// `ATELIER_SKIP` passes, rather than having nothing to act on: a branch
    /// `start` would create, or one a take-over would switch to; a question
    /// to decide; a finding to settle, or one settled already.
    pub fn is_rule(&self, action: Action) -> bool {
        !matches!(
            (action, self),
            (Action::Start, Condition::BranchExists)
                | (
                    _,
                    Condition::NoBranch | Condition::NoFinding(_) | Condition::Settled { .. }
                )
        )
    }
}

#[derive(Debug)]
pub enum Refusal {
    /// The caller holds a role on the item, and the action is another role's.
    Role(Role),
    /// The action does not run from the item's status.
    From {
        action: Action,
        status: Status,
    },
    /// The action needs the caller to hold its role on the item, the item
    /// itself or this round's review, and it does not.
    NotHolder {
        role: Role,
        holder: Option<Thread>,
    },
    Unmet(Vec<Condition>),
    Record(Problem),
}

impl Refusal {
    /// Whether this refusal of `action` breaks only rules, which `ATELIER_SKIP`
    /// passes. A record that does not read is none, nor is a verdict by a
    /// caller with no review file of the round to record it in.
    pub fn is_rule(&self, action: Action) -> bool {
        match self {
            Refusal::Record(_) => false,
            Refusal::NotHolder {
                role: Role::Reviewer,
                ..
            } => false,
            Refusal::Unmet(unmet) => unmet.iter().all(|c| c.is_rule(action)),
            Refusal::Role(_) | Refusal::From { .. } | Refusal::NotHolder { .. } => true,
        }
    }
}

/// The status `action` sets on the item, `None` for unchanged, or the first
/// reason it is refused.
pub fn check(action: Action, facts: &Facts) -> Result<Option<Status>, Refusal> {
    match found(action, facts, false) {
        Ok(found) => found.into_iter().next().map_or(Ok(row(action).to), Err),
        Err(problem) => Err(Refusal::Record(problem)),
    }
}

/// Every reason `action` is refused, in the order [`check`] gives the first:
/// what `ATELIER_SKIP` would have to pass.
pub fn refusals(action: Action, facts: &Facts) -> Result<Vec<Refusal>, Problem> {
    found(action, facts, true)
}

fn found(action: Action, facts: &Facts, all: bool) -> Result<Vec<Refusal>, Problem> {
    let row = row(action);
    let mut found = Vec::new();
    // A thread that holds one role and runs another's command is told its
    // role, whatever the status. `review` by an implementer is refused by
    // its own condition instead, which names every thread that may not run
    // it, and a planner's action on an item nobody holds
    // yet needs no role on it.
    if let Some(role) = facts.role().filter(|role| *role != row.role) {
        if !matches!(action, Action::Review) && row.role != Role::Planner {
            found.push(Refusal::Role(role));
        }
    }
    if !row.from.contains(&facts.status()) {
        found.push(Refusal::From {
            action,
            status: facts.status(),
        });
    }
    if !row.binds {
        let (holds, holder) = match row.role {
            // A mission's roles bind, so their actions never come here.
            Role::Planner | Role::Lead => (true, None),
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
            found.push(Refusal::NotHolder {
                role: row.role,
                holder: holder.cloned(),
            });
        }
    }
    // The conditions read the item's findings, which may not parse: a
    // refusal found before them is said all the same.
    if all || found.is_empty() {
        let unmet = unmet(action, facts)?;
        if !unmet.is_empty() {
            found.push(Refusal::Unmet(unmet));
        }
    }
    Ok(found)
}

/// The status `action`, a mission's, sets on the mission, `None` for
/// unchanged, or the first reason it is refused.
pub fn check_mission(
    action: Action,
    facts: &MissionFacts,
) -> Result<Option<MissionStatus>, Refusal> {
    match mission_refusals(action, facts) {
        Ok(found) => {
            (found.into_iter().next()).map_or(Ok(row(action).to.and_then(MissionStatus::of)), Err)
        }
        Err(problem) => Err(Refusal::Record(problem)),
    }
}

/// Every reason `action`, a mission's, is refused, as [`refusals`] gives an
/// item's.
pub fn mission_refusals(action: Action, facts: &MissionFacts) -> Result<Vec<Refusal>, Problem> {
    let row = row(action);
    let status = facts.mission.mission.status.status();
    let mut found = Vec::new();
    if !row.from.contains(&status) {
        found.push(Refusal::From { action, status });
    }
    // Merging a plan needs one of its planners; `submit` makes the caller
    // one, so a thread that is not takes the plan over there.
    let planners = &facts.mission.mission.planners;
    if action == Action::MergeMission && !planners.0.contains(facts.caller) {
        found.push(Refusal::NotHolder {
            role: Role::Planner,
            holder: planners.holder().cloned(),
        });
    }
    let unmet = unmet_mission(action, facts)?;
    if !unmet.is_empty() {
        found.push(Refusal::Unmet(unmet));
    }
    Ok(found)
}

/// The conditions of `action`, a mission's, that do not hold.
pub fn unmet_mission(action: Action, facts: &MissionFacts) -> Result<Vec<Condition>, Problem> {
    let mission = facts.mission;
    let mut unmet = Vec::new();
    match action {
        Action::CloseMission => {
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
        Action::PlanVerdictChanges | Action::PlanVerdictApprove => {
            let Some(file) = facts.review else {
                return Ok(unmet);
            };
            let findings = mission.findings()?;
            unmet = invalid(validate(&[], std::slice::from_ref(mission)), file);
            unmet.extend(verdict(action, file, &findings, &[]));
        }
        _ => {}
    }
    Ok(unmet)
}

/// The validation problems of the review file `file` among `problems`.
/// Its findings read, or no verdict gets this far.
fn invalid(problems: Vec<Problem>, file: &ReviewFile) -> Vec<Condition> {
    let own = problems.into_iter().filter(|p| p.file == file.path);
    own.map(Condition::Invalid).collect()
}

/// What a verdict in `file` needs, given every finding and decision of its
/// item or mission: `changes` an open fix-now finding of the file's own,
/// and `approve` none open among the file's and those of earlier rounds,
/// so also none the file should carry and does not. Another file of the
/// same round is another reviewer's, working blind, so what it carries
/// does not close a finding for this one.
fn verdict(
    action: Action,
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
    if matches!(action, Action::VerdictChanges | Action::PlanVerdictChanges) {
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

/// What a decision settled before `decide` was removed stays settled, and
/// nothing settles a finding that way now: no finding of `file` has a
/// `Decided:` line, nor carries a finding that a Decisions entry or a
/// `Decided:` line settled. `seen` is the file's findings and the earlier
/// rounds'. A fix-now finding is settled without a fix only by its
/// reviewer: withdrawn, or kept at a lower recommendation.
fn decided(file: &ReviewFile, seen: &[Finding], decisions: &[Decision]) -> Vec<Condition> {
    let settled = |name: &FindingName| {
        let decision = decisions
            .iter()
            .any(|d| d.finding.as_ref() == Some(name) && d.outcome.is_some());
        decision || seen.iter().any(|f| f.name == *name && f.decided.is_some())
    };
    seen.iter()
        .filter(|f| f.name.review == file.name)
        .filter(|f| f.decided.is_some() || f.was.as_ref().is_some_and(&settled))
        .map(|f| Condition::Decided(f.name.clone()))
        .collect()
}

/// The conditions of `action` that do not hold.
pub fn unmet(action: Action, facts: &Facts) -> Result<Vec<Condition>, Problem> {
    let item = facts.item;
    let mut unmet = Vec::new();
    match action {
        Action::Ready | Action::StartNew | Action::Drop | Action::Lead => {}
        // On no item: a backlog branch's actions, and a mission's.
        Action::StartMission
        | Action::StartBacklog
        | Action::SubmitBacklog
        | Action::MergeBacklog => {}
        Action::SubmitMission | Action::MergeMission => {}
        // The finding's conditions are checked by `check_settle`.
        Action::Settle => {}
        // A mission's action is checked with `unmet_mission`.
        Action::CloseMission | Action::PlanVerdictChanges | Action::PlanVerdictApprove => {}
        Action::Start => {
            for (id, status) in facts.blockers {
                if *status != Some(Status::Done) {
                    unmet.push(Condition::Blocked {
                        id: id.clone(),
                        status: *status,
                    });
                }
            }
            // Of its mission it asks only that it is agreed and not
            // dropped: on the base, and not the old `draft`.
            if let Some((id, status)) = &facts.mission {
                if matches!(status, None | Some(Status::Draft | Status::Dropped)) {
                    unmet.push(Condition::MissionNotOpen {
                        id: id.clone(),
                        status: *status,
                    });
                }
            }
            if facts.branch {
                unmet.push(Condition::BranchExists);
            }
        }
        Action::TakeOver => {
            if !facts.branch {
                unmet.push(Condition::NoBranch);
            }
            // A dropped mission's items start no more, taken over or not.
            unmet.extend(mission_dropped(facts));
        }
        Action::Submit => {
            for (ticked, text) in item.done_when() {
                if !ticked {
                    unmet.push(Condition::Unticked(text));
                }
            }
            if !item.report_written() {
                unmet.push(Condition::NoReport);
            }
            let responses = item.responses()?;
            let findings = item.findings()?;
            let rebutted = |name: &FindingName| {
                let rebuttal = |r: &Response| matches!(r, Response::Rebutted { finding, .. } if finding == name);
                responses.iter().any(rebuttal)
            };
            for finding in item.open_findings()? {
                if !responses.iter().any(|r| *r.finding() == finding.name) {
                    let carried = chain(&finding.name, &findings).into_iter().skip(1);
                    unmet.push(Condition::NoResponse {
                        rebutted: carried.into_iter().any(|f| rebutted(&f.name)),
                        finding: finding.name,
                        title: finding.title,
                    });
                }
            }
        }
        Action::Review => {
            if item.item.implementers.0.contains(facts.caller) {
                unmet.push(Condition::FormerImplementer);
            }
        }
        Action::VerdictChanges | Action::VerdictApprove => {
            let Some(file) = facts.reviewing() else {
                return Ok(unmet);
            };
            let (findings, decisions) = (item.findings()?, item.decisions()?);
            unmet = invalid(validate(std::slice::from_ref(item), &[]), file);
            unmet.extend(verdict(action, file, &findings, &decisions));
        }
        Action::Close => {
            if facts.branch {
                unmet.push(Condition::BranchExists);
            }
        }
        Action::Merge | Action::MergeTrivial => {
            if action == Action::MergeTrivial {
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
            // A stop of its mission stops its merge too.
            unmet.extend(mission_dropped(facts));
            let checkout = facts.checkout;
            if action == Action::Merge {
                match approval(item) {
                    None => unmet.push(Condition::NoApproval),
                    Some(review) => {
                        if let Some(since) = checkout.and_then(|c| c.since_approval.as_ref()) {
                            unmet.extend(stale(item, review, since));
                        }
                    }
                }
            }
            if let Some(checkout) = checkout.filter(|c| !c.uncommitted.is_empty()) {
                unmet.push(Condition::Uncommitted(checkout.uncommitted.clone()));
            }
        }
    }
    Ok(unmet)
}

/// The stop of the item's mission, when it is dropped.
fn mission_dropped(facts: &Facts) -> Option<Condition> {
    match &facts.mission {
        Some((id, Some(Status::Dropped))) => Some(Condition::MissionNotOpen {
            id: id.clone(),
            status: Some(Status::Dropped),
        }),
        _ => None,
    }
}

/// Every reason `new --from` or `drop` on finding `name` of the item is
/// refused: the action's row needs the item done, and the finding must be one
/// of its reviews', deferred and not yet settled.
pub fn settle_refusals(facts: &Facts, name: &FindingName) -> Result<Vec<Refusal>, Problem> {
    let mut found = refusals(Action::Settle, facts)?;
    let findings = facts.item.findings()?;
    let decisions = facts.item.decisions()?;
    // Settled comes first: it is no rule, so a skip never settles twice.
    let unmet = match findings.iter().find(|f| f.name == *name) {
        None => vec![Condition::NoFinding(name.clone())],
        Some(Finding {
            settlement: Some(settlement),
            ..
        }) => vec![Condition::Settled {
            finding: name.clone(),
            settlement: settlement.clone(),
        }],
        Some(finding) if !is_deferred(finding, &findings, &decisions) => {
            vec![Condition::NotDeferred(name.clone())]
        }
        Some(_) => Vec::new(),
    };
    if !unmet.is_empty() {
        found.push(Refusal::Unmet(unmet));
    }
    Ok(found)
}

/// The deferred findings of `item` that nobody has settled, when it is
/// done; what `list --deferred` shows.
pub fn unsettled(item: &ItemDir) -> Result<Vec<Finding>, Problem> {
    if item.item.status != Status::Done {
        return Ok(Vec::new());
    }
    let findings = item.findings()?;
    let decisions = item.decisions()?;
    Ok(findings
        .iter()
        .filter(|f| is_deferred(f, &findings, &decisions) && !f.is_settled())
        .cloned()
        .collect())
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

/// The action the caller would run next on the item, if any: start a ready
/// item; as its holder, submit it or merge it; as this round's reviewer,
/// `verdict changes` while its file holds an open fix-now finding, else
/// `verdict approve`; as anyone else, review it when it is in review and
/// they never implemented it, else take it over.
pub fn next(facts: &Facts) -> Result<Option<Action>, Problem> {
    let status = facts.status();
    if status == Status::Ready {
        return Ok(Some(Action::Start));
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
            Action::VerdictChanges
        } else {
            Action::VerdictApprove
        }));
    }
    if !facts.holds() {
        let implemented = facts.item.item.implementers.0.contains(facts.caller);
        if status == Status::InReview && !implemented {
            return Ok(Some(Action::Review));
        }
        // A reviewer whose review is done is not offered the item itself.
        return Ok(facts.reviewed().is_none().then_some(Action::TakeOver));
    }
    Ok(match status {
        Status::Approved => Some(Action::Merge),
        Status::InReview => None,
        _ => Some(Action::Submit),
    })
}
