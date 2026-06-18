use super::*;

// ==================== Init Tests ====================

#[test]
fn test_init_creates_atelier_directory() {
    let dir = tempdir().unwrap();
    let (success, stdout, _) = run_atelier(dir.path(), &["init"]);

    assert!(success);
    assert!(stdout.contains("Created") || stdout.contains("initialized"));
    assert!(stdout.contains("Created"));
    assert!(stdout.contains(".atelier/workflow.yaml"));
    assert!(stdout.contains("atelier lint"));
    assert!(stdout.contains("atelier issue create \"Task\""));
    assert!(stdout.contains("atelier man admin"));
    let lint_pos = stdout.find("atelier lint").unwrap();
    let issue_pos = stdout.find("atelier issue create \"Task\"").unwrap();
    assert!(
        lint_pos < issue_pos,
        "fresh init must verify setup before suggesting issue creation:\n{stdout}"
    );
    assert!(dir.path().join(".atelier").exists());
    assert!(dir.path().join(".atelier/runtime/state.db").exists());
    assert!(dir.path().join(".atelier").join("config.toml").exists());
    assert!(dir.path().join(".atelier").join("workflow.yaml").exists());
    assert!(!dir.path().join(".atelier").join("rules").exists());
    assert!(!dir.path().join(".atelier").join("rules.local").exists());
    assert!(!dir
        .path()
        .join(".atelier")
        .join("hook-config.json")
        .exists());
    assert!(!dir.path().join(".claude").exists());
    assert!(!dir.path().join(".mcp.json").exists());
}

#[test]
fn test_init_twice_is_idempotent() {
    let dir = tempdir().unwrap();

    run_atelier(dir.path(), &["init"]);
    let (success, stdout, _) = run_atelier(dir.path(), &["init"]);

    assert!(success);
    assert!(stdout.contains("Atelier initialized successfully"));
    assert!(dir.path().join(".atelier/runtime/state.db").exists());
    assert!(!dir.path().join(".atelier").join("rules").exists());
    assert!(!dir.path().join(".claude").exists());
}

#[test]
fn test_init_help_documents_import_beads_flag() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["init", "--help"]);
    assert!(success, "init help failed: {stderr}");

    assert!(stdout.contains("--import-beads"));
    assert!(stdout.contains(".beads/issues.manual.jsonl"));
}

#[test]
fn test_init_import_beads_requires_explicit_flag() {
    let dir = tempdir().unwrap();
    let beads_dir = dir.path().join(".beads");
    std::fs::create_dir_all(&beads_dir).unwrap();
    std::fs::write(
        beads_dir.join("issues.manual.jsonl"),
        include_str!("../fixtures/beads/issues.manual.jsonl"),
    )
    .unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["init"]);
    assert!(success, "init without import flag failed: {stderr}");
    assert!(stdout.contains("Detected Beads migration input"));
    assert!(stdout.contains("atelier init --import-beads"));
    assert!(!dir.path().join(".atelier/issues/atelier-0001.md").exists());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["init", "--import-beads"]);
    assert!(success, "init import-beads failed: {stderr}");
    assert!(stdout.contains("Imported Beads backup from"));
    assert!(stdout.contains(".beads/issues.manual.jsonl"));
    assert!(stdout.contains("source records: 3"));
    assert!(stdout.contains("imported issues: 3"));
    assert!(dir.path().join(".atelier/issues/atelier-0001.md").exists());
}

#[test]
fn test_integrations_command_is_removed() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    assert!(!dir.path().join(".claude").exists());
    assert!(!dir.path().join(".mcp.json").exists());
    assert!(!dir.path().join(".atelier").join("rules").exists());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["integrations", "claude", "install"]);
    assert!(
        !success,
        "removed integrations command unexpectedly succeeded"
    );
    assert!(stdout.is_empty(), "{stdout}");
    assert!(
        stderr.contains("unrecognized subcommand 'integrations'"),
        "{stderr}"
    );
    assert!(!stderr.contains("was removed"), "{stderr}");
    assert!(!stderr.contains("external assistant hooks"), "{stderr}");
    assert!(!stderr.contains("Claude"), "{stderr}");

    assert!(!dir.path().join(".claude").exists());
    assert!(!dir.path().join(".mcp.json").exists());
    assert!(!dir.path().join(".atelier/hook-config.json").exists());
    assert!(!dir.path().join(".atelier/rules").exists());
}

#[test]
fn test_doctor_human_separates_projection_and_runtime_state_health() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    run_atelier(dir.path(), &["issue", "create", "Health check"]);
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["doctor"]);
    assert!(success, "doctor failed: {stderr}");
    assert!(stdout.contains("Install health:"));
    assert!(stdout.contains("ignored_runtime_paths: ok"));
    assert!(stdout.contains("Projection rebuild:"));
    assert!(stdout.contains("rebuild_ready: ok"));
    assert!(stdout.contains("projection_fresh: ok"));
    assert!(stdout.contains("Cache health:"));
    assert!(stdout.contains("projection_metadata: ok"));
    assert!(stdout.contains("Projection database:"));
    assert!(stdout.contains("database: ok"));
    assert!(stdout.contains("diagnostics:"));
}

#[test]
fn test_doctor_distinguishes_missing_runtime_projection_database() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Missing projection db"]);
    assert!(success, "issue create failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();

    let (success, stdout, stderr) = run_atelier(dir.path(), &["doctor"]);
    assert!(success, "doctor failed: {stderr}");
    assert!(stdout.contains("Projection rebuild:"));
    assert!(stdout.contains("projection_fresh: not ok"));
    assert!(stdout.contains("Projection database:"));
    assert!(stdout.contains("database: missing"));
    assert!(stdout.contains("projection_metadata: stale"));
}

#[test]
fn test_doctor_help_documents_fix_boundary() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["doctor", "--help"]);
    assert!(success, "doctor help failed: {stderr}");
    assert!(stdout.contains("--fix"));
    assert!(stdout.contains("Repair ignored local runtime/cache/projection state"));
    assert!(stdout.contains("never edits tracked canonical records"));
}

