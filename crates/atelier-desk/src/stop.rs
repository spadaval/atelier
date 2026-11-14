//! The human's stop of a proposed plan, carried through by the Desk
//! (docs/architecture/desk.md, The stop): the drop written by the core's
//! own function in a clone of the Desk's, committed, pushed to the plan's
//! branch on the forge's remote, and the branch's pull request closed.
//! Nothing is written in a served repository's checkouts; the served
//! repository is only read and fetched.
//!
//! An open mission's stop is published by `submit` and `merge` of a
//! record-only branch, which live in `atelier-cli` until atelier-yclc moves
//! them into the core. Until then the Desk does not make it, and the
//! mission's page gives the commands.

use crate::api::ActionResponse;
use crate::{fetch, read};
use anyhow::{bail, Context, Result};
use atelier_core::model::{MissionStatus, Thread};
use atelier_core::{forge, git, Done, Refused, Repo, Skip};
use std::path::{Path, PathBuf};

/// What the stop needs besides the request.
pub struct Stopping<'a> {
    /// The served repository, which is only read and fetched.
    pub served: &'a Path,
    /// Its name, which names the Desk's clone of it.
    pub name: &'a str,
    /// The Desk's state directory; its clones are under `work/`.
    pub state: &'a Path,
    pub caller: &'a str,
    pub gh: &'a Path,
}

fn refused(text: impl Into<String>, next: Option<String>) -> ActionResponse {
    ActionResponse::Refused {
        rules: Vec::new(),
        text: text.into(),
        next,
    }
}

/// Stops the proposed plan of mission `id` with `reason`, or refuses,
/// changing nothing.
pub fn stop(at: &Stopping, id: &str, reason: &str) -> Result<ActionResponse> {
    let reason = reason.trim();
    if reason.is_empty() || reason.contains(['\n', '\r']) {
        return Ok(refused(
            "A stop needs your reason, on one line. Nothing changed.",
            None,
        ));
    }
    let skip = Skip::new(None);
    let (remote, branch, base) = {
        let mut repo = read::open(at.served, at.caller, &skip)?;
        let located = repo.find_mission(id)?;
        let Some(record) = located.record() else {
            return Ok(refused(
                format!("No mission {id} is found on the base or on a branch. Nothing changed."),
                None,
            ));
        };
        if record.mission.status == MissionStatus::Dropped {
            return Ok(refused(
                format!(
                    "Mission {id} is stopped already: {}",
                    record.dropped().unwrap_or("dropped")
                ),
                None,
            ));
        }
        if !located.proposed() {
            return Ok(refused(
                format!("Mission {id} is agreed: its plan has merged. The Desk does not publish the stop of an open mission yet (atelier-yclc), so it changed nothing."),
                Some("Run the commands its page gives in a checkout of the repository.".into()),
            ));
        }
        let Some(remote) = repo.remote().map(str::to_string) else {
            return Ok(refused(
                format!("This repository names no forge remote, so a stop of {id} is published by committing the drop on its branch in the planner's checkout. Nothing changed."),
                Some(format!("There: atelier drop {id} --reason \"<why>\", then commit it.")),
            ));
        };
        let branch = repo.branch_of(id);
        if git::remote_tip(&repo.root, &remote, &branch)?.is_none() {
            return Ok(refused(
                format!("The plan's branch {branch} is not on {remote} as this clone last fetched it, so the Desk cannot publish a stop of it. Nothing changed."),
                Some("Try again after the next fetch, or once its planner has pushed it.".into()),
            ));
        }
        (remote, branch, repo.base.clone())
    };
    let clone = clone_of(at, &remote)?;
    // The plan's branch as the forge has it now, and the base.
    let fetched = |spec: String| run(&clone, &["fetch", "--quiet", &remote, &spec]);
    fetched(format!(
        "+refs/heads/{branch}:refs/remotes/{remote}/{branch}"
    ))?;
    fetched(format!("+refs/heads/{base}:refs/remotes/{remote}/{base}"))?;
    let tracking = format!("refs/remotes/{remote}/{branch}");
    run(
        &clone,
        &["checkout", "--quiet", "--force", "-B", &branch, &tracking],
    )?;
    run(&clone, &["clean", "--quiet", "-fdx"])?;
    run(
        &clone,
        &[
            "branch",
            "--quiet",
            "-f",
            &base,
            &format!("refs/remotes/{remote}/{base}"),
        ],
    )?;
    let skip = Skip::new(None);
    let mut repo = Repo::open(&clone, Thread(at.caller.to_string()), &skip)?;
    // The plan may have merged since the served repository last fetched:
    // the clone has the forge's base as it is now. An agreed mission's stop
    // reaches the base by a merge, which the Desk does not make yet.
    if atelier_core::plan::agreed(&mut repo, id)? {
        let _ = fetch::fetch(at.served, &remote);
        return Ok(refused(
            format!("Mission {id} merged since the Desk last fetched, so it is agreed now: its stop reaches the base by a merge, which the Desk does not make yet (atelier-yclc). Nothing changed."),
            Some("Run the commands its page gives in a checkout of the repository.".into()),
        ));
    }
    let done = match atelier_core::plan::drop_mission(&mut repo, id, reason)? {
        Ok(done) => done,
        Err(Refused { rules, text, next }) => {
            return Ok(ActionResponse::Refused { rules, text, next })
        }
    };
    let paths: Vec<&str> = done.paths.iter().map(String::as_str).collect();
    git::commit_paths(&clone, &paths, &format!("Drop {id}"), &skip)
        .context("the Desk could not commit the drop in its clone")?;
    let commit = git::head(&clone)?;
    if let Err(error) = push(&clone, &remote, &branch) {
        bail!("the drop of {id} is committed in the Desk's clone and was not pushed, so nothing is stopped: {error:#}. Stop it again once the page shows what its branch holds now");
    }
    let comment = format!("Stopped from the Desk by {}: {reason}", at.caller);
    let closed = forge::close_pull_request(at.gh, &clone, &branch, &comment);
    // The served repository sees the push at once.
    let _ = fetch::fetch(at.served, &remote);
    let Done { text, paths, .. } = done;
    let said = match closed {
        Ok(true) => format!("Pushed {branch} to {remote} and closed its pull request. The plan is stopped."),
        Ok(false) => format!("Pushed {branch} to {remote}. It has no open pull request. The plan is stopped."),
        Err(error) => format!("Pushed {branch} to {remote}, but its pull request was not closed: {error:#}. The stop is published once it is closed: gh pr close {branch}."),
    };
    Ok(ActionResponse::Done {
        text: format!("{}\n{said}\n", first_line(&text)),
        paths,
        commit: Some(commit),
    })
}

