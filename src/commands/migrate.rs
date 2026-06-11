use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

use crate::record_store;
use crate::storage_layout::{StorageLayout, LEGACY_CANONICAL_DIR};

pub fn markdown_first(repo_root: &Path) -> Result<()> {
    let layout = StorageLayout::new(repo_root);
    let atelier_dir = layout.atelier_dir();
    let legacy_dir = layout.legacy_canonical_dir();

    if !legacy_dir.exists() {
        if crate::storage_layout::has_canonical_records(&atelier_dir) {
            println!("Already migrated: {}", atelier_dir.display());
            println!("State: {}", atelier_dir.display());
            return Ok(());
        }
        bail!(
            "No {} directory found to migrate from",
            LEGACY_CANONICAL_DIR
        );
    }

    fs::create_dir_all(&atelier_dir).context("Failed to create .atelier directory")?;
    let config_path = layout.config_path();
    if !config_path.exists() {
        fs::write(&config_path, super::init::PROJECT_CONFIG_TOML)
            .context("Failed to write .atelier/config.toml")?;
    }
    super::init::ensure_root_gitignore(repo_root, false)?;

    let mut moved = Vec::new();
    for dir in record_store::canonical_record_dirs() {
        let source = legacy_dir.join(dir);
        if !source.exists() {
            continue;
        }
        let target = atelier_dir.join(dir);
        ensure_target_empty(&target)?;
        fs::rename(&source, &target).with_context(|| {
            format!(
                "Failed to move {} to {}",
                source.display(),
                target.display()
            )
        })?;
        moved.push(dir);
    }

    remove_empty_dir(&legacy_dir)?;

    let db_path = layout.runtime_db_path();
    if db_path.exists() {
        super::rebuild::refresh_projection_preserving_runtime(&atelier_dir, &db_path)?;
    } else {
        super::rebuild::run(&atelier_dir, &db_path)?;
    }

    println!(
        "Migrated canonical records from {} to {}",
        legacy_dir.display(),
        atelier_dir.display()
    );
    if moved.is_empty() {
        println!("Moved: (none)");
    } else {
        println!("Moved: {}", moved.join(", "));
    }
    println!("State: {}", atelier_dir.display());
    println!("Database: {}", db_path.display());
    println!("\nNext Commands");
    println!("-------------");
    println!("  atelier export --check");
    println!("  atelier doctor");
    Ok(())
}

fn ensure_target_empty(target: &Path) -> Result<()> {
    if !target.exists() {
        return Ok(());
    }
    if target.read_dir()?.next().is_none() {
        fs::remove_dir(target)
            .with_context(|| format!("Failed to remove empty {}", target.display()))?;
        return Ok(());
    }
    bail!(
        "Refusing to overwrite non-empty target directory {}",
        target.display()
    );
}

fn remove_empty_dir(path: &Path) -> Result<()> {
    match fs::remove_dir(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::DirectoryNotEmpty => {
            println!(
                "Left {} in place because it still contains compatibility files",
                path.display()
            );
            Ok(())
        }
        Err(e) => {
            Err(anyhow::Error::from(e).context(format!("Failed to remove {}", path.display())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    use crate::commands::{export, rebuild};
    use crate::db::Database;
    use crate::storage_layout::ATELIER_DIR;

    #[test]
    fn markdown_first_moves_legacy_records_into_atelier() {
        let dir = tempdir().unwrap();
        let atelier_dir = dir.path().join(ATELIER_DIR);
        fs::create_dir_all(&atelier_dir).unwrap();
        let db_path = atelier_dir.join("state.db");
        let db = Database::open(&db_path).unwrap();
        db.create_issue("legacy issue", None, "high").unwrap();

        let legacy_dir = dir.path().join(LEGACY_CANONICAL_DIR);
        export::run_canonical(&db, &legacy_dir, false).unwrap();
        assert!(legacy_dir.join("issues").is_dir());

        markdown_first(dir.path()).unwrap();

        assert!(!legacy_dir.exists());
        assert!(atelier_dir.join("issues").is_dir());
        assert!(atelier_dir.join("config.toml").is_file());
        assert!(atelier_dir.join("state.db").is_file());
        assert!(!atelier_dir.join(".gitignore").exists());
        rebuild::validate_canonical_state(&atelier_dir).unwrap();

        let gitignore = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(gitignore.contains("/.atelier/state.db"));
        assert!(!gitignore.lines().any(|line| line.trim() == "/.atelier/"));
    }
}