#[test]
fn test_doctor_fix_repairs_missing_and_stale_local_projection_state() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Doctor fix projection"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    std::fs::remove_file(dir.path().join(".atelier/runtime/state.db")).unwrap();
    let (success, stdout, stderr) = run_atelier(dir.path(), &["doctor", "--fix"]);
    assert!(success, "doctor --fix failed for missing db: {stderr}");
    assert!(stdout.contains("Repair:"));
    assert!(stdout.contains("local_projection: repaired"));
    assert!(stdout.contains("canonical_records: unchanged"));
    assert!(stdout.contains("projection_fresh: ok"));
    assert!(stdout.contains("database: ok"));

    edit_canonical_issue(dir.path(), &issue_id, |markdown| {
        markdown.replace("Doctor fix projection", "Doctor fix projection repaired")
    });
    let (success, stdout, stderr) = run_atelier(dir.path(), &["doctor", "--fix"]);
    assert!(
        success,
        "doctor --fix failed for stale projection: {stderr}"
    );
    assert!(stdout.contains("local_projection: repaired"));
    assert!(stdout.contains("projection_fresh: ok"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success, "issue show failed after doctor --fix: {stderr}");
    assert!(stdout.contains("Doctor fix projection repaired"));
}

#[test]
fn test_doctor_fix_refuses_to_modify_malformed_canonical_records() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Doctor fix boundary"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);
    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");

    corrupt_issue_title_yaml(dir.path(), &issue_id, "Doctor fix boundary");
    let malformed = read_canonical_record(dir.path(), "issues", &issue_id);
    let (success, stdout, stderr) = run_atelier(dir.path(), &["doctor", "--fix"]);
    assert!(
        !success,
        "doctor --fix must fail on malformed canonical Markdown"
    );
    assert!(
        stdout.is_empty(),
        "doctor --fix should not print repair success"
    );
    assert!(
        stderr.contains("doctor --fix refused to edit tracked `.atelier/` canonical records")
            && stderr.contains("atelier lint")
            && stderr.contains("Invalid YAML front matter")
            && stderr.contains(&format!(".atelier/issues/{issue_id}.md")),
        "unexpected doctor --fix refusal: {stderr}"
    );
    assert_eq!(
        read_canonical_record(dir.path(), "issues", &issue_id),
        malformed,
        "doctor --fix must not rewrite malformed tracked canonical Markdown"
    );
}

#[test]
fn test_doctor_reports_runtime_health_without_becoming_canonical_lint() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let body = "## Description\n\nDoctor boundary body.\n\n## Outcome\n\nDoctor continues reporting runtime health when canonical Markdown is malformed.\n\n## Evidence\n\n- `atelier doctor` reports runtime health while `atelier lint` reports invalid YAML.";
    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Doctor runtime boundary",
            "--description",
            body,
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_ref(dir.path(), 1);

    corrupt_issue_title_yaml(dir.path(), &issue_id, "Doctor runtime boundary");

    let (lint_success, lintstdout, lint_stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(
        !lint_success,
        "lint must reject malformed canonical Markdown, stdout: {lintstdout}"
    );
    let lint_transcript = format!("{lintstdout}\n{lint_stderr}");
    assert!(
        lint_transcript.contains("Canonical tracker Markdown is invalid")
            && lint_transcript.contains("Invalid YAML front matter"),
        "unexpected lint error: {lint_transcript}"
    );

    let (doctor_success, doctorstdout, doctor_stderr) = run_atelier(dir.path(), &["doctor"]);
    assert!(
        doctor_success,
        "doctor should continue reporting health: {doctor_stderr}"
    );
    assert!(doctorstdout.contains("Projection rebuild:"));
    assert!(doctorstdout.contains("rebuild_ready: not ok"));
    assert!(doctorstdout.contains("Projection database:"));
    assert!(doctorstdout.contains("database: ok"));
}

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
        "init", "man", "status", "start", "issue", "mission", "bundle", "evidence", "history",
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
        "atelier session list --active",
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
        "abandon",
        "integrations",
        "repair",
        "timer",
        "milestone",
        "daemon",
        "cpitd",
        "usage",
        "agent",
        "locks",
        "sync",
        "work",
        "workflow",
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
fn test_root_active_pointer_cleanup_commands_are_removed() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["repair", "--help"]);
    assert!(!success, "repair should be removed:\n{stdout}");
    assert!(
        stderr.contains("unrecognized subcommand 'repair'"),
        "{stderr}"
    );

    let (success, stdout, stderr) = run_atelier_raw(
        dir.path(),
        &["abandon", "atelier-test", "--reason", "handoff"],
    );
    assert!(!success, "abandon should be removed:\n{stdout}");
    assert!(
        stderr.contains("unrecognized subcommand 'abandon'"),
        "{stderr}"
    );
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
    assert!(
        stderr.contains("unrecognized subcommand 'note'"),
        "{stderr}"
    );
    assert!(!stderr.contains("was removed"), "{stderr}");
    assert!(
        !stderr.contains("atelier issue note atelier-missing"),
        "{stderr}"
    );
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
    assert!(
        stderr.contains("unrecognized subcommand 'link'"),
        "{stderr}"
    );
    assert!(!stderr.contains("was removed"), "{stderr}");
    assert!(!stderr.contains("atelier mission add-work"), "{stderr}");
    assert!(!stderr.contains("atelier issue block"), "{stderr}");
    assert!(!stderr.contains("atelier evidence attach"), "{stderr}");
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
    assert!(stdout
        .contains("atelier evidence record --target issue/<id> --kind validation \"summary\""));
    assert!(stdout.contains("atelier evidence record --target issue/<id> --kind test -- <command>"));
    assert!(stdout.contains("Use `evidence attach` only when you need to reuse"));
}

#[test]
fn test_agent_factory_guidance_avoids_raw_workflow_validate_commands() {
    let guidance = std::fs::read_to_string(workspace_root().join("AGENTFACTORY.md")).unwrap();
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
fn test_mission_status_help_exposes_verbose_terminal_detail() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["mission", "status", "--help"]);
    assert!(success, "mission status help failed: {stderr}");

    assert!(stdout.contains("--verbose"));
    assert!(stdout.contains("Show verbose validator detail in the status summary"));
    assert!(!stdout.contains("--closeout"));
    assert!(!stdout.contains("closeout audit"));
}

