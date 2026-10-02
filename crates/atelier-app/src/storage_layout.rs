use anyhow::{anyhow, Result};
use std::env;
use std::path::{Path, PathBuf};

pub const ATELIER_DIR: &str = ".atelier";
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

    pub fn canonical_dir(&self) -> PathBuf {
        self.atelier_dir()
    }
}

pub fn find_repo_root() -> Result<PathBuf> {
    find_canonical_dir_from_cwd()?
        .and_then(|directory| directory.parent().map(Path::to_path_buf))
        .ok_or_else(|| {
            anyhow!("Not an Atelier repository (or any parent). Run 'atelier init' first.")
        })
}

pub fn find_canonical_dir_from_cwd() -> Result<Option<PathBuf>> {
    let mut current = env::current_dir()?;
    loop {
        let canonical_dir = current.join(ATELIER_DIR);
        if canonical_dir.is_dir() {
            return Ok(Some(canonical_dir));
        }
        if !current.pop() {
            return Ok(None);
        }
    }
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
}
