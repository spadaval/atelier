//! The update check in `doctor` and `status` (atelier-45gr). Nothing here
//! touches the network or the real home: each test has its own home and
//! cache, a fake `npx` on `PATH`, and a local release repository that Git's
//! `url.<path>.insteadOf` puts in place of GitHub.
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

use tempfile::TempDir;

const RUNNING: &str = env!("CARGO_PKG_VERSION");
const RELEASE_REPO: &str = "https://github.com/spadaval/atelier";
const INSTALLER: &str = "curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh | sh";
const ADD: &str = "npx skills add spadaval/atelier --skill agent-factory -g";
const UP_TO_DATE: &str = "\u{1b}[38;5;250m✓ All global skills are up to date\u{1b}[0m";

fn git(dir: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(dir)
        .args(["-c", "user.name=t", "-c", "user.email=t@t"])
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn on_path(program: &str) -> PathBuf {
    std::env::split_paths(&std::env::var_os("PATH").unwrap())
        .map(|dir| dir.join(program))
        .find(|path| path.is_file())
        .unwrap_or_else(|| panic!("{program} is not on PATH"))
}

struct Sandbox {
    _dir: TempDir,
    root: PathBuf,
    repo: PathBuf,
    home: PathBuf,
    cache: PathBuf,
    /// Holds the fake `npx`, when there is one.
    npx: PathBuf,
    /// Holds `git` and nothing else, so the real `npx` is never found.
    tools: PathBuf,
    /// Where `url.<path>.insteadOf` sends the release repository.
    release: PathBuf,
}

impl Sandbox {
    /// A committed Atelier repository, and a release repository with `tags`.
    fn new(tags: &[&str]) -> Self {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let [repo, home, cache, npx, tools, release] =
            ["repo", "home", "cache", "npx", "tools", "release"].map(|name| {
                let path = root.join(name);
                fs::create_dir_all(&path).unwrap();
                path
            });
        #[cfg(unix)]
        std::os::unix::fs::symlink(on_path("git"), tools.join("git")).unwrap();
        git(&repo, &["init", "-q", "-b", "master"]);
        let init = Command::new(env!("CARGO_BIN_EXE_atelier"))
            .current_dir(&repo)
            .args(["init"])
            .output()
            .unwrap();
        assert!(init.status.success(), "{init:?}");
        git(&repo, &["add", "-A"]);
        git(&repo, &["commit", "-qm", "Install Atelier"]);
        git(&release, &["init", "-q", "-b", "master"]);
        git(
            &release,
            &["commit", "-q", "--allow-empty", "-m", "release"],
        );
        for tag in tags {
            git(&release, &["tag", tag]);
        }
        Self {
            _dir: dir,
            root,
            repo,
            home,
            cache,
            npx,
            tools,
            release,
        }
    }

    /// A fake `npx` that logs its arguments and answers `skills ls -g
    /// --json` with `listing` and `skills check agent-factory -g` with
    /// `check` and `exit`.
    fn fake_npx(&self, listing: &str, check: &str, exit: i32) {
        let script = format!(
            "#!/bin/sh\nprintf '%s DISABLE_TELEMETRY=%s\\n' \"$*\" \"$DISABLE_TELEMETRY\" >> '{log}'\ncase \"$*\" in\n  *' ls -g --json') printf '%s\\n' '{listing}' ;;\n  *' check agent-factory -g') printf '%s\\n' '{check}'; exit {exit} ;;\n  *) exit 2 ;;\nesac\n",
            log = self.npx_log().display(),
        );
        let path = self.npx.join("npx");
        fs::write(&path, script).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }

    fn tracked(&self, check: &str, exit: i32) {
        self.fake_npx(&listing("tracked"), check, exit);
    }

    fn npx_log(&self) -> PathBuf {
        self.root.join("npx.log")
    }

    fn npx_calls(&self) -> String {
        fs::read_to_string(self.npx_log()).unwrap_or_default()
    }

    fn cache_file(&self) -> PathBuf {
        self.cache.join("atelier/update-check")
    }

    fn command(&self, args: &[&str]) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_atelier"));
        command
            .current_dir(&self.repo)
            .args(args)
            .env_remove("ATELIER_NO_UPDATE_CHECK")
            .env("HOME", &self.home)
            .env("XDG_CACHE_HOME", &self.cache)
            .env(
                "PATH",
                std::env::join_paths([&self.npx, &self.tools]).unwrap(),
            )
            .env("GIT_CONFIG_COUNT", "1")
            .env(
                "GIT_CONFIG_KEY_0",
                format!("url.{}.insteadOf", self.release.display()),
            )
            .env("GIT_CONFIG_VALUE_0", RELEASE_REPO);
        command
    }

    /// Runs atelier and returns whether it succeeded, its stdout and stderr.
    fn run(&self, command: &mut Command) -> (bool, String, String) {
        let output = command.stdin(Stdio::null()).output().unwrap();
        (
            output.status.success(),
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap(),
        )
    }

    /// `doctor`'s Updates lines; doctor itself must pass.
    fn doctor(&self) -> Vec<String> {
        let (passed, stdout, stderr) = self.run(&mut self.command(&["doctor"]));
        assert!(passed, "{stdout}{stderr}");
        assert!(
            stdout
                .ends_with("Doctor: every done claim and the whole review history were checked.\n"),
            "{stdout}"
        );
        updates(&stdout)
    }

    fn write_cache(&self, checked: chrono::DateTime<chrono::Utc>, body: &str) {
        fs::create_dir_all(self.cache.join("atelier")).unwrap();
        fs::write(
            self.cache_file(),
            format!("checked: {}\n{body}", checked.to_rfc3339()),
        )
        .unwrap();
    }

    /// A Git checkout holding the skill, its `HEAD` detached `behind`
    /// commits short of `master`, linked from `link` in the home.
    fn linked_checkout(&self, link: &str, behind: usize) -> PathBuf {
        let checkout = self.root.join("skill-checkout");
        fs::create_dir_all(checkout.join("skills/agent-factory")).unwrap();
        git(&checkout, &["init", "-q", "-b", "master"]);
        fs::write(checkout.join("skills/agent-factory/SKILL.md"), "v1\n").unwrap();
        git(&checkout, &["add", "-A"]);
        git(&checkout, &["commit", "-qm", "v1"]);
        for n in 0..behind {
            fs::write(
                checkout.join("skills/agent-factory/SKILL.md"),
                format!("v{}\n", n + 2),
            )
            .unwrap();
            git(&checkout, &["commit", "-qam", "next"]);
        }
        git(
            &checkout,
            &["checkout", "-q", "--detach", &format!("master~{behind}")],
        );
        let link = self.home.join(link);
        fs::create_dir_all(link.parent().unwrap()).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(checkout.join("skills/agent-factory"), &link).unwrap();
        // Doctor names the checkout Git reports, which resolves symlinks in
        // the temporary directory (/var is /private/var on macOS).
        fs::canonicalize(checkout).unwrap()
    }
}

