//! `atelier merge <id> --sync`: the catch-up step before a merge, in one
//! command. It merges the base into the owner's work branch with `git merge`
//! (never a rebase, which would orphan the commits reviews pin) and reports
//! whether the approval survived, by the same rule the gates apply. It
//! records no receipts: CI or the policy's merge checks test the merged tree
//! where the repository has them. It integrates nothing: `atelier merge`
//! still does that.
use super::merge::{owned_branch, Owned};
use crate::commands::git::{resolve, text};
use anyhow::{bail, Context, Result};
use atelier_app::{
    committed_snapshot::CommittedSnapshot,
    git_freshness as git,
    review_gate::{self, NotCurrent},
};
use atelier_core::count_noun;
use std::collections::BTreeSet;
use std::path::Path;

const CONFLICT: &str = "sync_conflict";
const APPROVAL_ERROR: &str = "sync_approval_error";
const LISTED_PATHS: usize = 20;

pub fn run(issue_ref: &str, quiet: bool) -> Result<()> {
    let say = |line: String| {
        if !quiet {
            println!("{line}");
        }
    };
    let (failures, stale) = match sync(issue_ref, &say) {
        Ok(synced) => synced,
        Err(error) => {
            eprintln!("Error: {error:?}");
            (vec![refusal_code(&error)], false)
        }
    };
    if failures.is_empty() {
        // A sync that leaves the approval stale did its work, but `merge
        // --check` will refuse: the verdict says so instead of "passed".
        let verdict = if stale {
            "synced — approval needs fresh review"
        } else {
            "passed"
        };
        if quiet {
            println!("{verdict}");
        } else {
            println!("merge: {verdict}");
        }
        return Ok(());
    }
    let codes: BTreeSet<_> = failures.iter().flatten().collect();
    println!(
        "merge: refused{}",
        if codes.is_empty() {
            String::new()
        } else {
            format!(
                " — {} ({})",
                count_noun(codes.len(), "check"),
                codes.into_iter().cloned().collect::<Vec<_>>().join(", ")
            )
        }
    );
    Err(super::session::Reported.into())
}

/// The leading `sync_*` or `merge_*` code of a refusal, if it has one.
fn refusal_code(error: &anyhow::Error) -> Option<String> {
    let message = error.to_string();
    let (code, _) = message.split_once(':')?;
    (code.chars().all(|c| c.is_ascii_lowercase() || c == '_')
        && (code.starts_with("sync_") || code.starts_with("merge_")))
    .then(|| code.to_owned())
}

/// The checks that failed, as codes (empty when the sync passed), and whether
/// the approval was left needing fresh review.
fn sync(issue_ref: &str, say: &dyn Fn(String)) -> Result<(Vec<Option<String>>, bool)> {
    let Owned {
        layout,
        head,
        branch,
        owner,
        base,
        ..
    } = owned_branch(issue_ref)?;
    let repo = layout.repo_root();
    let base_head = resolve(repo, &format!("refs/heads/{base}"))
        .with_context(|| format!("sync_base_missing: base branch {base} does not exist locally"))?;
    let contains = ancestor(repo, &base_head, &head);
    // Where the branch stood before the base came in: the head before the
    // merge, or the merge commit's first parent when the base was just merged
    // in by hand (a conflict resolved, say), so the approval report can say
    // whether the approval was current before.
    let before = if !contains {
        Some(head.clone())
    } else {
        hand_merge_parent(repo, &head, &base_head)?
    };

    if contains {
        say(format!(
            "{branch} already contains {base}; nothing to merge."
        ));
    } else {
        say(format!("Merging {base} into {branch}."));
        let merge = git::command(repo)
            .args([
                "-c",
                "core.hooksPath=/dev/null",
                "merge",
                "--no-edit",
                &base,
            ])
            .output()?;
        if !merge.status.success() {
            let conflicted = text(repo, &["diff", "--name-only", "--diff-filter=U"])?;
            if conflicted.is_empty() {
                bail!(
                    "sync_merge_failed: git merge {base} failed: {}",
                    String::from_utf8_lossy(&merge.stderr).trim()
                );
            }
            eprintln!(
                "{CONFLICT}: merging {base} into {branch} conflicts in:\n{}\nNothing was committed and the merge is still in progress. Resolve the conflicts and commit (`git commit`), or `git merge --abort`; then rerun `atelier merge {issue_ref} --sync`, and expect the approval to need fresh review of the resolved files.",
                conflicted.lines().map(|path| format!("  {path}")).collect::<Vec<_>>().join("\n")
            );
            return Ok((vec![Some(CONFLICT.into())], false));
        }
        say(format!(
            "Merged {base} into {branch} at {}.",
            crate::human_output::short_commit(&git::head_commit(repo)?)
        ));
    }

    let mut failures = Vec::new();
    let stale = match report_approval(repo, &layout, &owner, &base_head, before.as_deref(), say) {
        Ok(stale) => stale,
        Err(error) => {
            eprintln!("{APPROVAL_ERROR}: could not tell whether the approval survived: {error:#}");
            failures.push(Some(APPROVAL_ERROR.to_owned()));
            false
        }
    };
    if failures.is_empty() && !stale {
        say(format!("Next: atelier merge {issue_ref} --check"));
    }
    Ok((failures, stale))
}

