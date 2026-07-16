use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

const DERIVED_TOP_LEVEL_DIRS: &[&str] = &["runtime", "cache", "locks", "diagnostics"];

pub(crate) fn canonical_tree_fingerprint(state_dir: &Path) -> Result<String> {
    let metadata = fs::symlink_metadata(state_dir)
        .with_context(|| format!("Failed to inspect {}", state_dir.display()))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        bail!(
            "Canonical state root {} must be a real directory",
            state_dir.display()
        );
    }
    let mut hasher = Sha256::new();
    hash_directory(state_dir, state_dir, &mut hasher)?;
    Ok(format!("sha256:{:x}", hasher.finalize()))
}

fn hash_directory(root: &Path, directory: &Path, hasher: &mut Sha256) -> Result<()> {
    let mut entries = fs::read_dir(directory)
        .with_context(|| format!("Failed to read {}", directory.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let relative = path
            .strip_prefix(root)
            .expect("canonical fingerprint entry remains under root");
        if relative.components().count() == 1
            && relative
                .to_str()
                .is_some_and(|name| DERIVED_TOP_LEVEL_DIRS.contains(&name))
        {
            continue;
        }
        let file_type = entry
            .file_type()
            .with_context(|| format!("Failed to inspect {}", path.display()))?;
        if file_type.is_symlink() {
            bail!(
                "Canonical bulk mutation refuses symbolic link {}",
                path.display()
            );
        }
        let relative = relative.to_string_lossy();
        if file_type.is_dir() {
            hasher.update(b"directory\0");
            hasher.update(relative.as_bytes());
            hasher.update(b"\0");
            hash_directory(root, &path, hasher)?;
        } else if file_type.is_file() {
            let bytes =
                fs::read(&path).with_context(|| format!("Failed to read {}", path.display()))?;
            hasher.update(b"file\0");
            hasher.update(relative.as_bytes());
            hasher.update(b"\0");
            hasher.update((bytes.len() as u64).to_le_bytes());
            hasher.update(&bytes);
        } else {
            bail!(
                "Canonical bulk mutation refuses special filesystem entry {}",
                path.display()
            );
        }
    }
    Ok(())
}

pub(crate) fn ensure_unchanged(state_dir: &Path, expected: &str) -> Result<()> {
    let actual = canonical_tree_fingerprint(state_dir)?;
    if actual != expected {
        bail!(
            "canonical_bulk_concurrent_drift: canonical state changed while a bulk mutation was staged; live files were not replaced"
        );
    }
    Ok(())
}

pub(crate) fn validate_directory_path(path: &Path, required: bool) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => bail!(
            "Canonical bulk mutation refuses symbolic-link directory {}",
            path.display()
        ),
        Ok(metadata) if !metadata.is_dir() => bail!(
            "Canonical bulk mutation requires a real directory at {}",
            path.display()
        ),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && !required => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            bail!(
                "Canonical bulk mutation requires directory {}",
                path.display()
            )
        }
        Err(error) => Err(error).with_context(|| format!("Failed to inspect {}", path.display())),
    }
}

pub(crate) fn test_pause_after_snapshot() -> Result<()> {
    let Some(marker) = std::env::var_os("ATELIER_TEST_BULK_PAUSE_AFTER_SNAPSHOT") else {
        return Ok(());
    };
    let marker = std::path::PathBuf::from(marker);
    fs::write(&marker, b"snapshot complete")
        .with_context(|| format!("Failed to create test pause marker {}", marker.display()))?;
    let release = marker.with_extension("release");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while !release.exists() {
        if std::time::Instant::now() >= deadline {
            bail!("Timed out waiting for bulk mutation test release marker");
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn canonical_tree_cas_detects_noncooperative_drift_without_modifying_live_bytes() {
        let dir = tempdir().unwrap();
        let state = dir.path().join(".atelier");
        fs::create_dir_all(state.join("issues")).unwrap();
        let issue = state.join("issues/atelier-test.md");
        fs::write(&issue, "before").unwrap();
        let fingerprint = canonical_tree_fingerprint(&state).unwrap();

        fs::write(&issue, "noncooperative edit").unwrap();
        let error = ensure_unchanged(&state, &fingerprint).unwrap_err();

        assert!(error
            .to_string()
            .contains("canonical_bulk_concurrent_drift"));
        assert_eq!(fs::read_to_string(issue).unwrap(), "noncooperative edit");
    }

    #[cfg(unix)]
    #[test]
    fn canonical_tree_fingerprint_rejects_symlinks_and_special_files() {
        use std::os::unix::fs::{symlink, FileTypeExt};

        let dir = tempdir().unwrap();
        let state = dir.path().join(".atelier");
        fs::create_dir_all(state.join("issues")).unwrap();
        let outside = dir.path().join("outside.md");
        fs::write(&outside, "outside").unwrap();
        symlink(&outside, state.join("issues/link.md")).unwrap();
        let error = canonical_tree_fingerprint(&state).unwrap_err();
        assert!(error.to_string().contains("symbolic link"));

        fs::remove_file(state.join("issues/link.md")).unwrap();
        let fifo = state.join("issues/fifo");
        let status = std::process::Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .unwrap();
        assert!(status.success());
        assert!(fs::symlink_metadata(&fifo).unwrap().file_type().is_fifo());
        let error = canonical_tree_fingerprint(&state).unwrap_err();
        assert!(error.to_string().contains("special filesystem entry"));
    }
}
