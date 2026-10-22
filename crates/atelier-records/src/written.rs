//! A log of the record files this process writes, for commands that commit
//! exactly the records they wrote. Every record write goes through
//! `RecordStore`, which notes its path here once the file is in place.
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// `None` until [`start`], so processes that never ask keep nothing.
static WRITTEN: Mutex<Option<Vec<PathBuf>>> = Mutex::new(None);

/// Begins logging, and drops anything logged before.
pub fn start() {
    *WRITTEN.lock().unwrap_or_else(|e| e.into_inner()) = Some(Vec::new());
}

pub(crate) fn note(path: &Path) {
    if let Some(paths) = WRITTEN.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
        if !paths.iter().any(|known| known == path) {
            paths.push(path.to_owned());
        }
    }
}

/// The distinct paths written since [`start`], in first-write order.
pub fn take() -> Vec<PathBuf> {
    WRITTEN
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .take()
        .unwrap_or_default()
}
