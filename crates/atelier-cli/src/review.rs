//! `review` and `verdict`: the reviewer's side of the loop, on an item
//! and on a mission's plan.

use crate::commands::Outcome;
use crate::forge::Github;
use crate::output::{self, Brief, Landing, PlanBrief};
use crate::{git, merge};
use anyhow::{anyhow, Context, Result};
use atelier_core::actions::{check, plan_round, row, Action, Facts, MissionFacts, Refusal};
use atelier_core::model::{
    Forge, Review, ReviewName, ReviewSchema, SchemaVersion, Thread, Threads, Verdict,
};
use atelier_core::plan::{misplaced, plan_changed, Belongs};
use atelier_core::records::{create_review, validate_id, ReviewFile};
use atelier_core::repo::{facts, item_dir, item_path, mission_dir, mission_path};
use atelier_core::{carried, carried_plan, refused, Repo};

/// Opens this round's review file, or takes it over, and prints the brief.
/// The file is committed by `verdict`, not here.
pub fn review(repo: &mut Repo, id: &str) -> Result<Outcome> {
    validate_id(id).map_err(|message| anyhow!(message))?;
    if repo.root.join(mission_path(id)).exists() {
        return plan_review(repo, id);
    }
    let located = repo.locate(id)?;
    let Some(record) = located.record() else {
        return Ok(output::no_item(id, &repo.base).into());
    };
    if let Some(refused) = behind(repo, &located) {
        return Ok(refused);
    }
    let links = repo.links(record)?;
    let forge = repo.base_config()?.forge;
    let facts = facts(repo, &located, &links, None);
    let text = match check(Action::Review, &facts) {
        Ok(_) => None,
        // Its one condition: the caller never implemented the item.
        Err(Refusal::Unmet(_)) => {
            let text =
                output::author(id, &repo.caller) + &brief(repo, &facts, None, forge.as_ref())?;
            Some(text.into())
        }
        Err(refusal) => Some(refused(Action::Review, &refusal, &facts)?),
    };
    if let Some(refused) = text.and_then(|text| repo.skip.gated(Action::Review, &facts, text)) {
        return Ok(refused.into());
    }
    if repo.branch.as_deref() != Some(repo.branch_of(id).as_str()) {
        let checkout = git::checkout_of(&repo.root, &repo.branch_of(id))?;
        let branch = repo.branch.as_deref();
        let refused = output::elsewhere("review", id, branch, checkout.as_deref());
        return Ok(refused.into());
    }
    let round = facts.round();
    let (said, name) = open(repo, id, &item_dir(id), round.review, || {
        let carried = carried(facts.item, round.number)?;
        Ok(new_review(
            &repo.caller,
            round.number,
            tip(repo, id)?,
            &carried,
            true,
        ))
    })?;
    // The brief reads the file as it now stands.
    let located = repo.locate(id)?;
    let facts = self::facts(repo, &located, &links, None);
    let path = format!("{}/reviews/{name}.md", item_dir(id));
    Ok(Outcome::Done(
        said + &brief(repo, &facts, Some(path), forge.as_ref())?,
    ))
}

/// The caller's review file in the record directory `dir` of `id`: `file`,
/// the round's, which it holds or takes over, or else the new review that
/// `new` makes. Says what it took over.
fn open(
    repo: &Repo,
    id: &str,
    dir: &str,
    file: Option<&ReviewFile>,
    new: impl FnOnce() -> Result<Review>,
) -> Result<(String, ReviewName)> {
    let Some(file) = file else {
        let review = new()?;
        let name = create_review(&repo.root.join(dir), &review);
        return Ok((String::new(), name.context("cannot write the review file")?));
    };
    let before = file.review.reviewers.holder().cloned();
    if before.as_ref() == Some(&repo.caller) {
        return Ok((String::new(), file.name.clone()));
    }
    let mut taken = file.clone();
    taken.review.reviewers.take(repo.caller.clone());
    taken.write()?;
    let said = output::review_taken_over(id, file.name.round, before.as_ref());
    Ok((said, file.name.clone()))
}

