---
created_at: "2026-06-15T05:11:25.081157011+00:00"
id: "atelier-0fhv"
issue_type: "epic"
labels:
- "projection"
- "rewrite"
- "runtime"
- "sqlite"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3kap"
  - kind: "issue"
    id: "atelier-4j3k"
  - kind: "issue"
    id: "atelier-fchz"
  children:
  - kind: "issue"
    id: "atelier-5dgb"
  - kind: "issue"
    id: "atelier-rjua"
  - kind: "issue"
    id: "atelier-wng0"
  - kind: "issue"
    id: "atelier-xmvz"
  - kind: "issue"
    id: "atelier-yo9i"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T18:26:10.476164132+00:00"
status: "done"
title: "Epic: Rewrite SQLite projection and runtime storage"
updated_at: "2026-06-15T18:26:10.476164132+00:00"
---

## Description

Replace inherited SQLite schema and persistence boundaries with an `atelier-sqlite` crate that owns rebuildable ProjectionIndex queries and local RuntimeState. The schema may be replaced freely because canonical Markdown remains the source of truth.

## Outcome

- `atelier-sqlite` owns projection rebuild, freshness checks, query indexes, search/graph/readiness queries, and local runtime tables.
- Local SQLite rebuild from `.atelier/` is the supported migration path; old schema compatibility is not preserved unless an issue explicitly justifies it.
- SQLite `comments`, `sessions`, and `work_associations` are removed; preserved import comments and close reasons live as canonical activity sidecars.
- Runtime active-work and hidden claim state are removed as sources of truth, with current work derived from canonical issue status and checkout context.
- One `.atelier/runtime/state.db` remains the default unless a later artifact task proves a split database is needed.

## Evidence

- Child issue proof shows replacement schema definition, comments/sessions/work-association deletion, rebuild/query implementation, and active-work/claim removal.
- Missing and stale DB recovery transcripts prove `doctor` or safe rebuild restores query behavior from committed Markdown.
- Focused SQLite tests cover rebuild, freshness, ready/search/graph queries, and runtime-state boundaries.

## Notes

- Temporary adapters used while extracting `atelier-sqlite` must follow
  `docs/architecture/source-layout.md`: name the adapter marker, removal owner,
  removal condition, and proof that no public compatibility promise is being
  created.
