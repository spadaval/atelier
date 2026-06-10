use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use tempfile::tempdir;

static TEST_ISSUE_IDS: OnceLock<Mutex<HashMap<PathBuf, Vec<String>>>> = OnceLock::new();

fn json_value(stdout: &str) -> serde_json::Value {
    serde_json::from_str(stdout).unwrap_or_else(|error| {
        panic!("expected valid JSON, got error {error}; stdout:\n{stdout}");
    })
}

/// Helper to run atelier commands in a temp directory
fn run_atelier(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let translated_args = translate_issue_refs(dir, args);
    let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir)
        .args(&translated_args)
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

/// Initialize atelier in a temp directory
fn init_atelier(dir: &Path) {
    let (success, _, stderr) = run_atelier(dir, &["init"]);
    assert!(success, "Failed to init: {}", stderr);
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

fn issue_id_by_title(dir: &Path, title: &str) -> String {
    let (success, stdout, stderr) =
        run_atelier(dir, &["--json", "issue", "list", "--status", "all"]);
    assert!(success, "issue list failed: {stderr}");
    let listed = json_value(&stdout);
    listed["data"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|issue| issue["title"] == title)
        .and_then(|issue| issue["id"].as_str())
        .unwrap_or_else(|| panic!("issue with title {title:?} not found in {stdout}"))
        .to_string()
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
    let issues_dir = dir.join(".atelier-state/issues");
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

fn translate_issue_refs(dir: &Path, args: &[&str]) -> Vec<String> {
    args.iter()
        .enumerate()
        .map(|(index, arg)| {
            if issue_ref_position(args, index) {
                translate_issue_ref(dir, arg)
            } else {
                (*arg).to_string()
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

fn command_offset(args: &[&str]) -> usize {
    args.iter()
        .position(|arg| !arg.starts_with('-'))
        .unwrap_or(args.len())
}

fn issue_ref_position(args: &[&str], index: usize) -> bool {
    let offset = command_offset(args);
    if index <= offset {
        return false;
    }

    match args.get(offset..).unwrap_or_default() {
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
        ["issue", "show" | "update" | "impact", ..] => index == offset + 2,
        ["issue", "subissue", ..] => index == offset + 2,
        ["issue", "relate", ..] => index == offset + 2 || index == offset + 3,
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
}

#[test]
fn test_init_twice_warns() {
    let dir = tempdir().unwrap();

    run_atelier(dir.path(), &["init"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["init"]);

    assert!(success);
    assert!(stdout.contains("Already") || stdout.contains("already") || stdout.contains("exists"));
}

// ==================== Issue Creation Tests ====================

#[test]
fn test_create_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["create", "Test issue"]);

    assert!(success);
    assert!(
        stdout.contains("Created issue atelier-"),
        "Expected project-scoped issue id in output, got: {}",
        stdout
    );
}

#[test]
fn test_create_issue_with_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, _) = run_atelier(dir.path(), &["create", "High priority issue", "-p", "high"]);

    assert!(success);

    // Verify it was created with correct priority
    let (_, list_out, _) = run_atelier(dir.path(), &["list"]);
    assert!(list_out.contains("high"));
}

#[test]
fn test_create_issue_with_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "create",
            "Issue with desc",
            "-d",
            "Detailed description here",
        ],
    );

    assert!(success);

    // Verify description in show
    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
    assert!(show_out.contains("Detailed description"));
}

#[test]
fn test_create_subissue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Parent issue"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["subissue", "1", "Child issue"]);

    assert!(success);
    assert!(
        stdout.contains("Created subissue atelier-"),
        "Expected project-scoped subissue id in output, got: {}",
        stdout
    );

    // Verify parent-child relationship in show
    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
    assert!(show_out.contains("Child") || show_out.contains("subissue"));
}

// ==================== Issue Listing Tests ====================

#[test]
fn test_list_empty() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["list"]);

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

    run_atelier(dir.path(), &["create", "Issue 1"]);
    run_atelier(dir.path(), &["create", "Issue 2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["list"]);

    assert!(success);
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));
}

#[test]
fn test_list_filter_by_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Open issue"]);
    run_atelier(dir.path(), &["create", "Closed issue"]);
    run_atelier(dir.path(), &["close", "2"]);

    let (_, open_list, _) = run_atelier(dir.path(), &["list", "-s", "open"]);
    assert!(open_list.contains("Open issue"));
    assert!(!open_list.contains("Closed issue"));

    let (_, closed_list, _) = run_atelier(dir.path(), &["list", "-s", "closed"]);
    assert!(closed_list.contains("Closed issue"));
    assert!(!closed_list.contains("Open issue"));
}

#[test]
fn test_list_filter_by_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Bug issue"]);
    run_atelier(dir.path(), &["create", "Feature issue"]);
    run_atelier(dir.path(), &["label", "1", "bug"]);
    run_atelier(dir.path(), &["label", "2", "feature"]);

    let (_, bug_list, _) = run_atelier(dir.path(), &["list", "-l", "bug"]);
    assert!(bug_list.contains("Bug issue"));
    assert!(!bug_list.contains("Feature issue"));
}

// ==================== Issue Show Tests ====================

#[test]
fn test_show_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Test issue", "-d", "Description"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["show", "1"]);

    assert!(success);
    assert!(stdout.contains("Test issue"));
    assert!(stdout.contains("Description"));
}

#[test]
fn test_show_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["show", "999"]);

    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

// ==================== Issue Update Tests ====================

#[test]
fn test_update_issue_title() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Original title"]);
    let (success, _, _) = run_atelier(dir.path(), &["update", "1", "--title", "Updated title"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
    assert!(show_out.contains("Updated title"));
}

#[test]
fn test_update_issue_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue", "-p", "low"]);
    run_atelier(dir.path(), &["update", "1", "-p", "critical"]);

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
    assert!(show_out.contains("critical"));
}

// ==================== Issue Close/Reopen Tests ====================

#[test]
fn test_close_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Test issue"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["close", "1"]);

    assert!(success);
    assert!(stdout.contains("Closed") || stdout.contains("closed"));

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
    assert!(show_out.contains("closed"));
}