#[test]
fn test_mission_help_exposes_close_with_reason() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["mission", "--help"]);
    assert!(success, "mission help failed: {stderr}");

    assert!(stdout.contains("close"));
    assert!(stdout.contains("Close a mission after terminal checks pass"));
    assert!(!stdout.contains("audit"));
    assert!(!stdout.contains("closeout"));

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["mission", "close", "--help"]);
    assert!(success, "mission close help failed: {stderr}");
    assert!(stdout.contains("--reason <REASON>"));
    assert!(stdout.contains("Mission close reason recorded in the mission terminal notes"));
    assert!(!stdout.contains("closeout"));
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
    assert!(stdout.contains("Check runtime health (tracker records are current): atelier doctor"));
    assert!(!stdout.contains("workflow validate"));
    assert!(!stdout.contains("issue next"));
    assert!(stdout.contains("Active sessions: none"));
    assert!(!stdout.contains("session start"));

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

    for issue_id in [ready_id, blocked_id, blocker_id] {
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
    assert!(stdout.contains("Evidence Status"));
    assert!(stdout
        .contains("Attached Proof: missing - 2 issue(s) without validating evidence; 0 attached"));
    assert!(stdout.contains(&format!("  Missing: {blocker_id}")));
    assert!(stdout.contains(&format!("  Missing: {ready_id}")));
    assert!(
        stdout.contains("atelier evidence record --target issue/<id> --kind validation \"...\"")
    );
    assert!(stdout.contains("atelier evidence attach <evidence-id> issue <issue-id>"));
    assert!(stdout.contains("Active Mission"));
    assert!(stdout.contains(&format!("{mission_id} - Status focus")));
    assert!(stdout.contains("Health:   blocked"));
    assert!(stdout.contains("Work:     ready 2, blocked 1, done 0, backlog 0"));
    assert!(stdout.contains("Ready In Active Mission"));
    assert!(stdout.contains(ready_id));
    assert!(stdout.contains(blocker_id));
    assert!(stdout.contains(&format!(
        "ready {blocker_id} - Focus blocker | no open blockers; mission-linked root; proof missing"
    )));
    assert!(stdout.contains(&format!(
        "ready {ready_id} - Ready focus | no open blockers; mission-linked root; proof missing"
    )));
    assert!(stdout.contains("Blocked In Active Mission"));
    assert!(stdout.contains(&format!(
        "blocked {blocked_id} - Blocked focus | 1 blocker; details: atelier issue blocked {blocked_id}"
    )));
    assert!(
        stdout
            .find(&format!("ready {blocker_id} - Focus blocker"))
            .unwrap()
            < stdout
                .find(&format!("blocked {blocked_id} - Blocked focus"))
                .unwrap(),
        "visible blocker should appear before blocked dependent work:\n{stdout}"
    );
    assert!(stdout.contains("Immediate Blockers"));
    assert!(stdout.contains(blocker_id));
    assert!(stdout.contains("Recent Activity"));
    assert!(stdout.contains(ready_id));
    assert!(stdout.contains("Added note"));
    assert!(stdout.contains(&format!(
        "Inspect active mission health ({mission_id}): atelier mission status {mission_id}"
    )));
    assert!(stdout.contains(&format!(
        "Start selectable active-mission work (2 selectable issue(s)): atelier start {blocker_id}"
    )));
    assert!(
        !stdout.contains("workflow validate"),
        "normal status next actions must not route to raw workflow validators:\n{stdout}"
    );
}

#[test]
fn test_root_status_guides_current_work_to_transition_and_worktree_status() {
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
    assert!(stdout.contains("Current work:  1 issue(s)"));
    assert!(stdout.contains(&format!("  active {issue_id} - Active item")));
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
        "Inspect current work transitions ({issue_id}): atelier issue transition {issue_id} --options"
    )));
    assert!(stdout.contains(
        "Inspect worktree context if checkout state is unclear: atelier worktree status"
    ));
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
    assert!(stdout.contains("Evidence Status"));
    assert!(stdout.contains("Attached Proof: irrelevant - no current or ready work"));
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
    let docs =
        std::fs::read_to_string(workspace_root().join("docs/product/workflow-configuration.md"))
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
    let manifest = workspace_root();
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
    assert!(cli_surface.contains("stable for diagnostic tooling"));
    assert!(cli_surface.contains("not an automation contract for selecting work"));
    assert!(
        validation.contains("Diagnostics JSON from commands such as `atelier diagnostics slow`")
    );
    assert!(validation.contains("not proof of"));
    assert!(validation.contains("ready work, blockers, validation results"));
}

