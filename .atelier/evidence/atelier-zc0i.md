---
created_at: "2026-06-20T19:31:21.284378890+00:00"
id: "atelier-zc0i"
evidence_type: "validation"
captured_at: "2026-06-20T19:31:21.284376929+00:00"
target:
  kind: "issue"
  id: "atelier-n25m"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-n25m"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Extracted shared objective status helpers into crates/atelier-cli/src/commands/objective_status.rs and rewired root status plus mission status helpers to use shared issue traversal, blocker, proof, and ordering logic. Validation passed: cargo fmt -- --check; cargo check -p atelier-cli; cargo test -p atelier-cli --test cli_integration setup_guidance -- --nocapture; target/debug/atelier status; target/debug/atelier mission status atelier-e146; atelier lint atelier-n25m; git diff --check."
updated_at: "2026-06-20T19:31:24.086035458+00:00"
---

Extracted shared objective status helpers into crates/atelier-cli/src/commands/objective_status.rs and rewired root status plus mission status helpers to use shared issue traversal, blocker, proof, and ordering logic. Validation passed: cargo fmt -- --check; cargo check -p atelier-cli; cargo test -p atelier-cli --test cli_integration setup_guidance -- --nocapture; target/debug/atelier status; target/debug/atelier mission status atelier-e146; atelier lint atelier-n25m; git diff --check.
