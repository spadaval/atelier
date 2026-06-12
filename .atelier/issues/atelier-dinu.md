---
created_at: "2026-06-11T18:54:32.028629821+00:00"
id: "atelier-dinu"
issue_type: "task"
labels:
- "migration"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-dydv"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-v08o"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Migrate canonical records from .atelier-state into .atelier"
updated_at: "2026-06-11T23:26:41.680453642+00:00"
---

## Description

Move canonical tracker records into the single .atelier/ root and update export/rebuild/query paths accordingly.

Scope:
- Move canonical directories such as issues/, missions/, evidence/, milestones/, plans/, and projections from .atelier-state/ into .atelier/.
- Update RecordStore, export, rebuild, projection freshness checks, CLI help, tests, and fixtures to use the new canonical root.
- Keep .atelier/state.db as the ignored local SQLite runtime database.
- Preserve rebuild behavior from canonical records into state.db.

Out of scope:
- Hook integration cleanup.
- Reworking record schemas.

## Outcome

- A fresh checkout with tracked .atelier records can rebuild .atelier/state.db.
- export --check validates the new .atelier canonical paths.
- No code path requires .atelier-state for normal operation.

## Evidence

Evidence was not specified in the legacy issue record.
