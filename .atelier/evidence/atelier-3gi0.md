---
created_at: "2026-09-02T19:11:45.443512680+00:00"
id: "atelier-3gi0"
evidence_type: "test"
captured_at: "2026-09-02T19:11:32.884402472+00:00"
command: "cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-to59"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-to59"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run"
updated_at: "2026-09-02T19:11:45.445913569+00:00"
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

Bytes: 93139
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.15s
────────────
 Nextest run ID ccb5d61d-f059-453b-b9f2-bdb1477b42b7 with nextest profile: default
    Starting 762 tests across 9 binaries
        PASS [   0.008s] (  1/762) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.008s] (  2/762) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.011s] (  3/762) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.012s] (  4/762) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.013s] (  5/762) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.010s] (  6/762) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.021s] (  7/762) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.022s] (  8/762) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.022s] (  9/762) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.022s] ( 10/762) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.022s] ( 11/762) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.024s] ( 12/762) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.013s] ( 13/762) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.024s] ( 14/762) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.025s] ( 15/762) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.025s] ( 16/762) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.025s] ( 17/762) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.025s] ( 18/762) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.028s] ( 19/762) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.028s] ( 20/762) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.016s] ( 21/762) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.029s] ( 22/762) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.019s] ( 23/762) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.031s] ( 24/762) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.013s] ( 25/762) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.033s] ( 26/762) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.011s] ( 27/762) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work
        PASS [   0.022s] ( 28/762) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.011s] ( 29/762) atelier-app mission_overview::tests::deduplicates_overlapping_roots_per_mission_but_preserves_shared_membership
        PASS [   0.011s] ( 30/762) atelier-app mission_overview::tests::excludes_done_missions_and_reports_exceptional_work_until_all_is_requested
        PASS [   0.011s] ( 31/762) atelier-app objective_graph::tests::dependency_closure_fails_safely_with_an_actionable_cycle_path
        PA
```

