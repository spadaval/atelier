//! The GitHub forge `config.toml` names, reached through the `gh` CLI as
//! Atelier reaches Git through `git`: authentication is whatever
//! `gh auth login` set up. It finds or opens a branch's pull request and
//! reads the CI runs of its branch, for `submit` and `merge`.
//!
//! It calls the REST API through `gh api` with fixed fields rather than
//! parsing `gh run list`, whose output is `gh`'s own and changes between
//! versions.
//!
//! Copied from `crates/atelier-app/src/forge.rs`, with one change to its
//! interface: `Github::open` takes the new config's forge section, whose CI
//! workflow is optional, and the workflow is passed to `branch_runs`, which
//! reads a branch's runs where the old code read a commit's. `until` bounds
//! each call in time, for `merge`'s wait. Trimmed: the base's push runs, the lookup of a
//! commit's pull requests, the check that a pull request merged, and the
//! error codes and wording, which `output.rs` holds.

use crate::git;
use atelier_core::Forge;
use serde_json::Value;
use std::io::Read;
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// Why the forge could not answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForgeError {
    /// `gh` is not on `PATH`.
    GhMissing,
    /// `gh` is not signed in to the host.
    SignedOut { host: String },
    /// `gh` or the remote failed for another reason, quoted.
    Unreachable(String),
}

/// One run of the CI workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CiRun {
    pub id: u64,
    /// The commit it ran on.
    pub head_sha: String,
    /// `queued`, `in_progress`, `completed` and the like.
    pub status: String,
    /// Set once completed: `success`, `failure`, `cancelled`, `timed_out`…
    pub conclusion: Option<String>,
    pub url: String,
    /// RFC 3339 in UTC, so text order is time order.
    pub created_at: String,
}

impl CiRun {
    pub fn passed(&self) -> bool {
        self.status == "completed" && self.conclusion.as_deref() == Some("success")
    }

    /// Whether it passed or failed. A cancelled run did neither: this
    /// repository's CI cancels a pull request's run when a newer commit is
    /// pushed.
    pub fn finished(&self) -> bool {
        self.status == "completed" && self.conclusion.as_deref() != Some("cancelled")
    }

    pub fn running(&self) -> bool {
        self.status != "completed"
    }
}

/// A pull request, by number and web link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PullRequest {
    pub number: u64,
    pub url: String,
}

impl PullRequest {
    fn from_json(pull: &Value) -> Option<Self> {
        Some(Self {
            number: pull["number"].as_u64()?,
            url: pull["html_url"].as_str().unwrap_or_default().to_owned(),
        })
    }
}

/// A GitHub repository.
#[derive(Debug, Clone)]
pub struct Github {
    host: String,
    /// `owner/repo`.
    slug: String,
    /// When set, a `gh` call still running then is ended and fails.
    deadline: Option<Instant>,
}

impl Github {
    /// The repository `forge.remote` points at: a configured remote's URL,
    /// or else the URL the config gives. A remote name may hold a slash, so
    /// the name is tried first.
    pub fn open(repo: &Path, forge: &Forge) -> Result<Self, ForgeError> {
        let url = match git::run(repo, &["remote", "get-url", "--", &forge.remote]) {
            Ok(url) => url,
            Err(_) if forge.remote.contains(':') || forge.remote.contains('/') => {
                forge.remote.clone()
            }
            Err(_) => {
                return Err(ForgeError::Unreachable(format!(
                    "forge.remote names {}, which is not a Git remote here",
                    forge.remote
                )))
            }
        };
        let (host, slug) = github_repository(&url).ok_or_else(|| {
            ForgeError::Unreachable(format!(
                "forge.remote {} has URL {url}, which names no GitHub repository",
                forge.remote
            ))
        })?;
        Ok(Self {
            host,
            slug,
            deadline: None,
        })
    }

    /// This repository, with every call ended at `deadline`.
    pub fn until(&self, deadline: Instant) -> Self {
        Self {
            deadline: Some(deadline),
            ..self.clone()
        }
    }

