//! Real Git/CLI session boundaries; hook payloads follow Claude Code's hooks reference.
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
use tempfile::{tempdir, TempDir};
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
fn cli(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_atelier"))
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
impl Fixture {
    fn new() -> Self {
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
    fn terminal(&self, id: &str) {
        let mut record = self.store().load_issue_by_id(id).unwrap();
        record.issue.status = "done".into();
        record.issue.closed_at = Some(chrono::Utc::now());
        record.sections.close_reason = Some("completed claim".into());
        self.store().write_issue_atomic(&record).unwrap();
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
                .find_map(|line| line.strip_prefix("No started work on: "))
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

fn hook(dir: &Path, command: &str, payload: serde_json::Value) -> serde_json::Value {
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
        .write_all(serde_json::to_string(&payload).unwrap().as_bytes())
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
    ok(f.path(), &["issue", "note", &f.id, "Uncommitted handoff"]);
    let result = hook(unrelated.path(), "land", input.clone());
    assert_eq!(result["decision"], "block");
    assert!(result["reason"]
        .as_str()
        .unwrap()
        .contains("land_records_uncommitted"));
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
    assert!(invalid["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .unwrap()
        .contains("unavailable"));
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
        output.contains(&format!("{} @", f.branch))
            && output.contains(&format!("{} in_progress ", f.id))
            && output.contains("Other Local Work Branches (committed records)"),
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
#[test]
fn land_names_uncommitted_files_that_make_a_newer_pass_stale() {
    let f = Fixture::new();
    let child = epic_child(&f);
    f.terminal(&child);
    for result in ["fail", "pass"] {
        ok(
            f.path(),
            &[
                "evidence",
                "record",
                "--target",
                &format!("issue/{child}"),
                "--kind",
                "test",
                "--result",
                result,
                "Done claim proof",
            ],
        );
        commit(f.path(), &format!("{result}ing proof"));
    }
    ok(f.path(), &["land", &f.id]);
    fs::write(f.path().join("code"), "uncommitted change\n").unwrap();
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass
            && output.contains("was recorded before the uncommitted changes in this checkout")
            && output.contains("Uncommitted changes make passing proof stale: code."),
        "{output}"
    );
}
#[test]
fn a_current_claim_whose_pass_a_later_commit_staled_does_not_blame_the_checkout() {
    let f = Fixture::new();
    let child = epic_child(&f);
    f.terminal(&child);
    for result in ["fail", "pass"] {
        ok(
            f.path(),
            &[
                "evidence",
                "record",
                "--target",
                &format!("issue/{child}"),
                "--kind",
                "test",
                "--result",
                result,
                "Done claim proof",
            ],
        );
        commit(f.path(), &format!("{result}ing proof"));
    }
    fs::write(f.path().join("code"), "committed after the pass\n").unwrap();
    commit(f.path(), "code after the pass");
    for dirty in [false, true] {
        if dirty {
            fs::write(f.path().join("unrelated"), "uncommitted\n").unwrap();
        }
        let (pass, output) = cli(f.path(), &["land", &f.id]);
        assert!(
            !pass
                && output.contains("no longer proves the claim")
                && !output.contains("uncommitted changes in this checkout")
                && !output.contains("Uncommitted changes make passing proof stale"),
            "{output}"
        );
    }
}
#[test]
fn a_retained_claim_whose_pass_later_commits_staled_does_not_blame_the_checkout() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let done = ok(f.path(), &["-q", "issue", "create", "Done on the base"]);
    commit(f.path(), "file it");
    for result in ["fail", "pass"] {
        ok(
            f.path(),
            &[
                "evidence",
                "record",
                "--target",
                &format!("issue/{done}"),
                "--kind",
                "test",
                "--result",
                result,
                "Proof",
            ],
        );
        commit(f.path(), &format!("{result}ing proof"));
    }
    fs::write(f.path().join("later"), "code after the pass\n").unwrap();
    commit(f.path(), "code after the pass");
    f.terminal(&done);
    ok(f.path(), &["-q", "issue", "create", "Later planning"]);
    commit(f.path(), "later planning");
    fs::write(f.path().join("unrelated"), "uncommitted\n").unwrap();
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass
            && output.contains("no longer proves the claim")
            && !output.contains("Uncommitted changes make passing proof stale"),
        "{output}"
    );
}
#[test]
fn done_child_cannot_hide_open_dependencies_new_failure_or_missing_required_fields() {
    for violation in ["dependency", "failure", "required_field"] {
        let f = Fixture::new();
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
            "failure" => {
                ok(
                    f.path(),
                    &[
                        "evidence",
                        "record",
                        "--target",
                        &format!("issue/{child}"),
                        "--kind",
                        "test",
                        "--result",
                        "fail",
                        "Done claim proof failed",
                    ],
                );
            }
            _ => {
                let mut record = f.store().load_issue_by_id(&child).unwrap();
                record.sections.close_reason = None;
                f.store().write_issue_atomic(&record).unwrap();
            }
        }
        commit(f.path(), "invalid done child");
        f.refusal(match violation {
            "dependency" => "blockers.transitive_none_open",
            "failure" => "evidence.no_newer_failure",
            _ => "required_field.close_reason",
        });
    }
}
#[test]
fn trusted_base_child_proof_cannot_be_removed_on_candidate() {
    let f = Fixture::new();
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
    commit(f.path(), "base requires completion proof");
    git(f.path(), &["switch", &f.branch]);
    git(f.path(), &["rebase", "main"]);
    fs::write(&path, policy).unwrap();
    let child = epic_child(&f);
    f.terminal(&child);
    f.refusal("evidence.attached");
    ok(
        f.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{child}"),
            "--kind",
            "test",
            "--result",
            "pass",
            "Fresh explicit child proof",
        ],
    );
    commit(f.path(), "child proof");
    ok(f.path(), &["land"]);
    fs::write(f.path().join("code"), "relevant code changed\n").unwrap();
    commit(f.path(), "changed code after proof");
    f.refusal("evidence.attached");
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
    f.refusal("candidate workflow policy is invalid");
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
            row.contains(&format!("for {}", f.id)) && row.ends_with(&format!("Checked pass {n}")),
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
        branch_base: "main".into(),
    });
    f.store().write_issue_atomic(&record).unwrap();
    commit(f.path(), "custom started branch-only work");
    git(f.path(), &["switch", "main"]);
    let output = ok(f.path(), &["status"]);
    assert!(
        output.contains("custom/ship-session @") && output.contains(&id),
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
    let branch = format!("task/{other}");
    let mut record = f.store().load_issue_by_id(&other).unwrap();
    record.issue.status = "in_progress".into();
    record.issue.workflow_branch = Some(atelier_core::WorkflowBranch {
        work_branch: branch.clone(),
        branch_base: "main".into(),
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
    let branch = format!("task/{other}");
    let mut record = f.store().load_issue_by_id(&other).unwrap();
    record.issue.status = "in_progress".into();
    record.issue.workflow_branch = Some(atelier_core::WorkflowBranch {
        work_branch: branch.clone(),
        branch_base: "main".into(),
    });
    f.store().write_issue_atomic(&record).unwrap();
    commit(f.path(), "two owners in shared committed tree");
    git(f.path(), &["branch", &branch]);
    git(f.path(), &["switch", "main"]);
    let output = ok(f.path(), &["status"]);
    assert!(
        output.contains(&format!("{} @", f.branch)) && output.contains(&format!("{branch} @")),
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

#[test]
fn global_terminal_claims_require_fields_and_resolved_dependencies_on_any_checkout() {
    for integration_base in [false, true] {
        let f = Fixture::new();
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
    let f = Fixture::new();
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
    f.refusal("evidence.attached");
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
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass
            && output.contains("evidence.no_newer_failure")
            && output.contains("findings.disposed"),
        "{output}"
    );
}
#[test]
fn integration_base_new_completion_cannot_omit_required_proof() {
    let f = Fixture::new();
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
    let id = ok(f.path(), &["-q", "issue", "create", "Missing global proof"]);
    commit(f.path(), "base requires proof");
    f.terminal(&id);
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(!pass && output.contains("evidence.attached"), "{output}");
    fs::write(f.path().join("later-code"), "unrelated\n").unwrap();
    commit(f.path(), "unrelated later code");
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(!pass && output.contains("evidence.attached"), "{output}");
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
            "Valid retained proof",
        ],
    );
    commit(f.path(), "valid proof for earlier completion");
    ok(f.path(), &["land"]);
    let stale = ok(
        f.path(),
        &["-q", "issue", "create", "Never valid old proof"],
    );
    commit(f.path(), "new task before old proof");
    ok(
        f.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{stale}"),
            "--kind",
            "test",
            "--result",
            "pass",
            "Proof before later implementation",
        ],
    );
    commit(f.path(), "premature proof");
    fs::write(f.path().join("later-code"), "implementation after proof\n").unwrap();
    commit(f.path(), "changed code before completion");
    f.terminal(&stale);
    fs::write(f.path().join("later-unrelated-code"), "age invalid claim\n").unwrap();
    commit(
        f.path(),
        "later unrelated commit cannot legitimate stale claim",
    );
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass && output.contains(&format!("{stale} evidence.attached")),
        "{output}"
    );
}

