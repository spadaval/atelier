//! The planner's commands: `new`, `set`, `ready`, `drop` and `close`.
//! Planning happens on
//! the base and is committed with Git, so each reads and writes records in
//! the checkout of the base, validates what it writes, commits nothing, and
//! ends with the Git command that commits what it wrote.

use crate::commands::{refused as refused_act, Outcome};
use crate::repo::{git_at, item_path, mission_dir, mission_path, Repo};
use crate::{git, output};
use anyhow::{anyhow, Result};
use atelier_app::acts::{
    check, check_mission, Act, Facts, MissionFacts, Refusal, PLANNED, STARTED,
};
use atelier_app::Located;
use atelier_core::{
    Item, ItemSchema, Mission, MissionSchema, MissionStatus, Priority, Ready, SchemaVersion,
    Status, Threads,
};
use atelier_records::{put_first, validate, validate_id, ItemDir, MissionDir, Problem};

const ITEM_BODY: &str = "\n## Goal\n\n## Done when\n\n## Scope\n";
const MISSION_BODY: &str =
    "\n## Why\n\n## Outcome\n\n## Success criteria\n\n## Scope\n\n## Authority\n\n## Result\n";

/// A new draft item with empty Goal, Done when and Scope sections.
pub fn draft_item(id: &str, title: &str, mission: Option<String>) -> Item {
    Item {
        schema: ItemSchema::Item,
        schema_version: SchemaVersion,
        id: id.to_string(),
        title: title.to_string(),
        status: Status::Draft,
        mission,
        labels: Vec::new(),
        priority: Priority::Medium,
        blocked_by: Vec::new(),
        implementers: Threads::default(),
        ready: None,
        trivial: None,
        caused_by: None,
        created_at: chrono::Utc::now().into(),
        body: ITEM_BODY.to_string(),
    }
}

pub fn new(repo: &mut Repo, title: &str, into: Option<String>, mission: bool) -> Result<Outcome> {
    if let Some(refused) = off_base(repo, "new")? {
        return Ok(refused);
    }
    let id = repo.new_id()?;
    if mission {
        let mission = Mission {
            schema: MissionSchema::Mission,
            schema_version: SchemaVersion,
            id: id.clone(),
            title: title.to_string(),
            status: MissionStatus::Draft,
            labels: Vec::new(),
            lead: None,
            ready: None,
            created_at: chrono::Utc::now().into(),
            body: MISSION_BODY.to_string(),
        };
        let said = output::created_mission(&id);
        let message = format!("Plan mission {id}");
        return save(repo, "new", Vec::new(), vec![mission], &said, &message);
    }
    if let Some(mission) = into.as_deref() {
        if !repo.root.join(mission_path(mission)).exists() {
            return Ok(Outcome::Refused(output::no_mission(mission)));
        }
    }
    let item = draft_item(&id, title, into);
    let said = output::created_item(&id);
    save(
        repo,
        "new",
        vec![item],
        Vec::new(),
        &said,
        &format!("Plan {id}"),
    )
}

/// The fields `set` changes; `None` leaves a field as it is. Lists are
/// comma-separated, and an empty value clears a list or an optional field.
pub struct Fields {
    pub title: Option<String>,
    pub priority: Option<Priority>,
    pub labels: Option<String>,
    pub mission: Option<String>,
    pub blocked_by: Option<String>,
    pub caused_by: Option<String>,
}

impl Fields {
    /// The flags given that only an item has.
    fn item_only(&self) -> Vec<&'static str> {
        let given = [
            ("--priority", self.priority.is_some()),
            ("--mission", self.mission.is_some()),
            ("--blocked-by", self.blocked_by.is_some()),
            ("--caused-by", self.caused_by.is_some()),
        ];
        given
            .into_iter()
            .filter(|(_, g)| *g)
            .map(|(f, _)| f)
            .collect()
    }
}

