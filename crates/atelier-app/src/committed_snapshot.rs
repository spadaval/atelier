//! Verify committed records, hidden file bytes and tracked executable modes.
use crate::git_freshness::git_output;
use anyhow::{bail, Context, Result};
use atelier_records::is_local_atelier_path;
use sha1_checked::{Digest as Sha1Digest, Sha1};
use sha2::{Digest as Sha2Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

type Entries = BTreeMap<PathBuf, (String, String)>;

/// How a tracker record differs from the pinned commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordChange {
    /// Its working bytes or file mode differ.
    Modified,
    /// It is not in the commit.
    New,
    /// It is in the commit but not in the checkout.
    Deleted,
    /// Its index entry differs while its working bytes match.
    Staged,
    /// The commit holds it as something other than a regular file.
    NotRegular,
}
impl RecordChange {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Modified => "modified",
            Self::New => "new",
            Self::Deleted => "deleted",
            Self::Staged => "staged",
            Self::NotRegular => "not a regular file in HEAD",
        }
    }
}

/// The tracker records whose working bytes, index entry or file mode differ
/// from the pinned commit, every one of them. A command that names its own
/// check for this, as `land` does with `land_records_uncommitted`, replaces
/// the code and recovery step with its own and keeps `summary`.
#[derive(Debug)]
pub struct RecordsUncommitted {
    pub records: Vec<(PathBuf, RecordChange)>,
}
impl RecordsUncommitted {
    /// `2 tracker records differ from HEAD: a (modified), b (new)`, the
    /// first five, then a count.
    pub fn summary(&self) -> String {
        format!(
            "{} from HEAD: {}",
            match self.records.len() {
                1 => "1 tracker record differs".to_owned(),
                count => format!("{count} tracker records differ"),
            },
            atelier_core::first_five(
                self.records.iter().map(|(path, change)| format!(
                    "{} ({})",
                    path.display(),
                    change.as_str()
                )),
                ", "
            )
        )
    }
    /// The command that commits them; only staging them again when each one
    /// differs only in the index, so the working bytes already match HEAD and
    /// there is nothing to commit.
    pub fn commit_command(&self) -> &'static str {
        if self
            .records
            .iter()
            .all(|(_, change)| *change == RecordChange::Staged)
        {
            "git add .atelier"
        } else {
            "git add .atelier && git commit -m \"…\""
        }
    }
}

impl std::fmt::Display for RecordsUncommitted {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "canonical_snapshot_uncommitted: {}. Next: commit or restore the records, then retry",
            self.summary()
        )
    }
}

impl std::error::Error for RecordsUncommitted {}

fn records_uncommitted<T>(records: Vec<(PathBuf, RecordChange)>) -> Result<T> {
    Err(RecordsUncommitted { records }.into())
}

fn entries(repo: &Path, commit: &str) -> Result<Entries> {
    let tree = git_output(repo, &["ls-tree", "-rz", commit])?;
    let mut entries = BTreeMap::new();
    for entry in tree
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
    {
        let text = std::str::from_utf8(entry)?;
        let (metadata, path) = text.split_once('\t').context("invalid Git tree entry")?;
        let fields = metadata.split_whitespace().collect::<Vec<_>>();
        if fields.len() != 3 {
            bail!("invalid Git tree metadata");
        }
        entries.insert(PathBuf::from(path), (fields[0].into(), fields[2].into()));
    }
    Ok(entries)
}

fn collect(root: &Path, dir: &Path, paths: &mut BTreeSet<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(root)?;
        if is_local_atelier_path(relative) {
            continue;
        }
        if entry.file_type()?.is_dir() {
            collect(root, &path, paths)?;
        } else {
            paths.insert(Path::new(".atelier").join(relative));
        }
    }
    Ok(())
}

fn modified(repo: &Path, paths: BTreeSet<PathBuf>, entries: &Entries) -> Result<Vec<PathBuf>> {
    let mut changed = Vec::new();
    for path in paths {
        if matching_bytes(repo, &path, entries)?.is_none() {
            changed.push(path);
        }
    }
    Ok(changed)
}
fn matching_bytes(repo: &Path, path: &Path, entries: &Entries) -> Result<Option<Vec<u8>>> {
    let Some((mode, hash)) = entries.get(path) else {
        return Ok(None);
    };
    let metadata = match fs::symlink_metadata(repo.join(path)) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    if metadata.is_symlink() && mode == "120000" {
        let bytes = fs::read_link(repo.join(path))?
            .as_os_str()
            .as_encoded_bytes()
            .to_vec();
        return Ok((bytes == git_output(repo, &["cat-file", "blob", hash])?).then_some(bytes));
    }
    if !metadata.is_file() || !matches!(mode.as_str(), "100644" | "100755") {
        return Ok(None);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if (metadata.permissions().mode() & 0o111 != 0) != (mode == "100755") {
            return Ok(None);
        }
    }
    // Hash newly read bytes, including their actual length, without filters.
    let bytes = fs::read(repo.join(path))?;
    Ok((blob_hash(&bytes, hash)? == *hash).then_some(bytes))
}