#[test]
fn retained_invalid_dispositions_and_approvals_never_age_into_acceptance() {
    for (fix_now, approver, stale) in [
        (false, "Author", false),
        (true, "Author", false),
        (true, "Reviewer", true),
    ] {
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
        assert!(
            !pass && output.contains("canonical_snapshot_uncommitted"),
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
    for (original_identity, rewritten_identity) in [("Reviewer", "Author"), ("Author", "Reviewer")]
    {
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
        let path = f.path().join(format!(".atelier/reviews/{disposition}.md"));
        let original = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            original.replace(original_identity, rewritten_identity),
        )
        .unwrap();
        commit(f.path(), "rewrite root event identity");
        let (pass, output) = cli(f.path(), &["land"]);
        assert!(!pass && output.contains("review_append_only"), "{output}");
    }
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
        record.sections.close_reason = Some("Verified terminal claim".into());
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
        fs::remove_file(path).unwrap();
        if mutation != "partial" {
            fs::remove_file(f.path().join(format!(".atelier/reviews/{finding}.md"))).unwrap();
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
            .contains("No work owner on branch feature/unowned; it changes no Atelier tracker files since main"),
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
            && output.contains("land_work_untraceable: branch feature/unowned has no owner but changes Atelier tracker files since main")
            && output.contains(&format!(".atelier/issues/{filed}.md"))
            && output.contains("atelier start <id>"),
        "{output}"
    );
    assert_eq!(git(f.path(), &["rev-parse", "HEAD"]), head);
    assert_eq!(hook(f.path(), "land", stop)["decision"], "block");
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
        output.contains("Base commits: 1 unpushed record-only commit(s)"),
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
    let output = ok(f.path(), &["land"]);
    assert!(
        output.contains("Base commits: 1 unpushed record-only commit(s)"),
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
    ok(
        f.path(),
        &["issue", "transition", &work, "close", "--reason", "did it"],
    );
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
    ok(
        other.path(),
        &[
            "issue",
            "transition",
            &f.id,
            "close",
            "--reason",
            "merged elsewhere",
        ],
    );
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
    ok(
        f.path(),
        &["issue", "transition", &work, "close", "--reason", "did it"],
    );
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
    ok(
        other.path(),
        &[
            "issue",
            "transition",
            &shared,
            "close",
            "--reason",
            "merged upstream",
        ],
    );
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
        ok(
            f.path(),
            &[
                "issue",
                "transition",
                &work,
                "close",
                "--reason",
                "reviewed elsewhere",
            ],
        );
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
    git(f.path(), &["switch", "-q", "-c", &format!("task/{owner}")]);
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
    let output = ok(f.path(), &["land"]);
    assert!(
        output.contains("Base commits: 1 unpushed record-only commit(s)"),
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
    let output = ok(f.path(), &["land"]);
    assert!(
        output.contains("Base commits: 2 unpushed record-only commit(s)"),
        "{output}"
    );
}
#[test]
fn land_on_a_branch_behind_its_base_hands_off_and_says_to_merge_the_base() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    fs::write(f.path().join("other"), "landed on the base\n").unwrap();
    commit(f.path(), "other work landed on the base");
    git(f.path(), &["switch", "-q", &f.branch]);

    // Active work hands off; the base moving on is not its failure.
    let output = ok(f.path(), &["land", &f.id]);
    assert!(
        output.contains("Base main has 1 commit(s) this branch does not")
            && output.contains("git merge main"),
        "{output}"
    );

    // Completed work is still checked against where it left the base, and
    // the note survives a failing gate.
    f.terminal(&f.id);
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass
            && output.contains("review.current: no current independent approval")
            && output.contains("git merge main"),
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
    assert!(output.contains("git merge main"), "{output}");
    let (pass, output) = cli(f.path(), &["merge", &f.id]);
    assert!(!pass && output.contains("merge_diverged"), "{output}");

    // Following the note: a clean merge of the base keeps the approval.
    git(f.path(), &["merge", "-q", "--no-edit", "main"]);
    let output = ok(f.path(), &["land", &f.id]);
    assert!(!output.contains("git merge main"), "{output}");
    ok(f.path(), &["merge", &f.id]);
}
#[test]
fn land_names_a_base_with_no_shared_history() {
    let f = Fixture::new();
    git(f.path(), &["checkout", "-q", "--orphan", "unrelated"]);
    commit(f.path(), "the same tree with no shared history");
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
    git(f.path(), &["switch", "-q", &format!("task/{second}")]);
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
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &fresh,
            "close",
            "--reason",
            "claimed",
        ],
    );
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
            "validation",
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
        )) && output.contains(&format!("atelier issue update {check} --no-parent")),
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

    // Following the advice detaches the check, which then waits on the merge.
    ok(f.path(), &["issue", "update", &check, "--no-parent"]);
    ok(
        f.path(),
        &["issue", "link", &check, &parent, "--role", "blocked_by"],
    );
    commit(f.path(), "make the check a standalone validation");
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
        output.contains(&format!("{} ✓ done ", f.id)) && output.contains("| merged into main"),
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
#[test]
fn land_checks_only_changed_claims_and_doctor_checks_every_claim() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    ok(f.path(), &["doctor"]);
    // A done claim with failing proof reaches the base without the gates,
    // as a direct push would.
    let old = ok(f.path(), &["-q", "issue", "create", "Pushed directly"]);
    commit(f.path(), "plan it");
    ok(f.path(), &["issue", "transition", &old, "start"]);
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &old,
            "close",
            "--reason",
            "pushed directly",
        ],
    );
    commit(f.path(), "done without the gates");
    let fail = |summary: &str| {
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
                "fail",
                summary,
            ],
        );
    };
    fail("Proof that failed");
    commit(f.path(), "failing proof");
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
        !pass && output.contains(&format!("{old} evidence.no_newer_failure")),
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
        !pass && output.contains(&format!("{old} evidence.no_newer_failure")),
        "{output}"
    );
    git(f.path(), &["switch", "-q", "main"]);
    git(f.path(), &["reset", "-q", "--hard", "HEAD^"]);
    git(f.path(), &["switch", "-q", &f.branch]);

    // A branch that adds a record about it, or edits it, checks it again.
    fail("Another failing run");
    commit(f.path(), "more failing proof on the branch");
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass && output.contains(&format!("{old} evidence.no_newer_failure")),
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
        !pass && output.contains(&format!("{old} evidence.no_newer_failure")),
        "{output}"
    );

    // It is still checked once a later commit leaves it alone.
    git(
        f.path(),
        &["commit", "-q", "--allow-empty", "-m", "later work"],
    );
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass && output.contains(&format!("{old} evidence.no_newer_failure")),
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
        !pass && output.contains(&format!("{old} evidence.no_newer_failure")),
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
            "The proof never passed",
        ],
    );
    commit(f.path(), "finding on the old claim");
    let (pass, output) = cli(f.path(), &["land", &f.id]);
    assert!(
        !pass && output.contains(&format!("{old} evidence.no_newer_failure")),
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
        !pass && output.contains(&format!("{old} evidence.no_newer_failure")),
        "{output}"
    );
}
#[test]
fn land_checks_claims_that_linked_records_or_removed_proof_affect() {
    let f = Fixture::new();
    git(f.path(), &["switch", "-q", "main"]);
    let old = ok(f.path(), &["-q", "issue", "create", "Pushed directly"]);
    let child = ok(
        f.path(),
        &["-q", "issue", "create", "--parent", &old, "Done child"],
    );
    commit(f.path(), "plan it");
    for id in [&child, &old] {
        ok(f.path(), &["issue", "transition", id, "start"]);
        ok(
            f.path(),
            &["issue", "transition", id, "close", "--reason", "done"],
        );
    }
    let record = |result: &str, summary: &str| {
        ok(
            f.path(),
            &[
                "-q",
                "evidence",
                "record",
                "--target",
                &format!("issue/{old}"),
                "--kind",
                "test",
                "--result",
                result,
                summary,
            ],
        )
    };
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
        ok(
            f.path(),
            &["issue", "transition", id, "close", "--reason", "done"],
        );
    }
    record("fail", "Proof that failed");
    let passed = record("pass", "Proof that passed later");
    let passed = passed.split_whitespace().next().unwrap();
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

    // Removing proof changes no record that names the claim any more.
    git(
        f.path(),
        &["rm", "-q", &format!(".atelier/evidence/{passed}.md")],
    );
    commit(f.path(), "remove the passing proof");
    refused("evidence.no_newer_failure");

    // Off the base, history is trusted only up to the base's upstream: the
    // local base can hold commits that no land has checked.
    git(f.path(), &["switch", "-q", "main"]);
    record("fail", "A newer failure");
    commit(f.path(), "unpushed failing proof");
    git(f.path(), &["switch", "-q", "-c", "feature"]);
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(
        !pass && output.contains(&format!("{old} evidence.no_newer_failure")),
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
        !pass && output.contains(&format!("{old} evidence.no_newer_failure")),
        "{output}"
    );

    // Without an upstream, nothing is trusted.
    git(f.path(), &["branch", "--unset-upstream", "main"]);
    let (pass, output) = cli(f.path(), &["land", &owner]);
    assert!(
        !pass && output.contains(&format!("{old} evidence.no_newer_failure")),
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
    ok(
        f.path(),
        &["issue", "transition", &work, "close", "--reason", "done"],
    );
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
    let f = Fixture::new();
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
    ok(
        f.path(),
        &["issue", "transition", &old, "close", "--reason", "done"],
    );
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
        !pass && output.contains("rerun atelier land without --install"),
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
        output.contains("Base commits: none unpushed outside recorded review"),
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
        status.contains(&format!(": {} ✓ done ", f.id))
            && status.contains(" | not merged into main | "),
        "{status}"
    );
    git(f.path(), &["merge", "-q", "--ff-only", &f.branch]);
    git(f.path(), &["switch", "-q", "-c", "elsewhere"]);
    let status = ok(f.path(), &["status"]);
    assert!(status.contains(" | merged into main | "), "{status}");

    // An uncommitted close on a fresh branch is not on the base either.
    git(f.path(), &["switch", "-q", "main"]);
    let fresh = ok(f.path(), &["-q", "issue", "create", "Fresh owner"]);
    commit(f.path(), "plan fresh owner");
    ok(f.path(), &["start", &fresh]);
    ok(
        f.path(),
        &[
            "issue",
            "transition",
            &fresh,
            "close",
            "--reason",
            "Claimed",
        ],
    );
    let status = ok(f.path(), &["status"]);
    assert!(
        field(&status, "Owner").is_some_and(|owner| owner.starts_with(&format!("{fresh} ✓ done "))
            && owner.ends_with(" | not merged into main")),
        "{status}"
    );
}
#[test]
fn ready_work_in_flight_on_another_branch_is_marked_with_its_state_there() {
    let f = Fixture::new();
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
        (
            &finished,
            format!("    ↳ ✓ done on task/{finished}, not merged"),
        ),
        (&stuck, format!("    ↳ ⚠ blocked on task/{stuck}")),
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
        format!("{finished} ✓ done medium Finished elsewhere | not merged into main"),
        format!("{stuck} ⚠ blocked medium Stuck elsewhere"),
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
    let unassigned = section(&status, "Unassigned Work (6)");
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
        output.contains("Base commits: none unpushed outside recorded review"),
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
        !pass && output.contains(" and 2 more; run atelier start"),
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
    let branch = format!("task/{parent}");
    assert_eq!(git(path, &["branch", "--show-current"]), branch);
    fs::write(path.join("code"), "first slice\n").unwrap();
    commit(path, "first slice");
    ok(path, &["land"]);
    ok(
        path,
        &[
            "issue",
            "transition",
            &first,
            "close",
            "--reason",
            "First slice done",
        ],
    );
    commit(path, "first child done");

    // The parent closes only after every child is terminal.
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
    assert!(!pass && output.contains(&second), "{output}");
    ok(path, &["start", &second]);
    assert_eq!(git(path, &["branch", "--show-current"]), branch);
    ok(
        path,
        &[
            "issue",
            "transition",
            &second,
            "close",
            "--reason",
            "Second slice done",
        ],
    );
    ok(
        path,
        &[
            "issue",
            "transition",
            &parent,
            "close",
            "--reason",
            "Both slices done",
        ],
    );
    commit(path, "children and parent done");
    let (pass, output) = cli(path, &["land"]);
    assert!(!pass && output.contains("review.current"), "{output}");
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
    ok(path, &["land"]);
    ok(path, &["merge", &first]);
    assert_eq!(git(path, &["branch", "--show-current"]), "main");
    for id in [&parent, &first, &second] {
        let record = fs::read_to_string(path.join(format!(".atelier/issues/{id}.md"))).unwrap();
        assert!(record.contains("status: \"done\""), "{record}");
    }
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
