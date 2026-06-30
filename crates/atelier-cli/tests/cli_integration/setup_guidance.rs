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
    assert!(stdout.contains("atelier check"));
    assert!(stdout.contains("atelier issue create \"Task\""));
    assert!(stdout.contains("atelier man admin"));
    let lint_pos = stdout.find("atelier check").unwrap();
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

    let config = std::fs::read_to_string(dir.path().join(".atelier/config.toml")).unwrap();
    assert!(config.contains("state_root = \".atelier\""));
    assert!(!config.contains("compatibility_state_root"));
    assert!(!config.contains("runtime_dir"));
    assert!(!config.contains("runtime_database"));
    assert!(!config.contains("cache_dir"));
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
fn test_doctor_reports_runtime_health_without_becoming_canonical_lint() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let body = "## Description\n\nDoctor boundary body.\n\n## Outcome\n\nDoctor continues reporting runtime health when canonical Markdown is malformed.\n\n## Evidence\n\n- `atelier doctor` reports runtime health while `atelier check` reports invalid YAML.";
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
fn test_prune_dry_run_reports_diagnostics_without_removing_logs() {
    let dir = tempdir().unwrap();
    let diagnostics_dir = dir.path().join("diagnostics");
    let old = chrono::Utc::now()
        .date_naive()
        .checked_sub_days(chrono::Days::new(45))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    let recent = chrono::Utc::now().format("%Y-%m-%d").to_string();

    write_diagnostics_event(
        &diagnostics_dir,
        &old,
        serde_json::json!({
            "schema": "atelier.command_event",
            "schema_version": 1,
            "event_id": "old",
            "command": "status",
            "started_at": format!("{old}T01:00:00.000Z"),
            "finished_at": format!("{old}T01:00:01.000Z"),
            "duration_ms": 1000,
            "result": "success"
        }),
    );
    write_diagnostics_event(
        &diagnostics_dir,
        &recent,
        serde_json::json!({
            "schema": "atelier.command_event",
            "schema_version": 1,
            "event_id": "recent",
            "command": "status",
            "started_at": format!("{recent}T01:00:00.000Z"),
            "finished_at": format!("{recent}T01:00:01.000Z"),
            "duration_ms": 1000,
            "result": "success"
        }),
    );

    let old_path = diagnostics_dir
        .join("commands")
        .join(format!("{old}.ndjson"));
    let recent_path = diagnostics_dir
        .join("commands")
        .join(format!("{recent}.ndjson"));

    let (success, stdout, stderr) = run_atelier_with_env(
        dir.path(),
        &["prune", "--retention-days", "30"],
        &[("ATELIER_DIAGNOSTICS_DIR", diagnostics_dir.to_str().unwrap())],
    );
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(stdout.contains("Mode: dry-run"), "{stdout}");
    assert!(stdout.contains("Diagnostics Logs"), "{stdout}");
    assert!(stdout.contains("eligible diagnostics-log"), "{stdout}");
    assert!(stdout.contains(&old), "{stdout}");
    assert!(stdout.contains("Deferred Cleanup Classes"), "{stdout}");
    assert!(stdout.contains("Next: atelier prune --apply"), "{stdout}");
    assert!(old_path.exists(), "dry-run removed old diagnostics log");
    assert!(
        recent_path.exists(),
        "dry-run removed recent diagnostics log"
    );
}

