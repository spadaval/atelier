use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use tempfile::tempdir;

static TEST_ISSUE_IDS: OnceLock<Mutex<HashMap<PathBuf, Vec<String>>>> = OnceLock::new();

/// Helper to run atelier commands in a temp directory
fn run_atelier(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let translated_args = translate_issue_refs_owned(dir, args);
    run_atelier_raw(
        dir,
        &translated_args
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
    )
}

fn run_atelier_raw(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir)
        .args(args)
        .output()
        .expect("Failed to execute atelier");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        register_issue_ids_from_stdout(dir, &stdout);
        register_issue_ids_from_state(dir);
    }

    (output.status.success(), stdout, stderr)
}

fn run_atelier_with_env(
    dir: &Path,
    args: &[&str],
    envs: &[(&str, &str)],
) -> (bool, String, String) {
    let translated_args = translate_issue_refs_owned(dir, args);
    let mut command = Command::new(env!("CARGO_BIN_EXE_atelier"));
    command.current_dir(dir).args(&translated_args);
    for (key, value) in envs {
        command.env(key, value);
    }
    let output = command.output().expect("Failed to execute atelier");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        register_issue_ids_from_stdout(dir, &stdout);
        register_issue_ids_from_state(dir);
    }

    (output.status.success(), stdout, stderr)
}

/// Initialize atelier in a temp directory
fn init_atelier(dir: &Path) {
    let (success, _, stderr) = run_atelier(dir, &["init"]);
    assert!(success, "Failed to init: {}", stderr);
}

fn init_git_repo(dir: &Path) {
    let status = Command::new("git")
        .current_dir(dir)
        .args(["init", "-q"])
        .status()
        .unwrap();
    assert!(status.success(), "git init failed");
    for args in [
        ["config", "user.email", "atelier-test@example.com"],
        ["config", "user.name", "Atelier Test"],
    ] {
        let status = Command::new("git")
            .current_dir(dir)
            .args(args)
            .status()
            .unwrap();
        assert!(status.success(), "git config failed");
    }
}

fn commit_all(dir: &Path, message: &str) {
    let status = Command::new("git")
        .current_dir(dir)
        .args(["add", "-A"])
        .status()
        .unwrap();
    assert!(status.success(), "git add failed");
    let output = Command::new("git")
        .current_dir(dir)
        .args(["commit", "-q", "-m", message])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git commit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_atelier_with_telemetry_disabled(dir: &Path) {
    let (success, _, stderr) =
        run_atelier_with_env(dir, &["init"], &[("ATELIER_TELEMETRY", "off")]);
    assert!(success, "Failed to init: {}", stderr);
}

fn diagnostics_events(root: &Path) -> Vec<serde_json::Value> {
    let commands_dir = root.join("commands");
    if !commands_dir.exists() {
        return Vec::new();
    }
    let mut paths = std::fs::read_dir(&commands_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    paths.sort();
    let mut events = Vec::new();
    for path in paths {
        let content = fs::read_to_string(path).unwrap();
        for line in content.lines().filter(|line| !line.trim().is_empty()) {
            events.push(serde_json::from_str(line).unwrap());
        }
    }
    events
}

fn write_diagnostics_event(root: &Path, date: &str, event: serde_json::Value) {
    let commands_dir = root.join("commands");
    fs::create_dir_all(&commands_dir).unwrap();
    let path = commands_dir.join(format!("{date}.ndjson"));
    let mut line = serde_json::to_string(&event).unwrap();
    line.push('\n');
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap()
        .write_all(line.as_bytes())
        .unwrap();
}

fn registry() -> &'static Mutex<HashMap<PathBuf, Vec<String>>> {
    TEST_ISSUE_IDS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn issue_ref(dir: &Path, ordinal: usize) -> String {
    registry()
        .lock()
        .unwrap()
        .get(dir)
        .and_then(|ids| ids.get(ordinal - 1))
        .cloned()
        .unwrap_or_else(|| ordinal.to_string())
}

fn issue_key(id: &str) -> &str {
    id.rsplit_once('-').map(|(_, key)| key).unwrap_or(id)
}

fn issue_id_by_title(dir: &Path, title: &str) -> String {
    record_id_by_title(dir, "issues", title)
}

fn canonical_issue_path(dir: &Path, issue_id: &str) -> PathBuf {
    dir.join(".atelier")
        .join("issues")
        .join(format!("{issue_id}.md"))
}

fn ignored_test_source(ignore_attribute: &str, test_name: &str) -> String {
    format!("#[test]\n#[{ignore_attribute}]\nfn {test_name}() {{}}\n")
}

fn valid_command_surface_doc() -> &'static str {
    r#"# CLI Surface Tiers

## Core

- `atelier init`
- `atelier prime`
- `atelier status`
- `atelier start`
- `atelier finish`
- `atelier issue ...`
- `atelier dep add/remove/list`
- `atelier search <query>`
- `atelier link add/remove/list`
- `atelier graph impact/tree`
- `atelier note add`
- `atelier mission create/show/list/status/update`
- `atelier mission audit`
- `atelier mission add-work/add-blocker`
- `atelier plan create/show/list/revise/link/apply`
- `atelier evidence add/capture/show/list/attach`
- `atelier history`
- `atelier worktree for/status/merge/remove`
- `atelier export`
- `atelier rebuild`
- `atelier import-beads`
- `atelier integrations claude install`
- `atelier maintenance delete`
- `atelier diagnostics slow`
- `atelier lint`
- `atelier doctor`
"#
}

fn write_valid_command_guidance(dir: &Path) {
    let docs_dir = dir.join("docs/product");
    fs::create_dir_all(&docs_dir).unwrap();
    fs::write(docs_dir.join("cli-surface.md"), valid_command_surface_doc()).unwrap();
    fs::write(
        dir.join("AGENTFACTORY.md"),
        "# Agent Factory Binding\n\n- `atelier status`\n- `atelier mission status [<id>]`\n- `atelier mission audit <id>`\n- `atelier issue show <id>`\n",
    )
    .unwrap();
}

fn remove_issue_section(markdown: &str, heading: &str) -> String {
    let marker = format!("## {heading}\n");
    let start = markdown.find(&marker).expect("section heading missing");
    let rest = &markdown[start + marker.len()..];
    let end = rest
        .find("\n## ")
        .map(|offset| start + marker.len() + offset)
        .unwrap_or(markdown.len());
    format!("{}{}", &markdown[..start], &markdown[end..])
}

fn record_id_by_title(dir: &Path, directory: &str, title: &str) -> String {
    let record_dir = dir.join(".atelier").join(directory);
    let entries = std::fs::read_dir(&record_dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", record_dir.display()));
    for entry in entries {
        let path = entry.unwrap().path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        if text.contains(&format!("title: {title:?}")) {
            return path.file_stem().unwrap().to_string_lossy().to_string();
        }
    }
    panic!(
        "record with title {title:?} not found in {}",
        record_dir.display()
    );
}

fn resolve_test_issue_ref(dir: &Path, issue_ref_value: &str) -> String {
    issue_ref_value
        .parse::<usize>()
        .ok()
        .map(|ordinal| issue_ref(dir, ordinal))
        .unwrap_or_else(|| issue_ref_value.to_string())
}

fn attach_pass_evidence(dir: &Path, target_kind: &str, target_id: &str, summary: &str) -> String {
    let (success, evidence_out, stderr) = run_atelier(
        dir,
        &[
            "evidence",
            "add",
            "--kind",
            "validation",
            "--result",
            "pass",
            summary,
        ],
    );
    assert!(success, "evidence add failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass"), "{evidence_out}");
    let evidence_id = record_id_by_title(dir, "evidence", summary);
    let (success, _, stderr) = run_atelier(
        dir,
        &["evidence", "attach", &evidence_id, target_kind, target_id],
    );
    assert!(success, "evidence attach failed: {stderr}");
    evidence_id
}

fn attach_issue_pass_evidence(dir: &Path, issue_id: &str) -> String {
    attach_pass_evidence(
        dir,
        "issue",
        issue_id,
        &format!("issue close proof for {issue_id}"),
    )
}

fn close_issue_with_evidence(dir: &Path, issue_ref_value: &str, reason: Option<&str>) -> String {
    let issue_id = resolve_test_issue_ref(dir, issue_ref_value);
    attach_issue_pass_evidence(dir, &issue_id);
    let mut args = vec!["issue", "close", issue_ref_value];
    if let Some(reason) = reason {
        args.push("--reason");
        args.push(reason);
    }
    let (success, _, stderr) = run_atelier(dir, &args);
    assert!(success, "issue close failed: {stderr}");
    issue_id
}

fn issue_activity_texts(dir: &Path, issue_id: &str) -> Vec<String> {
    let activity_dir = dir
        .join(".atelier")
        .join("issues")
        .join(format!("{issue_id}.activity"));
    let mut paths = std::fs::read_dir(&activity_dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", activity_dir.display()))
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
        })
        .collect()
}

fn assert_activity_contains(texts: &[String], event_type: &str, expected: &[&str]) {
    let needle = format!("event_type: \"{event_type}\"");
    assert!(
        texts
            .iter()
            .any(|text| text.contains(&needle) && expected.iter().all(|part| text.contains(part))),
        "missing activity {event_type} containing {expected:?}; activities:\n{}",
        texts.join("\n--- activity ---\n")
    );
}

fn register_issue_id(dir: &Path, id: String) {
    if !is_record_id(&id) {
        return;
    }
    let mut map = registry().lock().unwrap();
    let ids = map.entry(dir.to_path_buf()).or_default();
    if !ids.contains(&id) {
        ids.push(id);
    }
}

fn register_issue_ids_from_stdout(dir: &Path, stdout: &str) {
    let bytes = stdout.as_bytes();
    let mut index = 0;
    while let Some(offset) = stdout[index..].find("atelier-") {
        let start = index + offset;
        let mut end = start;
        while end < bytes.len() && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'-') {
            end += 1;
        }
        register_issue_id(dir, stdout[start..end].to_string());
        index = end;
    }
}

fn register_issue_ids_from_state(dir: &Path) {
    let issues_dir = dir.join(".atelier/issues");
    let Ok(entries) = std::fs::read_dir(issues_dir) else {
        return;
    };
    let mut ids = entries
        .filter_map(Result::ok)
        .filter_map(|entry| entry.path().file_stem()?.to_str().map(str::to_owned))
        .filter(|id| is_record_id(id))
        .collect::<Vec<_>>();
    ids.sort();
    for id in ids {
        register_issue_id(dir, id);
    }
}

fn is_record_id(value: &str) -> bool {
    let Some((slug, suffix)) = value.rsplit_once('-') else {
        return false;
    };
    !slug.is_empty()
        && suffix.len() == 4
        && slug
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && suffix
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
}

fn translate_issue_refs_owned<T: AsRef<str>>(dir: &Path, args: &[T]) -> Vec<String> {
    args.iter()
        .enumerate()
        .map(|(index, arg)| {
            if issue_ref_position(args, index) {
                translate_issue_ref(dir, arg.as_ref())
            } else {
                arg.as_ref().to_string()
            }
        })
        .collect()
}

fn translate_issue_ref(dir: &Path, value: &str) -> String {
    let numeric = value.strip_prefix('#').unwrap_or(value);
    match numeric.parse::<usize>() {
        Ok(ordinal) => issue_ref(dir, ordinal),
        Err(_) => value.to_string(),
    }
}

fn command_offset<T: AsRef<str>>(args: &[T]) -> usize {
    args.iter()
        .position(|arg| !arg.as_ref().starts_with('-'))
        .unwrap_or(args.len())
}

fn issue_ref_position<T: AsRef<str>>(args: &[T], index: usize) -> bool {
    let offset = command_offset(args);
    if index <= offset {
        return false;
    }

    let rest = args
        .get(offset..)
        .unwrap_or_default()
        .iter()
        .map(|arg| arg.as_ref())
        .collect::<Vec<_>>();
    match rest.as_slice() {
        ["show" | "update" | "close" | "reopen" | "delete" | "start" | "related", ..] => {
            index == offset + 1
        }
        ["label" | "unlabel" | "comment", ..] => index == offset + 1,
        ["block" | "unblock" | "relate" | "unrelate", ..] => {
            index == offset + 1 || index == offset + 2
        }
        ["subissue", ..] => index == offset + 1,
        ["session", "work", ..] => index == offset + 2,
        ["archive", "add" | "remove", ..] => index == offset + 2,
        ["milestone", "add" | "remove", ..] => index > offset + 2,
        ["issue", "show" | "update" | "close" | "reopen" | "delete" | "related" | "impact", ..] => {
            index == offset + 2
        }
        ["issue", "label" | "unlabel" | "comment", ..] => index == offset + 2,
        ["issue", "block" | "unblock" | "relate" | "unrelate", ..] => {
            index == offset + 2 || index == offset + 3
        }
        ["issue", "subissue", ..] => index == offset + 2,
        ["link", "add" | "remove", source_kind, _, target_kind, ..] => {
            (*source_kind == "issue" && index == offset + 3)
                || (*target_kind == "issue" && index == offset + 5)
        }
        ["link", "list", target_kind, ..] => *target_kind == "issue" && index == offset + 3,
        ["graph", "impact", ..] => index == offset + 2,
        ["note", "add", target_kind, ..] => *target_kind == "issue" && index == offset + 3,
        ["maintenance", "delete", target_kind, ..] => {
            *target_kind == "issue" && index == offset + 3
        }
        ["dep", "list", ..] => index == offset + 2,
        ["dep", "add" | "remove", ..] => index == offset + 2 || index == offset + 3,
        _ => false,
    }
}

// ==================== Init Tests ====================

#[test]
fn test_init_creates_atelier_directory() {
    let dir = tempdir().unwrap();
    let (success, stdout, _) = run_atelier(dir.path(), &["init"]);

    assert!(success);
    assert!(stdout.contains("Created") || stdout.contains("initialized"));
    assert!(dir.path().join(".atelier").exists());
    assert!(dir.path().join(".atelier").join("state.db").exists());
    assert!(dir.path().join(".atelier").join("config.toml").exists());
    assert!(!dir.path().join(".atelier").join("rules").exists());
    assert!(!dir.path().join(".atelier").join("rules.local").exists());
    assert!(!dir
        .path()
        .join(".atelier")
        .join("hook-config.json")
        .exists());
    assert!(!dir.path().join(".claude").exists());
    assert!(!dir.path().join(".mcp.json").exists());
}

#[test]
fn test_init_twice_is_idempotent() {
    let dir = tempdir().unwrap();

    run_atelier(dir.path(), &["init"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["init"]);

    assert!(success);
    assert!(stdout.contains("Atelier initialized successfully"));
    assert!(dir.path().join(".atelier").join("state.db").exists());
    assert!(!dir.path().join(".atelier").join("rules").exists());
    assert!(!dir.path().join(".claude").exists());
}

#[test]
fn test_claude_integration_install_is_explicit() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    assert!(!dir.path().join(".claude").exists());
    assert!(!dir.path().join(".mcp.json").exists());
    assert!(!dir.path().join(".atelier").join("rules").exists());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["integrations", "claude", "install"]);
    assert!(success, "claude integration install failed: {stderr}");
    assert!(stdout.contains("Claude integration installed."));

    assert!(dir.path().join(".claude/settings.json").exists());
    assert!(dir.path().join(".claude/hooks/prompt-guard.py").exists());
    assert!(dir.path().join(".claude/hooks/post-edit-check.py").exists());
    assert!(dir.path().join(".claude/hooks/session-start.py").exists());
    assert!(dir.path().join(".claude/hooks/pre-web-check.py").exists());
    assert!(dir.path().join(".claude/hooks/work-check.py").exists());
    assert!(dir.path().join(".claude/hooks/atelier_config.py").exists());
    assert!(dir.path().join(".claude/mcp/safe-fetch-server.py").exists());
    assert!(dir.path().join(".atelier/hook-config.json").exists());
    assert!(!dir.path().join(".atelier/rules").exists());

    let mcp: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.path().join(".mcp.json")).unwrap())
            .unwrap();
    assert!(mcp["mcpServers"]["atelier-safe-fetch"].is_object());
}

#[test]
fn test_doctor_human_separates_projection_and_runtime_state_health() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    run_atelier(dir.path(), &["issue", "create", "Health check"]);
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["doctor"]);
    assert!(success, "doctor failed: {stderr}");
    assert!(stdout.contains("Install health:"));
    assert!(stdout.contains("ignored_runtime_paths: ok"));
    assert!(stdout.contains("Projection rebuild:"));
    assert!(stdout.contains("rebuild_ready: ok"));
    assert!(stdout.contains("projection_fresh: ok"));
    assert!(stdout.contains("Cache health:"));
    assert!(stdout.contains("projection_metadata: ok"));
    assert!(stdout.contains("Runtime state:"));
    assert!(stdout.contains("database: ok"));
    assert!(stdout.contains("local_tables: ok"));
    assert!(stdout.contains("diagnostics:"));
}

#[test]
fn test_doctor_distinguishes_missing_runtime_projection_database() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Missing projection db"]);
    assert!(success, "issue create failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/state.db")).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["doctor"]);
    assert!(success, "doctor failed: {stderr}");
    assert!(stdout.contains("Projection rebuild:"));
    assert!(stdout.contains("projection_fresh: not ok"));
    assert!(stdout.contains("Runtime state:"));
    assert!(stdout.contains("database: missing (runtime projection artifact)"));
    assert!(stdout.contains("projection_metadata: stale"));
}

#[test]
fn test_doctor_reports_runtime_health_without_becoming_canonical_lint() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let body = "## Description\n\nDoctor boundary body.\n\n## Outcome\n\nDoctor continues reporting runtime health when canonical Markdown is malformed.\n\n## Evidence\n\n- `atelier doctor` reports runtime health while `atelier lint` reports invalid YAML.";
    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Doctor runtime boundary",
            "--description",
            body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);

    let issue_path = dir
        .path()
        .join(".atelier/issues")
        .join(format!("{issue_id}.md"));
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(
        &issue_path,
        markdown.replace(
            "title: \"Doctor runtime boundary\"",
            "title: [Doctor runtime boundary",
        ),
    )
    .unwrap();

    let (lint_success, lint_stdout, lint_stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(
        !lint_success,
        "lint must reject malformed canonical Markdown, stdout: {lint_stdout}"
    );
    let lint_transcript = format!("{lint_stdout}\n{lint_stderr}");
    assert!(
        lint_transcript.contains("Canonical tracker Markdown is invalid")
            && lint_transcript.contains("Invalid YAML front matter"),
        "unexpected lint error: {lint_transcript}"
    );

    let (doctor_success, doctor_stdout, doctor_stderr) = run_atelier(dir.path(), &["doctor"]);
    assert!(
        doctor_success,
        "doctor should continue reporting runtime health: {doctor_stderr}"
    );
    assert!(doctor_stdout.contains("Projection rebuild:"));
    assert!(doctor_stdout.contains("rebuild_ready: not ok"));
    assert!(doctor_stdout.contains("Runtime state:"));
    assert!(doctor_stdout.contains("database: ok"));
    assert!(doctor_stdout.contains("local_tables: ok"));
}

#[test]
fn test_command_telemetry_records_success_event() {
    let dir = tempdir().unwrap();
    init_atelier_with_telemetry_disabled(dir.path());
    let diagnostics_dir = dir.path().join("diagnostics");

    let (success, _, stderr) = run_atelier_with_env(
        dir.path(),
        &["doctor"],
        &[("ATELIER_DIAGNOSTICS_DIR", diagnostics_dir.to_str().unwrap())],
    );
    assert!(success, "doctor failed: {stderr}");

    let events = diagnostics_events(&diagnostics_dir);
    assert_eq!(events.len(), 1);
    let event = &events[0];
    assert_eq!(event["schema"], "atelier.command_event");
    assert_eq!(event["schema_version"], 1);
    assert_eq!(event["command"], "doctor");
    assert_eq!(event["result"], "success");
    assert_eq!(event["exit_code"], 0);
    assert_eq!(event["argv_capture"], "none");
    assert_eq!(event["argv_redacted"].as_array().unwrap().len(), 0);
    assert!(event["duration_ms"].as_u64().is_some());
    assert!(event["workspace_id"].as_str().unwrap().len() >= 16);
    assert!(event["workspace_root"].is_null());
    assert_eq!(event["state_path"], ".atelier");
}

#[test]
fn test_command_telemetry_records_failure_event() {
    let dir = tempdir().unwrap();
    init_atelier_with_telemetry_disabled(dir.path());
    let diagnostics_dir = dir.path().join("diagnostics");

    let (success, _, _) = run_atelier_with_env(
        dir.path(),
        &["issue", "show", "missing"],
        &[("ATELIER_DIAGNOSTICS_DIR", diagnostics_dir.to_str().unwrap())],
    );
    assert!(!success);

    let events = diagnostics_events(&diagnostics_dir);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["command"], "issue show");
    assert_eq!(events[0]["result"], "failure");
    assert_eq!(events[0]["exit_code"], 1);
}

#[test]
fn test_command_telemetry_respects_opt_out_controls() {
    let dir = tempdir().unwrap();
    init_atelier_with_telemetry_disabled(dir.path());
    let diagnostics_dir = dir.path().join("diagnostics");

    let (success, _, stderr) = run_atelier_with_env(
        dir.path(),
        &["doctor"],
        &[
            ("ATELIER_DIAGNOSTICS_DIR", diagnostics_dir.to_str().unwrap()),
            ("ATELIER_TELEMETRY", "off"),
        ],
    );
    assert!(success, "doctor failed: {stderr}");

    let (success, _, stderr) = run_atelier_with_env(
        dir.path(),
        &["doctor"],
        &[
            ("ATELIER_DIAGNOSTICS_DIR", diagnostics_dir.to_str().unwrap()),
            ("ATELIER_DIAGNOSTICS", "disabled"),
        ],
    );
    assert!(success, "doctor failed: {stderr}");

    assert!(diagnostics_events(&diagnostics_dir).is_empty());
}

#[test]
fn test_command_telemetry_omits_sensitive_arguments_by_default() {
    let dir = tempdir().unwrap();
    init_atelier_with_telemetry_disabled(dir.path());
    let diagnostics_dir = dir.path().join("diagnostics");
    let secret_title = "secret-token-should-not-appear";

    let (success, _, stderr) = run_atelier_with_env(
        dir.path(),
        &["issue", "create", secret_title],
        &[("ATELIER_DIAGNOSTICS_DIR", diagnostics_dir.to_str().unwrap())],
    );
    assert!(success, "issue create failed: {stderr}");

    let events = diagnostics_events(&diagnostics_dir);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["command"], "issue create");
    let raw_event = serde_json::to_string(&events[0]).unwrap();
    assert!(
        !raw_event.contains(secret_title),
        "telemetry event leaked raw issue title: {raw_event}"
    );
    assert_eq!(events[0]["argv_redacted"].as_array().unwrap().len(), 0);
}

#[test]
fn test_command_telemetry_ignores_relative_diagnostics_dir() {
    let dir = tempdir().unwrap();
    init_atelier_with_telemetry_disabled(dir.path());

    let (success, _, stderr) = run_atelier_with_env(
        dir.path(),
        &["doctor"],
        &[("ATELIER_DIAGNOSTICS_DIR", "relative-diagnostics")],
    );
    assert!(success, "doctor failed: {stderr}");
    assert!(!dir.path().join("relative-diagnostics").exists());
}

#[test]
fn test_diagnostics_slow_handles_missing_telemetry_store() {
    let dir = tempdir().unwrap();
    let diagnostics_dir = dir.path().join("diagnostics");

    let (success, stdout, stderr) = run_atelier_with_env(
        dir.path(),
        &["diagnostics", "slow"],
        &[("ATELIER_DIAGNOSTICS_DIR", diagnostics_dir.to_str().unwrap())],
    );
    assert!(success, "diagnostics slow failed: {stderr}");

    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(parsed["schema"], "atelier.slow_commands");
    assert_eq!(parsed["schema_version"], 1);
    assert_eq!(parsed["window_days"], 7);
    assert_eq!(parsed["threshold_ms"], 1000);
    assert_eq!(parsed["rows"].as_array().unwrap().len(), 0);
}

#[test]
fn test_diagnostics_slow_summarizes_fixture_events() {
    let dir = tempdir().unwrap();
    let diagnostics_dir = dir.path().join("diagnostics");
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let old = chrono::Utc::now()
        .date_naive()
        .checked_sub_days(chrono::Days::new(20))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();

    write_diagnostics_event(
        &diagnostics_dir,
        &today,
        serde_json::json!({
            "schema": "atelier.command_event",
            "schema_version": 1,
            "event_id": "one",
            "command": "issue show",
            "started_at": format!("{today}T01:00:00.000Z"),
            "finished_at": format!("{today}T01:00:01.200Z"),
            "duration_ms": 1200,
            "result": "success",
            "workspace_id": "workspace-a"
        }),
    );
    write_diagnostics_event(
        &diagnostics_dir,
        &today,
        serde_json::json!({
            "schema": "atelier.command_event",
            "schema_version": 1,
            "event_id": "two",
            "command": "issue show",
            "started_at": format!("{today}T02:00:00.000Z"),
            "finished_at": format!("{today}T02:00:02.400Z"),
            "duration_ms": 2400,
            "result": "failure",
            "workspace_id": "workspace-a"
        }),
    );
    write_diagnostics_event(
        &diagnostics_dir,
        &today,
        serde_json::json!({
            "schema": "atelier.command_event",
            "schema_version": 1,
            "event_id": "fast",
            "command": "issue show",
            "started_at": format!("{today}T03:00:00.000Z"),
            "finished_at": format!("{today}T03:00:00.100Z"),
            "duration_ms": 100,
            "result": "success",
            "workspace_id": "workspace-a"
        }),
    );
    write_diagnostics_event(
        &diagnostics_dir,
        &today,
        serde_json::json!({
            "schema": "atelier.command_event",
            "schema_version": 1,
            "event_id": "three",
            "command": "doctor",
            "started_at": format!("{today}T04:00:00.000Z"),
            "finished_at": format!("{today}T04:00:02.000Z"),
            "duration_ms": 2000,
            "result": "success",
            "workspace_id": "workspace-b"
        }),
    );
    write_diagnostics_event(
        &diagnostics_dir,
        &old,
        serde_json::json!({
            "schema": "atelier.command_event",
            "schema_version": 1,
            "event_id": "old",
            "command": "doctor",
            "started_at": format!("{old}T04:00:00.000Z"),
            "finished_at": format!("{old}T04:00:05.000Z"),
            "duration_ms": 5000,
            "result": "success",
            "workspace_id": "workspace-z"
        }),
    );

    let (success, stdout, stderr) = run_atelier_with_env(
        dir.path(),
        &[
            "diagnostics",
            "slow",
            "--days",
            "7",
            "--threshold-ms",
            "1000",
        ],
        &[("ATELIER_DIAGNOSTICS_DIR", diagnostics_dir.to_str().unwrap())],
    );
    assert!(success, "diagnostics slow failed: {stderr}");

    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let rows = parsed["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 2);

    assert_eq!(rows[0]["workspace_id"], "workspace-a");
    assert_eq!(rows[0]["command"], "issue show");
    assert_eq!(rows[0]["bucket"], today);
    assert_eq!(rows[0]["count"], 2);
    assert_eq!(rows[0]["failure_count"], 1);
    assert_eq!(rows[0]["min_duration_ms"], 1200);
    assert_eq!(rows[0]["max_duration_ms"], 2400);
    assert_eq!(rows[0]["mean_duration_ms"], 1800.0);
    assert_eq!(rows[0]["p50_duration_ms"], 1200);
    assert_eq!(rows[0]["p95_duration_ms"], 2400);

    assert_eq!(rows[1]["workspace_id"], "workspace-b");
    assert_eq!(rows[1]["command"], "doctor");
    assert_eq!(rows[1]["count"], 1);
    assert_eq!(rows[1]["max_duration_ms"], 2000);
}

#[test]
fn test_top_level_help_only_shows_core_commands() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["--help"]);
    assert!(success, "help failed: {stderr}");

    for heading in [
        "Setup:",
        "Orientation:",
        "Issues:",
        "Missions and planning:",
        "Records:",
        "Advanced work:",
        "State management:",
        "Maintenance:",
        "Common commands:",
        "Options:",
    ] {
        assert!(stdout.contains(heading), "missing help heading {heading}");
    }

    for command in [
        "init",
        "prime",
        "status",
        "issue",
        "dep",
        "mission",
        "plan",
        "evidence",
        "history",
        "worktree",
        "export",
        "rebuild",
        "import-beads",
        "lint",
        "doctor",
    ] {
        assert!(stdout.contains(command), "missing core command {command}");
    }

    for common in [
        "atelier prime",
        "atelier issue list",
        "atelier issue list --ready",
        "atelier issue show <id>",
        "atelier mission list",
        "atelier mission show <id>",
        "atelier history --mission <id>",
        "atelier history --issue <id>",
        "atelier start <issue-id>",
        "atelier finish [issue-id]",
        "atelier doctor",
    ] {
        assert!(
            stdout.contains(common),
            "missing common command example {common}"
        );
    }
    assert!(!stdout.contains("workflow validate"));

    assert!(
        !stdout.contains("\nCommands:\n"),
        "top-level help should use categorized commands, not a flat command dump:\n{stdout}"
    );

    for removed in [
        "archive",
        "timer",
        "milestone",
        "session",
        "daemon",
        "cpitd",
        "usage",
        "agent",
        "locks",
        "sync",
        "work",
        "workflow",
    ] {
        assert!(
            !stdout.lines().any(|line| {
                let command = line.trim_start();
                command == removed || command.starts_with(&format!("{removed} "))
            }),
            "removed command {removed} is still visible in help:\n{stdout}"
        );
    }
}

#[test]
fn test_workflow_help_is_scoped_as_advanced_internal_diagnostic() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["workflow", "--help"]);
    assert!(success, "workflow help failed: {stderr}");
    assert!(stdout.contains("Advanced/internal workflow policy diagnostics"));
    assert!(stdout.contains("advanced diagnostic"));
}