#[test]
fn test_spec_representative_commands_match_signpost_surfaces() {
    let spec = std::fs::read_to_string(workspace_root().join("SPEC.md")).unwrap();

    assert!(!spec.contains("atelier work start"));
    assert!(!spec.contains("atelier work finish"));
    assert!(!spec.contains("atelier workflow validate"));
    assert!(spec.contains("atelier start atelier-z1p8"));
    assert!(spec.contains("atelier issue close atelier-z1p8 --reason \"done\""));
    assert!(!spec.contains("atelier abandon atelier-z1p8 --reason \"handoff\""));
    assert!(spec.contains("atelier status"));
    assert!(spec.contains("atelier issue transition atelier-z1p8 --options"));
    assert!(spec
        .contains("atelier evidence record --target issue/atelier-z1p8 --kind test -- <command>"));
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
fn test_man_worker_names_current_work() {
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
    assert!(stdout.contains("Current work:   1 issue(s)"));
    assert!(stdout.contains(&format!("{issue_id} - Man work")));
    assert!(stdout.contains("atelier status - Review the checkout's current-work set."));
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

#[test]
fn test_issue_ready_list_uses_current_workflow_commands() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(dir.path(), &["issue", "create", "Next item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Next item");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success, "issue ready list failed: {stderr}");
    assert!(stdout.contains("Issue Queue"));
    assert!(stdout.contains(&issue_id));
    assert!(stdout.contains("Next item"));
    assert!(
        !stdout.contains("session work"),
        "ready list must not suggest removed session workflow:\n{stdout}"
    );
}

#[test]
fn test_issue_transition_options_and_successful_execution_follow_workflow_policy() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Root workflow item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Root workflow item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "tracker setup");
    let git_before = git_status_short(dir.path());

    let (success, transition_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(success, "transition failed: {stderr}");
    assert!(transition_out.contains("Issue Transitions"));
    assert!(
        transition_out.contains("start [allowed]"),
        "{transition_out}"
    );
    assert!(
        transition_out.contains("block [allowed]"),
        "{transition_out}"
    );
    assert!(transition_out.contains(&format!("atelier issue transition {issue_id} start")));
    let git_after = git_status_short(dir.path());
    assert_eq!(
        git_before, git_after,
        "--options should not dirty the worktree:\nbefore:\n{git_before}\nafter:\n{git_after}"
    );

    let (success, start_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "transition start failed: {stderr}");
    assert!(
        start_out.contains("Applied transition start"),
        "{start_out}"
    );
    assert!(start_out.contains("To:       in_progress"), "{start_out}");

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(
        issue_text.contains("status: \"in_progress\""),
        "{issue_text}"
    );

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_applied",
        &[
            "Applied transition start",
            "transition: \"start\"",
            "to: \"in_progress\"",
        ],
    );
}

#[test]
fn test_issue_transition_options_do_not_write_but_blocked_transitions_do() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Options read-only item",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Options read-only item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "options read-only baseline");

    let git_before = git_status_short(dir.path());
    let (success, options_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(options_out.contains("Issue Transitions"), "{options_out}");
    let git_after = git_status_short(dir.path());
    assert_eq!(
        git_before, git_after,
        "--options should leave the git worktree unchanged:\nbefore:\n{git_before}\nafter:\n{git_after}"
    );

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "start failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_validation"],
    );
    assert!(
        !success,
        "request_validation should fail without a completed review"
    );
    assert!(stdout.contains("Blockers"), "{stdout}");
    assert!(stderr.contains("review_ready"), "{stderr}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_blocked",
        &[
            "Blocked transition request_validation from in_progress",
            "transition: \"request_validation\"",
            "reason: \"validator review_ready failed:",
        ],
    );
}

#[test]
fn test_root_start_applies_workflow_transition_and_records_active_work() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Root start item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Root start item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "workflow-ready start item");

    let (success, start_out, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "root start failed: {stderr}");
    assert!(
        start_out.contains("Applied transition start"),
        "{start_out}"
    );
    assert!(start_out.contains("To:       in_progress"), "{start_out}");
    assert!(
        start_out.contains(&format!("Started work on {issue_id}")),
        "{start_out}"
    );

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(
        issue_text.contains("status: \"in_progress\""),
        "{issue_text}"
    );

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(status_out.contains("Current work:  1 issue(s)"));
    assert!(status_out.contains(&format!("{issue_id} - Root start item")));

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_applied",
        &[
            "Applied transition start",
            "transition: \"start\"",
            "to: \"in_progress\"",
        ],
    );
    assert_activity_contains(
        &activities,
        "work_started",
        &["branch: ", "worktree_path: "],
    );
    let transition_index = activities
        .iter()
        .position(|text| text.contains("event_type: \"transition_applied\""))
        .expect("missing transition_applied activity");
    let work_started_index = activities
        .iter()
        .position(|text| text.contains("event_type: \"work_started\""))
        .expect("missing work_started activity");
    assert!(
        transition_index < work_started_index,
        "workflow transition should be recorded before work association:\n{}",
        activities.join("\n--- activity ---\n")
    );
}

#[test]
fn test_root_start_allows_multiple_current_work_issues_in_same_worktree() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Shared epic", "--issue-type", "epic"],
    );
    assert!(success, "epic issue create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Shared epic");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Active item", "--parent", &epic_id],
    );
    assert!(success, "active issue create failed: {stderr}");
    let active_id = issue_id_by_title(dir.path(), "Active item");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Next item", "--parent", &epic_id],
    );
    assert!(success, "next issue create failed: {stderr}");
    let next_id = issue_id_by_title(dir.path(), "Next item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "two startable items");

    let (success, active_out, stderr) = run_atelier(dir.path(), &["start", &active_id]);
    assert!(success, "initial start failed: {stderr}");
    let session_id = active_out
        .lines()
        .find_map(|line| line.strip_prefix("Session: "))
        .map(str::trim)
        .expect("initial start should create a session")
        .to_string();
    commit_all(dir.path(), "active item started");

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["start", &next_id, "--reuse-session", &session_id],
    );
    assert!(success, "second current work issue should start: {stderr}");
    assert!(
        stdout.contains(&format!("Started work on {next_id}")),
        "{stdout}"
    );

    let next_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &next_id)).unwrap();
    assert!(
        next_text.contains("status: \"in_progress\""),
        "started issue must transition to in_progress:\n{next_text}"
    );
    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(
        status_out.contains("Current work:  2 issue(s)"),
        "{status_out}"
    );
    assert!(status_out.contains(&format!("{active_id} - Active item")));
    assert!(status_out.contains(&format!("{next_id} - Next item")));
}

#[test]
fn test_root_start_same_issue_reports_already_started_work() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Restarted item"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Restarted item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "restartable item");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &issue_id, "--no-session"]);
    assert!(success, "first start failed: {stderr}");
    let (success, restart_out, stderr) =
        run_atelier(dir.path(), &["start", &issue_id, "--no-session"]);
    assert!(!success, "second start should be guarded:\n{restart_out}");
    assert!(
        stderr.contains("Transition 'start' is not available from status 'in_progress'"),
        "{stderr}"
    );
}

