---
created_at: "2026-06-14T07:23:47.244199200+00:00"
id: "atelier-nf3n"
evidence_type: "validation"
captured_at: "2026-06-14T07:23:47.244079155+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-j01c"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-j01c"
    role: "validates"
  - kind: "issue"
    id: "atelier-mxug"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Projection refresh and rebuild are lock-aware and atomic. Proof: cargo test test_concurrent_rebuilds_and_reads_are_serialized --test smoke_tests; cargo test concurrency --test smoke_tests; cargo test test_projection_index_rebuilds_changed_sources_before_issue_queries --test cli_integration; cargo test test_projection_index_rebuilds_dep_list_and_lint_but_ignores_derived_files --test cli_integration; cargo test test_rebuild_temp_files_are_ignored_by_query_lint_export_and_doctor --test cli_integration; cargo test rebuild --lib; cargo fmt -- --check; git diff --check; atelier lint; atelier export --check."
updated_at: "2026-06-14T07:23:52.164539766+00:00"
---

Projection refresh and rebuild are lock-aware and atomic. Proof: cargo test test_concurrent_rebuilds_and_reads_are_serialized --test smoke_tests; cargo test concurrency --test smoke_tests; cargo test test_projection_index_rebuilds_changed_sources_before_issue_queries --test cli_integration; cargo test test_projection_index_rebuilds_dep_list_and_lint_but_ignores_derived_files --test cli_integration; cargo test test_rebuild_temp_files_are_ignored_by_query_lint_export_and_doctor --test cli_integration; cargo test rebuild --lib; cargo fmt -- --check; git diff --check; atelier lint; atelier export --check.
