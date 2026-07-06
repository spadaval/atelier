---
created_at: "2026-07-06T20:28:31.278117637+00:00"
id: "atelier-0n1b"
evidence_type: "test"
captured_at: "2026-07-06T20:28:21.524713507+00:00"
command: "cargo nextest run -p atelier-cli --test cli_integration"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-u7wi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u7wi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-cli --test cli_integration"
updated_at: "2026-07-06T20:28:35.068573129+00:00"
---

## Summary

cargo nextest run -p atelier-cli --test cli_integration

## Command

```console
cargo nextest run -p atelier-cli --test cli_integration
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 35718
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/mska-xa9s/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.64s
────────────
 Nextest run ID 71ff2178-adc4-4bd4-8a05-99d5826ff118 with nextest profile: default
    Starting 269 tests across 1 binary
        PASS [   0.142s] (  1/269) atelier-cli::cli_integration issues::test_bundle_preview_rejects_missing_client_ref
        PASS [   0.145s] (  2/269) atelier-cli::cli_integration issues::test_bundle_preview_rejects_plan_and_milestone_resources
        PASS [   0.141s] (  3/269) atelier-cli::cli_integration issues::test_issue_create_help_is_markdown_first
        PASS [   0.159s] (  4/269) atelier-cli::cli_integration issues::test_bundle_preview_rejects_duplicate_client_refs
        PASS [   0.160s] (  5/269) atelier-cli::cli_integration issues::test_create_issue_rejects_work_flag
        PASS [   0.168s] (  6/269) atelier-cli::cli_integration issues::test_create_issue_with_description_is_rejected
        PASS [   0.153s] (  7/269) atelier-cli::cli_integration issues::test_issue_create_mission_type_requires_workflow_policy_declaration
        PASS [   0.194s] (  8/269) atelier-cli::cli_integration issues::test_bundle_preview_rejects_status_outside_workflow_policy
        PASS [   0.222s] (  9/269) atelier-cli::cli_integration issues::test_create_issue
        PASS [   0.153s] ( 10/269) atelier-cli::cli_integration issues::test_issue_list_ready_rejects_closed_status
        PASS [   0.353s] ( 11/269) atelier-cli::cli_integration issues::test_create_issue_with_priority
        PASS [   0.370s] ( 12/269) atelier-cli::cli_integration issues::test_history_empty_states_and_invalid_limit
        PASS [   0.403s] ( 13/269) atelier-cli::cli_integration issues::test_history_repo_wide_supports_filters_bounded_output_and_drill_downs
        PASS [   0.420s] ( 14/269) atelier-cli::cli_integration issues::test_issue_create_mission_type_uses_declared_workflow_policy
        PASS [   0.425s] ( 15/269) atelier-cli::cli_integration issues::test_issue_list_blocked_replaces_blocked_helper
        PASS [   0.431s] ( 16/269) atelier-cli::cli_integration issues::test_issue_create_rejects_invalid_hierarchy_shapes
        PASS [   0.465s] ( 17/269) atelier-cli::cli_integration issues::test_bundle_apply_accepts_configured_custom_issue_type
        PASS [   0.472s] ( 18/269) atelier-cli::cli_integration issues::test_first_class_detail_views_read_payloads_from_record_store
        PASS [   0.478s] ( 19/269) atelier-cli::cli_integration issues::test_add_label
        PASS [   0.488s] ( 20/269) atelier-cli::cli_integration issues::test_history_issue_scope_defaults_single_issue_and_can_include_descendants
        PASS [   0.508s] ( 21/269) atelier-cli::cli_integration issues::test_block_issue
        PASS [   0.157s] ( 22/269) atelier-cli::cli_integration issues::test_issue_ready_command_removed
        PASS [   0.369s] ( 23/269) atelier-cli::cli_integration issues::test_issue_list_orders_visible_blockers_before_blocked_rows
        PASS [   0.515s] ( 24/269) atelier-cli::cli_integration issues::test_issue_commands_accept_partial_issue_key
        PASS [   0.540s] ( 25/269) atelier-cli::cli_integration issues::test_add_comment
        PASS [   0.542s] ( 26/269) atelier-cli::cli_integration issues::test_issue_create_is_durable_without_manual_export
        PASS [   0.543s] ( 27/269) atelier-cli::cli_integration issues::test_issue_create_scaffold_edit_lint_show_flow
        PASS [   0.430s] ( 28/269) atelier-cli::cli_integration issues::test_issue_list_ready_excludes_blocked_and_quiet_matches_human_order
        PASS [   0.614s] ( 29/269) atelier-cli::cli_integration issues::test_configured_custom_issue_link_is_context_only
        PASS [   0.151s] ( 30/269) atelier-cli::cli_integration issues::test_issue_type_help_uses_workflow_policy_wording
        PASS [   0.698s] ( 31/269) atelier-cli::cli_integration issues::test_create_subissue
        PASS [   0.553s] ( 32/269) atelier-cli::cli_integration issues::test_issue_list_marks_external_ep
```

