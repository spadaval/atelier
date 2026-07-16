//! Cooperative repository lock for canonical tracker mutations.
//!
//! Ordinary record/activity writers take a shared lock. Repository-wide
//! migrations take the exclusive form so their validated snapshot cannot be
//! interleaved with another Atelier mutation.

use anyhow::{bail, Context, Result};
use fs2::FileExt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub const CANONICAL_MUTATION_LOCK_PATH: &str = "runtime/.canonical-mutation.lock";

#[derive(Clone, Copy)]
struct HeldLock {
    exclusive: bool,
    depth: usize,
}

thread_local! {
    static HELD_LOCKS: RefCell<HashMap<PathBuf, HeldLock>> = RefCell::new(HashMap::new());
}

pub struct CanonicalMutationLock {
    file: Option<File>,
    key: PathBuf,
    // Reentrancy bookkeeping is thread-local, so a guard must be dropped on
    // the thread that acquired it.
    _thread_bound: PhantomData<Rc<()>>,
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
        let key = fs::canonicalize(parent)
            .with_context(|| format!("Failed to resolve {}", parent.display()))?
            .join(
                path.file_name()
                    .expect("canonical mutation lock has a file name"),
            );
        let nested = HELD_LOCKS.with(|held| -> Result<bool> {
            let mut held = held.borrow_mut();
            let Some(current) = held.get_mut(&key) else {
                return Ok(false);
            };
            if exclusive && !current.exclusive {
                bail!(
                    "Cannot upgrade a shared canonical mutation transaction to exclusive in the same thread"
                );
            }
            current.depth += 1;
            Ok(true)
        })?;
        if nested {
            return Ok(Self {
                file: None,
                key,
                _thread_bound: PhantomData,
            });
        }
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
        HELD_LOCKS.with(|held| {
            held.borrow_mut().insert(
                key.clone(),
                HeldLock {
                    exclusive,
                    depth: 1,
                },
            );
        });
        Ok(Self {
            file: Some(file),
            key,
            _thread_bound: PhantomData,
        })
    }
}

impl Drop for CanonicalMutationLock {
    fn drop(&mut self) {
        HELD_LOCKS.with(|held| {
            let mut held = held.borrow_mut();
            let current = held
                .get_mut(&self.key)
                .expect("canonical mutation lock depth remains registered");
            current.depth -= 1;
            if current.depth == 0 {
                held.remove(&self.key);
            }
        });
        if let Some(file) = self.file.as_ref() {
            let _ = FileExt::unlock(file);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;
    use tempfile::tempdir;

    #[test]
    fn exclusive_transaction_allows_nested_writes_and_blocks_external_writer() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let exclusive = CanonicalMutationLock::exclusive(&state_dir).unwrap();
        let nested = CanonicalMutationLock::shared(&state_dir).unwrap();
        let (attempted_tx, attempted_rx) = mpsc::channel();
        let (acquired_tx, acquired_rx) = mpsc::channel();
        let writer_state = state_dir.clone();
        let writer = std::thread::spawn(move || {
            attempted_tx.send(()).unwrap();
            let _writer = CanonicalMutationLock::shared(&writer_state).unwrap();
            acquired_tx.send(()).unwrap();
        });

        attempted_rx.recv_timeout(Duration::from_secs(1)).unwrap();
        assert!(
            acquired_rx
                .recv_timeout(Duration::from_millis(100))
                .is_err(),
            "external writer interleaved with exclusive postflight"
        );
        drop(nested);
        assert!(
            acquired_rx
                .recv_timeout(Duration::from_millis(100))
                .is_err(),
            "nested guard released the outer exclusive transaction"
        );
        drop(exclusive);
        acquired_rx.recv_timeout(Duration::from_secs(1)).unwrap();
        writer.join().unwrap();
    }

    #[test]
    fn shared_transaction_cannot_upgrade_in_same_thread() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let _shared = CanonicalMutationLock::shared(&state_dir).unwrap();
        let error = CanonicalMutationLock::exclusive(&state_dir)
            .err()
            .expect("shared-to-exclusive upgrade must fail");
        assert!(error.to_string().contains("Cannot upgrade"));
    }
}
