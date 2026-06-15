---
created_at: "2026-06-15T16:44:35.393801075+00:00"
id: "atelier-ne6i"
evidence_type: "validation"
captured_at: "2026-06-15T16:44:35.393760575+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-4j3k"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4j3k"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Virtual workspace proof: root Cargo.toml has no [package], [lib], or [[bin]]; cargo metadata --no-deps --format-version 1 lists only atelier-app, atelier-core, atelier-records, atelier-workflow, atelier-sqlite, and atelier-cli packages and reports no root package; target/debug/atelier --help still shows the Atelier CLI; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets passed."
updated_at: "2026-06-15T16:44:37.696986449+00:00"
---

Virtual workspace proof: root Cargo.toml has no [package], [lib], or [[bin]]; cargo metadata --no-deps --format-version 1 lists only atelier-app, atelier-core, atelier-records, atelier-workflow, atelier-sqlite, and atelier-cli packages and reports no root package; target/debug/atelier --help still shows the Atelier CLI; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets passed.