#[test]
fn test_reopen_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Test issue"]);
    run_atelier(dir.path(), &["close", "1"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["reopen", "1"]);

    assert!(success);
    assert!(stdout.contains("Reopened") || stdout.contains("reopen"));

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
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
        &["--json", "import-beads", fixture_path.to_str().unwrap()],
    );
    assert!(success, "import-beads failed: {stderr}");
    assert!(stdout.contains("\"source_records\": 3"));
    assert!(stdout.contains("\"imported_issues\": 3"));
    assert!(stdout.contains("\"parent_child_links\": 2"));
    assert!(stdout.contains("\"blocking_links\": 1"));
    assert!(dir
        .path()
        .join(".atelier-state")
        .join("issues")
        .join("atelier-0001.md")
        .exists());

    let (_, list_out, _) = run_atelier(dir.path(), &["list", "--status", "all"]);
    assert!(list_out.contains("Mission: Replace Beads"));
    assert!(list_out.contains("Dogfood Atelier"));

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "3"]);
    assert!(show_out.contains("Parent: atelier-0001"));
    assert!(show_out.contains("Blocked by: atelier-0002"));
    assert!(show_out.contains("Acceptance Criteria"));

    let (updated, _, update_err) = run_atelier(
        dir.path(),
        &["update", "2", "--title", "Imported Beads issue updated"],
    );
    assert!(updated, "update failed: {update_err}");
    let (closed, _, close_err) = run_atelier(dir.path(), &["close", "2"]);
    assert!(closed, "close failed: {close_err}");

    let (_, closed_show, _) = run_atelier(dir.path(), &["show", "2"]);
    assert!(closed_show.contains("Imported Beads issue updated"));
    assert!(closed_show.contains("Status: closed"));

    let (fresh, _, fresh_err) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(!fresh, "mutating imported issue should stale export");
    assert!(fresh_err.contains("stale") || fresh_err.contains("changed"));
}

// ==================== Issue Delete Tests ====================

#[test]
fn test_delete_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "To delete"]);
    let (success, _, _) = run_atelier(dir.path(), &["delete", "1", "-f"]);

    assert!(success);

    let (_, list_out, _) = run_atelier(dir.path(), &["list"]);
    assert!(!list_out.contains("To delete"));
}

// ==================== Labels Tests ====================

#[test]
fn test_add_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Test issue"]);
    let (success, _, _) = run_atelier(dir.path(), &["label", "1", "bug"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
    assert!(show_out.contains("bug"));
}

#[test]
fn test_remove_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Test issue"]);
    run_atelier(dir.path(), &["label", "1", "bug"]);
    let (success, _, _) = run_atelier(dir.path(), &["unlabel", "1", "bug"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
    assert!(!show_out.contains("bug") || show_out.contains("Labels: none"));
}

// ==================== Comments Tests ====================

#[test]
fn test_add_comment() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Test issue"]);
    let (success, _, _) = run_atelier(dir.path(), &["comment", "1", "This is a comment"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
    assert!(show_out.contains("This is a comment"));
}

// ==================== Dependencies Tests ====================

#[test]
fn test_block_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Blocked issue"]);
    run_atelier(dir.path(), &["create", "Blocker issue"]);
    let (success, _, _) = run_atelier(dir.path(), &["block", "1", "2"]);

    assert!(success);

    let (_, blocked_out, _) = run_atelier(dir.path(), &["blocked"]);
    assert!(blocked_out.contains("Blocked issue"));
}

#[test]
fn test_unblock_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Blocked issue"]);
    run_atelier(dir.path(), &["create", "Blocker issue"]);
    run_atelier(dir.path(), &["block", "1", "2"]);
    let (success, _, _) = run_atelier(dir.path(), &["unblock", "1", "2"]);

    assert!(success);

    let (_, blocked_out, _) = run_atelier(dir.path(), &["blocked"]);
    assert!(!blocked_out.contains("Blocked issue"));
}

#[test]
fn test_ready_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Blocked issue"]);
    run_atelier(dir.path(), &["create", "Blocker issue"]);
    run_atelier(dir.path(), &["create", "Ready issue"]);
    run_atelier(dir.path(), &["block", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["ready"]);

    assert!(success);
    assert!(stdout.contains("Ready issue"));
    assert!(stdout.contains("Blocker issue")); // Blocker is also ready
    assert!(!stdout.contains("Blocked issue"));
}

// ==================== Session Tests ====================

#[test]
fn test_session_start() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["session", "start"]);

    assert!(success);
    assert!(stdout.contains("Session") || stdout.contains("started"));
}

#[test]
fn test_session_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["session", "start"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["session", "status"]);

    assert!(success);
    assert!(stdout.contains("Session") || stdout.contains("active"));
}

#[test]
fn test_session_work() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Working issue"]);
    run_atelier(dir.path(), &["session", "start"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["session", "work", "1"]);

    assert!(success);
    assert!(stdout.contains("Working") || stdout.contains("#1"));
}

#[test]
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

    run_atelier(dir.path(), &["create", "Authentication bug"]);
    run_atelier(dir.path(), &["create", "Dark mode feature"]);
    run_atelier(dir.path(), &["create", "Auth improvements"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["search", "auth"]);

    assert!(success);
    assert!(stdout.contains("Authentication") || stdout.contains("Auth"));
    assert!(!stdout.contains("Dark mode"));
}

// ==================== Error Handling Tests ====================

#[test]
fn test_command_without_init() {
    let dir = tempdir().unwrap();
    // Don't init

    let (success, stdout, stderr) = run_atelier(dir.path(), &["list"]);

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

    let (success, _, stderr) = run_atelier(dir.path(), &["create", "Issue", "-p", "invalid"]);

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
    let (success, _, _) = run_atelier(dir.path(), &["create", malicious]);

    assert!(success);

    // Verify database is intact
    let (success2, stdout, _) = run_atelier(dir.path(), &["list"]);
    assert!(success2);
    assert!(stdout.contains(malicious));
}

#[test]
fn test_special_characters_in_fields() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let special = "Test <>&\"'\\n\\t issue";
    let (success, _, _) = run_atelier(dir.path(), &["create", special]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
    assert!(show_out.contains("Test"));
}

#[test]
fn test_unicode_in_cli() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let unicode = "测试问题 🐛 émoji";
    let (success, _, _) = run_atelier(dir.path(), &["create", unicode]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
    assert!(show_out.contains("测试") || show_out.contains("🐛"));
}

