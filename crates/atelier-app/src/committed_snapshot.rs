//! Verify canonical inputs, hidden file bytes and tracked executable modes.
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

fn entries(repo: &Path, commit: &str, canonical: bool) -> Result<Entries> {
    let args = if canonical {
        vec!["ls-tree", "-rz", commit, "--", ".atelier"]
    } else {
        vec!["ls-tree", "-rz", commit]
    };
    let tree = git_output(repo, &args)?;
    let mut entries = BTreeMap::new();
    for entry in tree
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
    {
        let text = std::str::from_utf8(entry)?;
        let (metadata, path) = text.split_once('\t').context("invalid Git tree entry")?;
        if canonical && is_local_atelier_path(Path::new(path).strip_prefix(".atelier")?) {
            continue;
        }
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
    let mut dirty = Vec::new();
    for path in paths {
        let Some((mode, hash)) = entries.get(&path) else {
            dirty.push(path);
            continue;
        };
        let metadata = match fs::symlink_metadata(repo.join(&path)) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                dirty.push(path);
                continue;
            }
            Err(error) => return Err(error.into()),
        };
        if metadata.is_symlink() && mode == "120000" {
            if fs::read_link(repo.join(&path))?
                .as_os_str()
                .as_encoded_bytes()
                != git_output(repo, &["cat-file", "blob", hash])?
            {
                dirty.push(path);
            }
            continue;
        }
        if !metadata.is_file() || !matches!(mode.as_str(), "100644" | "100755") {
            dirty.push(path);
            continue;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if (metadata.permissions().mode() & 0o111 != 0) != (mode == "100755") {
                dirty.push(path);
                continue;
            }
        }
        // Hash newly read bytes, including their actual length, without filters.
        if blob_hash(&fs::read(repo.join(&path))?, hash)? != *hash {
            dirty.push(path);
        }
    }
    dirty.sort();
    Ok(dirty)
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

/// Immutable entries from one full captured OID. Filesystem checks remain fresh.
pub struct CommittedSnapshot {
    entries: Entries,
}
impl CommittedSnapshot {
    pub fn new(repo: &Path, commit: &str) -> Result<Self> {
        if !matches!(commit.len(), 40 | 64) || !commit.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            bail!("snapshot requires a full captured Git object ID");
        }
        Ok(Self {
            entries: entries(repo, commit, false)?,
        })
    }
    pub fn verified_working_paths(&self, repo: &Path) -> Result<Vec<PathBuf>> {
        let mut paths = crate::git_freshness::working_status_paths(repo)?;
        verify_records(repo, &self.entries)?;
        paths.retain(|path| !canonical(path));
        paths.extend(flagged_paths(
            repo,
            &self.entries,
            |path| !canonical(path),
            true,
        )?);
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
    pub fn require_clean(&self, repo: &Path) -> Result<()> {
        let mut paths = crate::git_freshness::working_status_paths(repo)?;
        verify_records(repo, &self.entries)?;
        paths.retain(|path| !canonical(path));
        let hidden = flagged_paths(repo, &self.entries, |path| !canonical(path), true)?;
        if !paths.is_empty() {
            bail!("merge_checkout_dirty: {} has uncommitted changes: {:?}; commit or stash them before retrying", repo.display(), paths);
        }
        require_hidden_clean(hidden)
    }
}

/// Canonical inventory, bytes, types and modes are freshly compared each call.
/// Machine-local files are excluded by the record loader's shared classifier.
pub fn require_committed(repo: &Path, commit: &str) -> Result<()> {
    let entries = entries(repo, commit, true)?;
    verify_records(repo, &entries)?;
    flagged_paths(repo, &entries, |_| false, true).map(|_| ())
}
fn verify_records(repo: &Path, entries: &Entries) -> Result<()> {
    let root = repo.join(".atelier");
    let mut paths = entries
        .keys()
        .filter(|path| canonical(path))
        .cloned()
        .collect::<BTreeSet<_>>();
    collect(&root, &root, &mut paths)?;
    for (path, (mode, _)) in entries.iter().filter(|(path, _)| canonical(path)) {
        if !matches!(mode.as_str(), "100644" | "100755") {
            bail!(
                "canonical_snapshot_uncommitted: {} is not a regular committed file",
                path.display()
            );
        }
    }
    if let Some(path) = modified(repo, paths, entries)?.first() {
        bail!("canonical_snapshot_uncommitted: {} path, bytes or mode differs from pinned HEAD; commit canonical records before continuing", path.display());
    }
    Ok(())
}