#[test]
fn test_prune_apply_removes_only_expired_diagnostics_logs() {
    let dir = tempdir().unwrap();
    let diagnostics_dir = dir.path().join("diagnostics");
    let old = chrono::Utc::now()
        .date_naive()
        .checked_sub_days(chrono::Days::new(45))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    let recent = chrono::Utc::now().format("%Y-%m-%d").to_string();

    write_diagnostics_event(
        &diagnostics_dir,
        &old,
        serde_json::json!({
            "schema": "atelier.command_event",
            "schema_version": 1,
            "event_id": "old",
            "command": "status",
            "started_at": format!("{old}T01:00:00.000Z"),
            "finished_at": format!("{old}T01:00:01.000Z"),
            "duration_ms": 1000,
            "result": "success"
        }),
    );
    write_diagnostics_event(
        &diagnostics_dir,
        &recent,
        serde_json::json!({
            "schema": "atelier.command_event",
            "schema_version": 1,
            "event_id": "recent",
            "command": "status",
            "started_at": format!("{recent}T01:00:00.000Z"),
            "finished_at": format!("{recent}T01:00:01.000Z"),
            "duration_ms": 1000,
            "result": "success"
        }),
    );

    let old_path = diagnostics_dir
        .join("commands")
        .join(format!("{old}.ndjson"));
    let recent_path = diagnostics_dir
        .join("commands")
        .join(format!("{recent}.ndjson"));

    let (success, stdout, stderr) = run_atelier_with_env(
        dir.path(),
        &["prune", "--apply", "--retention-days", "30"],
        &[("ATELIER_DIAGNOSTICS_DIR", diagnostics_dir.to_str().unwrap())],
    );
    assert!(success, "prune apply failed: {stderr}");
    assert!(stdout.contains("Mode: apply"), "{stdout}");
    assert!(stdout.contains("removed diagnostics-log"), "{stdout}");
    assert!(stdout.contains(&old), "{stdout}");
    assert!(
        !old_path.exists(),
        "apply left expired diagnostics log in place"
    );
    assert!(recent_path.exists(), "apply removed recent diagnostics log");
}

#[test]
fn test_prune_dry_run_reports_terminal_canonical_issue_without_removing_it() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Old terminal work"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 45);
    commit_all(dir.path(), "old terminal issue fixture");

    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(stdout.contains("Canonical Records"), "{stdout}");
    assert!(stdout.contains("eligible issue"), "{stdout}");
    assert!(stdout.contains(&issue_id), "{stdout}");
    assert!(
        stdout.contains(&format!("git log --all -- .atelier/issues/{issue_id}.md")),
        "{stdout}"
    );
    assert!(issue_path.exists(), "dry-run removed canonical issue");
}

#[test]
fn test_prune_uses_configured_canonical_retention_days_by_default() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    set_prune_canonical_retention_days(dir.path(), 7);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Configured retention target"],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 10);
    commit_all(dir.path(), "configured prune retention fixture");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune"]);
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(stdout.contains("Canonical Records"), "{stdout}");
    assert!(stdout.contains("Retention: 7 day(s)"), "{stdout}");
    assert!(stdout.contains("eligible issue"), "{stdout}");
    assert!(stdout.contains(&issue_id), "{stdout}");
}

#[test]
fn test_prune_retention_flag_overrides_project_config_for_canonical_records() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    set_prune_canonical_retention_days(dir.path(), 30);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Override retention target"],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 10);
    commit_all(dir.path(), "override prune retention fixture");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune"]);
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(stdout.contains("Canonical Records"), "{stdout}");
    assert!(stdout.contains("Retention: 30 day(s)"), "{stdout}");
    assert!(stdout.contains("Candidates: none"), "{stdout}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "7"]);
    assert!(success, "prune override dry-run failed: {stderr}");
    assert!(stdout.contains("Retention: 7 day(s)"), "{stdout}");
    assert!(stdout.contains("eligible issue"), "{stdout}");
    assert!(stdout.contains(&issue_id), "{stdout}");
}

