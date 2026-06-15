//! Application use-case boundary.
//!
//! Command orchestration moves here as request, outcome, and view-model APIs
//! that do not write directly to stdout or stderr.

/// Minimal outcome wrapper for early app-layer extraction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Outcome<T> {
    pub value: T,
}
