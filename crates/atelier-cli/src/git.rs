//! The Git calls the commands that stay in this crate make, on the
//! helpers of `atelier_app::git`, which holds the calls a repository and
//! the acts there make.

use anyhow::{bail, Context, Result};
pub use atelier_app::git::{
    branch_tip, changed, checkout_of, current_branch, has_commit, head, is_ancestor, relayed, root,
    run, uncommitted, untracked,
};
use atelier_app::git::{git, listing, names, output, run_with, with_skips};
use atelier_app::Skip;
use std::path::{Path, PathBuf};
use std::process::Stdio;

/// The commit the branch `branch` points at where `remote` pushes to, if
/// that has the branch. `remote` is a remote's name or a URL.
pub fn remote_tip(repo: &Path, remote: &str, branch: &str) -> Result<Option<String>> {
    let url = run(repo, &["remote", "get-url", "--push", "--", remote])
        .unwrap_or_else(|_| remote.to_string());
    let reference = format!("refs/heads/{branch}");
    let listed = run(repo, &["ls-remote", "--", &url, &reference])?;
    Ok(listed.lines().find_map(|line| {
        let (commit, name) = line.split_once('\t')?;
        (name == reference).then(|| commit.to_string())
    }))
}

/// Writes a commit of the tree `tree` names, with the file at `path` set
/// to `text`, on `parents`, with a trailer for each rule `skip` passed,
/// and returns it. No branch, index or working tree changes: the commit is
/// reachable from nothing until a ref names it.
pub fn commit_file(
    repo: &Path,
    tree: &str,
    parents: &[&str],
    path: &str,
    text: &str,
    message: &str,
    skip: &Skip,
) -> Result<String> {
    let index = PathBuf::from(run(
        repo,
        &[
            "rev-parse",
            "--path-format=absolute",
            "--git-path",
            &format!("atelier-{}.index", std::process::id()),
        ],
    )?);
    let result = (|| {
        let blob = run_with(repo, &["hash-object", "-w", "--stdin"], Some(text))?;
        let with_index = |args: &[&str]| -> Result<String> {
            let output = git(repo)
                .env("GIT_INDEX_FILE", &index)
                .args(args)
                .stdin(Stdio::null())
                .output()?;
            if !output.status.success() {
                bail!(
                    "git {} failed: {}",
                    args.join(" "),
                    String::from_utf8_lossy(&output.stderr).trim()
                );
            }
            Ok(String::from_utf8(output.stdout)?.trim().to_string())
        };
        with_index(&["read-tree", tree])?;
        with_index(&[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("100644,{blob},{path}"),
        ])?;
        let tree = with_index(&["write-tree"])?;
        let message = with_skips(repo, message, skip)?;
        let mut args = vec!["commit-tree", &tree, "-m", &message];
        for parent in parents {
            args.extend(["-p", parent]);
        }
        run(repo, &args)
    })();
    let _ = std::fs::remove_file(&index);
    result
}

/// Writes a commit of the tree `tree` on `parents`, with a trailer for
/// each rule `skip` passed, and returns it. Nothing else changes.
pub fn commit_tree(
    repo: &Path,
    tree: &str,
    parents: &[&str],
    message: &str,
    skip: &Skip,
) -> Result<String> {
    let message = with_skips(repo, message, skip)?;
    let mut args = vec!["commit-tree", tree, "-m", &message];
    for parent in parents {
        args.extend(["-p", parent]);
    }
    run(repo, &args)
}

/// The paths the commit `head` changes against the commit `base`: since
/// the two parted, as a pull request shows them.
pub fn changed_from_base(repo: &Path, base: &str, head: &str) -> Result<Vec<String>> {
    let range = format!("{base}...{head}");
    let listed = listing(
        repo,
        &["diff", "--name-only", "-z", "--no-renames", &range, "--"],
    )?;
    Ok(names(&listed))
}