#[test]
fn test_prune_apply_removes_terminal_canonical_issue_and_keeps_git_recovery() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Old terminal apply"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 45);
    commit_all(dir.path(), "old terminal issue fixture");

    let issue_path = canonical_issue_path(dir.path(), &issue_id);
    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["prune", "--apply", "--retention-days", "30"]);
    assert!(success, "prune apply failed: {stderr}");
    assert!(stdout.contains("removed issue"), "{stdout}");
    assert!(stdout.contains(&issue_id), "{stdout}");
    assert!(
        stdout.contains(&format!("git show <commit>:.atelier/issues/{issue_id}.md")),
        "{stdout}"
    );
    assert!(
        !issue_path.exists(),
        "apply left pruned issue in active tree"
    );

    let output = Command::new("git")
        .current_dir(dir.path())
        .args(["show", &format!("HEAD:.atelier/issues/{issue_id}.md")])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "Git history should recover pruned issue: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("Old terminal apply"),
        "Git recovery output should contain the old record"
    );
}

#[test]
fn test_prune_apply_refuses_canonical_cleanup_when_tracked_state_is_dirty() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Dirty prune target"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    make_issue_terminal_before_retention(dir.path(), &issue_id, 45);
    commit_all(dir.path(), "old terminal issue fixture");
    fs::write(dir.path().join("tracked.txt"), "dirty tracked file\n").unwrap();
    commit_all(dir.path(), "tracked file fixture");
    fs::write(
        dir.path().join("tracked.txt"),
        "dirty tracked file changed\n",
    )
    .unwrap();

    let (success, _stdout, stderr) =
        run_atelier(dir.path(), &["prune", "--apply", "--retention-days", "30"]);
    assert!(!success, "dirty canonical prune unexpectedly succeeded");
    assert!(
        stderr.contains("canonical prune requires a clean tracked checkout")
            && stderr.contains("git status --short --branch"),
        "{stderr}"
    );
    assert!(
        canonical_issue_path(dir.path(), &issue_id).exists(),
        "dirty apply removed canonical issue"
    );
}