fn listing(status: &str) -> String {
    format!(
        r#"[{{"name": "boat", "status": "untracked"}}, {{"name": "agent-factory", "path": "/x", "status": "{status}"}}]"#
    )
}

/// The lines of the Updates section, between its heading and the next
/// blank line.
fn updates(stdout: &str) -> Vec<String> {
    stdout
        .split("\nUpdates\n")
        .nth(1)
        .map(|rest| {
            rest.lines()
                .skip(1)
                .take_while(|line| !line.is_empty() && !line.starts_with("Doctor:"))
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn stale_binary(latest: &str) -> String {
    format!("⚠ atelier {RUNNING} is older than the latest release {latest}. Upgrade: `{INSTALLER}`")
}

#[test]
fn doctor_warns_of_a_stale_binary_and_skill_and_still_passes() {
    let s = Sandbox::new(&["v0.0.1", "v99.0.0", "v100.0.0-rc.1", "not-a-version"]);
    s.tracked("Checking global skill updates...\n1 update available", 1);
    assert_eq!(
        s.doctor(),
        [
            stale_binary("99.0.0"),
            "⚠ Agent Factory skill is out of date. Update: `npx skills update agent-factory -g`"
                .into(),
        ]
    );
    // The pinned CLI, without telemetry, decides the case and then checks.
    assert_eq!(
        s.npx_calls(),
        "-y skills@1.7.6 ls -g --json DISABLE_TELEMETRY=1\n-y skills@1.7.6 check agent-factory -g DISABLE_TELEMETRY=1\n"
    );
    let cache = fs::read_to_string(s.cache_file()).unwrap();
    assert!(
        cache.contains("\nlatest: 99.0.0\nskill: ⚠ Agent Factory skill is out of date."),
        "{cache}"
    );
}

#[test]
fn doctor_reports_a_current_binary_and_skill() {
    let s = Sandbox::new(&["v0.0.1"]);
    s.tracked(UP_TO_DATE, 0);
    assert_eq!(
        s.doctor(),
        [
            format!("atelier {RUNNING}: current (latest release 0.0.1)."),
            "Agent Factory skill: current.".into(),
        ]
    );
}

/// `skills check` exits 0 when it cannot reach the source, so its "Failed
/// to check" line decides.
#[test]
fn doctor_says_when_the_checks_cannot_run() {
    let s = Sandbox::new(&["v99.0.0"]);
    fs::remove_dir_all(&s.release).unwrap();
    s.tracked(
        &format!(
            "\u{1b}[2m  ✗ Failed to check skills from spadaval/atelier\u{1b}[0m\n{UP_TO_DATE}"
        ),
        0,
    );
    let lines = s.doctor();
    assert!(
        lines[0]
            .starts_with("atelier: could not check for a newer release (git ls-remote failed: "),
        "{lines:?}"
    );
    assert_eq!(
        lines[1],
        "Agent Factory skill: could not check (✗ Failed to check skills from spadaval/atelier)."
    );
    // Nothing is known, so the cache holds no warning.
    let cache = fs::read_to_string(s.cache_file()).unwrap();
    assert_eq!(cache.lines().count(), 1, "{cache}");
}

#[test]
fn doctor_reports_a_skill_linked_to_a_checkout_from_local_refs() {
    let s = Sandbox::new(&["v0.0.1"]);
    // No npx: a linked checkout needs none.
    let checkout = s.linked_checkout(".agents/skills/agent-factory", 2);
    let path = checkout.display();
    assert_eq!(
        s.doctor()[1],
        format!("⚠ Agent Factory skill: linked to a checkout at {path}, 2 commits behind master. Update: `git -C {path} checkout --detach master`")
    );
    git(&checkout, &["checkout", "-q", "--detach", "master"]);
    assert_eq!(
        s.doctor()[1],
        format!("Agent Factory skill: linked to a checkout at {path}, not behind master.")
    );
    assert_eq!(s.npx_calls(), "");
}

#[test]
fn doctor_finds_a_link_in_the_claude_skills_directory() {
    let s = Sandbox::new(&["v0.0.1"]);
    let checkout = s.linked_checkout(".claude/skills/agent-factory", 1);
    assert!(s.doctor()[1].contains(&format!(
        "linked to a checkout at {}, 1 commit behind master",
        checkout.display()
    )));
}

#[test]
fn doctor_warns_that_an_untracked_skill_cannot_be_checked() {
    let s = Sandbox::new(&["v0.0.1"]);
    s.fake_npx(&listing("untracked"), "", 0);
    assert_eq!(
        s.doctor()[1],
        format!("⚠ Agent Factory skill: not installed by `npx skills`, so updates cannot be checked. Reinstall: `{ADD}`")
    );
    assert!(!s.npx_calls().contains("check"));
}

#[test]
fn doctor_gives_the_install_line_for_a_missing_skill() {
    let s = Sandbox::new(&["v0.0.1"]);
    let missing = format!("Agent Factory skill: not installed. Install: `{ADD}`");
    s.fake_npx("[]", "", 0);
    assert_eq!(s.doctor()[1], missing);
    // The lock file names it, but it is gone from disk.
    s.fake_npx(&listing("missing"), "", 0);
    assert_eq!(s.doctor()[1], missing);
}

#[test]
fn doctor_says_the_skill_check_needs_node_without_npx() {
    let s = Sandbox::new(&["v0.0.1"]);
    assert_eq!(
        s.doctor()[1],
        format!(
            "Agent Factory skill: checking it needs Node (`npx` is not on PATH). Install: `{ADD}`"
        )
    );
}

#[test]
fn status_shows_the_cached_warnings_without_checking() {
    let s = Sandbox::new(&["v0.0.1"]);
    // Every check would fail, so only the cache can produce the warnings.
    fs::remove_dir_all(&s.release).unwrap();
    s.fake_npx("not json", "", 9);
    let skill =
        "⚠ Agent Factory skill is out of date. Update: `npx skills update agent-factory -g`";
    s.write_cache(
        chrono::Utc::now(),
        &format!("latest: 99.0.0\nskill: {skill}\n"),
    );
    let (passed, stdout, stderr) = s.run(&mut s.command(&["status"]));
    assert!(passed, "{stderr}");
    assert_eq!(updates(&stdout), [stale_binary("99.0.0"), skill.into()]);

    // Quiet stdout stays the summary.
    let (_, stdout, stderr) = s.run(&mut s.command(&["-q", "status"]));
    assert!(!stdout.contains('⚠'), "{stdout}");
    assert_eq!(stderr, format!("{}\n{skill}\n", stale_binary("99.0.0")));

    // The SessionStart hook carries them too.
    let mut hook = s.command(&["status", "--hook", "claude"]);
    let mut child = hook
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let payload = serde_json::json!({"hook_event_name": "SessionStart", "cwd": s.repo});
    std::io::Write::write_all(
        &mut child.stdin.take().unwrap(),
        payload.to_string().as_bytes(),
    )
    .unwrap();
    let output = child.wait_with_output().unwrap();
    let hook: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let context = hook["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .unwrap();
    assert_eq!(updates(context), [stale_binary("99.0.0"), skill.into()]);

    // A current cache shows nothing, and nothing ran.
    s.write_cache(chrono::Utc::now(), "latest: 0.0.1\n");
    let (_, stdout, _) = s.run(&mut s.command(&["status"]));
    assert!(!stdout.contains("Updates"), "{stdout}");
    assert_eq!(s.npx_calls(), "");
    assert!(!s.cache.join("atelier/update-check.refreshing").exists());
}

#[test]
fn status_refreshes_a_day_old_cache_in_the_background() {
    let s = Sandbox::new(&["v99.0.0"]);
    s.tracked("1 update available", 1);
    let old = chrono::Utc::now() - chrono::TimeDelta::hours(25);
    s.write_cache(old, "latest: 98.0.0\n");
    let (passed, stdout, stderr) = s.run(&mut s.command(&["status"]));
    assert!(passed, "{stderr}");
    // An old result is not shown.
    assert!(!stdout.contains("Updates"), "{stdout}");
    let deadline = Instant::now() + Duration::from_secs(30);
    let cache = loop {
        let cache = fs::read_to_string(s.cache_file()).unwrap();
        if cache.contains("latest: 99.0.0") || Instant::now() > deadline {
            break cache;
        }
        std::thread::sleep(Duration::from_millis(100));
    };
    assert!(
        cache.contains("latest: 99.0.0\nskill: ⚠ Agent Factory skill is out of date."),
        "{cache}"
    );
    let (_, stdout, _) = s.run(&mut s.command(&["status"]));
    assert_eq!(updates(&stdout)[0], stale_binary("99.0.0"));
}

#[test]
fn the_variable_turns_the_check_off() {
    let s = Sandbox::new(&["v99.0.0"]);
    s.tracked("1 update available", 1);
    let (passed, stdout, stderr) =
        s.run(s.command(&["doctor"]).env("ATELIER_NO_UPDATE_CHECK", "1"));
    assert!(passed, "{stderr}");
    assert_eq!(
        updates(&stdout),
        ["Update check off: ATELIER_NO_UPDATE_CHECK is set."]
    );
    assert!(!s.cache_file().exists());

    // Status neither reads the cache nor refreshes it.
    let old = chrono::Utc::now() - chrono::TimeDelta::hours(25);
    s.write_cache(old, "latest: 99.0.0\n");
    let (_, stdout, _) = s.run(s.command(&["status"]).env("ATELIER_NO_UPDATE_CHECK", "1"));
    assert!(!stdout.contains("Updates"), "{stdout}");
    assert!(!s.cache.join("atelier/update-check.refreshing").exists());
    assert_eq!(s.npx_calls(), "");
}
