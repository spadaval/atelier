//! `merge`, the one command that writes to the base
//! (docs/design.md, Merge and The forge).
//!
//! Copied from `crates/atelier-cli/src/commands/merge.rs` and trimmed to
//! what this command calls: the merged-tree checkout (`MergedCheckout` and
//! its sweep of checkouts a killed merge left, without the lock file each
//! held, since the merge lock covers everything after the wait); the merge
//! commit, built in a scratch index on the merged tree
//! (`git::commit_file`); the fast-forward of the base's checkout; and the
//! confirmation, after the checks and before any write, that the branch
//! tip, the base tip and the checkouts are the ones the checks ran on.
//! From `merge_push.rs`: pushing the branch, finding or opening its pull
//! request, and pushing the base. From
//! `crates/atelier-records/src/mutation_lock.rs`: the exclusive lock,
//! without its shared form and re-entry, which nothing here needs.
//! `committed_snapshot.rs`'s checks of file modes and index flags are not
//! carried over. The wait for CI before the lock, and which runs count for
//! the head, are new.

use crate::commands::{self, hold, Outcome};
use crate::forge::{CiRun, ForgeError, Github, PullRequest};
use crate::git;
use crate::output;
use anyhow::{Context, Result};
use atelier_app::acts::{check, Act, Refusal, STARTED};
use atelier_app::repo::{facts, item_dir, item_path};
use atelier_app::{refused, Located, Repo, Skip};
use atelier_core::{Config, Forge, Status, Thread};
use atelier_records::{is_record, load, parse_item, render_item, CommitReader, CONFIG};
use std::collections::HashMap;
use std::fs::{File, OpenOptions, TryLockError};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// When a question put to the forge under the lock is ended: `LOCKED`
/// from now, and never later than the wait. `review` bounds its look for
/// the pull request the same way.
pub(crate) fn locked_until() -> Instant {
    Instant::now() + LOCKED.min(wait_times().0)
}

/// How long `merge` waits for CI: under the 10 minutes a harness such as
/// Claude Code allows a command in the foreground.
pub(crate) const WAIT: Duration = Duration::from_secs(9 * 60);
/// How often it asks the forge while it waits.
const POLL: Duration = Duration::from_secs(15);
/// How long it gives the forge to answer under the lock, where it holds up
/// every other merge, and never longer than the wait.
const LOCKED: Duration = Duration::from_secs(60);

/// Merges the item of the checked-out branch into the base, in one merge
/// commit that marks it done, once its checks pass. With `trivial`, an
/// item nobody reviewed merges on that reason. Run again, it picks up
/// where the facts now stand. `caller` is who merges, and `skip` the
/// invocation's, whose rules it prints after the last repository it opens.
pub fn merge(root: &Path, caller: Thread, trivial: Option<String>, skip: &Skip) -> Result<Outcome> {
    if trivial.as_ref().is_some_and(|why| why.trim().is_empty()) {
        return Ok(Outcome::Refused(output::no_reason()));
    }
    // Until CI passes nothing is locked and nothing in the clone written,
    // so a merge interrupted then leaves it as it was; everything read
    // before is read again under the lock.
    let judged = match before_lock(root, caller.clone(), trivial.is_some(), skip)? {
        Ok(id) => id,
        Err(outcome) => return Ok(outcome),
    };
    // Everything is read under the lock, so a merge that waited for
    // another reads the base that one left.
    let _lock = lock(root)?;
    let repo = &mut Repo::open(root, caller, skip)?;
    // The branch it waited for is the one it merges.
    if repo.branch.as_deref() != Some(&judged) {
        let moved = Moved::Checkout(repo.root.clone());
        return Ok(Outcome::Refused(output::moved(&moved, &judged, &repo.base)));
    }
    let Judged {
        head,
        id,
        branch,
        records,
        config,
        merged,
        uncommitted,
        ..
    } = match read(repo, trivial.is_some())? {
        Ok(read) => read,
        Err(outcome) => return Ok(outcome),
    };
    if merged {
        return publish(repo, &config, &id);
    }

    // What the checks run on, confirmed unchanged before anything is
    // written.
    hold("ATELIER_TEST_HOLD_AFTER_READ");
    let root = repo.root.clone();
    let base = repo.base.clone();
    let base_tip = repo.base_tip.clone();
    let base_checkout = git::checkout_of(&root, &base)?;
    let mut base_dirt = None;
    if let Some(path) = &base_checkout {
        let dirty = git::uncommitted(path)?;
        if !dirty.is_empty() {
            let text = output::base_dirty(&base, path, &dirty);
            let rule = output::rule_base_dirty(&base, path, &dirty);
            if let Some(refused) = skip.refuse([rule], text) {
                return Ok(refused.into());
            }
        }
        base_dirt = Some(Dirt::read(path, dirty)?);
    }

    let tree = match git::merge_tree(&root, &base_tip, &head)? {
        Ok(tree) => tree,
        Err(conflicts) => return Ok(Outcome::Refused(output::conflicts(&id, &base, &conflicts))),
    };
    // A passing run must still count for the head as it now is.
    if let Some(forge) = &config.forge {
        if let Some(workflow) = &forge.ci_workflow {
            let github = match Github::open(&root, forge) {
                Ok(github) => github.until(locked_until()),
                Err(error) => return Ok(Outcome::Refused(output::forge_failed(&error, false))),
            };
            let mut known = HashMap::new();
            let own = (!records).then(|| item_dir(&id));
            let now = ci(
                &github,
                workflow,
                &root,
                &branch,
                own.as_deref(),
                &head,
                &mut known,
            )?;
            if let Some(stopped) = ci_stops(now, workflow, &head, skip) {
                return Ok(stopped);
            }
        }
    }
    let commit = match records {
        true => {
            let message = output::records_merge_message(&branch, &base);
            let commit = git::commit_tree(&root, &tree, &[&base_tip, &head], &message, skip)?;
            let problems = records_at(&root, &commit)?;
            if !problems.is_empty() {
                return Ok(Outcome::Refused(output::records_invalid(
                    &branch, &base, &problems,
                )));
            }
            (commit, false)
        }
        false => match merge_item(
            &root,
            &tree,
            (&base_tip, &head),
            &id,
            &base,
            &config,
            trivial,
            skip,
        )? {
            Ok(merged) => merged,
            Err(outcome) => return Ok(outcome),
        },
    };
    let base_read = (base_checkout.as_deref(), base_dirt.as_ref());
    finish(
        repo,
        (&head, &base_tip),
        &uncommitted,
        base_read,
        &id,
        commit,
        &config,
        records,
    )
}

