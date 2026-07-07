---
created_at: "2026-07-06T19:29:01.034715608+00:00"
id: "atelier-8szj"
evidence_type: "test"
captured_at: "2026-07-06T19:28:58.805192769+00:00"
command: "cargo nextest run -p atelier-app"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-0p7e"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0p7e"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-app"
updated_at: "2026-07-06T19:29:01.036671946+00:00"
---

## Summary

cargo nextest run -p atelier-app

## Command

```console
cargo nextest run -p atelier-app
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 10739
Truncated: yes

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
────────────
 Nextest run ID c422b983-7345-4bc1-b967-094a8e86f847 with nextest profile: default
    Starting 90 tests across 1 binary
        PASS [   0.010s] ( 1/90) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.010s] ( 2/90) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.010s] ( 3/90) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.012s] ( 4/90) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.012s] ( 5/90) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.012s] ( 6/90) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.023s] ( 7/90) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.024s] ( 8/90) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.013s] ( 9/90) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.013s] (10/90) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.013s] (11/90) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.024s] (12/90) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.013s] (13/90) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.013s] (14/90) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.013s] (15/90) atelier-app forgejo::tests::lists_review_comments_and_surfaces_api_failures
        PASS [   0.014s] (16/90) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.026s] (17/90) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.026s] (18/90) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.015s] (19/90) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.014s] (20/90) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.020s] (21/90) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.026s] (22/90) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.014s] (23/90) atelier-app pr::tests::parse_pull_request_reference_accepts_number_and_matching_url
        PASS [   0.014s] (24/90) atelier-app pr::tests::parse_pull_request_reference_rejects_mismatched_url_context
        PASS [   0.014s] (25/90) atelier-app pr::tests::parse_review_event_rejects_unknown_values
        PASS [   0.028s] (26/90) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.033s] (27/90) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.013s] (28/90) atelier-app project_config::tests::forgejo_loader_rejects_conflicting_workflow_role_authors
        PASS [   0.014s] (29/90) atelier-app project_config::tests::forgejo_loader_applies_workflow_role_authors
        PASS [   0.014s] (30/90) atelier-app project_config::tests::invalid_forgejo_config_names_and_legacy_role_authors
        PASS [   0.011s] (31/90) atelier-app project_config::tests::parses_global_user_forgejo_token
        PASS [   0.016s] (32/90) atelier-app project_config::tests::missing_forgejo_config_is_actionable
        PASS [   0.010s] (33/90) atelier-app project_config::tests::rejects_committed_runtime_and_compatibility_path_settings
        PASS [   0.011s] (34/90) atelier-app project_config::tests::rejec
```
