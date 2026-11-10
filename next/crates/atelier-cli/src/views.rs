//! `show`, `list` and `lead`. They read items and missions as `status`
//! does: the base's record from its last commit, a started item's from its
//! branch. `lead` writes planning, so it reads and writes its mission in the
//! checkout of the base.

use crate::commands::{facts, Outcome};
use crate::repo::{mission_path, Repo};
use crate::{plan, view_output};
use anyhow::{anyhow, Result};
use atelier_app::acts::{check_mission, Act, MissionFacts, Refusal};
use atelier_app::{entries, entry, in_progress, items_of, list as groups, Filter};
use atelier_records::validate_id;

pub fn show(repo: &mut Repo, id: &str) -> Result<Outcome> {
    validate_id(id).map_err(|message| anyhow!(message))?;
    let missions = repo.missions()?;
    if let Some(dir) = missions.iter().find(|dir| dir.mission.id == id) {
        let items = repo.everything()?;
        let entries = entries(&items, &missions, &repo.caller)?;
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
    let entry = entry(&facts(repo, &located, &links, None))?;
    Ok(Outcome::Done(view_output::item(
        record,
        &entry,
        links.mission.as_ref(),
        &record.findings()?,
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

/// Makes the caller the lead of a ready mission in the checkout of the
/// base, and says where each of its items stands.
pub fn lead(repo: &mut Repo, id: &str) -> Result<Outcome> {
    if let Some(refused) = plan::off_base(repo, "lead")? {
        return Ok(refused);
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
        return Ok(Outcome::Refused(view_output::lead_not_ready(id, status)));
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
    plan::save(repo, "lead", Vec::new(), vec![dir.mission], &said, &message)
}