#[test]
fn test_removed_abandon_rejects_and_starting_another_issue_is_allowed() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "First item"]);
    assert!(success, "first issue create failed: {stderr}");
    let first_id = issue_id_by_title(dir.path(), "First item");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Second item"]);
    assert!(success, "second issue create failed: {stderr}");
    let second_id = issue_id_by_title(dir.path(), "Second item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "switchable items");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &first_id, "--no-session"]);
    assert!(success, "first start failed: {stderr}");
    let (success, abandon_out, stderr) =
        run_atelier(dir.path(), &["abandon", &first_id, "--reason", "switching"]);
    assert!(!success, "abandon should be removed:\n{abandon_out}");
    assert!(
        stderr.contains("unrecognized subcommand 'abandon'"),
        "{stderr}"
    );

    let (success, second_out, stderr) =
        run_atelier(dir.path(), &["start", &second_id, "--no-session"]);
    assert!(success, "second start should not require abandon: {stderr}");
    assert!(
        second_out.contains(&format!("Started work on {second_id}")),
        "{second_out}"
    );
    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(
        status_out.contains("Current work:  2 issue(s)"),
        "{status_out}"
    );
    assert!(status_out.contains(&format!("{first_id} - First item")));
    assert!(status_out.contains(&format!("{second_id} - Second item")));
}

#[test]
fn test_separate_worktrees_can_have_different_active_issues() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Parallel focus"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Parallel focus");
    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "start", mission_id.as_str()]);
    assert!(success, "mission start failed: {stderr}");

    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Root work"]);
    assert!(success, "root issue create failed: {stderr}");
    let root_id = issue_id_by_title(dir.path(), "Root work");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Worktree work"]);
    assert!(success, "worktree issue create failed: {stderr}");
    let worktree_id = issue_id_by_title(dir.path(), "Worktree work");
    for issue_id in [&root_id, &worktree_id] {
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &["mission", "add-work", mission_id.as_str(), issue_id],
        );
        assert!(success, "mission add work failed for {issue_id}: {stderr}");
    }
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "parallel worktree items");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &root_id]);
    assert!(success, "root start failed: {stderr}");
    let worktree_path = dir.path().join(".atelier-worktrees").join(&worktree_id);
    let worktree_arg = worktree_path.to_string_lossy().to_string();
    let (success, worktree_out, stderr) = run_atelier(
        dir.path(),
        &["worktree", "for", &worktree_id, "--path", &worktree_arg],
    );
    assert!(success, "worktree for failed: {stderr}");
    assert!(worktree_out.contains(&worktree_arg));

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(status_out.contains(&format!("{root_id} - Root work")));
    assert!(status_out.contains(&format!("{worktree_id} - Worktree work")));

    let (success, mission_out, stderr) =
        run_atelier(dir.path(), &["mission", "status", mission_id.as_str()]);
    assert!(success, "mission status failed: {stderr}");
    let active_work_section = mission_out
        .split("Active Work")
        .nth(1)
        .expect("active work section missing")
        .split("Next Commands")
        .next()
        .expect("next commands section missing");
    assert!(active_work_section.contains(&root_id), "{mission_out}");
    assert!(active_work_section.contains(&worktree_id), "{mission_out}");

    let (success, worktree_status, stderr) = run_atelier(dir.path(), &["worktree", "status"]);
    assert!(success, "worktree status failed: {stderr}");
    assert!(worktree_status.contains(&worktree_arg), "{worktree_status}");
    assert!(
        worktree_status.contains(&format!("{worktree_id} [in_progress]")),
        "{worktree_status}"
    );
}

#[test]
fn test_root_start_reports_workflow_validator_failure() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Validator-gated start"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Validator-gated start");
    migrate_default_issue_workflow(dir.path());
    let policy_path = dir.path().join(".atelier").join("workflow.yaml");
    let policy = std::fs::read_to_string(&policy_path).unwrap();
    std::fs::write(
        &policy_path,
        policy.replace(
            "      start:\n        from: [todo, blocked]\n        to: in_progress\n",
            "      start:\n        from: [todo, blocked]\n        to: in_progress\n        validators: [proof_attached]\n",
        ),
    )
    .unwrap();
    commit_all(dir.path(), "validator-gated start policy");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(!success, "root start should fail when validators block it");
    assert!(stdout.contains("Blockers"), "{stdout}");
    assert!(stderr.contains("proof_attached"), "{stderr}");

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(issue_text.contains("status: \"todo\""), "{issue_text}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_blocked",
        &[
            "Blocked transition start from todo",
            "transition: \"start\"",
            "reason: \"validator proof_attached failed:",
        ],
    );
    assert!(
        !activities
            .iter()
            .any(|text| text.contains("event_type: \"work_started\"")),
        "blocked start should not record work_started:\n{}",
        activities.join("\n--- activity ---\n")
    );
}

#[test]
fn test_issue_transition_blocked_attempt_records_activity_without_evidence() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Validator blocked item",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Validator blocked item");
    migrate_default_issue_workflow(dir.path());

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "start failed: {stderr}");

    let evidence_dir = dir.path().join(".atelier").join("evidence");
    let evidence_before = std::fs::read_dir(&evidence_dir)
        .unwrap()
        .filter(|entry| {
            entry
                .as_ref()
                .ok()
                .and_then(|entry| entry.path().extension().map(|ext| ext == "md"))
                .unwrap_or(false)
        })
        .count();

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_validation"],
    );
    assert!(
        !success,
        "request_validation should fail without a completed review"
    );
    assert!(stdout.contains("Blockers"), "{stdout}");
    assert!(stderr.contains("review_ready"), "{stderr}");
    assert!(stderr.contains("blocked"), "{stderr}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_blocked",
        &[
            "Blocked transition request_validation from in_progress",
            "transition: \"request_validation\"",
            "reason: \"validator review_ready failed:",
        ],
    );

    let evidence_after = std::fs::read_dir(&evidence_dir)
        .unwrap()
        .filter(|entry| {
            entry
                .as_ref()
                .ok()
                .and_then(|entry| entry.path().extension().map(|ext| ext == "md"))
                .unwrap_or(false)
        })
        .count();
    assert_eq!(
        evidence_before, evidence_after,
        "blocked transition created evidence"
    );
}

