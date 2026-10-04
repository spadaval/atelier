//! Pinned, read-only branch snapshots shared by start and orientation.
use anyhow::{bail, Context, Result};
use atelier_app::{git_freshness, record_index::RecordIndex, workflow_policy};
use std::{
    collections::BTreeSet,
    path::Path,
    process::{Command, Stdio},
};

pub(crate) fn text(repo: &Path, args: &[&str]) -> Result<String> {
    Ok(String::from_utf8(git_freshness::git_output(repo, args)?)?
        .trim()
        .into())
}
pub(crate) fn current_branch(repo: &Path) -> Result<Option<String>> {
    Ok(checkout_branch(repo)?.map(|branch| branch.trim_end().to_owned()))
}
/// The recursive checkout branch identity, preserving trailing whitespace.
pub(crate) fn checkout_branch(repo: &Path) -> Result<Option<String>> {
    let output = atelier_app::git_freshness::command(repo)
        .args(["symbolic-ref", "--quiet", "HEAD"])
        .output()?;
    if output.status.code() == Some(1) {
        return Ok(None);
    }
    if !output.status.success() {
        bail!(
            "cannot resolve checkout branch: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let reference = String::from_utf8(output.stdout)?;
    Ok(Some(
        reference
            .strip_suffix('\n')
            .unwrap_or(&reference)
            .strip_prefix("refs/heads/")
            .context("checkout HEAD does not reference a local branch")?
            .to_owned(),
    ))
}
pub(crate) fn resolve(repo: &Path, reference: &str) -> Result<String> {
    text(
        repo,
        &["rev-parse", "--verify", &format!("{reference}^{{commit}}")],
    )
}
/// Prefer a local head; a unique remote-tracking head is an existing branch too.
pub(crate) fn work_ref(repo: &Path, branch: &str) -> Result<Option<(String, String)>> {
    let local = format!("refs/heads/{branch}");
    if let Ok(commit) = resolve(repo, &local) {
        return Ok(Some((local, commit)));
    }
    let refs = text(
        repo,
        &["for-each-ref", "--format=%(refname)", "refs/remotes/"],
    )?;
    let matches = refs
        .lines()
        .filter(|reference| {
            reference
                .strip_prefix("refs/remotes/")
                .and_then(|rest| rest.split_once('/'))
                .is_some_and(|(_, name)| name == branch)
        })
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Ok(None),
        [reference] => Ok(Some(((*reference).into(), resolve(repo, reference)?))),
        _ => bail!("start_remote_ambiguous: several remote-tracking refs own {branch}: {}. Create the local tracking branch explicitly, then retry.", matches.join(", ")),
    }
}

pub(crate) struct Snapshot {
    dir: tempfile::TempDir,
    pub index: RecordIndex,
}
impl Snapshot {
    pub fn at(repo: &Path, state: &Path, commit: &str) -> Result<Self> {
        let relative = state
            .strip_prefix(repo)
            .context("tracker is outside repository")?;
        let dir = tempfile::tempdir()?;
        let mut archive = atelier_app::git_freshness::command(repo)
            .args(["archive", "--format=tar", commit, "--"])
            .arg(relative)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let unpack = Command::new("tar")
            .current_dir(dir.path())
            .args(["-xf", "-"])
            .stdin(archive.stdout.take().context("archive has no stdout")?)
            .output()?;
        let archive = archive.wait_with_output()?;
        if !archive.status.success() || !unpack.status.success() {
            bail!(
                "branch_snapshot: cannot read tracker at {commit}: {}{}",
                String::from_utf8_lossy(&archive.stderr),
                String::from_utf8_lossy(&unpack.stderr)
            );
        }
        let index = RecordIndex::load_structural(&dir.path().join(relative))?;
        Ok(Self { dir, index })
    }
    pub fn root(&self) -> &Path {
        self.dir.path()
    }
}

pub(crate) fn branch_lines(repo: &Path, state: &Path, live: &RecordIndex) -> Result<Vec<String>> {
    let current = current_branch(repo)?.unwrap_or_default();
    let policy = workflow_policy::load(repo)?;
    branch_lines_for(
        repo,
        state,
        live,
        &current,
        &policy.branch_policy.base_branch,
    )
}

pub(crate) fn branch_lines_for(
    repo: &Path,
    state: &Path,
    live: &RecordIndex,
    current: &str,
    base_branch: &str,
) -> Result<Vec<String>> {
    let heads = text(
        repo,
        &[
            "for-each-ref",
            "--format=%(refname) %(objectname)",
            "refs/heads/",
        ],
    )?;
    let mut lines = Vec::new();
    for head in heads.lines() {
        let Some((reference, commit)) = head.split_once(' ') else {
            continue;
        };
        let branch = reference.trim_start_matches("refs/heads/");
        if branch == current || branch == base_branch {
            continue;
        }
        let view = (|| -> Result<Vec<String>> {
            // Legacy branches can contain thousands of obsolete records. Inspect
            // the small compatibility/owner blobs before materializing any tree.
            let yaml = git_freshness::blob(
                repo,
                commit,
                Path::new(workflow_policy::WORKFLOW_POLICY_PATH),
            )?;
            let branch_policy =
                atelier_workflow::parse_policy_text(&yaml, workflow_policy::WORKFLOW_POLICY_PATH)?;
            if let Ok(config) = git_freshness::blob(repo, commit, Path::new(".atelier/config.toml"))
            {
                atelier_app::project_config::parse_project_config(
                    &config,
                    Path::new(".atelier/config.toml"),
                )?;
            }
            let mut paths = live
                .issues()
                .filter(|issue| {
                    issue
                        .workflow_branch
                        .as_ref()
                        .is_some_and(|owner| owner.work_branch == branch)
                })
                .map(|issue| format!(".atelier/issues/{}.md", issue.id))
                .collect::<BTreeSet<_>>();
            if let Some(id) = branch
                .rsplit('/')
                .next()
                .filter(|id| atelier_records::validate_record_id(id).is_ok())
            {
                paths.insert(format!(".atelier/issues/{id}.md"));
            }
            let mut owners = Vec::new();
            for attempt in 0..2 {
                for path in &paths {
                    if let Ok(blob) = git_freshness::blob(repo, commit, Path::new(path)) {
                        let record = atelier_records::parse_issue_record(
                            &blob,
                            Path::new(path).strip_prefix(".atelier")?,
                        )?;
                        if record
                            .issue
                            .workflow_branch
                            .as_ref()
                            .is_some_and(|owner| owner.work_branch == branch)
                        {
                            owners.push(record.issue.id);
                        }
                    }
                }
                if !owners.is_empty() || attempt == 1 {
                    break;
                }
                // A custom branch can introduce its owner only on that branch.
                let found = text(
                    repo,
                    &[
                        "grep",
                        "-l",
                        "^workflow_branch:",
                        commit,
                        "--",
                        ".atelier/issues/",
                    ],
                )
                .unwrap_or_default();
                paths = found
                    .lines()
                    .filter_map(|line| line.split_once(':').map(|(_, path)| path.into()))
                    .collect();
            }
            if owners.is_empty() {
                bail!("no compatible started owner record");
            }
            let snapshot = Snapshot::at(repo, state, commit)?;
            snapshot
                .index
                .validate_policy(&branch_policy, snapshot.root())?;
            let mut rows = Vec::new();
            for id in owners {
                let owner = snapshot.index.require_issue(&id)?;
                let ids = atelier_app::review_gate::scope(&snapshot.index, &id)?;
                let active = ids
                    .iter()
                    .filter(|id| {
                        snapshot.index.get_issue(id).is_some_and(|issue| {
                            branch_policy.status_category(&issue.status) == Some("active")
                        })
                    })
                    .count();
                rows.push(format!(
                    "{branch} @ {}: {} [{}] {} — {active} active (committed branch records)",
                    &commit[..8],
                    id,
                    owner.status,
                    owner.title
                ));
            }
            Ok(rows)
        })();
        match view {
            Ok(rows) => lines.extend(rows),
            Err(error) => lines.push(format!("{branch}: unavailable — {error:#}")),
        }
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn git(repo: &Path, args: &[&str]) -> String {
        text(repo, args).unwrap()
    }

    #[test]
    fn native_checkout_queries_preserve_recursive_heads_and_refresh_linked_checkouts() {
        let repo = tempfile::tempdir().unwrap();
        for args in [
            vec!["init", "-qb", "main"],
            vec!["config", "user.name", "Author"],
            vec!["config", "user.email", "author@example.test"],
        ] {
            git(repo.path(), &args);
        }
        assert!(git_freshness::head_commit(repo.path()).is_err());
        fs::write(repo.path().join("code"), "base").unwrap();
        git(repo.path(), &["add", "."]);
        git(repo.path(), &["commit", "-qm", "base"]);
        let base = git_freshness::head_commit(repo.path()).unwrap();
        git(repo.path(), &["checkout", "-qb", "task/source"]);
        fs::write(repo.path().join("code"), "owner").unwrap();
        git(repo.path(), &["commit", "-qam", "owner"]);
        let original = git_freshness::head_commit(repo.path()).unwrap();
        git(
            repo.path(),
            &["tag", "-am", "shadow source", "task/source", &base],
        );
        git(repo.path(), &["tag", "HEAD", &base]);
        git(
            repo.path(),
            &["symbolic-ref", "refs/heads/alias", "refs/heads/task/source"],
        );
        git(repo.path(), &["symbolic-ref", "HEAD", "refs/heads/alias"]);
        assert_eq!(
            current_branch(repo.path()).unwrap().as_deref(),
            Some("task/source")
        );
        assert_eq!(git_freshness::head_commit(repo.path()).unwrap(), original);
        fs::write(
            repo.path().join(".git/refs/heads/broken"),
            format!("{}\n", "0".repeat(40)),
        )
        .unwrap();
        assert_eq!(git_freshness::head_commit(repo.path()).unwrap(), original);
        assert!(resolve(repo.path(), "refs/heads/broken").is_err());
        fs::remove_file(repo.path().join(".git/refs/heads/broken")).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;
            let opaque = repo
                .path()
                .join(".git/refs/heads")
                .join(std::ffi::OsString::from_vec(vec![0xff]));
            fs::write(opaque, format!("{base}\n")).unwrap();
            assert_eq!(git_freshness::head_commit(repo.path()).unwrap(), original);
            assert_eq!(
                current_branch(repo.path()).unwrap().as_deref(),
                Some("task/source")
            );
        }
        let worktree = tempfile::tempdir().unwrap();
        let linked = worktree.path().join("linked");
        git(
            repo.path(),
            &[
                "worktree",
                "add",
                "-q",
                "-b",
                "linked",
                linked.to_str().unwrap(),
                &base,
            ],
        );
        assert_eq!(current_branch(&linked).unwrap().as_deref(), Some("linked"));
        assert_eq!(git_freshness::head_commit(&linked).unwrap(), base);
        let whitespace_branch = "task/unicode\u{3000}";
        git(repo.path(), &["branch", whitespace_branch]);
        git(
            repo.path(),
            &[
                "symbolic-ref",
                "HEAD",
                &format!("refs/heads/{whitespace_branch}"),
            ],
        );
        assert_eq!(
            checkout_branch(repo.path()).unwrap().as_deref(),
            Some(whitespace_branch)
        );
        assert_eq!(
            current_branch(repo.path()).unwrap().as_deref(),
            Some("task/unicode")
        );
        git(
            repo.path(),
            &["symbolic-ref", "HEAD", "refs/heads/task/source"],
        );
        git(repo.path(), &["tag", "-d", "HEAD"]);
        git(repo.path(), &["checkout", "--detach", &original]);
        assert!(current_branch(repo.path()).unwrap().is_none());
        git(
            repo.path(),
            &["symbolic-ref", "HEAD", "refs/tags/task/source"],
        );
        assert!(current_branch(repo.path()).is_err());
        git(
            repo.path(),
            &["symbolic-ref", "HEAD", "refs/heads/task/source"],
        );
        fs::write(repo.path().join("code"), "new state").unwrap();
        git(repo.path(), &["commit", "-qam", "fresh ref state"]);
        assert_ne!(git_freshness::head_commit(repo.path()).unwrap(), original);
        let tag = git(repo.path(), &["rev-parse", "refs/tags/task/source"]);
        fs::write(
            repo.path().join(".git/refs/heads/tagged"),
            format!("{tag}\n"),
        )
        .unwrap();
        git(repo.path(), &["symbolic-ref", "HEAD", "refs/heads/tagged"]);
        assert_eq!(git_freshness::head_commit(repo.path()).unwrap(), tag);
    }

    #[test]
    fn native_base_refs_peel_annotated_objects_in_both_hash_formats() {
        for format in ["sha1", "sha256"] {
            let repo = tempfile::tempdir().unwrap();
            git(
                repo.path(),
                &["init", "-qb", "main", &format!("--object-format={format}")],
            );
            git(repo.path(), &["config", "user.name", "Author"]);
            git(
                repo.path(),
                &["config", "user.email", "author@example.test"],
            );
            fs::write(repo.path().join("code"), "base").unwrap();
            git(repo.path(), &["add", "."]);
            git(repo.path(), &["commit", "-qm", "base"]);
            let head = git_freshness::head_commit(repo.path()).unwrap();
            git(repo.path(), &["tag", "-am", "annotated", "base-tag"]);
            git(
                repo.path(),
                &[
                    "symbolic-ref",
                    "refs/heads/base-alias",
                    "refs/tags/base-tag",
                ],
            );
            assert_eq!(head.len(), if format == "sha1" { 40 } else { 64 });
            assert_eq!(resolve(repo.path(), "refs/heads/base-alias").unwrap(), head);
        }
    }
}
