//! The GitHub forge a policy names, reached through the `gh` CLI as Atelier
//! reaches Git through `git` (ADR 0031): authentication is whatever
//! `gh auth login` set up. It reads CI for a commit; nothing else it could
//! read feeds a gate.
//!
//! It calls the REST API through `gh api` with fixed fields rather than
//! parsing `gh run list`, whose output is `gh`'s own and changes between
//! versions (docs/strategies/forge-delivery.md, Risks).
use std::path::Path;
use std::process::Command;

use serde_json::Value;

use crate::workflow_policy::ForgeConfig;

/// Why the forge could not answer. Each names the step that fixes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForgeError {
    /// `gh` is not on `PATH` (`forge_gh_missing`).
    GhMissing,
    /// `gh` is not signed in to the host (`forge_gh_signed_out`).
    SignedOut { host: String },
    /// `gh` or the remote failed for another reason, quoted
    /// (`forge_unreachable`).
    Unreachable(String),
}
impl ForgeError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::GhMissing => "forge_gh_missing",
            Self::SignedOut { .. } => "forge_gh_signed_out",
            Self::Unreachable(_) => "forge_unreachable",
        }
    }
    pub fn reason(&self) -> String {
        match self {
            Self::GhMissing => {
                "gh, the GitHub CLI, is not on PATH, so Atelier cannot read CI".to_owned()
            }
            Self::SignedOut { host } => {
                format!("gh is not signed in to {host}, so Atelier cannot read CI")
            }
            Self::Unreachable(message) => format!("gh could not read CI: {message}"),
        }
    }
    pub fn next(&self) -> String {
        match self {
            Self::GhMissing => {
                "install the GitHub CLI (https://cli.github.com), then `gh auth login`".to_owned()
            }
            Self::SignedOut { .. } => "`gh auth login`".to_owned(),
            Self::Unreachable(_) => "retry; `gh auth status` shows the sign-in".to_owned(),
        }
    }
}

/// One run of the trusted workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CiRun {
    pub id: u64,
    /// `queued`, `in_progress`, `completed` and the like.
    pub status: String,
    /// Set once completed: `success`, `failure`, `cancelled`, `timed_out`…
    pub conclusion: Option<String>,
    pub url: String,
    pub head_sha: String,
    /// The pull request the run is for, when GitHub reports one; a pull
    /// request from a fork reports none.
    pub pull_request: Option<u64>,
    /// RFC 3339 in UTC, so text order is time order.
    pub created_at: String,
}
impl CiRun {
    pub fn passed(&self) -> bool {
        self.status == "completed" && self.conclusion.as_deref() == Some("success")
    }
}

/// A GitHub repository and the workflow the policy trusts.
#[derive(Debug, Clone)]
pub struct Github {
    host: String,
    /// `owner/repo`.
    slug: String,
    workflow: String,
}

impl Github {
    /// The repository `forge.remote` points at: a configured remote's URL,
    /// or else the URL the policy gives. A remote name may hold a slash, so
    /// the name is tried first.
    pub fn open(repo: &Path, forge: &ForgeConfig) -> Result<Self, ForgeError> {
        let output = crate::git_freshness::command(repo)
            .args(["remote", "get-url", "--", &forge.remote])
            .output()
            .map_err(|error| ForgeError::Unreachable(error.to_string()))?;
        let url = if output.status.success() {
            String::from_utf8_lossy(&output.stdout).trim().to_owned()
        } else if forge.remote.contains(':') || forge.remote.contains('/') {
            forge.remote.clone()
        } else {
            return Err(ForgeError::Unreachable(format!(
                "forge.remote names {}, which is not a Git remote here",
                forge.remote
            )));
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
            workflow: forge.ci_workflow.clone(),
        })
    }

    /// The trusted workflow's `pull_request` runs for `commit`.
    pub fn pull_request_runs(&self, commit: &str) -> Result<Vec<CiRun>, ForgeError> {
        let value = self.api(
            &format!(
                "repos/{}/actions/workflows/{}/runs",
                self.slug, self.workflow
            ),
            &[
                ("head_sha", commit),
                ("event", "pull_request"),
                ("per_page", "50"),
            ],
        )?;
        runs(&value)
    }

    /// The trusted workflow's newest `push` run on `branch`, which is for
    /// the branch's head as GitHub last saw it.
    pub fn newest_push_run(&self, branch: &str) -> Result<Option<CiRun>, ForgeError> {
        let value = self.api(
            &format!(
                "repos/{}/actions/workflows/{}/runs",
                self.slug, self.workflow
            ),
            &[("branch", branch), ("event", "push"), ("per_page", "1")],
        )?;
        Ok(runs(&value)?.into_iter().next())
    }

    /// Whether an open pull request into `base` has `commit`. GitHub
    /// refuses the lookup for a commit it has never seen, which is in none.
    pub fn has_pull_request(&self, commit: &str, base: &str) -> Result<bool, ForgeError> {
        let pulls = self.api(&format!("repos/{}/commits/{commit}/pulls", self.slug), &[])?;
        Ok(pulls.as_array().is_some_and(|pulls| {
            pulls
                .iter()
                .any(|pull| pull["state"] == "open" && pull["base"]["ref"].as_str() == Some(base))
        }))
    }

    fn api(&self, path: &str, fields: &[(&str, &str)]) -> Result<Value, ForgeError> {
        let mut command = Command::new("gh");
        command
            .args(["api", "--hostname", &self.host, "-X", "GET", path])
            .env("GH_PROMPT_DISABLED", "1")
            .env("NO_COLOR", "1");
        for (key, value) in fields {
            command.arg("-f").arg(format!("{key}={value}"));
        }
        let output = match command.output() {
            Ok(output) => output,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Err(ForgeError::GhMissing)
            }
            Err(error) => return Err(ForgeError::Unreachable(error.to_string())),
        };
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // gh exits 4 when it needs a sign-in.
            if output.status.code() == Some(4) || stderr.contains("gh auth login") {
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
                format!("gh api {path} exited with {:?}", output.status.code())
            } else {
                message
            }));
        }
        serde_json::from_slice(&output.stdout).map_err(|error| {
            ForgeError::Unreachable(format!("gh api {path} returned unreadable JSON: {error}"))
        })
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
                status: run["status"].as_str()?.to_owned(),
                conclusion: run["conclusion"].as_str().map(str::to_owned),
                url: run["html_url"].as_str().unwrap_or_default().to_owned(),
                head_sha: run["head_sha"].as_str().unwrap_or_default().to_owned(),
                pull_request: run["pull_requests"][0]["number"].as_u64(),
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
