//! What every planning command shares, and a mission's `drop`. A planning
//! command reads and writes records in the checkout it runs in, validates
//! what it writes, and commits nothing: the agent commits with Git, on the
//! branch the change belongs on, and the change reaches the base when that
//! branch merges. Nothing is written in the base's own checkout.

use crate::actions::{Action, Facts, STARTED};
use crate::model::{Item, Mission, MissionStatus, Named};
use crate::records::{put_first, validate, validate_id, ItemDir, MissionDir, Problem, ReviewFile};
use crate::repo::{is_planning, item_dir, item_path, mission_dir, mission_path};
use crate::skip::Refused;
use crate::{git, say, split_rounds, Located, Repo};
use anyhow::{anyhow, Result};
use std::path::PathBuf;

/// What an action did: the paths it wrote, the commit it made, if it made
/// one, and what it says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Done {
    pub text: String,
    pub paths: Vec<String>,
    pub commit: Option<String>,
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

/// What a planner's action on the item `located`, whose record is `record`,
/// is checked against: its record and branch, and nothing it waits for.
pub fn planned_facts<'a>(repo: &'a Repo, located: &Located, record: &'a ItemDir) -> Facts<'a> {
    Facts {
        item: record,
        branch: located.branch.is_some(),
        blockers: &[],
        mission: None,
        caller: &repo.caller,
        checkout: None,
        worktree: None,
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
    let facts = crate::actions::MissionFacts {
        mission: &dir,
        caller: &repo.caller,
        items: &[],
        review: None,
    };
    if let Some(refused) = repo.skip.mission_gate(Action::Drop, id, &facts)? {
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

/// The kind of branch a record change belongs on, by the action table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Belongs<'a> {
    /// A backlog branch, or a mission's branch for its plan.
    Backlog,
    /// Filing a new item: as `Backlog`, or the branch of any started item,
    /// whoever files it, since the new item reaches the base in its merge.
    New,
    /// Settling a finding: as `Backlog`, or the branch of a started item
    /// the caller implements.
    Settle,
    /// The branch of this started item.
    Item(&'a str),
    /// A review's checkout: as `Backlog`, or detached at the commit it
    /// reviews.
    Review,
}

/// Refuses `command`, whose change belongs on a branch of kind `belongs`,
/// where it runs: always in the base's own checkout, on a detached HEAD,
/// where only a review is written, since a commit there is on no branch,
/// and on a branch of another kind. Each is a rule. A branch the
/// convention does not name takes any change.
pub fn misplaced(repo: &mut Repo, command: &str, belongs: Belongs) -> Result<Option<Refused>> {
    let base = repo.base.clone();
    if repo.branch.as_deref() == Some(base.as_str()) {
        let text = say::on_base_checkout(command, &base);
        return Ok(repo.skip.refuse([say::rule_on_base(&base)], text));
    }
    if repo.branch.is_none() && belongs != Belongs::Review {
        let text = say::on_detached_head(command, &base);
        let rule = say::rule_on_detached_head(command);
        if let Some(refused) = repo.skip.refuse([rule], text) {
            return Ok(Some(refused));
        }
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
    if belongs == Belongs::New {
        let status = record.item.status;
        if STARTED.contains(&status) {
            return Ok(None);
        }
        // A branch that will not merge would strand what is filed on it.
        let text = say::files_on_unmerging(command, &current.id, &branch, status);
        return Ok(repo
            .skip
            .refuse([say::rule_files_on_unmerging(command, &branch)], text));
    }
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
        None,
    )
}

/// [`save`] of `item`, a new one, which on an item's branch reaches the
/// base in that item's merge and makes no approval of it stale.
pub fn save_new(
    repo: &mut Repo,
    command: &str,
    item: Item,
    said: &str,
    message: &str,
) -> Result<Result<Done, Refused>> {
    let reaches = repo.current()?.map(|on| say::Guidance::Filed(on.id));
    save_with(
        repo,
        command,
        vec![item],
        Vec::new(),
        Vec::new(),
        said,
        message,
        reaches,
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
    let reaches = Some(say::Guidance::Own);
    save_with(
        repo, command, items, missions, reviews, said, message, reaches,
    )
}

/// [`save`] without the commit, with `reviews` written too, and the Git
/// command that commits what it wrote; `reaches`, where given, is how the
/// change reaches the base, which the branch it is on says otherwise. A
/// new item it creates is removed again when a later write fails, so a
/// finding is never left untaken beside the item that took it.
#[allow(clippy::too_many_arguments)]
pub fn save_with(
    repo: &Repo,
    command: &str,
    items: Vec<Item>,
    missions: Vec<Mission>,
    reviews: Vec<ReviewFile>,
    said: &str,
    message: &str,
    reaches: Option<say::Guidance>,
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
    let by_branch = || match repo.named() {
        Some(Named::Record(id)) if repo.root.join(item_path(&id)).exists() => {
            let own = format!("{}/", item_dir(&id));
            match paths.iter().all(|path| path.starts_with(&own)) {
                true => say::Guidance::Own,
                false => say::Guidance::Settle(id),
            }
        }
        _ => say::Guidance::Branch,
    };
    let guidance = reaches.unwrap_or_else(by_branch);
    Ok(Ok(Done {
        text: say::planned(said, &guidance, at, &paths, message, &repo.skip.trailers()),
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
    let mut found = Vec::new();
    let (before, after) = (repo.mission_at(from, id)?, repo.mission_at(to, id)?);
    match (&before, &after) {
        (None, None) => {}
        (None, Some(_)) => found.push(format!("{} is added", mission_path(id))),
        (Some(_), None) => found.push(format!("{} is removed", mission_path(id))),
        (Some(before), Some(after)) => {
            let changed = changed_sections(before, after);
            found.extend(changed.into_iter().map(|heading| format!("its {heading}")));
        }
    }
    for path in git::changed(&repo.root, from, to)? {
        if let Some(rest) = path.strip_prefix(".atelier/items/") {
            let Some(item) = rest.split('/').next().filter(|_| items) else {
                continue;
            };
            if of_mission(repo, id, item, (from, to)) {
                found.push(path);
            }
            continue;
        }
        if is_plan_file(id, &path) {
            found.push(path);
        }
    }
    Ok(found)
}

/// The [`PLAN_SECTIONS`] that differ between a mission's versions.
pub(crate) fn changed_sections(before: &MissionDir, after: &MissionDir) -> Vec<&'static str> {
    let differs = |heading: &&str| {
        before.section(heading).map(str::trim) != after.section(heading).map(str::trim)
    };
    PLAN_SECTIONS.into_iter().filter(differs).collect()
}

/// Whether `path` is a file of mission `id`'s plan beside its `mission.md`:
/// any other in its directory but `notes.md` and its reviews.
pub(crate) fn is_plan_file(id: &str, path: &str) -> bool {
    let rest = path.strip_prefix(&format!("{}/", mission_dir(id)));
    rest.is_some_and(|rest| {
        rest != "mission.md" && rest != "notes.md" && !rest.starts_with("reviews/")
    })
}

/// Whether `item` is of mission `id` at the commit `from` or `to`. An item
/// whose record at either does not read is counted.
fn of_mission(repo: &mut Repo, id: &str, item: &str, (from, to): (&str, &str)) -> bool {
    [from, to].into_iter().any(|at| {
        repo.item_at(at, item).map_or(true, |dir| {
            dir.is_some_and(|d| d.item.mission.as_deref() == Some(id))
        })
    })
}

/// What a round of plan review has to check beside the findings it carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanDelta {
    /// The completed round before it, and the one commit that round reviewed.
    pub round: u32,
    pub from: String,
    /// The plan's paths changed since; none when the plan is unchanged.
    pub paths: Vec<String>,
}

/// The [`PlanDelta`] of round `round` of `mission`'s plan review at the
/// commit `head`; `None`, for a review of the whole plan, when no round
/// before it completed, or that round's reviews are not of one commit in
/// the history of `head`.
pub fn plan_delta(
    repo: &mut Repo,
    mission: &MissionDir,
    round: u32,
    head: &str,
) -> Result<Option<PlanDelta>> {
    let last = (mission.reviews.iter())
        .map(|file| file.name.round)
        .filter(|last| *last < round && completed(mission, *last))
        .max();
    let Some(last) = last else {
        return Ok(None);
    };
    let mut commits: Vec<&str> = (mission.reviews.iter())
        .filter(|file| file.name.round == last)
        .map(|file| file.review.commit.as_str())
        .collect();
    commits.dedup();
    let [from] = commits.as_slice() else {
        return Ok(None);
    };
    if !git::has_commit(&repo.root, from)? || !git::is_ancestor(&repo.root, from, head)? {
        return Ok(None);
    }
    Ok(Some(PlanDelta {
        round: last,
        from: from.to_string(),
        paths: plan_paths(repo, &mission.mission.id, (from, head))?,
    }))
}

/// Whether every review of round `round` of `mission`'s plan has a verdict.
fn completed(mission: &MissionDir, round: u32) -> bool {
    (mission.reviews.iter())
        .filter(|file| file.name.round == round)
        .all(|file| file.review.verdict.is_some())
}

/// The paths that differ between the commits `from` and `to` and are mission
/// `id`'s plan's own: its directory but its reviews and `notes.md`, and the
/// directories, but their reviews, of its items at either commit. Once it is
/// agreed, of the items only the paths the amendment itself changes against
/// the base at either commit, so what it took back is among them and what
/// the base merged is not. [`plan_changed_at`] is wider, and no diff to read.
fn plan_paths(repo: &mut Repo, id: &str, (from, to): (&str, &str)) -> Result<Vec<String>> {
    let amended = match agreed(repo, id)? {
        true => Some(git::changed_on_branch(
            &repo.root,
            &repo.base_tip,
            (from, to),
        )?),
        false => None,
    };
    let record = mission_path(id);
    let mut paths = git::changed(&repo.root, from, to)?;
    paths.retain(|path| {
        if *path == record || is_plan_file(id, path) {
            return true;
        }
        let in_item = path.strip_prefix(".atelier/items/");
        let Some((item, file)) = in_item.and_then(|rest| rest.split_once('/')) else {
            return false;
        };
        !file.starts_with("reviews/")
            && amended.as_ref().is_none_or(|paths| paths.contains(path))
            && of_mission(repo, id, item, (from, to))
    });
    Ok(paths)
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
    pub name: crate::model::ReviewName,
    pub by: Option<crate::model::Thread>,
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
    /// Whether the plan change under review has a review of its own: one at
    /// `head` that is not on the base.
    pub reviewed: bool,
    /// How many of that change's latest rounds split, in a row.
    pub splits: usize,
}

/// How many approving plan reviews a plan needs to merge.
pub const APPROVALS: usize = 2;

/// How many split rounds in a row end the review of a plan change, for the
/// human to rule.
pub const SPLITS: usize = 2;

impl Approvals {
    pub fn counted(&self) -> usize {
        self.reviews.iter().filter(|r| r.not.is_none()).count()
    }

    pub fn enough(&self) -> bool {
        self.counted() >= APPROVALS
    }

    /// Whether review of the plan change stops here for the human to rule.
    pub fn stopped(&self) -> bool {
        self.splits >= SPLITS && !self.enough()
    }
}

/// How many rounds at the end of `change`'s reviews split in a row: each
/// completed, with an approval and a request for changes.
fn trailing_splits(change: &MissionDir) -> usize {
    let split: Vec<u32> = (split_rounds(change).into_iter())
        .map(|split| split.round)
        .collect();
    let mut rounds: Vec<u32> = change.reviews.iter().map(|r| r.name.round).collect();
    rounds.sort_unstable_by(|a, b| b.cmp(a));
    rounds.dedup();
    (rounds.into_iter())
        .take_while(|round| split.contains(round) && completed(change, *round))
        .count()
}

/// The [`Approvals`] of mission `id`'s plan at the commit `head`.
pub fn approvals(repo: &mut Repo, id: &str, head: &str, on_base: bool) -> Result<Approvals> {
    let Some(mission) = repo.mission_at(head, id)? else {
        return Ok(Approvals {
            id: id.to_string(),
            round: 0,
            reviews: Vec::new(),
            reviewed: false,
            splits: 0,
        });
    };
    // The change's own reviews, by name: an amendment made beside this one
    // may have merged a round of the same number.
    let base = repo.base_tip.clone();
    let on_base_tip = repo
        .mission_at(&base, id)?
        .map_or(vec![], |dir| dir.reviews);
    let mut change = mission.clone();
    (change.reviews).retain(|file| !on_base_tip.iter().any(|base| base.name == file.name));
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
            Some(crate::model::Verdict::Changes) => Some(NotCounted::Changes),
            Some(crate::model::Verdict::Approve) => {
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
        reviewed: !change.reviews.is_empty(),
        splits: trailing_splits(&change),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::records::{parse_mission, parse_review};
    use std::path::Path;

    /// A mission with a plan review for each of `reviews`: its name and its
    /// verdict.
    fn mission(reviews: &[(&str, &str)]) -> MissionDir {
        let path = Path::new("mission.md");
        let text = "---\nschema: atelier.mission\nschema_version: 3\nid: demo-m1ss\ntitle: Greet\n\
                    status: open\nlabels: []\nlead: null\ncreated_at: 2026-11-22T09:00:00Z\n---\n";
        let reviews = reviews.iter().map(|(name, verdict)| {
            let name: crate::model::ReviewName = name.parse().unwrap();
            let text = format!(
                "---\nschema: atelier.review\nschema_version: 3\nround: {}\ncommit: abc1234\n\
                 reviewers:\n- r1\nmodel: null\nverdict: {verdict}\n\
                 created_at: 2026-11-22T09:00:00Z\n---\n",
                name.round
            );
            ReviewFile {
                path: path.to_path_buf(),
                review: parse_review(path, &text).unwrap(),
                name,
            }
        });
        MissionDir {
            path: path.to_path_buf(),
            mission: parse_mission(path, text).unwrap(),
            reviews: reviews.collect(),
        }
    }

    #[test]
    fn only_completed_split_rounds_at_the_end_count_toward_the_exit() {
        let (yes, no, open) = ("approve", "changes", "null");
        type Case<'a> = (&'a [(&'a str, &'a str)], usize);
        let cases: [Case; 7] = [
            (&[], 0),
            (&[("1-aaaa", yes), ("1-bbbb", no)], 1),
            (&[("1-aaaa", no), ("1-bbbb", no)], 0),
            (
                &[
                    ("1-aaaa", yes),
                    ("1-bbbb", no),
                    ("2-cccc", no),
                    ("2-dddd", yes),
                ],
                2,
            ),
            // A round that did not split breaks the row.
            (
                &[
                    ("1-aaaa", yes),
                    ("1-bbbb", no),
                    ("2-cccc", no),
                    ("3-dddd", yes),
                    ("3-eeee", no),
                ],
                1,
            ),
            // A round not over has not split.
            (&[("1-aaaa", yes), ("1-bbbb", no), ("2-cccc", open)], 0),
            (&[("1-aaaa", yes), ("1-bbbb", no), ("1-cccc", open)], 0),
        ];
        for (reviews, splits) in cases {
            assert_eq!(trailing_splits(&mission(reviews)), splits, "{reviews:?}");
        }
    }
}
