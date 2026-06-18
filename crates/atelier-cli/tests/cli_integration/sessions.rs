use super::*;

fn write_attempt_activity(
    dir: &std::path::Path,
    issue_id: &str,
    activity_id: &str,
    role: &str,
    serial: u32,
    lifecycle: &str,
) {
    let activity_dir = dir
        .join(".atelier")
        .join("issues")
        .join(format!("{issue_id}.activity"));
    std::fs::create_dir_all(&activity_dir).unwrap();
    std::fs::write(
        activity_dir.join(format!("{activity_id}.md")),
        format!(
            concat!(
                "---\n",
                "schema: \"atelier.activity\"\n",
                "schema_version: 1\n",
                "id: \"{activity_id}\"\n",
                "subject_kind: \"issue\"\n",
                "subject_id: \"{issue_id}\"\n",
                "event_type: \"work_started\"\n",
                "actor: \"codex\"\n",
                "created_at: \"2026-06-18T17:00:00.000000Z\"\n",
                "summary: \"Attempt event\"\n",
                "attempt:\n",
                "  role: {role}\n",
                "  serial: {serial}\n",
                "  lifecycle: {lifecycle}\n",
                "  agent: \"codex\"\n",
                "  subskill: \"implement\"\n",
                "---\n\n",
                "attempt body\n"
            ),
            activity_id = activity_id,
            issue_id = issue_id,
            role = role,
            serial = serial,
            lifecycle = lifecycle
        ),
    )
    .unwrap();
}

#[test]
fn session_list_and_show_are_derived_from_issue_activity() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Derived attempt item"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Derived attempt item");
    write_attempt_activity(
        dir.path(),
        &issue_id,
        "20260618T170000000000Z",
        "worker",
        1,
        "started",
    );

    let (success, list_out, stderr) = run_atelier(dir.path(), &["session", "list", "--active"]);
    assert!(success, "session list failed: {stderr}");
    let attempt_id = format!("{issue_id}/worker/1");
    assert!(list_out.contains(&attempt_id), "{list_out}");
    assert!(list_out.contains("active"), "{list_out}");
    assert!(
        list_out.contains(&format!("issue/{issue_id}")),
        "{list_out}"
    );

    let (success, show_out, stderr) = run_atelier(dir.path(), &["session", "show", &attempt_id]);
    assert!(success, "session show failed: {stderr}");
    assert!(
        show_out.contains(&format!("{attempt_id} [session] active")),
        "{show_out}"
    );
    assert!(show_out.contains("Role:        worker"), "{show_out}");
    assert!(show_out.contains("Serial:      1"), "{show_out}");
    assert!(show_out.contains("Subskill:    implement"), "{show_out}");
}

#[test]
fn session_projection_ignores_standalone_session_files() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let sessions_dir = dir.path().join(".atelier").join("sessions");
    std::fs::create_dir_all(&sessions_dir).unwrap();
    std::fs::write(
        sessions_dir.join("atelier-stale.md"),
        "this is intentionally not a parseable live session record\n",
    )
    .unwrap();

    let (success, list_out, stderr) = run_atelier(dir.path(), &["session", "list", "--active"]);
    assert!(
        success,
        "session list should ignore stale .atelier/sessions files: {stderr}"
    );
    assert!(list_out.contains("(none)"), "{list_out}");
    assert!(!list_out.contains("atelier-stale"), "{list_out}");
}

#[test]
fn start_does_not_create_standalone_session_records() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Start without standalone session"],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Start without standalone session");
    commit_all(dir.path(), "start item");

    let (success, start_out, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "start failed: {stderr}");
    assert!(!start_out.contains("Session:"), "{start_out}");
    let session_files = std::fs::read_dir(dir.path().join(".atelier").join("sessions"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("md"))
        .collect::<Vec<_>>();
    assert!(
        session_files.is_empty(),
        "start must not create .atelier/sessions records: {session_files:?}"
    );
}
