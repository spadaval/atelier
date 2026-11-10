//! `status`, `start` and `submit`, and what the commands share.

use crate::repo::{item_dir, item_path, Repo};
use crate::{git, output};
use anyhow::{anyhow, Context, Result};
use atelier_app::acts::{check, Act, Checkout, Facts, Refusal, STARTED};
use atelier_app::{entry, overview, Located};
use atelier_core::Status;
use atelier_records::{render_item, validate_id};

pub enum Outcome {
    Done(String),
    Refused(String),
    /// Nothing is wrong and the act cannot finish yet, as when CI has not
    /// finished; the same command run later goes on.
    NotYet(String),
}

pub fn status(repo: &mut Repo) -> Result<String> {
    if let Some(located) = repo.current()? {
        let record = located.record().expect("the current item has a record");
        let blockers = repo.blockers(record)?;
        let head = git::head(&repo.root)?;
        let checkout = repo.checkout(record, &head)?;
        let entry = entry(&facts(repo, &located, &blockers, Some(&checkout)))?;
        let config = repo.base_config()?;
        return Ok(output::item(&repo.caller, &entry, &config));
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
    let facts = facts(repo, &located, &blockers, None);
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
            &[&repo.base_tip],
            &item_path(id),
            &render_item(&item),
            &format!("Start {id}"),
        )?;
        hold("ATELIER_TEST_HOLD_BEFORE_BRANCH");
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
    let facts = facts(repo, &located, &blockers, None);
    if let Some(refused) = refusal(Act::Submit, &facts)? {
        return Ok(Outcome::Refused(refused));
    }
    let mut item = record.item.clone();
    item.status = Status::InReview;
    let message = format!("Submit {} for review", item.id);
    let file = [(item_path(&item.id), render_item(&item))];
    write_and_commit(repo, &file, &[&item_dir(&item.id)], &message)?;
    let left = repo.uncommitted(&item.id)?;
    Ok(Outcome::Done(output::submitted(&item.id, &left)))
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
    checkout: Option<&'a Checkout>,
) -> Facts<'a> {
    Facts {
        item: located.record().expect("a located item with a record"),
        branch: located.branch.is_some(),
        blockers,
        caller: &repo.caller,
        checkout,
    }
}

/// What the caller is told when `act` is refused, or `None` when it may run.
pub(crate) fn refusal(act: Act, facts: &Facts) -> Result<Option<String>> {
    match check(act, facts) {
        Ok(_) => Ok(None),
        Err(refusal) => refused(act, refusal, facts).map(Some),
    }
}

/// What the caller is told when `act` is refused for `refusal`. Only a
/// refusal that says what comes next reads the item's findings, so a
/// reviewer's draft that does not parse yet cannot hide its role line.
pub(crate) fn refused(act: Act, refusal: Refusal, facts: &Facts) -> Result<String> {
    let entry = match &refusal {
        Refusal::Record(problem) => return Err(problem.clone().into()),
        Refusal::From { .. } | Refusal::Unmet(_) => Some(entry(facts)?),
        Refusal::Role(_) | Refusal::NotHolder { .. } => None,
    };
    let id = &facts.item.item.id;
    Ok(output::refusal(
        act,
        &refusal,
        facts.caller,
        id,
        entry.as_ref(),
    ))
}

/// Tests stop a command at a point between two of its steps by naming a
/// file in `variable`: the command creates `<file>.held` and waits until
/// `<file>` exists. `start` stops between its commit and its branch
/// (`ATELIER_TEST_HOLD_BEFORE_BRANCH`), and `merge` after reading what it
/// checks (`ATELIER_TEST_HOLD_AFTER_READ`). Only debug builds have it.
pub(crate) fn hold(variable: &str) {
    #[cfg(debug_assertions)]
    if let Some(release) = std::env::var_os(variable) {
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
