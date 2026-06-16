---
created_at: "2026-06-16T18:24:45.535945559+00:00"
id: "atelier-r7wq"
evidence_type: "validation"
captured_at: "2026-06-16T18:24:44.845939801+00:00"
command: "cargo fmt -- --check"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-f7vd"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 0
    summary: ""
    truncated: false
  stderr:
    bytes: 0
    summary: ""
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-f7vd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Formatting check passes."
updated_at: "2026-06-16T18:24:49.177186501+00:00"
---

Formatting check passes.

Command: cargo fmt -- --check
Exit status: 0

Stdout summary:
(none)

Stderr summary:
(none)

