//! A repository's records as one commit holds them, read from the commit's
//! tree in memory (atelier-zpmb). `atelier serve` reads each repository's
//! base branch this way, so no checkout, working tree or temporary
//! directory is involved: every object comes through the read phase's one
//! `git cat-file --batch` process.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use atelier_records::is_local_atelier_path;

use crate::git_freshness::ReadContext;
use crate::project_config::{parse_project_config, ProjectConfig};
use crate::record_index::{read_record_paths, RecordIndex};
use crate::workflow_policy::WorkflowPolicy;

/// The records, workflow policy and project config of one commit.
pub struct CommitRecords {
    /// The full ID of the commit they were read from.
    pub commit: String,
    pub index: RecordIndex,
    pub policy: WorkflowPolicy,
    /// `None` when the commit holds no `.atelier/config.toml`.
    pub config: Option<ProjectConfig>,
}

const CONFIG_PATH: &str = ".atelier/config.toml";

/// Reads the records of the commit `read` is pinned to, validated against
/// the workflow policy and link types that same commit holds, as `atelier`
/// would load them from a checkout of it. The policy is the newest valid one
/// on the commit's first-parent history, as commands read a committed policy.
pub fn load(read: &mut ReadContext<'_>) -> Result<CommitRecords> {
    let commit = read.commit_id("HEAD")?;
    let policy = crate::base_policy::from_captured(read, &commit)?.policy;
    let config = match read.entry(&commit, Path::new(CONFIG_PATH))? {
        Some(_) => Some(parse_project_config(
            &read.blob(&commit, Path::new(CONFIG_PATH))?,
            Path::new(CONFIG_PATH),
        )?),
        None => None,
    };
    let mut bytes = BTreeMap::new();
    let mut paths = Vec::new();
    let mut unreadable = Vec::new();
    for (path, mode, oid) in read.files_under(&commit, Path::new(".atelier"))? {
        let relative = path
            .strip_prefix(".atelier")
            .map(Path::to_path_buf)
            .map_err(|_| anyhow!("record path {} is outside .atelier", path.display()))?;
        if is_local_atelier_path(&relative) {
            continue;
        }
        // A checkout follows a symbolic link to its file; a commit holds only
        // the link, so it is reported rather than read as record text.
        if mode != "100644" && mode != "100755" {
            unreadable.push(relative);
        } else {
            bytes.insert(relative.clone(), read.blob_bytes(&oid)?);
            paths.push(relative);
        }
    }
    // Each file's bytes are handed to the parser once, not copied.
    let bytes = std::cell::RefCell::new(bytes);
    let (mut files, _) = read_record_paths(Path::new(".atelier"), paths, &|relative: &Path| {
        // Never `None`, which would read the file from the process's directory.
        Ok((
            true,
            Some(bytes.borrow_mut().remove(relative).unwrap_or_default()),
        ))
    })?;
    for relative in unreadable {
        files.errors.push(crate::record_index::RecordFileError {
            error: anyhow!(
                "Record file .atelier/{} is not a regular file in commit {commit}",
                relative.display()
            ),
            path: relative,
        });
    }
    files
        .errors
        .sort_by(|left, right| left.path.cmp(&right.path));
    let custom = config
        .as_ref()
        .map(|config| {
            config
                .issue_links
                .custom_context_types
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let index = RecordIndex::from_commit_files(files, &custom, &policy)?;
    Ok(CommitRecords {
        commit,
        index,
        policy,
        config,
    })
}

/// The text of a repository-relative file at the commit `read` is pinned
/// to, such as a mission's strategy.
pub fn file_at(read: &mut ReadContext<'_>, path: &str) -> Result<String> {
    read.blob("HEAD", &PathBuf::from(path))
}
