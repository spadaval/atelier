---
created_at: "2026-06-15T17:54:09.174328097+00:00"
id: "atelier-hp1t"
evidence_type: "validation"
captured_at: "2026-06-15T17:54:09.174218168+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-v64l"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-v64l"
    role: "validates"
  - kind: "issue"
    id: "atelier-ycmp"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "CLI integration tests split into workflow modules; cargo nextest run, extended ignored nextest, RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets, fuzz cargo check, atelier lint, atelier export --check, atelier doctor, and git diff --check pass."
updated_at: "2026-06-15T17:54:52.105887654+00:00"
---

CLI integration tests split into workflow modules; cargo nextest run, extended ignored nextest, RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets, fuzz cargo check, atelier lint, atelier export --check, atelier doctor, and git diff --check pass.
