use super::support::*;

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
fn test_diagnostics_help_scopes_json_as_advanced_local_only() {
    let dir = tempdir().unwrap();

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["--help"]);
    assert!(success, "root help failed: {stderr}");
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("diagnostics ")),
        "root help should not expose diagnostics:\n{stdout}"
    );

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["diagnostics", "--help"]);
    assert!(success, "diagnostics help failed: {stderr}");
    assert!(stdout.contains(
        "Advanced local command diagnostics; JSON is local-only telemetry, not workflow state"
    ));
    assert!(stdout.contains("slow"));

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["diagnostics", "slow", "--help"]);
    assert!(success, "diagnostics slow help failed: {stderr}");
    assert!(stdout.contains(
        "Summarize slow command telemetry as stable local-only JSON for performance analysis"
    ));
}

#[test]
fn test_top_level_help_only_shows_core_commands() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["--help"]);
    assert!(success, "help failed: {stderr}");
    assert!(stdout.contains("Mission and proof oriented work coordination for agents"));

    for heading in [
        "Setup:",
        "Orientation:",
        "Issues:",
        "Missions and planning:",
        "Records:",
        "Advanced work:",
        "Maintenance:",
        "Common commands:",
        "Options:",
    ] {
        assert!(stdout.contains(heading), "missing help heading {heading}");
    }

    for command in [
        "init", "man", "status", "start", "issue", "mission", "plan", "evidence", "history",
        "worktree", "lint", "doctor",
    ] {
        assert!(stdout.contains(command), "missing core command {command}");
    }
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("prime ")),
        "root help should not expose removed prime command:\n{stdout}"
    );
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("import-beads ")),
        "root help should not expose import-beads:\n{stdout}"
    );
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("dep ")),
        "root help should not expose dep:\n{stdout}"
    );
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("note ")),
        "root help should not expose generic note:\n{stdout}"
    );
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("link ")),
        "root help should not expose generic link:\n{stdout}"
    );
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("integrations ")),
        "root help should not expose integrations:\n{stdout}"
    );
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("export ")),
        "root help should not present export as a normal operator command:\n{stdout}"
    );
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("rebuild ")),
        "root help should not present rebuild as a normal operator command:\n{stdout}"
    );
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("diagnostics ")),
        "root help should not present diagnostics as a normal operator command:\n{stdout}"
    );

    for common in [
        "atelier man",
        "atelier man worker",
        "atelier man reviewer",
        "atelier man manager",
        "atelier man admin",
        "atelier issue list",
        "atelier issue list --ready",
        "atelier issue show <id>",
        "atelier mission list",
        "atelier mission show <id>",
        "atelier history --mission <id>",
        "atelier history --issue <id>",
        "atelier start <issue-id>",
        "atelier issue transition <issue-id> --options",
        "atelier issue close <issue-id> --reason",
        "atelier doctor",
        "atelier doctor --fix",
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
        "integrations",
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
        "abandon",
        "repair",
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
fn test_removed_root_active_pointer_commands_reject_with_guidance() {
    let dir = tempdir().unwrap();
    let (success, _, stderr) = run_atelier_raw(dir.path(), &["abandon", "--help"]);
    assert!(!success, "abandon help should be removed");
    assert!(
        stderr.contains("unrecognized subcommand 'abandon'"),
        "{stderr}"
    );
    assert!(stderr.contains("`atelier abandon` was removed"), "{stderr}");
    assert!(stderr.contains("atelier issue note <id>"), "{stderr}");

    let (success, _, stderr) = run_atelier_raw(dir.path(), &["repair", "--help"]);
    assert!(!success, "repair help should be removed");
    assert!(
        stderr.contains("unrecognized subcommand 'repair'"),
        "{stderr}"
    );
    assert!(stderr.contains("`atelier repair` was removed"), "{stderr}");
    assert!(stderr.contains("atelier doctor --fix"), "{stderr}");
    assert!(stderr.contains("atelier worktree repair <id>"), "{stderr}");
}

#[test]
fn test_generic_note_command_rejects_with_record_specific_guidance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["note", "add", "issue", "atelier-missing", "legacy note"],
    );

    assert!(!success, "generic note command should be removed");
    assert!(stderr.contains("was removed"));
    assert!(stderr.contains("atelier issue note atelier-missing"));
}