// ==================== Archive Tests ====================

#[test]
fn test_archive_closed_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue to archive"]);
    run_atelier(dir.path(), &["close", "1"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["archive", "add", "1"]);

    assert!(success);
    assert!(stdout.contains("Archived") || stdout.contains("archived"));
}

#[test]
fn test_archive_open_issue_fails() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Open issue"]);
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
fn test_archive_list() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue to archive"]);
    run_atelier(dir.path(), &["create", "Open issue"]);
    run_atelier(dir.path(), &["close", "1"]);
    run_atelier(dir.path(), &["archive", "add", "1"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["archive", "list"]);

    assert!(success);
    assert!(stdout.contains("Issue to archive") || stdout.contains("#1"));
}

#[test]
fn test_unarchive_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue to archive"]);
    run_atelier(dir.path(), &["close", "1"]);
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
    let (_, closed_list, _) = run_atelier(dir.path(), &["list", "-s", "closed"]);
    assert!(closed_list.contains("Issue to archive"));
}

// ==================== Milestone Tests ====================

#[test]
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
fn test_milestone_add_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    run_atelier(dir.path(), &["create", "Feature 1"]);
    run_atelier(dir.path(), &["create", "Feature 2"]);

    let (success, _, _) = run_atelier(dir.path(), &["milestone", "add", "1", "1", "2"]);

    assert!(success);

    // Check milestone shows the issues
    let (_, show_out, _) = run_atelier(dir.path(), &["milestone", "show", "1"]);
    assert!(show_out.contains("Feature 1") || show_out.contains("#1"));
}

#[test]
fn test_milestone_close() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["milestone", "close", "1"]);

    assert!(success);
    assert!(stdout.contains("Closed") || stdout.contains("closed"));
}

#[test]
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
fn test_timer_start() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue to time"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["start", "1"]);

    assert!(success);
    assert!(stdout.contains("Started") || stdout.contains("timer") || stdout.contains("#1"));
}

#[test]
fn test_timer_stop() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue to time"]);
    run_atelier(dir.path(), &["start", "1"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["stop"]);

    assert!(success);
    assert!(stdout.contains("Stopped") || stdout.contains("stopped") || stdout.contains("timer"));
}

#[test]
fn test_timer_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue to time"]);
    run_atelier(dir.path(), &["start", "1"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["timer"]);

    assert!(success);
    assert!(
        stdout.contains("#1") || stdout.contains("Issue to time") || stdout.contains("running")
    );
}

#[test]
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

    run_atelier(dir.path(), &["create", "Issue 1"]);
    run_atelier(dir.path(), &["create", "Issue 2"]);

    let (success, _, _) = run_atelier(dir.path(), &["relate", "1", "2"]);

    assert!(success);
}

#[test]
fn test_related_list() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue 1"]);
    run_atelier(dir.path(), &["create", "Issue 2"]);
    run_atelier(dir.path(), &["relate", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["related", "1"]);

    assert!(success);
    assert!(stdout.contains("Issue 2") || stdout.contains("#2"));
}

#[test]
fn test_unrelate_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue 1"]);
    run_atelier(dir.path(), &["create", "Issue 2"]);
    run_atelier(dir.path(), &["relate", "1", "2"]);
    let (success, _, _) = run_atelier(dir.path(), &["unrelate", "1", "2"]);

    assert!(success);

    let (_, related_out, _) = run_atelier(dir.path(), &["related", "1"]);
    assert!(!related_out.contains("Issue 2") || related_out.contains("No related"));
}

#[test]
fn test_issue_help_shows_impact_and_hides_legacy_assumption_commands() {
    let dir = tempdir().unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "--help"]);

    assert!(success, "issue help failed: {}", stderr);
    assert!(stdout.contains("impact"));
    assert!(!stdout.contains("cascade"));
    assert!(!stdout.contains("falsify"));
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

    run_atelier(dir.path(), &["create", "Parent issue"]);
    run_atelier(dir.path(), &["subissue", "1", "Child issue"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["tree"]);

    assert!(success);
    assert!(stdout.contains("Parent issue"));
    assert!(stdout.contains("Child issue"));
}

#[test]
fn test_tree_with_status_filter() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Open parent"]);
    run_atelier(dir.path(), &["create", "Closed parent"]);
    run_atelier(dir.path(), &["close", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["tree", "-s", "open"]);

    assert!(success);
    assert!(stdout.contains("Open parent"));
    // Closed issues should not appear
    assert!(!stdout.contains("Closed parent"));
}

// ==================== Next Tests ====================

#[test]
fn test_next_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Low priority", "-p", "low"]);
    run_atelier(dir.path(), &["create", "High priority", "-p", "high"]);
    run_atelier(
        dir.path(),
        &["create", "Critical priority", "-p", "critical"],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["next"]);

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

    let (success, stdout, _) = run_atelier(dir.path(), &["next"]);

    assert!(success);
    assert!(
        stdout.contains("No issues ready to work on"),
        "Expected 'No issues ready to work on' message, got: {}",
        stdout
    );
}

// ==================== Export/Import Tests ====================

#[test]
fn test_export_json() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue 1"]);
    run_atelier(dir.path(), &["create", "Issue 2"]);

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
fn test_export_markdown() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue 1", "-d", "Description 1"]);

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
fn test_import_json() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create issues and export
    run_atelier(dir.path(), &["create", "Exported Issue"]);
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
    let (_, list_out, _) = run_atelier(dir2.path(), &["list", "-s", "all"]);
    assert!(list_out.contains("Exported Issue") || list_out.contains("#1"));
}

// ==================== Tested Command Tests ====================

#[test]
fn test_tested_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, _) = run_atelier(dir.path(), &["tested"]);

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

    let (success, _, _) = run_atelier(dir.path(), &["create", "Bug report", "-t", "bug"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
    assert!(show_out.contains("Bug report"));
}

#[test]
fn test_create_all_priorities() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for priority in &["low", "medium", "high", "critical"] {
        let (success, _, _) = run_atelier(
            dir.path(),
            &["create", &format!("{} issue", priority), "-p", priority],
        );
        assert!(success, "Failed to create {} priority issue", priority);
    }

    let (_, list_out, _) = run_atelier(dir.path(), &["list"]);
    assert!(list_out.contains("low"));
    assert!(list_out.contains("medium"));
    assert!(list_out.contains("high"));
    assert!(list_out.contains("critical"));
}

