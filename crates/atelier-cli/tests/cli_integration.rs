#![allow(unused_variables)]

use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use tempfile::tempdir;

static TEST_ISSUE_IDS: OnceLock<Mutex<HashMap<PathBuf, Vec<String>>>> = OnceLock::new();

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("atelier-cli crate should be nested under workspace crates/")
        .to_path_buf()
}

/// Helper to run atelier commands in a temp directory
fn run_atelier(dir: &Path, args: &[&str]) -> (bool, String, String) {
    ensure_git_for_workflow_fixture(dir, args);
    run_atelier_raw(dir, args)
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
        register_issue_ids_fromstdout(dir, &stdout);
        register_issue_ids_from_state(dir);
    }

    (output.status.success(), stdout, stderr)
}

fn run_atelier_with_env(
    dir: &Path,
    args: &[&str],
    envs: &[(&str, &str)],
) -> (bool, String, String) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_atelier"));
    command.current_dir(dir).args(args);
    for (key, value) in envs {
        command.env(key, value);
    }
    let output = command.output().expect("Failed to execute atelier");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        register_issue_ids_fromstdout(dir, &stdout);
        register_issue_ids_from_state(dir);
    }

    (output.status.success(), stdout, stderr)
}

/// Initialize atelier in a temp directory
fn init_atelier(dir: &Path) {
    init_atelier_without_workflow(dir);
    migrate_default_issue_workflow(dir);
}

fn init_atelier_without_workflow(dir: &Path) {
    let (success, _, stderr) = run_atelier(dir, &["init"]);
    assert!(success, "Failed to init: {}", stderr);
    let workflow_path = dir.join(".atelier").join("workflow.yaml");
    if workflow_path.exists() {
        fs::remove_file(workflow_path).unwrap();
    }
}

fn migrate_default_issue_workflow(dir: &Path) {
    let workflow_path = dir.join(".atelier").join("workflow.yaml");
    if workflow_path.exists() {
        return;
    }
    fs::write(&workflow_path, atelier_workflow::STARTER_POLICY_YAML)
        .expect("failed to write starter workflow policy");
}

fn init_git_repo(dir: &Path) {
    if !dir.join(".git").exists() {
        let status = Command::new("git")
            .current_dir(dir)
            .args(["init", "-q"])
            .status()
            .unwrap();
        assert!(status.success(), "git init failed");
    }
    let status = Command::new("git")
        .current_dir(dir)
        .args(["branch", "-M", "main"])
        .status()
        .unwrap();
    assert!(status.success(), "git branch -M main failed");
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
    ensure_initial_git_commit(dir);
}

fn ensure_git_for_workflow_fixture(dir: &Path, args: &[&str]) {
    let needs_git = matches!(
        args,
        ["issue", "transition", ..] | ["issue", "close", ..] | ["abandon", ..]
    );
    if needs_git && !dir.join(".git").exists() {
        init_git_repo(dir);
        commit_if_dirty(dir, "test fixture state before workflow command");
    }
    if matches!(args, ["issue", "transition", _, "--options"]) && dir.join(".git").exists() {
        commit_if_dirty(dir, "test fixture state before workflow options");
    }
    if matches!(args, ["issue", "close", ..]) && dir.join(".git").exists() {
        commit_if_dirty(dir, "test fixture state before transition close");
    }
}

