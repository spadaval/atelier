//! Where an item's record is read from, and what `status` shows.

use crate::acts::{
    mission_of, next, unmet, Act, Condition, Facts, ESCALATE_AFTER, PLANNED, STARTED,
};
use crate::review::chain;
use crate::work::{in_progress, items_of};
use atelier_core::{FindingName, MissionStatus, Priority, ReviewName, Status, Thread, Verdict};
use atelier_records::{ItemDir, MissionDir, Problem};
use std::collections::HashMap;

/// An item as the commands find it: its record at the base's tip, and the
/// local branch named for it, if one exists.
pub struct Located {
    pub id: String,
    pub base: Option<ItemDir>,
    pub branch: Option<Branch>,
}

pub struct Branch {
    /// Whether the branch's tip is in the base's history.
    pub merged: bool,
    /// The item's record on the branch: from the checkout on the current
    /// branch, else, or in a repository opened with
    /// [`Repo::open_committed`](crate::Repo::open_committed), from its tip.
    pub record: Option<ItemDir>,
}

/// Which of an item's records [`Located::record`] picks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadFrom {
    /// The base's record.
    Base,
    /// The record on the item's unmerged branch.
    Branch,
}

impl Located {
    /// The item's record: the base's when that says done or dropped, else
    /// the record on its unmerged branch, else the base's.
    pub fn record(&self) -> Option<&ItemDir> {
        self.picked().map(|(record, _)| record)
    }

    /// Where [`Located::record`] reads the item's record from.
    pub fn read_from(&self) -> Option<ReadFrom> {
        self.picked().map(|(_, from)| from)
    }

    fn picked(&self) -> Option<(&ItemDir, ReadFrom)> {
        let base = self.base.as_ref().map(|base| (base, ReadFrom::Base));
        if self.base.as_ref().is_some_and(Located::terminal) {
            return base;
        }
        let started = self.branch.as_ref().filter(|branch| !branch.merged);
        let branch = started.and_then(|branch| branch.record.as_ref());
        branch.map(|record| (record, ReadFrom::Branch)).or(base)
    }

    pub fn status(&self) -> Option<Status> {
        self.record().map(|dir| dir.item.status)
    }

    /// Whether the base's record `base` ends the item whatever its branch
    /// holds: done or dropped.
    pub fn terminal(base: &ItemDir) -> bool {
        matches!(base.item.status, Status::Done | Status::Dropped)
    }
}

/// One item as `status` shows it to the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub priority: Priority,
    pub status: Status,
    pub mission: Option<String>,
    pub labels: Vec<String>,
    /// Each item it is blocked by, with its status; `None` when no such
    /// item is found.
    pub blockers: Vec<(String, Option<Status>)>,
    pub holder: Option<Thread>,
    pub holds: bool,
    /// The caller holds it building, and its Goal or its Done when lines
    /// are not written yet, as after `start --new`, or as in a record
    /// whose Done when is prose: whether the Goal is unwritten, and
    /// whether the Done when lines are.
    pub undefined: Option<(bool, bool)>,
    /// This round's review, when the caller holds it.
    pub review: Option<ReviewName>,
    /// While it is in review, the thread holding this round's review file,
    /// if one is open.
    pub reviewer: Option<Thread>,
    /// For its implementer, while it is in review or changes are
    /// requested: the round under review, or the round that asked for
    /// the changes.
    pub round: Option<u32>,
    /// For its implementer, while it is in review or changes are
    /// requested: how many rounds have ended with no approve verdict, once
    /// that is enough to require escalation, with the open fix-now finding
    /// to escalate when changes are requested. `None` once a question has
    /// been asked about each open finding, since that is the escalation.
    pub unapproved: Option<(usize, Option<FindingName>)>,
    /// The last review the caller held, with its verdict, when it is not
    /// this round's reviewer.
    pub reviewed: Option<(ReviewName, Option<Verdict>)>,
    /// What the checkout has not committed, where the CLI gathered it.
    pub uncommitted: Vec<String>,
    /// Once approved, the time of the commit that recorded the approving
    /// verdict, where the CLI looked it up; nothing stores it.
    pub approved: Option<String>,
    /// Its unanswered questions, where the CLI read them.
    pub questions: Vec<String>,
    /// The act the caller would run next, and its conditions that do not
    /// hold.
    pub next: Option<Act>,
    pub unmet: Vec<Condition>,
    /// For a started item another thread holds: what its holder's next act
    /// needs, the conditions the holder's `status` lists as unmet.
    pub needs: Vec<Condition>,
}

