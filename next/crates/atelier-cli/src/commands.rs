//! `status`, `start` and `submit`, and what the commands share.

use crate::repo::{item_dir, item_path, Repo};
use crate::{git, output};
use anyhow::{anyhow, Context, Result};
use atelier_app::acts::{check, Act, Facts, Refusal, STARTED};
use atelier_app::{entry, overview, Located};
use atelier_core::Status;
use atelier_records::{render_item, validate_id};

pub enum Outcome {
    Done(String),
    Refused(String),
}

pub fn status(repo: &mut Repo) -> Result<String> {
    if let Some(located) = repo.current()? {
        let record = located.record().expect("the current item has a record");
        let blockers = repo.blockers(record)?;
        let entry = entry(&facts(repo, &located, &blockers))?;
        return Ok(output::item(&repo.caller, &entry));
    }
    let items = repo.everything()?;
    let overview = overview(&items, &repo.caller)?;
    Ok(output::overview(
        &repo.caller,
        repo.branch.as_deref(),
        &repo.base,
        &overview,
    ))
}

pub fn start(repo: &mut Repo, id: &str) -> Result<Outcome> {
    validate_id(id).map_err(|message| anyhow!(message))?;
    let located = repo.locate(id)?;
    let Some(record) = located.record() else {
        return Ok(Outcome::Refused(output::no_item(id, &repo.base)));
    };
    let blockers = repo.blockers(record)?;
    let facts = facts(repo, &located, &blockers);
    // `start` takes over an item someone holds, and starts any other.
    let act = if STARTED.contains(&record.item.status) {
        Act::TakeOver
    } else {
        Act::Start
    };
    if let Some(refused) = refusal(act, &facts)? {
        return Ok(Outcome::Refused(refused));
    }
    let said = if act == Act::Start {
        let mut item = record.item.clone();
        item.status = Status::Building;
        item.implementers.take(repo.caller.clone());
        let commit = git::commit_file(
            &repo.root,
            &repo.base_tip,
            &item_path(id),
            &render_item(&item),
            &format!("Start {id}"),
        )?;
        hold_before_branch();
        if !git::create_branch(&repo.root, id, &commit)? {
            let holder = repo
                .locate(id)?
                .record()
                .and_then(|r| r.item.implementers.holder().cloned());
            return Ok(Outcome::Refused(output::lost_race(id, holder.as_ref())));
        }
        git::switch(&repo.root, id).with_context(|| output::not_switched(id))?;
        output::started(id)
    } else {
        if repo.branch.as_deref() != Some(id) {
            git::switch(&repo.root, id).with_context(|| output::switch_failed(id))?;
        }
        let store = repo.store();
        let mut dir = store.item(id)?;
        let before = dir.item.implementers.holder().cloned();
        if before.as_ref() == Some(&repo.caller) {
            output::already_held(id)
        } else {
            dir.item.implementers.take(repo.caller.clone());
            let message = match &before {
                Some(thread) => format!("Take over {id} from {thread}"),
                None => format!("Take over {id}"),
            };
            let path = item_path(id);
            let file = [(path.clone(), render_item(&dir.item))];
            write_and_commit(repo, &file, &[&path], &message)?;
            output::taken_over(id, before.as_ref())
        }
    };
    let mut repo = Repo::open()?;
    Ok(Outcome::Done(format!("{said}\n{}", status(&mut repo)?)))
}

pub fn submit(repo: &mut Repo) -> Result<Outcome> {
    let Some(located) = repo.current()? else {
        return Ok(Outcome::Refused(output::not_on_item(
            "submit",
            repo.branch.as_deref(),
        )));
    };
    let record = located.record().expect("the current item has a record");
    let blockers = repo.blockers(record)?;
    let facts = facts(repo, &located, &blockers);
    if let Some(refused) = refusal(Act::Submit, &facts)? {
        return Ok(Outcome::Refused(refused));
    }
    let mut item = record.item.clone();
    item.status = Status::InReview;
    let message = format!("Submit {} for review", item.id);
    let file = [(item_path(&item.id), render_item(&item))];
    write_and_commit(repo, &file, &[&item_dir(&item.id)], &message)?;
    Ok(Outcome::Done(output::submitted(&item.id)))
}

/// Writes each of `files`, a path in the checkout and its text, and
/// commits `paths`. When the commit fails, each file is put back as it was,
/// so the act can be run again.
pub(crate) fn write_and_commit(
    repo: &Repo,
    files: &[(String, String)],
    paths: &[&str],
    message: &str,
) -> Result<()> {
    let before = files
        .iter()
        .map(|(path, _)| std::fs::read(repo.root.join(path)))
        .collect::<std::io::Result<Vec<_>>>()?;
    let put_back = || {
        for ((path, _), text) in files.iter().zip(&before) {
            let _ = std::fs::write(repo.root.join(path), text);
        }
    };
    let written = files
        .iter()
        .try_for_each(|(path, text)| std::fs::write(repo.root.join(path), text));
    if let Err(error) = written {
        put_back();
        return Err(error.into());
    }
    git::commit_paths(&repo.root, paths, message).map_err(|error| {
        put_back();
        error.context(output::not_committed())
    })
}

pub(crate) fn facts<'a>(
    repo: &'a Repo,
    located: &'a Located,
    blockers: &'a [(String, Option<Status>)],
) -> Facts<'a> {
    Facts {
        item: located.record().expect("a located item with a record"),
        branch: located.branch.is_some(),
        blockers,
        caller: &repo.caller,
    }
}

/// What the caller is told when `act` is refused, or `None` when it may run.
pub(crate) fn refusal(act: Act, facts: &Facts) -> Result<Option<String>> {
    match check(act, facts) {
        Ok(_) => Ok(None),
        Err(Refusal::Record(problem)) => Err(problem.into()),
        Err(refusal) => Ok(Some(output::refusal(
            act,
            &refusal,
            facts.caller,
            &entry(facts)?,
        ))),
    }
}

/// Tests stop `start` here, between its commit and its branch, by naming a
/// file in `ATELIER_TEST_HOLD_BEFORE_BRANCH`: `start` creates `<file>.held`
/// and waits until `<file>` exists. Only debug builds have it.
fn hold_before_branch() {
    #[cfg(debug_assertions)]
    if let Some(release) = std::env::var_os("ATELIER_TEST_HOLD_BEFORE_BRANCH") {
        let release = std::path::PathBuf::from(release);
        let _ = std::fs::write(release.with_extension("held"), "");
        for _ in 0..3000 {
            if release.exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }
}
