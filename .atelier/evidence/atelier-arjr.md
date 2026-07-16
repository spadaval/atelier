---
created_at: "2026-07-16T22:46:43.992112154+00:00"
id: "atelier-arjr"
evidence_type: "test"
captured_at: "2026-07-16T22:46:30.560523202+00:00"
command: "cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-72k4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-72k4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run"
updated_at: "2026-07-16T22:46:43.994296728+00:00"
---

## Summary

cargo nextest run

## Command

```console
cargo nextest run
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 94171
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.09s
────────────
 Nextest run ID ab61c412-b896-4230-996e-85cff5615568 with nextest profile: default
    Starting 766 tests across 9 binaries
        PASS [   0.009s] (  1/766) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.010s] (  2/766) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.010s] (  3/766) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.012s] (  4/766) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.012s] (  5/766) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.013s] (  6/766) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.012s] (  7/766) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.013s] (  8/766) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.014s] (  9/766) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.013s] ( 10/766) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.012s] ( 11/766) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.012s] ( 12/766) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.014s] ( 13/766) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.014s] ( 14/766) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.024s] ( 15/766) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.012s] ( 16/766) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.026s] ( 17/766) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.027s] ( 18/766) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.013s] ( 19/766) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.027s] ( 20/766) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.027s] ( 21/766) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.028s] ( 22/766) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.028s] ( 23/766) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.028s] ( 24/766) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.028s] ( 25/766) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.029s] ( 26/766) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.028s] ( 27/766) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.015s] ( 28/766) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.013s] ( 29/766) atelier-app mission_overview::tests::projects_directed_epics_direct_work_descendants_and_blocker_state
        PASS [   0.014s] ( 30/766) atelier-app mission_overview::tests::deduplicates_overlapping_roots_per_mission_but_preserves_shared_membership
        PASS [   0.014s] ( 31/766) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_
```
