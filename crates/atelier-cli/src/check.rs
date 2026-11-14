//! `check`, and the three Git hooks that run it as `check --hook <name>`
//! (docs/design.md, Hooks and the bypass). Hooks
//! guard, commands act: a rule about a commit is checked by the commit, and
//! a rule about a push by the push.

use crate::commands::{self, Outcome};
use crate::{git, output, setup};
use anyhow::{bail, Context, Result};
use atelier_app::acts::changes;
use atelier_app::repo::{is_planning, planning};
use atelier_app::Skip;
use atelier_core::Status;
use atelier_records::{
    is_record, load, parse_config, parse_item, parse_mission, CommitReader, Problem, ReadError,
    Store, CONFIG,
};
use clap::ValueEnum;
use std::io::{ErrorKind, Read, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum Hook {
    PreCommit,
    PrePush,
    PostCheckout,
}

/// `atelier check`: every record in the checkout, and the record changes
/// not committed yet, which change nothing about its outcome.
pub fn check(skip: &Skip) -> Result<Outcome> {
    let root = git::root(Path::new("."))?;
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
    let mut uncommitted = planning(&root)?;
    let config = git::uncommitted(&root)?.iter().any(|path| path == CONFIG)
        || !git::untracked(&root, CONFIG)?.is_empty();
    if config {
        uncommitted.insert(0, CONFIG.to_string());
    }
    let (unread, invalid) = (&loaded.unread, &loaded.invalid);
    // Nothing refuses a mission whose Why is not written yet, as a plan
    // just started; `check` says so.
    let unwritten: Vec<&str> = (loaded.missions.iter())
        .filter(|dir| dir.section("Why").is_none_or(|why| why.trim().is_empty()))
        .map(|dir| dir.mission.id.as_str())
        .collect();
    let why = output::why_unwritten(&unwritten);
    if unread.is_empty() && invalid.is_empty() {
        return Ok(Outcome::Done(output::checked(counts, &uncommitted) + &why));
    }
    let text = output::check_found(counts, unread, invalid, &uncommitted) + &why;
    // What a skip passes is still reported.
    Ok(rules(
        skip,
        unread,
        invalid.iter().map(output::valid),
        text.clone(),
    )
    .unwrap_or(Outcome::Done(text)))
}

/// A refusal `text` of the rules `rules`, which `ATELIER_SKIP` passes when
/// every record read: `None` when it does.
fn rules(
    skip: &Skip,
    unread: &[Problem],
    rules: impl IntoIterator<Item = String>,
    text: String,
) -> Option<Outcome> {
    match unread.is_empty() {
        true => skip.refuse(rules, text).map(Outcome::from),
        false => Some(Outcome::Refused(text)),
    }
}

/// `atelier check --hook <hook>`, which the hook's script runs with the
/// arguments and input Git gave it. Once its own rule passes, the clone's
/// own hook, which `setup` kept as `<hook>.local`, runs with the same.
pub fn hook(hook: Hook, args: &[String], skip: &Skip) -> Result<Outcome> {
    let name = hook.to_possible_value().expect("hooks are named");
    let name = name.get_name();
    let (outcome, input) = match hook {
        Hook::PreCommit => (pre_commit(&git::root(Path::new("."))?, skip)?, None),
        Hook::PrePush => {
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input)?;
            (
                pre_push(&git::root(Path::new("."))?, &input, skip)?,
                Some(input),
            )
        }
        // Nothing this hook runs or finds may fail the checkout, so it
        // ends in success whatever happens, a panic included.
        Hook::PostCheckout => {
            let said = catch_unwind(AssertUnwindSafe(|| post_checkout(args, skip)));
            print!("{}", said.unwrap_or_default());
            let _ = std::io::stdout().flush();
            let failed = match catch_unwind(AssertUnwindSafe(|| kept(name, args, None))) {
                Ok(Ok(failed)) => failed,
                Ok(Err(error)) => Some(format!("{error:#}")),
                Err(_) => Some("it could not be run".into()),
            };
            let said = failed.map(|why| output::kept_failed(name, &why));
            return Ok(Outcome::Done(said.unwrap_or_default()));
        }
    };
    if !matches!(outcome, Outcome::Done(_)) {
        return Ok(outcome);
    }
    Ok(match kept(name, args, input.as_deref())? {
        None => outcome,
        Some(why) => Outcome::Refused(output::kept_refused(name, &why)),
    })
}

