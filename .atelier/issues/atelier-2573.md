---
created_at: "2026-06-17T19:38:08.783848744+00:00"
id: "atelier-2573"
issue_type: "task"
labels:
- "refactor"
- "sqlite"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Narrow atelier-sqlite Database projection interface"
updated_at: "2026-06-17T23:08:48.063940024+00:00"
---

## Description

Narrow `atelier-sqlite::Database` so projection and query code cannot bypass
storage invariants through raw connection access or compatibility-style helper
APIs. Keep legacy import/test needs explicit instead of exposing them as normal
production APIs.

## Outcome

- Raw SQLite connection access is private or restricted to clearly named
  internal/test-only helpers.
- Projection query APIs are separated from legacy import and repair helpers.
- No-op runtime session/work-association APIs are deleted or isolated behind
  explicit future-session work so they cannot be mistaken for target behavior.
- Production callers use typed SQLite APIs rather than direct SQL through
  `Database::conn`.

## Evidence

- Search transcript proves production code does not access `Database::conn`
  directly.
- Focused SQLite tests cover projection queries and retained import/repair
  helpers after encapsulation.
- Search transcript proves no-op session/work compatibility APIs are removed,
  test-only, or explicitly isolated.
- `cargo fmt -- --check`, targeted SQLite tests, and `git diff --check` pass.
