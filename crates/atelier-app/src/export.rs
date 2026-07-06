use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

use atelier_sqlite::source_freshness;
use atelier_sqlite::Database;

use crate::{Outcome, Request, ViewModel};

/// Request for the retained record-file/cache freshness diagnostic.
///
/// The former write mode rendered durable records from SQLite. Record files are
/// now the sole durable state, so callers must explicitly select check mode.
pub struct CanonicalExportRequest<'a> {
    pub db: &'a Database,
    pub state_dir: PathBuf,
    pub check: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CanonicalExportView {
    pub state_dir: PathBuf,
    pub check: bool,
    pub stale_entries: Vec<String>,
    pub wrote: bool,
}

pub fn canonical_export(
    request: Request<CanonicalExportRequest<'_>>,
) -> Result<Outcome<ViewModel<CanonicalExportView>>> {
    let input = request.input;
    if !input.check {
        bail!(
            "SQLite-to-record export has been removed; durable tracker state is already stored in .atelier record files. Use `atelier export --check` only for targeted freshness diagnostics"
        );
    }

    let stale_entries = canonical_stale_entries(input.db, &input.state_dir)?;
    if !stale_entries.is_empty() {
        bail!("Cache diagnostic is stale:\n{}", stale_entries.join("\n"));
    }

    Ok(Outcome {
        value: ViewModel {
            data: CanonicalExportView {
                state_dir: input.state_dir,
                check: true,
                stale_entries,
                wrote: false,
            },
        },
    })
}

/// Validate durable record files and their disposable cache source metadata.
pub fn canonical_stale_entries(db: &Database, state_dir: &Path) -> Result<Vec<String>> {
    let mut stale = Vec::new();

    if !state_dir.exists() {
        if has_cached_records(db)? {
            stale.push(format!("missing: {}", state_dir.display()));
        }
        return Ok(stale);
    }

    if let Err(error) = crate::rebuild::validate_canonical_state(state_dir) {
        stale.push(format!(
            "invalid: a tracker record file is invalid while running a deterministic cache diagnostic: {error:#}\nrecovery: 1. run `atelier check`; 2. fix the named record file; 3. run `atelier check --fix`; 4. rerun the blocked command"
        ));
        return Ok(stale);
    }

    let freshness = source_freshness::check(db, state_dir)?;
    stale.extend(
        freshness
            .problem_messages()
            .into_iter()
            .map(|message| format!("cache: {message}")),
    );
    stale.sort();
    Ok(stale)
}

fn has_cached_records(db: &Database) -> Result<bool> {
    if !db.list_issues(Some("all"), None, None)?.is_empty() {
        return Ok(true);
    }
    for spec in atelier_records::FIRST_CLASS_RECORD_KINDS {
        if !db.list_records(spec.kind, None)?.is_empty() {
            return Ok(true);
        }
    }
    Ok(false)
}
