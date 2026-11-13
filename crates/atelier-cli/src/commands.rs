//! `status`, `start` and `submit`, and what the commands share.

use crate::forge::Github;
use crate::{git, merge, output, plan, setup};
use anyhow::{anyhow, Context, Result};
use atelier_app::acts::{check, row, unmet, Act, Checkout, Facts, STARTED};
use atelier_app::repo::{facts, item_dir, item_path, mission_path, planning, write_and_commit};
use atelier_app::{
    annotate, annotate_all, entry, overview, refused, Done, Entry, Located, Refused, Repo,
};
use atelier_core::{validate_slug, Forge, Named, Status};
use atelier_records::{render_item, render_mission, validate_id};

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
        let config = repo.base_config()?;
        let dropped = record.dropped();
        let leads = match (&links.mission, &links.lead) {
            (Some((mission, _)), Some(lead)) if *lead == repo.caller => Some(mission.as_str()),
            _ => None,
        };
        // The implementer's own directory is committed by `submit`.
        let own = entry.holds.then(|| item_dir(&located.id));
        let unsaved = unsaved(repo, own.as_deref())?;
        return Ok(output::item(
            &repo.caller,
            &entry,
            &config,
            dropped,
            leads,
            &unsaved,
        ));
    }
    let items = repo.everything()?;
    let missions = repo.missions()?;
    let mut overview = overview(&items, &missions, &repo.caller)?;
    annotate_all(repo, &mut overview.held, &items)?;
    for (_, entries, _) in &mut overview.led {
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
    // An item dropped on its branch whose drop the base has not taken.
    let unreached: Vec<(String, Option<String>)> = atelier_app::unreached(&items)
        .into_iter()
        .map(|(id, _, why)| (id, why))
        .collect();
    let kind = match repo.named() {
        Some(Named::Backlog(_)) => Some("a backlog branch for record changes"),
        Some(Named::Record(id)) if repo.root.join(mission_path(&id)).exists() => {
            Some("the branch of a mission's plan")
        }
        _ => None,
    };
    let unsaved = unsaved(repo, None)?;
    Ok(output::overview(
        &repo.caller,
        repo.branch.as_deref(),
        &repo.base,
        kind,
        &overview,
        &dropped,
        &unreached,
        &unsaved,
    ))
}

