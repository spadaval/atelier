---
created_at: "2026-06-16T17:32:37.820038408+00:00"
id: "atelier-rp25"
evidence_type: "docs"
captured_at: "2026-06-16T17:32:37.819907379+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-2sut"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2sut"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Defined CLI command categories and export/rebuild boundary. Product-doc search 'rg -n \"atelier export|export --check|canonical export freshness|normal .*export|handoff.*export|validation.*export|health.*export\" docs/product' found only excluded/non-normal or hidden/admin references. Validation passed: target/debug/atelier lint atelier-2sut; target/debug/atelier doctor; git diff --check."
updated_at: "2026-06-16T17:32:41.379741373+00:00"
---

Defined CLI command categories and export/rebuild boundary. Product-doc search 'rg -n "atelier export|export --check|canonical export freshness|normal .*export|handoff.*export|validation.*export|health.*export" docs/product' found only excluded/non-normal or hidden/admin references. Validation passed: target/debug/atelier lint atelier-2sut; target/debug/atelier doctor; git diff --check.
