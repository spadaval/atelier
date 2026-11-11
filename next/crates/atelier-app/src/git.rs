//! The Git calls a [`Repo`](crate::Repo) and the acts here make, and the
//! helpers `atelier-cli`'s own Git calls are built on. `run` and
//! `current_branch` are copied from `text` and `checkout_branch` in
//! `crates/atelier-cli/src/commands/git.rs`, trimmed to plain errors; the
//! rest is new.

use crate::skip::Skip;
use anyhow::{bail, Context, Result};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// A Git command run in `repo`.
pub fn git(repo: &Path) -> Command {
    let mut command = Command::new("git");
    command.current_dir(repo).arg("--no-replace-objects");
    command
}

/// What a Git command printed and how it ended, with `input` on its stdin.
pub fn output(repo: &Path, args: &[&str], input: Option<&str>) -> Result<Output> {
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

/// [`run`], with `input` on its stdin.
pub fn run_with(repo: &Path, args: &[&str], input: Option<&str>) -> Result<String> {
    Ok(listing_with(repo, args, input)?.trim().to_string())
}

/// The output of a Git command that must succeed, untrimmed, as a list
/// separated by NULs needs: a leading space can be part of its first entry.
pub fn listing(repo: &Path, args: &[&str]) -> Result<String> {
    listing_with(repo, args, None)
}

fn listing_with(repo: &Path, args: &[&str], input: Option<&str>) -> Result<String> {
    let output = output(repo, args, input)?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8(output.stdout)?)
}

/// The top of the checkout `dir` is in.
pub fn root(dir: &Path) -> Result<PathBuf> {
    let output = output(dir, &["rev-parse", "--show-toplevel"], None)?;
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

/// Whether this clone has the commit `commit`.
pub fn has_commit(repo: &Path, commit: &str) -> Result<bool> {
    let object = format!("{commit}^{{commit}}");
    Ok(output(repo, &["cat-file", "-e", &object], None)?
        .status
        .success())
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

/// `message` with a trailer `Atelier-Skip: <rule>: <reason>` for each rule
/// `skip` passed, in its trailer paragraph.
pub fn with_skips(repo: &Path, message: &str, skip: &Skip) -> Result<String> {
    let passed = skip.passed();
    if passed.is_empty() {
        return Ok(message.to_string());
    }
    let trailers: Vec<String> = passed
        .iter()
        .map(|line| format!("Atelier-Skip: {line}"))
        .collect();
    let mut args = vec!["interpret-trailers"];
    for trailer in &trailers {
        args.extend(["--trailer", trailer]);
    }
    run_with(repo, &args, Some(message))
}

/// Commits the changes under `paths`, and only those, including new files,
/// with a trailer for each rule `skip` passed. When the commit fails, as
/// when a hook refuses it, the index under `paths` is put back as it was.
pub fn commit_paths(repo: &Path, paths: &[&str], message: &str, skip: &Skip) -> Result<()> {
    // With no path, `git commit --` would commit whatever is staged.
    if paths.is_empty() {
        bail!("there is nothing of this command's to commit");
    }
    let on_paths = |args: &[&'static str]| -> Vec<&str> {
        args.iter().chain(&["--"]).chain(paths).copied().collect()
    };
    let staged = output(repo, &on_paths(&["ls-files", "--stage", "-z"]), None)?;
    if !staged.status.success() {
        bail!(
            "git ls-files failed: {}",
            String::from_utf8_lossy(&staged.stderr).trim()
        );
    }
    let message = with_skips(repo, message, skip)?;
    let mut commit = vec!["commit", "--quiet", "-m", &message, "--"];
    commit.extend(paths);
    let committed = run(repo, &on_paths(&["add"])).and_then(|_| relayed(repo, &commit));
    if committed.is_err() {
        let unstage = on_paths(&["rm", "--cached", "-r", "-q", "--ignore-unmatch"]);
        let _ = run(repo, &unstage);
        let staged = String::from_utf8_lossy(&staged.stdout);
        let _ = run_with(repo, &["update-index", "-z", "--index-info"], Some(&staged));
    }
    committed.map(drop)
}

/// Runs a Git command that must succeed, and passes on to stderr what it
/// printed, which is a hook's output, as the rules a hook skipped.
pub fn relayed(repo: &Path, args: &[&str]) -> Result<()> {
    let output = output(repo, args, None)?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let mut stderr = std::io::stderr();
    let _ = stderr.write_all(&output.stdout);
    let _ = stderr.write_all(&output.stderr);
    Ok(())
}

/// The entries of a list separated by NULs, as Git's `-z` prints it.
pub fn names(listed: &str) -> Vec<String> {
    listed
        .split('\0')
        .filter(|name| !name.is_empty())
        .map(str::to_string)
        .collect()
}

/// The tracked files of the checkout whose changes are not committed,
/// staged or not. An untracked file is no commit's, so it is not listed.
pub fn uncommitted(repo: &Path) -> Result<Vec<String>> {
    let listed = listing(
        repo,
        &[
            "status",
            "--porcelain=v1",
            "-z",
            "--no-renames",
            "--untracked-files=no",
        ],
    )?;
    // Each entry is two status letters, a space and the path.
    Ok(names(&listed)
        .into_iter()
        .filter_map(|entry| entry.get(3..).map(str::to_string))
        .collect())
}

/// The files under `dir` that Git does not track, ignored or not.
pub fn untracked(repo: &Path, dir: &str) -> Result<Vec<String>> {
    let listed = listing(repo, &["ls-files", "--others", "-z", "--", dir])?;
    Ok(names(&listed))
}

/// The paths under `dirs` whose files in the checkout or in its index
/// differ from the commit `from`, committed or not, and the files there Git
/// does not track.
pub fn changed_since(repo: &Path, from: &str, dirs: &[&str]) -> Result<Vec<String>> {
    let args = ["diff", "--name-only", "-z", "--no-renames", from, "--"];
    let mut paths = names(&listing(repo, &[&args[..], dirs].concat())?);
    let staged = [
        "diff",
        "--cached",
        "--name-only",
        "-z",
        "--no-renames",
        from,
        "--",
    ];
    paths.extend(names(&listing(repo, &[&staged[..], dirs].concat())?));
    for dir in dirs {
        paths.extend(untracked(repo, dir)?);
    }
    paths.sort();
    paths.dedup();
    Ok(paths)
}

/// The paths that differ between the commits `from` and `to`.
pub fn changed(repo: &Path, from: &str, to: &str) -> Result<Vec<String>> {
    let listed = listing(
        repo,
        &["diff", "--name-only", "-z", "--no-renames", from, to, "--"],
    )?;
    Ok(names(&listed))
}

/// The checkout that has the local branch `branch` checked out, if one has.
pub fn checkout_of(repo: &Path, branch: &str) -> Result<Option<PathBuf>> {
    let listed = listing(repo, &["worktree", "list", "--porcelain", "-z"])?;
    let wanted = format!("branch refs/heads/{branch}");
    let mut path = None;
    for line in listed.split('\0') {
        if let Some(found) = line.strip_prefix("worktree ") {
            path = Some(PathBuf::from(found));
        } else if line == wanted {
            return Ok(path);
        }
    }
    Ok(None)
}

/// The commit the checkout's HEAD points at.
pub fn head(repo: &Path) -> Result<String> {
    run(repo, &["rev-parse", "--verify", "HEAD^{commit}"])
}
