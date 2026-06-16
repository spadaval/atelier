---
created_at: "2026-06-16T18:37:15.335563510+00:00"
id: "atelier-6zcz"
evidence_type: "validation"
captured_at: "2026-06-16T18:37:14.574794054+00:00"
command: "target/debug/atelier lint atelier-qh52"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-qh52"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 13
    summary: "Lint passed.\n"
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
    id: "atelier-qh52"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Tracker lint for graph tree ordering issue passes"
updated_at: "2026-06-16T18:37:19.248911355+00:00"
---

Tracker lint for graph tree ordering issue passes

Command: target/debug/atelier lint atelier-qh52
Exit status: 0

Stdout summary:
Lint passed.

Stderr summary:
(none)

