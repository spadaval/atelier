---
created_at: "2026-06-15T18:33:18.275395327+00:00"
id: "atelier-86tq"
evidence_type: "validation"
captured_at: "2026-06-15T18:33:17.798661449+00:00"
command: "cargo test -p atelier-app -- --nocapture"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-4ren"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 2892
    summary: "\nrunning 36 tests\ntest command_storage::tests::access_modes_declare_projection_freshness_policy ... ok\ntest tests::app_entrypoint_returns_view_model_without_rendering ... ok\ntest storage_layout::tests::rebuild_temp_database_paths_are_local_atelier_paths ... ok\ntest storage_layout::tests::canonical_dir_is_the_atelier_tree ... ok\ntest workflow_policy::tests::rejects_unknown_top_level_field ... ok\ntest workflow_policy::tests::rejects_invalid_status_category ... ok\ntest workflow_policy::tests::rejects_unknown_template_variable ... ok\ntest workflow_policy::tests::rejects_missing_issue_type_mapping ... ok\ntest workflow_policy::tests::rejects_unknown_validator_reference ... ok\ntest workflow_policy::tests::rejects_invalid_evidence_validator_params ... ok\ntest workflow_policy::tests::parses_valid_policy ... ok\ntest rebuild::tests::record_table_rejects_non_canonical_record_kinds ... ok\ntest export::tests::test_canonical_noop_export_is_deterministic ... ok\ntest rebuild::tests::rebuild_reports_dangling_dependency_and_duplicate_link ... ok\ntest export::tests::test_canonical_issue_type_is_explicit_not_label_derived ... ok\ntest rebuild::tests::rebuild_reports_path_id_mismatch ... ok\ntest rebuild::tests::rebuild_rejects_activity_for_missing_issue ... ok\ntest export::tests::test_canonical_check_reports_stale_projection_metadata ... ok\ntest export::tests::test_canonical_check_ignores_sqlite_only_canonical_drift ... ok\ntest rebuild::tests::rebuild_reports_malformed_front_matter ... ok\ntest export::tests::test_canonical_export_removes_stale_record_file ... ok\ntest rebuild::tests::rebuild_reports_schema_mismatch ... ok\ntest export::tests::test_canonical_check_reports_invalid_duplicate_id ... ok\ntest rebuild::tests::rebuild_reports_invalid_relation_type ... ok\ntest export::tests::test_canonical_check_reports_dangling_link ... ok\ntest export::tests::test_canonical_markdown_serialization_stability ... ok\ntest export::tests::test_canonical_changed_record_export_rewrites_issue ... ok\ntest export::tests::test_canonical_export_preserves_issue_activity_sidecars ... ok\ntest rebuild::tests::rebuild_rejects_global_id_collision_across_record_kinds ... ok\ntest rebuild::tests::rebuild_recreates_canonical_projection_without_runtime_state ... ok\ntest rebuild::tests::rebuild_succeeds_without_manifest_or_graph ... ok\ntest rebuild::tests::rebuild_accepts_issue_activity_sidecars ... ok\ntest rebuild::tests::refresh_projection_rebuilds_without_runtime_state ... ok\ntest rebuild::tests::rebuild_allows_parent_records_after_children ... ok\ntest rebuild::tests::rebuild_round_trips_canonical_domain_records ... ok\ntest rebuild::tests::rebuild_round_trips_canonical_issue_state ... ok\n\ntest result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n"
    truncated: false
  stderr:
    bytes: 181
    summary: "    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s\n     Running unittests src/lib.rs (target/debug/deps/atelier_app-b996d9a0f399a6f5)\n   Doc-tests atelier_app\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4ren"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "atelier-app now owns storage/export/rebuild/workflow APIs with no direct stdout/stderr; CLI export renders app view model"
updated_at: "2026-06-15T18:33:21.753763509+00:00"
---

atelier-app now owns storage/export/rebuild/workflow APIs with no direct stdout/stderr; CLI export renders app view model

Command: cargo test -p atelier-app -- --nocapture
Exit status: 0

Stdout summary:

running 36 tests
test command_storage::tests::access_modes_declare_projection_freshness_policy ... ok
test tests::app_entrypoint_returns_view_model_without_rendering ... ok
test storage_layout::tests::rebuild_temp_database_paths_are_local_atelier_paths ... ok
test storage_layout::tests::canonical_dir_is_the_atelier_tree ... ok
test workflow_policy::tests::rejects_unknown_top_level_field ... ok
test workflow_policy::tests::rejects_invalid_status_category ... ok
test workflow_policy::tests::rejects_unknown_template_variable ... ok
test workflow_policy::tests::rejects_missing_issue_type_mapping ... ok
test workflow_policy::tests::rejects_unknown_validator_reference ... ok
test workflow_policy::tests::rejects_invalid_evidence_validator_params ... ok
test workflow_policy::tests::parses_valid_policy ... ok
test rebuild::tests::record_table_rejects_non_canonical_record_kinds ... ok
test export::tests::test_canonical_noop_export_is_deterministic ... ok
test rebuild::tests::rebuild_reports_dangling_dependency_and_duplicate_link ... ok
test export::tests::test_canonical_issue_type_is_explicit_not_label_derived ... ok
test rebuild::tests::rebuild_reports_path_id_mismatch ... ok
test rebuild::tests::rebuild_rejects_activity_for_missing_issue ... ok
test export::tests::test_canonical_check_reports_stale_projection_metadata ... ok
test export::tests::test_canonical_check_ignores_sqlite_only_canonical_drift ... ok
test rebuild::tests::rebuild_reports_malformed_front_matter ... ok
test export::tests::test_canonical_export_removes_stale_record_file ... ok
test rebuild::tests::rebuild_reports_schema_mismatch ... ok
test export::tests::test_canonical_check_reports_invalid_duplicate_id ... ok
test rebuild::tests::rebuild_reports_invalid_relation_type ... ok
test export::tests::test_canonical_check_reports_dangling_link ... ok
test export::tests::test_canonical_markdown_serialization_stability ... ok
test export::tests::test_canonical_changed_record_export_rewrites_issue ... ok
test export::tests::test_canonical_export_preserves_issue_activity_sidecars ... ok
test rebuild::tests::rebuild_rejects_global_id_collision_across_record_kinds ... ok
test rebuild::tests::rebuild_recreates_canonical_projection_without_runtime_state ... ok
test rebuild::tests::rebuild_succeeds_without_manifest_or_graph ... ok
test rebuild::tests::rebuild_accepts_issue_activity_sidecars ... ok
test rebuild::tests::refresh_projection_rebuilds_without_runtime_state ... ok
test rebuild::tests::rebuild_allows_parent_records_after_children ... ok
test rebuild::tests::rebuild_round_trips_canonical_domain_records ... ok
test rebuild::tests::rebuild_round_trips_canonical_issue_state ... ok

test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Stderr summary:
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-b996d9a0f399a6f5)
   Doc-tests atelier_app