fn list(text: &str) -> Vec<String> {
    let values = text.split(',').map(str::trim).filter(|v| !v.is_empty());
    values.map(str::to_string).collect()
}

fn optional(text: &str) -> Option<String> {
    Some(text.trim().to_string()).filter(|v| !v.is_empty())
}

pub fn set(repo: &mut Repo, id: &str, fields: Fields) -> Result<Outcome> {
    if let Some(refused) = off_base(repo, "set")? {
        return Ok(refused);
    }
    validate_id(id).map_err(|message| anyhow!(message))?;
    if repo.root.join(mission_path(id)).exists() {
        let item_only = fields.item_only();
        if !item_only.is_empty() {
            return Ok(Outcome::Refused(output::item_only(id, &item_only)));
        }
        let mut mission = repo.store().mission(id)?.mission;
        let status = mission.status.status();
        if !PLANNED.contains(&status) {
            return Ok(Outcome::Refused(output::not_planned(id, status)));
        }
        if let Some(title) = fields.title {
            mission.title = title;
        }
        if let Some(labels) = &fields.labels {
            mission.labels = list(labels);
        }
        let said = format!("Set the fields of mission {id}.\n");
        let message = format!("Set fields of {id}");
        return save(repo, "set", Vec::new(), vec![mission], &said, &message);
    }
    let located = repo.locate_planned(id)?;
    let Some(record) = located.record() else {
        return Ok(Outcome::Refused(output::set_unknown(id)));
    };
    let mut item = record.item.clone();
    if STARTED.contains(&item.status) {
        return Ok(Outcome::Refused(output::set_started(id, item.status)));
    }
    if !PLANNED.contains(&item.status) {
        return Ok(Outcome::Refused(output::not_planned(id, item.status)));
    }
    let Fields {
        title,
        priority,
        labels,
        mission,
        blocked_by,
        caused_by,
    } = fields;
    item.title = title.unwrap_or(item.title);
    item.priority = priority.unwrap_or(item.priority);
    if let Some(labels) = labels {
        item.labels = list(&labels);
    }
    if let Some(mission) = mission {
        item.mission = optional(&mission);
    }
    if let Some(blocked_by) = blocked_by {
        item.blocked_by = list(&blocked_by);
    }
    if let Some(caused_by) = caused_by {
        item.caused_by = optional(&caused_by);
    }
    let said = format!("Set the fields of item {id}.\n");
    save(
        repo,
        "set",
        vec![item],
        Vec::new(),
        &said,
        &format!("Set fields of {id}"),
    )
}

pub fn ready(repo: &mut Repo, ids: &[String], by: &str) -> Result<Outcome> {
    if let Some(refused) = off_base(repo, "ready")? {
        return Ok(refused);
    }
    let ready = Ready {
        by: by.to_string(),
        at: chrono::Utc::now().into(),
    };
    let (mut items, mut missions, mut unknown, mut refused) = (vec![], vec![], vec![], vec![]);
    let mut reviewed = String::new();
    let mut named: Vec<&str> = Vec::new();
    for id in ids.iter().map(String::as_str) {
        if named.contains(&id) {
            continue;
        }
        named.push(id);
        if is_mission(repo, id) {
            let dir = repo.store().mission(id)?;
            let facts = MissionFacts {
                mission: &dir,
                items: &[],
                review: None,
            };
            match check_mission(Act::ReadyMission, &facts) {
                Ok(to) => {
                    reviewed += &plan_reviewed(repo, &dir)?;
                    let mut mission = dir.mission;
                    mission.status = to.expect("ready sets a status");
                    mission.ready = Some(ready.clone());
                    missions.push(mission);
                }
                Err(Refusal::From { status, .. }) => refused.push((id, status)),
                Err(other) => unreachable!("a mission's ready has no conditions: {other:?}"),
            }
            continue;
        }
        let located = match validate_id(id) {
            Ok(()) => repo.locate_planned(id)?,
            Err(_) => {
                unknown.push(id);
                continue;
            }
        };
        let Some(record) = located.record() else {
            unknown.push(id);
            continue;
        };
        let facts = planned_facts(repo, &located, record);
        match check(Act::Ready, &facts) {
            Ok(to) => {
                let mut item = record.item.clone();
                item.status = to.expect("ready sets a status");
                item.ready = Some(ready.clone());
                items.push(item);
            }
            Err(Refusal::From { status, .. }) => refused.push((id, status)),
            Err(other) => return Ok(Outcome::Refused(refused_act(Act::Ready, other, &facts)?)),
        }
    }
    if !unknown.is_empty() || !refused.is_empty() {
        return Ok(Outcome::Refused(output::ready_refused(&unknown, &refused)));
    }
    let said = output::readied(&named, by) + &reviewed;
    let message = format!("Ready {}", named.join(", "));
    save(repo, "ready", items, missions, &said, &message)
}

