//! `status`, `start` and `submit`, and what the commands share.

use crate::forge::Github;
use crate::repo::{git_at, item_dir, item_path, planning, Links, Repo};
use crate::{git, merge, output, plan, setup, skip};
use anyhow::{anyhow, Context, Result};
use atelier_app::acts::{
    approval, check, mission_refusals, refusals, row, Act, Checkout, Facts, MissionFacts, Refusal,
    STARTED,
};
use atelier_app::{entry, overview, Entry, Located};
use atelier_core::{Forge, Item, MissionStatus, Status};
use atelier_records::{render_item, validate_id, ItemDir, Store};

pub enum Outcome {
    Done(String),
    Refused(String),
    /// Nothing is wrong and the act cannot finish yet, as when CI has not
    /// finished; the same command run later goes on.
    NotYet(String),
}

pub fn status(repo: &mut Repo) -> Result<String> {
    let hooks = match setup::installed(&repo.root)? {
        true => String::new(),
        false => output::no_hooks(),
    };
    Ok(hooks + &orientation(repo)?)
}

/// What `status` says to the caller about where it stands.
fn orientation(repo: &mut Repo) -> Result<String> {
    if let Some(located) = repo.current()? {
        let record = located.record().expect("the current item has a record");
        let links = repo.links(record)?;
        let head = git::head(&repo.root)?;
        let checkout = repo.checkout(record, &head)?;
        let facts = facts(repo, &located, &links, Some(&checkout));
        let mut entry = entry(&facts)?;
        annotate(repo, &mut entry, record)?;
        let decider = crate::questions::decider(repo, &facts, &links)?;
        let config = repo.base_config()?;
        let dropped = record.dropped();
        let said = output::item(&repo.caller, &entry, &config, dropped, decider.as_deref());
        return Ok(said);
    }
    let items = repo.everything()?;
    let missions = repo.missions()?;
    let mut overview = overview(&items, &missions, &repo.caller)?;
    annotate_all(repo, &mut overview.held, &items)?;
    for (_, entries) in &mut overview.led {
        annotate_all(repo, entries, &items)?;
    }
    // An item dropped on the base while its branch is there.
    let dropped: Vec<(String, Option<String>)> = items
        .iter()
        .filter_map(|located| Some((located, located.record()?)))
        .filter(|(located, record)| {
            record.item.status == Status::Dropped && located.branch.is_some()
        })
        .map(|(located, record)| (located.id.clone(), record.dropped().map(str::to_string)))
        .collect();
    let uncommitted = match repo.branch.as_deref() == Some(repo.base.as_str()) {
        true => planning(&repo.root)?,
        false => Vec::new(),
    };
    Ok(output::overview(
        &repo.caller,
        repo.branch.as_deref(),
        &repo.base,
        &overview,
        &dropped,
        &uncommitted,
        git_at(&repo.root),
    ))
}

