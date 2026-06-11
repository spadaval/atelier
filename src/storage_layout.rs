use anyhow::{bail, Result};
use std::env;
use std::path::{Path, PathBuf};

pub const ATELIER_DIR: &str = ".atelier";
pub const LEGACY_CANONICAL_DIR: &str = ".atelier-state";
pub const LEGACY_RUNTIME_DB: &str = "state.db";
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

    pub fn legacy_canonical_dir(&self) -> PathBuf {
        self.repo_root.join(LEGACY_CANONICAL_DIR)
    }

    pub fn canonical_dir(&self) -> PathBuf {
        let atelier_dir = self.atelier_dir();
        if has_canonical_records(&atelier_dir) {
            atelier_dir
        } else {
            self.legacy_canonical_dir()
        }
    }

    pub fn runtime_db_path(&self) -> PathBuf {
        // Compatibility path until the runtime relocation migration lands.
        self.atelier_dir().join(LEGACY_RUNTIME_DB)
    }

    pub fn target_runtime_db_path(&self) -> PathBuf {
        self.target_runtime_dir().join(LEGACY_RUNTIME_DB)
    }
}

pub fn find_atelier_dir() -> Result<PathBuf> {
    Ok(StorageLayout::discover()?.atelier_dir())
}

pub fn find_repo_root() -> Result<PathBuf> {
    let mut current = env::current_dir()?;
    loop {
        if current.join(ATELIER_DIR).is_dir() || current.join(LEGACY_CANONICAL_DIR).is_dir() {
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
    let Some(first) = relative_path.components().next() else {
        return false;
    };
    let first = first.as_os_str();
    first == ".locks-cache"
        || first == ".cache"
        || first == "runtime"
        || first == "cache"
        || first == "rules"
        || first == "rules.local"
        || relative_path == Path::new(CONFIG_FILE)
        || relative_path == Path::new("state.db")
        || relative_path == Path::new("state.db-shm")
        || relative_path == Path::new("state.db-wal")
        || relative_path == Path::new("agent.json")
        || relative_path == Path::new("hook-config.json")
        || relative_path == Path::new("hook-config.local.json")
}
