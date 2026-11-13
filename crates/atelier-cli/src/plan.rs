//! The planner's commands besides `ready` (`atelier_app::ready`): `new`,
//! `set`, `drop` and `close`. Each reads and writes records in the checkout
//! it runs in, on the branch the change belongs on, validates what it
//! writes (`atelier_app::plan`), commits nothing, and ends with the Git
//! command that commits what it wrote.

use crate::commands::Outcome;
use crate::output;
use anyhow::{anyhow, Result};
use atelier_app::acts::{check, settle_refusals, Act, MissionFacts, Refusal, PLANNED, STARTED};
use atelier_app::plan::{is_mission, misplaced, planned_facts, save, save_with, Belongs};
use atelier_app::repo::{item_dir, mission_path};
use atelier_app::{refused as refused_act, Located, Refused, Repo};
use atelier_core::{
    Finding, FindingName, Item, ItemSchema, Mission, MissionSchema, MissionStatus, Priority,
    SchemaVersion, Settlement, Status, Threads,
};
use atelier_records::{put_first, validate_id, ReviewFile};

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

pub fn new(
    repo: &mut Repo,
    title: &str,
    into: Option<String>,
    mission: bool,
    from: Option<&str>,
) -> Result<Outcome> {
    if mission {
        return Ok(Outcome::Refused(output::new_mission(title)));
    }
    let belongs = match from {
        Some(_) => Belongs::Settle,
        None => Belongs::Backlog,
    };
    let command = match from {
        Some(_) => "new --from",
        None => "new",
    };
    if let Some(refused) = misplaced(repo, command, belongs)? {
        return Ok(refused.into());
    }
    let id = repo.new_id()?;
    if let Some(mission) = into.as_deref() {
        if !repo.root.join(mission_path(mission)).exists() {
            return Ok(Outcome::Refused(output::no_mission(mission)));
        }
    }
    if let Some(name) = from {
        let take = Settle::Take { id, title, into };
        return settle(repo, "new --from", name, take);
    }
    let item = draft_item(&id, title, into);
    let said = output::created_item(&id);
    let message = format!("Plan {id}");
    Ok(save(repo, "new", vec![item], Vec::new(), &said, &message)?.into())
}

/// A new draft mission `id` titled `title`, with its sections empty, which
/// `start --mission` creates on its branch.
pub fn draft_mission(id: &str, title: &str) -> Mission {
    Mission {
        schema: MissionSchema::Mission,
        schema_version: SchemaVersion,
        id: id.to_string(),
        title: title.to_string(),
        status: MissionStatus::Draft,
        labels: Vec::new(),
        lead: None,
        ready: None,
        created_at: chrono::Utc::now().into(),
        body: MISSION_BODY.to_string(),
    }
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
    if let Some(refused) = misplaced(repo, "set", Belongs::Backlog)? {
        return Ok(refused.into());
    }
    validate_id(id).map_err(|message| anyhow!(message))?;
    if repo.root.join(mission_path(id)).exists() {
        let item_only = fields.item_only();
        if !item_only.is_empty() {
            return Ok(Outcome::Refused(output::item_only(id, &item_only)));
        }
        let mut mission = repo.store().mission(id)?.mission;
        let status = mission.status.status();
        if let Some(refused) = unplanned(repo, id, status, output::not_planned(id, status)) {
            return Ok(refused.into());
        }
        if let Some(title) = fields.title {
            mission.title = title;
        }
        if let Some(labels) = &fields.labels {
            mission.labels = list(labels);
        }
        let said = format!("Set the fields of mission {id}.\n");
        let message = format!("Set fields of {id}");
        return Ok(save(repo, "set", Vec::new(), vec![mission], &said, &message)?.into());
    }
    let located = repo.locate_planned(id)?;
    let Some(record) = located.record() else {
        return Ok(Outcome::Refused(output::set_unknown(id)));
    };
    let status = record.item.status;
    let text = match STARTED.contains(&status) {
        true => output::set_started(id, status),
        false => output::not_planned(id, status),
    };
    if let Some(refused) = unplanned(repo, id, status, text) {
        return Ok(refused.into());
    }
    // Planning is the base's record, whatever a branch holds.
    let mut item = located.base.as_ref().unwrap_or(record).item.clone();
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
    let message = format!("Set fields of {id}");
    Ok(save(repo, "set", vec![item], Vec::new(), &said, &message)?.into())
}

