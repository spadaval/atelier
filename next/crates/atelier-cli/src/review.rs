//! `review` and `verdict`: the reviewer's side of the loop.

use crate::commands::{facts, refusal, refused, write_and_commit, Outcome};
use crate::git;
use crate::output::{self, Brief};
use crate::repo::{item_dir, item_path, Repo};
use anyhow::{anyhow, Context, Result};
use atelier_app::acts::{check, row, Act, Facts, Refusal};
use atelier_app::carried;
use atelier_core::{Review, ReviewSchema, SchemaVersion, Threads, Verdict};
use atelier_records::{render_item, render_review, validate_id};

/// Opens this round's review file, or takes it over, and prints the brief.
/// The file is committed by `verdict`, not here.
pub fn review(repo: &mut Repo, id: &str) -> Result<Outcome> {
    validate_id(id).map_err(|message| anyhow!(message))?;
    let located = repo.locate(id)?;
    let Some(record) = located.record() else {
        return Ok(Outcome::Refused(output::no_item(id, &repo.base)));
    };
    let blockers = repo.blockers(record)?;
    let facts = facts(repo, &located, &blockers, None);
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
    let store = repo.store();
    let (said, name) = match round.review {
        Some(file) if file.review.reviewers.holder() == Some(&repo.caller) => {
            (String::new(), file.name.clone())
        }
        Some(file) => {
            let mut review = file.review.clone();
            let before = review.reviewers.holder().cloned();
            review.reviewers.take(repo.caller.clone());
            store.write_review(id, &file.name, &review)?;
            let said = output::review_taken_over(id, round.number, before.as_ref());
            (said, file.name.clone())
        }
        None => {
            let commit = tip(repo, id)?;
            let review = Review {
                schema: ReviewSchema::Review,
                schema_version: SchemaVersion,
                round: round.number,
                commit,
                reviewers: Threads(vec![repo.caller.clone()]),
                model: None,
                verdict: None,
                created_at: chrono::Utc::now().into(),
                body: output::template(&carried(facts.item, round.number)?),
            };
            let name = store
                .create_review(id, &review)
                .context("cannot write the review file")?;
            (String::new(), name)
        }
    };
    // The brief reads the file as it now stands.
    let located = repo.locate(id)?;
    let facts = self::facts(repo, &located, &blockers, None);
    let path = format!("{}/reviews/{name}.md", item_dir(id));
    Ok(Outcome::Done(said + &brief(repo, &facts, Some(path))?))
}

/// Records the caller's verdict in its review file and the item's new
/// status, and commits the two together.
pub fn verdict(repo: &mut Repo, verdict: Verdict, model: Option<String>) -> Result<Outcome> {
    let Some(located) = repo.current()? else {
        return Ok(Outcome::Refused(output::not_on_item(
            "verdict",
            repo.branch.as_deref(),
        )));
    };
    let record = located.record().expect("the current item has a record");
    let blockers = repo.blockers(record)?;
    let facts = facts(repo, &located, &blockers, None);
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
