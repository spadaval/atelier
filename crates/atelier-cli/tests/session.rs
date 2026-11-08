//! Real Git/CLI session boundaries; hook payloads follow Claude Code's hooks reference.
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
use tempfile::{tempdir, TempDir};
#[path = "support/next_steps.rs"]
mod next_steps;
fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap().trim().into()
}
/// Runs atelier, with whether it passed and its combined output. `land`
/// reports work in progress on a work branch and exits successfully
/// (atelier-md0v); these tests read that report, whose verdict is `land:
/// work in progress`, as not passed, as they read a refusal. Tests of the
/// exit status itself run the binary directly.
fn cli(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    (
        out.status.success() && !in_progress(&stdout),
        format!("{stdout}{}", String::from_utf8_lossy(&out.stderr)),
    )
}
/// Whether `stdout` ends in land's verdict for work in progress.
fn in_progress(stdout: &str) -> bool {
    stdout.lines().last().is_some_and(|line| {
        line.starts_with("land: work in progress") || line.starts_with("work in progress")
    })
}
/// A `git` command that may fail, with its success and combined output.
fn cli_git(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    (
        out.status.success(),
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        ),
    )
}
fn ok(dir: &Path, args: &[&str]) -> String {
    let (pass, out) = cli(dir, args);
    assert!(pass, "atelier {args:?}: {out}");
    out.trim().into()
}
fn commit(dir: &Path, message: &str) {
    git(dir, &["add", "."]);
    git(dir, &["commit", "-qm", message]);
}
/// A `Label: value` row of the status Local State, whatever its padding.
fn field<'a>(output: &'a str, label: &str) -> Option<&'a str> {
    output.lines().find_map(|line| {
        let (name, value) = line.split_once(": ")?;
        (name.trim_end() == label).then_some(value)
    })
}
/// The rows of a status section, between its underline and the next blank line.
fn section<'a>(output: &'a str, title: &str) -> Vec<&'a str> {
    let mut lines = output.lines().skip_while(|line| *line != title).skip(2);
    std::iter::from_fn(|| lines.next().filter(|line| !line.is_empty())).collect()
}
struct Fixture {
    dir: TempDir,
    id: String,
    branch: String,
}
/// Sets `id` done with a Report by editing its record, uncommitted: a done
/// claim made outside `atelier merge`, as an older Atelier's close made one.
fn mark_done(dir: &Path, id: &str) {
    let store = atelier_records::RecordStore::new(dir.join(".atelier"));
    let mut record = store.load_issue_by_id(id).unwrap();
    record.issue.status = "done".into();
    record.issue.closed_at = Some(chrono::Utc::now());
    record.sections.report = Some("completed claim".into());
    store.write_issue_atomic(&record).unwrap();
}
/// The starter policy before atelier-bqbz dropped the `blocked` and `review`
/// statuses; repository policies written from it still declare them.
const STARTER_WITH_BLOCKED_AND_REVIEW: &str =
    include_str!("../../atelier-workflow/fixtures/starter-policy-with-blocked-and-review.yaml");

/// The starter policy before atelier-stnv added `declined` and `decline`;
/// repository policies written from it lack them.
const STARTER_BEFORE_DECLINED: &str =
    include_str!("../../atelier-workflow/fixtures/starter-policy-before-declined.yaml");

impl Fixture {
    fn new() -> Self {
        Self::under(None)
    }
    /// A fixture whose policy is `policy` rather than the `init` starter.
    fn under(policy_text: Option<&str>) -> Self {
        let dir = tempdir().unwrap();
        git(dir.path(), &["init", "-qb", "main"]);
        git(dir.path(), &["config", "user.name", "Author"]);
        git(dir.path(), &["config", "user.email", "author@example.test"]);
        ok(dir.path(), &["init", "--slug", "atelier"]);
        let policy = dir.path().join(".atelier/workflow.yaml");
        if let Some(policy_text) = policy_text {
            fs::write(&policy, policy_text).unwrap();
        }
        let id = ok(
            dir.path(),
            &[
                "-q",
                "issue",
                "create",
                "Session owner",
                "--body",
                "## Goal\n\nShip session behavior.\n\n## Done when\n\nBoundary scenario passes.",
            ],
        );
        commit(dir.path(), "base plan");
        ok(dir.path(), &["start", &id]);
        fs::write(dir.path().join("code"), "tested code\n").unwrap();
        commit(dir.path(), "code and started record");
        let branch = git(dir.path(), &["branch", "--show-current"]);
        Self { dir, id, branch }
    }
    fn path(&self) -> &Path {
        self.dir.path()
    }
    fn store(&self) -> atelier_records::RecordStore {
        atelier_records::RecordStore::new(self.path().join(".atelier"))
    }
    /// Commits `id` as done by editing its record, as an older Atelier's
    /// close or a hand edit would; merge marks work done now (atelier-jnrh).
    fn terminal(&self, id: &str) {
        mark_done(self.path(), id);
        commit(self.path(), "terminal claim");
    }
    fn refusal(&self, expected: &str) {
        let head = git(self.path(), &["rev-parse", "HEAD"]);
        let status = git(self.path(), &["status", "--porcelain"]);
        let (pass, out) = cli(self.path(), &["land", &self.id]);
        assert!(!pass && out.contains(expected), "{out}");
        assert_eq!(git(self.path(), &["rev-parse", "HEAD"]), head);
        assert_eq!(git(self.path(), &["status", "--porcelain"]), status);
        assert_eq!(git(self.path(), &["branch", "--show-current"]), self.branch);
    }
}
#[test]
fn status_and_active_landing_are_fresh_and_read_only() {
    let f = Fixture::new();
    let head = git(f.path(), &["rev-parse", "HEAD"]);
    let status = ok(f.path(), &["status"]);
    assert!(
        status.contains(&f.id) && status.contains("Ship session behavior"),
        "{status}"
    );
    // An owner's session continues the owner instead of choosing ready work,
    // even though the owner itself is open work outside any mission.
    assert!(!status.contains("Ready Work\n"), "{status}");
    assert!(!status.contains("Unassigned Work"), "{status}");
    assert!(!status.contains("Unassigned work:"), "{status}");
    // Static guidance stays out of the view (ADR 0022 Decision 2).
    assert!(
        !status.contains("Guidance:") && !status.contains("Before handing off"),
        "{status}"
    );
    // The deprecated alias prints the same view after one deprecation line.
    let prime = ok(f.path(), &["prime"]);
    let (note, rest) = prime.split_once('\n').unwrap();
    assert!(note.contains("`atelier prime` is deprecated"), "{prime}");
    assert_eq!(rest, status);
    ok(f.path(), &["land"]);
    assert_eq!(git(f.path(), &["rev-parse", "HEAD"]), head);
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
    ok(f.path(), &["issue", "note", &f.id, "Fresh handoff marker"]);
    commit(f.path(), "handoff");
    assert!(ok(f.path(), &["status"]).contains("Fresh handoff marker"));
}
#[test]
fn status_preserves_native_branch_trimming_for_header_owner_and_orientation() {
    for whitespace in ['\u{a0}', '\u{2003}'] {
        for trailing in [true, false] {
            let f = Fixture::new();
            let branch = if trailing {
                format!("name{whitespace}")
            } else {
                format!("{whitespace}name")
            };
            let mut record = f.store().load_issue_by_id(&f.id).unwrap();
            record.issue.workflow_branch.as_mut().unwrap().work_branch = branch.clone();
            f.store().write_issue_atomic(&record).unwrap();
            commit(f.path(), "Unicode owner branch");
            git(f.path(), &["branch", &branch]);
            git(
                f.path(),
                &["symbolic-ref", "HEAD", &format!("refs/heads/{branch}")],
            );
            let head = git(f.path(), &["rev-parse", "HEAD"]);
            let native = git(f.path(), &["symbolic-ref", "--quiet", "HEAD"]);
            let normalized = native.strip_prefix("refs/heads/").unwrap();
            let stored = f.store().load_issue_by_id(&f.id).unwrap();
            let status = ok(f.path(), &["status"]);
            assert_eq!(
                field(&status, "Branch"),
                Some(format!("{normalized} @ {}", &head[..8]).as_str())
            );
            let owner = field(&status, "Owner").unwrap();
            if stored.issue.workflow_branch.unwrap().work_branch == normalized {
                assert!(owner.starts_with(&f.id), "{status}");
            } else {
                assert_eq!(owner, "none", "{status}");
            }
            let unstarted = status
                .lines()
                .find_map(|line| line.strip_prefix("Unowned: "))
                .unwrap_or_default();
            assert_eq!(
                unstarted.split(", ").any(|listed| listed == branch),
                trailing,
                "{status}"
            );
        }
    }
}
#[test]
fn canonical_changes_refuse_landing_in_every_git_state() {
    for state in ["unstaged", "staged", "deleted", "untracked", "policy"] {
        let f = Fixture::new();
        let path = f.path().join(format!(".atelier/issues/{}.md", f.id));
        match state {
            "deleted" => fs::remove_file(&path).unwrap(),
            "untracked" => {
                fs::copy(&path, f.path().join(".atelier/issues/new.md")).unwrap();
            }
            "policy" => {
                fs::write(f.path().join(".atelier/workflow.yaml"), "invalid: [").unwrap();
            }
            _ => {
                ok(f.path(), &["issue", "note", &f.id, "Uncommitted"]);
                if state == "staged" {
                    git(f.path(), &["add", "."]);
                }
            }
        }
        f.refusal("land_records_uncommitted");
    }
}
#[test]
fn done_owner_requires_approval_and_active_failures_can_handoff() {
    let f = Fixture::new();
    ok(
        f.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{}", f.id),
            "--kind",
            "test",
            "--result",
            "fail",
            "Work in progress failure",
        ],
    );
    ok(
        f.path(),
        &[
            "review",
            "finding",
            &f.id,
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "Pending active triage",
        ],
    );
    commit(f.path(), "unfinished findings and proof");
    let status = ok(f.path(), &["status"]);
    assert!(status.contains("Pending active triage"), "{status}");
    ok(f.path(), &["land"]);
    f.terminal(&f.id);
    f.refusal("review.current");
}

/// The Stop hook's answer to work in progress on a work branch, which it
/// reports without blocking a turn (atelier-md0v): `out` is land's report.
fn in_progress_message(out: &str) -> serde_json::Value {
    serde_json::json!({"systemMessage": format!(
        "Atelier: work in progress; land reports it and refuses nothing on a work branch.\n{}",
        out.trim_end()
    )})
}
fn hook(dir: &Path, command: &str, payload: serde_json::Value) -> serde_json::Value {
    hook_input(dir, command, &serde_json::to_string(&payload).unwrap())
}
/// A hook run on `input` as Claude Code would send it, valid JSON or not.
fn hook_input(dir: &Path, command: &str, input: &str) -> serde_json::Value {
    let mut child = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir)
        .args([command, "--hook", "claude"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}
#[test]
fn documented_hook_events_use_nested_cwd_and_stop_loop_protection() {
    let f = Fixture::new();
    let unrelated = tempdir().unwrap();
    let nested = f.path().join("nested");
    fs::create_dir(&nested).unwrap();
    for source in ["startup", "resume", "clear", "compact", "fork"] {
        let result = hook(
            unrelated.path(),
            "status",
            serde_json::json!({"session_id":"session", "transcript_path":"unused", "cwd":nested, "hook_event_name":"SessionStart", "source":source}),
        );
        assert_eq!(
            result["hookSpecificOutput"]["hookEventName"],
            "SessionStart"
        );
        assert!(result["hookSpecificOutput"]["additionalContext"]
            .as_str()
            .unwrap()
            .contains(&f.id));
    }
    let input = serde_json::json!({"session_id":"session", "cwd":nested, "hook_event_name":"Stop", "stop_hook_active":false, "last_assistant_message":"Work handoff"});
    assert_eq!(
        hook(unrelated.path(), "land", input.clone()),
        serde_json::json!({})
    );
    // Work in progress on the work branch is reported, not blocked.
    ok(f.path(), &["issue", "note", &f.id, "Uncommitted handoff"]);
    let result = hook(unrelated.path(), "land", input.clone());
    assert!(result.get("decision").is_none(), "{result}");
    let message = result["systemMessage"].as_str().unwrap();
    assert!(
        message.contains("land_records_uncommitted")
            && message
                .ends_with("\nland: work in progress — 1 check open (land_records_uncommitted)"),
        "{message}"
    );
    // On the base, a refusal blocks once, and a repeated stop is not
    // blocked again.
    git(f.path(), &["checkout", "-q", "--", "."]);
    git(f.path(), &["switch", "-q", "main"]);
    let policy = f.path().join(".atelier/workflow.yaml");
    let valid = fs::read_to_string(&policy).unwrap();
    fs::write(&policy, format!("{valid}garbage: [\n")).unwrap();
    commit(f.path(), "broken policy");
    let result = hook(unrelated.path(), "land", input.clone());
    assert_eq!(result["decision"], "block", "{result}");
    let mut repeated = input;
    repeated["stop_hook_active"] = true.into();
    let result = hook(unrelated.path(), "land", repeated);
    assert!(result.get("decision").is_none());
    assert!(result["systemMessage"]
        .as_str()
        .unwrap()
        .contains("still fails"));
    let invalid = hook(
        unrelated.path(),
        "status",
        serde_json::json!({"cwd":nested, "hook_event_name":"Stop"}),
    );
    assert_eq!(
        invalid["hookSpecificOutput"]["additionalContext"],
        "Atelier hook misconfigured: expected SessionStart hook payload; check .claude/settings.json"
    );
    // Adopters' installed hooks still call the deprecated alias.
    let alias = hook(
        unrelated.path(),
        "prime",
        serde_json::json!({"cwd":nested, "hook_event_name":"SessionStart", "source":"startup"}),
    );
    let context = alias["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .unwrap();
    assert!(
        context.starts_with("`atelier prime` is deprecated") && context.contains(&f.id),
        "{context}"
    );
}
#[test]
fn status_shows_policy_fallback_diagnostics_in_the_page_and_hook_context() {
    let f = Fixture::new();
    let mut policy = fs::OpenOptions::new()
        .append(true)
        .open(f.path().join(".atelier/workflow.yaml"))
        .unwrap();
    policy.write_all(b"garbage: [\n").unwrap();
    let context = hook(
        f.path(),
        "status",
        serde_json::json!({"cwd":f.path(), "hook_event_name":"SessionStart", "source":"startup"}),
    );
    let context = context["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .unwrap();
    assert!(
        context.contains("Invalid working policy")
            && context.contains("Using last valid policy from")
            && context.contains(&f.id),
        "{context}"
    );
    // Plain status prints them once, in the page rather than also on stderr.
    let out = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(f.path())
        .arg("status")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success() && stdout.contains("Policy Diagnostics"),
        "{stdout}"
    );
    assert_eq!(
        stdout.matches("Invalid working policy").count(),
        1,
        "{stdout}"
    );
    assert!(
        !String::from_utf8_lossy(&out.stderr).contains("Invalid working policy"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    // Quiet status keeps stdout to the summary and reports the fallback on stderr.
    for command in ["status", "prime"] {
        let out = Command::new(env!("CARGO_BIN_EXE_atelier"))
            .current_dir(f.path())
            .args(["-q", command])
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(out.status.success(), "{command}: {stderr}");
        assert!(
            stdout.trim().starts_with("work=") && stdout.trim().lines().count() == 1,
            "{command}: {stdout}"
        );
        assert!(
            stderr.contains("Invalid working policy")
                && stderr.contains("Using last valid policy from"),
            "{command}: {stderr}"
        );
    }
}
#[test]
fn land_and_merge_refuse_a_project_config_that_differs_from_head() {
    let f = Fixture::new();
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    let config = f.path().join(".atelier/config.toml");
    let committed = fs::read_to_string(&config).unwrap();
    // A custom link type in an uncommitted config would let records that the
    // committed config rejects pass the gates.
    fs::write(
        &config,
        format!("{committed}\n[issue_links]\ncustom_context_types = [\"inspired_by\"]\n"),
    )
    .unwrap();
    for ignored in [false, true] {
        if ignored {
            fs::write(f.path().join(".git/info/exclude"), ".atelier/config.toml\n").unwrap();
            git(f.path(), &["rm", "-q", "--cached", ".atelier/config.toml"]);
            git(f.path(), &["commit", "-qm", "stop tracking the config"]);
        }
        let (pass, output) = cli(f.path(), &["land", &f.id]);
        assert!(
            !pass && output.contains("land_config_uncommitted"),
            "{output}"
        );
        // merge refuses a modified tracked config as a dirty checkout first.
        let (pass, output) = cli(f.path(), &["merge", &f.id]);
        let expected = if ignored {
            "merge_config_uncommitted"
        } else {
            "merge_checkout_dirty"
        };
        assert!(!pass && output.contains(expected), "{output}");
    }
    git(f.path(), &["reset", "-q", "--hard", "HEAD~1"]);
    git(f.path(), &["restore", ".atelier/config.toml"]);
    ok(f.path(), &["land", &f.id]);
}
#[test]
#[cfg(unix)]
fn setup_keeps_an_existing_settings_mode_and_creates_new_ones_readable() {
    use std::os::unix::fs::PermissionsExt;
    let f = Fixture::new();
    let directory = f.path().join(".claude");
    fs::create_dir(&directory).unwrap();
    let settings = directory.join("settings.json");
    let mode = |path: &Path| fs::metadata(path).unwrap().permissions().mode() & 0o777;
    ok(f.path(), &["setup", "claude"]);
    assert_eq!(mode(&settings), 0o644);
    fs::write(&settings, "{}\n").unwrap();
    fs::set_permissions(&settings, fs::Permissions::from_mode(0o664)).unwrap();
    ok(f.path(), &["setup", "claude"]);
    assert_eq!(mode(&settings), 0o664);
}
#[test]
fn setup_warns_when_claude_md_keeps_agents_md_from_loading() {
    let f = Fixture::new();
    let warning = "Warning: CLAUDE.md does not import @AGENTS.md";
    // Without a CLAUDE.md, Claude Code reads AGENTS.md itself.
    fs::write(f.path().join("AGENTS.md"), "Use Atelier.\n").unwrap();
    let out = ok(f.path(), &["setup", "claude"]);
    assert!(!out.contains(warning), "{out}");
    fs::write(f.path().join("CLAUDE.md"), "Project notes.\n").unwrap();
    let (pass, out) = cli(f.path(), &["setup", "claude"]);
    assert!(
        pass && out.contains(warning)
            && out.contains("Add a line containing @AGENTS.md to CLAUDE.md"),
        "{out}"
    );
    // Warnings go to stderr, so quiet output stays the settings path.
    let quiet = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(f.path())
        .args(["-q", "setup", "claude"])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&quiet.stderr).contains(warning));
    assert!(!String::from_utf8_lossy(&quiet.stdout).contains(warning));
    fs::write(f.path().join("CLAUDE.md"), "Project notes.\n\n@AGENTS.md\n").unwrap();
    let out = ok(f.path(), &["setup", "claude"]);
    assert!(!out.contains(warning), "{out}");
    // Claude Code loads every project memory file, so one import suffices.
    fs::write(f.path().join(".claude/CLAUDE.md"), "Project notes.\n").unwrap();
    let out = ok(f.path(), &["setup", "claude"]);
    assert!(!out.contains("does not import"), "{out}");
    // The project memory may also live only in .claude/CLAUDE.md.
    fs::remove_file(f.path().join("CLAUDE.md")).unwrap();
    let out = ok(f.path(), &["setup", "claude"]);
    assert!(
        out.contains("Warning: .claude/CLAUDE.md does not import @AGENTS.md"),
        "{out}"
    );
    // Neither memory file importing it names both.
    fs::write(f.path().join("CLAUDE.md"), "Project notes.\n").unwrap();
    let out = ok(f.path(), &["setup", "claude"]);
    assert!(
        out.contains("Warning: CLAUDE.md and .claude/CLAUDE.md do not import @AGENTS.md")
            && out.contains("Add a line containing @AGENTS.md to CLAUDE.md."),
        "{out}"
    );
}
#[test]
fn setup_is_atomic_idempotent_and_preserves_other_handlers() {
    let f = Fixture::new();
    let directory = f.path().join(".claude");
    fs::create_dir(&directory).unwrap();
    let settings = directory.join("settings.json");
    let other = serde_json::json!({"matcher":"resume", "hooks":[{"type":"command", "command":"printf custom", "timeout":3}]});
    fs::write(&settings, serde_json::to_vec(&serde_json::json!({"permissions":{"deny":["Bash(rm *)"]}, "hooks":{"SessionStart":[other], "PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"guard"}]}]}})).unwrap()).unwrap();
    let binary_dir = f.path().join("bin's directory");
    fs::create_dir(&binary_dir).unwrap();
    let binary = binary_dir.join("atelier");
    fs::copy(env!("CARGO_BIN_EXE_atelier"), &binary).unwrap();
    ok(
        f.path(),
        &["setup", "claude", "--binary", binary.to_str().unwrap()],
    );
    let once = fs::read(&settings).unwrap();
    ok(
        f.path(),
        &["setup", "claude", "--binary", binary.to_str().unwrap()],
    );
    assert_eq!(fs::read(&settings).unwrap(), once);
    let parsed: serde_json::Value = serde_json::from_slice(&once).unwrap();
    assert_eq!(parsed["hooks"]["SessionStart"][0], other);
    assert_eq!(parsed["permissions"]["deny"][0], "Bash(rm *)");
    assert_eq!(
        parsed["hooks"]["PreToolUse"][0]["hooks"][0]["command"],
        "guard"
    );
    for (event, action) in [("SessionStart", "status"), ("Stop", "land")] {
        let groups = parsed["hooks"][event].as_array().unwrap();
        let handler = groups.last().unwrap();
        assert!(handler.get("matcher").is_none());
        assert_eq!(handler["hooks"][0]["type"], "command");
        assert!(handler["hooks"][0]["command"]
            .as_str()
            .unwrap()
            .ends_with(&format!(
                " {action} --hook claude # atelier-managed:{event}"
            )));
    }
    // Execute the installed quoted command, proving paths with spaces/quotes.
    let command = parsed["hooks"]["SessionStart"][1]["hooks"][0]["command"]
        .as_str()
        .unwrap();
    let mut child = Command::new("sh")
        .args(["-c", command])
        .current_dir(f.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(serde_json::to_string(&serde_json::json!({"hook_event_name":"SessionStart", "cwd":f.path(), "source":"startup"})).unwrap().as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    assert!(
        serde_json::from_slice::<serde_json::Value>(&out.stdout).unwrap()["hookSpecificOutput"]
            ["additionalContext"]
            .as_str()
            .unwrap()
            .contains(&f.id)
    );
    fs::write(&settings, "invalid: [").unwrap();
    let before = fs::read(&settings).unwrap();
    assert!(!cli(f.path(), &["setup", "claude"]).0);
    assert_eq!(fs::read(&settings).unwrap(), before);
    assert!(!cli(f.path(), &["setup", "codex"]).0);
}
#[test]
fn ignored_canonical_files_still_refuse_landing() {
    let f = Fixture::new();
    fs::write(f.path().join(".git/info/exclude"), ".atelier/reviews/\n").unwrap();
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Ignored review is not committed",
        ],
    );
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
    f.refusal("land_records_uncommitted");
}
#[test]
fn cross_branch_status_keeps_base_clean_and_reports_legacy_without_loading_it() {
    let f = Fixture::new();
    let record = f.store().load_issue_by_id(&f.id).unwrap();
    git(f.path(), &["tag", &f.branch, "main"]);
    git(f.path(), &["switch", "main"]);
    let head = git(f.path(), &["rev-parse", "HEAD"]);
    let output = ok(f.path(), &["status"]);
    assert!(
        output.contains(&format!(
            "{}  {} in_progress, ⚠ not approved, behind 0, merges cleanly  ",
            f.branch, f.id
        )) && output.contains("\nBranches\n--------\n"),
        "{output}"
    );
    // The branch rows come before the footer, which is the last section.
    let (body, footer) = output
        .rsplit_once("\n\nNext\n----\n")
        .unwrap_or_else(|| panic!("no Next section:\n{output}"));
    assert!(body.contains("\nBranches\n--------\n"), "{output}");
    assert!(
        footer.lines().all(|line| line.starts_with("  ")),
        "{output}"
    );
    assert_eq!(
        f.store().load_issue_by_id(&f.id).unwrap().issue.status,
        "todo"
    );
    assert_eq!(git(f.path(), &["rev-parse", "HEAD"]), head);
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
    // An invalid legacy record cannot hide valid sibling summaries.
    git(f.path(), &["switch", "-c", "epic/legacy"]);
    fs::write(
        f.path().join(".atelier/workflow.yaml"),
        "schema_version: 1\nlegacy: true\n",
    )
    .unwrap();
    fs::write(f.path().join(".atelier/issues/legacy.md"), "not a record\n").unwrap();
    commit(f.path(), "legacy branch");
    git(f.path(), &["switch", "main"]);
    let output = ok(f.path(), &["status"]);
    assert!(
        output.contains("epic/legacy: unavailable") && output.contains(&record.issue.title),
        "{output}"
    );
}
#[test]
fn start_existing_remote_branch_uses_its_tip_and_target_records() {
    let f = Fixture::new();
    let remote_head = git(f.path(), &["rev-parse", "HEAD"]);
    git(f.path(), &["switch", "main"]);
    let remote = tempdir().unwrap();
    git(remote.path(), &["init", "--bare", "-q"]);
    git(
        f.path(),
        &["remote", "add", "origin", remote.path().to_str().unwrap()],
    );
    git(
        f.path(),
        &[
            "update-ref",
            &format!("refs/remotes/origin/{}", f.branch),
            &remote_head,
        ],
    );
    git(f.path(), &["branch", "-D", &f.branch]);
    ok(f.path(), &["start", &f.id]);
    assert_eq!(git(f.path(), &["rev-parse", "HEAD"]), remote_head);
    assert_eq!(
        fs::read_to_string(f.path().join("code")).unwrap(),
        "tested code\n"
    );
    assert_eq!(
        f.store().load_issue_by_id(&f.id).unwrap().issue.status,
        "in_progress"
    );
    assert_eq!(
        git(
            f.path(),
            &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"]
        ),
        format!("origin/{}", f.branch)
    );
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
}
#[test]
fn ambiguous_remote_branch_refuses_before_switching_or_writing() {
    let f = Fixture::new();
    let tip = git(f.path(), &["rev-parse", "HEAD"]);
    git(f.path(), &["switch", "main"]);
    git(f.path(), &["branch", "-D", &f.branch]);
    for remote in ["origin", "other"] {
        git(
            f.path(),
            &[
                "update-ref",
                &format!("refs/remotes/{remote}/{}", f.branch),
                &tip,
            ],
        );
    }
    let status = git(f.path(), &["status", "--porcelain"]);
    let (pass, out) = cli(f.path(), &["start", &f.id]);
    assert!(!pass && out.contains("start_remote_ambiguous"), "{out}");
    assert_eq!(git(f.path(), &["branch", "--show-current"]), "main");
    assert_eq!(git(f.path(), &["status", "--porcelain"]), status);
}

fn epic_child(f: &Fixture) -> String {
    let child = ok(f.path(), &["-q", "issue", "create", "Completed child"]);
    let mut owner = f.store().load_issue_by_id(&f.id).unwrap();
    owner.issue.issue_type = "feature".into();
    owner.sections.change_type("feature");
    owner
        .relationships
        .children
        .push(atelier_records::issue_relationship_target(&child));
    f.store().write_issue_atomic(&owner).unwrap();
    commit(f.path(), "epic and child plan");
    child
}
#[test]
fn active_epic_hands_off_completed_children_without_batch_approval() {
    let f = Fixture::new();
    let child = epic_child(&f);
    f.terminal(&child);
    ok(
        f.path(),
        &[
            "review",
            "finding",
            &f.id,
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "Active epic triage",
        ],
    );
    commit(f.path(), "pending epic triage");
    ok(f.path(), &["land", &child]);
    fs::write(f.path().join("code"), "ongoing epic implementation\n").unwrap();
    ok(f.path(), &["land"]); // In-progress code is a valid honest handoff.
    git(f.path(), &["restore", "code"]);
    ok(
        f.path(),
        &[
            "review",
            "finding",
            &child,
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "Completed child contract is wrong",
        ],
    );
    commit(f.path(), "finding on done child");
    f.refusal("findings.disposed");
}
/// No gate reads receipts (atelier-md0v): a fail receipt with no pass after
/// it, on a done child and on the owner, blocks neither merge nor land, on
/// the branch or on the base.
#[test]
fn a_fail_receipt_blocks_neither_merge_nor_land() {
    let f = Fixture::new();
    let child = epic_child(&f);
    f.terminal(&child);
    for target in [&child, &f.id] {
        ok(
            f.path(),
            &[
                "evidence",
                "record",
                "--target",
                &format!("issue/{target}"),
                "--kind",
                "test",
                "--result",
                "fail",
                "Proof failed",
            ],
        );
    }
    commit(f.path(), "failing proof");
    let mut owner = f.store().load_issue_by_id(&f.id).unwrap();
    owner.sections.report = Some("complete".into());
    f.store().write_issue_atomic(&owner).unwrap();
    commit(f.path(), "implementer report");
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    let landed = ok(f.path(), &["land"]);
    assert!(
        landed.ends_with("land: passed") && !landed.contains("evidence"),
        "{landed}"
    );
    let checked = ok(f.path(), &["merge", &f.id, "--check"]);
    assert!(!checked.contains("evidence"), "{checked}");
    ok(f.path(), &["merge", &f.id]);
    let landed = ok(f.path(), &["land"]);
    assert!(landed.ends_with("land: passed"), "{landed}");
    ok(f.path(), &["doctor"]);
}
#[test]
fn done_child_cannot_hide_open_dependencies_or_missing_required_fields() {
    for violation in ["dependency", "required_field"] {
        // Work done claims answer to required fields only under an older
        // policy whose work `close` set them (atelier-jnrh).
        let f = if violation == "required_field" {
            Fixture::under(Some(STARTER_WITH_BLOCKED_AND_REVIEW))
        } else {
            Fixture::new()
        };
        let child = epic_child(&f);
        f.terminal(&child);
        match violation {
            "dependency" => {
                let blocker = ok(f.path(), &["-q", "issue", "create", "Open blocker"]);
                let mut record = f.store().load_issue_by_id(&blocker).unwrap();
                record
                    .relationships
                    .blocks
                    .push(atelier_records::issue_relationship_target(&child));
                f.store().write_issue_atomic(&record).unwrap();
            }
            _ => {
                let mut record = f.store().load_issue_by_id(&child).unwrap();
                record.sections.report = None;
                f.store().write_issue_atomic(&record).unwrap();
            }
        }
        commit(f.path(), "invalid done child");
        f.refusal(match violation {
            "dependency" => "blockers.transitive_none_open",
            _ => "required_field.close_reason",
        });
    }
}
#[test]
fn trusted_base_child_checks_cannot_be_removed_on_candidate() {
    // An older policy whose work `close` runs a check this child cannot
    // pass; merge marks work done under the current one (atelier-jnrh), and
    // the base's checks, not the candidate's, judge the done claim.
    let f = Fixture::under(Some(STARTER_WITH_BLOCKED_AND_REVIEW));
    git(f.path(), &["switch", "main"]);
    let path = f.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        policy.replace(
            "        - blockers.none_open\n",
            "        - blockers.none_open\n        - decision.recorded\n",
        ),
    )
    .unwrap();
    commit(f.path(), "base adds a completion check");
    git(f.path(), &["switch", &f.branch]);
    git(f.path(), &["rebase", "main"]);
    fs::write(&path, policy).unwrap();
    let child = epic_child(&f);
    f.terminal(&child);
    f.refusal("decision.recorded");
}
#[test]
fn done_owner_full_gates_and_invalid_policies_are_checked_without_writes() {
    let f = Fixture::new();
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    let head = git(f.path(), &["rev-parse", "HEAD"]);
    ok(f.path(), &["land"]);
    assert_eq!(git(f.path(), &["rev-parse", "HEAD"]), head);
    fs::write(f.path().join("code"), "unreviewed bytes\n").unwrap();
    f.refusal("land_completion_dirty");
    git(f.path(), &["restore", "code"]);
    fs::write(f.path().join(".atelier/workflow.yaml"), "invalid: [").unwrap();
    commit(f.path(), "invalid policy");
    f.refusal(
        "workflow_config_parse_error: .atelier/workflow.yaml: did not find expected node content",
    );
}
#[test]
fn hidden_tracked_record_edits_and_executable_modes_refuse_landing() {
    let f = Fixture::new();
    let relative = format!(".atelier/issues/{}.md", f.id);
    git(f.path(), &["update-index", "--assume-unchanged", &relative]);
    let path = f.path().join(&relative);
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        text.replace("Ship session behavior", "Uncommitted acceptance"),
    )
    .unwrap();
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
    f.refusal("land_records_uncommitted");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let f = Fixture::new();
        let path = f.path().join(format!(".atelier/issues/{}.md", f.id));
        git(f.path(), &["config", "core.filemode", "false"]);
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
        f.refusal("land_records_uncommitted");
    }
}
#[test]
fn start_child_installs_owner_metadata_and_child_transition_together() {
    let f = Fixture::new();
    git(f.path(), &["switch", "main"]);
    // Without the fixture's branch, whose own record names it the owner's,
    // start creates the owner's branch from main.
    git(f.path(), &["branch", "-qD", &f.branch]);
    let child = epic_child(&f);
    ok(f.path(), &["start", &child]);
    let owner = f.store().load_issue_by_id(&f.id).unwrap();
    let child = f.store().load_issue_by_id(&child).unwrap();
    assert!(owner.issue.workflow_branch.is_some());
    assert_eq!(child.issue.status, "in_progress");
    assert_eq!(git(f.path(), &["status", "--porcelain"]).lines().count(), 2);
}
#[test]
fn status_owner_block_shows_base_mission_strategy_and_eight_newest_reviews() {
    let f = Fixture::new();
    fs::create_dir_all(f.path().join("docs/strategies")).unwrap();
    fs::write(
        f.path().join("docs/strategies/plan.md"),
        "---\nstatus: draft\n---\n\n# Plan\n",
    )
    .unwrap();
    let mission = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "Ship sessions",
            "--issue-type",
            "mission",
            "--strategy",
            "docs/strategies/plan.md",
        ],
    );
    ok(
        f.path(),
        &["issue", "link", &f.id, &mission, "--role", "advances"],
    );
    commit(f.path(), "mission with a strategy");
    for n in 0..10 {
        ok(
            f.path(),
            &[
                "review",
                "record",
                &f.id,
                "--identity",
                "Reviewer",
                &format!("Checked pass {n}"),
            ],
        );
    }
    commit(f.path(), "ten reviews");
    let status = ok(f.path(), &["status"]);
    assert_eq!(field(&status, "Integration base"), Some("main"), "{status}");
    assert!(
        field(&status, "Mission")
            .is_some_and(|row| row == format!("{mission} draft medium Ship sessions")),
        "{status}"
    );
    assert_eq!(
        field(&status, "Strategy"),
        Some("docs/strategies/plan.md"),
        "{status}"
    );
    // A row without blockers names none.
    assert_eq!(
        section(&status, "Owner Work"),
        [format!("{} in_progress medium Session owner", f.id)],
        "{status}"
    );
    let reviews = section(&status, "Reviews");
    assert_eq!(reviews.len(), 8, "{status}");
    for (row, n) in reviews.iter().zip((2..10).rev()) {
        assert!(
            row.contains(&format!("  on {}  @", f.id))
                && row.ends_with(&format!("Checked pass {n}")),
            "{status}"
        );
    }
}
#[test]
fn status_branch_row_shows_upstream_divergence_only_when_it_differs() {
    let f = Fixture::new();
    let branch_row = |f: &Fixture| {
        field(&ok(f.path(), &["status"]), "Branch")
            .unwrap()
            .to_owned()
    };
    let short = |f: &Fixture| git(f.path(), &["rev-parse", "--short=8", "HEAD"]);
    // No upstream: no marker.
    assert_eq!(branch_row(&f), format!("{} @ {}", f.branch, short(&f)));
    let remote = tempdir().unwrap();
    git(remote.path(), &["init", "--bare", "-q"]);
    git(
        f.path(),
        &["remote", "add", "origin", remote.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", &f.branch]);
    // An upstream that matches: still no marker.
    assert_eq!(branch_row(&f), format!("{} @ {}", f.branch, short(&f)));
    fs::write(f.path().join("code"), "more code\n").unwrap();
    commit(f.path(), "ahead of upstream");
    assert_eq!(
        branch_row(&f),
        format!(
            "{} @ {} (ahead 1 of origin/{})",
            f.branch,
            short(&f),
            f.branch
        )
    );
    git(f.path(), &["reset", "-q", "--hard", "HEAD~2"]);
    assert_eq!(
        branch_row(&f),
        format!(
            "{} @ {} (behind 1 of origin/{})",
            f.branch,
            short(&f),
            f.branch
        )
    );
}
#[test]
fn literal_checkout_branch_is_not_confused_with_same_named_tags() {
    let f = Fixture::new();
    git(f.path(), &["tag", &f.branch, "main"]);
    git(f.path(), &["tag", "main", "HEAD"]);
    let status = ok(f.path(), &["status"]);
    assert!(
        field(&status, "Branch")
            .is_some_and(|branch| branch.starts_with(&format!("{} @", f.branch))),
        "{status}"
    );
    ok(f.path(), &["start", &f.id]);
    ok(f.path(), &["land"]);
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Review under shadow tags",
        ],
    );
    commit(f.path(), "approval");
    ok(f.path(), &["land"]);
    assert_eq!(
        git(f.path(), &["symbolic-ref", "HEAD"]),
        format!("refs/heads/{}", f.branch)
    );
}
#[test]
fn custom_branch_only_owner_is_discovered_without_overlaying_base() {
    let f = Fixture::new();
    git(f.path(), &["switch", "main"]);
    git(f.path(), &["switch", "-c", "custom/ship-session"]);
    let id = ok(f.path(), &["-q", "issue", "create", "Branch-only owner"]);
    let mut record = f.store().load_issue_by_id(&id).unwrap();
    record.issue.status = "in_progress".into();
    record.issue.workflow_branch = Some(atelier_core::WorkflowBranch {
        work_branch: "custom/ship-session".into(),
        branch_base: Some("main".into()),
    });
    f.store().write_issue_atomic(&record).unwrap();
    commit(f.path(), "custom started branch-only work");
    git(f.path(), &["switch", "main"]);
    let output = ok(f.path(), &["status"]);
    assert!(
        output.contains(&format!(
            "custom/ship-session  {id} in_progress, ⚠ not approved, behind 0, merges cleanly  "
        )),
        "{output}"
    );
    assert!(f.store().load_issue_by_id(&id).is_err());
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
}
#[test]
fn active_owner_skips_merge_shell_checks_but_terminal_owner_enforces_them() {
    let f = Fixture::new();
    git(f.path(), &["switch", "main"]);
    let path = f.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        format!("{policy}\nmerge_checks:\n  - shell.check: {{ command: 'false' }}\n"),
    )
    .unwrap();
    commit(f.path(), "base merge check");
    git(f.path(), &["switch", &f.branch]);
    git(f.path(), &["rebase", "main"]);
    ok(f.path(), &["land"]);
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Review completed owner",
        ],
    );
    commit(f.path(), "approval");
    f.refusal("shell.check");
}
#[test]
fn invalid_base_policy_fallback_is_visible_while_candidate_stays_strict() {
    let f = Fixture::new();
    git(f.path(), &["switch", "main"]);
    let path = f.path().join(".atelier/workflow.yaml");
    let valid = fs::read_to_string(&path).unwrap();
    fs::write(&path, "invalid: [").unwrap();
    commit(f.path(), "malformed base policy");
    git(f.path(), &["switch", &f.branch]);
    git(f.path(), &["merge", "--no-edit", "main"]);
    fs::write(&path, valid).unwrap();
    commit(f.path(), "valid candidate repair");
    let output = ok(f.path(), &["land"]);
    assert!(
        output.contains("Invalid policy at") && output.contains("Using last valid policy"),
        "{output}"
    );
}

