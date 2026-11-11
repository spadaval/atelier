//! `check`, and the three Git hooks that run it as `check --hook <name>`
//! (docs/strategies/redesign-architecture.md, Hooks and the bypass). Hooks
//! guard, commands act: a rule about a commit is checked by the commit, and
//! a rule about a push by the push.

use crate::commands::{self, Outcome};
use crate::repo::{self, Repo};
use crate::{git, output, skip};
use anyhow::{bail, Context, Result};
use atelier_app::acts::changes;
use atelier_core::Status;
use atelier_records::{
    is_record, load, parse_item, parse_mission, CommitReader, Problem, ReadError, Store, CONFIG,
};
use std::io::{ErrorKind, Read};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum Hook {
    PreCommit,
    PrePush,
    PostCheckout,
}

/// `atelier check`: every record in the checkout, and the record changes
/// not committed yet, which change nothing about its outcome.
pub fn check() -> Result<Outcome> {
    let root = git::root()?;
    // Tracked, and untracked whether Git ignores them or not: the commands
    // read every file there.
    let mut paths = git::staged_files(&root, ".atelier")?;
    paths.extend(git::untracked(&root, ".atelier")?);
    paths.sort();
    paths.dedup();
    let mut files = Vec::new();
    for path in paths {
        if !is_record(&path) {
            continue;
        }
        match std::fs::read(root.join(&path)) {
            Ok(bytes) => files.push((path, bytes)),
            // Tracked, and deleted in the checkout.
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => return Err(error).with_context(|| format!("cannot read {path}")),
        }
    }
    let loaded = load(files);
    let counts = (loaded.items.len(), loaded.missions.len());
    let mut uncommitted = repo::planning(&root)?;
    let config = git::uncommitted(&root)?.iter().any(|path| path == CONFIG)
        || !git::untracked(&root, CONFIG)?.is_empty();
    if config {
        uncommitted.insert(0, CONFIG.to_string());
    }
    let (unread, invalid) = (&loaded.unread, &loaded.invalid);
    if unread.is_empty() && invalid.is_empty() {
        return Ok(Outcome::Done(output::checked(counts, &uncommitted)));
    }
    let text = output::check_found(counts, unread, invalid, &uncommitted);
    // What a skip passes is still reported.
    Ok(rules(unread, invalid.iter().map(output::valid), text.clone())
        .unwrap_or(Outcome::Done(text)))
}

/// A refusal `text` of the rules `rules`, which `ATELIER_SKIP` passes when
/// every record read: `None` when it does.
fn rules(
    unread: &[Problem],
    rules: impl IntoIterator<Item = String>,
    text: String,
) -> Option<Outcome> {
    match unread.is_empty() {
        true => skip::refuse(rules, text),
        false => Some(Outcome::Refused(text)),
    }
}

/// `atelier check --hook <hook>`, which the hook's script runs with the
/// arguments Git gave it.
pub fn hook(hook: Hook, args: &[String]) -> Result<Outcome> {
    match hook {
        Hook::PreCommit => pre_commit(&git::root()?),
        Hook::PrePush => {
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input)?;
            pre_push(&git::root()?, &input)
        }
        // Nothing this hook runs or finds may fail the checkout, so it
        // ends in success whatever happens, a panic included.
        Hook::PostCheckout => {
            let said = catch_unwind(AssertUnwindSafe(|| post_checkout(args)));
            Ok(Outcome::Done(said.unwrap_or_default()))
        }
    }
}

/// Every staged record reads and validates, as staged, and each status a
/// record changes from `HEAD`'s is a change some act makes.
fn pre_commit(root: &Path) -> Result<Outcome> {
    let mut reader = CommitReader::new(root);
    let mut files = Vec::new();
    for path in git::staged_files(root, ".atelier")? {
        if is_record(&path) {
            let bytes = reader.staged(&path)?;
            files.push((path, bytes));
        }
    }
    // The whole index is read, so that a staged record is validated against
    // the others; only the staged records' problems are this commit's.
    let loaded = load(files);
    let changed: Vec<PathBuf> = git::staged_changes(root, ".atelier")?
        .into_iter()
        .map(PathBuf::from)
        .collect();
    let staged = |problem: &Problem| changed.contains(&problem.file);
    let unread: Vec<Problem> = loaded.unread.into_iter().filter(staged).collect();
    let invalid: Vec<Problem> = loaded.invalid.into_iter().filter(staged).collect();

    let mut acts = Vec::new();
    if git::has_commit(root, "HEAD")? {
        let head = git::head(root)?;
        let items = loaded.items.iter().map(|dir| (&dir.path, dir.item.status));
        let missions = (loaded.missions.iter()).map(|dir| (&dir.path, dir.mission.status.status()));
        for (path, now) in items.chain(missions) {
            if !changed.contains(path) {
                continue;
            }
            let path = path.to_string_lossy();
            let Some(Ok(before)) = status_at(&mut reader, &head, &path)? else {
                continue;
            };
            let mission = path.ends_with("/mission.md");
            if before != now && !changes(mission, before, now) {
                acts.push((path.into_owned(), before, now));
            }
        }
    }
    if unread.is_empty() && invalid.is_empty() && acts.is_empty() {
        return Ok(Outcome::Done(String::new()));
    }
    let text = output::commit_refused(&unread, &invalid, &acts);
    let broken = (invalid.iter().map(output::valid))
        .chain(acts.iter().map(|(path, from, to)| output::rule_status(path, *from, *to)));
    Ok(rules(&unread, broken, text).unwrap_or(Outcome::Done(String::new())))
}

