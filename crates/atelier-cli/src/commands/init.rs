use anyhow::Result;
use std::path::Path;

use atelier_sqlite::Database;

pub fn run(path: &Path, force: bool, import_beads: bool) -> Result<()> {
    let outcome = atelier_app::init::initialize(atelier_app::Request {
        input: atelier_app::init::InitRequest {
            path: path.to_path_buf(),
            force,
            import_beads,
        },
    })?;
    let view = outcome.value.data;
    for created_path in &view.created_paths {
        println!("Created {}", created_path.display());
    }
    match &view.beads_import {
        Some(atelier_app::init::BeadsImportView::Requested {
            input_path,
            state_dir,
        }) => {
            let db_path = atelier_app::storage_layout::StorageLayout::new(path).runtime_db_path();
            let db = Database::open(&db_path)?;
            crate::commands::import::run_beads_jsonl(&db, input_path, state_dir)?;
        }
        Some(atelier_app::init::BeadsImportView::Available { input_path }) => {
            println!(
                "Detected Beads migration input at {}. Re-run with `atelier init --import-beads` to import it.",
                input_path.display()
            );
        }
        None => {}
    }

    println!("Atelier initialized successfully!");
    println!("\nNext steps:");
    println!("  atelier lint                     # Verify tracker records and workflow setup");
    println!("  atelier issue create \"Task\"     # Create the first tracked issue");
    println!("  atelier man admin                # Review setup and repair guidance");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_run_fresh_init() {
        let dir = tempdir().unwrap();
        let result = run(dir.path(), false, false);
        assert!(result.is_ok());

        // Verify directories created
        assert!(dir.path().join(".atelier").exists());
        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(dir.path().join(".atelier/issues").exists());
        assert!(!dir.path().join(".atelier/missions").exists());
        assert!(!dir.path().join(".atelier/milestones").exists());
        assert!(!dir.path().join(".atelier/plans").exists());
        assert!(dir.path().join(".atelier/evidence").exists());
        assert!(dir.path().join(".atelier/workflow.yaml").exists());
        assert!(dir.path().join(".atelier/runtime/state.db").exists());
        assert!(!dir.path().join(".atelier/rules").exists());
        assert!(!dir.path().join(".atelier/rules.local").exists());
        assert!(!dir.path().join(".atelier/hook-config.json").exists());
        assert!(!dir.path().join(".claude").exists());
        assert!(!dir.path().join(".mcp.json").exists());
        assert!(!dir.path().join(".atelier/.gitignore").exists());

        let gitignore = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(!gitignore.contains("/.atelier/state.db"));
        assert!(!gitignore.contains("/.atelier/.state.db.*.rebuild-tmp*"));
        assert!(gitignore.contains("/.atelier/runtime/"));
        assert!(gitignore.contains("/.atelier/cache/"));
        assert!(!gitignore.contains("/.atelier/rules/"));
        assert!(!gitignore.contains("/.atelier/rules.local/"));
        assert!(!gitignore.contains("/.atelier/hook-config.json"));
        assert!(!gitignore.lines().any(|line| line.trim() == "/.atelier/"));

        let config = fs::read_to_string(dir.path().join(".atelier/config.toml")).unwrap();
        assert!(config.contains("state_root = \".atelier\""));
        assert!(!config.contains("compatibility_state_root"));
        assert!(!config.contains("runtime_dir"));
        assert!(!config.contains("runtime_database"));
        assert!(!config.contains("cache_dir"));
        assert!(config.contains("[review]"));
        assert!(config.contains("mode = \"room\""));

        let workflow = fs::read_to_string(dir.path().join(".atelier/workflow.yaml")).unwrap();
        assert!(workflow.contains("schema: atelier.workflow"));
        assert!(workflow.contains("schema_version: 3"));
        assert!(workflow.contains("branch_policy:"));
        assert!(workflow.contains("  task_delivery:"));
        assert!(workflow.contains("  epic_delivery:"));
        assert!(workflow.contains("  validation_delivery:"));
        assert!(workflow.contains("  spike_review:"));
        assert!(workflow.contains("applies_to:"));
    }

    #[test]
    fn test_run_preserves_existing_config_even_with_force() {
        let dir = tempdir().unwrap();
        run(dir.path(), false, false).unwrap();

        let config_path = dir.path().join(".atelier/config.toml");
        let custom_config = r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "custom"
"#;
        fs::write(&config_path, custom_config).unwrap();

        run(dir.path(), true, false).unwrap();

        assert_eq!(fs::read_to_string(config_path).unwrap(), custom_config);
    }

    #[test]
    fn test_run_already_initialized_no_force() {
        let dir = tempdir().unwrap();

        // First init
        run(dir.path(), false, false).unwrap();

        // Second init without force - should succeed but not recreate
        let result = run(dir.path(), false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_force_update() {
        let dir = tempdir().unwrap();

        // First init
        run(dir.path(), false, false).unwrap();

        // Force update
        run(dir.path(), true, false).unwrap();

        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(dir.path().join(".atelier/runtime/state.db").exists());
        assert!(!dir.path().join(".atelier/rules").exists());
        assert!(!dir.path().join(".claude").exists());
        assert!(!dir.path().join(".mcp.json").exists());
    }

    #[test]
    fn test_run_partial_init_atelier_only() {
        let dir = tempdir().unwrap();

        // Create only .atelier directory
        fs::create_dir_all(dir.path().join(".atelier")).unwrap();

        let result = run(dir.path(), false, false);
        assert!(result.is_ok());

        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(dir.path().join(".atelier/runtime/state.db").exists());
        assert!(!dir.path().join(".claude").exists());
    }

    #[test]
    fn test_run_partial_init_claude_only() {
        let dir = tempdir().unwrap();

        // Create only .claude directory
        fs::create_dir_all(dir.path().join(".claude")).unwrap();

        let result = run(dir.path(), false, false);
        assert!(result.is_ok());

        assert!(dir.path().join(".atelier").exists());
        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(dir.path().join(".atelier/runtime/state.db").exists());
    }

    #[test]
    fn test_run_database_usable() {
        let dir = tempdir().unwrap();
        run(dir.path(), false, false).unwrap();

        // Open the created database and verify it works
        let db_path = dir.path().join(".atelier/runtime/state.db");
        let db = Database::open(&db_path).unwrap();

        // Should be able to create an issue
        let id = db.create_issue("Test issue", None, "medium").unwrap();
        assert!(!id.is_empty());
    }

    #[test]
    fn test_run_recreates_missing_database() {
        let dir = tempdir().unwrap();
        run(dir.path(), false, false).unwrap();

        let db_path = dir.path().join(".atelier/runtime/state.db");
        fs::remove_file(&db_path).unwrap();

        run(dir.path(), false, false).unwrap();

        assert!(db_path.exists());
    }

    #[test]
    fn test_run_idempotent_with_force() {
        let dir = tempdir().unwrap();

        // Multiple force runs should all succeed
        for _ in 0..3 {
            let result = run(dir.path(), true, false);
            assert!(result.is_ok());
        }

        // All files should still exist
        assert!(dir.path().join(".atelier/runtime/state.db").exists());
        assert!(!dir.path().join(".claude/settings.json").exists());
        assert!(!dir.path().join(".atelier/rules").exists());
    }
}
