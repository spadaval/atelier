---
created_at: "2026-06-15T07:08:35.101672884+00:00"
id: "atelier-jll4"
evidence_type: "validation"
captured_at: "2026-06-15T07:08:35.101556380+00:00"
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
title: "Workspace scaffold proof: cargo metadata --no-deps --format-version 1 lists atelier-core, atelier-workflow, atelier-records, atelier-sqlite, atelier-app, and atelier-cli as workspace members alongside the transitional root package; cargo fmt -- --check passed; cargo check --workspace passed with existing root dead-code warnings only; target/debug/atelier --help prints the atelier CLI help; target/debug/atelier lint atelier-wz3t passed; target/debug/atelier export --check passed; git diff --check passed."
updated_at: "2026-06-15T07:08:38.386258565+00:00"
---

Workspace scaffold proof: cargo metadata --no-deps --format-version 1 lists atelier-core, atelier-workflow, atelier-records, atelier-sqlite, atelier-app, and atelier-cli as workspace members alongside the transitional root package; cargo fmt -- --check passed; cargo check --workspace passed with existing root dead-code warnings only; target/debug/atelier --help prints the atelier CLI help; target/debug/atelier lint atelier-wz3t passed; target/debug/atelier export --check passed; git diff --check passed.