/// What `ready` says of a mission's plan reviews, for whoever says go:
/// each review of the latest round with its verdict, and what in the plan
/// changed since the commit it reviewed. None of it refuses anything.
fn plan_reviewed(repo: &mut Repo, dir: &MissionDir) -> Result<String> {
    let id = &dir.mission.id;
    let Some(round) = dir.reviews.iter().map(|r| r.name.round).max() else {
        return Ok(output::no_plan_review(id));
    };
    let latest: Vec<_> = dir
        .reviews
        .iter()
        .filter(|r| r.name.round == round)
        .collect();
    let mut commits: Vec<&str> = latest.iter().map(|r| r.review.commit.as_str()).collect();
    commits.sort();
    commits.dedup();
    let mut changed = Vec::new();
    for commit in commits {
        changed.push((commit, plan_changed(repo, id, commit)?));
    }
    Ok(output::plan_reviews(id, round, &latest, &changed))
}

/// The paths of mission `id`'s plan, its directory less its reviews and
/// the directories of its items then or now, that differ in this checkout
/// from `commit`; `None` when this clone lacks that commit.
fn plan_changed(repo: &mut Repo, id: &str, commit: &str) -> Result<Option<Vec<String>>> {
    if !git::has_commit(&repo.root, commit)? {
        return Ok(None);
    }
    let own = format!("{}/", mission_dir(id));
    let reviews = format!("{own}reviews/");
    let now: Vec<String> = repo
        .store()
        .items()?
        .into_iter()
        .filter(|dir| dir.item.mission.as_deref() == Some(id))
        .map(|dir| dir.item.id)
        .collect();
    let mut changed = Vec::new();
    for path in git::changed_since(&repo.root, commit, &[".atelier/missions", ".atelier/items"])? {
        let item = path
            .strip_prefix(".atelier/items/")
            .and_then(|rest| rest.split('/').next());
        let in_plan = match item {
            None => path.starts_with(&own) && !path.starts_with(&reviews),
            // An item that left the mission since, or was deleted, counts;
            // one whose record then does not read is counted too.
            Some(item) => {
                now.iter().any(|i| i == item)
                    || repo.item_at(commit, item).map_or(true, |then| {
                        then.is_some_and(|d| d.item.mission.as_deref() == Some(id))
                    })
            }
        };
        if in_plan {
            changed.push(path);
        }
    }
    Ok(Some(changed))
}

/// What a planner's act on the item `located`, whose record is `record`,
/// is checked against: its record and branch, and nothing it waits for.
fn planned_facts<'a>(repo: &'a Repo, located: &Located, record: &'a ItemDir) -> Facts<'a> {
    Facts {
        item: record,
        branch: located.branch.is_some(),
        blockers: &[],
        mission: None,
        caller: &repo.caller,
        checkout: None,
    }
}

/// Whether `id` names a mission in this checkout.
fn is_mission(repo: &Repo, id: &str) -> bool {
    validate_id(id).is_ok() && repo.root.join(mission_path(id)).exists()
}