#[test]
fn hidden_dirty_code_is_an_honest_active_handoff_but_not_a_completion() {
    for flag in ["--assume-unchanged", "--skip-worktree"] {
        let f = Fixture::new();
        git(f.path(), &["update-index", flag, "code"]);
        fs::write(f.path().join("code"), "unfinished hidden change\n").unwrap();
        assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
        let output = ok(f.path(), &["land"]);
        assert!(output.contains("1 uncommitted path"), "{output}");
        f.terminal(&f.id);
        f.refusal("land_completion_dirty");
    }
}
#[test]
fn completed_owner_shell_cannot_hide_code_changes_from_landing_fence() {
    let f = Fixture::new();
    git(f.path(), &["switch", "main"]);
    let path = f.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!("{policy}\nmerge_checks:\n  - shell.check: {{ command: 'git update-index --assume-unchanged code && echo mutated > code' }}\n")).unwrap();
    commit(f.path(), "trusted mutating check");
    git(f.path(), &["switch", &f.branch]);
    git(f.path(), &["rebase", "main"]);
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    let head = git(f.path(), &["rev-parse", "HEAD"]);
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(!pass && output.contains("land_state_changed"), "{output}");
    assert_eq!(git(f.path(), &["rev-parse", "HEAD"]), head);
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
    assert_eq!(
        fs::read_to_string(f.path().join("code")).unwrap(),
        "mutated\n"
    );
}

/// `start` leaves an untracked file outside the tracker in place, as merge
/// does, but refuses an untracked record, which would join the started
/// branch's records, and an untracked file the branch it switches to holds,
/// each before anything changes (atelier-l457).
#[test]
fn start_leaves_untracked_files_in_place_unless_they_are_records_or_in_the_way() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    fs::write(f.path().join("scratch.txt"), "notes\n").unwrap();
    let unchanged = |f: &Fixture, status: &str| {
        assert_eq!(git(f.path(), &["branch", "--show-current"]), "main");
        assert_eq!(git(f.path(), &["status", "--porcelain"]), status);
    };

    // An untracked record refuses, with a stash that sets it aside.
    let other = ok(f.path(), &["-q", "issue", "create", "Other"]);
    let status = git(f.path(), &["status", "--porcelain"]);
    let (pass, out) = cli(f.path(), &["start", &other]);
    assert!(
        !pass
            && out.contains("start_checkout_dirty")
            && out.contains(&format!(".atelier/issues/{other}.md"))
            && !out.contains("scratch.txt")
            && out.contains("`git stash push --include-untracked`"),
        "{out}"
    );
    unchanged(&f, &status);

    // Committed, the item starts, and the scratch file comes along.
    git(f.path(), &["add", ".atelier"]);
    git(f.path(), &["commit", "-qm", "plan the other item"]);
    ok(f.path(), &["start", &other]);
    assert_eq!(git(f.path(), &["branch", "--show-current"]), other);
    assert_eq!(
        fs::read_to_string(f.path().join("scratch.txt")).unwrap(),
        "notes\n"
    );
    git(f.path(), &["add", ".atelier"]);
    git(f.path(), &["commit", "-qm", "start the other item"]);
    git(f.path(), &["switch", "-q", "main"]);

    // Switching to an existing branch that holds a file at an untracked
    // path would overwrite it, so start refuses first.
    fs::write(f.path().join("code"), "mine\n").unwrap();
    let status = git(f.path(), &["status", "--porcelain"]);
    let (pass, out) = cli(f.path(), &["start", &f.id]);
    assert!(
        !pass
            && out.contains(&format!(
                "start_untracked_overwritten: switching to branch '{}'",
                f.branch
            ))
            && out.contains("\ncode\n"),
        "{out}"
    );
    unchanged(&f, &status);
    assert_eq!(fs::read_to_string(f.path().join("code")).unwrap(), "mine\n");
    fs::remove_file(f.path().join("code")).unwrap();
    ok(f.path(), &["start", &f.id]);
    assert_eq!(git(f.path(), &["branch", "--show-current"]), f.branch);
    assert!(f.path().join("scratch.txt").exists());
}

#[test]
fn start_cannot_switch_away_from_hidden_modified_code() {
    let f = Fixture::new();
    git(f.path(), &["switch", "main"]);
    fs::write(f.path().join("base-code"), "base\n").unwrap();
    commit(f.path(), "base code");
    git(f.path(), &["update-index", "--skip-worktree", "base-code"]);
    fs::write(f.path().join("base-code"), "hidden unfinished change\n").unwrap();
    let (pass, output) = cli(f.path(), &["start", &f.id]);
    assert!(!pass && output.contains("start_checkout_dirty"), "{output}");
    assert_eq!(git(f.path(), &["symbolic-ref", "HEAD"]), "refs/heads/main");
    assert_eq!(
        fs::read_to_string(f.path().join("base-code")).unwrap(),
        "hidden unfinished change\n"
    );
}

#[cfg(unix)]
#[test]
fn execute_bits_hidden_by_core_filemode_follow_git_status_in_land() {
    use std::os::unix::fs::PermissionsExt;
    // core.filemode=false: a local chmod is not an uncommitted change.
    let f = Fixture::new();
    git(f.path(), &["config", "core.filemode", "false"]);
    fs::set_permissions(f.path().join("code"), fs::Permissions::from_mode(0o755)).unwrap();
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
    assert!(!ok(f.path(), &["land"]).contains("uncommitted path"));
    f.terminal(&f.id);
    // The claim is still unreviewed, so land refuses on review, not on a
    // dirty checkout.
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass && output.contains("review.current") && !output.contains("land_completion_dirty"),
        "{output}"
    );

    let f = Fixture::new();
    git(f.path(), &["switch", "main"]);
    let path = f.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        format!("{policy}\nmerge_checks:\n  - shell.check: {{ command: 'chmod +x code' }}\n"),
    )
    .unwrap();
    commit(f.path(), "trusted executable-mode check");
    git(f.path(), &["switch", &f.branch]);
    git(f.path(), &["rebase", "main"]);
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    git(f.path(), &["config", "core.filemode", "false"]);
    let head = git(f.path(), &["rev-parse", "HEAD"]);
    // A trusted check that only flips an execute bit Git ignores is not a
    // state change.
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(pass, "{output}");
    assert_eq!(git(f.path(), &["rev-parse", "HEAD"]), head);
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
    assert_ne!(
        fs::metadata(f.path().join("code"))
            .unwrap()
            .permissions()
            .mode()
            & 0o111,
        0
    );
}

#[test]
fn review_regression_owned_checkout_checks_all_active_work_traceability() {
    let f = Fixture::new();
    let other = ok(f.path(), &["-q", "issue", "create", "Untraced active work"]);
    let mut record = f.store().load_issue_by_id(&other).unwrap();
    record.issue.status = "in_progress".into();
    f.store().write_issue_atomic(&record).unwrap();
    commit(f.path(), "untraced unrelated active work");
    f.refusal("land_work_untraceable");
    let f = Fixture::new();
    let child = epic_child(&f);
    let mut record = f.store().load_issue_by_id(&child).unwrap();
    record.issue.status = "in_progress".into();
    assert!(record.issue.workflow_branch.is_none());
    f.store().write_issue_atomic(&record).unwrap();
    commit(f.path(), "active child inherits existing owner branch");
    ok(f.path(), &["land"]);
}
#[test]
fn landing_rechecks_unrelated_active_branches_after_arbitrary_checks() {
    let f = Fixture::new();
    git(f.path(), &["switch", "main"]);
    let other = ok(f.path(), &["-q", "issue", "create", "Other active owner"]);
    let branch = other.clone();
    let mut record = f.store().load_issue_by_id(&other).unwrap();
    record.issue.status = "in_progress".into();
    record.issue.workflow_branch = Some(atelier_core::WorkflowBranch {
        work_branch: branch.clone(),
        branch_base: Some("main".into()),
    });
    f.store().write_issue_atomic(&record).unwrap();
    let path = f.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!("{policy}\nmerge_checks:\n  - shell.check: {{ command: 'if test -f .git/delete-unrelated; then git branch -D {branch}; fi' }}\n")).unwrap();
    commit(f.path(), "trusted check and unrelated active owner");
    git(f.path(), &["branch", &branch]);
    git(f.path(), &["switch", &f.branch]);
    git(f.path(), &["rebase", "main"]);
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    ok(f.path(), &["land"]);
    fs::write(f.path().join(".git/delete-unrelated"), "run check").unwrap();
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass && output.contains("land_work_untraceable"),
        "{output}"
    );
    fs::remove_file(f.path().join(".git/delete-unrelated")).unwrap();
    git(f.path(), &["branch", &branch]);
    ok(f.path(), &["land"]);
}
#[test]
fn review_regression_identical_tracker_trees_keep_each_branch_owner_visible() {
    let f = Fixture::new();
    let other = ok(f.path(), &["-q", "issue", "create", "Other active owner"]);
    let branch = other.clone();
    let mut record = f.store().load_issue_by_id(&other).unwrap();
    record.issue.status = "in_progress".into();
    record.issue.workflow_branch = Some(atelier_core::WorkflowBranch {
        work_branch: branch.clone(),
        branch_base: Some("main".into()),
    });
    f.store().write_issue_atomic(&record).unwrap();
    commit(f.path(), "two owners in shared committed tree");
    git(f.path(), &["branch", &branch]);
    git(f.path(), &["switch", "main"]);
    let output = ok(f.path(), &["status"]);
    assert!(
        output.contains(&format!("{}  ", f.branch)) && output.contains(&format!("{branch}  ")),
        "{output}"
    );
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
}
#[test]
fn review_regression_setup_preserves_foreign_wrappers_and_updates_owned_binary() {
    let f = Fixture::new();
    let directory = f.path().join(".claude");
    fs::create_dir(&directory).unwrap();
    let path = directory.join("settings.json");
    let foreign = serde_json::json!({"hooks":[{"type":"command", "command":"printf audit; /usr/local/bin/company-session land --hook claude", "timeout":7}]});
    fs::write(
        &path,
        serde_json::to_vec(&serde_json::json!({"hooks":{"Stop":[foreign]}})).unwrap(),
    )
    .unwrap();
    ok(f.path(), &["setup", "claude"]);
    ok(
        f.path(),
        &["setup", "claude", "--binary", env!("CARGO_BIN_EXE_atelier")],
    );
    let once = fs::read(&path).unwrap();
    ok(
        f.path(),
        &["setup", "claude", "--binary", env!("CARGO_BIN_EXE_atelier")],
    );
    assert_eq!(fs::read(&path).unwrap(), once);
    let settings: serde_json::Value = serde_json::from_slice(&once).unwrap();
    assert_eq!(settings["hooks"]["Stop"][0], foreign);
    assert_eq!(settings["hooks"]["Stop"].as_array().unwrap().len(), 2);
    assert_eq!(
        settings["hooks"]["SessionStart"].as_array().unwrap().len(),
        1
    );
    assert!(settings["hooks"]["Stop"][1]["hooks"][0]["command"]
        .as_str()
        .unwrap()
        .contains(env!("CARGO_BIN_EXE_atelier")));
}

/// An older policy whose work `close` required the implementer report's
/// parts one by one still loads, and its claims need only a non-empty
/// Report: the parts are Agent Factory guidance now (atelier-jnrh).
#[test]
fn an_older_policys_report_parts_require_only_a_report() {
    let close = "        required_fields: [close_reason]\n        description: Done claim; commit it on the work branch before review. A parent closes after its children.";
    assert!(STARTER_WITH_BLOCKED_AND_REVIEW.contains(close));
    let policy = STARTER_WITH_BLOCKED_AND_REVIEW.replace(
        close,
        "        required_fields: [close_reason, report.deviations, report.decisions, report.surprises, report.open_items]\n        description: Done claim with the implementer report.",
    );
    let f = Fixture::under(Some(&policy));
    let child = ok(
        f.path(),
        &["-q", "issue", "create", "Child", "--parent", &f.id],
    );
    ok(f.path(), &["issue", "transition", &child, "start"]);
    commit(f.path(), "start a child");
    // The old close still runs, and a one-line Report satisfies it.
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &child,
            "close",
            "--reason",
            "Shipped.",
        ],
    );
    commit(f.path(), "close the child through the old transition");
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(pass, "{output}");
    assert!(!output.contains("report."), "{output}");

    // A claim with no Report at all still fails the one field left.
    let mut record = f.store().load_issue_by_id(&child).unwrap();
    record.sections.report = None;
    f.store().write_issue_atomic(&record).unwrap();
    commit(f.path(), "drop the report");
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass && output.contains("required_field.close_reason") && output.contains(&child),
        "{output}"
    );
    assert!(
        output.contains(&format!(
            "write the `## Report` section in .atelier/issues/{child}.md, then commit"
        )),
        "{output}"
    );
}

#[test]
fn status_shows_the_owner_decisions_in_brief() {
    let f = Fixture::new();
    let (_, output) = cli(f.path(), &["status"]);
    assert!(!output.contains("Decisions:"), "{output}");
    ok(
        f.path(),
        &[
            "issue",
            "note",
            &f.id,
            "--kind",
            "decision",
            "Keep the format",
            "--reason",
            "Readers rely on it",
        ],
    );
    ok(
        f.path(),
        &[
            "issue",
            "note",
            &f.id,
            "--kind",
            "decision",
            "Name it plainly",
            "--reason",
            "Short",
        ],
    );
    let (pass, output) = cli(f.path(), &["status"]);
    assert!(pass, "{output}");
    assert!(output.contains("Decisions: 2 current\n"), "{output}");
    assert!(
        !output.contains("Keep the format |") && !output.contains("Name it plainly |"),
        "the brief lists decision records, not notes: {output}"
    );
}

#[test]
fn global_terminal_claims_require_fields_and_resolved_dependencies_on_any_checkout() {
    for integration_base in [false, true] {
        // An older policy's work `close` requires the Report (atelier-jnrh).
        let f = Fixture::under(Some(STARTER_WITH_BLOCKED_AND_REVIEW));
        if integration_base {
            git(f.path(), &["switch", "main"]);
        }
        let done = ok(
            f.path(),
            &["-q", "issue", "create", "Unrelated completion claim"],
        );
        let blocker = ok(f.path(), &["-q", "issue", "create", "Open dependency"]);
        let mut record = f.store().load_issue_by_id(&done).unwrap();
        record.issue.status = "done".into();
        record.issue.closed_at = Some(chrono::Utc::now());
        f.store().write_issue_atomic(&record).unwrap();
        let mut blocked = f.store().load_issue_by_id(&blocker).unwrap();
        blocked
            .relationships
            .blocks
            .push(atelier_records::issue_relationship_target(&done));
        f.store().write_issue_atomic(&blocked).unwrap();
        commit(f.path(), "invalid unrelated terminal claim");
        let (pass, output) = cli(f.path(), &["land"]);
        assert!(
            !pass
                && output.contains("required_field.close_reason")
                && output.contains("blockers.transitive_none_open"),
            "{output}"
        );
    }
}

#[test]
fn retained_completion_keeps_valid_historical_proof_and_resolution_after_unrelated_code() {
    // An older policy whose work `close` runs the proof validator; merge
    // marks work done under the current one (atelier-jnrh).
    let f = Fixture::under(Some(STARTER_WITH_BLOCKED_AND_REVIEW));
    git(f.path(), &["switch", "main"]);
    let path = f.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        policy.replace(
            "        - blockers.none_open\n",
            "        - blockers.none_open\n        - evidence.attached: { min_count: 1 }\n",
        ),
    )
    .unwrap();
    let historical = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "Previously verified work",
            "--body",
            "## Goal\n\nOriginal acceptance.\n\n## Done when\n\nOriginal proof.",
        ],
    );
    fs::write(f.path().join("history-code"), "original\n").unwrap();
    commit(f.path(), "historical acceptance and code");
    ok(
        f.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{historical}"),
            "--kind",
            "test",
            "--result",
            "fail",
            "Original failing proof",
        ],
    );
    commit(f.path(), "historical failing proof");
    let finding = ok(
        f.path(),
        &[
            "-q",
            "review",
            "finding",
            &historical,
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "Original defect",
        ],
    );
    commit(f.path(), "historical finding");
    fs::write(f.path().join("history-code"), "corrected\n").unwrap();
    commit(f.path(), "historical resolution");
    let resolution = git(f.path(), &["rev-parse", "HEAD"]);
    ok(
        f.path(),
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "fix_now",
            "--resolution-commit",
            &resolution,
            "Resolved original defect",
        ],
    );
    commit(f.path(), "historical disposition");
    ok(
        f.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{historical}"),
            "--kind",
            "test",
            "--result",
            "pass",
            "Verified historical proof",
        ],
    );
    commit(f.path(), "historical passing proof");
    f.terminal(&historical);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &historical,
            "--identity",
            "Reviewer",
            "Historical independent approval",
        ],
    );
    commit(f.path(), "historical approval");
    fs::write(f.path().join("unrelated-code"), "later work\n").unwrap();
    commit(f.path(), "later unrelated code");
    ok(f.path(), &["land"]); // Base retains previously accepted proof and resolution.
    ok(
        f.path(),
        &[
            "issue",
            "note",
            &historical,
            "Historical note after unrelated code",
        ],
    );
    commit(f.path(), "historical note-only change");
    ok(f.path(), &["land"]);
    git(f.path(), &["switch", &f.branch]);
    git(f.path(), &["rebase", "main"]);
    fs::write(f.path().join("code"), "active unfinished work\n").unwrap();
    ok(f.path(), &["land"]); // Dirty active work does not re-prove retained claims.
    git(f.path(), &["restore", "code"]);
    let mut record = f.store().load_issue_by_id(&historical).unwrap();
    if let atelier_core::IssueBody::Issue { done_when, .. } = &mut record.sections.body {
        done_when.push_str("\nChanged acceptance criterion.\n");
    }
    f.store().write_issue_atomic(&record).unwrap();
    commit(f.path(), "changed terminal acceptance");
    // The changed claim is judged again, at HEAD, where the branch's code
    // made its approval stale.
    f.refusal("findings.disposed");
    git(
        f.path(),
        &[
            "restore",
            "--source=main",
            &format!(".atelier/issues/{historical}.md"),
        ],
    );
    commit(f.path(), "restore historical acceptance");
    ok(
        f.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{historical}"),
            "--kind",
            "test",
            "--result",
            "fail",
            "New honest failure",
        ],
    );
    ok(
        f.path(),
        &[
            "review",
            "finding",
            &historical,
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "New unresolved finding",
        ],
    );
    commit(f.path(), "new failure and finding on retained claim");
    // The new fail receipt gates nothing (atelier-md0v); the new finding does.
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass && !output.contains("evidence.") && output.contains("findings.disposed"),
        "{output}"
    );
}

#[test]
fn retained_stale_approvals_never_age_into_acceptance() {
    // A decision or approval by an author identity is accepted now
    // (atelier-md0v); a fix_now resolution whose approval went stale is not.
    for (fix_now, approver, stale) in [(true, "Reviewer", true)] {
        let f = Fixture::new();
        git(f.path(), &["switch", "main"]);
        let id = ok(
            f.path(),
            &["-q", "issue", "create", "Original author's completion"],
        );
        let mut replacement_pin = None;
        if fix_now {
            commit(f.path(), "original author plan");
        }
        let finding = ok(
            f.path(),
            &[
                "-q",
                "review",
                "finding",
                &id,
                "--identity",
                "Reviewer",
                "--recommendation",
                "fix_now",
                "Original work defect",
            ],
        );
        if fix_now {
            commit(f.path(), "original finding");
            fs::write(
                f.path().join("original-work"),
                "original author resolution\n",
            )
            .unwrap();
            commit(f.path(), "author resolves finding");
            let resolution = git(f.path(), &["rev-parse", "HEAD"]);
            replacement_pin = Some(resolution.clone());
            ok(
                f.path(),
                &[
                    "review",
                    "disposition",
                    &finding,
                    "--identity",
                    "Reviewer",
                    "--decision",
                    "fix_now",
                    "--resolution-commit",
                    &resolution,
                    "Requires independent approval",
                ],
            );
            ok(
                f.path(),
                &[
                    "review",
                    "approve",
                    &id,
                    "--identity",
                    approver,
                    "Resolution approval",
                ],
            );
        } else {
            ok(
                f.path(),
                &[
                    "review",
                    "disposition",
                    &finding,
                    "--identity",
                    "Author",
                    "--decision",
                    "no_action",
                    "Author cannot accept own dismissal",
                ],
            );
        }
        if stale {
            commit(f.path(), "independent approval before later implementation");
            fs::write(
                f.path().join("original-work"),
                "code changed after approval\n",
            )
            .unwrap();
            commit(f.path(), "implementation invalidates earlier approval");
        }
        f.terminal(&id);
        let (pass, output) = cli(f.path(), &["land"]);
        assert!(!pass && output.contains("findings.disposed"), "{output}");
        if stale {
            let pin = replacement_pin.unwrap();
            let head = git(f.path(), &["rev-parse", "HEAD"]);
            let parent = git(f.path(), &["rev-parse", &format!("{pin}^")]);
            let tree = git(f.path(), &["rev-parse", "HEAD^{tree}"]);
            let replacement = git(
                f.path(),
                &[
                    "commit-tree",
                    &tree,
                    "-p",
                    &parent,
                    "-m",
                    "reinterpret resolution pin",
                ],
            );
            git(f.path(), &["replace", &pin, &replacement]);
            let (pass, output) = cli(f.path(), &["land"]);
            assert!(!pass && output.contains("findings.disposed"), "{output}");
            assert_eq!(git(f.path(), &["rev-parse", "HEAD"]), head);
            git(f.path(), &["replace", "-d", &pin]);
        }
        fs::write(f.path().join("unrelated-work"), "later unrelated work\n").unwrap();
        commit(f.path(), "aging cannot create independent acceptance");
        let (pass, output) = cli(f.path(), &["land"]);
        assert!(!pass && output.contains("findings.disposed"), "{output}");
    }
}

#[test]
fn staged_canonical_changes_cannot_hide_behind_restored_worktree_bytes() {
    for base in [false, true] {
        let f = Fixture::new();
        if base {
            git(f.path(), &["switch", "main"]);
        }
        fs::write(f.path().join("unfinished-code"), "ordinary unfinished work").unwrap();
        ok(f.path(), &["land"]);
        let relative = format!(".atelier/issues/{}.md", f.id);
        let path = f.path().join(&relative);
        let original = fs::read_to_string(&path).unwrap();
        fs::write(&path, format!("{original}\nStaged canonical change\n")).unwrap();
        git(f.path(), &["add", &relative]);
        fs::write(&path, &original).unwrap();
        assert!(git(f.path(), &["status", "--porcelain"]).contains(&format!("MM {relative}")));
        let (pass, output) = cli(f.path(), &["land"]);
        // Land names its own check once, with one recovery step.
        assert!(
            !pass
                && output.contains(&format!("land_records_uncommitted: 1 tracker record differs from HEAD: {relative} (staged); commit them with the work. Next: git add .atelier\n"))
                && !output.contains("canonical_snapshot_uncommitted")
                && output.matches("Next:").count() == 1,
            "{output}"
        );
        git(f.path(), &["reset", "-q", "HEAD", "--", &relative]);
        ok(f.path(), &["land"]);
        fs::write(
            f.path().join(".gitattributes"),
            ".atelier/issues/*.md filter=uppercase\n",
        )
        .unwrap();
        git(
            f.path(),
            &["config", "filter.uppercase.clean", "tr a-z A-Z"],
        );
        fs::write(&path, &original).unwrap();
        assert!(git(f.path(), &["status", "--porcelain"])
            .lines()
            .any(|line| line.trim_start().starts_with("M ") && line.ends_with(&relative)));
        ok(f.path(), &["land"]); // Raw worktree and index still equal committed HEAD.
    }
}

#[test]
fn integration_base_refuses_review_rewrites_deletion_and_reverted_edits() {
    for (mutation, renames) in [
        ("rewrite", None),
        ("delete", None),
        ("edit_revert", None),
        ("rename", Some("true")),
        ("rename", Some("copies")),
        ("rename", Some("false")),
    ] {
        let f = Fixture::new();
        if let Some(value) = renames {
            git(f.path(), &["config", "diff.renames", value]);
        }
        git(f.path(), &["switch", "main"]);
        let id = ok(
            f.path(),
            &["-q", "issue", "create", "Immutable review history"],
        );
        let finding = ok(
            f.path(),
            &[
                "-q",
                "review",
                "finding",
                &id,
                "--identity",
                "Reviewer",
                "--recommendation",
                "fix_now",
                "Reviewed finding",
            ],
        );
        let disposition = ok(
            f.path(),
            &[
                "-q",
                "review",
                "disposition",
                &finding,
                "--identity",
                "Reviewer",
                "--decision",
                "no_action",
                "Independent accepted disposition",
            ],
        );
        f.terminal(&id);
        ok(f.path(), &["land"]);
        let path = f.path().join(format!(".atelier/reviews/{disposition}.md"));
        let original = fs::read_to_string(&path).unwrap();
        if mutation == "delete" {
            fs::remove_file(&path).unwrap();
        } else if mutation == "rename" {
            let renamed = f.path().join(".atelier/reviews/atelier-z999.md");
            fs::rename(&path, &renamed).unwrap();
            fs::write(
                &renamed,
                original
                    .replace(&disposition, "atelier-z999")
                    .replace("Reviewer", "Author"),
            )
            .unwrap();
        } else {
            fs::write(&path, original.replace("Reviewer", "Author")).unwrap();
        }
        commit(f.path(), "invalid historical event mutation");
        if mutation == "edit_revert" {
            fs::write(&path, &original).unwrap();
            commit(
                f.path(),
                "restore original bytes without erasing invalid history",
            );
        }
        fs::write(f.path().join("later-code"), "unrelated later work\n").unwrap();
        commit(f.path(), "aging cannot legitimate rewritten event");
        let (pass, output) = cli(f.path(), &["land"]);
        assert!(!pass && output.contains("review_append_only"), "{output}");
    }
}

