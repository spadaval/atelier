//! `status`, `start` and `submit`, and what the commands share.

use crate::forge::Github;
use crate::{git, merge, output, plan, setup};
use anyhow::{anyhow, Context, Result};
use atelier_app::acts::{check, row, unmet, Act, Checkout, Facts, STARTED};
use atelier_app::repo::{facts, item_dir, item_path, planning, write_and_commit};
use atelier_app::{
    annotate, annotate_all, decider, entry, overview, refused, Done, Entry, Located, Refused, Repo,
};
use atelier_core::{Forge, Item, Status};
use atelier_records::{render_item, validate_id};

pub enum Outcome {
    Done(String),
    Refused(String),
    /// Nothing is wrong and the act cannot finish yet, as when CI has not
    /// finished; the same command run later goes on.
    NotYet(String),
}

/// A refusal as the command prints it: its text, its way out, and, when it
/// is a rule's, the skip after the way out.
impl From<Refused> for Outcome {
    fn from(refused: Refused) -> Outcome {
        let mut text = refused.text;
        if let Some(next) = refused.next {
            text += &format!("{next}\n");
        }
        if !refused.rules.is_empty() {
            text += &output::skip_offer();
        }
        Outcome::Refused(text)
    }
}

impl From<Result<Done, Refused>> for Outcome {
    fn from(acted: Result<Done, Refused>) -> Outcome {
        match acted {
            Ok(done) => Outcome::Done(done.text),
            Err(refused) => refused.into(),
        }
    }
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
        let decider = decider(repo, &facts, &links)?;
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
        annotate_all(repo, entries.iter_mut(), &items)?;
        approved_needs(repo, entries, &items)?;
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
        repo.git_at(),
    ))
}

pub fn start(repo: &mut Repo, id: &str) -> Result<Outcome> {
    validate_id(id).map_err(|message| anyhow!(message))?;
    let located = repo.locate(id)?;
    let Some(record) = located.record() else {
        return Ok(output::no_item(id, &repo.base).into());
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
        Err(refusal) => match repo
            .skip
            .gated(act, &facts, refused(act, &refusal, &facts)?)
        {
            Some(refused) => return Ok(refused.into()),
            None => row(act).to,
        },
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
    let mut repo = crate::open(&repo.dir, repo.skip)?;
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
        repo.skip,
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
    let mut repo = crate::open(&repo.dir, repo.skip)?;
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
    let links = repo.links(record)?;
    let facts = facts(repo, &located, &links, None);
    if let Some(refused) = repo.skip.refusal(Act::Submit, &facts)? {
        return Ok(refused.into());
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
    if let Err(error) = merge::push(&repo.root, &forge.remote, &head, id, repo.skip) {
        return Ok(output::submit_not_pushed(id, &forge.remote, &error));
    }
    Ok(match merge::pull_request(&github, id, title, &repo.base) {
        Ok(pull) => output::pull_request(id, &forge.remote, &pull, true),
        Err(error) => output::submit_no_pull_request(id, &error, true),
    })
}

/// What the holder of each approved item of `entries`, which a lead is
/// shown, still needs before `merge`, read from the branch's last commit,
/// so a lead sees an approval gone stale as the holder's `status` does.
fn approved_needs(repo: &mut Repo, entries: &mut [Entry], items: &[Located]) -> Result<()> {
    let approved = entries
        .iter_mut()
        .filter(|e| e.status == Status::Approved && !e.holds);
    for entry in approved {
        let found = items.iter().find(|located| located.id == entry.id);
        let Some(record) = found.and_then(Located::record) else {
            continue;
        };
        let holder = record.item.implementers.holder();
        let tip = git::branch_tip(&repo.root, &entry.id)?;
        let (Some(holder), Some(tip)) = (holder, tip) else {
            continue;
        };
        let checkout = Checkout {
            uncommitted: Vec::new(),
            since_approval: repo.since_approval(record, &tip)?,
        };
        let facts = Facts {
            item: record,
            branch: true,
            blockers: &[],
            mission: None,
            caller: holder,
            checkout: Some(&checkout),
        };
        entry.needs = unmet(Act::Merge, &facts)?;
    }
    Ok(())
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
