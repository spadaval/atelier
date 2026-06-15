---
created_at: "2026-06-15T05:13:40.149211999+00:00"
id: "atelier-5dgb"
issue_type: "task"
labels:
- "architecture"
- "sqlite"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-xmvz"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T16:50:46.258753627+00:00"
status: "done"
title: "Define replacement ProjectionIndex and RuntimeState SQLite schema"
updated_at: "2026-06-15T16:50:46.258753627+00:00"
---

## Description

Define the replacement SQLite schema for ProjectionIndex and RuntimeState before implementation rewrites query code.

## Outcome

- Schema contract separates rebuildable projection tables from local runtime tables.
- Old schema compatibility is explicitly out of scope; rebuild from `.atelier/` is the migration path.
- The contract keeps one `.atelier/runtime/state.db` unless a later artifact task changes that decision.
- Runtime active-work and hidden claim tables are not part of the new source-of-truth model.

## Evidence

- File change review of architecture or schema documentation describes table ownership and rebuild semantics.
- Review artifact maps old table responsibilities to new projection/runtime ownership.
- `atelier lint atelier-5dgb` and `atelier export --check` pass.