#[test]
fn test_generic_link_command_rejects_with_record_specific_guidance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["link", "add", "issue", "atelier-a", "issue", "atelier-b"],
    );

    assert!(!success, "generic link command should be removed");
    assert!(stderr.contains("`atelier link` was removed"));
    assert!(stderr.contains("atelier mission add-work"));
    assert!(stderr.contains("atelier mission unlink"));
    assert!(stderr.contains("atelier issue block"));
    assert!(stderr.contains("atelier issue unblock"));
    assert!(stderr.contains("atelier evidence attach"));
    assert!(stderr.contains("atelier graph impact"));
}

#[test]
fn test_workflow_help_is_scoped_as_advanced_internal_diagnostic() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["workflow", "--help"]);
    assert!(success, "workflow help failed: {stderr}");
    assert!(stdout.contains("Advanced/debug workflow policy diagnostics"));
    assert!(!stdout.contains("\n  init"));
    assert!(stdout.contains("check"));
    assert!(stdout.contains("normal operator checks use lint and status surfaces"));
    assert!(!stdout.contains("validate"));
}

#[test]
fn test_evidence_help_hides_predecessor_subcommands() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["evidence", "--help"]);
    assert!(success, "evidence help failed: {stderr}");
    assert!(stdout.contains("record"));
    assert!(stdout.contains("attach"));
    assert!(!stdout.contains("\n  add "));
    assert!(!stdout.contains("\n  capture "));
}

#[test]
fn test_evidence_record_help_shows_issue_targeted_manual_and_command_flows() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["evidence", "record", "--help"]);
    assert!(success, "evidence record help failed: {stderr}");
    assert!(stdout.contains("issue/<id>"));
    assert!(stdout.contains(
        "atelier evidence record --target issue/<id> --kind validation --result pass \"summary\""
    ));
    assert!(stdout.contains(
        "atelier evidence record --target issue/<id> --kind test --result pass -- <command>"
    ));
    assert!(stdout.contains("Use `evidence attach` only when you need to reuse"));
}

#[test]
fn test_agent_factory_guidance_avoids_raw_workflow_validate_commands() {
    let guidance =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("AGENTFACTORY.md"))
            .unwrap();
    assert!(guidance.contains("Hidden workflow diagnostics are not normal"));
    assert!(guidance.contains("## Validation Routing"));
    assert!(!guidance.contains("atelier workflow validate issue"));
    assert!(!guidance.contains("atelier workflow validate mission"));
    assert!(!guidance.contains("## Checks"));
    assert!(!guidance.contains("atelier worktree remove <issue-id>"));
    assert!(!guidance.contains("cargo nextest run --profile extended --run-ignored=only"));
}

#[test]
fn test_mission_help_uses_show_not_view() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["mission", "--help"]);
    assert!(success, "mission help failed: {stderr}");

    assert!(stdout.contains("show"));
    assert!(stdout.contains("unlink"));
    assert!(!stdout.contains("view"));
}

#[test]
fn test_graph_help_describes_mission_issue_graphs() {
    let dir = tempdir().unwrap();

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["graph", "--help"]);
    assert!(success, "graph help failed: {stderr}");
    assert!(stdout.contains("Mission and issue graph commands"));

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["graph", "impact", "--help"]);
    assert!(success, "graph impact help failed: {stderr}");
    assert!(stdout.contains("mission work"));
    assert!(stdout.contains("Mission or issue ID"));

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["graph", "tree", "--help"]);
    assert!(success, "graph tree help failed: {stderr}");
    assert!(stdout.contains("missions and issues"));
}

