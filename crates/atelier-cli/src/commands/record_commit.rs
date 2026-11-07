//! `--commit-records` and `--stage-records`: commit the records a command
//! wrote, and only those, so a step's records land in one commit instead of
//! one commit per record (atelier-4ixu).
//!
//! The records come from the record store's write log, not from `git status`,
//! so staged or unstaged changes of any other kind are never touched. A
//! command with `--stage-records` stages its records and remembers them in
//! `.atelier/runtime/staged-records.json`; the next command with
//! `--commit-records` commits those together with its own. Before a command
//! writes anything, the index must hold nothing but those remembered records.
//!
//! Consecutive `--commit-records` commands add to one commit (atelier-5822):
//! when HEAD is the commit the last one made in this checkout, nothing else
//! has seen it and no record names it, the new records amend it instead of
//! adding another record-only commit. See [`can_add_to_head`].
use anyhow::{bail, Context, Result};
use atelier_app::git_freshness::git_output;
use atelier_app::storage_layout::StorageLayout;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The flags every record-writing command that offers grouping flattens in.
#[derive(clap::Args, Clone, Copy, Debug, Default)]
pub struct RecordCommitArgs {
    /// Stage the records this command wrote and commit them with a generated
    /// subject, together with any records earlier `--stage-records` commands
    /// staged; amends the record commit the previous `--commit-records` made
    /// while no record pins it and nothing else holds it. Refuses to start
    /// when the index holds other staged changes
    #[arg(long, conflicts_with = "stage_records")]
    pub commit_records: bool,
    /// Stage the records this command wrote without committing, so that a
    /// later `--commit-records` commits several commands' records as one
    #[arg(long)]
    pub stage_records: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Stage,
    Commit,
}

/// The pending group: record paths relative to the repository root, and one
/// label per command that contributed to it.
#[derive(Default, Deserialize, Serialize)]
struct Pending {
    paths: Vec<String>,
    labels: Vec<String>,
}

/// A command's claim on committing its records; inert without a flag.
pub struct RecordCommit {
    active: Option<Active>,
}

struct Active {
    mode: Mode,
    layout: StorageLayout,
    pending: Pending,
}

/// The commit the last `--commit-records` in this checkout made, and its
/// labels, so the next one can add to it.
#[derive(Deserialize, Serialize)]
struct LastCommit {
    commit: String,
    labels: Vec<String>,
}

