---
created_at: "2026-07-16T22:32:04.781517749+00:00"
id: "atelier-h8y1"
evidence_type: "test"
captured_at: "2026-07-16T22:31:51.837808452+00:00"
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
updated_at: "2026-07-16T22:32:04.783411987+00:00"
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

Bytes: 93752
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.08s
────────────
 Nextest run ID 7346331e-d8a4-4f69-a30b-cbc35aae2e5f with nextest profile: default
    Starting 763 tests across 9 binaries
        PASS [   0.010s] (  1/763) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.010s] (  2/763) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.010s] (  3/763) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.010s] (  4/763) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.011s] (  5/763) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.013s] (  6/763) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.013s] (  7/763) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.014s] (  8/763) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.010s] (  9/763) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.013s] ( 10/763) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.012s] ( 11/763) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.013s] ( 12/763) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.011s] ( 13/763) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.014s] ( 14/763) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.027s] ( 15/763) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.028s] ( 16/763) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.029s] ( 17/763) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.029s] ( 18/763) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.030s] ( 19/763) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.030s] ( 20/763) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.016s] ( 21/763) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.031s] ( 22/763) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.031s] ( 23/763) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.033s] ( 24/763) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.033s] ( 25/763) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.034s] ( 26/763) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.034s] ( 27/763) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.013s] ( 28/763) atelier-app mission_overview::tests::deduplicates_overlapping_roots_per_mission_but_preserves_shared_membership
        PASS [   0.036s] ( 29/763) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.021s] ( 30/763) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.013s] ( 31/763) atelier-app mission_overview::tests::excludes_done_missions_and_reports_exceptional_work_until_all_is_requested
```

