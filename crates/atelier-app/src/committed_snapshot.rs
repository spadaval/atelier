//! Verify canonical inputs independently of Git's ignore and index flags.
use crate::git_freshness::git_output;
use anyhow::{bail, Context, Result};
use atelier_records::is_local_atelier_path;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

fn collect(root: &Path, dir: &Path, paths: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(root)?;
        if is_local_atelier_path(relative) {
            continue;
        }
        let kind = entry.file_type()?;
        if kind.is_dir() {
            collect(root, &path, paths)?;
        } else if kind.is_file() {
            paths.push(relative.to_owned());
        } else {
            bail!(
                "canonical_snapshot_uncommitted: {} must be a regular file or directory",
                path.display()
            );
        }
    }
    Ok(())
}

/// Every canonical file must match the pinned commit's path, bytes and mode.
/// Machine-local runtime, identity, config, lock and temporary files are excluded
/// by the same classifier used by the record loader.
pub fn require_committed(repo: &Path, commit: &str) -> Result<()> {
    let tree = git_output(repo, &["ls-tree", "-rz", commit, "--", ".atelier"])?;
    let mut expected = BTreeMap::new();
    for entry in tree
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
    {
        let text = std::str::from_utf8(entry)?;
        let (metadata, path) = text.split_once('\t').context("invalid Git tree entry")?;
        let relative = Path::new(path).strip_prefix(".atelier")?;
        if is_local_atelier_path(relative) {
            continue;
        }
        let fields = metadata.split_whitespace().collect::<Vec<_>>();
        if fields.len() != 3 || fields[1] != "blob" || !matches!(fields[0], "100644" | "100755") {
            bail!("canonical_snapshot_uncommitted: {path} is not a regular committed file");
        }
        expected.insert(
            relative.to_owned(),
            (fields[0].to_owned(), fields[2].to_owned()),
        );
    }
    let root = repo.join(".atelier");
    let mut actual = Vec::new();
    collect(&root, &root, &mut actual)?;
    for relative in actual {
        let Some((mode, hash)) = expected.remove(&relative) else {
            bail!("canonical_snapshot_uncommitted: .atelier/{} is absent from pinned HEAD; commit canonical records before continuing", relative.display());
        };
        let path = root.join(&relative);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let executable = fs::metadata(&path)?.permissions().mode() & 0o111 != 0;
            if executable != (mode == "100755") {
                bail!(
                    "canonical_snapshot_uncommitted: .atelier/{} mode differs from pinned HEAD",
                    relative.display()
                );
            }
        }
        #[cfg(not(unix))]
        let _ = mode;
        if fs::read(&path)? != git_output(repo, &["cat-file", "blob", &hash])? {
            bail!(
                "canonical_snapshot_uncommitted: .atelier/{} bytes differ from pinned HEAD",
                relative.display()
            );
        }
    }
    if let Some(relative) = expected.keys().next() {
        bail!(
            "canonical_snapshot_uncommitted: .atelier/{} is missing from the checkout",
            relative.display()
        );
    }
    Ok(())
}