#[test]
fn test_agent_factory_guidance_avoids_raw_workflow_validate_commands() {
    let guidance =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("AGENTFACTORY.md"))
            .unwrap();
    assert!(guidance.contains("Hidden workflow diagnostics are not normal"));
    assert!(!guidance.contains("atelier workflow validate issue"));
    assert!(!guidance.contains("atelier workflow validate mission"));
}

#[test]
fn test_mission_help_uses_show_not_view() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["mission", "--help"]);
    assert!(success, "mission help failed: {stderr}");

    assert!(stdout.contains("show"));
    assert!(!stdout.contains("view"));
}

#[test]
fn test_root_status_summarizes_checkout_orientation() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(dir.path(), &["issue", "create", "Ready item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(stdout.contains("Atelier Status"));
    assert!(stdout.contains("Tracker:"));
    assert!(stdout.contains("Ready work:"));
    assert!(stdout.contains("Active work:"));
    assert!(stdout.contains("Active mission:"));
    assert!(stdout.contains("Next Actions"));
    assert!(
        stdout.contains("Inspect mission readiness (no mission is active): atelier mission status")
    );
    assert!(stdout
        .contains("Choose ready work (1 ready issue(s) available): atelier issue list --ready"));
    assert!(stdout.contains("Start selected work (ready work exists): atelier start <issue-id>"));
    assert!(stdout.contains("Check runtime health (tracker export is current): atelier doctor"));
    assert!(!stdout.contains("workflow validate"));
    assert!(!stdout.contains("issue next"));
    assert!(!stdout.contains("session"));

    let (success, quiet, stderr) = run_atelier(dir.path(), &["--quiet", "status"]);
    assert!(success, "quiet status failed: {stderr}");
    assert!(quiet.contains("work="));
    assert!(quiet.contains("ready="));
    assert!(quiet.contains("tracker="));
}

#[test]
fn test_root_status_no_ready_work_suggests_valid_blocked_list() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(stdout.contains(
        "Inspect blocked work (no ready work is available): atelier issue list --blocked"
    ));
    assert!(!stdout.contains("workflow validate"));
    assert!(!stdout.contains("issue blocked"));

    let (success, blocked_out, stderr) = run_atelier(dir.path(), &["issue", "list", "--blocked"]);
    assert!(success, "suggested blocked-list command failed: {stderr}");
    assert!(blocked_out.contains("No blocked issues."));
}

#[test]
fn test_prime_guides_empty_checkout_without_repeating_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prime"]);
    assert!(success, "prime failed: {stderr}");
    assert!(stdout.contains("Atelier Prime"));
    assert!(stdout.contains("Context Recovery"));
    assert!(stdout.contains("Active mission: none"));
    assert!(stdout.contains("Active work: none"));
    assert!(stdout.contains("Core Rules"));
    assert!(stdout.contains("Essential Commands"));
    assert!(stdout.contains("Common Workflows"));
    assert!(stdout.contains("Validation/Closeout Checklist"));
    assert!(stdout.contains("Repository Notes"));
    assert!(stdout.contains(
        "`atelier status` - Check active work, active mission, ready count, and tracker freshness."
    ));
    assert!(stdout.contains(
        "`atelier history --issue <id>` - Inspect full canonical activity instead of relying on chat memory."
    ));
    assert!(!stdout.contains("Atelier Status"));
    assert!(!stdout.contains("Generic"));
    assert!(!stdout.contains("etc."));
}

#[test]
fn test_prime_names_active_mission() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Prime mission"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Prime mission");
    let mission_id = mission_id.as_str();
    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "start", mission_id]);
    assert!(success, "mission start failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prime"]);
    assert!(success, "prime failed: {stderr}");
    assert!(stdout.contains(&format!("Active mission: {mission_id} - Prime mission")));
    assert!(stdout.contains(&format!(
        "`atelier mission status {mission_id}` - Drill into the active mission named above."
    )));
    assert!(stdout.contains("`atelier history --mission <id>`"));
}

#[test]
fn test_prime_names_active_work() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Prime work"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Prime work");
    let issue_id = issue_id.as_str();
    commit_all(dir.path(), "prime work setup");
    let (success, _, stderr) = run_atelier(dir.path(), &["start", issue_id]);
    assert!(success, "start failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prime"]);
    assert!(success, "prime failed: {stderr}");
    assert!(stdout.contains(&format!("Active work: {issue_id} - Prime work")));
    assert!(stdout.contains(&format!(
        "`atelier issue show {issue_id}` - Reopen the active work contract named above."
    )));
    assert!(stdout.contains("`atelier finish [issue-id]` - Finish active local work"));
}

#[test]
fn test_issue_next_uses_current_workflow_commands() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(dir.path(), &["issue", "create", "Next item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Next item");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "next"]);
    assert!(success, "issue next failed: {stderr}");
    assert!(stdout.contains("Next Actions"));
    assert!(stdout.contains(&format!("atelier issue show {issue_id}")));
    assert!(stdout.contains(&format!("atelier start {issue_id}")));
    assert!(stdout.contains("atelier status"));
    assert!(
        !stdout.contains("session work"),
        "issue next must not suggest removed session workflow:\n{stdout}"
    );
}

#[test]
fn test_root_start_finish_and_issue_transition_surface() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Root workflow item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("atelier start"));
    let issue_id = issue_id_by_title(dir.path(), "Root workflow item");
    commit_all(dir.path(), "tracker setup");

    let (success, transition_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(success, "transition failed: {stderr}");
    assert!(transition_out.contains("Issue Transitions"));
    assert!(transition_out.contains("Allowed Actions"));
    assert!(transition_out.contains("start"));
    assert!(transition_out.contains(&format!("atelier start {issue_id}")));
    assert!(transition_out.contains("Blocked Actions"));
    assert!(transition_out.contains("missing issue-level proof"));
    assert!(transition_out.contains("capture passing evidence or attach existing evidence"));

    let (success, start_out, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "root start failed: {stderr}");
    assert!(start_out.contains(&format!("Started work on {issue_id}")));

    let (success, active_transition, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(success, "active transition failed: {stderr}");
    assert!(active_transition.contains("Work:     active on this issue"));
    assert!(active_transition.contains(&format!("atelier finish {issue_id}")));

    let (success, finish_out, stderr) = run_atelier(dir.path(), &["finish"]);
    assert!(success, "root finish failed: {stderr}");
    assert!(finish_out.contains(&format!("Finished work on {issue_id}")));
}

#[test]
fn test_issue_help_uses_reduced_lifecycle_surface() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "--help"]);
    assert!(success, "issue help failed: {stderr}");
    for command in ["create", "list", "show", "transition", "update", "close"] {
        assert!(
            stdout
                .lines()
                .any(|line| line.trim_start().starts_with(command)),
            "missing reduced issue command {command}:\n{stdout}"
        );
    }
    for hidden in [
        "quick",
        "subissue",
        "reopen",
        "label",
        "unlabel",
        "blocked",
        "block",
        "unblock",
        "close-all",
        "delete",
        "next",
        "tested",
    ] {
        assert!(
            !stdout
                .lines()
                .any(|line| line.trim_start().starts_with(hidden)),
            "folded or moved command {hidden} is still visible:\n{stdout}"
        );
    }
}

#[test]
fn test_non_lifecycle_issue_flows_use_explicit_homes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Source graph item"]);
    run_atelier(dir.path(), &["issue", "create", "Target graph item"]);
    run_atelier(dir.path(), &["issue", "create", "Disposable item"]);
    let source_id = issue_ref(dir.path(), 1);
    let target_id = issue_ref(dir.path(), 2);
    let disposable_id = issue_ref(dir.path(), 3);

    let (success, search_out, stderr) = run_atelier(dir.path(), &["search", "Source"]);
    assert!(success, "search failed: {stderr}");
    assert!(search_out.contains("Source graph item"));

    let (success, link_out, stderr) = run_atelier(
        dir.path(),
        &[
            "link", "add", "issue", &source_id, "issue", &target_id, "--type", "derived",
        ],
    );
    assert!(success, "link add failed: {stderr}");
    assert!(link_out.contains("Linked"));

    let (success, list_out, stderr) =
        run_atelier(dir.path(), &["link", "list", "issue", &source_id]);
    assert!(success, "link list failed: {stderr}");
    assert!(list_out.contains("derived"));
    assert!(list_out.contains("Target graph item"));

    let (success, impact_out, stderr) = run_atelier(dir.path(), &["graph", "impact", &source_id]);
    assert!(success, "graph impact failed: {stderr}");
    assert!(impact_out.contains("downstream impact"));
    assert!(impact_out.contains("Target graph item"));

    let (success, tree_out, stderr) = run_atelier(dir.path(), &["graph", "tree", "--compact"]);
    assert!(success, "graph tree failed: {stderr}");
    assert!(tree_out.contains("Compact Issue Hierarchy"));

    let (success, note_out, stderr) = run_atelier(
        dir.path(),
        &[
            "note",
            "add",
            "issue",
            &source_id,
            "Explicit note body",
            "--kind",
            "observation",
        ],
    );
    assert!(success, "note add failed: {stderr}");
    assert!(note_out.contains("Added comment"));
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &source_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(show_out.contains("Explicit note body"));

    let (success, unlink_out, stderr) = run_atelier(
        dir.path(),
        &[
            "link", "remove", "issue", &source_id, "issue", &target_id, "--type", "derived",
        ],
    );
    assert!(success, "link remove failed: {stderr}");
    assert!(unlink_out.contains("Unlinked"));

    let (success, delete_out, stderr) = run_atelier(
        dir.path(),
        &["maintenance", "delete", "issue", &disposable_id, "--force"],
    );
    assert!(success, "maintenance delete failed: {stderr}");
    assert!(delete_out.contains("Deleted issue"));
}

#[test]
fn test_hidden_issue_helpers_print_replacement_guidance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Old surface source"]);
    run_atelier(dir.path(), &["issue", "create", "Old surface target"]);

    for (args, replacement) in [
        (vec!["issue", "search", "Old"], "atelier search <query>"),
        (
            vec!["issue", "comment", "1", "compat note"],
            "atelier note add issue <id>",
        ),
        (
            vec!["issue", "block", "1", "2"],
            "atelier dep add <blocked> <blocker>",
        ),
        (
            vec!["issue", "relate", "1", "2"],
            "atelier link add issue <id> issue <related>",
        ),
        (vec!["issue", "impact", "1"], "atelier graph impact <id>"),
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "{args:?} failed: {stderr}");
        assert!(
            stderr.contains("Compatibility") && stderr.contains(replacement),
            "{args:?} did not print replacement guidance:\n{stderr}"
        );
    }
}

#[test]
fn test_explicit_homes_reject_non_issue_targets_until_supported() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Target issue"]);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["link", "add", "mission", "atelier-none", "issue", "1"],
    );
    assert!(!success, "link add unexpectedly accepted a mission target");
    assert!(stderr.contains("supports issue records only"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "maintenance",
            "delete",
            "mission",
            "atelier-none",
            "--force",
        ],
    );
    assert!(
        !success,
        "maintenance delete unexpectedly accepted a mission target"
    );
    assert!(stderr.contains("supports issue records only"));
}

#[test]
fn test_removed_aliases_fail_as_unknown_commands() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for args in [
        vec!["show"],
        vec!["ready"],
        vec!["sync"],
        vec!["mission", "view"],
        vec!["work", "worktree"],
    ] {
        let (success, _, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(!success, "{args:?} unexpectedly succeeded");
        assert!(
            stderr.contains("unrecognized subcommand") || stderr.contains("unexpected argument"),
            "{args:?} did not fail as an unknown command:\n{stderr}"
        );
    }
}

// ==================== Issue Creation Tests ====================

#[test]
fn test_create_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "create", "Test issue"]);

    assert!(success);
    assert!(
        stdout.contains("Created issue atelier-"),
        "Expected project-scoped issue id in output, got: {}",
        stdout
    );
    let issue_id = issue_id_by_title(dir.path(), "Test issue");
    assert!(stdout.contains(&format!(".atelier/issues/{issue_id}.md")));
    assert!(stdout.contains(&format!("atelier lint {issue_id}")));
}

#[test]
fn test_create_issue_with_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "create", "High priority issue", "-p", "high"],
    );

    assert!(success);

    // Verify it was created with correct priority
    let (_, list_out, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(list_out.contains("high"));
}

#[test]
fn test_create_issue_with_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Issue with desc",
            "-d",
            "Detailed description here",
        ],
    );

    assert!(success);

    // Verify description in show
    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("Detailed description"));
}

#[test]
fn test_create_subissue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent issue"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "subissue", "1", "Child issue"]);

    assert!(success);
    assert!(
        stdout.contains("Created subissue atelier-"),
        "Expected project-scoped subissue id in output, got: {}",
        stdout
    );
    let child_id = issue_id_by_title(dir.path(), "Child issue");
    assert!(stdout.contains(&format!(".atelier/issues/{child_id}.md")));
    assert!(stdout.contains(&format!("atelier lint {child_id}")));

    // Verify parent-child relationship in show
    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("Child") || show_out.contains("subissue"));
}

#[test]
fn test_create_issue_with_work_prints_canonical_path() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Work path issue", "--work"],
    );
    assert!(success, "issue create --work failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Work path issue");
    assert!(stdout.contains(&format!(".atelier/issues/{issue_id}.md")));
    assert!(stdout.contains(&format!("atelier lint {issue_id}")));
    assert!(stdout.contains(&format!("atelier issue show {issue_id}")));
}

// ==================== Issue Listing Tests ====================

#[test]
fn test_list_empty() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);

    assert!(success);
    assert!(
        stdout.contains("No issues found."),
        "Expected 'No issues found.' in output, got: {}",
        stdout
    );
}

#[test]
fn test_list_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue 1"]);
    run_atelier(
        dir.path(),
        &["issue", "subissue", "1", "Issue 2", "-p", "high"],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);

    assert!(success);
    assert!(stdout.contains("Issue Queue"));
    assert!(stdout.contains("2 total | Status: open=2"));
    assert!(stdout.contains("atelier-"));
    assert!(stdout.contains("[task] atelier-"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));

    let (success, quiet_out, stderr) = run_atelier(dir.path(), &["--quiet", "issue", "list"]);
    assert!(success, "quiet issue list failed: {stderr}");
    assert!(!quiet_out.contains("Issue Queue"));
    assert!(!quiet_out.contains("Issue 1"));
    assert_eq!(quiet_out.lines().count(), 2);
    assert!(quiet_out.lines().all(|line| line.starts_with("atelier-")));
}

#[test]
fn test_list_filter_by_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Open issue"]);
    run_atelier(dir.path(), &["issue", "create", "Closed issue"]);
    run_atelier(dir.path(), &["issue", "close", "2"]);

    let (_, open_list, _) = run_atelier(dir.path(), &["issue", "list", "-s", "open"]);
    assert!(open_list.contains("Open issue"));
    assert!(!open_list.contains("Closed issue"));

    let (_, closed_list, _) = run_atelier(dir.path(), &["issue", "list", "-s", "closed"]);
    assert!(closed_list.contains("Closed issue"));
    assert!(!closed_list.contains("Open issue"));
}

#[test]
fn test_list_filter_by_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Bug issue"]);
    run_atelier(dir.path(), &["issue", "create", "Feature issue"]);
    run_atelier(dir.path(), &["issue", "label", "1", "bug"]);
    run_atelier(dir.path(), &["issue", "label", "2", "feature"]);

    let (_, bug_list, _) = run_atelier(dir.path(), &["issue", "list", "-l", "bug"]);
    assert!(bug_list.contains("Bug issue"));
    assert!(!bug_list.contains("Feature issue"));
}

// ==================== Issue Show Tests ====================

#[test]
fn test_show_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let body = "## Description\n\nDescription\n\n## Outcome\n\nThe issue show command renders parsed sections.\n\n## Evidence\n\n- Show output contains the section headings.\n\n## Notes\n\nCLI display context.";

    run_atelier(dir.path(), &["issue", "create", "Test issue", "-d", body]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success);
    assert!(stdout.contains("Test issue"));
    assert!(stdout.contains("Description"));
    assert!(stdout.contains("Outcome"));
    assert!(stdout.contains("The issue show command renders parsed sections."));
    assert!(stdout.contains("Evidence"));
    assert!(stdout.contains("- Show output contains the section headings."));
    assert!(stdout.contains("Notes"));
    assert!(stdout.contains("CLI display context."));
    assert!(!stdout.contains("Acceptance Criteria"));
}

#[test]
fn test_issue_commands_accept_partial_issue_key() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Partial key issue"]);
    let issue_id = issue_id_by_title(dir.path(), "Partial key issue");
    let key = issue_key(&issue_id);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", key]);

    assert!(success, "show by partial key failed: {stderr}");
    assert!(stdout.contains(&issue_id));
    assert!(stdout.contains("Partial key issue"));
}

#[test]
fn test_issue_reference_surfaces_accept_partial_issue_keys() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent key issue"]);
    run_atelier(dir.path(), &["issue", "create", "Related key issue"]);
    let parent_id = issue_id_by_title(dir.path(), "Parent key issue");
    let related_id = issue_id_by_title(dir.path(), "Related key issue");
    let parent_key = issue_key(&parent_id);
    let related_key = issue_key(&related_id);

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "subissue", parent_key, "Child key issue"],
    );
    assert!(success, "subissue by partial key failed: {stderr}");
    assert!(stdout.contains(&parent_id));
    let child_id = issue_id_by_title(dir.path(), "Child key issue");
    assert!(!child_id.is_empty());

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "relate", parent_key, related_key]);
    assert!(success, "relate by partial keys failed: {stderr}");
    assert!(stdout.contains(&parent_id));
    assert!(stdout.contains(&related_id));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "related", parent_key]);
    assert!(success, "related by partial key failed: {stderr}");
    assert!(stdout.contains(&related_id));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "issue",
            parent_key,
            "--validator",
            "no_open_blockers",
        ],
    );
    assert!(success, "workflow validate by partial key failed: {stderr}");
    assert!(stdout.contains(&format!("Workflow Validation: issue {parent_id}")));
}

#[test]
fn test_bulk_plan_apply_accepts_partial_issue_key_refs() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Existing issue target"]);
    let issue_id = issue_id_by_title(dir.path(), "Existing issue target");
    let issue_key = issue_key(&issue_id);
    let bulk_path = dir.path().join("partial-key-plan.json");
    std::fs::write(
        &bulk_path,
        format!(
            r#"{{
  "schema": "atelier.bulk-plan",
  "schema_version": 1,
  "title": "Partial key bulk apply",
  "apply": {{ "export": "auto" }},
  "records": {{
    "issues": [
      {{
        "client_ref": "issue.partial",
        "title": "Partial key dependent",
        "issue_type": "task",
        "priority": "medium",
        "depends_on": [{{ "id": "{issue_key}" }}]
      }}
    ]
  }}
}}"#
        ),
    )
    .unwrap();

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["plan", "apply", bulk_path.to_str().unwrap()]);
    assert!(success, "bulk apply by partial issue key failed: {stderr}");
    assert!(stdout.contains("Bulk plan applied."));

    let dependent_id = issue_id_by_title(dir.path(), "Partial key dependent");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["dep", "list", &dependent_id]);
    assert!(success, "dep list failed: {stderr}");
    assert!(stdout.contains(&issue_id));
}

#[test]
fn test_show_issue_rich_human_output() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent issue", "-p", "high"],
    );
    run_atelier(
        dir.path(),
        &["issue", "subissue", "1", "Target issue", "-p", "medium"],
    );
    run_atelier(
        dir.path(),
        &["issue", "subissue", "2", "Child issue", "-p", "low"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Blocking issue", "-p", "high"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Downstream issue", "-p", "low"],
    );
    run_atelier(dir.path(), &["issue", "block", "2", "4"]);
    run_atelier(dir.path(), &["issue", "block", "5", "2"]);
    run_atelier(dir.path(), &["note", "add", "issue", "2", "Recent note"]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "2"]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Target issue"));
    assert!(stdout.contains("Status:   open"));
    assert!(stdout.contains("Type:     task"));
    assert!(stdout.contains("Priority: medium"));
    let target_id = issue_id_by_title(dir.path(), "Target issue");
    assert!(stdout.contains(&format!(".atelier/issues/{target_id}.md")));
    assert!(stdout.contains("Parent issue"));
    assert!(stdout.contains("1 total | status: open=1 | priority: low=1"));
    assert!(stdout.contains("Blocking issue"));
    assert!(stdout.contains("(open blocker)"));
    assert!(stdout.contains("Downstream issue"));
    assert!(stdout.contains("Recent Activity"));
    assert!(stdout.contains("Recent note"));
    assert!(stdout.contains("Next Commands"));
    assert!(stdout.contains("atelier note add issue"));
    assert!(!stdout.contains("atelier issue comment"));
    assert!(stdout.contains("atelier issue transition"));
    assert!(stdout.contains("atelier start"));
}

#[test]
fn test_issue_show_human_shape_exposes_actionable_context() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "JSON issue", "-d", "JSON description"],
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("[task] open - JSON issue"));
    assert!(stdout.contains("Description"));
    assert!(stdout.contains("JSON description"));
    assert!(stdout.contains("Blocked by"));
    assert!(stdout.contains("Blocking"));
    assert!(stdout.contains("Recent Activity"));
    assert!(stdout.contains("Next Commands"));
}

#[test]
fn test_issue_show_reads_detail_body_from_record_store() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Canonical detail issue",
            "-d",
            "Canonical Markdown body",
        ],
    );
    let issue_id = issue_id_by_title(dir.path(), "Canonical detail issue");
    let conn = rusqlite::Connection::open(dir.path().join(".atelier/state.db")).unwrap();
    conn.execute(
        "UPDATE issues SET description = 'SQLite shadow body' WHERE id = ?1",
        [&issue_id],
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Canonical Markdown body"));
    assert!(!stdout.contains("SQLite shadow body"));
}

#[test]
fn test_first_class_detail_views_read_payloads_from_record_store() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "create",
            "Canonical mission",
            "--body",
            "Canonical mission body",
            "--constraint",
            "Canonical constraint",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Canonical mission");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "plan",
            "create",
            "Canonical plan",
            "--body",
            "Canonical plan body",
        ],
    );
    assert!(success, "plan create failed: {stderr}");
    let plan_id = record_id_by_title(dir.path(), "plans", "Canonical plan");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "add",
            "--kind",
            "test",
            "--result",
            "pass",
            "Canonical evidence summary",
        ],
    );
    assert!(success, "evidence add failed: {stderr}");
    let evidence_id = record_id_by_title(dir.path(), "evidence", "Canonical evidence summary");

    let conn = rusqlite::Connection::open(dir.path().join(".atelier/state.db")).unwrap();
    conn.execute(
        "UPDATE records SET body = 'SQLite mission body', data_json = ?1 WHERE id = ?2",
        [
            r#"{"constraints":["SQLite constraint"],"risks":[],"validation":[],"milestones":[],"plans":[],"evidence":[],"work":[]}"#,
            mission_id.as_str(),
        ],
    )
    .unwrap();
    conn.execute(
        "UPDATE records SET body = 'SQLite plan body', data_json = ?1 WHERE id = ?2",
        [
            r#"{"revision":99,"revisions":[{"revision":99,"reason":"sqlite","body":"SQLite plan body"}]}"#,
            plan_id.as_str(),
        ],
    )
    .unwrap();
    conn.execute(
        "UPDATE records SET body = 'SQLite evidence summary', data_json = ?1 WHERE id = ?2",
        [
            r#"{"kind":"sqlite","result":"fail","path":null,"uri":null,"producer":null,"captured_at":"2000-01-01T00:00:00Z"}"#,
            evidence_id.as_str(),
        ],
    )
    .unwrap();

    let (success, mission_out, stderr) = run_atelier(dir.path(), &["mission", "show", &mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(mission_out.contains("Canonical mission body"));
    assert!(mission_out.contains("Canonical constraint"));
    assert!(!mission_out.contains("SQLite mission body"));
    assert!(!mission_out.contains("SQLite constraint"));

    let (success, plan_out, stderr) = run_atelier(dir.path(), &["plan", "show", &plan_id]);
    assert!(success, "plan show failed: {stderr}");
    assert!(plan_out.contains("Canonical plan body"));
    assert!(plan_out.contains("Revision: 1"));
    assert!(!plan_out.contains("SQLite plan body"));
    assert!(!plan_out.contains("Revision: 99"));

    let (success, evidence_out, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &evidence_id]);
    assert!(success, "evidence show failed: {stderr}");
    assert!(evidence_out.contains("Canonical evidence summary"));
    assert!(evidence_out.contains("Result:      pass"));
    assert!(evidence_out.contains("Kind:        test"));
    assert!(!evidence_out.contains("SQLite evidence summary"));
    assert!(!evidence_out.contains("Kind:        sqlite"));
}

#[test]
fn test_issue_search_reads_payloads_from_record_store_and_activity() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Canonical search issue",
            "-d",
            "canonical body needle",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Canonical search issue");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "comment", &issue_id, "canonical activity needle"],
    );
    assert!(success, "issue comment failed: {stderr}");

    let conn = rusqlite::Connection::open(dir.path().join(".atelier/state.db")).unwrap();
    conn.execute(
        "UPDATE issues SET description = 'sqlite body needle' WHERE id = ?1",
        [&issue_id],
    )
    .unwrap();
    conn.execute(
        "UPDATE comments SET content = 'sqlite comment needle' WHERE issue_id = ?1",
        [&issue_id],
    )
    .unwrap();

    let (success, body_out, stderr) =
        run_atelier(dir.path(), &["issue", "search", "canonical body needle"]);
    assert!(success, "canonical body search failed: {stderr}");
    assert!(body_out.contains("Canonical search issue"));

    let (success, activity_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "search", "canonical activity needle"],
    );
    assert!(success, "canonical activity search failed: {stderr}");
    assert!(activity_out.contains("Canonical search issue"));

    let (success, shadow_body_out, stderr) =
        run_atelier(dir.path(), &["issue", "search", "sqlite body needle"]);
    assert!(success, "sqlite shadow body search failed: {stderr}");
    assert!(shadow_body_out.contains("No issues found"));

    let (success, shadow_comment_out, stderr) =
        run_atelier(dir.path(), &["issue", "search", "sqlite comment needle"]);
    assert!(success, "sqlite shadow comment search failed: {stderr}");
    assert!(shadow_comment_out.contains("No issues found"));
}

#[test]
fn test_show_closed_issue_includes_close_reason() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Closed issue"]);
    let issue_id = issue_ref(dir.path(), 1);
    attach_issue_pass_evidence(dir.path(), &issue_id);
    run_atelier(
        dir.path(),
        &["issue", "close", "1", "--reason", "Done enough"],
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success, "issue show failed: {stderr}");
    assert!(stdout.contains("Closed issue"));
    assert!(stdout.contains("Closed:"));
    assert!(stdout.contains("Close Reason"));
    assert!(stdout.contains("Done enough"));
    assert!(stdout.contains("atelier issue update"));
    assert!(stdout.contains("--status open"));
}

#[test]
fn test_show_issue_prefers_activity_sidecars_for_recent_activity() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Activity issue"]);
    let issue_id = issue_id_by_title(dir.path(), "Activity issue");
    let conn = rusqlite::Connection::open(dir.path().join(".atelier/state.db")).unwrap();
    conn.execute(
        "INSERT INTO comments (issue_id, content, created_at, kind) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![
            &issue_id,
            "Legacy note",
            "2026-06-10T18:18:20.123456Z",
            "note"
        ],
    )
    .unwrap();
    let activity_dir = dir
        .path()
        .join(".atelier")
        .join("issues")
        .join(format!("{issue_id}.activity"));
    std::fs::create_dir_all(&activity_dir).unwrap();
    std::fs::write(
        activity_dir.join("20260610T181920123456Z.md"),
        format!(
            "---\nschema: \"atelier.activity\"\nschema_version: 1\nid: \"20260610T181920123456Z\"\nsubject_kind: \"issue\"\nsubject_id: \"{issue_id}\"\nevent_type: \"comment\"\nactor: \"tester\"\ncreated_at: \"2026-06-10T18:19:20.123456Z\"\nsummary: \"Canonical activity\"\n---\n\nSidecar body\n"
        ),
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Canonical activity"));
    assert!(stdout.contains("Sidecar body"));
    assert!(!stdout.contains("Legacy note"));
}

#[test]
fn test_history_repo_wide_supports_filters_bounded_output_and_drill_downs() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "First issue"]);
    run_atelier(dir.path(), &["issue", "create", "Second issue"]);
    let first = issue_id_by_title(dir.path(), "First issue");
    let second = issue_id_by_title(dir.path(), "Second issue");
    write_activity_fixture(
        dir.path(),
        &first,
        "20260610T181920123456Z",
        "comment",
        "First comment",
        "First body",
    );
    write_activity_fixture(
        dir.path(),
        &second,
        "20260610T181921123456Z",
        "evidence_attached",
        "Evidence attached",
        "evidence_id: \"ev-1\"\nresult: \"pass\"",
    );
    write_activity_fixture(
        dir.path(),
        &second,
        "20260610T181922123456Z",
        "comment",
        "Second comment",
        "Second body",
    );

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--event-kind",
            "evidence_attached",
            "--limit",
            "1",
        ],
    );
    assert!(success, "history failed: {stderr}");
    assert!(stdout.contains("History"));
    assert!(stdout.contains("Scope:          repository"));
    assert!(stdout.contains("Source:         canonical .atelier"));
    assert!(stdout.contains("Ordering:       newest first"));
    assert!(stdout.contains("Showing:        1 of 1 matching events"));
    assert!(stdout.contains("Evidence attached"));
    assert!(!stdout.contains("First comment"));
    assert!(stdout.contains("Next Commands"));
    assert!(stdout.contains("atelier issue show <id>"));
    assert!(stdout.contains("atelier history --mission <id>"));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--issue",
            first.as_str(),
            "--event-kind",
            "comment",
            "--since",
            "2026-06-10",
        ],
    );
    assert!(success, "filtered history failed: {stderr}");
    assert!(stdout.contains("First comment"));
    assert!(!stdout.contains("Evidence attached"));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["history", "--event-kind", "comment", "--limit", "1"],
    );
    assert!(success, "history failed: {stderr}");
    assert!(stdout.contains("Second comment"));
    assert!(!stdout.contains("First comment"));
    assert!(stdout.contains("Omitted:"));
}

