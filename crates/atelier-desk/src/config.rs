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
    /// Repositories served, each under its directory name, before those of
    /// `roots`.
    #[serde(default)]
    pub repos: Vec<PathBuf>,
    /// Directories whose direct children are served, each under its
    /// directory name.
    #[serde(default)]
    pub roots: Vec<PathBuf>,
    /// How often, in seconds, the Desk fetches each repository's remote;
    /// `0` never.
    #[serde(default = "fetch_every")]
    pub fetch_seconds: u64,
    /// The Desk's own directory: what it has told the human of, and the
    /// clones it writes a stop in. Without one it remembers nothing across
    /// a restart and makes no stop. `load` defaults it to
    /// `~/.cache/atelier-desk`.
    pub state: Option<PathBuf>,
    /// The Desk's address as the human opens it, such as
    /// `http://buildbot.example.ts.net:3780`, for the links it sends.
    pub url: Option<String>,
    /// The `gh` command, for pull requests on a GitHub forge.
    #[serde(default = "gh")]
    pub gh: PathBuf,
    /// The addresses to listen on.
    #[serde(default = "loopback")]
    pub listen: Vec<SocketAddr>,
    /// The names a request's `Host` may give, without a port; any other is
    /// refused with 403.
    #[serde(default = "local_names")]
    pub hosts: Vec<String>,
    /// The built app's directory. Without one only `/api` is served.
    pub app: Option<PathBuf>,
    /// How to reach T3 Code. Without it the Desk makes no call to T3 Code
    /// and reports its threads as unavailable.
    pub t3: Option<T3Config>,
    /// How the human is told when a plan is proposed or merges. Without it
    /// the inbox lists the news and nothing is sent.
    pub notify: Option<NotifyConfig>,
}

/// The `[notify]` table.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NotifyConfig {
    /// The program and its first arguments; the Desk adds a title, a line
    /// of text and a link, as three more arguments.
    pub command: Vec<String>,
}

pub const FETCH_SECONDS: u64 = 300;

fn fetch_every() -> u64 {
    FETCH_SECONDS
}

fn gh() -> PathBuf {
    PathBuf::from("gh")
}

/// The `[t3]` table: `command` and `url` are needed.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct T3Config {
    /// The `t3` binary that matches the running T3 Code, run for tokens.
    pub command: PathBuf,
    /// T3 Code's MCP server, such as `http://127.0.0.1:3773/mcp`.
    pub url: String,
    /// The first message of a planning session's thread, with `{mission}`
    /// and `{repo}` replaced by the mission's ID and the repository's name.
    #[serde(default = "session_message")]
    pub session_message: String,
}

/// The first message of a planning session when the configuration gives
/// none.
pub const SESSION_MESSAGE: &str = "Start a planning session for mission {mission} in this repository ({repo}). Load the Agent Factory skill (/agent-factory) and plan {mission} with me: read the mission and its documents, then work with me on its plan.";