#[test]
fn test_prune_protects_old_evidence_attached_to_retained_issue() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Active evidence owner"]);
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_ref(dir.path(), 1);
    let evidence_id = attach_pass_evidence(
        dir.path(),
        "issue",
        &issue_id,
        "old evidence remains required by active work",
    );
    make_record_before_retention(dir.path(), "evidence", &evidence_id, 45);
    let (success, _, stderr) = run_atelier(dir.path(), &["doctor", "--fix"]);
    assert!(
        success,
        "doctor --fix failed after evidence fixture edit: {stderr}"
    );

    let (success, stdout, stderr) = run_atelier(dir.path(), &["prune", "--retention-days", "30"]);
    assert!(success, "prune dry-run failed: {stderr}");
    assert!(stdout.contains("protected evidence-record"), "{stdout}");
    assert!(stdout.contains(&evidence_id), "{stdout}");
    assert!(stdout.contains("attached to retained issue"), "{stdout}");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["prune", "--apply", "--retention-days", "30"]);
    assert!(success, "prune apply failed: {stderr}");
    assert!(stdout.contains("protected evidence-record"), "{stdout}");
    assert!(
        canonical_record_path(dir.path(), "evidence", &evidence_id).exists(),
        "apply removed protected evidence"
    );
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
        "Planning:",
        "Records:",
        "Maintenance:",
        "Common commands:",
        "Options:",
    ] {
        assert!(stdout.contains(heading), "missing help heading {heading}");
    }

    for command in [
        "init", "man", "status", "work", "issue", "bundle", "evidence", "history", "check",
    ] {
        assert!(stdout.contains(command), "missing core command {command}");
    }
    assert!(!stdout.contains("Search issues, relationships, and activity"));
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
    for hidden in ["forgejo", "branch", "maintenance", "lint", "doctor"] {
        assert!(
            !stdout.lines().any(|line| {
                let command = line.trim_start();
                command == hidden || command.starts_with(&format!("{hidden} "))
            }),
            "root help should not expose hidden admin command {hidden}:\n{stdout}"
        );
    }

    for common in [
        "atelier man",
        "atelier man worker",
        "atelier man reviewer",
        "atelier man validator",
        "atelier man manager",
        "atelier man admin",
        "atelier work ready",
        "atelier work blocked",
        "atelier issue list",
        "atelier work mission <mission-id>",
        "atelier issue show <id>",
        "atelier issue create \"...\" --issue-type mission",
        "atelier issue show <mission-id>",
        "atelier bundle preview <file>",
        "atelier bundle apply <file> --yes",
        "atelier history",
        "atelier issue transition <issue-id> start",
        "atelier issue transition <issue-id>",
        "atelier issue transition <mission-id> request_publish",
        "atelier check",
        "atelier check <issue-id>",
        "atelier check --fix",
    ] {
        assert!(
            stdout.contains(common),
            "missing common command example {common}"
        );
    }
    let common_commands = stdout
        .split("Common commands:")
        .nth(1)
        .and_then(|section| section.split("Options:").next())
        .unwrap_or("");
    assert!(
        !common_commands.contains("atelier doctor"),
        "common commands should not teach doctor as routine work:\n{stdout}"
    );
    assert!(
        !common_commands.contains("atelier lint"),
        "common commands should not teach lint as routine work:\n{stdout}"
    );
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
        "workflow",
        "mission",
        "search",
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
fn test_obsolete_command_surfaces_are_removed_without_aliases() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    for args in [
        vec!["mission", "list"],
        vec!["search", "needle"],
        vec!["issue", "table"],
    ] {
        let (success, stdout, stderr) = run_atelier_raw(dir.path(), &args);
        assert!(!success, "{args:?} should be removed:\n{stdout}");
        assert!(
            stderr.contains("unrecognized subcommand"),
            "{args:?} should fail as an unrecognized command, got:\n{stderr}"
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
fn test_forgejo_roles_provision_write_config_flag_is_removed() {
    let dir = tempdir().unwrap();

    let (success, help_out, stderr) =
        run_atelier_raw(dir.path(), &["forgejo", "roles", "provision", "--help"]);
    assert!(success, "forgejo roles provision help failed: {stderr}");
    assert!(
        !help_out.contains("--write-config"),
        "removed write-config flag should not appear in help:\n{help_out}"
    );

    let (success, stdout, stderr) = run_atelier_raw(
        dir.path(),
        &["forgejo", "roles", "provision", "--write-config"],
    );
    assert!(
        !success,
        "removed write-config flag should be rejected:\n{stdout}"
    );
    assert!(
        stderr.contains("unexpected argument '--write-config'"),
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
    assert!(!stderr.contains("atelier issue link"), "{stderr}");
    assert!(!stderr.contains("atelier issue link"), "{stderr}");
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
    assert!(stdout.contains(
        "atelier evidence record --target issue/<id> --kind validation \"checked claim X; result pass\""
    ));
    assert!(stdout.contains("atelier evidence record --target issue/<id> --kind test -- <command>"));
    assert!(stdout.contains("Use `evidence attach` only when you need to reuse"));
}

#[test]
fn test_agent_factory_guidance_avoids_raw_workflow_validate_commands() {
    let guidance =
        std::fs::read_to_string(workspace_root().join(".agents/skills/agent-factory/SKILL.md"))
            .unwrap();
    assert!(guidance.contains("Assign exactly one subskill"));
    assert!(!guidance.contains("atelier workflow validate issue"));
    assert!(!guidance.contains("atelier workflow validate mission"));
    assert!(!guidance.contains("## Checks"));
    assert!(!guidance.contains("atelier worktree remove <issue-id>"));
    assert!(!guidance.contains("cargo nextest run --profile extended --run-ignored=only"));
}

#[test]
fn test_graph_command_is_removed() {
    let dir = tempdir().unwrap();

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["graph", "--help"]);
    assert!(!success, "graph help should fail after removal: {stdout}");
    assert!(
        stderr.contains("unrecognized subcommand 'graph'"),
        "{stderr}"
    );
    assert!(!stderr.contains("graph impact"), "{stderr}");
    assert!(!stderr.contains("graph tree"), "{stderr}");
}

#[test]
fn test_mission_create_help_names_generated_sections() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["issue", "create", "--help"]);
    assert!(success, "mission create help failed: {stderr}");

    assert!(stdout.contains("--issue-type <ISSUE_TYPE>"));
    assert!(!stdout.contains("Mission intent/body text; requires --issue-type mission"));
    assert!(!stdout.contains("Constraints section bullet"));
    assert!(!stdout.contains("Risks section bullet"));
    assert!(!stdout.contains("Validation section bullet"));
}

