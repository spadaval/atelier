---
created_at: "2026-07-06T20:36:53.800315285+00:00"
id: "atelier-0ygc"
evidence_type: "test"
captured_at: "2026-07-06T20:36:45.535894157+00:00"
command: "sh -c 'cargo nextest run -p atelier-sqlite -p atelier-app -p atelier-workflow && cargo nextest run -p atelier-cli -E '\"'\"'test(/mission_cache_worktree|test_doctor_human_separates_cache_and_runtime_state_health|test_doctor_distinguishes_missing_runtime_cache_database|test_doctor_fix_repairs_missing_and_stale_local_cache_state|commands::prune::tests|cache_acquisition_tests/)'\"'\"''"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-xa9s"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xa9s"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "sh -c 'cargo nextest run -p atelier-sqlite -p atelier-app -p atelier-workflow && cargo nextest run -p atelier-cli -E '\"'\"'test(/mission_cache_worktree|test_doctor_human_separates_cache_and_runtime_state_health|test_doctor_distinguishes_missing_runtime_cache_database|test_doctor_fix_repairs_missing_and_stale_local_cache_state|commands::prune::tests|cache_acquisition_tests/)'\"'\"''"
updated_at: "2026-07-06T20:36:53.803487954+00:00"
---

## Summary

sh -c 'cargo nextest run -p atelier-sqlite -p atelier-app -p atelier-workflow && cargo nextest run -p atelier-cli -E '"'"'test(/mission_cache_worktree|test_doctor_human_separates_cache_and_runtime_state_health|test_doctor_distinguishes_missing_runtime_cache_database|test_doctor_fix_repairs_missing_and_stale_local_cache_state|commands::prune::tests|cache_acquisition_tests/)'"'"''

## Command

```console
sh -c 'cargo nextest run -p atelier-sqlite -p atelier-app -p atelier-workflow && cargo nextest run -p atelier-cli -E '"'"'test(/mission_cache_worktree|test_doctor_human_separates_cache_and_runtime_state_health|test_doctor_distinguishes_missing_runtime_cache_database|test_doctor_fix_repairs_missing_and_stale_local_cache_state|commands::prune::tests|cache_acquisition_tests/)'"'"''
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 25576
Truncated: yes

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
────────────
 Nextest run ID ac8eed59-236b-477c-972b-f485fb0580f4 with nextest profile: default
    Starting 139 tests across 3 binaries
        PASS [   0.008s] (  1/139) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.009s] (  2/139) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.009s] (  3/139) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.009s] (  4/139) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.009s] (  5/139) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.009s] (  6/139) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.011s] (  7/139) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.020s] (  8/139) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.022s] (  9/139) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.022s] ( 10/139) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.023s] ( 11/139) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.024s] ( 12/139) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.024s] ( 13/139) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.024s] ( 14/139) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.025s] ( 15/139) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.024s] ( 16/139) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.024s] ( 17/139) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.026s] ( 18/139) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.028s] ( 19/139) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.011s] ( 20/139) atelier-app pr::tests::parse_review_event_rejects_unknown_values
        PASS [   0.013s] ( 21/139) atelier-app pr::tests::parse_pull_request_reference_accepts_number_and_matching_url
        PASS [   0.014s] ( 22/139) atelier-app pr::tests::parse_pull_request_reference_rejects_mismatched_url_context
        PASS [   0.026s] ( 23/139) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.012s] ( 24/139) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.038s] ( 25/139) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.028s] ( 26/139) atelier-app forgejo::tests::lists_review_comments_and_surfaces_api_failures
        PASS [   0.029s] ( 27/139) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.012s] ( 28/139) atelier-app project_config::tests::forgejo_loader_applies_workflow_role_authors
        PASS [   0.012s] ( 29/139) atelier-app project_config::tests::invalid_forgejo_config_names_and_legacy_role_authors
        PASS [   0.011s] ( 30/139) atelier-app project_config::tests::missing_forgejo_config_is_actionable
        PASS [   0.011s] ( 31/139) atelier-app project_config::tests::parses_global_user_forgejo_token
        PASS [   0.014s] ( 32/139) atelier-app project_config::tests::forgejo_loader_rejects_conflicting_workflow_role_authors
        PASS [   0.009s] ( 33/139) atelier-app project_config::tests::parses_prune_canonical_retention_days
        PASS [   0.010s]
```