#[test]
fn root_review_introductions_cannot_be_hidden_by_log_configuration() {
    for original_identity in ["Reviewer", "Author"] {
        let f = Fixture::new();
        git(f.path(), &["switch", "main"]);
        let finding = ok(
            f.path(),
            &[
                "-q",
                "review",
                "finding",
                &f.id,
                "--identity",
                "Reviewer",
                "--recommendation",
                "fix_now",
                "Root finding",
            ],
        );
        let disposition = ok(
            f.path(),
            &[
                "-q",
                "review",
                "disposition",
                &finding,
                "--identity",
                original_identity,
                "--decision",
                "no_action",
                "Root disposition",
            ],
        );
        // Make the existing canonical tree the first commit of an independent repo.
        fs::remove_dir_all(f.path().join(".git")).unwrap();
        git(f.path(), &["init", "-qb", "main"]);
        git(f.path(), &["config", "user.name", "Author"]);
        git(f.path(), &["config", "user.email", "author@example.test"]);
        commit(f.path(), "root canonical records and review events");
        git(f.path(), &["config", "log.showRoot", "false"]);
        ok(f.path(), &["land"]);
        // The disposition is a field of the finding (ADR 0025), so rewrite
        // the finding's own identity, which never changes.
        assert_eq!(disposition, finding);
        let path = f.path().join(format!(".atelier/reviews/{finding}.md"));
        let original = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            original.replacen("identity: Reviewer", "identity: Rewritten", 1),
        )
        .unwrap();
        commit(f.path(), "rewrite root event identity");
        let (pass, output) = cli(f.path(), &["land"]);
        assert!(!pass && output.contains("review_append_only"), "{output}");
    }
}

#[test]
fn review_records_first_added_by_a_conflict_resolving_merge_are_found() {
    let f = Fixture::new();
    git(f.path(), &["switch", "main"]);
    fs::write(f.path().join("shared"), "base\n").unwrap();
    commit(f.path(), "shared file");
    git(f.path(), &["switch", "-qc", "side"]);
    fs::write(f.path().join("shared"), "side\n").unwrap();
    commit(f.path(), "side edit");
    git(f.path(), &["switch", "main"]);
    fs::write(f.path().join("shared"), "main\n").unwrap();
    commit(f.path(), "main edit");
    let (merged, _) = cli_git(f.path(), &["merge", "-q", "--no-edit", "side"]);
    assert!(!merged, "the merge conflicts");
    fs::write(f.path().join("shared"), "resolved\n").unwrap();
    // The issue and both review events first appear in the merge itself.
    let id = ok(f.path(), &["-q", "issue", "create", "Merged review work"]);
    let finding = ok(
        f.path(),
        &[
            "-q",
            "review",
            "finding",
            &id,
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "Concern raised while merging",
        ],
    );
    let disposition = ok(
        f.path(),
        &[
            "-q",
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "no_action",
            "Independent dismissal recorded while merging",
        ],
    );
    commit(f.path(), "resolve conflict with review records");
    let path = format!(".atelier/reviews/{disposition}.md");
    for parent in ["HEAD^1", "HEAD^2"] {
        let (present, _) = cli_git(f.path(), &["cat-file", "-e", &format!("{parent}:{path}")]);
        assert!(!present, "{parent} has {path}");
    }
    f.terminal(&id);
    ok(f.path(), &["land"]);
    ok(f.path(), &["doctor"]);
}

#[cfg(unix)]
#[test]
fn root_review_mode_must_be_regular_non_executable() {
    use std::os::unix::fs::PermissionsExt;
    let f = Fixture::new();
    git(f.path(), &["switch", "main"]);
    let finding = ok(
        f.path(),
        &[
            "-q",
            "review",
            "finding",
            &f.id,
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "Root finding",
        ],
    );
    fs::remove_dir_all(f.path().join(".git")).unwrap();
    git(f.path(), &["init", "-qb", "main"]);
    git(f.path(), &["config", "user.name", "Author"]);
    git(f.path(), &["config", "user.email", "author@example.test"]);
    let path = f.path().join(format!(".atelier/reviews/{finding}.md"));
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    commit(f.path(), "root executable review event");
    git(f.path(), &["config", "log.showRoot", "false"]);
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(!pass && output.contains("review_append_only"), "{output}");
}

#[test]
fn terminal_review_cluster_prune_remains_a_valid_handoff() {
    for (days_ago, retired) in [(2000, true), (2, true), (0, false)] {
        let f = Fixture::new();
        git(f.path(), &["switch", "main"]);
        let id = ok(
            f.path(),
            &["-q", "issue", "create", "Retired reviewed work"],
        );
        let finding = ok(
            f.path(),
            &[
                "-q",
                "review",
                "finding",
                &id,
                "--identity",
                "Reviewer",
                "--recommendation",
                "fix_now",
                "Reviewed concern",
            ],
        );
        let disposition = ok(
            f.path(),
            &[
                "-q",
                "review",
                "disposition",
                &finding,
                "--identity",
                "Reviewer",
                "--decision",
                "no_action",
                "Independent accepted disposition",
            ],
        );
        let mut record = f.store().load_issue_by_id(&id).unwrap();
        let date = chrono::Utc::now() - chrono::Duration::days(days_ago);
        record.issue.status = "done".into();
        record.issue.created_at = date;
        record.issue.updated_at = date;
        record.issue.closed_at = Some(date);
        record.sections.report = Some("Verified terminal claim".into());
        f.store().write_issue_atomic(&record).unwrap();
        if retired {
            let created = format!("created_at: {}", date.to_rfc3339());
            for review in [&finding, &disposition] {
                let path = f.path().join(format!(".atelier/reviews/{review}.md"));
                let text = fs::read_to_string(&path).unwrap();
                let old = text
                    .lines()
                    .map(|line| {
                        if line.starts_with("created_at:") {
                            created.as_str()
                        } else {
                            line
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                fs::write(path, format!("{old}\n")).unwrap();
            }
        }
        commit(f.path(), "original terminal review cluster");
        ok(f.path(), &["land"]);
        ok(f.path(), &["prune", "--apply", "--retention-days", "0"]);
        let issue_path = f.path().join(format!(".atelier/issues/{id}.md"));
        if !retired {
            assert!(issue_path.exists()); // Day-based cutoff still protects today's work.
            ok(f.path(), &["land"]);
        } else {
            assert!(!issue_path.exists());
            for review in [&finding, &disposition] {
                assert!(!f
                    .path()
                    .join(format!(".atelier/reviews/{review}.md"))
                    .exists());
            }
            commit(f.path(), "public prune retired complete terminal cluster");
            ok(f.path(), &["land"]);
            fs::write(f.path().join("later-code"), "later work\n").unwrap();
            commit(f.path(), "later unrelated work");
            ok(f.path(), &["land"]);
        }
    }
}

#[test]
fn retirement_cannot_hide_prior_rewrites_or_active_target_events() {
    for mutation in ["rewrite", "edit_revert", "active", "partial"] {
        let f = Fixture::new();
        git(f.path(), &["switch", "main"]);
        let id = ok(
            f.path(),
            &["-q", "issue", "create", "Protected review history"],
        );
        let finding = ok(
            f.path(),
            &[
                "-q",
                "review",
                "finding",
                &id,
                "--identity",
                "Reviewer",
                "--recommendation",
                "fix_now",
                "Concern",
            ],
        );
        let disposition = ok(
            f.path(),
            &[
                "-q",
                "review",
                "disposition",
                &finding,
                "--identity",
                "Reviewer",
                "--decision",
                "no_action",
                "Independent acceptance",
            ],
        );
        if mutation == "active" {
            commit(f.path(), "active target and reviews");
        } else {
            f.terminal(&id);
        }
        let path = f.path().join(format!(".atelier/reviews/{disposition}.md"));
        let original = fs::read_to_string(&path).unwrap();
        if matches!(mutation, "rewrite" | "edit_revert") {
            fs::write(&path, original.replace("Reviewer", "Author")).unwrap();
            commit(f.path(), "invalid event rewrite");
            if mutation == "edit_revert" {
                fs::write(&path, original).unwrap();
                commit(f.path(), "restore bytes without erasing rewrite history");
            }
        }
        fs::remove_file(f.path().join(format!(".atelier/issues/{id}.md"))).unwrap();
        // The disposition is the finding's own file (ADR 0025); a partial
        // cluster keeps it while its issue goes.
        assert_eq!(disposition, finding);
        if mutation != "partial" {
            fs::remove_file(path).unwrap();
        }
        commit(
            f.path(),
            "retire records after invalid history or with invalid cluster",
        );
        let (pass, output) = cli(f.path(), &["land"]);
        assert!(!pass, "{mutation}: {output}");
        if mutation != "partial" {
            assert!(output.contains("review_append_only"), "{output}");
        }
    }
}

#[test]
fn valid_independent_review_survives_merge_after_reviewer_authored_prior_base_work() {
    let dir = tempdir().unwrap();
    git(dir.path(), &["init", "-qb", "main"]);
    git(dir.path(), &["config", "user.name", "Author"]);
    git(dir.path(), &["config", "user.email", "author@example.test"]);
    ok(dir.path(), &["init", "--slug", "atelier"]);
    let id = ok(
        dir.path(),
        &[
            "-q",
            "issue",
            "create",
            "Planned before unrelated base work",
        ],
    );
    commit(dir.path(), "Author planned owner");
    git(dir.path(), &["config", "user.name", "Reviewer"]);
    git(
        dir.path(),
        &["config", "user.email", "reviewer@example.test"],
    );
    fs::write(
        dir.path().join("base-code"),
        "Reviewer authored unrelated base work\n",
    )
    .unwrap();
    commit(dir.path(), "Reviewer prior base work");
    git(dir.path(), &["config", "user.name", "Author"]);
    git(dir.path(), &["config", "user.email", "author@example.test"]);
    ok(dir.path(), &["start", &id]);
    fs::write(dir.path().join("owner-code"), "Author implementation\n").unwrap();
    commit(dir.path(), "Author owner implementation");
    let finding = ok(
        dir.path(),
        &[
            "-q",
            "review",
            "finding",
            &id,
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "Potential concern",
        ],
    );
    ok(
        dir.path(),
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "no_action",
            "Independent accepted disposition",
        ],
    );
    commit(dir.path(), "Reviewer events");
    let f = Fixture {
        branch: git(dir.path(), &["branch", "--show-current"]),
        dir,
        id,
    };
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent approval",
        ],
    );
    commit(f.path(), "Reviewer approval");
    ok(f.path(), &["land"]);
    ok(f.path(), &["merge", &f.id]);
    assert_eq!(git(f.path(), &["symbolic-ref", "HEAD"]), "refs/heads/main");
    ok(f.path(), &["land"]);
    fs::write(f.path().join("subsequent-code"), "unrelated future work\n").unwrap();
    commit(f.path(), "later unrelated base code");
    ok(f.path(), &["land"]);
}

#[test]
fn status_keeps_eight_newest_scoped_proofs_and_fresh_hidden_dirt() {
    let f = Fixture::new();
    let mut ids = Vec::new();
    for n in 0..10 {
        let id = ok(
            f.path(),
            &[
                "-q",
                "evidence",
                "record",
                "--target",
                &format!("issue/{}", f.id),
                "--kind",
                "test",
                "--result",
                "pass",
                &format!("Proof {n}"),
            ],
        );
        let id = id.split_whitespace().next().unwrap().to_owned();
        let mut record = f.store().load_evidence_by_id(&id).unwrap();
        // Paired times exercise the existing descending ID tie-break.
        record.data.captured_at =
            chrono::DateTime::from_timestamp(1_700_000_000 + n / 2, 0).unwrap();
        f.store().write_evidence_atomic(&record).unwrap();
        ids.push((record.data.captured_at, id));
    }
    commit(f.path(), "ten proofs");
    ids.sort_by(|a, b| b.cmp(a));
    let expected = ids
        .iter()
        .take(8)
        .map(|(_, id)| id.as_str())
        .collect::<Vec<_>>();
    let status = ok(f.path(), &["status"]);
    let actual = section(&status, "Evidence")
        .into_iter()
        .filter_map(|line| line.split_once(" — ").map(|(id, _)| id))
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
    assert_eq!(status.matches("(fresh)").count(), 8);
    git(f.path(), &["update-index", "--skip-worktree", "code"]);
    fs::write(f.path().join("code"), "hidden dirty bytes\n").unwrap();
    let status = ok(f.path(), &["status"]);
    assert_eq!(
        field(&status, "Checkout"),
        Some("dirty (1 path: code)"),
        "{status}"
    );
    assert_eq!(status.matches("(stale against current work)").count(), 8);
    fs::write(f.path().join("code"), "tested code\n").unwrap();
    let mut record = f.store().load_evidence_by_id(expected[0]).unwrap();
    record.data.commit = "f".repeat(40);
    f.store().write_evidence_atomic(&record).unwrap();
    let status = ok(f.path(), &["status"]);
    assert_eq!(status.matches("(stale against current work)").count(), 1);
    assert_eq!(status.matches("(fresh)").count(), 7);
}
#[test]
fn unowned_branch_lands_unless_it_changes_records() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "-c", "feature/unowned", "main"]);
    fs::write(f.path().join("feature"), "code only\n").unwrap();
    commit(f.path(), "code on a branch no item owns");
    let output = ok(f.path(), &["land"]);
    assert!(
        output
            .contains("No work owner on branch feature/unowned; it changes no Atelier tracker records since main"),
        "{output}"
    );
    let stop =
        serde_json::json!({"cwd":f.path(), "hook_event_name":"Stop", "stop_hook_active":false});
    assert_eq!(hook(f.path(), "land", stop.clone()), serde_json::json!({}));

    let filed = ok(
        f.path(),
        &["-q", "issue", "create", "Filed off an owner branch"],
    );
    commit(f.path(), "record change on a branch no item owns");
    let head = git(f.path(), &["rev-parse", "HEAD"]);
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass
            && output.contains("land_work_untraceable: branch feature/unowned has no owner but changes Atelier tracker records since main")
            && output.contains(&format!(".atelier/issues/{filed}.md"))
            && output.contains(&format!("Next: give the work an owner branch with `atelier start {filed}`")),
        "{output}"
    );
    assert_eq!(git(f.path(), &["rev-parse", "HEAD"]), head);
    // Off the base, it is work in progress: reported, not blocked
    // (atelier-md0v).
    assert_eq!(hook(f.path(), "land", stop), in_progress_message(&output));
}
#[test]
fn unowned_branch_landing_tolerates_stale_base_detached_head_and_reports_unrelated_history() {
    let f = Fixture::new();
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    let remote = upstream.path().to_str().unwrap();
    git(f.path(), &["remote", "add", "origin", remote]);
    git(f.path(), &["push", "-q", "origin", "main"]);
    // Upstream gains a record that the local base never fetches.
    let other = tempdir().unwrap();
    git(other.path(), &["clone", "-q", "-b", "main", remote, "."]);
    git(other.path(), &["config", "user.name", "Other"]);
    git(
        other.path(),
        &["config", "user.email", "other@example.test"],
    );
    ok(other.path(), &["-q", "issue", "create", "Filed upstream"]);
    commit(other.path(), "upstream record");
    git(other.path(), &["push", "-q", "origin", "main"]);
    git(f.path(), &["fetch", "-q", "origin"]);
    git(
        f.path(),
        &["switch", "-q", "-c", "pr/from-origin", "origin/main"],
    );
    fs::write(f.path().join("pr"), "code only\n").unwrap();
    commit(f.path(), "code on a PR branch from origin");
    let output = ok(f.path(), &["land"]);
    assert!(
        output.contains("No work owner on branch pr/from-origin"),
        "{output}"
    );

    git(f.path(), &["switch", "-q", "--detach", "main"]);
    let output = ok(f.path(), &["land"]);
    assert!(
        output.contains("No work owner on detached HEAD;"),
        "{output}"
    );

    git(f.path(), &["checkout", "-q", "--orphan", "unrelated"]);
    commit(f.path(), "history unrelated to the base");
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass
            && output
                .contains("land_work_untraceable: this unowned branch shares no history with main"),
        "{output}"
    );
}
#[test]
fn base_branch_accepts_record_only_commits_and_reviewed_merges_but_not_direct_code() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let output = ok(f.path(), &["land"]);
    assert!(
        output.contains("Base commits unchecked: main has no upstream"),
        "{output}"
    );
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        f.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);

    let mission = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "Planning on the base",
            "--issue-type",
            "mission",
        ],
    );
    let mut record = f.store().load_issue_by_id(&mission).unwrap();
    record.issue.strategy = Some(atelier_core::IssueStrategy::new("./plans/strategy.md").unwrap());
    f.store().write_issue_atomic(&record).unwrap();
    fs::create_dir(f.path().join("plans")).unwrap();
    fs::write(f.path().join("plans/strategy.md"), "# Strategy\n").unwrap();
    commit(f.path(), "file a mission and its strategy on the base");
    let output = ok(f.path(), &["land"]);
    assert!(
        output.contains("Base commits: 1 unpushed record-only commit outside"),
        "{output}"
    );

    fs::write(f.path().join("direct"), "unreviewed code\n").unwrap();
    commit(f.path(), "code straight on the base");
    let head = git(f.path(), &["rev-parse", "HEAD"]);
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass
            && output.contains(&format!("land_base_unreviewed: commit {}", &head[..8]))
            && output.contains("changes direct without recorded review"),
        "{output}"
    );
    // An unrelated self-recorded exemption does not cover base code, and a
    // branch started now would claim the commit in its own review.
    let unrelated = ok(f.path(), &["-q", "issue", "create", "Never started"]);
    ok(
        f.path(),
        &[
            "review",
            "trivial",
            &unrelated,
            "--identity",
            "Author",
            "Self-exempt",
        ],
    );
    commit(f.path(), "self-recorded exemption");
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(!pass && output.contains("land_base_unreviewed"), "{output}");
    let (pass, output) = cli(f.path(), &["start", &unrelated]);
    assert!(!pass && output.contains("land_base_unreviewed"), "{output}");
    assert_eq!(git(f.path(), &["branch", "--show-current"]), "main");
    git(f.path(), &["reset", "-q", "--hard", "HEAD~2"]);

    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["rebase", "-q", "main"]);
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    ok(f.path(), &["merge", &f.id]);
    git(f.path(), &["switch", "-q", "main"]);
    // The approval's record commit after its pin; the merge commit adds
    // only the done marks the approval covers (atelier-jnrh).
    let output = ok(f.path(), &["land"]);
    assert!(
        output.contains("Base commits: 1 unpushed record-only commit outside"),
        "{output}"
    );

    // Once pushed, a review recorded on the base for the merged owner does
    // not cover later direct code.
    git(f.path(), &["push", "-q"]);
    fs::write(f.path().join("later"), "direct again\n").unwrap();
    commit(f.path(), "code straight on the base after the merge");
    ok(
        f.path(),
        &[
            "review",
            "trivial",
            &f.id,
            "--identity",
            "Author",
            "Re-exempt",
        ],
    );
    commit(f.path(), "exemption recorded on the base");
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass && output.contains("changes later without recorded review"),
        "{output}"
    );
}
/// A trivial exemption pinned to a base commit, with no owner issue or work
/// branch, covers exactly that commit for `land` (atelier-mbrf).
#[test]
fn a_base_exemption_covers_exactly_its_commit() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        f.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);

    // Unexempted, a direct code commit is refused.
    fs::write(f.path().join("README"), "Fixed typo\n").unwrap();
    commit(f.path(), "fix a typo on the base");
    let typo = git(f.path(), &["rev-parse", "HEAD"]);
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass && output.contains(&format!("land_base_unreviewed: commit {}", &typo[..8])),
        "{output}"
    );

    // Off the base, or for a commit the base does not hold, it is refused.
    let side = git(f.path(), &["rev-parse", &f.branch]);
    let (pass, output) = cli(
        f.path(),
        &[
            "review",
            "trivial",
            "--base",
            &side,
            "--identity",
            "Author",
            "Not here",
        ],
    );
    assert!(!pass && output.contains("review_base_commit"), "{output}");
    git(f.path(), &["switch", "-q", &f.branch]);
    let (pass, output) = cli(
        f.path(),
        &[
            "review",
            "trivial",
            "--base",
            &typo,
            "--identity",
            "Author",
            "Not here",
        ],
    );
    assert!(
        !pass && output.contains("review_base_not_checked_out"),
        "{output}"
    );
    git(f.path(), &["switch", "-q", "main"]);
    let (pass, output) = cli(
        f.path(),
        &[
            "review",
            "trivial",
            &f.id,
            "--base",
            &typo,
            "--identity",
            "Author",
            "Both",
        ],
    );
    assert!(!pass, "an owner and --base together: {output}");

    let exemption = ok(
        f.path(),
        &[
            "-q",
            "review",
            "trivial",
            "--base",
            "HEAD",
            "--identity",
            "Author",
            "One-word typo fix in README; correctness is evident from the diff",
            "--commit-records",
        ],
    );
    let record =
        fs::read_to_string(f.path().join(format!(".atelier/reviews/{exemption}.md"))).unwrap();
    assert!(
        record.contains("\nbase: main\n")
            && record.contains(&format!("\ncommit: {typo}\n"))
            && !record.contains("\ntarget:")
            && record.contains("event: trivial_exemption"),
        "{record}"
    );
    let output = ok(f.path(), &["land"]);
    assert!(
        output.contains("1 unpushed record-only commit outside review; 1 unpushed commit exempted as trivial on main"),
        "{output}"
    );
    let listed = ok(f.path(), &["review", "list"]);
    assert!(
        listed.contains(&format!(
            "{exemption}  trivial_exemption  of main  @{}  Author  ",
            &typo[..8]
        )),
        "{listed}"
    );
    let shown = ok(f.path(), &["review", "show", &exemption]);
    assert!(
        shown.starts_with(&format!(
            "{exemption} trivial_exemption of main @{} by Author | ",
            &typo[..8]
        )) && shown.contains("\n\nOne-word typo fix in README"),
        "{shown}"
    );

    // The exemption does not cover a later commit, even one on top of it.
    fs::write(f.path().join("README"), "Fixed typo, and more\n").unwrap();
    commit(f.path(), "a later edit on the base");
    let later = git(f.path(), &["rev-parse", "HEAD"]);
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass
            && output.contains(&format!("land_base_unreviewed: commit {}", &later[..8]))
            && output.contains("changes README"),
        "{output}"
    );
    git(f.path(), &["reset", "-q", "--hard", "HEAD~1"]);

    // It covers no done claim: that still travels on a work branch.
    let mut record = f.store().load_issue_by_id(&f.id).unwrap();
    record.issue.status = "done".into();
    record.issue.closed_at = Some(chrono::Utc::now());
    record.sections.report = Some("completed claim".into());
    f.store().write_issue_atomic(&record).unwrap();
    fs::write(f.path().join("README"), "Fixed typo again\n").unwrap();
    commit(f.path(), "a claim and an edit on the base");
    ok(
        f.path(),
        &[
            "review",
            "trivial",
            "--base",
            "HEAD",
            "--identity",
            "Author",
            "Typo fix",
            "--commit-records",
        ],
    );
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(!pass && output.contains("land_base_done_claim"), "{output}");
}
#[test]
fn base_refuses_done_claims_outside_review_but_allows_supersede_and_decisions() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        f.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);

    let retired = ok(f.path(), &["-q", "issue", "create", "Not needed"]);
    let decision = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "Pick a format",
            "--issue-type",
            "decision",
            "--body",
            "## Question\n\nWhich format?\n\n## Decision\n\nTOML.",
        ],
    );
    commit(f.path(), "plan on the base");
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &retired,
            "supersede",
            "--reason",
            "obsolete",
        ],
    );
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &decision,
            "decide",
            "--reason",
            "TOML",
        ],
    );
    commit(f.path(), "supersede and decide on the base");
    ok(f.path(), &["land"]);

    // Starting and closing work on the base skips its branch and review.
    let work = ok(f.path(), &["-q", "issue", "create", "Branchless work"]);
    commit(f.path(), "file work on the base");
    ok(f.path(), &["issue", "transition", &work, "start"]);
    mark_done(f.path(), &work);
    commit(f.path(), "branchless done claim on the base");
    let head = git(f.path(), &["rev-parse", "HEAD"]);
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass
            && output.contains(&format!(
                "land_base_done_claim: commit {} on main marks {work} done",
                &head[..8]
            ))
            && output.contains(&format!("git revert {}", &head[..8]))
            && output.contains(&format!("atelier start {work}")),
        "{output}"
    );

    // Following the advice: revert the claim, then start the work.
    git(f.path(), &["revert", "--no-edit", "HEAD"]);
    ok(f.path(), &["land"]);
    ok(f.path(), &["start", &work]);
    git(f.path(), &["switch", "-q", "main"]);

    // A mission closes on the base.
    let mission = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "Outcome",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nShipped.\n\n## Success criteria\n\n- It works.",
        ],
    );
    commit(f.path(), "file a mission");
    for transition in ["ready", "start"] {
        ok(f.path(), &["issue", "transition", &mission, transition]);
    }
    ok(
        f.path(),
        &["issue", "transition", &mission, "close", "--reason", "met"],
    );
    commit(f.path(), "close the mission on the base");
    ok(f.path(), &["land"]);

    // A done claim that reached the upstream through review arrives in a
    // pull merge; the merge does not claim it.
    let remote = upstream.path().to_str().unwrap();
    let other = tempdir().unwrap();
    git(other.path(), &["clone", "-q", "-b", "main", remote, "."]);
    git(other.path(), &["config", "user.name", "Other"]);
    git(
        other.path(),
        &["config", "user.email", "other@example.test"],
    );
    ok(other.path(), &["issue", "transition", &f.id, "start"]);
    mark_done(other.path(), &f.id);
    commit(other.path(), "done claim that reached the upstream");
    git(other.path(), &["push", "-q", "origin", "main"]);
    ok(f.path(), &["-q", "issue", "create", "Local planning"]);
    commit(f.path(), "record-only planning on the base");
    git(f.path(), &["pull", "-q", "--no-rebase", "--no-edit"]);
    ok(f.path(), &["land"]);
}
#[test]
fn base_done_claims_survive_a_broken_record_and_merges_judge_every_parent() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let work = ok(f.path(), &["-q", "issue", "create", "Work"]);
    let shared = ok(f.path(), &["-q", "issue", "create", "Shared"]);
    commit(f.path(), "plan on the base");
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    let remote = upstream.path().to_str().unwrap();
    git(f.path(), &["remote", "add", "origin", remote]);
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);

    // Breaking the record in one commit and restoring it as done in the
    // next is still a done claim made on the base.
    let path = f.path().join(format!(".atelier/issues/{work}.md"));
    let original = fs::read_to_string(&path).unwrap();
    fs::write(&path, "not a record\n").unwrap();
    commit(f.path(), "break the record");
    fs::write(&path, &original).unwrap();
    ok(f.path(), &["issue", "transition", &work, "start"]);
    mark_done(f.path(), &work);
    commit(f.path(), "restore it as done");
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass
            && output.contains(&format!("land_base_done_claim: commit"))
            && output.contains(&work),
        "{output}"
    );
    git(f.path(), &["reset", "-q", "--hard", "main@{upstream}"]);

    // A done claim reviewed upstream arrives in a conflicted pull merge; the
    // merge's own parents decide, so it is not a claim made here.
    let other = tempdir().unwrap();
    git(other.path(), &["clone", "-q", "-b", "main", remote, "."]);
    git(other.path(), &["config", "user.name", "Other"]);
    git(
        other.path(),
        &["config", "user.email", "other@example.test"],
    );
    ok(other.path(), &["issue", "transition", &shared, "start"]);
    mark_done(other.path(), &shared);
    commit(other.path(), "done claim that reached the upstream");
    git(other.path(), &["push", "-q", "origin", "main"]);
    ok(
        f.path(),
        &["issue", "update", &shared, "--title", "Shared, retitled"],
    );
    commit(f.path(), "local planning on the same record");
    git(f.path(), &["fetch", "-q"]);
    let merged = Command::new("git")
        .current_dir(f.path())
        .args(["merge", "--no-edit", "origin/main"])
        .output()
        .unwrap();
    assert!(!merged.status.success(), "the shared record conflicts");
    git(
        f.path(),
        &[
            "checkout",
            "--theirs",
            "--",
            &format!(".atelier/issues/{shared}.md"),
        ],
    );
    // Keep the local title too, so the resolution differs from both parents.
    ok(
        f.path(),
        &["issue", "update", &shared, "--title", "Shared, retitled"],
    );
    git(f.path(), &["add", "."]);
    git(f.path(), &["commit", "-q", "--no-edit"]);
    ok(f.path(), &["land"]);
}
#[test]
fn base_done_claim_check_reads_the_nearest_parseable_earlier_copy() {
    for case in ["restored_unchanged", "reopened_then_restored"] {
        let f = Fixture::new();
        git(f.path(), &["switch", "-q", "main"]);
        let work = ok(f.path(), &["-q", "issue", "create", "Work"]);
        commit(f.path(), "plan on the base");
        ok(f.path(), &["issue", "transition", &work, "start"]);
        mark_done(f.path(), &work);
        commit(f.path(), "done claim that reached the upstream");
        let upstream = tempdir().unwrap();
        git(upstream.path(), &["init", "-q", "--bare"]);
        git(
            f.path(),
            &["remote", "add", "origin", upstream.path().to_str().unwrap()],
        );
        git(f.path(), &["push", "-q", "-u", "origin", "main"]);
        let path = f.path().join(format!(".atelier/issues/{work}.md"));
        let done = fs::read_to_string(&path).unwrap();
        if case == "reopened_then_restored" {
            fs::write(
                &path,
                done.replace("status: \"done\"", "status: \"in_progress\""),
            )
            .unwrap();
            commit(f.path(), "set it back by hand");
        }
        fs::write(&path, "not a record\n").unwrap();
        commit(f.path(), "break the record");
        fs::write(&path, &done).unwrap();
        commit(f.path(), "restore it as done");
        let (pass, output) = cli(f.path(), &["land"]);
        if case == "restored_unchanged" {
            assert!(pass, "{case}: {output}");
        } else {
            assert!(
                !pass && output.contains("land_base_done_claim"),
                "{case}: {output}"
            );
        }
    }
}
#[test]
fn migrating_an_already_done_epic_on_the_base_is_not_a_done_claim() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    // A base from before atelier-9vq3, with a finished epic.
    let policy_path = f.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&policy_path)
        .unwrap()
        .replace(
            "  bug:\n    label: Bug\n",
            "  bug:\n    label: Bug\n  epic:\n    label: Epic\n",
        )
        .replace(
            "    applies_to:\n    - mission\n",
            "    applies_to:\n    - mission\n    - epic\n",
        );
    fs::write(&policy_path, policy).unwrap();
    let epic = "atelier-ep1c";
    fs::write(
        f.path().join(format!(".atelier/issues/{epic}.md")),
        format!(
            "---\ncreated_at: \"2026-10-01T00:00:00+00:00\"\nid: \"{epic}\"\nissue_type: \"epic\"\nlabels: []\npriority: \"P2\"\nrelationships:\n  blocks: []\n  children: []\n  attachments: []\n  relates: []\nschema: \"atelier.issue\"\nschema_version: 2\nstatus: \"done\"\ntitle: \"Old epic\"\nupdated_at: \"2026-10-01T00:00:00+00:00\"\n---\n\n## Outcome\n\nThe increment works.\n\n## Close Reason\n\nShipped.\n"
        ),
    )
    .unwrap();
    commit(f.path(), "a base with a finished epic");
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        f.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);

    ok(f.path(), &["migrate", "epics", "--apply"]);
    commit(f.path(), "migrate epics on the base");
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass
            && !output.contains("land_base_done_claim")
            && output.contains("land_base_unreviewed"),
        "{output}"
    );
}
/// One rule for what an approval covers (atelier-n642, atelier-uw4x,
/// atelier-3jdm): a close after approval that changes no code, even one that
/// also edits the owner's Goal or adds a note, stays covered under `merge`
/// and under `land` on the base. Code after the approval, a done claim
/// outside the approved scope, or an approval `merge` would not accept
/// (atelier-n90q) leaves the claim uncovered under both: `merge` refuses
/// before integrating, and after a forced integration `land` refuses.
/// A follow-up filed after the approval and linked to its mission in a later
/// commit keeps it current, though its own done claim is not covered
/// (atelier-iaas). The owner's done claim is made by hand on the branch, as
/// an older Atelier's close made it; such a claim merges as it is
/// (atelier-jnrh).
#[test]
fn a_close_after_approval_is_covered_by_merge_and_land_alike() {
    let covered_cases = [
        "close_only",
        "children_close_only",
        "children_in_one_commit",
        "reviewer_base_close_then_approve",
        "reviewer_base_reapproved_after_code",
        "follow_up_linked",
        "base_issue_edited",
        "with_goal",
        "with_note",
        // An approval's identity is not compared with the authors, and the
        // approval after a fix_now finding resolves it (atelier-md0v).
        "self_approval",
        "open_fix_now",
    ];
    let stale_cases = [
        "with_code",
        "new_done_issue",
        "follow_up_done",
        "base_issue_edited_done",
        "sibling_close",
    ];
    for case in covered_cases.iter().chain(&stale_cases).copied() {
        close_after_approval(case, covered_cases.contains(&case));
    }
}
fn close_after_approval(case: &str, covered: bool) {
    let label = case;
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    // The reviewer authored work on the base that the branch merges in.
    let reviewer_base = case.starts_with("reviewer_base");
    if reviewer_base {
        fs::write(f.path().join("base_work"), "reviewed elsewhere\n").unwrap();
        git(f.path(), &["add", "."]);
        git(
            f.path(),
            &[
                "-c",
                "user.name=Reviewer",
                "-c",
                "user.email=reviewer@example.test",
                "commit",
                "-qm",
                "base work by the reviewer",
            ],
        );
    }
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        f.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);
    git(f.path(), &["switch", "-q", &f.branch]);
    if reviewer_base {
        git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    }
    let approve = |identity: &str| {
        ok(
            f.path(),
            &["review", "approve", &f.id, "--identity", identity, "Review"],
        );
        commit(f.path(), &format!("approval by {identity}"));
    };
    let child = case.starts_with("children").then(|| {
        let child = ok(
            f.path(),
            &["-q", "issue", "create", "Child", "--parent", &f.id],
        );
        commit(f.path(), "plan a child");
        child
    });
    let sibling = (case == "sibling_close").then(|| {
        let sibling = ok(f.path(), &["-q", "issue", "create", "Sibling"]);
        commit(f.path(), "plan a sibling outside the scope");
        sibling
    });
    // A mission outside the scope that a follow-up filed during review
    // advances (atelier-iaas).
    let mission = case.starts_with("follow_up").then(|| {
        let mission = ok(
            f.path(),
            &[
                "-q",
                "issue",
                "create",
                "Mission",
                "--issue-type",
                "mission",
            ],
        );
        commit(f.path(), "plan a mission");
        mission
    });
    if case == "open_fix_now" {
        ok(
            f.path(),
            &[
                "review",
                "finding",
                &f.id,
                "--identity",
                "Reviewer",
                "--recommendation",
                "fix_now",
                "Needs a fix",
            ],
        );
        commit(f.path(), "finding left open");
    }
    let mut new_done = None;
    if case == "reviewer_base_close_then_approve" {
        f.terminal(&f.id);
        approve("Reviewer");
    } else if case == "reviewer_base_reapproved_after_code" {
        approve("Lead");
        f.terminal(&f.id);
        fs::write(f.path().join("code"), "changed after the close\n").unwrap();
        commit(f.path(), "code after the close");
        approve("Reviewer");
    } else {
        let reviewer = if case == "self_approval" {
            "Author"
        } else {
            "Reviewer"
        };
        approve(reviewer);
        let close_issue = |id: &str| {
            ok(f.path(), &["issue", "transition", id, "start"]);
            mark_done(f.path(), id);
        };
        match case {
            "with_code" => fs::write(f.path().join("code"), "changed while closing\n").unwrap(),
            "with_goal" => {
                let path = f.path().join(format!(".atelier/issues/{}.md", f.id));
                let text = fs::read_to_string(&path).unwrap();
                fs::write(&path, text.replace("Ship session", "Ship more session")).unwrap();
            }
            "with_note" => {
                ok(f.path(), &["issue", "note", &f.id, "A note in the close"]);
            }
            "new_done_issue" => {
                let id = ok(f.path(), &["-q", "issue", "create", "Done elsewhere"]);
                close_issue(&id);
                new_done = Some(id);
            }
            "sibling_close" => close_issue(sibling.as_deref().unwrap()),
            "base_issue_edited" | "base_issue_edited_done" => {
                // An issue filed on the base after the approval, merged in
                // and then rewritten on the branch: planning, like a record
                // the base could take directly (atelier-g0mh).
                git(f.path(), &["switch", "-q", "main"]);
                let id = ok(f.path(), &["-q", "issue", "create", "Filed on the base"]);
                commit(f.path(), "file an issue on the base after the approval");
                git(f.path(), &["switch", "-q", &f.branch]);
                git(f.path(), &["merge", "-q", "--no-edit", "main"]);
                ok(
                    f.path(),
                    &[
                        "issue",
                        "update",
                        &id,
                        "--body",
                        "## Goal\n\nRewritten.\n\n## Done when\n\nRewritten too.",
                    ],
                );
                commit(f.path(), "rewrite the base issue on the branch");
                if case == "base_issue_edited_done" {
                    close_issue(&id);
                    new_done = Some(id);
                }
            }
            "follow_up_linked" | "follow_up_done" => {
                // Filed, then linked to its mission in a later commit, the
                // way an agent records a follow-up during review.
                let id = ok(
                    f.path(),
                    &[
                        "-q",
                        "issue",
                        "create",
                        "Follow-up found in review",
                        "--discovered-from",
                        &f.id,
                    ],
                );
                commit(f.path(), "file a follow-up after approval");
                ok(
                    f.path(),
                    &[
                        "issue",
                        "link",
                        &id,
                        mission.as_deref().unwrap(),
                        "--role",
                        "advances",
                    ],
                );
                commit(f.path(), "link the follow-up to its mission");
                if case == "follow_up_done" {
                    close_issue(&id);
                    new_done = Some(id);
                }
            }
            _ => {}
        }
        match (&child, case) {
            // Both claims in one commit with the owner's.
            (Some(child), "children_in_one_commit") => {
                let mut record = f.store().load_issue_by_id(child).unwrap();
                record.issue.status = "done".into();
                record.issue.closed_at = Some(chrono::Utc::now());
                record.sections.report = Some("completed claim".into());
                f.store().write_issue_atomic(&record).unwrap();
            }
            (Some(child), _) => f.terminal(child),
            _ => {}
        }
        f.terminal(&f.id);
    }
    let close = git(f.path(), &["rev-parse", "HEAD"]);

    let (pass, output) = cli(f.path(), &["merge", &f.id, "--check"]);
    if covered {
        assert!(
            pass && output.contains("claims.reviewed)"),
            "{label}: {output}"
        );
        ok(f.path(), &["merge", &f.id]);
        git(f.path(), &["switch", "-q", "main"]);
    } else {
        assert!(!pass, "{label}: {output}");
        if let Some(id) = &new_done {
            // Approval stays current (a new issue outside the scope is
            // planning), yet land would refuse the claim, so merge does.
            assert!(
                output.contains("review.current via")
                    && output.contains(&format!(
                        "claims.reviewed: commit {} marks {id} done",
                        &close[..8]
                    ))
                    && output.contains(&format!("from its own branch after `atelier start {id}`")),
                "{label}: {output}"
            );
        }
        // Integrate past the gate, so land on the base judges it too.
        git(f.path(), &["switch", "-q", "main"]);
        git(
            f.path(),
            &["merge", "-q", "--no-ff", "--no-edit", &f.branch],
        );
    }
    let (pass, output) = cli(f.path(), &["land"]);
    if covered {
        assert!(pass, "{label}: {output}");
    } else {
        let claimed = new_done.as_deref().unwrap_or("");
        assert!(
            !pass
                && output.contains(&format!(
                    "land_base_done_claim: commit {} on main marks {claimed}",
                    &close[..8]
                )),
            "{label}: {output}"
        );
    }
}
/// Declined work will not be done, so a base commit that only declines it is
/// no done claim, as for superseded work, and its open blockers do not hold
/// it: a blocked task and a loose end waiting on it both decline on the base
/// and land (atelier-stnv). The decline needs a Report.
#[test]
fn a_base_commit_that_only_declines_work_or_a_loose_end_lands() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        f.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);
    let blocker = ok(f.path(), &["-q", "issue", "create", "Prerequisite"]);
    let work = ok(
        f.path(),
        &["-q", "issue", "create", "Real but not worth it"],
    );
    ok(
        f.path(),
        &["issue", "link", &work, &blocker, "--role", "blocked_by"],
    );
    let loose_end = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "Tag the release",
            "--issue-type",
            "loose_end",
        ],
    );
    ok(
        f.path(),
        &[
            "issue",
            "link",
            &loose_end,
            &blocker,
            "--role",
            "blocked_by",
        ],
    );
    commit(f.path(), "plan on the base");
    git(f.path(), &["push", "-q"]);

    let (pass, output) = cli(f.path(), &["issue", "transition", &work, "decline"]);
    assert!(
        !pass && output.contains("missing required field close_reason"),
        "{output}"
    );
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &work,
            "decline",
            "--reason",
            "Real, but it costs more than it buys.",
        ],
    );
    commit(f.path(), "decline the work on the base");
    ok(f.path(), &["land"]);
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &loose_end,
            "decline",
            "--reason",
            "No release this cycle.",
        ],
    );
    commit(f.path(), "decline the loose end on the base");
    ok(f.path(), &["land"]);
    ok(f.path(), &["doctor"]);
    for id in [&work, &loose_end] {
        let show = ok(f.path(), &["issue", "show", id]);
        assert!(show.contains("declined"), "{show}");
    }
}
/// A repository whose policy predates `declined` loads and works as before:
/// check and land pass, no decline is offered or accepted, and superseding
/// on the base still lands (atelier-stnv).
#[test]
fn a_policy_without_declined_loads_and_works_unchanged() {
    let f = Fixture::under(Some(STARTER_BEFORE_DECLINED));
    ok(f.path(), &["check"]);
    git(f.path(), &["switch", "-q", "main"]);
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        f.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);
    let work = ok(f.path(), &["-q", "issue", "create", "Not needed"]);
    commit(f.path(), "plan on the base");
    let options = ok(f.path(), &["issue", "transition", &work]);
    assert!(
        options.contains("supersede") && !options.contains("decline"),
        "{options}"
    );
    let (pass, output) = cli(
        f.path(),
        &["issue", "transition", &work, "decline", "--reason", "no"],
    );
    assert!(!pass && output.contains("decline"), "{output}");
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &work,
            "supersede",
            "--reason",
            "obsolete",
        ],
    );
    commit(f.path(), "supersede on the base");
    ok(f.path(), &["land"]);
    ok(f.path(), &["doctor"]);
    ok(f.path(), &["work", "missions"]);
}
#[test]
fn superseding_a_child_of_a_blocked_parent_lands() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let blocker = ok(f.path(), &["-q", "issue", "create", "Prerequisite"]);
    let parent = ok(f.path(), &["-q", "issue", "create", "Parked parent"]);
    ok(
        f.path(),
        &["issue", "link", &parent, &blocker, "--role", "blocked_by"],
    );
    let child = ok(
        f.path(),
        &["-q", "issue", "create", "Child", "--parent", &parent],
    );
    commit(f.path(), "a parent waiting on a prerequisite");
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &child,
            "supersede",
            "--reason",
            "not needed",
        ],
    );
    commit(f.path(), "retire the child");
    // The work will not happen, so blockers inherited from its parent do not
    // hold its retirement (atelier-lq42).
    ok(f.path(), &["land"]);
}
#[test]
fn a_superseded_childs_own_blockers_do_not_hold_its_parents_merge() {
    let f = Fixture::new();
    let blocker = ok(f.path(), &["-q", "issue", "create", "Prerequisite"]);
    let child = ok(
        f.path(),
        &["-q", "issue", "create", "Child", "--parent", &f.id],
    );
    ok(
        f.path(),
        &["issue", "link", &child, &blocker, "--role", "blocked_by"],
    );
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &child,
            "supersede",
            "--reason",
            "not needed",
        ],
    );
    commit(f.path(), "retire a blocked child");
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    ok(f.path(), &["land", &f.id]);
    ok(f.path(), &["merge", &f.id]);
}
/// Declined like superseded: the child will not be done, so its blockers
/// do not hold the parent's merge (atelier-stnv).
#[test]
fn a_declined_childs_own_blockers_do_not_hold_its_parents_merge() {
    let f = Fixture::new();
    let blocker = ok(f.path(), &["-q", "issue", "create", "Prerequisite"]);
    let child = ok(
        f.path(),
        &["-q", "issue", "create", "Child", "--parent", &f.id],
    );
    ok(
        f.path(),
        &["issue", "link", &child, &blocker, "--role", "blocked_by"],
    );
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &child,
            "decline",
            "--reason",
            "not needed",
        ],
    );
    commit(f.path(), "decline a blocked child");
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    ok(f.path(), &["land", &f.id]);
    ok(f.path(), &["merge", &f.id]);
}
#[test]
fn land_recovery_advice_for_direct_base_code_ends_in_a_clean_merge() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        f.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);
    let owner = ok(f.path(), &["-q", "issue", "create", "Rescue direct code"]);
    commit(f.path(), "plan the rescue");
    git(f.path(), &["push", "-q"]);
    fs::write(f.path().join("direct"), "direct code\n").unwrap();
    commit(f.path(), "code straight on the base");

    // A branch made with plain Git still carries the direct commit.
    git(f.path(), &["switch", "-q", "-c", &owner]);
    let (pass, output) = cli(f.path(), &["start", &owner]);
    assert!(!pass && output.contains("land_base_unreviewed"), "{output}");

    // The advice: keep the commit on the work branch, reset the base, start.
    git(f.path(), &["branch", "-f", "main", "main@{upstream}"]);
    ok(f.path(), &["start", &owner]);
    commit(f.path(), "start the rescue");
    f.terminal(&owner);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &owner,
            "--identity",
            "Reviewer",
            "Reviewed the direct code",
        ],
    );
    commit(f.path(), "approval");
    ok(f.path(), &["merge", &owner]);
    git(f.path(), &["switch", "-q", "main"]);
    // The approval's record commit after its pin; the merge commit adds
    // only covered done marks.
    let output = ok(f.path(), &["land"]);
    assert!(
        output.contains("Base commits: 1 unpushed record-only commit outside"),
        "{output}"
    );
}
#[test]
fn base_pull_merge_counts_only_the_paths_it_introduces() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    let remote = upstream.path().to_str().unwrap();
    git(f.path(), &["remote", "add", "origin", remote]);
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);
    let other = tempdir().unwrap();
    git(other.path(), &["clone", "-q", "-b", "main", remote, "."]);
    git(other.path(), &["config", "user.name", "Other"]);
    git(
        other.path(),
        &["config", "user.email", "other@example.test"],
    );
    fs::write(other.path().join("upstream-code"), "reviewed upstream\n").unwrap();
    commit(other.path(), "code that landed upstream");
    git(other.path(), &["push", "-q", "origin", "main"]);

    ok(f.path(), &["-q", "issue", "create", "Local planning"]);
    commit(f.path(), "record-only planning on the base");
    git(f.path(), &["pull", "-q", "--no-rebase", "--no-edit"]);
    // The clean pull merge introduces nothing, so only the planning commit
    // counts.
    let output = ok(f.path(), &["land"]);
    assert!(
        output.contains("Base commits: 1 unpushed record-only commit outside"),
        "{output}"
    );
}
#[test]
fn land_on_a_branch_behind_its_base_hands_off_and_says_merge_lands_on_it() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    fs::write(f.path().join("other"), "landed on the base\n").unwrap();
    commit(f.path(), "other work landed on the base");
    git(f.path(), &["switch", "-q", &f.branch]);

    // Active work hands off; the base moving on is not its failure.
    let output = ok(f.path(), &["land", &f.id]);
    assert!(
        output.contains("Base main has 1 commit this branch does not")
            && output.contains("atelier merge --sync"),
        "{output}"
    );

    // Work an older Atelier closed is still checked against where it left
    // the base, and the note survives a failing gate.
    f.terminal(&f.id);
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass
            && output.contains("review.current: no independent approval or trivial exemption")
            && output.contains("Base main has 1 commit this branch does not"),
        "{output}"
    );
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    let output = ok(f.path(), &["land", &f.id]);
    assert!(
        output.contains("Base main has 1 commit this branch does not")
            && output.contains("Next: atelier merge"),
        "{output}"
    );
    // Merge lands it on the moved base with no catch-up merge.
    ok(f.path(), &["merge", &f.id]);
}