#[test]
fn test_issue_transition_close_reports_blockers_and_records_blocked_activity() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Blocked close item",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Blocked close item");
    migrate_default_issue_workflow(dir.path());

    for args in [
        vec!["issue", "transition", &issue_id, "start"],
        vec!["issue", "transition", &issue_id, "request_review"],
        vec!["issue", "transition", &issue_id, "request_validation"],
    ] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "transition {:?} failed: {stderr}", args);
    }

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "close"]);
    assert!(!success, "close should be blocked without reason and proof");
    assert!(stdout.contains("Blockers"), "{stdout}");
    assert!(
        stderr.contains("missing required field close_reason"),
        "{stderr}"
    );
    assert!(stderr.contains("proof_attached"), "{stderr}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_blocked",
        &[
            "Blocked transition close from validation",
            "transition: \"close\"",
            "reason: \"missing required field close_reason;",
        ],
    );
}

#[test]
fn test_issue_close_uses_terminal_transition_and_clears_active_work() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Closable workflow item",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Closable workflow item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "workflow-ready close item");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "start failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_review"],
    );
    assert!(success, "request_review failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_validation"],
    );
    assert!(success, "request_validation failed: {stderr}");
    attach_issue_pass_evidence(dir.path(), &issue_id);
    commit_all(dir.path(), "ready for close");

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &issue_id, "--reason", "done"],
    );
    assert!(success, "issue close failed: {stderr}");
    assert!(
        close_out.contains("Applied transition close"),
        "{close_out}"
    );
    assert!(close_out.contains("To:       done"), "{close_out}");

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(issue_text.contains("status: \"done\""), "{issue_text}");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(status_out.contains("Current work:  none"), "{status_out}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_applied",
        &[
            "Applied transition close",
            "transition: \"close\"",
            "to: \"done\"",
        ],
    );
}

#[test]
fn test_issue_close_requires_to_when_done_target_is_ambiguous_and_can_archive() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Archivable workflow item",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Archivable workflow item");
    migrate_default_issue_workflow(dir.path());
    let policy_path = dir.path().join(".atelier").join("workflow.yaml");
    let policy = std::fs::read_to_string(&policy_path).unwrap();
    std::fs::write(
        &policy_path,
        policy.replace(
            "      close:\n        from: [validation]\n        to: done\n        required_fields: [close_reason]\n        validators:\n          - proof_attached\n          - epic_child_proof\n          - blockers_clear\n          - lint_clear\n          - durable_current\n          - closeout_clean\n        guidance: [close_with_proof]\n",
            "      close:\n        from: [validation]\n        to: done\n        required_fields: [close_reason]\n        validators:\n          - proof_attached\n          - epic_child_proof\n          - blockers_clear\n          - lint_clear\n          - durable_current\n          - closeout_clean\n        guidance: [close_with_proof]\n      archive:\n        from: [validation]\n        to: archived\n        required_fields: [close_reason]\n        validators:\n          - proof_attached\n          - epic_child_proof\n          - blockers_clear\n          - lint_clear\n          - durable_current\n          - closeout_clean\n        guidance: [close_with_proof]\n",
        ),
    )
    .unwrap();
    commit_all(dir.path(), "archivable workflow policy");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "start failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_review"],
    );
    assert!(success, "request_review failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_validation"],
    );
    assert!(success, "request_validation failed: {stderr}");
    attach_issue_pass_evidence(dir.path(), &issue_id);
    commit_all(dir.path(), "ready for archive");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "close", &issue_id, "--reason", "needs archive"],
    );
    assert!(!success, "ambiguous close should require --to");
    assert!(
        stderr.contains("multiple terminal done targets"),
        "{stderr}"
    );
    assert!(stderr.contains("available: archived, done"), "{stderr}");

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Explicit archive workflow item",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "archive issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let archive_id = issue_id_by_title(dir.path(), "Explicit archive workflow item");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &archive_id, "--no-session"]);
    assert!(success, "archive start failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &archive_id, "request_review"],
    );
    assert!(success, "archive request_review failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &archive_id, "request_validation"],
    );
    assert!(success, "archive request_validation failed: {stderr}");
    attach_issue_pass_evidence(dir.path(), &archive_id);
    commit_all(dir.path(), "ready for explicit archive");

    let (success, archive_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "close",
            &archive_id,
            "--to",
            "archived",
            "--reason",
            "archived by policy",
        ],
    );
    assert!(success, "archived close failed: {stderr}");
    assert!(
        archive_out.contains("Applied transition archive"),
        "{archive_out}"
    );
    assert!(archive_out.contains("To:       archived"), "{archive_out}");

    let issue_text =
        std::fs::read_to_string(canonical_issue_path(dir.path(), &archive_id)).unwrap();
    assert!(issue_text.contains("status: \"archived\""), "{issue_text}");
}

#[test]
fn test_removed_abandon_preserves_issue_status_and_activity() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Abandonable workflow item"],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Abandonable workflow item");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "workflow-ready abandon item");

    let (success, _, stderr) = run_atelier(dir.path(), &["start", &issue_id]);
    assert!(success, "start failed: {stderr}");

    let (success, abandon_out, stderr) = run_atelier(
        dir.path(),
        &["abandon", &issue_id, "--reason", "paused for handoff"],
    );
    assert!(!success, "abandon should be removed:\n{abandon_out}");
    assert!(
        stderr.contains("unrecognized subcommand 'abandon'"),
        "{stderr}"
    );

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(
        issue_text.contains("status: \"in_progress\""),
        "{issue_text}"
    );

    let (success, status_out, stderr) = run_atelier(dir.path(), &["status"]);
    assert!(success, "status failed: {stderr}");
    assert!(
        status_out.contains("Current work:  1 issue(s)"),
        "{status_out}"
    );
    assert!(status_out.contains(&format!("{issue_id} - Abandonable workflow item")));

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert!(
        !activities
            .iter()
            .any(|activity| activity.contains("event_type: \"work_abandoned\"")),
        "removed abandon command must not record work_abandoned activity:\n{}",
        activities.join("\n--- activity ---\n")
    );
}

