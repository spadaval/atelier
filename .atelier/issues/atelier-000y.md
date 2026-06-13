---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-000y"
issue_type: "task"
labels:
- "feature"
- "projection"
- "spec"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000g"
  - kind: "issue"
    id: "atelier-0011"
  - kind: "issue"
    id: "atelier-0012"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-08T21:23:39+00:00"
status: "done"
title: "Implement deterministic per-record export and `export --check`"
updated_at: "2026-06-08T21:23:39+00:00"
---

## Description

Replace or augment backup-oriented export with canonical projections under `.atelier-state/`. `export --check` must compare live SQLite state with exported files and fail when projections are stale.
Export output is deterministic across repeated runs; stale projection cases fail export --check; mutating command behavior is documented; tests cover no-op export, changed record export, stale check failure, and JSON/Markdown serialization stability.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
