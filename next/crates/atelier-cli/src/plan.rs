//! The planner's commands: `new`, `set` and `ready`. Planning happens on
//! the base and is committed with Git, so each reads and writes records in
//! the checkout of the base, validates what it writes, commits nothing, and
//! ends with the Git command that commits what it wrote.

use crate::commands::{refused as refused_act, Outcome};
use crate::repo::{git_at, item_path, mission_path, Repo};
use crate::{git, output};
use anyhow::{anyhow, Result};
use atelier_app::acts::{check, check_mission, Act, Facts, Refusal, PLANNED, STARTED};
use atelier_core::{
    Item, ItemSchema, Mission, MissionSchema, MissionStatus, Priority, Ready, SchemaVersion,
    Status, Threads,
};
use atelier_records::{validate, validate_id, ItemDir, MissionDir, Problem};

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
    let mut named: Vec<&str> = Vec::new();
    for id in ids.iter().map(String::as_str) {
        if named.contains(&id) {
            continue;
        }
        named.push(id);
        if validate_id(id).is_ok() && repo.root.join(mission_path(id)).exists() {
            let mut mission = repo.store().mission(id)?.mission;
            match check_mission(Act::ReadyMission, mission.status) {
                Ok(to) => {
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
        let facts = Facts {
            item: record,
            branch: located.branch.is_some(),
            blockers: &[],
            mission: None,
            caller: &repo.caller,
            checkout: None,
        };
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
    let said = output::readied(&named, by);
    let message = format!("Ready {}", named.join(", "));
    save(repo, "ready", items, missions, &said, &message)
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
        all_missions.retain(|dir| dir.mission.id != mission.id);
        all_missions.push(MissionDir {
            path,
            mission: mission.clone(),
        });
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
