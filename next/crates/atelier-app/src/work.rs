//! Views of many items and missions: `list`'s groups, a mission's items,
//! and whether a mission is in progress.

use crate::acts::STARTED;
use crate::view::Entry;
use atelier_core::{MissionStatus, Status, Thread};
use atelier_records::MissionDir;
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
    /// its mission is ready, which are `start`'s conditions.
    pub fn startable(&self) -> bool {
        self.status == Status::Ready && self.unmet.is_empty()
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
    mission.mission.status == MissionStatus::Ready
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

/// What `list` shows with `filter`: each mission with an item kept, by
/// ID, and then the kept items in no mission. A mission with no item kept
/// is shown when the mission itself is kept, unless a flag only items have
/// (`ready`, `blocked`, `needs_human`) is given.
pub fn list<'a>(
    entries: &'a [Entry],
    missions: &'a [MissionDir],
    caller: &Thread,
    filter: &Filter,
) -> Vec<Group<'a>> {
    let find = |id: &str| missions.iter().find(|m| m.mission.id == id);
    let leads = |m: &MissionDir| m.mission.lead.as_ref() == Some(caller);
    let keep_item = |e: &Entry| {
        e.finished() == filter.done
            && (!filter.ready || e.startable())
            && (!filter.blocked || !e.waiting_on().is_empty())
            && (!filter.needs_human || e.needs_human())
            && (filter.mission.is_none() || e.mission == filter.mission)
            && filter.label.as_ref().is_none_or(|l| e.labels.contains(l))
            && (!filter.mine
                || e.holds
                || e.review.is_some()
                || e.mission.as_deref().and_then(find).is_some_and(leads))
    };
    let only_items = filter.ready || filter.blocked || filter.needs_human;
    let keep_mission = |m: &&MissionDir| {
        let finished = matches!(
            m.mission.status,
            MissionStatus::Done | MissionStatus::Dropped
        );
        finished == filter.done
            && !only_items
            && filter.mission.as_ref().is_none_or(|id| *id == m.mission.id)
            && filter
                .label
                .as_ref()
                .is_none_or(|l| m.mission.labels.contains(l))
            && (!filter.mine || leads(m))
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