fn session_message() -> String {
    SESSION_MESSAGE.to_string()
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
        for root in config.roots.iter_mut().chain(config.repos.iter_mut()) {
            *root = resolve(here, root);
        }
        let state = (config.state.take()).unwrap_or_else(|| PathBuf::from("~/.cache/atelier-desk"));
        config.state = Some(resolve(here, &state));
        config.app = config.app.map(|app| resolve(here, &app));
        if let Some(t3) = &mut config.t3 {
            t3.command = resolve(here, &t3.command);
        }
        if config.roots.is_empty() && config.repos.is_empty() {
            bail!("{} names no repository and no root", path.display());
        }
        if let Some(notify) = &mut config.notify {
            let Some(program) = notify.command.first_mut() else {
                bail!("{}: [notify] command names no program", path.display());
            };
            if program.starts_with("~/") {
                *program = resolve(here, Path::new(program)).display().to_string();
            }
        }
        Ok(config)
    }

    /// Every repository served, by name: those of `repos` in their order,
    /// then each root's by name. A name a repository before it already gave is listed with the reason it is
    /// not served.
    pub fn repositories(&self) -> Vec<(String, PathBuf, Option<String>)> {
        let mut found: Vec<(String, PathBuf, Option<String>)> = Vec::new();
        let mut dirs: Vec<PathBuf> = self.repos.clone();
        for root in &self.roots {
            let Ok(entries) = std::fs::read_dir(root) else {
                continue;
            };
            let mut children: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
            children.sort();
            dirs.extend(children.into_iter().filter(|dir| is_repository(dir)));
        }
        for dir in dirs {
            let Some(name) = dir.file_name() else {
                continue;
            };
            let name = name.to_string_lossy().into_owned();
            let taken = found.iter().find(|(other, _, _)| *other == name);
            let error = taken.map(|(_, first, _)| {
                format!(
                    "{} already serves a repository named {name}",
                    first.display()
                )
            });
            found.push((name, dir, error));
        }
        found
    }

    /// The directory of the repository served as `name`, if one is.
    pub fn repository(&self, name: &str) -> Option<PathBuf> {
        if name.is_empty() || name.contains(['/', '\\']) || name == "." || name == ".." {
            return None;
        }
        let named = |dir: &&PathBuf| dir.file_name().is_some_and(|n| n == name);
        if let Some(dir) = self.repos.iter().find(named) {
            return Some(dir.clone());
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
        && atelier_core::git::output(dir, &["cat-file", "-e", "HEAD:.atelier/config.toml"], None)
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
        assert_eq!(config.fetch_seconds, FETCH_SECONDS);
        assert!(config.notify.is_none());
        let home = PathBuf::from(std::env::var_os("HOME").unwrap());
        assert_eq!(config.state, Some(home.join(".cache/atelier-desk")));

        let given =
            "caller = \"s\"\nroots = [\"~/r\"]\nlisten = [\"100.64.0.1:80\"]\nhosts = [\"desk\"]\n";
        std::fs::write(&path, given).unwrap();
        let config = Config::load(&path).unwrap();
        assert_eq!(config.listen, [SocketAddr::from(([100, 64, 0, 1], 80))]);
        assert_eq!(config.hosts, ["desk"]);
        assert_eq!(config.roots, [home.join("r")]);

        assert!(config.t3.is_none());

        let t3 = "[t3]\ncommand = \"~/t3\"\nurl = \"http://127.0.0.1:3773/mcp\"\n";
        std::fs::write(&path, format!("caller = \"s\"\nroots = [\"r\"]\n{t3}")).unwrap();
        let t3 = Config::load(&path).unwrap().t3.unwrap();
        assert_eq!(t3.command, home.join("t3"));
        assert_eq!(t3.url, "http://127.0.0.1:3773/mcp");
        assert_eq!(t3.session_message, SESSION_MESSAGE);
        let own = "[t3]\ncommand = \"t3\"\nurl = \"u\"\nsession_message = \"Plan {mission}.\"\n";
        std::fs::write(&path, format!("caller = \"s\"\nroots = [\"r\"]\n{own}")).unwrap();
        assert_eq!(
            Config::load(&path).unwrap().t3.unwrap().session_message,
            "Plan {mission}."
        );
        let half = "caller = \"s\"\nroots = [\"r\"]\n[t3]\nurl = \"http://127.0.0.1:1/mcp\"\n";
        std::fs::write(&path, half).unwrap();
        assert!(
            Config::load(&path).is_err(),
            "a [t3] table needs its command"
        );

        std::fs::write(&path, "caller = \"s\"\nroots = []\n").unwrap();
        assert!(Config::load(&path).is_err());

        let repos = "caller = \"s\"\nrepos = [\"~/projects/a\"]\nfetch_seconds = 60\nstate = \"st\"\n[notify]\ncommand = [\"echo\", \"-n\"]\n";
        std::fs::write(&path, repos).unwrap();
        let config = Config::load(&path).unwrap();
        assert_eq!(config.repos, [home.join("projects/a")]);
        assert_eq!(config.fetch_seconds, 60);
        assert_eq!(config.state, Some(dir.path().join("st")));
        assert_eq!(config.notify.unwrap().command, ["echo", "-n"]);
        let mail =
            "caller = \"s\"\nrepos = [\"a\"]\n[notify]\ncommand = [\"~/bin/mail\", \"personal\"]\n";
        std::fs::write(&path, mail).unwrap();
        let command = Config::load(&path).unwrap().notify.unwrap().command;
        assert_eq!(
            command,
            [
                home.join("bin/mail").display().to_string(),
                "personal".into()
            ]
        );
        let empty = "caller = \"s\"\nrepos = [\"a\"]\n[notify]\ncommand = []\n";
        std::fs::write(&path, empty).unwrap();
        assert!(
            Config::load(&path).is_err(),
            "a notify command names a program"
        );
    }
}
