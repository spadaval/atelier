use crate::human_output::{print_footer, FooterAction};
use anyhow::Result;
use std::path::Path;

/// Quiet output is the project slug alone, new or already configured.
pub fn run(path: &Path, force: bool, project_slug: Option<String>, quiet: bool) -> Result<()> {
    let view = atelier_app::init::initialize(atelier_app::init::InitRequest {
        path: path.to_path_buf(),
        force,
        project_slug,
    })?;
    if quiet {
        let slug = match view.project_slug {
            Some((slug, _)) => slug,
            None => atelier_app::project_config::ProjectConfig::load(path)?.project_slug,
        };
        println!("{slug}");
        return Ok(());
    }
    for created_path in &view.created_paths {
        println!("Created {}", created_path.display());
    }
    for updated_path in &view.updated_paths {
        println!("Updated {}", updated_path.display());
    }

    if let Some((slug, source)) = &view.project_slug {
        println!("Project slug: {slug} (from {source}; record IDs look like {slug}-xxxx)");
    }
    if let Some((base, source)) = &view.base_branch {
        println!("Base branch: {base} (from {source})");
    }
    println!("Atelier initialized successfully!");
    let Some((base, _)) = &view.base_branch else {
        print_footer([
            FooterAction::new("Verify tracker records and workflow setup", "atelier check"),
            FooterAction::new(
                "Create the first tracked issue",
                "atelier issue create \"Task\"",
            ),
        ]);
        return Ok(());
    };
    // The install lands as a plain commit on the base: it predates the
    // gates, so `land` there exempts it from review (atelier-l457).
    print_footer(install_steps(base, view.current_branch.as_deref()));
    Ok(())
}

/// What follows writing a new policy: review it, commit the install on the
/// base, then create the first issue.
fn install_steps(base: &str, current: Option<&str>) -> [FooterAction; 3] {
    let switch = if current == Some(base) {
        String::new()
    } else {
        format!("git switch {base} && ")
    };
    [
        FooterAction::new(
            "Read the generated policy, its base branch included",
            atelier_app::workflow_policy::WORKFLOW_POLICY_PATH,
        ),
        FooterAction::new(
            format!("Commit the install on {base}; it predates the gates, so it needs no review"),
            format!("{switch}{}", atelier_app::init::INSTALL_COMMIT),
        ),
        FooterAction::new("Create the first issue", "atelier issue create \"Task\""),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::{tempdir, TempDir};

    /// A Git repository with no commits yet, on `main`.
    fn git_init(dir: &Path) {
        let status = Command::new("git")
            .current_dir(dir)
            .args(["init", "-q", "-b", "main"])
            .status()
            .unwrap();
        assert!(status.success());
    }

    fn repo() -> TempDir {
        let dir = tempdir().unwrap();
        git_init(dir.path());
        dir
    }

    /// Atelier keeps its records in Git, so init outside a repository says
    /// to run `git init` first and writes nothing (atelier-l457).
    #[test]
    fn init_outside_git_refuses_and_writes_nothing() {
        let dir = tempdir().unwrap();
        let error = run(dir.path(), false, None, false).unwrap_err().to_string();
        assert!(error.starts_with("init_no_git: "), "{error}");
        assert!(
            error.contains("run `git init`, then `atelier init`"),
            "{error}"
        );
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 0);
    }

    /// The steps init ends with commit the install on the base, switching
    /// to it first only from another branch.
    #[test]
    fn install_steps_commit_on_the_base() {
        let rendered = |current: Option<&str>| {
            install_steps("main", current)
                .map(|action| format!("{action:?}"))
                .join("\n")
        };
        let on_base = rendered(Some("main"));
        assert!(on_base.contains(".atelier/workflow.yaml"), "{on_base}");
        assert!(
            on_base.contains("command: \"git add .atelier .gitignore && git commit"),
            "{on_base}"
        );
        assert!(on_base.contains("atelier issue create"), "{on_base}");
        assert!(rendered(Some("feature")).contains("git switch main && git add"));
    }

    #[test]
    fn test_run_fresh_init() {
        let dir = repo();
        let result = run(dir.path(), false, None, false);
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
        assert!(workflow.contains("branch_policy:\n  base_branch: main\n"));
        assert!(workflow.contains("  task:"));
        assert!(!workflow.contains("epic"));
        assert!(workflow.contains("  validation:"));
        assert!(workflow.contains("  spike:"));
        assert!(workflow.contains("applies_to:"));
    }

    #[test]
    fn test_run_preserves_existing_config_even_with_force() {
        let dir = repo();
        run(dir.path(), false, None, false).unwrap();

        let config_path = dir.path().join(".atelier/config.toml");
        let custom_config = r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "custom"
"#;
        fs::write(&config_path, custom_config).unwrap();

        run(dir.path(), true, None, false).unwrap();

        assert_eq!(fs::read_to_string(config_path).unwrap(), custom_config);
    }

    #[test]
    fn init_derives_the_slug_from_the_directory_and_accepts_an_override() {
        let parent = tempdir().unwrap();
        let repo = parent.path().join("Astromech");
        fs::create_dir(&repo).unwrap();
        git_init(&repo);
        run(&repo, false, None, false).unwrap();
        let config = fs::read_to_string(repo.join(".atelier/config.toml")).unwrap();
        assert!(config.contains("project_slug = \"astromech\""), "{config}");

        let custom = parent.path().join("other");
        fs::create_dir(&custom).unwrap();
        git_init(&custom);
        assert!(run(&custom, false, Some("Bad Slug".into()), false).is_err());
        assert!(!custom.join(".atelier").exists());
        run(&custom, false, Some("droid-shop".into()), false).unwrap();
        let config = fs::read_to_string(custom.join(".atelier/config.toml")).unwrap();
        assert!(config.contains("project_slug = \"droid-shop\""), "{config}");
        assert!(run(&custom, true, Some("other".into()), false).is_err());
        run(&custom, true, Some("droid-shop".into()), false).unwrap();
    }

    #[test]
    fn test_run_already_initialized_no_force() {
        let dir = repo();

        // First init
        run(dir.path(), false, None, false).unwrap();

        // Second init without force - should succeed but not recreate
        let result = run(dir.path(), false, None, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_force_update() {
        let dir = repo();

        // First init
        run(dir.path(), false, None, false).unwrap();

        // Force update
        run(dir.path(), true, None, false).unwrap();

        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(!dir.path().join(".claude").exists());
        assert!(!dir.path().join(".mcp.json").exists());
    }

    #[test]
    fn test_run_partial_init_atelier_only() {
        let dir = repo();

        // Create only .atelier directory
        fs::create_dir_all(dir.path().join(".atelier")).unwrap();

        let result = run(dir.path(), false, None, false);
        assert!(result.is_ok());

        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(!dir.path().join(".claude").exists());
    }

    #[test]
    fn test_run_partial_init_claude_only() {
        let dir = repo();

        // Create only .claude directory
        fs::create_dir_all(dir.path().join(".claude")).unwrap();

        let result = run(dir.path(), false, None, false);
        assert!(result.is_ok());

        assert!(dir.path().join(".atelier").exists());
        assert!(dir.path().join(".atelier/config.toml").exists());
    }

    #[test]
    fn test_run_idempotent_with_force() {
        let dir = repo();

        // Multiple force runs should all succeed
        for _ in 0..3 {
            let result = run(dir.path(), true, None, false);
            assert!(result.is_ok());
        }

        assert!(dir.path().join(".atelier/config.toml").exists());
        assert!(!dir.path().join(".claude/settings.json").exists());
    }
}
