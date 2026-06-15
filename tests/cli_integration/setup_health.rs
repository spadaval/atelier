use super::support::*;

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
    assert!(
        stderr.contains("`atelier integrations` was removed"),
        "{stderr}"
    );
    assert!(
        stderr.contains("external assistant hooks are not an Atelier product feature"),
        "{stderr}"
    );
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
    assert!(stdout.contains("Runtime state:"));
    assert!(stdout.contains("database: ok"));
    assert!(stdout.contains("local_tables: ok"));
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
    assert!(stdout.contains("Runtime state:"));
    assert!(stdout.contains("database: missing (runtime projection artifact)"));
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

    let (lint_success, lint_stdout, lint_stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(
        !lint_success,
        "lint must reject malformed canonical Markdown, stdout: {lint_stdout}"
    );
    let lint_transcript = format!("{lint_stdout}\n{lint_stderr}");
    assert!(
        lint_transcript.contains("Canonical tracker Markdown is invalid")
            && lint_transcript.contains("Invalid YAML front matter"),
        "unexpected lint error: {lint_transcript}"
    );

    let (doctor_success, doctor_stdout, doctor_stderr) = run_atelier(dir.path(), &["doctor"]);
    assert!(
        doctor_success,
        "doctor should continue reporting runtime health: {doctor_stderr}"
    );
    assert!(doctor_stdout.contains("Projection rebuild:"));
    assert!(doctor_stdout.contains("rebuild_ready: not ok"));
    assert!(doctor_stdout.contains("Runtime state:"));
    assert!(doctor_stdout.contains("database: ok"));
    assert!(doctor_stdout.contains("local_tables: ok"));
}
