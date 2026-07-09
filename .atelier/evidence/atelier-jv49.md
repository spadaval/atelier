---
created_at: "2026-07-09T17:23:58.240664857+00:00"
id: "atelier-jv49"
evidence_type: "test"
captured_at: "2026-07-09T17:23:47.368588879+00:00"
command: "cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-wyxn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-wyxn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run"
updated_at: "2026-07-09T17:24:02.484669674+00:00"
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

Bytes: 79958
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/9661/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.01s
────────────
 Nextest run ID 568a0dcc-9ab0-44e1-94cd-6fbb755d23c8 with nextest profile: default
    Starting 667 tests across 9 binaries
        PASS [   0.009s] (  1/667) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.011s] (  2/667) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.011s] (  3/667) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.015s] (  4/667) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.021s] (  5/667) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.010s] (  6/667) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.022s] (  7/667) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.022s] (  8/667) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.023s] (  9/667) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.023s] ( 10/667) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.023s] ( 11/667) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.023s] ( 12/667) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.013s] ( 13/667) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.024s] ( 14/667) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.024s] ( 15/667) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.024s] ( 16/667) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.013s] ( 17/667) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.024s] ( 18/667) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.014s] ( 19/667) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.026s] ( 20/667) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.026s] ( 21/667) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.010s] ( 22/667) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.033s] ( 23/667) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.011s] ( 24/667) atelier-app pr::tests::parse_review_event_rejects_unknown_values
        PASS [   0.024s] ( 25/667) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.026s] ( 26/667) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.014s] ( 27/667) atelier-app pr::tests::parse_pull_request_reference_accepts_number_and_matching_url
        PASS [   0.022s] ( 28/667) atelier-app pr::tests::parse_pull_request_reference_rejects_mismatched_url_context
        PASS [   0.011s] ( 29/667) atelier-app project_config::tests::forgejo_loader_applies_workflow_role_authors
        PASS [   0.013s] ( 30/667) atelier-app project_config::tests::forgejo_loader_rejects_conflicting_workflow_role_authors
        PASS [   0.013s] ( 31/667) atelier-app project_config::tests::missing_forgejo_config_is_actionable
        PASS [   0.008s] ( 32/667) atelier-app project_config::tests::parses_global_user_forgejo_token
        PASS [   0.009s] ( 33/667) at
```
