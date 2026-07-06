---
created_at: "2026-07-06T20:28:09.760859074+00:00"
id: "atelier-hmba"
evidence_type: "test"
captured_at: "2026-07-06T20:28:06.322682822+00:00"
command: "cargo nextest run -p atelier-records -p atelier-app"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-u7wi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u7wi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-records -p atelier-app"
updated_at: "2026-07-06T20:28:13.883099802+00:00"
---

## Summary

cargo nextest run -p atelier-records -p atelier-app

## Command

```console
cargo nextest run -p atelier-records -p atelier-app
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 16769
Truncated: yes

```text
   Compiling atelier-app v0.2.0 (/root/.codex/worktrees/mska-xa9s/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.25s
────────────
 Nextest run ID 55ee0f0d-7ffb-404b-b2e4-892ebef2a49c with nextest profile: default
    Starting 141 tests across 2 binaries
        PASS [   0.010s] (  1/141) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.010s] (  2/141) atelier-app forgejo::tests::lists_review_comments_and_surfaces_api_failures
        PASS [   0.011s] (  3/141) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.011s] (  4/141) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.011s] (  5/141) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.013s] (  6/141) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.013s] (  7/141) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.032s] (  8/141) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.032s] (  9/141) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.033s] ( 10/141) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.033s] ( 11/141) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.012s] ( 12/141) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.034s] ( 13/141) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.012s] ( 14/141) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.034s] ( 15/141) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.034s] ( 16/141) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.036s] ( 17/141) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.014s] ( 18/141) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.037s] ( 19/141) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.017s] ( 20/141) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.039s] ( 21/141) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.020s] ( 22/141) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.044s] ( 23/141) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.011s] ( 24/141) atelier-app pr::tests::parse_review_event_rejects_unknown_values
        PASS [   0.014s] ( 25/141) atelier-app pr::tests::parse_pull_request_reference_rejects_mismatched_url_context
        PASS [   0.014s] ( 26/141) atelier-app pr::tests::parse_pull_request_reference_accepts_number_and_matching_url
        PASS [   0.014s] ( 27/141) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.013s] ( 28/141) atelier-app project_config::tests::forgejo_loader_rejects_conflicting_workflow_role_authors
        PASS [   0.014s] ( 29/141) atelier-app project_config::tests::forgejo_loader_applies_workflow_role_authors
        PASS [   0.013s] ( 30/141) atelier-app project_config::tests::missing_forgejo_config_is_actionable
        PASS [   0.013s] ( 31/141) atelier-app project_config::tests::invalid_forgejo_config_names_and_legacy_role_authors
        PASS [   0.017s] ( 32/141) atelier-app project_config::tests::parses_global_user_forgejo_token
        PASS [   0.013s] ( 33/141) atelier-app
```
