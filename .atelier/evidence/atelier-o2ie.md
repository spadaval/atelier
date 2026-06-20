---
created_at: "2026-06-20T01:34:42.527311960+00:00"
id: "atelier-o2ie"
evidence_type: "validation"
captured_at: "2026-06-20T01:34:34.839572855+00:00"
command: "bash -lc 'cargo test -p atelier-cli --test cli_integration issue_type && cargo test -p atelier-workflow && cargo test -p atelier-app rebuild --lib && cargo fmt -- --check && cargo build -p atelier-cli && target/debug/atelier workflow check && target/debug/atelier lint atelier-b9i4 && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-b9i4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-b9i4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Repo-defined issue types are used across issue create/update, bundle validation, rebuild, lint, and workflow checks"
updated_at: "2026-06-20T01:34:47.047814642+00:00"
---

## Summary

Repo-defined issue types are used across issue create/update, bundle validation, rebuild, lint, and workflow checks

## Command

```console
bash -lc 'cargo test -p atelier-cli --test cli_integration issue_type && cargo test -p atelier-workflow && cargo test -p atelier-app rebuild --lib && cargo fmt -- --check && cargo build -p atelier-cli && target/debug/atelier workflow check && target/debug/atelier lint atelier-b9i4 && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 4191
Truncated: yes

```text

running 6 tests
test issues::test_unregistered_issue_type_reports_configured_values ... ok
test issues::test_removed_issue_type_is_rejected ... ok
test issues::test_bundle_apply_accepts_configured_custom_issue_type ... ok
test mission_projection_worktree::test_issue_type_is_canonical_not_label_derived ... ok
test issues::test_issue_update_issue_type_persists_through_rebuild ... ok
test issues::test_issue_create_update_and_transition_use_custom_issue_type ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 357 filtered out; finished in 0.67s


running 29 tests
test tests::rejects_configured_branch_policy_without_base_branch ... ok
test tests::missing_branch_policy_is_rejected ... ok
test tests::branch_name_for_owner_renders_configured_templates ... ok
test tests::rejects_unknown_inline_validator ... ok
test tests::parses_custom_issue_type_registry ... ok
test tests::rejects_missing_issue_type_coverage ... ok
test tests::rejects_duplicate_transition_action ... ok
test tests::accepts_empty_action_param_object ... ok
test tests::parses_configured_branch_policy ... ok
test tests::parses_valid_policy ... ok
test tests::rejects_invalid_action_params ... ok
test tests::rejects_invalid_status_category ... ok
test tests::rejects_duplicate_issue_type_coverage ... ok
test tests::rejects_legacy_review_artifact_action_identifier ... ok
test tests::validates_review_field_shape ... ok
test tests::rejects_review_action_on_non_review_transition ... ok
test tests::rejects_legacy_pull_request_field_shape ... ok
test tests::rejects_mismatched_review_field_shape ... ok
test tests::rejects_unknown_transition_action ... ok
test tests::rejects_unknown_top_level_field ... ok
test tests::rejects_removed_top_level_fields ... ok
test tests::rejects_invalid_issue_type_name_and_label ... ok
test tests::rejects_legacy_transition_effects_field ... ok
test tests::rejects_missing_issue_type_registry_entry ... ok
test tests::starter_policy_does_not_require_legacy_pr_merge_gate ... ok
test tests::rejects_unknown_issue_type_in_record ... ok
test tests::rejects_invalid_evidence_validator_params ... ok
test tests::parses_forgejo_review_action_params ... ok
test tests::rejects_obsolete_flat_validator_names ... ok

test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 19 tests
test storage_layout::tests::rebuild_temp_database_paths_are_local_atelier_paths ... ok
test rebuild::tests::record_table_rejects_non_canonical_record_kinds ... ok
test rebuild::tests::rebuild_reports_schema_mismatch ... ok
test rebuild::tests::rebuild_reports_path_id_mismatch ... ok
test rebuild::tests::rebuild_reports_malformed_front_matter ... ok
test rebuild::tests::rebuild_reports_dangling_dependency_and_duplicate_link ... ok
test rebuild::tests::rebuild_rejects_global_id_collision_across_record_kinds ... ok
test rebuild::tests::rebuild_rejects_activity_for_missing_issue ... ok
test rebuild::tests::rebuild_rejects_issue_fields_that_violate_workflow_schema ... ok
test rebuild::tests::rebuild_reports_invalid_relation_type ... ok
test rebuild::tests::rebuild_rejects_child_local_pull_request_field ... ok
test rebuild::tests::refresh_projection_rebuilds_without_local_only_state ... ok
test rebuild::tests::rebuild_round_trips_canonical_domain_records ... ok
test rebuild::tests::rebuild_succeeds_without_manifest_or_graph ... ok
test rebuild::tests::rebuild_round_trips_canonical_issue_fields ... ok
test rebuild::tests::rebuild_accepts_issue_activity_sidecars ... ok
test rebuild::tests::rebuild_recreates_canonical_projection_without_local_only_state ... ok
test rebuild::tests::rebuild_allows_parent_records_after_children ... ok
test rebuild::tests::rebuild_round_trips_canonical_issue_state ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 54 filtered out; finished in 0.32s

Workflow Check
==============
Path:           .atelier/workflow.yaml
Policy:         pass
Applicability:  6
Statuses:       7
```

## Stderr

Bytes: 717
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.04s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/atelier_workflow-1c1848f4dab0f01c)
   Doc-tests atelier_workflow
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-91b312857bc4a702)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.92s
```
