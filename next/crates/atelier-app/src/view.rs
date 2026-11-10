//! Where an item's record is read from, and what `status` shows.

use crate::acts::{mission_of, next, unmet, Act, Condition, Facts, PLANNED, STARTED};
use crate::work::{in_progress, items_of};
use atelier_core::{MissionStatus, Priority, ReviewName, Status, Thread, Verdict};
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
    /// branch, else from its tip.
    pub record: Option<ItemDir>,
}

impl Located {
    /// The item's record: the base's when that says done or dropped, else
    /// the record on its unmerged branch, else the base's.
    pub fn record(&self) -> Option<&ItemDir> {
        let base = self.base.as_ref();
        if base.is_some_and(|dir| matches!(dir.item.status, Status::Done | Status::Dropped)) {
            return base;
        }
        let started = self.branch.as_ref().filter(|branch| !branch.merged);
        started.and_then(|branch| branch.record.as_ref()).or(base)
    }

    pub fn status(&self) -> Option<Status> {
        self.record().map(|dir| dir.item.status)
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
    /// are not written yet, as after `start --new`.
    pub undefined: bool,
    /// This round's review, when the caller holds it.
    pub review: Option<ReviewName>,
    /// While it is in review, the thread holding this round's review file,
    /// if one is open.
    pub reviewer: Option<Thread>,
    /// The last review the caller held, with its verdict, when it is not
    /// this round's reviewer.
    pub reviewed: Option<(ReviewName, Option<Verdict>)>,
    /// What the checkout has not committed, where the CLI gathered it.
    pub uncommitted: Vec<String>,
    /// The act the caller would run next, and its conditions that do not
    /// hold.
    pub next: Option<Act>,
    pub unmet: Vec<Condition>,
}

pub fn entry(facts: &Facts) -> Result<Entry, Problem> {
    let next = next(facts)?;
    let unmet = match next {
        Some(act) => unmet(act, facts)?,
        None => Vec::new(),
    };
    let item = &facts.item.item;
    let goal = facts.item.section("Goal").unwrap_or_default();
    let undefined = facts.holds()
        && item.status == Status::Building
        && (goal.trim().is_empty() || facts.item.done_when().is_empty());
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
        next,
        unmet,
    })
}

/// `status` away from an item's branch: what the caller holds, what is
/// ready, what other threads hold, and the missions.
#[derive(Debug, Default)]
pub struct Overview {
    pub held: Vec<Entry>,
    /// Ready items, except those waiting on a human.
    pub ready: Vec<Entry>,
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
        } else if entry.status == Status::Ready {
            overview.ready.push(entry);
        } else if STARTED.contains(&entry.status) {
            if entry.holds {
                overview.held.push(entry);
            } else {
                overview.started.push(entry);
            }
        }
    }
    // What can start now comes first, the most urgent first.
    let order = |e: &Entry| (!e.unmet.is_empty(), e.priority, e.id.clone());
    overview.ready.sort_by_key(order);
    overview.held.sort_by(|a, b| a.id.cmp(&b.id));
    overview.started.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(overview)
}