/// Creates the local branch `name` at `commit`, or returns `false` when a
/// branch of that name exists. Git creates a ref atomically, so of two
/// callers racing, exactly one gets `true`.
pub fn create_branch(repo: &Path, name: &str, commit: &str) -> Result<bool> {
    let reference = format!("refs/heads/{name}");
    let output = output(repo, &["update-ref", &reference, commit, ""], None)?;
    if output.status.success() {
        return Ok(true);
    }
    if branch_tip(repo, name)?.is_some() {
        return Ok(false);
    }
    bail!(
        "git update-ref {reference} failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    )
}

/// Moves the local branch `name` from `from` to `commit`, refused when it
/// is no longer at `from`.
pub fn move_branch(repo: &Path, name: &str, commit: &str, from: &str) -> Result<()> {
    let reference = format!("refs/heads/{name}");
    run(repo, &["update-ref", &reference, commit, from]).map(drop)
}

/// Deletes the local branch `name`, refused when it is no longer at `at`.
pub fn delete_branch(repo: &Path, name: &str, at: &str) -> Result<()> {
    let reference = format!("refs/heads/{name}");
    run(repo, &["update-ref", "-d", &reference, at]).map(drop)
}

pub fn switch(repo: &Path, branch: &str) -> Result<()> {
    run(repo, &["switch", "--quiet", branch]).map(drop)
}

/// The directory Git runs the checkout's hooks from: the clone's shared
/// `hooks`, or where `core.hooksPath` points.
pub fn hooks_dir(repo: &Path) -> Result<PathBuf> {
    let dir = run(
        repo,
        &["rev-parse", "--path-format=absolute", "--git-path", "hooks"],
    )?;
    Ok(PathBuf::from(dir))
}

/// The Git directory every worktree of the clone shares.
pub fn common_dir(repo: &Path) -> Result<PathBuf> {
    let dir = run(
        repo,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    Ok(PathBuf::from(dir))
}

/// The files under `dir` in the commit `commit`.
pub fn files_at(repo: &Path, commit: &str, dir: &str) -> Result<Vec<String>> {
    let args = ["ls-tree", "-r", "-z", "--name-only", "--full-tree", commit];
    Ok(names(&listing(repo, &[&args[..], &["--", dir]].concat())?))
}

/// The files the index holds under `dir`.
pub fn staged_files(repo: &Path, dir: &str) -> Result<Vec<String>> {
    Ok(names(&listing(repo, &["ls-files", "-z", "--", dir])?))
}

/// The files under `dir` the index adds or changes from `HEAD`, or that it
/// holds when there is no `HEAD` yet.
pub fn staged_changes(repo: &Path, dir: &str) -> Result<Vec<String>> {
    let args = [
        "diff",
        "--cached",
        "--name-only",
        "-z",
        "--no-renames",
        "--diff-filter=d",
        "--",
        dir,
    ];
    Ok(names(&listing(repo, &args)?))
}

/// The files under `dir` the index adds, changes or deletes from `HEAD`,
/// or that it holds when there is no `HEAD` yet.
pub fn staged_all(repo: &Path, dir: &str) -> Result<Vec<String>> {
    if !has_commit(repo, "HEAD")? {
        return staged_files(repo, dir);
    }
    let args = [
        "diff",
        "--cached",
        "--name-only",
        "-z",
        "--no-renames",
        "HEAD",
        "--",
        dir,
    ];
    Ok(names(&listing(repo, &args)?))
}

/// A commit on a first-parent line, as [`first_parent_line`] lists it.
pub struct LineCommit {
    pub commit: String,
    pub parents: Vec<String>,
    pub subject: String,
    /// Whether its message ends in an `Atelier-Merge:` trailer.
    pub atelier_merge: bool,
    /// The paths it changes from its first parent; a root commit's every
    /// path.
    pub paths: Vec<String>,
}

/// The commits on the first-parent line of `tip` that are not in the
/// history of `known`, newest first.
pub fn first_parent_line(repo: &Path, tip: &str, known: Option<&str>) -> Result<Vec<LineCommit>> {
    let not = known.map(|commit| format!("^{commit}"));
    let mut args = vec![
        "log",
        "-z",
        "--no-color",
        "--first-parent",
        "--diff-merges=first-parent",
        "--root",
        "--no-renames",
        "--name-only",
        "--format=%x01%H %P%x02%s%x02%(trailers:key=Atelier-Merge,valueonly)",
        tip,
    ];
    args.extend(not.as_deref());
    args.push("--");
    let listed = listing(repo, &args)?;
    // Each commit is \x01, its ID and parents, \x02, its subject, \x02,
    // its trailer's values, a NUL, a newline, and its paths each ended by
    // a NUL.
    listed
        .split('\x01')
        .skip(1)
        .map(|entry| {
            let mut fields = entry.splitn(3, '\x02');
            let (Some(ids), Some(subject), Some(rest)) =
                (fields.next(), fields.next(), fields.next())
            else {
                bail!("git log printed a commit this tool cannot read: {entry:?}");
            };
            let (trailer, paths) = rest.split_once('\0').unwrap_or((rest, ""));
            let mut ids = ids
                .split(' ')
                .filter(|id| !id.is_empty())
                .map(str::to_string);
            Ok(LineCommit {
                commit: ids.next().context("git log printed no commit ID")?,
                parents: ids.collect(),
                subject: subject.to_string(),
                atelier_merge: !trailer.trim().is_empty(),
                paths: names(paths.strip_prefix('\n').unwrap_or(paths)),
            })
        })
        .collect()
}

/// Git's merge of the commit `theirs` into `ours`, made without a
/// checkout: the merged tree, or the paths that conflict.
pub fn merge_tree(repo: &Path, ours: &str, theirs: &str) -> Result<Result<String, Vec<String>>> {
    let args = [
        "merge-tree",
        "--write-tree",
        "--name-only",
        "--no-messages",
        "-z",
        ours,
        theirs,
    ];
    let output = output(repo, &args, None)?;
    let listed = String::from_utf8(output.stdout)?;
    let mut fields = names(&listed).into_iter();
    match (output.status.code(), fields.next()) {
        (Some(0), Some(tree)) => Ok(Ok(tree)),
        (Some(1), Some(_)) => Ok(Err(fields.collect())),
        _ => bail!(
            "git merge-tree failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ),
    }
}
