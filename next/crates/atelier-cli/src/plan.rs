//! The planner's commands: `new`, `set`, `ready`, `drop` and `close`.
//! Planning happens on
//! the base and is committed with Git, so each reads and writes records in
//! the checkout of the base, validates what it writes, commits nothing, and
//! ends with the Git command that commits what it wrote.

use crate::commands::{gate, gated, mission_gate, refusal, refused as refused_act, Outcome};
use crate::repo::{git_at, item_dir, item_path, mission_dir, mission_path, Repo};
use crate::{git, output, skip};
use anyhow::{anyhow, Result};
use atelier_app::acts::{
    check, check_mission, row, settle_refusals, Act, Facts, MissionFacts, Refusal, PLANNED, STARTED,
};
use atelier_app::Located;
use atelier_core::{
    Finding, FindingName, Item, ItemSchema, Mission, MissionSchema, MissionStatus, Priority, Ready,
    SchemaVersion, Settlement, Status, Threads,
};
use atelier_records::{put_first, validate, validate_id, ItemDir, MissionDir, Problem, ReviewFile};
use std::path::PathBuf;

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
    let refused = match from {
        Some(_) => off_settling(repo, "new --from")?,
        None => off_base(repo, "new")?,
    };
    if let Some(refused) = refused {
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
    if let Some(name) = from {
        let take = Settle::Take { id, title, into };
        return settle(repo, "new --from", name, take);
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
        if let Some(refused) = unplanned(id, status, output::not_planned(id, status)) {
            return Ok(refused);
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
    let status = record.item.status;
    let text = match STARTED.contains(&status) {
        true => output::set_started(id, status),
        false => output::not_planned(id, status),
    };
    if let Some(refused) = unplanned(id, status, text) {
        return Ok(refused);
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
    save(
        repo,
        "set",
        vec![item],
        Vec::new(),
        &said,
        &format!("Set fields of {id}"),
    )
}

/// `set` on a record that is `status`, refused unless it is a draft or
/// ready, with `text`.
fn unplanned(id: &str, status: Status, text: String) -> Option<Outcome> {
    match PLANNED.contains(&status) {
        true => None,
        false => skip::refuse([output::rule_from(id, PLANNED, status)], text),
    }
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
                Ok(_) => {}
                Err(Refusal::From { status, .. }) => refused.push((id, status)),
                Err(other) => unreachable!("a mission's ready has no conditions: {other:?}"),
            }
            reviewed += &plan_reviewed(repo, &dir)?;
            let mut mission = dir.mission;
            mission.status = MissionStatus::Ready;
            mission.ready = Some(ready.clone());
            missions.push(mission);
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
            Ok(_) => {}
            Err(Refusal::From { status, .. }) => refused.push((id, status)),
            Err(other) => unreachable!("an item's ready has no role or conditions: {other:?}"),
        }
        let mut item = record.item.clone();
        item.status = Status::Ready;
        item.ready = Some(ready.clone());
        items.push(item);
    }
    if !unknown.is_empty() || !refused.is_empty() {
        let text = output::ready_refused(&unknown, &refused);
        let from = row(Act::Ready).from;
        let rules = (refused.iter()).map(|(id, status)| output::rule_from(id, from, *status));
        let refused = match unknown.is_empty() {
            true => skip::refuse(rules, text),
            false => Some(Outcome::Refused(text)),
        };
        if let Some(refused) = refused {
            return Ok(refused);
        }
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
    let finding = id.contains('/');
    let refused = match finding {
        true => off_settling(repo, "drop")?,
        false => off_base(repo, "drop")?,
    };
    if let Some(refused) = refused {
        return Ok(refused);
    }
    let Some(reason) = reason
        .map(str::trim)
        .filter(|r| !r.is_empty() && !r.contains(['\n', '\r']))
    else {
        return Ok(Outcome::Refused(output::no_drop_reason(id)));
    };
    if finding {
        return settle(repo, "drop", id, Settle::Drop(reason));
    }
    let line = format!("Dropped: {reason}");
    if is_mission(repo, id) {
        let dir = repo.store().mission(id)?;
        let facts = MissionFacts {
            mission: &dir,
            items: &[],
            review: None,
        };
        if let Some(refused) = mission_gate(Act::Drop, id, &facts)? {
            return Ok(refused);
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
    if let Some(refused) = refusal(Act::Drop, &facts)? {
        return Ok(refused);
    }
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
    item.status = Status::Dropped;
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
        if let Some(refused) = mission_gate(Act::CloseMission, id, &facts)? {
            return Ok(refused);
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
    if let Err(refusal) = check(Act::Close, &facts) {
        // An item with a branch ends by its merge, or by `drop`.
        let text =
            match facts.branch && !matches!(record.item.status, Status::Done | Status::Dropped) {
                true => output::close_started(id),
                false => refused_act(Act::Close, &refusal, &facts)?,
            };
        if let Some(refused) = gated(Act::Close, &facts, text) {
            return Ok(refused);
        }
    }
    let mut item = record.item.clone();
    item.status = Status::Done;
    item.body = put_first(&item.body, "Report", result);
    let said = output::closed(id, "item");
    let message = format!("Close {id}");
    save(repo, "close", vec![item], Vec::new(), &said, &message)
}

/// Refuses a planning command run anywhere but on the base branch.
pub(crate) fn off_base(repo: &Repo, command: &str) -> Result<Option<Outcome>> {
    if repo.branch.as_deref() == Some(repo.base.as_str()) {
        return Ok(None);
    }
    let checkout = git::checkout_of(&repo.root, &repo.base)?;
    let (base, branch) = (&repo.base, repo.branch.as_deref());
    let text = output::off_base(command, base, branch, checkout.as_deref());
    Ok(skip::refuse(
        [output::rule_off_base(base, branch, false)],
        text,
    ))
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

/// Refuses settling a finding anywhere but on the base branch, as
/// planning, or on the branch of a started item the caller implements,
/// whose merge then carries it to the base.
fn off_settling(repo: &mut Repo, command: &str) -> Result<Option<Outcome>> {
    if repo.branch.as_deref() == Some(repo.base.as_str()) {
        return Ok(None);
    }
    let current = repo.current()?;
    let held = current.as_ref().and_then(Located::record).is_some_and(|r| {
        STARTED.contains(&r.item.status) && r.item.implementers.holder() == Some(&repo.caller)
    });
    if held {
        return Ok(None);
    }
    let checkout = git::checkout_of(&repo.root, &repo.base)?;
    let (base, branch) = (&repo.base, repo.branch.as_deref());
    let text = output::settle_elsewhere(command, base, branch, checkout.as_deref());
    Ok(skip::refuse(
        [output::rule_off_base(base, branch, true)],
        text,
    ))
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
        if let Some(refused) = gate(Act::Settle, id, &all, text) {
            return Ok(refused);
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
    save_with(
        repo,
        &command,
        items,
        Vec::new(),
        vec![review],
        &said,
        &message,
    )
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

/// Writes `items` and `missions` into the checkout, once each validates
/// against every record of the checkout, and says what it wrote and how to
/// commit it; or, when one has a problem, writes nothing and says why.
/// `said` opens the output, and `message` is the commit's.
pub(crate) fn save(
    repo: &Repo,
    command: &str,
    items: Vec<Item>,
    missions: Vec<Mission>,
    said: &str,
    message: &str,
) -> Result<Outcome> {
    save_with(repo, command, items, missions, Vec::new(), said, message)
}

/// [`save`], with `reviews` written too. A new item it creates is removed
/// again when a later write fails, so a finding is never left untaken
/// beside the item that took it.
fn save_with(
    repo: &Repo,
    command: &str,
    items: Vec<Item>,
    missions: Vec<Mission>,
    reviews: Vec<ReviewFile>,
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
    for review in &reviews {
        let shown = review.path.strip_prefix(&repo.root).unwrap_or(&review.path);
        written.push((review.path.clone(), shown.display().to_string()));
        let found = all_items.iter_mut().flat_map(|dir| dir.reviews.iter_mut());
        if let Some(file) = found.into_iter().find(|file| file.path == review.path) {
            *file = review.clone();
        }
    }
    let problems: Vec<Problem> = validate(&all_items, &all_missions)
        .into_iter()
        .filter(|problem| written.iter().any(|(path, _)| *path == problem.file))
        .collect();
    if !problems.is_empty() {
        let text = output::invalid(command, &repo.root, &problems);
        let rules = problems
            .iter()
            .map(|problem| output::valid(&output::from_root(&repo.root, problem)));
        if let Some(refused) = skip::refuse(rules, text) {
            return Ok(refused);
        }
    }
    let created: Vec<PathBuf> = (items.iter())
        .map(|item| repo.root.join(item_dir(&item.id)))
        .filter(|dir| !dir.exists())
        .collect();
    let writes = || -> std::io::Result<()> {
        for item in &items {
            store.write_item(item)?;
        }
        for mission in &missions {
            store.write_mission(mission)?;
        }
        reviews.iter().try_for_each(ReviewFile::write)
    };
    if let Err(error) = writes() {
        let left: Vec<String> = (created.iter())
            .filter(|dir| std::fs::remove_dir_all(dir).is_err())
            .map(|dir| {
                dir.strip_prefix(&repo.root)
                    .unwrap_or(dir)
                    .display()
                    .to_string()
            })
            .collect();
        let said = output::write_failed(command, !created.is_empty(), &left);
        return Err(anyhow!(error).context(said));
    }
    let paths: Vec<String> = written.into_iter().map(|(_, shown)| shown).collect();
    let at = git_at(&repo.root);
    let item = repo.branch.as_deref().filter(|branch| *branch != repo.base);
    Ok(Outcome::Done(output::planned(
        said, item, at, &paths, message,
    )))
}
