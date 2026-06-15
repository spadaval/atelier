use super::support::*;

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

    let (workflow_success, _workflow_out, workflow_err) =
        run_atelier(dir.path(), &["workflow", "check"]);
    assert!(!workflow_success, "workflow check must fail closed");
    assert!(workflow_err.contains("Canonical tracker Markdown is invalid"));
    assert!(workflow_err.contains("atelier lint"));
}

fn assert_degraded_repair_guidance(stderr: &str, issue_id: &str) {
    for needle in [
        "Tracker degraded".to_string(),
        "orientation only".to_string(),
        "Recovery: 1. run `atelier lint`".to_string(),
        "4. rerun the blocked command".to_string(),
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
    let (success, closed_work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Newest closed work"]);
    assert!(success, "closed work create failed: {stderr}");
    assert!(closed_work_out.contains("Created issue atelier-"));
    let closed_work_id = issue_id_by_title(dir.path(), "Newest closed work");
    let closed_work_id = closed_work_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-work", closed_id, closed_work_id],
    );
    assert!(success, "closed mission add work failed: {stderr}");
    close_issue_with_evidence(dir.path(), closed_work_id, Some("done"));

    let (success, closed_evidence, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
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
        &[
            "mission",
            "close",
            &closed_id,
            "--reason",
            "closed by integration fixture",
        ],
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
    close_issue_with_evidence(dir.path(), done_id, Some("done"));

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
            "record",
            "--kind",
            "test",
            "--result",
            "pass",
            "older mission evidence",
        ],
    );
    assert!(success, "evidence record failed: {stderr}");
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
    assert!(stdout.contains("Evidence gaps: 4"), "{stdout}");
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
            "[epic] {epic_id} [todo] medium - Mission epic | ready 1, blocked 1, done 1"
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
    assert!(status_out.contains("Selectable Work"));
    assert!(status_out.contains(&format!(
        "Ready status work | ready: no open blockers; parent {epic_id}; proof missing"
    )));
    assert!(status_out.contains("Blocked Work"));
    assert!(status_out.contains(&format!(
        "Blocked status work | blocked by {blocker_id}; parent {epic_id}; proof missing"
    )));
    assert!(status_out.contains("Blockers"));
    assert!(status_out.contains("Evidence"));
    assert!(status_out.contains("Direct mission evidence: none"));
    assert!(status_out.contains("Reliability"));
    assert!(status_out.contains("Projection Freshness: current"));
    assert!(status_out.contains("Malformed Work: none"));
    assert!(status_out.contains("Missing Outcome Sections: none"));
    assert!(status_out.contains("Missing Evidence Sections: none"));
    assert!(status_out.contains("Attached Proof: missing"));
    assert!(status_out.contains("Open Blockers: 1 open"));
    assert!(status_out.contains(&format!("atelier mission status --closeout {mission_id}")));
    assert!(status_out.contains("atelier lint"));
    assert!(status_out.contains("atelier doctor"));
    assert!(status_out.contains("Closeout Gates"));
    assert!(!status_out.contains("Advanced Validator Detail"));
    assert!(!status_out.contains("advanced closeout validator failure detected."));
    let (success, verbose_status_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", "--verbose", mission_id]);
    assert!(success, "verbose mission status failed: {stderr}");
    assert!(verbose_status_out.contains("Advanced Validator Detail"));
    assert!(verbose_status_out.contains("advanced closeout validator failure detected."));
    assert!(status_out.contains("Next Commands"));
    assert!(status_out.contains(&format!(
        "Inspect mission record (durable intent and linked work): atelier mission show {mission_id}"
    )));
    assert!(status_out.contains(&format!(
        "Refresh mission status (current blockers and closeout gates): atelier mission status {mission_id}"
    )));
    assert!(status_out.contains("Resolve open blockers before assigning more implementation work"));
    assert!(!status_out.contains("ready item(s)): atelier issue list --ready"));
    assert!(!status_out.contains("selectable issue(s)): atelier start"));
    assert!(
        status_out.contains("Record validation proof"),
        "{status_out}"
    );
    assert!(
        status_out.contains(
            "atelier evidence record --target issue/<id> --kind validation --result pass \"...\""
        ),
        "{status_out}"
    );
    assert!(
        !status_out.contains("workflow validate"),
        "normal mission next commands must not route to raw workflow validators:\n{status_out}"
    );

    let (success, quiet_out, stderr) =
        run_atelier(dir.path(), &["--quiet", "mission", "status", mission_id]);
    assert!(success, "quiet mission status failed: {stderr}");
    assert!(quiet_out.contains(&format!("{mission_id} health=blocked")));
    assert!(quiet_out.contains("evidence_gaps="), "{quiet_out}");
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
    close_issue_with_evidence(dir.path(), work_id, Some("done"));
    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--result",
            "pass",
            "closeout evidence",
        ],
    );
    assert!(success, "closeout evidence record failed: {stderr}");
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
    assert!(
        closeout_status.contains("Closeout: ready"),
        "unexpected closeout mission status:\n{closeout_status}"
    );
    assert!(closeout_status.contains("Reliability"));
    assert!(closeout_status.contains("Attached Proof: complete"));
    assert!(closeout_status.contains("Docs/Help Drift: clear"));
    assert!(closeout_status.contains("Ignored Test Review: current"));
    assert!(closeout_status.contains("Open Blockers: none"));
    assert!(closeout_status.contains(&format!(
        "Close mission (all closeout gates pass): atelier mission close {closeout_mission} --reason \"...\""
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
    assert!(!stale_status.contains("advanced closeout validator failure detected."));
}

#[test]
fn test_mission_status_deduplicates_duplicate_reachability() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "create", "Duplicate reachability"]);
    assert!(success, "mission create failed: {stderr}");
    assert!(mission_out.contains("Mission atelier-"));
    let mission_id = record_id_by_title(dir.path(), "missions", "Duplicate reachability");

    let (success, epic_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Duplicate epic", "--issue-type", "epic"],
    );
    assert!(success, "epic create failed: {stderr}");
    assert!(epic_out.contains("Created issue atelier-"));
    let epic_id = issue_id_by_title(dir.path(), "Duplicate epic");

    let (success, child_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "subissue",
            epic_id.as_str(),
            "Duplicate child",
            "--description",
            "## Description\n\nDuplicate reachability child.\n\n## Outcome\n\nMission status counts this child once.\n\n## Evidence\n\n- `atelier mission status <mission-id>` counts this child once and reports duplicate reachability.",
        ],
    );
    assert!(success, "child create failed: {stderr}");
    assert!(child_out.contains(&epic_id));
    let child_id = issue_id_by_title(dir.path(), "Duplicate child");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-work", mission_id.as_str(), epic_id.as_str()],
    );
    assert!(success, "mission add epic failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "mission",
            "add-work",
            mission_id.as_str(),
            child_id.as_str(),
        ],
    );
    assert!(success, "mission add child failed: {stderr}");

    let (success, status_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", mission_id.as_str()]);
    assert!(success, "mission status failed: {stderr}");
    assert!(status_out.contains("Total: 2 ready"));
    assert!(status_out.contains(&format!(
        "Graph Hygiene: warning - duplicate reachability for 1 issue(s): {child_id} ({epic_id} + direct)"
    )));
    assert!(status_out.contains(
        "Totals count each unique issue once. Keep mission links on root issues or epics and let child issues flow through hierarchy."
    ));
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
    migrate_default_issue_workflow(dir.path());

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
    migrate_default_issue_workflow(dir.path());
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

    let (success, work_out, stderr) = run_atelier(dir.path(), &["start", issue_id]);
    assert!(success, "root start failed: {stderr}");
    assert!(work_out.contains(&format!("Mission: {mission_id} (active)")));
    assert!(work_out.contains("Tracked work is now derived from issue workflow status."));
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
    migrate_default_issue_workflow(dir.path());

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
    migrate_default_issue_workflow(dir.path());
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

    let (success, work_out, stderr) = run_atelier(dir.path(), &["start", issue_id]);
    assert!(success, "outside root start failed: {stderr}");
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
    let (success, closed_work_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Closed only work"]);
    assert!(success, "closed-only work create failed: {stderr}");
    assert!(closed_work_out.contains("Created issue atelier-"));
    let closed_work_id = issue_id_by_title(dir.path(), "Closed only work");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "add-work", &closed_id, &closed_work_id],
    );
    assert!(success, "closed-only mission add work failed: {stderr}");
    close_issue_with_evidence(dir.path(), &closed_work_id, Some("done"));

    let (success, evidence_out, stderr) = run_atelier(
        dir.path(),
        &[
            "evidence",
            "record",
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
        &[
            "mission",
            "close",
            &closed_id,
            "--reason",
            "closed by integration fixture",
        ],
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
    edit_canonical_record(dir.path(), "missions", &mission_id, |markdown| {
        markdown.replace("schema: \"atelier.mission\"", "schema: \"atelier.issue\"")
    });
    remove_projection_state(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(!success, "rebuild should reject mission schema drift");
    assert!(
        stderr.contains("Unsupported schema 'atelier.issue'")
            && stderr.contains("expected atelier.mission"),
        "unexpected rebuild error: {stderr}"
    );
}

#[test]
fn test_projection_query_distinguishes_schema_drift_from_malformed_records() {
    let schema_dir = tempdir().unwrap();
    init_atelier(schema_dir.path());

    let (success, _, stderr) = run_atelier(schema_dir.path(), &["issue", "create", "Schema drift"]);
    assert!(success, "issue create failed: {stderr}");
    let schema_issue_id = issue_id_by_title(schema_dir.path(), "Schema drift");
    edit_canonical_issue(schema_dir.path(), &schema_issue_id, |markdown| {
        markdown.replace("schema_version: 1", "schema_version: 99")
    });
    remove_projection_state(schema_dir.path());

    let (success, _, stderr) = run_atelier(schema_dir.path(), &["issue", "list"]);
    assert!(!success, "schema drift should block projection query");
    assert!(
        stderr.contains("schema this atelier binary does not understand")
            && stderr.contains("target/debug/atelier")
            && stderr.contains("update the installed `atelier` binary")
            && stderr.contains("Unsupported schema_version 99"),
        "schema drift diagnostic should name stale-binary repair: {stderr}"
    );
    assert!(
        !stderr.contains("fix canonical tracker records before querying"),
        "schema drift should not be presented as ordinary malformed records: {stderr}"
    );

    let malformed_dir = tempdir().unwrap();
    init_atelier(malformed_dir.path());
    let (success, _, stderr) = run_atelier(
        malformed_dir.path(),
        &["issue", "create", "Malformed source"],
    );
    assert!(success, "issue create failed: {stderr}");
    let malformed_issue_id = issue_id_by_title(malformed_dir.path(), "Malformed source");
    corrupt_issue_title_yaml(
        malformed_dir.path(),
        &malformed_issue_id,
        "Malformed source",
    );
    remove_projection_state(malformed_dir.path());

    let (success, _, stderr) = run_atelier(malformed_dir.path(), &["issue", "list"]);
    assert!(!success, "malformed records should block projection query");
    assert!(
        stderr.contains("recovery: 1. run `atelier lint`")
            && stderr.contains("2. fix the named canonical Markdown record")
            && stderr.contains("4. rerun the blocked command")
            && stderr.contains("Invalid YAML front matter"),
        "malformed diagnostic should stay record-focused: {stderr}"
    );
    assert!(
        !stderr.contains("schema this atelier binary does not understand"),
        "malformed records should not be presented as stale binary drift: {stderr}"
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

    let (success, list_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(success, "fresh list failed: {stderr}");
    assert!(list_out.contains("Indexed title"));

    edit_canonical_issue(dir.path(), &issue_id, |markdown| {
        markdown.replace("Indexed title", "Markdown title")
    });

    let (success, list_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(success, "stale list should transparently rebuild: {stderr}");
    assert!(list_out.contains("Markdown title"));
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
        edit_canonical_issue(dir.path(), issue_id, |markdown| {
            markdown.replace(
                &format!("title: \"Bulk indexed {index}\""),
                &format!("title: \"Bulk markdown {index}\""),
            )
        });
    }

    let (success, stdout, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(!success, "export check should report stale projection");
    assert!(
        stderr.contains("12 indexed sources changed")
            && stderr.contains("showing first 5")
            && stderr.contains("recovery: 1. run `atelier lint`")
            && stderr.contains("3. run `atelier doctor --fix`")
            && stderr.contains("4. rerun the blocked command"),
        "stale diagnostics should be bounded and actionable: {stderr}"
    );
    assert!(
        stderr.lines().count() < 12,
        "stale diagnostics should not dump every changed source: {stderr}"
    );

    let (success, list_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
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

    let first_path = canonical_issue_path(dir.path(), &first_id);
    let first_markdown = read_canonical_record(dir.path(), "issues", &first_id);
    std::fs::remove_file(&first_path).unwrap();

    let (success, list_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
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
status: "todo"
title: "Unindexed issue"
updated_at: "2026-06-10T12:00:00+00:00"
---

## Description

Body

## Outcome

The unindexed issue is discoverable after rebuild.

## Evidence

- `atelier search Unindexed` shows the record.
"#,
    )
    .unwrap();

    let (success, search_out, stderr) = run_atelier(dir.path(), &["search", "Unindexed"]);
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
    let first_body = "## Description\n\nProjection root body.\n\n## Outcome\n\nProjection root remains queryable after rebuild.\n\n## Evidence\n\n- manual check: `atelier lint` output prints `Lint passed.` after automatic rebuild.";
    let second_body = "## Description\n\nProjection leaf body.\n\n## Outcome\n\nProjection leaf remains linked after rebuild.\n\n## Evidence\n\n- manual check: `atelier issue blocked <id>` output shows the linked root.";

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
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "block", &second_id, &first_id]);
    assert!(success, "issue block failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    ensure_issue_closeout_sections(dir.path(), &first_id);
    ensure_issue_closeout_sections(dir.path(), &second_id);

    std::fs::write(dir.path().join(".atelier/manifest.json"), "{}\n").unwrap();
    std::fs::write(dir.path().join(".atelier/graph.json"), "{}\n").unwrap();
    let (success, ready_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(
        success,
        "derived files should not stale issue list --ready: {stderr}"
    );
    assert!(ready_out.contains("Projection root"));

    edit_canonical_issue(dir.path(), &first_id, |markdown| {
        markdown.replace("Projection root", "Projection root changed")
    });

    let (success, dep_out, stderr) = run_atelier(dir.path(), &["issue", "blocked", &second_id]);
    assert!(
        success,
        "stale issue blocked should transparently rebuild: {stderr}"
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

    let body = "## Description\n\nTemp rebuild filter body.\n\n## Outcome\n\nQuery, lint, export, and doctor ignore rebuild temp files.\n\n## Evidence\n\n- manual check: `atelier lint` output prints `Lint passed.`, `atelier export --check` exits 0, and `atelier doctor` exits 0 while rebuild temp files exist.";
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
    ensure_issue_closeout_sections(dir.path(), &issue_id);
    write_ignored_canonical_artifacts(dir.path(), &issue_id);

    edit_canonical_issue(dir.path(), &issue_id, |markdown| {
        markdown.replace("Temp rebuild filter", "Temp rebuild filter changed")
    });

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
        assert!(
            success,
            "{args:?} should ignore rebuild tmp file:\nstdout: {stdout}\nstderr: {stderr}"
        );
        let combined = format!("{stdout}\n{stderr}");
        assert!(
            !combined.contains("rebuild-tmp")
                && !combined.contains(".md.lock")
                && !combined.contains(".md-journal")
                && !combined.contains("projection.lock"),
            "{args:?} diagnostics must not report ignored local artifacts: {combined}"
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

    let markdown = read_canonical_record(dir.path(), "issues", &issue_id);
    corrupt_issue_title_yaml(dir.path(), &issue_id, "Invalid Markdown source");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["export", "--check"]);
    assert!(
        !success,
        "invalid canonical Markdown should fail export check"
    );
    assert!(
        stderr.contains("canonical tracker Markdown is invalid")
            && stderr.contains("while running `atelier export --check`")
            && stderr.contains("atelier lint")
            && stderr.contains("2. fix the named canonical Markdown record")
            && stderr.contains("4. rerun `atelier export --check`")
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
            && stderr.contains("Recovery: 1. run `atelier lint`")
            && stderr.contains("4. rerun the blocked command")
            && stderr.contains("atelier lint")
            && stderr.contains("Invalid YAML front matter")
            && stderr.contains(&format!(".atelier/issues/{issue_id}.md")),
        "unexpected invalid Markdown error: {stderr}"
    );
    assert!(
        !stderr.contains("Projection index was stale; rebuilt local SQLite projection"),
        "invalid Markdown must not be silently repaired: {stderr}"
    );

    write_canonical_record(dir.path(), "issues", &issue_id, markdown);
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(
        success,
        "restored canonical Markdown should query: {stderr}"
    );
    assert!(show_out.contains("Invalid Markdown source"));
}

#[test]
fn test_lint_validates_canonical_markdown_after_source_edit() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Lint canonical source"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);

    let markdown = read_canonical_record(dir.path(), "issues", &issue_id);
    let invalid_markdown = markdown.replace(
        "title: \"Lint canonical source\"",
        "title: [Lint canonical source",
    );
    write_canonical_record(dir.path(), "issues", &issue_id, invalid_markdown);

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
    remove_projection_state(dir.path());

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

    edit_canonical_issue(dir.path(), &issue_id, |markdown| {
        markdown.replace(
            "  blocks: []",
            "  blocks:\n  - kind: \"issue\"\n    id: \"atelier-missing\"",
        )
    });

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
        edit_canonical_issue(dir.path(), issue_id, |markdown| {
            markdown.replace(
                "  blocks: []",
                &format!("  blocks:\n  - kind: \"issue\"\n    id: \"{blocked_id}\""),
            )
        });
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
        |markdown, _issue_id| markdown.replace("status: \"todo\"", "status: \"bad-status\""),
        &["Invalid status", "Invalid status 'bad-status'"],
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
            edit_canonical_issue(dir, issue_id, |markdown| edit(&markdown, issue_id));
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
        "status": "done",
        "labels": ["bulk"]
      },
      {
        "client_ref": "issue.work",
        "title": "Implement bulk output",
        "issue_type": "feature",
        "priority": "high",
        "status": "in_progress",
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
        "labels": ["bulk", "mission"],
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

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild after bulk apply failed: {stderr}");

    let (success, view_out, stderr) = run_atelier(dir.path(), &["mission", "show", &mission_id]);
    assert!(success, "mission show after bulk apply failed: {stderr}");
    assert!(view_out.contains("Records: plans=1 milestones=1 evidence=1"));
    assert!(view_out.contains("Work: ready=0 blocked=0 done=0 backlog=1"));
    let mission_markdown = std::fs::read_to_string(
        dir.path()
            .join(".atelier/missions")
            .join(format!("{mission_id}.md")),
    )
    .unwrap();
    assert!(mission_markdown.contains("- \"bulk\"\n"));
    assert!(mission_markdown.contains("- \"mission\"\n"));
}
