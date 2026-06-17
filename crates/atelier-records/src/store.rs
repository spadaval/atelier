//! Canonical tracked-file store entrypoint.
//!
//! `RecordStore` owns filesystem discovery, atomic writes, deletes, and
//! relationship mutations. Record parsing and rendering live behind the
//! per-record ownership modules.

pub use crate::RecordStore;