#[test]
fn test_mission_help_exposes_close_with_reason() {
    let dir = tempdir().unwrap();
    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["issue", "transition", "--help"]);
    assert!(success, "mission close help failed: {stderr}");
    assert!(stdout.contains("--reason <CLOSE_REASON>"));
    assert!(stdout.contains("Close reason used by transitions that require it"));
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
    assert!(stdout.contains("Tracker"));
    assert!(stdout.contains("Ready work:"));
    assert!(stdout.contains("Current work:"));
    assert!(stdout.contains("Current missions:"));
    assert!(stdout.contains("Next Actions"));
    assert!(stdout.contains("Choose ready work"));
    assert!(stdout.contains("Inspect selected work transitions"));
    assert!(!stdout.contains(
        "Start selected work (ready work exists): atelier issue transition <issue-id> start"
    ));
    assert!(!stdout.contains("atelier doctor"));
    assert!(!stdout.contains("workflow validate"));
    assert!(!stdout.contains("Evidence Status"));
    assert!(!stdout.contains("Attached Proof:"));
    assert!(!stdout.contains("Recent Activity"));
    assert!(!stdout.contains("no active mission focus"));
    let removed_ready_command = ["atelier", "issue", "next"].join(" ");
    assert!(!stdout.contains(&removed_ready_command));
    assert!(stdout.contains("Active roles:   none"));
    assert!(!stdout.contains("atelier session"));

    let (success, quiet, stderr) = run_atelier(dir.path(), &["--quiet", "status"]);
    assert!(success, "quiet status failed: {stderr}");
    assert!(quiet.contains("work="));
    assert!(quiet.contains("ready="));
    assert!(quiet.contains("tracker="));
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
    assert!(docs.contains("lint.none_blocking"));
    assert!(docs.contains("Projection freshness is an"));
    assert!(docs.contains("internal command-storage health concern"));
    assert!(!docs.contains("| `tracker.current` |"));
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
    assert!(cli_surface.contains("stable for diagnostic"));
    assert!(cli_surface.contains("not an automation contract for selecting work"));
    assert!(
        validation.contains("Diagnostics JSON from commands such as `atelier diagnostics slow`")
    );
    assert!(validation.contains("not proof of"));
    assert!(validation.contains("ready work, blockers, validation results"));
}

