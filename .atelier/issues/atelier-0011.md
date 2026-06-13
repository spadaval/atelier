---
created_at: "2026-06-08T19:39:30+00:00"
id: "atelier-0011"
issue_type: "task"
labels:
- "agent-factory"
- "feature"
- "import"
- "migration"
- "mission"
- "storage"
- "tracker"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0012"
  - kind: "issue"
    id: "atelier-0013"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-08T21:51:20+00:00"
status: "done"
title: "Implement Beads data import into Atelier state"
updated_at: "2026-06-08T21:51:20+00:00"
---

## Description

Implement an import path from current Beads data into Atelier. The first supported source should be this repo's safe manual backup `.beads/issues.manual.jsonl`; optionally support direct Beads/Dolt import only after JSONL import is reliable. Preserve issue IDs or define a deterministic mapping with aliases.

Scope includes issues, status, priority, type, labels, parent-child links, blocking dependencies, descriptions, acceptance criteria, notes/comments where available, closed state, and timestamps where practical.

## Outcome

A command can import .beads/issues.manual.jsonl into Atelier SQLite and/or .atelier-state; migrated record counts match the source; dependencies and parent-child relationships round-trip; migrated records can be listed, shown, updated, and closed through Atelier; validation reports unmigrated or lossy fields; tests use a fixture based on this repo's Beads export.

## Evidence

Evidence was not specified in the legacy issue record.
