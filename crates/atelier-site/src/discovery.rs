//! Finding every Atelier repository under the roots, once each, and the
//! commit of its base branch, without a checkout (atelier-zpmb).
//!
//! A repository is found through any checkout of it that holds `.atelier/`,
//! and is named by its Git common directory, so a repository with several
//! worktrees appears once. Its base branch is read from refs and commits
//! alone: the default branch (`origin/HEAD`, else whichever one of local
//! `main` and `master` exists), the workflow policy committed there, and
//! the `base_branch` that policy names, whose own committed policy must name
//! it too. Nothing falls back to `HEAD` or a working tree.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Result};
use atelier_app::git_freshness::{git_output, ReadContext};

/// Directories never searched for checkouts: build output and dependencies.
const SKIPPED: &[&str] = &["node_modules", "target"];
/// How deep below a root checkouts are looked for.
const MAX_DEPTH: usize = 4;

/// One repository, found through one or more checkouts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found {
    /// The Git common directory, or why it could not be read.
    pub common_dir: std::result::Result<PathBuf, String>,
    /// The checkouts it was found through, in path order.
    pub checkouts: Vec<PathBuf>,
}

impl Found {
    /// The name a repository goes by before its records say: its directory.
    pub fn directory_name(&self) -> String {
        let path = match &self.common_dir {
            Ok(common_dir) if common_dir.file_name().is_some_and(|name| name == ".git") => {
                common_dir.parent().unwrap_or(common_dir)
            }
            Ok(common_dir) => common_dir.as_path(),
            Err(_) => self.checkouts[0].as_path(),
        };
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "repository".to_owned());
        name.strip_suffix(".git").unwrap_or(&name).to_owned()
    }
}

/// Every repository with a checkout holding `.atelier/` under `roots`, once
/// per Git common directory, ordered by that directory. A checkout is not
/// searched further, nor are hidden directories, `node_modules` or `target`.
pub fn discover(roots: &[PathBuf]) -> Vec<Found> {
    let mut checkouts = Vec::new();
    for root in roots {
        collect(root, 0, &mut checkouts);
    }
    checkouts.sort();
    checkouts.dedup();
    let mut found = BTreeMap::<std::result::Result<PathBuf, String>, Vec<PathBuf>>::new();
    for checkout in checkouts {
        let common_dir = common_dir(&checkout).map_err(|error| {
            format!(
                "cannot read the Git directory of {}: {error:#}",
                checkout.display()
            )
        });
        found.entry(common_dir).or_default().push(checkout);
    }
    found
        .into_iter()
        .map(|(common_dir, checkouts)| Found {
            common_dir,
            checkouts,
        })
        .collect()
}

fn collect(dir: &Path, depth: usize, checkouts: &mut Vec<PathBuf>) {
    if dir.join(".git").exists() {
        if dir.join(".atelier").is_dir() {
            checkouts.push(fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf()));
        }
        return;
    }
    if depth >= MAX_DEPTH {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut children = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| !name.starts_with('.') && !SKIPPED.contains(&name))
        })
        .collect::<Vec<_>>();
    children.sort();
    for child in children {
        collect(&child, depth + 1, checkouts);
    }
}

fn common_dir(checkout: &Path) -> Result<PathBuf> {
    let output = git_output(
        checkout,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    let path = PathBuf::from(String::from_utf8(output)?.trim());
    Ok(fs::canonicalize(&path).unwrap_or(path))
}

/// A repository's base branch as one captured commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base {
    pub branch: String,
    /// The full ID of the commit the branch named when it was read.
    pub commit: String,
    /// The `project_slug` committed there, when its config reads.
    pub slug: Option<String>,
}

/// The base branch of the repository whose common Git directory is
/// `common_dir`, read from refs and commits alone.
pub fn base(common_dir: &Path) -> Result<Base> {
    let default = default_branch(common_dir)?;
    let default_commit = branch_commit(common_dir, &default).ok_or_else(|| {
        anyhow!("the default branch {default} has no local branch refs/heads/{default}")
    })?;
    let named = policy_base(common_dir, &default_commit)
        .map_err(|error| anyhow!("cannot read the workflow policy on {default}: {error:#}"))?;
    let commit = if named == default {
        default_commit
    } else {
        let commit = branch_commit(common_dir, &named).ok_or_else(|| {
            anyhow!("the workflow policy on {default} names base branch {named}, which has no local branch")
        })?;
        let own = policy_base(common_dir, &commit)
            .map_err(|error| anyhow!("cannot read the workflow policy on {named}: {error:#}"))?;
        if own != named {
            bail!(
                "the workflow policy on {default} names base branch {named}, but the policy on {named} names {own}, not itself"
            );
        }
        commit
    };
    let mut read = ReadContext::at_commit(common_dir, commit.clone());
    let slug = atelier_app::commit_records::file_at(&mut read, ".atelier/config.toml")
        .ok()
        .and_then(|text| {
            atelier_app::project_config::parse_project_config(
                &text,
                Path::new(".atelier/config.toml"),
            )
            .ok()
        })
        .map(|config| config.project_slug);
    Ok(Base {
        branch: named,
        commit,
        slug,
    })
}

/// The repository's default branch: the branch `refs/remotes/origin/HEAD`
/// points at, else whichever one of local `main` and `master` exists.
fn default_branch(common_dir: &Path) -> Result<String> {
    let listed = String::from_utf8(git_output(
        common_dir,
        &[
            "for-each-ref",
            "--format=%(refname) %(symref)",
            "refs/remotes/origin/HEAD",
            "refs/heads/main",
            "refs/heads/master",
        ],
    )?)?;
    let mut local = Vec::new();
    for line in listed.lines() {
        let (name, target) = line.split_once(' ').unwrap_or((line, ""));
        match name {
            "refs/remotes/origin/HEAD" => {
                let branch = target.strip_prefix("refs/remotes/origin/").ok_or_else(|| {
                    anyhow!("refs/remotes/origin/HEAD does not name a branch of origin")
                })?;
                return Ok(branch.to_owned());
            }
            _ => local.extend(name.strip_prefix("refs/heads/").map(str::to_owned)),
        }
    }
    match local.as_slice() {
        [one] => Ok(one.clone()),
        [] => bail!(
            "no default branch: refs/remotes/origin/HEAD is not set and neither main nor master exists"
        ),
        _ => bail!(
            "no default branch: both main and master exist and refs/remotes/origin/HEAD does not say which is the default"
        ),
    }
}

fn branch_commit(common_dir: &Path, branch: &str) -> Option<String> {
    git_output(
        common_dir,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}^{{commit}}"),
        ],
    )
    .ok()
    .and_then(|output| String::from_utf8(output).ok())
    .map(|output| output.trim().to_owned())
    .filter(|output| !output.is_empty())
}

/// The base branch the workflow policy committed at `commit` names.
fn policy_base(common_dir: &Path, commit: &str) -> Result<String> {
    let mut read = ReadContext::at_commit(common_dir, commit.to_owned());
    Ok(atelier_app::base_policy::from_captured(&mut read, commit)?
        .policy
        .branch_policy
        .base_branch)
}