/// `drop`: ends an item or a mission that will not be done, on the base,
/// whatever an item's branch holds, with the reason as the first line of
/// its Report or Result.
pub fn drop(repo: &mut Repo, id: &str, reason: Option<&str>) -> Result<Outcome> {
    if let Some(refused) = off_base(repo, "drop")? {
        return Ok(refused);
    }
    let Some(reason) = reason
        .map(str::trim)
        .filter(|r| !r.is_empty() && !r.contains(['\n', '\r']))
    else {
        return Ok(Outcome::Refused(output::no_drop_reason(id)));
    };
    let line = format!("Dropped: {reason}");
    if is_mission(repo, id) {
        let dir = repo.store().mission(id)?;
        let facts = MissionFacts {
            mission: &dir,
            items: &[],
            review: None,
        };
        if let Err(refusal) = check_mission(Act::Drop, &facts) {
            return Ok(Outcome::Refused(output::mission_refused(
                Act::Drop,
                id,
                &refusal,
            )));
        }
        let mut mission = dir.mission;
        mission.status = MissionStatus::Dropped;
        mission.body = put_first(&mission.body, "Result", &line);
        let said = output::dropped(id, "mission", reason, false);
        let message = format!("Drop {id}");
        return save(repo, "drop", Vec::new(), vec![mission], &said, &message);
    }
    validate_id(id).map_err(|message| anyhow!(message))?;
    let located = repo.locate_planned(id)?;
    let Some(record) = located.record() else {
        return Ok(Outcome::Refused(output::unknown_refused("drop", id)));
    };
    let facts = planned_facts(repo, &located, record);
    let to = match check(Act::Drop, &facts) {
        Ok(to) => to,
        Err(refusal) => return Ok(Outcome::Refused(refused_act(Act::Drop, refusal, &facts)?)),
    };
    // The base's record, whatever the branch holds. An item only on its
    // branch, as after `start --new`, gets its definition there; its
    // Report, Decisions and reviews stay on the branch, so nothing on the
    // base names a review the base lacks.
    let mut item = match &located.base {
        Some(base) => base.item.clone(),
        None => {
            let mut item = record.item.clone();
            let definition = ["Goal", "Done when", "Scope"].into_iter();
            let section = |h| match record.section(h)?.trim() {
                "" => Some(format!("\n## {h}\n")),
                text => Some(format!("\n## {h}\n\n{text}\n")),
            };
            item.body = definition.filter_map(section).collect();
            item
        }
    };
    item.status = to.expect("drop sets a status");
    item.body = put_first(&item.body, "Report", &line);
    let started = STARTED.contains(&record.item.status);
    let said = output::dropped(id, "item", reason, started);
    let message = format!("Drop {id}");
    save(repo, "drop", vec![item], Vec::new(), &said, &message)
}

