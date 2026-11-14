//! Views of many items and missions: `list`'s groups, a mission's items,
//! and whether a mission is in progress.

use crate::acts::STARTED;
use crate::model::{MissionStatus, Status, Thread};
use crate::records::MissionDir;
use crate::view::Entry;
use std::collections::BTreeMap;

/// The label of an item that waits on the human, such as a question for
/// him.
pub const NEEDS_HUMAN: &str = "needs-human";

/// The statuses in the order of the lifecycle, which views group items by.
pub const LIFECYCLE: [Status; 8] = [
    Status::Draft,
    Status::Ready,
    Status::Building,
    Status::InReview,
    Status::ChangesRequested,
    Status::Approved,
    Status::Done,
    Status::Dropped,
];

impl Entry {
    /// A thread could start it now: it is ready, its blockers are done and
    /// its mission is ready, which are `start`'s conditions, and it is on
    /// the base, not proposed on a branch that has not merged.
    pub fn startable(&self) -> bool {
        self.status == Status::Ready && self.unmet.is_empty() && !self.proposed
    }

    pub fn needs_human(&self) -> bool {
        self.labels.iter().any(|label| label == NEEDS_HUMAN)
    }

    pub fn finished(&self) -> bool {
        matches!(self.status, Status::Done | Status::Dropped)
    }

    /// Its blockers that are not done.
    pub fn waiting_on(&self) -> Vec<&(String, Option<Status>)> {
        let undone = self.blockers.iter();
        undone.filter(|(_, s)| *s != Some(Status::Done)).collect()
    }

    /// Its labels sorted, which `list` groups items by.
    pub fn label_set(&self) -> Vec<String> {
        let mut labels = self.labels.clone();
        labels.sort();
        labels
    }
}

/// Whether `mission` is in progress: ready, with a started item among
/// `entries`. That is shown, never stored.
pub fn in_progress(mission: &MissionDir, entries: &[Entry]) -> bool {
    let id = &mission.mission.id;
    mission.mission.status == MissionStatus::Open
        && entries
            .iter()
            .any(|e| e.mission.as_ref() == Some(id) && STARTED.contains(&e.status))
}

/// The items of mission `id` among `entries`, in the order of the
/// lifecycle, then the most urgent first.
pub fn items_of<'a>(id: &str, entries: &'a [Entry]) -> Vec<&'a Entry> {
    let mut items: Vec<&Entry> = entries
        .iter()
        .filter(|e| e.mission.as_deref() == Some(id))
        .collect();
    let stage = |status: Status| LIFECYCLE.iter().position(|s| *s == status);
    items.sort_by_key(|e| (stage(e.status), e.priority, e.id.clone()));
    items
}

/// `list`'s flags. Each one given narrows what is shown.
#[derive(Debug, Default)]
pub struct Filter {
    /// Items a thread could start now.
    pub ready: bool,
    /// Items with a blocker that is not done.
    pub blocked: bool,
    pub needs_human: bool,
    pub mission: Option<String>,
    pub label: Option<String>,
    /// What the caller holds as implementer, reviewer or lead; a mission it
    /// leads keeps its items.
    pub mine: bool,
    /// Finished missions and items, done or dropped, instead of open ones.
    pub done: bool,
    /// What is proposed on branches not yet merged, instead of the base's
    /// backlog.
    pub proposed: bool,
}

/// One mission's kept items, or the kept items in no mission.
pub struct Group<'a> {
    /// The mission's ID; `None` for items in no mission.
    pub id: Option<String>,
    /// The mission's record at the base, when there is one.
    pub mission: Option<&'a MissionDir>,
    pub in_progress: bool,
    /// By label set, then the most urgent first.
    pub items: Vec<&'a Entry>,
}

/// Whether `entry` passes `filter`'s narrowing flags, all but `done` and
/// `proposed`; its mission is looked up among `missions`.
pub fn narrows_item(
    entry: &Entry,
    missions: &[MissionDir],
    caller: &Thread,
    filter: &Filter,
) -> bool {
    let find = |id: &str| missions.iter().find(|m| m.mission.id == id);
    let leads = |m: &MissionDir| m.mission.lead.as_ref() == Some(caller);
    (!filter.ready || entry.startable())
        && (!filter.blocked || !entry.waiting_on().is_empty())
        && (!filter.needs_human || entry.needs_human())
        && (filter.mission.is_none() || entry.mission == filter.mission)
        && filter
            .label
            .as_ref()
            .is_none_or(|l| entry.labels.contains(l))
        && (!filter.mine
            || entry.holds
            || entry.review.is_some()
            || entry.mission.as_deref().and_then(find).is_some_and(leads))
}

/// Whether `mission` passes `filter`'s narrowing flags, all but `done` and
/// `proposed`: none that only items have is given, and the rest keep it.
pub fn narrows_mission(mission: &MissionDir, caller: &Thread, filter: &Filter) -> bool {
    let only_items = filter.ready || filter.blocked || filter.needs_human;
    !only_items
        && filter
            .mission
            .as_ref()
            .is_none_or(|id| *id == mission.mission.id)
        && (filter.label.as_ref()).is_none_or(|l| mission.mission.labels.contains(l))
        && (!filter.mine || mission.mission.lead.as_ref() == Some(caller))
}

/// What `list` shows with `filter`: each mission with an item kept, by
/// ID, and then the kept items in no mission. A mission with no item kept
/// is shown when the mission itself is kept, unless a flag only items have
/// (`ready`, `blocked`, `needs_human`) is given. `proposed` names the
/// missions of `missions` that are proposed: they and the proposed items
/// are kept with `filter.proposed`, and only then.
pub fn list<'a>(
    entries: &'a [Entry],
    missions: &'a [MissionDir],
    proposed: &[String],
    caller: &Thread,
    filter: &Filter,
) -> Vec<Group<'a>> {
    let find = |id: &str| missions.iter().find(|m| m.mission.id == id);
    let keep_item = |e: &Entry| {
        e.finished() == filter.done
            && e.proposed == filter.proposed
            && narrows_item(e, missions, caller, filter)
    };
    let keep_mission = |m: &&MissionDir| {
        let finished = matches!(
            m.mission.status,
            MissionStatus::Done | MissionStatus::Dropped
        );
        finished == filter.done
            && proposed.contains(&m.mission.id) == filter.proposed
            && narrows_mission(m, caller, filter)
    };
    // Items in no mission sort last.
    let mut groups: BTreeMap<(bool, Option<String>), Vec<&Entry>> = BTreeMap::new();
    for mission in missions.iter().filter(keep_mission) {
        groups
            .entry((false, Some(mission.mission.id.clone())))
            .or_default();
    }
    for entry in entries.iter().filter(|e| keep_item(e)) {
        let key = (entry.mission.is_none(), entry.mission.clone());
        groups.entry(key).or_default().push(entry);
    }
    groups
        .into_iter()
        .map(|((_, id), mut items)| {
            items.sort_by_key(|e| (e.label_set(), e.priority, e.id.clone()));
            let mission = id.as_deref().and_then(find);
            Group {
                in_progress: mission.is_some_and(|m| in_progress(m, entries)),
                id,
                mission,
                items,
            }
        })
        .collect()
}