/// Merge always creates a merge commit, so a branch behind its base merges
/// without catching up first, and land on the base accepts the result
/// (atelier-jnrh).
#[test]
fn a_branch_behind_its_base_merges_without_catching_up() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    fs::write(f.path().join("other"), "landed on the base\n").unwrap();
    commit(f.path(), "other work landed on the base");
    let base = git(f.path(), &["rev-parse", "HEAD"]);
    git(f.path(), &["switch", "-q", &f.branch]);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    let head = git(f.path(), &["rev-parse", "HEAD"]);
    let output = ok(f.path(), &["merge", &f.id]);
    assert!(
        output.contains(&format!("which marks {} done", f.id)),
        "{output}"
    );
    assert_eq!(git(f.path(), &["rev-parse", "main^1"]), base);
    assert_eq!(git(f.path(), &["rev-parse", "main^2"]), head);
    let record = fs::read_to_string(f.path().join(format!(".atelier/issues/{}.md", f.id))).unwrap();
    assert!(record.contains("status: \"done\""), "{record}");
    ok(f.path(), &["land"]);
}
#[test]
fn land_names_a_base_with_no_shared_history() {
    let f = Fixture::new();
    // The work branch itself is replaced by history the base does not share.
    git(f.path(), &["checkout", "-q", "--orphan", "unrelated"]);
    commit(f.path(), "the same tree with no shared history");
    git(f.path(), &["branch", "-D", &f.branch]);
    git(f.path(), &["branch", "-m", &f.branch]);
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass
            && output.contains("land_base_unrelated: main shares no history with this checkout")
            && output.contains(&format!("atelier start {}", f.id)),
        "{output}"
    );
}
#[test]
fn approval_from_before_a_clean_base_merge_lands_on_both_branches() {
    let f = Fixture::new();
    let lines = |first: &str, last: &str| format!("{first}\n2\n3\n4\n5\n6\n7\n8\n{last}\n");
    git(f.path(), &["switch", "-q", "main"]);
    fs::write(f.path().join("shared"), lines("1", "9")).unwrap();
    commit(f.path(), "shared file before the branch's work");
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    let remote = upstream.path().to_str().unwrap();
    git(f.path(), &["remote", "add", "origin", remote]);
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    let finding = ok(
        f.path(),
        &[
            "-q",
            "review",
            "finding",
            &f.id,
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "Code needs a fix",
        ],
    );
    commit(f.path(), "finding");
    fs::write(f.path().join("code"), "fixed code\n").unwrap();
    fs::write(f.path().join("shared"), lines("branch", "9")).unwrap();
    commit(f.path(), "fix");
    let fix = git(f.path(), &["rev-parse", "HEAD"]);
    ok(
        f.path(),
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "fix_now",
            "--resolution-commit",
            &fix,
            "Fix verified",
        ],
    );
    commit(f.path(), "disposition");
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");

    // Other reviewed work lands upstream; the branch merges it in.
    let other = tempdir().unwrap();
    git(other.path(), &["clone", "-q", "-b", "main", remote, "."]);
    git(other.path(), &["config", "user.name", "Other"]);
    git(
        other.path(),
        &["config", "user.email", "other@example.test"],
    );
    // The base edits the far end of a file the branch also edited, so the
    // merge combines both sides of one file.
    fs::write(other.path().join("shared"), lines("1", "upstream")).unwrap();
    commit(other.path(), "code that landed upstream");
    git(other.path(), &["push", "-q", "origin", "main"]);
    git(f.path(), &["fetch", "-q"]);
    git(f.path(), &["branch", "-f", "main", "origin/main"]);
    git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    assert_eq!(
        fs::read_to_string(f.path().join("shared")).unwrap(),
        lines("branch", "upstream")
    );

    ok(f.path(), &["land", &f.id]);
    ok(f.path(), &["merge", &f.id]);
    git(f.path(), &["switch", "-q", "main"]);
    let output = ok(f.path(), &["land"]);
    assert!(output.contains("Base commits:"), "{output}");
}
#[test]
fn base_land_retains_claims_a_merge_tip_brings_in_through_its_other_parent() {
    let f = Fixture::new();
    let approve = |id: &str| {
        ok(
            f.path(),
            &[
                "review",
                "approve",
                id,
                "--identity",
                "Reviewer",
                "Independent review",
            ],
        );
        commit(f.path(), "approval");
    };
    // A second owner branches from the base before the first one merges.
    git(f.path(), &["switch", "-q", "main"]);
    let second = ok(f.path(), &["-q", "issue", "create", "Second owner"]);
    commit(f.path(), "plan the second owner");
    ok(f.path(), &["start", &second]);
    fs::write(f.path().join("second"), "second code\n").unwrap();
    commit(f.path(), "second owner's code");
    f.terminal(&second);
    approve(&second);

    // The first owner merges with a fixed FIX NOW finding in its history.
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    let finding = ok(
        f.path(),
        &[
            "-q",
            "review",
            "finding",
            &f.id,
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "Code needs a fix",
        ],
    );
    commit(f.path(), "finding");
    fs::write(f.path().join("code"), "fixed code\n").unwrap();
    commit(f.path(), "fix");
    let fix = git(f.path(), &["rev-parse", "HEAD"]);
    ok(
        f.path(),
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "fix_now",
            "--resolution-commit",
            &fix,
            "Fix verified",
        ],
    );
    commit(f.path(), "disposition");
    f.terminal(&f.id);
    approve(&f.id);
    ok(f.path(), &["merge", &f.id]);
    // Other code lands on the base after the first owner's approval.
    fs::write(f.path().join("later"), "later base code\n").unwrap();
    commit(f.path(), "later code on the base");

    // The second owner merges the base in last, so the base's new tip is a
    // merge whose first parent is the second owner's side.
    git(f.path(), &["switch", "-q", &second]);
    git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    ok(f.path(), &["merge", &second]);
    git(f.path(), &["switch", "-q", "main"]);
    assert_eq!(
        git(f.path(), &["rev-list", "--parents", "-n", "1", "HEAD"])
            .split(' ')
            .count(),
        3,
        "the base tip is a merge commit"
    );
    ok(f.path(), &["land"]);
}
#[test]
fn a_kept_branch_whose_done_owner_was_pruned_from_the_base_shows_merged() {
    let f = Fixture::new();
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    let tip = git(f.path(), &["rev-parse", "HEAD"]);
    ok(f.path(), &["merge", &f.id]);
    git(f.path(), &["switch", "-q", "main"]);
    // Branch cleanup kept the work branch, and prune later removed the
    // owner's record from the base.
    git(f.path(), &["branch", &f.branch, &tip]);
    git(
        f.path(),
        &["rm", "-q", "-r", &format!(".atelier/issues/{}.md", f.id)],
    );
    git(f.path(), &["rm", "-q", "-r", ".atelier/reviews"]);
    commit(f.path(), "prune the merged owner");
    git(f.path(), &["switch", "-q", &f.branch]);
    for command in ["prime", "status"] {
        let output = ok(f.path(), &[command]);
        assert!(
            output.contains(&format!("{} ✓ done ", f.id)) && output.contains("| merged into main"),
            "{command}: {output}"
        );
    }
    // An uncommitted close on a fresh branch from the base is not merged.
    git(f.path(), &["switch", "-q", "main"]);
    let fresh = ok(f.path(), &["-q", "issue", "create", "Fresh work"]);
    commit(f.path(), "plan fresh work");
    ok(f.path(), &["start", &fresh]);
    commit(f.path(), "start fresh work");
    mark_done(f.path(), &fresh);
    let output = ok(f.path(), &["status"]);
    assert!(output.contains("not merged into main"), "{output}");
}
#[test]
fn start_and_status_warn_when_a_branch_owner_has_a_needs_human_child() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let parent = ok(f.path(), &["-q", "issue", "create", "Feature"]);
    let check = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "Play-test the release build",
            "--issue-type",
            "task",
            "--parent",
            &parent,
            "-l",
            "needs-human",
        ],
    );
    commit(f.path(), "a feature with its game check as a child");
    let output = ok(f.path(), &["start", &parent]);
    assert!(
        output.contains(&format!(
            "{check} labelled needs-human is a child of {parent}"
        )) && output.contains(&format!(
            "atelier issue update {check} --no-parent --issue-type loose_end"
        )),
        "{output}"
    );
    commit(f.path(), "start the feature");
    let output = ok(f.path(), &["status"]);
    assert!(
        output.contains(&format!(
            "Needs a human: {check} labelled needs-human is a child of {parent}"
        )),
        "{output}"
    );

    // Following the advice makes the check a loose end that waits on the
    // merge.
    ok(
        f.path(),
        &[
            "issue",
            "update",
            &check,
            "--no-parent",
            "--issue-type",
            "loose_end",
        ],
    );
    ok(
        f.path(),
        &["issue", "link", &check, &parent, "--role", "blocked_by"],
    );
    commit(f.path(), "make the check a loose end");
    let output = ok(f.path(), &["status"]);
    assert!(
        !output.contains("labelled needs-human is a child"),
        "{output}"
    );
}
#[test]
fn pruned_owner_labels_cover_other_branch_rows_and_an_uncommitted_status() {
    let f = Fixture::new();
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    let tip = git(f.path(), &["rev-parse", "HEAD"]);
    ok(f.path(), &["merge", &f.id]);
    git(f.path(), &["switch", "-q", "main"]);
    git(f.path(), &["branch", &f.branch, &tip]);
    git(
        f.path(),
        &["rm", "-q", &format!(".atelier/issues/{}.md", f.id)],
    );
    git(f.path(), &["rm", "-q", "-r", ".atelier/reviews"]);
    commit(f.path(), "prune the merged owner");

    // The kept branch's row on the base reads its committed records.
    let output = ok(f.path(), &["status"]);
    assert!(
        output.contains(&format!(
            "{}  {} ✓ done, merged into main  ",
            f.branch, f.id
        )),
        "{output}"
    );

    // On the kept branch, an uncommitted status the tip does not have is
    // not merged, though the tip is in the base and the base has no copy.
    git(f.path(), &["switch", "-q", &f.branch]);
    let mut record = f.store().load_issue_by_id(&f.id).unwrap();
    record.issue.status = "superseded".into();
    f.store().write_issue_atomic(&record).unwrap();
    let output = ok(f.path(), &["status"]);
    assert!(
        field(&output, "Owner").is_some_and(|owner| owner
            .starts_with(&format!("{} ✓ superseded ", f.id))
            && owner.ends_with(" | not merged into main")),
        "{output}"
    );
}
/// With no commits yet, as right after `git init` and `atelier init`,
/// `init`, `status` and `check` work, and `land`, `doctor` and the Stop hook
/// say there is nothing to land, name the install commit and exit
/// successfully; after that commit, `land` on the base passes (atelier-l457).
#[test]
fn an_unborn_head_is_reported_and_holds_nothing_up() {
    let dir = tempdir().unwrap();
    let repo = dir.path();
    git(repo, &["init", "-qb", "master"]);
    git(repo, &["config", "user.name", "Author"]);
    git(repo, &["config", "user.email", "author@example.test"]);
    let init = ok(repo, &["init", "--slug", "atelier"]);
    assert!(
        init.contains("Base branch: master (from the current branch)"),
        "{init}"
    );
    let install = "git add .atelier .gitignore && git commit -m \"Install Atelier\"";

    let status = ok(repo, &["status"]);
    assert_eq!(
        field(&status, "Branch"),
        Some("master (no commits yet)"),
        "{status}"
    );
    assert!(
        field(&status, "Checkout").is_some_and(|checkout| checkout.starts_with("dirty (3 paths")),
        "{status}"
    );
    assert!(
        status.ends_with(&format!(
            "Next\n----\n  Commit the Atelier install: {install}"
        )),
        "{status}"
    );
    assert_eq!(ok(repo, &["check"]), "Check passed.");

    let report = format!(
        "No commits yet on master, so there is nothing to land. Next: commit the Atelier install with `{install}`"
    );
    let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(repo)
        .arg("land")
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("{report}\nland: passed\n")
    );
    assert_eq!(
        hook(
            repo,
            "land",
            serde_json::json!({"hook_event_name": "Stop", "cwd": repo, "stop_hook_active": false})
        ),
        serde_json::json!({"systemMessage": format!("Atelier: {report}")})
    );
    assert!(ok(repo, &["doctor"]).starts_with("No commits yet on master"));

    let committed = Command::new("sh")
        .current_dir(repo)
        .args(["-c", install])
        .output()
        .unwrap();
    assert!(committed.status.success(), "{committed:?}");
    assert!(ok(repo, &["land"]).ends_with("land: passed"));
    assert_eq!(
        hook(
            repo,
            "land",
            serde_json::json!({"hook_event_name": "Stop", "cwd": repo, "stop_hook_active": false})
        ),
        serde_json::json!({})
    );
}

