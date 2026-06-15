use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use atelier_sqlite::Database;

use crate::storage_layout::StorageLayout;

pub const STANDARD_BEADS_IMPORT_PATH: &str = ".beads/issues.manual.jsonl";

pub const PROJECT_CONFIG_TOML: &str = r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "atelier"

[paths]
state_root = ".atelier"
runtime_dir = ".atelier/runtime"
runtime_database = ".atelier/runtime/state.db"
cache_dir = ".atelier/cache"
"#;

pub const ROOT_GITIGNORE_ENTRIES: &[&str] = &[
    "/.atelier/agent.json",
    "/.atelier/.cache/",
    "/.atelier/runtime/",
    "/.atelier/cache/",
    "/.atelier-worktrees/",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitRequest {
    pub path: PathBuf,
    pub force: bool,
    pub import_beads: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitView {
    pub created_paths: Vec<PathBuf>,
    pub beads_import: Option<BeadsImportView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BeadsImportView {
    Requested {
        input_path: PathBuf,
        state_dir: PathBuf,
    },
    Available {
        input_path: PathBuf,
    },
}

pub fn initialize(
    request: crate::Request<InitRequest>,
) -> Result<crate::Outcome<crate::ViewModel<InitView>>> {
    let input = request.input;
    let layout = StorageLayout::new(&input.path);
    let atelier_dir = layout.atelier_dir();
    let mut created_paths = Vec::new();

    if !atelier_dir.exists() {
        fs::create_dir_all(&atelier_dir).context("Failed to create .atelier directory")?;
        created_paths.push(atelier_dir.clone());
    }

    for dir in atelier_records::canonical_record_dirs() {
        fs::create_dir_all(atelier_dir.join(dir))
            .with_context(|| format!("Failed to create .atelier/{} directory", dir))?;
    }

    let project_config_path = layout.config_path();
    if !project_config_path.exists() {
        fs::write(&project_config_path, PROJECT_CONFIG_TOML)
            .context("Failed to write .atelier/config.toml")?;
        created_paths.push(project_config_path);
    }

    fs::create_dir_all(layout.target_runtime_dir())
        .context("Failed to create .atelier/runtime directory")?;
    let db_path = layout.runtime_db_path();
    let db_existed = db_path.exists();
    let _db = Database::open(&db_path)?;
    if !db_existed {
        created_paths.push(db_path);
    }

    let workflow_path = input
        .path
        .join(crate::workflow_policy::WORKFLOW_POLICY_PATH);
    if !workflow_path.exists() {
        fs::write(&workflow_path, crate::workflow_policy::STARTER_POLICY_YAML)
            .context("Failed to write .atelier/workflow.yaml")?;
        crate::workflow_policy::load(&input.path)?;
        created_paths.push(workflow_path);
    }

    ensure_root_gitignore(&input.path, input.force)?;

    let beads_import_path = input.path.join(STANDARD_BEADS_IMPORT_PATH);
    let beads_import = if input.import_beads {
        if !beads_import_path.exists() {
            anyhow::bail!(
                "Beads migration input not found at {}",
                beads_import_path.display()
            );
        }
        Some(BeadsImportView::Requested {
            input_path: beads_import_path,
            state_dir: atelier_dir,
        })
    } else if beads_import_path.exists() {
        Some(BeadsImportView::Available {
            input_path: beads_import_path,
        })
    } else {
        None
    };

    Ok(crate::Outcome {
        value: crate::ViewModel {
            data: InitView {
                created_paths,
                beads_import,
            },
        },
    })
}

pub fn ensure_root_gitignore(path: &Path, force: bool) -> Result<()> {
    let gitignore_path = path.join(".gitignore");
    let mut existing = match fs::read_to_string(&gitignore_path) {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(anyhow::Error::from(e).context("Failed to read .gitignore")),
    };
    let mut changed = force && !gitignore_path.exists();
    if !existing.is_empty() && !existing.ends_with('\n') {
        existing.push('\n');
        changed = true;
    }

    let missing = ROOT_GITIGNORE_ENTRIES
        .iter()
        .copied()
        .filter(|entry| !existing.lines().any(|line| line.trim() == *entry))
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        if !existing.is_empty() {
            existing.push('\n');
        }
        existing.push_str("# Atelier local runtime/cache\n");
        for entry in missing {
            existing.push_str(entry);
            existing.push('\n');
        }
        changed = true;
    }

    if changed {
        fs::write(&gitignore_path, existing).context("Failed to write .gitignore")?;
    }
    Ok(())
}