#[test]
fn test_history_mission_scope_includes_linked_work_descendants_and_evidence() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "History mission"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "History mission");

    run_atelier(
        dir.path(),
        &["issue", "create", "History epic", "--issue-type", "epic"],
    );
    let epic_id = issue_id_by_title(dir.path(), "History epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "subissue", &epic_id, "History child"],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "History child");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", &mission_id, &epic_id]);
    assert!(success, "mission add-work failed: {stderr}");
    write_activity_fixture(
        dir.path(),
        &child_id,
        "20260610T191920123456Z",
        "note",
        "Child note",
        "Child body",
    );
    let (success, _evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "add",
            "--kind",
            "test",
            "--result",
            "pass",
            "Cargo test passed",
        ],
    );
    assert!(success, "evidence add failed: {stderr}");
    let evidence_id = record_id_by_title(dir.path(), "evidence", "Cargo test passed");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "attach",
            &evidence_id,
            "issue",
            child_id.as_str(),
        ],
    );
    assert!(success, "evidence attach failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--mission",
            mission_id.as_str(),
            "--event-kind",
            "evidence_attached",
        ],
    );

    assert!(success, "history failed: {stderr}");
    assert!(stdout.contains(&format!("Scope:          mission {mission_id}")));
    assert!(stdout.contains(&format!("Attached evidence {evidence_id}")));
    assert!(stdout.contains(&child_id));
    assert!(stdout.contains(&format!("atelier mission show {mission_id}")));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--mission",
            mission_id.as_str(),
            "--event-kind",
            "note",
        ],
    );
    assert!(success, "mission note history failed: {stderr}");
    assert!(stdout.contains("Child note"));
    assert!(stdout.contains(&child_id));
}

#[test]
fn test_history_issue_scope_defaults_single_issue_and_can_include_descendants() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent history"]);
    let parent_id = issue_id_by_title(dir.path(), "Parent history");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "subissue", &parent_id, "Child history"],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Child history");
    write_activity_fixture(
        dir.path(),
        &parent_id,
        "20260610T181920123456Z",
        "note",
        "Parent note",
        "Parent body",
    );
    write_activity_fixture(
        dir.path(),
        &child_id,
        "20260610T181921123456Z",
        "note",
        "Child note",
        "Child body",
    );

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--issue",
            parent_id.as_str(),
            "--event-kind",
            "note",
        ],
    );
    assert!(success, "issue history failed: {stderr}");
    assert!(stdout.contains(&format!("Scope:          issue {parent_id}")));
    assert!(stdout.contains("Parent note"));
    assert!(!stdout.contains("Child note"));
    assert!(stdout.contains(&format!(
        "atelier history --issue {parent_id} --include-descendants"
    )));

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--issue",
            parent_id.as_str(),
            "--include-descendants",
            "--event-kind",
            "note",
        ],
    );
    assert!(success, "descendant issue history failed: {stderr}");
    assert!(stdout.contains("Parent note"));
    assert!(stdout.contains("Child note"));
}

#[test]
fn test_history_empty_states_and_invalid_limit() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["history"]);
    assert!(success, "empty history failed: {stderr}");
    assert!(stdout.contains("No canonical history found for repository."));
    assert!(stdout.contains("Source:"));
    assert!(stdout.contains("Next Commands"));

    run_atelier(dir.path(), &["issue", "create", "Filtered history"]);
    let issue_id = issue_id_by_title(dir.path(), "Filtered history");
    write_activity_fixture(
        dir.path(),
        &issue_id,
        "20260610T181920123456Z",
        "note",
        "Filter note",
        "Filter body",
    );
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "history",
            "--issue",
            issue_id.as_str(),
            "--event-kind",
            "evidence_attached",
        ],
    );
    assert!(success, "filtered empty history failed: {stderr}");
    assert!(stdout.contains("History exists for"));
    assert!(stdout.contains("no events matched the current filters"));

    let (success, _, stderr) = run_atelier(dir.path(), &["history", "--limit", "0"]);
    assert!(!success, "zero limit should fail");
    assert!(stderr.contains("--limit must be greater than 0"));
}

#[test]
fn test_show_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", "999"]);

    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

fn write_activity_fixture(
    dir: &Path,
    issue_id: &str,
    activity_id: &str,
    event_type: &str,
    summary: &str,
    body: &str,
) {
    let activity_dir = dir
        .join(".atelier")
        .join("issues")
        .join(format!("{issue_id}.activity"));
    std::fs::create_dir_all(&activity_dir).unwrap();
    std::fs::write(
        activity_dir.join(format!("{activity_id}.md")),
        format!(
            "---\nschema: \"atelier.activity\"\nschema_version: 1\nid: \"{activity_id}\"\nsubject_kind: \"issue\"\nsubject_id: \"{issue_id}\"\nevent_type: \"{event_type}\"\nactor: \"tester\"\ncreated_at: \"{}-{}-{}T{}:{}:{}.123456Z\"\nsummary: \"{summary}\"\n---\n\n{body}\n",
            &activity_id[0..4],
            &activity_id[4..6],
            &activity_id[6..8],
            &activity_id[9..11],
            &activity_id[11..13],
            &activity_id[13..15],
        ),
    )
    .unwrap();
}

// ==================== Issue Update Tests ====================

#[test]
fn test_update_issue_title() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Original title"]);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "update", "1", "--title", "Updated title"],
    );

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("Updated title"));
}

#[test]
fn test_update_issue_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue", "-p", "low"]);
    run_atelier(dir.path(), &["issue", "update", "1", "-p", "critical"]);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("critical"));
}

#[test]
fn test_update_issue_remove_label_replaces_unlabel_helper() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Label lifecycle", "--label", "keep-me"],
    );
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            "1",
            "--label",
            "remove-me",
            "--remove-label",
            "keep-me",
        ],
    );
    assert!(success, "update label replacement failed: {stderr}");
    assert!(stdout.contains("Updated issue"));

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    let labels_line = show_out
        .lines()
        .find(|line| line.starts_with("Labels:"))
        .unwrap_or("");
    assert!(labels_line.contains("remove-me"), "{show_out}");
    assert!(!labels_line.contains("keep-me"), "{show_out}");
}

// ==================== Issue Close/Reopen Tests ====================

#[test]
fn test_close_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    let issue_id = issue_ref(dir.path(), 1);
    attach_issue_pass_evidence(dir.path(), &issue_id);
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "close", "1"]);

    assert!(success);
    assert!(stdout.contains("Closed") || stdout.contains("closed"));

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("closed"));
}

#[test]
fn test_close_all_is_durable_without_manual_export() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Close all one"]);
    assert!(success, "first create failed: {stderr}");
    let first_id = issue_ref(dir.path(), 1);
    attach_issue_pass_evidence(dir.path(), &first_id);
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Close all two"]);
    assert!(success, "second create failed: {stderr}");
    let second_id = issue_ref(dir.path(), 2);
    attach_issue_pass_evidence(dir.path(), &second_id);

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "close-all"]);
    assert!(success, "close-all failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed after close-all: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    for issue_id in [first_id, second_id] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
        assert!(success, "show failed for {issue_id}: {stderr}");
        assert!(stdout.contains("Status:   closed"), "{stdout}");
    }
}

#[test]
fn test_reopen_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    close_issue_with_evidence(dir.path(), "1", None);
    let (success, stdout, _) =
        run_atelier(dir.path(), &["issue", "update", "1", "--status", "open"]);

    assert!(success);
    assert!(stdout.contains("Updated issue"));

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("open"));
}

#[test]
fn test_import_beads_jsonl_fixture_round_trip() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let fixture_path = dir.path().join("issues.manual.jsonl");
    std::fs::write(
        &fixture_path,
        include_str!("fixtures/beads/issues.manual.jsonl"),
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["import-beads", fixture_path.to_str().unwrap()],
    );
    assert!(success, "import-beads failed: {stderr}");
    assert!(stdout.contains("source records: 3"));
    assert!(stdout.contains("imported issues: 3"));
    assert!(stdout.contains("parent-child relationships: 2"));
    assert!(stdout.contains("blocking relationships: 1"));
    assert!(dir
        .path()
        .join(".atelier")
        .join("issues")
        .join("atelier-0001.md")
        .exists());

    let (_, list_out, _) = run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(list_out.contains("Mission: Replace Beads"));
    assert!(list_out.contains("Dogfood Atelier"));

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "3"]);
    assert!(show_out.contains("Parent: atelier-0001"));
    assert!(show_out.contains("Blocked by"));
    assert!(show_out.contains("atelier-0002 [open]"));
    assert!(show_out.contains("(open blocker)"));
    assert!(show_out.contains("Acceptance Criteria"));

    let (updated, _, update_err) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            "2",
            "--title",
            "Imported Beads issue updated",
        ],
    );
    assert!(updated, "update failed: {update_err}");
    let (closed, _, close_err) = run_atelier(dir.path(), &["issue", "close", "2"]);
    assert!(closed, "close failed: {close_err}");

    let (_, closed_show, _) = run_atelier(dir.path(), &["issue", "show", "2"]);
    assert!(closed_show.contains("Imported Beads issue updated"));
    assert!(closed_show.contains("Status:   closed"));

    let (fresh, _, fresh_err) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(
        fresh,
        "export --check validates canonical Markdown/projection state, not SQLite-only drift: {fresh_err}"
    );
}

// ==================== Issue Delete Tests ====================

#[test]
fn test_delete_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "To delete"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "delete", "1", "-f"]);

    assert!(success);

    let (_, list_out, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(!list_out.contains("To delete"));
}

#[test]
fn test_delete_issue_is_durable_without_manual_export() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Durable delete"]);
    assert!(success, "create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "delete", &issue_id, "-f"]);
    assert!(success, "delete failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed after delete: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(!success, "deleted issue still exists after rebuild");
    assert!(stderr.contains("was not found"), "{stderr}");
}

// ==================== Labels Tests ====================

#[test]
fn test_add_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "label", "1", "bug"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("bug"));
}

#[test]
fn test_remove_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    run_atelier(dir.path(), &["issue", "label", "1", "bug"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "unlabel", "1", "bug"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(!show_out.contains("bug") || show_out.contains("Labels: none"));
}

// ==================== Comments Tests ====================

#[test]
fn test_add_comment() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "comment", "1", "This is a comment"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("This is a comment"));
}

#[test]
fn test_issue_mutations_create_activity_sidecars() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Activity issue"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Activity issue");
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    for (kind, body) in [
        ("human", "Plain comment body"),
        ("note", "Operator note body"),
        ("plan", "Plan body"),
        ("handoff", "Handoff body"),
    ] {
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["issue", "comment", &issue_id, body, "--kind", kind],
        );
        assert!(success, "issue comment {kind} failed: {stderr}");
    }

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "comment",
            &issue_id,
            "Invalid body",
            "--kind",
            "decision",
        ],
    );
    assert!(!success, "invalid comment kind should be rejected");
    assert!(stderr.contains("Invalid comment kind 'decision'"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &issue_id,
            "--append-notes",
            "Append note body",
            "--claim",
        ],
    );
    assert!(success, "issue update notes/claim failed: {stderr}");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &issue_id,
            "--title",
            "Activity issue renamed",
            "--priority",
            "high",
            "--status",
            "in_progress",
            "--label",
            "activity-label",
        ],
    );
    assert!(success, "issue update fields failed: {stderr}");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", &issue_id, "--status", "closed"],
    );
    assert!(success, "issue update status closed failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &issue_id, "--reason", "Close reason body"],
    );
    assert!(success, "issue close reason failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "reopen", &issue_id]);
    assert!(success, "issue reopen failed: {stderr}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(&activities, "comment", &["Plain comment body"]);
    assert_activity_contains(&activities, "note", &["Operator note body"]);
    assert_activity_contains(&activities, "note", &["Append note body"]);
    assert_activity_contains(&activities, "plan", &["Plan body"]);
    assert_activity_contains(&activities, "handoff", &["Handoff body"]);
    assert_activity_contains(
        &activities,
        "field_changed",
        &[
            "field: \"title\"",
            "old: \"Activity issue\"",
            "new: \"Activity issue renamed\"",
        ],
    );
    assert_activity_contains(
        &activities,
        "field_changed",
        &["field: \"priority\"", "old: \"medium\"", "new: \"high\""],
    );
    assert_activity_contains(
        &activities,
        "field_changed",
        &["field: \"labels\"", "new: \"activity-label\""],
    );
    assert_activity_contains(
        &activities,
        "field_changed",
        &["field: \"assignee\"", "new: "],
    );
    assert_activity_contains(
        &activities,
        "status_changed",
        &["old: \"open\"", "new: \"in_progress\""],
    );
    assert_activity_contains(
        &activities,
        "status_changed",
        &["old: \"open\"", "new: \"closed\""],
    );
    assert_activity_contains(
        &activities,
        "status_changed",
        &["old: \"closed\"", "new: \"open\""],
    );
    assert_activity_contains(&activities, "close_reason", &["Close reason body"]);
}

#[test]
fn test_issue_show_json_recovers_activity_fields_after_rebuild() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Rebuild activity"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Rebuild activity");
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "comment",
            &issue_id,
            "Canonical comment",
            "--kind",
            "human",
        ],
    );
    assert!(success, "comment failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &issue_id,
            "--append-notes",
            "Canonical handoff",
            "--claim",
        ],
    );
    assert!(success, "append-notes/claim failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &issue_id, "--reason", "Canonical close"],
    );
    assert!(success, "close failed: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Canonical comment"));
    assert!(stdout.contains("Canonical handoff"));
    assert!(stdout.contains("Close Reason"));
    assert!(stdout.contains("Canonical close"));
    assert!(stdout.contains("Assignee:"));
}

#[test]
fn test_issue_create_is_durable_without_manual_export() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Create-only durable",
            "--description",
            "Created body",
            "--priority",
            "high",
        ],
    );
    assert!(success, "create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Create-only durable");

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed after create: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "show failed after rebuild: {stderr}");
    assert!(stdout.contains("[task] open - Create-only durable"));
    assert!(stdout.contains("Created body"));
    assert!(stdout.contains("Priority: high"));
}

#[test]
fn test_issue_mutations_are_durable_without_manual_export() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Mutation source"]);
    assert!(success, "source create failed: {stderr}");
    let source_id = issue_id_by_title(dir.path(), "Mutation source");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Mutation target"]);
    assert!(success, "target create failed: {stderr}");
    let target_id = issue_id_by_title(dir.path(), "Mutation target");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &source_id,
            "--title",
            "Mutation source updated",
            "--description",
            "Durable body",
            "--priority",
            "high",
        ],
    );
    assert!(success, "update failed: {stderr}");

    for args in [
        vec!["issue", "label", &source_id, "remove-me"],
        vec!["issue", "unlabel", &source_id, "remove-me"],
        vec!["issue", "label", &source_id, "keep-me"],
        vec!["issue", "block", &source_id, &target_id],
        vec!["issue", "unblock", &source_id, &target_id],
        vec![
            "issue", "relate", &source_id, &target_id, "--type", "related",
        ],
        vec![
            "issue", "unrelate", &source_id, &target_id, "--type", "related",
        ],
        vec![
            "issue", "relate", &source_id, &target_id, "--type", "derived",
        ],
        vec!["issue", "close", &source_id, "--reason", "Temporary close"],
        vec!["issue", "reopen", &source_id],
        vec!["issue", "block", &source_id, &target_id],
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "{args:?} failed: {stderr}");
    }

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed before rebuild: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &source_id]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("[task] open - Mutation source updated"));
    assert!(stdout.contains("Durable body"));
    assert!(stdout.contains("Priority: high"));
    assert!(stdout.contains("keep-me"));
    assert!(!stdout.contains("remove-me"));
    assert!(stdout.contains(&target_id));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "related", &source_id]);
    assert!(success, "related failed: {stderr}");
    assert!(stdout.contains("derived"));
    assert!(stdout.contains("Mutation target"));
}

// ==================== Dependencies Tests ====================

#[test]
fn test_block_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "block", "1", "2"]);

    assert!(success);

    let (_, blocked_out, _) = run_atelier(dir.path(), &["issue", "blocked"]);
    assert!(blocked_out.contains("Blocked issue"));
}

#[test]
fn test_issue_list_blocked_replaces_blocked_helper() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    let blocked_id = issue_ref(dir.path(), 1);
    let blocker_id = issue_ref(dir.path(), 2);
    let (success, _, stderr) = run_atelier(dir.path(), &["dep", "add", &blocked_id, &blocker_id]);
    assert!(success, "dep add failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--blocked"]);
    assert!(success, "issue list --blocked failed: {stderr}");
    assert!(stdout.contains("Blocked issue"));
    assert!(stdout.contains(&blocker_id));
}

#[test]
fn test_unblock_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    run_atelier(dir.path(), &["issue", "block", "1", "2"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "unblock", "1", "2"]);

    assert!(success);

    let (_, blocked_out, _) = run_atelier(dir.path(), &["issue", "blocked"]);
    assert!(!blocked_out.contains("Blocked issue"));
}

#[test]
fn test_dep_alias_mutations_are_durable_without_manual_export() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Alias blocked"]);
    assert!(success, "blocked create failed: {stderr}");
    let blocked_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Alias blocker"]);
    assert!(success, "blocker create failed: {stderr}");
    let blocker_id = issue_ref(dir.path(), 2);

    let (success, _, stderr) = run_atelier(dir.path(), &["dep", "add", &blocked_id, &blocker_id]);
    assert!(success, "dep add failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed after dep add: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild after dep add failed: {stderr}");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["dep", "list", &blocked_id]);
    assert!(success, "dep list after dep add failed: {stderr}");
    assert!(stdout.contains(&blocker_id), "{stdout}");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["dep", "remove", &blocked_id, &blocker_id]);
    assert!(success, "dep remove failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed after dep remove: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild after dep remove failed: {stderr}");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["dep", "list", &blocked_id]);
    assert!(success, "dep list after dep remove failed: {stderr}");
    assert!(
        stdout.contains("No dependencies found."),
        "dependency should be removed after rebuild: {stdout}"
    );
}

#[test]
fn test_ready_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked issue"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker issue"]);
    run_atelier(dir.path(), &["issue", "create", "Ready issue"]);
    run_atelier(dir.path(), &["issue", "block", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list", "--ready"]);

    assert!(success);
    assert!(stdout.contains("2 total"));
    assert!(stdout.contains("Ready issue"));
    assert!(stdout.contains("Blocker issue")); // Blocker is also ready
    assert!(!stdout.contains("Blocked issue"));
}

#[test]
fn test_issue_ready_command_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _stdout, stderr) = run_atelier(dir.path(), &["issue", "ready"]);

    assert!(!success);
    assert!(
        stderr.contains("unrecognized subcommand") || stderr.contains("unexpected argument"),
        "expected clap unknown command error, got: {stderr}"
    );
}

#[test]
fn test_quiet_issue_list_ready_outputs_ids_only() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Ready issue"]);

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["--quiet", "issue", "list", "--ready"]);

    assert!(success, "quiet ready list failed: {stderr}");
    assert_eq!(stdout.lines().count(), 1);
    assert!(stdout.lines().all(|line| line.starts_with("atelier-")));
    assert!(!stdout.contains("Ready issue"));
}

#[test]
fn test_issue_list_ready_rejects_closed_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "list", "--ready", "--status", "closed"],
    );

    assert!(!success);
    assert!(stderr.contains("--ready can only be used with open issues"));
}

#[test]
fn test_issue_list_ready_treats_internal_epic_blockers_as_ready() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent epic", "--issue-type", "epic"],
    );
    run_atelier(dir.path(), &["issue", "subissue", "1", "Ready child"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Sequenced child"]);
    run_atelier(dir.path(), &["issue", "block", "3", "2"]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);

    assert!(success, "ready list failed: {stderr}");
    assert!(stdout.contains("Parent epic"));
    assert!(stdout.contains("Ready child"));
    assert!(!stdout.contains("Sequenced child"));
}

#[test]
fn test_issue_list_marks_external_epic_blockers_by_id() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Parent epic", "--issue-type", "epic"],
    );
    run_atelier(dir.path(), &["issue", "subissue", "1", "Blocked child"]);
    run_atelier(dir.path(), &["issue", "create", "Outside blocker"]);
    let blocker_id = issue_ref(dir.path(), 3);
    run_atelier(dir.path(), &["issue", "block", "2", "3"]);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list"]);

    assert!(success, "issue list failed: {stderr}");
    assert!(stdout.contains("Parent epic"));
    assert!(stdout.contains(&format!("blocked by {blocker_id}")));
    assert!(!stdout.contains("open blocker"));
}

#[test]
fn test_issue_update_issue_type_persists_through_rebuild() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Container work"]);

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", "1", "--issue-type", "epic"],
    );
    assert!(success, "issue type update failed: {stderr}");
    assert!(stdout.contains("Type:     epic"));

    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");
    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("[epic] open - Container work"));
}

#[test]
fn test_removed_issue_type_is_rejected() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Artifact task",
            "--issue-type",
            "decision",
        ],
    );

    assert!(!success, "removed issue type should be rejected");
    assert!(stderr.contains("Invalid issue_type 'decision'"));
}

// ==================== Session Tests ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_start() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["session", "start"]);

    assert!(success);
    assert!(stdout.contains("Session") || stdout.contains("started"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["session", "start"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["session", "status"]);

    assert!(success);
    assert!(stdout.contains("Session") || stdout.contains("active"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Working issue"]);
    run_atelier(dir.path(), &["session", "start"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["session", "work", "1"]);

    assert!(success);
    assert!(stdout.contains("Working") || stdout.contains("#1"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_end() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["session", "start"]);
    let (success, stdout, _) =
        run_atelier(dir.path(), &["session", "end", "--notes", "Finished work"]);

    assert!(success);
    assert!(stdout.contains("ended") || stdout.contains("Session"));
}

// ==================== Search Tests ====================

#[test]
fn test_search_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Authentication bug"]);
    run_atelier(dir.path(), &["issue", "create", "Dark mode feature"]);
    run_atelier(dir.path(), &["issue", "create", "Auth improvements"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "search", "auth"]);

    assert!(success);
    assert!(stdout.contains("Search Results: auth"));
    assert!(stdout.contains("Standalone"));
    assert!(stdout.contains("2 total"));
    assert!(stdout.contains("Authentication") || stdout.contains("Auth"));
    assert!(!stdout.contains("Dark mode"));
}

// ==================== Error Handling Tests ====================

#[test]
fn test_command_without_init() {
    let dir = tempdir().unwrap();
    // Don't init

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list"]);

    // The CLI walks up parent directories to find .atelier, so from a temp dir
    // inside the project tree, it may find the project's own .atelier.
    // In that case it succeeds with the project DB. If truly isolated, it fails.
    if !success {
        assert!(
            stderr.contains("Not an Atelier repository") || stderr.contains("atelier init"),
            "Error should mention missing repo, got stderr: {}",
            stderr
        );
    } else {
        // Found a parent .atelier - list should work normally
        assert!(
            stdout.contains("No issues") || stdout.contains("#"),
            "Should show valid list output, got: {}",
            stdout
        );
    }
}

#[test]
fn test_invalid_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Issue", "-p", "invalid"]);

    assert!(!success, "Creating issue with invalid priority should fail");
    assert!(
        stderr.contains("Invalid") || stderr.contains("priority"),
        "Error should mention invalid priority, got stderr: {}",
        stderr
    );
}

// ==================== Security Tests ====================

#[test]
fn test_sql_injection_in_title_cli() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let malicious = "'; DROP TABLE issues; --";
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", malicious]);

    assert!(success);

    // Verify database is intact
    let (success2, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success2);
    assert!(stdout.contains(malicious));
}

#[test]
fn test_special_characters_in_fields() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let special = "Test <>&\"'\\n\\t issue";
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", special]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("Test"));
}

#[test]
fn test_unicode_in_cli() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let unicode = "测试问题 🐛 émoji";
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", unicode]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("测试") || show_out.contains("🐛"));
}

// ==================== Archive Tests ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_archive_closed_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue to archive"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["archive", "add", "1"]);

    assert!(success);
    assert!(stdout.contains("Archived") || stdout.contains("archived"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_archive_open_issue_fails() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Open issue"]);
    let (success, stdout, stderr) = run_atelier(dir.path(), &["archive", "add", "1"]);

    // Should fail or warn - can't archive open issues
    assert!(
        !success
            || stderr.contains("closed")
            || stderr.contains("open")
            || stdout.contains("not closed")
            || stdout.contains("Cannot")
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_archive_list() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue to archive"]);
    run_atelier(dir.path(), &["issue", "create", "Open issue"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);
    run_atelier(dir.path(), &["archive", "add", "1"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["archive", "list"]);

    assert!(success);
    assert!(stdout.contains("Issue to archive") || stdout.contains("#1"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_unarchive_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue to archive"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);
    run_atelier(dir.path(), &["archive", "add", "1"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["archive", "remove", "1"]);

    assert!(success);
    assert!(
        stdout.contains("Unarchived")
            || stdout.contains("restored")
            || stdout.contains("removed")
            || stdout.contains("Restored")
    );

    // Should now be in closed list, not archived
    let (_, closed_list, _) = run_atelier(dir.path(), &["issue", "list", "-s", "closed"]);
    assert!(closed_list.contains("Issue to archive"));
}

// ==================== Milestone Tests ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_create() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(
        dir.path(),
        &["milestone", "create", "v1.0", "-d", "First release"],
    );

    assert!(success);
    assert!(stdout.contains("v1.0") || stdout.contains("#1") || stdout.contains("Created"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_list() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    run_atelier(dir.path(), &["milestone", "create", "v2.0"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["milestone", "list"]);

    assert!(success);
    assert!(stdout.contains("v1.0"));
    assert!(stdout.contains("v2.0"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_show() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["milestone", "create", "v1.0", "-d", "First release"],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["milestone", "show", "1"]);

    assert!(success);
    assert!(stdout.contains("v1.0"));
    assert!(stdout.contains("First release") || stdout.contains("description"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_add_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    run_atelier(dir.path(), &["issue", "create", "Feature 1"]);
    run_atelier(dir.path(), &["issue", "create", "Feature 2"]);

    let (success, _, _) = run_atelier(dir.path(), &["milestone", "add", "1", "1", "2"]);

    assert!(success);

    // Check milestone shows the issues
    let (_, show_out, _) = run_atelier(dir.path(), &["milestone", "show", "1"]);
    assert!(show_out.contains("Feature 1") || show_out.contains("#1"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_close() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["milestone", "close", "1"]);

    assert!(success);
    assert!(stdout.contains("Closed") || stdout.contains("closed"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_delete() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    let (success, _, _) = run_atelier(dir.path(), &["milestone", "delete", "1"]);

    assert!(success);

    // Should no longer appear in list
    let (_, list_out, _) = run_atelier(dir.path(), &["milestone", "list", "-s", "all"]);
    assert!(!list_out.contains("v1.0") || list_out.contains("No milestones"));
}

// ==================== Timer Tests ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_timer_start() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue to time"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["start", "1"]);

    assert!(success);
    assert!(stdout.contains("Started") || stdout.contains("timer") || stdout.contains("#1"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_timer_stop() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue to time"]);
    run_atelier(dir.path(), &["start", "1"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["stop"]);

    assert!(success);
    assert!(stdout.contains("Stopped") || stdout.contains("stopped") || stdout.contains("timer"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_timer_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue to time"]);
    run_atelier(dir.path(), &["start", "1"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["timer"]);

    assert!(success);
    assert!(
        stdout.contains("#1") || stdout.contains("Issue to time") || stdout.contains("running")
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_timer_status_no_timer() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["timer"]);

    assert!(success);
    assert!(
        stdout.contains("No timer running"),
        "Expected 'No timer running' message, got: {}",
        stdout
    );
}

// ==================== Relate Tests ====================

#[test]
fn test_relate_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue 1"]);
    run_atelier(dir.path(), &["issue", "create", "Issue 2"]);

    let (success, _, _) = run_atelier(dir.path(), &["issue", "relate", "1", "2"]);

    assert!(success);
}

#[test]
fn test_related_list() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue 1"]);
    run_atelier(dir.path(), &["issue", "create", "Issue 2"]);
    run_atelier(dir.path(), &["issue", "relate", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "related", "1"]);

    assert!(success);
    assert!(stdout.contains("Issue 2") || stdout.contains("#2"));
}

#[test]
fn test_unrelate_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue 1"]);
    run_atelier(dir.path(), &["issue", "create", "Issue 2"]);
    run_atelier(dir.path(), &["issue", "relate", "1", "2"]);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "unrelate", "1", "2"]);

    assert!(success);

    let (_, related_out, _) = run_atelier(dir.path(), &["issue", "related", "1"]);
    assert!(!related_out.contains("Issue 2") || related_out.contains("No related"));
}

#[test]
fn test_issue_help_hides_non_lifecycle_assumption_commands() {
    let dir = tempdir().unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "--help"]);

    assert!(success, "issue help failed: {}", stderr);
    assert!(!stdout.contains("impact"));
    assert!(!stdout.contains("cascade"));
    assert!(!stdout.contains("falsify"));
}

#[test]
fn test_cascade_commands_are_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Source issue"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Child issue"]);

    let (top_success, _, top_stderr) = run_atelier(dir.path(), &["cascade", "1"]);
    assert!(!top_success);
    assert!(top_stderr.contains("unrecognized subcommand"));

    let (issue_success, _, issue_stderr) = run_atelier(dir.path(), &["issue", "cascade", "1"]);
    assert!(!issue_success);
    assert!(issue_stderr.contains("unrecognized subcommand"));
}

#[test]
fn test_falsify_commands_are_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Source issue"]);

    let (top_success, _, top_stderr) = run_atelier(dir.path(), &["falsify", "1"]);
    assert!(!top_success);
    assert!(top_stderr.contains("unrecognized subcommand"));

    let (issue_success, _, issue_stderr) = run_atelier(dir.path(), &["issue", "falsify", "1"]);
    assert!(!issue_success);
    assert!(issue_stderr.contains("unrecognized subcommand"));
}

#[test]
fn test_issue_impact_reports_downstream_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Source issue"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Child issue"]);
    run_atelier(dir.path(), &["issue", "create", "Derived issue"]);
    run_atelier(dir.path(), &["issue", "create", "Caused issue"]);
    run_atelier(
        dir.path(),
        &["issue", "relate", "1", "3", "--type", "derived"],
    );
    run_atelier(
        dir.path(),
        &["issue", "relate", "1", "4", "--type", "caused-by"],
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "impact", "1"]);

    assert!(success, "issue impact failed: {}", stderr);
    assert!(stdout.contains(&issue_ref(dir.path(), 2)));
    assert!(stdout.contains("Child issue"));
    assert!(stdout.contains(&issue_ref(dir.path(), 3)));
    assert!(stdout.contains("Derived issue"));
    assert!(stdout.contains(&issue_ref(dir.path(), 4)));
    assert!(stdout.contains("Caused issue"));
    assert!(stdout.contains("downstream impact"));
}

// ==================== Tree Tests ====================

#[test]
fn test_tree_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent issue"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Child issue"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "tree"]);

    assert!(success);
    assert!(stdout.contains("Parent issue"));
    assert!(stdout.contains("Child issue"));
}

#[test]
fn test_tree_with_status_filter() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Open parent"]);
    run_atelier(dir.path(), &["issue", "create", "Closed parent"]);
    run_atelier(dir.path(), &["issue", "close", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "tree", "-s", "open"]);

    assert!(success);
    assert!(stdout.contains("Open parent"));
    // Closed issues should not appear
    assert!(!stdout.contains("Closed parent"));
}

