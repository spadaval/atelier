---
created_at: "2026-08-06T00:24:53.804608273+00:00"
id: "atelier-kqon"
evidence_type: "test"
captured_at: "2026-08-06T00:24:41.379510929+00:00"
command: "cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-sfaq"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-sfaq"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run"
updated_at: "2026-08-06T00:24:53.806494379+00:00"
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

Bytes: 93014
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.23s
────────────
 Nextest run ID 9dff65a6-4b7a-45b1-9516-4314403d120b with nextest profile: default
    Starting 761 tests across 9 binaries
        PASS [   0.009s] (  1/761) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.011s] (  2/761) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.011s] (  3/761) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.011s] (  4/761) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.012s] (  5/761) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.018s] (  6/761) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.010s] (  7/761) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.009s] (  8/761) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.009s] (  9/761) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.012s] ( 10/761) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.013s] ( 11/761) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.027s] ( 12/761) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.027s] ( 13/761) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.010s] ( 14/761) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.028s] ( 15/761) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.028s] ( 16/761) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.028s] ( 17/761) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.028s] ( 18/761) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.027s] ( 19/761) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.029s] ( 20/761) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.029s] ( 21/761) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.029s] ( 22/761) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.030s] ( 23/761) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.030s] ( 24/761) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.030s] ( 25/761) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.030s] ( 26/761) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.021s] ( 27/761) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.021s] ( 28/761) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.023s] ( 29/761) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.016s] ( 30/761) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work
        PASS [   0.012s] ( 31/761) atelier-app objective_graph::tests::dependency_closure_reports_complete_direct_and_transitive_paths
        PASS [   0.013s] ( 32/761) atelier-app mission_overview::tests::deduplicates_
```