/// When the push updates the base branch, each new commit on its
/// first-parent line is an Atelier merge or touches only planning, and
/// none but a merge sets an item done while its branch exists. `input` is
/// what Git gives the hook: a line per ref pushed.
fn pre_push(root: &Path, input: &str) -> Result<Outcome> {
    let base = match Store::new(root.join(".atelier")).config() {
        Ok(config) => config.base_branch,
        Err(mut problem) => {
            problem.file = PathBuf::from(CONFIG);
            return Ok(Outcome::Refused(output::push_no_config(&problem)));
        }
    };
    let target = format!("refs/heads/{base}");
    let mut reader = CommitReader::new(root);
    let mut old = None;
    let mut unread = Vec::new();
    let mut code = Vec::new();
    let mut done = Vec::new();
    for line in input.lines() {
        let [_, local, remote_ref, remote] = line.split(' ').collect::<Vec<_>>()[..] else {
            bail!("the pre-push hook was given a line it cannot read: {line:?}");
        };
        // A deletion adds no commit.
        if remote_ref != target || zero(local) {
            continue;
        }
        // A new base, or one this clone has not fetched, has no commit
        // known to be on it, so every commit is new.
        let known = match zero(remote) || !git::has_commit(root, remote)? {
            true => None,
            false => Some(remote),
        };
        old = known;
        for commit in git::first_parent_line(root, local, known)? {
            if commit.atelier_merge && commit.parents.len() > 1 {
                continue;
            }
            let outside: Vec<String> = commit
                .paths
                .iter()
                .filter(|path| !repo::is_planning(path))
                .cloned()
                .collect();
            if !outside.is_empty() {
                code.push((commit.commit.clone(), commit.subject.clone(), outside));
            }
            for path in &commit.paths {
                let Some(id) = path
                    .strip_prefix(".atelier/items/")
                    .and_then(|rest| rest.strip_suffix("/item.md"))
                    .filter(|id| !id.contains('/'))
                else {
                    continue;
                };
                let now = match status_at(&mut reader, &commit.commit, path)? {
                    None => continue,
                    Some(Err(problem)) => {
                        unread.push(problem);
                        continue;
                    }
                    Some(Ok(status)) => status,
                };
                let before = match commit.parents.first() {
                    Some(parent) => status_at(&mut reader, parent, path)?,
                    None => None,
                };
                if now == Status::Done
                    && !matches!(before, Some(Ok(Status::Done)))
                    && git::branch_tip(root, id)?.is_some()
                {
                    done.push((
                        commit.commit.clone(),
                        commit.subject.clone(),
                        id.to_string(),
                    ));
                }
            }
        }
    }
    if unread.is_empty() && code.is_empty() && done.is_empty() {
        return Ok(Outcome::Done(String::new()));
    }
    let text = output::push_refused(&base, old, &unread, &code, &done);
    let broken = (code.iter().map(|(commit, subject, _)| output::rule_code(&base, commit, subject)))
        .chain(done.iter().map(|(commit, _, id)| output::rule_done(&base, commit, id)));
    Ok(rules(&unread, broken, text).unwrap_or(Outcome::Done(String::new())))
}

/// On a branch switch, what `status` prints; else nothing. A failure is
/// printed, never returned.
fn post_checkout(args: &[String]) -> String {
    // Git's third argument is 1 for a switch of branches, 0 for files.
    if args.get(2).map(String::as_str) != Some("1") {
        return String::new();
    }
    match Repo::open().and_then(|mut repo| commands::status(&mut repo)) {
        Ok(said) => said,
        Err(error) => output::no_status(&error),
    }
}

/// The status of the item or mission record at `path` in `commit`: `None`
/// when the commit has no file there, and the problem when it does not
/// parse.
fn status_at(
    reader: &mut CommitReader,
    commit: &str,
    path: &str,
) -> Result<Option<Result<Status, Problem>>> {
    let text = match reader.text(commit, path) {
        Err(ReadError::Missing { .. }) => return Ok(None),
        Err(ReadError::Record(problem)) => return Ok(Some(Err(problem))),
        text => text?,
    };
    let shown = PathBuf::from(format!("{}:{path}", &commit[..commit.len().min(7)]));
    Ok(Some(match path.ends_with("/mission.md") {
        true => parse_mission(&shown, &text).map(|mission| mission.status.status()),
        false => parse_item(&shown, &text).map(|item| item.status),
    }))
}

/// Git's name for no object: all zeros, in SHA-1 or SHA-256.
fn zero(object: &str) -> bool {
    object.bytes().all(|byte| byte == b'0')
}