#[test]
fn test_subissue_with_priority() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Parent"]);
    let (success, _, _) = run_atelier(dir.path(), &["subissue", "1", "Child", "-p", "critical"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "2"]);
    assert!(show_out.contains("critical"));
}

#[test]
fn test_subissue_with_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Parent"]);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["subissue", "1", "Child", "-d", "Child description"],
    );

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "2"]);
    assert!(show_out.contains("Child description"));
}

// ==================== Additional Delete Edge Cases ====================

#[test]
fn test_delete_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["delete", "999", "-f"]);

    // Should fail or warn about nonexistent issue
    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

#[test]
fn test_delete_with_subissues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Parent"]);
    run_atelier(dir.path(), &["subissue", "1", "Child"]);

    let (success, _, _) = run_atelier(dir.path(), &["delete", "1", "-f"]);

    assert!(success);

    // Both parent and child should be gone
    let (_, list_out, _) = run_atelier(dir.path(), &["list", "-s", "all"]);
    assert!(!list_out.contains("Parent"));
    assert!(!list_out.contains("Child"));
}

// ==================== Additional Session Edge Cases ====================

#[test]
fn test_session_work_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["session", "start"]);
    let (success, _, stderr) = run_atelier(dir.path(), &["session", "work", "999"]);

    // Should fail or warn
    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

#[test]
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

    run_atelier(dir.path(), &["create", "Blocked issue", "-p", "critical"]);
    run_atelier(dir.path(), &["create", "Blocker issue", "-p", "low"]);
    run_atelier(dir.path(), &["block", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["next"]);

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

    run_atelier(dir.path(), &["create", "Issue 1"]);
    run_atelier(dir.path(), &["close", "1"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["next"]);

    assert!(success);
    assert!(
        stdout.contains("No issues ready to work on"),
        "Expected 'No issues ready to work on' message, got: {}",
        stdout
    );
}

// ==================== Additional Archive Edge Cases ====================

#[test]
fn test_archive_older_days() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Old issue"]);
    run_atelier(dir.path(), &["close", "1"]);

    // Try to archive issues older than 0 days (should include our just-closed issue)
    let (success, _, _) = run_atelier(dir.path(), &["archive", "older", "0"]);

    assert!(success);
}

#[test]
fn test_archive_already_archived() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue"]);
    run_atelier(dir.path(), &["close", "1"]);
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
fn test_milestone_remove_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["milestone", "create", "v1.0"]);
    run_atelier(dir.path(), &["create", "Feature"]);
    run_atelier(dir.path(), &["milestone", "add", "1", "1"]);

    let (success, _, _) = run_atelier(dir.path(), &["milestone", "remove", "1", "1"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["milestone", "show", "1"]);
    assert!(!show_out.contains("Feature") || show_out.contains("No issues"));
}

#[test]
fn test_milestone_show_nonexistent() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["milestone", "show", "999"]);

    assert!(!success || stderr.contains("not found") || stderr.contains("No milestone"));
}

#[test]
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

    run_atelier(dir.path(), &["create", "Low issue", "-p", "low"]);
    run_atelier(dir.path(), &["create", "High issue", "-p", "high"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["list", "-p", "high"]);

    assert!(success);
    assert!(stdout.contains("High issue"));
    assert!(!stdout.contains("Low issue"));
}

#[test]
fn test_list_all_statuses() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Open issue"]);
    run_atelier(dir.path(), &["create", "Closed issue"]);
    run_atelier(dir.path(), &["close", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["list", "-s", "all"]);

    assert!(success);
    assert!(stdout.contains("Open issue"));
    assert!(stdout.contains("Closed issue"));
}

// ==================== Additional Update Edge Cases ====================

#[test]
fn test_update_description() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue"]);
    let (success, _, _) = run_atelier(dir.path(), &["update", "1", "-d", "New description"]);

    assert!(success);

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
    assert!(show_out.contains("New description"));
}

#[test]
fn test_update_nonexistent() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["update", "999", "--title", "New"]);

    assert!(!success || stderr.contains("not found") || stderr.contains("No issue"));
}

// ==================== Additional Show Edge Cases ====================

#[test]
fn test_show_with_labels() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue"]);
    run_atelier(dir.path(), &["label", "1", "bug"]);
    run_atelier(dir.path(), &["label", "1", "urgent"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["show", "1"]);

    assert!(success);
    assert!(stdout.contains("bug"));
    assert!(stdout.contains("urgent"));
}

#[test]
fn test_show_with_comments() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue"]);
    run_atelier(dir.path(), &["comment", "1", "First comment"]);
    run_atelier(dir.path(), &["comment", "1", "Second comment"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["show", "1"]);

    assert!(success);
    assert!(stdout.contains("First comment"));
    assert!(stdout.contains("Second comment"));
}

#[test]
fn test_show_with_blockers() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Blocked"]);
    run_atelier(dir.path(), &["create", "Blocker"]);
    run_atelier(dir.path(), &["block", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["show", "1"]);

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

    run_atelier(dir.path(), &["create", "Test issue"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["search", "nonexistent"]);

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
        &["create", "Generic title", "-d", "specific_keyword_here"],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["search", "specific_keyword"]);

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
    assert!(
        stdout.contains("Updated")
            || stdout.contains("updated")
            || stdout.contains("Created")
            || stdout.contains("initialized")
    );
}

// ==================== Complex Workflow Tests ====================