/// A new review of `commit` in `round` held by `caller`, carrying `carried`.
fn new_review(
    caller: &Thread,
    round: u32,
    commit: String,
    carried: &[atelier_core::Carried],
    item: bool,
) -> Review {
    Review {
        schema: ReviewSchema::Review,
        schema_version: SchemaVersion,
        round,
        commit,
        reviewers: Threads(vec![caller.clone()]),
        model: None,
        verdict: None,
        created_at: chrono::Utc::now().into(),
        body: output::template(carried, item),
    }
}

/// A review write refused while the forge's copy of the item's branch is
/// ahead of this clone's, as after a fetch of another reviewer's push: the
/// files here are not the record read, so pull first.
fn behind(repo: &Repo, located: &atelier_core::Located) -> Option<Outcome> {
    let branch = located.branch.as_ref().filter(|branch| branch.remote)?;
    let remote = repo.remote().unwrap_or("the remote");
    let name = repo.branch_of(&located.id);
    let text = output::behind(&located.id, &name, remote, &branch.tip);
    Some(Outcome::Refused(text))
}

/// `command` refused in the base's own checkout, where nothing is written.
fn on_base(repo: &mut Repo, command: &str) -> Result<Option<Outcome>> {
    Ok(misplaced(repo, command, Belongs::Backlog)?.map(Outcome::from))
}

/// The sections of a mission's record, in order.
const MISSION_SECTIONS: [&str; 6] = [
    "Why",
    "Outcome",
    "Success criteria",
    "Scope",
    "Authority",
    "Result",
];

/// `review` on a mission: opens the caller's review of its plan as this
/// checkout holds it at HEAD, or takes over one with no verdict, and prints
/// the plan's brief, read from the commit the review records. It runs in
/// any checkout, so two reviewers can each work blind in a detached
/// worktree of their own, and commits nothing.
fn plan_review(repo: &mut Repo, id: &str) -> Result<Outcome> {
    // There `verdict` records the item's review, not this one.
    if repo.current()?.is_some() {
        let refused = output::plan_review_on_item(id, repo.branch.as_deref(), &repo.base);
        return Ok(Outcome::Refused(refused));
    }
    if let Some(refused) = on_base(repo, "review")? {
        return Ok(refused);
    }
    let store = repo.store();
    let mission = store.mission(id)?;
    // The plan's reviews are what let it merge, so no planner gives one.
    if mission.mission.planners.0.contains(&repo.caller) {
        let text = output::planner_reviews(id, &repo.caller);
        if let Some(refused) = repo.skip.refuse([output::rule_planner_reviews(id)], text) {
            return Ok(refused.into());
        }
    }
    let round = plan_round(&mission, &repo.caller);
    let commit = git::head(&repo.root)?;
    // The plan as the commit the review records holds it: this checkout's
    // HEAD for a new review.
    let reviewed = round.review.map_or(&commit, |file| &file.review.commit);
    let Some(plan) = repo.mission_at(reviewed, id)? else {
        let line = output::commit_line(repo.git_at(), &[mission_dir(id)], &format!("Plan {id}"));
        return Ok(Outcome::Refused(output::plan_not_committed(
            id, reviewed, &line,
        )));
    };
    let (said, name) = open(repo, id, &mission_dir(id), round.review, || {
        let carried = carried_plan(&mission, round.number)?;
        Ok(new_review(
            &repo.caller,
            round.number,
            commit.clone(),
            &carried,
            false,
        ))
    })?;
    // The brief reads the file as it now stands.
    let mission = store.mission(id)?;
    let file = mission
        .reviews
        .iter()
        .find(|r| r.name == name)
        .expect("it was just written");
    let commit = &file.review.commit;
    let documents = git::files_at(&repo.root, commit, &mission_dir(id))?
        .into_iter()
        .filter(|path| {
            *path != mission_path(id) && !path.starts_with(&format!("{}/reviews/", mission_dir(id)))
        })
        .collect();
    let items = repo
        .items_at(commit)?
        .into_iter()
        .filter(|dir| dir.item.mission.as_deref() == Some(id))
        .map(|dir| (item_path(&dir.item.id), dir.item.status, dir.item.title))
        .collect();
    let sections = MISSION_SECTIONS
        .into_iter()
        .filter_map(|heading| {
            Some((
                heading,
                plan.section(heading).filter(|c| !c.trim().is_empty())?,
            ))
        })
        .collect();
    let changed = plan_changed(repo, id, commit)?.unwrap_or_default();
    let landing = landing(repo, id)?;
    let brief = output::plan_brief(&PlanBrief {
        id,
        title: &plan.mission.title,
        caller: &repo.caller,
        round: file.name.round,
        sections,
        commit,
        documents,
        items,
        changed,
        carried: &carried_plan(&mission, file.name.round)?,
        path: &format!("{}/reviews/{name}.md", mission_dir(id)),
        landing: &landing,
    });
    Ok(Outcome::Done(said + &brief))
}

