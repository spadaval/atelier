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
            const HEX: &[u8] = b"0123456789abcdef";
            Ok(digest
                .iter()
                .flat_map(|byte| {
                    [
                        HEX[(byte >> 4) as usize] as char,
                        HEX[(byte & 15) as usize] as char,
                    ]
                })
                .collect())
        }
        _ => bail!("invalid Git object ID length"),
    }
}

/// Every canonical file must match the pinned commit's path, bytes and mode.
/// Machine-local runtime, identity, config, lock and temporary files are excluded
/// by the same classifier used by the record loader. Hashes bypass Git filters.
pub fn require_committed(repo: &Path, commit: &str) -> Result<()> {
    let entries = entries(repo, commit, true)?;
    let root = repo.join(".atelier");
    let mut paths = entries.keys().cloned().collect::<BTreeSet<_>>();
    collect(&root, &root, &mut paths)?;
    for (path, (mode, _)) in &entries {
        if !matches!(mode.as_str(), "100644" | "100755") {
            bail!(
                "canonical_snapshot_uncommitted: {} is not a regular committed file",
                path.display()
            );
        }
    }
    if let Some(path) = modified(repo, paths, &entries)?.first() {
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
    let output = git_output(repo, &["ls-files", "-v", "-z"])?;
    let mut flagged = BTreeSet::new();
    for entry in output
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
    {
        if entry.len() < 3 || entry[1] != b' ' {
            bail!("invalid Git index flag entry");
        }
        if entry[0].is_ascii_lowercase() || entry[0] == b'S' {
            let path = PathBuf::from(std::str::from_utf8(&entry[2..])?);
            if include(&path) {
                flagged.insert(path);
            }
        }
    }
    if flagged.is_empty() && !cfg!(unix) {
        return Ok(Vec::new());
    }
    let entries = entries(repo, commit, false)?;
    let mut dirty = modified(repo, flagged, &entries)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for (path, (mode, _)) in &entries {
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
    if let Some(path) = flagged_modified_paths(repo, commit)?.first() {
        bail!("tracked_snapshot_uncommitted: {} differs from pinned HEAD despite Git index flags or filemode settings; restore or commit the change before continuing", path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