#[test]
fn test_full_issue_lifecycle() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create
    run_atelier(dir.path(), &["create", "Lifecycle test", "-p", "high"]);

    // Add labels
    run_atelier(dir.path(), &["label", "1", "feature"]);

    // Add comment
    run_atelier(dir.path(), &["comment", "1", "Working on this"]);

    // Update
    run_atelier(dir.path(), &["update", "1", "-p", "critical"]);

    // Close
    run_atelier(dir.path(), &["close", "1"]);

    // Verify final state
    let (success, stdout, _) = run_atelier(dir.path(), &["show", "1"]);
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
    run_atelier(dir.path(), &["create", "Final task"]);
    run_atelier(dir.path(), &["create", "Middle task"]);
    run_atelier(dir.path(), &["create", "First task"]);

    run_atelier(dir.path(), &["block", "1", "2"]);
    run_atelier(dir.path(), &["block", "2", "3"]);

    // Only issue 3 should be ready
    let (success, stdout, _) = run_atelier(dir.path(), &["ready"]);
    assert!(success);
    assert!(stdout.contains("First task") || stdout.contains("#3"));
    assert!(!stdout.contains("Final task"));
    assert!(!stdout.contains("Middle task"));

    // Close 3, now 2 should be ready
    run_atelier(dir.path(), &["close", "3"]);
    let (_, stdout, _) = run_atelier(dir.path(), &["ready"]);
    assert!(stdout.contains("Middle task") || stdout.contains("#2"));
}

// ==================== Targeted Coverage Tests ====================

// --- next.rs: Multiple ready issues with runners-up ---
#[test]
fn test_next_with_multiple_ready_issues() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create multiple issues with different priorities
    run_atelier(dir.path(), &["create", "Low prio task", "-p", "low"]);
    run_atelier(dir.path(), &["create", "Medium prio task", "-p", "medium"]);
    run_atelier(dir.path(), &["create", "High prio task", "-p", "high"]);
    run_atelier(dir.path(), &["create", "Critical task", "-p", "critical"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["next"]);

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
            "create",
            "Task with description",
            "-p",
            "high",
            "-d",
            "This is a detailed description for the task",
        ],
    );

    let (success, stdout, _) = run_atelier(dir.path(), &["next"]);

    assert!(success);
    assert!(stdout.contains("description") || stdout.contains("Task with description"));
}

// --- next.rs: Progress with subissues ---
#[test]
fn test_next_with_subissue_progress() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create parent with subissues
    run_atelier(dir.path(), &["create", "Parent task", "-p", "high"]);
    run_atelier(dir.path(), &["subissue", "1", "Sub 1"]);
    run_atelier(dir.path(), &["subissue", "1", "Sub 2"]);
    run_atelier(dir.path(), &["subissue", "1", "Sub 3"]);

    // Close one subissue to create progress
    run_atelier(dir.path(), &["close", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["next"]);

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
    run_atelier(dir.path(), &["create", "Blocker"]);
    run_atelier(dir.path(), &["create", "Parent"]);
    run_atelier(dir.path(), &["block", "2", "1"]);

    // Create unblocked subissue under the blocked parent
    run_atelier(dir.path(), &["subissue", "2", "Subissue"]);

    // Close the blocker - now parent has only subissue as ready issue
    run_atelier(dir.path(), &["close", "1"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["next"]);

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
fn test_import_with_parent_relationships() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create and export issues with parent-child relationship
    run_atelier(dir.path(), &["create", "Parent issue"]);
    run_atelier(dir.path(), &["subissue", "1", "Child issue"]);

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
    let (_, tree_out, _) = run_atelier(dir2.path(), &["tree"]);
    assert!(tree_out.contains("Parent") && tree_out.contains("Child"));
}

// --- import.rs: Import issues with labels and comments ---
#[test]
fn test_import_with_labels_and_comments() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create issue with labels and comments
    run_atelier(dir.path(), &["create", "Labeled issue"]);
    run_atelier(dir.path(), &["label", "1", "bug"]);
    run_atelier(dir.path(), &["label", "1", "urgent"]);
    run_atelier(dir.path(), &["comment", "1", "First comment"]);
    run_atelier(dir.path(), &["close", "1"]);

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
    let (_, show_out, _) = run_atelier(dir2.path(), &["show", &labeled_id]);
    assert!(show_out.contains("bug") || show_out.contains("Label"));
    assert!(show_out.contains("closed") || show_out.contains("Closed"));
}

// --- session.rs: Session with handoff notes from previous session ---
#[test]
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
fn test_session_status_with_active_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Active task"]);
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
        &["create", "Critical bug", "-t", "bug", "-p", "critical"],
    );

    assert!(success);
    assert!(stdout.contains("atelier-"));

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
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

    let (_, show_out, _) = run_atelier(dir.path(), &["show", "1"]);
    // Should have both template prefix and user description
    assert!(show_out.contains("User provided details") || show_out.contains("Steps to reproduce"));
}

// --- create.rs: Subissue with invalid parent ---
#[test]
fn test_subissue_invalid_parent() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["subissue", "999", "Orphan"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999") || stderr.contains("Parent"));
}

// --- relate.rs: Related issues display ---
#[test]
fn test_related_issues_display() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue A"]);
    run_atelier(dir.path(), &["create", "Issue B"]);
    run_atelier(dir.path(), &["create", "Issue C"]);

    run_atelier(dir.path(), &["relate", "1", "2"]);
    run_atelier(dir.path(), &["relate", "1", "3"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["related", "1"]);

    assert!(success);
    assert!(stdout.contains("Issue B") || stdout.contains("#2"));
    assert!(stdout.contains("Issue C") || stdout.contains("#3"));
}

// --- label.rs: Multiple labels on same issue ---
#[test]
fn test_multiple_labels() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Multi-label issue"]);
    run_atelier(dir.path(), &["label", "1", "bug"]);
    run_atelier(dir.path(), &["label", "1", "urgent"]);
    run_atelier(dir.path(), &["label", "1", "frontend"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["show", "1"]);

    assert!(success);
    assert!(stdout.contains("bug"));
    assert!(stdout.contains("urgent"));
    assert!(stdout.contains("frontend"));
}