#[test]
fn test_tree_compact_collapses_deep_hierarchy() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Level 0"]);
    for level in 1..=6 {
        let parent = issue_ref(dir.path(), level);
        let title = format!("Level {level}");
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "subissue", &parent, &title]);
        assert!(success, "subissue failed: {stderr}");
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "tree", "--compact"]);

    assert!(success, "compact tree failed: {stderr}");
    assert!(stdout.contains("Compact Issue Hierarchy"));
    assert!(stdout.contains("Level 3"));
    assert!(stdout.contains("descendants collapsed"));
    assert!(!stdout.contains("Level 4"));
}

#[test]
fn test_tree_compact_omits_wide_sibling_sets() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Wide parent"]);
    let parent = issue_ref(dir.path(), 1);
    for index in 1..=8 {
        let title = format!("Child {index}");
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "subissue", &parent, &title]);
        assert!(success, "subissue failed: {stderr}");
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "tree", "--compact"]);

    assert!(success, "compact tree failed: {stderr}");
    assert!(stdout.contains("Wide parent children=8 open=8 closed=0"));
    assert!(stdout.contains("... 2 more children omitted"));
}

#[test]
fn test_tree_compact_omits_wide_root_sets() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for index in 1..=8 {
        let title = format!("Root {index}");
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", &title]);
        assert!(success, "root create failed: {stderr}");
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "tree", "--compact"]);

    assert!(success, "compact tree failed: {stderr}");
    assert_eq!(
        stdout.lines().filter(|line| line.contains("Root ")).count(),
        6
    );
    assert!(stdout.contains("... 2 more root issues omitted"));
}

#[test]
fn test_tree_compact_summarizes_mixed_open_closed_subtrees() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Mixed parent"]);
    let parent = issue_ref(dir.path(), 1);
    run_atelier(dir.path(), &["issue", "subissue", &parent, "Open child"]);
    run_atelier(dir.path(), &["issue", "subissue", &parent, "Closed child"]);
    let closed = issue_id_by_title(dir.path(), "Closed child");
    close_issue_with_evidence(dir.path(), &closed, None);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "tree", "--compact"]);
    assert!(success, "compact tree failed: {stderr}");
    assert!(stdout.contains("Mixed parent children=2 open=1 closed=1"));
    assert!(stdout.contains("[closed"));

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "tree", "--compact", "-s", "open"]);
    assert!(success, "compact open tree failed: {stderr}");
    assert!(stdout.contains("Mixed parent children=1 open=1 closed=0"));
    assert!(!stdout.contains("Closed child"));
}

// ==================== Next Tests ====================

#[test]
fn test_next_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Low priority", "-p", "low"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "High priority", "-p", "high"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Critical priority", "-p", "critical"],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    // Should suggest critical or high priority issue
    assert!(
        stdout.contains("Critical priority")
            || stdout.contains("High priority")
            || stdout.contains("#3")
            || stdout.contains("#2")
    );
}

#[test]
fn test_next_no_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    assert!(
        stdout.contains("No issues ready to work on"),
        "Expected 'No issues ready to work on' message, got: {}",
        stdout
    );
}

// ==================== Export/Import Tests ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_export_json() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue 1"]);
    run_atelier(dir.path(), &["issue", "create", "Issue 2"]);

    let export_path = dir.path().join("export.json");
    let (success, _, _) = run_atelier(
        dir.path(),
        &["export", "-o", export_path.to_str().unwrap(), "-f", "json"],
    );

    assert!(success);
    assert!(export_path.exists());

    // Verify JSON content
    let content = std::fs::read_to_string(&export_path).unwrap();
    assert!(content.contains("Issue 1"));
    assert!(content.contains("Issue 2"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_export_markdown() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Issue 1", "-d", "Description 1"],
    );

    let export_path = dir.path().join("export.md");
    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "export",
            "-o",
            export_path.to_str().unwrap(),
            "-f",
            "markdown",
        ],
    );

    assert!(success);
    assert!(export_path.exists());

    let content = std::fs::read_to_string(&export_path).unwrap();
    assert!(
        content.contains("Issue 1"),
        "Exported markdown should contain issue title, got: {}",
        &content[..content.len().min(200)]
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_import_json() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create issues and export
    run_atelier(dir.path(), &["issue", "create", "Exported Issue"]);
    let export_path = dir.path().join("export.json");
    run_atelier(
        dir.path(),
        &["export", "-o", export_path.to_str().unwrap(), "-f", "json"],
    );

    // Create a fresh atelier instance and import
    let dir2 = tempdir().unwrap();
    init_atelier(dir2.path());

    let (success, _, _) = run_atelier(dir2.path(), &["import", export_path.to_str().unwrap()]);

    assert!(success);

    // Verify imported issue exists
    let (_, list_out, _) = run_atelier(dir2.path(), &["issue", "list", "-s", "all"]);
    assert!(list_out.contains("Exported Issue") || list_out.contains("#1"));
}

// ==================== Tested Command Tests ====================

#[test]
fn test_tested_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "tested"]);

    assert!(success);
    assert!(
        stdout.contains("Marked tests as run"),
        "Expected 'Marked tests as run' in output, got: {}",
        stdout
    );
}

// ==================== Additional Create Edge Cases ====================

#[test]
fn test_create_with_template() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", "Bug report", "-t", "bug"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("Bug report"));
}

#[test]
fn test_create_all_priorities() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for priority in &["low", "medium", "high", "critical"] {
        let (success, _, _) = run_atelier(
            dir.path(),
            &[
                "issue",
                "create",
                &format!("{} issue", priority),
                "-p",
                priority,
            ],
        );
        assert!(success, "Failed to create {} priority issue", priority);
    }

    let (_, list_out, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(list_out.contains("low"));
    assert!(list_out.contains("medium"));
    assert!(list_out.contains("high"));
    assert!(list_out.contains("critical"));
}

#[test]
fn test_subissue_with_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent"]);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "subissue", "1", "Child", "-p", "critical"],
    );

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "2"]);
    assert!(show_out.contains("critical"));
}

#[test]
fn test_subissue_with_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent"]);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "subissue", "1", "Child", "-d", "Child description"],
    );

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "2"]);
    assert!(show_out.contains("Child description"));
}

// ==================== Additional Delete Edge Cases ====================

#[test]
fn test_delete_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "delete", "999", "-f"]);

    // Should fail or warn about nonexistent issue
    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

#[test]
fn test_delete_with_subissues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Parent"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Child"]);

    let (success, _, _) = run_atelier(dir.path(), &["issue", "delete", "1", "-f"]);

    assert!(success);

    // Both parent and child should be gone
    let (_, list_out, _) = run_atelier(dir.path(), &["issue", "list", "-s", "all"]);
    assert!(!list_out.contains("Parent"));
    assert!(!list_out.contains("Child"));
}

// ==================== Additional Session Edge Cases ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_work_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["session", "start"]);
    let (success, _, stderr) = run_atelier(dir.path(), &["session", "work", "999"]);

    // Should fail or warn
    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_end_without_start() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["session", "end"]);

    // Should fail or report no active session
    assert!(
        !success || stdout.contains("No active") || stderr.contains("No active"),
        "Ending without starting should fail or report no active session, got stdout: {}, stderr: {}", stdout, stderr
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_status_without_session() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["session", "status"]);

    assert!(success);
    assert!(
        stdout.contains("No active session"),
        "Expected 'No active session' message, got: {}",
        stdout
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_multiple_starts() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["session", "start"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["session", "start"]);

    // Second start should either warn or start a new session
    assert!(success);
    assert!(stdout.contains("already") || stdout.contains("Session") || stdout.contains("started"));
}

// ==================== Additional Next Edge Cases ====================

#[test]
fn test_next_with_blocked_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Blocked issue", "-p", "critical"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Blocker issue", "-p", "low"],
    );
    run_atelier(dir.path(), &["issue", "block", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    // Should suggest the blocker, not the blocked issue
    assert!(
        stdout.contains("Blocker issue"),
        "Next should recommend the unblocked blocker, got: {}",
        stdout
    );
    assert!(
        !stdout.contains("Next: #1"),
        "Next should not recommend the blocked issue as top pick"
    );
}

#[test]
fn test_next_all_closed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue 1"]);
    close_issue_with_evidence(dir.path(), "1", None);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    assert!(
        stdout.contains("No issues ready to work on"),
        "Expected 'No issues ready to work on' message, got: {}",
        stdout
    );
}

// ==================== Additional Archive Edge Cases ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_archive_older_days() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Old issue"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);

    // Try to archive issues older than 0 days (should include our just-closed issue)
    let (success, _, _) = run_atelier(dir.path(), &["archive", "older", "0"]);

    assert!(success);
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_archive_already_archived() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);
    run_atelier(dir.path(), &["archive", "add", "1"]);

    // Try to archive again
    let (success, stdout, stderr) = run_atelier(dir.path(), &["archive", "add", "1"]);

    // Should report already archived or fail
    assert!(
        stdout.contains("already") || stderr.contains("already") || !success,
        "Archiving twice should indicate already archived, got stdout: {}, stderr: {}",
        stdout,
        stderr
    );
}

// ==================== Additional Milestone Edge Cases ====================

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_remove_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    run_atelier(dir.path(), &["issue", "create", "Feature"]);
    run_atelier(dir.path(), &["milestone", "add", "1", "1"]);

    let (success, _, _) = run_atelier(dir.path(), &["milestone", "remove", "1", "1"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["milestone", "show", "1"]);
    assert!(!show_out.contains("Feature") || show_out.contains("No issues"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_show_nonexistent() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["milestone", "show", "999"]);

    assert!(!success || stderr.contains("not found") || stderr.contains("No milestone"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_list_closed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    run_atelier(dir.path(), &["milestone", "create", "v2.0"]);
    run_atelier(dir.path(), &["milestone", "close", "1"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["milestone", "list", "-s", "closed"]);

    assert!(success);
    assert!(stdout.contains("v1.0"));
    assert!(!stdout.contains("v2.0"));
}

// ==================== Additional List Edge Cases ====================

#[test]
fn test_list_filter_by_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Low issue", "-p", "low"]);
    run_atelier(dir.path(), &["issue", "create", "High issue", "-p", "high"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list", "-p", "high"]);

    assert!(success);
    assert!(stdout.contains("High issue"));
    assert!(!stdout.contains("Low issue"));
}

#[test]
fn test_list_all_statuses() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Open issue"]);
    run_atelier(dir.path(), &["issue", "create", "Closed issue"]);
    run_atelier(dir.path(), &["issue", "close", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list", "-s", "all"]);

    assert!(success);
    assert!(stdout.contains("Open issue"));
    assert!(stdout.contains("Closed issue"));
}

// ==================== Additional Update Edge Cases ====================

#[test]
fn test_update_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "update", "1", "-d", "New description"],
    );

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("New description"));
}

#[test]
fn test_update_nonexistent() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "update", "999", "--title", "New"]);

    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

// ==================== Additional Show Edge Cases ====================

#[test]
fn test_show_with_labels() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    run_atelier(dir.path(), &["issue", "label", "1", "bug"]);
    run_atelier(dir.path(), &["issue", "label", "1", "urgent"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success);
    assert!(stdout.contains("bug"));
    assert!(stdout.contains("urgent"));
}

#[test]
fn test_show_with_comments() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    run_atelier(dir.path(), &["issue", "comment", "1", "First comment"]);
    run_atelier(dir.path(), &["issue", "comment", "1", "Second comment"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success);
    assert!(stdout.contains("First comment"));
    assert!(stdout.contains("Second comment"));
}

#[test]
fn test_show_with_blockers() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Blocked"]);
    run_atelier(dir.path(), &["issue", "create", "Blocker"]);
    run_atelier(dir.path(), &["issue", "block", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success);
    assert!(
        stdout.contains("Blocker")
            || stdout.contains(&issue_ref(dir.path(), 2))
            || stdout.contains("blocked")
    );
}

// ==================== Additional Search Edge Cases ====================

#[test]
fn test_search_no_results() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test issue"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "search", "nonexistent"]);

    assert!(success);
    assert!(
        stdout.contains("No issues found matching"),
        "Expected 'No issues found matching' message, got: {}",
        stdout
    );
}

#[test]
fn test_search_in_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Generic title",
            "-d",
            "specific_keyword_here",
        ],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "search", "specific_keyword"]);

    assert!(success);
    assert!(stdout.contains("Generic title") || stdout.contains("#1"));
}

// ==================== Init Edge Cases ====================

#[test]
fn test_init_force_update() {
    let dir = tempdir().unwrap();

    run_atelier(dir.path(), &["init"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["init", "--force"]);

    assert!(success);
    assert!(stdout.contains("Atelier initialized successfully"));
    assert!(dir.path().join(".atelier").join("state.db").exists());
    assert!(!dir.path().join(".atelier").join("rules").exists());
    assert!(!dir.path().join(".claude").exists());
    assert!(!dir.path().join(".mcp.json").exists());
}

// ==================== Complex Workflow Tests ====================

#[test]
fn test_full_issue_lifecycle() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create
    run_atelier(
        dir.path(),
        &["issue", "create", "Lifecycle test", "-p", "high"],
    );

    // Add labels
    run_atelier(dir.path(), &["issue", "label", "1", "feature"]);

    // Add comment
    run_atelier(dir.path(), &["issue", "comment", "1", "Working on this"]);

    // Update
    run_atelier(dir.path(), &["issue", "update", "1", "-p", "critical"]);

    // Close
    run_atelier(dir.path(), &["issue", "close", "1"]);

    // Verify final state
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success);
    assert!(stdout.contains("Lifecycle test"));
    assert!(stdout.contains("critical"));
    assert!(stdout.contains("feature"));
    assert!(stdout.contains("Working on this"));
    assert!(stdout.contains("closed"));
}

#[test]
fn test_dependency_chain() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create a chain: 1 <- 2 <- 3 (3 blocks 2, 2 blocks 1)
    run_atelier(dir.path(), &["issue", "create", "Final task"]);
    run_atelier(dir.path(), &["issue", "create", "Middle task"]);
    run_atelier(dir.path(), &["issue", "create", "First task"]);

    run_atelier(dir.path(), &["issue", "block", "1", "2"]);
    run_atelier(dir.path(), &["issue", "block", "2", "3"]);

    // Only issue 3 should be ready
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success);
    assert!(stdout.contains("First task") || stdout.contains("#3"));
    assert!(!stdout.contains("Final task"));
    assert!(!stdout.contains("Middle task"));

    // Close 3, now 2 should be ready
    run_atelier(dir.path(), &["issue", "close", "3"]);
    let (_, stdout, _) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(stdout.contains("Middle task") || stdout.contains("#2"));
}

// ==================== Targeted Coverage Tests ====================

// --- next.rs: Multiple ready issues with runners-up ---
#[test]
fn test_next_with_multiple_ready_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create multiple issues with different priorities
    run_atelier(
        dir.path(),
        &["issue", "create", "Low prio task", "-p", "low"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Medium prio task", "-p", "medium"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "High prio task", "-p", "high"],
    );
    run_atelier(
        dir.path(),
        &["issue", "create", "Critical task", "-p", "critical"],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    // Should recommend highest priority first
    assert!(stdout.contains("Critical") || stdout.contains("#4"));
    // Should show "Also ready" section
    assert!(stdout.contains("Also ready") || stdout.contains("ready"));
}

// --- next.rs: Issue with description preview ---
#[test]
fn test_next_with_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Task with description",
            "-p",
            "high",
            "-d",
            "This is a detailed description for the task",
        ],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    assert!(stdout.contains("description") || stdout.contains("Task with description"));
}

// --- next.rs: Progress with subissues ---
#[test]
fn test_next_with_subissue_progress() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create parent with subissues
    run_atelier(
        dir.path(),
        &["issue", "create", "Parent task", "-p", "high"],
    );
    run_atelier(dir.path(), &["issue", "subissue", "1", "Sub 1"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Sub 2"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Sub 3"]);

    // Close one subissue to create progress
    run_atelier(dir.path(), &["issue", "close", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    // Should show progress
    assert!(
        stdout.contains("Progress")
            || stdout.contains("1/3")
            || stdout.contains("subissue")
            || stdout.contains("Parent task")
    );
}

// --- next.rs: Only subissues ready (no parents) ---
#[test]
fn test_next_only_subissues_ready() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create parent that is blocked
    run_atelier(dir.path(), &["issue", "create", "Blocker"]);
    run_atelier(dir.path(), &["issue", "create", "Parent"]);
    run_atelier(dir.path(), &["issue", "block", "2", "1"]);

    // Create unblocked subissue under the blocked parent
    run_atelier(dir.path(), &["issue", "subissue", "2", "Subissue"]);

    // Close the blocker - now parent has only subissue as ready issue
    run_atelier(dir.path(), &["issue", "close", "1"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "next"]);

    assert!(success);
    // Should show something - either parent or subissue
    assert!(
        stdout.contains("Next")
            || stdout.contains("#2")
            || stdout.contains("#3")
            || stdout.contains("Parent")
            || stdout.contains("Subissue")
    );
}

// --- import.rs: Import with parent relationships ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_import_with_parent_relationships() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create and export issues with parent-child relationship
    run_atelier(dir.path(), &["issue", "create", "Parent issue"]);
    run_atelier(dir.path(), &["issue", "subissue", "1", "Child issue"]);

    let export_path = dir.path().join("export.json");
    run_atelier(
        dir.path(),
        &["export", "-o", export_path.to_str().unwrap(), "-f", "json"],
    );

    // Initialize a fresh directory and import
    let dir2 = tempdir().unwrap();
    init_atelier(dir2.path());

    // Copy export file to new location
    std::fs::copy(&export_path, dir2.path().join("import.json")).unwrap();

    let import_path = dir2.path().join("import.json");
    let (success, stdout, _) = run_atelier(dir2.path(), &["import", import_path.to_str().unwrap()]);

    assert!(success);
    assert!(stdout.contains("Imported") || stdout.contains("import"));

    // Verify the parent-child relationship was preserved
    let (_, tree_out, _) = run_atelier(dir2.path(), &["issue", "tree"]);
    assert!(tree_out.contains("Parent") && tree_out.contains("Child"));
}

// --- import.rs: Import issues with labels and comments ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_import_with_labels_and_comments() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create issue with labels and comments
    run_atelier(dir.path(), &["issue", "create", "Labeled issue"]);
    run_atelier(dir.path(), &["issue", "label", "1", "bug"]);
    run_atelier(dir.path(), &["issue", "label", "1", "urgent"]);
    run_atelier(dir.path(), &["issue", "comment", "1", "First comment"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);

    let export_path = dir.path().join("export.json");
    run_atelier(
        dir.path(),
        &["export", "-o", export_path.to_str().unwrap(), "-f", "json"],
    );

    // Import to fresh directory
    let dir2 = tempdir().unwrap();
    init_atelier(dir2.path());

    std::fs::copy(&export_path, dir2.path().join("import.json")).unwrap();

    let import_path = dir2.path().join("import.json");
    let (success, _, _) = run_atelier(dir2.path(), &["import", import_path.to_str().unwrap()]);

    assert!(success);

    // Verify labels and status were preserved
    let labeled_id = issue_id_by_title(dir2.path(), "Labeled issue");
    let (_, show_out, _) = run_atelier(dir2.path(), &["issue", "show", &labeled_id]);
    assert!(show_out.contains("bug") || show_out.contains("Label"));
    assert!(show_out.contains("closed") || show_out.contains("Closed"));
}

// --- session.rs: Session with handoff notes from previous session ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_start_shows_handoff_notes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Start and end first session with handoff notes
    run_atelier(dir.path(), &["session", "start"]);
    run_atelier(
        dir.path(),
        &["session", "end", "--notes", "Remember to check auth module"],
    );

    // Start new session - should show handoff notes
    let (success, stdout, _) = run_atelier(dir.path(), &["session", "start"]);

    assert!(success);
    assert!(
        stdout.contains("Remember to check auth module")
            || stdout.contains("Handoff")
            || stdout.contains("Previous")
            || stdout.contains("notes")
    );
}

// --- session.rs: Session status with active issue ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_status_with_active_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Active task"]);
    run_atelier(dir.path(), &["session", "start"]);
    run_atelier(dir.path(), &["session", "work", "1"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["session", "status"]);

    assert!(success);
    assert!(stdout.contains("Active task") || stdout.contains("#1") || stdout.contains("Working"));
}

// --- create.rs: Template with user priority override ---
#[test]
fn test_template_with_priority_override() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Bug template defaults to high, override to critical
    let (success, stdout, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Critical bug",
            "-t",
            "bug",
            "-p",
            "critical",
        ],
    );

    assert!(success);
    assert!(stdout.contains("atelier-"));

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(show_out.contains("critical"));
}

// --- create.rs: Template with user description ---
#[test]
fn test_template_with_user_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Bug with details",
            "-t",
            "bug",
            "-d",
            "User provided details here",
        ],
    );

    assert!(success);
    assert!(stdout.contains("atelier-"));

    let (_, show_out, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    // Should have both template prefix and user description
    assert!(show_out.contains("User provided details") || show_out.contains("Steps to reproduce"));
}

// --- create.rs: Subissue with invalid parent ---
#[test]
fn test_subissue_invalid_parent() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "subissue", "999", "Orphan"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999") || stderr.contains("Parent"));
}

// --- relate.rs: Related issues display ---
#[test]
fn test_related_issues_display() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue A"]);
    run_atelier(dir.path(), &["issue", "create", "Issue B"]);
    run_atelier(dir.path(), &["issue", "create", "Issue C"]);

    run_atelier(dir.path(), &["issue", "relate", "1", "2"]);
    run_atelier(dir.path(), &["issue", "relate", "1", "3"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "related", "1"]);

    assert!(success);
    assert!(stdout.contains("Issue B") || stdout.contains("#2"));
    assert!(stdout.contains("Issue C") || stdout.contains("#3"));
}

// --- label.rs: Multiple labels on same issue ---
#[test]
fn test_multiple_labels() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Multi-label issue"]);
    run_atelier(dir.path(), &["issue", "label", "1", "bug"]);
    run_atelier(dir.path(), &["issue", "label", "1", "urgent"]);
    run_atelier(dir.path(), &["issue", "label", "1", "frontend"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success);
    assert!(stdout.contains("bug"));
    assert!(stdout.contains("urgent"));
    assert!(stdout.contains("frontend"));
}

// --- Export markdown format test ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_export_markdown_format() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue for markdown"]);
    run_atelier(dir.path(), &["issue", "label", "1", "test"]);
    run_atelier(dir.path(), &["issue", "comment", "1", "Test comment"]);

    let export_path = dir.path().join("export.md");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "export",
            "-o",
            export_path.to_str().unwrap(),
            "-f",
            "markdown",
        ],
    );

    assert!(success);
    assert!(
        stderr.contains("Exported"),
        "Expected 'Exported' in stderr, got: {}",
        stderr
    );

    // Verify file exists and has the issue content
    let content = std::fs::read_to_string(&export_path).unwrap();
    assert!(
        content.contains("Issue for markdown"),
        "Exported markdown should contain issue title, got: {}",
        &content[..content.len().min(200)]
    );
}

// --- Archive older days test ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_archive_older_no_matches() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create and close an issue (just now, so not old)
    run_atelier(dir.path(), &["issue", "create", "New issue"]);
    run_atelier(dir.path(), &["issue", "close", "1"]);

    // Archive issues older than 30 days - should find none
    let (success, stdout, _) = run_atelier(dir.path(), &["archive", "older", "30"]);

    assert!(success);
    assert!(
        stdout.contains("No issues to archive") || stdout.contains("Archived 0"),
        "Should report no issues to archive, got: {}",
        stdout
    );
}

// ==================== Additional Edge Case Coverage ====================

// --- relate.rs: Error cases ---
#[test]
fn test_relate_nonexistent_first_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Existing"]);

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "relate", "999", "1"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_relate_nonexistent_second_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Existing"]);

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "relate", "1", "999"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_relate_already_related() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue A"]);
    run_atelier(dir.path(), &["issue", "create", "Issue B"]);
    run_atelier(dir.path(), &["issue", "relate", "1", "2"]);

    // Try to relate again
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "relate", "1", "2"]);

    assert!(success);
    assert!(stdout.contains("already") || stdout.contains("related"));
}

#[test]
fn test_unrelate_no_relation() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue A"]);
    run_atelier(dir.path(), &["issue", "create", "Issue B"]);

    // Try to unrelate issues that aren't related
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "unrelate", "1", "2"]);

    assert!(success);
    assert!(
        stdout.contains("relation found between"),
        "Expected 'No relation found' message, got: {}",
        stdout
    );
}

#[test]
fn test_related_no_relations() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Solo issue"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "related", "1"]);

    assert!(success);
    assert!(
        stdout.contains("No related issues"),
        "Expected 'No related issues' message, got: {}",
        stdout
    );
}

#[test]
fn test_related_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "related", "999"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

// --- label.rs: Error cases ---
#[test]
fn test_label_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "label", "999", "bug"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_label_already_exists() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    run_atelier(dir.path(), &["issue", "label", "1", "bug"]);

    // Try to add same label again
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "label", "1", "bug"]);

    assert!(success);
    assert!(stdout.contains("already") || stdout.contains("exists"));
}

#[test]
fn test_unlabel_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "unlabel", "999", "bug"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_unlabel_nonexistent_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "unlabel", "1", "nonexistent"]);

    assert!(success);
    assert!(
        stdout.contains("not found"),
        "Expected 'not found' message for non-existent label, got: {}",
        stdout
    );
}

// --- create.rs: Invalid priority ---
#[test]
fn test_create_invalid_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Issue", "-p", "invalid"]);

    assert!(!success);
    assert!(
        stderr.contains("Invalid") || stderr.contains("priority") || stderr.contains("invalid")
    );
}

// --- create.rs: Unknown template ---
#[test]
fn test_create_unknown_template() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Issue", "-t", "unknown"]);

    assert!(!success);
    assert!(
        stderr.contains("Unknown") || stderr.contains("template") || stderr.contains("unknown")
    );
}

// --- block.rs: Error cases ---
#[test]
fn test_block_self() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "block", "1", "1"]);

    assert!(!success, "Blocking an issue by itself should fail");
    assert!(
        stderr.contains("cannot block itself"),
        "Error should mention self-blocking, got stderr: {}",
        stderr
    );
}

#[test]
fn test_block_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Issue"]);

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "block", "1", "999"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

// --- session.rs: Session status deleted issue ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_session_status_deleted_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "To delete"]);
    run_atelier(dir.path(), &["session", "start"]);
    run_atelier(dir.path(), &["session", "work", "1"]);
    run_atelier(dir.path(), &["issue", "delete", "1", "-f"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["session", "status"]);

    assert!(success);
    // Should show issue not found or empty working status
    assert!(stdout.contains("not found") || stdout.contains("#1") || stdout.contains("Session"));
}

// --- show.rs: Show with related issues ---
#[test]
fn test_show_with_related_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Main issue"]);
    run_atelier(dir.path(), &["issue", "create", "Related issue"]);
    run_atelier(dir.path(), &["issue", "relate", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);

    assert!(success);
    assert!(stdout.contains("Related") || stdout.contains("#2") || stdout.contains("Main issue"));
}

// --- milestone.rs: Edge cases ---
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_add_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["milestone", "add", "1", "999"]);

    // Command succeeds but warns about nonexistent issue
    assert!(success);
    assert!(
        stdout.contains("not found")
            || stdout.contains("999")
            || stdout.contains("Warning")
            || stdout.contains("skipping")
    );
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_milestone_delete_nonexistent() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["milestone", "delete", "999"]);

    // Command succeeds but reports not found
    assert!(success);
    assert!(stdout.contains("not found") || stdout.contains("999"));
}

// ==================== Security & Stress Tests ====================

/// Test with very long title — within limit should succeed, over limit should fail
#[test]
fn test_stress_very_long_title() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Within the 512-char limit: should succeed
    let ok_title = "A".repeat(512);
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "create", &ok_title]);
    assert!(success);
    assert!(stdout.contains("atelier-"));

    // Verify it can be listed and shown
    let (success, _, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);

    let (success, _, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success);

    // Over the 512-char limit: should fail
    let too_long = "A".repeat(513);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", &too_long]);
    assert!(!success);
}

