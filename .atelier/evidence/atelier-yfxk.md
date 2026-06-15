---
created_at: "2026-06-15T07:29:28.913488269+00:00"
id: "atelier-yfxk"
evidence_type: "validation"
captured_at: "2026-06-15T07:29:28.913388600+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-8wvr"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-8wvr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "blocked"
title: "Records extraction validation: cargo test -p atelier-records passed 39 tests; cargo check -p atelier-records passed; target/debug/atelier lint atelier-8wvr passed; target/debug/atelier export --check reported canonical export current; git diff --check passed. Required cargo check -p atelier-records -p atelier-tracker and cargo fmt -- --check are blocked because crates/atelier-sqlite/src/lib.rs is missing in this shared worktree."
updated_at: "2026-06-15T07:29:32.252061430+00:00"
---

Records extraction validation: cargo test -p atelier-records passed 39 tests; cargo check -p atelier-records passed; target/debug/atelier lint atelier-8wvr passed; target/debug/atelier export --check reported canonical export current; git diff --check passed. Required cargo check -p atelier-records -p atelier-tracker and cargo fmt -- --check are blocked because crates/atelier-sqlite/src/lib.rs is missing in this shared worktree.
