---
created_at: "2026-06-18T01:04:20.994223724+00:00"
id: "atelier-07ek"
evidence_type: "validation"
captured_at: "2026-06-18T01:03:42.584507977+00:00"
command: "bash -lc 'set -euo pipefail\ncargo nextest run\ncargo nextest run --profile extended --run-ignored=only\ncargo fmt -- --check\ntarget/debug/atelier export --check\ntarget/debug/atelier lint\ntarget/debug/atelier doctor\ntarget/debug/atelier workflow check\ngit diff --check'"
exit_status: "100"
target:
  kind: "issue"
  id: "atelier-c5oz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-c5oz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Forgejo PR integration epic passes validation suite"
updated_at: "2026-06-18T01:04:24.979666347+00:00"
---

## Summary

Forgejo PR integration epic passes validation suite

## Command

```console
bash -lc 'set -euo pipefail
cargo nextest run
cargo nextest run --profile extended --run-ignored=only
cargo fmt -- --check
target/debug/atelier export --check
target/debug/atelier lint
target/debug/atelier doctor
target/debug/atelier workflow check
git diff --check'
```

Exit status: 100

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 85199
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
   Compiling atelier-core v0.2.0 (/root/atelier/crates/atelier-core)
   Compiling atelier-workflow v0.2.0 (/root/atelier/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/root/atelier/crates/atelier-records)
   Compiling atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)
   Compiling atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.88s
────────────
 Nextest run ID 277c8ce2-f78e-4023-bbc6-c6c79c216461 with nextest profile: default
    Starting 693 tests across 9 binaries (68 tests skipped)
        PASS [   0.014s] (  1/693) atelier-app command_storage::tests::access_modes_declare_projection_freshness_policy
        PASS [   0.009s] (  2/693) atelier-app project_config::tests::missing_forgejo_config_is_actionable
        PASS [   0.026s] (  3/693) atelier-app forgejo::tests::lists_review_comments_and_surfaces_api_failures
        PASS [   0.026s] (  4/693) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.011s] (  5/693) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.010s] (  6/693) atelier-app storage_layout::tests::canonical_dir_is_the_atelier_tree
        PASS [   0.021s] (  7/693) atelier-app project_config::tests::invalid_forgejo_config_names_missing_role_and_token
        PASS [   0.024s] (  8/693) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.025s] (  9/693) atelier-app project_config::tests::parses_valid_forgejo_config_and_sudo_mapping
        PASS [   0.008s] ( 10/693) atelier-app storage_layout::tests::rebuild_temp_database_paths_are_local_atelier_paths
        PASS [   0.009s] ( 11/693) atelier-app tests::app_entrypoint_returns_view_model_without_rendering
        PASS [   0.010s] ( 12/693) atelier-app use_cases::tests::use_case_storage_selectors_are_named_for_target_workflows
        PASS [   0.011s] ( 13/693) atelier-app use_cases::tests::evidence_target_parser_requires_kind_id_syntax
        PASS [   0.009s] ( 14/693) atelier-cli command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.017s] ( 15/693) atelier-cli command_surface::tests::expands_slash_command_references
        PASS [   0.012s] ( 16/693) atelier-cli command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.225s] ( 17/693) atelier-app rebuild::tests::rebuild_reports_schema_mismatch
        PASS [   0.226s] ( 18/693) atelier-app rebuild::tests::rebuild_reports_path_id_mismatch
        PASS [   0.230s] ( 19/693) atelier-app rebuild::tests::rebuild_reports_dangling_dependency_and_duplicate_link
        PASS [   0.233s] ( 20/693) atelier-app rebuild::tests::rebuild_rejects_global_id_collision_across_record_kinds
        PASS [   0.239s] ( 21/693) atelier-app rebuild::tests::rebuild_rejects_issue_fields_that_violate_workflow_schema
        PASS [   0.239s] ( 22/693) atelier-app export::tests::test_canonical_noop_export_is_deterministic
        PASS [   0.246s] ( 23/693) atelier-app export::tests::test_canonical_check_reports_stale_projection_metadata
        PASS [   0.248s] ( 24/693) atelier-app export::tests::test_canonical_issue_type_is_explicit_not_label_derived
        PASS [   0.249s] ( 25/693) atelier-app rebuild::tests::rebuild_rejects_child_local_forge_pr_field
        PASS [   0.250s] ( 26/693) atelier-app export::tests::test_canonical_check_reports_dangling_link
        PASS [   0.233s] ( 27/693) atelier-app rebuild::tests::rebuild_rejects_activity_for_missing_issue
        PASS [   0.252s] ( 28/693) atelier-app rebuild::tests::rebuild_reports_malformed_front_matter
        PASS [   0.259s] ( 29/693) atelier-app export::tests::test_canonical_check_ignores_sqlite_only_canonical_drift
        PASS [   0.236s] ( 30/693) atelier-app rebuild::tests::record_table_rejects_non_canonical_record_kinds
        PASS [   0.264s] ( 31/693) atelier-app expo
```

