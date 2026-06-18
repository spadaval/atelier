use super::*;

fn session_id_from_start_output(stdout: &str) -> String {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("Session: "))
        .map(str::trim)
        .map(str::to_string)
        .expect("start output should include a session id")
}

#[test]
fn session_begin_show_list_and_end_round_trip() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, begin_out, stderr) = run_atelier(
        dir.path(),
        &[
            "session",
            "begin",
            "--role",
            "worker",
            "--agent",
            "codex",
            "--subskill",
            "implementer",
        ],
    );
    assert!(success, "session begin failed: {stderr}");
    assert!(begin_out.contains("[session] active"));
    assert!(begin_out.contains("Role:        worker"));
    assert!(begin_out.contains("Agent:       codex"));
    let session_id = begin_out
        .split_whitespace()
        .next()
        .expect("session id should print");

    let (success, list_out, stderr) = run_atelier(dir.path(), &["session", "list", "--active"]);
    assert!(success, "session list failed: {stderr}");
    assert!(list_out.contains(session_id));
    assert!(list_out.contains("active"));
    assert!(list_out.contains("worker"));

    let (success, show_out, stderr) = run_atelier(dir.path(), &["session", "show", session_id]);
    assert!(success, "session show failed: {stderr}");
    assert!(show_out.contains(session_id));
    assert!(show_out.contains("Subskill:    implementer"));

    let (success, end_out, stderr) = run_atelier(
        dir.path(),
        &["session", "end", session_id, "--reason", "done"],
    );
    assert!(success, "session end failed: {stderr}");
    assert!(end_out.contains("Ended session"));

    let (success, list_out, stderr) = run_atelier(dir.path(), &["session", "list", "--active"]);
    assert!(success, "session list after end failed: {stderr}");
    assert!(!list_out.contains(session_id));
}

#[test]
fn session_begin_rejects_invalid_role() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["session", "begin", "--role", "driver"]);

    assert!(!success);
    assert!(stderr.contains("Invalid session role"));
    assert!(stderr.contains("worker, reviewer, manager, admin"));
}

#[test]
fn start_auto_creates_active_mutating_session() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Session start item"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Session start item");
    commit_all(dir.path(), "start item");

    let (success, start_out, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "start failed: {stderr}");
    let session_id = session_id_from_start_output(&start_out);

    let (success, list_out, stderr) = run_atelier(dir.path(), &["session", "list", "--active"]);
    assert!(success, "session list failed: {stderr}");
    assert!(list_out.contains(&session_id), "{list_out}");
    assert!(list_out.contains("worker"), "{list_out}");
    assert!(list_out.contains("mutating"), "{list_out}");
    assert!(
        list_out.contains(&format!("issue/{issue_id}")),
        "{list_out}"
    );
}

#[test]
fn start_no_session_suppresses_session_creation() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "No session item"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "No session item");
    commit_all(dir.path(), "no session item");

    let (success, start_out, stderr) =
        run_atelier(dir.path(), &["start", &issue_id, "--no-session"]);
    assert!(success, "start --no-session failed: {stderr}");
    assert!(!start_out.contains("Session:"), "{start_out}");

    let (success, list_out, stderr) = run_atelier(dir.path(), &["session", "list", "--active"]);
    assert!(success, "session list failed: {stderr}");
    assert!(list_out.contains("(none)"), "{list_out}");
}

#[test]
fn start_requires_explicit_reuse_for_active_mutating_session() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Session epic", "--issue-type", "epic"],
    );
    assert!(success, "epic issue create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Session epic");
    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "First session item",
            "--parent",
            &epic_id,
        ],
    );
    assert!(success, "first issue create failed: {stderr}");
    let first_id = issue_id_by_title(dir.path(), "First session item");
    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Second session item",
            "--parent",
            &epic_id,
        ],
    );
    assert!(success, "second issue create failed: {stderr}");
    let second_id = issue_id_by_title(dir.path(), "Second session item");
    commit_all(dir.path(), "session reuse items");

    let (success, start_out, stderr) = run_atelier(dir.path(), &["start", &first_id]);
    assert!(success, "first start failed: {stderr}");
    let session_id = session_id_from_start_output(&start_out);
    commit_all(dir.path(), "first session started");

    let (success, _stdout, stderr) = run_atelier(dir.path(), &["start", &second_id]);
    assert!(!success, "second start should require explicit reuse");
    assert!(stderr.contains("Active mutating session"), "{stderr}");
    assert!(stderr.contains("--reuse-session"), "{stderr}");
    assert!(stderr.contains("--no-session"), "{stderr}");

    let (success, reuse_out, stderr) = run_atelier(
        dir.path(),
        &["start", &second_id, "--reuse-session", &session_id],
    );
    assert!(success, "reuse start failed: {stderr}");
    assert!(
        reuse_out.contains(&format!("Session: {session_id}")),
        "{reuse_out}"
    );
}

#[test]
fn start_rejects_no_session_with_reuse_session() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Flag conflict item"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Flag conflict item");
    commit_all(dir.path(), "flag conflict item");

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "start",
            &issue_id,
            "--no-session",
            "--reuse-session",
            "atelier-missing",
        ],
    );
    assert!(!success, "conflicting flags should fail");
    assert!(
        stderr.contains("Use either --no-session or --reuse-session"),
        "{stderr}"
    );
}

#[test]
fn status_man_and_history_show_session_context() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Visible session item"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Visible session item");
    commit_all(dir.path(), "visible session item");

    let (success, start_out, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "start failed: {stderr}");
    let session_id = session_id_from_start_output(&start_out);

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(
        status_out.contains("Current work:  1 issue(s)"),
        "{status_out}"
    );
    assert!(status_out.contains("Active sessions: 1"), "{status_out}");
    assert!(status_out.contains(&session_id), "{status_out}");
    assert!(
        status_out.contains(&format!("issue/{issue_id}")),
        "{status_out}"
    );

    let (success, man_out, stderr) = run_atelier(dir.path(), &["man", "worker"]);
    assert!(success, "man worker failed: {stderr}");
    assert!(man_out.contains("Active sessions: 1"), "{man_out}");
    assert!(man_out.contains(&session_id), "{man_out}");
    assert!(
        man_out.contains("atelier session list --active"),
        "{man_out}"
    );
    assert!(!man_out.contains("atelier session start"), "{man_out}");

    let (success, history_out, stderr) =
        run_atelier(dir.path(), &["history", "--issue", &issue_id]);
    assert!(success, "history issue failed: {stderr}");
    assert!(history_out.contains("session_started"), "{history_out}");
    assert!(
        history_out.contains(&format!("session/{session_id}")),
        "{history_out}"
    );

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &["session", "end", &session_id, "--reason", "done"],
    );
    assert!(success, "session end failed: {stderr}");

    let (success, history_out, stderr) =
        run_atelier(dir.path(), &["history", "--issue", &issue_id]);
    assert!(success, "history issue after end failed: {stderr}");
    assert!(history_out.contains("session_ended"), "{history_out}");
}
