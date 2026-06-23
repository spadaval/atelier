---
created_at: "2026-06-23T23:11:16.377730292+00:00"
id: "atelier-vgoy"
evidence_type: "validation"
captured_at: "2026-06-23T23:11:16.377717690+00:00"
target:
  kind: "issue"
  id: "atelier-4wmp"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4wmp"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo fmt -- --check; cargo check -p atelier-cli; cargo nextest run -p atelier-cli (471 passed, 2 slow); cargo build -p atelier-cli; target/debug/atelier lint atelier-c0qc; target/debug/atelier export --check; git diff --check"
updated_at: "2026-06-23T23:11:21.644824079+00:00"
---

cargo fmt -- --check; cargo check -p atelier-cli; cargo nextest run -p atelier-cli (471 passed, 2 slow); cargo build -p atelier-cli; target/debug/atelier lint atelier-c0qc; target/debug/atelier export --check; git diff --check