fn ensure_initial_git_commit(dir: &Path) {
    if git_has_commit(dir) {
        return;
    }
    if git_has_changes(dir) {
        commit_all(dir, "initial tracker state");
    } else {
        let output = Command::new("git")
            .current_dir(dir)
            .args([
                "commit",
                "--allow-empty",
                "-q",
                "-m",
                "initial tracker state",
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git empty commit failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn git_has_commit(dir: &Path) -> bool {
    Command::new("git")
        .current_dir(dir)
        .args(["rev-parse", "--verify", "HEAD"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn commit_if_dirty(dir: &Path, message: &str) {
    if !git_has_changes(dir) && git_has_commit(dir) {
        return;
    }
    commit_all(dir, message);
}

fn git_has_changes(dir: &Path) -> bool {
    let output = Command::new("git")
        .current_dir(dir)
        .args(["status", "--porcelain"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git status --porcelain failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    !output.stdout.is_empty()
}

fn commit_all(dir: &Path, message: &str) {
    let status = Command::new("git")
        .current_dir(dir)
        .args(["add", "-A"])
        .status()
        .unwrap();
    assert!(status.success(), "git add failed");
    let status = Command::new("git")
        .current_dir(dir)
        .args(["diff", "--cached", "--quiet"])
        .status()
        .unwrap();
    if status.success() {
        return;
    }
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

fn complete_room_review(dir: &Path, issue_id: &str) {
    let (success, _, stderr) = run_atelier(
        dir,
        &[
            "review",
            "approve",
            "--issue",
            issue_id,
            "--role",
            "reviewer",
            "--body",
            "fixture approval",
        ],
    );
    assert!(success, "review approve failed for {issue_id}: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir,
        &["review", "merge", "--issue", issue_id, "--role", "manager"],
    );
    assert!(success, "review merge failed for {issue_id}: {stderr}");
}

fn git_status_short(dir: &Path) -> String {
    let output = Command::new("git")
        .current_dir(dir)
        .args(["status", "--short"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git status --short failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn git_current_branch(dir: &Path) -> String {
    let output = Command::new("git")
        .current_dir(dir)
        .args(["branch", "--show-current"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git branch --show-current failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn git_rev_parse(dir: &Path, rev: &str) -> String {
    let output = Command::new("git")
        .current_dir(dir)
        .args(["rev-parse", rev])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git rev-parse {rev} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn git_log_oneline(dir: &Path, rev: &str, count: usize) -> String {
    let output = Command::new("git")
        .current_dir(dir)
        .args(["log", "--oneline", &format!("-{count}"), rev])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git log failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn init_atelier_with_telemetry_disabled(dir: &Path) {
    let (success, _, stderr) =
        run_atelier_with_env(dir, &["init"], &[("ATELIER_TELEMETRY", "off")]);
    assert!(success, "Failed to init: {}", stderr);
    migrate_default_issue_workflow(dir);
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

fn canonical_record_path(dir: &Path, directory: &str, record_id: &str) -> PathBuf {
    dir.join(".atelier")
        .join(directory)
        .join(format!("{record_id}.md"))
}

fn canonical_issue_path(dir: &Path, issue_id: &str) -> PathBuf {
    canonical_record_path(dir, "issues", issue_id)
}

fn read_canonical_record(dir: &Path, directory: &str, record_id: &str) -> String {
    let path = canonical_record_path(dir, directory, record_id);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn write_canonical_record(dir: &Path, directory: &str, record_id: &str, markdown: String) {
    let path = canonical_record_path(dir, directory, record_id);
    std::fs::write(&path, markdown)
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", path.display()));
}

fn edit_canonical_record(
    dir: &Path,
    directory: &str,
    record_id: &str,
    edit: impl FnOnce(String) -> String,
) {
    let markdown = read_canonical_record(dir, directory, record_id);
    write_canonical_record(dir, directory, record_id, edit(markdown));
}

fn edit_canonical_issue(dir: &Path, issue_id: &str, edit: impl FnOnce(String) -> String) {
    edit_canonical_record(dir, "issues", issue_id, edit);
}

fn make_issue_terminal_before_retention(dir: &Path, issue_id: &str, days_old: u64) {
    let timestamp = chrono::Utc::now()
        .checked_sub_days(chrono::Days::new(days_old))
        .unwrap()
        .to_rfc3339();
    edit_canonical_issue(dir, issue_id, |markdown| {
        let markdown = replace_front_matter_scalar(&markdown, "status", "done");
        let markdown = replace_front_matter_scalar(&markdown, "updated_at", &timestamp);
        if markdown.lines().any(|line| line.starts_with("closed_at: ")) {
            replace_front_matter_scalar(&markdown, "closed_at", &timestamp)
        } else {
            markdown.replace(
                &format!("updated_at: {timestamp:?}"),
                &format!("updated_at: {timestamp:?}\nclosed_at: {timestamp:?}"),
            )
        }
    });
}

fn make_record_before_retention(dir: &Path, directory: &str, record_id: &str, days_old: u64) {
    let timestamp = chrono::Utc::now()
        .checked_sub_days(chrono::Days::new(days_old))
        .unwrap()
        .to_rfc3339();
    edit_canonical_record(dir, directory, record_id, |markdown| {
        let markdown = replace_front_matter_scalar(&markdown, "created_at", &timestamp);
        let markdown = replace_front_matter_scalar(&markdown, "updated_at", &timestamp);
        replace_front_matter_scalar(&markdown, "captured_at", &timestamp)
    });
}

fn set_prune_canonical_retention_days(dir: &Path, days: u64) {
    let path = dir.join(".atelier/config.toml");
    let config = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    let updated = if config.contains("[prune]") {
        config
            .lines()
            .map(|line| {
                if line.starts_with("canonical_retention_days = ") {
                    format!("canonical_retention_days = {days}")
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    } else {
        config.replace(
            "[review]",
            &format!("[prune]\ncanonical_retention_days = {days}\n\n[review]"),
        )
    };
    fs::write(&path, updated)
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", path.display()));
}

fn set_issue_description(dir: &Path, issue_id: &str, description: &str) {
    if description.trim().is_empty() {
        return;
    }
    edit_canonical_issue(dir, issue_id, |markdown| {
        if description.contains("## Description") {
            replace_markdown_body(&markdown, description)
        } else {
            replace_issue_section(&markdown, "Description", description)
        }
    });
}

fn replace_front_matter_scalar(markdown: &str, key: &str, value: &str) -> String {
    let needle = format!("{key}: ");
    markdown
        .lines()
        .map(|line| {
            if line.starts_with(&needle) {
                format!("{key}: {value:?}")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn replace_markdown_body(markdown: &str, body: &str) -> String {
    let (_, _) = markdown
        .split_once("\n---\n")
        .expect("canonical record front matter terminator missing");
    let body_start = markdown.find("\n---\n").unwrap() + "\n---\n".len();
    format!("{}{}\n", &markdown[..body_start], body.trim())
}

fn replace_issue_section(markdown: &str, heading: &str, replacement: &str) -> String {
    let marker = format!("## {heading}\n");
    let start = markdown.find(&marker).expect("section heading missing");
    let body_start = start + marker.len();
    let rest = &markdown[body_start..];
    let end = rest
        .find("\n## ")
        .map(|offset| body_start + offset)
        .unwrap_or(markdown.len());
    format!(
        "{}{}\n{}",
        &markdown[..body_start],
        format!("\n{}\n", replacement.trim()),
        &markdown[end..]
    )
}

fn write_ignored_canonical_artifacts(dir: &Path, issue_id: &str) {
    let runtime_dir = dir.join(".atelier/runtime");
    std::fs::create_dir_all(&runtime_dir).unwrap();
    std::fs::write(
        runtime_dir.join(".state.db.123.456.rebuild-tmp"),
        "partial sqlite rebuild",
    )
    .unwrap();
    std::fs::write(
        runtime_dir.join(".state.db.123.456.rebuild-tmp-journal"),
        "partial sqlite rebuild journal",
    )
    .unwrap();
    let cache_dir = dir.join(".atelier/cache");
    std::fs::create_dir_all(&cache_dir).unwrap();
    std::fs::write(cache_dir.join("projection.lock"), "cache lock").unwrap();
    let issue_dir = dir.join(".atelier/issues");
    std::fs::write(issue_dir.join(format!("{issue_id}.md.lock")), "lock").unwrap();
    std::fs::write(issue_dir.join(format!("{issue_id}.md-journal")), "journal").unwrap();
    std::fs::write(
        issue_dir.join(format!(".{issue_id}.md.123.456.tmp")),
        "partial canonical write",
    )
    .unwrap();
}

fn corrupt_issue_title_yaml(dir: &Path, issue_id: &str, title: &str) {
    edit_canonical_issue(dir, issue_id, |markdown| {
        markdown.replace(&format!("title: {title:?}"), &format!("title: [{title}"))
    });
}

fn remove_projection_state(dir: &Path) {
    std::fs::remove_file(dir.join(".atelier/runtime/state.db")).unwrap();
}

fn canonical_evidence_front_matter(dir: &Path, evidence_id: &str) -> serde_json::Value {
    canonical_record_front_matter(dir, "evidence", evidence_id)
}

fn canonical_record_front_matter(
    dir: &Path,
    directory: &str,
    record_id: &str,
) -> serde_json::Value {
    let text = read_canonical_record(dir, directory, record_id);
    let path = canonical_record_path(dir, directory, record_id);
    let front = text
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .map(|(front, _)| front)
        .unwrap_or_else(|| panic!("missing front matter in {}", path.display()));
    let yaml: serde_yaml::Value = serde_yaml::from_str(front).unwrap();
    serde_json::to_value(yaml).unwrap()
}

fn ignored_test_source(ignore_attribute: &str, test_name: &str) -> String {
    format!("#[test]\n#[{ignore_attribute}]\nfn {test_name}() {{}}\n")
}

fn valid_command_surface_doc() -> &'static str {
    r#"# CLI Surface Tiers

## Core

- `atelier init`
- `atelier man`
- `atelier status`
- `atelier issue ...`
- `atelier issue transition <issue-id> start`
- `atelier issue create "..." --issue-type mission`
- `atelier issue show <objective-id>`
- `atelier issue status <objective-id>`
- `atelier issue link <objective-id> <issue-id> --role advances`
- `atelier issue block <objective-id> <issue-id>`
- `atelier search <query>`
- `atelier issue note`
- `atelier bundle preview/apply`
- `atelier evidence record/show/list/attach`
- `atelier review open/status/show/comments/comment/approve/request-changes`
- `atelier forgejo roles check`
- `atelier history`
- `atelier prune`
- `atelier maintenance delete`
- `atelier lint`

## Advanced Repair

- `atelier branch for-epic/status/merge`
- `atelier doctor`
"#
}

fn write_command_surface_doc(dir: &Path, content: &str) {
    let docs_dir = dir.join("docs/product");
    fs::create_dir_all(&docs_dir).unwrap();
    fs::write(docs_dir.join("cli-surface.md"), content).unwrap();
}

fn write_valid_command_guidance(dir: &Path) {
    write_command_surface_doc(dir, valid_command_surface_doc());
    fs::write(
        dir.join("AGENTS.md"),
        "# Agent Instructions\n\n- `atelier issue list --ready`\n- `atelier lint`\n",
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
    let search_directory = if directory == "missions" {
        "issues"
    } else {
        directory
    };
    let record_dir = dir.join(".atelier").join(search_directory);
    let entries = std::fs::read_dir(&record_dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", record_dir.display()));
    for entry in entries {
        let path = entry.unwrap().path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let record_id = path.file_stem().unwrap().to_string_lossy().to_string();
        let text = read_canonical_record(dir, search_directory, &record_id);
        if directory == "missions"
            && !text.contains("issue_type: mission")
            && !text.contains("issue_type: \"mission\"")
            && !text.contains("issue_type: 'mission'")
        {
            continue;
        }
        if text.contains(&format!("title: {title:?}"))
            || text.contains(&format!("title: {title}\n"))
        {
            return record_id;
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

fn attach_evidence(
    dir: &Path,
    target_kind: &str,
    target_id: &str,
    result: &str,
    summary: &str,
) -> String {
    let target_kind = if target_kind == "mission" {
        "issue"
    } else {
        target_kind
    };
    if target_kind == "issue" {
        ensure_all_issue_completion_sections(dir);
    }
    let (success, evidence_out, stderr) = run_atelier(
        dir,
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--target",
            &format!("{target_kind}/{target_id}"),
            summary,
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    assert!(
        evidence_out.contains("[evidence] recorded"),
        "{evidence_out}"
    );
    let evidence_id = record_id_by_title(dir, "evidence", summary);
    let success = result == "pass";
    edit_canonical_record(dir, "evidence", &evidence_id, |markdown| {
        let markdown = replace_front_matter_scalar(&markdown, "status", result);
        if markdown.contains("\nsuccess:") {
            markdown
                .lines()
                .map(|line| {
                    if line.starts_with("success:") {
                        format!("success: {}", if success { "true" } else { "false" })
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
                + "\n"
        } else {
            markdown.replace(
                "schema_version: 1\n",
                &format!(
                    "schema_version: 1\nsuccess: {}\n",
                    if success { "true" } else { "false" }
                ),
            )
        }
    });
    let rebuild = run_atelier_raw(dir, &["rebuild"]);
    assert!(
        rebuild.0,
        "test fixture projection rebuild failed after evidence result edit: {}",
        rebuild.2
    );
    evidence_id
}

fn attach_pass_evidence(dir: &Path, target_kind: &str, target_id: &str, summary: &str) -> String {
    attach_evidence(dir, target_kind, target_id, "pass", summary)
}

fn attach_non_pass_evidence(
    dir: &Path,
    target_kind: &str,
    target_id: &str,
    result: &str,
    summary: &str,
) -> String {
    assert!(result != "pass");
    attach_evidence(dir, target_kind, target_id, result, summary)
}

fn attach_issue_pass_evidence(dir: &Path, issue_id: &str) -> String {
    ensure_all_issue_completion_sections(dir);
    attach_pass_evidence(
        dir,
        "issue",
        issue_id,
        &format!("transition close proof for {issue_id}"),
    )
}

fn ensure_issue_completion_sections(dir: &Path, issue_id: &str) {
    edit_canonical_issue(dir, issue_id, |mut markdown| {
        markdown = markdown.replace(
            "Outcome was not specified.",
            &format!(
                "The issue outcome is complete and ready for terminal checks.\n\n## Evidence\n\n{}",
                format!("- Evidence record attached to issue/{issue_id} validates completion.")
            ),
        );
        markdown.replace(
            "Evidence was not specified.",
            &format!("- Evidence record attached to issue/{issue_id} validates completion."),
        )
    });
}

fn ensure_all_issue_completion_sections(dir: &Path) {
    let issues_dir = dir.join(".atelier").join("issues");
    if !issues_dir.exists() {
        return;
    }
    for entry in std::fs::read_dir(&issues_dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", issues_dir.display()))
    {
        let path = entry.unwrap().path();
        if path.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        if let Some(issue_id) = path.file_stem().and_then(|value| value.to_str()) {
            ensure_issue_completion_sections(dir, issue_id);
        }
    }
}

fn move_issue_to_validation(dir: &Path, issue_ref_value: &str) -> String {
    migrate_default_issue_workflow(dir);
    let issue_id = resolve_test_issue_ref(dir, issue_ref_value);
    for transition in ["start", "request_review", "request_validation"] {
        let (success, options, stderr) =
            run_atelier(dir, &["issue", "transition", &issue_id, "--options"]);
        assert!(
            success,
            "transition options failed for {issue_id}: {stderr}"
        );
        let option_present = options.contains(&format!("{transition} ["));
        if !option_present {
            continue;
        }
        let (success, _, stderr) = if transition == "start" && dir.join(".git").exists() {
            run_atelier(dir, &["issue", "transition", &issue_id, "start"])
        } else {
            run_atelier(dir, &["issue", "transition", &issue_id, transition])
        };
        if options.contains(&format!("{transition} [allowed]")) {
            assert!(success, "{transition} failed for {issue_id}: {stderr}");
            if transition == "request_review" {
                complete_room_review(dir, &issue_id);
            }
        }
    }
    issue_id
}

fn close_issue_with_evidence(dir: &Path, issue_ref_value: &str, reason: Option<&str>) -> String {
    let issue_id = resolve_test_issue_ref(dir, issue_ref_value);
    move_issue_to_validation(dir, &issue_id);
    ensure_all_issue_completion_sections(dir);
    attach_issue_pass_evidence(dir, &issue_id);
    if dir.join(".git").exists() {
        commit_all(dir, &format!("ready to close {issue_id}"));
    }
    let mut args = vec!["issue", "transition", &issue_id, "close"];
    args.push("--reason");
    args.push(reason.unwrap_or("done"));
    let (success, _, stderr) = run_atelier(dir, &args);
    assert!(success, "issue transition close failed: {stderr}");
    issue_id
}

fn write_provider_config_without_role_authors(dir: &Path) {
    fs::create_dir_all(dir.join(".atelier")).unwrap();
    fs::write(
        dir.join(".atelier/config.toml"),
        r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "atelier-test"

[paths]
state_root = ".atelier"

[review]
mode = "provider"
provider = "forgejo"

[review.providers.forgejo]
host = "https://forge.example.test"
owner = "tools"
repo = "atelier"
admin_token_env = "ATELIER_CLI_INTEGRATION_FORGEJO_TOKEN"
"#,
    )
    .unwrap();
}

fn write_provider_review_action_workflow(dir: &Path) {
    let workflow = atelier_workflow::STARTER_POLICY_YAML.replace(
        "          - review.open: { role: worker }",
        "          - review.open:\n              provider: forgejo\n              role: worker\n              role_authors:\n                worker: forge-worker\n                reviewer: forge-reviewer\n                validator: forge-validator\n                manager: forge-manager",
    );
    fs::write(dir.join(".atelier/workflow.yaml"), workflow).unwrap();
}

fn write_branch_action_workflow(dir: &Path) {
    let task_start_without_actions =
        "      start:\n        from: [todo, blocked]\n        to: in_progress\n        description: \"Start active work on this item.\"\n";
    let task_start_with_prepare =
        "      start:\n        from: [todo, blocked]\n        to: in_progress\n        description: \"Start active work on this item.\"\n        actions:\n          - branch_prepare\n";
    let epic_start_without_actions = "      start:\n        from: [todo, blocked]\n        to: in_progress\n        description: \"Start active work on this item.\"\n        validators:\n          - git.on_base_branch\n";
    let epic_start_with_prepare = "      start:\n        from: [todo, blocked]\n        to: in_progress\n        description: \"Start active work on this item.\"\n        validators:\n          - git.on_base_branch\n        actions:\n          - branch_prepare\n";
    let mut workflow = atelier_workflow::STARTER_POLICY_YAML.replacen(
        task_start_without_actions,
        task_start_with_prepare,
        1,
    );
    workflow = workflow.replace(epic_start_without_actions, epic_start_with_prepare);
    workflow = workflow.replace(
        "          - tracker.current\n\n  epic_delivery:",
        "          - tracker.current\n        actions:\n          - tracker.commit\n          - branch_integrate\n\n  epic_delivery:",
    );
    workflow = workflow.replace(
        "          - tracker.commit\n          - branch.push\n          - review.merge\n          - base.sync",
        "          - tracker.commit\n          - branch_integrate",
    );
    fs::write(dir.join(".atelier/workflow.yaml"), workflow).unwrap();
}

#[test]
fn provider_review_open_action_reads_workflow_config_and_env_secret() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    write_provider_config_without_role_authors(dir.path());
    write_provider_review_action_workflow(dir.path());

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Provider epic", "--issue-type", "epic"],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Provider epic");

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "start failed: {stderr}");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(stdout.contains("request_review [blocked]"), "{stdout}");
    assert!(stdout.contains("review.open"), "{stdout}");
    assert!(stdout.contains("provider=forgejo"), "{stdout}");
    assert!(stdout.contains("role=worker"), "{stdout}");
    assert!(
        stdout.contains("ATELIER_CLI_INTEGRATION_FORGEJO_TOKEN"),
        "{stdout}"
    );
    assert!(!stdout.contains("role_authors"), "{stdout}");
}

#[test]
fn request_review_preserves_review_artifact_field() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Review field epic",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Review field epic");

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "start failed: {stderr}");
    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_review"],
    );
    assert!(success, "request_review failed: {stderr}");

    let front_matter = canonical_record_front_matter(dir.path(), "issues", &issue_id);
    assert_eq!(front_matter["status"], "review");
    assert_eq!(front_matter["review"]["kind"], "room");
    assert!(
        front_matter["review"]["id"]
            .as_str()
            .is_some_and(|id| id.starts_with("atelier-")),
        "{front_matter:#}"
    );
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
    if !canonical_issue_path(dir, &id).exists() {
        return;
    }
    let mut map = registry().lock().unwrap();
    let ids = map.entry(dir.to_path_buf()).or_default();
    if !ids.contains(&id) {
        ids.push(id);
    }
}

fn register_issue_ids_fromstdout(dir: &Path, stdout: &str) {
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

#[path = "cli_integration/issues.rs"]
mod issues;
#[path = "cli_integration/mission_projection_worktree.rs"]
mod mission_projection_worktree;
#[path = "cli_integration/records_evidence.rs"]
mod records_evidence;
#[path = "cli_integration/setup_guidance.rs"]
mod setup_guidance;
#[path = "cli_integration/unicode.rs"]
mod unicode;