/// The merge commit of item `id`'s branch at `head` into the base at
/// `base_tip`, whose merged tree is `tree`, with the item marked done
/// there, once the merge checks pass on it, and whether a skip passed one
/// that failed.
#[allow(clippy::too_many_arguments)]
fn merge_item(
    root: &Path,
    tree: &str,
    (base_tip, head): (&str, &str),
    id: &str,
    base: &str,
    config: &Config,
    trivial: Option<String>,
    skip: &Skip,
) -> Result<Result<(String, bool), Outcome>> {
    // The item as the merged tree holds it, marked done.
    let path = item_path(id);
    let text = git::run(root, &["cat-file", "blob", &format!("{tree}:{path}")])?;
    let mut item = parse_item(Path::new(&path), &text)?;
    item.status = Status::Done;
    if trivial.is_some() {
        item.trivial = trivial;
    }
    let message = output::merge_message(id, &item.title, base, config);
    // Its message carries each rule skipped so far.
    let commit_merge = || {
        let item = render_item(&item);
        git::commit_file(root, tree, &[base_tip, head], &path, &item, &message, skip)
    };
    let mut commit = commit_merge()?;

    let mut skipped = false;
    if !config.merge_checks.is_empty() {
        let checkout = MergedCheckout::add(root, &commit)?;
        for check in &config.merge_checks {
            if let Some((code, tail)) = run_check(&checkout.path, &check.run)? {
                let text = output::check_failed(id, base, &check.run, code, &tail);
                let rule = output::rule_check(&check.run, code);
                if let Some(refused) = skip.refuse([rule], text) {
                    return Ok(Err(refused.into()));
                }
                skipped = true;
            }
        }
        // The same merge, with the checks it skipped in its message.
        if skipped {
            commit = commit_merge()?;
        }
    }
    Ok(Ok((commit, skipped)))
}

