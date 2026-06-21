---
created_at: '2026-06-11T15:53:53.956344586+00:00'
id: atelier-dvxc
issue_type: mission
labels:
- mission
priority: P2
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: evidence
    id: atelier-uu8j
    type: validates
  - kind: issue
    id: atelier-52ev
    type: advances
  - kind: issue
    id: atelier-64w5
    type: advances
  - kind: issue
    id: atelier-6xoh
    type: advances
  - kind: issue
    id: atelier-8ptg
    type: advances
  - kind: issue
    id: atelier-ncog
    type: advances
  - kind: issue
    id: atelier-rr6y
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-11T16:30:27.591718302+00:00'
status: closed
title: Finish Markdown-first storage transition
updated_at: '2026-06-11T16:30:27.591718302+00:00'
---

## Description

Complete the second phase of the Markdown-first storage transition. The product outcome is that durable project state is owned by Markdown records, while SQLite behaves as an automatically repaired hot cache/projection for query speed and UI inputs. Operators should not need to understand or manually repair the cache during ordinary read workflows.

## Outcome

### Constraints

- Canonical durable records live in .atelier-state/ and are mutated through RecordStore before projection refresh.
- SQLite remains a rebuildable ProjectionIndex plus RuntimeState, not a competing durable source of truth.
- Ordinary projection-backed read commands transparently rebuild stale cache state when .atelier-state/ validates.

### Risks

- Transparent rebuild could hide invalid or partially written Markdown unless validation and locking boundaries are strict.
- A broad command migration could regress existing Agent Factory issue and mission workflows if slices are not validated incrementally.

## Evidence

- Manual check: Linked work proves durable mutation command audit, Markdown-first write migration, transparent stale projection rebuild, invalid Markdown failure behavior, concurrency safety, export freshness, lint, doctor, and rebuild-from-checkout scenarios.

## Notes

Migrated from `.atelier/missions/atelier-dvxc.md` as a declared mission objective issue.