/// `close`: ends a ready mission whose items are all done or dropped and
/// whose Result is written, or, with its result, a draft or ready item that
/// has no branch.
pub fn close(repo: &mut Repo, id: &str, result: Option<&str>) -> Result<Outcome> {
    if let Some(refused) = off_base(repo, "close")? {
        return Ok(refused);
    }
    if is_mission(repo, id) {
        if result.is_some() {
            return Ok(Outcome::Refused(output::close_mission_text(id)));
        }
        let dir = repo.store().mission(id)?;
        let mut items = Vec::new();
        for located in repo.planned()? {
            if let Some(record) = located.record() {
                if record.item.mission.as_deref() == Some(id) {
                    items.push((located.id.clone(), record.item.status));
                }
            }
        }
        let facts = MissionFacts {
            mission: &dir,
            items: &items,
            review: None,
        };
        if let Err(refusal) = check_mission(Act::CloseMission, &facts) {
            let refused = output::mission_refused(Act::CloseMission, id, &refusal);
            return Ok(Outcome::Refused(refused));
        }
        let mut mission = dir.mission;
        mission.status = MissionStatus::Done;
        let said = output::closed(id, "mission");
        let message = format!("Close {id}");
        return save(repo, "close", Vec::new(), vec![mission], &said, &message);
    }
    validate_id(id).map_err(|message| anyhow!(message))?;
    let Some(result) = result.map(str::trim).filter(|r| !r.is_empty()) else {
        return Ok(Outcome::Refused(output::no_result(id)));
    };
    let located = repo.locate_planned(id)?;
    let Some(record) = located.record() else {
        return Ok(Outcome::Refused(output::unknown_refused("close", id)));
    };
    let facts = planned_facts(repo, &located, record);
    let to = match check(Act::Close, &facts) {
        Ok(to) => to,
        // An item with a branch ends by its merge, or by `drop`.
        Err(_) if facts.branch && !matches!(record.item.status, Status::Done | Status::Dropped) => {
            return Ok(Outcome::Refused(output::close_started(id)))
        }
        Err(refusal) => return Ok(Outcome::Refused(refused_act(Act::Close, refusal, &facts)?)),
    };
    let mut item = record.item.clone();
    item.status = to.expect("close sets a status");
    item.body = put_first(&item.body, "Report", result);
    let said = output::closed(id, "item");
    let message = format!("Close {id}");
    save(repo, "close", vec![item], Vec::new(), &said, &message)
}

/// Refuses a planning command run anywhere but on the base branch.
fn off_base(repo: &Repo, command: &str) -> Result<Option<Outcome>> {
    if repo.branch.as_deref() == Some(repo.base.as_str()) {
        return Ok(None);
    }
    let checkout = git::checkout_of(&repo.root, &repo.base)?;
    Ok(Some(Outcome::Refused(output::off_base(
        command,
        &repo.base,
        repo.branch.as_deref(),
        checkout.as_deref(),
    ))))
}

/// Writes `items` and `missions` into the checkout, once each validates
/// against every record of the checkout, and says what it wrote and how to
/// commit it; or, when one has a problem, writes nothing and says why.
/// `said` opens the output, and `message` is the commit's.
fn save(
    repo: &Repo,
    command: &str,
    items: Vec<Item>,
    missions: Vec<Mission>,
    said: &str,
    message: &str,
) -> Result<Outcome> {
    let store = repo.store();
    let mut all_items = store.items()?;
    let mut all_missions = store.missions()?;
    let mut written = Vec::new();
    for item in &items {
        let path = repo.root.join(item_path(&item.id));
        written.push((path.clone(), item_path(&item.id)));
        match all_items.iter_mut().find(|dir| dir.item.id == item.id) {
            Some(dir) => dir.item = item.clone(),
            None => all_items.push(ItemDir {
                path,
                item: item.clone(),
                reviews: Vec::new(),
            }),
        }
    }
    for mission in &missions {
        let path = repo.root.join(mission_path(&mission.id));
        written.push((path.clone(), mission_path(&mission.id)));
        match all_missions
            .iter_mut()
            .find(|dir| dir.mission.id == mission.id)
        {
            Some(dir) => dir.mission = mission.clone(),
            None => all_missions.push(MissionDir {
                path,
                mission: mission.clone(),
                reviews: Vec::new(),
            }),
        }
    }
    let problems: Vec<Problem> = validate(&all_items, &all_missions)
        .into_iter()
        .filter(|problem| written.iter().any(|(path, _)| *path == problem.file))
        .collect();
    if !problems.is_empty() {
        return Ok(Outcome::Refused(output::invalid(
            command, &repo.root, &problems,
        )));
    }
    for item in &items {
        store.write_item(item)?;
    }
    for mission in &missions {
        store.write_mission(mission)?;
    }
    let paths: Vec<String> = written.into_iter().map(|(_, shown)| shown).collect();
    let at = git_at(&repo.root);
    Ok(Outcome::Done(output::planned(said, at, &paths, message)))
}