fn blob_hash(bytes: &[u8], expected: &str) -> Result<String> {
    if !expected.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        bail!("invalid Git object ID");
    }
    let header = format!("blob {}\0", bytes.len());
    match expected.len() {
        40 => {
            let digest = Sha1::new()
                .chain_update(header.as_bytes())
                .chain_update(bytes)
                .try_finalize();
            if digest.has_collision() {
                bail!("Git blob SHA-1 collision detected");
            }
            Ok(format!("{:x}", digest.hash()))
        }
        64 => {
            let digest = Sha256::new()
                .chain_update(header.as_bytes())
                .chain_update(bytes)
                .finalize();
            Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
        }
        _ => bail!("invalid Git object ID length"),
    }
}

fn canonical(path: &Path) -> bool {
    path.strip_prefix(".atelier")
        .is_ok_and(|path| !is_local_atelier_path(path))
}

/// A full pinned Git object ID. Each fence reads its tree and working state afresh.
pub struct CommittedSnapshot {
    commit: String,
}
impl CommittedSnapshot {
    pub fn new(commit: &str) -> Result<Self> {
        if !matches!(commit.len(), 40 | 64) || !commit.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            bail!("snapshot requires a full captured Git object ID");
        }
        Ok(Self {
            commit: commit.into(),
        })
    }
    /// Verify fresh canonical bytes and index state before consuming parser errors.
    pub fn read_record_files(
        &self,
        repo: &Path,
        clean: bool,
    ) -> Result<(crate::record_index::RecordFiles, Vec<PathBuf>)> {
        let entries = entries(repo, &self.commit)?;
        let mut paths = crate::git_freshness::working_status_paths(repo)?;
        let records = record_paths(repo, &entries)?;
        let relative = records
            .into_iter()
            .map(|path| path.strip_prefix(".atelier").map(Path::to_path_buf))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let (files, changed) =
            crate::record_index::read_record_paths(&repo.join(".atelier"), relative, &|path| {
                matching_bytes(repo, &Path::new(".atelier").join(path), &entries)
                    .map(|bytes| (bytes.is_some(), bytes))
            })?;
        let changed = changed
            .into_iter()
            .map(|path| Path::new(".atelier").join(path))
            .collect::<Vec<_>>();
        require_records_match(repo, &entries, changed)?;
        paths.retain(|path| !canonical(path));
        let hidden = flagged_paths(repo, &entries, |path| !canonical(path))?;
        if clean {
            require_paths_clean(repo, &paths, hidden)?;
        } else {
            paths.extend(hidden);
        }
        paths.sort();
        paths.dedup();
        Ok((files, paths))
    }
    pub fn verified_working_paths(&self, repo: &Path) -> Result<Vec<PathBuf>> {
        let entries = entries(repo, &self.commit)?;
        verify_records(repo, &entries)?;
        self.working_paths(repo)
    }
    /// The uncommitted paths outside the tracker, including those Git index
    /// flags hide, without verifying the records: what a command reports
    /// beside records it has already found uncommitted.
    pub fn working_paths(&self, repo: &Path) -> Result<Vec<PathBuf>> {
        let entries = entries(repo, &self.commit)?;
        let mut paths = crate::git_freshness::working_status_paths(repo)?;
        paths.retain(|path| !canonical(path));
        paths.extend(flagged_paths(repo, &entries, |path| !canonical(path))?);
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
    pub fn require_clean(&self, repo: &Path) -> Result<()> {
        let entries = entries(repo, &self.commit)?;
        verify_records(repo, &entries)?;
        self.require_code_clean(repo)
    }
    /// Refuses uncommitted paths outside the tracker, without verifying the
    /// records.
    pub fn require_code_clean(&self, repo: &Path) -> Result<()> {
        let entries = entries(repo, &self.commit)?;
        let mut paths = crate::git_freshness::working_status_paths(repo)?;
        paths.retain(|path| !canonical(path));
        let hidden = flagged_paths(repo, &entries, |path| !canonical(path))?;
        require_paths_clean(repo, &paths, hidden)
    }
}