/// `verdict` on the plan review the caller holds in this checkout, which
/// has no item's branch: records it in the file, without committing it,
/// changing no status.
fn plan_verdict(repo: &mut Repo, verdict: Verdict, model: Option<String>) -> Result<Outcome> {
    if let Some(refused) = on_base(repo, "verdict")? {
        return Ok(refused);
    }
    let store = repo.store();
    let mut held = Vec::new();
    for id in store.ids("missions")? {
        let mission = store.mission(&id)?;
        let round = plan_round(&mission, &repo.caller);
        let mine = round
            .review
            .filter(|f| f.review.reviewers.holder() == Some(&repo.caller));
        if let Some(name) = mine.map(|file| file.name.clone()) {
            held.push((mission, name));
        }
    }
    // A verdict records one review: none held, or several, is refused.
    if held.len() != 1 {
        let paths: Vec<String> = held
            .iter()
            .map(|(m, name)| format!("{}/reviews/{name}.md", mission_dir(&m.mission.id)))
            .collect();
        let branch = repo.branch.as_deref();
        return Ok(Outcome::Refused(output::no_review_held(branch, &paths)));
    }
    let (mission, name) = held.remove(0);
    let id = mission.mission.id.clone();
    let file = mission
        .reviews
        .iter()
        .find(|r| r.name == name)
        .expect("it was found");
    let action = match verdict {
        Verdict::Approve => Action::PlanVerdictApprove,
        Verdict::Changes => Action::PlanVerdictChanges,
    };
    let facts = MissionFacts {
        mission: &mission,
        caller: &repo.caller,
        items: &[],
        review: Some(file),
    };
    if let Some(refused) = repo.skip.mission_gate(action, &id, &facts)? {
        return Ok(refused.into());
    }
    let mut review = file.review.clone();
    review.verdict = Some(verdict);
    if let Some(model) = model.filter(|model| !model.trim().is_empty()) {
        review.model = Some(model);
    }
    let path = format!("{}/reviews/{name}.md", mission_dir(&id));
    let message = format!(
        "Review the plan of {id}, round {}: {}",
        name.round,
        output::verdict_name(verdict)
    );
    let mut written = file.clone();
    written.review = review;
    written.write().context("cannot write the review file")?;
    let line = output::commit_line(repo.git_at(), std::slice::from_ref(&path), &message);
    let landing = landing(repo, &id)?;
    Ok(Outcome::Done(output::plan_verdict_recorded(
        &id, name.round, verdict, &path, &line, &landing,
    )))
}

/// Where a review of mission `id`'s plan goes once committed: its branch,
/// while it is proposed, on the forge's remote or in this clone.
fn landing(repo: &mut Repo, id: &str) -> Result<Landing> {
    if atelier_core::plan::agreed(repo, id)? {
        return Ok(Landing {
            branch: None,
            remote: repo.remote().map(str::to_string),
            checkout: None,
        });
    }
    let branch = repo.branch_of(id);
    let remote = repo.remote().map(str::to_string);
    let checkout = match remote {
        Some(_) => None,
        None => git::checkout_of(&repo.root, &branch)?,
    };
    Ok(Landing {
        branch: Some(branch),
        remote,
        checkout,
    })
}

