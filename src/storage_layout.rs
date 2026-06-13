use anyhow::{bail, Result};
use std::env;
use std::path::{Path, PathBuf};

pub const ATELIER_DIR: &str = ".atelier";
pub const RUNTIME_DB_FILE: &str = "state.db";
pub const TARGET_RUNTIME_DIR: &str = "runtime";
pub const CACHE_DIR: &str = "cache";
pub const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Clone)]
pub struct StorageLayout {
    repo_root: PathBuf,
}

impl StorageLayout {
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
        }
    }

    pub fn discover() -> Result<Self> {
        let repo_root = find_repo_root()?;
        Ok(Self::new(repo_root))
    }

    pub fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    pub fn atelier_dir(&self) -> PathBuf {
        self.repo_root.join(ATELIER_DIR)
    }

    pub fn config_path(&self) -> PathBuf {
        self.atelier_dir().join(CONFIG_FILE)
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.atelier_dir().join(CACHE_DIR)
    }

    pub fn target_runtime_dir(&self) -> PathBuf {
        self.atelier_dir().join(TARGET_RUNTIME_DIR)
    }

    pub fn canonical_dir(&self) -> PathBuf {
        self.atelier_dir()
    }

    pub fn runtime_db_path(&self) -> PathBuf {
        self.target_runtime_dir().join(RUNTIME_DB_FILE)
    }
}

pub fn find_atelier_dir() -> Result<PathBuf> {
    Ok(StorageLayout::discover()?.atelier_dir())
}

pub fn find_repo_root() -> Result<PathBuf> {
    let mut current = env::current_dir()?;
    loop {
        if current.join(ATELIER_DIR).is_dir() {
            return Ok(current);
        }
        if !current.pop() {
            bail!("Not an Atelier repository (or any parent). Run 'atelier init' first.");
        }
    }
}

pub fn find_canonical_dir_from_cwd() -> Result<Option<PathBuf>> {
    let mut current = env::current_dir()?;
    loop {
        let layout = StorageLayout::new(&current);
        let canonical_dir = layout.canonical_dir();
        if canonical_dir.is_dir() {
            return Ok(Some(canonical_dir));
        }
        if current.join(ATELIER_DIR).is_dir() {
            return Ok(None);
        }
        if !current.pop() {
            return Ok(None);
        }
    }
}

pub fn has_canonical_records(atelier_dir: &Path) -> bool {
    ["issues", "missions", "milestones", "plans", "evidence"]
        .iter()
        .any(|dir| atelier_dir.join(dir).is_dir())
}

pub fn is_local_atelier_path(relative_path: &Path) -> bool {
    if is_runtime_rebuild_temp_path(relative_path) {
        return true;
    }

    let Some(first) = relative_path.components().next() else {
        return false;
    };
    let first = first.as_os_str();
    first == ".cache"
        || first == "runtime"
        || first == "cache"
        || first == "rules"
        || first == "rules.local"
        || relative_path == Path::new(CONFIG_FILE)
        || relative_path == Path::new("agent.json")
        || relative_path == Path::new("hook-config.json")
        || relative_path == Path::new("hook-config.local.json")
}

fn is_runtime_rebuild_temp_path(relative_path: &Path) -> bool {
    if relative_path.components().count() != 1 {
        return false;
    }
    let Some(file_name) = relative_path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    file_name.starts_with(&format!(".{RUNTIME_DB_FILE}."))
        && (file_name.ends_with(".rebuild-tmp")
            || file_name.ends_with(".rebuild-tmp-shm")
            || file_name.ends_with(".rebuild-tmp-wal")
            || file_name.ends_with(".rebuild-tmp-journal"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn canonical_dir_is_the_atelier_tree() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".atelier/issues")).unwrap();

        let layout = StorageLayout::new(dir.path());

        assert_eq!(layout.canonical_dir(), dir.path().join(".atelier"));
    }

    #[test]
    fn rebuild_temp_database_paths_are_local_atelier_paths() {
        assert!(is_local_atelier_path(Path::new(
            "runtime/.state.db.123.456.rebuild-tmp"
        )));
        assert!(is_local_atelier_path(Path::new(
            "runtime/.state.db.123.456.rebuild-tmp-wal"
        )));
        assert!(is_local_atelier_path(Path::new(
            "runtime/.state.db.123.456.rebuild-tmp-journal"
        )));
        assert!(!is_local_atelier_path(Path::new(
            "issues/.state.db.123.456.rebuild-tmp"
        )));
        assert!(!is_local_atelier_path(Path::new("state.db")));
    }
}
