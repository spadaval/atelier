//! Where an item's record is read from, and what `status` shows.

use crate::acts::{mission_of, next, unmet, Act, Condition, Facts, PLANNED, STARTED};
use crate::disputed::{disputes, Disputes};
use crate::work::{in_progress, items_of};
use atelier_core::{MissionStatus, Priority, ReviewName, Status, Thread, Verdict};
use atelier_records::{ItemDir, MissionDir, Problem};
use std::collections::HashMap;

/// What the reader asks of a record of either kind, an item or a mission.
pub trait Record {
    fn record_id(&self) -> &str;
    fn title(&self) -> &str;
    /// Done or dropped: a final copy on the base wins over any branch's.
    fn finished(&self) -> bool;
    fn is_dropped(&self) -> bool;
    /// Why it was dropped, as its record says.
    fn why_dropped(&self) -> Option<&str>;
    /// Whether a copy read from its own unmerged branch, where the base has
    /// none, is proposed: a mission's plan is, an item started there is not.
    const OWN_BRANCH_PROPOSES: bool;
}

impl Record for ItemDir {
    fn record_id(&self) -> &str {
        &self.item.id
    }
    fn title(&self) -> &str {
        &self.item.title
    }
    fn finished(&self) -> bool {
        matches!(self.item.status, Status::Done | Status::Dropped)
    }
    fn is_dropped(&self) -> bool {
        self.item.status == Status::Dropped
    }
    fn why_dropped(&self) -> Option<&str> {
        self.dropped()
    }
    const OWN_BRANCH_PROPOSES: bool = false;
}

impl Record for MissionDir {
    fn record_id(&self) -> &str {
        &self.mission.id
    }
    fn title(&self) -> &str {
        &self.mission.title
    }
    fn finished(&self) -> bool {
        matches!(
            self.mission.status,
            MissionStatus::Done | MissionStatus::Dropped
        )
    }
    fn is_dropped(&self) -> bool {
        self.mission.status == MissionStatus::Dropped
    }
    fn why_dropped(&self) -> Option<&str> {
        self.dropped()
    }
    const OWN_BRANCH_PROPOSES: bool = true;
}

/// A record as the commands find it: its copy at the base's tip, its copy
/// on the branch named for it, and its copies on the other unmerged named
/// branches that add, change or remove it. [`Located::record`] picks the
/// one that counts, by the rule in `docs/design.md`.
pub struct Located<R = ItemDir> {
    pub id: String,
    pub base: Option<R>,
    /// The branch named for it, local or the forge's remote-tracking one.
    pub branch: Option<Branch<R>>,
    /// Each other unmerged named branch whose changes since it forked from
    /// the base touch the record, by branch name, with its copy there;
    /// `None` where that branch removes it. Filled by the readers that
    /// read everything ([`Repo::everything`](crate::Repo::everything),
    /// [`Repo::find`](crate::Repo::find) and the missions'); empty from
    /// [`Repo::locate`](crate::Repo::locate).
    pub elsewhere: Vec<Branch<R>>,
}

pub struct Branch<R = ItemDir> {
    /// Whether the branch's tip is in the base's history.
    pub merged: bool,
    /// The tip the record was read at.
    pub tip: String,
    /// Whether it was read from the forge's remote-tracking branch, as for
    /// an item started in another clone, rather than the local branch.
    pub remote: bool,
    /// The ref read, as it is shown: the branch's name, or
    /// `<remote>/<branch>` for a remote-tracking one.
    pub name: String,
    /// The local branch and its remote-tracking branch have diverged, and
    /// this one was read.
    pub diverged: bool,
    /// The record on the branch: from the checkout on the current branch,
    /// else, or in a repository opened with
    /// [`Repo::open_committed`](crate::Repo::open_committed), from its tip.
    pub record: Option<R>,
}

/// Which of a record's copies [`Located::record`] picks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadFrom {
    /// The base's record.
    Base,
    /// The record on an unmerged branch: its own, or, for a record the
    /// base does not hold, the branch that adds it.
    Branch,
}

impl<R: Record> Located<R> {
    /// The record that counts: the base's when it is final, else the copy
    /// on its own unmerged branch, else the base's, else, for a record the
    /// base does not hold, the copy on the first unmerged branch by name
    /// that adds it.
    pub fn record(&self) -> Option<&R> {
        self.picked().map(|(record, _)| record)
    }

