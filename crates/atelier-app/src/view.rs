//! Where an item's record is read from, and what `status` shows.

use crate::acts::{mission_of, next, unmet, Act, Condition, Facts, PLANNED, STARTED};
use crate::disputed::{disputes, Disputes};
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
    /// The tip the record was read at.
    pub tip: String,
    /// Whether it was read from the forge's remote-tracking branch, as for
    /// an item started in another clone, rather than the local branch.
    pub remote: bool,
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

/// Each item of `items` dropped on its unmerged branch whose base copy is
/// not dropped yet: its ID, its title and why it was dropped.
pub fn unreached(items: &[Located]) -> Vec<(String, String, Option<String>)> {
    items
        .iter()
        .filter_map(|located| {
            let branch = located.branch.as_ref().filter(|b| !b.merged)?;
            let record = branch.record.as_ref()?;
            let base = located.base.as_ref().map(|b| b.item.status);
            (record.item.status == Status::Dropped && base != Some(Status::Dropped)).then(|| {
                let why = record.dropped().map(str::to_string);
                (located.id.clone(), record.item.title.clone(), why)
            })
        })
        .collect()
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
    /// The last review the caller held, with its verdict, when it is not
    /// this round's reviewer.
    pub reviewed: Option<(ReviewName, Option<Verdict>)>,
    /// What the checkout has not committed, where the CLI gathered it.
    pub uncommitted: Vec<String>,
    /// Once approved, the time of the commit that recorded the approving
    /// verdict, where the CLI looked it up; nothing stores it.
    pub approved: Option<String>,
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
        next,
        unmet,
        needs: needs(facts)?,
    })
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
    /// [`items_of`] orders them, and its disagreements.
    pub led: Vec<(MissionDir, Vec<Entry>, Disputes)>,
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
            let id = &mission.mission.id;
            let of_mission = items_of(id, &entries).into_iter().cloned().collect();
            let disputed = disputes(items, std::slice::from_ref(mission), Some(id))?;
            overview.led.push((mission.clone(), of_mission, disputed));
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
