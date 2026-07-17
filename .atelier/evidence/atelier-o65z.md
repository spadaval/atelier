---
created_at: "2026-07-17T00:13:01.598758218+00:00"
id: "atelier-o65z"
evidence_type: "test"
captured_at: "2026-07-17T00:12:47.967278880+00:00"
command: "cargo nextest run --no-fail-fast"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-t876"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-t876"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run --no-fail-fast"
updated_at: "2026-07-17T00:13:01.600651206+00:00"
---

## Summary

cargo nextest run --no-fail-fast

## Command

```console
cargo nextest run --no-fail-fast
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 96449
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p4z2/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.20s
────────────
 Nextest run ID 377476b9-5889-457c-9b13-50ec7c4f7ec2 with nextest profile: default
    Starting 781 tests across 9 binaries
        PASS [   0.009s] (  1/781) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.009s] (  2/781) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.009s] (  3/781) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.011s] (  4/781) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.012s] (  5/781) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.013s] (  6/781) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.027s] (  7/781) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.027s] (  8/781) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.028s] (  9/781) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.028s] ( 10/781) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.028s] ( 11/781) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.012s] ( 12/781) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.012s] ( 13/781) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.012s] ( 14/781) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.029s] ( 15/781) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.012s] ( 16/781) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.030s] ( 17/781) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.030s] ( 18/781) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.014s] ( 19/781) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.014s] ( 20/781) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.014s] ( 21/781) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.014s] ( 22/781) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.014s] ( 23/781) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.031s] ( 24/781) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.032s] ( 25/781) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.016s] ( 26/781) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.035s] ( 27/781) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.019s] ( 28/781) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.021s] ( 29/781) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.011s] ( 30/781) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work
        PASS [   0.011s] ( 31/781) atelier-app mission_overview::tests::projects_directed_epics_direct_work_descendants_and_blocker_state
        PASS [   0.011s] (
```
