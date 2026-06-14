---
created_at: "2026-06-11T20:09:59.949770763+00:00"
id: "atelier-29oi"
issue_type: "epic"
labels:
- "config"
- "migration"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-24sg"
  children:
  - kind: "issue"
    id: "atelier-3es3"
  - kind: "issue"
    id: "atelier-kxko"
  - kind: "issue"
    id: "atelier-pgkd"
  - kind: "issue"
    id: "atelier-ru15"
  attachments:
  - kind: "evidence"
    id: "atelier-iimw"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T23:35:48.137166586+00:00"
status: "done"
title: "Implement markdown-first layout migration and compatibility"
updated_at: "2026-06-11T23:35:48.137166586+00:00"
---

## Description

Move path resolution and migration behavior toward the new .atelier/ layout while preserving a temporary read/migrate-only compatibility path for existing .atelier-state repositories.
- One storage layout module owns canonical paths, runtime/cache paths, legacy discovery, and ignore policy decisions.
- atelier migrate markdown-first or equivalent one-shot migration moves committed records from .atelier-state/* into .atelier/* without moving runtime state into committed paths.
- .gitignore ignores only runtime/cache subpaths, not all of .atelier/.
- After migration, canonical writes target only the new layout; old .atelier-state is not silently rewritten.
- Existing init/export/rebuild flows agree on the new layout.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
