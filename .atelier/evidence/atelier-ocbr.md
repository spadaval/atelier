---
created_at: "2026-07-16T18:38:48.032347603+00:00"
id: "atelier-ocbr"
evidence_type: "test"
captured_at: "2026-07-16T18:38:42.920803444+00:00"
command: "bash -lc 'cargo test -p atelier-records mission_plan_review -- --nocapture && cargo test -p atelier-app mission_plan_review -- --nocapture && cargo nextest run -p atelier-records -p atelier-app && cargo fmt -- --check && git diff --check'"
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
title: "Mission-plan review repair: all 8 focused provenance/freshness regressions, deterministic rebuild coverage, 154 records/app tests, formatting, and whitespace checks pass."
updated_at: "2026-07-16T18:38:52.154225158+00:00"
---

## Summary

Mission-plan review repair: all 8 focused provenance/freshness regressions, deterministic rebuild coverage, 154 records/app tests, formatting, and whitespace checks pass.

## Command

```console
bash -lc 'cargo test -p atelier-records mission_plan_review -- --nocapture && cargo test -p atelier-app mission_plan_review -- --nocapture && cargo nextest run -p atelier-records -p atelier-app && cargo fmt -- --check && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 1130
Truncated: no

```text

running 8 tests
test mission_plan_review::tests::rejects_whitespace_variant_self_approval ... ok
test mission_plan_review::tests::rejects_provenance_loss_non_independent_approval_and_unresolved_findings ... ok
test mission_plan_review::tests::rejects_malformed_typed_activity_metadata ... ok
test mission_plan_review::tests::every_material_edit_class_changes_revision_but_notes_and_order_do_not ... ok
test mission_plan_review::tests::direct_mission_blocked_by_change_stales_approval ... ok
test mission_plan_review::tests::material_change_stales_approval_and_requires_attribution_for_new_revision ... ok
test mission_plan_review::tests::grandfather_requires_exact_cutover_status_and_migration_provenance ... ok
test mission_plan_review::tests::typed_events_round_trip_and_project_fresh_independent_approval ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 50 filtered out; finished in 0.02s


running 1 test
test rebuild::tests::rebuild_preserves_complete_mission_plan_review_projection_deterministically ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 95 filtered out; finished in 0.11s
```

## Stderr

Bytes: 18756
Truncated: yes

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-7f050eb419aa17f6)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-08ce193fbeb04291)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
────────────
 Nextest run ID a7c2738e-bd3e-4c02-9122-7a1ea1a813f9 with nextest profile: default
    Starting 154 tests across 2 binaries
        PASS [   0.009s] (  1/154) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.010s] (  2/154) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.012s] (  3/154) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.012s] (  4/154) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.012s] (  5/154) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.013s] (  6/154) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.013s] (  7/154) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.017s] (  8/154) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.017s] (  9/154) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.017s] ( 10/154) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.027s] ( 11/154) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.027s] ( 12/154) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.012s] ( 13/154) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.028s] ( 14/154) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.033s] ( 15/154) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.033s] ( 16/154) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.034s] ( 17/154) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.033s] ( 18/154) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.033s] ( 19/154) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.017s] ( 20/154) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.035s] ( 21/154) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.011s] ( 22/154) atelier-app pr::tests::parse_pull_request_reference_accepts_number_and_matching_url
        PASS [   0.010s] ( 23/154) atelier-app pr::tests::parse_review_event_rejects_unknown_values
        PASS [   0.038s] ( 24/154) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.011s] ( 25/154) atelier-app pr::tests::parse_pull_request_reference_rejects_mismatched_url_context
        PASS [   0.030s] ( 26/154) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.033s] ( 27/154) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.015s] ( 28/154) atelier-app project_config::tests::forgejo_loader_rejects_conflicting_workflow_role_authors
        PASS [   0.015s] ( 29/154) atelier-app project_config::tests::forgejo_loader_applies_workflow_role_authors
        PASS [   0.012s] ( 30/154) atelier-app project_config::tests::invalid_forgejo_config_names_and_legacy_role_authors
        PASS [   0.0
```