fn verify_records(repo: &Path, entries: &Entries) -> Result<()> {
    let changed = modified(repo, record_paths(repo, entries)?, entries)?;
    require_records_match(repo, entries, changed)
}
fn record_paths(repo: &Path, entries: &Entries) -> Result<BTreeSet<PathBuf>> {
    let root = repo.join(".atelier");
    let mut paths = entries
        .keys()
        .filter(|path| canonical(path))
        .cloned()
        .collect::<BTreeSet<_>>();
    collect(&root, &root, &mut paths)?;
    let irregular = entries
        .iter()
        .filter(|(path, (mode, _))| {
            canonical(path) && !matches!(mode.as_str(), "100644" | "100755")
        })
        .map(|(path, _)| (path.clone(), RecordChange::NotRegular))
        .collect::<Vec<_>>();
    if !irregular.is_empty() {
        return records_uncommitted(irregular);
    }
    Ok(paths)
}
/// Refuses records whose working bytes differ (`changed`) or whose index
/// entry differs from the pinned commit, naming every one.
fn require_records_match(repo: &Path, entries: &Entries, changed: Vec<PathBuf>) -> Result<()> {
    let mut records = changed
        .into_iter()
        .map(|path| {
            let change = if !entries.contains_key(&path) {
                RecordChange::New
            } else if fs::symlink_metadata(repo.join(&path)).is_err() {
                RecordChange::Deleted
            } else {
                RecordChange::Modified
            };
            (path, change)
        })
        .collect::<Vec<_>>();
    for path in index_mismatches(repo, entries)? {
        if !records.iter().any(|(listed, _)| *listed == path) {
            records.push((path, RecordChange::Staged));
        }
    }
    if records.is_empty() {
        return Ok(());
    }
    records.sort_by(|(left, _), (right, _)| left.cmp(right));
    records_uncommitted(records)
}
/// Records whose index entry differs from the pinned commit, or which the
/// index lacks.
fn index_mismatches(repo: &Path, entries: &Entries) -> Result<Vec<PathBuf>> {
    let output = git_output(repo, &["ls-files", "--stage", "-z"])?;
    let mut indexed = BTreeSet::new();
    let mut mismatched = Vec::new();
    for entry in output
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
    {
        let text = std::str::from_utf8(entry)?;
        let (metadata, path) = text.split_once('\t').context("invalid Git index entry")?;
        let path = Path::new(path);
        if !canonical(path) {
            continue;
        }
        if !entries.get(path).is_some_and(|(mode, hash)| {
            metadata
                .split_whitespace()
                .eq([mode.as_str(), hash.as_str(), "0"])
        }) {
            mismatched.push(path.to_path_buf());
        }
        indexed.insert(path.to_path_buf());
    }
    mismatched.extend(
        entries
            .keys()
            .filter(|path| canonical(path) && !indexed.contains(*path))
            .cloned(),
    );
    Ok(mismatched)
}
fn require_paths_clean(repo: &Path, paths: &[PathBuf], hidden: Vec<PathBuf>) -> Result<()> {
    if !paths.is_empty() {
        let mut paths = paths.to_vec();
        paths.sort();
        bail!(
            "merge_checkout_dirty: {} has {}: {}. Next: commit them with `git add -A && git commit -m \"…\"`, or set them aside with `git stash push --include-untracked`",
            repo.display(),
            atelier_core::count_noun(paths.len(), "uncommitted path"),
            atelier_core::first_five(paths.iter().map(|path| path.display().to_string()), ", ")
        );
    }
    require_hidden_clean(hidden)
}

/// Tracked changes hidden by assume-unchanged or skip-worktree index flags.
/// Unflagged paths are left to Git status, which honours core.filemode.
pub(crate) fn flagged_modified_paths(repo: &Path, commit: &str) -> Result<Vec<PathBuf>> {
    flagged_paths(repo, &entries(repo, commit)?, |_| true)
}
fn flagged_paths(
    repo: &Path,
    entries: &Entries,
    include: impl Fn(&Path) -> bool,
) -> Result<Vec<PathBuf>> {
    let output = git_output(repo, &["ls-files", "--stage", "-v", "-z"])?;
    let mut flagged = BTreeSet::new();
    for entry in output
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
    {
        if entry.len() < 3 || entry[1] != b' ' {
            bail!("invalid Git index flag entry");
        }
        let text = std::str::from_utf8(&entry[2..])?;
        let (_, path) = text.split_once('\t').context("invalid Git index entry")?;
        let path = Path::new(path);
        if (entry[0].is_ascii_lowercase() || entry[0] == b'S') && include(path) {
            flagged.insert(path.to_path_buf());
        }
    }
    // Unflagged paths follow Git's own status, including core.filemode.
    modified(repo, flagged, entries)
}

