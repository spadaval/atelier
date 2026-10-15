use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};

use crate::storage_layout::StorageLayout;

pub fn project_config_toml(project_slug: &str) -> String {
    format!(
        r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "{project_slug}"

[paths]
state_root = ".atelier"

[prune]
canonical_retention_days = 7
"#
    )
}

pub const ROOT_GITIGNORE_ENTRIES: &[&str] = &[
    "/.atelier/agent.json",
    "/.atelier/runtime/",
    "/.atelier-bundle-stage-*/",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitRequest {
    pub path: PathBuf,
    pub force: bool,
    /// Overrides the slug derived from the repository directory name.
    pub project_slug: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitView {
    pub created_paths: Vec<PathBuf>,
}

pub fn initialize(input: InitRequest) -> Result<InitView> {
    let layout = StorageLayout::new(&input.path);
    let atelier_dir = layout.atelier_dir();
    let mut created_paths = Vec::new();
    let project_config_path = layout.config_path();
    let new_config = if project_config_path.exists() {
        if let Some(requested) = &input.project_slug {
            let existing = crate::project_config::ProjectConfig::load(&input.path)?.project_slug;
            if *requested != existing {
                bail!(
                    "{} already sets project_slug = \"{existing}\"; edit it there instead of passing --slug",
                    project_config_path.display()
                );
            }
        }
        None
    } else {
        let slug = match &input.project_slug {
            Some(slug) => slug.clone(),
            None => default_project_slug(&input.path)?,
        };
        atelier_records::validate_project_slug(&slug)?;
        Some(project_config_toml(&slug))
    };

    if !atelier_dir.exists() {
        fs::create_dir_all(&atelier_dir).context("Failed to create .atelier directory")?;
        created_paths.push(atelier_dir.clone());
    }

    for dir in atelier_records::canonical_record_dirs() {
        fs::create_dir_all(atelier_dir.join(dir))
            .with_context(|| format!("Failed to create .atelier/{} directory", dir))?;
    }

    if let Some(config) = new_config {
        fs::write(&project_config_path, config).context("Failed to write .atelier/config.toml")?;
        created_paths.push(project_config_path);
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

    Ok(InitView { created_paths })
}

/// The repository directory name, normalized to a valid slug.
fn default_project_slug(path: &Path) -> Result<String> {
    let path =
        fs::canonicalize(path).with_context(|| format!("failed to resolve {}", path.display()))?;
    path.file_name()
        .and_then(|name| atelier_records::derive_project_slug(&name.to_string_lossy()))
        .ok_or_else(|| {
            anyhow!(
                "cannot derive a project slug from {}; pass --slug <slug>",
                path.display()
            )
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
        existing.push_str("# Atelier local runtime state\n");
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
