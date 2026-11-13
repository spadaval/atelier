//! What `show` shows of a mission and of an item, as data, read as
//! `status` reads: the base's record from its last commit, a started
//! item's from its branch.

use crate::acts::approval;
use crate::disputed::{disputes, Disputes};
use crate::repo::{facts, item_dir, Links};
use crate::view::{entries, entry, Entry, Located, ReadFrom};
use crate::work::{in_progress, items_of};
use crate::{git, Repo};
use anyhow::Result;
use atelier_core::{Decision, Finding, Status};
use atelier_records::{ItemDir, MissionDir};

/// A mission as `show <mission>` shows it.
pub struct MissionView {
    /// The mission's record, with its plan reviews in `reviews`.
    pub dir: MissionDir,
    /// Whether it is ready with a started item ([`in_progress`]).
    pub in_progress: bool,
    /// Its items, as [`items_of`] orders them, each [`annotate`]d.
    pub items: Vec<Entry>,
    /// Each question asked on one of its items before `escalate` was
    /// removed, with that item's ID, in item ID order. The Desk shows them
    /// until atelier-v958 removes its decide page; `show` does not.
    pub questions: Vec<(String, Decision)>,
    /// Its items' rebuttals and its plan's split rounds.
    pub disputes: Disputes,
    /// The records of each of its items, base and branch, in item ID
    /// order, for what a caller shows beside an entry.
    pub located: Vec<Located>,
}

/// The mission `id` at the base's tip, or `None` when it has none.
pub fn mission_view(repo: &mut Repo, id: &str) -> Result<Option<MissionView>> {
    let missions = repo.missions()?;
    let Some(dir) = missions.iter().find(|dir| dir.mission.id == id) else {
        return Ok(None);
    };
    let items = repo.everything()?;
    let entries = entries(&items, &missions, &repo.caller)?;
    let mut of_mission: Vec<Entry> = items_of(id, &entries).into_iter().cloned().collect();
    annotate_all(repo, &mut of_mission, &items)?;
    let mut questions = Vec::new();
    for record in items.iter().filter_map(Located::record) {
        if record.item.mission.as_deref() == Some(id) {
            let asked = record.decisions()?.into_iter();
            questions.extend(asked.map(|d| (record.item.id.clone(), d)));
        }
    }
    let disputes = disputes(&items, std::slice::from_ref(dir), Some(id))?;
    let in_progress = in_progress(dir, &entries);
    let dir = dir.clone();
    let of =
        |located: &Located| located.record().and_then(|r| r.item.mission.as_deref()) == Some(id);
    Ok(Some(MissionView {
        in_progress,
        dir,
        items: of_mission,
        questions,
        disputes,
        located: items.into_iter().filter(of).collect(),
    }))
}

/// An item as `show <item>` shows it.
pub struct ItemView {
    /// The item's records; [`ItemView::record`] is the one shown.
    pub located: Located,
    /// Whether the record shown is the base's or its branch's.
    pub read_from: ReadFrom,
    /// What its start depends on.
    pub links: Links,
    /// What `status` shows of it to the caller, [`annotate`]d.
    pub entry: Entry,
    /// The findings of its reviews, with any settlement this checkout
    /// holds uncommitted (see [`item_view`]).
    pub findings: Vec<Finding>,
    /// Its open findings, as its record holds them.
    pub open: Vec<Finding>,
}

impl ItemView {
    /// The record shown.
    pub fn record(&self) -> &ItemDir {
        self.located.record().expect("an item view has a record")
    }
}

/// The item `id`, or `None` when no record of it is found. A settlement
/// is written in the checkout it is run in, on the base or on an
/// implementer's branch, so for a done item one this checkout holds is
/// shown too, unless the repository was opened with
/// [`Repo::open_committed`]. The item's own branch holds none, merged or
/// not, and its record there is not read, so one that does not parse
/// cannot hide the base's.
pub fn item_view(repo: &mut Repo, id: &str) -> Result<Option<ItemView>> {
    let located = repo.locate(id)?;
    let (Some(record), Some(read_from)) = (located.record(), located.read_from()) else {
        return Ok(None);
    };
    let links = repo.links(record)?;
    let mut entry = entry(&facts(repo, &located, &links, None))?;
    annotate(repo, &mut entry, record)?;
    let own_branch = repo.branch.as_deref() == Some(id);
    let mut findings = record.findings()?;
    let settling = record.item.status == Status::Done && !own_branch && !repo.committed;
    if let Some(here) = settling
        .then(|| repo.checkout_item(id))
        .transpose()?
        .flatten()
    {
        for settled in here.findings()?.into_iter().filter(|f| f.is_settled()) {
            if let Some(finding) = findings.iter_mut().find(|f| f.name == settled.name) {
                finding.settlement = settled.settlement;
            }
        }
    }
    let open = record.open_findings()?;
    Ok(Some(ItemView {
        located,
        read_from,
        links,
        entry,
        findings,
        open,
    }))
}

/// Adds to `entry`, which `record` gives, what views show besides the act
/// table: when it was approved, read from the item's branch.
pub fn annotate(repo: &Repo, entry: &mut Entry, record: &ItemDir) -> Result<()> {
    let id = &record.item.id;
    let approved = approval(record).filter(|_| record.item.status == Status::Approved);
    if let (Some(review), Some(tip)) = (approved, git::branch_tip(&repo.root, id)?) {
        let path = format!("{}/reviews/{}.md", item_dir(id), review.name);
        entry.approved = git::approved_at(&repo.root, &tip, &path)?;
    }
    Ok(())
}

/// [`annotate`] for each of `entries`, found among `items`.
pub fn annotate_all<'a>(
    repo: &Repo,
    entries: impl IntoIterator<Item = &'a mut Entry>,
    items: &[Located],
) -> Result<()> {
    for entry in entries {
        let found = items.iter().find(|located| located.id == entry.id);
        if let Some(record) = found.and_then(Located::record) {
            annotate(repo, entry, record)?;
        }
    }
    Ok(())
}
