//! The act table: each lifecycle act, the role that runs it, the statuses
//! it runs from, the status it sets, and its conditions. The commands check
//! an act with [`check`], and `status` lists what [`unmet`] gives for the
//! act it names next, so the two cannot disagree.

use atelier_core::{FindingName, Status, Thread};
use atelier_records::{ItemDir, Problem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Act {
    Start,
    TakeOver,
    Submit,
    Review,
    VerdictChanges,
    VerdictApprove,
    Merge,
    MergeTrivial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Implementer,
    Reviewer,
}

pub struct Row {
    pub act: Act,
    /// The command, after `atelier`.
    pub command: &'static str,
    pub role: Role,
    /// Whether the act binds the caller to its role. An act that does not
    /// needs the caller to hold the role already.
    pub binds: bool,
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

pub const TABLE: [Row; 8] = [
    Row {
        act: Act::Start,
        command: "start",
        role: Role::Implementer,
        binds: true,
        from: &[Status::Ready],
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
    pub caller: &'a Thread,
}

impl Facts<'_> {
    pub fn status(&self) -> Status {
        self.item.item.status
    }

    pub fn holder(&self) -> Option<&Thread> {
        self.item.item.implementers.holder()
    }

    pub fn holds(&self) -> bool {
        self.holder() == Some(self.caller)
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
    /// A branch named for the item exists, so `start` cannot create it.
    BranchExists,
    /// No branch named for the item exists to take over.
    NoBranch,
    /// A Done when line is not ticked.
    Unticked(String),
    NoReport,
    /// An open fix-now finding has no line in Responses and no decision.
    NoResponse {
        finding: FindingName,
        title: String,
    },
    /// A Decisions entry has no answer.
    Unanswered(String),
    /// The caller has been the item's implementer, so cannot review it.
    FormerImplementer,
}

#[derive(Debug)]
pub enum Refusal {
    /// The act does not run from the item's status.
    From {
        act: Act,
        status: Status,
    },
    /// The act needs the caller to hold the item, and it does not.
    NotHolder {
        holder: Option<Thread>,
    },
    Unmet(Vec<Condition>),
    Record(Problem),
}

/// The status `act` sets on the item, `None` for unchanged, or why it is
/// refused.
pub fn check(act: Act, facts: &Facts) -> Result<Option<Status>, Refusal> {
    let row = row(act);
    if !row.from.contains(&facts.status()) {
        return Err(Refusal::From {
            act,
            status: facts.status(),
        });
    }
    if !row.binds && row.role == Role::Implementer && !facts.holds() {
        return Err(Refusal::NotHolder {
            holder: facts.holder().cloned(),
        });
    }
    let unmet = unmet(act, facts).map_err(Refusal::Record)?;
    if !unmet.is_empty() {
        return Err(Refusal::Unmet(unmet));
    }
    Ok(row.to)
}

/// The conditions of `act` that do not hold. The review, verdict and merge
/// conditions arrive with those commands.
pub fn unmet(act: Act, facts: &Facts) -> Result<Vec<Condition>, Problem> {
    let item = facts.item;
    let mut unmet = Vec::new();
    match act {
        Act::Start => {
            for (id, status) in facts.blockers {
                if *status != Some(Status::Done) {
                    unmet.push(Condition::Blocked {
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
            for finding in item.open_findings()? {
                let answered = responses.iter().any(|r| *r.finding() == finding.name)
                    || decisions
                        .iter()
                        .any(|d| d.finding.as_ref() == Some(&finding.name));
                if !answered {
                    unmet.push(Condition::NoResponse {
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
        Act::VerdictChanges | Act::VerdictApprove | Act::Merge | Act::MergeTrivial => {}
    }
    Ok(unmet)
}

/// The act the caller would run next on the item, if any: start a ready
/// item; as its holder, submit it or merge it; as anyone else, review it
/// when it is in review and they never implemented it, else take it over.
pub fn next(facts: &Facts) -> Option<Act> {
    let status = facts.status();
    if status == Status::Ready {
        return Some(Act::Start);
    }
    if !STARTED.contains(&status) {
        return None;
    }
    if !facts.holds() {
        let implemented = facts.item.item.implementers.0.contains(facts.caller);
        let review = status == Status::InReview && !implemented;
        return Some(if review { Act::Review } else { Act::TakeOver });
    }
    match status {
        Status::Approved => Some(Act::Merge),
        Status::InReview => None,
        _ => Some(Act::Submit),
    }
}