#[test]
fn test_product_intent_representative_commands_match_signpost_surfaces() {
    let product_intent =
        std::fs::read_to_string(workspace_root().join("PRODUCT_INTENT.md")).unwrap();

    assert!(!product_intent.contains("atelier work start"));
    assert!(!product_intent.contains("atelier work finish"));
    assert!(!product_intent.contains("atelier workflow validate"));
    assert!(product_intent.contains("atelier issue transition atelier-z1p8 start"));
    assert!(
        product_intent.contains("atelier issue transition atelier-z1p8 close --reason \"done\"")
    );
    assert!(!product_intent.contains("atelier abandon atelier-z1p8 --reason \"handoff\""));
    assert!(product_intent.contains("atelier status"));
    assert!(product_intent.contains("atelier issue transition atelier-z1p8"));
    assert!(product_intent
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
    assert!(stdout.contains("Current work:   none"));
    assert!(stdout.contains("Current work:   none"));
    assert!(stdout.contains("Most Relevant Commands"));
    assert!(stdout.contains("Normal Loop"));
    assert!(stdout.contains("Not Usually For This Role"));
    assert!(stdout.contains("atelier work ready"));
    assert!(stdout.contains("atelier issue transition <id>"));
    assert!(!stdout.contains("Atelier Status"));
    assert!(!stdout.contains("Generic"));
    assert!(!stdout.contains("etc."));
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
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "transition", issue_id, "start"]);
    assert!(success, "start failed: {stderr}");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["man", "worker"]);
    assert!(success, "man worker failed: {stderr}");
    assert!(stdout.contains("Current work:   1 issue(s)"));
    assert!(stdout.contains(&format!("{issue_id}")));
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
    assert!(stderr.contains("Valid roles: worker, reviewer, validator, manager, admin"));

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["man", "worker"]);
    assert!(!success, "worker guide should require tracker state");
    assert!(stdout.is_empty());
    assert!(stderr.contains("Not an Atelier repository"));
    assert!(stderr.contains("atelier man admin"));

    let (success, stdout, stderr) = run_atelier_raw(dir.path(), &["man", "admin"]);
    assert!(success, "admin guide should degrade before init: {stderr}");
    assert!(stdout.contains("Atelier Man: Admin"));
    assert!(stdout.contains("Tracker"));
    assert!(stdout.contains("Not an Atelier repository"));
    assert!(stdout.contains("atelier init"));
    assert!(stdout.contains("docs/product/workflow-configuration.md"));
    assert!(stdout.contains("atelier issue transition <id>"));
    assert!(stdout.contains("atelier prune --dry-run"));
}