/// `set` on a record that is `status`, refused unless it is a draft or
/// ready, with `text`.
fn unplanned(repo: &Repo, id: &str, status: Status, text: String) -> Option<Refused> {
    match PLANNED.contains(&status) {
        true => None,
        false => repo
            .skip
            .refuse([output::rule_from(id, PLANNED, status)], text),
    }
}

/// `drop`: ends an item or a mission that will not be done, with the
/// reason as the first line of its Report or Result, in this checkout. A
/// started item is dropped on its own branch; the base's copy of it, and
/// an item not started or a mission, on a backlog branch.
pub fn drop(repo: &mut Repo, id: &str, reason: Option<&str>) -> Result<Outcome> {
    let finding = id.contains('/');
    let Some(reason) = reason
        .map(str::trim)
        .filter(|r| !r.is_empty() && !r.contains(['\n', '\r']))
    else {
        return Ok(Outcome::Refused(output::no_drop_reason(id)));
    };
    if finding {
        if let Some(refused) = misplaced(repo, "drop", Belongs::Settle)? {
            return Ok(refused.into());
        }
        return settle(repo, "drop", id, Settle::Drop(reason));
    }
    let line = format!("Dropped: {reason}");
    if is_mission(repo, id) {
        if let Some(refused) = misplaced(repo, "drop", Belongs::Backlog)? {
            return Ok(refused.into());
        }
        let dir = repo.store().mission(id)?;
        let facts = MissionFacts {
            mission: &dir,
            items: &[],
            review: None,
        };
        if let Some(refused) = repo.skip.mission_gate(Act::Drop, id, &facts)? {
            return Ok(refused.into());
        }
        let mut mission = dir.mission;
        mission.status = MissionStatus::Dropped;
        mission.body = put_first(&mission.body, "Result", &line);
        let said = output::dropped(id, "mission", reason);
        let message = format!("Drop {id}");
        return Ok(save(repo, "drop", Vec::new(), vec![mission], &said, &message)?.into());
    }
    validate_id(id).map_err(|message| anyhow!(message))?;
    let located = repo.locate_planned(id)?;
    let Some(record) = located.record() else {
        return Ok(Outcome::Refused(output::unknown_refused("drop", id)));
    };
    let branch = located.branch.as_ref();
    // Merged, as the base or the forge's copy of it holds it, whatever
    // this clone's base says.
    if let Some(branch) = branch {
        if record.item.status != Status::Dropped && repo.merged_anywhere(&branch.tip)? {
            return Ok(Outcome::Refused(output::drop_merged(id)));
        }
    }
    let on_branch = branch.filter(|b| !b.merged).and_then(|b| b.record.as_ref());
    match on_branch.map(|r| r.item.status) {
        // Dropped on its branch: what is left is the base's copy, which
        // this checkout holds unless the item is only on its branch.
        Some(Status::Dropped) => carry_drop(repo, id, &located, reason),
        Some(status) if STARTED.contains(&status) => {
            drop_started(repo, id, &located, &line, reason)
        }
        _ => {
            if let Some(refused) = misplaced(repo, "drop", Belongs::Backlog)? {
                return Ok(refused.into());
            }
            let facts = planned_facts(repo, &located, record);
            if let Some(refused) = repo.skip.refusal(Act::Drop, &facts)? {
                return Ok(refused.into());
            }
            let mut item = record.item.clone();
            item.status = Status::Dropped;
            item.body = put_first(&item.body, "Report", &line);
            let said = output::dropped(id, "item", reason);
            let message = format!("Drop {id}");
            Ok(save(repo, "drop", vec![item], Vec::new(), &said, &message)?.into())
        }
    }
}

