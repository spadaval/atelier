//! `ready`, and what every planning command shares. A planning command
//! reads and writes records in the checkout it runs in, validates what it
//! writes, and commits nothing: the agent commits with Git, on the branch
//! the change belongs on, and the change reaches the base when that branch
//! merges. Nothing is written in the base's own checkout.

use crate::acts::{check, check_mission, row, Act, Facts, MissionFacts, Refusal, STARTED};
use crate::repo::{is_planning, item_dir, item_path, mission_dir, mission_path};
use crate::skip::Refused;
use crate::{git, say, Located, Repo};
use anyhow::{anyhow, Result};
use atelier_core::{Item, Mission, MissionStatus, Ready, Status};
use atelier_records::{validate, validate_id, ItemDir, MissionDir, Problem, ReviewFile};
use std::path::PathBuf;

/// What an act did: the paths it wrote, the commit it made, if it made
/// one, and what it says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Done {
    pub text: String,
    pub paths: Vec<String>,
    pub commit: Option<String>,
}

/// `ready`: marks each of `ids`, a draft item or mission in this checkout,
/// ready, as `by` said, and writes it without committing; or refuses,
/// writing nothing, when one is unknown or not a draft.
pub fn ready(repo: &mut Repo, ids: &[String], by: &str) -> Result<Result<Done, Refused>> {
    if let Some(refused) = misplaced(repo, "ready", Belongs::Backlog)? {
        return Ok(Err(refused));
    }
    if ids.is_empty() {
        return Ok(Err(say::ready_nothing()));
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
        let text = say::ready_refused(&unknown, &refused);
        let from = row(Act::Ready).from;
        let rules = (refused.iter()).map(|(id, status)| say::rule_from(id, from, *status));
        let refused = match unknown.is_empty() {
            true => repo.skip.refuse(rules, text),
            false => Some(text),
        };
        if let Some(refused) = refused {
            return Ok(Err(refused));
        }
    }
    let said = say::readied(&named, by) + &reviewed;
    let message = format!("Ready {}", named.join(", "));
    save(repo, "ready", items, missions, &said, &message)
}