#[test]
fn test_issue_ready_list_uses_current_workflow_commands() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, issue_out, stderr) = run_atelier(dir.path(), &["issue", "create", "Next item"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(issue_out.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Next item");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["work", "queue", "--ready"]);
    assert!(success, "issue ready list failed: {stderr}");
    assert!(stdout.contains("Work Queue"));
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
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--verbose"]);
    assert!(success, "transition failed: {stderr}");
    assert!(transition_out.contains("Issue Transitions"));
    assert!(
        transition_out.contains("start [allowed]"),
        "{transition_out}"
    );
    assert!(
        transition_out.contains("Requirements: satisfied"),
        "{transition_out}"
    );
    assert!(
        transition_out.contains("block [allowed]"),
        "{transition_out}"
    );
    assert!(
        transition_out.contains("Planned Actions"),
        "{transition_out}"
    );
    assert!(
        !transition_out.contains("Planned Effects"),
        "{transition_out}"
    );
    assert!(transition_out.contains("Commands"), "{transition_out}");
    assert!(transition_out.contains(&format!("atelier issue transition {issue_id} start")));
    let git_after = git_status_short(dir.path());
    assert_eq!(
        git_before, git_after,
        "no-argument transition should not dirty the worktree:\nbefore:\n{git_before}\nafter:\n{git_after}"
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
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--verbose"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(options_out.contains("Issue Transitions"), "{options_out}");
    let git_after = git_status_short(dir.path());
    assert_eq!(
        git_before, git_after,
        "no-argument transition should leave the git worktree unchanged:\nbefore:\n{git_before}\nafter:\n{git_after}"
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
    assert!(stderr.contains("review.complete"), "{stderr}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_blocked",
        &[
            "Blocked transition request_validation from in_progress",
            "transition: \"request_validation\"",
            "reason: \"validator review.complete failed:",
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

    let (success, start_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
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
    assert!(status_out.contains(&format!("{issue_id}")));

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

    let (success, active_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &active_id, "start"]);
    assert!(success, "initial start failed: {stderr}");
    assert!(!active_out.contains("Session:"), "{active_out}");
    commit_all(dir.path(), "active item started");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &next_id, "start"]);
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
    assert!(status_out.contains(&format!("{active_id}")));
    assert!(status_out.contains(&format!("{next_id}")));
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

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "first start failed: {stderr}");
    let (success, restart_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
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

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &first_id, "start"]);
    assert!(success, "first start failed: {stderr}");
    let (success, abandon_out, stderr) =
        run_atelier(dir.path(), &["abandon", &first_id, "--reason", "switching"]);
    assert!(!success, "abandon should be removed:\n{abandon_out}");
    assert!(
        stderr.contains("unrecognized subcommand 'abandon'"),
        "{stderr}"
    );

    let (success, second_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &second_id, "start"]);
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
    assert!(status_out.contains(&format!("{first_id}")));
    assert!(status_out.contains(&format!("{second_id}")));
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
        policy.replacen(
            "      start:\n        from: [todo, blocked]\n        to: in_progress\n",
            "      start:\n        from: [todo, blocked]\n        to: in_progress\n        validators: [evidence.attached]\n",
            1,
        ),
    )
    .unwrap();
    commit_all(dir.path(), "validator-gated start policy");

    let (success, options_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--verbose"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(options_out.contains("start [blocked]"), "{options_out}");
    assert!(
        options_out.contains("no validating evidence link found"),
        "{options_out}"
    );
    assert!(
        options_out.contains("Hint: record proof with `atelier evidence record --target issue/<id> --kind validation \"...\"`"),
        "{options_out}"
    );

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(!success, "root start should fail when validators block it");
    assert!(stdout.contains("Blockers"), "{stdout}");
    assert!(
        stdout.contains("no validating evidence link found"),
        "{stdout}"
    );
    assert!(
        stdout.contains("Hint: record proof with `atelier evidence record --target issue/<id> --kind validation \"...\"`"),
        "{stdout}"
    );
    assert!(stderr.contains("evidence.attached"), "{stderr}");

    let issue_text = std::fs::read_to_string(canonical_issue_path(dir.path(), &issue_id)).unwrap();
    assert!(issue_text.contains("status: \"todo\""), "{issue_text}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_blocked",
        &[
            "Blocked transition start from todo",
            "transition: \"start\"",
            "reason: \"validator evidence.attached failed:",
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
    assert!(stderr.contains("review.complete"), "{stderr}");
    assert!(stderr.contains("blocked"), "{stderr}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_blocked",
        &[
            "Blocked transition request_validation from in_progress",
            "transition: \"request_validation\"",
            "reason: \"validator review.complete failed:",
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
        if args[3] == "request_review" {
            complete_room_review(dir.path(), &issue_id);
        }
    }

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "close"]);
    assert!(!success, "close should be blocked without reason and proof");
    assert!(stdout.contains("Blockers"), "{stdout}");
    assert!(
        stdout.contains("expected at least 1 validating evidence record(s); found 0"),
        "{stdout}"
    );
    assert!(
        stdout.contains("Hint: record proof with `atelier evidence record --target issue/<id> --kind validation \"...\"`"),
        "{stdout}"
    );
    assert!(stderr.contains("evidence.attached"), "{stderr}");
    assert!(stderr.contains("git.worktree_clean"), "{stderr}");

    let activities = issue_activity_texts(dir.path(), &issue_id);
    assert_activity_contains(
        &activities,
        "transition_blocked",
        &[
            "Blocked transition close from validation",
            "transition: \"close\"",
            "reason: \"validator evidence.attached failed:",
            "validator git.worktree_clean failed:",
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

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
    assert!(success, "start failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_review"],
    );
    assert!(success, "request_review failed: {stderr}");
    complete_room_review(dir.path(), &issue_id);
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "transition", &issue_id, "request_validation"],
    );
    assert!(success, "request_validation failed: {stderr}");
    attach_issue_pass_evidence(dir.path(), &issue_id);
    commit_all(dir.path(), "ready for close");

    let (success, close_out, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "transition",
            &issue_id,
            "close",
            "--reason",
            "done",
        ],
    );
    assert!(success, "issue transition close failed: {stderr}");
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
fn test_removed_issue_close_command_rejects_to_and_reason_flags() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Removed close command item"],
    );
    assert!(success, "issue create failed: {stderr}");
    let issue_id = issue_id_by_title(dir.path(), "Removed close command item");

    let (success, _stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "close",
            &issue_id,
            "--to",
            "archived",
            "--reason",
            "archived by policy",
        ],
    );
    assert!(
        !success,
        "removed issue close command should reject --to usage"
    );
    assert!(
        stderr.contains("unrecognized subcommand 'close'"),
        "{stderr}"
    );
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

    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "start"]);
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
    assert!(status_out.contains(&format!("{issue_id}")));

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

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "transition", &issue_id]);
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
    assert!(
        !success,
        "rebuild should reject legacy status before transition handling"
    );
    assert!(stderr.contains("workflow_issue_status_invalid"), "{stderr}");
    assert!(stderr.contains("status 'open'"), "{stderr}");
    assert!(stderr.contains("not valid for workflow 'task'"), "{stderr}");
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
        if args[3] == "request_review" {
            complete_room_review(dir.path(), &issue_id);
        }
    }

    let (success, options_out, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &issue_id, "--verbose"]);
    assert!(success, "transition options failed: {stderr}");
    assert!(options_out.contains("close [blocked]"), "{options_out}");
    assert!(options_out.contains("  Decision: blocked"), "{options_out}");
    assert!(options_out.contains("Description"), "{options_out}");
    assert!(
        options_out.contains(
            "Closing requires attached evidence, complete child proof, review merge, and a clean worktree."
        ),
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
        "show",
        "transition",
        "update",
        "note",
        "link",
        "unlink",
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
        "close",
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
    assert!(update_help.contains("--status <STATUS>"));
    assert!(!update_help.contains("--description"));
}