/// Moves the base to `commit`, the merge of the branch whose `head` the
/// checks ran on with the base at `base_tip`, once nothing they read has
/// moved, and pushes it where a forge is configured.
#[allow(clippy::too_many_arguments)]
fn finish(
    repo: &mut Repo,
    (head, base_tip): (&str, &str),
    uncommitted: &Dirt,
    (base_checkout, base_dirt): (Option<&Path>, Option<&Dirt>),
    id: &str,
    (commit, skipped): (String, bool),
    config: &Config,
    records: bool,
) -> Result<Outcome> {
    let commit = commit.as_str();
    let (root, base, skip) = (repo.root.clone(), repo.base.clone(), repo.skip);
    let read = (head, base_tip, uncommitted);
    let base_read = base_checkout.zip(base_dirt);
    if let Some(moved) = moved(repo, (id, records), read, base_read)? {
        return Ok(Outcome::Refused(output::moved(&moved, id, &base)));
    }
    match base_checkout {
        Some(path) => git::run(
            path,
            &[
                "-c",
                "core.hooksPath=/dev/null",
                "merge",
                "--ff-only",
                // A local file Git ignores is still someone's work.
                "--no-overwrite-ignore",
                "--quiet",
                commit,
            ],
        ),
        None => git::run(
            &root,
            &[
                "update-ref",
                &format!("refs/heads/{base}"),
                commit,
                base_tip,
            ],
        ),
    }
    .with_context(|| output::not_merged(&base))?;
    // A check a skip passed failed, and its skip's line says so.
    let mut said = match (records, skipped) {
        (true, _) => output::records_checked(&base),
        (false, true) => String::new(),
        (false, false) => output::checks_passed(&base, config.merge_checks.len()),
    };
    said += &match records {
        true => output::records_merged(id, &base, commit),
        false => output::merged(id, &base, commit),
    };
    if let Some(forge) = &config.forge {
        if let Err(error) = push(&root, &forge.remote, commit, &base, skip) {
            return Ok(Outcome::Refused(
                said + &output::base_push_failed(&base, &forge.remote, &error),
            ));
        }
        said += &output::base_pushed(&base, &forge.remote);
    }
    Ok(Outcome::Done(said))
}

/// What `merge` judges, read from the clone as it now stands.
struct Judged {
    /// The commit every later step is checked against.
    head: String,
    /// The item's ID, or for a records branch the branch.
    id: String,
    /// The branch merged.
    branch: String,
    /// The branch holds record changes alone, and merges on `atelier
    /// check` with no review.
    records: bool,
    /// Its pull request's title and body.
    text: (String, String),
    /// The base's.
    config: Config,
    /// The item is done and its branch merged into the base.
    merged: bool,
    /// What the checkout holds that no commit does, which a skip let stay.
    uncommitted: Dirt,
}

/// What a checkout holds that no commit does, as `merge` read it: its
/// HEAD, the paths, their index entries, and each one as it is there, so
/// that a change to what a skip let stay uncommitted is seen as a move.
#[derive(PartialEq)]
struct Dirt {
    head: String,
    paths: Vec<String>,
    index: String,
    entries: Vec<Entry>,
}

#[derive(PartialEq)]
enum Entry {
    Gone,
    /// A file's mode and contents; a symbolic link's are its target.
    File(u32, Vec<u8>),
    /// A directory, which is a submodule, read as a checkout of its own.
    Submodule(Dirt),
}

impl Dirt {
    /// The checkout at `root`, whose `paths` hold what no commit does.
    fn read(root: &Path, paths: Vec<String>) -> Result<Dirt> {
        use std::os::unix::ffi::OsStringExt;
        use std::os::unix::fs::MetadataExt;
        let mut args = vec!["ls-files", "--stage", "-z", "--"];
        args.extend(paths.iter().map(String::as_str));
        // With no path, ls-files would list the whole index.
        let index = match paths.is_empty() {
            true => String::new(),
            false => git::run(root, &args)?,
        };
        let entry = |path: &String| -> Result<Entry> {
            let path = root.join(path);
            let meta = match std::fs::symlink_metadata(&path) {
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    return Ok(Entry::Gone)
                }
                meta => meta?,
            };
            let kind = meta.file_type();
            Ok(if kind.is_symlink() {
                Entry::File(
                    meta.mode(),
                    std::fs::read_link(&path)?.into_os_string().into_vec(),
                )
            } else if kind.is_dir() {
                let mut paths = git::uncommitted(&path)?;
                paths.extend(git::untracked(&path, ".")?);
                Entry::Submodule(Dirt::read(&path, paths)?)
            } else {
                Entry::File(meta.mode(), std::fs::read(&path)?)
            })
        };
        Ok(Dirt {
            head: git::head(root)?,
            entries: paths.iter().map(entry).collect::<Result<_>>()?,
            paths,
            index,
        })
    }
}

