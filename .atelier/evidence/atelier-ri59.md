---
created_at: "2026-07-16T19:09:45.234012450+00:00"
id: "atelier-ri59"
evidence_type: "test"
captured_at: "2026-07-16T19:09:32.358474422+00:00"
command: "bash -lc 'cargo test -p atelier-records mission_plan_review -- --nocapture && cargo test -p atelier-app rebuild_preserves -- --nocapture && cargo nextest run -p atelier-records -p atelier-app && cargo fmt -- --check && cargo check -p atelier-cli && cargo clippy -p atelier-records --tests --no-deps -- -D warnings -A clippy::too-many-arguments -A clippy::single-match -A clippy::unnecessary-lazy-evaluations && git diff --check'"
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
title: "Re-review repair: actor-v1 identities reject U+200B/U+200D and non-NFC aliases; manifest-bound grandfather receipts reject missing, forged, late, post-cutover, duplicate, and wrong-mission events; focused, rebuild, broad, format, CLI, and scoped Clippy checks pass."
updated_at: "2026-07-16T19:09:45.236643339+00:00"
---

## Summary

Re-review repair: actor-v1 identities reject U+200B/U+200D and non-NFC aliases; manifest-bound grandfather receipts reject missing, forged, late, post-cutover, duplicate, and wrong-mission events; focused, rebuild, broad, format, CLI, and scoped Clippy checks pass.

## Command

```console
bash -lc 'cargo test -p atelier-records mission_plan_review -- --nocapture && cargo test -p atelier-app rebuild_preserves -- --nocapture && cargo nextest run -p atelier-records -p atelier-app && cargo fmt -- --check && cargo check -p atelier-cli && cargo clippy -p atelier-records --tests --no-deps -- -D warnings -A clippy::too-many-arguments -A clippy::single-match -A clippy::unnecessary-lazy-evaluations && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 1450
Truncated: no

```text

running 10 tests
test mission_plan_review::tests::rejects_malformed_typed_activity_metadata ... ok
test mission_plan_review::tests::rejects_default_ignorable_and_non_nfc_actor_aliases_before_independence ... ok
test mission_plan_review::tests::rejects_whitespace_variant_self_approval ... ok
test mission_plan_review::tests::rejects_provenance_loss_non_independent_approval_and_unresolved_findings ... ok
test mission_plan_review::tests::rejects_forged_late_duplicate_and_wrong_mission_grandfather_receipts ... ok
test mission_plan_review::tests::every_material_edit_class_changes_revision_but_notes_and_order_do_not ... ok
test mission_plan_review::tests::material_change_stales_approval_and_requires_attribution_for_new_revision ... ok
test mission_plan_review::tests::direct_mission_blocked_by_change_stales_approval ... ok
test mission_plan_review::tests::grandfather_requires_exact_cutover_status_and_migration_provenance ... ok
test mission_plan_review::tests::typed_events_round_trip_and_project_fresh_independent_approval ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 50 filtered out; finished in 0.02s


running 2 tests
test rebuild::tests::rebuild_preserves_exactly_once_legacy_cutover_manifest_and_receipt ... ok
test rebuild::tests::rebuild_preserves_complete_mission_plan_review_projection_deterministically ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 109 filtered out; finished in 0.14s
```

## Stderr

Bytes: 22118
Truncated: yes

```text
   Compiling atelier-records v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-records)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.24s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-1337ea3d562087a1)
   Compiling atelier-sqlite v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-sqlite)
   Compiling atelier-app v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.53s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-45b1180188299181)
   Compiling atelier-records v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-records)
   Compiling atelier-app v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.30s
────────────
 Nextest run ID 1e32cb40-6b30-44a7-8d44-e4a2175fb3c0 with nextest profile: default
    Starting 171 tests across 2 binaries
        PASS [   0.009s] (  1/171) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.009s] (  2/171) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.009s] (  3/171) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.010s] (  4/171) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.010s] (  5/171) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.011s] (  6/171) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.012s] (  7/171) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.011s] (  8/171) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.012s] (  9/171) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.011s] ( 10/171) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.024s] ( 11/171) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.011s] ( 12/171) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.024s] ( 13/171) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.025s] ( 14/171) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.025s] ( 15/171) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.025s] ( 16/171) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.026s] ( 17/171) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.026s] ( 18/171) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.026s] ( 19/171) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.017s] ( 20/171) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.013s] ( 21/171) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.026s] ( 22/171) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.026s] ( 23/171) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.017s] ( 24/171) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.027s] ( 25/171) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.028s] ( 26/171) atelier-app forgejo::tests::lists_top_level_pull_comments
        PA
```
