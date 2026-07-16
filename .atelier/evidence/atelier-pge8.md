---
created_at: "2026-07-16T18:54:44.340085379+00:00"
id: "atelier-pge8"
evidence_type: "test"
captured_at: "2026-07-16T18:54:33.469369252+00:00"
command: "cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-amfw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-amfw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run"
updated_at: "2026-07-16T18:54:44.342209346+00:00"
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

Bytes: 87051
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.98s
────────────
 Nextest run ID 6370afd0-267f-4ca3-ab29-b42fa6ffb2ea with nextest profile: default
    Starting 718 tests across 9 binaries
        PASS [   0.010s] (  1/718) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.009s] (  2/718) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.010s] (  3/718) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.010s] (  4/718) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.011s] (  5/718) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.012s] (  6/718) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.012s] (  7/718) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.012s] (  8/718) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.010s] (  9/718) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.010s] ( 10/718) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.009s] ( 11/718) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.010s] ( 12/718) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.009s] ( 13/718) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.030s] ( 14/718) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.030s] ( 15/718) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.031s] ( 16/718) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.032s] ( 17/718) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.032s] ( 18/718) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.032s] ( 19/718) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.032s] ( 20/718) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.012s] ( 21/718) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work
        PASS [   0.012s] ( 22/718) atelier-app mission_overview::tests::deduplicates_overlapping_roots_per_mission_but_preserves_shared_membership
        PASS [   0.024s] ( 23/718) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.034s] ( 24/718) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.033s] ( 25/718) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.034s] ( 26/718) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.036s] ( 27/718) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.010s] ( 28/718) atelier-app objective_graph::tests::dependency_closure_fails_safely_with_an_actionable_cycle_path
        PASS [   0.011s] ( 29/718) atelier-app objective_graph::tests::dependency_closure_reports_complete_direct_and_transitive_paths
        PASS [   0.014s] ( 30/718) atelier-app mission_overview::tests::orders_rows_deterministically_and_applies_budgets_after_counting
        PASS [   0.025s] ( 31/718) atelier-app mission_overview::tests::excludes_done_missions_and_reports
```
