---
created_at: "2026-06-20T21:58:53.505598305+00:00"
id: "atelier-mcgl"
evidence_type: "validation"
captured_at: "2026-06-20T21:58:45.151063891+00:00"
command: "bash -lc 'set -euo pipefail\ncargo fmt -- --check\ngit diff --check\ntarget/debug/atelier --help >/tmp/atelier-help.txt\nif target/debug/atelier worktree --help >/tmp/worktree-help.txt 2>/tmp/worktree-err.txt; then\n  exit 1\nfi\nif target/debug/atelier mission --help >/tmp/mission-help.txt 2>/tmp/mission-err.txt; then\n  exit 1\nfi\ncargo build -p atelier-cli\ncargo test -p atelier-cli --test cli_integration setup_guidance -- --nocapture\n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-hdff"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-hdff"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Removed command machinery epic validation"
updated_at: "2026-06-20T21:58:58.278065679+00:00"
---

## Summary

Removed command machinery epic validation

## Command

```console
bash -lc 'set -euo pipefail
cargo fmt -- --check
git diff --check
target/debug/atelier --help >/tmp/atelier-help.txt
if target/debug/atelier worktree --help >/tmp/worktree-help.txt 2>/tmp/worktree-err.txt; then
  exit 1
fi
if target/debug/atelier mission --help >/tmp/mission-help.txt 2>/tmp/mission-err.txt; then
  exit 1
fi
cargo build -p atelier-cli
cargo test -p atelier-cli --test cli_integration setup_guidance -- --nocapture
'
```

Exit status: 0

## Stdout

Bytes: 6331
Truncated: yes

```text

running 75 tests
test setup_guidance::test_agent_factory_guidance_avoids_raw_workflow_validate_commands ... ok
test setup_guidance::test_diagnostics_json_docs_define_local_operator_boundary ... ok
test setup_guidance::test_doctor_help_documents_fix_boundary ... ok
test setup_guidance::test_evidence_record_help_shows_issue_targeted_manual_and_command_flows ... ok
test setup_guidance::test_diagnostics_slow_handles_missing_telemetry_store ... ok
test setup_guidance::test_evidence_help_hides_predecessor_subcommands ... ok
test setup_guidance::test_init_help_documents_import_beads_flag ... ok
test setup_guidance::test_diagnostics_slow_summarizes_fixture_events ... ok
test setup_guidance::test_graph_command_is_removed ... ok
test setup_guidance::test_forgejo_roles_provision_write_config_flag_is_removed ... ok
test setup_guidance::test_diagnostics_help_scopes_json_as_advanced_local_only ... ok
test setup_guidance::test_generic_note_command_rejects_with_record_specific_guidance ... ok
test setup_guidance::test_generic_link_command_rejects_with_record_specific_guidance ... ok
test setup_guidance::test_command_telemetry_ignores_relative_diagnostics_dir ... ok
test setup_guidance::test_init_twice_is_idempotent ... ok
test setup_guidance::test_man_lists_roles ... ok
test setup_guidance::test_init_creates_atelier_directory ... ok
test setup_guidance::test_explicit_homes_reject_non_issue_targets_until_supported ... ok
test setup_guidance::test_command_telemetry_respects_opt_out_controls ... ok
test setup_guidance::test_integrations_command_is_removed ... ok
test setup_guidance::test_command_telemetry_records_failure_event ... ok
test setup_guidance::test_mission_create_help_names_generated_sections ... ok
test setup_guidance::test_command_telemetry_records_success_event ... ok
test setup_guidance::test_man_rejects_unknown_roles_and_admin_degrades_before_init ... ok
test setup_guidance::test_mission_help_exposes_close_with_reason ... ok
test setup_guidance::test_mission_status_help_exposes_verbose_terminal_detail ... ok
test setup_guidance::test_mission_namespace_is_removed ... ok
test setup_guidance::test_prune_apply_removes_only_expired_diagnostics_logs ... ok
test setup_guidance::test_prune_dry_run_reports_diagnostics_without_removing_logs ... ok
test setup_guidance::test_issue_help_uses_reduced_lifecycle_surface ... ok
test setup_guidance::test_init_import_beads_requires_explicit_flag ... ok
test setup_guidance::test_root_active_pointer_cleanup_commands_are_removed ... ok
test setup_guidance::test_man_worker_guides_empty_checkout_without_repeating_status ... ok
test setup_guidance::test_command_telemetry_omits_sensitive_arguments_by_default ... ok
test setup_guidance::test_generic_link_rejection_is_plain_unknown_command ... ok
test setup_guidance::test_issue_ready_list_uses_current_workflow_commands ... ok
test setup_guidance::test_doctor_human_separates_projection_and_runtime_state_health ... ok
test setup_guidance::test_doctor_fix_refuses_to_modify_malformed_canonical_records ... ok
test setup_guidance::test_doctor_reports_runtime_health_without_becoming_canonical_lint ... ok
test setup_guidance::test_removed_aliases_fail_as_unknown_commands ... ok
test setup_guidance::test_spec_representative_commands_match_signpost_surfaces ... ok
test setup_guidance::test_top_level_help_only_shows_core_commands ... ok
test setup_guidance::test_workflow_configuration_docs_describe_internal_diagnostics ... ok
test setup_guidance::test_removed_commands_fail_without_compatibility_guidance ... ok
test setup_guidance::test_workflow_help_is_scoped_as_advanced_internal_diagnostic ... ok
test setup_guidance::test_issue_transition_rejects_unknown_transition_name ... ok
test setup_guidance::test_doctor_distinguishes_missing_runtime_projection_database ... ok
test setup_guidance::test_issue_transition_rejects_unmigrated_issue_status ... ok
test setup_guidance::test_issue_transition_requires_workflow_policy_file ... ok
test setup_guidance::test_root_status_no_ready_work_suggests_valid_blocked_list ... ok
test setup_guidance::test_removed_issue_close_comma
```

## Stderr

Bytes: 370
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.31s
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.33s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
```

