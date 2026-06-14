use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::db::Database;

const STANDARD_BEADS_IMPORT_PATH: &str = ".beads/issues.manual.jsonl";

pub(crate) const PROJECT_CONFIG_TOML: &str = r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "atelier"

[paths]
state_root = ".atelier"
runtime_dir = ".atelier/runtime"
runtime_database = ".atelier/runtime/state.db"
cache_dir = ".atelier/cache"
"#;
pub(crate) const ROOT_GITIGNORE_ENTRIES: &[&str] = &[
    "/.atelier/agent.json",
    "/.atelier/.cache/",
    "/.atelier/runtime/",
    "/.atelier/cache/",
    "/.atelier-worktrees/",
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

pub fn run(path: &Path, force: bool, import_beads: bool) -> Result<()> {
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

    fs::create_dir_all(layout.target_runtime_dir())
        .context("Failed to create .atelier/runtime directory")?;
    let db_path = layout.runtime_db_path();
    let db_existed = db_path.exists();
    let db = Database::open(&db_path)?;
    if !db_existed {
        println!("Created {}", db_path.display());
    }

    let workflow_path = path.join(crate::workflow_policy::WORKFLOW_POLICY_PATH);
    if !workflow_path.exists() {
        fs::write(&workflow_path, crate::workflow_policy::STARTER_POLICY_YAML)
            .context("Failed to write .atelier/workflow.yaml")?;
        crate::workflow_policy::load(path)?;
        println!("Created {}", workflow_path.display());
    }

    ensure_root_gitignore(path, force)?;

    let beads_import_path = path.join(STANDARD_BEADS_IMPORT_PATH);
    if import_beads {
        if !beads_import_path.exists() {
            anyhow::bail!(
                "Beads migration input not found at {}",
                beads_import_path.display()
            );
        }
        crate::commands::import::run_beads_jsonl(&db, &beads_import_path, &atelier_dir)?;
    } else if beads_import_path.exists() {
        println!(
            "Detected Beads migration input at {}. Re-run with `atelier init --import-beads` to import it.",
            beads_import_path.display()
        );
    }

    println!("Atelier initialized successfully!");
    println!("\nNext steps:");
    println!("  atelier lint                     # Verify tracker records and workflow setup");
    println!("  atelier issue create \"Task\"     # Create the first tracked issue");
    println!("  atelier prime                    # Review orientation and handoff guidance");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert!(dir.path().join(".atelier/missions").exists());
        assert!(dir.path().join(".atelier/milestones").exists());
        assert!(dir.path().join(".atelier/plans").exists());
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
        assert!(gitignore.contains("/.atelier/state.db"));
        assert!(!gitignore.contains("/.atelier/.state.db.*.rebuild-tmp*"));
        assert!(gitignore.contains("/.atelier/runtime/"));
        assert!(gitignore.contains("/.atelier/cache/"));
        assert!(!gitignore.contains("/.atelier/rules/"));
        assert!(!gitignore.contains("/.atelier/rules.local/"));
        assert!(!gitignore.contains("/.atelier/hook-config.json"));
        assert!(!gitignore.lines().any(|line| line.trim() == "/.atelier/"));

        let config = fs::read_to_string(dir.path().join(".atelier/config.toml")).unwrap();
        assert!(config.contains("state_root = \".atelier\""));
        assert!(config.contains("runtime_dir = \".atelier/runtime\""));
        assert!(config.contains("runtime_database = \".atelier/runtime/state.db\""));

        let workflow = fs::read_to_string(dir.path().join(".atelier/workflow.yaml")).unwrap();
        assert!(workflow.contains("schema: atelier.workflow"));
        assert!(workflow.contains("standard_review_proof"));
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
