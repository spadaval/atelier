//! `show`, `list` and `lead`. They read items and missions as `status`
//! does: the base's record from its last commit, a started item's from its
//! branch. `lead` writes planning, so it reads and writes its mission in the
//! checkout of the base, and commits it.

use crate::commands::{annotate, annotate_all, Outcome};
use crate::view_output;
use anyhow::{anyhow, Result};
use atelier_app::acts::{check_mission, unsettled, Act, MissionFacts, Refusal};
use atelier_app::plan::{off_base, save};
use atelier_app::repo::{facts, mission_path};
use atelier_app::{entries, entry, in_progress, items_of, list as groups, Filter, Repo};
use atelier_core::Status;
use atelier_records::validate_id;

pub fn show(repo: &mut Repo, id: &str) -> Result<Outcome> {
    validate_id(id).map_err(|message| anyhow!(message))?;
    let missions = repo.missions()?;
    if let Some(dir) = missions.iter().find(|dir| dir.mission.id == id) {
        let items = repo.everything()?;
        let mut entries = entries(&items, &missions, &repo.caller)?;
        let of_mission = entries
            .iter_mut()
            .filter(|e| e.mission.as_deref() == Some(id));
        annotate_all(repo, of_mission, &items)?;
        let mut questions = Vec::new();
        for record in items.iter().filter_map(|located| located.record()) {
            if record.item.mission.as_deref() == Some(id) {
                let asked = record.decisions()?.into_iter();
                questions.extend(asked.map(|d| (record.item.id.clone(), d)));
            }
        }
        return Ok(Outcome::Done(view_output::mission(
            dir,
            in_progress(dir, &entries),
            &items_of(id, &entries),
            &questions,
            &dir.reviews,
        )));
    }
    let located = repo.locate(id)?;
    let Some(record) = located.record() else {
        return Ok(Outcome::Refused(view_output::unknown(id, &repo.base)));
    };
    let links = repo.links(record)?;
    let mut entry = entry(&facts(repo, &located, &links, None))?;
    annotate(repo, &mut entry, record)?;
    // A settlement is written in the checkout it is run in, on the base or
    // on an implementer's branch, so one this checkout holds is shown too.
    // The item's own branch holds none, merged or not, and its record there
    // is not read, so one that does not parse cannot hide the base's.
    let own_branch = repo.branch.as_deref() == Some(id);
    let mut findings = record.findings()?;
    let settling = record.item.status == Status::Done && !own_branch;
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
    Ok(Outcome::Done(view_output::item(
        record,
        &entry,
        links.mission.as_ref(),
        &findings,
        &record.open_findings()?,
    )))
}

pub fn list(repo: &mut Repo, filter: &Filter) -> Result<Outcome> {
    let items = repo.everything()?;
    let missions = repo.missions()?;
    let entries = entries(&items, &missions, &repo.caller)?;
    let groups = groups(&entries, &missions, &repo.caller, filter);
    Ok(Outcome::Done(view_output::list(&groups, filter)))
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

/// Makes the caller the lead of a ready mission in the checkout of the
/// base, and says where each of its items stands.
pub fn lead(repo: &mut Repo, id: &str) -> Result<Outcome> {
    if let Some(refused) = off_base(repo, "lead")? {
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
    // A thread that already leads it changes nothing, so has nothing to
    // commit.
    if before.as_ref() == Some(&repo.caller) {
        return Ok(Outcome::Done(said));
    }
    let message = format!("Lead {id}");
    Ok(save(
        repo,
        "lead",
        Vec::new(),
        vec![dir.mission],
        &said,
        &message,
        true,
    )?
    .into())
}
