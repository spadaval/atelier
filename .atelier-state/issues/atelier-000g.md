---
acceptance: []
blocks:
- "atelier-000p"
- "atelier-000v"
- "atelier-0011"
created_at: "2026-06-08T17:33:27+00:00"
depends_on:
- "atelier-000y"
evidence_required: []
id: "atelier-000g"
issue_type: "task"
labels:
- "feature"
- "rebuild"
- "spec"
- "storage"
links: []
parent: "atelier-0009"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Implement SQLite rebuild from `.atelier-state`"
updated_at: "2026-06-08T21:31:43+00:00"
---


Add `rebuild` support that recreates local SQLite runtime state from committed exported files after checkout, merge, pull, or clone.

## Acceptance Criteria

A fresh worktree can rebuild SQLite from .atelier-state/; rebuild validates manifest/schema compatibility; missing or corrupt projection files produce actionable errors; round-trip tests prove export, delete runtime DB, rebuild, and query equivalence.
