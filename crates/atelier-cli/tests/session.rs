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
        ok(dir.path(), &["init"]);
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
fn prime_and_active_landing_are_fresh_and_read_only() {
    let f = Fixture::new();
    let head = git(f.path(), &["rev-parse", "HEAD"]);
    let prime = ok(f.path(), &["prime"]);
    assert!(
        prime.contains(&f.id)
            && prime.contains("Ship session behavior")
            && prime.contains("AGENTS.md"),
        "{prime}"
    );
    ok(f.path(), &["land"]);
    assert_eq!(git(f.path(), &["rev-parse", "HEAD"]), head);
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
    ok(f.path(), &["issue", "note", &f.id, "Fresh handoff marker"]);
    commit(f.path(), "handoff");
    assert!(ok(f.path(), &["prime"]).contains("Fresh handoff marker"));
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
    let prime = ok(f.path(), &["prime"]);
    assert!(prime.contains("Pending active triage"), "{prime}");
    ok(f.path(), &["land"]);
    f.terminal(&f.id);
    f.refusal("review.current");
}

fn hook(dir: &Path, landing: bool, payload: serde_json::Value) -> serde_json::Value {
    let mut child = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir)
        .args([if landing { "land" } else { "prime" }, "--hook", "claude"])
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
            false,
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
        hook(unrelated.path(), true, input.clone()),
        serde_json::json!({})
    );
    ok(f.path(), &["issue", "note", &f.id, "Uncommitted handoff"]);
    let result = hook(unrelated.path(), true, input.clone());
    assert_eq!(result["decision"], "block");
    assert!(result["reason"]
        .as_str()
        .unwrap()
        .contains("land_records_uncommitted"));
    let mut repeated = input;
    repeated["stop_hook_active"] = true.into();
    let result = hook(unrelated.path(), true, repeated);
    assert!(result.get("decision").is_none());
    assert!(result["systemMessage"]
        .as_str()
        .unwrap()
        .contains("still fails"));
    let invalid = hook(
        unrelated.path(),
        false,
        serde_json::json!({"cwd":nested, "hook_event_name":"Stop"}),
    );
    assert!(invalid["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .unwrap()
        .contains("unavailable"));
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
    for (event, action) in [("SessionStart", "prime"), ("Stop", "land")] {
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
            && output.contains("[in_progress]")
            && output.contains("committed branch records"),
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
    owner.issue.issue_type = "epic".into();
    owner.sections.change_type("epic");
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
fn literal_checkout_branch_is_not_confused_with_same_named_tags() {
    let f = Fixture::new();
    git(f.path(), &["tag", &f.branch, "main"]);
    git(f.path(), &["tag", "main", "HEAD"]);
    let prime = ok(f.path(), &["prime"]);
    assert!(
        prime.contains(&format!("Branch: {} @", f.branch)),
        "{prime}"
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
fn hidden_executable_modes_allow_active_handoff_but_fence_completion() {
    use std::os::unix::fs::PermissionsExt;
    let f = Fixture::new();
    git(f.path(), &["config", "core.filemode", "false"]);
    fs::set_permissions(f.path().join("code"), fs::Permissions::from_mode(0o755)).unwrap();
    assert!(git(f.path(), &["status", "--porcelain"]).is_empty());
    assert!(ok(f.path(), &["land"]).contains("1 uncommitted path"));
    f.terminal(&f.id);
    f.refusal("land_completion_dirty");

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
    let (pass, output) = cli(f.path(), &["land"]);
    assert!(!pass && output.contains("land_state_changed"), "{output}");
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
        fs::write(f.path().join("unrelated-work"), "later unrelated work\n").unwrap();
        commit(f.path(), "aging cannot create independent acceptance");
        let (pass, output) = cli(f.path(), &["land"]);
        assert!(!pass && output.contains("findings.disposed"), "{output}");
    }
}
