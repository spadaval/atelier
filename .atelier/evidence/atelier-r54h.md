---
created_at: "2026-07-16T19:21:12.952273452+00:00"
id: "atelier-r54h"
evidence_type: "test"
captured_at: "2026-07-16T19:21:01.505045988+00:00"
command: "bash -lc 'cargo test -p atelier-records rejects_sub_microsecond_timestamps_before_emission_or_load && cargo test -p atelier-records cutover_manifest_rejects_sub_microsecond_precision_before_receipt_hashing && cargo test -p atelier-records rejects_grandfather_receipt_with_wrong_revision_or_legacy_status && cargo test -p atelier-app rebuild_rejects_noncanonical_cutover_and_malformed_receipt_bindings && cargo test -p atelier-app rebuild_preserves_complete_mission_plan_review_projection_deterministically && cargo test -p atelier-app rebuild_preserves_exactly_once_legacy_cutover_manifest_and_receipt && cargo nextest run -p atelier-records -p atelier-app && cargo fmt -- --check && git diff --check && cargo check -p atelier-cli && cargo clippy -p atelier-records --tests --no-deps -- -D warnings -A clippy::too-many-arguments -A clippy::single-match -A clippy::unnecessary-lazy-evaluations'"
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
title: "bash -lc 'cargo test -p atelier-records rejects_sub_microsecond_timestamps_before_emission_or_load && cargo test -p atelier-records cutover_manifest_rejects_sub_microsecond_precision_before_receipt_hashing && cargo test -p atelier-records rejects_grandfather_receipt_with_wrong_revision_or_legacy_status && cargo test -p atelier-app rebuild_rejects_noncanonical_cutover_and_malformed_receipt_bindings && cargo test -p atelier-app rebuild_preserves_complete_mission_plan_review_projection_deterministically && cargo test -p atelier-app rebuild_preserves_exactly_once_legacy_cutover_manifest_and_receipt && cargo nextest run -p atelier-records -p atelier-app && cargo fmt -- --check && git diff --check && cargo check -p atelier-cli && cargo clippy -p atelier-records --tests --no-deps -- -D warnings -A clippy::too-many-arguments -A clippy::single-match -A clippy::unnecessary-lazy-evaluations'"
updated_at: "2026-07-16T19:21:12.955154323+00:00"
---

## Summary

bash -lc 'cargo test -p atelier-records rejects_sub_microsecond_timestamps_before_emission_or_load && cargo test -p atelier-records cutover_manifest_rejects_sub_microsecond_precision_before_receipt_hashing && cargo test -p atelier-records rejects_grandfather_receipt_with_wrong_revision_or_legacy_status && cargo test -p atelier-app rebuild_rejects_noncanonical_cutover_and_malformed_receipt_bindings && cargo test -p atelier-app rebuild_preserves_complete_mission_plan_review_projection_deterministically && cargo test -p atelier-app rebuild_preserves_exactly_once_legacy_cutover_manifest_and_receipt && cargo nextest run -p atelier-records -p atelier-app && cargo fmt -- --check && git diff --check && cargo check -p atelier-cli && cargo clippy -p atelier-records --tests --no-deps -- -D warnings -A clippy::too-many-arguments -A clippy::single-match -A clippy::unnecessary-lazy-evaluations'

## Command

```console
bash -lc 'cargo test -p atelier-records rejects_sub_microsecond_timestamps_before_emission_or_load && cargo test -p atelier-records cutover_manifest_rejects_sub_microsecond_precision_before_receipt_hashing && cargo test -p atelier-records rejects_grandfather_receipt_with_wrong_revision_or_legacy_status && cargo test -p atelier-app rebuild_rejects_noncanonical_cutover_and_malformed_receipt_bindings && cargo test -p atelier-app rebuild_preserves_complete_mission_plan_review_projection_deterministically && cargo test -p atelier-app rebuild_preserves_exactly_once_legacy_cutover_manifest_and_receipt && cargo nextest run -p atelier-records -p atelier-app && cargo fmt -- --check && git diff --check && cargo check -p atelier-cli && cargo clippy -p atelier-records --tests --no-deps -- -D warnings -A clippy::too-many-arguments -A clippy::single-match -A clippy::unnecessary-lazy-evaluations'
```
Exit status: 0

## Stdout

Bytes: 1283
Truncated: no

```text

running 1 test
test activity::tests::rejects_sub_microsecond_timestamps_before_emission_or_load ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 62 filtered out; finished in 0.00s


running 1 test
test mission_plan_review::tests::cutover_manifest_rejects_sub_microsecond_precision_before_receipt_hashing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 62 filtered out; finished in 0.00s


running 1 test
test mission_plan_review::tests::rejects_grandfather_receipt_with_wrong_revision_or_legacy_status ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 62 filtered out; finished in 0.00s


running 1 test
test rebuild::tests::rebuild_rejects_noncanonical_cutover_and_malformed_receipt_bindings ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 111 filtered out; finished in 0.02s


running 1 test
test rebuild::tests::rebuild_preserves_complete_mission_plan_review_projection_deterministically ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 111 filtered out; finished in 0.12s


running 1 test
test rebuild::tests::rebuild_preserves_exactly_once_legacy_cutover_manifest_and_receipt ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 111 filtered out; finished in 0.09s
```

## Stderr

Bytes: 23207
Truncated: yes

```text
   Compiling atelier-records v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-records)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.92s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-1337ea3d562087a1)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-1337ea3d562087a1)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-1337ea3d562087a1)
   Compiling atelier-sqlite v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-sqlite)
   Compiling atelier-app v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.56s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-45b1180188299181)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-45b1180188299181)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-45b1180188299181)
   Compiling atelier-records v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-records)
   Compiling atelier-app v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.91s
────────────
 Nextest run ID 894c2194-1eb3-4a36-a15f-4c364059207b with nextest profile: default
    Starting 175 tests across 2 binaries
        PASS [   0.009s] (  1/175) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.010s] (  2/175) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.011s] (  3/175) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.011s] (  4/175) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.011s] (  5/175) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.012s] (  6/175) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.013s] (  7/175) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.021s] (  8/175) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.021s] (  9/175) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.021s] ( 10/175) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.021s] ( 11/175) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.021s] ( 12/175) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.013s] ( 13/175) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.022s] ( 14/175) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.012s] ( 15/175) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.023s] ( 16/175) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.014s] ( 17/175) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.012s] ( 18/175) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.023s] ( 19/175) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.023s] ( 20/175) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.023s] ( 21/175) atelier-app health::tests::review_backend_health_reports_missing_role_au
```