/// What `status` says the checkout has not saved: its record changes not
/// committed, but for those in `own`, and its branch's commits not pushed
/// to the remote.
fn unsaved(repo: &Repo, own: Option<&str>) -> Result<String> {
    let mut records = planning(&repo.root)?;
    if let Some(own) = own {
        records.retain(|path| !path.starts_with(&format!("{own}/")));
    }
    let remote = repo.remote();
    let unpushed = match (&repo.branch, remote) {
        (Some(branch), Some(remote)) => {
            let since = match atelier_app::git::remote_tip(&repo.root, remote, branch)? {
                Some(tip) => tip,
                None => repo.base_tip.clone(),
            };
            let range = format!("{since}..HEAD");
            git::run(&repo.root, &["rev-list", "--count", &range])?
                .parse()
                .unwrap_or(0)
        }
        _ => 0,
    };
    Ok(output::unsaved(
        repo.git_at(),
        &records,
        repo.branch.as_deref(),
        remote,
        unpushed,
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
    let branch = repo.branch_of(id);
    if act == Act::Start {
        let mut item = record.item.clone();
        item.status = to.expect("start sets a status");
        item.implementers.take(repo.caller.clone());
        let claim = Claim {
            branch,
            record: Some((item_path(id), render_item(&item), format!("Start {id}"))),
            item: Some(id),
        };
        return begin(repo, claim, output::started(id, &repo.branch_of(id)));
    }
    take_over(repo, id, &branch, &located)
}

/// `start` on an item someone holds: the caller takes it over on its
/// branch, which it switches to, creating the local branch from the
/// forge's where only that exists. In the base's own checkout the record
/// is committed on the branch without switching.
fn take_over(repo: &mut Repo, id: &str, branch: &str, located: &Located) -> Result<Outcome> {
    let root = repo.root.clone();
    let on_base = repo.branch.as_deref() == Some(repo.base.as_str());
    let found = located
        .branch
        .as_ref()
        .expect("a started item has a branch");
    let tip = found.tip.clone();
    let local = git::branch_tip(&root, branch)?;
    // Bring the local branch to the forge's tip where the forge is ahead,
    // so the take-over builds on what the forge has.
    match &local {
        None => {
            git::create_branch(&root, branch, &tip)?;
        }
        Some(local) if found.remote && *local != tip => {
            if repo.branch.as_deref() != Some(branch) {
                git::move_branch(&root, branch, &tip, local)?;
            }
        }
        _ => {}
    }
    if !on_base && repo.branch.as_deref() != Some(branch) {
        if let Some(path) = git::checkout_of(&root, branch)? {
            return Ok(Outcome::Refused(output::checked_out_elsewhere(
                id, branch, &path,
            )));
        }
        git::switch(&root, branch).with_context(|| output::switch_failed(id, branch))?;
    }
    if !on_base && found.remote && git::head(&root)? != tip {
        git::run(&root, &["merge", "--ff-only", "--quiet", &tip])
            .with_context(|| output::switch_failed(id, branch))?;
    }
    let mut item = found
        .record
        .clone()
        .or_else(|| located.record().cloned())
        .expect("a started item has a record")
        .item;
    let before = item.implementers.holder().cloned();
    let said = if before.as_ref() == Some(&repo.caller) {
        output::already_held(id, branch)
    } else {
        item.implementers.take(repo.caller.clone());
        let message = match &before {
            Some(thread) => format!("Take over {id} from {thread}"),
            None => format!("Take over {id}"),
        };
        let path = item_path(id);
        let text = render_item(&item);
        let parent = match on_base {
            true => tip.clone(),
            false => git::head(&root)?,
        };
        if on_base {
            let commit = git::commit_file(
                &root,
                &parent,
                &[&parent],
                &path,
                &text,
                &message,
                repo.skip,
            )?;
            git::move_branch(&root, branch, &commit, &parent)?;
        } else {
            let file = [(path.clone(), text)];
            write_and_commit(repo, &file, &[&path], &message)?;
        }
        if let Some(remote) = repo.remote().map(str::to_string) {
            let head = git::branch_tip(&root, branch)?.expect("the branch was just made");
            if let Err(error) = merge::push(&root, &remote, &head, branch, repo.skip) {
                return Ok(Outcome::Refused(output::take_over_not_pushed(
                    id, branch, &remote, &error,
                )));
            }
        }
        output::taken_over(id, before.as_ref())
    };
    let tail = match on_base {
        true => output::worktree(branch, &worktree_path(repo, branch)),
        false => String::new(),
    };
    let mut repo = crate::open(&repo.dir, repo.skip)?;
    Ok(Outcome::Done(format!(
        "{said}{tail}\n{}",
        status(&mut repo)?
    )))
}

/// `start --new`: an item created already building, held by the caller,
/// on a new branch named for it.
pub fn start_new(repo: &mut Repo, title: &str) -> Result<Outcome> {
    let id = repo.new_id()?;
    let mut item = plan::draft_item(&id, title, None);
    item.status = row(Act::StartNew).to.expect("start --new sets a status");
    item.implementers.take(repo.caller.clone());
    let claim = Claim {
        branch: repo.branch_of(&id),
        record: Some((item_path(&id), render_item(&item), format!("Start {id}"))),
        item: Some(&id),
    };
    begin(repo, claim, output::started_new(&id, &repo.branch_of(&id)))
}

/// `start --mission`: a new draft mission, created on a new branch named
/// for it, where its plan is written.
pub fn start_mission(repo: &mut Repo, title: &str) -> Result<Outcome> {
    let id = repo.new_id()?;
    let mission = plan::draft_mission(&id, title);
    let branch = repo.branch_of(&id);
    let claim = Claim {
        branch: branch.clone(),
        record: Some((
            mission_path(&id),
            render_mission(&mission),
            format!("Plan mission {id}"),
        )),
        item: None,
    };
    begin(repo, claim, output::started_mission(&id, &branch))
}

/// `start --backlog <slug>`: a branch `backlog/<slug>` at the base's tip,
/// for record changes tied to no item or mission.
pub fn start_backlog(repo: &mut Repo, slug: &str) -> Result<Outcome> {
    if let Err(message) = validate_slug(slug) {
        return Ok(Outcome::Refused(output::bad_backlog_slug(slug, &message)));
    }
    let branch = repo.config.backlog_branch(slug);
    let remote = match repo.remote() {
        Some(remote) => match git::remote_tip(&repo.root, remote, &branch) {
            Ok(tip) => tip.map(|_| remote.to_string()),
            Err(error) => {
                let remote = remote.to_string();
                return Ok(Outcome::Refused(output::unreachable(
                    &branch, &remote, &error,
                )));
            }
        },
        None => None,
    };
    let local = git::branch_tip(&repo.root, &branch)?.is_some();
    if local || remote.is_some() {
        let text = output::backlog_exists(&branch, remote.as_deref());
        let rule = output::rule_backlog_exists(&branch);
        if let Some(refused) = repo.skip.refuse([rule], text) {
            return Ok(refused.into());
        }
    }
    let claim = Claim {
        branch: branch.clone(),
        record: None,
        item: None,
    };
    begin(repo, claim, output::started_backlog(&branch))
}

/// What `start` makes: the branch, and the one record change that is the
/// claim, with its commit message, where there is one. `item` is the item
/// whose claim it is, whose holder a lost claim names.
struct Claim<'a> {
    branch: String,
    record: Option<(String, String, String)>,
    item: Option<&'a str>,
}

/// Makes `claim`: commits its record on the base's tip, then creates its
/// branch at that commit, so an interrupted run leaves no branch, pushes
/// it where a remote is configured, and switches this checkout to it,
/// except in the base's own checkout, where it prints the `git worktree
/// add` that gives the branch a checkout of its own.
fn begin(repo: &mut Repo, claim: Claim, said: String) -> Result<Outcome> {
    let root = repo.root.clone();
    let branch = &claim.branch;
    let commit = match &claim.record {
        Some((path, text, message)) => git::commit_file(
            &root,
            &repo.base_tip,
            &[&repo.base_tip],
            path,
            text,
            message,
            repo.skip,
        )?,
        None => repo.base_tip.clone(),
    };
    hold("ATELIER_TEST_HOLD_BEFORE_BRANCH");
    if !git::create_branch(&root, branch, &commit)? {
        let holder = match claim.item {
            Some(id) => repo
                .locate(id)?
                .record()
                .and_then(|r| r.item.implementers.holder().cloned()),
            None => None,
        };
        let id = claim.item.unwrap_or(branch);
        return Ok(Outcome::Refused(output::lost_race(id, holder.as_ref())));
    }
    if let Some(remote) = repo.remote().map(str::to_string) {
        if let Err(error) = merge::push(&root, &remote, &commit, branch, repo.skip) {
            // The claim is the remote's branch: this clone keeps none.
            git::delete_branch(&root, branch, &commit)?;
            return Ok(Outcome::Refused(lost_claim(repo, &claim, &remote, &error)?));
        }
    }
    if repo.branch.as_deref() == Some(repo.base.as_str()) {
        let path = worktree_path(repo, branch);
        let mut repo = crate::open(&repo.dir, repo.skip)?;
        return Ok(Outcome::Done(format!(
            "{said}{}\n{}",
            output::worktree(branch, &path),
            status(&mut repo)?
        )));
    }
    git::switch(&root, branch).with_context(|| output::not_switched(branch))?;
    let mut repo = crate::open(&repo.dir, repo.skip)?;
    Ok(Outcome::Done(format!("{said}\n{}", status(&mut repo)?)))
}

/// Why the push of `claim`'s branch to `remote` failed with `error`: the
/// remote has the branch, whose holder is fetched and named; or it cannot
/// be reached; or it refused the push.
fn lost_claim(
    repo: &mut Repo,
    claim: &Claim,
    remote: &str,
    error: &anyhow::Error,
) -> Result<String> {
    let branch = &claim.branch;
    let tip = match git::remote_tip(&repo.root, remote, branch) {
        Ok(tip) => tip,
        Err(_) => return Ok(output::unreachable(branch, remote, error)),
    };
    let Some(_) = tip else {
        return Ok(output::claim_push_refused(branch, remote, error));
    };
    let fetched = format!("refs/heads/{branch}:refs/remotes/{remote}/{branch}");
    let _ = git::run(&repo.root, &["fetch", "--quiet", remote, &fetched]);
    let holder = match claim.item {
        Some(id) => repo
            .locate(id)?
            .record()
            .and_then(|r| r.item.implementers.holder().cloned()),
        None => None,
    };
    Ok(output::lost_claim(
        claim.item.unwrap_or(branch),
        branch,
        remote,
        holder.as_ref(),
    ))
}

/// Where the checkout of `branch` goes, beside the repository's root.
fn worktree_path(repo: &Repo, branch: &str) -> std::path::PathBuf {
    let name = repo
        .root
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| repo.slug.clone());
    let leaf = branch.replace('/', "-");
    repo.root
        .parent()
        .unwrap_or(&repo.root)
        .join(format!("{name}-{leaf}"))
}

