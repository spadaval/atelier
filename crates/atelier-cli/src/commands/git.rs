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

/// Record-only changes may be committed on the base branch directly; any
/// other unpushed base commit must be covered by an approval or trivial
/// exemption of a completed owner that branched from this base, which is what
/// `atelier merge` integrates. Commits that install the workflow policy
/// predate the gates and are exempt.
pub(crate) fn check_base_commits(
    repo: &Path,
    index: &RecordIndex,
    policy: &atelier_workflow::WorkflowPolicy,
    tip: &str,
) -> Result<String> {
    let base = &policy.branch_policy.base_branch;
    let Ok(upstream) = text(
        repo,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("{base}@{{upstream}}"),
        ],
    ) else {
        return Ok(format!(
            "Base commits unchecked: {base} has no upstream, so there is no pushed point to check from."
        ));
    };
    // Gates start once the pushed base has the policy; until then the
    // commits that install Atelier, and any made alongside them, predate them.
    if git_freshness::git_output(
        repo,
        &[
            "ls-tree",
            &upstream,
            "--",
            workflow_policy::WORKFLOW_POLICY_PATH,
        ],
    )?
    .is_empty()
    {
        return Ok(format!(
            "Base commits unchecked: {base}@{{upstream}} has no workflow policy yet, so these commits install Atelier."
        ));
    }
    let mut covered = BTreeSet::new();
    for review in index.reviews().filter(|review| {
        matches!(
            review.event,
            atelier_core::ReviewEvent::Approval | atelier_core::ReviewEvent::TrivialExemption
        )
    }) {
        let Some(owner) = index.get_issue(&review.target) else {
            continue;
        };
        if !owner
            .workflow_branch
            .as_ref()
            .is_some_and(|branch| branch.branch_base == *base)
            || policy.status_category(&owner.status) != Some("done")
            || resolve(repo, &review.commit).is_err()
        {
            continue;
        }
        let path = format!(".atelier/issues/{}.md", owner.id);
        // An owner already done upstream was integrated before; a review
        // recorded for it now covers nothing new.
        let pushed_done = git_freshness::blob(repo, &upstream, Path::new(&path))
            .ok()
            .and_then(|blob| {
                atelier_records::parse_issue_record(
                    &blob,
                    Path::new(&path).strip_prefix(".atelier").ok()?,
                )
                .ok()
            })
            .is_some_and(|record| policy.status_category(&record.issue.status) == Some("done"));
        if pushed_done {
            continue;
        }
        let range = [
            "rev-list",
            review.commit.as_str(),
            "--not",
            upstream.as_str(),
        ];
        covered.extend(text(repo, &range)?.lines().map(str::to_owned));
    }
    let unreviewed = text(repo, &["rev-list", tip, "--not", &upstream])?
        .lines()
        .filter(|commit| !covered.contains(*commit))
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let normalized = |path: &Path| {
        path.components()
            .filter(|part| !matches!(part, std::path::Component::CurDir))
            .collect::<std::path::PathBuf>()
    };
    let strategies = index
        .issues()
        .filter(|issue| issue.issue_type == "mission")
        .filter_map(|issue| issue.strategy.as_ref())
        .map(|strategy| normalized(Path::new(&strategy.path)))
        .collect::<BTreeSet<_>>();
    let mut checked = 0;
    for commit in &unreviewed {
        let parent = format!("{commit}^");
        if resolve(repo, &parent).is_err()
            || git_freshness::git_output(
                repo,
                &[
                    "ls-tree",
                    &parent,
                    "--",
                    workflow_policy::WORKFLOW_POLICY_PATH,
                ],
            )?
            .is_empty()
        {
            continue;
        }
        checked += 1;
        // A merge (such as a non-rebasing pull) introduces only the paths that
        // differ from every parent.
        let changed = text(
            repo,
            &[
                "diff-tree",
                "-r",
                "-c",
                "--no-commit-id",
                "--name-only",
                "-z",
                commit,
            ],
        )?;
        let changed = if text(repo, &["rev-list", "--parents", "-n1", commit])?
            .split(' ')
            .count()
            > 2
        {
            changed
                .split('\0')
                .filter(|path| !path.is_empty())
                .map(std::path::PathBuf::from)
                .collect::<Vec<_>>()
        } else {
            git_freshness::changed_paths(repo, &parent, commit)?
        };
        let other = changed
            .into_iter()
            .filter(|path| !git_freshness::is_record_path(path) && !strategies.contains(path))
            .collect::<Vec<_>>();
        if let Some(first) = other.first() {
            bail!(
                "land_base_unreviewed: commit {} on {base} changes {}{} without recorded review; only records and mission strategies may be committed on {base} directly. Move it to a work branch: git switch -c <issue_type>/<id>, git branch -f {base} {base}@{{upstream}}, then atelier start <id> there; get it reviewed and integrate it with atelier merge.",
                &commit[..8.min(commit.len())],
                first.display(),
                match other.len() {
                    1 => String::new(),
                    count => format!(" and {} other path(s)", count - 1),
                },
            );
        }
    }
    Ok(match checked {
        0 => "Base commits: none unpushed outside recorded review.".to_owned(),
        count => format!("Base commits: {count} unpushed record-only commit(s) outside review."),
    })
}

