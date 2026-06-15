---
created_at: "2026-06-15T16:49:21.081973844+00:00"
id: "atelier-njtn"
evidence_type: "validation"
captured_at: "2026-06-15T16:49:21.081925469+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-uz8g"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-uz8g"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Crate-level test proof: atelier-core, atelier-workflow, atelier-records, atelier-sqlite, and atelier-app now each have local boundary tests; cargo nextest run executed the new crate tests successfully before failing on unrelated stale CLI/smoke expectations for removed commands and active-work semantics; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets and cargo check --manifest-path fuzz/Cargo.toml --bins passed."
updated_at: "2026-06-15T16:49:23.222669799+00:00"
---

Crate-level test proof: atelier-core, atelier-workflow, atelier-records, atelier-sqlite, and atelier-app now each have local boundary tests; cargo nextest run executed the new crate tests successfully before failing on unrelated stale CLI/smoke expectations for removed commands and active-work semantics; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets and cargo check --manifest-path fuzz/Cargo.toml --bins passed.
