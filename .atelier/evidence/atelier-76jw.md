---
created_at: "2026-06-18T01:09:51.107556336+00:00"
id: "atelier-76jw"
evidence_type: "validation"
captured_at: "2026-06-18T01:07:56.977173877+00:00"
command: "bash -lc 'set -euo pipefail\ncargo nextest run\ncargo nextest run --profile extended --run-ignored=only\ncargo fmt -- --check\ntarget/debug/atelier export --check\ntarget/debug/atelier lint\ntarget/debug/atelier doctor\ntarget/debug/atelier workflow check\ngit diff --check'"
exit_status: "0"
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
updated_at: "2026-06-18T01:09:54.931941596+00:00"
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

Bytes: 81530
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.14s
────────────
 Nextest run ID d8ec1e3c-60dc-4821-8813-f603a650b338 with nextest profile: default
    Starting 693 tests across 9 binaries (68 tests skipped)
        PASS [   0.011s] (  1/693) atelier-app project_config::tests::parses_valid_forgejo_config_and_sudo_mapping
        PASS [   0.012s] (  2/693) atelier-app command_storage::tests::access_modes_declare_projection_freshness_policy
        PASS [   0.013s] (  3/693) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.015s] (  4/693) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.020s] (  5/693) atelier-app project_config::tests::invalid_forgejo_config_names_missing_role_and_token
        PASS [   0.019s] (  6/693) atelier-app forgejo::tests::lists_review_comments_and_surfaces_api_failures
        PASS [   0.026s] (  7/693) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.012s] (  8/693) atelier-app storage_layout::tests::canonical_dir_is_the_atelier_tree
        PASS [   0.010s] (  9/693) atelier-app storage_layout::tests::rebuild_temp_database_paths_are_local_atelier_paths
        PASS [   0.008s] ( 10/693) atelier-app tests::app_entrypoint_returns_view_model_without_rendering
        PASS [   0.012s] ( 11/693) atelier-app project_config::tests::missing_forgejo_config_is_actionable
        PASS [   0.010s] ( 12/693) atelier-app use_cases::tests::evidence_target_parser_requires_kind_id_syntax
        PASS [   0.009s] ( 13/693) atelier-app use_cases::tests::use_case_storage_selectors_are_named_for_target_workflows
        PASS [   0.010s] ( 14/693) atelier-cli command_surface::tests::expands_slash_command_references
        PASS [   0.014s] ( 15/693) atelier-cli command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.013s] ( 16/693) atelier-cli command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.241s] ( 17/693) atelier-app export::tests::test_canonical_issue_type_is_explicit_not_label_derived
        PASS [   0.241s] ( 18/693) atelier-app rebuild::tests::rebuild_reports_path_id_mismatch
        PASS [   0.254s] ( 19/693) atelier-app export::tests::test_canonical_noop_export_is_deterministic
        PASS [   0.254s] ( 20/693) atelier-app export::tests::test_canonical_export_preserves_issue_activity_sidecars
        PASS [   0.253s] ( 21/693) atelier-app rebuild::tests::rebuild_rejects_global_id_collision_across_record_kinds
        PASS [   0.254s] ( 22/693) atelier-app export::tests::test_canonical_check_reports_stale_projection_metadata
        PASS [   0.256s] ( 23/693) atelier-app export::tests::test_canonical_check_reports_invalid_duplicate_id
        PASS [   0.264s] ( 24/693) atelier-app rebuild::tests::rebuild_rejects_activity_for_missing_issue
        PASS [   0.254s] ( 25/693) atelier-app rebuild::tests::rebuild_reports_schema_mismatch
        PASS [   0.255s] ( 26/693) atelier-app rebuild::tests::rebuild_reports_invalid_relation_type
        PASS [   0.268s] ( 27/693) atelier-app export::tests::test_canonical_check_ignores_sqlite_only_canonical_drift
        PASS [   0.269s] ( 28/693) atelier-app export::tests::test_canonical_check_reports_dangling_link
        PASS [   0.271s] ( 29/693) atelier-app export::tests::test_canonical_markdown_serialization_stability
        PASS [   0.231s] ( 30/693) atelier-app workflow_policy::tests::effective_forge_pr_field_rejects_child_duplicate
        PASS [   0.260s] ( 31/693) atelier-app rebuild::tests::rebuild_reports_malformed_front_matter
        PASS [   0.261s] ( 32/693) atelier-app rebuild::tests::record_table_rejects_non_canonical_record_kinds
        PASS [   0.239s] ( 33/693) atelier-app workflow_policy::tests::effective_forge_pr_field_inherits_from_nearest_parent_epic
        PASS [   0.264s] ( 34/693) atelier-app rebuild::tests::reb
```

