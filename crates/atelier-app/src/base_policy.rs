//! Git-backed policy resolution; callers surface fallback diagnostics.
use crate::{
    git_freshness as git,
    workflow_policy::{WorkflowPolicy, WORKFLOW_POLICY_PATH},
};
use anyhow::{bail, Result};
use std::path::Path;

pub struct PolicySource {
    pub policy: WorkflowPolicy,
    pub commit: String,
    pub diagnostics: Vec<String>,
}

/// Uses the newest valid version on this ref's first-parent history.
pub fn from_ref(repo: &Path, reference: &str) -> Result<PolicySource> {
    let tip = String::from_utf8(git::git_output(
        repo,
        &["rev-parse", "--verify", &format!("{reference}^{{commit}}")],
    )?)?
    .trim()
    .to_string();
    let history = String::from_utf8(git::git_output(
        repo,
        &[
            "rev-list",
            "--first-parent",
            &tip,
            "--",
            WORKFLOW_POLICY_PATH,
        ],
    )?)?;
    let mut commits = vec![tip];
    commits.extend(history.lines().map(str::to_owned));
    commits.dedup();
    let mut diagnostics = Vec::new();
    for commit in commits {
        let loaded = git::blob(repo, &commit, Path::new(WORKFLOW_POLICY_PATH))
            .and_then(|text| atelier_workflow::parse_policy_text(&text, WORKFLOW_POLICY_PATH));
        match loaded {
            Ok(policy) => {
                if !diagnostics.is_empty() {
                    diagnostics.push(format!("Using last valid policy from {commit}; fix .atelier/workflow.yaml before merging."));
                }
                return Ok(PolicySource {
                    policy,
                    commit,
                    diagnostics,
                });
            }
            Err(error) => diagnostics.push(format!("Invalid policy at {commit}: {error:#}")),
        }
    }
    bail!("No valid policy on {reference}: {}", diagnostics.join("\n"))
}

/// Ordinary commands may read an uncommitted policy; malformed policy falls
/// back to committed history. Lint and merge-candidate syntax remain strict.
pub fn working(repo: &Path) -> Result<PolicySource> {
    match atelier_workflow::load(repo) {
        Ok(policy) => Ok(PolicySource {
            policy,
            commit: "working tree".into(),
            diagnostics: Vec::new(),
        }),
        Err(error) => {
            let mut source = from_ref(repo, "HEAD")
                .map_err(|fallback| anyhow::anyhow!("{error:#}\n{fallback:#}"))?;
            source
                .diagnostics
                .insert(0, format!("Invalid working policy: {error:#}"));
            if !source
                .diagnostics
                .iter()
                .any(|message| message.starts_with("Using last valid"))
            {
                source.diagnostics.push(format!(
                    "Using last valid policy from {}; fix .atelier/workflow.yaml.",
                    source.commit
                ));
            }
            Ok(source)
        }
    }
}
