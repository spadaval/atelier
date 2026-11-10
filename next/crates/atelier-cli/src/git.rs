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
    Ok(listing_with(repo, args, input)?.trim().to_string())
}

/// The output of a Git command that must succeed, untrimmed, as a list
/// separated by NULs needs: a leading space can be part of its first entry.
fn listing(repo: &Path, args: &[&str]) -> Result<String> {
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

/// Whether this clone has the commit `commit`.
pub fn has_commit(repo: &Path, commit: &str) -> Result<bool> {
    let object = format!("{commit}^{{commit}}");
    Ok(output(repo, &["cat-file", "-e", &object], None)?
        .status
        .success())
}

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

/// Writes a commit of the tree `tree` names, with the file at `path` set
/// to `text`, on `parents`, and returns it. No branch, index or working
/// tree changes: the commit is reachable from nothing until a ref names it.
pub fn commit_file(
    repo: &Path,
    tree: &str,
    parents: &[&str],
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
        with_index(&["read-tree", tree])?;
        with_index(&[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("100644,{blob},{path}"),
        ])?;
        let tree = with_index(&["write-tree"])?;
        let mut args = vec!["commit-tree", &tree, "-m", message];
        for parent in parents {
            args.extend(["-p", parent]);
        }
        run(repo, &args)
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

/// Commits the changes under `paths`, and only those, including new files.
/// When the commit fails, as when a hook refuses it, the index under
/// `paths` is put back as it was.
pub fn commit_paths(repo: &Path, paths: &[&str], message: &str) -> Result<()> {
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
    let mut commit = vec!["commit", "--quiet", "-m", message, "--"];
    commit.extend(paths);
    let committed = run(repo, &on_paths(&["add"])).and_then(|_| run(repo, &commit));
    if committed.is_err() {
        let unstage = on_paths(&["rm", "--cached", "-r", "-q", "--ignore-unmatch"]);
        let _ = run(repo, &unstage);
        let staged = String::from_utf8_lossy(&staged.stdout);
        let _ = run_with(repo, &["update-index", "-z", "--index-info"], Some(&staged));
    }
    committed.map(drop)
}

/// The commit the checkout's HEAD points at.
pub fn head(repo: &Path) -> Result<String> {
    run(repo, &["rev-parse", "--verify", "HEAD^{commit}"])
}

/// The Git directory every worktree of the clone shares.
pub fn common_dir(repo: &Path) -> Result<PathBuf> {
    let dir = run(
        repo,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    Ok(PathBuf::from(dir))
}

fn names(listed: &str) -> Vec<String> {
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

/// The files under `dir` that are tracked, or untracked and not ignored.
pub fn files(repo: &Path, dir: &str) -> Result<Vec<String>> {
    let args = [
        "ls-files",
        "--cached",
        "--others",
        "--exclude-standard",
        "-z",
    ];
    let listed = listing(repo, &[&args[..], &["--", dir]].concat())?;
    Ok(names(&listed))
}

/// The paths under `dirs` whose files in the checkout differ from the
/// commit `from`, committed or not, and the files there Git does not track.
pub fn changed_since(repo: &Path, from: &str, dirs: &[&str]) -> Result<Vec<String>> {
    let args = ["diff", "--name-only", "-z", "--no-renames", from, "--"];
    let mut paths = names(&listing(repo, &[&args[..], dirs].concat())?);
    for dir in dirs {
        paths.extend(untracked(repo, dir)?);
    }
    paths.sort();
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

/// The committer time, in RFC 3339, of the last commit in `rev`'s history
/// that set `verdict: approve` in the review file at `path`.
pub fn approved_at(repo: &Path, rev: &str, path: &str) -> Result<Option<String>> {
    let time = run(
        repo,
        &[
            "log",
            "-1",
            "--format=%cI",
            "-Sverdict: approve",
            rev,
            "--",
            path,
        ],
    )?;
    Ok(Some(time).filter(|time| !time.is_empty()))
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