#[test]
fn test_non_lifecycle_issue_flows_use_explicit_homes() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Source graph item",
            "--issue-type",
            "epic",
        ],
    );
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

    let (success, impact_out, stderr) = run_atelier(dir.path(), &["issue", "show", &source_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(impact_out.contains("Impact"));
    assert!(impact_out.contains("Target graph item"));

    let (success, status_out, stderr) = run_atelier(dir.path(), &["issue", "show", &source_id]);
    assert!(success, "issue show failed: {stderr}");
    assert!(status_out.contains("Source graph item"));
    assert!(status_out.contains("Target graph item"));

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
fn test_issue_status_includes_linked_issue_hierarchy() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Relationship Objective",
            "--issue-type",
            "epic",
        ],
    );
    assert!(success, "objective create failed: {stderr}");
    let objective_id = issue_id_by_title(dir.path(), "Relationship Objective");
    let objective_id = objective_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Linked Epic", "--issue-type", "epic"],
    );
    assert!(success, "linked epic create failed: {stderr}");
    let epic_id = issue_id_by_title(dir.path(), "Linked Epic");
    let epic_id = epic_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "create", "Linked Child", "--parent", epic_id],
    );
    assert!(success, "child issue create failed: {stderr}");
    let child_id = issue_id_by_title(dir.path(), "Linked Child");
    let child_id = child_id.as_str();

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "link", objective_id, epic_id, "--role", "advances"],
    );
    assert!(success, "issue link failed: {stderr}");

    let (success, status_out, stderr) = run_atelier(dir.path(), &["issue", "show", objective_id]);
    assert!(success, "issue status failed: {stderr}");
    assert!(status_out.contains("Objective Rollup"));
    assert!(status_out.contains("Relationship Objective"));
    assert!(status_out.contains("Relationship Objective"));
}

#[test]
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
        vec!["issue", "close", "1", "--reason", "done"],
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
    assert!(!stderr.contains("atelier issue link"), "{stderr}");
    assert!(!stderr.contains("atelier issue link"), "{stderr}");
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
        vec!["worktree"],
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
