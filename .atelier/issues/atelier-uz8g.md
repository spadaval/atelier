---
created_at: "2026-06-15T05:13:52.866673842+00:00"
id: "atelier-uz8g"
issue_type: "task"
labels:
- "rust"
- "tests"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-7vfj"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add crate-level tests for domain records workflow and SQLite"
updated_at: "2026-06-15T05:13:52.866673842+00:00"
---

## Description

Add crate-level tests for the new internal boundaries so behavior is proved close to the code that owns it.

## Outcome

- `atelier-core` has tests for IDs, relationships, and value validation.
- `atelier-records` has record parsing/rendering and mutation tests.
- `atelier-workflow` has policy parsing and transition tests.
- `atelier-sqlite` has rebuild, freshness, query, and runtime-boundary tests.

## Evidence

- Crate-level test transcript shows the new tests running under `cargo nextest run`.
- Test file inventory transcript maps key invariants to crate-local tests.
- Test inventory transcript proves broad CLI integration is no longer the only proof for storage and workflow invariants.
