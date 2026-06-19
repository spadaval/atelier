---
created_at: "2026-06-17T23:19:02.445047292+00:00"
id: "atelier-4z2d"
evidence_type: "validation"
captured_at: "2026-06-17T23:19:02.445039097+00:00"
target:
  kind: "issue"
  id: "atelier-3h90"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3h90"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Unified priority handling through atelier_core::IssuePriority. Canonical Markdown writes and parses P0-P3 tokens; runtime/projection values remain human labels; CLI validation, SQLite filters, Beads import mapping, next scoring, work-order sorting, and Agent Factory ranking delegate to the shared API. Search transcript showed remaining priority wrappers are aliases or delegating functions, not independent vocabularies/rank tables. Validation passed: cargo test -p atelier-core; cargo test -p atelier-records; cargo test -p atelier-sqlite; cargo test -p atelier-cli priority; cargo test -p atelier-cli commands::import::tests; cargo test -p atelier-cli mission_projection_worktree::test_lint_has_stable_diagnostics_for_hard_invalid_markdown_records; cargo fmt -- --check; target/debug/atelier lint atelier-3h90; git diff --check; cargo build -p atelier-cli."
updated_at: "2026-06-17T23:19:06.521153928+00:00"
---

Unified priority handling through atelier_core::IssuePriority. Canonical Markdown writes and parses P0-P3 tokens; runtime/projection values remain human labels; CLI validation, SQLite filters, Beads import mapping, next scoring, work-order sorting, and Agent Factory ranking delegate to the shared API. Search transcript showed remaining priority wrappers are aliases or delegating functions, not independent vocabularies/rank tables. Validation passed: cargo test -p atelier-core; cargo test -p atelier-records; cargo test -p atelier-sqlite; cargo test -p atelier-cli priority; cargo test -p atelier-cli commands::import::tests; cargo test -p atelier-cli mission_projection_worktree::test_lint_has_stable_diagnostics_for_hard_invalid_markdown_records; cargo fmt -- --check; target/debug/atelier lint atelier-3h90; git diff --check; cargo build -p atelier-cli.