/// Reads the head, then the item as the head holds it and the base's
/// config, and checks the act's conditions on them. The outcome is a
/// refusal.
fn read(repo: &mut Repo, trivial: bool) -> Result<Result<Judged, Outcome>> {
    // Read first: a commit made after this is caught before anything is
    // written.
    let head = git::head(&repo.root)?;
    if let Some(branch) = commands::records_branch(repo)? {
        if trivial {
            return Ok(Err(Outcome::Refused(output::trivial_records(&branch))));
        }
        return read_records(repo, head, branch);
    }
    let Some(mut located) = repo.current()? else {
        return Ok(Err(Outcome::Refused(output::not_on_item(
            "merge",
            repo.branch.as_deref(),
        ))));
    };
    let act = if trivial {
        Act::MergeTrivial
    } else {
        Act::Merge
    };
    // A reviewer is known by the review file it holds, which it may not
    // have committed, so its role is told from the checkout, as every
    // command tells it.
    let links = repo.links(located.record().expect("the current item has a record"))?;
    let seen = facts(repo, &located, &links, None);
    if let Err(role @ Refusal::Role(_)) = check(act, &seen) {
        let text = refused(act, &role, &seen)?;
        if let Some(refused) = repo.skip.gate(act, &located.id, &[role], text) {
            return Ok(Err(refused.into()));
        }
    }
    // What merges is the head, so the item is judged as it holds it, not
    // as the checkout does, whose uncommitted changes are a condition of
    // their own. A terminal status on the base wins, so then the head's
    // record is not read.
    let terminal = located.base.as_ref().is_some_and(Located::terminal);
    if let Some(branch) = located.branch.as_mut().filter(|_| !terminal) {
        // The forge's copy is ahead: a drop there stops the merge, and
        // anything else there must be pulled first.
        if branch.remote {
            let dropped =
                (branch.record.as_ref()).is_some_and(|r| r.item.status == Status::Dropped);
            if !dropped {
                let remote = repo.remote().unwrap_or("the remote").to_string();
                let name = repo.branch.clone().unwrap_or_default();
                return Ok(Err(Outcome::Refused(output::behind(
                    &located.id,
                    &name,
                    &remote,
                    &branch.tip,
                ))));
            }
        } else {
            branch.record = repo.item_at(&head, &located.id)?;
        }
    }
    let config = repo.base_config()?;
    let Some(record) = located.record() else {
        // Its record is in the checkout and in no commit.
        return Ok(Err(Outcome::Refused(output::not_on_item(
            "merge",
            repo.branch.as_deref(),
        ))));
    };
    let id = record.item.id.clone();
    let title = record.item.title.clone();
    let merged = record.item.status == Status::Done
        && located.branch.as_ref().is_some_and(|branch| branch.merged);
    let uncommitted = Dirt::read(&repo.root, repo.uncommitted(&id)?)?;
    if !merged {
        let checkout = repo.checkout(record, &head)?;
        let facts = facts(repo, &located, &links, Some(&checkout));
        if let Some(refused) = repo.skip.refusal(act, &facts)? {
            return Ok(Err(refused.into()));
        }
    }
    Ok(Ok(Judged {
        head,
        branch: repo.branch_of(&id),
        text: output::pull_request_text(&id, &title, &repo.base),
        records: false,
        id,
        config,
        merged,
        uncommitted,
    }))
}

/// [`read`] on `branch`, a backlog or mission branch at `head`: every path
/// it changes against the base is under `.atelier/`, it adds no mission
/// and changes no mission's plan, and nothing is uncommitted.
fn read_records(repo: &mut Repo, head: String, branch: String) -> Result<Result<Judged, Outcome>> {
    let root = repo.root.clone();
    let config = repo.base_config()?;
    let merged = git::is_ancestor(&root, &head, &repo.base_tip)?;
    let mut uncommitted = git::uncommitted(&root)?;
    uncommitted.extend(atelier_app::repo::planning(&root)?);
    uncommitted.sort();
    uncommitted.dedup();
    let text = output::records_pull_request_text(&branch, &repo.base);
    let judged = |uncommitted| Judged {
        text: text.clone(),
        head: head.clone(),
        id: branch.clone(),
        branch: branch.clone(),
        records: true,
        config: config.clone(),
        merged,
        uncommitted,
    };
    if merged {
        return Ok(Ok(judged(Dirt::read(&root, uncommitted)?)));
    }
    let changed = git::changed_from_base(&root, &repo.base_tip, &head)?;
    let code: Vec<String> = (changed.iter())
        .filter(|path| !path.starts_with(".atelier/") || *path == CONFIG)
        .cloned()
        .collect();
    if !code.is_empty() {
        return Ok(Err(Outcome::Refused(output::records_code(
            &branch, &repo.base, &code,
        ))));
    }
    // A started item's record changes on its own branch, and reaches the
    // base in its own merge.
    let mut started = Vec::new();
    for path in &changed {
        let Some(id) = path
            .strip_prefix(".atelier/items/")
            .and_then(|rest| rest.strip_suffix("/item.md"))
        else {
            continue;
        };
        let status = repo.item_at(&head, id)?.map(|dir| dir.item.status);
        if let Some(status) = status.filter(|status| STARTED.contains(status)) {
            started.push((id.to_string(), status));
        }
    }
    if !started.is_empty() {
        let text = output::records_started(&branch, &started);
        let rule = output::rule_records_started(&branch);
        if let Some(refused) = repo.skip.refuse([rule], text) {
            return Ok(Err(refused.into()));
        }
    }
    let planned = plans_changed(repo, &head, &changed)?;
    if !planned.is_empty() {
        let text = output::records_plan(&branch, &planned);
        let rule = output::rule_records_plan(&branch);
        if let Some(refused) = repo.skip.refuse([rule], text) {
            return Ok(Err(refused.into()));
        }
    }
    if !uncommitted.is_empty() {
        let text = output::records_dirty(&branch, &uncommitted);
        let rule = output::rule_records_dirty(&branch);
        if let Some(refused) = repo.skip.refuse([rule], text) {
            return Ok(Err(refused.into()));
        }
    }
    Ok(Ok(judged(Dirt::read(&root, uncommitted)?)))
}

