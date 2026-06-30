---
created_at: "2026-06-30T04:03:10.132624794+00:00"
id: "atelier-qdce"
evidence_type: "test"
captured_at: "2026-06-30T04:02:36.146751515+00:00"
command: "cargo nextest run"
exit_status: "0"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0n94"
    role: "validates"
  - kind: "issue"
    id: "atelier-1a2n"
    role: "validates"
  - kind: "issue"
    id: "atelier-1mga"
    role: "validates"
  - kind: "issue"
    id: "atelier-3zcr"
    role: "validates"
  - kind: "issue"
    id: "atelier-45yt"
    role: "validates"
  - kind: "issue"
    id: "atelier-4d3f"
    role: "validates"
  - kind: "issue"
    id: "atelier-97p7"
    role: "validates"
  - kind: "issue"
    id: "atelier-brqs"
    role: "validates"
  - kind: "issue"
    id: "atelier-dyp1"
    role: "validates"
  - kind: "issue"
    id: "atelier-ef94"
    role: "validates"
  - kind: "issue"
    id: "atelier-fxhy"
    role: "validates"
  - kind: "issue"
    id: "atelier-gsor"
    role: "validates"
  - kind: "issue"
    id: "atelier-h9vl"
    role: "validates"
  - kind: "issue"
    id: "atelier-i0ze"
    role: "validates"
  - kind: "issue"
    id: "atelier-i9bo"
    role: "validates"
  - kind: "issue"
    id: "atelier-kfey"
    role: "validates"
  - kind: "issue"
    id: "atelier-kzu2"
    role: "validates"
  - kind: "issue"
    id: "atelier-p19n"
    role: "validates"
  - kind: "issue"
    id: "atelier-pc7s"
    role: "validates"
  - kind: "issue"
    id: "atelier-pi3o"
    role: "validates"
  - kind: "issue"
    id: "atelier-qdp8"
    role: "validates"
  - kind: "issue"
    id: "atelier-ql9k"
    role: "validates"
  - kind: "issue"
    id: "atelier-qo3w"
    role: "validates"
  - kind: "issue"
    id: "atelier-r6a2"
    role: "validates"
  - kind: "issue"
    id: "atelier-see0"
    role: "validates"
  - kind: "issue"
    id: "atelier-siu5"
    role: "validates"
  - kind: "issue"
    id: "atelier-ubf2"
    role: "validates"
  - kind: "issue"
    id: "atelier-v132"
    role: "validates"
  - kind: "issue"
    id: "atelier-w0mr"
    role: "validates"
  - kind: "issue"
    id: "atelier-xbpd"
    role: "validates"
  - kind: "issue"
    id: "atelier-yr2v"
    role: "validates"
  - kind: "issue"
    id: "atelier-zjnp"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run"
updated_at: "2026-06-30T04:06:18.744474401+00:00"
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

Bytes: 80589
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/5aa7/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.81s
────────────
 Nextest run ID e20c99d8-cc25-4302-b6d5-6b301f77d62a with nextest profile: default
    Starting 690 tests across 9 binaries (4 tests skipped)
        PASS [   0.009s] (  1/690) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.009s] (  2/690) atelier-app command_storage::tests::access_modes_declare_projection_freshness_policy
        PASS [   0.010s] (  3/690) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.010s] (  4/690) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.011s] (  5/690) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.013s] (  6/690) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.013s] (  7/690) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.017s] (  8/690) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.009s] (  9/690) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.027s] ( 10/690) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.028s] ( 11/690) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.028s] ( 12/690) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.030s] ( 13/690) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.014s] ( 14/690) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.031s] ( 15/690) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.014s] ( 16/690) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.031s] ( 17/690) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.031s] ( 18/690) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.015s] ( 19/690) atelier-app forgejo::tests::lists_review_comments_and_surfaces_api_failures
        PASS [   0.033s] ( 20/690) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.034s] ( 21/690) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.021s] ( 22/690) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.021s] ( 23/690) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.011s] ( 24/690) atelier-app pr::tests::parse_pull_request_reference_rejects_mismatched_url_context
        PASS [   0.012s] ( 25/690) atelier-app pr::tests::parse_review_event_rejects_unknown_values
        PASS [   0.011s] ( 26/690) atelier-app project_config::tests::forgejo_loader_applies_workflow_role_authors
        PASS [   0.030s] ( 27/690) atelier-app pr::tests::parse_pull_request_reference_accepts_number_and_matching_url
        PASS [   0.012s] ( 28/690) atelier-app project_config::tests::forgejo_loader_rejects_conflicting_workflow_role_authors
        PASS [   0.011s] ( 29/690) atelier-app project_config::tests::invalid_forgejo_config_names_and_legacy_role_authors
        PASS [   0.013s] ( 30/690) atelier-app project_config::tests::missing_forgejo_config_is_actionable
        PASS [   0.011s] ( 31/690) atelier-app project_config::tests::parses_prune_canonical_retention_days
        PASS [   0.009s] ( 32/690) atelier-app project_config::tests::parses_valid_forgejo_config_without_role_authors
        PASS [   0.010s] ( 3
```