#[test]
fn test_issue_transition_rejects_unknown_transition_name() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Unknown transition item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Unknown transition item");
    migrate_default_issue_workflow(dir.path());

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "ship_it"]);
    assert!(!success, "unknown transition should fail");
    assert!(stderr.contains("Unknown transition 'ship_it'"), "{stderr}");
    assert!(
        stderr.contains("available from 'todo' are: block, start"),
        "{stderr}"
    );
}

#[test]
fn test_issue_transition_requires_workflow_policy_file() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Missing policy item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Missing policy item");

    std::fs::remove_file(dir.path().join(".atelier").join("workflow.yaml")).unwrap();

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(
        !success,
        "transition options should fail without workflow policy"
    );
    assert!(stderr.contains("workflow_config_missing"), "{stderr}");
}

#[test]
fn test_issue_transition_rejects_unmigrated_issue_status() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Unmigrated transition item"],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Unmigrated transition item");
    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let issue_text = std::fs::read_to_string(&issue_path).unwrap();
    std::fs::write(
        &issue_path,
        issue_text.replace("status: \"todo\"", "status: \"open\""),
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild failed: {stderr}");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(!success, "transition options should reject legacy status");
    assert!(stderr.contains("status 'open'"), "{stderr}");
    assert!(
        stderr.contains("not allowed by the workflow policy"),
        "{stderr}"
    );
}

#[test]
fn test_issue_transition_options_render_guidance_and_exact_command() {
    let dir = tempdir().unwrap();
    init_atelier_without_workflow(dir.path());
    migrate_default_issue_workflow(dir.path());

    let (success, issue_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Guided transition item",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Guided transition item");
    for args in [
        vec!["issue", "transition", &issue_id, "start"],
        vec!["issue", "transition", &issue_id, "request_review"],
        vec!["issue", "transition", &issue_id, "request_validation"],
    ] {
        let (success, stdout, stderr) = run_atelier(dir.path(), &args);
        assert!(success, "transition {:?} failed: {stderr}", args);
    }

    let (success, options_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--options"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(options_out.contains("close [blocked]"), "{options_out}");
    assert!(options_out.contains("Guidance"), "{options_out}");
    assert!(
        options_out.contains(&format!(
            "Closing {} requires attached evidence and no open blockers.",
            issue_id
        )),
        "{options_out}"
    );
    assert!(options_out.contains(&format!(
        "atelier issue transition {issue_id} close --reason \"...\""
    )));
}

#[test]
fn test_issue_help_uses_reduced_lifecycle_surface() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "--help"]);
    assert!(success, "issue help failed: {stderr}");
    for command in [
        "create",
        "list",
        "show",
        "transition",
        "update",
        "close",
        "block",
        "unblock",
        "blocked",
    ] {
        assert!(
            stdout
                .lines()
                .any(|line| line.trim_start().starts_with(command)),
            "missing reduced issue command {command}:\n{stdout}"
        );
    }
    for hidden in [
        "quick",
        "subissue",
        "reopen",
        "label",
        "unlabel",
        "close-all",
        "delete",
        "next",
        "tested",
    ] {
        assert!(
            !stdout
                .lines()
                .any(|line| line.trim_start().starts_with(hidden)),
            "folded or moved command {hidden} is still visible:\n{stdout}"
        );
    }

    let (success, update_help, stderr) = run_atelier(dir.path(), &["issue", "update", "--help"]);
    assert!(success, "issue update help failed: {stderr}");
    assert!(!update_help.contains("--claim"));
    assert!(!update_help.contains("--status"));
    assert!(!update_help.contains("--description"));
}

#[test]
fn test_non_lifecycle_issue_flows_use_explicit_homes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Source graph item"]);
    let source_id = issue_ref(dir.path(), 1);
    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Target graph item",
            "--parent",
            &source_id,
        ],
    );
    run_atelier(dir.path(), &["issue", "create", "Disposable item"]);
    let disposable_id = issue_ref(dir.path(), 3);

    let (success, search_out, stderr) = run_atelier(dir.path(), &["search", "Source"]);
    assert!(success, "search failed: {stderr}");
    assert!(search_out.contains("Source graph item"));

    let (success, impact_out, stderr) = run_atelier(dir.path(), &["graph", "impact", &source_id]);
    assert!(success, "graph impact failed: {stderr}");
    assert!(impact_out.contains("downstream impact"));
    assert!(impact_out.contains("Target graph item"));

    let (success, tree_out, stderr) = run_atelier(dir.path(), &["graph", "tree", "--compact"]);
    assert!(success, "graph tree failed: {stderr}");
    assert!(tree_out.contains("Compact Issue Hierarchy"));

    let (success, note_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "note",
            &source_id,
            "Explicit note body",
            "--kind",
            "observation",
        ],
    );
    assert!(success, "issue note failed: {stderr}");
    assert!(note_out.contains("Added note to issue"));
    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", &source_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(show_out.contains("Explicit note body"));

    let (success, delete_out, stderr) = run_atelier(
        dir.path(),
        &["maintenance", "delete", "issue", &disposable_id, "--force"],
    );
    assert!(success, "maintenance delete failed: {stderr}");
    assert!(delete_out.contains("Deleted issue"));
}

#[test]
fn test_graph_tree_orders_children_by_visible_blockers() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &["issue", "create", "Graph parent", "--issue-type", "epic"],
    );
    run_atelier(
        dir.path(),
        &["issue", "subissue", "1", "Implementation node"],
    );
    run_atelier(dir.path(), &["issue", "subissue", "1", "Contract node"]);
    let parent_id = issue_ref(dir.path(), 1);
    let implementation_id = issue_ref(dir.path(), 2);
    let contract_id = issue_ref(dir.path(), 3);
    run_atelier(
        dir.path(),
        &["issue", "block", &implementation_id, &contract_id],
    );

    let (success, full_out, stderr) = run_atelier(dir.path(), &["graph", "tree"]);
    assert!(success, "graph tree failed: {stderr}");
    assert!(full_out.contains("Legend: ready, blocked"));
    assert!(
        full_out.find("Contract node").unwrap() < full_out.find("Implementation node").unwrap(),
        "{full_out}"
    );
    assert!(
        full_out.contains(&format!(
            "[blocked] #{implementation_id} medium - Implementation node (1 blocker; details: atelier issue blocked {implementation_id})"
        )),
        "{full_out}"
    );
    assert!(!full_out.contains("todo/todo"), "{full_out}");

    let (success, compact_out, stderr) = run_atelier(dir.path(), &["graph", "tree", "--compact"]);
    assert!(success, "compact graph tree failed: {stderr}");
    assert!(compact_out.contains("Compact Issue Hierarchy"));
    assert!(
        compact_out.find("Contract node").unwrap()
            < compact_out.find("Implementation node").unwrap(),
        "{compact_out}"
    );
    assert!(compact_out.contains(&parent_id), "{compact_out}");
}

