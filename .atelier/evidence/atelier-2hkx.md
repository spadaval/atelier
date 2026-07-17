---
created_at: "2026-07-17T00:06:42.648829263+00:00"
id: "atelier-2hkx"
evidence_type: "test"
captured_at: "2026-07-17T00:06:14.719428190+00:00"
command: "env CARGO_TARGET_DIR=/root/atelier-worktrees/atelier-p2wk/target cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-h3wg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h3wg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "env CARGO_TARGET_DIR=/root/atelier-worktrees/atelier-p2wk/target cargo nextest run"
updated_at: "2026-07-17T00:06:42.651420531+00:00"
---

## Summary

env CARGO_TARGET_DIR=/root/atelier-worktrees/atelier-p2wk/target cargo nextest run

## Command

```console
env CARGO_TARGET_DIR=/root/atelier-worktrees/atelier-p2wk/target cargo nextest run
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 97145
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-h3wg/crates/atelier-cli)
   Compiling atelier-workflow v0.2.0 (/root/atelier-worktrees/atelier-h3wg/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/root/atelier-worktrees/atelier-h3wg/crates/atelier-records)
   Compiling atelier-sqlite v0.2.0 (/root/atelier-worktrees/atelier-h3wg/crates/atelier-sqlite)
   Compiling atelier-app v0.2.0 (/root/atelier-worktrees/atelier-h3wg/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 10.66s
────────────
 Nextest run ID ae9321eb-af90-427c-b710-9eaa0bf03acc with nextest profile: default
    Starting 781 tests across 9 binaries
        PASS [   0.016s] (  1/781) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.017s] (  2/781) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.020s] (  3/781) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.019s] (  4/781) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.020s] (  5/781) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.021s] (  6/781) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.030s] (  7/781) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.018s] (  8/781) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.031s] (  9/781) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.033s] ( 10/781) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.034s] ( 11/781) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.035s] ( 12/781) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.035s] ( 13/781) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.024s] ( 14/781) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.036s] ( 15/781) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.036s] ( 16/781) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.021s] ( 17/781) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.037s] ( 18/781) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.023s] ( 19/781) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.041s] ( 20/781) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.026s] ( 21/781) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.025s] ( 22/781) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.042s] ( 23/781) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.044s] ( 24/781) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.026s] ( 25/781) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.030s] ( 26/781) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.029s] ( 27/781) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.031s] ( 28/781) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.051s] ( 29/781)
```