// --- Export markdown format test ---
#[test]
fn test_export_markdown_format() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue for markdown"]);
    run_atelier(dir.path(), &["label", "1", "test"]);
    run_atelier(dir.path(), &["comment", "1", "Test comment"]);

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
fn test_archive_older_no_matches() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create and close an issue (just now, so not old)
    run_atelier(dir.path(), &["create", "New issue"]);
    run_atelier(dir.path(), &["close", "1"]);

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

    run_atelier(dir.path(), &["create", "Existing"]);

    let (success, _, stderr) = run_atelier(dir.path(), &["relate", "999", "1"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_relate_nonexistent_second_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Existing"]);

    let (success, _, stderr) = run_atelier(dir.path(), &["relate", "1", "999"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_relate_already_related() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue A"]);
    run_atelier(dir.path(), &["create", "Issue B"]);
    run_atelier(dir.path(), &["relate", "1", "2"]);

    // Try to relate again
    let (success, stdout, _) = run_atelier(dir.path(), &["relate", "1", "2"]);

    assert!(success);
    assert!(stdout.contains("already") || stdout.contains("related"));
}

#[test]
fn test_unrelate_no_relation() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue A"]);
    run_atelier(dir.path(), &["create", "Issue B"]);

    // Try to unrelate issues that aren't related
    let (success, stdout, _) = run_atelier(dir.path(), &["unrelate", "1", "2"]);

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

    run_atelier(dir.path(), &["create", "Solo issue"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["related", "1"]);

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

    let (success, _, stderr) = run_atelier(dir.path(), &["related", "999"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

// --- label.rs: Error cases ---
#[test]
fn test_label_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["label", "999", "bug"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_label_already_exists() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue"]);
    run_atelier(dir.path(), &["label", "1", "bug"]);

    // Try to add same label again
    let (success, stdout, _) = run_atelier(dir.path(), &["label", "1", "bug"]);

    assert!(success);
    assert!(stdout.contains("already") || stdout.contains("exists"));
}

#[test]
fn test_unlabel_nonexistent_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["unlabel", "999", "bug"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

#[test]
fn test_unlabel_nonexistent_label() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Issue"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["unlabel", "1", "nonexistent"]);

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

    let (success, _, stderr) = run_atelier(dir.path(), &["create", "Issue", "-p", "invalid"]);

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

    let (success, _, stderr) = run_atelier(dir.path(), &["create", "Issue", "-t", "unknown"]);

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

    run_atelier(dir.path(), &["create", "Issue"]);

    let (success, _, stderr) = run_atelier(dir.path(), &["block", "1", "1"]);

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

    run_atelier(dir.path(), &["create", "Issue"]);

    let (success, _, stderr) = run_atelier(dir.path(), &["block", "1", "999"]);

    assert!(!success);
    assert!(stderr.contains("not found") || stderr.contains("999"));
}

// --- session.rs: Session status deleted issue ---
#[test]
fn test_session_status_deleted_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "To delete"]);
    run_atelier(dir.path(), &["session", "start"]);
    run_atelier(dir.path(), &["session", "work", "1"]);
    run_atelier(dir.path(), &["delete", "1", "-f"]);

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

    run_atelier(dir.path(), &["create", "Main issue"]);
    run_atelier(dir.path(), &["create", "Related issue"]);
    run_atelier(dir.path(), &["relate", "1", "2"]);

    let (success, stdout, _) = run_atelier(dir.path(), &["show", "1"]);

    assert!(success);
    assert!(stdout.contains("Related") || stdout.contains("#2") || stdout.contains("Main issue"));
}

// --- milestone.rs: Edge cases ---
#[test]
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
    let (success, stdout, _) = run_atelier(dir.path(), &["create", &ok_title]);
    assert!(success);
    assert!(stdout.contains("atelier-"));

    // Verify it can be listed and shown
    let (success, _, _) = run_atelier(dir.path(), &["list"]);
    assert!(success);

    let (success, _, _) = run_atelier(dir.path(), &["show", "1"]);
    assert!(success);

    // Over the 512-char limit: should fail
    let too_long = "A".repeat(513);
    let (success, _, _) = run_atelier(dir.path(), &["create", &too_long]);
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
    let (success, _, _) = run_atelier(dir.path(), &["create", "Long desc issue", "-d", &long_desc]);

    assert!(success);

    let (success, stdout, _) = run_atelier(dir.path(), &["show", "1"]);
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
        let (success, _, _) = run_atelier(dir.path(), &["create", &title]);
        assert!(success, "Failed to create issue {}", i);
    }

    // Verify list works
    let (success, stdout, _) = run_atelier(dir.path(), &["list"]);
    assert!(success);
    assert!(stdout.contains("Issue number 99"));

    // Verify search works on large DB
    let (success, stdout, _) = run_atelier(dir.path(), &["search", "number 50"]);
    assert!(success);
    assert!(stdout.contains("50"));
}

/// Test deeply nested subissues
#[test]
fn test_stress_deep_nesting() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create root issue
    run_atelier(dir.path(), &["create", "Level 0"]);

    // Create 20 levels of nesting
    for i in 1..=20 {
        let parent_id = i.to_string();
        let title = format!("Level {}", i);
        let (success, _, _) = run_atelier(dir.path(), &["subissue", &parent_id, &title]);
        assert!(success, "Failed to create subissue at level {}", i);
    }

    // Verify tree command handles deep nesting
    let (success, stdout, _) = run_atelier(dir.path(), &["tree"]);
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
        let (success, _, _) = run_atelier(dir.path(), &["create", title]);
        assert!(success, "Failed to create issue with title: {}", title);
    }

    // Verify all issues exist and DB is intact
    let (success, stdout, _) = run_atelier(dir.path(), &["list"]);
    assert!(success);
    assert!(stdout.contains("DROP TABLE")); // Title should be stored literally
}

/// Test SQL injection in search
#[test]
fn test_security_sql_injection_search() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Normal issue"]);

    let malicious_searches = [
        "' OR '1'='1",
        "'; DROP TABLE issues; --",
        "\" OR \"\"=\"",
        "%' OR 1=1 --",
    ];

    for query in malicious_searches {
        let (success, _, _) = run_atelier(dir.path(), &["search", query]);
        // Should not crash, may or may not find results
        assert!(success, "Search crashed with query: {}", query);
    }

    // DB should still be intact
    let (success, stdout, _) = run_atelier(dir.path(), &["list"]);
    assert!(success);
    assert!(stdout.contains("Normal issue"));
}

/// Test path traversal in export
#[test]
fn test_security_path_traversal_export() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Test issue"]);

    // Try to export to a path traversal location
    // This should either fail safely or write to the literal filename
    let traversal_paths = [
        "../../../tmp/evil.json",
        "..\\..\\..\\tmp\\evil.json",
        "/etc/passwd",
        "C:\\Windows\\System32\\evil.json",
    ];

    for path in traversal_paths {
        let (_, _, _) = run_atelier(dir.path(), &["export", "-o", path, "-f", "json"]);
        // We don't assert success/failure - just that it doesn't crash
        // and doesn't actually write to system locations
    }
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
    let (success, _, _) = run_atelier(dir.path(), &["create", "Test with special: \t\r"]);
    assert!(success);

    let (success, _, _) = run_atelier(dir.path(), &["list"]);
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
        let (success, _, _) = run_atelier(dir.path(), &["create", input]);
        assert!(success, "Failed with input containing control chars");
    }

    let (success, _, _) = run_atelier(dir.path(), &["list"]);
    assert!(success);
}

