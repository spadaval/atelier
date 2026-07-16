//! Cooperative repository lock for canonical tracker mutations.
//!
//! Ordinary record/activity writers take a shared lock. Repository-wide
//! migrations take the exclusive form so their validated snapshot cannot be
//! interleaved with another Atelier mutation.

use anyhow::{Context, Result};
use fs2::FileExt;
use std::fs::{self, File, OpenOptions};
use std::path::Path;

pub const CANONICAL_MUTATION_LOCK_PATH: &str = "runtime/.canonical-mutation.lock";

pub struct CanonicalMutationLock {
    file: File,
}

impl CanonicalMutationLock {
    pub fn shared(state_dir: &Path) -> Result<Self> {
        Self::acquire(state_dir, false)
    }

    pub fn exclusive(state_dir: &Path) -> Result<Self> {
        Self::acquire(state_dir, true)
    }

    fn acquire(state_dir: &Path, exclusive: bool) -> Result<Self> {
        let path = state_dir.join(CANONICAL_MUTATION_LOCK_PATH);
        let parent = path.parent().expect("canonical mutation lock has a parent");
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .with_context(|| format!("Failed to open {}", path.display()))?;
        if exclusive {
            file.lock_exclusive()
                .with_context(|| format!("Failed to exclusively lock {}", path.display()))?;
        } else {
            FileExt::lock_shared(&file)
                .with_context(|| format!("Failed to share-lock {}", path.display()))?;
        }
        Ok(Self { file })
    }
}

impl Drop for CanonicalMutationLock {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.file);
    }
}
