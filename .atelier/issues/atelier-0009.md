---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-0009"
issue_type: "epic"
labels:
- "epic"
- "milestone"
- "spec"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000j"
  - kind: "issue"
    id: "atelier-000m"
  children:
  - kind: "issue"
    id: "atelier-000g"
  - kind: "issue"
    id: "atelier-000p"
  - kind: "issue"
    id: "atelier-000w"
  - kind: "issue"
    id: "atelier-000y"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Milestone 2: Canonical export and rebuild"
updated_at: "2026-06-11T14:41:57.657636476+00:00"
---

## Description

Build the storage contract from SPEC.md: SQLite is runtime state, `.atelier-state/` is deterministic mergeable repo state, export freshness is checkable, and SQLite can be rebuilt after checkout, merge, pull, or clone.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.

## Notes

### Success Criteria

- A deterministic `.atelier-state/` layout exists.
- Mutating commands update exported state by default or have a documented staged rollout.
- `export --check` detects stale projections.
- `rebuild` recreates SQLite from exports.
- Validation proves round-trip and stale-export behavior.
