//! The Git calls the commands make. `run` and `current_branch` are copied
//! from `text` and `checkout_branch` in
//! `crates/atelier-cli/src/commands/git.rs`, trimmed to plain errors; the
//! rest is new.

use anyhow::{bail, Context, Result};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

fn git(repo: &Path) -> Command {
    let mut command = Command::new("git");
    command.current_dir(repo).arg("--no-replace-objects");
    command
}

fn output(repo: &Path, args: &[&str], input: Option<&str>) -> Result<Output> {
    let mut child = git(repo)
        .args(args)
        .stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("cannot start git {}", args.join(" ")))?;
    if let Some(input) = input {
        // Inputs are one record, small enough not to block on the output.
        let mut stdin = child.stdin.take().expect("stdin is piped");
        stdin.write_all(input.as_bytes())?;
    }
    Ok(child.wait_with_output()?)
}

/// The trimmed output of a Git command that must succeed.
pub fn run(repo: &Path, args: &[&str]) -> Result<String> {
    run_with(repo, args, None)
}

fn run_with(repo: &Path, args: &[&str], input: Option<&str>) -> Result<String> {
    let output = output(repo, args, input)?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

/// The top of the checkout the command runs in.
pub fn root() -> Result<PathBuf> {
    let output = output(Path::new("."), &["rev-parse", "--show-toplevel"], None)?;
    if !output.status.success() {
        bail!(
            "this is not a Git checkout: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(PathBuf::from(String::from_utf8(output.stdout)?.trim()))
}

/// The branch checked out, or `None` on a detached HEAD.
pub fn current_branch(repo: &Path) -> Result<Option<String>> {
    let output = output(repo, &["symbolic-ref", "--quiet", "HEAD"], None)?;
    if output.status.code() == Some(1) {
        return Ok(None);
    }
    if !output.status.success() {
        bail!(
            "cannot read the checked-out branch: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let reference = String::from_utf8(output.stdout)?;
    Ok(reference
        .trim_end_matches('\n')
        .strip_prefix("refs/heads/")
        .map(str::to_string))
}

/// The commit the local branch `name` points at, if it exists.
pub fn branch_tip(repo: &Path, name: &str) -> Result<Option<String>> {
    let reference = format!("refs/heads/{name}^{{commit}}");
    let output = output(
        repo,
        &["rev-parse", "--verify", "--quiet", &reference],
        None,
    )?;
    Ok(output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string()))
}

/// Every local branch, with the commit it points at.
pub fn branches(repo: &Path) -> Result<Vec<(String, String)>> {
    let listed = run(
        repo,
        &[
            "for-each-ref",
            "--format=%(refname:strip=2) %(objectname)",
            "refs/heads/",
        ],
    )?;
    Ok(listed
        .lines()
        .filter_map(|line| line.rsplit_once(' '))
        .map(|(name, tip)| (name.to_string(), tip.to_string()))
        .collect())
}

/// Whether `commit` is in the history of `of`.
pub fn is_ancestor(repo: &Path, commit: &str, of: &str) -> Result<bool> {
    let output = output(repo, &["merge-base", "--is-ancestor", commit, of], None)?;
    match output.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => bail!(
            "git merge-base --is-ancestor failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ),
    }
}

/// Writes a commit on top of `parent` that changes only the file at
/// `path` to `text`, and returns it. No branch, index or working tree
/// changes: the commit is reachable from nothing until a branch names it.
pub fn commit_file(
    repo: &Path,
    parent: &str,
    path: &str,
    text: &str,
    message: &str,
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
        with_index(&["read-tree", parent])?;
        with_index(&[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("100644,{blob},{path}"),
        ])?;
        let tree = with_index(&["write-tree"])?;
        run(repo, &["commit-tree", &tree, "-p", parent, "-m", message])
    })();
    let _ = std::fs::remove_file(&index);
    result
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

pub fn switch(repo: &Path, branch: &str) -> Result<()> {
    run(repo, &["switch", "--quiet", branch]).map(drop)
}

/// Commits the changes under `path`, and only those, including new files.
pub fn commit_path(repo: &Path, path: &str, message: &str) -> Result<()> {
    run(repo, &["add", "--", path])?;
    run(repo, &["commit", "--quiet", "-m", message, "--", path]).map(drop)
}