fn first_line(text: &str) -> &str {
    text.lines().next().unwrap_or_default()
}

/// The Desk's clone of the served repository, made on first use from the
/// served one's objects, with its remote pointed where the served one's
/// is and its committer named as the served one names it.
fn clone_of(at: &Stopping, remote: &str) -> Result<PathBuf> {
    let clone = at.state.join("work").join(at.name);
    if !clone.join(".git").exists() {
        std::fs::create_dir_all(clone.parent().expect("a clone has a parent"))?;
        let _ = std::fs::remove_dir_all(&clone);
        let source = at.served.display().to_string();
        let target = clone.display().to_string();
        // Run where the Desk runs, which the paths may be relative to.
        let made = git::git(Path::new("."))
            .args([
                "clone",
                "--quiet",
                "--no-checkout",
                "--origin",
                remote,
                &source,
                &target,
            ])
            .output()?;
        if !made.status.success() {
            bail!(
                "cannot clone {source}: {}",
                String::from_utf8_lossy(&made.stderr).trim()
            );
        }
    }
    let url = git::run(at.served, &["remote", "get-url", remote])?;
    run(&clone, &["remote", "set-url", remote, &url])?;
    let push = git::run(
        at.served,
        &["config", "--get", &format!("remote.{remote}.pushurl")],
    );
    match push {
        Ok(push) if !push.is_empty() => {
            run(&clone, &["remote", "set-url", "--push", remote, &push])?
        }
        _ => {
            let _ = git::run(
                &clone,
                &["config", "--unset", &format!("remote.{remote}.pushurl")],
            );
        }
    }
    for key in ["user.name", "user.email"] {
        if let Ok(value) = git::run(at.served, &["config", "--get", key]) {
            run(&clone, &["config", key, &value])?;
        }
    }
    Ok(clone)
}

fn run(dir: &Path, args: &[&str]) -> Result<()> {
    let output = git::unattended(&mut git::git(dir))
        .args(args)
        .output()
        .with_context(|| format!("cannot run git {}", args.join(" ")))?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}

/// Pushes `branch`, never forcing: a branch that moved on the remote
/// since the fetch refuses it.
fn push(clone: &Path, remote: &str, branch: &str) -> Result<()> {
    run(
        clone,
        &[
            "push",
            "--quiet",
            remote,
            &format!("refs/heads/{branch}:refs/heads/{branch}"),
        ],
    )
}