/// The missions `changed`, the paths a branch at `head` changes, add or
/// whose plan they change, each with what: its Why, Outcome, Success
/// criteria, Scope or Authority, or a file beside `mission.md` other than
/// `notes.md` and its reviews. atelier-2vdq gives these their gate.
fn plans_changed(repo: &mut Repo, head: &str, changed: &[String]) -> Result<Vec<(String, String)>> {
    // What the branch itself changed: its plan sections as it forked from
    // the base, against its head, so a plan edit the base made since is
    // not taken for the branch's.
    let fork = git::run(&repo.root, &["merge-base", &repo.base_tip, head])?;
    let mut found: Vec<(String, String)> = Vec::new();
    for path in changed {
        let Some(rest) = path.strip_prefix(".atelier/missions/") else {
            continue;
        };
        let Some((id, file)) = rest.split_once('/') else {
            continue;
        };
        if found.iter().any(|(m, _)| m == id) {
            continue;
        }
        let Some(base) = repo.mission_at(&fork, id)? else {
            found.push((id.to_string(), "adds the mission".into()));
            continue;
        };
        if file == "notes.md" || file.starts_with("reviews/") {
            continue;
        }
        if file != "mission.md" {
            found.push((id.to_string(), format!("changes {path}")));
            continue;
        }
        let Some(now) = repo.mission_at(head, id)? else {
            found.push((id.to_string(), "removes the mission".into()));
            continue;
        };
        let sections = ["Why", "Outcome", "Success criteria", "Scope", "Authority"];
        let differ: Vec<&str> = sections
            .into_iter()
            .filter(|h| base.section(h).map(str::trim) != now.section(h).map(str::trim))
            .collect();
        if !differ.is_empty() {
            found.push((id.to_string(), format!("changes its {}", differ.join(", "))));
        }
    }
    Ok(found)
}

/// The problems of the records the commit `commit` holds, as `atelier
/// check` finds them.
fn records_at(root: &Path, commit: &str) -> Result<Vec<String>> {
    let mut reader = CommitReader::new(root);
    let mut files = Vec::new();
    for path in git::files_at(root, commit, ".atelier")? {
        if is_record(&path) {
            let text = reader.text(commit, &path)?;
            files.push((path, text.into_bytes()));
        }
    }
    let loaded = load(files);
    let problems = loaded.unread.iter().chain(&loaded.invalid);
    Ok(problems.map(output::problem_line).collect())
}