pub fn entry(facts: &Facts) -> Result<Entry, Problem> {
    let next = next(facts)?;
    let unmet = match next {
        Some(act) => unmet(act, facts)?,
        None => Vec::new(),
    };
    let item = &facts.item.item;
    let goal = facts.item.section("Goal").unwrap_or_default();
    let unwritten = (goal.trim().is_empty(), facts.item.done_when().is_empty());
    let undefined =
        (facts.holds() && item.status == Status::Building && unwritten != (false, false))
            .then_some(unwritten);
    let round = match item.status {
        _ if !facts.holds() => None,
        Status::InReview => Some(facts.round().number),
        Status::ChangesRequested => facts.item.reviews.iter().map(|r| r.name.round).max(),
        _ => None,
    };
    let rounds = Some(facts.unapproved_rounds())
        .filter(|rounds| facts.holds() && round.is_some() && *rounds >= ESCALATE_AFTER);
    let unapproved = match (rounds, item.status) {
        (Some(rounds), Status::ChangesRequested) => {
            unasked(facts.item)?.map(|finding| (rounds, Some(finding)))
        }
        // In review, which finding stands waits for the round's verdict.
        (Some(rounds), _) => Some((rounds, None)),
        (None, _) => None,
    };
    Ok(Entry {
        id: item.id.clone(),
        title: item.title.clone(),
        priority: item.priority,
        status: item.status,
        mission: item.mission.clone(),
        labels: item.labels.clone(),
        blockers: facts.blockers.to_vec(),
        holder: facts.holder().cloned(),
        holds: facts.holds(),
        undefined,
        review: facts.reviewing().map(|file| file.name.clone()),
        round,
        unapproved,
        reviewer: match facts.status() {
            Status::InReview => facts
                .round()
                .review
                .and_then(|file| file.review.reviewers.holder().cloned()),
            _ => None,
        },
        reviewed: facts
            .reviewing()
            .is_none()
            .then(|| facts.reviewed())
            .flatten()
            .map(|file| (file.name.clone(), file.review.verdict)),
        uncommitted: facts
            .checkout
            .map(|checkout| checkout.uncommitted.clone())
            .unwrap_or_default(),
        approved: None,
        questions: Vec::new(),
        next,
        unmet,
        needs: needs(facts)?,
    })
}

/// The first open fix-now finding of `item` that no question is about, in
/// its round or any it was carried from.
fn unasked(item: &ItemDir) -> Result<Option<FindingName>, Problem> {
    let (findings, decisions) = (item.findings()?, item.decisions()?);
    let asked = |name: &FindingName| {
        chain(name, &findings).iter().any(|f| {
            decisions
                .iter()
                .any(|d| d.finding.as_ref() == Some(&f.name))
        })
    };
    let open = item.open_findings()?.into_iter();
    Ok(open.map(|f| f.name).find(|name| !asked(name)))
}

/// For a started item another thread holds: the conditions its holder's
/// next act lists as unmet.
fn needs(facts: &Facts) -> Result<Vec<Condition>, Problem> {
    let holder = match facts.holder() {
        Some(holder) if !facts.holds() && STARTED.contains(&facts.status()) => holder,
        _ => return Ok(Vec::new()),
    };
    let theirs = Facts {
        caller: holder,
        mission: facts.mission.clone(),
        ..*facts
    };
    Ok(match next(&theirs)? {
        Some(act) => unmet(act, &theirs)?,
        None => Vec::new(),
    })
}

/// `status` away from an item's branch: what the caller holds, what is
/// ready, what other threads hold, and the missions.
#[derive(Debug, Default)]
pub struct Overview {
    pub held: Vec<Entry>,
    /// Ready items `start` would accept now, except those waiting on a
    /// human.
    pub ready: Vec<Entry>,
    /// The other ready items not waiting on a human, which `start` would
    /// refuse for the conditions each lists as unmet.
    pub cannot_start: Vec<Entry>,
    pub started: Vec<Entry>,
    /// Items labelled needs-human that nobody has started.
    pub waiting: Vec<Entry>,
    /// Ready missions with a started item.
    pub in_progress: Vec<MissionDir>,
    /// The missions the caller leads, each with its items, as
    /// [`items_of`] orders them.
    pub led: Vec<(MissionDir, Vec<Entry>)>,
}

/// What `status` shows the caller of each of `items`, which are every item
/// found on the base and on branches; a blocker is looked up among them,
/// and a mission among `missions`.
pub fn entries(
    items: &[Located],
    missions: &[MissionDir],
    caller: &Thread,
) -> Result<Vec<Entry>, Problem> {
    let statuses: HashMap<&str, Status> = items
        .iter()
        .filter_map(|located| Some((located.id.as_str(), located.status()?)))
        .collect();
    let mut entries = Vec::new();
    for located in items {
        let Some(record) = located.record() else {
            continue;
        };
        let blockers: Vec<(String, Option<Status>)> = record
            .item
            .blocked_by
            .iter()
            .map(|id| (id.clone(), statuses.get(id.as_str()).copied()))
            .collect();
        let facts = Facts {
            item: record,
            branch: located.branch.is_some(),
            blockers: &blockers,
            mission: mission_of(record, missions),
            caller,
            checkout: None,
        };
        entries.push(entry(&facts)?);
    }
    Ok(entries)
}

/// The overview of `items` and `missions`, as for [`entries`].
pub fn overview(
    items: &[Located],
    missions: &[MissionDir],
    caller: &Thread,
) -> Result<Overview, Problem> {
    let entries = entries(items, missions, caller)?;
    let mut overview = Overview::default();
    for mission in missions {
        if in_progress(mission, &entries) {
            overview.in_progress.push(mission.clone());
        }
        let ready = mission.mission.status == MissionStatus::Ready;
        if ready && mission.mission.lead.as_ref() == Some(caller) {
            let items = items_of(&mission.mission.id, &entries);
            overview
                .led
                .push((mission.clone(), items.into_iter().cloned().collect()));
        }
    }
    for entry in entries {
        if entry.needs_human() && PLANNED.contains(&entry.status) {
            overview.waiting.push(entry);
        } else if entry.startable() {
            overview.ready.push(entry);
        } else if entry.status == Status::Ready {
            overview.cannot_start.push(entry);
        } else if STARTED.contains(&entry.status) {
            if entry.holds {
                overview.held.push(entry);
            } else {
                overview.started.push(entry);
            }
        }
    }
    // The most urgent first.
    overview.ready.sort_by_key(|e| (e.priority, e.id.clone()));
    overview.held.sort_by(|a, b| a.id.cmp(&b.id));
    overview.started.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(overview)
}
