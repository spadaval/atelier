use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn run(atelier_dir: &Path) -> Result<()> {
    let marker_file = atelier_dir.join("last_test_run");

    // Create or update the marker file
    fs::write(&marker_file, "").context("Failed to update test marker")?;

    println!("✓ Marked tests as run");
    println!("  Test reminder will reset on next code change.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use tempfile::tempdir;

    #[test]
    fn test_run_creates_marker_file() {
        let dir = tempdir().unwrap();
        let atelier_dir = dir.path().join(".atelier");
        std::fs::create_dir_all(&atelier_dir).unwrap();

        let result = run(&atelier_dir);
        assert!(result.is_ok());

        let marker_path = atelier_dir.join("last_test_run");
        assert!(marker_path.exists());
    }

    #[test]
    fn test_run_updates_existing_marker() {
        let dir = tempdir().unwrap();
        let atelier_dir = dir.path().join(".atelier");
        std::fs::create_dir_all(&atelier_dir).unwrap();

        let marker_path = atelier_dir.join("last_test_run");
        std::fs::write(&marker_path, "old content").unwrap();

        let result = run(&atelier_dir);
        assert!(result.is_ok());

        let content = std::fs::read_to_string(&marker_path).unwrap();
        assert_eq!(content, "");
    }

    #[test]
    fn test_run_fails_on_nonexistent_dir() {
        let dir = tempdir().unwrap();
        let nonexistent = dir.path().join("nonexistent");

        let result = run(&nonexistent);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_fails_on_readonly_dir() {
        // Skip on Windows as permissions work differently
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let dir = tempdir().unwrap();
            let atelier_dir = dir.path().join(".atelier");
            std::fs::create_dir_all(&atelier_dir).unwrap();

            // Make directory read-only
            let mut perms = std::fs::metadata(&atelier_dir).unwrap().permissions();
            perms.set_mode(0o444);
            std::fs::set_permissions(&atelier_dir, perms).unwrap();

            let marker_path = atelier_dir.join("last_test_run");
            if std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&marker_path)
                .is_ok()
            {
                let mut perms = std::fs::metadata(&atelier_dir).unwrap().permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&atelier_dir, perms).unwrap();
                std::fs::remove_file(marker_path).ok();
                return;
            }

            let result = run(&atelier_dir);
            assert!(result.is_err());

            // Restore permissions for cleanup
            let mut perms = std::fs::metadata(&atelier_dir).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&atelier_dir, perms).unwrap();
        }
    }

    #[test]
    fn test_run_idempotent() {
        let dir = tempdir().unwrap();
        let atelier_dir = dir.path().join(".atelier");
        std::fs::create_dir_all(&atelier_dir).unwrap();

        // Run multiple times
        for _ in 0..3 {
            let result = run(&atelier_dir);
            assert!(result.is_ok());
        }

        let marker_path = atelier_dir.join("last_test_run");
        assert!(marker_path.exists());
    }

    proptest! {
        #[test]
        fn prop_run_never_panics_with_valid_dir(subdir in "[a-z]{1,10}") {
            let dir = tempdir().unwrap();
            let atelier_dir = dir.path().join(&subdir);
            std::fs::create_dir_all(&atelier_dir).unwrap();

            let result = run(&atelier_dir);
            prop_assert!(result.is_ok());
        }
    }
}
