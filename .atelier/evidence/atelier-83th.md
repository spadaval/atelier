---
created_at: "2026-07-16T22:49:21.885390988+00:00"
id: "atelier-83th"
evidence_type: "test"
captured_at: "2026-07-16T22:49:08.714163874+00:00"
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
  - kind: "issue"
    id: "atelier-qi40"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run"
updated_at: "2026-07-16T23:52:03.766430947+00:00"
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

Bytes: 94321
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.22s
────────────
 Nextest run ID aef778de-89e4-4db2-bf50-73209f350b95 with nextest profile: default
    Starting 767 tests across 9 binaries
        PASS [   0.009s] (  1/767) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.009s] (  2/767) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.009s] (  3/767) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.013s] (  4/767) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.010s] (  5/767) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.010s] (  6/767) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.011s] (  7/767) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.011s] (  8/767) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.012s] (  9/767) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.032s] ( 10/767) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.013s] ( 11/767) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.013s] ( 12/767) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.013s] ( 13/767) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.013s] ( 14/767) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.013s] ( 15/767) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.014s] ( 16/767) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.013s] ( 17/767) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.014s] ( 18/767) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.014s] ( 19/767) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.014s] ( 20/767) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.014s] ( 21/767) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.034s] ( 22/767) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.035s] ( 23/767) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.016s] ( 24/767) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.017s] ( 25/767) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.011s] ( 26/767) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.009s] ( 27/767) atelier-app mission_overview::tests::excludes_done_missions_and_reports_exceptional_work_until_all_is_requested
        PASS [   0.011s] ( 28/767) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work
        PASS [   0.014s] ( 29/767) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.013s] ( 30/767) atelier-app mission_overview::tests::deduplicates_overlapping_roots_per_mission_but_preserves_shared_membership
        PASS [   0.014s] ( 31/767) atelier-app mission_overview::tests::projects_directed_epic
```