impl RecordCommit {
    /// Checks the index and starts logging record writes. Call before the
    /// command writes, so a refusal leaves no half-done step.
    pub fn begin(args: RecordCommitArgs) -> Result<Self> {
        let mode = match (args.commit_records, args.stage_records) {
            (true, _) => Mode::Commit,
            (_, true) => Mode::Stage,
            _ => return Ok(Self { active: None }),
        };
        let layout = StorageLayout::discover()?;
        let mut pending = read_pending(&layout)?;
        let root = layout.repo_root();
        let staged = staged_paths(root)?;
        // A group survives only while its records are still staged: a hand
        // commit, reset or branch switch ends it.
        pending.paths.retain(|path| staged.contains(path));
        if pending.paths.is_empty() {
            pending.labels.clear();
        }
        let unstaged = git_paths(root, &["diff", "--name-only", "-z", "--no-renames"])?;
        if let Some(path) = pending.paths.iter().find(|path| unstaged.contains(path)) {
            bail!(
                "record_commit_pending_modified: {path} was staged by an earlier `--stage-records` but has unstaged changes now; stage them with `git add {path}` or undo them with `git restore {path}`, then rerun. Nothing was written"
            );
        }
        let others: Vec<&String> = staged
            .iter()
            .filter(|path| !pending.paths.contains(path))
            .collect();
        if !others.is_empty() {
            bail!(
                "record_commit_index_not_clean: the index already holds staged changes that are not records staged by an earlier `--stage-records` ({}); commit or unstage them with `git restore --staged <path>`, then rerun. Nothing was written",
                others
                    .iter()
                    .take(3)
                    .map(|path| path.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        atelier_records::written::start();
        Ok(Self {
            active: Some(Active {
                mode,
                layout,
                pending,
            }),
        })
    }

    /// Stages the records written since `begin`, and commits them (with the
    /// group staged earlier) under `--commit-records`. `verb` names what the
    /// command did, such as `finding`, for the generated subject.
    pub fn finish(self, verb: &str, quiet: bool) -> Result<()> {
        let Some(Active {
            mode,
            layout,
            mut pending,
        }) = self.active
        else {
            return Ok(());
        };
        let root = layout.repo_root();
        let written = atelier_records::written::take();
        let relative = |path: &PathBuf| -> Result<String> {
            let relative = path
                .strip_prefix(root)
                .with_context(|| format!("{} is outside the repository", path.display()))?;
            Ok(relative.to_string_lossy().into_owned())
        };
        let mut ids = Vec::new();
        let mut own = Vec::new();
        for path in &written {
            let path_text = relative(path)?;
            if !pending.paths.contains(&path_text) {
                pending.paths.push(path_text.clone());
            }
            own.push(path_text);
            if let Some(stem) = path.file_stem() {
                ids.push(stem.to_string_lossy().into_owned());
            }
        }
        if !ids.is_empty() {
            pending.labels.push(format!("{verb} {}", ids.join(" ")));
        }
        if pending.paths.is_empty() {
            if !quiet {
                println!("No records were written, so there is nothing to stage or commit.");
            }
            return Ok(());
        }
        // Remember the group before touching the index, and forget it only
        // once the commit has succeeded, so an interruption or a rejecting
        // hook leaves a state the next command understands.
        write_pending(&layout, &pending)?;
        let mut add = vec!["add", "--"];
        add.extend(own.iter().map(String::as_str));
        git_output(root, &add)?;
        if mode == Mode::Stage {
            if !quiet {
                println!(
                    "Staged {} for the next --commit-records.",
                    atelier_core::count_noun(pending.paths.len(), "record")
                );
            }
            return Ok(());
        }
        let head = atelier_app::git_freshness::head_commit(root)?;
        let last = read_last(&layout)?.filter(|last| last.commit == head);
        let amend = match last {
            Some(last) if can_add_to_head(&layout, &head, &pending.paths)? => Some(last),
            _ => None,
        };
        let mut labels = amend
            .as_ref()
            .map(|last| last.labels.clone())
            .unwrap_or_default();
        labels.extend(pending.labels.iter().cloned());
        let message = commit_message(&labels);
        let mut commit = vec!["commit", "-q"];
        if amend.is_some() {
            commit.push("--amend");
        }
        commit.extend(["-m", message.as_str(), "--"]);
        commit.extend(pending.paths.iter().map(String::as_str));
        git_output(root, &commit)?;
        clear_pending(&layout)?;
        let new_head = atelier_app::git_freshness::head_commit(root)?;
        write_last(
            &layout,
            &LastCommit {
                commit: new_head.clone(),
                labels,
            },
        )?;
        if !quiet {
            let records = atelier_core::count_noun(pending.paths.len(), "record");
            let short = |commit: &str| commit.get(..8).unwrap_or(commit).to_owned();
            if amend.is_some() {
                println!(
                    "Added {records} to the previous record commit {}, now {}.",
                    short(&head),
                    short(&new_head)
                );
            } else {
                println!("Committed {records} as {}.", short(&new_head));
            }
        }
        Ok(())
    }
}

/// `Record <first label>` for one command; with several, `Record <first
/// label> and N more`, and every label on its own line below.
fn commit_message(labels: &[String]) -> String {
    let Some((first, rest)) = labels.split_first() else {
        return "Record changes".to_owned();
    };
    if rest.is_empty() {
        return format!("Record {first}");
    }
    let body = labels
        .iter()
        .map(|label| format!("- {label}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("Record {first} and {} more\n\n{body}", rest.len())
}

/// Whether records may be added to HEAD, the commit the last
/// `--commit-records` here made, by amending it. Amending replaces the commit,
/// so it is allowed only while nothing can hold the old one:
///
/// - HEAD is on a branch, and no other ref (a remote-tracking branch, another
///   branch, a tag) and no other worktree's detached HEAD contains it, so it
///   has not been pushed, fetched or built on;
/// - its Git author is the current one, so authorship stays true;
/// - it changes none of the records being committed, so every committed
///   version of a record, such as an author's proposed disposition the
///   reviewer replaces, stays in Git history (ADR 0025);
/// - no record being committed and no other uncommitted record names HEAD,
///   so no pin is left pointing at a commit that no longer exists.
///
/// Otherwise the records get a commit of their own, as before.
fn can_add_to_head(layout: &StorageLayout, head: &str, paths: &[String]) -> Result<bool> {
    let root = layout.repo_root();
    let text = |args: &[&str]| -> Result<String> {
        Ok(String::from_utf8_lossy(&git_output(root, args)?).into_owned())
    };
    let Ok(branch) = git_output(root, &["symbolic-ref", "-q", "HEAD"]) else {
        return Ok(false);
    };
    let branch = String::from_utf8_lossy(&branch).trim().to_owned();
    let refs = text(&["for-each-ref", "--contains", head, "--format=%(refname)"])?;
    if refs.lines().any(|name| name != branch) {
        return Ok(false);
    }
    let worktrees = text(&["worktree", "list", "--porcelain"])?;
    for entry in worktrees.split("\n\n") {
        let detached = entry.lines().any(|line| line == "detached");
        let other = entry.lines().find_map(|line| line.strip_prefix("HEAD "));
        if let (true, Some(other)) = (detached, other) {
            if git_output(root, &["merge-base", "--is-ancestor", head, other]).is_ok() {
                return Ok(false);
            }
        }
    }
    let ident = text(&["var", "GIT_AUTHOR_IDENT"])?;
    let current = ident.rsplitn(3, ' ').nth(2).unwrap_or_default();
    if text(&["log", "-1", "--format=%an <%ae>", head])?.trim() != current {
        return Ok(false);
    }
    let changed = git_paths(
        root,
        &[
            "diff-tree",
            "--no-commit-id",
            "--name-only",
            "-r",
            "-z",
            "--no-renames",
            head,
        ],
    )?;
    if paths.iter().any(|path| changed.contains(path)) {
        return Ok(false);
    }
    let atelier = layout
        .atelier_dir()
        .strip_prefix(root)
        .map(|dir| dir.to_string_lossy().into_owned())
        .unwrap_or_else(|_| ".atelier".to_owned());
    let mut records = paths.to_vec();
    records.extend(git_paths(
        root,
        &[
            "diff",
            "--name-only",
            "-z",
            "--no-renames",
            "HEAD",
            "--",
            &atelier,
        ],
    )?);
    records.extend(git_paths(
        root,
        &[
            "ls-files",
            "--others",
            "--exclude-standard",
            "-z",
            "--",
            &atelier,
        ],
    )?);
    for path in records {
        if let Ok(bytes) = std::fs::read(root.join(&path)) {
            if bytes
                .windows(head.len())
                .any(|window| window == head.as_bytes())
            {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

fn last_file(layout: &StorageLayout) -> PathBuf {
    layout
        .atelier_dir()
        .join("runtime")
        .join("last-record-commit.json")
}

fn read_last(layout: &StorageLayout) -> Result<Option<LastCommit>> {
    let path = last_file(layout);
    match std::fs::read_to_string(&path) {
        Ok(text) => Ok(serde_json::from_str(&text).ok()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("cannot read {}", path.display())),
    }
}

fn write_last(layout: &StorageLayout, last: &LastCommit) -> Result<()> {
    let path = last_file(layout);
    std::fs::create_dir_all(path.parent().expect("last commit file has a directory"))?;
    std::fs::write(&path, serde_json::to_string(last)?)
        .with_context(|| format!("cannot write {}", path.display()))
}

fn pending_file(layout: &StorageLayout) -> PathBuf {
    layout
        .atelier_dir()
        .join("runtime")
        .join("staged-records.json")
}

fn read_pending(layout: &StorageLayout) -> Result<Pending> {
    let path = pending_file(layout);
    match std::fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str(&text)
            .with_context(|| format!("{} is not valid; delete it", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Pending::default()),
        Err(error) => Err(error).with_context(|| format!("cannot read {}", path.display())),
    }
}

fn write_pending(layout: &StorageLayout, pending: &Pending) -> Result<()> {
    let path = pending_file(layout);
    std::fs::create_dir_all(path.parent().expect("pending file has a directory"))?;
    std::fs::write(&path, serde_json::to_string(pending)?)
        .with_context(|| format!("cannot write {}", path.display()))
}

fn clear_pending(layout: &StorageLayout) -> Result<()> {
    match std::fs::remove_file(pending_file(layout)) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

/// Every path with a staged change, relative to the repository root.
fn staged_paths(root: &Path) -> Result<Vec<String>> {
    git_paths(
        root,
        &["diff", "--cached", "--name-only", "-z", "--no-renames"],
    )
}

fn git_paths(root: &Path, args: &[&str]) -> Result<Vec<String>> {
    let output = git_output(root, args)?;
    Ok(output
        .split(|byte| *byte == 0)
        .filter(|name| !name.is_empty())
        .map(|name| String::from_utf8_lossy(name).into_owned())
        .collect())
}
