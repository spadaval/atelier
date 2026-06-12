---
created_at: "2026-06-11T18:22:54.490755940+00:00"
id: "atelier-afir"
issue_type: "epic"
labels:
- "compatibility"
- "export"
- "markdown"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ybt6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Retire export-as-writer compatibility path"
updated_at: "2026-06-11T18:48:51.129399578+00:00"
---

## Description

Retire export_current_state as the normal command durability path after mutation-family epics are complete. Scope includes removing or narrowing compatibility writer calls, keeping explicit export/export --check repair behavior, updating diagnostics and docs, and proving no public durable mutation relies on a later export step for recovery. Acceptance: export remains a sync/check/repair surface; normal public mutations write via RecordStore; audit residue is documented; tests fail if representative commands mutate SQLite-only state.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.
