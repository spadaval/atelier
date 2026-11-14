//! `show`, `list` and `lead`. They read items and missions as `status`
//! does: the base's record from its last commit, a started item's from its
//! branch. `lead` writes its mission in the checkout it runs in, a backlog
//! branch's, and commits nothing.

use crate::commands::Outcome;
use crate::view_output;
use anyhow::{anyhow, Result};
use atelier_app::acts::{check_mission, unsettled, Act, MissionFacts, Refusal};
use atelier_app::plan::{misplaced, save, Belongs};
use atelier_app::repo::mission_path;
use atelier_app::{
    disputes, entries, git, item_view, items_of, list as groups, mission_view, narrows_item,
    narrows_mission, Entry, Filter, Located, Record, Repo,
};
use atelier_records::{validate_id, ItemDir, MissionDir};
use view_output::{Change, Whence};

pub fn show(repo: &mut Repo, id: &str) -> Result<Outcome> {
    validate_id(id).map_err(|message| anyhow!(message))?;
    if let Some(view) = mission_view(repo, id)? {
        let items: Vec<&Entry> = view.items.iter().collect();
        let whence = whence(repo, &view.mission, |dir| dir.mission.status.status())?;
        return Ok(Outcome::Done(view_output::mission(
            &view.dir,
            &whence,
            view.in_progress,
            &items,
            &view.disputes,
            &view.dir.reviews,
        )));
    }
    let Some(view) = item_view(repo, id)? else {
        return Ok(Outcome::Refused(view_output::unknown(id, &repo.base)));
    };
    let whence = whence(repo, &view.located, |dir| dir.item.status)?;
    Ok(Outcome::Done(view_output::item(
        view.record(),
        &whence,
        &view.entry,
        view.links.mission.as_ref(),
        &view.findings,
        &view.open,
    )))
}

/// Where `show` read `located`'s record from, and the other branches that
/// add or change it; `status` gives a copy's status.
fn whence<R: Record>(
    repo: &Repo,
    located: &Located<R>,
    status: impl Fn(&R) -> atelier_core::Status,
) -> Result<Whence> {
    let source = located.source();
    let fetched = match source.is_some_and(|branch| branch.remote) {
        true => Some(git::fetched_at(&repo.root)?),
        false => None,
    };
    let names = |branches: Vec<&atelier_app::Branch<R>>| {
        branches.iter().map(|b| b.name.clone()).collect::<Vec<_>>()
    };
    Ok(Whence {
        from: source.map_or(repo.base.clone(), |branch| branch.name.clone()),
        fetched,
        proposed: located.proposed(),
        diverged: source.is_some_and(|branch| branch.diverged),
        also_added: names(located.also_added()),
        changes: located
            .changes()
            .into_iter()
            .map(|branch| (branch.name.clone(), branch.record.as_ref().map(&status)))
            .collect(),
    })
}

pub fn list(repo: &mut Repo, filter: &Filter) -> Result<Outcome> {
    let items = repo.everything()?;
    let located = repo.all_missions()?;
    let missions: Vec<MissionDir> = located.iter().filter_map(|l| l.record().cloned()).collect();
    let proposed: Vec<String> = (located.iter())
        .filter(|l| l.proposed())
        .map(|l| l.id.clone())
        .collect();
    let mut entries = entries(&items, &missions, &repo.caller)?;
    // A plan dropped on its branch proposes nothing, its items included.
    let dropped = |id: &String| {
        let found = missions.iter().find(|m| m.mission.id == *id);
        found.is_some_and(|m| m.mission.status == atelier_core::MissionStatus::Dropped)
    };
    entries.retain(|e| !(e.proposed && e.mission.as_ref().is_some_and(dropped)));
    let mut text = match repo.remote() {
        Some(remote) => view_output::fetched(remote, git::fetched_at(&repo.root)?),
        None => String::new(),
    };
    let on = |id: &str| {
        let found = located.iter().find(|l| l.id == id && l.proposed());
        found.and_then(|l| l.source()).map(|b| b.name.clone())
    };
    let narrowed =
        filter.done || filter.ready || filter.blocked || filter.needs_human || filter.mine;
    let mut base = String::new();
    let mut nothing = false;
    if !filter.proposed {
        let groups = groups(&entries, &missions, &proposed, &repo.caller, filter);
        nothing = groups.is_empty();
        base = view_output::list(&groups, filter, &on);
    }
    let mut elsewhere = String::new();
    if filter.proposed || !narrowed {
        let only = Filter {
            proposed: true,
            mission: filter.mission.clone(),
            label: filter.label.clone(),
            ..*filter
        };
        let groups = groups(&entries, &missions, &proposed, &repo.caller, &only);
        // A change is kept as the flags would keep its record.
        let mut changes = Vec::new();
        for item in &items {
            let entry = entries.iter().find(|e| e.id == item.id);
            let kept = entry.is_some_and(|e| narrows_item(e, &missions, &repo.caller, filter));
            if let Some(record) = item.record().filter(|_| kept) {
                changes.extend(change(item, record, |dir: &ItemDir| dir.item.status));
            }
        }
        for mission in &located {
            let kept = |r: &&MissionDir| narrows_mission(r, &repo.caller, filter);
            if let Some(record) = mission.record().filter(kept) {
                changes.extend(change(mission, record, |dir: &MissionDir| {
                    dir.mission.status.status()
                }));
            }
        }
        elsewhere = view_output::proposed(&groups, &changes, filter.proposed, &on);
    }
    // Nothing open on the base is worth saying only when nothing is
    // proposed either.
    if !nothing || elsewhere.is_empty() {
        text += &base;
    }
    text += elsewhere.trim_start_matches(|c| nothing && c == '\n');
    // A drop that has not reached the base is shown as the flags narrow
    // the rest: it can start nobody, block nothing, nor be anyone's now.
    if !narrowed && !filter.proposed {
        let record = |id: &str| {
            let found = items.iter().find(|located| located.id == id);
            found.and_then(|located| located.record())
        };
        let unreached: Vec<_> = atelier_app::unreached(&items)
            .into_iter()
            .filter(|(id, _, _)| {
                let Some(record) = record(id) else {
                    return false;
                };
                let item = &record.item;
                let mission = (filter.mission.as_ref())
                    .is_none_or(|mission| item.mission.as_ref() == Some(mission));
                let label = (filter.label.as_ref()).is_none_or(|label| item.labels.contains(label));
                mission && label
            })
            .collect();
        text += &view_output::unreached(&unreached);
        let dropped: Vec<_> = atelier_app::unreached(&located)
            .into_iter()
            .filter(|(id, _, _)| filter.mission.as_ref().is_none_or(|m| m == id))
            .filter(|(id, _, _)| {
                let found = located.iter().find(|l| l.id == *id);
                let labels = found.and_then(|l| l.record()).map(|r| &r.mission.labels);
                (filter.label.as_ref()).is_none_or(|l| labels.is_some_and(|ls| ls.contains(l)))
            })
            .map(|(id, title, why)| {
                let found = located.iter().find(|l| l.id == id);
                let on = found
                    .and_then(|l| l.branch.as_ref())
                    .map(|b| b.name.clone());
                (id, title, why, on.unwrap_or_default())
            })
            .collect();
        text += &view_output::dropped_missions(&dropped);
    }
    Ok(Outcome::Done(text))
}

