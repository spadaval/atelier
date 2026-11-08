//! Whether the atelier binary and the Agent Factory skill are out of date
//! (atelier-45gr). `doctor` checks over the network and caches the result;
//! `status` reads only the cache, so it stays fast. Nothing here blocks: a
//! stale install, or a check that cannot run, is a line of output.
use std::env;
use std::fmt;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};

/// Set to anything but `0` or empty, it turns off the network calls, the
/// cache and `status`'s refresh. Tests and CI set it.
pub const NO_UPDATE_CHECK: &str = "ATELIER_NO_UPDATE_CHECK";
/// The hidden subcommand `status` re-executes, detached, to refresh the cache.
pub const REFRESH_COMMAND: &str = "refresh-update-check";

const RELEASE_REPO: &str = "https://github.com/spadaval/atelier";
/// The release installer line from `README.md`.
const INSTALLER: &str = "curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh | sh";
/// Vercel's `skills` CLI, pinned so its output and exit codes stay the ones
/// this module reads.
const SKILLS_CLI: &str = "skills@1.7.6";
const SKILL: &str = "agent-factory";
const SKILL_ADD: &str = "npx skills add spadaval/atelier --skill agent-factory -g";
const SKILL_UPDATE: &str = "npx skills update agent-factory -g";
const GIT_TIMEOUT: Duration = Duration::from_secs(5);
/// A cold `npx` download alone takes about 2.5 s.
const NPX_TIMEOUT: Duration = Duration::from_secs(10);
const CACHE_FILE: &str = "update-check";
const CACHE_MAX_AGE: chrono::TimeDelta = chrono::TimeDelta::hours(24);
/// Marks a refresh `status` started, so concurrent sessions start one.
const REFRESH_STAMP: &str = "update-check.refreshing";
const REFRESH_RETRY: Duration = Duration::from_secs(10 * 60);

/// Whether `ATELIER_NO_UPDATE_CHECK` turns the check off.
pub fn disabled() -> bool {
    env::var_os(NO_UPDATE_CHECK).is_some_and(|value| !value.is_empty() && value != "0")
}

/// `doctor`'s lines: it checks both, and caches the result for `status`.
pub fn doctor_lines() -> Vec<String> {
    if disabled() {
        return vec![format!("Update check off: {NO_UPDATE_CHECK} is set.")];
    }
    let report = Report::check();
    let mut lines = report.lines();
    if let Some(dir) = cache_dir() {
        if let Err(error) = report.cache().write(&dir) {
            lines.push(format!(
                "Could not cache the update check in {}: {error}",
                dir.display()
            ));
        }
    }
    lines
}

/// The hidden refresh `status` starts: check and cache, printing nothing.
pub fn refresh() {
    if disabled() {
        return;
    }
    if let Some(dir) = cache_dir() {
        let _ = Report::check().cache().write(&dir);
    }
}

/// `status`'s warning lines, read from the cache only. A missing or
/// day-old cache shows nothing and starts a detached refresh.
pub fn status_warnings() -> Vec<String> {
    if disabled() {
        return Vec::new();
    }
    let Some(dir) = cache_dir() else {
        return Vec::new();
    };
    match Cache::read(&dir.join(CACHE_FILE)) {
        Some(cache) if Utc::now() - cache.checked < CACHE_MAX_AGE => cache.warnings(),
        _ => {
            start_refresh(&dir);
            Vec::new()
        }
    }
}

/// `$XDG_CACHE_HOME/atelier`, else the platform's user cache directory.
pub fn cache_dir() -> Option<PathBuf> {
    if let Some(dir) = env::var_os("XDG_CACHE_HOME").filter(|dir| Path::new(dir).is_absolute()) {
        return Some(PathBuf::from(dir).join("atelier"));
    }
    let home = home()?;
    Some(if cfg!(target_os = "macos") {
        home.join("Library/Caches/atelier")
    } else {
        home.join(".cache/atelier")
    })
}

fn home() -> Option<PathBuf> {
    env::var_os("HOME")
        .filter(|home| !home.is_empty())
        .map(PathBuf::from)
}

/// Re-executes this binary's hidden refresh with no terminal and in its own
/// process group, so neither the hook runner nor a Ctrl-C waits on it or
/// stops it. A refresh started in the last few minutes is not repeated.
fn start_refresh(dir: &Path) {
    let stamp = dir.join(REFRESH_STAMP);
    let recent = fs::metadata(&stamp)
        .and_then(|meta| meta.modified())
        .is_ok_and(|at| at.elapsed().is_ok_and(|age| age < REFRESH_RETRY));
    if recent || fs::create_dir_all(dir).is_err() || fs::write(&stamp, "").is_err() {
        return;
    }
    let Ok(exe) = env::current_exe() else {
        return;
    };
    let mut command = Command::new(exe);
    command
        .arg(REFRESH_COMMAND)
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(&mut command, 0);
    let _ = command.spawn();
}