#[test]
fn test_mission_create_help_names_generated_sections() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["mission", "create", "--help"]);
    assert!(success, "mission create help failed: {stderr}");

    assert!(stdout.contains("generated Intent, Constraints, Risks, and Validation sections"));
    assert!(stdout
        .contains("Intent section text; this does not replace the full mission Markdown body"));
    assert!(stdout.contains("Constraints section bullet"));
    assert!(stdout.contains("Risks section bullet"));
    assert!(stdout.contains("Validation section bullet"));
}

#[test]
fn test_mission_status_help_exposes_closeout_drilldown() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["mission", "status", "--help"]);
    assert!(success, "mission status help failed: {stderr}");

    assert!(stdout.contains("--closeout"));
    assert!(stdout.contains("Show closeout audit detail"));
}

#[test]
fn test_mission_help_exposes_close_with_reason() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["mission", "--help"]);
    assert!(success, "mission help failed: {stderr}");

    assert!(stdout.contains("close"));
    assert!(stdout.contains("Close a mission after all closeout gates pass"));

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["mission", "close", "--help"]);
    assert!(success, "mission close help failed: {stderr}");
    assert!(stdout.contains("--reason <REASON>"));
    assert!(stdout.contains("Mission closeout reason recorded in the mission closeout notes"));
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
    assert!(stdout.contains("Current work:"));
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
fn test_root_status_reports_active_mission_contract_fields() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Status focus"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Status focus");
    let mission_id = mission_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", mission_id, "--status", "active"],
    );
    assert!(success, "mission activate failed: {stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Ready focus"]);
    assert!(success, "ready issue create failed: {stderr}");
    let ready_id = issue_id_by_title(dir.path(), "Ready focus");
    let ready_id = ready_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Blocked focus"]);
    assert!(success, "blocked issue create failed: {stderr}");
    let blocked_id = issue_id_by_title(dir.path(), "Blocked focus");
    let blocked_id = blocked_id.as_str();

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Focus blocker"]);
    assert!(success, "blocker issue create failed: {stderr}");
    let blocker_id = issue_id_by_title(dir.path(), "Focus blocker");
    let blocker_id = blocker_id.as_str();

    for issue_id in [ready_id, blocked_id] {
        let (success, _, stderr) =
            run_atelier(dir.path(), &["mission", "add-work", mission_id, issue_id]);
        assert!(success, "mission add work failed for {issue_id}: {stderr}");
    }
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "block", blocked_id, blocker_id]);
    assert!(success, "block issue failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "note", ready_id, "Recent focus note"],
    );
    assert!(success, "issue note failed: {stderr}");
    commit_all(dir.path(), "status focus baseline");
    std::fs::write(dir.path().join("status-dirty.txt"), "dirty").unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(stdout.contains("Local State"));
    assert!(stdout.contains("Worktree: dirty"));
    assert!(stdout.contains("status-dirty.txt"));
    assert!(stdout.contains("Branch:"));
    assert!(stdout.contains("Active Mission"));
    assert!(stdout.contains(&format!("{mission_id} - Status focus")));
    assert!(stdout.contains("Health:   blocked"));
    assert!(stdout.contains("Work:     ready 1, blocked 1, done 0, backlog 0"));
    assert!(stdout.contains("Ready In Active Mission"));
    assert!(stdout.contains(ready_id));
    assert!(stdout.contains(&format!(
        "{ready_id} - Ready focus | ready: no open blockers; mission-linked root; proof missing"
    )));
    assert!(stdout.contains("Immediate Blockers"));
    assert!(stdout.contains(blocker_id));
    assert!(stdout.contains("Recent Activity"));
    assert!(stdout.contains(ready_id));
    assert!(stdout.contains("Added note"));
    assert!(stdout.contains(&format!(
        "Inspect active mission health ({mission_id}): atelier mission status {mission_id}"
    )));
    assert!(stdout.contains(&format!(
        "Start selectable active-mission work (1 selectable issue(s)): atelier start {ready_id}"
    )));
    assert!(
        !stdout.contains("workflow validate"),
        "normal status next actions must not route to raw workflow validators:\n{stdout}"
    );
}

