---
created_at: "2026-06-15T16:35:38.088281267+00:00"
id: "atelier-3ha2"
evidence_type: "validation"
captured_at: "2026-06-15T16:35:38.088233682+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-wz3t"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-wz3t"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Workspace scaffold proof: Cargo.toml now defines workspace members '.', crates/atelier-core, crates/atelier-workflow, crates/atelier-records, crates/atelier-sqlite, crates/atelier-app, and crates/atelier-cli; cargo metadata --no-deps --format-version 1 lists those workspace members; cargo fmt -- --check passed; cargo check --workspace passed with pre-existing root warning debt; target/debug/atelier --help still shows the existing Atelier CLI help."
updated_at: "2026-06-15T16:35:40.116072717+00:00"
---

Workspace scaffold proof: Cargo.toml now defines workspace members '.', crates/atelier-core, crates/atelier-workflow, crates/atelier-records, crates/atelier-sqlite, crates/atelier-app, and crates/atelier-cli; cargo metadata --no-deps --format-version 1 lists those workspace members; cargo fmt -- --check passed; cargo check --workspace passed with pre-existing root warning debt; target/debug/atelier --help still shows the existing Atelier CLI help.