/// Test with very long description
/// Note: Windows has ~8191 char command line limit, so we use a smaller size
#[test]
fn test_stress_very_long_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Use 5000 chars - safe for Windows command line limits
    let long_desc = "B".repeat(5000);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "create", "Long desc issue", "-d", &long_desc],
    );

    assert!(success);

    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success);
    // Should contain at least part of the description
    assert!(stdout.contains("BBBB"));
}

/// Test creating many issues (stress test)
#[test]
fn test_stress_many_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create 100 issues
    for i in 0..100 {
        let title = format!("Issue number {}", i);
        let (success, _, _) = run_atelier(dir.path(), &["issue", "create", &title]);
        assert!(success, "Failed to create issue {}", i);
    }

    // Verify list works
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("Issue number 99"));

    // Verify search works on large DB
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "search", "number 50"]);
    assert!(success);
    assert!(stdout.contains("50"));
}

/// Test deeply nested subissues
#[test]
fn test_stress_deep_nesting() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create root issue
    run_atelier(dir.path(), &["issue", "create", "Level 0"]);

    // Create 20 levels of nesting
    for i in 1..=20 {
        let parent_id = i.to_string();
        let title = format!("Level {}", i);
        let (success, _, _) = run_atelier(dir.path(), &["issue", "subissue", &parent_id, &title]);
        assert!(success, "Failed to create subissue at level {}", i);
    }

    // Verify tree command handles deep nesting
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "tree"]);
    assert!(success);
    assert!(stdout.contains("Level 20"));
}

/// Test SQL injection in title (should be safely escaped)
#[test]
fn test_security_sql_injection_title() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let malicious_titles = [
        "'; DROP TABLE issues; --",
        "\" OR 1=1 --",
        "Robert'); DROP TABLE issues;--",
        "1; DELETE FROM issues WHERE 1=1; --",
        "' UNION SELECT * FROM sqlite_master --",
    ];

    for title in malicious_titles {
        let (success, _, _) = run_atelier(dir.path(), &["issue", "create", title]);
        assert!(success, "Failed to create issue with title: {}", title);
    }

    // Verify all issues exist and DB is intact
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("DROP TABLE")); // Title should be stored literally
}

/// Test SQL injection in search
#[test]
fn test_security_sql_injection_search() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Normal issue"]);

    let malicious_searches = [
        "' OR '1'='1",
        "'; DROP TABLE issues; --",
        "\" OR \"\"=\"",
        "%' OR 1=1 --",
    ];

    for query in malicious_searches {
        let (success, _, _) = run_atelier(dir.path(), &["issue", "search", query]);
        // Should not crash, may or may not find results
        assert!(success, "Search crashed with query: {}", query);
    }

    // DB should still be intact
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("Normal issue"));
}

/// Test null bytes in input
/// Note: OS rejects null bytes in command args - this is correct security behavior
#[test]
fn test_security_null_bytes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Null bytes are rejected at the OS level (can't pass via command line)
    // This is actually GOOD security behavior - we test that the app
    // handles other special chars correctly instead
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", "Test with special: \t\r"]);
    assert!(success);

    let (success, _, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
}

/// Test newlines and control characters in input
#[test]
fn test_security_control_characters() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let control_inputs = [
        "Line1\nLine2",
        "Tab\there",
        "Return\rhere",
        "Bell\x07sound",
        "Escape\x1b[31mred",
    ];

    for input in control_inputs {
        let (success, _, _) = run_atelier(dir.path(), &["issue", "create", input]);
        assert!(success, "Failed with input containing control chars");
    }

    let (success, _, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
}

/// Test empty strings
#[test]
fn test_edge_empty_strings() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Empty title - should either fail or succeed (both acceptable, just don't crash)
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "create", ""]);
    if success {
        // If it accepted empty title, verify the issue was created
        assert!(
            stdout.contains("Created issue"),
            "If success, should show created message, got: {}",
            stdout
        );
    }

    // Empty comment
    run_atelier(dir.path(), &["issue", "create", "Issue"]);
    let (_, _, _) = run_atelier(dir.path(), &["issue", "comment", "1", ""]);

    // Empty label
    let (_, _, _) = run_atelier(dir.path(), &["issue", "label", "1", ""]);
}

/// Test integer overflow in IDs
#[test]
fn test_edge_large_ids() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Test"]);

    // Very large IDs - should fail with "not found" since issue doesn't exist
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", "9223372036854775807"]); // i64::MAX
    assert!(!success, "Show with non-existent large ID should fail");
    assert!(
        stderr.contains("not found"),
        "Error should say not found, got: {}",
        stderr
    );

    // Overflow ID - should fail with parse error or not found
    let (success, _, _) = run_atelier(dir.path(), &["issue", "show", "99999999999999999999999"]);
    assert!(!success, "Show with overflow ID should fail");

    // Negative IDs - clap may reject or db returns not found
    let (success, _, _) = run_atelier(dir.path(), &["issue", "show", "-1"]);
    assert!(!success, "Show with negative ID should fail");
}

/// Test concurrent-like rapid operations
#[test]
fn test_stress_rapid_operations() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Rapid create/close/reopen cycle
    for i in 0..20 {
        let title = format!("Rapid issue {}", i);
        run_atelier(dir.path(), &["issue", "create", &title]);
        let id = (i + 1).to_string();
        run_atelier(dir.path(), &["issue", "close", &id]);
        run_atelier(dir.path(), &["issue", "reopen", &id]);
        run_atelier(dir.path(), &["issue", "comment", &id, "Rapid comment"]);
        run_atelier(dir.path(), &["issue", "label", &id, "rapid"]);
    }

    // Verify all operations completed
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("Rapid issue 19"));
}

/// Test export/import round-trip preserves data
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_integrity_export_import_roundtrip() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create complex data
    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Parent",
            "-p",
            "high",
            "-d",
            "Parent desc",
        ],
    );
    run_atelier(dir.path(), &["issue", "subissue", "1", "Child"]);
    run_atelier(dir.path(), &["issue", "label", "1", "important"]);
    run_atelier(dir.path(), &["issue", "comment", "1", "Test comment"]);

    // Export
    let export_path = dir.path().join("backup.json");
    let (success, _, _) = run_atelier(
        dir.path(),
        &["export", "-o", export_path.to_str().unwrap(), "-f", "json"],
    );
    assert!(success);

    // Import to new location
    let dir2 = tempdir().unwrap();
    init_atelier(dir2.path());
    std::fs::copy(&export_path, dir2.path().join("backup.json")).unwrap();

    let (success, _, _) = run_atelier(
        dir2.path(),
        &["import", dir2.path().join("backup.json").to_str().unwrap()],
    );
    assert!(success);

    // Verify data integrity - title and structure preserved
    let parent_id = issue_id_by_title(dir2.path(), "Parent");
    let (success, stdout, _) = run_atelier(dir2.path(), &["issue", "show", &parent_id]);
    assert!(success);
    assert!(stdout.contains("Parent"));

    // Verify child was imported
    let (success, stdout, _) = run_atelier(dir2.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("Child") || stdout.contains("#2"));
}

#[test]
fn test_command_result_json_mode_is_rejected_and_human_subset_works() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier_raw(dir.path(), &["--json", "doctor"]);
    assert!(!success, "--json should not be accepted");
    assert!(stderr.contains("unexpected argument '--json'"));

    for args in [
        vec!["issue", "list", "--json"],
        vec!["issue", "show", "1", "--json"],
        vec!["issue", "update", "1", "--claim", "--json"],
        vec!["mission", "list", "--json"],
        vec![
            "workflow",
            "validate",
            "issue",
            "1",
            "--transition",
            "close",
            "--json",
        ],
        vec!["doctor", "--json"],
    ] {
        let (success, _, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(!success, "{args:?} should reject --json");
        assert!(
            stderr.contains("unexpected argument '--json'"),
            "{args:?} stderr did not reject --json: {stderr}"
        );
    }

    for args in [
        vec!["issue", "--help"],
        vec!["mission", "--help"],
        vec!["workflow", "--help"],
        vec!["doctor", "--help"],
    ] {
        let (success, stdout, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(success, "{args:?} help failed: {stderr}");
        assert!(
            !stdout.contains("--json"),
            "{args:?} help still advertises --json:\n{stdout}"
        );
    }

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Agent Factory task",
            "--issue-type",
            "feature",
            "--label",
            "agent-factory",
        ],
    );
    assert!(success, "create failed: {stderr}");
    assert!(stdout.contains("Created issue atelier-"));
    assert!(stdout.contains("Type:     feature"));
    assert!(stdout.contains("Next Commands"));
    let task_id = issue_id_by_title(dir.path(), "Agent Factory task");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            "1",
            "--claim",
            "--append-notes",
            "handoff note",
            "--priority",
            "high",
        ],
    );
    assert!(success, "update failed: {stderr}");
    assert!(stdout.contains(&format!("Updated issue {task_id}")));
    assert!(stdout.contains("Priority: high"));
    assert!(stdout.contains("Assignee:"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "#1"]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains(&task_id));
    assert!(stdout.contains("handoff note"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success, "ready failed: {stderr}");
    assert!(stdout.contains("1 total"));

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Blocker"]);
    assert!(success, "blocker create failed: {stderr}");
    let blocker_id = issue_ref(dir.path(), 2);
    let (success, stdout, stderr) = run_atelier(dir.path(), &["dep", "add", "1", "2"]);
    assert!(success, "dep add failed: {stderr}");
    assert!(stdout.contains(&task_id));
    assert!(stdout.contains(&blocker_id));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["dep", "list", "1"]);
    assert!(success, "dep list failed: {stderr}");
    assert!(stdout.contains(&blocker_id));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["dep", "remove", "1", "2"]);
    assert!(success, "dep remove failed: {stderr}");
    assert!(stdout.contains(&task_id));
    assert!(stdout.contains(&blocker_id));

    for args in [
        vec!["issue", "list", "--status", "all"],
        vec!["issue", "search", "Factory"],
        vec!["lint"],
        vec!["export"],
        vec!["export", "--check"],
        vec!["doctor"],
        vec!["rebuild"],
    ] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "{args:?} failed: {stderr}");
        assert!(!stdout.trim_start().starts_with('{'), "{args:?}");
    }

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "show", "missing"]);
    assert!(!success);
    assert!(stderr.contains("missing"));
}

#[test]
fn test_first_class_records_export_rebuild_and_validate() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "create",
            "Ship records",
            "--body",
            "Mission body",
            "--constraint",
            "Keep issues accountable",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Ship records");
    let mission_id = mission_id.as_str();
    let (success, mission_update, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "update",
            mission_id,
            "--body",
            "Updated mission body",
            "--risk",
            "Projection drift",
            "--validation",
            "Run focused mission checks",
        ],
    );
    assert!(success, "mission update failed: {stderr}");
    assert!(mission_update.contains("Status: ready"));

    let (success, plan_out, stderr) = run_atelier(
        dir.path(),
        &["plan", "create", "Execution plan", "--body", "Do the thing"],
    );
    assert!(success, "plan create failed: {stderr}");
    assert!(plan_out.contains("[plan] open - Execution plan"));
    let plan_id = record_id_by_title(dir.path(), "plans", "Execution plan");
    let plan_id = plan_id.as_str();
    let (success, revise_out, stderr) = run_atelier(
        dir.path(),
        &[
            "plan",
            "revise",
            plan_id,
            "Do the thing, then verify the projection.",
            "--reason",
            "projection-first",
        ],
    );
    assert!(success, "plan revise failed: {stderr}");
    assert!(revise_out.contains("Revision: 2"));

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "add",
            "--kind",
            "test",
            "--result",
            "pass",
            "cargo test passed",
        ],
    );
    assert!(success, "evidence add failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - cargo test passed"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "cargo test passed");
    let evidence_id = evidence_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "plan",
            "link",
            plan_id,
            "mission",
            mission_id,
            "--type",
            "planned_by",
        ],
    );
    assert!(success, "mission-plan link failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", evidence_id, "mission", mission_id],
    );
    assert!(success, "evidence attach failed: {stderr}");

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Wire mission work",
            "--issue-type",
            "task",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Wire mission work");
    let issue_id = issue_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, issue_id]);
    assert!(success, "mission-work link failed: {stderr}");
    let (success, blocker_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Resolve mission blocker",
            "--issue-type",
            "task",
        ],
    );
    assert!(success, "blocker issue create failed: {stderr}");
    assert!(blocker_out.contains("Created issue atelier-"));
    let blocker_id = issue_id_by_title(dir.path(), "Resolve mission blocker");
    let blocker_id = blocker_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-blocker", mission_id, blocker_id],
    );
    assert!(success, "mission-blocker link failed: {stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check failed: {stderr}");

    let mission_path = dir
        .path()
        .join(".atelier")
        .join("missions")
        .join(format!("{mission_id}.md"));
    let mission_markdown = std::fs::read_to_string(&mission_path).unwrap();
    assert!(mission_markdown.contains("schema: \"atelier.mission\""));
    assert!(mission_markdown.contains("schema_version: 1"));
    assert!(!mission_markdown.contains("\ndata: "));
    assert!(mission_markdown.contains("labels:\n- \"mission\"\n"));
    assert!(mission_markdown.contains("## Intent\n\nUpdated mission body"));
    assert!(mission_markdown.contains("## Constraints\n\n- Keep issues accountable"));
    assert!(mission_markdown.contains("## Risks\n\n- Projection drift"));
    assert!(mission_markdown.contains("## Validation\n\n- Run focused mission checks"));
    assert!(mission_markdown.contains("relationships:"));
    assert!(!mission_markdown.contains("  attachments:\n  - kind: \"issue\""));
    assert!(mission_markdown.contains(&format!(
        "  - kind: \"issue\"\n    id: \"{blocker_id}\"\n    type: \"blocked_by\""
    )));
    assert!(mission_markdown.contains(&format!(
        "  - kind: \"issue\"\n    id: \"{issue_id}\"\n    type: \"advances\""
    )));

    let plan_path = dir
        .path()
        .join(".atelier")
        .join("plans")
        .join(format!("{plan_id}.md"));
    let plan_markdown = std::fs::read_to_string(&plan_path).unwrap();
    assert!(plan_markdown.contains("schema: \"atelier.plan\""));
    assert!(plan_markdown.contains("Do the thing, then verify the projection."));
    assert!(plan_markdown.contains("\\\"revision\\\":2"));
    assert!(plan_markdown.contains(&format!("id: \"{mission_id}\"")));

    let evidence_path = dir
        .path()
        .join(".atelier")
        .join("evidence")
        .join(format!("{evidence_id}.md"));
    let evidence_markdown = std::fs::read_to_string(&evidence_path).unwrap();
    assert!(evidence_markdown.contains("schema: \"atelier.evidence\""));
    assert!(evidence_markdown.contains(&format!("id: \"{mission_id}\"")));

    std::fs::remove_file(dir.path().join(".atelier/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, validate_out, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "mission",
            mission_id,
            "--validator",
            "durable_state_current",
            "--validator",
            "evidence_attached",
        ],
    );
    assert!(success, "workflow validate failed: {stderr}");
    assert!(validate_out.contains("pass  durable_state_current"));
    assert!(validate_out.contains("pass  evidence_attached"));

    let (success, validate_human, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "mission",
            mission_id,
            "--validator",
            "durable_state_current",
            "--validator",
            "evidence_attached",
        ],
    );
    assert!(success, "human workflow validate failed: {stderr}");
    assert!(validate_human.contains("Workflow Validation: mission"));
    assert!(validate_human.contains("Transition: close"));
    assert!(validate_human.contains("Results"));
    assert!(validate_human.contains("pass  durable_state_current"));
    assert!(validate_human.contains("Reason: canonical export is current"));
    assert!(!validate_human.contains("pass durable_state_current:"));

    let (success, view_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(view_out.contains("Records: plans=1 milestones=0 evidence=1"));
    assert!(view_out.contains("Work: ready=1 blocked=0 done=0 backlog=0"));
    assert!(view_out.contains(&blocker_id));

    let (success, show_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(show_out.contains("Plans"));
    assert!(show_out.contains("Evidence"));
    assert!(show_out.contains("Mission Blockers"));

    let (success, human_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "human mission show failed: {stderr}");
    assert!(human_out.contains(&format!("Mission {mission_id} [ready] - Ship records")));
    assert!(human_out.contains("Constraints"));
    assert!(human_out.contains("Keep issues accountable"));
    assert!(human_out.contains("Progress"));
    assert!(human_out.contains("Records: plans=1 milestones=0 evidence=1"));
    assert!(human_out.contains("Work: ready=1 blocked=0 done=0 backlog=0"));
    assert!(human_out.contains("Plans"));
    assert!(human_out.contains("Execution plan"));
    assert!(human_out.contains("Evidence"));
    assert!(human_out.contains("cargo test passed"));
    assert!(human_out.contains("Mission Blockers"));
    assert!(human_out.contains("Resolve mission blocker"));
    assert!(human_out.contains("(open blocker)"));
    assert!(human_out.contains("Linked Work"));
    assert!(human_out.contains("Ready (1)"));
    assert!(human_out.contains("Wire mission work"));
    assert!(human_out.contains("Evidence Gaps"));
    assert!(human_out.contains("(none)"));
    assert!(human_out.contains("Next Commands"));
    assert!(human_out.contains("atelier mission status"));

    let (success, plan_show, stderr) = run_atelier(dir.path(), &["plan", "show", plan_id]);
    assert!(success, "human plan show failed: {stderr}");
    assert!(plan_show.contains(&format!("{plan_id} [plan] open - Execution plan")));
    assert!(plan_show.contains("Revision: 2"));
    assert!(plan_show.contains("Body"));
    assert!(plan_show.contains("Do the thing, then verify the projection."));
    assert!(plan_show.contains("Links:"));

    let (success, plan_list, stderr) = run_atelier(dir.path(), &["plan", "list"]);
    assert!(success, "human plan list failed: {stderr}");
    assert!(plan_list.contains("Plans"));
    assert!(plan_list.contains("1 total"));
    assert!(plan_list.contains("Execution plan"));

    let (success, evidence_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", evidence_id]);
    assert!(success, "human evidence show failed: {stderr}");
    assert!(evidence_show.contains(&format!(
        "{evidence_id} [evidence] pass - cargo test passed"
    )));
    assert!(evidence_show.contains("Result:"));
    assert!(evidence_show.contains("Kind:"));
    assert!(evidence_show.contains("Summary"));

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "human evidence list failed: {stderr}");
    assert!(evidence_list.contains("Evidence"));
    assert!(evidence_list.contains("1 total"));
    assert!(evidence_list.contains("cargo test passed"));
}

#[test]
fn test_mission_relationship_filtering_keeps_supporting_records_out_of_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Filtered mission"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Filtered mission");
    let mission_id = mission_id.as_str();

    let (success, work_out, stderr) = run_atelier(dir.path(), &["issue", "create", "Counted work"]);
    assert!(success, "work issue create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Counted work");
    let work_id = work_id.as_str();

    let (success, support_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Supporting reference"]);
    assert!(success, "support issue create failed: {stderr}");
    assert!(support_out.contains("Created issue atelier-"));
    let support_id = issue_id_by_title(dir.path(), "Supporting reference");
    let support_id = support_id.as_str();

    let (success, blocker_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Direct blocker"]);
    assert!(success, "blocker issue create failed: {stderr}");
    assert!(blocker_out.contains("Created issue atelier-"));
    let blocker_id = issue_id_by_title(dir.path(), "Direct blocker");
    let blocker_id = blocker_id.as_str();

    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, work_id]);
    assert!(success, "mission add-work failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-blocker", mission_id, blocker_id],
    );
    assert!(success, "mission add-blocker failed: {stderr}");

    let mission_path = dir
        .path()
        .join(".atelier")
        .join("missions")
        .join(format!("{mission_id}.md"));
    let mission_markdown = std::fs::read_to_string(&mission_path).unwrap();
    std::fs::write(
        &mission_path,
        mission_markdown.replace(
            "schema: \"atelier.mission\"",
            &format!(
                "  - kind: \"issue\"\n    id: \"{support_id}\"\n    type: \"related\"\nschema: \"atelier.mission\""
            ),
        ),
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(
        success,
        "rebuild after supporting relation edit failed: {stderr}"
    );

    let (success, show_out, stderr) = run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(success, "mission show failed: {stderr}");
    assert!(show_out.contains("Work: ready=1 blocked=0 done=0 backlog=0"));
    assert!(show_out.contains("Mission Blockers: 1"));
    assert!(show_out.contains("Linked Work"));
    assert!(show_out.contains("Counted work"));
    assert!(show_out.contains("Supporting Records"));
    assert!(show_out.contains("Supporting reference (related)"));

    let linked_work = show_out
        .split("Linked Work")
        .nth(1)
        .and_then(|text| text.split("Supporting Records").next())
        .unwrap_or("");
    assert!(
        !linked_work.contains("Supporting reference"),
        "supporting relation was rendered as linked work:\n{show_out}"
    );

    let (success, status_out, stderr) = run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Total: 1 ready"));
    assert!(status_out.contains("Mission blockers: 1 open"));
}

#[test]
fn test_evidence_capture_records_command_metadata_and_attaches_targets() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Capture issue", "--issue-type", "task"],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Capture issue");
    let issue_id = issue_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Capture epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Capture epic");
    let epic_id = epic_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Capture mission"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Capture mission");
    let mission_id = mission_id.as_str();

    let (success, issue_capture, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "capture",
            "--kind",
            "validation",
            "--result",
            "pass",
            "--summary",
            "issue command proof",
            "--target-kind",
            "issue",
            "--target-id",
            issue_id,
            "--",
            "sh",
            "-c",
            "printf 'pass stdout\\n'; printf 'pass stderr\\n' >&2",
        ],
    );
    assert!(success, "issue capture failed: {stderr}");
    assert!(issue_capture.contains("[evidence] pass - issue command proof"));
    assert!(issue_capture.contains("Command:     sh -c"));
    assert!(issue_capture.contains("Exit Status: 0"));
    assert!(issue_capture.contains(&format!("Target:      issue/{issue_id} (validates)")));
    assert!(issue_capture.contains("Captured:"));
    assert!(issue_capture.contains("pass stdout"));
    assert!(issue_capture.contains("pass stderr"));
    let issue_evidence_id = record_id_by_title(dir.path(), "evidence", "issue command proof");

    let (success, issue_validate, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "issue",
            issue_id,
            "--validator",
            "evidence_attached",
        ],
    );
    assert!(success, "issue workflow validate failed: {stderr}");
    assert!(issue_validate.contains("pass  evidence_attached"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "capture",
            "--kind",
            "validation",
            "--result",
            "fail",
            "--summary",
            "epic failing command proof",
            "--target-kind",
            "epic",
            "--target-id",
            epic_id,
            "--",
            "sh",
            "-c",
            "printf 'failing stdout\\n'; printf 'failing stderr\\n' >&2; exit 7",
        ],
    );
    assert!(success, "epic capture failed: {stderr}");
    let epic_evidence_id = record_id_by_title(dir.path(), "evidence", "epic failing command proof");
    let (success, epic_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &epic_evidence_id]);
    assert!(success, "epic evidence show failed: {stderr}");
    assert!(epic_show.contains("Result:      fail"));
    assert!(epic_show.contains("Exit Status: 7"));
    assert!(epic_show.contains(&format!("Target:      epic/{epic_id} (validates)")));
    assert!(epic_show.contains("failing stdout"));
    assert!(epic_show.contains("failing stderr"));

    let (success, epic_validate, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "issue",
            epic_id,
            "--validator",
            "evidence_attached",
        ],
    );
    assert!(success, "epic workflow validate failed: {stderr}");
    assert!(epic_validate.contains("pass  evidence_attached"));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "add",
            "--kind",
            "validation",
            "--result",
            "pass",
            "manual epic attach proof",
        ],
    );
    assert!(success, "manual evidence add failed: {stderr}");
    let manual_epic_evidence_id =
        record_id_by_title(dir.path(), "evidence", "manual epic attach proof");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "attach",
            &manual_epic_evidence_id,
            "epic",
            epic_id,
        ],
    );
    assert!(success, "manual epic evidence attach failed: {stderr}");
    let (success, manual_epic_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &manual_epic_evidence_id]);
    assert!(success, "manual epic evidence show failed: {stderr}");
    assert!(manual_epic_show.contains(&format!("Target:      epic/{epic_id} (validates)")));

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "capture",
            "--kind",
            "validation",
            "--result",
            "blocked",
            "--summary",
            "mission blocked command proof",
            "--target-kind",
            "mission",
            "--target-id",
            mission_id,
            "--",
            "sh",
            "-c",
            "i=0; while [ $i -lt 350 ]; do printf 'blocked-line-%03d\\n' \"$i\"; i=$((i + 1)); done; printf 'blocked stderr\\n' >&2; exit 2",
        ],
    );
    assert!(success, "mission blocked capture failed: {stderr}");
    let mission_evidence_id =
        record_id_by_title(dir.path(), "evidence", "mission blocked command proof");
    let (success, mission_show, stderr) =
        run_atelier(dir.path(), &["evidence", "show", &mission_evidence_id]);
    assert!(success, "mission evidence show failed: {stderr}");
    assert!(mission_show.contains("Result:      blocked"));
    assert!(mission_show.contains("Exit Status: 2"));
    assert!(mission_show.contains(&format!("Target:      mission/{mission_id} (validates)")));
    assert!(mission_show.contains("blocked-line-000"));
    assert!(!mission_show.contains("blocked-line-349"));
    assert!(mission_show.contains("Stdout: "));
    assert!(mission_show.contains("truncated: yes"));

    let (success, mission_validate, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "mission",
            mission_id,
            "--validator",
            "evidence_attached",
        ],
    );
    assert!(success, "mission workflow validate failed: {stderr}");
    assert!(mission_validate.contains("pass  evidence_attached"));

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "evidence list failed: {stderr}");
    assert!(evidence_list.contains(&issue_evidence_id));
    assert!(evidence_list.contains("exit=0"));
    assert!(evidence_list.contains(&format!("target=issue/{issue_id}")));
    assert!(evidence_list.contains("command=sh -c"));
    assert!(evidence_list.contains(&epic_evidence_id));
    assert!(evidence_list.contains("exit=7"));
    assert!(evidence_list.contains(&format!("target=epic/{epic_id}")));
    assert!(evidence_list.contains(&manual_epic_evidence_id));
    assert!(evidence_list.contains(&mission_evidence_id));
    assert!(evidence_list.contains("exit=2"));
    assert!(evidence_list.contains(&format!("target=mission/{mission_id}")));
}

#[test]
fn test_evidence_capture_rejects_failed_commands_as_pass_proof() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "capture",
            "--kind",
            "validation",
            "--result",
            "pass",
            "--summary",
            "bad pass proof",
            "--",
            "sh",
            "-c",
            "printf 'not a pass\\n'; exit 3",
        ],
    );
    assert!(!success, "nonzero command cannot become pass evidence");
    assert!(stderr.contains("cannot record pass evidence"));

    let (success, evidence_list, stderr) = run_atelier(dir.path(), &["evidence", "list"]);
    assert!(success, "evidence list failed: {stderr}");
    assert!(evidence_list.contains("(none)"));
}

#[test]
fn test_workflow_validate_fails_without_required_evidence() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Needs evidence"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Needs evidence");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "mission",
            &mission_id,
            "--validator",
            "evidence_attached",
        ],
    );
    assert!(!success, "workflow validate should fail without evidence");
    assert!(stdout.contains("fail  evidence_attached"));
    assert!(stdout.contains("no validating evidence link found"));
    assert!(stderr.contains("workflow validation failed"));
}

#[test]
fn test_workflow_validate_reports_ignored_tests_without_owner() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    std::fs::create_dir_all(dir.path().join("tests")).unwrap();
    std::fs::write(
        dir.path().join("tests/ignored_inventory.rs"),
        ignored_test_source(
            "ignore = \"reason: product parser migration; product: yes\"",
            "hidden_product_behavior",
        ),
    )
    .unwrap();

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Ignored inventory"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Ignored inventory");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "mission",
            &mission_id,
            "--validator",
            "ignored_tests_reviewed",
        ],
    );

    assert!(!success, "ignored test inventory should fail");
    assert!(stdout.contains("fail  ignored_tests_reviewed"));
    assert!(stdout.contains("hidden_product_behavior"));
    assert!(stdout.contains("missing owner or linked issue"));
    assert!(stdout.contains("product=yes"));
    assert!(stderr.contains("workflow validation failed"));
}

#[test]
fn test_mission_status_shows_ignored_product_behavior_closeout_blocker() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &["mission", "create", "Ignored blocker mission"],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Ignored blocker mission");
    let mission_id = mission_id.as_str();

    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Finished behavior work"]);
    assert!(success, "work create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Finished behavior work");
    let work_id = work_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, work_id]);
    assert!(success, "mission add-work failed: {stderr}");
    attach_issue_pass_evidence(dir.path(), work_id);
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "close", work_id, "--reason", "done"]);
    assert!(success, "close work failed: {stderr}");

    let (success, followup_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Resolve ignored behavior test"],
    );
    assert!(success, "follow-up create failed: {stderr}");
    assert!(followup_out.contains("Created issue atelier-"));
    let followup_id = issue_id_by_title(dir.path(), "Resolve ignored behavior test");
    let followup_id = followup_id.as_str();

    std::fs::create_dir_all(dir.path().join("tests")).unwrap();
    std::fs::write(
        dir.path().join("tests/product_gap.rs"),
        ignored_test_source(
            &format!(
                "ignore = \"reason: product behavior pending migration; issue: {followup_id}; product: yes\""
            ),
            "ignored_product_closeout_gap",
        ),
    )
    .unwrap();

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "add",
            "--kind",
            "validation",
            "--result",
            "pass",
            "ignored blocker evidence",
        ],
    );
    assert!(success, "evidence add failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - ignored blocker evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "ignored blocker evidence");
    let evidence_id = evidence_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", evidence_id, "mission", mission_id],
    );
    assert!(success, "evidence attach failed: {stderr}");
    commit_all(dir.path(), "ignored test closeout blocker");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Closeout: blocked"));
    assert!(status_out.contains("Ignored Test Review: needed"));
    assert!(status_out.contains("Advanced Validator Detail"));
    assert!(status_out.contains("fail  ignored_tests_reviewed"));
    assert!(status_out.contains("ignored_product_closeout_gap"));
    assert!(status_out.contains(followup_id));
    assert!(status_out.contains("ignored product-behavior test is still blocking closeout"));
    assert!(!status_out.contains(&format!(
        "atelier mission update {mission_id} --status closed"
    )));
}

