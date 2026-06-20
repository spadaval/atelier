---
created_at: "2026-06-20T21:06:58.124746264+00:00"
id: "atelier-2ecc"
evidence_type: "test"
captured_at: "2026-06-20T21:06:20.815353984+00:00"
command: "cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-v2o6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-v2o6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run"
updated_at: "2026-06-20T21:07:01.322425504+00:00"
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

Bytes: 85766
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.15s
────────────
 Nextest run ID cf4ca0f5-5be5-497e-8c7d-d2db619f5a59 with nextest profile: default
    Starting 733 tests across 9 binaries (69 tests skipped)
        PASS [   0.010s] (  1/733) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.011s] (  2/733) atelier-app command_storage::tests::access_modes_declare_projection_freshness_policy
        PASS [   0.019s] (  3/733) atelier-app pr::tests::parse_pull_request_reference_rejects_mismatched_url_context
        PASS [   0.027s] (  4/733) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.028s] (  5/733) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.028s] (  6/733) atelier-app pr::tests::parse_review_event_rejects_unknown_values
        PASS [   0.028s] (  7/733) atelier-app forgejo::tests::lists_review_comments_and_surfaces_api_failures
        PASS [   0.029s] (  8/733) atelier-app pr::tests::parse_pull_request_reference_accepts_number_and_matching_url
        PASS [   0.029s] (  9/733) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.029s] ( 10/733) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.029s] ( 11/733) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.029s] ( 12/733) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.029s] ( 13/733) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.034s] ( 14/733) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.035s] ( 15/733) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.014s] ( 16/733) atelier-app project_config::tests::forgejo_loader_rejects_conflicting_workflow_role_authors
        PASS [   0.018s] ( 17/733) atelier-app project_config::tests::forgejo_loader_applies_workflow_role_authors
        PASS [   0.013s] ( 18/733) atelier-app project_config::tests::invalid_forgejo_config_names_and_legacy_role_authors
        PASS [   0.033s] ( 19/733) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.033s] ( 20/733) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.034s] ( 21/733) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.016s] ( 22/733) atelier-app project_config::tests::missing_forgejo_config_is_actionable
        PASS [   0.012s] ( 23/733) atelier-app project_config::tests::rejects_legacy_top_level_forgejo_config
        PASS [   0.013s] ( 24/733) atelier-app project_config::tests::rejects_committed_runtime_and_compatibility_path_settings
        PASS [   0.013s] ( 25/733) atelier-app project_config::tests::rejects_mixed_room_and_provider_config
        PASS [   0.019s] ( 26/733) atelier-app project_config::tests::parses_valid_forgejo_config_without_role_authors
        PASS [   0.208s] ( 27/733) atelier-app export::tests::test_canonical_export_preserves_issue_activity_sidecars
        PASS [   0.208s] ( 28/733) atelier-app export::tests::test_canonical_check_reports_stale_projection_metadata
        PASS [   0.209s] ( 29/733) atelier-app export::tests::test_canonical_check_reports_dangling_link
        PASS [   0.213s] ( 30/733) atelier-app export::tests::test_canonical_check_ignores_sqlite_only_canonical_drift
        PASS [   0.221s] ( 31/733) atelier-app export::tests::test_canonical_issue_type_is_explicit_not_label_derived
        PASS [   0.224s] ( 32/733) atelier-app export::tests::test_canonical_markdown_serialization_stability
        PASS [   0.232s] ( 33/733) atelier-app export::tests::test_canonical_check_reports_invalid_duplicate_id
```

