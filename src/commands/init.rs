use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::db::Database;

pub(crate) const PROJECT_CONFIG_TOML: &str = r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "atelier"

[paths]
state_root = ".atelier"
runtime_dir = ".atelier"
runtime_database = ".atelier/state.db"
cache_dir = ".atelier/cache"
compatibility_state_root = ".atelier-state"
"#;
const ROOT_GITIGNORE_ENTRIES: &[&str] = &[
    "/.atelier/.locks-cache/",
    "/.atelier/state.db",
    "/.atelier/state.db-shm",
    "/.atelier/state.db-wal",
    "/.atelier/agent.json",
    "/.atelier/.cache/",
    "/.atelier/runtime/",
    "/.atelier/cache/",
    "/.atelier-worktrees/",
    "/.atelier-state/cache/",
];

pub(crate) fn ensure_root_gitignore(path: &Path, force: bool) -> Result<()> {
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

pub fn run(path: &Path, force: bool) -> Result<()> {
    let layout = crate::storage_layout::StorageLayout::new(path);
    let atelier_dir = layout.atelier_dir();

    let atelier_exists = atelier_dir.exists();
    if !atelier_exists {
        fs::create_dir_all(&atelier_dir).context("Failed to create .atelier directory")?;
        println!("Created {}", atelier_dir.display());
    }

    for dir in crate::record_store::canonical_record_dirs() {
        fs::create_dir_all(atelier_dir.join(dir))
            .with_context(|| format!("Failed to create .atelier/{} directory", dir))?;
    }

    let project_config_path = layout.config_path();
    if !project_config_path.exists() {
        fs::write(&project_config_path, PROJECT_CONFIG_TOML)
            .context("Failed to write .atelier/config.toml")?;
        println!("Created {}", project_config_path.display());
    }

    let db_path = layout.runtime_db_path();
    let db_existed = db_path.exists();
    Database::open(&db_path)?;
    if !db_existed {
        println!("Created {}", db_path.display());
    }

    ensure_root_gitignore(path, force)?;

    println!("Atelier initialized successfully!");
    println!("\nNext steps:");
    println!("  atelier issue create \"Task\"     # Create an issue");
    println!("  atelier work start <issue-id>    # Start tracked work");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_run_fresh_init() {
        let dir = tempdir().unwrap();
        let result = run(dir.path(), false);
        assert!(result.is_ok());

        // Verify directories created
        assert!(dir.path().join(".atelier").exists());
        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(dir.path().join(".atelier/issues").exists());
        assert!(dir.path().join(".atelier/missions").exists());
        assert!(dir.path().join(".atelier/milestones").exists());
        assert!(dir.path().join(".atelier/plans").exists());
        assert!(dir.path().join(".atelier/evidence").exists());
        assert!(dir.path().join(".atelier/state.db").exists());
        assert!(!dir.path().join(".atelier/rules").exists());
        assert!(!dir.path().join(".atelier/rules.local").exists());
        assert!(!dir.path().join(".atelier/hook-config.json").exists());
        assert!(!dir.path().join(".claude").exists());
        assert!(!dir.path().join(".mcp.json").exists());
        assert!(!dir.path().join(".atelier/.gitignore").exists());

        let gitignore = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(gitignore.contains("/.atelier/state.db"));
        assert!(gitignore.contains("/.atelier/.locks-cache/"));
        assert!(gitignore.contains("/.atelier/cache/"));
        assert!(!gitignore.contains("/.atelier/rules/"));
        assert!(!gitignore.contains("/.atelier/rules.local/"));
        assert!(!gitignore.contains("/.atelier/hook-config.json"));
        assert!(!gitignore.lines().any(|line| line.trim() == "/.atelier/"));

        let config = fs::read_to_string(dir.path().join(".atelier/config.toml")).unwrap();
        assert!(config.contains("state_root = \".atelier\""));
        assert!(config.contains("runtime_database = \".atelier/state.db\""));
    }

    #[test]
    fn test_run_preserves_existing_config_even_with_force() {
        let dir = tempdir().unwrap();
        run(dir.path(), false).unwrap();

        let config_path = dir.path().join(".atelier/config.toml");
        let custom_config = r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "custom"
"#;
        fs::write(&config_path, custom_config).unwrap();

        run(dir.path(), true).unwrap();

        assert_eq!(fs::read_to_string(config_path).unwrap(), custom_config);
    }

    #[test]
    fn test_run_already_initialized_no_force() {
        let dir = tempdir().unwrap();

        // First init
        run(dir.path(), false).unwrap();

        // Second init without force - should succeed but not recreate
        let result = run(dir.path(), false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_force_update() {
        let dir = tempdir().unwrap();

        // First init
        run(dir.path(), false).unwrap();

        // Force update
        run(dir.path(), true).unwrap();

        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(dir.path().join(".atelier/state.db").exists());
        assert!(!dir.path().join(".atelier/rules").exists());
        assert!(!dir.path().join(".claude").exists());
        assert!(!dir.path().join(".mcp.json").exists());
    }

    #[test]
    fn test_run_partial_init_atelier_only() {
        let dir = tempdir().unwrap();

        // Create only .atelier directory
        fs::create_dir_all(dir.path().join(".atelier")).unwrap();

        let result = run(dir.path(), false);
        assert!(result.is_ok());

        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(dir.path().join(".atelier/state.db").exists());
        assert!(!dir.path().join(".claude").exists());
    }

    #[test]
    fn test_run_partial_init_claude_only() {
        let dir = tempdir().unwrap();

        // Create only .claude directory
        fs::create_dir_all(dir.path().join(".claude")).unwrap();

        let result = run(dir.path(), false);
        assert!(result.is_ok());

        assert!(dir.path().join(".atelier").exists());
        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(dir.path().join(".atelier/state.db").exists());
    }

    #[test]
    fn test_run_database_usable() {
        let dir = tempdir().unwrap();
        run(dir.path(), false).unwrap();

        // Open the created database and verify it works
        let db_path = dir.path().join(".atelier/state.db");
        let db = Database::open(&db_path).unwrap();

        // Should be able to create an issue
        let id = db.create_issue("Test issue", None, "medium").unwrap();
        assert!(!id.is_empty());
    }

    #[test]
    fn test_run_recreates_missing_database() {
        let dir = tempdir().unwrap();
        run(dir.path(), false).unwrap();

        let db_path = dir.path().join(".atelier/state.db");
        fs::remove_file(&db_path).unwrap();

        run(dir.path(), false).unwrap();

        assert!(db_path.exists());
    }

    #[test]
    fn test_run_idempotent_with_force() {
        let dir = tempdir().unwrap();

        // Multiple force runs should all succeed
        for _ in 0..3 {
            let result = run(dir.path(), true);
            assert!(result.is_ok());
        }

        // All files should still exist
        assert!(dir.path().join(".atelier/state.db").exists());
        assert!(!dir.path().join(".claude/settings.json").exists());
        assert!(!dir.path().join(".atelier/rules").exists());
    }
}
