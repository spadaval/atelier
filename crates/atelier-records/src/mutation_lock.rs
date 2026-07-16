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

struct HeldLock {
    file: File,
    exclusive: bool,
    depth: usize,
}

thread_local! {
    static HELD_LOCKS: RefCell<HashMap<PathBuf, HeldLock>> = RefCell::new(HashMap::new());
}

pub struct CanonicalMutationLock {
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
                    file,
                    exclusive,
                    depth: 1,
                },
            );
        });
        Ok(Self {
            key,
            _thread_bound: PhantomData,
        })
    }
}

impl Drop for CanonicalMutationLock {
    fn drop(&mut self) {
        let file = HELD_LOCKS.with(|held| {
            let mut held = held.borrow_mut();
            let current = held
                .get_mut(&self.key)
                .expect("canonical mutation lock depth remains registered");
            current.depth -= 1;
            if current.depth == 0 {
                held.remove(&self.key).map(|state| state.file)
            } else {
                None
            }
        });
        if let Some(file) = file.as_ref() {
            let _ = FileExt::unlock(file);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread::JoinHandle;
    use std::time::Duration;
    use tempfile::tempdir;

    fn waiting_lock(
        state_dir: PathBuf,
        exclusive: bool,
    ) -> (JoinHandle<()>, mpsc::Receiver<()>, mpsc::Receiver<()>) {
        let (attempted_tx, attempted_rx) = mpsc::channel();
        let (acquired_tx, acquired_rx) = mpsc::channel();
        let writer = std::thread::spawn(move || {
            attempted_tx.send(()).unwrap();
            let _writer = if exclusive {
                CanonicalMutationLock::exclusive(&state_dir).unwrap()
            } else {
                CanonicalMutationLock::shared(&state_dir).unwrap()
            };
            acquired_tx.send(()).unwrap();
        });
        (writer, attempted_rx, acquired_rx)
    }

    fn assert_blocked(acquired: &mpsc::Receiver<()>, message: &str) {
        assert!(
            acquired.recv_timeout(Duration::from_millis(100)).is_err(),
            "{message}"
        );
    }

    #[test]
    fn exclusive_transaction_allows_nested_writes_and_blocks_external_writer() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let exclusive = CanonicalMutationLock::exclusive(&state_dir).unwrap();
        let nested = CanonicalMutationLock::shared(&state_dir).unwrap();
        let (writer, attempted_rx, acquired_rx) = waiting_lock(state_dir.clone(), false);

        attempted_rx.recv_timeout(Duration::from_secs(1)).unwrap();
        assert_blocked(
            &acquired_rx,
            "external writer interleaved with exclusive postflight",
        );
        drop(nested);
        assert_blocked(
            &acquired_rx,
            "nested guard released the outer exclusive transaction",
        );
        drop(exclusive);
        acquired_rx.recv_timeout(Duration::from_secs(1)).unwrap();
        writer.join().unwrap();
    }

    #[test]
    fn inverse_drop_order_retains_os_lock_until_nested_guard_drops() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let outer = CanonicalMutationLock::exclusive(&state_dir).unwrap();
        let nested = CanonicalMutationLock::shared(&state_dir).unwrap();
        let (writer, attempted, acquired) = waiting_lock(state_dir, false);
        attempted.recv_timeout(Duration::from_secs(1)).unwrap();

        drop(outer);
        assert_blocked(
            &acquired,
            "dropping the outer guard released a lock still owned by nested guard",
        );
        drop(nested);
        acquired.recv_timeout(Duration::from_secs(1)).unwrap();
        writer.join().unwrap();
    }

    #[test]
    fn three_level_non_lifo_drops_keep_reentrant_state_locked() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let first = CanonicalMutationLock::exclusive(&state_dir).unwrap();
        let second = CanonicalMutationLock::shared(&state_dir).unwrap();
        let third = CanonicalMutationLock::shared(&state_dir).unwrap();
        let (writer, attempted, acquired) = waiting_lock(state_dir.clone(), false);
        attempted.recv_timeout(Duration::from_secs(1)).unwrap();

        drop(second);
        drop(first);
        let fourth = CanonicalMutationLock::shared(&state_dir).unwrap();
        assert_blocked(
            &acquired,
            "non-LIFO drops released the file while reentrant guards remained",
        );
        drop(third);
        assert_blocked(
            &acquired,
            "penultimate guard released the final reentrant file lock",
        );
        drop(fourth);
        acquired.recv_timeout(Duration::from_secs(1)).unwrap();
        writer.join().unwrap();
    }

    #[test]
    fn rejected_upgrade_and_unwind_leave_no_stale_reentrant_state() {
        let dir = tempdir().unwrap();
        let state_dir = dir.path().join(".atelier");
        let shared = CanonicalMutationLock::shared(&state_dir).unwrap();
        let error = CanonicalMutationLock::exclusive(&state_dir)
            .err()
            .expect("shared-to-exclusive upgrade must fail");
        assert!(error.to_string().contains("Cannot upgrade"));
        drop(shared);

        let unwind_state = state_dir.clone();
        let result = std::panic::catch_unwind(move || {
            let _outer = CanonicalMutationLock::exclusive(&unwind_state).unwrap();
            let _nested = CanonicalMutationLock::shared(&unwind_state).unwrap();
            panic!("exercise lock guard unwind cleanup");
        });
        assert!(result.is_err());

        let (writer, attempted, acquired) = waiting_lock(state_dir, true);
        attempted.recv_timeout(Duration::from_secs(1)).unwrap();
        acquired.recv_timeout(Duration::from_secs(1)).unwrap();
        writer.join().unwrap();
    }
}