/// cosplay-studio's history holds review records in the form before
/// Markdown, `.atelier/reviews/<id>.yaml` files with no front matter, which
/// the commit that migrated its records deleted. They are not review
/// records Atelier reads, so the append-only check of the whole history
/// passes over them, and `doctor` passes (atelier-l457).
#[test]
fn doctor_passes_over_review_files_in_the_form_before_markdown() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let old = f.path().join(".atelier/reviews/atelier-old1.yaml");
    fs::write(
        &old,
        "created_at: \"2026-07-12T06:23:03Z\"\nevents:\n  - actor: reviewer\n    kind: approval\nid: \"atelier-old1\"\nissue: \"atelier-gone\"\n",
    )
    .unwrap();
    commit(f.path(), "a review record in the form before Markdown");
    fs::remove_file(&old).unwrap();
    commit(f.path(), "migrate the records to Markdown");
    ok(f.path(), &["doctor"]);
    ok(f.path(), &["land"]);
}
#[test]
fn land_checks_only_changed_claims_and_doctor_checks_every_claim() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    ok(f.path(), &["doctor"]);
    // A done claim with an open blocker reaches the base without the gates,
    // as a direct push would.
    let old = ok(f.path(), &["-q", "issue", "create", "Pushed directly"]);
    let blocker = ok(f.path(), &["-q", "issue", "create", "Still open"]);
    commit(f.path(), "plan it");
    ok(f.path(), &["issue", "transition", &old, "start"]);
    mark_done(f.path(), &old);
    commit(f.path(), "done without the gates");
    let mut record = f.store().load_issue_by_id(&blocker).unwrap();
    record
        .relationships
        .blocks
        .push(atelier_records::issue_relationship_target(&old));
    f.store().write_issue_atomic(&record).unwrap();
    commit(f.path(), "an open blocker, also without the gates");
    // A record about the claim: a receipt, which gates nothing itself.
    let receipt = |summary: &str| {
        ok(
            f.path(),
            &[
                "evidence",
                "record",
                "--target",
                &format!("issue/{old}"),
                "--kind",
                "test",
                "--result",
                "pass",
                summary,
            ],
        );
    };
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        f.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);

    // On the base and on a work branch, land does not re-check it.
    ok(f.path(), &["land"]);
    let (pass, output) = cli(f.path(), &["doctor"]);
    assert!(
        !pass && output.contains(&format!("{old} blockers.transitive_none_open")),
        "{output}"
    );
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    ok(f.path(), &["land", &f.id]);
    let (pass, _) = cli(f.path(), &["doctor"]);
    assert!(!pass);

    // The gates come from the base's policy, which can change after the
    // branch left it.
    git(f.path(), &["switch", "-q", "main"]);
    let base_policy = f.path().join(".atelier/workflow.yaml");
    let mut base_revised = fs::read_to_string(&base_policy).unwrap();
    base_revised.push_str("# Revised policy.\n");
    fs::write(&base_policy, base_revised).unwrap();
    commit(f.path(), "revise the policy on the base");
    git(f.path(), &["switch", "-q", &f.branch]);
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass && output.contains(&format!("{old} blockers.transitive_none_open")),
        "{output}"
    );
    git(f.path(), &["switch", "-q", "main"]);
    git(f.path(), &["reset", "-q", "--hard", "HEAD^"]);
    git(f.path(), &["switch", "-q", &f.branch]);

    // A branch that adds a record about it, or edits it, checks it again.
    receipt("Another run");
    commit(f.path(), "a receipt on the branch");
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass && output.contains(&format!("{old} blockers.transitive_none_open")),
        "{output}"
    );
    git(f.path(), &["reset", "-q", "--hard", "HEAD^"]);
    ok(f.path(), &["land", &f.id]);
    let path = f.path().join(format!(".atelier/issues/{old}.md"));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        text.replace("title: \"Pushed directly\"", "title: \"Edited\""),
    )
    .unwrap();
    commit(f.path(), "edit the old claim");
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass && output.contains(&format!("{old} blockers.transitive_none_open")),
        "{output}"
    );

    // It is still checked once a later commit leaves it alone.
    git(
        f.path(),
        &["commit", "-q", "--allow-empty", "-m", "later work"],
    );
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass && output.contains(&format!("{old} blockers.transitive_none_open")),
        "{output}"
    );

    // A policy change checks every claim under the new policy.
    git(f.path(), &["reset", "-q", "--hard", "HEAD~2"]);
    let policy = f.path().join(".atelier/workflow.yaml");
    let mut revised = fs::read_to_string(&policy).unwrap();
    revised.push_str("# Revised policy.\n");
    fs::write(&policy, revised).unwrap();
    commit(f.path(), "revise the policy");
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass && output.contains(&format!("{old} blockers.transitive_none_open")),
        "{output}"
    );
    git(f.path(), &["reset", "-q", "--hard", "HEAD^"]);
    ok(
        f.path(),
        &[
            "review",
            "finding",
            &old,
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "The blocker is still open",
        ],
    );
    commit(f.path(), "finding on the old claim");
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass && output.contains(&format!("{old} blockers.transitive_none_open")),
        "{output}"
    );

    // On the base, a claim edited since the upstream is checked though the
    // last commit left it alone.
    git(f.path(), &["switch", "-q", "main"]);
    fs::write(
        &path,
        text.replace("title: \"Pushed directly\"", "title: \"Edited\""),
    )
    .unwrap();
    commit(f.path(), "edit the old claim on the base");
    git(
        f.path(),
        &["commit", "-q", "--allow-empty", "-m", "later work"],
    );
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass && output.contains(&format!("{old} blockers.transitive_none_open")),
        "{output}"
    );
}
#[test]
fn land_checks_claims_that_linked_records_affect() {
    // An older policy, whose work `close` validators judge the claims; merge
    // marks work done under the current one (atelier-jnrh).
    let f = Fixture::under(Some(STARTER_WITH_BLOCKED_AND_REVIEW));
    git(f.path(), &["switch", "-q", "main"]);
    let old = ok(f.path(), &["-q", "issue", "create", "Pushed directly"]);
    let child = ok(
        f.path(),
        &["-q", "issue", "create", "--parent", &old, "Done child"],
    );
    commit(f.path(), "plan it");
    for id in [&child, &old] {
        ok(f.path(), &["issue", "transition", id, "start"]);
        mark_done(f.path(), id);
    }
    // A done child of an open parent, and a done item behind a superseded
    // blocker.
    let parent = ok(f.path(), &["-q", "issue", "create", "Open parent"]);
    let nested = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "--parent",
            &parent,
            "Done nested child",
        ],
    );
    let retired = ok(f.path(), &["-q", "issue", "create", "Retired blocker"]);
    let behind = ok(f.path(), &["-q", "issue", "create", "Done behind it"]);
    ok(
        f.path(),
        &["issue", "link", &behind, &retired, "--role", "blocked_by"],
    );
    commit(f.path(), "plan the chains");
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &retired,
            "supersede",
            "--reason",
            "obsolete",
        ],
    );
    for id in [&nested, &behind] {
        ok(f.path(), &["issue", "transition", id, "start"]);
        mark_done(f.path(), id);
    }
    commit(f.path(), "done without the gates");
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        f.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    ok(f.path(), &["land", &f.id]);
    let refused = |check: &str| {
        let (pass, output) = cli(f.path(), &["land", &f.id]);
        assert!(
            !pass && output.contains(&format!("{old} {check}")),
            "{check}: {output}"
        );
        git(f.path(), &["reset", "-q", "--hard", "HEAD^"]);
    };

    // A blocker link is written only to the blocked record (atelier-ihss).
    let blocker = ok(f.path(), &["-q", "issue", "create", "Open blocker"]);
    ok(
        f.path(),
        &["issue", "link", &old, &blocker, "--role", "blocked_by"],
    );
    commit(f.path(), "block the done claim");
    refused("blockers.none_open");

    // Blockers pass from a parent to its children and through retired
    // blockers.
    for (link, claim) in [(&parent, &nested), (&retired, &behind)] {
        let blocker = ok(f.path(), &["-q", "issue", "create", "Open blocker"]);
        ok(
            f.path(),
            &["issue", "link", link, &blocker, "--role", "blocked_by"],
        );
        commit(f.path(), "block the chain");
        let (pass, output) = cli(f.path(), &["land", &f.id]);
        assert!(
            !pass && output.contains(&format!("{claim} blockers.transitive_none_open")),
            "{output}"
        );
        git(f.path(), &["reset", "-q", "--hard", "HEAD^"]);
    }

    // An open child changes the parent's record only in its children.
    ok(
        f.path(),
        &["-q", "issue", "create", "--parent", &old, "Open child"],
    );
    commit(f.path(), "open a child of the done claim");
    refused("work.terminal");

    // Reopening a child changes only the child's record.
    let path = f.path().join(format!(".atelier/issues/{child}.md"));
    let text = fs::read_to_string(&path).unwrap();
    assert!(text.contains("status: \"done\""), "{text}");
    fs::write(&path, text.replace("status: \"done\"", "status: \"todo\"")).unwrap();
    commit(f.path(), "reopen the child");
    refused("work.terminal");

    // Off the base, history is trusted only up to the base's upstream: the
    // local base can hold commits that no land has checked.
    git(f.path(), &["switch", "-q", "main"]);
    let unpushed = ok(f.path(), &["-q", "issue", "create", "Unpushed blocker"]);
    ok(
        f.path(),
        &["issue", "link", &old, &unpushed, "--role", "blocked_by"],
    );
    commit(f.path(), "unpushed blocker of the done claim");
    git(f.path(), &["switch", "-q", "-c", "feature"]);
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass && output.contains(&format!("{old} blockers.none_open")),
        "{output}"
    );

    // So is an owner's branch started from that local base commit.
    git(f.path(), &["switch", "-q", "main"]);
    let owner = ok(f.path(), &["-q", "issue", "create", "Started from main"]);
    commit(f.path(), "plan owned work");
    ok(f.path(), &["start", &owner]);
    commit(f.path(), "start owned work");
    let (pass, output) = cli(f.path(), &["land", &owner]);
    assert!(
        !pass && output.contains(&format!("{old} blockers.none_open")),
        "{output}"
    );

    // Without an upstream, nothing is trusted.
    git(f.path(), &["branch", "--unset-upstream", "main"]);
    let (pass, output) = cli(f.path(), &["land", &owner]);
    assert!(
        !pass && output.contains(&format!("{old} blockers.none_open")),
        "{output}"
    );
}
#[test]
fn land_checks_a_mission_whose_advancing_work_reopens() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    // A policy whose missions close only once their advancing work is done.
    let policy = f.path().join(".atelier/workflow.yaml");
    let text = fs::read_to_string(&policy).unwrap();
    let objective = text.find("  objective:").unwrap();
    let close = objective
        + text[objective..]
            .find("        - blockers.none_open\n")
            .unwrap();
    let mut revised = text.clone();
    revised.insert_str(close, "        - work.terminal\n");
    fs::write(&policy, revised).unwrap();
    let mission = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "--issue-type",
            "mission",
            "--body",
            "## Outcome\n\nShipped.\n\n## Success criteria\n\nIt ships.",
            "Mission",
        ],
    );
    let work = ok(f.path(), &["-q", "issue", "create", "Advancing work"]);
    ok(f.path(), &["issue", "link", &work, &mission]);
    commit(f.path(), "plan the mission");
    ok(f.path(), &["issue", "transition", &work, "start"]);
    mark_done(f.path(), &work);
    for transition in ["ready", "start"] {
        ok(f.path(), &["issue", "transition", &mission, transition]);
    }
    ok(
        f.path(),
        &["issue", "transition", &mission, "close", "--reason", "done"],
    );
    commit(f.path(), "complete the mission without the gates");
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        f.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    ok(f.path(), &["land", &f.id]);

    // Reopening the work changes only its own record.
    let path = f.path().join(format!(".atelier/issues/{work}.md"));
    let text = fs::read_to_string(&path).unwrap();
    assert!(text.contains("status: \"done\""), "{text}");
    fs::write(&path, text.replace("status: \"done\"", "status: \"todo\"")).unwrap();
    commit(f.path(), "reopen the advancing work");
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass && output.contains(&format!("{mission} work.terminal")),
        "{output}"
    );
}
#[test]
fn land_checks_every_claim_after_a_record_change_under_a_lint_gate() {
    // An older policy, whose work `close` validators judge the claims; merge
    // marks work done under the current one (atelier-jnrh).
    let f = Fixture::under(Some(STARTER_WITH_BLOCKED_AND_REVIEW));
    git(f.path(), &["switch", "-q", "main"]);
    // A policy whose work closes only while the whole tracker lints clean.
    let policy = f.path().join(".atelier/workflow.yaml");
    let text = fs::read_to_string(&policy).unwrap();
    let work = text.find("  work:").unwrap();
    let close = work + text[work..].find("        - blockers.none_open\n").unwrap();
    let mut revised = text.clone();
    revised.insert_str(close, "        - lint.none_blocking\n");
    fs::write(&policy, revised).unwrap();
    let old = ok(f.path(), &["-q", "issue", "create", "Done before"]);
    commit(f.path(), "plan it");
    ok(f.path(), &["issue", "transition", &old, "start"]);
    mark_done(f.path(), &old);
    commit(f.path(), "done without the gates");
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        f.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    ok(f.path(), &["land", &f.id]);

    // A lint finding on an unrelated record fails the old claim's gate.
    let unrelated = ok(f.path(), &["-q", "issue", "create", "Unrelated"]);
    let path = f.path().join(format!(".atelier/issues/{unrelated}.md"));
    let text = fs::read_to_string(&path).unwrap();
    assert!(text.contains("labels: []"), "{text}");
    fs::write(
        &path,
        text.replace("labels: []", "labels: [\"size-m\", \"size-s\"]"),
    )
    .unwrap();
    commit(f.path(), "an unrelated record with conflicting labels");
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass && output.contains(&format!("{old} lint.none_blocking")),
        "{output}"
    );
}
#[test]
fn an_adopted_repositorys_branch_cannot_pass_as_an_install() {
    let dir = tempdir().unwrap();
    git(dir.path(), &["init", "-qb", "main"]);
    git(dir.path(), &["config", "user.name", "Author"]);
    git(dir.path(), &["config", "user.email", "author@example.test"]);
    fs::write(dir.path().join("app"), "existing code\n").unwrap();
    commit(dir.path(), "before adoption");
    git(dir.path(), &["branch", "old"]);
    ok(dir.path(), &["init", "--slug", "adopted"]);
    commit(dir.path(), "adopt Atelier on main");
    // An unowned branch points its policy's base at the pre-adoption branch,
    // so land sees a base without a policy (atelier-bibd).
    git(dir.path(), &["switch", "-q", "-c", "sneaky"]);
    let policy = dir.path().join(".atelier/workflow.yaml");
    let text = fs::read_to_string(&policy).unwrap();
    fs::write(
        &policy,
        text.replace("base_branch: main", "base_branch: old"),
    )
    .unwrap();
    ok(dir.path(), &["-q", "issue", "create", "Untracked planning"]);
    commit(dir.path(), "point the base at a pre-adoption branch");
    let (pass, output) = cli(dir.path(), &["land"]);
    assert!(
        !pass
            && output.contains("land_install_unconfirmed")
            && !output.contains("Installing Atelier"),
        "{output}"
    );
    // On an unowned branch of the adopted repository, the flag is refused.
    git(dir.path(), &["switch", "-q", "-c", "feature", "main"]);
    ok(dir.path(), &["land"]);
    let (pass, output) = cli(dir.path(), &["land", "--install"]);
    assert!(
        !pass && output.contains("land_install_invalid: main already has"),
        "{output}"
    );
    // An owner's branch cannot claim to be an install either.
    git(dir.path(), &["switch", "-q", "main"]);
    let id = ok(dir.path(), &["-q", "issue", "create", "Owned work"]);
    commit(dir.path(), "plan owned work");
    ok(dir.path(), &["start", &id]);
    commit(dir.path(), "start owned work");
    let (pass, output) = cli(dir.path(), &["land", "--install"]);
    assert!(
        !pass && output.contains("not for an owner's branch or the base. Next: atelier land"),
        "{output}"
    );
}
#[test]
fn work_starts_on_an_adoption_branch_before_the_base_has_atelier() {
    let dir = tempdir().unwrap();
    git(dir.path(), &["init", "-qb", "main"]);
    git(dir.path(), &["config", "user.name", "Author"]);
    git(dir.path(), &["config", "user.email", "author@example.test"]);
    fs::write(dir.path().join("app"), "existing code\n").unwrap();
    commit(dir.path(), "before adoption");
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        dir.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(dir.path(), &["push", "-q", "-u", "origin", "main"]);
    git(dir.path(), &["switch", "-q", "-c", "adopt"]);
    ok(dir.path(), &["init", "--slug", "adopter"]);
    let id = ok(dir.path(), &["-q", "issue", "create", "First tracked work"]);
    commit(dir.path(), "install Atelier on a branch");
    ok(dir.path(), &["start", &id]);
}
#[test]
fn base_commit_that_installs_atelier_is_exempt_from_review_coverage() {
    let dir = tempdir().unwrap();
    git(dir.path(), &["init", "-qb", "main"]);
    git(dir.path(), &["config", "user.name", "Author"]);
    git(dir.path(), &["config", "user.email", "author@example.test"]);
    fs::write(dir.path().join("app"), "existing code\n").unwrap();
    commit(dir.path(), "before adoption");
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        dir.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(dir.path(), &["push", "-q", "-u", "origin", "main"]);
    ok(dir.path(), &["init", "--slug", "adopter"]);
    fs::write(dir.path().join("AGENTS.md"), "Use Atelier.\n").unwrap();
    commit(dir.path(), "install Atelier");
    let output = ok(dir.path(), &["land"]);
    assert!(
        output.contains("No unpushed base commits need review"),
        "{output}"
    );
    // Code committed on the base after the install is gated even before the
    // install is pushed.
    fs::write(dir.path().join("after"), "unreviewed\n").unwrap();
    commit(dir.path(), "code after the install");
    let (pass, output) = cli(dir.path(), &["land"]);
    assert!(!pass && output.contains("land_base_unreviewed"), "{output}");
}
#[test]
fn status_current_work_row_reads_like_its_owner_row() {
    let f = Fixture::new();
    let status = ok(f.path(), &["status"]);
    let owner = field(&status, "Owner").unwrap_or_else(|| panic!("no owner: {status}"));
    assert!(
        owner.starts_with(&format!("{} in_progress ", f.id)),
        "{status}"
    );
    assert_eq!(
        section(&status, "Current Work")
            .iter()
            .map(|row| row.trim())
            .collect::<Vec<_>>(),
        [owner],
        "{status}"
    );
}
#[test]
fn done_claims_on_work_branches_say_they_are_not_merged() {
    let f = Fixture::new();
    // While the owner is in progress, its branch row counts it as active,
    // with how it would merge, since merge is what marks it done.
    git(f.path(), &["switch", "-q", "main"]);
    let status = ok(f.path(), &["status"]);
    assert!(
        status.lines().any(|line| line.starts_with(&format!(
            "{}  {} in_progress, ⚠ not approved, behind 0, merges cleanly  ",
            f.branch, f.id
        ))),
        "{status}"
    );
    git(f.path(), &["switch", "-q", &f.branch]);
    f.terminal(&f.id);
    let status = ok(f.path(), &["status"]);
    assert!(
        field(&status, "Owner")
            .is_some_and(|owner| owner.starts_with(&format!("{} ✓ done ", f.id))
                && owner.ends_with(" | not merged into main")),
        "{status}"
    );

    git(f.path(), &["switch", "-q", "main"]);
    let status = ok(f.path(), &["status"]);
    assert!(
        status
            .lines()
            .any(|line| line.starts_with(&format!("{}  {} ✓ done, not merged, ", f.branch, f.id))),
        "{status}"
    );
    git(f.path(), &["merge", "-q", "--ff-only", &f.branch]);
    git(f.path(), &["switch", "-q", "-c", "elsewhere"]);
    let status = ok(f.path(), &["status"]);
    assert!(
        status.lines().any(|line| line
            == format!(
                "{}  {} ✓ done, merged into main  Session owner",
                f.branch, f.id
            )),
        "{status}"
    );

    // An uncommitted close on a fresh branch is not on the base either.
    git(f.path(), &["switch", "-q", "main"]);
    let fresh = ok(f.path(), &["-q", "issue", "create", "Fresh owner"]);
    commit(f.path(), "plan fresh owner");
    ok(f.path(), &["start", &fresh]);
    mark_done(f.path(), &fresh);
    let status = ok(f.path(), &["status"]);
    assert!(
        field(&status, "Owner").is_some_and(|owner| owner.starts_with(&format!("{fresh} ✓ done "))
            && owner.ends_with(" | not merged into main")),
        "{status}"
    );
}
#[test]
fn ready_work_in_flight_on_another_branch_is_marked_with_its_state_there() {
    // A policy from the earlier starter, whose `blocked` status shows here.
    let f = Fixture::under(Some(STARTER_WITH_BLOCKED_AND_REVIEW));
    git(f.path(), &["switch", "-q", "main"]);
    let finished = ok(f.path(), &["-q", "issue", "create", "Finished elsewhere"]);
    let idle = ok(f.path(), &["-q", "issue", "create", "Nobody's work"]);
    let stuck = ok(f.path(), &["-q", "issue", "create", "Stuck elsewhere"]);
    commit(f.path(), "plan more work");
    ok(f.path(), &["start", &finished]);
    f.terminal(&finished);
    git(f.path(), &["switch", "-q", "main"]);
    ok(f.path(), &["start", &stuck]);
    ok(f.path(), &["issue", "transition", &stuck, "block"]);
    commit(f.path(), "blocked elsewhere");
    git(f.path(), &["switch", "-q", "main"]);
    let marks = [
        (&f.id, format!("    ↳ in_progress on {}", f.branch)),
        (&finished, format!("    ↳ ✓ done on {finished}, not merged")),
        (&stuck, format!("    ↳ ⚠ blocked on {stuck}")),
    ];
    for args in [&["work", "ready"][..], &["work", "all"], &["status"]] {
        let output = ok(f.path(), args);
        let ready = section(&output, "Ready Work");
        for (id, mark) in &marks {
            let row = ready
                .iter()
                .position(|line| line.contains(&format!(" {id} todo")))
                .unwrap_or_else(|| panic!("{args:?} lists {id} as ready: {output}"));
            assert_eq!(
                ready.get(row + 1),
                Some(&mark.as_str()),
                "{args:?}: {output}"
            );
        }
        let row = ready
            .iter()
            .position(|line| line.contains(&format!(" {idle} todo")))
            .unwrap();
        assert!(
            ready.get(row + 1).is_none_or(|next| !next.contains('↳')),
            "{args:?}: {output}"
        );
    }
    // The branch rows mark the same states as the marks.
    let status = ok(f.path(), &["status"]);
    for row in [
        format!("{finished}  {finished} ✓ done, not merged, ⚠ not approved, behind 0, merges cleanly  Finished elsewhere"),
        format!("{stuck}  {stuck} ⚠ blocked, ⚠ not approved, behind 0, merges cleanly  Stuck elsewhere"),
    ] {
        assert!(status.contains(&row), "{row}: {status}");
    }
    // The checkout's own records are not overlaid.
    assert_eq!(
        f.store().load_issue_by_id(&f.id).unwrap().issue.status,
        "todo"
    );
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
}
/// The `Branches` row that starts with `<branch>  `.
fn branch_row(status: &str, branch: &str) -> String {
    status
        .lines()
        .find(|line| line.starts_with(&format!("{branch}  ")))
        .unwrap_or_else(|| panic!("no row for {branch}: {status}"))
        .to_owned()
}
fn prove(f: &Fixture, id: &str) {
    ok(
        f.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{id}"),
            "--kind",
            "test",
            "--result",
            "pass",
            "Proof",
        ],
    );
    commit(f.path(), "proof");
}
fn approve(f: &Fixture, id: &str) {
    ok(
        f.path(),
        &[
            "review",
            "approve",
            id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
}
#[test]
fn waiting_branches_show_merge_readiness_against_the_base() {
    let f = Fixture::new();
    prove(&f, &f.id);
    f.terminal(&f.id);
    approve(&f, &f.id);
    git(f.path(), &["switch", "-q", "main"]);
    // Clean: approved, and the base has not moved.
    let status = ok(f.path(), &["status"]);
    assert_eq!(
        branch_row(&status, &f.branch),
        format!(
            "{}  {} ✓ done, not merged, ✓ approved, behind 0, merges cleanly  Session owner",
            f.branch, f.id
        ),
        "{status}"
    );
    assert!(!status.contains("Shared files:"), "{status}");

    // Conflicted: the base adds the same file differently.
    fs::write(f.path().join("code"), "base code\n").unwrap();
    commit(f.path(), "base code");
    let status = ok(f.path(), &["status"]);
    assert_eq!(
        branch_row(&status, &f.branch),
        format!(
            "{}  {} ✓ done, not merged, ✓ approved, behind 1, conflicts: code  Session owner",
            f.branch, f.id
        ),
        "{status}"
    );

    // Another waiting branch, unapproved and with no receipts, changes the
    // same file; having no receipt is no gap (atelier-s66u).
    let other = ok(f.path(), &["-q", "issue", "create", "Other work"]);
    commit(f.path(), "plan other work");
    ok(f.path(), &["start", &other]);
    fs::write(f.path().join("code"), "other code\n").unwrap();
    fs::write(f.path().join("other"), "other only\n").unwrap();
    commit(f.path(), "other code");
    f.terminal(&other);
    git(f.path(), &["switch", "-q", "main"]);
    let status = ok(f.path(), &["status"]);
    assert_eq!(
        branch_row(&status, &other),
        format!(
            "{other}  {other} ✓ done, not merged, ⚠ not approved, behind 0, merges cleanly  Other work"
        ),
        "{status}"
    );
    // A failing receipt with no pass after it is the one proof fact shown,
    // and a later pass clears it.
    let receipt = |result: &str| {
        git(f.path(), &["switch", "-q", &other]);
        let id = ok(
            f.path(),
            &[
                "-q",
                "evidence",
                "record",
                "--target",
                &format!("issue/{other}"),
                "--kind",
                "test",
                "--result",
                result,
                "Other proof",
            ],
        );
        commit(f.path(), &format!("{result}ing receipt"));
        git(f.path(), &["switch", "-q", "main"]);
        id.split_whitespace().next().unwrap().to_owned()
    };
    let fail = receipt("fail");
    let status = ok(f.path(), &["status"]);
    assert_eq!(
        branch_row(&status, &other),
        format!(
            "{other}  {other} ✓ done, not merged, ⚠ not approved, behind 0, ⚠ failing evidence {fail}, merges cleanly  Other work"
        ),
        "{status}"
    );
    receipt("pass");
    let status = ok(f.path(), &["status"]);
    assert!(
        branch_row(&status, &other).contains("behind 0, merges cleanly"),
        "{status}"
    );
    // Branches are listed in name order.
    let mut pair = [f.branch.clone(), other.clone()];
    pair.sort();
    assert!(
        status.lines().any(
            |line| line == format!("Shared files: {} and {} both touch code", pair[0], pair[1])
        ),
        "{status}"
    );

    // Stale: code after the approval.
    git(f.path(), &["switch", "-q", &f.branch]);
    fs::write(f.path().join("code"), "changed after review\n").unwrap();
    commit(f.path(), "late change");
    git(f.path(), &["switch", "-q", "main"]);
    let status = ok(f.path(), &["status"]);
    assert_eq!(
        branch_row(&status, &f.branch),
        format!(
            "{}  {} ✓ done, not merged, ⚠ approval stale, behind 2, conflicts: code  Session owner",
            f.branch, f.id
        ),
        "{status}"
    );

    // Bringing the base in cleanly keeps the approval current.
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["restore", "--source", "HEAD~1", "code"]);
    commit(f.path(), "undo late change");
    approve(&f, &f.id);
    git(f.path(), &["switch", "-q", "main"]);
    git(f.path(), &["rm", "-q", "code"]);
    git(f.path(), &["commit", "-qm", "drop base code"]);
    let base = git(f.path(), &["rev-parse", "HEAD"]);
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["merge", "-q", "--no-edit", &base]);
    git(f.path(), &["switch", "-q", "main"]);
    let refs = git(f.path(), &["for-each-ref"]);
    let status = ok(f.path(), &["status"]);
    assert!(
        branch_row(&status, &f.branch).contains(", ✓ approved, behind 0, "),
        "{status}"
    );
    // The view changes no Git state.
    assert_eq!(git(f.path(), &["for-each-ref"]), refs);
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
}
#[test]
fn a_parent_owned_branch_shows_its_childrens_activity() {
    let f = Fixture::new();
    let done = ok(
        f.path(),
        &["-q", "issue", "create", "Done child", "--parent", &f.id],
    );
    let active = ok(
        f.path(),
        &["-q", "issue", "create", "Active child", "--parent", &f.id],
    );
    ok(
        f.path(),
        &["-q", "issue", "create", "Waiting child", "--parent", &f.id],
    );
    commit(f.path(), "plan children");
    let mut record = f.store().load_issue_by_id(&active).unwrap();
    record.issue.status = "in_progress".into();
    f.store().write_issue_atomic(&record).unwrap();
    commit(f.path(), "start a child");
    f.terminal(&done);
    git(f.path(), &["switch", "-q", "main"]);
    let status = ok(f.path(), &["status"]);
    assert_eq!(
        branch_row(&status, &f.branch),
        format!(
            "{}  {} in_progress, ⚠ not approved, behind 0, merges cleanly, 1 of 3 children done, 1 active  Session owner",
            f.branch, f.id
        ),
        "{status}"
    );
}
#[test]
fn unassigned_work_in_flight_on_another_branch_is_marked_past_the_ready_list() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    // Five higher-priority items fill the ready list, so the owner's item is
    // listed only under unassigned work.
    for n in 0..5 {
        let title = format!("Filler {n}");
        ok(
            f.path(),
            &[
                "-q",
                "issue",
                "create",
                &title,
                "--priority",
                "high",
                "--label",
                "filler",
            ],
        );
    }
    commit(f.path(), "plan fillers");
    let status = ok(f.path(), &["status"]);
    assert!(
        !section(&status, "Ready Work")
            .iter()
            .any(|line| line.contains(&f.id)),
        "{status}"
    );
    let unassigned = section(&status, "Unassigned Work");
    let row = unassigned
        .iter()
        .position(|line| line.starts_with(&format!("  {} todo", f.id)))
        .unwrap_or_else(|| panic!("owner is listed as unassigned: {status}"));
    assert_eq!(
        unassigned.get(row + 1),
        Some(&format!("    ↳ in_progress on {}", f.branch).as_str()),
        "{status}"
    );
    assert!(
        unassigned
            .iter()
            .filter(|line| line.contains("Filler"))
            .count()
            == 5
            && unassigned.iter().filter(|line| line.contains('↳')).count() == 1,
        "{status}"
    );
}
/// The `↳` line under a ready row, if any.
fn ready_mark<'a>(output: &'a str, id: &str) -> Option<&'a str> {
    let ready = section(output, "Ready Work");
    let row = ready
        .iter()
        .position(|line| line.contains(&format!(" {id} todo")))
        .unwrap_or_else(|| panic!("{id} is listed as ready: {output}"));
    ready
        .get(row + 1)
        .and_then(|line| line.trim_start().strip_prefix("↳ "))
}
#[test]
fn ready_work_owned_on_several_branches_lists_each_branch_state() {
    let f = Fixture::new();
    // A second branch claims the same item.
    let again = format!("again/{}", f.id);
    git(f.path(), &["switch", "-q", "-c", &again]);
    let mut record = f.store().load_issue_by_id(&f.id).unwrap();
    record.issue.workflow_branch.as_mut().unwrap().work_branch = again.clone();
    f.store().write_issue_atomic(&record).unwrap();
    commit(f.path(), "claim on a second branch");
    git(f.path(), &["switch", "-q", "main"]);
    let expected = format!("in_progress on {again}; in_progress on {}", f.branch);
    for args in [&["work", "ready"][..], &["status"]] {
        let output = ok(f.path(), args);
        assert_eq!(
            ready_mark(&output, &f.id),
            Some(expected.as_str()),
            "{output}"
        );
    }
}
#[test]
fn ready_work_on_an_unreadable_branch_says_its_state_is_unknown() {
    let f = Fixture::new();
    // The owner's branch commits a policy that cannot be parsed.
    fs::write(f.path().join(".atelier/workflow.yaml"), "not: [a policy\n").unwrap();
    commit(f.path(), "break the branch policy");
    // A branch whose owner cannot be found is named on the page instead.
    git(f.path(), &["switch", "-q", "-c", "custom/broken", "main"]);
    fs::write(f.path().join(".atelier/workflow.yaml"), "not: [a policy\n").unwrap();
    commit(f.path(), "broken branch without an owner");
    git(f.path(), &["switch", "-q", "main"]);
    let unknown = format!("state unknown on {}: ", f.branch);
    for args in [&["work", "ready"][..], &["work", "all"], &["status"]] {
        let output = ok(f.path(), args);
        assert!(
            ready_mark(&output, &f.id).is_some_and(|mark| mark.starts_with(&unknown)),
            "{args:?}: {output}"
        );
        assert!(output.contains("custom/broken"), "{args:?}: {output}");
    }
    let output = ok(f.path(), &["work", "ready"]);
    assert!(
        output
            .lines()
            .any(|line| line.starts_with("State unknown on custom/broken, owner not found: ")),
        "{output}"
    );
}
#[test]
fn the_branch_that_installs_atelier_lands_with_guidance_and_merges_without_gates() {
    let dir = tempdir().unwrap();
    git(dir.path(), &["init", "-qb", "main"]);
    git(dir.path(), &["config", "user.name", "Author"]);
    git(dir.path(), &["config", "user.email", "author@example.test"]);
    fs::write(dir.path().join("app"), "existing code\n").unwrap();
    commit(dir.path(), "before adoption");
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        dir.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(dir.path(), &["push", "-q", "-u", "origin", "main"]);

    git(dir.path(), &["switch", "-q", "-c", "chore/install-atelier"]);
    ok(dir.path(), &["init", "--slug", "adopter"]);
    for title in ["One", "Two", "Three", "Four", "Five", "Six"] {
        ok(dir.path(), &["-q", "issue", "create", title]);
    }
    commit(dir.path(), "install Atelier and import the backlog");
    fs::write(dir.path().join("AGENTS.md"), "Use Atelier.\n").unwrap();
    commit(dir.path(), "review fixes to the install");
    // Installing is stated, not inferred from the branch's own policy.
    let (pass, output) = cli(dir.path(), &["land"]);
    assert!(
        !pass
            && output.contains("land_install_unconfirmed")
            && output.contains("atelier land --install"),
        "{output}"
    );
    let output = ok(dir.path(), &["land", "--install"]);
    assert!(
        output.contains("Installing Atelier: main has no workflow policy yet")
            && output.contains("git merge --no-ff"),
        "{output}"
    );

    // A fast-forward leaves no install merge, so the review fixes are gated.
    git(dir.path(), &["switch", "-q", "main"]);
    git(
        dir.path(),
        &["merge", "-q", "--ff-only", "chore/install-atelier"],
    );
    let (pass, output) = cli(dir.path(), &["land"]);
    assert!(!pass && output.contains("land_base_unreviewed"), "{output}");
    let (pass, output) = cli(dir.path(), &["land", "--install"]);
    assert!(!pass && output.contains("land_install_invalid"), "{output}");
    git(dir.path(), &["reset", "-q", "--hard", "main@{upstream}"]);

    // Merged as one merge commit by the repository's own review, it lands.
    git(
        dir.path(),
        &[
            "merge",
            "-q",
            "--no-ff",
            "--no-edit",
            "chore/install-atelier",
        ],
    );
    let output = ok(dir.path(), &["land"]);
    assert!(
        output.contains("No unpushed base commits need review"),
        "{output}"
    );

    // Once pushed, the gates apply, and an unowned branch's record changes are
    // listed briefly.
    git(dir.path(), &["push", "-q"]);
    fs::write(dir.path().join("direct"), "direct code\n").unwrap();
    commit(dir.path(), "code straight on the base");
    let (pass, output) = cli(dir.path(), &["land"]);
    assert!(!pass && output.contains("land_base_unreviewed"), "{output}");
    git(dir.path(), &["reset", "-q", "--hard", "HEAD^"]);
    git(dir.path(), &["switch", "-q", "-c", "feature/unowned"]);
    for title in ["A", "B", "C", "D", "E", "F", "G"] {
        ok(dir.path(), &["-q", "issue", "create", title]);
    }
    commit(dir.path(), "records on an unowned branch");
    let (pass, output) = cli(dir.path(), &["land"]);
    assert!(
        !pass
            && output.contains(
                " and 2 more. Next: give the work an owner branch with `atelier start adopter-"
            ),
        "{output}"
    );
}
#[test]
fn a_task_parent_owns_its_children_through_start_land_and_merge() {
    let dir = tempdir().unwrap();
    let path = dir.path();
    git(path, &["init", "-qb", "main"]);
    git(path, &["config", "user.name", "Author"]);
    git(path, &["config", "user.email", "author@example.test"]);
    ok(path, &["init", "--slug", "atelier"]);
    let body = "## Goal\n\nShip the slice.\n\n## Done when\n\nIts children are done.";
    let parent = ok(
        path,
        &["-q", "issue", "create", "Parent task", "--body", body],
    );
    let first = ok(
        path,
        &["-q", "issue", "create", "First child", "--parent", &parent],
    );
    let second = ok(
        path,
        &["-q", "issue", "create", "Second child", "--parent", &parent],
    );
    commit(path, "plan a parent task");

    ok(path, &["start", &first]);
    let branch = parent.clone();
    assert_eq!(git(path, &["branch", "--show-current"]), branch);
    fs::write(path.join("code"), "first slice\n").unwrap();
    commit(path, "first slice");
    ok(path, &["land"]);

    // Merge marks the work done, so nothing closes, and nothing has a close
    // order to keep (atelier-jnrh).
    ok(path, &["start", &parent]);
    let (pass, output) = cli(
        path,
        &[
            "issue",
            "transition",
            &parent,
            "close",
            "--reason",
            "Too early",
        ],
    );
    assert!(
        !pass && output.contains("Invalid transition 'close'"),
        "{output}"
    );
    ok(path, &["start", &second]);
    assert_eq!(git(path, &["branch", "--show-current"]), branch);
    commit(path, "start the parent and the second child");
    // Work in progress lands without a review; merge is what needs one.
    let output = ok(path, &["land"]);
    assert!(!output.contains("Next: atelier merge"), "{output}");
    ok(
        path,
        &[
            "review",
            "approve",
            &parent,
            "--identity",
            "Reviewer",
            "Reviewed the parent and children",
        ],
    );
    commit(path, "approval");
    let output = ok(path, &["land"]);
    assert!(
        output.contains(&format!("Next: atelier merge {parent}")),
        "{output}"
    );
    let head = git(path, &["rev-parse", "HEAD"]);
    ok(path, &["merge", &first]);
    assert_eq!(git(path, &["branch", "--show-current"]), "main");
    // One merge commit marks the parent and both children done.
    assert_eq!(git(path, &["rev-parse", "main^2"]), head);
    for id in [&parent, &first, &second] {
        let record = fs::read_to_string(path.join(format!(".atelier/issues/{id}.md"))).unwrap();
        assert!(record.contains("status: \"done\""), "{record}");
        let at_head = git(path, &["show", &format!("{head}:.atelier/issues/{id}.md")]);
        assert!(at_head.contains("status: \"in_progress\""), "{at_head}");
    }
    ok(path, &["land"]);
}

