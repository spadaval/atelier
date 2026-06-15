pub(crate) use sha2::{Digest, Sha256};
pub(crate) use std::collections::HashMap;
pub(crate) use std::fs;
pub(crate) use std::io::Write;
pub(crate) use std::path::{Path, PathBuf};
pub(crate) use std::process::Command;
use std::sync::{Mutex, OnceLock};
pub(crate) use tempfile::tempdir;

static TEST_ISSUE_IDS: OnceLock<Mutex<HashMap<PathBuf, Vec<String>>>> = OnceLock::new();

/// Helper to run atelier commands in a temp directory
pub(crate) fn run_atelier(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let current_surface_args = translate_legacy_fixture_command(args);
    let translated_args = translate_issue_refs_owned(dir, &current_surface_args);
    run_atelier_raw(
        dir,
        &translated_args
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
    )
}

pub(crate) fn run_atelier_raw(dir: &Path, args: &[&str]) -> (bool, String, String) {
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

pub(crate) fn run_atelier_with_env(
    dir: &Path,
    args: &[&str],
    envs: &[(&str, &str)],
) -> (bool, String, String) {
    let current_surface_args = translate_legacy_fixture_command(args);
    let translated_args = translate_issue_refs_owned(dir, &current_surface_args);
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

fn translate_legacy_fixture_command(args: &[&str]) -> Vec<String> {
    match args {
        ["issue", "subissue", parent, title, rest @ ..] => {
            let mut translated = vec![
                "issue".to_string(),
                "create".to_string(),
                (*title).to_string(),
                "--parent".to_string(),
                (*parent).to_string(),
            ];
            translated.extend(rest.iter().map(|value| match *value {
                "-d" => "--description".to_string(),
                "-p" => "--priority".to_string(),
                other => other.to_string(),
            }));
            translated
        }
        ["issue", "comment", issue, body] => vec![
            "issue".to_string(),
            "note".to_string(),
            (*issue).to_string(),
            (*body).to_string(),
        ],
        ["issue", "label", issue, label] => vec![
            "issue".to_string(),
            "update".to_string(),
            (*issue).to_string(),
            "--label".to_string(),
            (*label).to_string(),
        ],
        ["issue", "unlabel", issue, label] => vec![
            "issue".to_string(),
            "update".to_string(),
            (*issue).to_string(),
            "--remove-label".to_string(),
            (*label).to_string(),
        ],
        ["issue", "update", issue, "-d", description] => vec![
            "issue".to_string(),
            "update".to_string(),
            (*issue).to_string(),
            "--description".to_string(),
            (*description).to_string(),
        ],
        ["issue", "update", issue, "-p", priority] => vec![
            "issue".to_string(),
            "update".to_string(),
            (*issue).to_string(),
            "--priority".to_string(),
            (*priority).to_string(),
        ],
        ["mission", "update", mission, "--status", "closed"] => vec![
            "mission".to_string(),
            "close".to_string(),
            (*mission).to_string(),
            "--reason".to_string(),
            "closed by integration fixture".to_string(),
        ],
        _ => args.iter().map(|value| (*value).to_string()).collect(),
    }
}

/// Initialize atelier in a temp directory
pub(crate) fn init_atelier(dir: &Path) {
    init_atelier_without_workflow(dir);
    migrate_default_issue_workflow(dir);
}

pub(crate) fn init_atelier_without_workflow(dir: &Path) {
    let (success, _, stderr) = run_atelier(dir, &["init"]);
    assert!(success, "Failed to init: {}", stderr);
    let workflow_path = dir.join(".atelier").join("workflow.yaml");
    if workflow_path.exists() {
        fs::remove_file(workflow_path).unwrap();
    }
}

pub(crate) fn migrate_default_issue_workflow(dir: &Path) {
    let workflow_path = dir.join(".atelier").join("workflow.yaml");
    if workflow_path.exists() {
        return;
    }
    fs::write(
        &workflow_path,
        atelier::workflow_policy::STARTER_POLICY_YAML,
    )
    .expect("failed to write starter workflow policy");
}

pub(crate) fn init_git_repo(dir: &Path) {
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

pub(crate) fn commit_all(dir: &Path, message: &str) {
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

pub(crate) fn git_status_short(dir: &Path) -> String {
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

pub(crate) fn init_atelier_with_telemetry_disabled(dir: &Path) {
    let (success, _, stderr) =
        run_atelier_with_env(dir, &["init"], &[("ATELIER_TELEMETRY", "off")]);
    assert!(success, "Failed to init: {}", stderr);
    migrate_default_issue_workflow(dir);
}

pub(crate) fn diagnostics_events(root: &Path) -> Vec<serde_json::Value> {
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

pub(crate) fn write_diagnostics_event(root: &Path, date: &str, event: serde_json::Value) {
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

pub(crate) fn registry() -> &'static Mutex<HashMap<PathBuf, Vec<String>>> {
    TEST_ISSUE_IDS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) fn issue_ref(dir: &Path, ordinal: usize) -> String {
    registry()
        .lock()
        .unwrap()
        .get(dir)
        .and_then(|ids| ids.get(ordinal - 1))
        .cloned()
        .unwrap_or_else(|| ordinal.to_string())
}

pub(crate) fn issue_key(id: &str) -> &str {
    id.rsplit_once('-').map(|(_, key)| key).unwrap_or(id)
}

pub(crate) fn issue_id_by_title(dir: &Path, title: &str) -> String {
    record_id_by_title(dir, "issues", title)
}

pub(crate) fn canonical_record_path(dir: &Path, directory: &str, record_id: &str) -> PathBuf {
    dir.join(".atelier")
        .join(directory)
        .join(format!("{record_id}.md"))
}

pub(crate) fn canonical_issue_path(dir: &Path, issue_id: &str) -> PathBuf {
    canonical_record_path(dir, "issues", issue_id)
}

pub(crate) fn active_work_association_count(dir: &Path, issue_id: &str) -> i64 {
    let conn = rusqlite::Connection::open(dir.join(".atelier/runtime/state.db")).unwrap();
    conn.query_row(
        "SELECT COUNT(*) FROM work_associations WHERE issue_id = ?1 AND status = 'active'",
        rusqlite::params![issue_id],
        |row| row.get(0),
    )
    .unwrap()
}

pub(crate) fn read_canonical_record(dir: &Path, directory: &str, record_id: &str) -> String {
    let path = canonical_record_path(dir, directory, record_id);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

pub(crate) fn write_canonical_record(
    dir: &Path,
    directory: &str,
    record_id: &str,
    markdown: String,
) {
    let path = canonical_record_path(dir, directory, record_id);
    std::fs::write(&path, markdown)
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", path.display()));
}

pub(crate) fn edit_canonical_record(
    dir: &Path,
    directory: &str,
    record_id: &str,
    edit: impl FnOnce(String) -> String,
) {
    let markdown = read_canonical_record(dir, directory, record_id);
    write_canonical_record(dir, directory, record_id, edit(markdown));
}

pub(crate) fn edit_canonical_issue(
    dir: &Path,
    issue_id: &str,
    edit: impl FnOnce(String) -> String,
) {
    edit_canonical_record(dir, "issues", issue_id, edit);
}

pub(crate) fn write_ignored_canonical_artifacts(dir: &Path, issue_id: &str) {
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

pub(crate) fn corrupt_issue_title_yaml(dir: &Path, issue_id: &str, title: &str) {
    edit_canonical_issue(dir, issue_id, |markdown| {
        markdown.replace(&format!("title: {title:?}"), &format!("title: [{title}"))
    });
}

pub(crate) fn remove_projection_state(dir: &Path) {
    std::fs::remove_file(dir.join(".atelier/runtime/state.db")).unwrap();
}

pub(crate) fn remove_runtime_and_cache_dirs(dir: &Path) {
    let runtime_dir = dir.join(".atelier/runtime");
    let cache_dir = dir.join(".atelier/cache");
    std::fs::create_dir_all(&cache_dir).unwrap();
    std::fs::write(cache_dir.join("projection.lock"), "cache").unwrap();
    std::fs::remove_dir_all(&runtime_dir).unwrap();
    std::fs::remove_dir_all(&cache_dir).unwrap();
    assert!(!runtime_dir.exists());
    assert!(!cache_dir.exists());
}

pub(crate) fn canonical_evidence_front_matter(dir: &Path, evidence_id: &str) -> serde_json::Value {
    canonical_record_front_matter(dir, "evidence", evidence_id)
}

pub(crate) fn canonical_record_front_matter(
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

pub(crate) fn ignored_test_source(ignore_attribute: &str, test_name: &str) -> String {
    format!("#[test]\n#[{ignore_attribute}]\nfn {test_name}() {{}}\n")
}

pub(crate) fn valid_command_surface_doc() -> &'static str {
    r#"# CLI Surface Tiers

## Core

- `atelier init`
- `atelier man`
- `atelier status`
- `atelier start`
- `atelier issue ...`
- `atelier search <query>`
- `atelier graph impact/tree`
- `atelier issue note`
- `atelier mission note`
- `atelier mission create/show/list/status/update`
- `atelier mission audit`
- `atelier mission add-work/unlink/add-blocker`
- `atelier plan create/show/list/revise/link/apply`
- `atelier evidence record/show/list/attach`
- `atelier history`
- `atelier worktree for/status/merge/repair/remove`
- `atelier branch for-epic/status/merge`
- `atelier maintenance delete`
- `atelier lint`
- `atelier doctor`
"#
}

pub(crate) fn write_command_surface_doc(dir: &Path, content: &str) {
    let docs_dir = dir.join("docs/product");
    fs::create_dir_all(&docs_dir).unwrap();
    fs::write(docs_dir.join("cli-surface.md"), content).unwrap();
}

pub(crate) fn write_valid_command_guidance(dir: &Path) {
    write_command_surface_doc(dir, valid_command_surface_doc());
    fs::write(
        dir.join("AGENTS.md"),
        "# Agent Instructions\n\n- `atelier issue list --ready`\n- `atelier export --check`\n",
    )
    .unwrap();
    fs::write(
        dir.join("AGENTFACTORY.md"),
        "# Agent Factory Binding\n\n- `atelier status`\n- `atelier mission status [<id>]`\n- `atelier mission audit <id>`\n- `atelier issue show <id>`\n",
    )
    .unwrap();
}

pub(crate) fn remove_issue_section(markdown: &str, heading: &str) -> String {
    let marker = format!("## {heading}\n");
    let start = markdown.find(&marker).expect("section heading missing");
    let rest = &markdown[start + marker.len()..];
    let end = rest
        .find("\n## ")
        .map(|offset| start + marker.len() + offset)
        .unwrap_or(markdown.len());
    format!("{}{}", &markdown[..start], &markdown[end..])
}

pub(crate) fn record_id_by_title(dir: &Path, directory: &str, title: &str) -> String {
    let record_dir = dir.join(".atelier").join(directory);
    let entries = std::fs::read_dir(&record_dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", record_dir.display()));
    for entry in entries {
        let path = entry.unwrap().path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let record_id = path.file_stem().unwrap().to_string_lossy().to_string();
        let text = read_canonical_record(dir, directory, &record_id);
        if text.contains(&format!("title: {title:?}")) {
            return record_id;
        }
    }
    panic!(
        "record with title {title:?} not found in {}",
        record_dir.display()
    );
}

pub(crate) fn resolve_test_issue_ref(dir: &Path, issue_ref_value: &str) -> String {
    issue_ref_value
        .parse::<usize>()
        .ok()
        .map(|ordinal| issue_ref(dir, ordinal))
        .unwrap_or_else(|| issue_ref_value.to_string())
}

pub(crate) fn attach_evidence(
    dir: &Path,
    target_kind: &str,
    target_id: &str,
    result: &str,
    summary: &str,
) -> String {
    if target_kind == "issue" {
        ensure_all_issue_closeout_sections(dir);
    }
    let (success, evidence_out, stderr) = run_atelier(
        dir,
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            result,
            "--target",
            &format!("{target_kind}/{target_id}"),
            summary,
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
    assert!(
        evidence_out.contains(&format!("[evidence] {result}")),
        "{evidence_out}"
    );
    record_id_by_title(dir, "evidence", summary)
}

pub(crate) fn attach_pass_evidence(
    dir: &Path,
    target_kind: &str,
    target_id: &str,
    summary: &str,
) -> String {
    attach_evidence(dir, target_kind, target_id, "pass", summary)
}

pub(crate) fn attach_non_pass_evidence(
    dir: &Path,
    target_kind: &str,
    target_id: &str,
    result: &str,
    summary: &str,
) -> String {
    assert!(result != "pass");
    attach_evidence(dir, target_kind, target_id, result, summary)
}

pub(crate) fn attach_issue_pass_evidence(dir: &Path, issue_id: &str) -> String {
    ensure_all_issue_closeout_sections(dir);
    attach_pass_evidence(
        dir,
        "issue",
        issue_id,
        &format!("issue close proof for {issue_id}"),
    )
}

pub(crate) fn ensure_issue_closeout_sections(dir: &Path, issue_id: &str) {
    edit_canonical_issue(dir, issue_id, |mut markdown| {
        markdown = markdown.replace(
            "Outcome was not specified.",
            "The issue outcome is complete and ready for closeout.",
        );
        markdown.replace(
            "Evidence was not specified.",
            &format!("- Evidence record attached to issue/{issue_id} validates closeout."),
        )
    });
}

pub(crate) fn ensure_all_issue_closeout_sections(dir: &Path) {
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
            ensure_issue_closeout_sections(dir, issue_id);
        }
    }
}

pub(crate) fn move_issue_to_validation(dir: &Path, issue_ref_value: &str) -> String {
    migrate_default_issue_workflow(dir);
    let issue_id = resolve_test_issue_ref(dir, issue_ref_value);
    let issue_markdown = std::fs::read_to_string(canonical_issue_path(dir, &issue_id)).unwrap();
    let review_gated = !issue_markdown.contains("issue_type: \"task\"");
    let transitions = if review_gated {
        vec![
            ("start", "start"),
            ("request_review", "request_review"),
            ("request_validation", "request_validation"),
        ]
    } else {
        vec![("start", "start")]
    };
    for (transition, label) in transitions {
        let (success, _, stderr) =
            run_atelier(dir, &["issue", "transition", &issue_id, transition]);
        assert!(success, "{label} failed for {issue_id}: {stderr}");
    }
    issue_id
}

pub(crate) fn close_issue_with_evidence(
    dir: &Path,
    issue_ref_value: &str,
    reason: Option<&str>,
) -> String {
    let issue_id = resolve_test_issue_ref(dir, issue_ref_value);
    move_issue_to_validation(dir, &issue_id);
    ensure_all_issue_closeout_sections(dir);
    attach_issue_pass_evidence(dir, &issue_id);
    if dir.join(".git").exists() {
        commit_all(dir, &format!("ready to close {issue_id}"));
    }
    let mut args = vec!["issue", "close", issue_ref_value];
    args.push("--reason");
    args.push(reason.unwrap_or("done"));
    let (success, _, stderr) = run_atelier(dir, &args);
    assert!(success, "issue close failed: {stderr}");
    issue_id
}

pub(crate) fn issue_activity_texts(dir: &Path, issue_id: &str) -> Vec<String> {
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

pub(crate) fn assert_activity_contains(texts: &[String], event_type: &str, expected: &[&str]) {
    let needle = format!("event_type: \"{event_type}\"");
    assert!(
        texts
            .iter()
            .any(|text| text.contains(&needle) && expected.iter().all(|part| text.contains(part))),
        "missing activity {event_type} containing {expected:?}; activities:\n{}",
        texts.join("\n--- activity ---\n")
    );
}

pub(crate) fn register_issue_id(dir: &Path, id: String) {
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

pub(crate) fn register_issue_ids_from_stdout(dir: &Path, stdout: &str) {
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

pub(crate) fn register_issue_ids_from_state(dir: &Path) {
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

pub(crate) fn is_record_id(value: &str) -> bool {
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

pub(crate) fn translate_issue_refs_owned<T: AsRef<str>>(dir: &Path, args: &[T]) -> Vec<String> {
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

pub(crate) fn translate_issue_ref(dir: &Path, value: &str) -> String {
    let numeric = value.strip_prefix('#').unwrap_or(value);
    match numeric.parse::<usize>() {
        Ok(ordinal) => issue_ref(dir, ordinal),
        Err(_) => value.to_string(),
    }
}

pub(crate) fn command_offset<T: AsRef<str>>(args: &[T]) -> usize {
    args.iter()
        .position(|arg| !arg.as_ref().starts_with('-'))
        .unwrap_or(args.len())
}

pub(crate) fn issue_ref_position<T: AsRef<str>>(args: &[T], index: usize) -> bool {
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
        ["issue", "blocked", ..] => index == offset + 2,
        ["issue", "label" | "unlabel" | "comment", ..] => index == offset + 2,
        ["issue", "block" | "unblock" | "relate" | "unrelate", ..] => {
            index == offset + 2 || index == offset + 3
        }
        ["issue", "subissue", ..] => index == offset + 2,
        ["graph", "impact", ..] => index == offset + 2,
        ["note", "add", target_kind, ..] => *target_kind == "issue" && index == offset + 3,
        ["maintenance", "delete", target_kind, ..] => {
            *target_kind == "issue" && index == offset + 3
        }
        _ => false,
    }
}
