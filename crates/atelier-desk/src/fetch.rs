//! Fetching each repository's remote on a timer, so the reader finds work
//! pushed from other clones, and the forge's pull requests read with it.
//! No command of the tracker fetches; the Desk does, for the human
//! (docs/design.md, No command fetches).

use crate::api::{FetchMark, PrMark};
use crate::read::PrState;
use anyhow::{bail, Context, Result};
use atelier_app::git;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
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
    prs: Option<HashMap<String, PrMark>>,
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
        let read = github.then(|| pull_requests(&self.gh, dir));
        let mut state = self.state.lock().unwrap();
        let kept = state.entry(name.to_string()).or_default();
        kept.error = error;
        kept.github = github;
        match read {
            None => (kept.prs, kept.forge_error) = (None, None),
            Some(Ok(prs)) => (kept.prs, kept.forge_error) = (Some(prs), None),
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

    /// What the forge says of the pull request whose head is `branch` in
    /// `name`, the newest where several are: unknown while no list of its
    /// GitHub forge has been read, or when the last read failed and the
    /// list before it had none, and none for a repository with no GitHub
    /// forge.
    pub fn pull_request(&self, name: &str, branch: &str) -> PrState {
        let state = self.state.lock().unwrap();
        match state.get(name) {
            Some(kept) if !kept.github => PrState::Known(None),
            Some(Fetched {
                prs: Some(prs),
                forge_error,
                ..
            }) => match prs.get(branch) {
                // A pull request the last good list named stays as it was.
                Some(pr) => PrState::Known(Some(pr.clone())),
                // Its absence is confirmed only by a list read just now:
                // one may have been opened since a list that failed after.
                None if forge_error.is_none() => PrState::Known(None),
                None => PrState::Unknown,
            },
            _ => PrState::Unknown,
        }
    }
}

/// `git fetch --prune <remote>`, asking nothing of a terminal.
pub fn fetch(dir: &Path, remote: &str) -> Result<()> {
    let output = Command::new("git")
        .current_dir(dir)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_SSH_COMMAND", "ssh -o BatchMode=yes")
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

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Listed {
    number: u32,
    url: String,
    state: String,
    head_ref_name: String,
}

/// The forge's last 200 pull requests of any state, by head branch, the
/// newest for each, through `gh` in `dir`.
fn pull_requests(gh: &Path, dir: &Path) -> Result<HashMap<String, PrMark>> {
    let output = Command::new(gh)
        .current_dir(dir)
        .env("GH_PROMPT_DISABLED", "1")
        .args([
            "pr",
            "list",
            "--state",
            "all",
            "--limit",
            "200",
            "--json",
            "number,url,state,headRefName",
        ])
        .output()
        .with_context(|| format!("cannot run {}", gh.display()))?;
    if !output.status.success() {
        bail!(
            "gh pr list failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let listed: Vec<Listed> =
        serde_json::from_slice(&output.stdout).context("gh pr list gave unreadable JSON")?;
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
    Ok(prs)
}
