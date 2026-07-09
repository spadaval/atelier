---
created_at: "2026-07-06T19:53:43.983320134+00:00"
id: "atelier-evxl"
evidence_type: "test"
captured_at: "2026-07-06T19:53:32.365063215+00:00"
command: "cargo nextest run -p atelier-sqlite -p atelier-app -p atelier-cli"
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
  - kind: "issue"
    id: "atelier-5m81"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-sqlite -p atelier-app -p atelier-cli"
updated_at: "2026-07-06T19:54:42.809559435+00:00"
---

## Summary

cargo nextest run -p atelier-sqlite -p atelier-app -p atelier-cli

## Command

```console
cargo nextest run -p atelier-sqlite -p atelier-app -p atelier-cli
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 66694
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/mska-ckca/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.04s
────────────
 Nextest run ID e082af7e-6328-492c-9f79-6d7784ce788a with nextest profile: default
    Starting 546 tests across 6 binaries
        PASS [   0.009s] (  1/546) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.009s] (  2/546) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.009s] (  3/546) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.010s] (  4/546) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.010s] (  5/546) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.011s] (  6/546) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.013s] (  7/546) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.014s] (  8/546) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.014s] (  9/546) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.022s] ( 10/546) atelier-app forgejo::tests::lists_review_comments_and_surfaces_api_failures
        PASS [   0.024s] ( 11/546) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.024s] ( 12/546) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.012s] ( 13/546) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.025s] ( 14/546) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.026s] ( 15/546) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.025s] ( 16/546) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.026s] ( 17/546) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.027s] ( 18/546) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.013s] ( 19/546) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.014s] ( 20/546) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.028s] ( 21/546) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.028s] ( 22/546) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.021s] ( 23/546) atelier-app pr::tests::parse_pull_request_reference_accepts_number_and_matching_url
        PASS [   0.023s] ( 24/546) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.023s] ( 25/546) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.012s] ( 26/546) atelier-app pr::tests::parse_review_event_rejects_unknown_values
        PASS [   0.016s] ( 27/546) atelier-app pr::tests::parse_pull_request_reference_rejects_mismatched_url_context
        PASS [   0.012s] ( 28/546) atelier-app project_config::tests::forgejo_loader_applies_workflow_role_authors
        PASS [   0.010s] ( 29/546) atelier-app project_config::tests::missing_forgejo_config_is_actionable
        PASS [   0.013s] ( 30/546) atelier-app project_config::tests::invalid_forgejo_config_names_and_legacy_role_authors
        PASS [   0.015s] ( 31/546) atelier-app project_config::tests::forgejo_loader_rejects_conflicting_workflow_role_authors
        PASS [   0.014s] ( 32/546) atelier-app project_config::tests::parses_global_user_forgejo_token
        PASS [   0.010s] ( 33/546) atelier-app
```
