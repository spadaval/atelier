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

/// The checkout's Git common directory, read from its `.git` the way Git
/// reads it, with no Git process: a directory is the common directory, and
/// a `gitdir:` file names a worktree's directory, whose `commondir` file
/// names the common one.
fn common_dir(checkout: &Path) -> Result<PathBuf> {
    let dot_git = checkout.join(".git");
    let git_dir = if dot_git.is_dir() {
        dot_git
    } else {
        let text = fs::read_to_string(&dot_git)?;
        let named = text
            .strip_prefix("gitdir:")
            .map(str::trim)
            .filter(|named| !named.is_empty())
            .ok_or_else(|| {
                anyhow!(
                    "{} is neither a directory nor a gitdir file",
                    dot_git.display()
                )
            })?;
        checkout.join(named)
    };
    let common = match fs::read_to_string(git_dir.join("commondir")) {
        Ok(named) => git_dir.join(named.trim()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => git_dir,
        Err(error) => return Err(error.into()),
    };
    if !common.join("HEAD").is_file() {
        bail!("{} is not a Git directory", common.display());
    }
    Ok(fs::canonicalize(&common)?)
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

/// What one commit's committed policy and config say, which nothing but the
/// commit decides.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitFacts {
    /// The base branch its workflow policy names, or why it cannot be read.
    pub base_branch: std::result::Result<String, String>,
    pub slug: Option<String>,
}

/// Reads [`CommitFacts`] at `commit`, through one `git cat-file --batch`.
pub fn commit_facts(common_dir: &Path, commit: &str) -> CommitFacts {
    let mut read = ReadContext::at_commit(common_dir, commit.to_owned());
    let base_branch = atelier_app::base_policy::from_captured(&mut read, commit)
        .map(|source| source.policy.branch_policy.base_branch)
        .map_err(|error| format!("{error:#}"));
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
    CommitFacts { base_branch, slug }
}

/// The base branch of the repository whose common Git directory is
/// `common_dir`, read from its refs, with one Git process, and from
/// `facts`, which answers [`commit_facts`] for a commit, as a cache may.
pub fn base(common_dir: &Path, facts: &mut dyn FnMut(&str) -> CommitFacts) -> Result<Base> {
    let refs = Refs::read(common_dir)?;
    let default = refs.default_branch()?;
    let default_commit = refs.branch(&default).ok_or_else(|| {
        anyhow!("the default branch {default} has no local branch refs/heads/{default}")
    })?;
    let on_default = facts(default_commit);
    let named = on_default
        .base_branch
        .map_err(|error| anyhow!("cannot read the workflow policy on {default}: {error}"))?;
    if named == default {
        return Ok(Base {
            branch: named,
            commit: default_commit.to_owned(),
            slug: on_default.slug,
        });
    }
    let commit = refs.branch(&named).ok_or_else(|| {
        anyhow!(
            "the workflow policy on {default} names base branch {named}, which has no local branch"
        )
    })?;
    let on_base = facts(commit);
    let own = on_base
        .base_branch
        .map_err(|error| anyhow!("cannot read the workflow policy on {named}: {error}"))?;
    if own != named {
        bail!(
            "the workflow policy on {default} names base branch {named}, but the policy on {named} names {own}, not itself"
        );
    }
    Ok(Base {
        branch: named,
        commit: commit.to_owned(),
        slug: on_base.slug,
    })
}

/// The local branches' commits and the branch `refs/remotes/origin/HEAD`
/// points at, from one `git for-each-ref`.
struct Refs {
    branches: BTreeMap<String, String>,
    origin_head: Option<String>,
}

impl Refs {
    fn read(common_dir: &Path) -> Result<Self> {
        let listed = String::from_utf8(git_output(
            common_dir,
            &[
                "for-each-ref",
                "--format=%(refname) %(objectname) %(symref)",
                "refs/heads",
                "refs/remotes/origin/HEAD",
            ],
        )?)?;
        let mut refs = Self {
            branches: BTreeMap::new(),
            origin_head: None,
        };
        for line in listed.lines() {
            let mut fields = line.splitn(3, ' ');
            let (Some(name), Some(commit)) = (fields.next(), fields.next()) else {
                continue;
            };
            let target = fields.next().unwrap_or("");
            if name == "refs/remotes/origin/HEAD" {
                let branch = target.strip_prefix("refs/remotes/origin/").ok_or_else(|| {
                    anyhow!("refs/remotes/origin/HEAD does not name a branch of origin")
                })?;
                refs.origin_head = Some(branch.to_owned());
            } else if let Some(branch) = name.strip_prefix("refs/heads/") {
                refs.branches.insert(branch.to_owned(), commit.to_owned());
            }
        }
        Ok(refs)
    }

    fn branch(&self, name: &str) -> Option<&str> {
        self.branches.get(name).map(String::as_str)
    }

    /// The branch `refs/remotes/origin/HEAD` points at, else whichever one
    /// of local `main` and `master` exists.
    fn default_branch(&self) -> Result<String> {
        if let Some(branch) = &self.origin_head {
            return Ok(branch.clone());
        }
        match (self.branch("main"), self.branch("master")) {
            (Some(_), None) => Ok("main".to_owned()),
            (None, Some(_)) => Ok("master".to_owned()),
            (None, None) => bail!(
                "no default branch: refs/remotes/origin/HEAD is not set and neither main nor master exists"
            ),
            (Some(_), Some(_)) => bail!(
                "no default branch: both main and master exist and refs/remotes/origin/HEAD does not say which is the default"
            ),
        }
    }
}