/// Low-priority follow-ups batched as children of one parent share its
/// branch and one review: a single approval on the parent covers every child
/// on the branch, through `land`, `merge`, which marks them all done, and
/// `land` on the base (atelier-u6f3, atelier-jnrh).
#[test]
fn one_approval_on_the_parent_covers_follow_ups_closed_on_its_branch() {
    let dir = tempdir().unwrap();
    let path = dir.path();
    git(path, &["init", "-qb", "main"]);
    git(path, &["config", "user.name", "Author"]);
    git(path, &["config", "user.email", "author@example.test"]);
    ok(path, &["init", "--slug", "atelier"]);
    let body = "## Goal\n\nClose the follow-ups.\n\n## Done when\n\nEach follow-up is done.";
    let parent = ok(
        path,
        &[
            "-q",
            "issue",
            "create",
            "Batched follow-ups",
            "--body",
            body,
        ],
    );
    let children = ["First", "Second", "Third"].map(|title| {
        ok(
            path,
            &[
                "-q",
                "issue",
                "create",
                title,
                "--parent",
                &parent,
                "--priority",
                "low",
            ],
        )
    });
    commit(path, "plan the batch");
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        path,
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(path, &["push", "-q", "-u", "origin", "main"]);

    // Each follow-up starts on the parent's branch, with its own code and no
    // review of its own.
    let branch = parent.clone();
    for (number, child) in children.iter().enumerate() {
        ok(path, &["start", child]);
        assert_eq!(git(path, &["branch", "--show-current"]), branch);
        fs::write(path.join(format!("fix{number}")), "follow-up fix\n").unwrap();
        commit(path, &format!("fix {child}"));
    }
    ok(path, &["start", &parent]);
    commit(path, "start the batch");
    let (pass, output) = cli(path, &["merge", &parent, "--check"]);
    assert!(!pass && output.contains("review.current"), "{output}");

    ok(
        path,
        &[
            "review",
            "approve",
            &parent,
            "--identity",
            "Reviewer",
            "Reviewed the branch: every follow-up",
            "--commit-records",
        ],
    );
    ok(path, &["land"]);
    ok(path, &["merge", &children[0]]);
    assert_eq!(git(path, &["branch", "--show-current"]), "main");
    for id in std::iter::once(&parent).chain(&children) {
        let record = fs::read_to_string(path.join(format!(".atelier/issues/{id}.md"))).unwrap();
        assert!(record.contains("status: \"done\""), "{record}");
    }
    // On the base, the one approval covers the code and every done claim,
    // the merge commit's marks included; only the approval's own record
    // commit follows its pin (atelier-jnrh).
    let output = ok(path, &["land"]);
    assert!(
        output.contains("Base commits: 1 unpushed record-only commit outside review.")
            && output.contains("land: passed"),
        "{output}"
    );
    let reviews = ok(path, &["-q", "review", "list"]);
    assert_eq!(reviews.lines().count(), 1, "{reviews}");
}

/// Git invocations made by `atelier land`, recorded by a `git` wrapper on PATH.
fn land_git_calls(dir: &Path) -> Vec<String> {
    let real = std::env::split_paths(&std::env::var_os("PATH").unwrap())
        .map(|dir| dir.join("git"))
        .find(|path| path.is_file())
        .expect("git on PATH");
    let wrapper = tempdir().unwrap();
    let log = wrapper.path().join("calls");
    let script = wrapper.path().join("git");
    fs::write(
        &script,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> '{}'\nexec '{}' \"$@\"\n",
            log.display(),
            real.display()
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    let path = std::env::join_paths(
        std::iter::once(wrapper.path().to_path_buf())
            .chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
    )
    .unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir)
        .env("PATH", path)
        .arg("land")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    fs::read_to_string(&log)
        .unwrap_or_default()
        .lines()
        .map(str::to_owned)
        .collect()
}

#[test]
fn landing_reads_git_objects_through_one_batch_not_per_record_processes() {
    // atelier-0349: per-record object queries made `land` take a minute.
    let f = Fixture::new();
    git(f.path(), &["switch", "main"]);
    let accept = |count: usize| {
        for n in 0..count {
            let id = ok(
                f.path(),
                &[
                    "-q",
                    "issue",
                    "create",
                    &format!("Accepted work {n}"),
                    "--body",
                    "## Goal\n\nAccepted.\n\n## Done when\n\nReviewed.",
                ],
            );
            fs::write(f.path().join(format!("work-{id}")), "code\n").unwrap();
            commit(f.path(), "work");
            f.terminal(&id);
            ok(
                f.path(),
                &[
                    "review",
                    "approve",
                    &id,
                    "--identity",
                    "Reviewer",
                    "Accepted",
                ],
            );
            commit(f.path(), "approval");
        }
        fs::write(f.path().join("later"), format!("{count}\n")).unwrap();
        commit(f.path(), "later unrelated work");
        land_git_calls(f.path())
    };
    let per_object = |calls: &[String]| {
        calls
            .iter()
            .filter(|call| {
                let call = call.trim_start_matches("--no-replace-objects ");
                [
                    "ls-tree -z",
                    "--literal-pathspecs ls-tree",
                    "cat-file -t",
                    "cat-file blob",
                    "show",
                    "diff",
                    "rev-parse --verify",
                ]
                .iter()
                .any(|query| call.starts_with(query))
            })
            .cloned()
            .collect::<Vec<_>>()
    };
    let small = accept(3);
    let large = accept(6);
    assert_eq!(
        per_object(&small).len(),
        per_object(&large).len(),
        "object reads grew with records: {:#?}",
        per_object(&large)
    );
    // History walks per review event remain, bounded per accepted record.
    assert!(
        large.len() <= small.len() + 6 * 16,
        "{} Git calls for 9 records, {} for 3",
        large.len(),
        small.len()
    );
}

/// Two IDs ordered so the first sorts before the second.
fn sorted_pair(dir: &Path, titles: [&str; 2]) -> (String, String) {
    let mut ids = titles.map(|title| ok(dir, &["-q", "issue", "create", title]));
    ids.sort();
    let [first, second] = ids;
    (first, second)
}

#[test]
fn scoped_views_offer_the_top_ready_item_in_work_order_that_no_branch_started() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let mission = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "Ordered mission",
            "--issue-type",
            "mission",
        ],
    );
    // The higher-priority item has the later ID, so ID order would pick the
    // other one.
    let (earlier, later) = sorted_pair(f.path(), ["Ready one", "Ready two"]);
    ok(
        f.path(),
        &["issue", "update", &earlier, "--priority", "low"],
    );
    ok(f.path(), &["issue", "update", &later, "--priority", "high"]);
    let elsewhere = ok(f.path(), &["-q", "issue", "create", "Started elsewhere"]);
    ok(
        f.path(),
        &["issue", "update", &elsewhere, "--priority", "critical"],
    );
    for id in [&earlier, &later, &elsewhere] {
        ok(
            f.path(),
            &["issue", "link", &mission, id, "--role", "advances"],
        );
    }
    let parent = ok(f.path(), &["-q", "issue", "create", "Ordered parent"]);
    let (first_child, second_child) = sorted_pair(f.path(), ["Child one", "Child two"]);
    for (child, priority) in [(&first_child, "low"), (&second_child, "high")] {
        ok(
            f.path(),
            &[
                "issue",
                "update",
                child,
                "--parent",
                &parent,
                "--priority",
                priority,
            ],
        );
    }
    commit(f.path(), "plan ordered work");
    ok(f.path(), &["start", &elsewhere]);
    commit(f.path(), "started elsewhere");
    git(f.path(), &["switch", "-q", "main"]);

    let view = ok(f.path(), &["work", "mission", &mission]);
    let ready = section(&view, "Ready Work");
    let row = |id: &str| {
        ready
            .iter()
            .position(|line| line.contains(&format!(" {id} todo")))
            .unwrap_or_else(|| panic!("{id} is not ready: {view}"))
    };
    assert!(
        row(&elsewhere) < row(&later) && row(&later) < row(&earlier),
        "{view}"
    );
    assert!(ready[row(&elsewhere) + 1].contains('↳'), "{view}");
    assert!(
        view.ends_with(&format!("Start the top ready item: atelier start {later}")),
        "{view}"
    );

    let view = ok(f.path(), &["work", "parent", &parent]);
    let ready = section(&view, "Ready Work");
    assert!(ready[0].contains(&second_child), "{view}");
    assert!(
        view.ends_with(&format!(
            "Start the top ready item: atelier start {second_child}"
        )),
        "{view}"
    );
}

/// An item whose only blocker is closed but itself waits on open work fails
/// `start`'s readiness gate, so no view lists it as ready or offers to start
/// it, however high its priority.
#[test]
fn scoped_views_offer_start_only_for_work_that_passes_the_readiness_gates() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let mission = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "Gated mission",
            "--issue-type",
            "mission",
        ],
    );
    let parent = ok(f.path(), &["-q", "issue", "create", "Gated parent"]);
    let gated = ok(f.path(), &["-q", "issue", "create", "Gated work"]);
    let tail = ok(f.path(), &["-q", "issue", "create", "Open tail"]);
    let closed = ok(f.path(), &["-q", "issue", "create", "Closed blocker"]);
    for (child, priority) in [(&gated, "critical"), (&tail, "low")] {
        ok(
            f.path(),
            &[
                "issue",
                "update",
                child,
                "--parent",
                &parent,
                "--priority",
                priority,
            ],
        );
    }
    ok(
        f.path(),
        &["issue", "link", &mission, &parent, "--role", "advances"],
    );
    ok(
        f.path(),
        &["issue", "link", &gated, &closed, "--role", "blocked_by"],
    );
    ok(
        f.path(),
        &["issue", "link", &closed, &tail, "--role", "blocked_by"],
    );
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &closed,
            "supersede",
            "--reason",
            "folded into the tail",
        ],
    );
    commit(f.path(), "plan gated work");

    for view in [
        ok(f.path(), &["work", "mission", &mission]),
        ok(f.path(), &["work", "parent", &parent]),
    ] {
        let ready = section(&view, "Ready Work");
        assert!(
            ready.iter().any(|line| line.contains(&tail)),
            "{tail} is not ready: {view}"
        );
        assert!(
            !ready.iter().any(|line| line.contains(&gated)),
            "{gated} is ready: {view}"
        );
        assert!(
            section(&view, "Blocked Work")
                .iter()
                .any(|line| line.contains(&gated) && line.contains(&tail)),
            "{gated} is not blocked by {tail}: {view}"
        );
        assert!(
            view.ends_with(&format!("Start the top ready item: atelier start {tail}")),
            "{view}"
        );
    }
    let view = ok(f.path(), &["work", "ready"]);
    assert!(!section(&view, "Ready Work")
        .iter()
        .any(|line| line.contains(&gated)));
    let (started, out) = cli(f.path(), &["start", &gated]);
    assert!(
        !started && out.contains(&format!("{gated} -> {closed} -> {tail}")),
        "{out}"
    );
}

/// A local bare remote (never the network) holding `main` and the owner branch.
fn with_remote(f: &Fixture) -> TempDir {
    let remote = tempdir().unwrap();
    git(remote.path(), &["init", "-q", "--bare"]);
    let url = remote.path().to_str().unwrap();
    git(f.path(), &["remote", "add", "origin", url]);
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);
    git(f.path(), &["push", "-q", "-u", "origin", &f.branch]);
    remote
}
fn last_line(output: &str) -> &str {
    output.trim_end().lines().last().unwrap()
}

#[test]
fn land_ends_with_a_verdict_line_and_push_follows_a_pass_or_work_in_progress() {
    let f = Fixture::new();
    let remote = with_remote(&f);
    let tip = |dir: &Path, branch: &str| git(dir, &["rev-parse", &format!("refs/heads/{branch}")]);
    // Plain land states its verdict last.
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(pass && last_line(&out) == "land: passed", "{out}");
    // A pass with --push publishes the branch and says so last.
    fs::write(f.path().join("more"), "more\n").unwrap();
    commit(f.path(), "more code");
    let (pass, out) = cli(f.path(), &["land", "--push"]);
    assert!(
        pass && last_line(&out) == format!("land: passed, pushed to origin/{}", f.branch),
        "{out}"
    );
    assert_eq!(
        tip(remote.path(), &f.branch),
        git(f.path(), &["rev-parse", "HEAD"])
    );
    // Work in progress on the branch is reported, and --push publishes the
    // committed branch to its own upstream (atelier-md0v).
    fs::write(f.path().join("unpushed"), "unpushed\n").unwrap();
    commit(f.path(), "unpushed code");
    let mut record = f.store().load_issue_by_id(&f.id).unwrap();
    record.issue.priority = "low".into();
    f.store().write_issue_atomic(&record).unwrap();
    let (_, out) = cli(f.path(), &["land", "--push"]);
    assert_eq!(
        last_line(&out),
        format!(
            "land: work in progress — 1 check open (land_records_uncommitted), pushed to origin/{}",
            f.branch
        )
    );
    assert_eq!(
        tip(remote.path(), &f.branch),
        git(f.path(), &["rev-parse", "HEAD"])
    );
    // On the base a refusal pushes nothing, exits non-zero, and ends with
    // the refusal even when the output is piped.
    git(f.path(), &["checkout", "-q", "--", "."]);
    git(f.path(), &["switch", "-q", "main"]);
    ok(f.path(), &["issue", "note", &f.id, "Uncommitted base note"]);
    let before = tip(remote.path(), "main");
    let (pass, out) = cli(f.path(), &["land", "--push"]);
    assert!(!pass, "{out}");
    assert_eq!(
        last_line(&out),
        "land: refused — 1 check (land_records_uncommitted), nothing pushed"
    );
    assert_eq!(tip(remote.path(), "main"), before);
    // pipefail is not POSIX; older dash (ubuntu-24.04's sh) rejects it.
    let piped = Command::new("bash")
        .args(["-c", "set -o pipefail; \"$0\" land --push 2>&1 | tail -1"])
        .arg(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(f.path())
        .output()
        .unwrap();
    let last = String::from_utf8_lossy(&piped.stdout);
    assert!(!piped.status.success(), "{last}");
    assert!(last.starts_with("land: refused"), "{last}");
}

#[test]
fn land_push_reports_a_failed_push_and_a_missing_upstream() {
    let f = Fixture::new();
    let remote = with_remote(&f);
    fs::write(f.path().join("more"), "more\n").unwrap();
    commit(f.path(), "more code");
    let hook = remote.path().join("hooks/pre-receive");
    fs::write(&hook, "#!/bin/sh\necho rejected by test >&2\nexit 1\n").unwrap();
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
    }
    let before = git(
        remote.path(),
        &["rev-parse", &format!("refs/heads/{}", f.branch)],
    );
    let (pass, out) = cli(f.path(), &["land", "--push"]);
    assert!(!pass && out.contains("rejected by test"), "{out}");
    assert_eq!(
        last_line(&out),
        "land: refused — 1 check (land_push_failed), nothing pushed"
    );
    assert_eq!(
        git(
            remote.path(),
            &["rev-parse", &format!("refs/heads/{}", f.branch)]
        ),
        before
    );
    git(f.path(), &["branch", "--unset-upstream"]);
    let (pass, out) = cli(f.path(), &["land", "--push"]);
    assert!(!pass && out.contains("land_push_no_upstream"), "{out}");
    assert_eq!(
        last_line(&out),
        "land: refused — 1 check (land_push_no_upstream), nothing pushed"
    );
}

/// A second, unstarted issue committed on `main`, with `main` checked out.
fn unstarted_on_main(f: &Fixture) -> String {
    git(f.path(), &["switch", "-q", "main"]);
    let id = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "Second item",
            "--body",
            "## Goal\n\nStart from anywhere.\n\n## Done when\n\nIt starts.",
        ],
    );
    commit(f.path(), "plan second item");
    id
}
fn recorded_branch(f: &Fixture, id: &str) -> Option<(String, Option<String>)> {
    f.store()
        .load_issue_by_id(id)
        .unwrap()
        .issue
        .workflow_branch
        .map(|branch| (branch.work_branch, branch.branch_base))
}

#[test]
fn start_takes_an_unowned_current_branch_whatever_its_name() {
    // Branch names carry no issue type (atelier-md0v): off the base, on a
    // branch no item owns, start takes it.
    let f = Fixture::new();
    let id = unstarted_on_main(&f);
    let named = format!("feature/{id}");
    git(f.path(), &["switch", "-q", "-c", &named]);
    let out = ok(f.path(), &["start", &id]);
    assert!(out.contains(&format!("Already on branch {named}")), "{out}");
    assert_eq!(git(f.path(), &["branch", "--show-current"]), named);
    assert_eq!(recorded_branch(&f, &id), Some((named, Some("main".into()))));
}

#[test]
fn start_refuses_a_detached_head_and_on_the_base_creates_the_bare_id_branch() {
    let f = Fixture::new();
    let id = unstarted_on_main(&f);
    let tip = git(f.path(), &["rev-parse", "main"]);
    git(f.path(), &["switch", "-q", "--detach", "main"]);
    let (pass, out) = cli(f.path(), &["start", &id]);
    // The step creates the branch from the base, so it works in a linked
    // worktree while the base is checked out elsewhere (finding atelier-ywpf).
    assert!(
        !pass
            && out.contains("start_detached_head")
            && out.contains(&format!("git switch -c {id} main && atelier start {id}")),
        "{out}"
    );
    assert_eq!(recorded_branch(&f, &id), None);
    git(f.path(), &["switch", "-q", "main"]);
    // `--base`, which older guidance still passes, is accepted and ignored.
    let out = ok(f.path(), &["start", &id, "--base", "main"]);
    assert!(
        out.contains("Note: start --base is deprecated and ignored"),
        "{out}"
    );
    assert_eq!(git(f.path(), &["branch", "--show-current"]), id);
    assert_eq!(git(f.path(), &["rev-parse", "HEAD"]), tip);
    assert_eq!(
        recorded_branch(&f, &id),
        Some((id.clone(), Some("main".into())))
    );
    assert_eq!(
        f.store().load_issue_by_id(&id).unwrap().issue.status,
        "in_progress"
    );
}

/// In a linked worktree detached at the base while the primary checkout has
/// the base checked out, `git switch <base>` fails, so the detached-HEAD
/// refusal's step creates the branch from the base instead, and that step
/// works there (finding atelier-ywpf).
#[test]
fn the_detached_head_step_works_in_a_linked_worktree() {
    let f = Fixture::new();
    let id = unstarted_on_main(&f);
    let (_holder, other) = second_worktree(&f);
    let (pass, out) = cli(&other, &["start", &id]);
    let step = format!("git switch -c {id} main && atelier start {id}");
    assert!(
        !pass && out.contains("start_detached_head") && out.contains(&step),
        "{out}"
    );
    let (switched, why) = cli_git(&other, &["switch", "main"]);
    assert!(
        !switched,
        "the base is checked out in the primary checkout: {why}"
    );
    git(&other, &["switch", "-q", "-c", &id, "main"]);
    ok(&other, &["start", &id]);
    assert_eq!(git(&other, &["branch", "--show-current"]), id);
}

/// On the base, an uncommitted edit of `base_branch` does not make land read
/// the base as a work branch: land judges it by HEAD's committed policy, so
/// its refusal of unreviewed code stands, nothing is pushed and the Stop
/// hook blocks (finding atelier-bo2i).
#[test]
fn an_uncommitted_base_branch_edit_on_the_base_leaves_its_refusal_standing() {
    let f = Fixture::new();
    let remote = with_remote(&f);
    git(f.path(), &["switch", "-q", "main"]);
    fs::write(f.path().join("unreviewed"), "code nobody reviewed\n").unwrap();
    commit(f.path(), "unreviewed code on the base");
    let path = f.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    assert!(policy.contains("base_branch: main"), "{policy}");
    fs::write(
        &path,
        policy.replace("base_branch: main", "base_branch: renamed"),
    )
    .unwrap();
    let before = git(remote.path(), &["rev-parse", "refs/heads/main"]);
    let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(f.path())
        .args(["land", "--push"])
        .output()
        .unwrap();
    let out = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!output.status.success(), "{out}");
    assert!(
        out.contains("land_base_unreviewed")
            && last_line(&out).starts_with("land: refused")
            && last_line(&out).ends_with(", nothing pushed"),
        "{out}"
    );
    assert_eq!(
        git(remote.path(), &["rev-parse", "refs/heads/main"]),
        before
    );
    assert_eq!(hook(f.path(), "land", stop(f.path()))["decision"], "block");
}

#[test]
fn land_push_refuses_an_upstream_that_is_not_the_branchs_own_remote_branch() {
    let f = Fixture::new();
    let remote = with_remote(&f);
    fs::write(f.path().join("more"), "unreviewed\n").unwrap();
    commit(f.path(), "more code");
    let main_before = git(remote.path(), &["rev-parse", "refs/heads/main"]);
    // Tracking the remote base, as `git switch -c <branch> origin/main` does.
    git(f.path(), &["branch", "-q", "--set-upstream-to=origin/main"]);
    let (pass, out) = cli(f.path(), &["land", "--push"]);
    assert!(
        !pass && out.contains("land_push_upstream_mismatch"),
        "{out}"
    );
    assert!(
        out.contains(&format!("git push -u <remote> {}", f.branch)),
        "{out}"
    );
    assert_eq!(
        last_line(&out),
        "land: refused — 1 check (land_push_upstream_mismatch), nothing pushed"
    );
    assert_eq!(
        git(remote.path(), &["rev-parse", "refs/heads/main"]),
        main_before
    );
    // A local upstream (remote ".") would move a local branch.
    git(f.path(), &["branch", "-q", "--set-upstream-to=main"]);
    let main_local = git(f.path(), &["rev-parse", "main"]);
    let (pass, out) = cli(f.path(), &["land", "--push"]);
    assert!(
        !pass && out.contains("land_push_upstream_mismatch"),
        "{out}"
    );
    assert_eq!(git(f.path(), &["rev-parse", "main"]), main_local);
}

#[test]
fn land_push_refuses_pushing_a_base_whose_upstream_does_not_resolve() {
    let f = Fixture::new();
    let remote = with_remote(&f);
    git(f.path(), &["switch", "-q", "main"]);
    fs::write(f.path().join("unreviewed"), "code nobody reviewed\n").unwrap();
    commit(f.path(), "unreviewed code on the base");
    let before = git(remote.path(), &["rev-parse", "refs/heads/main"]);
    // The remote branch was deleted and pruned: git shows the upstream as gone.
    git(f.path(), &["update-ref", "-d", "refs/remotes/origin/main"]);
    let url = remote.path().to_str().unwrap().to_owned();
    for case in ["gone", "url"] {
        if case == "url" {
            git(f.path(), &["fetch", "-q", "origin"]);
            git(f.path(), &["config", "branch.main.remote", &url]);
        }
        // Plain land says why the base was not checked.
        let (pass, out) = cli(f.path(), &["land"]);
        assert!(
            pass && out.contains("main's upstream does not resolve"),
            "{case}: {out}"
        );
        let (pass, out) = cli(f.path(), &["land", "--push"]);
        assert!(
            !pass && out.contains("land_push_base_upstream_unresolved"),
            "{case}: {out}"
        );
        assert!(out.contains("git fetch --prune"), "{case}: {out}");
        assert!(!out.contains("Landing checks passed"), "{case}: {out}");
        assert_eq!(
            last_line(&out),
            "land: refused — 1 check (land_push_base_upstream_unresolved), nothing pushed"
        );
        assert_eq!(
            git(remote.path(), &["rev-parse", "refs/heads/main"]),
            before,
            "{case}"
        );
    }
}

#[test]
fn land_push_still_pushes_a_work_branch_when_the_base_upstream_does_not_resolve() {
    let f = Fixture::new();
    let remote = with_remote(&f);
    fs::write(f.path().join("more"), "more\n").unwrap();
    commit(f.path(), "more code");
    git(f.path(), &["update-ref", "-d", "refs/remotes/origin/main"]);
    let (pass, out) = cli(f.path(), &["land", "--push"]);
    assert!(
        pass && last_line(&out) == format!("land: passed, pushed to origin/{}", f.branch),
        "{out}"
    );
    // A base that tracks a path or URL remote pushes work branches too.
    fs::write(f.path().join("again"), "again\n").unwrap();
    commit(f.path(), "again");
    git(f.path(), &["fetch", "-q", "origin"]);
    let url = remote.path().to_str().unwrap();
    git(f.path(), &["config", "branch.main.remote", url]);
    let (pass, out) = cli(f.path(), &["land", "--push"]);
    assert!(pass, "{out}");
}

#[test]
fn land_push_pushes_nothing_when_a_check_moves_head_during_land() {
    let f = Fixture::new();
    git(f.path(), &["switch", "main"]);
    let path = f.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!("{policy}\nmerge_checks:\n  - shell.check: {{ command: 'git -c user.name=x -c user.email=x@x.test commit -q --allow-empty -m moved' }}\n")).unwrap();
    commit(f.path(), "trusted committing check");
    git(f.path(), &["switch", &f.branch]);
    git(f.path(), &["rebase", "main"]);
    f.terminal(&f.id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    let remote = with_remote(&f);
    let before = git(
        remote.path(),
        &["rev-parse", &format!("refs/heads/{}", f.branch)],
    );
    let (pass, out) = cli(f.path(), &["land", "--push"]);
    assert!(!pass && out.contains("land_state_changed"), "{out}");
    assert_eq!(
        git(
            remote.path(),
            &["rev-parse", &format!("refs/heads/{}", f.branch)]
        ),
        before
    );
}

fn lease_path(f: &Fixture) -> std::path::PathBuf {
    f.path()
        .join(".atelier/runtime/leases")
        .join(format!("{}.json", f.id))
}
/// A second worktree of the fixture's repository, on a detached HEAD.
fn second_worktree(f: &Fixture) -> (TempDir, std::path::PathBuf) {
    let holder = tempdir().unwrap();
    let path = holder.path().join("other");
    git(
        f.path(),
        &[
            "worktree",
            "add",
            "-q",
            "--detach",
            path.to_str().unwrap(),
            "main",
        ],
    );
    (holder, path)
}

#[test]
fn start_refuses_an_item_a_live_lease_holds_until_taken_over() {
    let f = Fixture::new();
    assert!(lease_path(&f).exists(), "start leaves a lease");
    let (holder, other) = second_worktree(&f);
    // The holder's own checkout sees no conflict.
    assert!(!ok(f.path(), &["status"]).contains("also held"));
    // Another worktree is refused while the lease is live.
    git(f.path(), &["switch", "-q", "main"]);
    let (pass, out) = cli(&other, &["start", &f.id]);
    assert!(
        !pass && out.contains("start_item_held") && out.contains("--take-over"),
        "{out}"
    );
    assert!(git(&other, &["branch", "--show-current"]).is_empty());
    // status on the owner branch reports the other holder.
    let lease: serde_json::Value =
        serde_json::from_slice(&fs::read(lease_path(&f)).unwrap()).unwrap();
    let mut forged = lease.clone();
    forged["worktree"] = holder.path().to_str().unwrap().into();
    fs::write(lease_path(&f), forged.to_string()).unwrap();
    git(f.path(), &["switch", "-q", &f.branch]);
    let status = ok(f.path(), &["status"]);
    assert!(
        status.contains("Held")
            && status.contains("also held by")
            && status.contains(holder.path().to_str().unwrap()),
        "{status}"
    );
    // --take-over starts anyway and the lease moves to the new worktree.
    git(f.path(), &["switch", "-q", "main"]);
    let out = ok(&other, &["start", &f.id, "--take-over"]);
    assert!(out.contains("Took over"), "{out}");
    assert_eq!(git(&other, &["branch", "--show-current"]), f.branch);
    let moved: serde_json::Value =
        serde_json::from_slice(&fs::read(lease_path(&f)).unwrap()).unwrap();
    assert_eq!(
        fs::canonicalize(moved["worktree"].as_str().unwrap()).unwrap(),
        fs::canonicalize(&other).unwrap()
    );
}

#[test]
fn start_replaces_a_stale_lease_with_a_note() {
    let f = Fixture::new();
    let (_holder, other) = second_worktree(&f);
    git(f.path(), &["switch", "-q", "main"]);
    // Stale by age.
    let mut lease: serde_json::Value =
        serde_json::from_slice(&fs::read(lease_path(&f)).unwrap()).unwrap();
    lease["heartbeat"] = (chrono::Utc::now() - chrono::Duration::hours(5))
        .to_rfc3339()
        .into();
    fs::write(lease_path(&f), lease.to_string()).unwrap();
    let out = ok(&other, &["start", &f.id]);
    assert!(
        out.contains("Replaced a stale lease") && out.contains("over 4 hours old"),
        "{out}"
    );
    // Stale because the holder's worktree is gone.
    git(&other, &["switch", "-q", "--detach", "main"]);
    let gone = tempdir().unwrap().path().join("deleted");
    lease["worktree"] = gone.to_str().unwrap().into();
    lease["heartbeat"] = chrono::Utc::now().to_rfc3339().into();
    fs::write(lease_path(&f), lease.to_string()).unwrap();
    let out = ok(&other, &["start", &f.id]);
    assert!(
        out.contains("Replaced a stale lease") && out.contains("no longer exists"),
        "{out}"
    );
}

#[test]
fn start_warns_instead_of_failing_when_the_lease_cannot_be_written() {
    let f = Fixture::new();
    let id = unstarted_on_main(&f);
    // A file where the lease directory belongs makes the lease write fail.
    let leases = lease_path(&f).parent().unwrap().to_path_buf();
    fs::remove_dir_all(&leases).unwrap();
    fs::write(&leases, "not a directory\n").unwrap();
    let (pass, out) = cli(f.path(), &["start", &id]);
    assert!(
        pass && out.contains("Warning") && out.contains("session lease"),
        "{out}"
    );
    assert_eq!(git(f.path(), &["branch", "--show-current"]), id);
    assert_eq!(
        f.store().load_issue_by_id(&id).unwrap().issue.status,
        "in_progress"
    );
}

#[cfg(feature = "test-hooks")]
#[test]
fn status_does_not_write_the_old_holder_back_over_a_take_over() {
    let f = Fixture::new();
    let (holder, _other) = second_worktree(&f);
    let marker = holder.path().join("checked");
    let mut status = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(f.path())
        .arg("status")
        .env("ATELIER_TEST_LEASE_PAUSE_BEFORE_REFRESH", &marker)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    // status has seen its own lease and is about to renew it.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    while !marker.exists() {
        assert!(std::time::Instant::now() < deadline, "status never paused");
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    // Another worktree takes the lease over in that window.
    let mut taken: serde_json::Value =
        serde_json::from_slice(&fs::read(lease_path(&f)).unwrap()).unwrap();
    taken["worktree"] = holder.path().join("other").to_str().unwrap().into();
    fs::write(lease_path(&f), taken.to_string()).unwrap();
    fs::write(marker.with_extension("release"), "go").unwrap();
    assert!(status.wait().unwrap().success());
    let after: serde_json::Value =
        serde_json::from_slice(&fs::read(lease_path(&f)).unwrap()).unwrap();
    assert_eq!(
        after["worktree"], taken["worktree"],
        "old holder written back"
    );
}

