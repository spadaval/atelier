---
created_at: "2026-06-15T15:39:11.003753537+00:00"
id: "atelier-zry7"
evidence_type: "validation"
captured_at: "2026-06-15T15:39:11.003720114+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-qsib"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qsib"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "blocked"
title: "Crate migration guard script and validation docs were added. Self-test passes and live guard correctly fails on the current root package paths, but the issue cannot close yet because RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets fails on existing unused-code/test warnings before root deletion."
updated_at: "2026-06-15T15:39:13.366370519+00:00"
---

Crate migration guard script and validation docs were added. Self-test passes and live guard correctly fails on the current root package paths, but the issue cannot close yet because RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets fails on existing unused-code/test warnings before root deletion.