/// Each change an unmerged branch proposes to `located`'s base copy,
/// where `record` is its copy that counts.
fn change<R: Record>(
    located: &Located<R>,
    record: &R,
    status: impl Fn(&R) -> atelier_core::Status,
) -> Vec<Change> {
    let Some(base) = located.base.as_ref() else {
        return Vec::new();
    };
    located
        .changes()
        .into_iter()
        .map(|branch| Change {
            id: located.id.clone(),
            title: record.title().to_string(),
            on: branch.name.clone(),
            before: status(base),
            after: branch.record.as_ref().map(&status),
        })
        .collect()
}

/// `list --deferred`: the deferred findings of done items that nobody has
/// settled. It reads the items of this checkout, committed or not, as
/// `new --from` and `drop` do, so it lists what they would settle here.
pub fn deferred(repo: &mut Repo) -> Result<Outcome> {
    let mut found = Vec::new();
    for dir in repo.store().items()? {
        let unsettled = unsettled(&dir)?.into_iter();
        found.extend(unsettled.map(|finding| (dir.item.id.clone(), finding)));
    }
    Ok(Outcome::Done(view_output::deferred(&found)))
}

/// `list --disputed`: each rebuttal of an open or done item, with what the
/// next round made of it, and each plan-review round whose reviewers'
/// verdicts differ; with `mission`, only that mission's. It reads as
/// `status` does, a started item from its branch.
pub fn disputed(repo: &mut Repo, mission: Option<&str>) -> Result<Outcome> {
    let items = repo.everything()?;
    let missions = repo.missions()?;
    let found = disputes(&items, &missions, mission)?;
    Ok(Outcome::Done(view_output::disputed(&found, mission)))
}

/// Makes the caller the lead of a ready mission in this checkout, without
/// committing it, and says where each of its items stands.
pub fn lead(repo: &mut Repo, id: &str) -> Result<Outcome> {
    if let Some(refused) = misplaced(repo, "lead", Belongs::Backlog)? {
        return Ok(refused.into());
    }
    validate_id(id).map_err(|message| anyhow!(message))?;
    if !repo.root.join(mission_path(id)).exists() {
        return Ok(Outcome::Refused(view_output::lead_unknown(id)));
    }
    let mut dir = repo.store().mission(id)?;
    let facts = MissionFacts {
        mission: &dir,
        items: &[],
        review: None,
    };
    if let Err(refusal) = check_mission(Act::Lead, &facts) {
        let Refusal::From { status, .. } = refusal else {
            unreachable!("lead has no conditions: {refusal:?}")
        };
        let text = view_output::lead_not_ready(id, status);
        if let Some(refused) = repo.skip.gate(Act::Lead, id, &[refusal], text) {
            return Ok(refused.into());
        }
    }
    let before = dir.mission.lead.replace(repo.caller.clone());
    // Its items as `start` will find them, so what can start now can.
    let items = repo.everything()?;
    let entries = entries(&items, &repo.missions()?, &repo.caller)?;
    let said = view_output::lead(
        &dir.mission,
        before.as_ref(),
        &repo.caller,
        &items_of(id, &entries),
    );
    // A thread that already leads it changes nothing, so writes nothing.
    if before.as_ref() == Some(&repo.caller) {
        return Ok(Outcome::Done(said));
    }
    let message = format!("Lead {id}");
    Ok(save(repo, "lead", Vec::new(), vec![dir.mission], &said, &message)?.into())
}
