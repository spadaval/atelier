//! The Desk's configuration file, and the repositories it serves.

use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

/// The configuration file, in TOML. Paths may start with `~/`; a relative
/// path is read from the file's directory.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// The name the human acts under, which views are computed for.
    pub caller: String,
    /// Directories whose direct children are served, each under its
    /// directory name.
    pub roots: Vec<PathBuf>,
    /// The addresses to listen on.
    #[serde(default = "loopback")]
    pub listen: Vec<SocketAddr>,
    /// The names a request's `Host` may give, without a port; any other is
    /// refused with 403.
    #[serde(default = "local_names")]
    pub hosts: Vec<String>,
    /// The built app's directory. Without one only `/api` is served.
    pub app: Option<PathBuf>,
}

pub const DEFAULT_PORT: u16 = 3780;

fn loopback() -> Vec<SocketAddr> {
    vec![SocketAddr::from(([127, 0, 0, 1], DEFAULT_PORT))]
}

fn local_names() -> Vec<String> {
    ["localhost", "127.0.0.1", "[::1]"]
        .map(String::from)
        .to_vec()
}

impl Config {
    pub fn load(path: &Path) -> Result<Config> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("cannot read the configuration {}", path.display()))?;
        let mut config: Config = toml::from_str(&text)
            .with_context(|| format!("{} is not a Desk configuration", path.display()))?;
        let here = path.parent().unwrap_or(Path::new("."));
        for root in &mut config.roots {
            *root = resolve(here, root);
        }
        config.app = config.app.map(|app| resolve(here, &app));
        if config.roots.is_empty() {
            bail!("{} names no root", path.display());
        }
        Ok(config)
    }

    /// Every repository served, by name, in root order and then by name. A
    /// name a root before it already gave is listed with the reason it is
    /// not served.
    pub fn repositories(&self) -> Vec<(String, PathBuf, Option<String>)> {
        let mut found: Vec<(String, PathBuf, Option<String>)> = Vec::new();
        for root in &self.roots {
            let Ok(entries) = std::fs::read_dir(root) else {
                continue;
            };
            let mut children: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
            children.sort();
            for dir in children.into_iter().filter(|dir| is_repository(dir)) {
                let name = dir.file_name().unwrap().to_string_lossy().into_owned();
                let taken = found.iter().find(|(other, _, _)| *other == name);
                let error = taken.map(|(_, first, _)| {
                    format!(
                        "{} already serves a repository named {name}",
                        first.display()
                    )
                });
                found.push((name, dir, error));
            }
        }
        found
    }

    /// The directory of the repository served as `name`, if one is.
    pub fn repository(&self, name: &str) -> Option<PathBuf> {
        if name.is_empty() || name.contains(['/', '\\']) || name == "." || name == ".." {
            return None;
        }
        let mut dirs = self.roots.iter().map(|root| root.join(name));
        dirs.find(|dir| is_repository(dir))
    }
}

fn resolve(here: &Path, path: &Path) -> PathBuf {
    match (path.strip_prefix("~"), std::env::var_os("HOME")) {
        (Ok(rest), Some(home)) => PathBuf::from(home).join(rest),
        _ => here.join(path),
    }
}

/// Whether `dir` is the root of a Git checkout whose `HEAD` holds an
/// Atelier configuration. A checkout's files are not read: the
/// configuration is looked up in the commit.
fn is_repository(dir: &Path) -> bool {
    dir.join(".git").exists()
        && atelier_app::git::output(dir, &["cat-file", "-e", "HEAD:.atelier/config.toml"], None)
            .is_ok_and(|output| output.status.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_configuration_listens_on_loopback_unless_told_otherwise() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("desk.toml");
        std::fs::write(
            &path,
            "caller = \"sumeet\"\nroots = [\"repos\", \"/srv\"]\n",
        )
        .unwrap();
        let config = Config::load(&path).unwrap();
        assert_eq!(
            config.listen,
            [SocketAddr::from(([127, 0, 0, 1], DEFAULT_PORT))]
        );
        assert_eq!(config.hosts, ["localhost", "127.0.0.1", "[::1]"]);
        assert_eq!(
            config.roots,
            [dir.path().join("repos"), PathBuf::from("/srv")]
        );
        assert_eq!(config.app, None);

        let given =
            "caller = \"s\"\nroots = [\"~/r\"]\nlisten = [\"100.64.0.1:80\"]\nhosts = [\"desk\"]\n";
        std::fs::write(&path, given).unwrap();
        let config = Config::load(&path).unwrap();
        assert_eq!(config.listen, [SocketAddr::from(([100, 64, 0, 1], 80))]);
        assert_eq!(config.hosts, ["desk"]);
        let home = PathBuf::from(std::env::var_os("HOME").unwrap());
        assert_eq!(config.roots, [home.join("r")]);

        std::fs::write(&path, "caller = \"s\"\nroots = []\n").unwrap();
        assert!(Config::load(&path).is_err());
    }
}
