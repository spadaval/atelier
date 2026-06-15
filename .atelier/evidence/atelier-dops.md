---
created_at: "2026-06-14T07:13:33.223879223+00:00"
id: "atelier-dops"
evidence_type: "validation"
captured_at: "2026-06-14T07:13:33.223836072+00:00"
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
title: "Validation passed for projection auto-refresh and rebuild locking: cargo test test_projection_index_rebuilds_changed_sources_before_issue_queries --test cli_integration; cargo test test_projection_index_rebuilds_dep_list_and_lint_but_ignores_derived_files --test cli_integration; cargo test concurrency --test smoke_tests; cargo fmt -- --check; git diff --check; atelier lint; atelier export --check."
updated_at: "2026-06-14T07:13:41.969729205+00:00"
---

Validation passed for projection auto-refresh and rebuild locking: cargo test test_projection_index_rebuilds_changed_sources_before_issue_queries --test cli_integration; cargo test test_projection_index_rebuilds_dep_list_and_lint_but_ignores_derived_files --test cli_integration; cargo test concurrency --test smoke_tests; cargo fmt -- --check; git diff --check; atelier lint; atelier export --check.