/// A done claim on a work branch is not base completion until the base
/// branch's own copy of the record carries it.
pub(crate) fn unmerged_done_label(
    repo: &Path,
    id: &str,
    status: &str,
    branch: &str,
    base_branch: &str,
) -> String {
    let path = format!(".atelier/issues/{id}.md");
    let merged = resolve(repo, &format!("refs/heads/{base_branch}"))
        .and_then(|base| git_freshness::blob(repo, &base, Path::new(&path)))
        .ok()
        .and_then(|blob| {
            atelier_records::parse_issue_record(
                &blob,
                Path::new(&path).strip_prefix(".atelier").ok()?,
            )
            .ok()
        })
        .is_some_and(|record| record.issue.status == status);
    if merged {
        format!("{status}, merged into {base_branch}")
    } else {
        format!("{status} on branch {branch}, not merged into {base_branch}")
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
    let mut untracked = Vec::new();
    let mut unstarted = Vec::new();
    for head in heads.lines() {
        let Some((reference, commit)) = head.split_once(' ') else {
            continue;
        };
        let branch = reference.trim_start_matches("refs/heads/");
        if branch == current || branch == base_branch {
            continue;
        }
        // Branches from before Atelier was initialised have no tracker to read.
        // `ls-tree` prints nothing for an absent path but still fails on
        // missing objects, which are reported below as real errors.
        let listed = git_freshness::git_output(
            repo,
            &[
                "ls-tree",
                commit,
                "--",
                workflow_policy::WORKFLOW_POLICY_PATH,
            ],
        );
        if listed.as_ref().is_ok_and(|output| output.is_empty()) {
            untracked.push(branch.to_string());
            continue;
        }
        let view = (|| -> Result<Option<Vec<String>>> {
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
                return Ok(None);
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
                let status = if branch_policy.status_category(&owner.status) == Some("done") {
                    unmerged_done_label(repo, &id, &owner.status, branch, base_branch)
                } else {
                    owner.status.clone()
                };
                rows.push(format!(
                    "{branch} @ {}: {} [{status}] {} — {active} active (committed branch records)",
                    &commit[..8],
                    id,
                    owner.title
                ));
            }
            Ok(Some(rows))
        })();
        match view {
            Ok(Some(rows)) => lines.extend(rows),
            Ok(None) => unstarted.push(branch.to_string()),
            Err(error) => lines.push(format!("{branch}: unavailable — {error:#}")),
        }
    }
    if !unstarted.is_empty() {
        lines.push(format!("No started work on: {}", unstarted.join(", ")));
    }
    if !untracked.is_empty() {
        lines.push(format!("No tracker on: {}", untracked.join(", ")));
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
        // Git 2.53+ refuses `git tag HEAD`; older repositories may still hold
        // this ref, so write it directly.
        git(repo.path(), &["update-ref", "refs/tags/HEAD", &base]);
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
        git(
            repo.path(),
            &["branch", whitespace_branch, "refs/heads/task/source"],
        );
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
