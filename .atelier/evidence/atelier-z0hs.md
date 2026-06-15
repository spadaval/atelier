---
created_at: "2026-06-15T07:08:05.427774786+00:00"
id: "atelier-z0hs"
evidence_type: "validation"
captured_at: "2026-06-15T07:08:05.427658724+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-5dgb"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5dgb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "docs/architecture/markdown-first-record-store.md now defines the target ProjectionIndex and RuntimeState SQLite schema: one .atelier/runtime/state.db, rebuildable projection tables, local runtime tables, no old-schema compatibility shims, no active-work/claim source-of-truth tables, and a review artifact mapping old table responsibilities to new ownership. Validation passed: target/debug/atelier lint atelier-5dgb; target/debug/atelier export --check; git diff --check."
updated_at: "2026-06-15T07:08:08.625049269+00:00"
---

docs/architecture/markdown-first-record-store.md now defines the target ProjectionIndex and RuntimeState SQLite schema: one .atelier/runtime/state.db, rebuildable projection tables, local runtime tables, no old-schema compatibility shims, no active-work/claim source-of-truth tables, and a review artifact mapping old table responsibilities to new ownership. Validation passed: target/debug/atelier lint atelier-5dgb; target/debug/atelier export --check; git diff --check.