/// `drop` of a started item, in a checkout of its branch: its record there
/// is dropped, and the agent commits and pushes it.
fn drop_started(
    repo: &mut Repo,
    id: &str,
    located: &Located,
    line: &str,
    reason: &str,
) -> Result<Outcome> {
    if let Some(refused) = misplaced(repo, "drop", Belongs::Item(id))? {
        return Ok(refused.into());
    }
    let record = located.record().expect("a started item has a record");
    let facts = planned_facts(repo, located, record);
    if let Some(refused) = repo.skip.refusal(Act::Drop, &facts)? {
        return Ok(refused.into());
    }
    let mut item = record.item.clone();
    item.status = Status::Dropped;
    item.body = put_first(&item.body, "Report", line);
    let message = format!("Drop {id}");
    let said = output::dropped_started(id, &repo.branch_of(id), reason, repo.remote());
    Ok(save(repo, "drop", vec![item], Vec::new(), &said, &message)?.into())
}

/// `drop` of an item dropped on its branch: writes the base's copy, as
/// this checkout holds it, dropped, for a backlog branch to carry to the
/// base. An item only on its branch, as after `start --new`, gets its
/// branch's front matter and its Goal, Done when and Scope; its Report,
/// any Decisions and its reviews stay on the branch, so nothing on the
/// base names a review the base lacks.
fn carry_drop(repo: &mut Repo, id: &str, located: &Located, reason: &str) -> Result<Outcome> {
    if let Some(refused) = misplaced(repo, "drop", Belongs::Backlog)? {
        return Ok(refused.into());
    }
    let branch = located.record().expect("a dropped item has a record");
    let mut item = match &located.base {
        Some(base) if base.item.status == Status::Dropped => {
            return Ok(Outcome::Refused(output::drop_reached_base(id)));
        }
        Some(base) => base.item.clone(),
        None => {
            let mut item = branch.item.clone();
            let definition = ["Goal", "Done when", "Scope"].into_iter();
            let section = |h| match branch.section(h)?.trim() {
                "" => Some(format!("\n## {h}\n")),
                text => Some(format!("\n## {h}\n\n{text}\n")),
            };
            item.body = definition.filter_map(section).collect();
            item
        }
    };
    let why = branch.dropped().unwrap_or(reason).to_string();
    item.status = Status::Dropped;
    item.body = put_first(&item.body, "Report", &format!("Dropped: {why}"));
    let said = output::dropped(id, "item", &why);
    let message = format!("Drop {id}");
    Ok(save(repo, "drop", vec![item], Vec::new(), &said, &message)?.into())
}