/// What one check found about the binary and the skill.
struct Report {
    binary: Binary,
    skill: Skill,
}

impl Report {
    /// The skill check runs while `git ls-remote` waits on the network.
    fn check() -> Self {
        let binary = thread::spawn(Binary::check);
        let skill = match home() {
            Some(home) => Skill::check(&home),
            None => Skill::Unknown("HOME is not set".into()),
        };
        Self {
            binary: binary
                .join()
                .unwrap_or_else(|_| Binary::Unknown("the check panicked".into())),
            skill,
        }
    }

    fn lines(&self) -> Vec<String> {
        vec![self.binary.line(), self.skill.line()]
    }

    fn cache(&self) -> Cache {
        Cache {
            checked: Utc::now(),
            latest: match self.binary {
                Binary::Current { latest, .. } | Binary::Stale { latest, .. } => Some(latest),
                Binary::Unknown(_) => None,
            },
            skill_warning: self.skill.warning(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Version(u64, u64, u64);

impl Version {
    /// `X.Y.Z`, ignoring `+<build>` metadata; anything else, such as a
    /// pre-release, is not a release version.
    fn parse(text: &str) -> Option<Self> {
        let core = text.split('+').next()?;
        let mut parts = core.split('.').map(|part| {
            (!part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
                .then(|| part.parse().ok())
                .flatten()
        });
        let version = Self(parts.next()??, parts.next()??, parts.next()??);
        parts.next().is_none().then_some(version)
    }

    fn running() -> Option<Self> {
        Self::parse(option_env!("ATELIER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

enum Binary {
    Current { running: Version, latest: Version },
    Stale { running: Version, latest: Version },
    Unknown(String),
}

impl Binary {
    fn check() -> Self {
        let Some(running) = Version::running() else {
            return Self::Unknown("this build has no release version".into());
        };
        match latest_release() {
            Ok(latest) => Self::compare(running, latest),
            Err(reason) => Self::Unknown(reason),
        }
    }

    fn compare(running: Version, latest: Version) -> Self {
        if running < latest {
            Self::Stale { running, latest }
        } else {
            Self::Current { running, latest }
        }
    }

    fn line(&self) -> String {
        match self {
            Self::Current { running, latest } => {
                format!("atelier {running}: current (latest release {latest}).")
            }
            Self::Stale { running, latest } => stale_binary_warning(*running, *latest),
            Self::Unknown(reason) => {
                format!("atelier: could not check for a newer release ({reason}).")
            }
        }
    }
}

fn stale_binary_warning(running: Version, latest: Version) -> String {
    format!("⚠ atelier {running} is older than the latest release {latest}. Upgrade: `{INSTALLER}`")
}

/// The highest `vX.Y.Z` tag of the release repository. `git ls-remote`
/// needs no HTTP client and has no API rate limit.
fn latest_release() -> Result<Version, String> {
    let mut command = Command::new("git");
    command
        .args(["ls-remote", "--tags", "--refs", RELEASE_REPO])
        .env("GIT_TERMINAL_PROMPT", "0");
    let output =
        run(&mut command, GIT_TIMEOUT).map_err(|reason| format!("git ls-remote {reason}"))?;
    if !output.status.success() {
        return Err(format!(
            "git ls-remote failed: {}",
            first_line(&output.stderr).unwrap_or("no error message")
        ));
    }
    output
        .stdout
        .lines()
        .filter_map(|line| line.split('\t').nth(1)?.strip_prefix("refs/tags/v"))
        .filter_map(Version::parse)
        .max()
        .ok_or_else(|| "no vX.Y.Z release tag found".into())
}

/// The Agent Factory skill paths agents read, in the order checked.
pub fn skill_paths(home: &Path) -> [PathBuf; 2] {
    [
        home.join(".agents/skills").join(SKILL),
        home.join(".claude/skills").join(SKILL),
    ]
}

/// The Git checkout a skill path links into: `path` is a symlink whose
/// target lies in a Git work tree outside the `skills` CLI's own store
/// (`~/.agents/skills`), where `skills add` links agents' copies.
pub fn linked_checkout(home: &Path, path: &Path) -> Option<PathBuf> {
    if !fs::symlink_metadata(path).ok()?.file_type().is_symlink() {
        return None;
    }
    let target = fs::canonicalize(path).ok()?;
    let store = fs::canonicalize(home.join(".agents/skills")).ok();
    if store.is_some_and(|store| target.starts_with(store)) {
        return None;
    }
    git_text(&target, &["rev-parse", "--show-toplevel"]).map(PathBuf::from)
}

enum Skill {
    Current,
    Stale,
    Unknown(String),
    Linked {
        checkout: PathBuf,
        /// Commits `HEAD` lacks from the base branch, and its name.
        behind: Result<(u64, String), String>,
    },
    Untracked,
    Missing,
    NoNpx,
}

impl Skill {
    /// A linked checkout is read from local refs, before any `npx`; the
    /// `skills` CLI decides every other case.
    fn check(home: &Path) -> Self {
        let linked = skill_paths(home)
            .iter()
            .find_map(|path| linked_checkout(home, path));
        if let Some(checkout) = linked {
            let behind = behind_base(&checkout);
            return Self::Linked { checkout, behind };
        }
        let Some(npx) = find_on_path("npx") else {
            return Self::NoNpx;
        };
        let listed = match skills(&npx, home, &["ls", "-g", "--json"]) {
            Ok(output) if output.status.success() => output.stdout,
            Ok(output) => return Self::Unknown(skills_failure("ls", &output)),
            Err(reason) => return Self::Unknown(reason),
        };
        match listed_status(&listed) {
            Err(reason) => Self::Unknown(reason),
            Ok(Some(status)) if status == "tracked" => {
                match skills(&npx, home, &["check", SKILL, "-g"]) {
                    Ok(output) => match output.status.code() {
                        Some(1) => Self::Stale,
                        // `check` also exits 0 when it could not reach the source.
                        Some(0) if !output.stdout.contains("Failed to check") => Self::Current,
                        Some(0) => Self::Unknown(
                            first_line_containing(&output.stdout, "Failed to check")
                                .unwrap_or_else(|| "it failed to check".into()),
                        ),
                        _ => Self::Unknown(skills_failure("check", &output)),
                    },
                    Err(reason) => Self::Unknown(reason),
                }
            }
            Ok(Some(status)) if status == "untracked" => Self::Untracked,
            Ok(Some(status)) if status == "missing" => Self::Missing,
            Ok(Some(status)) => Self::Unknown(format!("`skills ls` reported status {status}")),
            Ok(None) => Self::Missing,
        }
    }

    fn line(&self) -> String {
        if let Some(warning) = self.warning() {
            return warning;
        }
        match self {
            Self::Current => "Agent Factory skill: current.".into(),
            Self::Unknown(reason) => format!("Agent Factory skill: could not check ({reason})."),
            Self::Linked {
                checkout,
                behind: Ok((_, base)),
            } => format!(
                "Agent Factory skill: linked to a checkout at {}, not behind {base}.",
                checkout.display()
            ),
            Self::Linked {
                checkout,
                behind: Err(reason),
            } => format!(
                "Agent Factory skill: linked to a checkout at {} ({reason}).",
                checkout.display()
            ),
            Self::Untracked => format!(
                "⚠ Agent Factory skill: not installed by `npx skills`, so updates cannot be checked. Reinstall: `{SKILL_ADD}`"
            ),
            Self::Missing => format!("Agent Factory skill: not installed. Install: `{SKILL_ADD}`"),
            Self::NoNpx => format!(
                "Agent Factory skill: checking it needs Node (`npx` is not on PATH). Install: `{SKILL_ADD}`"
            ),
            Self::Stale => unreachable!("a stale skill has a warning"),
        }
    }

    /// The warning `status` repeats: the skill is behind.
    fn warning(&self) -> Option<String> {
        match self {
            Self::Stale => Some(format!(
                "⚠ Agent Factory skill is out of date. Update: `{SKILL_UPDATE}`"
            )),
            Self::Linked {
                checkout,
                behind: Ok((count, base)),
            } if *count > 0 => Some(format!(
                "⚠ Agent Factory skill: linked to a checkout at {path}, {count} commit{s} behind {base}. Update: `git -C {path} checkout --detach {base}`",
                path = checkout.display(),
                s = if *count == 1 { "" } else { "s" },
            )),
            _ => None,
        }
    }
}

/// How many commits the checkout's `HEAD` lacks from its local base branch:
/// the branch `origin/HEAD` names, else `master`, else `main`. Local refs
/// only; nothing is fetched.
fn behind_base(checkout: &Path) -> Result<(u64, String), String> {
    let remote_head = git_text(
        checkout,
        &["symbolic-ref", "--short", "refs/remotes/origin/HEAD"],
    );
    let base = remote_head
        .as_deref()
        .and_then(|head| head.split_once('/'))
        .map(|(_, branch)| branch.to_owned())
        .into_iter()
        .chain(["master".to_owned(), "main".to_owned()])
        .find(|branch| {
            git_text(
                checkout,
                &[
                    "rev-parse",
                    "--verify",
                    "--quiet",
                    &format!("refs/heads/{branch}"),
                ],
            )
            .is_some()
        })
        .ok_or("no local master or main branch to compare with")?;
    let count = git_text(
        checkout,
        &["rev-list", "--count", &format!("HEAD..refs/heads/{base}")],
    )
    .and_then(|count| count.parse().ok())
    .ok_or_else(|| format!("could not compare HEAD with {base}"))?;
    Ok((count, base))
}

/// The `status` the `skills` CLI's JSON listing gives this skill, if listed.
fn listed_status(listing: &str) -> Result<Option<String>, String> {
    let start = listing.find('[').unwrap_or(0);
    let entries: Vec<serde_json::Value> = serde_json::from_str(&listing[start..])
        .map_err(|error| format!("could not read `skills ls -g --json`: {error}"))?;
    Ok(entries
        .iter()
        .find(|entry| entry["name"] == SKILL)
        .map(|entry| entry["status"].as_str().unwrap_or("unknown").to_owned()))
}

fn skills(npx: &Path, home: &Path, args: &[&str]) -> Result<Output, String> {
    let mut command = Command::new(npx);
    command
        .args(["-y", SKILLS_CLI])
        .args(args)
        .env("DISABLE_TELEMETRY", "1")
        .current_dir(home);
    run(&mut command, NPX_TIMEOUT).map_err(|reason| format!("`npx {SKILLS_CLI}` {reason}"))
}

fn skills_failure(subcommand: &str, output: &Output) -> String {
    format!(
        "`skills {subcommand}` exited with {}{}",
        output
            .status
            .code()
            .map_or("a signal".to_owned(), |code| code.to_string()),
        first_line(&output.stderr)
            .map(|line| format!(": {line}"))
            .unwrap_or_default()
    )
}

fn find_on_path(program: &str) -> Option<PathBuf> {
    env::split_paths(&env::var_os("PATH")?)
        .map(|dir| dir.join(program))
        .find(|path| is_executable(path))
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path).is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}

fn git_text(dir: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn first_line(text: &str) -> Option<&str> {
    text.lines().map(str::trim).find(|line| !line.is_empty())
}

fn first_line_containing(text: &str, needle: &str) -> Option<String> {
    text.lines()
        .map(|line| strip_ansi(line).trim().to_owned())
        .find(|line| line.contains(needle))
}

/// Drops the colour codes (`ESC [ ... m`) the `skills` CLI wraps lines in.
fn strip_ansi(line: &str) -> String {
    let mut plain = String::with_capacity(line.len());
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            // Skip to the sequence's final byte, a letter.
            for c in chars.by_ref() {
                if c.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            plain.push(c);
        }
    }
    plain
}

struct Output {
    status: ExitStatus,
    stdout: String,
    stderr: String,
}

/// Runs a command with no input, killing it and everything it started when
/// it outlives `timeout`. The error completes "`<program>` ...".
fn run(command: &mut Command, timeout: Duration) -> Result<Output, String> {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    // Its own process group, so a timeout also stops what it started, such
    // as the `node` process `npx` runs.
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(command, 0);
    let mut child = command
        .spawn()
        .map_err(|error| format!("could not start: {error}"))?;
    let read = |pipe: Option<Box<dyn Read + Send>>| {
        thread::spawn(move || {
            let mut bytes = Vec::new();
            if let Some(mut pipe) = pipe {
                let _ = pipe.read_to_end(&mut bytes);
            }
            String::from_utf8_lossy(&bytes).into_owned()
        })
    };
    let stdout = read(child.stdout.take().map(|pipe| Box::new(pipe) as _));
    let stderr = read(child.stderr.take().map(|pipe| Box::new(pipe) as _));
    let deadline = Instant::now() + timeout;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(20)),
            result => {
                kill_group(&mut child);
                return Err(match result {
                    Err(error) => format!("could not be waited on: {error}"),
                    _ => format!("timed out after {} s", timeout.as_secs_f32()),
                });
            }
        }
    };
    Ok(Output {
        status,
        stdout: stdout.join().unwrap_or_default(),
        stderr: stderr.join().unwrap_or_default(),
    })
}

fn kill_group(child: &mut std::process::Child) {
    #[cfg(unix)]
    if let Ok(group) = i32::try_from(child.id()) {
        // SAFETY: kill(2) with a negative pid signals the process group the
        // child leads; it touches no memory.
        unsafe {
            libc::kill(-group, libc::SIGKILL);
        }
    }
    let _ = child.kill();
    let _ = child.wait();
}

/// The cached result: when it was checked, the latest release, and the
/// skill's warning when it was behind. A plain `key: value` file.
#[derive(Debug, PartialEq)]
struct Cache {
    checked: DateTime<Utc>,
    latest: Option<Version>,
    skill_warning: Option<String>,
}

impl Cache {
    fn read(path: &Path) -> Option<Self> {
        let text = fs::read_to_string(path).ok()?;
        Self::parse(&text)
    }

    fn parse(text: &str) -> Option<Self> {
        let mut checked = None;
        let mut latest = None;
        let mut skill_warning = None;
        for line in text.lines() {
            match line.split_once(": ") {
                Some(("checked", value)) => checked = DateTime::parse_from_rfc3339(value).ok(),
                Some(("latest", value)) => latest = Version::parse(value),
                Some(("skill", value)) => skill_warning = Some(value.to_owned()),
                _ => {}
            }
        }
        Some(Self {
            checked: checked?.with_timezone(&Utc),
            latest,
            skill_warning,
        })
    }

    fn render(&self) -> String {
        let mut text = format!("checked: {}\n", self.checked.to_rfc3339());
        if let Some(latest) = self.latest {
            text.push_str(&format!("latest: {latest}\n"));
        }
        if let Some(warning) = &self.skill_warning {
            text.push_str(&format!("skill: {warning}\n"));
        }
        text
    }

    /// Written whole, then renamed, so `status` never reads half a file.
    fn write(&self, dir: &Path) -> std::io::Result<()> {
        fs::create_dir_all(dir)?;
        let partial = dir.join(format!("{CACHE_FILE}.{}", std::process::id()));
        fs::write(&partial, self.render())?;
        fs::rename(&partial, dir.join(CACHE_FILE))?;
        let _ = fs::remove_file(dir.join(REFRESH_STAMP));
        Ok(())
    }

    /// The binary is judged against this build's version, so an upgrade
    /// clears its warning without a new check.
    fn warnings(&self) -> Vec<String> {
        let binary = match (Version::running(), self.latest) {
            (Some(running), Some(latest)) if running < latest => {
                Some(stale_binary_warning(running, latest))
            }
            _ => None,
        };
        binary
            .into_iter()
            .chain(self.skill_warning.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_ignore_build_metadata_and_reject_pre_releases() {
        assert_eq!(Version::parse("0.3.1+abc123-dirty"), Some(Version(0, 3, 1)));
        assert_eq!(Version::parse("10.0.2"), Some(Version(10, 0, 2)));
        assert_eq!(Version::parse("1.0.0-rc.1"), None);
        assert_eq!(Version::parse("1.0"), None);
        assert_eq!(Version::parse("1.0.0.0"), None);
        assert!(Version(0, 10, 0) > Version(0, 9, 9));
    }

    #[test]
    fn a_command_that_outlives_its_timeout_is_stopped() {
        let started = Instant::now();
        let result = run(
            Command::new("sh").args(["-c", "sleep 30 & sleep 30"]),
            Duration::from_millis(200),
        );
        assert_eq!(result.err().as_deref(), Some("timed out after 0.2 s"));
        assert!(started.elapsed() < Duration::from_secs(5));
    }

    #[test]
    fn the_cache_round_trips() {
        let cache = Cache {
            checked: DateTime::parse_from_rfc3339("2026-11-08T06:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            latest: Some(Version(0, 4, 0)),
            skill_warning: Some("⚠ Agent Factory skill is out of date.".into()),
        };
        assert_eq!(Cache::parse(&cache.render()), Some(cache));
        assert_eq!(Cache::parse("latest: 0.4.0\n"), None);
    }

    #[test]
    fn a_failed_check_line_loses_its_colour_codes() {
        let output = "\u{1b}[2m  ✗ Failed to check skills from spadaval/atelier\u{1b}[0m\n\u{1b}[38;5;250m✓ All global skills are up to date\u{1b}[0m";
        assert_eq!(
            first_line_containing(output, "Failed to check").as_deref(),
            Some("✗ Failed to check skills from spadaval/atelier")
        );
    }

    #[test]
    fn the_listing_decides_the_skills_case() {
        let listing = r#"[{"name": "boat", "status": "untracked"}, {"name": "agent-factory", "status": "tracked"}]"#;
        assert_eq!(listed_status(listing), Ok(Some("tracked".into())));
        assert_eq!(listed_status("[]"), Ok(None));
        assert!(listed_status("npm warn\nnot json").is_err());
    }
}