/// A planning commit that only adds an issue advancing a mission re-checks
/// no other done claim in that mission: a sibling's claim does not depend on
/// the mission (atelier-ss25). `land` on the base and on a work branch, and
/// `merge`, skip an old sibling whose proof no longer verifies; `doctor`
/// still reports it.
#[test]
fn adding_an_issue_to_a_mission_rechecks_no_old_done_claim_in_it() {
    let strategy = "merge";
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let mission = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "--issue-type",
            "mission",
            "Mission",
        ],
    );
    let old = ok(f.path(), &["-q", "issue", "create", "Pushed directly"]);
    let blocker = ok(f.path(), &["-q", "issue", "create", "Still open"]);
    ok(
        f.path(),
        &["issue", "link", &old, &mission, "--role", "advances"],
    );
    commit(f.path(), "plan the mission");
    ok(f.path(), &["issue", "transition", &old, "start"]);
    mark_done(f.path(), &old);
    let mut record = f.store().load_issue_by_id(&blocker).unwrap();
    record
        .relationships
        .blocks
        .push(atelier_records::issue_relationship_target(&old));
    f.store().write_issue_atomic(&record).unwrap();
    commit(f.path(), "done without the gates");
    let upstream = tempdir().unwrap();
    git(upstream.path(), &["init", "-q", "--bare"]);
    git(
        f.path(),
        &["remote", "add", "origin", upstream.path().to_str().unwrap()],
    );
    git(f.path(), &["push", "-q", "-u", "origin", "main"]);
    let reported = |output: &str| output.contains(&format!("{old} blockers.transitive_none_open"));

    let add_to_mission = |title: &str| {
        let id = ok(f.path(), &["-q", "issue", "create", title]);
        ok(
            f.path(),
            &["issue", "link", &id, &mission, "--role", "advances"],
        );
        commit(f.path(), &format!("add {title} to the mission"));
    };
    add_to_mission("Planned on the base");
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(pass && !reported(&output), "{strategy}: {output}");
    let (pass, output) = cli(f.path(), &["doctor"]);
    assert!(!pass && reported(&output), "{strategy}: {output}");
    git(f.path(), &["push", "-q"]);

    // A work branch that files one too lands and merges.
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    add_to_mission("Planned on the branch");
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Review",
        ],
    );
    commit(f.path(), "approval");
    f.terminal(&f.id);
    let (pass, output) = cli(f.path(), &["merge", &f.id, "--check"]);
    assert!(pass && !reported(&output), "{strategy}: {output}");
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(pass && !reported(&output), "{strategy}: {output}");
}
/// A done mission that a branch gives open work is a claim `land` refuses,
/// so `merge` refuses it before integrating (atelier-bu66).
#[test]
fn merge_refuses_a_done_mission_the_branch_gives_open_work() {
    {
        let strategy = "merge";
        let f = Fixture::new();
        git(f.path(), &["switch", "-q", "main"]);
        // A policy whose missions close only once their advancing work is
        // done.
        let policy = f.path().join(".atelier/workflow.yaml");
        let text = fs::read_to_string(&policy).unwrap();
        let objective = text.find("  objective:").unwrap();
        let close = objective
            + text[objective..]
                .find("        - blockers.none_open\n")
                .unwrap();
        let mut revised = text.clone();
        revised.insert_str(close, "        - work.terminal\n");
        fs::write(&policy, revised).unwrap();
        let mission = ok(
            f.path(),
            &[
                "-q",
                "issue",
                "create",
                "--issue-type",
                "mission",
                "--body",
                "## Outcome\n\nShipped.\n\n## Success criteria\n\nIt ships.",
                "Mission",
            ],
        );
        commit(f.path(), "plan the mission");
        for transition in ["ready", "start"] {
            ok(f.path(), &["issue", "transition", &mission, transition]);
        }
        ok(
            f.path(),
            &["issue", "transition", &mission, "close", "--reason", "done"],
        );
        commit(f.path(), "close the mission");
        let upstream = tempdir().unwrap();
        git(upstream.path(), &["init", "-q", "--bare"]);
        git(
            f.path(),
            &["remote", "add", "origin", upstream.path().to_str().unwrap()],
        );
        git(f.path(), &["push", "-q", "-u", "origin", "main"]);

        git(f.path(), &["switch", "-q", &f.branch]);
        git(f.path(), &["merge", "-q", "--no-edit", "main"]);
        let work = ok(f.path(), &["-q", "issue", "create", "Open work"]);
        ok(
            f.path(),
            &["issue", "link", &work, &mission, "--role", "advances"],
        );
        commit(f.path(), "file open work for the done mission");
        ok(
            f.path(),
            &[
                "review",
                "approve",
                &f.id,
                "--identity",
                "Reviewer",
                "Review",
            ],
        );
        commit(f.path(), "approval");
        f.terminal(&f.id);
        let refused = format!("{mission} work.terminal");
        let (pass, output) = cli(f.path(), &["land", &f.id]);
        assert!(!pass && output.contains(&refused), "{strategy}: {output}");
        let (pass, output) = cli(f.path(), &["merge", &f.id, "--check"]);
        assert!(!pass && output.contains(&refused), "{strategy}: {output}");
    }
}

/// A refused land lists each failed check first, one line per record with
/// its cause and fix, folds a check that repeats another's cause into that
/// line, lists the passes, and counts every failed check in the verdict
/// (atelier-9jr8).
#[test]
fn land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check() {
    let f = Fixture::new();
    let id = f.id.clone();
    let blocker = blocker_on_base(&f);
    let child = ok(
        f.path(),
        &["-q", "issue", "create", "Blocked child", "--parent", &id],
    );
    ok(
        f.path(),
        &["issue", "link", &child, &blocker, "--role", "blocked_by"],
    );
    commit(f.path(), "plan a blocked child");
    f.terminal(&child);
    let finding = ok(
        f.path(),
        &[
            "-q",
            "review",
            "finding",
            &id,
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "Contract failure in code",
        ],
    );
    commit(f.path(), "finding");
    f.terminal(&id);
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    assert_eq!(
        out,
        format!(
            "⚠ {id} review.current: no independent approval or trivial exemption. Next: atelier review request {id}\n\
             ⚠ {id} blockers.none_open: done but blocked by {blocker} todo medium Open blocker (also {child} blockers.transitive_none_open). Next: start it with `git switch main && atelier start {blocker}`, or drop the link with `atelier issue unlink {child} {blocker} --role blocked_by` if it is wrong\n\
             ⚠ {id} findings.disposed: 1 fix_now finding awaits an approval after its fix (see review.current). Next: atelier review request {id}\n\
             ✓ records committed\n\
             ✓ 1 check passes (blockers.transitive_none_open)\n\
             land: work in progress — 4 checks open (review.current, blockers.none_open, findings.disposed, {child} blockers.transitive_none_open)\n"
        )
    );
    // Every fix it prints runs as printed: the review requests, and start or
    // unlink the blocker.
    assert_eq!(next_steps::assert_fixes_run(f.path(), &out, &["land"]), 4);
    // The Stop hook reports the same lines, ending with the verdict, so no
    // fix command runs into other text, and blocks no turn (atelier-md0v).
    let output = hook(
        f.path(),
        "land",
        serde_json::json!({"hook_event_name": "Stop", "cwd": f.path(), "stop_hook_active": false}),
    );
    assert_eq!(output, in_progress_message(&out));
    let _ = finding;
}

/// A passing land says what it checked and, for a done owner, what comes
/// next; an active owner's land names the uncommitted paths (atelier-bvl3),
/// and names untracked ones apart, since merge leaves them in place
/// (atelier-l457).
#[test]
fn land_pass_lists_what_it_checked_and_the_next_step() {
    let f = Fixture::new();
    let id = f.id.clone();
    let names = ["p1", "p2", "p3", "p4", "p5", "p6"];
    for name in names {
        fs::write(f.path().join(name), "unfinished\n").unwrap();
    }
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(pass);
    assert_eq!(
        out,
        "✓ records committed\n\
         ✓ 1 check passes (findings.disposed)\n\
         Untracked: 6 paths: p1, p2, p3, p4, p5 and 1 more. No commit holds them, so merge leaves them in place; commit any that belong to the work.\n\
         land: passed\n"
    );
    let mut add = vec!["add"];
    add.extend(names);
    git(f.path(), &add);
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(pass);
    assert_eq!(
        out,
        "✓ records committed\n\
         ✓ 1 check passes (findings.disposed)\n\
         Unfinished checkout: 6 uncommitted paths: p1, p2, p3, p4, p5 and 1 more. See `atelier status` before handing off.\n\
         land: passed\n"
    );
    let mut unstage = vec!["rm", "-q", "--cached"];
    unstage.extend(names);
    git(f.path(), &unstage);
    for name in names {
        fs::remove_file(f.path().join(name)).unwrap();
    }
    let child = ok(
        f.path(),
        &["-q", "issue", "create", "Child", "--parent", &id],
    );
    commit(f.path(), "plan a child");
    f.terminal(&child);
    f.terminal(&id);
    let approval = ok(
        f.path(),
        &[
            "-q",
            "review",
            "approve",
            &id,
            "--identity",
            "Reviewer",
            "Reviewed",
        ],
    );
    commit(f.path(), "approval");
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(pass);
    assert_eq!(
        out,
        format!(
            "✓ records committed\n\
             ✓ 4 checks pass (blockers.none_open, blockers.transitive_none_open, findings.disposed, review.current via {approval})\n\
             ✓ 1 done claim holds\n\
             Next: atelier merge {id}\n\
             land: passed\n"
        )
    );
    assert_eq!(ok(f.path(), &["land", "-q"]), "passed");
}

/// An open item planned on the base and merged into the fixture's branch,
/// which is checked out again: a blocker whose own work starts from the
/// base (atelier-md0v).
fn blocker_on_base(f: &Fixture) -> String {
    git(f.path(), &["switch", "-q", "main"]);
    let blocker = ok(f.path(), &["-q", "issue", "create", "Open blocker"]);
    commit(f.path(), "plan a blocker on the base");
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    blocker
}

/// An item's status set directly, as a stored record would hold it.
fn set_status(f: &Fixture, id: &str, status: &str) {
    let mut record = f.store().load_issue_by_id(id).unwrap();
    record.issue.status = status.into();
    f.store().write_issue_atomic(&record).unwrap();
}

/// Uncommitted records do not stop land: it names every one with the git
/// command that commits them, names the dirty paths a completion leaves, and
/// still runs the gates against HEAD's records, so a missing approval is
/// reported in the same run (atelier-kiry, atelier-grsi, atelier-7ts2).
#[test]
fn land_reports_uncommitted_records_dirty_paths_and_the_gates_in_one_run() {
    let f = Fixture::new();
    let id = f.id.clone();
    f.terminal(&id);
    ok(f.path(), &["issue", "note", &id, "Uncommitted note"]);
    let mut new = (1..=5)
        .map(|n| ok(f.path(), &["-q", "issue", "create", &format!("New {n}")]))
        .collect::<Vec<_>>();
    new.push(id.clone());
    new.sort();
    for name in ["p1", "p2", "p3", "p4", "p5", "p6"] {
        fs::write(f.path().join(name), "unfinished\n").unwrap();
    }
    // Staged, they are tracked changes a completion leaves; untracked, merge
    // would leave them in place (atelier-l457).
    git(f.path(), &["add", "p1", "p2", "p3", "p4", "p5", "p6"]);
    let records = new
        .iter()
        .take(5)
        .map(|record| {
            format!(
                ".atelier/issues/{record}.md ({})",
                if *record == id { "modified" } else { "new" }
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    assert_eq!(
        out,
        format!(
            "⚠ {id} land_records_uncommitted: 6 tracker records differ from HEAD: {records} and 1 more; commit them with the work. Next: git add .atelier && git commit -m \"…\"\n\
             ⚠ {id} land_completion_dirty: done, but the checkout has 6 uncommitted paths: p1, p2, p3, p4, p5 and 1 more. Next: commit them with `git commit -am \"…\"`, or set them aside with `git stash push`\n\
             ⚠ {id} review.current: no independent approval or trivial exemption, judged on HEAD's copy; the checkout's differs. Next: git add .atelier && git commit -m \"…\"\n\
             land: work in progress — 3 checks open (land_records_uncommitted, land_completion_dirty, review.current)\n"
        )
    );
    // Each fix runs as printed and clears its own failure.
    assert_eq!(next_steps::assert_fixes_run(f.path(), &out, &["land"]), 4);
    // The Stop hook reports the same run without blocking.
    let output = hook(
        f.path(),
        "land",
        serde_json::json!({"hook_event_name": "Stop", "cwd": f.path(), "stop_hook_active": false}),
    );
    assert_eq!(output, in_progress_message(&out));
}

/// `land <id>` away from the owner's branch says where it is owned and how
/// to land there; a mission says it has no branch; an item no branch owns
/// names its start (atelier-ns04).
#[test]
fn land_with_an_id_points_to_the_owners_branch() {
    let f = Fixture::new();
    let id = f.id.clone();
    let not_checked = "Not checked: the owner's review, findings, proof and blocker gates and the done claims; land_branch_mismatch stops land before them.";
    git(f.path(), &["switch", "-q", "main"]);
    // The base's record of the item says nothing of its branch.
    let (pass, out) = cli(f.path(), &["land", &id]);
    assert!(!pass);
    // On the base it is a refusal; on another branch, work in progress
    // (atelier-md0v).
    let mismatch = |here: &str| {
        format!(
            "⚠ {id} land_branch_mismatch: owned on {}, not on {here}. Next: git switch {} && atelier land\n{not_checked}\n{}\n",
            f.branch,
            f.branch,
            if here == "main" {
                "land: refused — 1 check (land_branch_mismatch)"
            } else {
                "land: work in progress — 1 check open (land_branch_mismatch)"
            }
        )
    };
    assert_eq!(out, mismatch("main"));
    assert_eq!(
        next_steps::assert_fixes_run(f.path(), &out, &["land", &id]),
        1
    );
    // Another branch holds the item's own record of its branch.
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["switch", "-q", "-c", "elsewhere"]);
    let (pass, out) = cli(f.path(), &["land", &id]);
    assert!(!pass);
    assert_eq!(out, mismatch("elsewhere"));

    git(f.path(), &["switch", "-q", "main"]);
    let mission = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "A mission",
            "--issue-type",
            "mission",
        ],
    );
    let unstarted = ok(f.path(), &["-q", "issue", "create", "Unstarted"]);
    commit(f.path(), "plan a mission and unstarted work");
    let (pass, out) = cli(f.path(), &["land", &mission]);
    assert!(!pass);
    assert_eq!(
        out,
        format!(
            "⚠ {mission} land_mission: a mission, and missions have no branch; land one of its items. Next: atelier work mission {mission}\n\
             Not checked: the owner's review, findings, proof and blocker gates and the done claims; land_mission stops land before them.\n\
             land: refused — 1 check (land_mission)\n"
        )
    );
    assert_eq!(
        next_steps::assert_fixes_run(f.path(), &out, &["land", &mission]),
        1
    );
    let (pass, out) = cli(f.path(), &["land", &unstarted]);
    assert!(!pass);
    assert_eq!(
        out,
        format!(
            "⚠ {unstarted} land_work_untraceable: {unstarted} has no work branch. Next: atelier start {unstarted}\n\
             Not checked: the owner's review, findings, proof and blocker gates and the done claims; land_work_untraceable stops land before them.\n\
             land: refused — 1 check (land_work_untraceable)\n"
        )
    );
    assert_eq!(
        next_steps::assert_fixes_run(f.path(), &out, &["land", &unstarted]),
        1
    );
    let (pass, out) = cli(f.path(), &["land", "nope"]);
    assert!(!pass && out.contains("issue nope not found"), "{out}");
}

/// An invalid committed policy is one failed check, printed once with one
/// code and one step, and the other gates still run (atelier-y0en).
#[test]
fn land_and_merge_check_report_an_invalid_policy_once() {
    let f = Fixture::new();
    let id = f.id.clone();
    f.terminal(&id);
    fs::write(f.path().join(".atelier/workflow.yaml"), "invalid: [").unwrap();
    commit(f.path(), "invalid policy");
    let cause = "workflow_config_parse_error: .atelier/workflow.yaml: did not find expected node content at line 2 column 1, while parsing a flow node. Next: fix .atelier/workflow.yaml and commit it";
    let review = format!("⚠ {id} review.current: no independent approval or trivial exemption. Next: atelier review request {id}");
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    assert_eq!(
        out,
        format!(
            "⚠ {id} {cause}\n{review}\n\
             ✓ records committed\n\
             ✓ 3 checks pass (blockers.none_open, blockers.transitive_none_open, findings.disposed)\n\
             land: work in progress — 2 checks open (workflow_config_parse_error, review.current)\n"
        )
    );
    let (pass, out) = cli(f.path(), &["merge", &id, "--check"]);
    assert!(!pass);
    assert_eq!(
        out,
        format!(
            "⚠ {id} {cause}\n{} (also claims.reviewed). Next: atelier review request {id}\n\
             ✓ 3 checks pass (blockers.none_open, blockers.transitive_none_open, findings.disposed)\n\
             merge: refused — 3 checks (workflow_config_parse_error, review.current, claims.reviewed)\n",
            review.split(". Next: ").next().unwrap()
        )
    );
    // On the base, without an owner, the fallback says which policy ran.
    git(f.path(), &["switch", "-q", "main"]);
    fs::write(f.path().join(".atelier/workflow.yaml"), "invalid: [").unwrap();
    commit(f.path(), "invalid base policy");
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    let fallback = out
        .lines()
        .find(|line| line.starts_with("Using last valid policy from "))
        .unwrap_or_else(|| panic!("{out}"));
    assert_eq!(
        out,
        format!(
            "⚠ {cause}\n✓ records committed\n{fallback}\nland: refused — 1 check (workflow_config_parse_error)\n"
        )
    );
}

/// Every printed fix for a blocker that declares its link with `blocks`
/// runs as printed and clears its own failure (atelier-r3oy). The owner was
/// closed under an older policy, whose close still checks its blockers; open
/// children under it are no longer refused, since merge marks them done
/// (atelier-jnrh).
#[test]
fn land_fixes_for_a_blocks_link_clear_their_failures() {
    let f = Fixture::under(Some(STARTER_WITH_BLOCKED_AND_REVIEW));
    let id = f.id.clone();
    // The blocker is planned on the base, where its own work starts.
    let blocker = blocker_on_base(&f);
    ok(
        f.path(),
        &["issue", "link", &blocker, &id, "--role", "blocks"],
    );
    commit(f.path(), "a blocker");
    f.terminal(&id);
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    assert!(!out.contains(" work.terminal:"), "{out}");
    assert!(
        out.contains(&format!(
            "or drop the link with `atelier issue unlink {blocker} {id} --role blocks` if it is wrong"
        )),
        "{out}"
    );
    assert!(!out.contains("reopen"), "{out}");
    // The review request, and the blocker's start or unlink.
    assert_eq!(next_steps::assert_fixes_run(f.path(), &out, &["land"]), 3);
}

/// A done owner reopened in the checkout only: the gates read HEAD's done
/// copy, so its lines say so, fix by committing, and no pass is shown for it
/// (atelier-6ajc).
#[test]
fn land_shows_no_pass_for_an_owner_whose_checkout_record_differs() {
    let f = Fixture::new();
    let id = f.id.clone();
    f.terminal(&id);
    set_status(&f, &id, "in_progress");
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    assert!(!out.contains("✓ "), "{out}");
    assert!(
        out.contains(&format!(
            "⚠ {id} review.current: no independent approval or trivial exemption, judged on HEAD's copy; the checkout's differs. Next: git add .atelier && git commit -m \"…\"\n"
        )),
        "{out}"
    );
    // The records line and the review line both fix by the records commit.
    assert_eq!(next_steps::assert_fixes_run(f.path(), &out, &["land"]), 2);
}

/// A base the policy does not trust is fixed by recording the policy's base
/// in the owner's record, for any legal branch name (atelier-9a6e,
/// atelier-hen4).
#[test]
fn a_recorded_branch_base_is_ignored_and_the_policy_base_is_the_base() {
    // An older Atelier recorded the branch `start` ran on as the base, and
    // land refused it; the base is the policy's now, and the field is
    // written but not read (atelier-md0v).
    let f = Fixture::new();
    let path = f.path().join(format!(".atelier/issues/{}.md", f.id));
    let text = fs::read_to_string(&path).unwrap();
    assert!(text.contains("  branch_base: main\n"), "{text}");
    fs::write(
        &path,
        text.replace("  branch_base: main\n", "  branch_base: gone\n"),
    )
    .unwrap();
    commit(f.path(), "a base an older Atelier recorded");
    let (_, out) = cli(f.path(), &["land"]);
    assert!(
        !out.contains("land_base_mismatch") && !out.contains("gone"),
        "{out}"
    );
    let mut record = f.store().load_issue_by_id(&f.id).unwrap();
    record.sections.report = Some("complete".into());
    f.store().write_issue_atomic(&record).unwrap();
    commit(f.path(), "implementer report");
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &f.id,
            "--identity",
            "Reviewer",
            "Independent review",
        ],
    );
    commit(f.path(), "approval");
    let landed = ok(f.path(), &["land"]);
    assert!(landed.ends_with("land: passed"), "{landed}");
    let status = ok(f.path(), &["status"]);
    assert_eq!(field(&status, "Integration base"), Some("main"), "{status}");
    let merged = ok(f.path(), &["merge", &f.id]);
    assert!(merged.contains("into main"), "{merged}");
    assert_eq!(git(f.path(), &["branch", "--show-current"]), "main");
}

/// An unowned branch that changes an item owned on a local branch is told
/// to land there, not to start it again (atelier-9a6e).
#[test]
fn an_unowned_branch_changing_an_owned_item_points_to_its_branch() {
    let f = Fixture::new();
    let id = f.id.clone();
    git(f.path(), &["switch", "-q", "-c", "feature/unowned", "main"]);
    ok(
        f.path(),
        &["issue", "note", &id, "Noted off the owner's branch"],
    );
    commit(f.path(), "note on an unowned branch");
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    assert!(
        out.contains(&format!(
            "{id} is owned on {}. Next: land it there with `git switch {} && atelier land`, or move the tracker records onto {}\n",
            f.branch, f.branch, f.branch
        )),
        "{out}"
    );
    assert!(!out.contains("atelier start"), "{out}");
    assert_eq!(next_steps::assert_fixes_run(f.path(), &out, &["land"]), 1);
}

/// An approval recorded but not committed: the gates read HEAD, which has
/// none, so the review line says it was judged on HEAD and its fix is the
/// records commit, not a review request, and no pass is shown
/// (atelier-q5uh).
#[test]
fn an_uncommitted_approval_is_fixed_by_committing_it() {
    let f = Fixture::new();
    let id = f.id.clone();
    f.terminal(&id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &id,
            "--identity",
            "Reviewer",
            "Reviewed",
        ],
    );
    // `merge` folds the done claim the approval would cover into the line.
    let judged = format!(
        "⚠ {id} review.current: no independent approval or trivial exemption, judged on HEAD's copy; the checkout's differs"
    );
    for gate in [vec!["land"], vec!["merge", id.as_str(), "--check"]] {
        let (pass, out) = cli(f.path(), &gate);
        assert!(!pass);
        let line = out
            .lines()
            .find(|line| line.starts_with(&judged))
            .unwrap_or_else(|| panic!("{gate:?}: {out}"));
        assert!(
            line.ends_with(". Next: git add .atelier && git commit -m \"…\""),
            "{gate:?}: {out}"
        );
        assert!(!out.contains("✓ "), "{gate:?}: {out}");
        assert!(!out.contains("atelier review request"), "{gate:?}: {out}");
        assert!(next_steps::assert_fixes_run(f.path(), &out, &gate) >= 1);
    }
}

/// A committed review record that was changed or deleted is restored from
/// HEAD, never committed; once such a commit is on the branch, the fix drops
/// it: a reset at the tip, a rebase that leaves it out below (atelier-v4rd).
#[test]
fn a_changed_review_record_is_restored_and_a_committed_rewrite_is_dropped() {
    let f = Fixture::new();
    let id = f.id.clone();
    f.terminal(&id);
    let approval = ok(
        f.path(),
        &[
            "-q",
            "review",
            "approve",
            &id,
            "--identity",
            "Reviewer",
            "Reviewed",
        ],
    );
    commit(f.path(), "approval");
    let path = format!(".atelier/reviews/{approval}.md");
    let note = ok(f.path(), &["-q", "issue", "create", "Uncommitted work"]);
    for change in ["deleted", "modified"] {
        let committed = fs::read_to_string(f.path().join(&path)).unwrap();
        if change == "deleted" {
            fs::remove_file(f.path().join(&path)).unwrap();
        } else {
            fs::write(
                f.path().join(&path),
                committed.replace("Reviewed", "Edited"),
            )
            .unwrap();
        }
        let (pass, out) = cli(f.path(), &["land"]);
        assert!(!pass);
        let mut records = [
            format!("{path} ({change})"),
            format!(".atelier/issues/{note}.md (new)"),
        ];
        records.sort();
        assert!(
            out.contains(&format!(
                "land_records_uncommitted: 2 tracker records differ from HEAD: {}; approvals, trivial exemptions and review records on the base are append-only, so restore those and commit the rest with the work. Next: git restore --source=HEAD --staged --worktree -- {path} && git add .atelier && git commit -m \"…\"\n",
                records.join(", ")
            )),
            "{change}: {out}"
        );
        assert_eq!(next_steps::assert_fixes_run(f.path(), &out, &["land"]), 1);
        fs::write(f.path().join(&path), committed).unwrap();
    }
    fs::remove_file(f.path().join(format!(".atelier/issues/{note}.md"))).unwrap();

    // Committed, the deletion is left out by a rebase named by the commit,
    // at the tip and below it.
    fs::remove_file(f.path().join(&path)).unwrap();
    commit(f.path(), "delete the approval");
    let bad = git(f.path(), &["rev-parse", "HEAD"]);
    let line = format!(
        "⚠ {id} review_append_only: {path} was rewritten beyond a finding's disposition, deleted or changed mode in {bad}; a later commit cannot undo it, so leave that commit out of the branch, which brings the record back (only a finding's disposition may change; record a new event for anything else). Next: git rebase --autostash --onto {bad}~1 {bad}\n"
    );
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass && out.contains(&line), "{out}");
    assert_eq!(next_steps::assert_fixes_run(f.path(), &out, &["land"]), 1);
    fs::write(f.path().join("later"), "later work\n").unwrap();
    commit(f.path(), "later work");
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    assert!(
        out.contains(&format!(
            "which rewrites the commits after it: get them reviewed again. Next: git rebase --autostash --onto {bad}~1 {bad}\n"
        )),
        "{out}"
    );
    assert_eq!(next_steps::assert_fixes_run(f.path(), &out, &["land"]), 1);
}

/// When the commit that broke review append-only is a merge, the recovery
/// that leaves it out also drops the work it merged in, and land says so
/// (atelier-zegg).
#[test]
fn leaving_out_a_merge_that_rewrote_a_review_warns_that_its_work_goes_too() {
    let f = Fixture::new();
    let id = f.id.clone();
    f.terminal(&id);
    let approval = ok(
        f.path(),
        &[
            "-q",
            "review",
            "approve",
            &id,
            "--identity",
            "Reviewer",
            "Reviewed",
        ],
    );
    commit(f.path(), "approval");
    git(f.path(), &["switch", "-q", "-c", "side"]);
    fs::write(f.path().join("side-work"), "side work\n").unwrap();
    commit(f.path(), "side work");
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["merge", "-q", "--no-ff", "--no-commit", "side"]);
    let path = format!(".atelier/reviews/{approval}.md");
    git(f.path(), &["rm", "-q", &path]);
    git(
        f.path(),
        &["commit", "-qm", "merge side, deleting the approval"],
    );
    let bad = git(f.path(), &["rev-parse", "HEAD"]);
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    assert!(
        out.contains(&format!(
            "⚠ {id} review_append_only: {path} was rewritten beyond a finding's disposition, deleted or changed mode in {bad}; a later commit cannot undo it, so leave that commit out of the branch, which brings the record back (only a finding's disposition may change; record a new event for anything else); it is a merge, so this also drops the 1 commit it merged in: merge or redo any of that work that should stay. Next: git rebase --autostash --onto {bad}~1 {bad}\n"
        )),
        "{out}"
    );
    assert_eq!(next_steps::assert_fixes_run(f.path(), &out, &["land"]), 1);
    // The fix does what the warning says: the record is back and the merged
    // work is gone.
    git(
        f.path(),
        &[
            "rebase",
            "-q",
            "--autostash",
            "--onto",
            &format!("{bad}~1"),
            &bad,
        ],
    );
    assert!(f.path().join(&path).exists());
    assert!(!f.path().join("side-work").exists());
}

/// Every fix land prints, run in the order printed, leaves a branch that
/// lands: the append-only recovery leads and is addressed by commit, so the
/// records commit after it survives (atelier-61il).
#[test]
fn printed_fixes_run_in_order_recover_a_committed_review_deletion() {
    let f = Fixture::new();
    let id = f.id.clone();
    f.terminal(&id);
    let approval = ok(
        f.path(),
        &[
            "-q",
            "review",
            "approve",
            &id,
            "--identity",
            "Reviewer",
            "Reviewed",
        ],
    );
    commit(f.path(), "approval");
    fs::remove_file(f.path().join(format!(".atelier/reviews/{approval}.md"))).unwrap();
    commit(f.path(), "delete the approval");
    let follow_up = ok(f.path(), &["-q", "issue", "create", "Follow-up"]);
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    let fixes = out
        .lines()
        .filter_map(|line| line.split_once(". Next: ").map(|(_, fix)| fix.to_owned()))
        .collect::<Vec<_>>();
    assert_eq!(fixes.len(), 2, "{out}");
    assert!(
        fixes[0].starts_with("git rebase --autostash --onto "),
        "{out}"
    );
    assert_eq!(fixes[1], "git add .atelier && git commit -m \"…\"", "{out}");
    for fix in &fixes {
        let run = Command::new("sh")
            .current_dir(f.path())
            .args(["-c", &fix.replace("\"…\"", "\"Follow-up\"")])
            .output()
            .unwrap();
        assert!(
            run.status.success(),
            "{fix}: {}",
            String::from_utf8_lossy(&run.stderr)
        );
    }
    assert!(f
        .path()
        .join(format!(".atelier/issues/{follow_up}.md"))
        .exists());
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
    ok(f.path(), &["land"]);
}

/// A rebase, merge, cherry-pick or revert stopped on a conflict is reported
/// first, with the command that abandons it, and no gate reads the
/// half-done state (atelier-vlhn, atelier-zegg).
#[test]
fn an_operation_left_in_progress_is_reported_before_any_gate() {
    for operation in ["rebase", "merge", "cherry-pick", "revert"] {
        let f = Fixture::new();
        let id = f.id.clone();
        git(f.path(), &["switch", "-q", "main"]);
        fs::write(f.path().join("code"), "base code\n").unwrap();
        commit(f.path(), "base changes the code");
        git(f.path(), &["switch", "-q", &f.branch]);
        let target = if operation == "revert" {
            // Reverting the commit that wrote the code conflicts with a
            // later change to it.
            fs::write(f.path().join("code"), "later code\n").unwrap();
            commit(f.path(), "later code");
            "HEAD~1"
        } else {
            "main"
        };
        assert!(
            !cli_git(f.path(), &[operation, target]).0,
            "{operation} conflicts"
        );
        let abort = format!("git {operation} --abort");
        let finish = match operation {
            "merge" => "resolve the conflicts and commit the merge".to_owned(),
            _ => format!("resolve the conflicts and continue with git {operation} --continue"),
        };
        let code = operation.replace('-', "_");
        for (gate, code) in [
            (vec!["land"], format!("land_{code}_in_progress")),
            (
                vec!["merge", id.as_str(), "--check"],
                format!("merge_{code}_in_progress"),
            ),
        ] {
            let (pass, out) = cli(f.path(), &gate);
            assert!(!pass);
            assert!(
                out.starts_with(&format!(
                    "⚠ {code}: a {operation} is in progress, so HEAD and the index are mid-{operation}. Next: `{abort}`, or {finish}\n"
                )),
                "{gate:?}: {out}"
            );
            // On a work branch land reports it; a rebase leaves HEAD
            // detached, where land refuses (atelier-md0v).
            assert!(
                out.ends_with(&format!("refused — 1 check ({code})\n"))
                    || gate[0] == "land"
                        && operation != "rebase"
                        && out.ends_with(&format!("work in progress — 1 check open ({code})\n")),
                "{gate:?}: {out}"
            );
            assert_eq!(next_steps::assert_fixes_run(f.path(), &out, &gate), 1);
        }
    }
}

/// A merged done claim's proof reads as land and doctor judge it: after a
/// later code commit on the base, `evidence list` and `evidence show` mark
/// the pass held at its close, not stale, while on the work branch the
/// same pass is judged at HEAD (atelier-2isj). A pass recorded over
/// uncommitted code says in its verdict that it does not prove the item
/// yet, as the list then marks it (atelier-g2pl).
#[test]
fn evidence_views_judge_a_merged_claim_as_land_and_doctor_do() {
    let f = Fixture::new();
    let id = f.id.clone();
    let out = ok(
        f.path(),
        &[
            "evidence", "record", "--target", &id, "--kind", "test", "--", "grep", "-q", "tested",
            "code",
        ],
    );
    assert!(out.ends_with("evidence record: passed"), "{out}");
    let pass = ok(f.path(), &["-q", "evidence", "list", &id]);
    commit(f.path(), "proof");
    f.terminal(&id);
    let claim = git(f.path(), &["rev-parse", "HEAD"]);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &id,
            "--identity",
            "Reviewer",
            "Reviewed",
        ],
    );
    commit(f.path(), "approval");
    let proof_line = |out: &str| {
        out.lines()
            .find(|line| line.starts_with("Proof: "))
            .unwrap()
            .to_string()
    };
    // On its own work branch the claim is judged at HEAD, as merge --check does.
    let listed = ok(f.path(), &["evidence", "list", &id]);
    assert_eq!(proof_line(&listed), format!("Proof: ✓ fresh pass {pass}"));
    ok(f.path(), &["merge", &id]);
    assert_eq!(git(f.path(), &["branch", "--show-current"]), "main");

    fs::write(f.path().join("later"), "later base work\n").unwrap();
    commit(f.path(), "later code on the base");
    let (pass_land, land) = cli(f.path(), &["land"]);
    assert!(pass_land, "{land}");
    let (pass_doctor, doctor) = cli(f.path(), &["doctor"]);
    assert!(pass_doctor, "{doctor}");

    let short = &claim[..8];
    let listed = ok(f.path(), &["evidence", "list", &id]);
    assert!(
        listed.contains(&format!(" · ✓ held at close {short} · grep -q tested code")),
        "{listed}"
    );
    assert!(!listed.contains("stale"), "{listed}");
    assert_eq!(
        proof_line(&listed),
        format!("Proof: ✓ pass {pass} held at close {short}")
    );
    let shown = ok(f.path(), &["evidence", "show", &pass]);
    assert!(
        shown
            .lines()
            .next()
            .unwrap()
            .ends_with(&format!(" · ✓ held at close {short}")),
        "{shown}"
    );

    // Open work is judged at HEAD: a pass over uncommitted code is stale,
    // and its own confirmation says so.
    let open = ok(f.path(), &["-q", "issue", "create", "Open work"]);
    commit(f.path(), "plan open work");
    fs::write(f.path().join("later"), "uncommitted\n").unwrap();
    let out = ok(
        f.path(),
        &[
            "evidence",
            "record",
            "--target",
            &open,
            "--kind",
            "test",
            "--result",
            "pass",
            "Looked right",
        ],
    );
    assert!(
        out.ends_with(&format!(
            "evidence record: recorded pass — ⚠ stale (uncommitted code changes), so it does not prove {open}"
        )),
        "{out}"
    );
    let listed = ok(f.path(), &["evidence", "list", &open]);
    assert!(
        listed.contains(" · ⚠ stale (uncommitted code changes) · "),
        "{listed}"
    );
}

