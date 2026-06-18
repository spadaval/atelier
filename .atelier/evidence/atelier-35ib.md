---
created_at: "2026-06-18T00:25:41.146553268+00:00"
id: "atelier-35ib"
evidence_type: "test"
captured_at: "2026-06-18T00:25:41.146542256+00:00"
target:
  kind: "issue"
  id: "atelier-nmkm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nmkm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo test -p atelier-workflow; cargo check -p atelier-workflow; cargo check -p atelier-cli; cargo fmt -- --check; cargo build -p atelier-cli; target/debug/atelier lint atelier-nmkm; target/debug/atelier workflow check; git diff --check"
updated_at: "2026-06-18T00:25:45.030389525+00:00"
---

cargo test -p atelier-workflow; cargo check -p atelier-workflow; cargo check -p atelier-cli; cargo fmt -- --check; cargo build -p atelier-cli; target/debug/atelier lint atelier-nmkm; target/debug/atelier workflow check; git diff --check
