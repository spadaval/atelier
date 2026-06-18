use super::*;

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