/// Records the caller's verdict in its review file and the item's new
/// status, without committing them. Where the checkout is on no
/// item's branch, it records the verdict of the plan review the caller
/// holds there.
pub fn verdict(repo: &mut Repo, verdict: Verdict, model: Option<String>) -> Result<Outcome> {
    let Some(located) = repo.current()? else {
        return plan_verdict(repo, verdict, model);
    };
    if let Some(refused) = behind(repo, &located) {
        return Ok(refused);
    }
    let record = located.record().expect("the current item has a record");
    let links = repo.links(record)?;
    let facts = facts(repo, &located, &links, None);
    let action = match verdict {
        Verdict::Approve => Action::VerdictApprove,
        Verdict::Changes => Action::VerdictChanges,
    };
    if let Some(refused) = repo.skip.refusal(action, &facts)? {
        return Ok(refused.into());
    }
    let file = facts.reviewing().expect("check found the caller's review");
    let mut review = file.review.clone();
    review.verdict = Some(verdict);
    if let Some(model) = model.filter(|model| !model.trim().is_empty()) {
        review.model = Some(model);
    }
    let mut item = record.item.clone();
    item.status = row(action).to.expect("a verdict sets the status");
    let id = item.id.clone();
    let review_path = format!("{}/reviews/{}.md", item_dir(&id), file.name);
    let message = format!(
        "Review {id}, round {}: {}",
        file.name.round,
        output::verdict_name(verdict)
    );
    let mut written = file.clone();
    written.review = review;
    written.write().context("cannot write the review file")?;
    repo.store()
        .write_item(&item)
        .context("cannot write the item's record")?;
    let paths = [item_path(&id), review_path];
    let line = output::commit_line(repo.git_at(), &paths, &message);
    Ok(Outcome::Done(output::verdict_recorded(
        &id,
        file.name.round,
        verdict,
        &paths,
        facts.holder(),
        &line,
    )))
}

/// The brief for the round `facts` shows, with the caller's review file at
/// `path`, or none for an author, and the pull request on `forge`, the
/// one the base's config names.
fn brief(
    repo: &Repo,
    facts: &Facts,
    path: Option<String>,
    forge: Option<&Forge>,
) -> Result<String> {
    let item = facts.item;
    let round = facts.round();
    let commit = match round.review {
        Some(file) => file.review.commit.clone(),
        None => tip(repo, &item.item.id)?,
    };
    let previous = item
        .reviews
        .iter()
        .filter(|review| review.name.round < round.number)
        .max_by(|a, b| a.name.cmp(&b.name))
        .map(|review| review.review.commit.clone());
    let sections = ["Goal", "Done when", "Scope", "Report"]
        .into_iter()
        .filter_map(|heading| Some((heading, item.section(heading)?)))
        .collect();
    Ok(output::brief(&Brief {
        pull_request: forge.map(|forge| pull_request(repo, forge, &item.item.id)),
        id: &item.item.id,
        title: &item.item.title,
        caller: &repo.caller,
        round: round.number,
        sections,
        base: &repo.base,
        commit: &commit,
        previous: previous.as_deref(),
        carried: &carried(item, round.number)?,
        path: path.as_deref(),
    }))
}

/// What the brief says of the pull request of item `id`'s branch on
/// `forge`, where CI's result is read. The forge is asked under the bound
/// `merge` gives a question under its lock.
fn pull_request(repo: &Repo, forge: &Forge, id: &str) -> String {
    let branch = repo.branch_of(id);
    let found = Github::open(&repo.root, forge).and_then(|github| {
        github
            .until(merge::locked_until())
            .open_pull_request(&branch, &repo.base)
    });
    output::brief_pull_request(&branch, &found)
}

/// The commit branch `id` points at.
fn tip(repo: &Repo, id: &str) -> Result<String> {
    let branch = repo.branch_of(id);
    git::branch_tip(&repo.root, &branch)?.with_context(|| format!("branch {branch} does not exist"))
}