/// Runs the clone's own hook `name`, kept as `<name>.local`, with Git's
/// arguments and input, and says how it failed, if it did. One that is
/// absent or not executable is not run, as Git would not run it.
fn kept(name: &str, args: &[String], input: Option<&str>) -> Result<Option<String>> {
    let path = git::hooks_dir(&git::root(Path::new("."))?)?.join(format!("{name}.local"));
    if !setup::runs(&path) {
        return Ok(None);
    }
    let spawn = |command: &mut Command| {
        let stdin = match input {
            Some(_) => Stdio::piped(),
            None => Stdio::inherit(),
        };
        command.args(args).stdin(stdin).spawn()
    };
    // Git runs a script with no `#!` line through the shell, as when exec
    // fails with ENOEXEC, so the kept hook is run the same way.
    const ENOEXEC: i32 = 8;
    let spawned = match spawn(&mut Command::new(&path)) {
        Err(error) if error.raw_os_error() == Some(ENOEXEC) => {
            spawn(Command::new("/bin/sh").arg(&path))
        }
        spawned => spawned,
    };
    let mut child = match spawned {
        Ok(child) => child,
        Err(error) => return Ok(Some(format!("it could not start: {error}"))),
    };
    if let (Some(input), Some(mut stdin)) = (input, child.stdin.take()) {
        // A hook that does not read its input closes the pipe early.
        match stdin.write_all(input.as_bytes()) {
            Err(error) if error.kind() != ErrorKind::BrokenPipe => return Err(error.into()),
            _ => {}
        }
    }
    let status = child.wait()?;
    Ok(match (status.success(), status.code()) {
        (true, _) => None,
        (false, Some(code)) => Some(format!("it exited with {code}")),
        (false, None) => Some(format!("it ended with {status}")),
    })
}

/// Every staged record reads and validates, as staged, and each status a
/// record changes from `HEAD`'s is a change some act makes.
fn pre_commit(root: &Path, skip: &Skip) -> Result<Outcome> {
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
    // Only a merge writes the base, so no record changes in a commit on it.
    // The base is the one `HEAD`'s config names, which a change in the
    // checkout, staged or not, cannot move; only a repository's first
    // commit, or one whose `HEAD` holds none that reads, reads the
    // checkout's. Where neither names one, the commit is refused, since the
    // rule cannot be judged.
    let has_head = git::has_commit(root, "HEAD")?;
    let at_head = match has_head {
        true => match reader.text(&git::head(root)?, CONFIG) {
            Ok(text) => parse_config(Path::new(CONFIG), &text)
                .ok()
                .map(|config| config.base_branch),
            Err(_) => None,
        },
        false => None,
    };
    let base = at_head.or_else(|| {
        Store::new(root.join(".atelier"))
            .config()
            .ok()
            .map(|config| config.base_branch)
    });
    let branch = git::current_branch(root)?;
    let on_base = match base {
        Some(base) => (branch.as_deref() == Some(base.as_str())).then_some(base),
        None if has_head => {
            let rule = output::rule_no_base();
            return Ok(rules(skip, &[], [rule], output::commit_no_base())
                .unwrap_or(Outcome::Done(String::new())));
        }
        None => None,
    };
    let base_records = match &on_base {
        Some(_) => git::staged_all(root, ".atelier")?,
        None => Vec::new(),
    };
    if unread.is_empty() && invalid.is_empty() && acts.is_empty() && base_records.is_empty() {
        return Ok(Outcome::Done(String::new()));
    }
    let base = on_base.unwrap_or_default();
    let text = output::commit_refused(&unread, &invalid, &acts, &base, &base_records);
    let broken = (invalid.iter().map(output::valid))
        .chain(
            acts.iter()
                .map(|(path, from, to)| output::rule_status(path, *from, *to)),
        )
        .chain((!base_records.is_empty()).then(|| output::rule_record_on_base(&base)));
    Ok(rules(skip, &unread, broken, text).unwrap_or(Outcome::Done(String::new())))
}

/// When the push updates the base branch, each new commit on its
/// first-parent line is an Atelier merge or touches only planning, and
/// none but a merge sets an item done while its branch exists. `input` is
/// what Git gives the hook: a line per ref pushed.
fn pre_push(root: &Path, input: &str, skip: &Skip) -> Result<Outcome> {
    let config = match Store::new(root.join(".atelier")).config() {
        Ok(config) => config,
        Err(mut problem) => {
            problem.file = PathBuf::from(CONFIG);
            return Ok(Outcome::Refused(output::push_no_config(&problem)));
        }
    };
    let base = config.base_branch.clone();
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
                .filter(|path| !is_planning(path))
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
                    && git::branch_tip(root, &config.record_branch(id))?.is_some()
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
    let broken = (code
        .iter()
        .map(|(commit, subject, _)| output::rule_code(&base, commit, subject)))
    .chain(
        done.iter()
            .map(|(commit, _, id)| output::rule_done(&base, commit, id)),
    );
    Ok(rules(skip, &unread, broken, text).unwrap_or(Outcome::Done(String::new())))
}

/// On a branch switch, what `status` prints; else nothing. A failure is
/// printed, never returned.
fn post_checkout(args: &[String], skip: &Skip) -> String {
    // Git's third argument is 1 for a switch of branches, 0 for files.
    if args.get(2).map(String::as_str) != Some("1") {
        return String::new();
    }
    let opened = std::env::current_dir()
        .map_err(anyhow::Error::from)
        .and_then(|here| crate::open(&here, skip));
    match opened.and_then(|mut repo| commands::status(&mut repo)) {
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
