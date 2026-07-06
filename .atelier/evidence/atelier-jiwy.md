---
created_at: "2026-07-06T18:13:42.524644411+00:00"
id: "atelier-jiwy"
evidence_type: "test"
captured_at: "2026-07-06T18:13:39.420162268+00:00"
command: "sh -c 'tmp_home=$(mktemp -d); HOME=\"$tmp_home\" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run -p atelier-cli -E \"test(test_removed_maintenance_delete_is_unknown) or test(test_forgejo_role_setup_is_hidden_from_normal_guidance_but_callable_for_recovery) or test(test_branch_recovery_is_hidden_from_routine_work_but_callable_explicitly) or test(test_hidden_diagnostic_help_routes_normal_health_to_check) or test(test_init_help_documents_import_beads_flag) or test(test_diagnostics_help_scopes_json_as_advanced_local_only) or test(test_agent_factory_guidance_avoids_raw_workflow_validate_commands)\"'"
exit_status: "0"
agent_identity: "independent-validator"
target:
  kind: "issue"
  id: "atelier-eqq6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-eqq6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "sh -c 'tmp_home=$(mktemp -d); HOME=\"$tmp_home\" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run -p atelier-cli -E \"test(test_removed_maintenance_delete_is_unknown) or test(test_forgejo_role_setup_is_hidden_from_normal_guidance_but_callable_for_recovery) or test(test_branch_recovery_is_hidden_from_routine_work_but_callable_explicitly) or test(test_hidden_diagnostic_help_routes_normal_health_to_check) or test(test_init_help_documents_import_beads_flag) or test(test_diagnostics_help_scopes_json_as_advanced_local_only) or test(test_agent_factory_guidance_avoids_raw_workflow_validate_commands)\"'"
updated_at: "2026-07-06T18:13:46.926573488+00:00"
---

## Summary

sh -c 'tmp_home=$(mktemp -d); HOME="$tmp_home" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run -p atelier-cli -E "test(test_removed_maintenance_delete_is_unknown) or test(test_forgejo_role_setup_is_hidden_from_normal_guidance_but_callable_for_recovery) or test(test_branch_recovery_is_hidden_from_routine_work_but_callable_explicitly) or test(test_hidden_diagnostic_help_routes_normal_health_to_check) or test(test_init_help_documents_import_beads_flag) or test(test_diagnostics_help_scopes_json_as_advanced_local_only) or test(test_agent_factory_guidance_avoids_raw_workflow_validate_commands)"'

## Command

```console
sh -c 'tmp_home=$(mktemp -d); HOME="$tmp_home" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run -p atelier-cli -E "test(test_removed_maintenance_delete_is_unknown) or test(test_forgejo_role_setup_is_hidden_from_normal_guidance_but_callable_for_recovery) or test(test_branch_recovery_is_hidden_from_routine_work_but_callable_explicitly) or test(test_hidden_diagnostic_help_routes_normal_health_to_check) or test(test_init_help_documents_import_beads_flag) or test(test_diagnostics_help_scopes_json_as_advanced_local_only) or test(test_agent_factory_guidance_avoids_raw_workflow_validate_commands)"'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1392
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-eqq6/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.46s
────────────
 Nextest run ID b09d81ab-c161-438f-a01e-152bfa8e9a90 with nextest profile: default
    Starting 7 tests across 4 binaries (437 tests skipped)
        PASS [   0.009s] (1/7) atelier-cli::cli_integration setup_guidance::test_agent_factory_guidance_avoids_raw_workflow_validate_commands
        PASS [   0.017s] (2/7) atelier-cli::cli_integration setup_guidance::test_init_help_documents_import_beads_flag
        PASS [   0.028s] (3/7) atelier-cli::cli_integration setup_guidance::test_diagnostics_help_scopes_json_as_advanced_local_only
        PASS [   0.035s] (4/7) atelier-cli::cli_integration setup_guidance::test_hidden_diagnostic_help_routes_normal_health_to_check
        PASS [   0.135s] (5/7) atelier-cli::cli_integration setup_guidance::test_removed_maintenance_delete_is_unknown
        PASS [   0.237s] (6/7) atelier-cli::cli_integration setup_guidance::test_forgejo_role_setup_is_hidden_from_normal_guidance_but_callable_for_recovery
        PASS [   0.339s] (7/7) atelier-cli::cli_integration setup_guidance::test_branch_recovery_is_hidden_from_routine_work_but_callable_explicitly
────────────
     Summary [   0.341s] 7 tests run: 7 passed, 437 skipped
```