/// Tracked changes hidden by index flags or core.filemode=false. Byte hashes
/// are needed only for flagged paths; Unix executable modes are always checked.
pub fn flagged_modified_paths(repo: &Path, commit: &str) -> Result<Vec<PathBuf>> {
    flagged_modified_paths_matching(repo, commit, |_| true)
}
pub(crate) fn flagged_modified_paths_matching(
    repo: &Path,
    commit: &str,
    include: impl Fn(&Path) -> bool,
) -> Result<Vec<PathBuf>> {
    flagged_paths(repo, &entries(repo, commit, false)?, include, false)
}
fn flagged_paths(
    repo: &Path,
    entries: &Entries,
    include: impl Fn(&Path) -> bool,
    verify_index: bool,
) -> Result<Vec<PathBuf>> {
    let output = git_output(repo, &["ls-files", "--stage", "-v", "-z"])?;
    let mut indexed = BTreeSet::new();
    let mut flagged = BTreeSet::new();
    for entry in output
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
    {
        if entry.len() < 3 || entry[1] != b' ' {
            bail!("invalid Git index flag entry");
        }
        let text = std::str::from_utf8(&entry[2..])?;
        let (metadata, path) = text.split_once('\t').context("invalid Git index entry")?;
        let path = PathBuf::from(path);
        if verify_index && canonical(&path) {
            let fields = metadata.split_whitespace().collect::<Vec<_>>();
            if fields.len() != 3
                || fields[2] != "0"
                || !entries
                    .get(&path)
                    .is_some_and(|(mode, hash)| fields[0] == mode && fields[1] == hash)
            {
                bail!("canonical_snapshot_uncommitted: {} index path, bytes or mode differs from pinned HEAD", path.display());
            }
            indexed.insert(path.clone());
        }
        if (entry[0].is_ascii_lowercase() || entry[0] == b'S') && include(&path) {
            flagged.insert(path);
        }
    }
    if verify_index {
        if let Some(path) = entries
            .keys()
            .find(|path| canonical(path) && !indexed.contains(*path))
        {
            bail!(
                "canonical_snapshot_uncommitted: {} is absent from the Git index",
                path.display()
            );
        }
    }
    if flagged.is_empty() && !cfg!(unix) {
        return Ok(Vec::new());
    }
    let mut dirty = modified(repo, flagged, entries)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for (path, (mode, _)) in entries {
            if !include(path) || !matches!(mode.as_str(), "100644" | "100755") {
                continue;
            }
            let metadata = match fs::symlink_metadata(repo.join(path)) {
                Ok(metadata) => metadata,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => return Err(error.into()),
            };
            if metadata.is_file()
                && (metadata.permissions().mode() & 0o111 != 0) != (mode == "100755")
            {
                dirty.push(path.clone());
            }
        }
    }
    dirty.sort();
    dirty.dedup();
    Ok(dirty)
}

pub fn require_flagged_clean(repo: &Path, commit: &str) -> Result<()> {
    require_hidden_clean(flagged_modified_paths(repo, commit)?)
}
fn require_hidden_clean(paths: Vec<PathBuf>) -> Result<()> {
    if let Some(path) = paths.first() {
        bail!("tracked_snapshot_uncommitted: {} differs from pinned HEAD despite Git index flags or filemode settings; restore or commit the change before continuing", path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reused_immutable_entries_never_reuse_filesystem_verification() {
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
        let snapshot = CommittedSnapshot::new(repo.path(), &pin).unwrap();
        assert!(snapshot
            .verified_working_paths(repo.path())
            .unwrap()
            .is_empty());
        assert!(CommittedSnapshot::new(repo.path(), "HEAD").is_err());
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
        assert!(require_committed(repo.path(), &pin).is_err());
        git_output(repo.path(), &["reset", "-q", "HEAD", "--", record]).unwrap();
        snapshot.require_clean(repo.path()).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            git_output(repo.path(), &["config", "core.filemode", "false"]).unwrap();
            for path in [record, "code"] {
                fs::set_permissions(repo.path().join(path), fs::Permissions::from_mode(0o755))
                    .unwrap();
                assert!(snapshot.require_clean(repo.path()).is_err());
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
            let snapshot = CommittedSnapshot::new(repo.path(), &pin).unwrap();
            let hash = &snapshot.entries[Path::new(path)].1;
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
            assert!(require_committed(repo.path(), &pin).is_err(), "{state}");
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
            fs::write(
                repo.path().join(".gitattributes"),
                "*.data text eol=lf filter=uppercase\n",
            )
            .unwrap();
            git_output(
                repo.path(),
                &["config", "filter.uppercase.clean", "tr a-z A-Z"],
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
            require_committed(repo.path(), "HEAD").unwrap();
            let path = ".atelier/issues/case-1 \n\t名.data";
            git_output(
                repo.path(),
                &["update-index", "--assume-unchanged", "--", path],
            )
            .unwrap();
            fs::write(repo.path().join(path), "hidden changed bytes").unwrap();
            assert!(require_committed(repo.path(), "HEAD").is_err());
            fs::write(repo.path().join(path), &data[1]).unwrap();
            require_committed(repo.path(), "HEAD").unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                git_output(repo.path(), &["config", "core.filemode", "false"]).unwrap();
                fs::set_permissions(repo.path().join(path), fs::Permissions::from_mode(0o755))
                    .unwrap();
                assert!(require_committed(repo.path(), "HEAD").is_err());
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
        let started = std::time::Instant::now();
        require_committed(repo.path(), "HEAD").unwrap();
        println!("1000 canonical files verified in {:?}", started.elapsed());
        fs::write(issues.join("issue-0999.md"), "changed last file").unwrap();
        let error = require_committed(repo.path(), "HEAD").unwrap_err();
        assert!(error.to_string().contains("issue-0999.md"), "{error}");
    }
}
