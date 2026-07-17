---
created_at: "2026-07-16T19:40:13.928092911+00:00"
id: "atelier-ikzi"
evidence_type: "test"
captured_at: "2026-07-16T19:40:01.953802463+00:00"
command: "cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-fqzt"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fqzt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mission review lifecycle matrix and full workspace regression suite"
updated_at: "2026-07-16T19:40:13.930297075+00:00"
---

## Summary

Mission review lifecycle matrix and full workspace regression suite

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

Bytes: 88998
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.02s
────────────
 Nextest run ID 6fad35d0-3699-48e7-aee6-f35ba30bcfb4 with nextest profile: default
    Starting 731 tests across 9 binaries
        PASS [   0.009s] (  1/731) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.009s] (  2/731) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.009s] (  3/731) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.009s] (  4/731) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.030s] (  5/731) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.010s] (  6/731) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.031s] (  7/731) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.031s] (  8/731) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.032s] (  9/731) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.014s] ( 10/731) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.034s] ( 11/731) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.014s] ( 12/731) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.034s] ( 13/731) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.015s] ( 14/731) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.015s] ( 15/731) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.017s] ( 16/731) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.017s] ( 17/731) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.018s] ( 18/731) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.018s] ( 19/731) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.018s] ( 20/731) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.038s] ( 21/731) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.016s] ( 22/731) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.026s] ( 23/731) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.013s] ( 24/731) atelier-app mission_overview::tests::deduplicates_overlapping_roots_per_mission_but_preserves_shared_membership
        PASS [   0.018s] ( 25/731) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.014s] ( 26/731) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work
        PASS [   0.028s] ( 27/731) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.028s] ( 28/731) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.012s] ( 29/731) atelier-app objective_graph::tests::dependency_closure_fails_safely_with_an_actionable_cycle_path
        PASS [   0.029s] ( 30/731) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.015s] ( 31/731) atelier-app mission_overview::tests::orders_rows_deterministically_and_applies_budgets_after_counting
        PASS [   0.031s]
```
