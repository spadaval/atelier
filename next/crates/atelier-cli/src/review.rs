//! `review` and `verdict`: the reviewer's side of the loop, on an item
//! and on a mission's plan.

use crate::commands::{facts, refusal, refused, write_and_commit, Outcome};
use crate::git;
use crate::output::{self, Brief, PlanBrief};
use crate::repo::{item_dir, item_path, mission_dir, mission_path, Repo};
use anyhow::{anyhow, Context, Result};
use atelier_app::acts::{check, check_mission, plan_round, row, Act, Facts, MissionFacts, Refusal};
use atelier_app::{carried, carried_plan};
use atelier_core::{Review, ReviewName, ReviewSchema, SchemaVersion, Thread, Threads, Verdict};
use atelier_records::{create_review, render_item, render_review, validate_id, ReviewFile};

/// Opens this round's review file, or takes it over, and prints the brief.
/// The file is committed by `verdict`, not here.
pub fn review(repo: &mut Repo, id: &str) -> Result<Outcome> {
    validate_id(id).map_err(|message| anyhow!(message))?;
    if repo.root.join(mission_path(id)).exists() {
        return plan_review(repo, id);
    }
    let located = repo.locate(id)?;
    let Some(record) = located.record() else {
        return Ok(Outcome::Refused(output::no_item(id, &repo.base)));
    };
    let links = repo.links(record)?;
    let facts = facts(repo, &located, &links, None);
    match check(Act::Review, &facts) {
        Ok(_) => {}
        // Its one condition: the caller never implemented the item.
        Err(Refusal::Unmet(_)) => {
            let brief = brief(repo, &facts, None)?;
            return Ok(Outcome::Refused(output::author(id, &repo.caller) + &brief));
        }
        Err(refusal) => return Ok(Outcome::Refused(refused(Act::Review, refusal, &facts)?)),
    }
    if repo.branch.as_deref() != Some(id) {
        return Ok(Outcome::Refused(output::review_elsewhere(
            id,
            repo.branch.as_deref(),
        )));
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
    Ok(Outcome::Done(said + &brief(repo, &facts, Some(path))?))
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
    carried: &[atelier_app::Carried],
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

/// `review` on a mission: opens the caller's review of its plan as this
/// checkout holds it at HEAD, or takes over one with no verdict, and prints
/// the plan's brief. It runs in any checkout, so two reviewers can each
/// work blind in a detached worktree of their own, and commits nothing.
fn plan_review(repo: &mut Repo, id: &str) -> Result<Outcome> {
    let store = repo.store();
    let mission = store.mission(id)?;
    let round = plan_round(&mission, &repo.caller);
    let commit = git::head(&repo.root)?;
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
    let documents = git::files(&repo.root, &mission_dir(id))?
        .into_iter()
        .filter(|path| {
            *path != mission_path(id) && !path.starts_with(&format!("{}/reviews/", mission_dir(id)))
        })
        .collect();
    let items = store
        .items()?
        .into_iter()
        .filter(|dir| dir.item.mission.as_deref() == Some(id))
        .map(|dir| (item_path(&dir.item.id), dir.item.status, dir.item.title))
        .collect();
    let sections = [
        "Why",
        "Outcome",
        "Success criteria",
        "Scope",
        "Authority",
        "Result",
    ]
    .into_iter()
    .filter_map(|heading| Some((heading, mission.section(heading)?)))
    .collect();
    let brief = output::plan_brief(&PlanBrief {
        id,
        title: &mission.mission.title,
        caller: &repo.caller,
        round: file.name.round,
        sections,
        commit: &file.review.commit,
        documents,
        items,
        carried: &carried_plan(&mission, file.name.round)?,
        path: &format!("{}/reviews/{name}.md", mission_dir(id)),
        base: &repo.base,
    });
    Ok(Outcome::Done(said + &brief))
}

/// `verdict` on the plan review the caller holds in this checkout, which
/// has no item's branch: records it and commits the file alone, changing
/// no status.
fn plan_verdict(repo: &mut Repo, verdict: Verdict, model: Option<String>) -> Result<Outcome> {
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
    let (mission, name) = match held.len() {
        0 => {
            return Ok(Outcome::Refused(output::no_review_held(
                repo.branch.as_deref(),
            )))
        }
        1 => held.remove(0),
        _ => {
            let paths: Vec<String> = held
                .iter()
                .map(|(m, name)| format!("{}/reviews/{name}.md", mission_dir(&m.mission.id)))
                .collect();
            return Ok(Outcome::Refused(output::plan_reviews_held(&paths)));
        }
    };
    let id = mission.mission.id.clone();
    let file = mission
        .reviews
        .iter()
        .find(|r| r.name == name)
        .expect("it was found");
    let act = match verdict {
        Verdict::Approve => Act::PlanVerdictApprove,
        Verdict::Changes => Act::PlanVerdictChanges,
    };
    let facts = MissionFacts {
        mission: &mission,
        items: &[],
        review: Some(file),
    };
    match check_mission(act, &facts) {
        Ok(_) => {}
        Err(Refusal::Record(problem)) => return Err(problem.into()),
        Err(refusal) => {
            return Ok(Outcome::Refused(output::mission_refused(
                act, &id, &refusal,
            )))
        }
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
    write_and_commit(
        repo,
        &[(path.clone(), render_review(&review))],
        &[&path],
        &message,
    )?;
    let commit = git::head(&repo.root)?;
    let on_base = repo.branch.as_deref() == Some(repo.base.as_str());
    Ok(Outcome::Done(output::plan_verdict_recorded(
        &id, name.round, verdict, &path, &commit, &repo.base, on_base,
    )))
}

/// Records the caller's verdict in its review file and the item's new
/// status, and commits the two together. Where the checkout is on no
/// item's branch, it records the verdict of the plan review the caller
/// holds there.
pub fn verdict(repo: &mut Repo, verdict: Verdict, model: Option<String>) -> Result<Outcome> {
    let Some(located) = repo.current()? else {
        return plan_verdict(repo, verdict, model);
    };
    let record = located.record().expect("the current item has a record");
    let links = repo.links(record)?;
    let facts = facts(repo, &located, &links, None);
    let act = match verdict {
        Verdict::Approve => Act::VerdictApprove,
        Verdict::Changes => Act::VerdictChanges,
    };
    if let Some(refused) = refusal(act, &facts)? {
        return Ok(Outcome::Refused(refused));
    }
    let file = facts.reviewing().expect("check found the caller's review");
    let mut review = file.review.clone();
    review.verdict = Some(verdict);
    if let Some(model) = model.filter(|model| !model.trim().is_empty()) {
        review.model = Some(model);
    }
    let mut item = record.item.clone();
    item.status = row(act).to.expect("a verdict sets the status");
    let id = item.id.clone();
    let review_path = format!("{}/reviews/{}.md", item_dir(&id), file.name);
    let files = [
        (item_path(&id), render_item(&item)),
        (review_path.clone(), render_review(&review)),
    ];
    let message = format!(
        "Review {id}, round {}: {}",
        file.name.round,
        output::verdict_name(verdict)
    );
    write_and_commit(repo, &files, &[&files[0].0, &review_path], &message)?;
    Ok(Outcome::Done(output::verdict_recorded(
        &id,
        file.name.round,
        verdict,
        &review_path,
        facts.holder(),
    )))
}

/// The brief for the round `facts` shows, with the caller's review file at
/// `path`, or none for an author.
fn brief(repo: &Repo, facts: &Facts, path: Option<String>) -> Result<String> {
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

/// The commit branch `id` points at.
fn tip(repo: &Repo, id: &str) -> Result<String> {
    git::branch_tip(&repo.root, id)?.with_context(|| format!("branch {id} does not exist"))
}