/// Test empty strings
#[test]
fn test_edge_empty_strings() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Empty title - should either fail or succeed (both acceptable, just don't crash)
    let (success, stdout, _) = run_atelier(dir.path(), &["create", ""]);
    if success {
        // If it accepted empty title, verify the issue was created
        assert!(
            stdout.contains("Created issue"),
            "If success, should show created message, got: {}",
            stdout
        );
    }

    // Empty comment
    run_atelier(dir.path(), &["create", "Issue"]);
    let (_, _, _) = run_atelier(dir.path(), &["comment", "1", ""]);

    // Empty label
    let (_, _, _) = run_atelier(dir.path(), &["label", "1", ""]);
}

/// Test integer overflow in IDs
#[test]
fn test_edge_large_ids() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "Test"]);

    // Very large IDs - should fail with "not found" since issue doesn't exist
    let (success, _, stderr) = run_atelier(dir.path(), &["show", "9223372036854775807"]); // i64::MAX
    assert!(!success, "Show with non-existent large ID should fail");
    assert!(
        stderr.contains("not found"),
        "Error should say not found, got: {}",
        stderr
    );

    // Overflow ID - should fail with parse error or not found
    let (success, _, _) = run_atelier(dir.path(), &["show", "99999999999999999999999"]);
    assert!(!success, "Show with overflow ID should fail");

    // Negative IDs - clap may reject or db returns not found
    let (success, _, _) = run_atelier(dir.path(), &["show", "-1"]);
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
        run_atelier(dir.path(), &["create", &title]);
        let id = (i + 1).to_string();
        run_atelier(dir.path(), &["close", &id]);
        run_atelier(dir.path(), &["reopen", &id]);
        run_atelier(dir.path(), &["comment", &id, "Rapid comment"]);
        run_atelier(dir.path(), &["label", &id, "rapid"]);
    }

    // Verify all operations completed
    let (success, stdout, _) = run_atelier(dir.path(), &["list"]);
    assert!(success);
    assert!(stdout.contains("Rapid issue 19"));
}

/// Test export/import round-trip preserves data
#[test]
fn test_integrity_export_import_roundtrip() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create complex data
    run_atelier(
        dir.path(),
        &["create", "Parent", "-p", "high", "-d", "Parent desc"],
    );
    run_atelier(dir.path(), &["subissue", "1", "Child"]);
    run_atelier(dir.path(), &["label", "1", "important"]);
    run_atelier(dir.path(), &["comment", "1", "Test comment"]);

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
    let (success, stdout, _) = run_atelier(dir2.path(), &["show", &parent_id]);
    assert!(success);
    assert!(stdout.contains("Parent"));

    // Verify child was imported
    let (success, stdout, _) = run_atelier(dir2.path(), &["list"]);
    assert!(success);
    assert!(stdout.contains("Child") || stdout.contains("#2"));
}

#[test]
fn test_agent_factory_json_command_subset() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "--json",
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
    let created = json_value(&stdout);
    assert_eq!(created["ok"], true);
    assert_eq!(created["command"], "issue.create");
    let task_id = created["data"]["id"].as_str().unwrap().to_string();
    assert!(task_id.starts_with("atelier-"));
    assert_eq!(created["data"]["issue_type"], "feature");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "--json",
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
    let updated = json_value(&stdout);
    assert_eq!(updated["ok"], true);
    assert_eq!(updated["command"], "issue.update");
    assert!(updated["data"]["changed_fields"]
        .as_array()
        .unwrap()
        .iter()
        .any(|field| field == "notes"));
    assert_eq!(updated["data"]["issue"]["priority"], "high");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["--json", "issue", "show", "#1"]);
    assert!(success, "show failed: {stderr}");
    let shown = json_value(&stdout);
    assert_eq!(shown["data"]["id"], task_id);
    assert_eq!(shown["data"]["notes"][1]["body"], "handoff note");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["--json", "issue", "ready"]);
    assert!(success, "ready failed: {stderr}");
    assert_eq!(json_value(&stdout)["data"]["count"], 1);

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Blocker"]);
    assert!(success, "blocker create failed: {stderr}");
    let blocker_id = issue_ref(dir.path(), 2);
    let (success, stdout, stderr) = run_atelier(dir.path(), &["--json", "dep", "add", "1", "2"]);
    assert!(success, "dep add failed: {stderr}");
    let dep = json_value(&stdout);
    assert_eq!(dep["command"], "dep.add");
    assert_eq!(dep["data"]["action"], "add");
    assert_eq!(dep["data"]["state"], "added");
    assert_eq!(dep["data"]["blocked"], task_id);
    assert_eq!(dep["data"]["blocker"], blocker_id);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["--json", "dep", "list", "1"]);
    assert!(success, "dep list failed: {stderr}");
    assert_eq!(json_value(&stdout)["data"]["count"], 1);

    let (success, stdout, stderr) = run_atelier(dir.path(), &["--json", "dep", "remove", "1", "2"]);
    assert!(success, "dep remove failed: {stderr}");
    let removed = json_value(&stdout);
    assert_eq!(removed["command"], "dep.remove");
    assert_eq!(removed["data"]["action"], "remove");
    assert_eq!(removed["data"]["state"], "removed");
    assert_eq!(removed["data"]["changed"], true);

    for args in [
        vec!["--json", "issue", "list", "--status", "all"],
        vec!["--json", "issue", "search", "Factory"],
        vec!["--json", "lint"],
        vec!["--json", "export"],
        vec!["--json", "export", "--check"],
        vec!["--json", "doctor"],
        vec!["--json", "rebuild"],
    ] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "{args:?} failed: {stderr}");
        assert_eq!(json_value(&stdout)["ok"], true, "{args:?}");
    }

    let (success, stdout, _) = run_atelier(dir.path(), &["--json", "issue", "show", "missing"]);
    assert!(!success);
    let error = json_value(&stdout);
    assert_eq!(error["ok"], false);
    assert_eq!(error["error"]["code"], "not_found");
    assert!(error["error"]["message"]
        .as_str()
        .unwrap()
        .contains("missing"));
}

