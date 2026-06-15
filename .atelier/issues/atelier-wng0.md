---
created_at: "2026-06-15T15:16:59.251743840+00:00"
id: "atelier-wng0"
issue_type: "task"
labels:
- "migration"
- "sqlite"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Retarget storage callers to atelier-sqlite APIs"
updated_at: "2026-06-15T15:16:59.251743840+00:00"
---

## Description

Retarget rebuild, import, export, doctor, status, graph, workflow, and query callers to the atelier-sqlite projection/runtime APIs so command code no longer reaches into root src/db modules.

## Outcome

- Rebuild, import, export, doctor, status, graph, workflow, and query callers use `atelier-sqlite` projection/runtime APIs rather than root `src/db` modules.
- Projection freshness, schema setup, rebuildable query methods, and runtime database opening are owned by `atelier-sqlite`.
- No root database compatibility module remains after caller retargeting.

## Evidence

- Search transcript proves callers no longer import `crate::db`, `atelier::db`, or root database compatibility modules.
- Targeted command transcripts cover rebuild/import/export/doctor/status behavior through the new storage API.
- SQLite-focused tests cover projection freshness, schema setup, rebuildable queries, and runtime database opening.