/// What comes before the lock where a forge is configured: the branch
/// pushed and its pull request opened, each only when missing, then the
/// wait for CI. It prints as it goes, since the wait is long. It returns
/// the item it judged, or the outcome when `merge` stops here.
fn before_lock(
    root: &Path,
    caller: Thread,
    trivial: bool,
    skip: &Skip,
) -> Result<Result<String, Outcome>> {
    let repo = &mut Repo::open(root, caller, skip)?;
    let judged = match read(repo, trivial)? {
        Ok(judged) => judged,
        Err(outcome) => return Ok(Err(outcome)),
    };
    let Some(forge) = judged.config.forge.as_ref().filter(|_| !judged.merged) else {
        return Ok(Ok(judged.branch));
    };
    let (root, base) = (&repo.root, &repo.base);
    let Judged { head, id, .. } = &judged;
    let branch = &judged.branch;
    let own = (!judged.records).then(|| item_dir(id));
    let own = own.as_deref();
    // CI does not run on a pull request that conflicts.
    if let Err(conflicts) = git::merge_tree(root, &repo.base_tip, head)? {
        return Ok(Err(Outcome::Refused(output::conflicts(
            id, base, &conflicts,
        ))));
    }
    let github = match Github::open(root, forge).and_then(|g| g.signed_in().map(|()| g)) {
        Ok(github) => github,
        Err(error) => return Ok(Err(Outcome::Refused(output::forge_failed(&error, false)))),
    };
    // A push would start CI again, and cancel a run in progress, so the
    // branch is pushed only when no run on the forge's copy would count.
    let mut known = HashMap::new();
    let pushed = match git::remote_tip(root, &forge.remote, branch)? {
        Some(tip) if counts(root, own, head, &tip, &mut known)? => false,
        _ => {
            if let Err(error) = push(root, &forge.remote, head, branch, skip) {
                let failed = output::branch_push_failed(branch, &forge.remote, &error);
                return Ok(Err(Outcome::Refused(failed)));
            }
            true
        }
    };
    let pull = match pull_request(&github, branch, judged.text.clone(), base) {
        Ok(pull) => pull,
        Err(error) => return Ok(Err(Outcome::Refused(output::forge_failed(&error, pushed)))),
    };
    say(&output::pull_request(branch, &forge.remote, &pull, pushed));
    let Some(workflow) = &forge.ci_workflow else {
        return Ok(Ok(judged.branch.clone()));
    };
    let (wait, poll) = wait_times();
    let deadline = Instant::now() + wait;
    let github = github.until(deadline);
    // Said before the first poll, which may itself take the whole bound.
    say(&output::waiting_for_ci(workflow, &pull, wait));
    let mut last = None;
    loop {
        let now = match ci(&github, workflow, root, branch, own, head, &mut known)? {
            // A skip passes a run that has not finished without waiting
            // for it; a forge that does not answer is waited for.
            Ci::Pending(now) if now.is_err() || skip.reason().is_none() => now,
            now => {
                return Ok(match ci_stops(now, workflow, head, skip) {
                    Some(stopped) => Err(stopped),
                    None => Ok(judged.branch.clone()),
                })
            }
        };
        // A poll that fails, or that the bound ends, does not replace what
        // an earlier one said: the last answer, or until one comes, the
        // first failure.
        let pending = match (last.take(), now) {
            (Some(kept), Err(_)) => kept,
            (_, now) => now,
        };
        let left = deadline.saturating_duration_since(Instant::now());
        std::thread::sleep(poll.min(left));
        if left <= poll {
            let pending = output::ci_pending(workflow, head, &pending);
            return Ok(Err(Outcome::NotYet(pending)));
        }
        last = Some(pending);
    }
}

/// What stops `merge` where CI stands `now`: nothing once it passed, or
/// once `ATELIER_SKIP` passed the rule that it pass; else a refusal when it
/// failed, and "not yet" when it has not finished or the forge did not
/// answer, which is no rule.
fn ci_stops(now: Ci, workflow: &str, head: &str, skip: &Skip) -> Option<Outcome> {
    let pending = |now| Some(Outcome::NotYet(output::ci_pending(workflow, head, &now)));
    match now {
        Ci::Passed => None,
        Ci::Failed(run) => {
            let rule = output::rule_ci(workflow, Some(&run));
            skip.refuse([rule], output::ci_failed(workflow, &run))
                .map(Outcome::from)
        }
        Ci::Pending(Ok(run)) if skip.pass([output::rule_ci(workflow, run.as_ref())]) => None,
        Ci::Pending(now) => pending(now),
    }
}

/// Prints `text` at once, while `merge` goes on.
fn say(text: &str) {
    let mut stdout = std::io::stdout();
    let _ = stdout.write_all(text.as_bytes());
    let _ = stdout.flush();
}

/// The bound of the wait and the time between polls. Tests shorten them
/// through `ATELIER_TEST_CI_WAIT`, `<wait ms>,<poll ms>`, which only debug
/// builds read.
fn wait_times() -> (Duration, Duration) {
    #[cfg(debug_assertions)]
    if let Some((wait, poll)) = std::env::var("ATELIER_TEST_CI_WAIT")
        .ok()
        .as_deref()
        .and_then(|times| times.split_once(','))
        .and_then(|(wait, poll)| Some((wait.parse().ok()?, poll.parse().ok()?)))
    {
        return (Duration::from_millis(wait), Duration::from_millis(poll));
    }
    (WAIT, POLL)
}

/// Where CI stands for the head.
enum Ci {
    Passed,
    Failed(CiRun),
    /// No run that counts has finished, or the latest finished one failed
    /// and a newer one is running: the newest that counts. Or the forge did
    /// not answer, and why.
    Pending(Result<Option<CiRun>, ForgeError>),
}