    /// The latest `pull_request` runs of `workflow` on the pull requests
    /// from `branch`, on whichever commit each ran.
    pub fn branch_runs(&self, workflow: &str, branch: &str) -> Result<Vec<CiRun>, ForgeError> {
        let value = self.api(
            &format!("repos/{}/actions/workflows/{workflow}/runs", self.slug),
            &[
                ("branch", branch),
                ("event", "pull_request"),
                ("per_page", "50"),
            ],
        )?;
        runs(&value)
    }

    /// Whether `gh` is installed and signed in to this host, checked before
    /// anything is pushed.
    pub fn signed_in(&self) -> Result<(), ForgeError> {
        self.gh(
            &["auth", "status", "--hostname", &self.host],
            "gh auth status",
        )
        .map(|_| ())
    }

    /// The open pull request from this repository's `branch` into `base`.
    pub fn open_pull_request(
        &self,
        branch: &str,
        base: &str,
    ) -> Result<Option<PullRequest>, ForgeError> {
        let owner = self.slug.split('/').next().unwrap_or_default();
        let pulls = self.api(
            &format!("repos/{}/pulls", self.slug),
            &[
                ("head", &format!("{owner}:{branch}")),
                ("base", base),
                ("state", "open"),
            ],
        )?;
        Ok(pulls
            .as_array()
            .and_then(|pulls| pulls.first())
            .and_then(PullRequest::from_json))
    }

    /// Opens a pull request from this repository's `branch` into `base`.
    pub fn create_pull_request(
        &self,
        branch: &str,
        base: &str,
        title: &str,
        body: &str,
    ) -> Result<PullRequest, ForgeError> {
        let path = format!("repos/{}/pulls", self.slug);
        let pull = self.call(
            "POST",
            &path,
            &[
                ("title", title),
                ("head", branch),
                ("base", base),
                ("body", body),
            ],
        )?;
        PullRequest::from_json(&pull).ok_or_else(|| {
            ForgeError::Unreachable(format!("gh api {path} returned no pull request number"))
        })
    }

    fn api(&self, path: &str, fields: &[(&str, &str)]) -> Result<Value, ForgeError> {
        self.call("GET", path, fields)
    }

    /// Calls the REST API: a GET sends `fields` as query parameters, a POST
    /// as its JSON body.
    fn call(&self, method: &str, path: &str, fields: &[(&str, &str)]) -> Result<Value, ForgeError> {
        let mut args = vec![
            "api".to_owned(),
            "--hostname".to_owned(),
            self.host.clone(),
            "-X".to_owned(),
            method.to_owned(),
            path.to_owned(),
        ];
        for (key, value) in fields {
            args.push("-f".to_owned());
            args.push(format!("{key}={value}"));
        }
        let args = args.iter().map(String::as_str).collect::<Vec<_>>();
        let stdout = self.gh(&args, &format!("gh api {path}"))?;
        serde_json::from_slice(&stdout).map_err(|error| {
            ForgeError::Unreachable(format!("gh api {path} returned unreadable JSON: {error}"))
        })
    }

    /// Runs `gh` and returns its standard output, telling a missing or
    /// signed-out `gh` from other failures.
    fn gh(&self, args: &[&str], what: &str) -> Result<Vec<u8>, ForgeError> {
        let mut child = match Command::new("gh")
            .args(args)
            .env("GH_PROMPT_DISABLED", "1")
            .env("NO_COLOR", "1")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Err(ForgeError::GhMissing)
            }
            Err(error) => return Err(ForgeError::Unreachable(error.to_string())),
        };
        let stdout = read_all(child.stdout.take());
        let stderr = read_all(child.stderr.take());
        let status = match wait(&mut child, self.deadline) {
            Ok(Some(status)) => status,
            // The readers are left to end with the pipes.
            Ok(None) => {
                return Err(ForgeError::Unreachable(format!(
                    "{what} did not answer in time"
                )))
            }
            Err(error) => return Err(ForgeError::Unreachable(error.to_string())),
        };
        let stdout = stdout.join().unwrap_or_default();
        let stderr = stderr.join().unwrap_or_default();
        if !status.success() {
            let stderr = String::from_utf8_lossy(&stderr);
            // gh exits 4 when it needs a sign-in; `gh auth status` exits 1
            // and names `gh auth login`.
            if status.code() == Some(4) || stderr.contains("gh auth login") {
                return Err(ForgeError::SignedOut {
                    host: self.host.clone(),
                });
            }
            let message = stderr
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            return Err(ForgeError::Unreachable(if message.is_empty() {
                format!("{what} exited with {:?}", status.code())
            } else {
                message
            }));
        }
        Ok(stdout)
    }
}

