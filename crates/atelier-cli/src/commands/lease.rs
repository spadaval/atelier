//! Runtime leases: which worktree is working an owner's item.
//!
//! A lease is a small JSON file under `.atelier/runtime/leases/` of the
//! repository's primary checkout, which every linked worktree shares (a
//! worktree's own `.atelier/runtime` is private to it). `start` writes it,
//! `status` refreshes it, and neither is a lock: a lease only lets a second
//! session learn that the item is already held (atelier-81e5).
//!
//! A lease is live while its worktree still exists and its heartbeat is under
//! [`STALE_AFTER`]. A deleted worktree expires it at once; so does the same
//! worktree starting a different owner, which releases its other leases. A
//! crashed session expires with its heartbeat. The branch is not consulted: a
//! holder that merely switched away is indistinguishable from one still
//! working, and `--take-over` settles it. Process IDs are not used: `start` is
//! short-lived, and the agent process that outlives it is not known to
//! Atelier. Two sessions in one worktree cannot be told apart.
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

use crate::commands::git;

pub const STALE_AFTER: Duration = Duration::hours(4);

#[derive(Debug, Clone)]
pub struct Lease {
    pub owner: String,
    pub worktree: PathBuf,
    pub branch: String,
    pub heartbeat: DateTime<Utc>,
}

/// A lease held by another worktree: live, or stale with the reason.
pub enum Held {
    Live(Lease),
    Stale(Lease, &'static str),
}

fn directory(repo: &Path) -> Result<PathBuf> {
    let common = PathBuf::from(git::text(
        repo,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?);
    let primary = match common.file_name().is_some_and(|name| name == ".git") {
        true => common.parent().unwrap_or(&common).to_path_buf(),
        false => common,
    };
    Ok(primary.join(".atelier/runtime/leases"))
}

fn worktree_root(repo: &Path) -> PathBuf {
    std::fs::canonicalize(repo).unwrap_or_else(|_| repo.to_path_buf())
}

fn read(repo: &Path, owner: &str) -> Result<Option<Lease>> {
    let path = directory(repo)?.join(format!("{owner}.json"));
    let Ok(bytes) = std::fs::read(&path) else {
        return Ok(None);
    };
    // A lease that cannot be read holds nothing.
    let Ok(value) = serde_json::from_slice::<Value>(&bytes) else {
        return Ok(None);
    };
    let field = |name: &str| value[name].as_str();
    let (Some(worktree), Some(branch), Some(heartbeat)) =
        (field("worktree"), field("branch"), field("heartbeat"))
    else {
        return Ok(None);
    };
    let Ok(heartbeat) = DateTime::parse_from_rfc3339(heartbeat) else {
        return Ok(None);
    };
    Ok(Some(Lease {
        owner: owner.to_owned(),
        worktree: PathBuf::from(worktree),
        branch: branch.to_owned(),
        heartbeat: heartbeat.with_timezone(&Utc),
    }))
}

fn write(repo: &Path, owner: &str, branch: &str) -> Result<()> {
    let directory = directory(repo)?;
    std::fs::create_dir_all(&directory)?;
    let body = json!({
        "worktree": worktree_root(repo),
        "branch": branch,
        "heartbeat": Utc::now().to_rfc3339(),
    });
    let staged = tempfile::NamedTempFile::new_in(&directory)?;
    std::fs::write(staged.path(), serde_json::to_vec_pretty(&body)?)?;
    staged
        .persist(directory.join(format!("{owner}.json")))
        .map_err(|error| error.error)
        .context("cannot write the session lease")?;
    Ok(())
}

/// The lease on `owner` if a worktree other than `repo` holds it.
pub fn held_by_other(repo: &Path, owner: &str) -> Result<Option<Held>> {
    let Some(lease) = read(repo, owner)? else {
        return Ok(None);
    };
    if lease.worktree == worktree_root(repo) {
        return Ok(None);
    }
    Ok(Some(if !lease.worktree.exists() {
        Held::Stale(lease, "its worktree no longer exists")
    } else if Utc::now() - lease.heartbeat > STALE_AFTER {
        Held::Stale(lease, "its heartbeat is over 4 hours old")
    } else {
        Held::Live(lease)
    }))
}

/// Records that `repo` holds `owner`, replacing any earlier lease.
pub fn claim(repo: &Path, owner: &str, branch: &str) -> Result<()> {
    // A worktree works one owner at a time: starting this one releases the
    // others it held.
    let directory = directory(repo)?;
    if let Ok(entries) = std::fs::read_dir(&directory) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let Some(other) = name.strip_suffix(".json").filter(|other| *other != owner) else {
                continue;
            };
            if matches!(read(repo, other), Ok(Some(lease)) if lease.worktree == worktree_root(repo))
            {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }
    write(repo, owner, branch)
}

/// Renews the lease on `owner` when `repo` already holds it.
pub fn refresh(repo: &Path, owner: &str, branch: &str) {
    if matches!(read(repo, owner), Ok(Some(lease)) if lease.worktree == worktree_root(repo)) {
        let _ = write(repo, owner, branch);
    }
}

/// One line describing a lease for output.
pub fn describe(lease: &Lease) -> String {
    let minutes = (Utc::now() - lease.heartbeat).num_minutes().max(0);
    format!(
        "{} on {} (heartbeat {}m ago)",
        lease.worktree.display(),
        lease.branch,
        minutes
    )
}