#[test]
fn test_workflow_validate_reports_agent_factory_command_drift() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_valid_command_guidance(dir.path());
    fs::write(
        dir.path().join("AGENTFACTORY.md"),
        "# Agent Factory Binding\n\n- `atelier status`\n- `atelier mission view <id>`\n",
    )
    .unwrap();

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Agent Factory drift"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Agent Factory drift");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "mission",
            &mission_id,
            "--validator",
            "command_surface_current",
        ],
    );

    assert!(!success, "stale Agent Factory guidance should fail");
    assert!(stdout.contains("fail  command_surface_current"));
    assert!(stdout.contains("AGENTFACTORY.md"));
    assert!(stdout.contains("atelier mission view"));
    assert!(stderr.contains("workflow validation failed"));
}

#[test]
fn test_workflow_validate_reports_docs_help_root_surface_drift() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_valid_command_guidance(dir.path());
    let stale_doc = valid_command_surface_doc().replace("- `atelier diagnostics slow`\n", "");
    fs::write(dir.path().join("docs/product/cli-surface.md"), stale_doc).unwrap();

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Docs help drift"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Docs help drift");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "mission",
            &mission_id,
            "--validator",
            "command_surface_current",
        ],
    );

    assert!(!success, "docs/help drift should fail");
    assert!(stdout.contains("fail  command_surface_current"));
    assert!(stdout.contains("docs/product/cli-surface.md"));
    assert!(stdout.contains("help command `atelier diagnostics`"));
    assert!(stderr.contains("workflow validation failed"));
}

#[test]
fn test_mission_closeout_blocks_undeferred_obsolete_command_test() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    write_valid_command_guidance(dir.path());

    fs::create_dir_all(dir.path().join("tests")).unwrap();
    fs::write(
        dir.path().join("tests/legacy_session.rs"),
        concat!(
            "#[test]\n",
            "fn legacy_session_still_works() {\n",
            "    let (success, _, _) = run_atelier(dir.path(), &[\"session\", \"start\"]);\n",
            "    assert!(success);\n",
            "}\n"
        ),
    )
    .unwrap();

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Stale test closeout"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Stale test closeout");

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "add",
            "--kind",
            "validation",
            "--result",
            "pass",
            "stale test evidence",
        ],
    );
    assert!(success, "evidence add failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - stale test evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "stale test evidence");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", &evidence_id, "mission", &mission_id],
    );
    assert!(success, "evidence attach failed: {stderr}");
    commit_all(dir.path(), "stale test closeout baseline");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", &mission_id, "--status", "closed"],
    );

    assert!(
        !success,
        "mission closeout should block undeferred obsolete-command tests"
    );
    assert!(stdout.contains("Mission closeout blocked"));
    assert!(stdout.contains("validator command_surface_current failed"));
    assert!(stdout.contains("tests/legacy_session.rs"));
    assert!(stdout.contains("legacy_session_still_works"));
    assert!(stdout.contains("atelier session start"));
    assert!(stderr.contains("mission closeout blocked"));
}

#[test]
fn test_mission_audit_reports_missing_partial_and_ready_proof() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "create",
            "Audit proof",
            "--validation",
            "Mission audit validates authored outcomes.",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Audit proof");
    let mission_id = mission_id.as_str();

    let (success, missing_out, stderr) = run_atelier(dir.path(), &["mission", "audit", mission_id]);
    assert!(!success, "audit without work should fail");
    assert!(missing_out.contains("Mission Contract Audit"));
    assert!(missing_out.contains("[fail]"));
    assert!(missing_out.contains("No linked mission work exists"));
    assert!(stderr.contains("mission contract audit failed"));

    let epic_body = "## Description\n\nAudit epic body.\n\n## Outcome\n\n- Linked epic outcome is proven.\n\n## Evidence\n\n- Attached validation evidence proves the epic outcome.";
    let (success, epic_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Audit epic",
            "--issue-type",
            "epic",
            "--description",
            epic_body,
        ],
    );
    assert!(success, "epic create failed: {stderr}");
    assert!(epic_out.contains("Created issue atelier-"));
    let epic_id = issue_id_by_title(dir.path(), "Audit epic");
    let epic_id = epic_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, epic_id]);
    assert!(success, "mission add work failed: {stderr}");
    attach_issue_pass_evidence(dir.path(), epic_id);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", epic_id, "--reason", "epic proof"],
    );
    assert!(success, "epic close failed: {stderr}");

    let (success, partial_out, stderr) = run_atelier(dir.path(), &["mission", "audit", mission_id]);
    assert!(!success, "audit without mission evidence should fail");
    assert!(partial_out.contains("pass"));
    assert!(partial_out.contains(epic_id));
    assert!(partial_out.contains("Linked epic outcome is proven."));
    assert!(partial_out.contains("No validation evidence is attached to the mission."));
    assert!(stderr.contains("mission contract audit failed"));

    attach_pass_evidence(
        dir.path(),
        "mission",
        mission_id,
        "mission audit proof evidence",
    );
    let (success, ready_out, stderr) = run_atelier(dir.path(), &["mission", "audit", mission_id]);
    assert!(success, "ready audit should pass: {stderr}");
    assert!(ready_out.contains("[pass]"));
    assert!(ready_out.contains("Summary: 2 pass, 0 fail, 2 total"));
    assert!(ready_out.contains(&format!(
        "Close mission when other gates pass: atelier mission update {mission_id} --status closed"
    )));
}

#[test]
fn test_mission_closeout_uses_contract_audit() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "create",
            "Audit closeout",
            "--validation",
            "Authored validation must map to linked work.",
        ],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Audit closeout");
    let mission_id = mission_id.as_str();
    attach_pass_evidence(
        dir.path(),
        "mission",
        mission_id,
        "mission evidence without work",
    );
    commit_all(dir.path(), "audit closeout baseline");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", mission_id, "--status", "closed"],
    );
    assert!(
        !success,
        "mission closeout should fail when contract audit fails"
    );
    assert!(stdout.contains("Mission closeout blocked"));
    assert!(stdout.contains("contract audit failed"));
    assert!(stdout.contains(&format!("atelier mission audit {mission_id}")));
    assert!(stderr.contains("mission closeout blocked"));
}

#[test]
fn test_epic_closeout_requires_closed_children_and_parent_evidence() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let epic_body = "## Description\n\nEpic body.\n\n## Outcome\n\nEpic parent outcome is independently proven.\n\n## Evidence\n\n- Attached validation evidence proves the epic parent outcome.";
    let (success, epic_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Closeout epic",
            "--issue-type",
            "epic",
            "--description",
            epic_body,
        ],
    );
    assert!(success, "epic create failed: {stderr}");
    assert!(epic_out.contains("Created issue atelier-"));
    let epic_id = issue_id_by_title(dir.path(), "Closeout epic");
    let epic_id = epic_id.as_str();

    let child_body = "## Description\n\nChild body.\n\n## Outcome\n\nChild outcome is complete.\n\n## Evidence\n\n- Attached validation evidence proves the child outcome.";
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Open child",
            "--parent",
            epic_id,
            "--description",
            child_body,
        ],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Open child");
    let child_id = child_id.as_str();

    attach_issue_pass_evidence(dir.path(), epic_id);
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", epic_id, "--reason", "parent done"],
    );
    assert!(!success, "open child should block epic closeout");
    assert!(stdout.is_empty());
    assert!(stderr.contains("child work is still open"));
    assert!(stderr.contains(child_id));

    attach_issue_pass_evidence(dir.path(), child_id);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", child_id, "--reason", "child done"],
    );
    assert!(success, "child close failed: {stderr}");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", epic_id, "--reason", "parent done"],
    );
    assert!(
        success,
        "epic close after child and parent proof failed: {stderr}"
    );
}

#[test]
fn test_closed_children_alone_do_not_close_epic_parent() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, epic_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Unproven epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    assert!(epic_out.contains("Created issue atelier-"));
    let epic_id = issue_id_by_title(dir.path(), "Unproven epic");
    let epic_id = epic_id.as_str();

    let child_body = "## Description\n\nChild body.\n\n## Outcome\n\nChild is done.\n\n## Evidence\n\n- Attached validation evidence proves the child.";
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Closed child",
            "--parent",
            epic_id,
            "--description",
            child_body,
        ],
    );
    assert!(success, "child create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Closed child");
    let child_id = child_id.as_str();
    attach_issue_pass_evidence(dir.path(), child_id);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", child_id, "--reason", "child done"],
    );
    assert!(success, "child close failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", epic_id, "--reason", "parent done"],
    );
    assert!(
        !success,
        "missing parent evidence should block epic closeout"
    );
    assert!(stdout.is_empty());
    assert!(stderr.contains("no validating evidence is linked"));
    assert!(stderr.contains("atelier evidence capture"));
}

#[test]
fn test_workflow_validate_can_use_parsed_issue_sections() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Parsed section workflow"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Parsed section workflow");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "issue",
            &issue_id,
            "--validator",
            "issue_sections_parseable",
        ],
    );
    assert!(success, "workflow validate failed: {stderr}");
    assert!(stdout.contains("pass  issue_sections_parseable"));
    assert!(stdout.contains("parsed required sections Description, Outcome, Evidence"));

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Section mission"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Section mission");
    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", &mission_id, &issue_id]);
    assert!(success, "mission add-work failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "mission",
            &mission_id,
            "--validator",
            "issue_sections_parseable",
        ],
    );
    assert!(success, "mission workflow validate failed: {stderr}");
    assert!(stdout.contains("pass  issue_sections_parseable"));
    assert!(stdout.contains("for 1 issue(s)"));
}

#[test]
fn test_workflow_validate_defaults_are_target_and_transition_aware() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let issue_body = "## Description\n\nDefault validator issue body.\n\n## Outcome\n\nWorkflow validation reports target-aware issue close blockers.\n\n## Evidence\n\n- `atelier workflow validate issue <id>` reports missing evidence.";
    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Default close validators",
            "--description",
            issue_body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Default close validators");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "issue",
            &issue_id,
            "--transition",
            "close",
        ],
    );
    assert!(!success, "issue close defaults should require evidence");
    assert!(stdout.contains("pass  durable_state_current"));
    assert!(stdout.contains("pass  issue_sections_parseable"));
    assert!(stdout.contains("pass  no_open_blockers"));
    assert!(stdout.contains("fail  evidence_attached"));
    assert!(stdout.contains("no validating evidence link found"));
    assert!(stderr.contains("workflow validation failed"));

    let (success, blocker_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Default start blocker",
            "--description",
            "## Description\n\nBlocker body.\n\n## Outcome\n\nBlocker remains open.\n\n## Evidence\n\n- `atelier workflow validate issue <id> --transition start` reports the open blocker.",
        ],
    );
    assert!(success, "blocker create failed: {stderr}");
    assert!(blocker_out.contains("Created issue atelier-"));
    let blocker_id = issue_id_by_title(dir.path(), "Default start blocker");
    let (success, _, stderr) = run_atelier(dir.path(), &["dep", "add", &issue_id, &blocker_id]);
    assert!(success, "dep add failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "issue",
            &issue_id,
            "--transition",
            "start",
        ],
    );
    assert!(!success, "issue start defaults should report blockers");
    assert!(stdout.contains("fail  no_open_blockers"));
    assert!(stdout.contains(&blocker_id));
    assert!(
        !stdout.contains("evidence_attached"),
        "start defaults must not require validation evidence:\n{stdout}"
    );
    assert!(stderr.contains("workflow validation failed"));

    let (success, mission_out, stderr) = run_atelier(
        dir.path(),
        &["mission", "create", "Default mission validators"],
    );
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Default mission validators");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", &mission_id, &issue_id]);
    assert!(success, "mission add-work failed: {stderr}");
    commit_all(dir.path(), "default validator setup");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "mission",
            &mission_id,
            "--transition",
            "close",
        ],
    );
    assert!(
        !success,
        "mission close defaults should report closeout blockers"
    );
    for validator in [
        "durable_state_current",
        "issue_sections_parseable",
        "no_open_work",
        "evidence_attached",
        "no_open_blockers",
        "no_blocking_lints",
        "ignored_tests_reviewed",
        "git_worktree_clean",
    ] {
        assert!(
            stdout.contains(validator),
            "mission default output missing {validator}:\n{stdout}"
        );
    }
    assert!(stdout.contains("open linked work"));
    assert!(stdout.contains(&issue_id));
    assert!(stdout.contains("no validating evidence link found"));
    assert!(stderr.contains("workflow validation failed"));
}

#[test]
fn test_workflow_validate_defaults_for_evidence_and_tracker_health_targets() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "add",
            "--kind",
            "validation",
            "--result",
            "pass",
            "target-aware evidence proof",
        ],
    );
    assert!(success, "evidence add failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - target-aware evidence proof"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "target-aware evidence proof");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "evidence",
            &evidence_id,
            "--transition",
            "attach",
        ],
    );
    assert!(success, "evidence target defaults failed: {stderr}");
    assert!(stdout.contains("pass  durable_state_current"));
    assert!(
        !stdout.contains("issue_sections_parseable"),
        "evidence defaults must not run issue validators:\n{stdout}"
    );

    commit_all(dir.path(), "tracker health setup");
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "tracker",
            "health",
            "--transition",
            "health",
        ],
    );
    assert!(success, "tracker health defaults failed: {stderr}");
    for validator in [
        "durable_state_current",
        "no_blocking_lints",
        "ignored_tests_reviewed",
        "git_worktree_clean",
    ] {
        assert!(
            stdout.contains(&format!("pass  {validator}")),
            "tracker health output missing passing {validator}:\n{stdout}"
        );
    }
    assert!(
        !stdout.contains("issue_sections_parseable"),
        "tracker health defaults must not run issue validators:\n{stdout}"
    );
}

#[test]
fn test_lint_rejects_missing_required_issue_section() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Missing outcome lint"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Missing outcome lint");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(&issue_path, remove_issue_section(&markdown, "Outcome")).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(!success, "lint should fail for missing Outcome");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Outcome")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing structural diagnostic in stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stderr.contains("Lint failed"));
}

#[test]
fn test_lint_rejects_empty_required_issue_section() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Empty outcome lint"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Empty outcome lint");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    let invalid = markdown.replace("## Outcome\n\nOutcome was not specified.", "## Outcome\n\n");
    std::fs::write(&issue_path, invalid).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(!success, "lint should fail for empty Outcome");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Outcome")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing structural diagnostic in stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[test]
fn test_lint_rejects_missing_evidence_section() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Missing evidence lint"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Missing evidence lint");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(&issue_path, remove_issue_section(&markdown, "Evidence")).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(!success, "lint should fail for missing Evidence");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Evidence")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing Evidence diagnostic in stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stderr.contains("Lint failed"));
}

#[test]
fn test_lint_rejects_empty_evidence_section() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Empty evidence lint"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Empty evidence lint");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    let invalid = markdown.replace(
        "## Evidence\n\nEvidence was not specified.",
        "## Evidence\n\n",
    );
    std::fs::write(&issue_path, invalid).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(!success, "lint should fail for empty Evidence");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Evidence")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing Evidence diagnostic in stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[test]
fn test_lint_rejects_vague_evidence_even_when_notes_name_a_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let body = "## Description\n\nDescription\n\n## Outcome\n\nLint flags vague Evidence entries.\n\n## Evidence\n\n- Validation complete.\n\n## Notes\n\n- `cargo test --test cli_integration vague_evidence` passes.";

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Vague evidence lint",
            "--description",
            body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Vague evidence lint");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(!success, "lint should fail for vague Evidence");
    let transcript = format!("{stdout}\n{stderr}");
    for needle in [
        &issue_id,
        "section Evidence",
        ".atelier/issues/",
        "observable proof target",
        "command, transcript, evidence record, test, review artifact, file change, or manual check",
    ] {
        assert!(
            transcript.contains(needle),
            "vague Evidence diagnostic missing {needle:?}: {transcript}"
        );
    }
}

#[test]
fn test_lint_accepts_concrete_evidence_without_optional_notes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let body = "## Description\n\nDescription\n\n## Outcome\n\nLint accepts concrete Evidence entries without optional Notes.\n\n## Evidence\n\n- `cargo test --test cli_integration concrete_evidence` passes.\n- Manual check confirms the lint diagnostic names the issue, section, and path.";

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Concrete evidence lint",
            "--description",
            body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Concrete evidence lint");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(success, "lint should accept concrete Evidence: {stderr}");
    assert!(stdout.contains("Lint passed."));
}

#[test]
fn test_lint_rejects_duplicate_recognized_issue_heading() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Duplicate outcome lint"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Duplicate outcome lint");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    let invalid = markdown.replace(
        "## Evidence",
        "## Outcome\n\nSecond outcome should be rejected.\n\n## Evidence",
    );
    std::fs::write(&issue_path, invalid).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(!success, "lint should fail for duplicate Outcome");
    assert!(
        stdout.contains(&format!("issue {issue_id}"))
            && stdout.contains("section Outcome")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing structural diagnostic in stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[test]
fn test_work_start_refuses_structurally_invalid_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Invalid work start"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Invalid work start");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(&issue_path, remove_issue_section(&markdown, "Outcome")).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["work", "start", &issue_id]);
    assert!(!success, "work start should refuse invalid issue");
    assert!(
        stderr.contains(&format!("issue {issue_id}"))
            && stderr.contains("section Outcome")
            && stderr.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing refusal diagnostic, stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[test]
fn test_issue_closeout_refuses_structurally_invalid_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Invalid closeout"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Invalid closeout");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(&issue_path, remove_issue_section(&markdown, "Outcome")).unwrap();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &issue_id, "--reason", "done"],
    );
    assert!(!success, "issue close should refuse invalid issue");
    assert!(
        stderr.contains(&format!("issue {issue_id}"))
            && stderr.contains("section Outcome")
            && stderr.contains(&format!(".atelier/issues/{issue_id}.md")),
        "missing closeout diagnostic, stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[test]
fn test_git_worktree_clean_validator_fails_on_tracked_changes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Tracked dirty"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Tracked dirty");
    commit_all(dir.path(), "baseline");

    let mission_path = dir
        .path()
        .join(".atelier")
        .join("missions")
        .join(format!("{mission_id}.md"));
    let mission_markdown = std::fs::read_to_string(&mission_path).unwrap();
    std::fs::write(
        &mission_path,
        mission_markdown.replace("Tracked dirty", "Tracked dirty changed"),
    )
    .unwrap();
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "mission",
            &mission_id,
            "--validator",
            "git_worktree_clean",
        ],
    );
    assert!(
        !success,
        "git_worktree_clean should fail on tracked changes"
    );
    assert!(stdout.contains("fail  git_worktree_clean"));
    assert!(stdout.contains(&mission_id));
    assert!(stderr.contains("workflow validation failed"));
}

#[test]
fn test_git_worktree_clean_validator_fails_on_untracked_changes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Untracked dirty"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Untracked dirty");
    commit_all(dir.path(), "baseline");

    std::fs::write(dir.path().join("untracked.txt"), "dirty").unwrap();
    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "mission",
            &mission_id,
            "--validator",
            "git_worktree_clean",
        ],
    );
    assert!(
        !success,
        "git_worktree_clean should fail on untracked changes"
    );
    assert!(stdout.contains("fail  git_worktree_clean"));
    assert!(stdout.contains("untracked.txt"));
    assert!(stderr.contains("workflow validation failed"));
}

#[test]
fn test_mission_closeout_enforces_gates_and_reopen_skips_close_validators() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Strict closeout"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Strict closeout");

    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Closeout work"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Closeout work");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", &mission_id, &work_id]);
    assert!(success, "mission add work failed: {stderr}");

    let (success, closeout_blocked_out, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", &mission_id, "--status", "closed"],
    );
    assert!(
        !success,
        "mission close should fail with open work and no evidence"
    );
    assert!(closeout_blocked_out.contains("Mission closeout blocked"));
    assert!(closeout_blocked_out.contains("open mission work"));
    assert!(closeout_blocked_out.contains("missing mission proof"));
    assert!(closeout_blocked_out.contains("contract audit failed"));
    assert!(stderr.contains("mission closeout blocked"));

    attach_issue_pass_evidence(dir.path(), &work_id);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &work_id, "--reason", "done"],
    );
    assert!(success, "issue close failed: {stderr}");
    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "add",
            "--kind",
            "validation",
            "--result",
            "pass",
            "strict closeout evidence",
        ],
    );
    assert!(success, "evidence add failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - strict closeout evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "strict closeout evidence");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", &evidence_id, "mission", &mission_id],
    );
    assert!(success, "evidence attach failed: {stderr}");
    commit_all(dir.path(), "ready to close");

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", &mission_id, "--status", "closed"],
    );
    assert!(
        success,
        "mission close should succeed after gates pass: {stderr}"
    );
    assert!(close_out.contains("Status: closed"));
    commit_all(dir.path(), "closed mission");

    std::fs::write(dir.path().join("dirty-after-close.txt"), "dirty").unwrap();
    let (success, reopen_out, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", &mission_id, "--status", "ready"],
    );
    assert!(
        success,
        "mission reopen should skip closeout validators: {stderr}"
    );
    assert!(reopen_out.contains("Status: ready"));
}

#[test]
fn test_dirty_worktree_blocks_mission_closeout() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Dirty closeout"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Dirty closeout");
    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "add",
            "--kind",
            "validation",
            "--result",
            "pass",
            "dirty closeout evidence",
        ],
    );
    assert!(success, "evidence add failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - dirty closeout evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "dirty closeout evidence");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", &evidence_id, "mission", &mission_id],
    );
    assert!(success, "evidence attach failed: {stderr}");
    commit_all(dir.path(), "ready except dirty");
    std::fs::write(dir.path().join("untracked-closeout.txt"), "dirty").unwrap();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", &mission_id, "--status", "closed"],
    );
    assert!(!success, "dirty worktree must block mission closeout");
    assert!(stdout.contains("Mission closeout blocked"));
    assert!(stdout.contains("worktree: dirty"));
    assert!(stdout.contains("commit or remove untracked worktree changes"));
    assert!(stdout.contains("untracked-closeout.txt"));
    assert!(stderr.contains("mission closeout blocked"));
}

#[test]
fn test_mission_status_names_concrete_closeout_blockers() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Status blockers"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Status blockers");
    let (success, work_out, stderr) = run_atelier(dir.path(), &["issue", "create", "Still open"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Still open");
    let (success, blocker_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Open blocker"]);
    assert!(success, "blocker create failed: {stderr}");
    assert!(blocker_out.contains("Created issue atelier-"));
    let blocker_id = issue_id_by_title(dir.path(), "Open blocker");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", &mission_id, &work_id]);
    assert!(success, "mission add work failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "block", &work_id, &blocker_id]);
    assert!(success, "issue block failed: {stderr}");
    commit_all(dir.path(), "status baseline");
    std::fs::write(dir.path().join("status-dirty.txt"), "dirty").unwrap();

    let (success, status_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", &mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Closeout Gates"));
    assert!(status_out.contains("Work: open"));
    assert!(status_out.contains(&work_id));
    assert!(status_out.contains("Blockers: open"));
    assert!(status_out.contains(&blocker_id));
    assert!(status_out.contains("Mission Proof: missing"));
    assert!(status_out.contains("Worktree: dirty"));
    assert!(status_out.contains("status-dirty.txt"));
    assert!(status_out.contains("Advanced Validator Detail"));
    assert!(status_out.contains("advanced closeout validator failure"));
}

#[test]
fn test_mission_status_names_stale_and_malformed_record_blockers() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Record health blockers"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Record health blockers");
    let mission_id = mission_id.as_str();
    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Record health work"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Record health work");
    let issue_id = issue_id.as_str();
    let (success, evidence_issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Record evidence work"]);
    assert!(success, "evidence issue create failed: {stderr}");
    assert!(evidence_issue_out.contains("Created issue atelier-"));
    let evidence_issue_id = issue_id_by_title(dir.path(), "Record evidence work");
    let evidence_issue_id = evidence_issue_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, issue_id]);
    assert!(success, "mission add work failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-work", mission_id, evidence_issue_id],
    );
    assert!(success, "mission add evidence work failed: {stderr}");
    commit_all(dir.path(), "record health baseline");

    let issue_path = dir
        .path()
        .join(".atelier")
        .join("issues")
        .join(format!("{issue_id}.md"));
    let evidence_issue_path = dir
        .path()
        .join(".atelier")
        .join("issues")
        .join(format!("{evidence_issue_id}.md"));
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(
        &issue_path,
        markdown.replace("Record health work", "Record health work stale"),
    )
    .unwrap();
    commit_all(dir.path(), "stale record source");

    let (success, stale_status, stderr) =
        run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "stale mission status failed: {stderr}");
    assert!(
        stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "valid stale projection should be named and repaired before mission status:\nstdout:\n{stale_status}\nstderr:\n{stderr}"
    );
    assert!(stale_status.contains("Tracker:  ok"));
    assert!(stale_status.contains("Tracker State: current"));

    let stale_markdown = std::fs::read_to_string(&issue_path).unwrap();
    let malformed = stale_markdown.replace("\n## Outcome\n\nOutcome was not specified.\n", "\n");
    std::fs::write(&issue_path, malformed).unwrap();
    let evidence_markdown = std::fs::read_to_string(&evidence_issue_path).unwrap();
    let malformed_evidence =
        evidence_markdown.replace("\n## Evidence\n\nEvidence was not specified.\n", "\n");
    std::fs::write(&evidence_issue_path, malformed_evidence).unwrap();
    let conn = rusqlite::Connection::open(dir.path().join(".atelier/state.db")).unwrap();
    for (path, id) in [
        (&issue_path, issue_id),
        (&evidence_issue_path, evidence_issue_id),
    ] {
        let metadata = std::fs::metadata(path).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(std::fs::read(path).unwrap());
        let invalid_hash = format!("{:x}", hasher.finalize());
        conn.execute(
            "UPDATE projection_index_sources
             SET size_bytes = ?1, sha256 = ?2
             WHERE path = ?3",
            rusqlite::params![
                i64::try_from(metadata.len()).unwrap(),
                invalid_hash,
                format!("issues/{id}.md")
            ],
        )
        .unwrap();
    }
    commit_all(dir.path(), "malformed record source");

    let (success, malformed_status, stderr) =
        run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "malformed mission status failed: {stderr}");
    assert!(malformed_status.contains("Reliability"));
    assert!(malformed_status.contains("Malformed Work: found"));
    assert!(malformed_status.contains("Missing Outcome Sections: 1 issue(s)"));
    assert!(malformed_status.contains("Missing Evidence Sections: 1 issue(s)"));
    assert!(malformed_status.contains("Linked Issue Records: malformed"));
    assert!(malformed_status.contains("Missing required issue body section 'Outcome'"));
    assert!(malformed_status.contains("Missing required issue body section 'Evidence'"));
    assert!(malformed_status.contains("atelier lint"));
}

#[test]
fn test_orientation_commands_enter_degraded_mode_for_malformed_records() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let valid_body = "## Description\n\nValid orientation body.\n\n## Outcome\n\nValid linked work remains visible during degraded orientation.\n\n## Evidence\n\n- `atelier mission status <id>` lists valid linked work.";
    let malformed_body = "## Description\n\nMalformed orientation body.\n\n## Outcome\n\nMalformed linked work is reported as a degraded blocker.\n\n## Evidence\n\n- `atelier lint <id>` reports the malformed record.";

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Degraded orientation"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Degraded orientation");
    let mission_id = mission_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", mission_id, "--status", "active"],
    );
    assert!(success, "mission activate failed: {stderr}");

    let (success, valid_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Valid degraded work",
            "--description",
            valid_body,
        ],
    );
    assert!(success, "valid issue create failed: {stderr}");
    assert!(valid_out.contains("Created issue atelier-"));
    let valid_id = issue_id_by_title(dir.path(), "Valid degraded work");
    let valid_id = valid_id.as_str();

    let (success, malformed_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Malformed degraded work",
            "--description",
            malformed_body,
        ],
    );
    assert!(success, "malformed issue create failed: {stderr}");
    assert!(malformed_out.contains("Created issue atelier-"));
    let malformed_id = issue_id_by_title(dir.path(), "Malformed degraded work");
    let malformed_id = malformed_id.as_str();

    for issue_id in [valid_id, malformed_id] {
        let (success, _, stderr) =
            run_atelier(dir.path(), &["mission", "add-work", mission_id, issue_id]);
        assert!(success, "mission add work failed for {issue_id}: {stderr}");
    }
    commit_all(dir.path(), "valid degraded orientation baseline");

    let malformed_path = dir
        .path()
        .join(".atelier")
        .join("issues")
        .join(format!("{malformed_id}.md"));
    let markdown = std::fs::read_to_string(&malformed_path).unwrap();
    std::fs::write(&malformed_path, remove_issue_section(&markdown, "Outcome")).unwrap();
    commit_all(dir.path(), "malformed degraded orientation record");

    let (status_success, status_out, status_err) = run_atelier(dir.path(), &["status"]);
    assert!(
        status_success,
        "status should degrade instead of failing: {status_err}"
    );
    assert!(status_out.contains("Atelier Status"));
    assert!(status_out.contains(&format!("Active mission: {mission_id}")));
    assert_degraded_repair_guidance(&status_err, malformed_id);

    let (mission_success, mission_out, mission_err) =
        run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(
        mission_success,
        "mission status should degrade instead of failing: {mission_err}"
    );
    assert!(mission_out.contains("Mission Status"));
    assert!(mission_out.contains(valid_id));
    assert!(mission_out.contains(malformed_id));
    assert!(mission_out.contains("Reliability"));
    assert!(mission_out.contains("Malformed Work: found"));
    assert!(mission_out.contains("Linked Issue Records: malformed"));
    assert!(mission_out.contains("Missing required issue body section 'Outcome'"));
    assert_degraded_repair_guidance(&mission_err, malformed_id);

    let (mission_show_success, mission_show_out, mission_show_err) =
        run_atelier(dir.path(), &["mission", "show", mission_id]);
    assert!(
        mission_show_success,
        "mission show should degrade instead of failing: {mission_show_err}"
    );
    assert!(mission_show_out.contains("Valid degraded work"));
    assert!(mission_show_out.contains("Malformed degraded work"));
    assert_degraded_repair_guidance(&mission_show_err, malformed_id);

    let (show_success, show_out, show_err) =
        run_atelier(dir.path(), &["issue", "show", malformed_id]);
    assert!(
        show_success,
        "issue show should degrade instead of failing: {show_err}"
    );
    assert!(show_out.contains("Tracker Degraded"));
    assert!(show_out.contains("Fallback: showing the last valid local projection"));
    assert!(show_out.contains("Missing required issue body section 'Outcome'"));
    assert!(show_out.contains(&format!("Next: atelier lint {malformed_id}")));
    assert_degraded_repair_guidance(&show_err, malformed_id);

    let (doctor_success, doctor_out, doctor_err) = run_atelier(dir.path(), &["doctor"]);
    assert!(doctor_success, "doctor should remain usable: {doctor_err}");
    assert!(doctor_out.contains("Projection rebuild:"));
    assert!(doctor_out.contains("rebuild_ready: not ok"));

    let (lint_success, lint_out, lint_err) = run_atelier(dir.path(), &["lint"]);
    assert!(
        !lint_success,
        "global lint must fail closed for malformed records"
    );
    let lint_transcript = format!("{lint_out}\n{lint_err}");
    assert_degraded_lint_diagnostic(&lint_transcript, malformed_id);

    let (focused_success, focused_out, focused_err) =
        run_atelier(dir.path(), &["lint", malformed_id]);
    assert!(
        !focused_success,
        "focused lint must fail closed for malformed records"
    );
    let focused_transcript = format!("{focused_out}\n{focused_err}");
    assert_degraded_lint_diagnostic(&focused_transcript, malformed_id);

    let (close_success, _close_out, close_err) = run_atelier(
        dir.path(),
        &["issue", "close", malformed_id, "--reason", "done"],
    );
    assert!(!close_success, "issue closeout must fail closed");
    assert!(close_err.contains("Canonical tracker Markdown is invalid"));
    assert!(close_err.contains("atelier lint"));

    let (workflow_success, _workflow_out, workflow_err) = run_atelier(
        dir.path(),
        &[
            "workflow",
            "validate",
            "mission",
            mission_id,
            "--transition",
            "close",
        ],
    );
    assert!(!workflow_success, "workflow gates must fail closed");
    assert!(workflow_err.contains("Canonical tracker Markdown is invalid"));
    assert!(workflow_err.contains("atelier lint"));
}

