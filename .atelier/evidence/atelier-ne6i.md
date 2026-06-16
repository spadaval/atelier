---
created_at: "2026-06-15T16:44:35.393801075+00:00"
id: "atelier-ne6i"
evidence_type: "validation"
captured_at: "2026-06-15T16:44:35.393760575+00:00"
target:
  kind: "issue"
  id: "atelier-4j3k"
  role: "validates"
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
status: "recorded"
title: "Virtual workspace proof: root Cargo.toml has no [package], [lib], or [[bin]]; cargo metadata --no-deps --format-version 1 lists only atelier-app, atelier-core, atelier-records, atelier-workflow, atelier-sqlite, and atelier-cli packages and reports no root package; target/debug/atelier --help still shows the Atelier CLI; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets passed."
updated_at: "2026-06-15T16:44:37.696986449+00:00"
---

Virtual workspace proof: root Cargo.toml has no [package], [lib], or [[bin]]; cargo metadata --no-deps --format-version 1 lists only atelier-app, atelier-core, atelier-records, atelier-workflow, atelier-sqlite, and atelier-cli packages and reports no root package; target/debug/atelier --help still shows the Atelier CLI; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets passed.