fn require_hidden_clean(paths: Vec<PathBuf>) -> Result<()> {
    if !paths.is_empty() {
        bail!(
            "tracked_snapshot_uncommitted: {} from HEAD, hidden by Git index flags: {}; restore or commit the change before continuing",
            match paths.len() {
                1 => "1 path differs".to_owned(),
                count => format!("{count} paths differ"),
            },
            atelier_core::first_five(paths.iter().map(|path| path.display().to_string()), ", ")
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_raw_reads_report_hash_errors_before_earlier_mismatches() {
        for length in [40, 64] {
            let repo = tempfile::tempdir().unwrap();
            let paths = (0..130)
                .map(|number| PathBuf::from(format!("file-{number:04}")))
                .collect::<BTreeSet<_>>();
            let hash = blob_hash(b"committed bytes", &"0".repeat(length)).unwrap();
            let mut entries = paths
                .iter()
                .map(|path| {
                    fs::write(repo.path().join(path), b"committed bytes").unwrap();
                    (path.clone(), ("100644".to_string(), hash.clone()))
                })
                .collect::<Entries>();
            fs::write(repo.path().join("file-0000"), b"earlier mismatch").unwrap();
            entries.get_mut(Path::new("file-0064")).unwrap().1 = "invalid".to_string();
            entries.get_mut(Path::new("file-0129")).unwrap().1 = "000".to_string();
            let error = modified(repo.path(), paths.clone(), &entries).unwrap_err();
            assert_eq!(error.to_string(), "invalid Git object ID");
            entries.get_mut(Path::new("file-0064")).unwrap().1 = hash.clone();
            let error = modified(repo.path(), paths.clone(), &entries).unwrap_err();
            assert_eq!(error.to_string(), "invalid Git object ID length");
            entries.get_mut(Path::new("file-0129")).unwrap().1 = hash;
            fs::write(repo.path().join("file-0065"), b"later mismatch").unwrap();
            assert_eq!(
                modified(repo.path(), paths, &entries).unwrap(),
                [PathBuf::from("file-0000"), PathBuf::from("file-0065")]
            );
        }
    }

    #[test]
    fn initial_proof_checks_union_config_types_and_index_before_parse_errors() {
        for format in ["sha1", "sha256"] {
            for mutation in [
                "unknown",
                "missing",
                "config",
                "directory",
                "symlink",
                "mode",
                "flag",
                "index",
            ] {
                let repo = tempfile::tempdir().unwrap();
                for args in [
                    vec!["init", "-q", &format!("--object-format={format}")],
                    vec!["config", "user.name", "Author"],
                    vec!["config", "user.email", "author@example.test"],
                ] {
                    git_output(repo.path(), &args).unwrap();
                }
                fs::create_dir_all(repo.path().join(".atelier/issues")).unwrap();
                let record = ".atelier/issues/atelier-good.md";
                let bytes = atelier_records::render_issue_record(
                    &crate::test_records::issue_record("atelier-good", "task", "todo"),
                )
                .unwrap();
                fs::write(repo.path().join(record), &bytes).unwrap();
                fs::write(
                    repo.path().join(".atelier/issues/atelier-bad.md"),
                    "committed invalid record",
                )
                .unwrap();
                fs::write(
                    repo.path().join(".atelier/workflow.yaml"),
                    "committed policy config",
                )
                .unwrap();
                git_output(repo.path(), &["add", "."]).unwrap();
                git_output(repo.path(), &["commit", "-qm", "initial files"]).unwrap();
                let head = crate::git_freshness::head_commit(repo.path()).unwrap();
                let snapshot = CommittedSnapshot::new(&head).unwrap();
                fs::write(
                    repo.path().join(".git/info/exclude"),
                    ".atelier/config.toml\n",
                )
                .unwrap();
                fs::write(
                    repo.path().join(".atelier/config.toml"),
                    "uncommitted local config",
                )
                .unwrap();
                let files =
                    crate::record_index::read_record_files(&repo.path().join(".atelier")).unwrap();
                assert_eq!(files.issues.len(), 1);
                assert_eq!(files.errors.len(), 1);
                let parse_error = files.into_index().unwrap_err().to_string();
                assert_eq!(
                    parse_error,
                    "Missing YAML front matter in .atelier/issues/atelier-bad.md"
                );
                let (index, paths) = snapshot.read_record_files(repo.path(), false).unwrap();
                assert!(paths.is_empty());
                assert_eq!(index.into_index().unwrap_err().to_string(), parse_error);
                if mutation == "unknown" {
                    fs::write(
                        repo.path().join("unfinished-code"),
                        "unfinished ordinary code",
                    )
                    .unwrap();
                    assert_eq!(
                        snapshot.read_record_files(repo.path(), false).unwrap().1,
                        [PathBuf::from("unfinished-code")]
                    );
                    assert!(snapshot
                        .read_record_files(repo.path(), true)
                        .unwrap_err()
                        .to_string()
                        .starts_with("merge_checkout_dirty"));
                    fs::remove_file(repo.path().join("unfinished-code")).unwrap();
                }
                match mutation {
                    "unknown" => {
                        fs::write(repo.path().join(".git/info/exclude"), ".atelier/unknown\n")
                            .unwrap();
                        fs::write(
                            repo.path().join(".atelier/unknown"),
                            "ignored unknown addition",
                        )
                        .unwrap();
                    }
                    "missing" => fs::remove_file(repo.path().join(record)).unwrap(),
                    "config" => {
                        fs::write(repo.path().join(".atelier/workflow.yaml"), "changed policy")
                            .unwrap()
                    }
                    "directory" => {
                        fs::remove_file(repo.path().join(record)).unwrap();
                        fs::create_dir(repo.path().join(record)).unwrap();
                    }
                    "symlink" => {
                        #[cfg(unix)]
                        {
                            fs::remove_file(repo.path().join(record)).unwrap();
                            std::os::unix::fs::symlink("atelier-bad.md", repo.path().join(record))
                                .unwrap();
                        }
                        #[cfg(not(unix))]
                        {
                            continue;
                        }
                    }
                    "mode" => {
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            git_output(repo.path(), &["config", "core.filemode", "false"]).unwrap();
                            fs::set_permissions(
                                repo.path().join(record),
                                fs::Permissions::from_mode(0o755),
                            )
                            .unwrap();
                        }
                        #[cfg(not(unix))]
                        {
                            continue;
                        }
                    }
                    "flag" => {
                        git_output(repo.path(), &["update-index", "--assume-unchanged", record])
                            .unwrap();
                        fs::write(repo.path().join(record), "hidden changed bytes").unwrap();
                    }
                    "index" => {
                        fs::write(repo.path().join(record), "staged changed bytes").unwrap();
                        git_output(repo.path(), &["add", record]).unwrap();
                        fs::write(repo.path().join(record), &bytes).unwrap();
                    }
                    _ => unreachable!(),
                }
                let error = snapshot.verified_working_paths(repo.path()).unwrap_err();
                assert_eq!(
                    snapshot
                        .read_record_files(repo.path(), false)
                        .unwrap_err()
                        .to_string(),
                    error.to_string()
                );
                assert!(
                    error
                        .to_string()
                        .starts_with("canonical_snapshot_uncommitted"),
                    "{format} {mutation}: {error:#}"
                );
            }
        }
    }

    #[test]
    fn postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error() {
        let repo = tempfile::tempdir().unwrap();
        for args in [
            vec!["init", "-q"],
            vec!["config", "user.name", "Author"],
            vec!["config", "user.email", "author@example.test"],
        ] {
            git_output(repo.path(), &args).unwrap();
        }
        fs::create_dir_all(repo.path().join(".atelier/issues")).unwrap();
        let record = ".atelier/issues/atelier-good.md";
        let mut issue = crate::test_records::issue_record("atelier-good", "task", "todo");
        issue
            .relationships
            .blocks
            .push(atelier_records::issue_relationship_target(
                "atelier-missing",
            ));
        let bytes = atelier_records::render_issue_record(&issue).unwrap();
        fs::write(repo.path().join(record), &bytes).unwrap();
        git_output(repo.path(), &["add", "."]).unwrap();
        git_output(repo.path(), &["commit", "-qm", "committed graph error"]).unwrap();
        let head = crate::git_freshness::head_commit(repo.path()).unwrap();
        let snapshot = CommittedSnapshot::new(&head).unwrap();
        let expected =
            crate::record_index::RecordIndex::load_structural(&repo.path().join(".atelier"))
                .unwrap_err()
                .to_string();
        assert!(expected.contains("atelier-missing"));
        let graph_error = || {
            snapshot
                .read_record_files(repo.path(), false)
                .unwrap()
                .0
                .into_index()
                .unwrap_err()
                .to_string()
        };
        assert_eq!(graph_error(), expected);
        fs::write(repo.path().join(record), "staged different bytes").unwrap();
        git_output(repo.path(), &["add", record]).unwrap();
        fs::write(repo.path().join(record), &bytes).unwrap();
        let error = snapshot.read_record_files(repo.path(), false).unwrap_err();
        assert!(error.to_string().contains(&format!("{record} (staged)")));
        git_output(repo.path(), &["reset", "-q", "HEAD", "--", record]).unwrap();
        assert_eq!(graph_error(), expected);
        fs::write(repo.path().join("unfinished"), "unfinished code").unwrap();
        assert_eq!(graph_error(), expected);
        let error = snapshot.read_record_files(repo.path(), true).unwrap_err();
        assert!(error.to_string().starts_with("merge_checkout_dirty"));
    }

    #[test]
    fn pinned_commit_fences_refresh_tree_and_filesystem_verification() {
        let repo = tempfile::tempdir().unwrap();
        for args in [
            vec!["init", "-q"],
            vec!["config", "user.name", "Author"],
            vec!["config", "user.email", "author@example.test"],
        ] {
            git_output(repo.path(), &args).unwrap();
        }
        fs::create_dir_all(repo.path().join(".atelier/issues")).unwrap();
        let record = ".atelier/issues/record.md";
        fs::write(repo.path().join(record), "committed record").unwrap();
        fs::write(repo.path().join("code"), "committed code").unwrap();
        git_output(repo.path(), &["add", "."]).unwrap();
        git_output(repo.path(), &["commit", "-qm", "original files"]).unwrap();
        let pin = crate::git_freshness::head_commit(repo.path()).unwrap();
        let snapshot = CommittedSnapshot::new(&pin).unwrap();
        assert!(snapshot
            .verified_working_paths(repo.path())
            .unwrap()
            .is_empty());
        assert!(CommittedSnapshot::new("HEAD").is_err());
        fs::write(repo.path().join(".git/info/exclude"), ".atelier/reviews/\n").unwrap();
        fs::create_dir(repo.path().join(".atelier/reviews")).unwrap();
        let ignored = repo.path().join(".atelier/reviews/hidden.md");
        fs::write(&ignored, "uncommitted ignored event").unwrap();
        assert!(snapshot.verified_working_paths(repo.path()).is_err());
        fs::remove_file(ignored).unwrap();
        git_output(
            repo.path(),
            &["update-index", "--assume-unchanged", "--", record, "code"],
        )
        .unwrap();
        fs::write(repo.path().join(record), "hidden record mutation").unwrap();
        assert!(snapshot.require_clean(repo.path()).is_err());
        fs::write(repo.path().join(record), "committed record").unwrap();
        fs::write(repo.path().join("code"), "hidden code mutation").unwrap();
        assert_eq!(
            snapshot.verified_working_paths(repo.path()).unwrap(),
            [PathBuf::from("code")]
        );
        assert!(snapshot.require_clean(repo.path()).is_err());
        fs::write(repo.path().join("code"), "committed code").unwrap();
        git_output(
            repo.path(),
            &[
                "update-index",
                "--no-assume-unchanged",
                "--",
                record,
                "code",
            ],
        )
        .unwrap();
        fs::write(repo.path().join("code"), "staged code").unwrap();
        git_output(repo.path(), &["add", "code"]).unwrap();
        fs::write(repo.path().join("code"), "committed code").unwrap();
        assert!(snapshot.require_clean(repo.path()).is_err());
        git_output(repo.path(), &["reset", "-q", "HEAD", "--", "code"]).unwrap();
        snapshot.require_clean(repo.path()).unwrap();
        fs::write(repo.path().join(record), "staged record").unwrap();
        git_output(repo.path(), &["add", record]).unwrap();
        fs::write(repo.path().join(record), "committed record").unwrap();
        assert!(snapshot.verified_working_paths(repo.path()).is_err());
        assert!(CommittedSnapshot::new(&pin)
            .unwrap()
            .verified_working_paths(repo.path())
            .is_err());
        git_output(repo.path(), &["reset", "-q", "HEAD", "--", record]).unwrap();
        snapshot.require_clean(repo.path()).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            git_output(repo.path(), &["config", "core.filemode", "false"]).unwrap();
            // Canonical records keep their committed modes; other files follow
            // Git's core.filemode setting.
            for (path, refused) in [(record, true), ("code", false)] {
                fs::set_permissions(repo.path().join(path), fs::Permissions::from_mode(0o755))
                    .unwrap();
                assert_eq!(
                    snapshot.require_clean(repo.path()).is_err(),
                    refused,
                    "{path}"
                );
                fs::set_permissions(repo.path().join(path), fs::Permissions::from_mode(0o644))
                    .unwrap();
                snapshot.require_clean(repo.path()).unwrap();
            }
        }
    }

    #[test]
    fn canonical_index_fence_checks_stages_modes_and_lossless_paths() {
        for state in ["flag", "delete", "rename", "extra", "conflict", "mode"] {
            let repo = tempfile::tempdir().unwrap();
            for args in [
                vec!["init", "-q"],
                vec!["config", "user.name", "Author"],
                vec!["config", "user.email", "author@example.test"],
            ] {
                git_output(repo.path(), &args).unwrap();
            }
            fs::create_dir_all(repo.path().join(".atelier/issues")).unwrap();
            let path = ".atelier/issues/tab\tline\n雪.md";
            fs::write(repo.path().join(path), "original").unwrap();
            git_output(repo.path(), &["add", "."]).unwrap();
            git_output(repo.path(), &["commit", "-qm", "original"]).unwrap();
            let pin = crate::git_freshness::head_commit(repo.path()).unwrap();
            let snapshot = CommittedSnapshot::new(&pin).unwrap();
            let tree = entries(repo.path(), &pin).unwrap();
            let hash = &tree[Path::new(path)].1;
            match state {
                "flag" => {
                    fs::write(repo.path().join(path), "staged").unwrap();
                    git_output(repo.path(), &["add", path]).unwrap();
                    git_output(repo.path(), &["update-index", "--assume-unchanged", path]).unwrap();
                }
                "delete" => {
                    git_output(repo.path(), &["rm", "--cached", path]).unwrap();
                }
                "rename" => {
                    git_output(repo.path(), &["mv", path, "ordinary-draft"]).unwrap();
                    fs::remove_file(repo.path().join("ordinary-draft")).unwrap();
                }
                "extra" => {
                    let extra = ".atelier/issues/extra.md";
                    fs::write(repo.path().join(extra), "extra").unwrap();
                    git_output(repo.path(), &["add", extra]).unwrap();
                    fs::remove_file(repo.path().join(extra)).unwrap();
                }
                "conflict" => {
                    git_output(repo.path(), &["rm", "--cached", path]).unwrap();
                    use std::io::Write;
                    use std::process::Stdio;
                    let mut child = crate::git_freshness::command(repo.path())
                        .args(["update-index", "-z", "--index-info"])
                        .stdin(Stdio::piped())
                        .spawn()
                        .unwrap();
                    child
                        .stdin
                        .take()
                        .unwrap()
                        .write_all(format!("100644 {hash} 1\t{path}\0").as_bytes())
                        .unwrap();
                    assert!(child.wait().unwrap().success());
                }
                "mode" => {
                    git_output(repo.path(), &["config", "core.filemode", "false"]).unwrap();
                    git_output(repo.path(), &["update-index", "--chmod=+x", path]).unwrap();
                }
                _ => unreachable!(),
            }
            fs::write(repo.path().join(path), "original").unwrap();
            assert!(
                snapshot.verified_working_paths(repo.path()).is_err(),
                "{state}"
            );
            assert!(
                CommittedSnapshot::new(&pin)
                    .unwrap()
                    .verified_working_paths(repo.path())
                    .is_err(),
                "{state}"
            );
        }
    }

    #[test]
    fn raw_blob_hashes_match_git_for_both_formats_without_filters() {
        for format in ["sha1", "sha256"] {
            let repo = tempfile::tempdir().unwrap();
            for args in [
                vec![
                    "init",
                    "-q",
                    if format == "sha1" {
                        "--object-format=sha1"
                    } else {
                        "--object-format=sha256"
                    },
                ],
                vec!["config", "user.name", "Author"],
                vec!["config", "user.email", "author@example.test"],
            ] {
                git_output(repo.path(), &args).unwrap();
            }
            let dir = repo.path().join(".atelier/issues");
            fs::create_dir_all(&dir).unwrap();
            let data = [
                Vec::new(),
                b"raw CRLF\r\nline\r\n".to_vec(),
                (0..=255u8).cycle().take(2 * 1024 * 1024).collect(),
            ];
            for (number, bytes) in data.iter().enumerate() {
                fs::write(dir.join(format!("case-{number} \n\t名.data")), bytes).unwrap();
            }
            git_output(repo.path(), &["add", "."]).unwrap();
            git_output(repo.path(), &["commit", "-qm", "raw blobs"]).unwrap();
            let pin = crate::git_freshness::head_commit(repo.path()).unwrap();
            let snapshot = CommittedSnapshot::new(&pin).unwrap();
            fs::write(
                repo.path().join(".gitattributes"),
                "*.data text eol=lf filter=uppercase\n",
            )
            .unwrap();
            let marker = repo.path().join("filter-ran");
            git_output(
                repo.path(),
                &[
                    "config",
                    "filter.uppercase.clean",
                    &format!("touch {}; tr a-z A-Z", marker.display()),
                ],
            )
            .unwrap();
            git_output(repo.path(), &["config", "core.autocrlf", "true"]).unwrap();
            for (number, bytes) in data.iter().enumerate() {
                let path = format!(".atelier/issues/case-{number} \n\t名.data");
                let git = String::from_utf8(
                    git_output(repo.path(), &["hash-object", "--no-filters", "--", &path]).unwrap(),
                )
                .unwrap();
                assert_eq!(blob_hash(bytes, git.trim()).unwrap(), git.trim());
            }
            // These live raw verification helpers must bypass clean filters.
            // Native Git status, exercised below, may independently run them.
            verify_records(repo.path(), &entries(repo.path(), &pin).unwrap()).unwrap();
            flagged_paths(repo.path(), &entries(repo.path(), &pin).unwrap(), |_| false).unwrap();
            assert!(
                index_mismatches(repo.path(), &entries(repo.path(), &pin).unwrap())
                    .unwrap()
                    .is_empty()
            );
            assert!(!marker.exists());
            snapshot.verified_working_paths(repo.path()).unwrap();
            let path = ".atelier/issues/case-1 \n\t名.data";
            git_output(
                repo.path(),
                &["update-index", "--assume-unchanged", "--", path],
            )
            .unwrap();
            fs::write(repo.path().join(path), "hidden changed bytes").unwrap();
            assert!(snapshot.verified_working_paths(repo.path()).is_err());
            fs::write(repo.path().join(path), &data[1]).unwrap();
            snapshot.verified_working_paths(repo.path()).unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                git_output(repo.path(), &["config", "core.filemode", "false"]).unwrap();
                fs::set_permissions(repo.path().join(path), fs::Permissions::from_mode(0o755))
                    .unwrap();
                assert!(snapshot.verified_working_paths(repo.path()).is_err());
            }
        }
        for invalid in ["", "abcd", &"0".repeat(41), &"g".repeat(40)] {
            assert!(blob_hash(b"bytes", invalid).is_err());
        }
    }

    #[test]
    fn fresh_hashes_check_the_last_of_one_thousand_canonical_files() {
        let repo = tempfile::tempdir().unwrap();
        for args in [
            vec!["init", "-q"],
            vec!["config", "user.name", "Author"],
            vec!["config", "user.email", "author@example.test"],
        ] {
            git_output(repo.path(), &args).unwrap();
        }
        let issues = repo.path().join(".atelier/issues");
        fs::create_dir_all(&issues).unwrap();
        for number in 0..1000 {
            fs::write(
                issues.join(format!("issue-{number:04}.md")),
                format!("canonical bytes {number}\n"),
            )
            .unwrap();
        }
        git_output(repo.path(), &["add", "."]).unwrap();
        git_output(repo.path(), &["commit", "-qm", "canonical snapshot"]).unwrap();
        let pin = crate::git_freshness::head_commit(repo.path()).unwrap();
        let snapshot = CommittedSnapshot::new(&pin).unwrap();
        let started = std::time::Instant::now();
        snapshot.verified_working_paths(repo.path()).unwrap();
        println!("1000 canonical files verified in {:?}", started.elapsed());
        fs::write(issues.join("issue-0999.md"), "changed last file").unwrap();
        let error = snapshot.verified_working_paths(repo.path()).unwrap_err();
        assert!(error.to_string().contains("issue-0999.md"), "{error}");
    }
}