#[test]
fn test_graph_commands_include_mission_linked_issue_hierarchy() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(dir.path(), &["mission", "create", "Graph Mission"]);
    assert!(success, "mission create failed: {stderr}");
    let mission_id = record_id_by_title(dir.path(), "missions", "Graph Mission");

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Mission Epic", "--issue-type", "epic"],
    );
    assert!(success, "issue create failed: {stderr}");
    let epic_id = issue_ref(dir.path(), 1);

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Mission Child", "--parent", &epic_id],
    );
    assert!(success, "child issue create failed: {stderr}");

    let (success, _, stderr) =
        run_atelier(dir.path(), &["mission", "add-work", &mission_id, &epic_id]);
    assert!(success, "mission add-work failed: {stderr}");

    let (success, impact_out, stderr) = run_atelier(dir.path(), &["graph", "impact", &mission_id]);
    assert!(success, "graph impact failed: {stderr}");
    assert!(impact_out.contains("Mission"));
    assert!(impact_out.contains("Graph Mission"));
    assert!(impact_out.contains("Mission Epic"));
    assert!(impact_out.contains("Mission Child"));

    let (success, tree_out, stderr) = run_atelier(dir.path(), &["graph", "tree", "--compact"]);
    assert!(success, "graph tree failed: {stderr}");
    assert!(tree_out.contains("Compact Issue Hierarchy"));
    assert!(tree_out.contains("[mission ready]"));
    assert!(tree_out.contains("Graph Mission"));
    assert!(tree_out.contains("Mission Epic"));
    assert!(tree_out.contains("Mission Child"));
}

#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_hidden_issue_helpers_do_not_emit_compatibility_guidance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Old surface source"]);
    run_atelier(dir.path(), &["issue", "create", "Old surface target"]);

    for args in [
        vec!["issue", "quick", "Old quick"],
        vec!["issue", "subissue", "1", "Old child"],
        vec!["issue", "search", "Old"],
        vec!["issue", "comment", "1", "compat note"],
        vec!["issue", "label", "1", "old-label"],
        vec!["issue", "unlabel", "1", "old-label"],
        vec!["issue", "relate", "1", "2"],
        vec!["issue", "unrelate", "1", "2"],
        vec!["issue", "related", "1"],
        vec!["issue", "impact", "1"],
        vec!["issue", "next"],
        vec!["issue", "tree"],
        vec!["issue", "tested"],
        vec!["issue", "delete", "1", "--force"],
        vec!["issue", "close-all"],
    ] {
        let (success, _, stderr) = run_atelier(dir.path(), &args);
        assert!(!success, "{args:?} unexpectedly succeeded");
        assert!(
            stderr.contains("unrecognized subcommand"),
            "{args:?} should be removed, got:\n{stderr}"
        );
        assert!(
            !stderr.contains("was removed"),
            "{args:?} should not emit compatibility guidance:\n{stderr}"
        );
    }
}

#[test]
fn test_generic_link_rejection_is_plain_unknown_command() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "Target issue"]);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["link", "add", "mission", "atelier-none", "issue", "1"],
    );
    assert!(!success, "generic link command should be removed");
    assert!(
        stderr.contains("unrecognized subcommand 'link'"),
        "{stderr}"
    );
    assert!(!stderr.contains("was removed"), "{stderr}");
    assert!(!stderr.contains("atelier mission add-work"), "{stderr}");
    assert!(!stderr.contains("atelier issue block"), "{stderr}");
    assert!(!stderr.contains("atelier evidence attach"), "{stderr}");
}

#[test]
fn test_explicit_homes_reject_non_issue_targets_until_supported() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "maintenance",
            "delete",
            "mission",
            "atelier-none",
            "--force",
        ],
    );
    assert!(
        !success,
        "maintenance delete unexpectedly accepted a mission target"
    );
    assert!(stderr.contains("supports issue records only"));
}

#[test]
fn test_removed_aliases_fail_as_unknown_commands() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for args in [
        vec!["show"],
        vec!["ready"],
        vec!["sync"],
        vec!["mission", "view"],
        vec!["work", "status"],
        vec!["work", "worktree"],
    ] {
        let (success, _, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(!success, "{args:?} unexpectedly succeeded");
        assert!(
            stderr.contains("unrecognized subcommand") || stderr.contains("unexpected argument"),
            "{args:?} did not fail as an unknown command:\n{stderr}"
        );
    }
}

#[test]
fn test_removed_commands_fail_without_compatibility_guidance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for args in [
        vec!["workflow", "check", "--json"],
        vec!["finish"],
        vec!["current-work"],
        vec!["issue", "new", "Replacement test"],
        vec!["work", "start", "atelier-z1p8"],
        vec!["archive", "add", "atelier-z1p8"],
        vec!["session", "status"],
        vec!["timer"],
    ] {
        let (success, stdout, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(!success, "{args:?} unexpectedly succeeded");
        assert!(
            stdout.is_empty(),
            "{args:?} should not execute a compatibility path:\n{stdout}"
        );
        assert!(
            stderr.contains("unrecognized subcommand")
                || stderr.contains("unexpected argument")
                || stderr.contains("Usage:"),
            "{args:?} did not fail through Clap:\n{stderr}"
        );
        assert!(
            !stderr.contains("was removed"),
            "{args:?} emitted compatibility guidance:\n{stderr}"
        );
    }
}