/// `close`: ends a ready mission whose items are all done or dropped and
/// whose Result is written, or, with its result, a draft or ready item that
/// has no branch.
pub fn close(repo: &mut Repo, id: &str, result: Option<&str>) -> Result<Outcome> {
    if let Some(refused) = misplaced(repo, "close", Belongs::Backlog)? {
        return Ok(refused.into());
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
        if let Some(refused) = repo.skip.mission_gate(Act::CloseMission, id, &facts)? {
            return Ok(refused.into());
        }
        let mut mission = dir.mission;
        mission.status = MissionStatus::Done;
        let said = output::closed(id, "mission");
        let message = format!("Close {id}");
        return Ok(save(repo, "close", Vec::new(), vec![mission], &said, &message)?.into());
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
    if let Err(refusal) = check(Act::Close, &facts) {
        // An item with a branch ends by its merge, or by `drop`.
        let text: Refused =
            match facts.branch && !matches!(record.item.status, Status::Done | Status::Dropped) {
                true => output::close_started(id).into(),
                false => refused_act(Act::Close, &refusal, &facts)?,
            };
        if let Some(refused) = repo.skip.gated(Act::Close, &facts, text) {
            return Ok(refused.into());
        }
    }
    let mut item = record.item.clone();
    item.status = Status::Done;
    item.body = put_first(&item.body, "Report", result);
    let said = output::closed(id, "item");
    let message = format!("Close {id}");
    Ok(save(repo, "close", vec![item], Vec::new(), &said, &message)?.into())
}

/// How `new --from` or `drop` settles a deferred finding.
enum Settle<'a> {
    /// Into a new draft item with this ID and title, in this mission if
    /// any.
    Take {
        id: String,
        title: &'a str,
        into: Option<String>,
    },
    /// With this reason.
    Drop(&'a str),
}

/// Settles the deferred finding `name`, `<item>/<finding>`, in this
/// checkout: writes its `Taken:` or `Dropped by` line, and for `Take` the
/// new item, both or neither.
fn settle(repo: &mut Repo, command: &str, name: &str, how: Settle) -> Result<Outcome> {
    let parsed = name.split_once('/').and_then(|(id, finding)| {
        validate_id(id).ok()?;
        Some((id, finding.parse::<FindingName>().ok()?))
    });
    let Some((id, finding)) = parsed else {
        return Ok(Outcome::Refused(output::not_a_finding(command, name)));
    };
    let located = repo.locate_planned(id)?;
    let Some(record) = located.record() else {
        let none = Refusal::Unmet(vec![atelier_app::acts::Condition::NoFinding(finding)]);
        return Ok(Outcome::Refused(output::settle_refused(
            command,
            name,
            id,
            &[none],
        )));
    };
    let facts = planned_facts(repo, &located, record);
    let all = settle_refusals(&facts, &finding)?;
    if !all.is_empty() {
        let text = output::settle_refused(command, name, id, &all);
        if let Some(refused) = repo.skip.gate(Act::Settle, id, &all, text) {
            return Ok(refused.into());
        }
    }
    let found = (record.findings()?.into_iter())
        .find(|f| f.name == finding)
        .expect("a finding to settle exists");
    let mut review = (record.reviews.iter())
        .find(|r| r.name == finding.review)
        .expect("a finding is in one of its item's reviews")
        .clone();
    let by = repo.caller.clone();
    let (items, settlement, said, message) = match how {
        Settle::Drop(reason) => {
            let settlement = Settlement::Dropped {
                by,
                reason: reason.to_string(),
            };
            let said = output::finding_dropped(name, reason);
            (Vec::new(), settlement, said, format!("Drop finding {name}"))
        }
        Settle::Take {
            id: new,
            title,
            into,
        } => {
            let mut item = draft_item(&new, title, into);
            item.body = taken_body(name, &review, &found);
            let said = output::finding_taken(name, &new);
            let message = format!("Take finding {name} into {new}");
            let settlement = Settlement::Taken { item: new, by };
            (vec![item], settlement, said, message)
        }
    };
    assert!(review.settle(finding.number, &settlement));
    let command = format!("{command} {name}");
    let saved = save_with(
        repo,
        &command,
        items,
        Vec::new(),
        vec![review],
        &said,
        &message,
    )?;
    Ok(saved.into())
}

/// The body of an item taken from finding `name`: its Goal names the
/// finding and its review file and quotes the finding as written.
fn taken_body(name: &str, review: &ReviewFile, finding: &Finding) -> String {
    let number = finding.name.number;
    let text = review.finding_text(number).unwrap_or_default().trim_end();
    let heading = format!("Finding {number}: {}\n{text}", finding.title);
    let quoted: String = (heading.lines())
        .map(|line| match line.is_empty() {
            true => ">\n".to_string(),
            false => format!("> {line}\n"),
        })
        .collect();
    let id = name.split('/').next().unwrap_or_default();
    let from = format!(
        "Taken from deferred finding {name}, in {}/reviews/{}.md:",
        item_dir(id),
        review.name
    );
    ITEM_BODY.replacen("## Goal\n", &format!("## Goal\n\n{from}\n\n{quoted}"), 1)
}