/// Where CI stands for `head`, on `branch`, from the runs that count for
/// it: the latest finished one decides, unless it failed and a newer one
/// is running. `known` keeps what [`counts`] found for each commit, and
/// `own` is the item's directory, if the branch is an item's.
fn ci(
    github: &Github,
    workflow: &str,
    root: &Path,
    branch: &str,
    own: Option<&str>,
    head: &str,
    known: &mut HashMap<String, bool>,
) -> Result<Ci> {
    let runs = match github.branch_runs(workflow, branch) {
        Ok(runs) => runs,
        Err(error) => return Ok(Ci::Pending(Err(error))),
    };
    let mut counting = Vec::new();
    for run in runs {
        if counts(root, own, head, &run.head_sha, known)? {
            counting.push(run);
        }
    }
    counting.sort_by(|a, b| (&a.created_at, a.id).cmp(&(&b.created_at, b.id)));
    let latest = counting.iter().rposition(CiRun::finished);
    Ok(match latest {
        Some(at) if counting[at].passed() => Ci::Passed,
        Some(at) if !counting[at + 1..].iter().any(CiRun::running) => {
            Ci::Failed(counting.swap_remove(at))
        }
        _ => Ci::Pending(Ok(counting.pop())),
    })
}

/// Whether a run on `commit` counts for `head`: `commit` is `head`, or an
/// earlier commit of it from which `head` differs only inside `own`, the
/// item's own directory, as when `verdict approve` committed after CI
/// started.
fn counts(
    root: &Path,
    own: Option<&str>,
    head: &str,
    commit: &str,
    known: &mut HashMap<String, bool>,
) -> Result<bool> {
    if let Some(counts) = known.get(commit) {
        return Ok(*counts);
    }
    let Some(own) = own.map(|own| format!("{own}/")) else {
        return Ok(commit == head);
    };
    let counts = commit == head
        || (git::has_commit(root, commit)?
            && git::is_ancestor(root, commit, head)?
            && git::changed(root, commit, head)?
                .iter()
                .all(|path| path.starts_with(&own)));
    known.insert(commit.to_string(), counts);
    Ok(counts)
}

/// `merge` run again on an item it merged: pushes the base where a forge
/// is configured, since a run that stopped after the merge commit left
/// that undone.
fn publish(repo: &Repo, config: &Config, id: &str) -> Result<Outcome> {
    let base = &repo.base;
    let mut said = output::already_merged(id, base);
    if let Some(Forge { remote, .. }) = &config.forge {
        if let Err(error) = push(&repo.root, remote, &repo.base_tip, base, repo.skip) {
            return Ok(Outcome::Refused(
                said + &output::base_push_failed(base, remote, &error),
            ));
        }
        said += &output::base_pushed(base, remote);
    }
    Ok(Outcome::Done(said))
}

/// What moved while the checks ran.
pub enum Moved {
    Branch,
    Base,
    Checkout(PathBuf),
}

/// The first input of the merge that is no longer what the checks ran on:
/// the branch's tip, the base's tip, or a checkout of either.
fn moved(
    repo: &Repo,
    (id, records): (&str, bool),
    (head, base_tip, uncommitted): (&str, &str, &Dirt),
    base_checkout: Option<(&Path, &Dirt)>,
) -> Result<Option<Moved>> {
    let root = &repo.root;
    let branch = repo.branch.as_deref().expect("merge runs on a branch");
    if git::branch_tip(root, branch)?.as_deref() != Some(head) {
        return Ok(Some(Moved::Branch));
    }
    if git::branch_tip(root, &repo.base)?.as_deref() != Some(base_tip) {
        return Ok(Some(Moved::Base));
    }
    let unchanged = |path: &Path, branch: &str, tip: &str| -> Result<bool> {
        Ok(git::current_branch(path)?.as_deref() == Some(branch) && git::head(path)? == tip)
    };
    // What a skip let the checkout keep uncommitted stays as it was read.
    if !unchanged(root, branch, head)?
        || Dirt::read(root, now_uncommitted(repo, id, records)?)? != *uncommitted
    {
        return Ok(Some(Moved::Checkout(root.clone())));
    }
    let now = git::checkout_of(root, &repo.base)?;
    if now.as_deref() != base_checkout.map(|(path, _)| path) {
        let path = now.or_else(|| base_checkout.map(|(path, _)| path.to_path_buf()));
        return Ok(Some(Moved::Checkout(path.expect("one of them is a path"))));
    }
    match base_checkout {
        Some((path, dirt))
            if !unchanged(path, &repo.base, base_tip)?
                || Dirt::read(path, git::uncommitted(path)?)? != *dirt =>
        {
            Ok(Some(Moved::Checkout(path.to_path_buf())))
        }
        _ => Ok(None),
    }
}