#[test]
fn test_root_status_guides_current_work_to_transition_options() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Active focus"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Active focus");
    let mission_id = mission_id.as_str();
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["mission", "update", mission_id, "--status", "active"],
    );
    assert!(success, "mission activate failed: {stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Active item"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Active item");
    let issue_id = issue_id.as_str();
    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", mission_id, issue_id]);
    assert!(success, "mission add work failed: {stderr}");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "active status baseline");

    let (success, start_out, stderr) = run_atelier(dir.path(), &["start", issue_id]);
    assert!(success, "start failed: {stderr}");
    assert!(start_out.contains("Next Commands"));
    assert!(start_out.contains("Inspect checkout status: atelier status"));
    assert!(start_out.contains(&format!(
        "Inspect mission selection and blockers: atelier mission status {mission_id}"
    )));
    assert!(start_out.contains(&format!(
        "Inspect work transitions: atelier issue transition {issue_id} --options"
    )));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(stdout.contains("Ready work:    0"));
    assert!(stdout.contains("Current work:  1 issue"));
    assert!(stdout.contains(&format!("{issue_id} - Active item [in_progress]")));
    assert!(stdout.contains("Health:   active"));
    assert!(stdout.contains("Work:     ready 0, active 1, blocked 0, done 0, backlog 0"));
    assert!(stdout.contains("Ready In Active Mission"));
    let ready_section = stdout
        .split("Ready In Active Mission")
        .nth(1)
        .expect("ready section missing")
        .split("Immediate Blockers")
        .next()
        .expect("immediate blockers section missing");
    assert!(!ready_section.contains(&format!("{issue_id} - Active item")));
    assert!(stdout.contains(&format!(
        "Inspect current work transitions (1 in progress; first {issue_id}): atelier issue transition {issue_id} --options"
    )));
    assert!(!stdout.contains("atelier abandon"));
    assert!(!stdout.contains("Start ready active-mission work"));
    assert!(!stdout.contains(&format!("atelier start {issue_id}")));
    assert!(
        !stdout.contains("workflow validate"),
        "normal status next actions must not route to raw workflow validators:\n{stdout}"
    );
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
fn test_workflow_configuration_docs_describe_internal_diagnostics() {
    let docs = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/product/workflow-configuration.md"),
    )
    .unwrap();
    assert!(!docs.contains("The future `atelier workflow validate` command"));
    assert!(
        !docs.contains("emit JSON containing `path`, `sha256`, `result`, `errors`, and `warnings`")
    );
    assert!(docs.contains("atelier lint"));
    assert!(docs.contains("atelier doctor"));
}

#[test]
fn test_diagnostics_json_docs_define_local_operator_boundary() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let diagnostics =
        std::fs::read_to_string(manifest.join("docs/architecture/local-command-diagnostics.md"))
            .unwrap();
    let cli_surface =
        std::fs::read_to_string(manifest.join("docs/product/cli-surface.md")).unwrap();
    let validation =
        std::fs::read_to_string(manifest.join("docs/architecture/quality/validation.md")).unwrap();

    assert!(diagnostics.contains("Diagnostics JSON is for inspecting Atelier itself."));
    assert!(diagnostics.contains("stable for local diagnostic tooling"));
    assert!(diagnostics.contains("must not appear in ordinary Agent Factory or"));
    assert!(diagnostics.contains("operator recipes for mission selection"));
    assert!(cli_surface.contains("diagnostics commands is an Atelier-maintenance interface"));
    assert!(cli_surface.contains("not an automation contract for selecting work"));
    assert!(
        validation.contains("Diagnostics JSON from commands such as `atelier diagnostics slow`")
    );
    assert!(validation.contains("not proof of"));
    assert!(validation.contains("ready work, blockers, validation results"));
}

