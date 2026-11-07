//! The check every record write passes before it lands (atelier-ilba).
//!
//! Atelier refuses an invalid record when it is written, not after it has
//! blocked every later command. A program installs one guard, as the CLI
//! does with the rules `atelier check` applies, and [`RecordStore`] hands it
//! each set of changes before writing any of them: a single write, or every
//! write of a [`RecordStore::batch`]. Nothing is written when it refuses.
//! Without an installed guard, writes are not checked.
//!
//! [`RecordStore`]: crate::RecordStore
//! [`RecordStore::batch`]: crate::RecordStore::batch
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// One record file as a write would leave it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    /// The path relative to the state directory, such as
    /// `issues/atelier-abcd.md`.
    pub path: PathBuf,
    /// The new text, or `None` when the file is removed.
    pub text: Option<String>,
}

/// Checks changes to the records under a state directory, all together,
/// before any of them is written.
pub type Guard = fn(state_dir: &Path, changes: &[Change]) -> Result<()>;

static GUARD: OnceLock<Guard> = OnceLock::new();

/// Installs the guard for this process. The first one installed stays.
pub fn install(guard: Guard) {
    let _ = GUARD.set(guard);
}

pub(crate) fn check(state_dir: &Path, changes: &[Change]) -> Result<()> {
    GUARD
        .get()
        .map_or(Ok(()), |guard| guard(state_dir, changes))
}
