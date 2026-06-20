---
created_at: "2026-06-20T21:02:45.174526062+00:00"
id: "atelier-c6o2"
evidence_type: "test"
captured_at: "2026-06-20T21:02:07.279406098+00:00"
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
updated_at: "2026-06-20T21:02:48.134461258+00:00"
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.09s
────────────
 Nextest run ID 2ccfd3a0-6ba3-495f-994f-d4b469fa7340 with nextest profile: default
    Starting 733 tests across 9 binaries (69 tests skipped)
        PASS [   0.008s] (  1/733) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.010s] (  2/733) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.010s] (  3/733) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.011s] (  4/733) atelier-app forgejo::tests::lists_review_comments_and_surfaces_api_failures
        PASS [   0.011s] (  5/733) atelier-app command_storage::tests::access_modes_declare_projection_freshness_policy
        PASS [   0.012s] (  6/733) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.012s] (  7/733) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.012s] (  8/733) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.023s] (  9/733) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.032s] ( 10/733) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.034s] ( 11/733) atelier-app pr::tests::parse_pull_request_reference_rejects_mismatched_url_context
        PASS [   0.035s] ( 12/733) atelier-app pr::tests::parse_pull_request_reference_accepts_number_and_matching_url
        PASS [   0.035s] ( 13/733) atelier-app pr::tests::parse_review_event_rejects_unknown_values
        PASS [   0.036s] ( 14/733) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.016s] ( 15/733) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.044s] ( 16/733) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.013s] ( 17/733) atelier-app project_config::tests::invalid_forgejo_config_names_and_legacy_role_authors
        PASS [   0.013s] ( 18/733) atelier-app project_config::tests::missing_forgejo_config_is_actionable
        PASS [   0.016s] ( 19/733) atelier-app project_config::tests::forgejo_loader_applies_workflow_role_authors
        PASS [   0.011s] ( 20/733) atelier-app project_config::tests::parses_valid_forgejo_config_without_role_authors
        PASS [   0.020s] ( 21/733) atelier-app project_config::tests::forgejo_loader_rejects_conflicting_workflow_role_authors
        PASS [   0.056s] ( 22/733) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.011s] ( 23/733) atelier-app project_config::tests::rejects_committed_runtime_and_compatibility_path_settings
        PASS [   0.061s] ( 24/733) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.011s] ( 25/733) atelier-app project_config::tests::rejects_legacy_top_level_forgejo_config
        PASS [   0.012s] ( 26/733) atelier-app project_config::tests::rejects_mixed_room_and_provider_config
        PASS [   0.198s] ( 27/733) atelier-app export::tests::test_canonical_check_reports_stale_projection_metadata
        PASS [   0.214s] ( 28/733) atelier-app export::tests::test_canonical_issue_type_is_explicit_not_label_derived
        PASS [   0.215s] ( 29/733) atelier-app pr::tests::pr_merge_rejects_missing_and_mismatched_pr_context
        PASS [   0.223s] ( 30/733) atelier-app pr::tests::pr_open_rejects_branch_mismatch_before_remote_create
        PASS [   0.229s] ( 31/733) atelier-app pr::tests::pr_comment_posts_to_linked_pull_and_records_owner_action
        PASS [   0.245s] ( 32/733) atelier-app export::tests::test_canonical_check_reports_dangling_link
        PASS [   0.248s] ( 33/733) atelier-app export::tests::test_canonical_noop_export_is_deterministic
        PASS [
```

