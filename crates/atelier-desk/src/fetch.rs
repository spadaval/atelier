//! Fetching each repository's remote on a timer, so the reader finds work
//! pushed from other clones, and the forge's pull requests read with it.
//! No command of the tracker fetches; the Desk does, for the human
//! (docs/design.md, No command fetches).

use crate::api::{FetchMark, PrMark};
use crate::read::PrState;
use anyhow::{bail, Context, Result};
use atelier_core::forge::{self, Listed};
use atelier_core::git;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// What the Desk last fetched, and the pull requests it last read, by
/// repository name.
#[derive(Default)]
pub struct Fetches {
    every: u64,
    gh: PathBuf,
    state: Mutex<HashMap<String, Fetched>>,
}

#[derive(Default, Clone)]
struct Fetched {
    error: Option<String>,
    /// Whether the repository has a GitHub forge, whose pull requests are
    /// read.
    github: bool,
    /// The pull requests of the forge, by head branch, as last read; `None`
    /// while none has been read.
    prs: Option<Prs>,
    /// The tip of each of the remote's branches, as fetched just before
    /// that list was read. The list says what the forge held of a branch
    /// at that tip, and of no later one.
    tips: HashMap<String, String>,
    /// Why the last read of the pull requests failed.
    forge_error: Option<String>,
}

/// The remote the Desk fetches in `dir`, whose `config.toml` at `HEAD`
/// names `forge`: the forge's remote, else `origin` where it exists.
pub fn remote_of(dir: &Path, forge: Option<&str>) -> Option<String> {
    if let Some(forge) = forge {
        return Some(forge.to_string());
    }
    let remotes = git::run(dir, &["remote"]).ok()?;
    remotes
        .lines()
        .any(|r| r == "origin")
        .then(|| "origin".to_string())
}

impl Fetches {
    pub fn new(every: u64, gh: PathBuf) -> Fetches {
        Fetches {
            every,
            gh,
            state: Mutex::default(),
        }
    }

    /// Fetches the repository `name` at `dir` from `remote`, pruning
    /// branches the remote no longer has, and reads its pull requests when
    /// `github`. A failure is kept to be shown, and the refs stay as they
    /// were.
    pub fn fetch(&self, name: &str, dir: &Path, remote: Option<&str>, github: bool) {
        let error = remote.and_then(|remote| fetch(dir, remote).err().map(|e| format!("{e:#}")));
        let github = github && remote.is_some();
        // The tips are read before the list, so a branch the list was read
        // at is at that tip or later: a list never vouches for a commit it
        // may have been read before.
        let tips = remote
            .map(|remote| remote_tips(dir, remote))
            .unwrap_or_default();
        let read = github.then(|| pull_requests(&self.gh, dir));
        let mut state = self.state.lock().unwrap();
        let kept = state.entry(name.to_string()).or_default();
        kept.error = error;
        kept.github = github;
        match read {
            None => (kept.prs, kept.forge_error) = (None, None),
            Some(Ok(prs)) => (kept.prs, kept.tips, kept.forge_error) = (Some(prs), tips, None),
            // A failed read keeps the last list, which still says what was
            // open, and says why it failed.
            Some(Err(e)) => kept.forge_error = Some(format!("{e:#}")),
        }
    }

    /// When `name` at `dir` was last fetched and how.
    pub fn mark(&self, name: &str, dir: &Path, remote: Option<&str>) -> FetchMark {
        let state = self.state.lock().unwrap();
        let kept = state.get(name);
        FetchMark {
            remote: remote.map(str::to_string),
            at: last_fetched(dir),
            error: kept.and_then(|k| {
                let errors = [k.error.clone(), k.forge_error.clone()];
                let errors: Vec<String> = errors.into_iter().flatten().collect();
                (!errors.is_empty()).then(|| errors.join("; "))
            }),
            every_seconds: u32::try_from(self.every).unwrap_or(u32::MAX),
        }
    }

    /// The pull request whose head is `branch` in `name`, as the last list
    /// that was read gave it, for a page to link.
    pub fn pull_request(&self, name: &str, branch: &str) -> Option<PrMark> {
        let state = self.state.lock().unwrap();
        let prs = state.get(name)?.prs.as_ref()?;
        (prs.open.get(branch).or(prs.recent.get(branch))).cloned()
    }

