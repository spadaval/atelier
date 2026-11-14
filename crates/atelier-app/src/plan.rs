//! `ready`, and what every planning command shares. A planning command
//! reads and writes records in the checkout it runs in, validates what it
//! writes, and commits nothing: the agent commits with Git, on the branch
//! the change belongs on, and the change reaches the base when that branch
//! merges. Nothing is written in the base's own checkout.

use crate::acts::{check, row, Act, Facts, Refusal, STARTED};
use crate::repo::{is_planning, item_dir, item_path, mission_dir, mission_path};
use crate::skip::Refused;
use crate::{git, say, Located, Repo};
use anyhow::{anyhow, Result};
use atelier_core::{Item, Mission, MissionStatus, Named, Ready, Status};
use atelier_records::{put_first, validate, validate_id, ItemDir, MissionDir, Problem, ReviewFile};
use std::path::PathBuf;

/// What an act did: the paths it wrote, the commit it made, if it made
/// one, and what it says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Done {
    pub text: String,
    pub paths: Vec<String>,
    pub commit: Option<String>,
}

/// `ready`: marks each of `ids`, a draft item in this checkout, ready, as
/// `by` said, and writes it without committing; or refuses, writing
/// nothing, when one is unknown, not a draft, or a mission, which needs no
/// go-ahead.
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
    let mut named: Vec<&str> = Vec::new();
    for id in ids.iter().map(String::as_str) {
        if named.contains(&id) {
            continue;
        }
        named.push(id);
        if is_mission(repo, id) {
            missions.push(id);
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
    if !unknown.is_empty() || !refused.is_empty() || !missions.is_empty() {
        let text = say::ready_refused(&unknown, &refused, &missions);
        let from = row(Act::Ready).from;
        let rules = (refused.iter()).map(|(id, status)| say::rule_from(id, from, *status));
        let refused = match unknown.is_empty() && missions.is_empty() {
            true => repo.skip.refuse(rules, text),
            false => Some(text),
        };
        if let Some(refused) = refused {
            return Ok(Err(refused));
        }
    }
    let said = say::readied(&named, by);
    let message = format!("Ready {}", named.join(", "));
    save(repo, "ready", items, Vec::new(), &said, &message)
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

/// Whether mission `id` is agreed: the base's last commit has it, and not
/// as an old `draft`. A mission only on a branch is proposed.
pub fn agreed(repo: &mut Repo, id: &str) -> Result<bool> {
    let base = repo.base_tip.clone();
    let found = repo.mission_at(&base, id)?;
    Ok(found.is_some_and(|dir| dir.mission.status != MissionStatus::Draft))
}

/// `drop` of mission `id`, in this checkout, with `reason`, a line the
/// caller has checked is not empty: writes it dropped, with `Dropped:
/// <reason>` first in its Result, and commits nothing. An agreed mission
/// is dropped on a backlog branch and the stop counts once that merges; a
/// proposed plan on its own branch, and the stop counts once the branch is
/// pushed and its pull request closed. The `atelier drop` command and the
/// Desk's stop both write a stop through this.
pub fn drop_mission(repo: &mut Repo, id: &str, reason: &str) -> Result<Result<Done, Refused>> {
    if let Some(refused) = misplaced(repo, "drop", Belongs::Backlog)? {
        return Ok(Err(refused));
    }
    let dir = repo.store().mission(id)?;
    let facts = crate::acts::MissionFacts {
        mission: &dir,
        caller: &repo.caller,
        items: &[],
        review: None,
    };
    if let Some(refused) = repo.skip.mission_gate(Act::Drop, id, &facts)? {
        return Ok(Err(refused));
    }
    let mut mission = dir.mission;
    mission.status = MissionStatus::Dropped;
    mission.body = put_first(&mission.body, "Result", &format!("Dropped: {reason}"));
    let branch = repo.branch.clone().unwrap_or_default();
    let message = format!("Drop {id}");
    if agreed(repo, id)? {
        let said = say::dropped_mission(id, &branch, reason);
        return save(repo, "drop", Vec::new(), vec![mission], &said, &message);
    }
    let said = say::dropped_plan(id, &branch, reason, repo.remote());
    save_own(repo, "drop", vec![mission], &said, &message)
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
    save_with(
        repo,
        command,
        items,
        missions,
        Vec::new(),
        said,
        message,
        false,
    )
}

/// [`save`] of a record change whose `said` already says how it reaches
/// the base, as the drop of a proposed plan, which is pushed and never
/// merged.
pub fn save_own(
    repo: &Repo,
    command: &str,
    missions: Vec<Mission>,
    said: &str,
    message: &str,
) -> Result<Result<Done, Refused>> {
    let (items, reviews) = (Vec::new(), Vec::new());
    save_with(repo, command, items, missions, reviews, said, message, true)
}

/// [`save`] without the commit, with `reviews` written too, and the Git
/// command that commits what it wrote; with `own`, `said` says how the
/// change reaches the base. A new item it creates is removed again when a
/// later write fails, so a finding is never left untaken beside the item
/// that took it.
#[allow(clippy::too_many_arguments)]
pub fn save_with(
    repo: &Repo,
    command: &str,
    items: Vec<Item>,
    missions: Vec<Mission>,
    reviews: Vec<ReviewFile>,
    said: &str,
    message: &str,
    own: bool,
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
    // On an item's branch, a write outside its own directory, as settling
    // another item's finding, reaches the base in the item's merge; one
    // inside it, as its own drop, is the item's.
    let item = match repo.named() {
        _ if own => say::Guidance::Own,
        Some(Named::Record(id)) if repo.root.join(item_path(&id)).exists() => {
            let own = format!("{}/", item_dir(&id));
            match paths.iter().all(|path| path.starts_with(&own)) {
                true => say::Guidance::Own,
                false => say::Guidance::Settle(id),
            }
        }
        _ => say::Guidance::Branch,
    };
    Ok(Ok(Done {
        text: say::planned(said, &item, at, &paths, message),
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

/// The sections of `mission.md` that are part of a mission's plan. Its
/// Result and its front matter are not.
pub const PLAN_SECTIONS: [&str; 5] = ["Why", "Outcome", "Success criteria", "Scope", "Authority"];

/// What of mission `id`'s plan differs between the commits `from` and
/// `to`: each section of `mission.md` in [`PLAN_SECTIONS`], each other
/// file of its directory but `notes.md` and its reviews, and, with
/// `items`, as while the mission is not on the base, each path in the
/// directory of an item of the mission at either commit. Each is named as
/// a reader is told it: "its Why", or the path.
pub fn plan_diff(
    repo: &mut Repo,
    id: &str,
    (from, to): (&str, &str),
    items: bool,
) -> Result<Vec<String>> {
    let own = format!("{}/", mission_dir(id));
    let mut found = Vec::new();
    let (before, after) = (repo.mission_at(from, id)?, repo.mission_at(to, id)?);
    match (&before, &after) {
        (None, None) => {}
        (None, Some(_)) => found.push(format!("{} is added", mission_path(id))),
        (Some(_), None) => found.push(format!("{} is removed", mission_path(id))),
        (Some(before), Some(after)) => {
            for heading in PLAN_SECTIONS {
                let text =
                    |dir: &MissionDir| dir.section(heading).map(str::trim).map(str::to_string);
                if text(before) != text(after) {
                    found.push(format!("its {heading}"));
                }
            }
        }
    }
    let notes = format!("{own}notes.md");
    let reviews = format!("{own}reviews/");
    for path in git::changed(&repo.root, from, to)? {
        if let Some(rest) = path.strip_prefix(".atelier/items/") {
            let Some(item) = rest.split('/').next().filter(|_| items) else {
                continue;
            };
            // An item whose record at either commit does not read is
            // counted.
            let mut of_mission = |at: &str| {
                repo.item_at(at, item).map_or(true, |dir| {
                    dir.is_some_and(|d| d.item.mission.as_deref() == Some(id))
                })
            };
            if of_mission(from) || of_mission(to) {
                found.push(path);
            }
            continue;
        }
        let other = path.starts_with(&own)
            && path != mission_path(id)
            && path != notes
            && !path.starts_with(&reviews);
        if other {
            found.push(path);
        }
    }
    Ok(found)
}

/// Why a plan review of the current round does not count toward a merge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotCounted {
    NoVerdict,
    Changes,
    /// Its reviewer is one of the mission's planners.
    Planner,
    /// Its reviewer has an approval of the round counted already.
    SameReviewer,
    /// The commit it reviewed is not in this clone.
    Unknown,
    /// The plan changed since the commit it reviewed: these.
    Stale(Vec<String>),
}

/// One review of a plan's current round, as `merge` counts it.
#[derive(Debug, Clone)]
pub struct Counted {
    pub name: atelier_core::ReviewName,
    pub by: Option<atelier_core::Thread>,
    pub commit: String,
    /// `None` when it counts.
    pub not: Option<NotCounted>,
}

/// The approvals a merge of mission `id`'s plan at `head` has: each review
/// of the current round, its latest, at `head`, and whether it counts. One
/// counts when it approves, its reviewer is not a planner of the mission
/// as `head` holds it nor counted already, and the plan as its commit
/// holds it is the plan at `head`, items included when `on_base` is false.
pub struct Approvals {
    pub id: String,
    /// The current round; 0 when the plan has no review.
    pub round: u32,
    pub reviews: Vec<Counted>,
}

/// How many approving plan reviews a plan needs to merge.
pub const APPROVALS: usize = 2;

impl Approvals {
    pub fn counted(&self) -> usize {
        self.reviews.iter().filter(|r| r.not.is_none()).count()
    }

    pub fn enough(&self) -> bool {
        self.counted() >= APPROVALS
    }
}

/// The [`Approvals`] of mission `id`'s plan at the commit `head`.
pub fn approvals(repo: &mut Repo, id: &str, head: &str, on_base: bool) -> Result<Approvals> {
    let Some(mission) = repo.mission_at(head, id)? else {
        return Ok(Approvals {
            id: id.to_string(),
            round: 0,
            reviews: Vec::new(),
        });
    };
    let round = mission
        .reviews
        .iter()
        .map(|r| r.name.round)
        .max()
        .unwrap_or(0);
    let planners = &mission.mission.planners;
    let mut reviews: Vec<Counted> = Vec::new();
    for file in mission.reviews.iter().filter(|r| r.name.round == round) {
        let by = file.review.reviewers.holder().cloned();
        let commit = file.review.commit.clone();
        let not = match file.review.verdict {
            None => Some(NotCounted::NoVerdict),
            Some(atelier_core::Verdict::Changes) => Some(NotCounted::Changes),
            Some(atelier_core::Verdict::Approve) => {
                let counted = |r: &Counted| r.not.is_none() && r.by == by;
                if file
                    .review
                    .reviewers
                    .0
                    .iter()
                    .any(|t| planners.0.contains(t))
                {
                    Some(NotCounted::Planner)
                } else if reviews.iter().any(counted) {
                    Some(NotCounted::SameReviewer)
                } else if !git::has_commit(&repo.root, &commit)? {
                    Some(NotCounted::Unknown)
                } else {
                    let changed = plan_diff(repo, id, (&commit, head), !on_base)?;
                    (!changed.is_empty()).then_some(NotCounted::Stale(changed))
                }
            }
        };
        reviews.push(Counted {
            name: file.name.clone(),
            by,
            commit,
            not,
        });
    }
    Ok(Approvals {
        id: id.to_string(),
        round,
        reviews,
    })
}
