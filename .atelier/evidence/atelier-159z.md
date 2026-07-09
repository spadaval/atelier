---
created_at: "2026-07-06T18:34:27.713826543+00:00"
id: "atelier-159z"
evidence_type: "validation"
captured_at: "2026-07-06T18:33:49.682209377+00:00"
command: "bash -lc 'tmp=$(mktemp -d); HOME=\"$tmp\" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ye11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ye11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent full Rust suite under isolated HOME to avoid the documented provider-token fixture contamination."
updated_at: "2026-07-06T18:34:31.453813326+00:00"
---

## Summary

Independent full Rust suite under isolated HOME to avoid the documented provider-token fixture contamination.

## Command

```console
bash -lc 'tmp=$(mktemp -d); HOME="$tmp" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 84227
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-ye11/crates/atelier-cli)
   Compiling atelier-core v0.2.0 (/root/atelier-worktrees/atelier-ye11/crates/atelier-core)
   Compiling atelier-workflow v0.2.0 (/root/atelier-worktrees/atelier-ye11/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/root/atelier-worktrees/atelier-ye11/crates/atelier-records)
   Compiling atelier-sqlite v0.2.0 (/root/atelier-worktrees/atelier-ye11/crates/atelier-sqlite)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.82s
────────────
 Nextest run ID 0aebdcf5-acdf-4bd2-9642-46a55811b75f with nextest profile: default
    Starting 713 tests across 9 binaries (4 tests skipped)
        PASS [   0.009s] (  1/713) atelier-app command_storage::tests::access_modes_declare_projection_freshness_policy
        PASS [   0.010s] (  2/713) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.010s] (  3/713) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.012s] (  4/713) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.012s] (  5/713) atelier-app command_storage::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.012s] (  6/713) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.012s] (  7/713) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.013s] (  8/713) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.013s] (  9/713) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.011s] ( 10/713) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.011s] ( 11/713) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.009s] ( 12/713) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.013s] ( 13/713) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.010s] ( 14/713) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.023s] ( 15/713) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.024s] ( 16/713) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.026s] ( 17/713) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.017s] ( 18/713) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.029s] ( 19/713) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.029s] ( 20/713) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.030s] ( 21/713) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.032s] ( 22/713) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.015s] ( 23/713) atelier-app pr::tests::parse_pull_request_reference_accepts_number_and_matching_url
        PASS [   0.015s] ( 24/713) atelier-app pr::tests::parse_pull_request_reference_rejects_mismatched_url_context
        PASS [   0.031s] ( 25/713) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.031s] ( 26/713) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.020s] ( 27/713) atelier-app pr::tests::parse_review_event_rejects_unknown_values
        PASS [   0.045s] ( 28/713) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.013s] ( 29/713) atelier-app project_config::tests::forgejo_loade
```