    /// What the forge says of the pull request whose head is `branch` in
    /// `name` while that branch's tip is `tip`: the last list's word, when
    /// that list was read with the branch at `tip`, and for a branch with
    /// none open there, when it listed every open one; unknown when no list
    /// has been read, when the branch has moved since the last one, as when
    /// a drop was pushed after it, since that list says nothing of the
    /// drop, or when the open ones were too many to list; and none for a
    /// repository with no GitHub forge.
    pub fn pr_state(&self, name: &str, branch: &str, tip: Option<&str>) -> PrState {
        let state = self.state.lock().unwrap();
        match state.get(name) {
            Some(kept) if !kept.github => PrState::Known(None),
            Some(Fetched {
                prs: Some(prs),
                tips,
                ..
            }) if tip.is_some() && tips.get(branch).map(String::as_str) == tip => {
                match prs.open.get(branch) {
                    Some(open) => PrState::Known(Some(open.clone())),
                    // None open: so only if every open one was listed.
                    None if prs.every_open => PrState::Known(prs.recent.get(branch).cloned()),
                    None => PrState::Unknown,
                }
            }
            _ => PrState::Unknown,
        }
    }
}

/// The tip of each of `remote`'s branches in `dir`, by branch name.
fn remote_tips(dir: &Path, remote: &str) -> HashMap<String, String> {
    let prefix = format!("refs/remotes/{remote}/");
    let listed = git::run(
        dir,
        &["for-each-ref", "--format=%(refname) %(objectname)", &prefix],
    );
    let listed = listed.unwrap_or_default();
    let tips = listed.lines().filter_map(|line| {
        let (name, tip) = line.split_once(' ')?;
        Some((name.strip_prefix(&prefix)?.to_string(), tip.to_string()))
    });
    tips.collect()
}

/// `git fetch --prune <remote>`, asking nothing of a terminal.
pub fn fetch(dir: &Path, remote: &str) -> Result<()> {
    let output = git::unattended(&mut git::git(dir))
        .args(["fetch", "--quiet", "--prune", remote])
        .output()
        .with_context(|| format!("cannot run git fetch in {}", dir.display()))?;
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        bail!("git fetch {remote} failed: {}", error.trim());
    }
    Ok(())
}

/// When `dir`'s clone last fetched, by anything, as RFC 3339.
fn last_fetched(dir: &Path) -> Option<String> {
    let at = git::fetched_at(dir).ok().flatten()?;
    Some(chrono::DateTime::<chrono::Utc>::from(at).to_rfc3339())
}

/// How many pull requests a list asks for: the recent ones of any state,
/// for links, and the open ones, which settle whether a branch has one
/// open.
const RECENT: usize = 200;
const OPEN: usize = 1000;

/// The forge's pull requests, by head branch: the newest of its last
/// `RECENT` of any state, for links; every open one, the newest open where a
/// branch has several, since a newer closed one does not close it; and
/// whether the open ones were all listed, fewer than `OPEN` of them, so a
/// branch missing from them has none open.
fn pull_requests(gh: &Path, dir: &Path) -> Result<Prs> {
    let recent = newest(forge::list_pull_requests(gh, dir, "all", RECENT)?);
    let open = forge::list_pull_requests(gh, dir, "open", OPEN)?;
    let every_open = open.len() < OPEN;
    Ok(Prs {
        recent,
        open: newest(open),
        every_open,
    })
}

/// The pull requests of a forge, as one round read them.
#[derive(Default, Clone)]
struct Prs {
    recent: HashMap<String, PrMark>,
    open: HashMap<String, PrMark>,
    every_open: bool,
}

/// `listed`, the newest for each head branch.
fn newest(listed: Vec<Listed>) -> HashMap<String, PrMark> {
    let mut prs: HashMap<String, PrMark> = HashMap::new();
    for pr in listed {
        let newer = prs
            .get(&pr.head_ref_name)
            .is_none_or(|kept| kept.number < pr.number);
        if newer {
            let mark = PrMark {
                number: pr.number,
                url: pr.url,
                state: pr.state,
            };
            prs.insert(pr.head_ref_name, mark);
        }
    }
    prs
}
