---
created_at: "2026-07-16T20:17:24.379936081+00:00"
id: "atelier-8iqw"
evidence_type: "test"
captured_at: "2026-07-16T20:17:11.665523499+00:00"
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
title: "Final P1 remediation: atomic canonical graph-reference validation and authenticated cross-revision mission-plan rework/re-request lifecycle"
updated_at: "2026-07-16T20:17:24.382056494+00:00"
---

## Summary

Final P1 remediation: atomic canonical graph-reference validation and authenticated cross-revision mission-plan rework/re-request lifecycle

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

Bytes: 90717
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.24s
────────────
 Nextest run ID c98784d1-3f4a-4d57-a332-7124a142498d with nextest profile: default
    Starting 742 tests across 9 binaries
        PASS [   0.009s] (  1/742) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.010s] (  2/742) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.011s] (  3/742) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.010s] (  4/742) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.011s] (  5/742) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.012s] (  6/742) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.016s] (  7/742) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.009s] (  8/742) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.012s] (  9/742) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.012s] ( 10/742) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.014s] ( 11/742) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.009s] ( 12/742) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.017s] ( 13/742) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.026s] ( 14/742) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.027s] ( 15/742) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.027s] ( 16/742) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.028s] ( 17/742) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.028s] ( 18/742) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.029s] ( 19/742) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.029s] ( 20/742) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.019s] ( 21/742) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.030s] ( 22/742) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.032s] ( 23/742) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.032s] ( 24/742) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.033s] ( 25/742) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.034s] ( 26/742) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.035s] ( 27/742) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.011s] ( 28/742) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work
        PASS [   0.009s] ( 29/742) atelier-app objective_graph::tests::dependency_closure_reports_complete_direct_and_transitive_paths
        PASS [   0.012s] ( 30/742) atelier-app mission_overview::tests::excludes_done_missions_and_reports_exceptional_work_until_all_is_requested
        PASS [   0.013s] ( 31/742) atelier-app mission_overview::tests::orders_rows_deterministically_and_applies_budgets_after_counting
        P
```
