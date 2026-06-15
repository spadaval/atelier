---
created_at: "2026-06-15T06:26:16.534157858+00:00"
id: "atelier-1f25"
evidence_type: "validation"
captured_at: "2026-06-15T06:26:16.534050778+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-yyuc"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yyuc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Runtime recovery proof: disposable copy deleted .atelier/runtime and .atelier/cache, target/debug/atelier rebuild recreated .atelier/runtime/state.db, and target/debug/atelier status showed atelier-yyuc as current work from canonical in_progress Markdown. Focused test passed: cargo test --test cli_integration test_status_preserves_current_work_after_runtime_database_rebuild -- --nocapture. Required checks passed: cargo fmt -- --check, target/debug/atelier lint atelier-yyuc, target/debug/atelier export --check, git diff --check."
updated_at: "2026-06-15T06:26:19.591507115+00:00"
---

Runtime recovery proof: disposable copy deleted .atelier/runtime and .atelier/cache, target/debug/atelier rebuild recreated .atelier/runtime/state.db, and target/debug/atelier status showed atelier-yyuc as current work from canonical in_progress Markdown. Focused test passed: cargo test --test cli_integration test_status_preserves_current_work_after_runtime_database_rebuild -- --nocapture. Required checks passed: cargo fmt -- --check, target/debug/atelier lint atelier-yyuc, target/debug/atelier export --check, git diff --check.
