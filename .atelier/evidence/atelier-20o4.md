---
created_at: "2026-06-15T16:44:44.251824748+00:00"
id: "atelier-20o4"
evidence_type: "validation"
captured_at: "2026-06-15T16:44:44.251776841+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-cwgx"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-cwgx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Root test and fuzz relocation proof: integration and smoke tests now live under crates/atelier-cli/tests; fuzz/Cargo.toml points dependency package atelier-cli at ../crates/atelier-cli; cargo check --manifest-path fuzz/Cargo.toml --bins passed after retargeting fuzz harnesses to current internal APIs."
updated_at: "2026-06-15T16:44:46.802114382+00:00"
---

Root test and fuzz relocation proof: integration and smoke tests now live under crates/atelier-cli/tests; fuzz/Cargo.toml points dependency package atelier-cli at ../crates/atelier-cli; cargo check --manifest-path fuzz/Cargo.toml --bins passed after retargeting fuzz harnesses to current internal APIs.
