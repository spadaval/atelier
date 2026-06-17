use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use atelier_sqlite::Database;

pub struct DoctorRequest<'a> {
    pub db: &'a Database,
    pub repo_root: PathBuf,
    pub state_dir: PathBuf,
    pub db_path: PathBuf,
    pub projection_db_existed: bool,
    pub fix: bool,
    pub diagnostics_enabled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DoctorView {
    pub db_path: PathBuf,
    pub state_dir: PathBuf,
    pub fix: bool,
    pub config_ok: bool,
    pub ignore_rules_current: bool,
    pub state_dir_ok: bool,
    pub rebuild_ready: bool,
    pub projection_fresh: bool,
    pub cache_dir_status: &'static str,
    pub runtime_db_available: bool,
    pub diagnostics: &'static str,
    pub health: BTreeMap<&'static str, bool>,
}

pub fn doctor(
    request: crate::Request<DoctorRequest<'_>>,
) -> Result<crate::Outcome<crate::ViewModel<DoctorView>>> {
    let input = request.input;
    let layout = crate::storage_layout::StorageLayout::new(&input.repo_root);
    let config_path = layout.config_path();
    let cache_dir = layout.cache_dir();

    let repaired_db;
    let active_db = if input.fix {
        crate::rebuild::validate_canonical_state(&input.state_dir).with_context(|| {
            "doctor --fix refused to edit tracked `.atelier/` canonical records; \
             run `atelier lint`, fix the named canonical Markdown record, then rerun `atelier doctor --fix`"
        })?;
        crate::rebuild::refresh_projection(&input.state_dir, &input.db_path).with_context(
            || {
                format!(
                    "doctor --fix failed while repairing ignored local projection state at {}",
                    input.db_path.display()
                )
            },
        )?;
        repaired_db =
            Database::open(&input.db_path).context("Failed to reopen repaired database")?;
        &repaired_db
    } else {
        input.db
    };

    let rebuild_ready = crate::rebuild::validate_canonical_state(&input.state_dir).is_ok();
    let projection_fresh = atelier_sqlite::projection_index::check(active_db, &input.state_dir)
        .map(|report| report.is_fresh())
        .unwrap_or(false);
    let runtime_db_available = if input.fix {
        input.db_path.exists()
    } else {
        input.projection_db_existed
    };
    let state_dir_ok = input.state_dir.is_dir();
    let ignore_rules_current = runtime_gitignore_entries_present(&input.repo_root);
    let diagnostics = if input.diagnostics_enabled {
        "enabled"
    } else {
        "disabled"
    };
    let mut health = BTreeMap::new();
    health.insert("config", config_path.exists());
    health.insert("database", runtime_db_available);
    health.insert("ignore_rules", ignore_rules_current);
    health.insert("projection_fresh", projection_fresh);
    health.insert("rebuild_ready", rebuild_ready);

    Ok(crate::Outcome {
        value: crate::ViewModel {
            data: DoctorView {
                db_path: input.db_path,
                state_dir: input.state_dir,
                fix: input.fix,
                config_ok: config_path.exists(),
                ignore_rules_current,
                state_dir_ok,
                rebuild_ready,
                projection_fresh,
                cache_dir_status: optional_dir_status(&cache_dir),
                runtime_db_available,
                diagnostics,
                health,
            },
        },
    })
}

fn runtime_gitignore_entries_present(repo_root: &Path) -> bool {
    let Ok(gitignore) = std::fs::read_to_string(repo_root.join(".gitignore")) else {
        return false;
    };
    crate::init::ROOT_GITIGNORE_ENTRIES
        .iter()
        .all(|entry| gitignore.lines().any(|line| line.trim() == *entry))
}

fn optional_dir_status(path: &Path) -> &'static str {
    if path.is_dir() {
        "ok"
    } else {
        "missing (optional)"
    }
}
