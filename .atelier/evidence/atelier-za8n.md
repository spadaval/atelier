---
created_at: "2026-07-09T16:41:17.687069922+00:00"
id: "atelier-za8n"
evidence_type: "test"
captured_at: "2026-07-09T16:41:06.481915610+00:00"
command: "cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-g5fl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-g5fl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run"
updated_at: "2026-07-09T16:41:17.688930085+00:00"
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

Bytes: 82321
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/e613/atelier-c0mp-coordination/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.72s
────────────
 Nextest run ID 7fca3d62-2e5f-4df9-b528-bc684ec585f6 with nextest profile: default
    Starting 684 tests across 9 binaries
        PASS [   0.009s] (  1/684) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.010s] (  2/684) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.011s] (  3/684) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.012s] (  4/684) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.013s] (  5/684) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.018s] (  6/684) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.010s] (  7/684) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.009s] (  8/684) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.012s] (  9/684) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.010s] ( 10/684) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.013s] ( 11/684) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.028s] ( 12/684) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.028s] ( 13/684) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.029s] ( 14/684) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.029s] ( 15/684) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.030s] ( 16/684) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.031s] ( 17/684) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.032s] ( 18/684) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.032s] ( 19/684) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.033s] ( 20/684) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.033s] ( 21/684) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.017s] ( 22/684) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.034s] ( 23/684) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.034s] ( 24/684) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.012s] ( 25/684) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work
        PASS [   0.036s] ( 26/684) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.038s] ( 27/684) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.039s] ( 28/684) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.013s] ( 29/684) atelier-app mission_overview::tests::projects_directed_epics_direct_work_descendants_and_blocker_state
        PASS [   0.013s] ( 30/684) atelier-app mission_overview::tests::deduplicates_overlapping_roots_per_mission_but_preserves_shared_membership
        PASS [   0.014s] ( 31/684) atelier-app mission_overview::tests::excludes_done_missions_and_reports_exceptional_work_until_all_is_requested
        PASS [   0.014s]
```
