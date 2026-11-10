//! Reading, writing and validating Atelier's records: items, their reviews
//! and `config.toml` (docs/strategies/redesign-architecture.md, Records).

mod body;
mod document;
mod ids;
mod store;
mod validate;

pub use ids::{allocate_id, validate_id};
pub use store::{
    parse_config, parse_item, parse_review, render_config, render_item, render_review, ItemDir,
    ReviewFile, Store,
};
pub use validate::validate;

use std::fmt;
use std::path::{Path, PathBuf};

/// A record that does not read or validate, with its file and the field
/// at fault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    pub file: PathBuf,
    pub field: String,
    pub message: String,
}

impl Problem {
    pub(crate) fn new(file: &Path, field: impl Into<String>, message: impl Into<String>) -> Self {
        Problem {
            file: file.to_path_buf(),
            field: field.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.file.display())?;
        if !self.field.is_empty() {
            write!(f, "{}: ", self.field)?;
        }
        f.write_str(&self.message)
    }
}

impl std::error::Error for Problem {}
