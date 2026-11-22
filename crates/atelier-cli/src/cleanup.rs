//! What `merge` does once a branch has merged (docs/design.md, After the
//! merge): it closes the pull request of each item a backlog branch
//! dropped, and deletes the branch, on the forge's remote and here. None of
//! it can fail a merge that is made: what cannot be done is said.

use crate::{git, merge, output};
use anyhow::Result;
use atelier_core::forge::Github;
use atelier_core::model::{Forge, Named, Status};
use atelier_core::Repo;

/// The items the checked-out backlog branch at `head` drops, which the base
/// at `base_tip` does not hold dropped; none on a branch of another kind.
pub fn dropped_here(repo: &mut Repo, base_tip: &str, head: &str) -> Result<Vec<String>> {
    if !matches!(repo.named(), Some(Named::Backlog(_))) {
        return Ok(Vec::new());
    }
    let mut dropped = Vec::new();
    for path in git::changed_from_base(&repo.root, base_tip, head)? {
        let Some(id) = path
            .strip_prefix(".atelier/items/")
            .and_then(|rest| rest.strip_suffix("/item.md"))
        else {
            continue;
        };
        // A base copy that does not read is not dropped, and must not stop
        // the merge that replaces it.
        let mut is_dropped = |at: &str| {
            let found = repo.item_at(at, id).ok().flatten();
            found.is_some_and(|dir| dir.item.status == Status::Dropped)
        };
        if is_dropped(head) && !is_dropped(base_tip) {
            dropped.push(id.to_string());
        }
    }
    Ok(dropped)
}

/// Closes the open pull request from the branch of each of `dropped`,
/// which merged branch `by` dropped. The forge is asked for each, whatever
/// this clone has of its branch, since it may have been started in another
/// clone; one never started has no pull request.
pub fn close_pull_requests(repo: &Repo, forge: &Forge, dropped: &[String], by: &str) -> String {
    if dropped.is_empty() {
        return String::new();
    }
    let github = match Github::open(&repo.root, forge) {
        Ok(github) => github.until(merge::locked_until()),
        Err(error) => return output::dropped_not_closed(&dropped.join(", "), &error),
    };
    let mut said = String::new();
    for id in dropped {
        let closed = github
            .open_pull_request(&repo.branch_of(id), &repo.base)
            .and_then(|pull| match pull {
                None => Ok(None),
                Some(pull) => {
                    let comment = output::dropped_comment(id, &repo.base, by);
                    github.close_pull_request(pull.number, &comment)?;
                    Ok(Some(pull))
                }
            });
        said += &match closed {
            Ok(None) => String::new(),
            Ok(Some(pull)) => output::dropped_closed(id, &pull),
            Err(error) => output::dropped_not_closed(id, &error),
        };
    }
    said
}

/// Deletes `branch`, merged at `head`, and says what it did. Neither ref
/// goes unless the base this clone reads holds `head`, and with a forge the
/// forge's base as fetched too, since the remote's copy is deleted. The
/// remote's goes only while still at `head`, so a late push survives; the
/// local one only from a checkout with nothing uncommitted, which is left
/// on a detached HEAD at `head`.
pub fn delete_merged(repo: &mut Repo, forge: Option<&Forge>, branch: &str, head: &str) -> String {
    deleted(repo, forge, branch, head)
        .unwrap_or_else(|error| output::cleanup_failed(branch, &error))
}

fn deleted(repo: &mut Repo, forge: Option<&Forge>, branch: &str, head: &str) -> Result<String> {
    repo.reread_base()?;
    let root = repo.root.clone();
    let mut holds = git::is_ancestor(&root, head, &repo.base_tip)?;
    if let Some(forge) = forge {
        holds &= match atelier_core::git::remote_tip(&root, &forge.remote, &repo.base)? {
            Some(tip) => git::is_ancestor(&root, head, &tip)?,
            None => false,
        };
    }
    if !holds {
        let remote = forge.map(|forge| forge.remote.as_str());
        return Ok(output::branch_kept(branch, &repo.base, remote));
    }
    let mut said = String::new();
    if let Some(forge) = forge {
        let remote = &forge.remote;
        said += &match git::delete_remote_branch(&root, remote, branch, head, repo.skip) {
            Ok(()) => output::remote_deleted(branch, remote),
            Err(error) => match git::remote_tip(&root, remote, branch) {
                Ok(None) => output::remote_already_deleted(branch, remote),
                _ => output::remote_not_deleted(branch, remote, &error),
            },
        };
    }
    let here = git::current_branch(&root)?.as_deref() == Some(branch)
        && git::head(&root)? == head
        && !unsaved(repo)?;
    if !here {
        return Ok(said + &output::branch_attached(branch));
    }
    let local = git::detach(&root, head).and_then(|()| git::delete_branch(&root, branch, head));
    said += &match local {
        Ok(()) => output::branch_deleted(branch, head),
        Err(error) => output::cleanup_failed(branch, &error),
    };
    Ok(said)
}

/// Whether the checkout holds what no commit does that leaving its branch
/// would strand: a change to a tracked file, or a new record.
pub fn unsaved(repo: &Repo) -> Result<bool> {
    Ok(!git::uncommitted(&repo.root)?.is_empty()
        || !atelier_core::repo::planning(&repo.root)?.is_empty())
}
