---
created_at: "2026-06-18T00:46:40.228514901+00:00"
id: "atelier-x8x3"
evidence_type: "validation"
captured_at: "2026-06-18T00:44:41.712715768+00:00"
command: "bash -lc 'set -euo pipefail\ncargo nextest run\ncargo nextest run --profile extended --run-ignored=only\ncargo fmt -- --check\ntarget/debug/atelier export --check\ntarget/debug/atelier lint\ntarget/debug/atelier doctor\ntarget/debug/atelier workflow check\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-7g43"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7g43"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Typed issue fields epic passes full validation suite"
updated_at: "2026-06-18T00:46:44.306365826+00:00"
---

## Summary

Typed issue fields epic passes full validation suite

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

Exit status: 0

## Stdout

Bytes: 883
Truncated: no

```text
Canonical export is current
State: /root/atelier/.atelier
Lint passed.
Database: /root/atelier/.atelier/runtime/state.db
State: /root/atelier/.atelier
Install health:
  config: ok
  ignored_runtime_paths: ok
Projection rebuild:
  state_dir: ok
  rebuild_ready: ok
  projection_fresh: ok
  tables: issues, labels, dependencies, relations, records, record_labels, record_links, evidence, projection_sources
Cache health:
  cache_dir: missing (optional)
  projection_metadata: ok
Projection database:
  database: ok
  diagnostics: enabled
Compatibility:
  tables: 
Legacy health:
config: ok
database: ok
ignore_rules: ok
projection_fresh: ok
rebuild_ready: ok
Workflow Check
==============
Path:           .atelier/workflow.yaml
Policy:         pass
Issue Types:    6
Statuses:       7
Validators:     7
Workflows:      3
Record Health:  pass
Issues Checked: 608
Docs/Help Drift: clear
```

## Stderr

Bytes: 80763
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
   Compiling atelier-core v0.2.0 (/root/atelier/crates/atelier-core)
   Compiling atelier-workflow v0.2.0 (/root/atelier/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/root/atelier/crates/atelier-records)
   Compiling atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)
   Compiling atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.78s
────────────
 Nextest run ID 9b9a0b86-43bd-49e9-a74c-7564f93d4618 with nextest profile: default
    Starting 683 tests across 9 binaries (68 tests skipped)
        PASS [   0.008s] (  1/683) atelier-app command_storage::tests::access_modes_declare_projection_freshness_policy
        PASS [   0.008s] (  2/683) atelier-app use_cases::tests::evidence_target_parser_requires_kind_id_syntax
        PASS [   0.008s] (  3/683) atelier-app use_cases::tests::use_case_storage_selectors_are_named_for_target_workflows
        PASS [   0.026s] (  4/683) atelier-app tests::app_entrypoint_returns_view_model_without_rendering
        PASS [   0.026s] (  5/683) atelier-app storage_layout::tests::canonical_dir_is_the_atelier_tree
        PASS [   0.026s] (  6/683) atelier-app storage_layout::tests::rebuild_temp_database_paths_are_local_atelier_paths
        PASS [   0.011s] (  7/683) atelier-cli command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.011s] (  8/683) atelier-cli command_surface::tests::expands_slash_command_references
        PASS [   0.010s] (  9/683) atelier-cli command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.223s] ( 10/683) atelier-app export::tests::test_canonical_check_ignores_sqlite_only_canonical_drift
        PASS [   0.227s] ( 11/683) atelier-app rebuild::tests::rebuild_rejects_global_id_collision_across_record_kinds
        PASS [   0.227s] ( 12/683) atelier-app rebuild::tests::record_table_rejects_non_canonical_record_kinds
        PASS [   0.228s] ( 13/683) atelier-app export::tests::test_canonical_issue_type_is_explicit_not_label_derived
        PASS [   0.229s] ( 14/683) atelier-app export::tests::test_canonical_markdown_serialization_stability
        PASS [   0.229s] ( 15/683) atelier-app export::tests::test_canonical_check_reports_dangling_link
        PASS [   0.230s] ( 16/683) atelier-app export::tests::test_canonical_check_reports_stale_projection_metadata
        PASS [   0.232s] ( 17/683) atelier-app export::tests::test_canonical_check_reports_invalid_duplicate_id
        PASS [   0.234s] ( 18/683) atelier-app export::tests::test_canonical_export_removes_stale_record_file
        PASS [   0.235s] ( 19/683) atelier-app rebuild::tests::rebuild_reports_malformed_front_matter
        PASS [   0.237s] ( 20/683) atelier-app export::tests::test_canonical_changed_record_export_rewrites_issue
        PASS [   0.241s] ( 21/683) atelier-app rebuild::tests::rebuild_rejects_issue_fields_that_violate_workflow_schema
        PASS [   0.245s] ( 22/683) atelier-app export::tests::test_canonical_noop_export_is_deterministic
        PASS [   0.224s] ( 23/683) atelier-app workflow_policy::tests::effective_forge_pr_field_rejects_child_duplicate
        PASS [   0.251s] ( 24/683) atelier-app rebuild::tests::rebuild_reports_schema_mismatch
        PASS [   0.229s] ( 25/683) atelier-app workflow_policy::tests::effective_forge_pr_field_inherits_from_nearest_parent_epic
        PASS [   0.252s] ( 26/683) atelier-app rebuild::tests::rebuild_reports_dangling_dependency_and_duplicate_link
        PASS [   0.254s] ( 27/683) atelier-app export::tests::test_canonical_export_preserves_issue_activity_sidecars
        PASS [   0.258s] ( 28/683) atelier-app rebuild::tests::rebuild_reports_path_id_mismatch
        PASS [   0.261s] ( 29/683) atelier-app rebuild::tests::rebuild_rejects_child_local_forge_pr_field
        PASS [   0.011s] ( 30/683) atelier-cli commands::comment::tests::test_validate_known_kinds
        PASS [   0.
```

