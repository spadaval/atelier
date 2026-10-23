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
    resolve(&mut git::ReadContext::new(repo), &tip, reference)
}

/// Resolve trusted policy from an immutable full commit captured by the caller.
pub fn from_captured(read: &mut git::ReadContext<'_>, commit: &str) -> Result<PolicySource> {
    if !matches!(commit.len(), 40 | 64) || !commit.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        bail!("policy resolution requires a full captured Git commit ID");
    }
    resolve(read, commit, commit)
}
fn resolve(read: &mut git::ReadContext<'_>, tip: &str, reference: &str) -> Result<PolicySource> {
    let tip = read.commit_id(&format!("{tip}^{{commit}}"))?;
    let mut diagnostics = Vec::new();
    let newer = match read
        .blob(&tip, Path::new(WORKFLOW_POLICY_PATH))
        .and_then(|text| atelier_workflow::parse_policy_text(&text, WORKFLOW_POLICY_PATH))
    {
        Ok(policy) => {
            return Ok(PolicySource {
                policy,
                commit: tip,
                diagnostics,
            })
        }
        Err(error) => {
            diagnostics.push(format!("Invalid policy at {tip}: {error:#}"));
            atelier_workflow::needs_newer_atelier(&error)
        }
    };
    let history = String::from_utf8(git::git_output(
        read.repo,
        &[
            "rev-list",
            "--first-parent",
            &tip,
            "--",
            WORKFLOW_POLICY_PATH,
        ],
    )?)?;
    for commit in history.lines().filter(|commit| *commit != tip) {
        let loaded = read
            .blob(commit, Path::new(WORKFLOW_POLICY_PATH))
            .and_then(|text| atelier_workflow::parse_policy_text(&text, WORKFLOW_POLICY_PATH));
        match loaded {
            Ok(policy) => {
                diagnostics.push(fallback_note(commit, newer));
                return Ok(PolicySource {
                    policy,
                    commit: commit.to_owned(),
                    diagnostics,
                });
            }
            Err(error) => diagnostics.push(format!("Invalid policy at {commit}: {error:#}")),
        }
    }
    bail!("No valid policy on {reference}: {}", diagnostics.join("\n"))
}

/// The note that a fallback policy is in use: with the edit that ends it,
/// or alone when a newer Atelier wrote the policy, whose diagnostic already
/// names the reinstall (atelier-u4ql).
fn fallback_note(commit: &str, newer: bool) -> String {
    if newer {
        format!("Using last valid policy from {commit}.")
    } else {
        format!("Using last valid policy from {commit}; fix .atelier/workflow.yaml.")
    }
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
            let newer = atelier_workflow::needs_newer_atelier(&error);
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
                let note = fallback_note(&source.commit, newer);
                source.diagnostics.push(note);
            }
            Ok(source)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    fn commit(repo: &Path, policy: &str, message: &str) -> String {
        fs::create_dir_all(repo.join(".atelier")).unwrap();
        fs::write(repo.join(WORKFLOW_POLICY_PATH), policy).unwrap();
        git::git_output(repo, &["add", "."]).unwrap();
        git::git_output(repo, &["commit", "-qm", message]).unwrap();
        git::head_commit(repo).unwrap()
    }
    #[test]
    fn valid_tip_policy_does_not_require_available_older_ancestry() {
        for format in ["sha1", "sha256"] {
            let repo = tempfile::tempdir().unwrap();
            for args in [
                vec!["init", "-qb", "main", &format!("--object-format={format}")],
                vec!["config", "user.name", "Author"],
                vec!["config", "user.email", "author@example.test"],
            ] {
                git::git_output(repo.path(), &args).unwrap();
            }
            let valid = atelier_workflow::STARTER_POLICY_YAML;
            let root = commit(repo.path(), valid, "valid root");
            let tip = commit(repo.path(), &format!("{valid}\n"), "valid tip");
            fs::remove_file(
                repo.path()
                    .join(".git/objects")
                    .join(&root[..2])
                    .join(&root[2..]),
            )
            .unwrap();
            assert!(git::git_output(
                repo.path(),
                &[
                    "rev-list",
                    "--first-parent",
                    &tip,
                    "--",
                    WORKFLOW_POLICY_PATH
                ]
            )
            .is_err());
            let source = from_captured(&mut git::ReadContext::new(repo.path()), &tip).unwrap();
            assert_eq!(source.commit, tip);
            assert!(source.diagnostics.is_empty());
        }
    }
    #[test]
    fn captured_policy_preserves_first_parent_fallback_peeling_and_object_kind() {
        for format in ["sha1", "sha256"] {
            let repo = tempfile::tempdir().unwrap();
            for args in [
                vec!["init", "-qb", "main", &format!("--object-format={format}")],
                vec!["config", "user.name", "Author"],
                vec!["config", "user.email", "author@example.test"],
            ] {
                git::git_output(repo.path(), &args).unwrap();
            }
            let valid = atelier_workflow::STARTER_POLICY_YAML;
            let base = commit(repo.path(), valid, "valid root policy");
            let mut read = git::ReadContext::new(repo.path());
            let source = from_captured(&mut read, &base).unwrap();
            assert_eq!(source.commit, base);
            assert!(source.diagnostics.is_empty());
            git::git_output(
                repo.path(),
                &["tag", "-am", "annotated policy", "policy", &base],
            )
            .unwrap();
            let tag = String::from_utf8(
                git::git_output(repo.path(), &["rev-parse", "refs/tags/policy"]).unwrap(),
            )
            .unwrap();
            assert_eq!(from_captured(&mut read, tag.trim()).unwrap().commit, base);
            assert_eq!(from_ref(repo.path(), "policy").unwrap().commit, base);
            git::git_output(repo.path(), &["checkout", "-qb", "side"]).unwrap();
            let side = commit(
                repo.path(),
                &valid.replace("base_branch: main", "base_branch: side"),
                "side policy",
            );
            git::git_output(repo.path(), &["checkout", "main"]).unwrap();
            let invalid = commit(repo.path(), "invalid: [", "invalid tip policy");
            git::git_output(
                repo.path(),
                &[
                    "merge",
                    "--no-ff",
                    "-s",
                    "ours",
                    "-qm",
                    "retain invalid first parent",
                    "side",
                ],
            )
            .unwrap();
            let head = git::head_commit(repo.path()).unwrap();
            let captured = from_captured(&mut read, &head).unwrap();
            let named = from_ref(repo.path(), "HEAD").unwrap();
            assert_eq!(captured.commit, base);
            assert_eq!(captured.diagnostics, named.diagnostics);
            assert!(captured
                .diagnostics
                .iter()
                .any(|line| line.contains(&invalid)));
            assert!(!captured.diagnostics.iter().any(|line| line.contains(&side)));
            let tree = String::from_utf8(
                git::git_output(repo.path(), &["rev-parse", "HEAD^{tree}"]).unwrap(),
            )
            .unwrap();
            assert!(from_captured(&mut read, tree.trim()).is_err());
            assert!(from_captured(&mut read, "HEAD").is_err());
        }
    }
}
