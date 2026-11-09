//! A work branch as the base sees it, read from refs and commits alone, for
//! `atelier status` and the site's board: which issues own it, and how it
//! meets the base. Nothing here changes a ref, the index or a checkout.

use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::git_freshness;
use crate::record_index::RecordIndex;

fn text(repo: &Path, args: &[&str]) -> Result<String> {
    Ok(String::from_utf8(git_freshness::git_output(repo, args)?)?
        .trim()
        .into())
}

/// How a branch at `tip` meets the base at `base_head`; the same for every
/// owner of the branch.
pub struct BranchMerge {
    /// Where the branch left the base.
    pub fork_point: String,
    /// Base commits the branch does not have.
    pub behind: usize,
    /// The paths `git merge-tree` cannot merge into the base.
    pub conflicts: Vec<PathBuf>,
    /// The paths the branch changes since `fork_point`, sorted.
    pub changed: Vec<PathBuf>,
}

/// Reads the branch's merge into the base with `git merge-tree`, which
/// writes only unreachable objects, so no ref, index or checkout changes.
pub fn branch_merge(repo: &Path, tip: &str, base_head: &str) -> Result<BranchMerge> {
    let fork_point = text(repo, &["merge-base", base_head, tip])?;
    let behind = text(
        repo,
        &["rev-list", "--count", &format!("{tip}..{base_head}")],
    )?
    .parse()
    .context("git rev-list --count printed no count")?;
    let conflicts = if behind == 0 {
        Vec::new()
    } else {
        git_freshness::merge_tree(repo, base_head, tip)?
            .context("git merge-tree could not merge the branch into the base")?
            .1
    };
    let changed = git_freshness::changed_paths(repo, &fork_point, tip)?;
    Ok(BranchMerge {
        fork_point,
        behind,
        conflicts,
        changed,
    })
}

/// The issues a branch's committed records say own it, read from their small
/// record blobs without materializing the tracker.
pub fn branch_owner_ids(
    repo: &Path,
    live: &RecordIndex,
    branch: &str,
    commit: &str,
) -> Result<Vec<String>> {
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
    Ok(owners)
}
