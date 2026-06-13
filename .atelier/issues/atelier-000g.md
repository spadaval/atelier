---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-000g"
issue_type: "task"
labels:
- "feature"
- "rebuild"
- "spec"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000p"
  - kind: "issue"
    id: "atelier-000v"
  - kind: "issue"
    id: "atelier-0011"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-08T21:31:43+00:00"
status: "done"
title: "Implement SQLite rebuild from `.atelier-state`"
updated_at: "2026-06-08T21:31:43+00:00"
---

## Description

Add `rebuild` support that recreates local SQLite runtime state from committed exported files after checkout, merge, pull, or clone.

## Outcome

A fresh worktree can rebuild SQLite from .atelier-state/; rebuild validates manifest/schema compatibility; missing or corrupt projection files produce actionable errors; round-trip tests prove export, delete runtime DB, rebuild, and query equivalence.

## Evidence

Evidence was not specified in the legacy issue record.
