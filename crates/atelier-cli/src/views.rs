//! `show`, `list` and `lead`. They read items and missions as `status`
//! does: the base's record from its last commit, a started item's from its
//! branch. `lead` writes planning, so it reads and writes its mission in the
//! checkout of the base, and commits it.

use crate::commands::Outcome;
use crate::view_output;
use anyhow::{anyhow, Result};
use atelier_app::acts::{check_mission, unsettled, Act, MissionFacts, Refusal};
use atelier_app::plan::{misplaced, save, Belongs};
use atelier_app::repo::mission_path;
use atelier_app::{
    disputes, entries, item_view, items_of, list as groups, mission_view, Entry, Filter, Repo,
};
use atelier_records::validate_id;

pub fn show(repo: &mut Repo, id: &str) -> Result<Outcome> {
    validate_id(id).map_err(|message| anyhow!(message))?;
    if let Some(view) = mission_view(repo, id)? {
        let items: Vec<&Entry> = view.items.iter().collect();
        return Ok(Outcome::Done(view_output::mission(
            &view.dir,
            view.in_progress,
            &items,
            &view.disputes,
            &view.dir.reviews,
        )));
    }
    let Some(view) = item_view(repo, id)? else {
        return Ok(Outcome::Refused(view_output::unknown(id, &repo.base)));
    };
    Ok(Outcome::Done(view_output::item(
        view.record(),
        &view.entry,
        view.links.mission.as_ref(),
        &view.findings,
        &view.open,
    )))
}

pub fn list(repo: &mut Repo, filter: &Filter) -> Result<Outcome> {
    let items = repo.everything()?;
    let missions = repo.missions()?;
    let entries = entries(&items, &missions, &repo.caller)?;
    let groups = groups(&entries, &missions, &repo.caller, filter);
    let mut text = view_output::list(&groups, filter);
    if !filter.done {
        text += &view_output::unreached(&atelier_app::unreached(&items));
    }
    Ok(Outcome::Done(text))
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
