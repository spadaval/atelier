---
created_at: "2026-06-14T17:26:14.236586832+00:00"
id: "atelier-inhy"
evidence_type: "validation"
captured_at: "2026-06-14T17:26:14.236464492+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-xq7i"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xq7i"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Command-surface visibility validation passed: cargo test --test cli_integration test_workflow_check_ -- --nocapture ran 10 tests covering visible, hidden, advanced, removal-history, and nonexistent option cases; target/debug/atelier export --check, git diff --check, and target/debug/atelier lint passed."
updated_at: "2026-06-14T17:26:16.949258137+00:00"
---

Command-surface visibility validation passed: cargo test --test cli_integration test_workflow_check_ -- --nocapture ran 10 tests covering visible, hidden, advanced, removal-history, and nonexistent option cases; target/debug/atelier export --check, git diff --check, and target/debug/atelier lint passed.