/// A done claim is judged at its close only once the base holds it, as
/// land decides: a sibling branch that merged an unmerged owner's work, or
/// a detached checkout of it, judges the child's proof at HEAD, so a pass a
/// later code commit staled reads stale there, where land refuses
/// evidence.attached (atelier-yr1n).
#[test]
fn evidence_views_judge_an_unmerged_claim_at_head() {
    let dir = tempdir().unwrap();
    let path = dir.path();
    git(path, &["init", "-qb", "main"]);
    git(path, &["config", "user.name", "Author"]);
    git(path, &["config", "user.email", "author@example.test"]);
    ok(path, &["init", "--slug", "atelier"]);
    // An older policy whose work `close` names evidence.attached, which is
    // read and ignored (atelier-md0v): the claims it makes are judged by
    // its other validators, as before atelier-jnrh.
    let policy = path.join(".atelier/workflow.yaml");
    let text = STARTER_WITH_BLOCKED_AND_REVIEW.replace(
        "        - blockers.none_open\n",
        "        - blockers.none_open\n        - evidence.attached: { min_count: 1 }\n",
    );
    fs::write(&policy, text).unwrap();
    commit(path, "require proof");
    let owner = ok(path, &["-q", "issue", "create", "Owner two"]);
    let child = ok(
        path,
        &["-q", "issue", "create", "Child two", "--parent", &owner],
    );
    commit(path, "plan");
    ok(path, &["start", &owner]);
    fs::write(path.join("code"), "code\n").unwrap();
    commit(path, "code");
    let record = |target: &str| {
        ok(
            path,
            &[
                "evidence", "record", "--target", target, "--kind", "test", "--", "true",
            ],
        );
    };
    record(&child);
    commit(path, "child proof");
    ok(path, &["issue", "transition", &child, "start"]);
    ok(
        path,
        &["issue", "transition", &child, "close", "--reason", "Done."],
    );
    commit(path, "close child");
    fs::write(path.join("code"), "later code\n").unwrap();
    commit(path, "later code");
    record(&owner);
    commit(path, "owner proof");
    ok(
        path,
        &["issue", "transition", &owner, "close", "--reason", "Done."],
    );
    ok(
        path,
        &[
            "review",
            "approve",
            &owner,
            "--identity",
            "Reviewer",
            "Reviewed",
        ],
    );
    commit(path, "close and approve owner");
    let owner_branch = git(path, &["branch", "--show-current"]);

    let proof = || {
        ok(path, &["evidence", "list", &child])
            .lines()
            .find(|line| line.starts_with("Proof: "))
            .unwrap()
            .to_string()
    };
    let stale = |proof: &str| proof.starts_with("Proof: no fresh pass; newest pass ");

    // A sibling branch that merged the owner's unmerged work.
    git(path, &["switch", "-q", "main"]);
    let sibling = ok(path, &["-q", "issue", "create", "Owner four"]);
    commit(path, "plan sibling");
    ok(path, &["start", &sibling]);
    commit(path, "start sibling");
    git(path, &["merge", "-q", "--no-edit", &owner_branch]);
    // The view marks the child's pass stale; no gate reads it.
    let (_, land) = cli(path, &["land"]);
    assert!(!land.contains("evidence."), "{land}");
    let (_, check) = cli(path, &["merge", &sibling, "--check"]);
    assert!(!check.contains("evidence."), "{check}");
    assert!(stale(&proof()), "{}", proof());

    // A detached checkout of the owner's tip: land refuses the untraceable
    // work before judging it, and the view judges the claim at HEAD.
    git(path, &["checkout", "-q", "--detach", &owner_branch]);
    let (landed, land) = cli(path, &["land"]);
    assert!(!landed && land.contains("land_work_untraceable"), "{land}");
    assert!(stale(&proof()), "{}", proof());
    let listed = ok(path, &["evidence", "list", &child]);
    assert!(!listed.contains("held at close"), "{listed}");
}

/// The Stop payload for `dir`.
fn stop(dir: &Path) -> serde_json::Value {
    serde_json::json!({"hook_event_name": "Stop", "cwd": dir, "stop_hook_active": false})
}

/// A hook payload the adapter cannot use is reported as the hook's
/// misconfiguration, which does not block a turn, never as a landing
/// failure; SessionStart says the same (atelier-ft88).
#[test]
fn hook_payload_errors_are_reported_apart_from_landing_failures() {
    let f = Fixture::new();
    let misconfigured =
        |error: &str| format!("Atelier hook misconfigured: {error}; check .claude/settings.json");
    let missing = f.path().join("missing");
    for (input, error) in [
        ("not json".to_owned(), "expected ident at line 1 column 2".to_owned()),
        (
            serde_json::json!({"hook_event_name": "Stop"}).to_string(),
            "hook payload requires cwd".to_owned(),
        ),
        (
            serde_json::json!({"hook_event_name": "SessionStart", "cwd": f.path(), "source": "startup"})
                .to_string(),
            "expected Stop hook payload".to_owned(),
        ),
        (
            serde_json::json!({"hook_event_name": "Stop", "cwd": missing}).to_string(),
            format!(
                "hook payload cwd {}: No such file or directory (os error 2)",
                missing.display()
            ),
        ),
    ] {
        assert_eq!(
            hook_input(f.path(), "land", &input),
            serde_json::json!({"systemMessage": misconfigured(&error)}),
            "{input}"
        );
    }
    let context = hook(
        f.path(),
        "status",
        serde_json::json!({"hook_event_name": "SessionStart", "cwd": missing, "source": "startup"}),
    );
    assert_eq!(
        context["hookSpecificOutput"]["additionalContext"],
        misconfigured(&format!(
            "hook payload cwd {}: No such file or directory (os error 2)",
            missing.display()
        ))
    );
    // A failure on a work branch is work in progress: reported, with land's
    // report, and not blocked (atelier-md0v).
    f.terminal(&f.id);
    fs::write(f.path().join("scratch"), "unfinished\n").unwrap();
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    assert_eq!(
        hook(f.path(), "land", stop(f.path())),
        in_progress_message(&out)
    );
    // Outside any Atelier repository there is nothing to land: the hook says
    // so and exits successfully.
    let outside = tempdir().unwrap();
    let outside = outside.path().canonicalize().unwrap();
    assert_eq!(
        hook(
            f.path(),
            "land",
            serde_json::json!({"hook_event_name": "Stop", "cwd": outside})
        ),
        serde_json::json!({"systemMessage": format!(
            "Atelier: {} is not in an Atelier repository, so there is nothing to land.",
            outside.display()
        )})
    );
}

/// On the base, uncommitted records alone are reported without blocking a
/// turn, while `land` still refuses, and any other failure blocks
/// (atelier-mp6v); on a work branch nothing blocks (atelier-md0v).
#[test]
fn uncommitted_records_alone_block_no_turn_on_the_base() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    ok(
        f.path(),
        &["issue", "note", &f.id, "Planning note on the base"],
    );
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    assert!(
        out.contains(&format!(
            "⚠ land_records_uncommitted: 1 tracker record differs from HEAD: .atelier/issues/{}.md (modified)",
            f.id
        )) && out.ends_with("land: refused — 1 check (land_records_uncommitted)\n"),
        "{out}"
    );
    assert_eq!(next_steps::assert_fixes_run(f.path(), &out, &["land"]), 1);
    assert_eq!(
        hook(f.path(), "land", stop(f.path())),
        serde_json::json!({"systemMessage": format!(
            "Atelier: the base has uncommitted records; land refuses until they are committed, but they do not block a turn.\n{}",
            out.trim_end()
        )})
    );
    // Another failure on the base blocks: a committed policy that does not
    // parse, which the gates run past on the last valid one.
    let policy = f.path().join(".atelier/workflow.yaml");
    let valid = fs::read_to_string(&policy).unwrap();
    fs::write(&policy, format!("{valid}garbage: [\n")).unwrap();
    git(
        f.path(),
        &[
            "commit",
            "-qm",
            "broken policy",
            "--",
            ".atelier/workflow.yaml",
        ],
    );
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(
        !pass
            && out.ends_with(
                "land: refused — 2 checks (land_records_uncommitted, workflow_config_parse_error)\n"
            ),
        "{out}"
    );
    assert_eq!(
        hook(f.path(), "land", stop(f.path())),
        serde_json::json!({"decision": "block", "reason": out.trim_end()})
    );
    // On a work branch, uncommitted records are work in progress: land
    // reports them and exits successfully, and the hook does not block.
    git(f.path(), &["checkout", "-q", "--", "."]);
    git(f.path(), &["switch", "-q", &f.branch]);
    ok(f.path(), &["issue", "note", &f.id, "Work-branch note"]);
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(
        !pass
            && out.ends_with("land: work in progress — 1 check open (land_records_uncommitted)\n"),
        "{out}"
    );
    assert_eq!(
        hook(f.path(), "land", stop(f.path())),
        in_progress_message(&out)
    );
}

/// On a work branch, land reports what is open and exits successfully, and
/// the Stop hook reports it without blocking a turn, whatever is open: the
/// owner's missing review, uncommitted code, an open finding (atelier-md0v).
#[test]
fn work_in_progress_on_a_branch_is_reported_and_blocks_no_turn() {
    let f = Fixture::new();
    let id = f.id.clone();
    f.terminal(&id);
    // `land` itself exits successfully, with the report on stdout.
    let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(f.path())
        .arg("land")
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let out = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        out,
        format!(
            "⚠ {id} review.current: no independent approval or trivial exemption. Next: atelier review request {id}\n\
             ✓ records committed\n\
             ✓ 3 checks pass (blockers.none_open, blockers.transitive_none_open, findings.disposed)\n\
             land: work in progress — 1 check open (review.current)\n"
        )
    );
    assert!(output.stderr.is_empty());
    assert_eq!(next_steps::assert_fixes_run(f.path(), &out, &["land"]), 1);
    assert_eq!(
        hook(f.path(), "land", stop(f.path())),
        in_progress_message(&out)
    );
    // An untracked file holds nothing up; uncommitted code in a done
    // checkout is reported too.
    fs::write(f.path().join("scratch"), "unfinished\n").unwrap();
    let (_, out) = cli(f.path(), &["land"]);
    assert!(
        !out.contains("land_completion_dirty") && out.contains("review.current"),
        "{out}"
    );
    git(f.path(), &["add", "scratch"]);
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(
        !pass && out.contains("land_completion_dirty") && out.contains("review.current"),
        "{out}"
    );
    assert_eq!(
        hook(f.path(), "land", stop(f.path())),
        in_progress_message(&out)
    );
    git(f.path(), &["rm", "-q", "--cached", "scratch"]);
    fs::remove_file(f.path().join("scratch")).unwrap();
    // So is an open finding.
    ok(
        f.path(),
        &[
            "review",
            "finding",
            &id,
            "--identity",
            "Reviewer",
            "--recommendation",
            "fix_now",
            "Contract failure in code",
        ],
    );
    commit(f.path(), "finding");
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(
        !pass
            && out.ends_with(
                "land: work in progress — 2 checks open (review.current, findings.disposed)\n"
            ),
        "{out}"
    );
    assert_eq!(
        hook(f.path(), "land", stop(f.path())),
        in_progress_message(&out)
    );
    // `--quiet` prints the verdict alone.
    let quiet = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(f.path())
        .args(["-q", "land"])
        .output()
        .unwrap();
    assert!(quiet.status.success());
    assert_eq!(
        String::from_utf8(quiet.stdout).unwrap(),
        "work in progress — 2 checks open (review.current, findings.disposed)\n"
    );
}

/// A record a newer Atelier wrote says to reinstall, in `status`, its
/// SessionStart context, `land` and the Stop hook, as its one step; like
/// any record that fails to parse, it carries no code (atelier-u4ql).
#[test]
fn a_record_from_a_newer_atelier_says_to_reinstall() {
    let f = Fixture::new();
    let path = f.path().join(format!(".atelier/issues/{}.md", f.id));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        text.replace("schema_version: 2", "schema_version: 3"),
    )
    .unwrap();
    commit(f.path(), "a record from a newer atelier");
    let newer = format!(
        ".atelier/issues/{}.md has schema_version 3, and this atelier reads 2, so records need a newer atelier. Next: upgrade to the latest release with `curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh | sh`, or from Atelier's source checkout run `cargo install --path crates/atelier-cli --locked`",
        f.id
    );
    let (pass, out) = cli(f.path(), &["status"]);
    assert!(!pass);
    assert_eq!(out, format!("Error: {newer}\n"));
    let context = hook(
        f.path(),
        "status",
        serde_json::json!({"hook_event_name": "SessionStart", "cwd": f.path(), "source": "startup"}),
    );
    assert_eq!(
        context["hookSpecificOutput"]["additionalContext"],
        format!("Atelier context unavailable: {newer}")
    );
    // On a work branch it is reported and blocks no turn (atelier-md0v).
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    assert_eq!(out, format!("Error: {newer}\nland: work in progress\n"));
    assert_eq!(
        hook(f.path(), "land", stop(f.path())),
        in_progress_message(&out)
    );
}

/// A newer Atelier's workflow policy says to reinstall, as its own failed
/// check in land, not to edit the policy (atelier-u4ql).
#[test]
fn a_policy_from_a_newer_atelier_says_to_reinstall() {
    let f = Fixture::new();
    let path = f.path().join(".atelier/workflow.yaml");
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        text.replace("schema_version: 3", "schema_version: 4"),
    )
    .unwrap();
    commit(f.path(), "a policy from a newer atelier");
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    assert!(
        out.starts_with(&format!(
            "⚠ {} workflow_config_schema_newer: .atelier/workflow.yaml field schema_version: workflow schema_version 4 is newer than the 3 this atelier reads, so the policy needs a newer atelier. Next: upgrade to the latest release with `curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh | sh`, or from Atelier's source checkout run `cargo install --path crates/atelier-cli --locked`\n",
            f.id
        )),
        "{out}"
    );
    // It says one thing: the reinstall, not also an edit.
    assert!(!out.contains("fix .atelier/workflow.yaml"), "{out}");
    // Other commands fall back to the last valid policy and say why, once.
    let (_, out) = cli(f.path(), &["issue", "list"]);
    assert!(!out.contains("fix .atelier/workflow.yaml"), "{out}");
    assert!(
        out.contains("Next: upgrade to the latest release with `curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh | sh`, or from Atelier's source checkout run `cargo install --path crates/atelier-cli --locked`"),
        "{out}"
    );
}

/// A Git failure while land reads the checkout keeps Git's own message: it
/// is not reported as uncommitted records (atelier-467y).
#[test]
fn a_git_failure_during_land_is_not_called_uncommitted_records() {
    let f = Fixture::new();
    fs::write(f.path().join(".git/index"), "not an index").unwrap();
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(!pass);
    assert!(
        !out.contains("land_records_uncommitted") && out.contains("index"),
        "{out}"
    );
    let output = hook(f.path(), "land", stop(f.path()));
    let message = output["systemMessage"].as_str().unwrap();
    assert!(output.get("decision").is_none(), "{output}");
    assert!(
        !message.contains("land_records_uncommitted") && message.contains("index"),
        "{message}"
    );
}

/// Writes a review event in schema version 1, as Atelier wrote it before
/// ADR 0025: `event` is the event's YAML under `event:`.
fn v1_review(dir: &Path, id: &str, target: &str, identity: &str, pin: &str, at: &str, event: &str) {
    fs::write(
        dir.join(format!(".atelier/reviews/{id}.md")),
        format!(
            "---\nschema: atelier.review\nschema_version: 1\nid: {id}\ntitle: {id}\ntarget: {target}\nidentity: {identity}\ncommit: {pin}\ncreated_at: {at}\nevent:\n{event}---\n\n{id} text\n"
        ),
    )
    .unwrap();
}
/// A separate disposition event in schema version 1.
#[allow(clippy::too_many_arguments)]
fn v1_disposition(
    dir: &Path,
    (id, target, identity, pin, at): (&str, &str, &str, &str, &str),
    finding: &str,
    decision: &str,
    supersedes: Option<&str>,
    resolution: Option<&str>,
    follow_up: Option<&str>,
) {
    let value = |field: Option<&str>| field.unwrap_or("null").to_owned();
    v1_review(
        dir,
        id,
        target,
        identity,
        pin,
        at,
        &format!(
            "  event: disposition\n  finding: {finding}\n  decision: {decision}\n  supersedes: {}\n  resolution_commit: {}\n  follow_up: {}\n",
            value(supersedes),
            value(resolution),
            value(follow_up)
        ),
    );
}

/// A repository from before ADR 0025: a lifecycle merged into the base with
/// findings in schema version 1 and separate disposition events, the defer
/// proposed by the author and accepted by the reviewer in a successor.
/// Unless `merged`, the approved branch is left unmerged. Returns the
/// fixture and the three findings (fix_now, defer, no_action).
fn pre_adr_0025_repository(no_action_identity: &str, merged: bool) -> (Fixture, [String; 3]) {
    let f = Fixture::new();
    let id = f.id.clone();
    let follow_up = ok(
        f.path(),
        &[
            "-q",
            "issue",
            "create",
            "Follow up later",
            "--body",
            "## Goal\n\nFollow up later.\n\n## Done when\n\n- [ ] The follow-up test passes.",
        ],
    );
    commit(f.path(), "plan the follow-up");
    let pin = git(f.path(), &["rev-parse", "HEAD"]);
    let findings = ["atelier-f001", "atelier-f002", "atelier-f003"].map(str::to_owned);
    for (finding, recommendation) in findings.iter().zip(["fix_now", "defer", "no_action"]) {
        v1_review(
            f.path(),
            finding,
            &id,
            "Reviewer",
            &pin,
            "2026-10-01T00:00:00Z",
            &format!("  event: finding\n  recommendation: {recommendation}\n"),
        );
    }
    commit(f.path(), "findings");
    fs::write(f.path().join("code"), "fixed code\n").unwrap();
    commit(f.path(), "fix");
    let fix = git(f.path(), &["rev-parse", "HEAD"]);
    let at = |second: u32| format!("2026-10-02T00:00:{second:02}Z");
    v1_disposition(
        f.path(),
        ("atelier-d001", &id, "Author", &fix, &at(1)),
        &findings[0],
        "fix_now",
        None,
        Some(&fix),
        None,
    );
    v1_disposition(
        f.path(),
        ("atelier-d002", &id, "Author", &fix, &at(2)),
        &findings[1],
        "defer",
        None,
        None,
        Some(&follow_up),
    );
    v1_disposition(
        f.path(),
        ("atelier-d003", &id, "Reviewer", &fix, &at(3)),
        &findings[1],
        "defer",
        Some("atelier-d002"),
        None,
        Some(&follow_up),
    );
    v1_disposition(
        f.path(),
        ("atelier-d004", &id, no_action_identity, &fix, &at(4)),
        &findings[2],
        "no_action",
        None,
        None,
        None,
    );
    commit(f.path(), "dispositions");
    f.terminal(&id);
    let head = git(f.path(), &["rev-parse", "HEAD"]);
    v1_review(
        f.path(),
        "atelier-a001",
        &id,
        "Reviewer",
        &head,
        &at(5),
        "  event: approval\n",
    );
    commit(f.path(), "approval");
    if merged {
        // The old binary's merge, a fast-forward under this policy.
        git(f.path(), &["switch", "-q", "main"]);
        git(f.path(), &["merge", "-q", "--ff-only", &f.branch]);
    }
    (f, findings)
}

/// `atelier migrate dispositions` folds each finding's latest disposition
/// event into the finding and removes the events; the migrated repository
/// passes `doctor` and `land`, and each migrated defer or no_action is
/// still judged by who set it, traced to the event it came from (ADR 0025).
#[test]
fn migrating_dispositions_folds_events_into_findings_and_passes_doctor() {
    let (f, findings) = pre_adr_0025_repository("Reviewer", true);
    // Unmigrated, the events read as folded into their findings, and
    // `check` passes with a note naming the migration (atelier-l457).
    assert_eq!(
        ok(f.path(), &["check"]),
        "Check passed.\nNote: 4 separate disposition events in the form before ADR 0025, read folded into their findings. `atelier migrate dispositions --apply` writes the fold."
    );
    ok(f.path(), &["doctor"]);
    ok(f.path(), &["land"]);
    let reviews = f.path().join(".atelier/reviews");
    let before = fs::read_dir(&reviews).unwrap().count();
    // A dry run lists the findings and writes nothing.
    assert_eq!(
        ok(f.path(), &["-q", "migrate", "dispositions"]),
        findings.join("\n")
    );
    let dry = ok(f.path(), &["migrate", "dispositions"]);
    assert!(
        dry.contains(&format!(
            "{}: defer by Reviewer from atelier-d003, removing 1 earlier event it superseded",
            findings[1]
        )) && dry.contains("Dry run"),
        "{dry}"
    );
    assert_eq!(git(f.path(), &["status", "--porcelain"]), "");
    ok(f.path(), &["migrate", "dispositions", "--apply"]);
    commit(f.path(), "migrate dispositions");
    assert_eq!(ok(f.path(), &["check"]), "Check passed.");
    // Four disposition events are gone; the findings and approval remain.
    assert_eq!(fs::read_dir(&reviews).unwrap().count(), before - 4);
    let deferred = fs::read_to_string(reviews.join(format!("{}.md", findings[1]))).unwrap();
    assert!(
        deferred.contains("schema_version: 2")
            && deferred.contains("    identity: Reviewer\n")
            && deferred.contains("    migrated_from: atelier-d003\n")
            && deferred.contains("    reason: atelier-d003 text\n"),
        "{deferred}"
    );
    ok(f.path(), &["doctor"]);
    ok(f.path(), &["land"]);
    assert_eq!(ok(f.path(), &["-q", "migrate", "dispositions"]), "");
    // A successor event an older binary recorded, as a branch brings in
    // when it merges the migrated base, continues the folded one.
    let fix = git(f.path(), &["rev-parse", "HEAD"]);
    v1_disposition(
        f.path(),
        (
            "atelier-d005",
            &f.id,
            "Reviewer",
            &fix,
            "2026-10-03T00:00:00Z",
        ),
        &findings[2],
        "no_action",
        Some("atelier-d004"),
        None,
        None,
    );
    commit(f.path(), "a successor disposition event");
    assert_eq!(
        ok(f.path(), &["-q", "migrate", "dispositions", "--apply"]),
        findings[2]
    );
    commit(f.path(), "migrate the successor");
    ok(f.path(), &["doctor"]);
    // Restoring the migrated disposition by hand is one more decision,
    // judged by no identity (atelier-md0v).
    let path = reviews.join(format!("{}.md", findings[2]));
    let migrated = fs::read_to_string(&path).unwrap();
    ok(
        f.path(),
        &[
            "review",
            "disposition",
            &findings[2],
            "--identity",
            "Someone",
            "--decision",
            "no_action",
            "--commit-records",
            "Changed",
        ],
    );
    ok(f.path(), &["doctor"]);
    fs::write(&path, migrated).unwrap();
    commit(f.path(), "restore the migrated disposition by hand");
    ok(f.path(), &["doctor"]);
}

/// No identity is compared with the work's authors (atelier-md0v): a
/// no_action the author set in a separate event is accepted, unmigrated
/// and once folded in.
#[test]
fn a_migrated_author_no_action_is_accepted() {
    let (f, _) = pre_adr_0025_repository("Author", true);
    ok(f.path(), &["doctor"]);
    ok(f.path(), &["migrate", "dispositions", "--apply"]);
    commit(f.path(), "migrate dispositions");
    assert_eq!(ok(f.path(), &["check"]), "Check passed.");
    ok(f.path(), &["doctor"]);
}

/// Setting or changing a disposition after approval keeps the approval
/// current, and editing anything else in the finding does not (ADR 0025).
#[test]
fn a_disposition_after_approval_keeps_it_current() {
    let f = Fixture::new();
    let id = f.id.clone();
    let finding = ok(
        f.path(),
        &[
            "-q",
            "review",
            "finding",
            &id,
            "--identity",
            "Reviewer",
            "--recommendation",
            "no_action",
            "Concern",
        ],
    );
    commit(f.path(), "finding");
    f.terminal(&id);
    ok(
        f.path(),
        &[
            "review",
            "approve",
            &id,
            "--identity",
            "Reviewer",
            "Approved",
        ],
    );
    commit(f.path(), "approval");
    for (identity, decision) in [("Author", "fix_now"), ("Reviewer", "no_action")] {
        let mut args = vec![
            "review",
            "disposition",
            &finding,
            "--identity",
            identity,
            "--decision",
            decision,
        ];
        let fix = git(f.path(), &["rev-parse", "HEAD~2"]);
        if decision == "fix_now" {
            args.extend(["--resolution-commit", &fix]);
        }
        args.extend(["--commit-records", "Decided"]);
        ok(f.path(), &args);
        let (_, out) = cli(f.path(), &["merge", &id, "--check"]);
        assert!(out.contains("review.current via"), "{decision}: {out}");
    }
    let (pass, out) = cli(f.path(), &["merge", &id, "--check"]);
    assert!(pass, "{out}");
    let path = f.path().join(format!(".atelier/reviews/{finding}.md"));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        text.replace("\nConcern\n", "\nA different concern\n"),
    )
    .unwrap();
    // The finding has not reached the base, so it may still be corrected
    // (atelier-ilba); every merge is a merge commit, so no fast-forward made
    // it final (atelier-jnrh).
    commit(f.path(), "rewrite the finding");
    let (pass, out) = cli(f.path(), &["merge", &id, "--check"]);
    assert!(pass && !out.contains("review_append_only"), "{out}");
}

/// An approved work branch whose dispositions are separate events passes
/// check, land and the merge gates exactly as it does once migrated, and
/// migrating keeps its approval current (ADR 0025).
#[test]
fn migrating_dispositions_on_an_approved_branch_keeps_the_approval() {
    let mut reports = Vec::new();
    for migrated in [false, true] {
        let (f, _) = pre_adr_0025_repository("Reviewer", false);
        if migrated {
            ok(f.path(), &["migrate", "dispositions", "--apply"]);
            commit(f.path(), "migrate dispositions");
        }
        ok(f.path(), &["check"]);
        let land = ok(f.path(), &["land", &f.id]);
        let out = ok(f.path(), &["merge", &f.id, "--check"]);
        assert!(
            out.contains("review.current via atelier-a001") && out.contains("merge: passed"),
            "{out}"
        );
        reports.push(format!("{land}\n{out}").replace(&f.id, "<owner>"));
        ok(f.path(), &["merge", &f.id]);
        ok(f.path(), &["doctor"]);
        ok(f.path(), &["land"]);
    }
    assert_eq!(reports[0], reports[1]);
}

/// An uncommitted disposition is committed with the work, not restored: it
/// is the one change a review record gets (ADR 0025).
#[test]
fn an_uncommitted_disposition_is_committed_not_restored() {
    let f = Fixture::new();
    let finding = ok(
        f.path(),
        &[
            "-q",
            "review",
            "finding",
            &f.id,
            "--identity",
            "Reviewer",
            "--recommendation",
            "no_action",
            "Concern",
        ],
    );
    commit(f.path(), "finding");
    ok(
        f.path(),
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "no_action",
            "Fine",
        ],
    );
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(
        !pass
            && out.contains(&format!(
                "land_records_uncommitted: 1 tracker record differs from HEAD: .atelier/reviews/{finding}.md (disposition changed); commit them with the work. Next: git add .atelier && git commit -m \"…\"\n"
            )),
        "{out}"
    );
}

/// A separate disposition event read as folded keeps the old binary's
/// refusals (atelier-pfbp): deleting a finding's latest event outside the
/// fold that `review disposition` or the migration writes is refused, as is
/// restoring an event a later commit removed.
#[test]
fn separate_disposition_events_cannot_be_deleted_or_restored_by_hand() {
    for case in ["delete_latest", "restore_removed"] {
        let f = Fixture::new();
        git(f.path(), &["switch", "-q", "main"]);
        let id = ok(f.path(), &["-q", "issue", "create", "Event history"]);
        let finding = ok(
            f.path(),
            &[
                "-q",
                "review",
                "finding",
                &id,
                "--identity",
                "Reviewer",
                "--recommendation",
                "fix_now",
                "Concern",
            ],
        );
        commit(f.path(), "finding");
        let pin = git(f.path(), &["rev-parse", "HEAD"]);
        let first = if case == "delete_latest" {
            "Reviewer"
        } else {
            "Helper"
        };
        v1_disposition(
            f.path(),
            ("atelier-e001", &id, first, &pin, "2026-10-02T00:00:01Z"),
            &finding,
            "no_action",
            None,
            None,
            None,
        );
        commit(f.path(), "first disposition event");
        if case == "delete_latest" {
            v1_disposition(
                f.path(),
                (
                    "atelier-e002",
                    &id,
                    "Reviewer",
                    &pin,
                    "2026-10-02T00:00:02Z",
                ),
                &finding,
                "fix_now",
                Some("atelier-e001"),
                None,
                None,
            );
            commit(f.path(), "the reviewer's later fix_now");
            f.terminal(&id);
            let (pass, out) = cli(f.path(), &["doctor"]);
            assert!(!pass && out.contains("findings.disposed"), "{out}");
            git(f.path(), &["rm", "-q", ".atelier/reviews/atelier-e002.md"]);
            commit(f.path(), "delete the latest event");
        } else {
            git(f.path(), &["config", "user.name", "Helper"]);
            fs::write(f.path().join("helper-code"), "work by Helper\n").unwrap();
            commit(f.path(), "Helper's work");
            git(f.path(), &["config", "user.name", "Author"]);
            f.terminal(&id);
            ok(f.path(), &["doctor"]);
            ok(
                f.path(),
                &[
                    "review",
                    "disposition",
                    &finding,
                    "--identity",
                    "Other",
                    "--decision",
                    "no_action",
                    "--commit-records",
                    "Set again",
                ],
            );
            assert!(!f.path().join(".atelier/reviews/atelier-e001.md").exists());
            ok(f.path(), &["doctor"]);
            git(f.path(), &["checkout", "HEAD~1", "--", ".atelier/reviews/"]);
            commit(f.path(), "restore the removed event by hand");
        }
        let (pass, out) = cli(f.path(), &["doctor"]);
        assert!(!pass && out.contains("review_append_only"), "{case}: {out}");
    }
}

/// A branch whose older Atelier recorded a successor disposition event takes
/// a migrated base with a plain merge: the base never held the successor
/// event, so the branch may still correct it (atelier-ilba), and since every
/// merge is a merge commit no fast-forward makes it final first
/// (atelier-jnrh). Migrating the branch then folds it. Under
/// `fast_forward_only` the plain merge was refused, naming a migrating merge
/// (atelier-j5oq).
#[test]
fn a_migrated_base_merges_into_a_branch_with_events_by_migrating_the_merge() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let finding = ok(
        f.path(),
        &[
            "-q",
            "review",
            "finding",
            &f.id,
            "--identity",
            "Reviewer",
            "--recommendation",
            "no_action",
            "Concern",
        ],
    );
    let pin = git(f.path(), &["rev-parse", "HEAD"]);
    v1_disposition(
        f.path(),
        (
            "atelier-e001",
            &f.id,
            "Reviewer",
            &pin,
            "2026-10-02T00:00:01Z",
        ),
        &finding,
        "no_action",
        None,
        None,
        None,
    );
    commit(f.path(), "finding and its disposition event on the base");
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    v1_disposition(
        f.path(),
        (
            "atelier-e002",
            &f.id,
            "Reviewer",
            &pin,
            "2026-10-02T00:00:02Z",
        ),
        &finding,
        "no_action",
        Some("atelier-e001"),
        None,
        None,
    );
    commit(f.path(), "an older Atelier's successor event on the branch");
    git(f.path(), &["switch", "-q", "main"]);
    ok(f.path(), &["migrate", "dispositions", "--apply"]);
    commit(f.path(), "migrate dispositions on the base");
    git(f.path(), &["switch", "-q", &f.branch]);
    git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(pass && !out.contains("review_append_only"), "{out}");
    ok(f.path(), &["check"]);
    ok(f.path(), &["migrate", "dispositions", "--apply"]);
    commit(f.path(), "migrate the branch's successor event");
    assert!(!f.path().join(".atelier/reviews/atelier-e002.md").exists());
    let (pass, out) = cli(f.path(), &["land"]);
    assert!(pass && !out.contains("review_append_only"), "{out}");
    ok(f.path(), &["check"]);
}