pub fn submit(repo: &mut Repo) -> Result<Outcome> {
    if records_branch(repo)?.is_some() {
        return submit_records(repo);
    }
    let Some(located) = repo.current()? else {
        return Ok(Outcome::Refused(output::not_on_item(
            "submit",
            repo.branch.as_deref(),
        )));
    };
    let record = located.record().expect("the current item has a record");
    // The forge's copy is ahead, as after another clone's drop or
    // take-over: a submit from here would build on what it lacks.
    if let Some(branch) = located.branch.as_ref().filter(|b| b.remote) {
        if record.item.status != Status::Dropped {
            let remote = repo.remote().unwrap_or("the remote").to_string();
            let refused = output::behind(
                &located.id,
                repo.branch.as_deref().unwrap_or(""),
                &remote,
                &branch.tip,
            );
            return Ok(Outcome::Refused(refused));
        }
    }
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
    let branch = repo.branch_of(&item.id);
    let shared = match &config.forge {
        Some(forge) => {
            let text = output::pull_request_text(&item.id, &item.title, &repo.base);
            share(repo, forge, &branch, text)?
        }
        None => String::new(),
    };
    Ok(Outcome::Done(output::submitted(&item.id, &left, &shared)))
}

/// The checked-out branch, when it holds record changes alone by the
/// convention: a backlog branch, or a mission's branch.
pub fn records_branch(repo: &mut Repo) -> Result<Option<String>> {
    let branch = repo.branch.clone();
    Ok(match repo.named() {
        Some(Named::Backlog(_)) => branch,
        Some(Named::Record(id)) if repo.root.join(mission_path(&id)).exists() => {
            match repo.current()? {
                Some(_) => None,
                None => branch,
            }
        }
        _ => None,
    })
}

