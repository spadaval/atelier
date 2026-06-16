---
created_at: "2026-06-16T16:40:20.706170113+00:00"
id: "atelier-s8fy"
evidence_type: "test"
captured_at: "2026-06-16T16:40:20.706060465+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-89by"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-89by"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Supplemental local-binary proof after workflow parser change: cargo build -p atelier-cli; target/debug/atelier issue close atelier-89by; target/debug/atelier lint atelier-89by; target/debug/atelier export --check; git diff --check."
updated_at: "2026-06-16T16:40:24.219193461+00:00"
---

Supplemental local-binary proof after workflow parser change: cargo build -p atelier-cli; target/debug/atelier issue close atelier-89by; target/debug/atelier lint atelier-89by; target/debug/atelier export --check; git diff --check.