/// Whether `head` is a merge commit that brought the base in and nothing
/// has been committed since; its first parent is where the branch stood.
fn hand_merge_parent(repo: &Path, head: &str, base_head: &str) -> Result<Option<String>> {
    let line = text(repo, &["rev-list", "--parents", "-n", "1", head])?;
    let parts = line.split_whitespace().collect::<Vec<_>>();
    Ok(match parts.as_slice() {
        [_, first, second] if ancestor(repo, second, base_head) => Some((*first).to_owned()),
        _ => None,
    })
}

fn ancestor(repo: &Path, earlier: &str, later: &str) -> bool {
    git::git_output(repo, &["merge-base", "--is-ancestor", earlier, later]).is_ok()
}

/// Says whether the owner's approval survived, judged and explained by
/// `review_gate::approval_standing` as `review.current` judges it, and when
/// not, which paths the new review has to read. Returns whether the approval
/// needs fresh review.
fn report_approval(
    repo: &Path,
    layout: &atelier_app::storage_layout::StorageLayout,
    owner: &str,
    base_head: &str,
    before: Option<&str>,
    say: &dyn Fn(String),
) -> Result<bool> {
    let head = git::head_commit(repo)?;
    let index = {
        let _transaction =
            atelier_records::mutation_lock::CanonicalMutationLock::shared(&layout.canonical_dir())?;
        CommittedSnapshot::new(&head)?
            .read_record_files(repo, false)?
            .0
            .into_index()?
    };
    let ids = review_gate::scope(&index, owner)?;
    let mut read = git::ReadContext::from_snapshot(repo, head.clone(), &[]);
    // `merge`'s `review.current` also requires that the approval not predate
    // where the branch left the base (atelier-ns03); ask it the same way.
    let fork = text(repo, &["merge-base", base_head, &head])?;
    let standing = review_gate::approval_standing(&index, &mut read, owner, &fork)?;
    let (Some(review), Some(why)) = (standing.record, &standing.not_current) else {
        match standing.record {
            Some(review) => say(format!(
                "Approval kept: {} still covers the branch.",
                review.header.id
            )),
            None => say(format!(
                "No independent approval covers {owner}'s scope yet; there is none to keep."
            )),
        }
        return Ok(false);
    };
    let paths = match why {
        NotCurrent::Author => {
            say(format!(
                "No independent approval covers {owner}'s scope yet: {} by {} is not independent, as the identity matches a Git author of the branch ({}).",
                review.header.id,
                review.identity,
                standing.shown_authors().join(", ")
            ));
            return Ok(false);
        }
        NotCurrent::PredatesFork => {
            // The base already holds the work after this approval (the branch
            // was merged there), so no commit on the branch is the one that
            // made it stale: the approval predates the point the branch left
            // the base.
            let mut paths = git::changed_paths(repo, &review.commit, &head)?
                .into_iter()
                .filter(|path| !git::is_record_path(path))
                .collect::<Vec<_>>();
            paths.sort();
            paths.dedup();
            say(format!(
                "Approval needs fresh review: {} predates the point where this branch left the base, which already holds the work since. Review {}, then record a fresh approval:",
                review.header.id,
                count_noun(paths.len(), "path"),
            ));
            paths
        }
        NotCurrent::Stale(edges) => {
            let fork =
                before.and_then(|before| text(repo, &["merge-base", before, base_head]).ok());
            let was_current = match (before, fork) {
                (Some(before), Some(fork)) => {
                    let mut earlier = git::ReadContext::from_snapshot(repo, before.to_owned(), &[]);
                    review_gate::approval_is_current_to(
                        &mut earlier,
                        &review.commit,
                        &ids,
                        "HEAD",
                        &fork,
                    )
                    .unwrap_or(false)
                }
                _ => false,
            };
            let mut paths = edges
                .iter()
                .flat_map(|edge| edge.paths.iter().cloned())
                .collect::<Vec<_>>();
            paths.sort();
            paths.dedup();
            say(format!(
                "Approval needs fresh review: {} first went stale at {} ({}). Review {}, then record a fresh approval:",
                review.header.id,
                crate::human_output::short_commit(&edges[0].commit),
                if was_current {
                    "it was current before this sync"
                } else {
                    "it was already stale before this sync"
                },
                count_noun(paths.len(), "path"),
            ));
            paths
        }
    };
    for path in paths.iter().take(LISTED_PATHS) {
        say(format!("  {}", path.display()));
    }
    if paths.len() > LISTED_PATHS {
        say(format!("  and {} more", paths.len() - LISTED_PATHS));
    }
    Ok(true)
}
