---
created_at: "2026-07-16T23:43:23.956753477+00:00"
id: "atelier-5vf4"
evidence_type: "validation"
captured_at: "2026-07-16T23:43:11.001466371+00:00"
command: "cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-p2wk"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-p2wk"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run"
updated_at: "2026-07-16T23:43:23.958683612+00:00"
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

Bytes: 96449
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.24s
────────────
 Nextest run ID 30385d7c-0ce0-42a3-b817-907959363306 with nextest profile: default
    Starting 781 tests across 9 binaries
        PASS [   0.010s] (  1/781) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.010s] (  2/781) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.010s] (  3/781) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.011s] (  4/781) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.012s] (  5/781) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.013s] (  6/781) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.022s] (  7/781) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.012s] (  8/781) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.022s] (  9/781) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.013s] ( 10/781) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.012s] ( 11/781) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.023s] ( 12/781) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.013s] ( 13/781) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.024s] ( 14/781) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.012s] ( 15/781) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.014s] ( 16/781) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.024s] ( 17/781) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.025s] ( 18/781) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.025s] ( 19/781) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.014s] ( 20/781) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.014s] ( 21/781) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.026s] ( 22/781) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.027s] ( 23/781) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.027s] ( 24/781) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.027s] ( 25/781) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.032s] ( 26/781) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.011s] ( 27/781) atelier-app mission_overview::tests::excludes_done_missions_and_reports_exceptional_work_until_all_is_requested
        PASS [   0.011s] ( 28/781) atelier-app mission_overview::tests::projects_directed_epics_direct_work_descendants_and_blocker_state
        PASS [   0.012s] ( 29/781) atelier-app mission_overview::tests::orders_rows_deterministically_and_applies_budgets_after_counting
        PASS [   0.013s] ( 30/781) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work
        PASS [   0.029s] ( 31/781) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.0
```