/// Reads all of `pipe` on a thread of its own, so neither pipe of a child
/// fills while the other is read.
fn read_all(pipe: Option<impl Read + Send + 'static>) -> JoinHandle<Vec<u8>> {
    std::thread::spawn(move || {
        let mut bytes = Vec::new();
        if let Some(mut pipe) = pipe {
            let _ = pipe.read_to_end(&mut bytes);
        }
        bytes
    })
}

/// Waits for `child` to exit, or until `deadline`, when it is killed and
/// `None` is returned.
fn wait(child: &mut Child, deadline: Option<Instant>) -> std::io::Result<Option<ExitStatus>> {
    let Some(deadline) = deadline else {
        return child.wait().map(Some);
    };
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(Some(status));
        }
        let now = Instant::now();
        if now >= deadline {
            child.kill()?;
            child.wait()?;
            return Ok(None);
        }
        std::thread::sleep((deadline - now).min(Duration::from_millis(20)));
    }
}

fn runs(value: &Value) -> Result<Vec<CiRun>, ForgeError> {
    let Some(list) = value["workflow_runs"].as_array() else {
        return Err(ForgeError::Unreachable(
            "GitHub's workflow runs answer has no workflow_runs list".to_owned(),
        ));
    };
    Ok(list
        .iter()
        .filter_map(|run| {
            Some(CiRun {
                id: run["id"].as_u64()?,
                head_sha: run["head_sha"].as_str()?.to_owned(),
                status: run["status"].as_str()?.to_owned(),
                conclusion: run["conclusion"].as_str().map(str::to_owned),
                url: run["html_url"].as_str().unwrap_or_default().to_owned(),
                created_at: run["created_at"].as_str().unwrap_or_default().to_owned(),
            })
        })
        .collect())
}

/// The host and `owner/repo` a GitHub remote URL names: `git@host:o/r.git`,
/// `ssh://git@host/o/r.git` or `https://host/o/r`.
fn github_repository(url: &str) -> Option<(String, String)> {
    let rest = if let Some((scheme, rest)) = url.split_once("://") {
        if !["https", "http", "ssh", "git"].contains(&scheme) {
            return None;
        }
        rest
    } else {
        url
    };
    let rest = rest.rsplit_once('@').map_or(rest, |(_, rest)| rest);
    let (host, path) = rest.split_once([':', '/'])?;
    // `host:port/path` in an ssh:// URL.
    let path = match path.split_once('/') {
        Some((port, path)) if port.chars().all(|c| c.is_ascii_digit()) => path,
        _ => path,
    };
    let path = path.trim_matches('/');
    let path = path.strip_suffix(".git").unwrap_or(path);
    let mut parts = path.split('/');
    let (Some(owner), Some(name), None) = (parts.next(), parts.next(), parts.next()) else {
        return None;
    };
    if host.is_empty() || owner.is_empty() || name.is_empty() {
        return None;
    }
    Some((host.to_owned(), format!("{owner}/{name}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remote_urls_name_the_host_and_repository() {
        for url in [
            "git@github.com:spadaval/atelier.git",
            "ssh://git@github.com/spadaval/atelier.git",
            "ssh://git@github.com:22/spadaval/atelier",
            "https://github.com/spadaval/atelier",
            "https://github.com/spadaval/atelier.git/",
            "https://user@github.com/spadaval/atelier.git",
        ] {
            assert_eq!(
                github_repository(url),
                Some(("github.com".to_owned(), "spadaval/atelier".to_owned())),
                "{url}"
            );
        }
        for url in [
            "/srv/git/atelier.git",
            "file:///srv/atelier",
            "https://github.com/spadaval",
        ] {
            assert_eq!(github_repository(url), None, "{url}");
        }
    }
}
