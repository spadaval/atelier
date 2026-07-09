---
created_at: "2026-07-06T18:15:23.299354056+00:00"
id: "atelier-7dc3"
evidence_type: "test"
captured_at: "2026-07-06T18:14:53.023218791+00:00"
command: "sh -c 'tmp_home=$(mktemp -d); HOME=\"$tmp_home\" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run'"
exit_status: "100"
agent_identity: "independent-validator"
target:
  kind: "issue"
  id: "atelier-eqq6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-eqq6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "sh -c 'tmp_home=$(mktemp -d); HOME=\"$tmp_home\" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run'"
updated_at: "2026-07-06T18:15:29.079369266+00:00"
---

## Summary

sh -c 'tmp_home=$(mktemp -d); HOME="$tmp_home" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run'

## Command

```console
sh -c 'tmp_home=$(mktemp -d); HOME="$tmp_home" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run'
```

Exit status: 100

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 88051
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-eqq6/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.26s
────────────
 Nextest run ID 0ca2b137-a6aa-49e5-970d-bdc18f678283 with nextest profile: default
    Starting 708 tests across 9 binaries (4 tests skipped)
        PASS [   0.009s] (  1/708) atelier-app command_storage::tests::access_modes_declare_projection_freshness_policy
        PASS [   0.011s] (  2/708) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.011s] (  3/708) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.012s] (  4/708) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.012s] (  5/708) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.013s] (  6/708) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.013s] (  7/708) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.016s] (  8/708) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.016s] (  9/708) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.011s] ( 10/708) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.028s] ( 11/708) atelier-app command_storage::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.028s] ( 12/708) atelier-app forgejo::tests::lists_review_comments_and_surfaces_api_failures
        PASS [   0.022s] ( 13/708) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.022s] ( 14/708) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.028s] ( 15/708) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.029s] ( 16/708) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.030s] ( 17/708) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.030s] ( 18/708) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.030s] ( 19/708) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.032s] ( 20/708) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.029s] ( 21/708) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.013s] ( 22/708) atelier-app pr::tests::parse_pull_request_reference_accepts_number_and_matching_url
        PASS [   0.033s] ( 23/708) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.016s] ( 24/708) atelier-app pr::tests::parse_pull_request_reference_rejects_mismatched_url_context
        PASS [   0.043s] ( 25/708) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.025s] ( 26/708) atelier-app pr::tests::parse_review_event_rejects_unknown_values
        PASS [   0.054s] ( 27/708) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.020s] ( 28/708) atelier-app project_config::tests::forgejo_loader_rejects_conflicting_workflow_role_authors
        PASS [   0.037s] ( 29/708) atelier-app project_config::tests::forgejo_loader_applies_workflow_role_authors
        PASS [   0.027s] ( 30/708) atelier-app project_config::tests::invalid_forgejo_config_names_and_legacy_role_authors
        PASS [   0.028s] ( 31/708) atelier-app project_config::tests::missing_forgejo_config_is_actionable
        PASS [   0.020s] ( 32/708) atelier-app project_config::tests::parses_global_user_forgejo_token
        PASS [   0.025s
```
