//! The act table: each lifecycle act, the role that runs it, the statuses
//! it runs from, the status it sets, and its conditions. The commands check
//! an act with [`check`], and `status` lists what [`unmet`] gives for the
//! act it names next, so the two cannot disagree.

use atelier_core::{is_open, FindingName, MissionStatus, Status, Thread, Verdict};
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
    Merge,
    MergeTrivial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Planner,
    Implementer,
    Reviewer,
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

pub const TABLE: [Row; 11] = [
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

/// The round of review an item is in.
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
    /// An open fix-now finding has no line in Responses and no decision.
    NoResponse {
        finding: FindingName,
        title: String,
    },
    /// A Decisions entry has no answer.
    Unanswered(String),
    /// The caller has been the item's implementer, so cannot review it.
    FormerImplementer,
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
    // role, whatever the status. `review` by an implementer is refused by
    // its own condition instead, so the author sees the brief, and a
    // planner's act on an item nobody holds yet needs no role on it.
    if let Some(role) = facts.role().filter(|role| *role != row.role) {
        if act != Act::Review && row.role != Role::Planner {
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
            Role::Planner => (true, None),
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

/// The status `act`, a mission's, sets on a mission that is `status`, or
/// why it is refused. Its rows have no conditions yet.
pub fn check_mission(act: Act, status: MissionStatus) -> Result<Option<MissionStatus>, Refusal> {
    let row = row(act);
    if !row.from.contains(&status.status()) {
        return Err(Refusal::From {
            act,
            status: status.status(),
        });
    }
    Ok(row.to.and_then(MissionStatus::of))
}

/// The conditions of `act` that do not hold.
pub fn unmet(act: Act, facts: &Facts) -> Result<Vec<Condition>, Problem> {
    let item = facts.item;
    let mut unmet = Vec::new();
    match act {
        Act::Ready | Act::ReadyMission | Act::StartNew => {}
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
        Act::VerdictChanges | Act::VerdictApprove => {
            let Some(file) = facts.reviewing() else {
                return Ok(unmet);
            };
            let problems = validate(std::slice::from_ref(item), &[]);
            if let Some(problem) = problems.into_iter().find(|p| p.file == file.path) {
                return Err(problem);
            }
            let findings = item.findings()?;
            let decisions = item.decisions()?;
            let open = findings
                .iter()
                .filter(|f| is_open(f, &findings, &decisions));
            if act == Act::VerdictChanges {
                if !open.clone().any(|f| f.name.review == file.name) {
                    unmet.push(Condition::NoOpenFinding);
                }
            } else {
                // One the file should carry and does not stays open too.
                unmet.extend(open.map(|f| Condition::OpenFinding {
                    finding: f.name.clone(),
                    title: f.title.clone(),
                }));
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
