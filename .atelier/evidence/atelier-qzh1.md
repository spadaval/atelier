---
created_at: "2026-08-05T18:09:20.651127574+00:00"
id: "atelier-qzh1"
evidence_type: "test"
captured_at: "2026-08-05T18:09:08.243831649+00:00"
command: "cargo nextest run"
exit_status: "0"
agent_identity: "codex"
target:
  kind: "issue"
  id: "atelier-2a1t"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2a1t"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PASS: full workspace regression suite for adaptive contract candidate"
updated_at: "2026-08-05T18:09:20.653848212+00:00"
---

## Summary

PASS: full workspace regression suite for adaptive contract candidate

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

Bytes: 90589
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.47s
────────────
 Nextest run ID 6126249f-ebc3-4825-be27-8867a00f701f with nextest profile: default
    Starting 743 tests across 9 binaries
        PASS [   0.008s] (  1/743) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.008s] (  2/743) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.009s] (  3/743) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.009s] (  4/743) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.009s] (  5/743) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.011s] (  6/743) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.012s] (  7/743) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.008s] (  8/743) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.019s] (  9/743) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.019s] ( 10/743) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.011s] ( 11/743) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.012s] ( 12/743) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.012s] ( 13/743) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.010s] ( 14/743) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.021s] ( 15/743) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.021s] ( 16/743) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.022s] ( 17/743) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.022s] ( 18/743) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.014s] ( 19/743) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.022s] ( 20/743) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.022s] ( 21/743) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.023s] ( 22/743) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.024s] ( 23/743) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.018s] ( 24/743) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.018s] ( 25/743) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.029s] ( 26/743) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.031s] ( 27/743) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.010s] ( 28/743) atelier-app mission_overview::tests::deduplicates_overlapping_roots_per_mission_but_preserves_shared_membership
        PASS [   0.031s] ( 29/743) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.012s] ( 30/743) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work
        PASS [   0.011s] ( 31/743) atelier-app objective_graph::tests::dependency_closure_reports_complete_direct_and_transitive_paths
        PASS [   0.023s] ( 32/743) atelier-app command_surface::tests::extracts_vi
```