    /// Where [`Located::record`] reads the record from.
    pub fn read_from(&self) -> Option<ReadFrom> {
        self.picked().map(|(_, from)| match from {
            None => ReadFrom::Base,
            Some(_) => ReadFrom::Branch,
        })
    }

    /// The branch [`Located::record`] reads the record from; `None` for
    /// the base's.
    pub fn source(&self) -> Option<&Branch<R>> {
        self.picked().and_then(|(_, from)| from)
    }

    fn picked(&self) -> Option<(&R, Option<&Branch<R>>)> {
        let base = self.base.as_ref().map(|base| (base, None));
        if self.base.as_ref().is_some_and(R::finished) {
            return base;
        }
        let own = self.branch.as_ref().filter(|branch| !branch.merged);
        let own = own.and_then(|branch| Some((branch.record.as_ref()?, Some(branch))));
        let added = || {
            let mut adding = self.elsewhere.iter();
            adding.find_map(|branch| Some((branch.record.as_ref()?, Some(branch))))
        };
        match self.base {
            Some(_) => own.or(base),
            None => own.or_else(added),
        }
    }

    /// Whether it is proposed: open, not on the base, and read from an
    /// unmerged branch that adds it, or, for a mission, from its own.
    pub fn proposed(&self) -> bool {
        let Some((record, Some(from))) = self.picked() else {
            return false;
        };
        let own = self.branch.as_ref().is_some_and(|b| std::ptr::eq(b, from));
        self.base.is_none() && !record.finished() && (!own || R::OWN_BRANCH_PROPOSES)
    }

    /// The other unmerged branches that add it, where it is read from the
    /// first of them.
    pub fn also_added(&self) -> Vec<&Branch<R>> {
        let from = self.source();
        let adding = self.elsewhere.iter().filter(|b| b.record.is_some());
        match self.base {
            None => adding
                .filter(|b| !from.is_some_and(|f| std::ptr::eq(*b, f)))
                .collect(),
            Some(_) => Vec::new(),
        }
    }

    /// The unmerged branches that change or remove the base's copy: changes
    /// proposed, which take effect when the branch merges.
    pub fn changes(&self) -> Vec<&Branch<R>> {
        match self.base {
            Some(_) => self.elsewhere.iter().collect(),
            None => Vec::new(),
        }
    }
}

impl Located {
    pub fn status(&self) -> Option<Status> {
        self.record().map(|dir| dir.item.status)
    }

    /// Whether the base's record `base` ends the item whatever its branch
    /// holds: done or dropped.
    pub fn terminal(base: &ItemDir) -> bool {
        base.finished()
    }
}

/// Each record of `located` dropped on its unmerged branch whose base copy
/// is not dropped yet: its ID, its title and why it was dropped.
pub fn unreached<R: Record>(located: &[Located<R>]) -> Vec<(String, String, Option<String>)> {
    located
        .iter()
        .filter_map(|located| {
            let branch = located.branch.as_ref().filter(|b| !b.merged)?;
            let record = branch.record.as_ref()?;
            let base_dropped = located.base.as_ref().is_some_and(R::is_dropped);
            (record.is_dropped() && !base_dropped).then(|| {
                let why = record.why_dropped().map(str::to_string);
                (located.id.clone(), record.title().to_string(), why)
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
    /// The ref its record was read from, where views name it: a
    /// remote-tracking branch, or the unmerged branch a proposed item is
    /// on. Set by [`entries`], not by [`entry`].
    pub on: Option<String>,
    /// It is proposed ([`Located::proposed`]). Set by [`entries`].
    pub proposed: bool,
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
        on: None,
        proposed: false,
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
        let mut entry = entry(&facts)?;
        mark(&mut entry, located);
        entries.push(entry);
    }
    Ok(entries)
}

/// Marks `entry` with where `located`, its item, was read from.
pub fn mark(entry: &mut Entry, located: &Located) {
    entry.proposed = located.proposed();
    let source = located.source();
    entry.on = source
        .filter(|branch| branch.remote || entry.proposed)
        .map(|branch| branch.name.clone());
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
        let ready = mission.mission.status == MissionStatus::Open;
        if ready && mission.mission.lead.as_ref() == Some(caller) {
            let id = &mission.mission.id;
            let of_mission = items_of(id, &entries).into_iter().cloned().collect();
            let disputed = disputes(items, std::slice::from_ref(mission), Some(id))?;
            overview.led.push((mission.clone(), of_mission, disputed));
        }
    }
    // A proposed item is not on the base yet: nobody can start it.
    for entry in entries.into_iter().filter(|entry| !entry.proposed) {
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