pub fn start(repo: &mut Repo, id: &str) -> Result<Outcome> {
    validate_id(id).map_err(|message| anyhow!(message))?;
    let located = repo.locate(id)?;
    let Some(record) = located.record() else {
        let refused = not_committed(repo, id, None, None)?;
        return Ok(Outcome::Refused(
            refused.unwrap_or_else(|| output::no_item(id, &repo.base)),
        ));
    };
    let links = repo.links(record)?;
    let facts = facts(repo, &located, &links, None);
    // `start` takes over an item someone holds, and starts any other.
    let act = if STARTED.contains(&record.item.status) {
        Act::TakeOver
    } else {
        Act::Start
    };
    let to = match check(act, &facts) {
        Ok(to) => to,
        Err(refusal) => {
            let base = located.base.as_ref().map(|dir| dir.item.status);
            let text = match not_committed(repo, id, base, links.mission.as_ref())? {
                Some(pending) => pending,
                None => refused(act, &refusal, &facts)?,
            };
            match gated(act, &facts, text) {
                Some(refused) => return Ok(refused),
                None => row(act).to,
            }
        }
    };
    if act == Act::Start {
        let mut item = record.item.clone();
        item.status = to.expect("start sets a status");
        item.implementers.take(repo.caller.clone());
        return begin(repo, &item, output::started(id));
    }
    let said = {
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

/// `start --new`: an item created already building, held by the caller,
/// on a new branch named for it.
pub fn start_new(repo: &mut Repo, title: &str) -> Result<Outcome> {
    let id = repo.new_id()?;
    let mut item = plan::draft_item(&id, title, None);
    item.status = row(Act::StartNew).to.expect("start --new sets a status");
    item.implementers.take(repo.caller.clone());
    begin(repo, &item, output::started_new(&id))
}

/// Starts `item`: commits its record on the base's tip, then creates its
/// branch at that commit and switches to it, so an interrupted run leaves
/// no branch.
fn begin(repo: &mut Repo, item: &Item, said: String) -> Result<Outcome> {
    let id = &item.id;
    let commit = git::commit_file(
        &repo.root,
        &repo.base_tip,
        &[&repo.base_tip],
        &item_path(id),
        &render_item(item),
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
    let mut repo = Repo::open()?;
    Ok(Outcome::Done(format!("{said}\n{}", status(&mut repo)?)))
}

/// What a refused `start` says instead when the item, or its mission, is
/// ready in the base's checkout and not at the base's tip, which is what
/// `start` reads. `base` is the item's status at the tip, and `mission`
/// the mission the tip's record names, with its status there.
fn not_committed(
    repo: &Repo,
    id: &str,
    base: Option<Status>,
    mission: Option<&(String, Option<Status>)>,
) -> Result<Option<String>> {
    let Some(checkout) = git::checkout_of(&repo.root, &repo.base)? else {
        return Ok(None);
    };
    // A record there that does not read is no planning to commit.
    let store = Store::new(checkout.join(".atelier"));
    let mut ready = Vec::new();
    if base != Some(Status::Ready)
        && store
            .item(id)
            .is_ok_and(|dir| dir.item.status == Status::Ready)
    {
        ready.push(id.to_string());
    }
    if let Some((mission, status)) = mission {
        if *status != Some(Status::Ready)
            && store
                .mission(mission)
                .is_ok_and(|dir| dir.mission.status == MissionStatus::Ready)
        {
            ready.push(format!("mission {mission}"));
        }
    }
    if ready.is_empty() {
        return Ok(None);
    }
    let other = (checkout != repo.root).then_some(checkout.as_path());
    Ok(Some(output::not_committed_ready(
        id,
        &ready,
        &repo.base,
        other,
        git_at(&checkout),
        &planning(&checkout)?,
    )))
}

pub fn submit(repo: &mut Repo) -> Result<Outcome> {
    let Some(located) = repo.current()? else {
        return Ok(Outcome::Refused(output::not_on_item(
            "submit",
            repo.branch.as_deref(),
        )));
    };
    let record = located.record().expect("the current item has a record");
    let links = repo.links(record)?;
    let facts = facts(repo, &located, &links, None);
    if let Some(refused) = refusal(Act::Submit, &facts)? {
        return Ok(refused);
    }
    let config = repo.base_config()?;
    let mut item = record.item.clone();
    item.status = Status::InReview;
    let message = format!("Submit {} for review", item.id);
    let file = [(item_path(&item.id), render_item(&item))];
    write_and_commit(repo, &file, &[&item_dir(&item.id)], &message)?;
    let left = repo.uncommitted(&item.id)?;
    let shared = match &config.forge {
        Some(forge) => share(repo, forge, &item.id, &item.title)?,
        None => String::new(),
    };
    Ok(Outcome::Done(output::submitted(&item.id, &left, &shared)))
}

/// Pushes the branch and opens its pull request, so CI runs during the
/// review, and says what it did. A failure is only a warning: the item is
/// in review, and `merge` pushes and opens what is missing.
fn share(repo: &Repo, forge: &Forge, id: &str, title: &str) -> Result<String> {
    let github = match Github::open(&repo.root, forge).and_then(|g| g.signed_in().map(|()| g)) {
        Ok(github) => github,
        Err(error) => return Ok(output::submit_no_pull_request(id, &error, false)),
    };
    let head = git::head(&repo.root)?;
    if let Err(error) = merge::push(&repo.root, &forge.remote, &head, id) {
        return Ok(output::submit_not_pushed(id, &forge.remote, &error));
    }
    Ok(match merge::pull_request(&github, id, title, &repo.base) {
        Ok(pull) => output::pull_request(id, &forge.remote, &pull, true),
        Err(error) => output::submit_no_pull_request(id, &error, true),
    })
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

/// Adds to `entry`, which `record` gives, what views show besides the act
/// table: when it was approved, read from Git, and its unanswered questions.
pub(crate) fn annotate(repo: &Repo, entry: &mut Entry, record: &ItemDir) -> Result<()> {
    let id = &record.item.id;
    let approved = approval(record).filter(|_| record.item.status == Status::Approved);
    if let (Some(review), Some(tip)) = (approved, git::branch_tip(&repo.root, id)?) {
        let path = format!("{}/reviews/{}.md", item_dir(id), review.name);
        entry.approved = git::approved_at(&repo.root, &tip, &path)?;
    }
    let decisions = record.decisions()?.into_iter();
    entry.questions = decisions
        .filter(|d| d.answer.is_none())
        .map(|d| d.question)
        .collect();
    Ok(())
}

/// [`annotate`] for each of `entries`, found among `items`.
pub(crate) fn annotate_all<'a>(
    repo: &Repo,
    entries: impl IntoIterator<Item = &'a mut Entry>,
    items: &[Located],
) -> Result<()> {
    for entry in entries {
        let found = items.iter().find(|located| located.id == entry.id);
        if let Some(record) = found.and_then(Located::record) {
            annotate(repo, entry, record)?;
        }
    }
    Ok(())
}

pub(crate) fn facts<'a>(
    repo: &'a Repo,
    located: &'a Located,
    links: &'a Links,
    checkout: Option<&'a Checkout>,
) -> Facts<'a> {
    Facts {
        item: located.record().expect("a located item with a record"),
        branch: located.branch.is_some(),
        blockers: &links.blockers,
        mission: links.mission.clone(),
        caller: &repo.caller,
        checkout,
    }
}

/// What the caller is told when `act` is refused, or `None` when it may
/// run, as when `ATELIER_SKIP` passed every refusal.
pub(crate) fn refusal(act: Act, facts: &Facts) -> Result<Option<Outcome>> {
    match check(act, facts) {
        Ok(_) => Ok(None),
        Err(refusal) => {
            let text = refused(act, &refusal, facts)?;
            Ok(gated(act, facts, text))
        }
    }
}

/// `text`, which refuses `act` and names the way out, as the caller gets
/// it: `None` when `ATELIER_SKIP` passes every refusal of the act.
pub(crate) fn gated(act: Act, facts: &Facts, text: String) -> Option<Outcome> {
    let all = refusals(act, facts).unwrap_or_else(|problem| vec![Refusal::Record(problem)]);
    gate(act, &facts.item.item.id, &all, text)
}

/// [`gated`] for the refusals `all` of `act` on `id`: only when every one
/// is a rule can they be skipped.
pub(crate) fn gate(act: Act, id: &str, all: &[Refusal], text: String) -> Option<Outcome> {
    if !all.iter().all(|refusal| refusal.is_rule(act)) {
        return Some(Outcome::Refused(text));
    }
    skip::refuse(all.iter().flat_map(|r| output::rules(act, id, r)), text)
}

/// What the caller is told when `act`, a mission's, is refused, or `None`
/// when it may run, as when `ATELIER_SKIP` passed every refusal. A record
/// that does not read is an error.
pub(crate) fn mission_gate(act: Act, id: &str, facts: &MissionFacts) -> Result<Option<Outcome>> {
    let all = mission_refusals(act, facts)?;
    Ok(match all.first() {
        None => None,
        Some(first) => gate(act, id, &all, output::mission_refused(act, id, first)),
    })
}

/// What the caller is told when `act` is refused for `refusal`. Only a
/// refusal that says what comes next reads the item's findings, so a
/// reviewer's draft that does not parse yet cannot hide its role line.
pub(crate) fn refused(act: Act, refusal: &Refusal, facts: &Facts) -> Result<String> {
    let entry = match refusal {
        Refusal::Record(problem) => return Err(problem.clone().into()),
        Refusal::From { .. } | Refusal::Unmet(_) => Some(entry(facts)?),
        Refusal::Role(_) | Refusal::NotHolder { .. } => None,
    };
    let id = &facts.item.item.id;
    Ok(output::refusal(
        act,
        refusal,
        facts.caller,
        id,
        entry.as_ref(),
        facts.item.dropped(),
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