fn assert_degraded_repair_guidance(stderr: &str, issue_id: &str) {
    for needle in [
        "Tracker degraded".to_string(),
        "orientation only".to_string(),
        "Repair: run `atelier lint`".to_string(),
        format!(".atelier/issues/{issue_id}.md"),
        "Missing required issue body section 'Outcome'".to_string(),
    ] {
        assert!(
            stderr.contains(&needle),
            "degraded stderr missing {needle:?}:\n{stderr}"
        );
    }
}

fn assert_degraded_lint_diagnostic(transcript: &str, issue_id: &str) {
    for needle in [
        format!(".atelier/issues/{issue_id}.md"),
        "Missing required issue body section 'Outcome'".to_string(),
        "section Outcome".to_string(),
    ] {
        assert!(
            transcript.contains(&needle),
            "lint transcript missing {needle:?}:\n{transcript}"
        );
    }
}

#[test]
fn test_mission_list_human_overview_orders_and_summarizes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, older_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Older ready"]);
    assert!(success, "older mission create failed: {stderr}");
    assert!(older_out.contains("Mission atelier-"));
    let older_id = record_id_by_title(dir.path(), "missions", "Older ready");
    let older_id = older_id.as_str();

    std::thread::sleep(std::time::Duration::from_millis(5));
    let (success, active_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Active mission"]);
    assert!(success, "active mission create failed: {stderr}");
    assert!(active_out.contains("Mission atelier-"));
    let active_id = record_id_by_title(dir.path(), "missions", "Active mission");
    let active_id = active_id.as_str();

    std::thread::sleep(std::time::Duration::from_millis(5));
    let (success, closed_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Newest closed"]);
    assert!(success, "closed mission create failed: {stderr}");
    assert!(closed_out.contains("Mission atelier-"));
    let closed_id = record_id_by_title(dir.path(), "missions", "Newest closed");
    let closed_id = closed_id.as_str();
    let (success, closed_evidence, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "add",
            "--kind",
            "validation",
            "--result",
            "pass",
            "newest closed evidence",
        ],
    );
    assert!(success, "closed evidence create failed: {stderr}");
    assert!(closed_evidence.contains("[evidence] pass - newest closed evidence"));
    let closed_evidence_id = record_id_by_title(dir.path(), "evidence", "newest closed evidence");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "attach",
            &closed_evidence_id,
            "mission",
            closed_id,
        ],
    );
    assert!(success, "closed evidence attach failed: {stderr}");
    commit_all(dir.path(), "close newest mission");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", &closed_id, "--status", "closed"],
    );
    assert!(success, "close mission failed: {stderr}");

    let (success, epic_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Mission epic", "--issue-type", "epic"],
    );
    assert!(success, "epic issue create failed: {stderr}");
    assert!(epic_out.contains("Created issue atelier-"));
    let epic_id = issue_id_by_title(dir.path(), "Mission epic");
    let epic_id = epic_id.as_str();

    let (success, ready_out, stderr) =
        run_atelier(dir.path(), &["issue", "subissue", epic_id, "Ready work"]);
    assert!(success, "ready subissue create failed: {stderr}");
    assert!(ready_out.contains(epic_id));

    let (success, blocked_out, stderr) =
        run_atelier(dir.path(), &["issue", "subissue", epic_id, "Blocked work"]);
    assert!(success, "blocked subissue create failed: {stderr}");
    assert!(blocked_out.contains(epic_id));
    let blocked_id = issue_id_by_title(dir.path(), "Blocked work");
    let blocked_id = blocked_id.as_str();
    let (success, blocker_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Work blocker"]);
    assert!(success, "work blocker create failed: {stderr}");
    assert!(blocker_out.contains("Created issue atelier-"));
    let blocker_id = issue_id_by_title(dir.path(), "Work blocker");
    let blocker_id = blocker_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "block", &blocked_id, &blocker_id]);
    assert!(success, "block issue failed: {stderr}");

    let (success, done_out, stderr) =
        run_atelier(dir.path(), &["issue", "subissue", epic_id, "Done work"]);
    assert!(success, "done subissue create failed: {stderr}");
    assert!(done_out.contains(epic_id));
    let done_id = issue_id_by_title(dir.path(), "Done work");
    let done_id = done_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &done_id, "--reason", "done"],
    );
    assert!(success, "close issue failed: {stderr}");

    let (success, loose_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Loose mission work"]);
    assert!(success, "loose issue create failed: {stderr}");
    assert!(loose_out.contains("Created issue atelier-"));
    let loose_id = issue_id_by_title(dir.path(), "Loose mission work");
    let loose_id = loose_id.as_str();

    let (success, mission_blocker_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Mission blocker"]);
    assert!(success, "mission blocker create failed: {stderr}");
    assert!(mission_blocker_out.contains("Created issue atelier-"));
    let mission_blocker_id = issue_id_by_title(dir.path(), "Mission blocker");
    let mission_blocker_id = mission_blocker_id.as_str();

    for issue_id in [&epic_id, &loose_id] {
        let (success, _, stderr) =
            run_atelier(dir.path(), &["mission", "add-work", &active_id, issue_id]);
        assert!(success, "link work {issue_id} failed: {stderr}");
    }
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-blocker", &active_id, &mission_blocker_id],
    );
    assert!(success, "link mission blocker failed: {stderr}");

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "add",
            "--kind",
            "test",
            "--result",
            "pass",
            "older mission evidence",
        ],
    );
    assert!(success, "evidence add failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - older mission evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "older mission evidence");
    let evidence_id = evidence_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", &evidence_id, "mission", &older_id],
    );
    assert!(success, "link evidence failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["mission", "list"]);
    assert!(success, "mission list failed: {stderr}");
    assert!(stdout.contains("Missions"));
    assert!(stdout.contains("2 ready missions | 1 blocked"));
    assert!(!stdout.contains("Updated:"));
    assert!(stdout.contains("Evidence gaps: 1"));
    assert!(!stdout.contains("ready="));
    assert!(stdout.contains("Ready"));
    assert!(!stdout.contains("Closed"));

    let active_row = format!("{active_id} [ready] - Active mission");
    let older_row = format!("{older_id} [ready] - Older ready");
    let closed_row = format!("{closed_id} [closed] - Newest closed");
    let active_pos = stdout.find(&active_row).expect("missing active row");
    let older_pos = stdout.find(&older_row).expect("missing older row");
    assert!(
        active_pos < older_pos,
        "newer ready mission should sort first:\n{stdout}"
    );
    assert!(!stdout.contains(&closed_row));
    assert!(
        stdout.contains(&format!(
            "[epic] {epic_id} [open] medium - Mission epic | ready 1, blocked 1, done 1"
        )),
        "missing linked epic summary:\n{stdout}"
    );
    assert!(stdout.contains("Other linked work: 1 ready"));
    assert!(stdout.contains("Mission blockers: 1 open"));
    assert!(stdout.contains("No linked epics."));
    assert!(!stdout.contains("Loose mission work"));
    assert!(stdout.contains(&format!("atelier mission status {active_id}")));
    assert!(stdout.contains(&format!("atelier mission show {active_id}")));
    assert!(stdout.contains("atelier mission status"));
    assert!(stdout.contains("atelier mission create \"...\""));

    let (success, all_out, stderr) =
        run_atelier(dir.path(), &["mission", "list", "--status", "all"]);
    assert!(success, "all mission list failed: {stderr}");
    assert!(all_out.contains("3 missions | 1 closed, 2 ready | 1 blocked"));
    assert!(all_out.contains(&active_row));
    assert!(all_out.contains(&older_row));
    assert!(all_out.contains(&closed_row));
    let active_pos = all_out.find(&active_row).expect("missing active row");
    let closed_pos = all_out.find(&closed_row).expect("missing closed row");
    assert!(
        active_pos < closed_pos,
        "current missions should sort before closed missions:\n{all_out}"
    );

    let (success, ready_out, stderr) =
        run_atelier(dir.path(), &["mission", "list", "--status", "ready"]);
    assert!(success, "filtered mission list failed: {stderr}");
    assert!(ready_out.contains(&active_row));
    assert!(ready_out.contains(&older_row));
    assert!(!ready_out.contains(&closed_row));

    let (success, open_out, stderr) =
        run_atelier(dir.path(), &["mission", "list", "--status", "open"]);
    assert!(!success, "mission status alias should be rejected");
    assert!(open_out.is_empty());
    assert!(stderr.contains("Invalid mission status 'open'"));

    let (success, empty_out, stderr) =
        run_atelier(dir.path(), &["mission", "list", "--status", "draft"]);
    assert!(success, "empty filtered mission list failed: {stderr}");
    assert!(empty_out.contains("0 missions"));
    assert!(empty_out.contains("(none)"));
    assert!(empty_out.contains("atelier mission create \"...\""));

    let (success, list_out, stderr) = run_atelier(dir.path(), &["mission", "list"]);
    assert!(success, "mission list failed: {stderr}");
    assert!(list_out.contains(&active_row));
}

#[test]
fn test_mission_status_cli_reports_control_state() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Autonomy status"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Autonomy status");
    let mission_id = mission_id.as_str();

    let (success, epic_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Status epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    assert!(epic_out.contains("Created issue atelier-"));
    let epic_id = issue_id_by_title(dir.path(), "Status epic");
    let epic_id = epic_id.as_str();

    let (success, ready_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "subissue",
            epic_id,
            "Ready status work",
            "--description",
            "## Description\n\nReady status body.\n\n## Outcome\n\nMission status reports ready linked work.\n\n## Evidence\n\n- `atelier mission status <mission-id>` lists this work as ready.",
        ],
    );
    assert!(success, "ready work create failed: {stderr}");
    assert!(ready_out.contains(epic_id));

    let (success, blocked_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "subissue",
            epic_id,
            "Blocked status work",
            "--description",
            "## Description\n\nBlocked status body.\n\n## Outcome\n\nMission status reports blocked linked work.\n\n## Evidence\n\n- `atelier mission status <mission-id>` lists this work as blocked.",
        ],
    );
    assert!(success, "blocked work create failed: {stderr}");
    assert!(blocked_out.contains(epic_id));
    let blocked_id = issue_id_by_title(dir.path(), "Blocked status work");
    let blocked_id = blocked_id.as_str();
    let (success, blocker_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Status blocker",
            "--description",
            "## Description\n\nStatus blocker body.\n\n## Outcome\n\nMission status reports this issue as an open blocker.\n\n## Evidence\n\n- `atelier mission status <mission-id>` lists this blocker.",
        ],
    );
    assert!(success, "blocker create failed: {stderr}");
    assert!(blocker_out.contains("Created issue atelier-"));
    let blocker_id = issue_id_by_title(dir.path(), "Status blocker");
    let blocker_id = blocker_id.as_str();
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "block", blocked_id, blocker_id]);
    assert!(success, "block issue failed: {stderr}");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, epic_id]);
    assert!(success, "mission add work failed: {stderr}");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains(&format!(
        "Mission Status {mission_id} [ready] - Autonomy status"
    )));
    assert!(status_out.contains("Health:   blocked"));
    assert!(status_out.contains("Tracker:  ok"));
    assert!(status_out.contains("Work"));
    assert!(status_out.contains("ready"));
    assert!(status_out.contains("blocked"));
    assert!(status_out.contains("Blockers"));
    assert!(status_out.contains("Evidence"));
    assert!(status_out.contains("Gap: no evidence records are linked to this mission."));
    assert!(status_out.contains("Reliability"));
    assert!(status_out.contains("Projection Freshness: current"));
    assert!(status_out.contains("Malformed Work: none"));
    assert!(status_out.contains("Missing Outcome Sections: none"));
    assert!(status_out.contains("Missing Evidence Sections: none"));
    assert!(status_out.contains("Attached Proof: missing"));
    assert!(status_out.contains("Open Blockers: 1 open"));
    assert!(status_out.contains(&format!("atelier mission audit {mission_id}")));
    assert!(status_out.contains("atelier lint"));
    assert!(status_out.contains("atelier doctor"));
    assert!(status_out.contains("Closeout Gates"));
    assert!(status_out.contains("Mission Proof: missing"));
    assert!(status_out.contains("Advanced Validator Detail"));
    assert!(status_out.contains("advanced closeout validator failure detected."));
    assert!(status_out.contains("Next Commands"));
    assert!(status_out.contains(&format!(
        "Inspect mission record (durable intent and linked work): atelier mission show {mission_id}"
    )));
    assert!(status_out.contains(&format!(
        "Refresh mission status (current blockers and closeout gates): atelier mission status {mission_id}"
    )));
    assert!(status_out.contains("Choose ready work ("));
    assert!(status_out.contains("ready item(s)): atelier issue list --ready"));
    assert!(status_out.contains(
        "Record validation proof (1 evidence gap(s)): atelier evidence add --kind validation --result pass \"...\""
    ));
    assert!(
        !status_out.contains("workflow validate"),
        "normal mission next commands must not route to raw workflow validators:\n{status_out}"
    );

    let (success, quiet_out, stderr) =
        run_atelier(dir.path(), &["--quiet", "mission", "status", mission_id]);
    assert!(success, "quiet mission status failed: {stderr}");
    assert!(quiet_out.contains(&format!("{mission_id} health=blocked")));
    assert!(quiet_out.contains("evidence_gaps=1"));
    assert!(quiet_out.contains("tracker=ok"));

    let (success, dashboard_out, stderr) = run_atelier(dir.path(), &["mission", "status"]);
    assert!(success, "mission status dashboard failed: {stderr}");
    assert!(dashboard_out.contains("Mission Status"));
    assert!(dashboard_out.contains("1 ready mission | 1 blocked | tracker ok"));
    assert!(dashboard_out.contains(&format!("{mission_id} [blocked] ready - Autonomy status")));

    let closeout_mission = {
        let (success, out, stderr) =
            run_atelier(dir.path(), &["mission", "create", "Closeout mission"]);
        assert!(success, "closeout mission create failed: {stderr}");
        assert!(out.contains("Mission atelier-"));
        record_id_by_title(dir.path(), "missions", "Closeout mission")
    };
    let closeout_mission = closeout_mission.as_str();
    let (success, work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Finished mission work"]);
    assert!(success, "finished work create failed: {stderr}");
    assert!(work_out.contains("Created issue atelier-"));
    let work_id = issue_id_by_title(dir.path(), "Finished mission work");
    let work_id = work_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-work", closeout_mission, work_id],
    );
    assert!(success, "closeout mission add work failed: {stderr}");
    attach_issue_pass_evidence(dir.path(), work_id);
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "close", work_id, "--reason", "done"]);
    assert!(success, "closeout work close failed: {stderr}");
    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "add",
            "--kind",
            "validation",
            "--result",
            "pass",
            "closeout evidence",
        ],
    );
    assert!(success, "closeout evidence add failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - closeout evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "closeout evidence");
    let evidence_id = evidence_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "attach",
            evidence_id,
            "mission",
            closeout_mission,
        ],
    );
    assert!(success, "closeout evidence attach failed: {stderr}");
    commit_all(dir.path(), "closeout status ready");

    let (success, closeout_status, stderr) =
        run_atelier(dir.path(), &["mission", "status", closeout_mission]);
    assert!(success, "closeout mission status failed: {stderr}");
    assert!(closeout_status.contains("Health:   closeout"));
    assert!(closeout_status.contains("Closeout: ready"));
    assert!(closeout_status.contains("Reliability"));
    assert!(closeout_status.contains("Attached Proof: complete"));
    assert!(closeout_status.contains("Docs/Help Drift: clear"));
    assert!(closeout_status.contains("Ignored Test Review: current"));
    assert!(closeout_status.contains("Open Blockers: none"));
    assert!(closeout_status.contains(&format!(
        "Close mission (all closeout gates pass): atelier mission update {closeout_mission} --status closed"
    )));

    let mission_path = dir
        .path()
        .join(".atelier")
        .join("missions")
        .join(format!("{mission_id}.md"));
    let mission_markdown = std::fs::read_to_string(&mission_path).unwrap();
    std::fs::write(
        &mission_path,
        mission_markdown.replace("Autonomy status", "Autonomy status stale"),
    )
    .unwrap();
    let (success, stale_status, stderr) =
        run_atelier(dir.path(), &["mission", "status", mission_id]);
    assert!(success, "stale mission status failed: {stderr}");
    assert!(stale_status.contains("Autonomy status stale"));
    assert!(stale_status.contains("Tracker:  ok"));
    assert!(stale_status.contains("Worktree: dirty"));
    assert!(stale_status.contains("advanced closeout validator failure detected."));
}

#[test]
fn test_active_mission_focus_guides_status_and_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    Command::new("git")
        .current_dir(dir.path())
        .args(["init", "-q"])
        .status()
        .unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Active focus"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Active focus");
    let mission_id = mission_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Mission work"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Mission work");
    let issue_id = issue_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, issue_id]);
    assert!(success, "mission add work failed: {stderr}");

    let (success, start_out, stderr) = run_atelier(dir.path(), &["mission", "start", mission_id]);
    assert!(success, "mission start failed: {stderr}");
    assert!(start_out.contains(&format!("Active mission: {mission_id}")));
    assert!(start_out.contains(&format!("atelier mission status {mission_id}")));

    let (success, status_out, stderr) = run_atelier(dir.path(), &["mission", "status"]);
    assert!(success, "active mission status failed: {stderr}");
    assert!(status_out.contains(&format!(
        "Mission Status {mission_id} [active] - Active focus"
    )));

    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    Command::new("git")
        .current_dir(dir.path())
        .args(["add", "."])
        .status()
        .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["commit", "-q", "-m", "active mission"])
        .status()
        .unwrap();

    let (success, work_out, stderr) = run_atelier(dir.path(), &["work", "start", issue_id]);
    assert!(success, "work start failed: {stderr}");
    assert!(work_out.contains(&format!("Mission: {mission_id} (active)")));
    assert!(work_out.contains(&format!("Started work on {issue_id}")));
}

#[test]
fn test_mission_start_requires_explicit_switch_and_warns_for_outside_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    Command::new("git")
        .current_dir(dir.path())
        .args(["init", "-q"])
        .status()
        .unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "First mission"]);
    assert!(success, "first mission create failed: {stderr}");
    let first_id = record_id_by_title(dir.path(), "missions", "First mission");
    let first_id = first_id.as_str();
    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Second mission"]);
    assert!(success, "second mission create failed: {stderr}");
    let second_id = record_id_by_title(dir.path(), "missions", "Second mission");
    let second_id = second_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "start", first_id]);
    assert!(success, "first mission start failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "start", second_id]);
    assert!(!success, "second mission start without switch should fail");
    assert!(
        stderr.contains("--switch"),
        "expected switch guidance in stderr: {stderr}"
    );
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "start", second_id, "--switch"]);
    assert!(success, "mission switch failed: {stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Outside work"]);
    assert!(success, "outside issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Outside work");
    let issue_id = issue_id.as_str();
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    Command::new("git")
        .current_dir(dir.path())
        .args(["add", "."])
        .status()
        .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["commit", "-q", "-m", "switched mission"])
        .status()
        .unwrap();

    let (success, work_out, stderr) = run_atelier(dir.path(), &["work", "start", issue_id]);
    assert!(success, "outside work start failed: {stderr}");
    assert!(work_out.contains(&format!(
        "Warning: {issue_id} is outside active mission {second_id}"
    )));
}

#[test]
fn test_mission_list_default_current_empty_state() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, closed_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Closed only"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(closed_out.contains("Mission atelier-"));
    let closed_id = record_id_by_title(dir.path(), "missions", "Closed only");
    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "add",
            "--kind",
            "validation",
            "--result",
            "pass",
            "closed only evidence",
        ],
    );
    assert!(success, "evidence create failed: {stderr}");
    assert!(evidence_out.contains("[evidence] pass - closed only evidence"));
    let evidence_id = record_id_by_title(dir.path(), "evidence", "closed only evidence");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["evidence", "attach", &evidence_id, "mission", &closed_id],
    );
    assert!(success, "evidence attach failed: {stderr}");
    commit_all(dir.path(), "close only mission");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", &closed_id, "--status", "closed"],
    );
    assert!(success, "close mission failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["mission", "list"]);
    assert!(success, "mission list failed: {stderr}");
    assert!(stdout.contains("0 missions | 0 blocked"));
    assert!(stdout.contains("(none)"));
    assert!(!stdout.contains("Closed only"));

    let (success, closed_stdout, stderr) =
        run_atelier(dir.path(), &["mission", "list", "--status", "closed"]);
    assert!(success, "closed mission list failed: {stderr}");
    assert!(closed_stdout.contains("1 closed mission | 0 blocked"));
    assert!(closed_stdout.contains("Closed only"));
}

#[test]
fn test_first_class_record_rebuild_rejects_schema_drift() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Guard schema"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Guard schema");
    let mission_path = dir
        .path()
        .join(".atelier")
        .join("missions")
        .join(format!("{mission_id}.md"));

    let mission_markdown = std::fs::read_to_string(&mission_path).unwrap();
    std::fs::write(
        &mission_path,
        mission_markdown.replace("schema: \"atelier.mission\"", "schema: \"atelier.issue\""),
    )
    .unwrap();
    std::fs::remove_file(dir.path().join(".atelier/state.db")).unwrap();

    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(!success, "rebuild should reject mission schema drift");
    assert!(
        stderr.contains("Unsupported schema 'atelier.issue'")
            && stderr.contains("expected atelier.mission"),
        "unexpected rebuild error: {stderr}"
    );
}

#[test]
fn test_projection_index_rebuilds_changed_sources_before_issue_queries() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Indexed title"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    let (success, ready_out, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success, "fresh ready failed: {stderr}");
    assert!(ready_out.contains("Indexed title"));

    let issue_path = dir
        .path()
        .join(".atelier/issues")
        .join(format!("{issue_id}.md"));
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(
        &issue_path,
        markdown.replace("Indexed title", "Markdown title"),
    )
    .unwrap();

    let (success, ready_out, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(
        success,
        "stale ready should transparently rebuild: {stderr}"
    );
    assert!(ready_out.contains("Markdown title"));
    assert!(
        stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "missing automatic rebuild diagnostic: {stderr}"
    );
}

#[test]
fn test_projection_index_bounds_many_changed_sources_and_rebuilds() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let mut issue_ids = Vec::new();
    for index in 0..12 {
        let title = format!("Bulk indexed {index}");
        let (success, issue_out, stderr) = run_atelier(dir.path(), &["issue", "create", &title]);
        assert!(success, "issue create failed: {stderr}");
        assert!(issue_out.contains("Created issue atelier-"));
        issue_ids.push(issue_ref(dir.path(), index + 1));
    }
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    for (index, issue_id) in issue_ids.iter().enumerate() {
        let issue_path = dir
            .path()
            .join(".atelier/issues")
            .join(format!("{issue_id}.md"));
        let markdown = std::fs::read_to_string(&issue_path).unwrap();
        std::fs::write(
            &issue_path,
            markdown.replace(
                &format!("title: \"Bulk indexed {index}\""),
                &format!("title: \"Bulk markdown {index}\""),
            ),
        )
        .unwrap();
    }

    let (success, _stdout, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(!success, "export check should report stale projection");
    assert!(
        stderr.contains("12 indexed sources changed")
            && stderr.contains("showing first 5")
            && stderr.contains("atelier rebuild"),
        "stale diagnostics should be bounded and actionable: {stderr}"
    );
    assert!(
        stderr.lines().count() < 12,
        "stale diagnostics should not dump every changed source: {stderr}"
    );

    let (success, list_out, stderr) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(
        success,
        "many changed sources should transparently rebuild: {stderr}"
    );
    assert!(list_out.contains("Bulk markdown 0"));
    assert!(list_out.contains("Bulk markdown 11"));
    assert!(
        stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "missing automatic rebuild diagnostic: {stderr}"
    );
}

#[test]
fn test_projection_index_rebuilds_deleted_and_unindexed_sources_before_issue_queries() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, first_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "First indexed issue"]);
    assert!(success, "first create failed: {stderr}");
    assert!(first_out.contains("Created issue atelier-"));
    let first_id = issue_ref(dir.path(), 1);
    let (success, second_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Second indexed issue"]);
    assert!(success, "second create failed: {stderr}");
    assert!(second_out.contains("Created issue atelier-"));
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    let first_path = dir
        .path()
        .join(".atelier/issues")
        .join(format!("{first_id}.md"));
    let first_markdown = std::fs::read_to_string(&first_path).unwrap();
    std::fs::remove_file(&first_path).unwrap();

    let (success, list_out, stderr) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(
        success,
        "deleted source list should transparently rebuild: {stderr}"
    );
    assert!(!list_out.contains("First indexed issue"));
    assert!(list_out.contains("Second indexed issue"));
    assert!(
        stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "missing automatic rebuild diagnostic: {stderr}"
    );

    std::fs::write(&first_path, first_markdown).unwrap();
    let unindexed_path = dir.path().join(".atelier/issues/atelier-zzzz.md");
    std::fs::write(
        &unindexed_path,
        r#"---
created_at: "2026-06-10T12:00:00+00:00"
id: "atelier-zzzz"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Unindexed issue"
updated_at: "2026-06-10T12:00:00+00:00"
---

## Description

Body

## Outcome

The unindexed issue is discoverable after rebuild.

## Evidence

- `atelier issue search Unindexed` shows the record.
"#,
    )
    .unwrap();

    let (success, search_out, stderr) = run_atelier(dir.path(), &["issue", "search", "Unindexed"]);
    assert!(
        success,
        "unindexed search should transparently rebuild: {stderr}"
    );
    assert!(search_out.contains("Unindexed issue"));
    assert!(
        stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "missing automatic rebuild diagnostic: {stderr}"
    );
}

