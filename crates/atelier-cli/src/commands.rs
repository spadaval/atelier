//! `status`, `start` and `submit`, and what the commands share.

use crate::forge::Github;
use crate::{git, merge, output, plan, setup};
use anyhow::{anyhow, Context, Result};
use atelier_app::acts::{check, row, unmet, Act, Checkout, Facts, MissionFacts, STARTED};
use atelier_app::repo::{facts, item_dir, item_path, mission_path, planning, write_and_commit};
use atelier_app::{
    annotate, annotate_all, entry, overview, refused, Done, Entry, Located, Refused, Repo,
};
use atelier_core::{validate_slug, Forge, MissionStatus, Named, Status};
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
        let unsaved = unsaved(repo)?;
        // Dropped on this branch, with the base's copy not dropped yet.
        let unreached = !atelier_app::unreached(std::slice::from_ref(&located)).is_empty();
        return Ok(output::item(
            &repo.caller,
            &entry,
            &config,
            dropped,
            leads,
            &unsaved,
            unreached,
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
    let mut unsaved = unsaved(repo)?;
    // A stop of a mission this checkout holds dropped, until published.
    for id in repo.store().ids("missions")? {
        let dropped =
            |dir: &atelier_records::MissionDir| dir.mission.status == MissionStatus::Dropped;
        let base = repo.base_tip.clone();
        if !dropped(&repo.store().mission(&id)?)
            || repo.mission_at(&base, &id)?.is_some_and(|d| dropped(&d))
        {
            continue;
        }
        let located = repo.find_mission(&id)?;
        unsaved += &crate::views::unpublished_stop(repo, &located)?.unwrap_or_default();
    }
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

/// The record changes the checkout has not committed, as `check` lists
/// them: `config.toml`, and the changed and new files under the missions
/// and items.
fn record_changes(repo: &Repo) -> Result<Vec<String>> {
    let mut records = planning(&repo.root)?;
    let config = atelier_records::CONFIG;
    if git::uncommitted(&repo.root)?
        .iter()
        .any(|path| path == config)
        || !git::untracked(&repo.root, config)?.is_empty()
    {
        records.insert(0, config.to_string());
    }
    Ok(records)
}

/// What `status` says the checkout has not saved: its record changes not
/// committed, and its branch's commits not pushed to the remote.
fn unsaved(repo: &Repo) -> Result<String> {
    let records = record_changes(repo)?;
    let remote = repo.remote();
    let unpushed: Vec<String> = match (&repo.branch, remote) {
        (Some(branch), Some(remote)) => {
            let since = match atelier_app::git::remote_tip(&repo.root, remote, branch)? {
                Some(tip) => tip,
                None => repo.base_tip.clone(),
            };
            let range = format!("{since}..HEAD");
            let listed = git::run(&repo.root, &["log", "--reverse", "--format=%h %s", &range])?;
            listed.lines().map(str::to_string).collect()
        }
        _ => Vec::new(),
    };
    Ok(output::unsaved(
        repo.git_at(),
        &records,
        repo.branch.as_deref(),
        remote,
        &unpushed,
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
    // A branch another worktree has out is neither moved nor switched to:
    // its checkout's index and files would no longer match it.
    if repo.branch.as_deref() != Some(branch) {
        if let Some(path) = git::checkout_of(&root, branch)? {
            return Ok(Outcome::Refused(output::checked_out_elsewhere(
                id, branch, &path,
            )));
        }
    }
    let local = git::branch_tip(&root, branch)?;
    // Bring the local branch to the forge's tip where the forge is ahead,
    // so the take-over builds on what the forge has.
    match &local {
        None => {
            git::create_branch(&root, branch, &tip)?;
        }
        // Only forward, and not under a checkout, which merges it below.
        Some(local)
            if found.remote
                && *local != tip
                && repo.branch.as_deref() != Some(branch)
                && git::is_ancestor(&root, local, &tip)? =>
        {
            git::move_branch(&root, branch, &tip, local)?;
        }
        _ => {}
    }
    if !on_base && repo.branch.as_deref() != Some(branch) {
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

/// `start --mission`: a new mission, created open on a new branch named
/// for it, where its plan is written, with the caller as its planner.
pub fn start_mission(repo: &mut Repo, title: &str) -> Result<Outcome> {
    let id = repo.new_id()?;
    let mission = plan::new_mission(&id, title, repo.caller.clone());
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
        if let Err(error) = claim_push(&root, &remote, &commit, branch, repo.skip) {
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

/// Pushes `commit` to `remote` as the new branch `branch`, which the remote
/// must not have. Git reports a push of a commit the remote already holds
/// there as up to date, and succeeds; only a ref the push created is a
/// claim, so of two claims of one commit one wins.
fn claim_push(
    root: &std::path::Path,
    remote: &str,
    commit: &str,
    branch: &str,
    skip: &atelier_app::Skip,
) -> Result<()> {
    let to = format!("{commit}:refs/heads/{branch}");
    let lease = format!("--force-with-lease=refs/heads/{branch}:");
    let mut command = atelier_app::git::git(root);
    command
        .args(["push", "--porcelain", &lease, remote, &to])
        .stdin(std::process::Stdio::null());
    match skip.reason() {
        Some(reason) => command.env(atelier_app::SKIP_VARIABLE, reason),
        None => command.env_remove(atelier_app::SKIP_VARIABLE),
    };
    let output = command.output().context("cannot start git push")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Each ref is `<flag>\t<from>:<to>\t<summary>`; `*` is a new ref.
    let created = stdout
        .lines()
        .any(|line| line.starts_with("*\t") && line.contains(&format!(":refs/heads/{branch}\t")));
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        anyhow::bail!("git push failed: {}", stderr.trim());
    }
    if !created {
        anyhow::bail!("{remote} already has branch {branch}");
    }
    // What the hooks printed, as the rules a hook skipped.
    eprint!("{stderr}");
    Ok(())
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
    let uncommitted = record_changes(repo)?;
    if !uncommitted.is_empty() {
        let line = output::commit_line(repo.git_at(), &uncommitted, "<what they change>");
        return Ok(Outcome::Refused(output::records_uncommitted(
            &branch,
            &uncommitted,
            &line,
        )));
    }
    // The forge's copy is ahead, as after a reviewer pushed its file: a
    // submit from here would publish a branch without it.
    if let Some(remote) = repo.remote().map(str::to_string) {
        let fetched = atelier_app::git::remote_tip(&repo.root, &remote, &branch)?;
        let head = git::head(&repo.root)?;
        if let Some(tip) = fetched.filter(|tip| *tip != head) {
            if !git::is_ancestor(&repo.root, &tip, &head)? {
                let refused = output::records_behind(&branch, &remote, &tip);
                return Ok(Outcome::Refused(refused));
            }
        }
    }
    // The plans the branch proposes or changes: the caller becomes a
    // planner of each, so that none of its reviewers is.
    let plans = match planned_here(repo)? {
        Ok(plans) => plans,
        Err(refused) => return Ok(refused),
    };
    let config = repo.base_config()?;
    let mut text = match plans.is_empty() {
        true => output::records_pull_request_text(&branch, &repo.base),
        false => output::plan_pull_request_text(&branch, &plans, &repo.base),
    };
    if let [(id, _)] = plans.as_slice() {
        let title = repo.store().mission(id)?.mission.title;
        text.0 = format!("{id}: {title}");
    }
    let shared = match &config.forge {
        Some(forge) => share(repo, forge, &branch, text)?,
        None => String::new(),
    };
    if plans.is_empty() {
        return Ok(Outcome::Done(output::records_submitted(&branch, &shared)));
    }
    let head = git::head(&repo.root)?;
    Ok(Outcome::Done(output::plan_submitted(
        &branch, &plans, &head, &shared,
    )))
}

/// The missions whose plan the checked-out records branch proposes or
/// changes, each with the round of plan review its submit opens, once the
/// caller is written as a planner of each and that is committed; or the
/// refusal of a plan dropped on its branch. A proposed plan's old `draft`
/// is written `open`.
fn planned_here(repo: &mut Repo) -> Result<Result<Vec<(String, u32)>, Outcome>> {
    let head = git::head(&repo.root)?;
    let fork = git::run(&repo.root, &["merge-base", &repo.base_tip, &head])?;
    let changed = git::changed_from_base(&repo.root, &repo.base_tip, &head)?;
    let mut ids: Vec<String> = merge::plans_changed(repo, (&fork, &head), &changed)?
        .into_iter()
        .map(|(id, _, _)| id)
        .collect();
    // A plan on its own branch is submitted with nothing changed yet, as
    // after `start --mission`.
    let own = match repo.named() {
        Some(Named::Record(id)) if !crate::plan::agreed(repo, &id)? => Some(id),
        _ => None,
    };
    if let Some(id) = own.as_ref().filter(|id| !ids.contains(id)) {
        ids.insert(0, id.clone());
    }
    let mut plans = Vec::new();
    let mut written = Vec::new();
    let mut messages = Vec::new();
    for id in ids {
        let dir = repo.store().mission(&id)?;
        if own.as_deref() == Some(id.as_str()) {
            let facts = MissionFacts {
                mission: &dir,
                caller: &repo.caller,
                items: &[],
                review: None,
            };
            if let Some(refused) = repo.skip.mission_gate(Act::SubmitMission, &id, &facts)? {
                return Ok(Err(refused.into()));
            }
        }
        plans.push((id.clone(), next_round(&dir)));
        let mut mission = dir.mission;
        let mut changed = false;
        if mission.status == MissionStatus::Draft && own.as_deref() == Some(id.as_str()) {
            mission.status = MissionStatus::Open;
            changed = true;
        }
        if !mission.planners.0.contains(&repo.caller) {
            messages.push(match mission.planners.holder() {
                Some(before) => format!("Take over the plan of {id} from {before}"),
                None => format!("Plan {id}"),
            });
            mission.planners.take(repo.caller.clone());
            changed = true;
        }
        if changed {
            written.push((mission_path(&id), render_mission(&mission)));
        }
    }
    if !written.is_empty() {
        let paths: Vec<&str> = written.iter().map(|(path, _)| path.as_str()).collect();
        let message = match messages.as_slice() {
            [] => format!("Submit the plan of {}", plans[0].0),
            _ => messages.join("; "),
        };
        write_and_commit(repo, &written, &paths, &message)?;
    }
    Ok(Ok(plans))
}

/// The round of plan review a submit of `dir`'s plan opens: the latest
/// round while a review of it has no verdict, else the next.
fn next_round(dir: &atelier_records::MissionDir) -> u32 {
    let latest = dir.reviews.iter().map(|r| r.name.round).max().unwrap_or(0);
    let open = (dir.reviews.iter()).any(|r| r.name.round == latest && r.review.verdict.is_none());
    match latest > 0 && open {
        true => latest,
        false => latest + 1,
    }
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
