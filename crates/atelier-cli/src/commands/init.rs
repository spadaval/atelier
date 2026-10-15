use anyhow::Result;
use std::path::Path;

pub fn run(path: &Path, force: bool, project_slug: Option<String>) -> Result<()> {
    let view = atelier_app::init::initialize(atelier_app::init::InitRequest {
        path: path.to_path_buf(),
        force,
        project_slug,
    })?;
    for created_path in &view.created_paths {
        println!("Created {}", created_path.display());
    }

    if let Some((slug, source)) = &view.project_slug {
        println!("Project slug: {slug} (from {source}; record IDs look like {slug}-xxxx)");
    }
    println!("Atelier initialized successfully!");
    println!("\nNext steps:");
    println!("  atelier check                     # Verify tracker records and workflow setup");
    println!("  atelier issue create \"Task\"     # Create the first tracked issue");

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
        let result = run(dir.path(), false, None);
        assert!(result.is_ok());

        // Verify directories created
        assert!(dir.path().join(".atelier").exists());
        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(dir.path().join(".atelier/issues").exists());
        assert!(!dir.path().join(".atelier/missions").exists());
        assert!(dir.path().join(".atelier/evidence").exists());
        assert!(dir.path().join(".atelier/workflow.yaml").exists());
        assert!(!dir.path().join(".atelier/hook-config.json").exists());
        assert!(!dir.path().join(".claude").exists());
        assert!(!dir.path().join(".mcp.json").exists());
        assert!(!dir.path().join(".atelier/.gitignore").exists());

        let gitignore = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(gitignore.contains("/.atelier/runtime/"));
        assert!(!gitignore.contains("cache"));
        assert!(gitignore.contains("/.atelier-bundle-stage-*/"));
        assert!(!gitignore.contains("/.atelier/hook-config.json"));
        assert!(!gitignore.lines().any(|line| line.trim() == "/.atelier/"));

        let config = fs::read_to_string(dir.path().join(".atelier/config.toml")).unwrap();
        assert!(config.contains("state_root = \".atelier\""));
        assert!(!config.contains("compatibility_state_root"));
        assert!(!config.contains("runtime_dir"));
        assert!(!config.contains("runtime_database"));
        assert!(!config.contains("cache_dir"));
        assert!(!config.contains("[review]"));

        let workflow = fs::read_to_string(dir.path().join(".atelier/workflow.yaml")).unwrap();
        assert!(workflow.contains("schema: atelier.workflow"));
        assert!(workflow.contains("schema_version: 3"));
        assert!(workflow.contains("branch_policy:"));
        assert!(workflow.contains("  task:"));
        assert!(workflow.contains("  epic:"));
        assert!(workflow.contains("  validation:"));
        assert!(workflow.contains("  spike:"));
        assert!(workflow.contains("applies_to:"));
    }

    #[test]
    fn test_run_preserves_existing_config_even_with_force() {
        let dir = tempdir().unwrap();
        run(dir.path(), false, None).unwrap();

        let config_path = dir.path().join(".atelier/config.toml");
        let custom_config = r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "custom"
"#;
        fs::write(&config_path, custom_config).unwrap();

        run(dir.path(), true, None).unwrap();

        assert_eq!(fs::read_to_string(config_path).unwrap(), custom_config);
    }

    #[test]
    fn init_derives_the_slug_from_the_directory_and_accepts_an_override() {
        let parent = tempdir().unwrap();
        let repo = parent.path().join("Astromech");
        fs::create_dir(&repo).unwrap();
        run(&repo, false, None).unwrap();
        let config = fs::read_to_string(repo.join(".atelier/config.toml")).unwrap();
        assert!(config.contains("project_slug = \"astromech\""), "{config}");

        let custom = parent.path().join("other");
        fs::create_dir(&custom).unwrap();
        assert!(run(&custom, false, Some("Bad Slug".into())).is_err());
        assert!(!custom.join(".atelier").exists());
        run(&custom, false, Some("droid-shop".into())).unwrap();
        let config = fs::read_to_string(custom.join(".atelier/config.toml")).unwrap();
        assert!(config.contains("project_slug = \"droid-shop\""), "{config}");
        assert!(run(&custom, true, Some("other".into())).is_err());
        run(&custom, true, Some("droid-shop".into())).unwrap();
    }

    #[test]
    fn test_run_already_initialized_no_force() {
        let dir = tempdir().unwrap();

        // First init
        run(dir.path(), false, None).unwrap();

        // Second init without force - should succeed but not recreate
        let result = run(dir.path(), false, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_force_update() {
        let dir = tempdir().unwrap();

        // First init
        run(dir.path(), false, None).unwrap();

        // Force update
        run(dir.path(), true, None).unwrap();

        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(!dir.path().join(".claude").exists());
        assert!(!dir.path().join(".mcp.json").exists());
    }

    #[test]
    fn test_run_partial_init_atelier_only() {
        let dir = tempdir().unwrap();

        // Create only .atelier directory
        fs::create_dir_all(dir.path().join(".atelier")).unwrap();

        let result = run(dir.path(), false, None);
        assert!(result.is_ok());

        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(!dir.path().join(".claude").exists());
    }

    #[test]
    fn test_run_partial_init_claude_only() {
        let dir = tempdir().unwrap();

        // Create only .claude directory
        fs::create_dir_all(dir.path().join(".claude")).unwrap();

        let result = run(dir.path(), false, None);
        assert!(result.is_ok());

        assert!(dir.path().join(".atelier").exists());
        assert!(dir.path().join(".atelier/config.toml").exists());
    }

    #[test]
    fn test_run_idempotent_with_force() {
        let dir = tempdir().unwrap();

        // Multiple force runs should all succeed
        for _ in 0..3 {
            let result = run(dir.path(), true, None);
            assert!(result.is_ok());
        }

        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(!dir.path().join(".claude/settings.json").exists());
    }
}