/// What the checkout holds that no commit does, as [`read`] gathers it:
/// for the item `id`, or for a records branch.
fn now_uncommitted(repo: &Repo, id: &str, records: bool) -> Result<Vec<String>> {
    if !records {
        return repo.uncommitted(id);
    }
    let mut paths = git::uncommitted(&repo.root)?;
    paths.extend(atelier_app::repo::planning(&repo.root)?);
    paths.sort();
    paths.dedup();
    Ok(paths)
}

/// Pushes `commit` to the branch `branch` of `remote`, never forcing, with
/// the pre-push hook given `skip`'s reason.
pub(crate) fn push(
    root: &Path,
    remote: &str,
    commit: &str,
    branch: &str,
    skip: &Skip,
) -> Result<()> {
    let to = format!("{commit}:refs/heads/{branch}");
    git::relayed(root, &["push", "--quiet", remote, &to], skip)
}

/// The open pull request from `branch` into `base`, opened with `text`,
/// its title and body, when there is none.
pub(crate) fn pull_request(
    github: &Github,
    branch: &str,
    text: (String, String),
    base: &str,
) -> Result<PullRequest, ForgeError> {
    if let Some(pull) = github.open_pull_request(branch, base)? {
        return Ok(pull);
    }
    let (title, body) = text;
    github.create_pull_request(branch, base, &title, &body)
}

/// Runs one merge check in `dir`. When it fails, returns its exit code and
/// the end of its output.
fn run_check(dir: &Path, run: &str) -> Result<Option<(Option<i32>, String)>> {
    let (mut reader, writer) = std::io::pipe()?;
    let mut child = Command::new("sh")
        .args(["-c", run])
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(writer.try_clone()?)
        .stderr(writer)
        .spawn()
        .with_context(|| format!("cannot start the merge check `{run}`"))?;
    let mut text = Vec::new();
    // The command and its writers hold the pipe's other end; reading ends
    // when the last of them exits.
    reader.read_to_end(&mut text)?;
    let status = child.wait()?;
    if status.success() {
        return Ok(None);
    }
    let text = String::from_utf8_lossy(&text);
    let lines: Vec<&str> = text.lines().collect();
    let tail = lines[lines.len().saturating_sub(20)..].join("\n");
    Ok(Some((status.code(), tail)))
}

/// The lock that serialises merges in a clone: an exclusive lock on a file
/// in the Git directory all its worktrees share, held from the read after
/// the wait for CI to the end of the merge.
/// A second merge waits for it; the operating system releases it when a
/// merge ends, killed or not.
fn lock(root: &Path) -> Result<File> {
    let path = git::common_dir(root)?.join("atelier-merge.lock");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&path)
        .with_context(|| format!("cannot open {}", path.display()))?;
    match file.try_lock() {
        Ok(()) => return Ok(file),
        Err(TryLockError::WouldBlock) => eprint!("{}", output::waiting()),
        Err(TryLockError::Error(error)) => return Err(error.into()),
    }
    file.lock()
        .with_context(|| format!("cannot lock {}", path.display()))?;
    Ok(file)
}

const MERGE_DIR_PREFIX: &str = "atelier-merge-";

/// A detached checkout of the merge commit in the Git directory, outside
/// every working tree, so the merge checks run on exactly the tree merging
/// puts on the base. `Drop` removes it; a killed merge cannot, and the next
/// merge, which holds the lock, removes what it left.
struct MergedCheckout {
    repo: PathBuf,
    path: PathBuf,
}

impl MergedCheckout {
    fn add(repo: &Path, commit: &str) -> Result<Self> {
        let common = git::common_dir(repo)?;
        Self::sweep(repo, &common)?;
        let path = common.join(format!("{MERGE_DIR_PREFIX}{}", std::process::id()));
        let shown = path.to_string_lossy();
        git::run(
            repo,
            &[
                "-c",
                "core.hooksPath=/dev/null",
                "worktree",
                "add",
                "--quiet",
                "--detach",
                &shown,
                commit,
            ],
        )
        .context("cannot check out the merged tree for the merge checks")?;
        Ok(Self {
            repo: repo.to_path_buf(),
            path,
        })
    }

    /// Removes the checkouts earlier merges left when they were killed
    /// during their checks. Under the lock, none belongs to a live merge.
    fn sweep(repo: &Path, common: &Path) -> Result<()> {
        for entry in std::fs::read_dir(common)? {
            let dir = entry?.path();
            let leftover = dir
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(MERGE_DIR_PREFIX) && !name.ends_with(".lock"));
            if leftover {
                std::fs::remove_dir_all(&dir)
                    .with_context(|| format!("cannot remove {}", dir.display()))?;
            }
        }
        git::run(repo, &["worktree", "prune"]).map(drop)
    }
}

impl Drop for MergedCheckout {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
        let _ = git::run(&self.repo, &["worktree", "prune"]);
    }
}
