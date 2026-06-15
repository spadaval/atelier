---
created_at: "2026-06-15T16:57:21.297105075+00:00"
id: "atelier-a2w7"
evidence_type: "validation"
captured_at: "2026-06-15T16:57:21.193929831+00:00"
command: "cargo check --manifest-path fuzz/Cargo.toml --bins"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-7vfj"
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
    bytes: 72
    summary: "    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7vfj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Fuzz harnesses compile against atelier-sqlite ProjectionIndex and no longer import atelier::db::Database or atelier::models::Issue; rg old imports in fuzz returned no matches."
updated_at: "2026-06-15T16:57:23.697884214+00:00"
---

Fuzz harnesses compile against atelier-sqlite ProjectionIndex and no longer import atelier::db::Database or atelier::models::Issue; rg old imports in fuzz returned no matches.

Command: cargo check --manifest-path fuzz/Cargo.toml --bins
Exit status: 0

Stdout summary:
(none)

Stderr summary:
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s

