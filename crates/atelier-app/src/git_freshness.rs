//! Git history primitives shared by evidence and review freshness checks.
use anyhow::{bail, Context, Result};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    process::Command,
};

pub struct CommitEdge {
    pub parent: String,
    pub commit: String,
    pub paths: Vec<PathBuf>,
}

pub fn git_output(repo: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new("git").current_dir(repo).args(args).output()?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(output.stdout)
}
pub fn head_commit(repo: &Path) -> Result<String> {
    Ok(
        String::from_utf8(git_output(repo, &["rev-parse", "HEAD"])?)?
            .trim()
            .to_string(),
    )
}
pub fn blob(repo: &Path, commit: &str, path: &Path) -> Result<String> {
    let spec = format!("{commit}:{}", path.display());
    String::from_utf8(git_output(repo, &["show", &spec])?).context("Git record blob is not UTF-8")
}
fn paths(bytes: Vec<u8>) -> Result<Vec<PathBuf>> {
    bytes
        .split(|byte| *byte == 0)
        .filter(|part| !part.is_empty())
        .map(|part| Ok(PathBuf::from(std::str::from_utf8(part)?)))
        .collect()
}
pub fn changed_paths(repo: &Path, before: &str, after: &str) -> Result<Vec<PathBuf>> {
    paths(git_output(
        repo,
        &["diff", "--name-only", "-z", before, after, "--"],
    )?)
}
/// Inspects all introduced commits, including side-branch edit/revert history.
/// Merge results are compared to a parent containing the tested pin when possible:
/// older parents must not reintroduce changes already present at the pin.
pub fn commit_edges(repo: &Path, pin: &str) -> Result<Vec<CommitEdge>> {
    if !matches!(pin.len(), 40 | 64) || !pin.bytes().all(|b| b.is_ascii_hexdigit()) {
        bail!("pin must be a full Git commit ID");
    }
    git_output(repo, &["cat-file", "-e", &format!("{pin}^{{commit}}")])?;
    git_output(repo, &["merge-base", "--is-ancestor", pin, "HEAD"])?;
    let range = format!("{pin}..HEAD");
    let commits = String::from_utf8(git_output(repo, &["rev-list", "--parents", &range])?)?;
    let descendants =
        String::from_utf8(git_output(repo, &["rev-list", "--ancestry-path", &range])?)?;
    let mut tested_context: HashSet<&str> = descendants.lines().collect();
    tested_context.insert(pin);
    let mut edges = Vec::new();
    for line in commits.lines() {
        let mut parts = line.split_whitespace();
        let commit = parts.next().expect("rev-list commit");
        let parents: Vec<_> = parts.collect();
        if let Some(parent) = parents
            .iter()
            .find(|parent| tested_context.contains(**parent))
            .or_else(|| parents.first())
        {
            edges.push(CommitEdge {
                parent: parent.to_string(),
                commit: commit.to_string(),
                paths: changed_paths(repo, parent, commit)?,
            });
        }
    }
    Ok(edges)
}
pub fn working_paths(repo: &Path) -> Result<Vec<PathBuf>> {
    let mut changed = paths(git_output(
        repo,
        &["diff", "--cached", "--name-only", "-z", "HEAD", "--"],
    )?)?;
    changed.extend(paths(git_output(
        repo,
        &["diff", "--name-only", "-z", "--"],
    )?)?);
    changed.extend(paths(git_output(
        repo,
        &["ls-files", "--others", "--exclude-standard", "-z"],
    )?)?);
    Ok(changed)
}
/// Record-only paths; policy and project config are intentionally excluded.
pub fn is_record_path(path: &Path) -> bool {
    let mut parts = path.components();
    parts
        .next()
        .is_some_and(|part| part.as_os_str() == ".atelier")
        && parts.next().is_some_and(|part| {
            ["issues", "evidence", "reviews"]
                .iter()
                .any(|dir| part.as_os_str() == *dir)
        })
        && path.extension().is_some_and(|extension| extension == "md")
}
