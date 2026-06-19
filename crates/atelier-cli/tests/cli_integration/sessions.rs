use super::*;

fn write_attempt_activity(
    dir: &std::path::Path,
    issue_id: &str,
    activity_id: &str,
    role: &str,
    serial: u32,
    lifecycle: &str,
    summary: &str,
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
                "summary: \"{summary}\"\n",
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
            lifecycle = lifecycle,
            summary = summary
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
        "Worker attempt started",
    );
    write_attempt_activity(
        dir.path(),
        &issue_id,
        "20260618T170001000000Z",
        "worker",
        1,
        "updated",
        "Worker attempt progressed",
    );

    let activity_before = issue_activity_texts(dir.path(), &issue_id);
    let (success, list_out, stderr) = run_atelier(dir.path(), &["session", "list", "--active"]);
    assert!(success, "session list failed: {stderr}");
    let attempt_id = format!("{issue_id}/worker/1");
    assert!(list_out.contains(&attempt_id), "{list_out}");
    assert!(list_out.contains("active"), "{list_out}");
    assert!(
        list_out.contains(&format!("issue/{issue_id}")),
        "{list_out}"
    );
    assert!(
        list_out.contains("recent=\"work_started updated - Worker attempt progressed\""),
        "{list_out}"
    );
    assert_eq!(
        activity_before,
        issue_activity_texts(dir.path(), &issue_id),
        "session list must not mutate issue activity"
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
    assert!(show_out.contains("Activity:"), "{show_out}");
    assert!(
        show_out.contains("work_started started - Worker attempt started"),
        "{show_out}"
    );
    assert!(
        show_out.contains("work_started updated - Worker attempt progressed"),
        "{show_out}"
    );
    assert_eq!(
        activity_before,
        issue_activity_texts(dir.path(), &issue_id),
        "session show must not mutate issue activity"
    );
}

#[test]
fn session_help_exposes_only_inspection_commands() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, help_out, stderr) = run_atelier(dir.path(), &["session", "--help"]);
    assert!(success, "session help failed: {stderr}");
    assert!(help_out.contains("show"), "{help_out}");
    assert!(help_out.contains("list"), "{help_out}");
    assert!(!help_out.contains("begin"), "{help_out}");
    assert!(!help_out.contains("end"), "{help_out}");
    assert!(!help_out.contains("mutating"), "{help_out}");
    assert!(!help_out.contains("admin"), "{help_out}");
    assert!(!help_out.contains("coordination"), "{help_out}");
}

#[test]
fn session_begin_and_end_are_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for command in ["begin", "end"] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &["session", command]);
        assert!(!success, "session {command} should be rejected:\n{stdout}");
        assert!(
            stderr.contains(&format!("unrecognized subcommand '{command}'")),
            "{stderr}"
        );
    }
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

    let (success, start_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
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

#[test]
fn workflow_milestones_emit_issue_attempt_metadata_without_session_records() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Attempt metadata epic",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Attempt metadata epic");
    commit_all(dir.path(), "attempt metadata item");

    let (success, _, stderr) = run_atelier_with_env(
        dir.path(),
        &["issue", "transition", &issue_id, "start"],
        &[
            ("ATELIER_AGENT", "worker-agent"),
            ("ATELIER_SUBSKILL", "implement"),
        ],
    );
    assert!(success, "start failed: {stderr}");

    let (success, _, stderr) = run_atelier_with_env(
        dir.path(),
        &["issue", "transition", &issue_id, "request_review"],
        &[
            ("ATELIER_AGENT", "review-agent"),
            ("ATELIER_SUBSKILL", "review"),
        ],
    );
    assert!(success, "request_review failed: {stderr}");

    let (success, _, stderr) = run_atelier_with_env(
        dir.path(),
        &["issue", "transition", &issue_id, "request_validation"],
        &[
            ("ATELIER_AGENT", "validator-agent"),
            ("ATELIER_SUBSKILL", "validate"),
        ],
    );
    assert!(success, "request_validation failed: {stderr}");

    let (success, _, stderr) = run_atelier_with_env(
        dir.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{issue_id}"),
            "--kind",
            "validation",
            "attempt metadata proof",
        ],
        &[
            ("ATELIER_AGENT", "validator-agent"),
            ("ATELIER_SUBSKILL", "validate"),
        ],
    );
    assert!(success, "evidence record failed: {stderr}");

    let activity = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activity,
        "transition_applied",
        &[
            "transition: \"start\"",
            "attempt:\n",
            "  role: worker",
            "  serial: 1",
            "  lifecycle: started",
            "  agent: worker-agent",
            "  subskill: implement",
        ],
    );
    assert_activity_contains(
        &activity,
        "work_finished",
        &[
            "transition: \"request_review\"",
            "  role: worker",
            "  serial: 1",
            "  lifecycle: finished",
        ],
    );
    assert_activity_contains(
        &activity,
        "transition_applied",
        &[
            "transition: \"request_review\"",
            "  role: reviewer",
            "  serial: 1",
            "  lifecycle: started",
            "  subskill: review",
        ],
    );
    assert_activity_contains(
        &activity,
        "work_finished",
        &[
            "transition: \"request_validation\"",
            "  role: reviewer",
            "  serial: 1",
            "  lifecycle: finished",
        ],
    );
    assert_activity_contains(
        &activity,
        "transition_applied",
        &[
            "transition: \"request_validation\"",
            "  role: validator",
            "  serial: 1",
            "  lifecycle: started",
            "  subskill: validate",
        ],
    );
    assert_activity_contains(
        &activity,
        "evidence_attached",
        &[
            "attempt:\n",
            "  role: validator",
            "  serial: 1",
            "  lifecycle: updated",
        ],
    );

    let (success, list_out, stderr) = run_atelier(dir.path(), &["session", "list"]);
    assert!(success, "session list failed: {stderr}");
    assert!(
        list_out.contains(&format!("{issue_id}/worker/1")),
        "{list_out}"
    );
    assert!(
        list_out.contains(&format!("{issue_id}/reviewer/1")),
        "{list_out}"
    );
    assert!(
        list_out.contains(&format!("{issue_id}/validator/1")),
        "{list_out}"
    );
    assert!(list_out.contains("finished"), "{list_out}");
    assert!(list_out.contains("active"), "{list_out}");

    assert!(
        session_record_files(dir.path()).is_empty(),
        "workflow milestones must not create standalone session records"
    );
}

fn session_record_files(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    std::fs::read_dir(dir.join(".atelier").join("sessions"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("md"))
        .collect()
}