/// What `ready` says of a mission's plan reviews, for whoever says go:
/// each review of the latest round with its verdict, and what in the plan
/// changed since the commit it reviewed. None of it refuses anything.
fn plan_reviewed(repo: &mut Repo, dir: &MissionDir) -> Result<String> {
    let id = &dir.mission.id;
    let Some(round) = dir.reviews.iter().map(|r| r.name.round).max() else {
        return Ok(say::no_plan_review(id));
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
    Ok(say::plan_reviews(id, round, &latest, &changed))
}

/// The paths of mission `id`'s plan, its directory less its reviews and
/// the directories of its items then or now, that differ in this checkout
/// from `commit`; `None` when this clone lacks that commit.
pub fn plan_changed(repo: &mut Repo, id: &str, commit: &str) -> Result<Option<Vec<String>>> {
    if !git::has_commit(&repo.root, commit)? {
        return Ok(None);
    }
    let dirs = [".atelier/missions", ".atelier/items"];
    let paths = git::changed_since(&repo.root, commit, &dirs)?;
    // An item whose record here does not read is judged by the commit.
    let store = repo.store();
    let now = |_: &mut Repo, item: &str| {
        store
            .item(item)
            .is_ok_and(|dir| dir.item.mission.as_deref() == Some(id))
    };
    in_plan(repo, id, commit, paths, now).map(Some)
}

/// [`plan_changed`] read from commits only: the paths of mission `id`'s
/// plan that differ between the commits `commit` and `tip`, with the
/// items of the mission now as `tip` holds them; `None` when this clone
/// lacks `commit`.
pub fn plan_changed_at(
    repo: &mut Repo,
    id: &str,
    commit: &str,
    tip: &str,
) -> Result<Option<Vec<String>>> {
    if !git::has_commit(&repo.root, commit)? {
        return Ok(None);
    }
    let paths = git::changed(&repo.root, commit, tip)?;
    let paths = paths.into_iter().filter(|path| is_planning(path)).collect();
    // An item whose record at `tip` does not read is judged by `commit`.
    let now = |repo: &mut Repo, item: &str| {
        repo.item_at(tip, item)
            .is_ok_and(|dir| dir.is_some_and(|d| d.item.mission.as_deref() == Some(id)))
    };
    in_plan(repo, id, commit, paths, now).map(Some)
}

/// Those of `paths`, changed since `commit`, that are in mission `id`'s
/// plan: in its directory but not its reviews, or in the directory of an
/// item of the mission now, as `now` says, or at `commit`.
fn in_plan(
    repo: &mut Repo,
    id: &str,
    commit: &str,
    paths: Vec<String>,
    mut now: impl FnMut(&mut Repo, &str) -> bool,
) -> Result<Vec<String>> {
    let own = format!("{}/", mission_dir(id));
    let reviews = format!("{own}reviews/");
    let mut changed = Vec::new();
    for path in paths {
        let item = path
            .strip_prefix(".atelier/items/")
            .and_then(|rest| rest.split('/').next());
        let in_plan = match item {
            None => path.starts_with(&own) && !path.starts_with(&reviews),
            // An item that left the mission since, or was deleted, counts;
            // one whose record then does not read is counted too.
            Some(item) => {
                now(repo, item)
                    || repo.item_at(commit, item).map_or(true, |then| {
                        then.is_some_and(|d| d.item.mission.as_deref() == Some(id))
                    })
            }
        };
        if in_plan {
            changed.push(path);
        }
    }
    Ok(changed)
}

/// What a planner's act on the item `located`, whose record is `record`,
/// is checked against: its record and branch, and nothing it waits for.
pub fn planned_facts<'a>(repo: &'a Repo, located: &Located, record: &'a ItemDir) -> Facts<'a> {
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
pub fn is_mission(repo: &Repo, id: &str) -> bool {
    validate_id(id).is_ok() && repo.root.join(mission_path(id)).exists()
}

/// The kind of branch a record change belongs on, by the act table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Belongs<'a> {
    /// A backlog branch, or a mission's branch for its plan.
    Backlog,
    /// Settling a finding: as `Backlog`, or the branch of a started item
    /// the caller implements.
    Settle,
    /// The branch of this started item.
    Item(&'a str),
}

/// Refuses `command`, whose change belongs on a branch of kind `belongs`,
/// where it runs: always in the base's own checkout, and on a branch of
/// another kind. Each is a rule. A detached checkout, or a branch the
/// convention does not name, takes any change, as a reviewer's checkout
/// at a commit does.
pub fn misplaced(repo: &mut Repo, command: &str, belongs: Belongs) -> Result<Option<Refused>> {
    let base = repo.base.clone();
    if repo.branch.as_deref() == Some(base.as_str()) {
        let text = say::on_base_checkout(command, &base);
        return Ok(repo.skip.refuse([say::rule_on_base(&base)], text));
    }
    if let Belongs::Item(id) = belongs {
        let branch = repo.branch_of(id);
        if repo.branch.as_deref() == Some(branch.as_str()) {
            return Ok(None);
        }
        let checkout = git::checkout_of(&repo.root, &branch)?;
        let text = say::drop_off_branch(id, &branch, repo.branch.as_deref(), checkout.as_deref());
        return Ok(repo
            .skip
            .refuse([say::rule_drop_off_branch(id, &branch)], text));
    }
    let Some(current) = repo.current()? else {
        return Ok(None);
    };
    let record = current.record().expect("the current item has a record");
    let held = STARTED.contains(&record.item.status)
        && record.item.implementers.holder() == Some(&repo.caller);
    if belongs == Belongs::Settle && held {
        return Ok(None);
    }
    let branch = repo.branch.clone().unwrap_or_default();
    let text = say::belongs_on_backlog(command, &current.id, &branch);
    Ok(repo
        .skip
        .refuse([say::rule_belongs_on_backlog(command, &branch)], text))
}

/// Writes `items` and `missions` into the checkout, once each validates
/// against every record of the checkout, and says what it wrote and the
/// Git command that commits it; or, when one has a problem, writes nothing
/// and says why. `said` opens what it says, and `message` is the commit's
/// the Git command gives.
pub fn save(
    repo: &Repo,
    command: &str,
    items: Vec<Item>,
    missions: Vec<Mission>,
    said: &str,
    message: &str,
) -> Result<Result<Done, Refused>> {
    save_with(repo, command, items, missions, Vec::new(), said, message)
}

/// [`save`] without the commit, with `reviews` written too, and the Git
/// command that commits what it wrote. A new item it creates is removed
/// again when a later write fails, so a finding is never left untaken
/// beside the item that took it.
pub fn save_with(
    repo: &Repo,
    command: &str,
    items: Vec<Item>,
    missions: Vec<Mission>,
    reviews: Vec<ReviewFile>,
    said: &str,
    message: &str,
) -> Result<Result<Done, Refused>> {
    let paths = match checked(repo, command, &items, &missions, &reviews)? {
        Ok(paths) => paths,
        Err(refused) => return Ok(Err(refused)),
    };
    let store = repo.store();
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
        let said = say::write_failed(command, !created.is_empty(), &left);
        return Err(anyhow!(error).context(said));
    }
    let at = repo.git_at();
    let item = repo.branch.as_deref().filter(|branch| *branch != repo.base);
    Ok(Ok(Done {
        text: say::planned(said, item, at, &paths, message),
        paths,
        commit: None,
    }))
}

/// The paths, from the root, that `command` would write `items`,
/// `missions` and `reviews` to, once each validates against every record
/// of the checkout; or, when one has a problem, the refusal saying why.
fn checked(
    repo: &Repo,
    command: &str,
    items: &[Item],
    missions: &[Mission],
    reviews: &[ReviewFile],
) -> Result<Result<Vec<String>, Refused>> {
    let store = repo.store();
    let mut all_items = store.items()?;
    let mut all_missions = store.missions()?;
    let mut written = Vec::new();
    for item in items {
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
    for mission in missions {
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
    for review in reviews {
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
        let text = say::invalid(command, &repo.root, &problems);
        let rules = problems
            .iter()
            .map(|problem| say::valid(&say::from_root(&repo.root, problem)));
        if let Some(refused) = repo.skip.refuse(rules, text) {
            return Ok(Err(refused));
        }
    }
    Ok(Ok(written.into_iter().map(|(_, shown)| shown).collect()))
}