#[test]
fn test_projection_index_rebuilds_dep_list_and_lint_but_ignores_derived_files() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let first_body = "## Description\n\nProjection root body.\n\n## Outcome\n\nProjection root remains queryable after rebuild.\n\n## Evidence\n\n- `atelier lint` passes after automatic rebuild.";
    let second_body = "## Description\n\nProjection leaf body.\n\n## Outcome\n\nProjection leaf remains linked after rebuild.\n\n## Evidence\n\n- `atelier dep list` shows the linked root.";

    let (success, first_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Projection root",
            "--description",
            first_body,
        ],
    );
    assert!(success, "first create failed: {stderr}");
    assert!(first_out.contains("Created issue atelier-"));
    let first_id = issue_ref(dir.path(), 1);
    let (success, second_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Projection leaf",
            "--description",
            second_body,
        ],
    );
    assert!(success, "second create failed: {stderr}");
    assert!(second_out.contains("Created issue atelier-"));
    let second_id = issue_ref(dir.path(), 2);
    let (success, _, stderr) = run_atelier(dir.path(), &["dep", "add", &second_id, &first_id]);
    assert!(success, "dep add failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    std::fs::write(dir.path().join(".atelier/manifest.json"), "{}\n").unwrap();
    std::fs::write(dir.path().join(".atelier/graph.json"), "{}\n").unwrap();
    let (success, ready_out, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(
        success,
        "derived files should not stale issue list --ready: {stderr}"
    );
    assert!(ready_out.contains("Projection root"));

    let issue_path = dir
        .path()
        .join(".atelier/issues")
        .join(format!("{first_id}.md"));
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(
        &issue_path,
        markdown.replace("Projection root", "Projection root changed"),
    )
    .unwrap();

    let (success, dep_out, stderr) = run_atelier(dir.path(), &["dep", "list"]);
    assert!(
        success,
        "stale dep list should transparently rebuild: {stderr}"
    );
    assert!(dep_out.contains("Projection root changed"));
    assert!(
        stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "missing automatic rebuild diagnostic: {stderr}"
    );

    let (success, lint_out, stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(
        success,
        "lint should run after automatic rebuild:\nstdout:\n{lint_out}\nstderr:\n{stderr}"
    );
    assert!(lint_out.contains("Lint passed."));
}

#[test]
fn test_rebuild_temp_files_are_ignored_by_query_lint_export_and_doctor() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let body = "## Description\n\nTemp rebuild filter body.\n\n## Outcome\n\nQuery, lint, export, and doctor ignore rebuild temp files.\n\n## Evidence\n\n- `atelier lint`, `atelier export --check`, and `atelier doctor` ignore rebuild temp files.";
    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Temp rebuild filter",
            "--description",
            body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    let temp_path = dir.path().join(".atelier/.state.db.123.456.rebuild-tmp");
    std::fs::write(&temp_path, "partial sqlite rebuild").unwrap();

    let issue_path = dir
        .path()
        .join(".atelier/issues")
        .join(format!("{issue_id}.md"));
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(
        &issue_path,
        markdown.replace("Temp rebuild filter", "Temp rebuild filter changed"),
    )
    .unwrap();

    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "query should ignore rebuild tmp file: {stderr}");
    assert!(show_out.contains("Temp rebuild filter changed"));
    assert!(
        !stderr.contains("rebuild-tmp"),
        "query diagnostics must not report rebuild tmp path: {stderr}"
    );

    let commands: &[&[&str]] = &[&["lint"], &["export", "--check"], &["doctor"]];
    for args in commands {
        let (success, stdout, stderr) = run_atelier(dir.path(), args);
        assert!(success, "{args:?} should ignore rebuild tmp file: {stderr}");
        let combined = format!("{stdout}\n{stderr}");
        assert!(
            !combined.contains("rebuild-tmp"),
            "{args:?} diagnostics must not report rebuild tmp path: {combined}"
        );
    }
}

#[test]
fn test_projection_index_rejects_invalid_markdown_without_rebuild() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Invalid Markdown source"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    let issue_path = dir
        .path()
        .join(".atelier/issues")
        .join(format!("{issue_id}.md"));
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(
        &issue_path,
        markdown.replace(
            "title: \"Invalid Markdown source\"",
            "title: [Invalid Markdown source",
        ),
    )
    .unwrap();

    let (success, _stdout, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(
        !success,
        "invalid canonical Markdown should fail export check"
    );
    assert!(
        stderr.contains("canonical tracker Markdown is invalid")
            && stderr.contains("atelier lint")
            && stderr.contains(&format!(".atelier/issues/{issue_id}.md")),
        "unexpected invalid export error: {stderr}"
    );
    assert!(
        !stderr.contains("indexed source changed"),
        "invalid canonical errors must not be obscured by stale metadata: {stderr}"
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(
        success,
        "invalid canonical Markdown should show degraded fallback: {stderr}"
    );
    assert!(stdout.contains("Tracker Degraded"));
    assert!(stdout.contains("Invalid YAML front matter"));
    assert!(stdout.contains("Fallback: showing the last valid local projection"));
    assert!(stdout.contains(&format!("Next: atelier lint {issue_id}")));
    assert!(
        stderr.contains("Tracker degraded")
            && stderr.contains("atelier lint")
            && stderr.contains("Invalid YAML front matter")
            && stderr.contains(&format!(".atelier/issues/{issue_id}.md")),
        "unexpected invalid Markdown error: {stderr}"
    );
    assert!(
        !stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "invalid Markdown must not be silently repaired: {stderr}"
    );

    std::fs::write(&issue_path, markdown).unwrap();
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(
        success,
        "restored canonical Markdown should query: {stderr}"
    );
    assert!(show_out.contains("Invalid Markdown source"));
}

#[test]
fn test_lint_validates_canonical_markdown_even_when_projection_metadata_is_fresh() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Lint canonical source"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);

    let issue_path = dir
        .path()
        .join(".atelier/issues")
        .join(format!("{issue_id}.md"));
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    let invalid_markdown = markdown.replace(
        "title: \"Lint canonical source\"",
        "title: [Lint canonical source",
    );
    std::fs::write(&issue_path, invalid_markdown.as_bytes()).unwrap();

    let metadata = std::fs::metadata(&issue_path).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(invalid_markdown.as_bytes());
    let invalid_hash = format!("{:x}", hasher.finalize());
    let conn = rusqlite::Connection::open(dir.path().join(".atelier/state.db")).unwrap();
    conn.execute(
        "UPDATE projection_index_sources
         SET size_bytes = ?1, sha256 = ?2
         WHERE path = ?3",
        rusqlite::params![
            i64::try_from(metadata.len()).unwrap(),
            invalid_hash,
            format!("issues/{issue_id}.md")
        ],
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(
        !success,
        "lint must reject malformed canonical Markdown, stdout: {stdout}"
    );
    assert!(
        stdout.contains("Invalid YAML front matter")
            && stdout.contains(&format!(".atelier/issues/{issue_id}.md")),
        "unexpected lint output: {stdout}\nstderr: {stderr}"
    );
    assert!(stderr.contains("Lint failed"));
    assert!(
        !stdout.contains("Lint passed."),
        "lint must not pass from stale SQLite rows: {stdout}"
    );
}

#[test]
fn test_lint_validates_canonical_markdown_when_state_db_is_missing() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let body = "## Description\n\nDescription\n\n## Outcome\n\nLint rebuilds a missing state database from canonical Markdown.\n\n## Evidence\n\n- `atelier lint` prints `Lint passed.` after rebuilding state.db.";

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Lint without state db",
            "--description",
            body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    std::fs::remove_file(dir.path().join(".atelier/state.db")).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(success, "lint should rebuild missing state.db: {stderr}");
    assert!(stdout.contains("Lint passed."));
    assert!(
        stderr.contains("Runtime projection database was missing; rebuilt local SQLite projection"),
        "missing rebuild diagnostic: {stderr}"
    );
}

#[test]
fn test_focused_lint_validates_missing_relationship_targets() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let body = "## Description\n\nFocused lint missing target body.\n\n## Outcome\n\nFocused lint reports a missing relationship target.\n\n## Evidence\n\n- `atelier lint <issue-id>` reports the missing issue relationship target.";
    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Focused lint missing target",
            "--description",
            body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);

    let issue_path = dir
        .path()
        .join(".atelier/issues")
        .join(format!("{issue_id}.md"));
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(
        &issue_path,
        markdown.replace(
            "  blocks: []",
            "  blocks:\n  - kind: \"issue\"\n    id: \"atelier-missing\"",
        ),
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &issue_id]);
    assert!(
        !success,
        "focused lint should reject missing relationship target, stdout: {stdout}"
    );
    let transcript = format!("{stdout}\n{stderr}");
    assert!(
        transcript.contains("has blocks reference to missing issue atelier-missing")
            && transcript.contains(&issue_id)
            && transcript.contains("Canonical tracker Markdown is invalid"),
        "unexpected focused lint error: {transcript}"
    );
}

#[test]
fn test_focused_lint_validates_dependency_cycles() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let first_body = "## Description\n\nFocused lint cycle root body.\n\n## Outcome\n\nFocused lint reports dependency cycles.\n\n## Evidence\n\n- `atelier lint <issue-id>` reports relationships.blocks contains a cycle.";
    let second_body = "## Description\n\nFocused lint cycle leaf body.\n\n## Outcome\n\nFocused lint reports dependency cycles.\n\n## Evidence\n\n- `atelier lint <issue-id>` reports relationships.blocks contains a cycle.";
    let (success, first_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Focused lint cycle root",
            "--description",
            first_body,
        ],
    );
    assert!(success, "first issue create failed: {stderr}");
    assert!(first_out.contains("Created issue atelier-"));
    let first_id = issue_ref(dir.path(), 1);
    let (success, second_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Focused lint cycle leaf",
            "--description",
            second_body,
        ],
    );
    assert!(success, "second issue create failed: {stderr}");
    assert!(second_out.contains("Created issue atelier-"));
    let second_id = issue_ref(dir.path(), 2);

    for (issue_id, blocked_id) in [(&first_id, &second_id), (&second_id, &first_id)] {
        let issue_path = dir
            .path()
            .join(".atelier/issues")
            .join(format!("{issue_id}.md"));
        let markdown = std::fs::read_to_string(&issue_path).unwrap();
        std::fs::write(
            &issue_path,
            markdown.replace(
                "  blocks: []",
                &format!("  blocks:\n  - kind: \"issue\"\n    id: \"{blocked_id}\""),
            ),
        )
        .unwrap();
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint", &first_id]);
    assert!(
        !success,
        "focused lint should reject dependency cycle, stdout: {stdout}"
    );
    let transcript = format!("{stdout}\n{stderr}");
    assert!(
        transcript.contains("relationships.blocks contains a cycle"),
        "unexpected focused lint cycle error: {transcript}"
    );
}

#[test]
fn test_lint_has_stable_diagnostics_for_hard_invalid_markdown_records() {
    assert_lint_rejects_issue_edit(
        "Invalid status fixture",
        |markdown, _issue_id| markdown.replace("status: \"open\"", "status: \"bogus\""),
        &["Invalid status", "Invalid status 'bogus'"],
    );
    assert_lint_rejects_issue_edit(
        "Invalid type fixture",
        |markdown, _issue_id| markdown.replace("issue_type: \"task\"", "issue_type: \"bogus\""),
        &["Invalid issue_type", "Invalid issue_type 'bogus'"],
    );
    assert_lint_rejects_issue_edit(
        "Invalid priority fixture",
        |markdown, _issue_id| markdown.replace("priority: \"P2\"", "priority: \"urgent\""),
        &[
            "Invalid priority",
            "unsupported canonical priority 'urgent'",
        ],
    );
    assert_lint_rejects_issue_edit(
        "Invalid schema fixture",
        |markdown, _issue_id| {
            markdown.replace("schema: \"atelier.issue\"", "schema: \"atelier.graph\"")
        },
        &["Unsupported schema 'atelier.graph'"],
    );
    assert_lint_rejects_issue_edit(
        "Invalid schema version fixture",
        |markdown, _issue_id| markdown.replace("schema_version: 1", "schema_version: 99"),
        &["Unsupported schema_version 99"],
    );
    assert_lint_rejects_issue_edit(
        "ID path mismatch fixture",
        |markdown, issue_id| {
            markdown.replace(&format!("id: \"{issue_id}\""), "id: \"atelier-zzzz\"")
        },
        &["does not match canonical path"],
    );

    assert_lint_rejects_canonical_mutation(
        "Malformed activity sidecar fixture",
        |dir, issue_id| {
            let activity_path = dir
                .join(".atelier/issues")
                .join(format!("{issue_id}.activity"))
                .join("bad.md");
            std::fs::create_dir_all(activity_path.parent().unwrap()).unwrap();
            std::fs::write(activity_path, "not front matter\n").unwrap();
        },
        &["Missing YAML front matter", ".activity/bad.md"],
    );
    assert_lint_rejects_canonical_mutation(
        "Unsupported committed file fixture",
        |dir, _issue_id| {
            std::fs::write(dir.join(".atelier/issues/junk.txt"), "junk\n").unwrap();
        },
        &[
            "Unsupported canonical issue file",
            ".atelier/issues/junk.txt",
        ],
    );
    assert_lint_rejects_canonical_mutation(
        "Duplicate ID fixture",
        |dir, issue_id| {
            let (success, mission_out, stderr) =
                run_atelier(dir, &["mission", "create", "Duplicate ID mission"]);
            assert!(success, "mission create failed: {stderr}");
            let mission_id = mission_out
                .lines()
                .find_map(|line| {
                    line.strip_prefix("Mission ")
                        .and_then(|rest| rest.split(':').next())
                })
                .expect("mission create output should include an id")
                .to_string();
            let old_path = dir
                .join(".atelier/missions")
                .join(format!("{mission_id}.md"));
            let new_path = dir.join(".atelier/missions").join(format!("{issue_id}.md"));
            let mission_markdown = std::fs::read_to_string(&old_path).unwrap().replace(
                &format!("id: \"{mission_id}\""),
                &format!("id: \"{issue_id}\""),
            );
            std::fs::write(&new_path, mission_markdown).unwrap();
            std::fs::remove_file(old_path).unwrap();
        },
        &["Duplicate record ID in canonical projection"],
    );
}

fn assert_lint_rejects_issue_edit(
    title: &str,
    edit: impl FnOnce(&str, &str) -> String,
    expected: &[&str],
) {
    assert_lint_rejects_canonical_mutation(
        title,
        |dir, issue_id| {
            let issue_path = dir.join(".atelier/issues").join(format!("{issue_id}.md"));
            let markdown = std::fs::read_to_string(&issue_path).unwrap();
            std::fs::write(&issue_path, edit(&markdown, issue_id)).unwrap();
        },
        expected,
    );
}

fn assert_lint_rejects_canonical_mutation(
    title: &str,
    mutate: impl FnOnce(&Path, &str),
    expected: &[&str],
) {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let body = format!(
        "## Description\n\n{title} body.\n\n## Outcome\n\nCanonical lint rejects the targeted malformed record.\n\n## Evidence\n\n- `atelier lint` reports the targeted malformed canonical record."
    );

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", title, "--description", &body],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);

    mutate(dir.path(), &issue_id);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(
        !success,
        "lint should reject {title}, stdout: {stdout}, stderr: {stderr}"
    );
    let transcript = format!("{stdout}\n{stderr}");
    assert!(
        transcript.contains("Canonical tracker Markdown is invalid")
            || transcript.contains("Lint found"),
        "lint should identify canonical markdown failure for {title}: {transcript}"
    );
    for needle in expected {
        assert!(
            transcript.contains(needle),
            "lint diagnostic for {title} missing {needle:?}: {transcript}"
        );
    }
}

#[test]
fn test_bulk_plan_apply_records_links_export_and_rebuild() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let bulk_path = dir.path().join("bulk-plan.json");
    std::fs::write(
        &bulk_path,
        r#"{
  "schema": "atelier.bulk-plan",
  "schema_version": 1,
  "title": "Bulk apply smoke",
  "apply": { "export": "auto" },
  "records": {
    "issues": [
      {
        "client_ref": "issue.blocker",
        "title": "Complete prerequisite",
        "issue_type": "task",
        "priority": "medium",
        "status": "closed",
        "labels": ["bulk"]
      },
      {
        "client_ref": "issue.work",
        "title": "Implement bulk output",
        "issue_type": "feature",
        "priority": "high",
        "depends_on": [{ "client_ref": "issue.blocker" }],
        "acceptance": ["summary maps client refs"],
        "evidence_required": ["export check passes"]
      }
    ],
    "missions": [
      {
        "client_ref": "mission.bulk",
        "title": "Bulk mission",
        "body": "Mission from bulk plan",
        "work": [{ "client_ref": "issue.work" }],
        "plans": [{ "client_ref": "plan.bulk" }],
        "milestones": [{ "client_ref": "milestone.bulk" }]
      }
    ],
    "milestones": [
      {
        "client_ref": "milestone.bulk",
        "title": "Bulk checkpoint",
        "desired_state": "Records are durable",
        "scope": ["records"],
        "validation_criteria": ["rebuild preserves links"],
        "missions": [{ "client_ref": "mission.bulk" }],
        "contributing_work": [{ "client_ref": "issue.work" }]
      }
    ],
    "plans": [
      {
        "client_ref": "plan.bulk",
        "title": "Bulk plan",
        "body": "Apply the graph.",
        "applies_to": [{ "client_ref": "mission.bulk" }]
      }
    ],
    "evidence": [
      {
        "client_ref": "evidence.bulk",
        "title": "Bulk evidence",
        "evidence_type": "test",
        "result": "pass",
        "body": "The apply smoke test passed.",
        "validates": [{ "client_ref": "mission.bulk" }]
      }
    ]
  }
}"#,
    )
    .unwrap();
    let bulk_arg = bulk_path.to_str().unwrap();

    let (success, dry_run_out, stderr) =
        run_atelier(dir.path(), &["plan", "apply", bulk_arg, "--dry-run"]);
    assert!(success, "bulk dry-run failed: {stderr}");
    assert!(dry_run_out.contains("Bulk plan preview is valid."));
    assert!(dry_run_out.contains("Applied:       false"));
    assert!(dry_run_out.contains("missions: 1"));

    let (success, apply_out, stderr) = run_atelier(dir.path(), &["plan", "apply", bulk_arg]);
    assert!(success, "bulk apply failed: {stderr}");
    assert!(apply_out.contains("Bulk plan applied."));
    assert!(apply_out.contains("Applied:       true"));
    assert!(apply_out.contains("atelier mission show"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Bulk mission");

    let (success, _, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(success, "export check after bulk apply failed: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild after bulk apply failed: {stderr}");

    let (success, view_out, stderr) = run_atelier(dir.path(), &["mission", "show", &mission_id]);
    assert!(success, "mission show after bulk apply failed: {stderr}");
    assert!(view_out.contains("Records: plans=1 milestones=1 evidence=1"));
    assert!(view_out.contains("Work: ready=1 blocked=0 done=0 backlog=0"));
}

#[test]
fn test_work_lifecycle_human_output_and_guards() {
    let dir = tempdir().unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["init", "-q"])
        .status()
        .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["config", "user.email", "test@example.com"])
        .status()
        .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["config", "user.name", "Test"])
        .status()
        .unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "create", "Work item"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = stdout
        .split_whitespace()
        .find(|part| part.starts_with("atelier-"))
        .unwrap()
        .to_string();

    let (success, _, _) = run_atelier(dir.path(), &["work", "start", &issue_id]);
    assert!(!success, "dirty worktree should reject work start");

    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    std::fs::write(
        dir.path().join("atelier.workflow.yaml"),
        r#"schema: atelier.workflow_config
schema_version: 1
record_types: {}
workflows: {}
validators: {}
hooks:
  write_setup_marker:
    event: worktree_setup
    command:
      argv: [sh, -c, "printf setup > .atelier/setup-marker"]
      env: {}
"#,
    )
    .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["add", "."])
        .status()
        .unwrap();
    Command::new("git")
        .current_dir(dir.path())
        .args(["commit", "-q", "-m", "init"])
        .status()
        .unwrap();

    let (success, start_out, stderr) = run_atelier(dir.path(), &["work", "start", &issue_id]);
    assert!(success, "work start failed: {stderr}");
    assert!(start_out.contains(&format!("Started work on {issue_id}")));
    assert!(start_out.contains("Branch:"));
    assert!(start_out.contains("Worktree:"));

    let (success, status_out, stderr) = run_atelier(dir.path(), &["work", "status"]);
    assert!(success, "work status failed: {stderr}");
    assert!(status_out.contains(&format!("Issue:    {issue_id} - Work item")));

    let (success, status_human, stderr) = run_atelier(dir.path(), &["work", "status"]);
    assert!(success, "human work status failed: {stderr}");
    assert!(status_human.contains("Work Status"));
    assert!(status_human.contains("Active:   yes"));
    assert!(status_human.contains(&format!("Issue:    {issue_id} - Work item")));
    assert!(status_human.contains("Branch:"));
    assert!(status_human.contains("Worktree:"));

    let (success, finish_out, stderr) = run_atelier(dir.path(), &["work", "finish", &issue_id]);
    assert!(success, "work finish failed: {stderr}");
    assert!(finish_out.contains(&format!("Finished work on {issue_id}")));
    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "work_started",
        &["branch: ", "worktree_path: "],
    );
    assert_activity_contains(&activities, "work_finished", &["finished: true"]);

    let worktree_path = dir.path().join(".atelier-worktrees").join(&issue_id);
    let worktree_arg = worktree_path.to_string_lossy().to_string();
    let (success, worktree_out, stderr) = run_atelier(
        dir.path(),
        &["worktree", "for", &issue_id, "--path", &worktree_arg],
    );
    assert!(success, "worktree for failed: {stderr}");
    assert!(worktree_out.contains(&worktree_arg));
    assert!(worktree_path.join(".atelier/state.db").exists());
    assert!(worktree_path.join(".atelier/setup-marker").exists());

    let (success, status_out, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(success, "worktree status failed: {stderr}");
    assert!(status_out.contains(&worktree_arg));
    assert!(status_out.contains(&format!("{issue_id} [active]")));

    let (success, status_human, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(success, "human worktree status failed: {stderr}");
    assert!(status_human.contains("Worktree Status"));
    assert!(status_human.contains(&worktree_arg));
    assert!(status_human.contains("Branch:"));
    assert!(status_human.contains("State:"));
    assert!(status_human.contains("Associated Work"));
    assert!(status_human.contains(&format!("{issue_id} [active]")));
    assert!(!status_human.contains("work:"));
    assert!(!status_human.contains("export:"));

    let (success, remove_out, stderr) =
        run_atelier(dir.path(), &["worktree", "remove", &issue_id, "--force"]);
    assert!(success, "worktree remove failed: {stderr}");
    assert!(remove_out.contains("Removed worktree"));
    assert!(!worktree_path.exists());
}

#[test]
fn test_start_refuses_shared_section_diagnostic() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Malformed section work"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(stdout.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Malformed section work");
    commit_all(dir.path(), "valid issue");

    let issue_path = dir
        .path()
        .join(".atelier/issues")
        .join(format!("{issue_id}.md"));
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    let malformed = markdown.replace("\n## Outcome\n\nOutcome was not specified.\n", "\n");
    std::fs::write(&issue_path, malformed).unwrap();
    commit_all(dir.path(), "malformed issue section");

    let (lint_success, lint_stdout, lint_stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(!lint_success, "lint should report malformed issue sections");
    let lint_transcript = format!("{lint_stdout}\n{lint_stderr}");
    for needle in [
        "Missing required issue body section 'Outcome'",
        &issue_id,
        "section Outcome",
        ".atelier/issues/",
    ] {
        assert!(
            lint_transcript.contains(needle),
            "lint diagnostic missing {needle:?}: {lint_transcript}"
        );
    }

    let (start_success, start_stdout, start_stderr) =
        run_atelier(dir.path(), &["start", &issue_id]);
    assert!(
        !start_success,
        "start should refuse malformed issue sections"
    );
    let start_transcript = format!("{start_stdout}\n{start_stderr}");
    for needle in [
        "Missing required issue body section 'Outcome'",
        &issue_id,
        "section Outcome",
        ".atelier/issues/",
    ] {
        assert!(
            start_transcript.contains(needle),
            "start diagnostic missing {needle:?}: {start_transcript}"
        );
    }
}

#[test]
fn test_issue_type_is_canonical_not_label_derived() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Typed issue",
            "--issue-type",
            "validation",
            "--label",
            "epic",
        ],
    );
    assert!(success, "create failed: {stderr}");
    assert!(stdout.contains("Type:     validation"));
    let issue_id = issue_id_by_title(dir.path(), "Typed issue");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("[validation] open - Typed issue"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(success, "list failed: {stderr}");
    assert!(stdout.contains("validation"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success, "ready failed: {stderr}");
    assert!(stdout.contains("validation"));

    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    let issue_record = std::fs::read_to_string(
        dir.path()
            .join(".atelier/issues")
            .join(format!("{issue_id}.md")),
    )
    .unwrap();
    assert!(issue_record.contains("issue_type: \"validation\"\n"));
    assert!(issue_record.contains("labels:\n- \"epic\"\n"));
}

#[test]
fn test_import_beads_reports_mapping_without_tracker_provenance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/beads/issues.manual.jsonl");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["import-beads", fixture.to_str().unwrap()]);
    assert!(success, "import-beads failed: {stderr}");
    assert!(stdout.contains("imported issues: 3"));
    assert!(stdout.contains("blocking relationships: 1"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "3"]);
    assert!(success, "mapped show failed: {stderr}");
    assert!(stdout.contains("atelier-0003"));
    assert!(stdout.contains("[task]"));
    assert!(stdout.contains("Parent: atelier-0001"));
    assert!(stdout.contains("atelier-0002"));
    assert!(!stdout.contains("beads:"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["dep", "list", "3"]);
    assert!(success, "mapped dep list failed: {stderr}");
    assert!(stdout.contains("atelier-0003"));
    assert!(stdout.contains("atelier-0002"));
}

// ============================================================
// Unicode E2E Tests - Comprehensive multi-byte character handling
// ============================================================

/// Test issue creation and listing with Unicode arrows
#[test]
fn test_unicode_arrows_in_title() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // The exact issue that caused the original panic
    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Add keyboard shortcuts for swiping (← →)",
        ],
    );
    assert!(success);

    // List should not panic
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("←") || stdout.contains("...")); // Either shows or truncates
}

/// Test various Unicode characters in issue titles
#[test]
fn test_unicode_variety_in_titles() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let unicode_titles = vec![
        "日本語タイトル",                 // Japanese
        "中文标题测试",                   // Chinese
        "Тест на русском языке",          // Russian
        "العربية اختبار",                 // Arabic (RTL)
        "🎉 Emoji celebration 🎊🎈",      // Emoji
        "Mixed: Hello 世界 مرحبا мир 🌍", // Mixed scripts
        "Math: ∑∏∫∂ √∞ ≈≠≤≥",             // Math symbols
        "Arrows: ← → ↑ ↓ ↔ ↕ ⇐ ⇒",        // Arrows
        "Currency: $ € £ ¥ ₹ ₽ ₿",        // Currency
        "Box: ─│┌┐└┘├┤┬┴┼",               // Box drawing
    ];

    for (i, title) in unicode_titles.iter().enumerate() {
        let (success, _, _) = run_atelier(dir.path(), &["issue", "create", title]);
        assert!(success, "Failed to create issue with title: {}", title);

        // Verify it can be shown without panic
        let id = (i + 1).to_string();
        let (success, _, _) = run_atelier(dir.path(), &["issue", "show", &id]);
        assert!(
            success,
            "Failed to show issue #{} with title: {}",
            i + 1,
            title
        );
    }

    // List all - tests truncation on long Unicode
    let (success, _, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
}

/// Test Unicode in descriptions and comments
#[test]
fn test_unicode_in_descriptions_and_comments() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create with Unicode description
    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Unicode test",
            "-d",
            "Description with 日本語 and émojis 🚀",
        ],
    );
    assert!(success);

    // Add Unicode comment
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "comment", "1", "Comment: ← back, → forward, ↑ up"],
    );
    assert!(success);

    // Show should display without panic
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success);
    assert!(
        stdout.contains("日本語"),
        "Show output should contain the Unicode description text, got: {}",
        stdout
    );
}

/// Test search with Unicode queries
#[test]
fn test_unicode_search() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "日本語のテスト"]);
    run_atelier(dir.path(), &["issue", "create", "Test with arrows ← →"]);
    run_atelier(dir.path(), &["issue", "create", "Emoji test 🎉"]);

    // Search for Japanese
    let (success, _, _) = run_atelier(dir.path(), &["issue", "search", "日本"]);
    assert!(success);

    // Search for emoji
    let (success, _, _) = run_atelier(dir.path(), &["issue", "search", "🎉"]);
    assert!(success);

    // Search for arrow
    let (success, _, _) = run_atelier(dir.path(), &["issue", "search", "←"]);
    assert!(success);
}

/// Test very long Unicode strings (stress test truncation)
#[test]
fn test_unicode_long_string_truncation() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create title that's definitely longer than truncation limit
    // Using 3-byte UTF-8 chars (←) to maximize byte/char mismatch
    let long_arrows = "←".repeat(60);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "create", &format!("Long: {}", long_arrows)],
    );
    assert!(success);

    // List must not panic on truncation
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("...") || stdout.contains("Long:"));

    // Create title with mixed byte-length chars
    let mixed = "a←b→c↑d↓e🎉f".repeat(10);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", &mixed]);
    assert!(success);

    let (success, _, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
}

/// Test blocked/ready lists with Unicode
#[test]
fn test_unicode_in_dependencies() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "ブロッカー (blocker) ←"]);
    run_atelier(dir.path(), &["issue", "create", "待機中 (waiting) →"]);
    run_atelier(dir.path(), &["issue", "block", "2", "1"]);

    // Blocked list with Unicode
    let (success, _, _) = run_atelier(dir.path(), &["issue", "blocked"]);
    assert!(success);

    // Ready list
    let (success, _, _) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success);
}

/// Test export/import preserves Unicode
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_unicode_export_import_roundtrip() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let unicode_title = "Test: 日本語 ← → 🎉";
    let unicode_desc = "Description: 中文 العربية Русский";

    run_atelier(
        dir.path(),
        &["issue", "create", unicode_title, "-d", unicode_desc],
    );
    run_atelier(dir.path(), &["issue", "comment", "1", "コメント (comment)"]);

    // Export
    let export_path = dir.path().join("unicode_backup.json");
    let (success, _, _) = run_atelier(
        dir.path(),
        &["export", "-o", export_path.to_str().unwrap(), "-f", "json"],
    );
    assert!(success);

    // Import to new location
    let dir2 = tempdir().unwrap();
    init_atelier(dir2.path());
    std::fs::copy(&export_path, dir2.path().join("unicode_backup.json")).unwrap();

    let (success, _, _) = run_atelier(
        dir2.path(),
        &[
            "import",
            dir2.path().join("unicode_backup.json").to_str().unwrap(),
        ],
    );
    assert!(success);

    // Verify Unicode preserved
    let unicode_id = issue_id_by_title(dir2.path(), unicode_title);
    let (success, stdout, _) = run_atelier(dir2.path(), &["issue", "show", &unicode_id]);
    assert!(success);
    assert!(
        stdout.contains("日本語") || stdout.contains("Test:"),
        "Unicode should be preserved in export/import"
    );
}

/// Test zero-width and special Unicode characters
#[test]
fn test_unicode_special_characters() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Zero-width characters (shouldn't break anything)
    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Test\u{200B}with\u{200B}zero\u{200B}width",
        ],
    );
    assert!(success);

    // RTL override characters
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "create", "Test \u{202E}desrever\u{202C} normal"],
    );
    assert!(success);

    // Combining characters (accent marks)
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", "Café résumé naïve"]);
    assert!(success);

    // All should list without panic
    let (success, _, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
}
