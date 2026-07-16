---
created_at: "2026-07-16T20:01:19.000095711+00:00"
id: "atelier-4z9h"
evidence_type: "test"
captured_at: "2026-07-16T20:01:06.907847585+00:00"
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
title: "P1 remediation: public mission-plan review API/CLI, typed start authorization, generic-body forgery rejection, and direct/transitive blocker bypass enforcement"
updated_at: "2026-07-16T20:01:19.002142104+00:00"
---

## Summary

P1 remediation: public mission-plan review API/CLI, typed start authorization, generic-body forgery rejection, and direct/transitive blocker bypass enforcement

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

Bytes: 90098
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.11s
────────────
 Nextest run ID 86823732-5258-417b-9284-e920927efb72 with nextest profile: default
    Starting 738 tests across 9 binaries
        PASS [   0.010s] (  1/738) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.010s] (  2/738) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.011s] (  3/738) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.012s] (  4/738) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.010s] (  5/738) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.012s] (  6/738) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.012s] (  7/738) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.012s] (  8/738) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.012s] (  9/738) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.013s] ( 10/738) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.022s] ( 11/738) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.012s] ( 12/738) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.023s] ( 13/738) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.023s] ( 14/738) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.024s] ( 15/738) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.015s] ( 16/738) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.014s] ( 17/738) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.024s] ( 18/738) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.015s] ( 19/738) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.025s] ( 20/738) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.025s] ( 21/738) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.025s] ( 22/738) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.016s] ( 23/738) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.018s] ( 24/738) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.028s] ( 25/738) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.028s] ( 26/738) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.033s] ( 27/738) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.012s] ( 28/738) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work
        PASS [   0.013s] ( 29/738) atelier-app mission_overview::tests::excludes_done_missions_and_reports_exceptional_work_until_all_is_requested
        PASS [   0.036s] ( 30/738) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.025s] ( 31/738) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [
```
