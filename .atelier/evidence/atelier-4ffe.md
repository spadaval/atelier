---
created_at: "2026-07-06T17:51:52.999951894+00:00"
id: "atelier-4ffe"
evidence_type: "test"
captured_at: "2026-07-06T17:51:46.312857265+00:00"
command: "cargo nextest run -p atelier-cli setup_guidance::test_"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-vqdm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vqdm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-cli setup_guidance::test_"
updated_at: "2026-07-06T17:51:57.352382563+00:00"
---

## Summary

cargo nextest run -p atelier-cli setup_guidance::test_

## Command

```console
cargo nextest run -p atelier-cli setup_guidance::test_
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 10544
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-eqq6/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.74s
────────────
 Nextest run ID b090837f-9d4c-4251-8cf8-133ea7b2ff7e with nextest profile: default
    Starting 75 tests across 4 binaries (369 tests skipped)
        PASS [   0.012s] ( 1/75) atelier-cli::cli_integration setup_guidance::test_agent_factory_guidance_avoids_raw_workflow_validate_commands
        PASS [   0.019s] ( 2/75) atelier-cli::cli_integration setup_guidance::test_evidence_record_help_shows_issue_targeted_manual_and_command_flows
        PASS [   0.025s] ( 3/75) atelier-cli::cli_integration setup_guidance::test_diagnostics_json_docs_define_local_operator_boundary
        PASS [   0.024s] ( 4/75) atelier-cli::cli_integration setup_guidance::test_diagnostics_slow_summarizes_fixture_events
        PASS [   0.035s] ( 5/75) atelier-cli::cli_integration setup_guidance::test_doctor_help_documents_fix_boundary
        PASS [   0.037s] ( 6/75) atelier-cli::cli_integration setup_guidance::test_graph_command_is_removed
        PASS [   0.045s] ( 7/75) atelier-cli::cli_integration setup_guidance::test_init_help_documents_import_beads_flag
        PASS [   0.033s] ( 8/75) atelier-cli::cli_integration setup_guidance::test_diagnostics_slow_handles_missing_telemetry_store
        PASS [   0.055s] ( 9/75) atelier-cli::cli_integration setup_guidance::test_forgejo_roles_provision_write_config_flag_is_removed
        PASS [   0.057s] (10/75) atelier-cli::cli_integration setup_guidance::test_diagnostics_help_scopes_json_as_advanced_local_only
        PASS [   0.062s] (11/75) atelier-cli::cli_integration setup_guidance::test_hidden_diagnostic_help_routes_normal_health_to_check
        PASS [   0.070s] (12/75) atelier-cli::cli_integration setup_guidance::test_evidence_help_hides_predecessor_subcommands
        PASS [   0.025s] (13/75) atelier-cli::cli_integration setup_guidance::test_man_lists_roles
        PASS [   0.044s] (14/75) atelier-cli::cli_integration setup_guidance::test_man_rejects_unknown_roles_and_admin_degrades_before_init
        PASS [   0.261s] (15/75) atelier-cli::cli_integration setup_guidance::test_generic_note_command_rejects_with_record_specific_guidance
        PASS [   0.268s] (16/75) atelier-cli::cli_integration setup_guidance::test_integrations_command_is_removed
        PASS [   0.270s] (17/75) atelier-cli::cli_integration setup_guidance::test_command_telemetry_records_failure_event
        PASS [   0.259s] (18/75) atelier-cli::cli_integration setup_guidance::test_generic_link_command_rejects_with_record_specific_guidance
        PASS [   0.275s] (19/75) atelier-cli::cli_integration setup_guidance::test_init_twice_is_idempotent
        PASS [   0.024s] (20/75) atelier-cli::cli_integration setup_guidance::test_mission_help_exposes_close_with_reason
        PASS [   0.028s] (21/75) atelier-cli::cli_integration setup_guidance::test_mission_create_help_names_generated_sections
        PASS [   0.300s] (22/75) atelier-cli::cli_integration setup_guidance::test_init_creates_atelier_directory
        PASS [   0.307s] (23/75) atelier-cli::cli_integration setup_guidance::test_command_telemetry_records_success_event
        PASS [   0.294s] (24/75) atelier-cli::cli_integration setup_guidance::test_issue_help_uses_reduced_lifecycle_surface
        PASS [   0.014s] (25/75) atelier-cli::cli_integration setup_guidance::test_product_intent_representative_commands_match_signpost_surfaces
        PASS [   0.317s] (26/75) atelier-cli::cli_integration setup_guidance::test_command_telemetry_ignores_relative_diagnostics_dir
        PASS [   0.323s] (27/75) atelier-cli::cli_integration setup_guidance::test_command_telemetry_respects_opt_out_controls
        PASS [   0.023s] (28/75) atelier-cli::cli_integration setup_guidance::test_prune_dry_run_reports_diagnostics_without_removing_logs
        PASS [   0.029s] (29/75) atelier-cli::cli_integration setup_guidance::test_prune_apply_removes_only_expired_diagnostics_logs
        PASS [   0.340s]
```