#[test]
fn test_spec_representative_commands_match_signpost_surfaces() {
    let spec =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("SPEC.md")).unwrap();

    assert!(!spec.contains("atelier work start"));
    assert!(!spec.contains("atelier work finish"));
    assert!(!spec.contains("atelier workflow validate"));
    assert!(spec.contains("atelier start atelier-z1p8"));
    assert!(spec.contains("atelier issue close atelier-z1p8 --reason \"done\""));
    assert!(spec.contains("atelier issue note atelier-z1p8 \"handoff\""));
    assert!(spec.contains("atelier status"));
    assert!(spec.contains("atelier issue transition atelier-z1p8 --options"));
    assert!(spec.contains(
        "atelier evidence record --target issue/atelier-z1p8 --kind test --result pass -- <command>"
    ));
}

#[test]
fn test_man_lists_roles() {
    let dir = tempdir().unwrap();

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["man"]);
    assert!(success, "man failed: {stderr}");
    assert!(stdout.contains("Atelier Man"));
    assert!(stdout.contains("worker"));
    assert!(stdout.contains("reviewer"));
    assert!(stdout.contains("manager"));
    assert!(stdout.contains("admin"));
    assert!(stdout.contains("atelier man worker"));
}

#[test]
fn test_man_worker_guides_empty_checkout_without_repeating_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["man", "worker"]);
    assert!(success, "man worker failed: {stderr}");
    assert!(stdout.contains("Atelier Man: Worker"));
    assert!(stdout.contains("Current State"));
    assert!(stdout.contains("Active mission: none"));
    assert!(stdout.contains("Current work:   none"));
    assert!(stdout.contains("Most Relevant Commands"));
    assert!(stdout.contains("Normal Loop"));
    assert!(stdout.contains("Not Usually For This Role"));
    assert!(stdout.contains("atelier issue list --ready"));
    assert!(stdout.contains("atelier start <id>"));
    assert!(!stdout.contains("Atelier Status"));
    assert!(!stdout.contains("Generic"));
    assert!(!stdout.contains("etc."));
}

#[test]
fn test_man_manager_names_active_mission() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Man mission"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Man mission");
    let mission_id = mission_id.as_str();
    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "start", mission_id]);
    assert!(success, "mission start failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["man", "manager"]);
    assert!(success, "man manager failed: {stderr}");
    assert!(stdout.contains("Atelier Man: Manager"));
    assert!(stdout.contains(&format!("Active mission: {mission_id} - Man mission")));
    assert!(stdout.contains("atelier mission status"));
    assert!(stdout.contains("atelier mission add-work <mission-id> <issue-id>"));
}

#[test]
fn test_man_worker_names_active_work() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Man work"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Man work");
    let issue_id = issue_id.as_str();
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "man work setup");
    let (success, _, stderr) = run_atelier(dir.path(), &["start", issue_id]);
    assert!(success, "start failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["man", "worker"]);
    assert!(success, "man worker failed: {stderr}");
    assert!(stdout.contains("Current work:   1 issue"));
    assert!(stdout.contains(&format!("{issue_id} - Man work [in_progress]")));
    assert!(stdout.contains("atelier status - Review the current in-progress work set."));
    assert!(stdout.contains("atelier evidence record --target issue/<id>"));
}

#[test]
fn test_man_rejects_unknown_roles_and_admin_degrades_before_init() {
    let dir = tempdir().unwrap();

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["man", "bogus"]);
    assert!(!success, "unknown role should fail");
    assert!(stdout.is_empty());
    assert!(stderr.contains("unknown man role 'bogus'"));
    assert!(stderr.contains("Valid roles: worker, reviewer, manager, admin"));

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["man", "worker"]);
    assert!(!success, "worker guide should require tracker state");
    assert!(stdout.is_empty());
    assert!(stderr.contains("Not an Atelier repository"));
    assert!(stderr.contains("atelier man admin"));

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["man", "admin"]);
    assert!(success, "admin guide should degrade before init: {stderr}");
    assert!(stdout.contains("Atelier Man: Admin"));
    assert!(stdout.contains("Tracker: unavailable"));
    assert!(stdout.contains("Not an Atelier repository"));
    assert!(stdout.contains("atelier init"));
}