/// `submit` on a backlog or mission branch: refused while a record the
/// branch changes is not committed; else, where a forge is configured, the
/// branch is pushed and its pull request opened.
fn submit_records(repo: &mut Repo) -> Result<Outcome> {
    let branch = records_branch(repo)?.expect("on a records branch");
    let uncommitted = planning(&repo.root)?;
    if !uncommitted.is_empty() {
        let line = output::commit_line(repo.git_at(), &uncommitted, "<what they change>");
        return Ok(Outcome::Refused(output::records_uncommitted(
            &branch,
            &uncommitted,
            &line,
        )));
    }
    let config = repo.base_config()?;
    let shared = match &config.forge {
        Some(forge) => {
            let text = output::records_pull_request_text(&branch, &repo.base);
            share(repo, forge, &branch, text)?
        }
        None => String::new(),
    };
    Ok(Outcome::Done(output::records_submitted(&branch, &shared)))
}

/// Pushes `branch` and opens its pull request, titled and described by
/// `text`, so CI runs during the review, and says what it did. A failure
/// is only a warning: `merge` pushes and opens what is missing.
fn share(repo: &Repo, forge: &Forge, branch: &str, text: (String, String)) -> Result<String> {
    let github = match Github::open(&repo.root, forge).and_then(|g| g.signed_in().map(|()| g)) {
        Ok(github) => github,
        Err(error) => return Ok(output::submit_no_pull_request(branch, &error, false)),
    };
    let head = git::head(&repo.root)?;
    if let Err(error) = merge::push(&repo.root, &forge.remote, &head, branch, repo.skip) {
        return Ok(output::submit_not_pushed(branch, &forge.remote, &error));
    }
    Ok(
        match merge::pull_request(&github, branch, text, &repo.base) {
            Ok(pull) => output::pull_request(branch, &forge.remote, &pull, true),
            Err(error) => output::submit_no_pull_request(branch, &error, true),
        },
    )
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
        let tip = git::branch_tip(&repo.root, &repo.branch_of(&entry.id))?;
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
