//! Canonical Markdown record storage boundary.
//!
//! Record discovery, parsing, rendering, ID allocation, relationship rendering,
//! and atomic tracked-file mutation move here during extraction.

pub use atelier_core::RecordId;

/// Canonical record kind vocabulary shared by parser and application code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordKind {
    Evidence,
    Issue,
    Mission,
    Plan,
}
