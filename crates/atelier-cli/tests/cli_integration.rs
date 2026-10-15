#![allow(unused_variables)]

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
    let (success, _, stderr) = run_atelier(dir, &["init", "--slug", "atelier"]);
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

fn move_mission_directly_to_ready(dir: &Path, mission_id: &str) {
    edit_canonical_issue(dir, mission_id, |markdown| {
        markdown
            .replace(
                "Success criteria were not specified.",
                "The mission scenario demonstrates its acceptance claims.",
            )
            .replace(
                "Goal was not specified.",
                "Coordinate an observable increment of tracker work.",
            )
    });
    let (success, _, stderr) = run_atelier(dir, &["issue", "transition", mission_id, "ready"]);
    assert!(success, "mission ready transition failed: {stderr}");
}

/// Put an epic fixture at the execution boundary without exercising expansion.
///
/// Lifecycle integration tests use this when their subject is branching, review,
/// or closeout rather than the draft-to-ready validators themselves.
fn move_epic_directly_to_ready(dir: &Path, epic_id: &str) {
    edit_canonical_issue(dir, epic_id, |markdown| {
        let markdown = replace_front_matter_scalar(&markdown, "status", "todo");
        if markdown.contains("## Proof") {
            markdown
        } else {
            format!("{markdown}\n## Proof\n\nFocused tests show the outcome.\n")
        }
    });
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
        ["issue", "transition", ..]
            | ["issue", "close", ..]
            | ["abandon", ..]
            | ["evidence", "record", ..]
            | ["--quiet", "evidence", "record", ..]
    );
    if needs_git && !dir.join(".git").exists() {
        init_git_repo(dir);
        commit_if_dirty(dir, "test fixture state before workflow command");
    }
    if matches!(
        args,
        ["issue", "close", ..] | ["evidence", "record", ..] | ["--quiet", "evidence", "record", ..]
    ) && dir.join(".git").exists()
    {
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

fn append_custom_issue_links(dir: &Path, roles: &[&str]) {
    let path = dir.join(".atelier/config.toml");
    let values = roles
        .iter()
        .map(|role| format!("\"{role}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap_or_else(|error| panic!("failed to open {}: {error}", path.display()));
    writeln!(file, "\n[issue_links]\ncustom_context_types = [{values}]")
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", path.display()));
}

fn set_issue_description(dir: &Path, issue_id: &str, description: &str) {
    if description.trim().is_empty() {
        return;
    }
    edit_canonical_issue(dir, issue_id, |markdown| {
        if description.contains("## Goal") {
            replace_markdown_body(&markdown, description)
        } else {
            replace_issue_section(&markdown, "Goal", description)
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
        "{}\n{}\n\n{}",
        &markdown[..body_start],
        replacement.trim(),
        &markdown[end..]
    )
}

fn write_ignored_canonical_artifacts(dir: &Path, issue_id: &str) {
    let runtime_dir = dir.join(".atelier/runtime");
    std::fs::create_dir_all(&runtime_dir).unwrap();
    std::fs::write(runtime_dir.join(".canonical-mutation.lock"), "").unwrap();
    let issue_dir = dir.join(".atelier/issues");
    std::fs::write(issue_dir.join(format!("{issue_id}.md.lock")), "lock").unwrap();
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
            "--result",
            result,
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

    record_id_by_title(dir, "evidence", summary)
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
    edit_canonical_issue(dir, issue_id, |markdown| {
        markdown.replace(
            "Done when was not specified.",
            "A focused regression test proves the intended behavior.",
        )
    })
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

fn move_issue_to_review(dir: &Path, issue_ref_value: &str) -> String {
    migrate_default_issue_workflow(dir);
    let issue_id = resolve_test_issue_ref(dir, issue_ref_value);
    for transition in ["start", "request_review"] {
        let (success, options, stderr) = run_atelier(dir, &["issue", "transition", &issue_id]);
        assert!(
            success,
            "transition options failed for {issue_id}: {stderr}"
        );
        if !options.contains(&format!("{transition} [")) {
            continue;
        }
        let (success, _, stderr) =
            run_atelier(dir, &["issue", "transition", &issue_id, transition]);
        if options.contains(&format!("{transition} [allowed]")) {
            assert!(success, "{transition} failed for {issue_id}: {stderr}");
        }
    }
    issue_id
}

fn close_issue_with_evidence(dir: &Path, issue_ref_value: &str, reason: Option<&str>) -> String {
    let issue_id = resolve_test_issue_ref(dir, issue_ref_value);
    move_issue_to_review(dir, &issue_id);
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

/// Adds a validator to every `close` transition in the starter policy.
fn add_close_validator(dir: &Path, validator: &str) {
    let path = dir.join(".atelier/workflow.yaml");
    let workflow = fs::read_to_string(&path).unwrap();
    let close_validators = "        - blockers.none_open\n      supersede:";
    assert_eq!(workflow.matches(close_validators).count(), 2, "{workflow}");
    fs::write(
        &path,
        workflow.replace(
            close_validators,
            &format!("        - blockers.none_open\n        - {validator}\n      supersede:"),
        ),
    )
    .unwrap();
}

/// Starter policy whose work `close` also requires passing evidence.
fn write_evidence_gated_workflow(dir: &Path) {
    let workflow = atelier_workflow::STARTER_POLICY_YAML.replace(
        "        description: Done claim; commit it on the work branch before review.\n        validators:\n        - blockers.none_open\n      supersede:\n        from: [todo, blocked, in_progress]\n",
        "        description: Done claim; commit it on the work branch before review.\n        validators:\n        - evidence.attached: { min_count: 1 }\n        - blockers.none_open\n      supersede:\n        from: [todo, blocked, in_progress]\n",
    );
    assert_ne!(workflow, atelier_workflow::STARTER_POLICY_YAML);
    fs::write(dir.join(".atelier/workflow.yaml"), workflow).unwrap();
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
#[path = "cli_integration/mission_worktree.rs"]
mod mission_worktree;
#[path = "cli_integration/records_evidence.rs"]
mod records_evidence;
#[path = "cli_integration/setup_guidance.rs"]
mod setup_guidance;
#[path = "cli_integration/unicode.rs"]
mod unicode;
#[path = "cli_integration/verified_records.rs"]
mod verified_records;
