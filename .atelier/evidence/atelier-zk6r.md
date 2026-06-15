---
created_at: "2026-06-15T15:45:34.525161610+00:00"
id: "atelier-zk6r"
evidence_type: "test"
captured_at: "2026-06-15T15:45:34.525137063+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-c0f1"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-c0f1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Removed issue create --work parser/input and session assignment path. Fresh target/debug/atelier issue create --help has no --work or session wording; target/debug/atelier issue create 'Work path issue' --work rejects the flag; cargo test --test cli_integration test_create_issue_rejects_work_flag -- --exact passed; atelier lint atelier-c0f1, atelier export --check, and git diff --check passed."
updated_at: "2026-06-15T15:45:36.657386196+00:00"
---

Removed issue create --work parser/input and session assignment path. Fresh target/debug/atelier issue create --help has no --work or session wording; target/debug/atelier issue create 'Work path issue' --work rejects the flag; cargo test --test cli_integration test_create_issue_rejects_work_flag -- --exact passed; atelier lint atelier-c0f1, atelier export --check, and git diff --check passed.
