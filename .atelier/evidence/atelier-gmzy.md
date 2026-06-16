---
created_at: "2026-06-13T23:31:41.274412667+00:00"
id: "atelier-gmzy"
evidence_type: "validation"
captured_at: "2026-06-13T23:31:41.274306682+00:00"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-gzel"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Removed unused direct signal-hook dependency from Cargo.toml/Cargo.lock after source search found no signal_hook imports. cargo machete now reports no unused dependencies. cargo check, cargo build --bin atelier, cargo fmt -- --check, git diff --check, target/debug/atelier lint, and target/debug/atelier export --check pass. cargo nextest run executed 714 tests: 708 passed, 6 failed in pre-existing tracker projection/legacy evidence tests unrelated to the dependency removal."
updated_at: "2026-06-13T23:31:43.239413245+00:00"
---

Removed unused direct signal-hook dependency from Cargo.toml/Cargo.lock after source search found no signal_hook imports. cargo machete now reports no unused dependencies. cargo check, cargo build --bin atelier, cargo fmt -- --check, git diff --check, target/debug/atelier lint, and target/debug/atelier export --check pass. cargo nextest run executed 714 tests: 708 passed, 6 failed in pre-existing tracker projection/legacy evidence tests unrelated to the dependency removal.