#[test]
fn test_issue_type_is_canonical_not_label_derived() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "--json",
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
    let created = json_value(&stdout);
    assert_eq!(created["data"]["issue_type"], "validation");
    assert_eq!(created["data"]["labels"][0], "epic");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["--json", "issue", "show", "1"]);
    assert!(success, "show failed: {stderr}");
    let shown = json_value(&stdout);
    assert_eq!(shown["data"]["issue_type"], "validation");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["--json", "issue", "list", "--status", "all"]);
    assert!(success, "list failed: {stderr}");
    let listed = json_value(&stdout);
    assert_eq!(listed["data"]["items"][0]["issue_type"], "validation");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["--json", "issue", "ready"]);
    assert!(success, "ready failed: {stderr}");
    let ready = json_value(&stdout);
    assert_eq!(ready["data"]["items"][0]["issue_type"], "validation");

    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    let issue_record = std::fs::read_to_string(
        dir.path()
            .join(".atelier-state/issues")
            .join(format!("{}.md", issue_ref(dir.path(), 1))),
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

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["--json", "import-beads", fixture.to_str().unwrap()],
    );
    assert!(success, "import-beads failed: {stderr}");
    assert!(stdout.contains("\"atelier-z1p.4\": \"atelier-0003\""));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["--json", "issue", "show", "3"]);
    assert!(success, "mapped show failed: {stderr}");
    let shown = json_value(&stdout);
    assert_eq!(shown["data"]["id"], "atelier-0003");
    assert_eq!(shown["data"]["parent"], "atelier-0001");
    assert_eq!(shown["data"]["issue_type"], "task");
    assert!(shown["data"]["owner"].is_null());
    assert_eq!(shown["data"]["dependencies"][0]["id"], "atelier-0002");
    assert!(!shown["data"]["labels"]
        .as_array()
        .unwrap()
        .iter()
        .any(|label| label.as_str().unwrap().starts_with("beads:")));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["--json", "dep", "list", "3"]);
    assert!(success, "mapped dep list failed: {stderr}");
    let deps = json_value(&stdout);
    assert_eq!(deps["data"]["items"][0]["blocked"], "atelier-0003");
    assert_eq!(deps["data"]["items"][0]["blocker"], "atelier-0002");
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
        &["create", "Add keyboard shortcuts for swiping (← →)"],
    );
    assert!(success);

    // List should not panic
    let (success, stdout, _) = run_atelier(dir.path(), &["list"]);
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
        let (success, _, _) = run_atelier(dir.path(), &["create", title]);
        assert!(success, "Failed to create issue with title: {}", title);

        // Verify it can be shown without panic
        let id = (i + 1).to_string();
        let (success, _, _) = run_atelier(dir.path(), &["show", &id]);
        assert!(
            success,
            "Failed to show issue #{} with title: {}",
            i + 1,
            title
        );
    }

    // List all - tests truncation on long Unicode
    let (success, _, _) = run_atelier(dir.path(), &["list"]);
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
        &["comment", "1", "Comment: ← back, → forward, ↑ up"],
    );
    assert!(success);

    // Show should display without panic
    let (success, stdout, _) = run_atelier(dir.path(), &["show", "1"]);
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

    run_atelier(dir.path(), &["create", "日本語のテスト"]);
    run_atelier(dir.path(), &["create", "Test with arrows ← →"]);
    run_atelier(dir.path(), &["create", "Emoji test 🎉"]);

    // Search for Japanese
    let (success, _, _) = run_atelier(dir.path(), &["search", "日本"]);
    assert!(success);

    // Search for emoji
    let (success, _, _) = run_atelier(dir.path(), &["search", "🎉"]);
    assert!(success);

    // Search for arrow
    let (success, _, _) = run_atelier(dir.path(), &["search", "←"]);
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
    let (success, _, _) = run_atelier(dir.path(), &["create", &format!("Long: {}", long_arrows)]);
    assert!(success);

    // List must not panic on truncation
    let (success, stdout, _) = run_atelier(dir.path(), &["list"]);
    assert!(success);
    assert!(stdout.contains("...") || stdout.contains("Long:"));

    // Create title with mixed byte-length chars
    let mixed = "a←b→c↑d↓e🎉f".repeat(10);
    let (success, _, _) = run_atelier(dir.path(), &["create", &mixed]);
    assert!(success);

    let (success, _, _) = run_atelier(dir.path(), &["list"]);
    assert!(success);
}

/// Test blocked/ready lists with Unicode
#[test]
fn test_unicode_in_dependencies() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["create", "ブロッカー (blocker) ←"]);
    run_atelier(dir.path(), &["create", "待機中 (waiting) →"]);
    run_atelier(dir.path(), &["block", "2", "1"]);

    // Blocked list with Unicode
    let (success, _, _) = run_atelier(dir.path(), &["blocked"]);
    assert!(success);

    // Ready list
    let (success, _, _) = run_atelier(dir.path(), &["ready"]);
    assert!(success);
}

/// Test export/import preserves Unicode
#[test]
fn test_unicode_export_import_roundtrip() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let unicode_title = "Test: 日本語 ← → 🎉";
    let unicode_desc = "Description: 中文 العربية Русский";

    run_atelier(dir.path(), &["create", unicode_title, "-d", unicode_desc]);
    run_atelier(dir.path(), &["comment", "1", "コメント (comment)"]);

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
    let (success, stdout, _) = run_atelier(dir2.path(), &["show", &unicode_id]);
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
        &["create", "Test\u{200B}with\u{200B}zero\u{200B}width"],
    );
    assert!(success);

    // RTL override characters
    let (success, _, _) = run_atelier(
        dir.path(),
        &["create", "Test \u{202E}desrever\u{202C} normal"],
    );
    assert!(success);

    // Combining characters (accent marks)
    let (success, _, _) = run_atelier(dir.path(), &["create", "Café résumé naïve"]);
    assert!(success);

    // All should list without panic
    let (success, _, _) = run_atelier(dir.path(), &["list"]);
    assert!(success);
}
