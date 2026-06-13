---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-000x"
issue_type: "task"
labels:
- "spec"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000w"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-08T19:47:40+00:00"
status: "done"
title: "Decide `.atelier/state.db` gitignore and runtime-state policy"
updated_at: "2026-06-08T19:47:40+00:00"
---

## Description

Resolve whether `.atelier/state.db` is always ignored, optionally committed for convenience, or handled by another policy while still being rebuildable from `.atelier-state/`.
Outcome was not specified in the legacy issue record.
Evidence was not specified in the legacy issue record.
### Resolution

Always treat `.atelier/state.db` and the rest of `.atelier/` as local runtime state. Keep `.atelier/` ignored. Commit deterministic, rebuildable state only under `.atelier-state/`, with local cache subpaths such as `.atelier-state/cache/` ignored when they are derived artifacts.

### Rationale

SQLite is the fast local runtime store, not the durable merge surface. The target storage contract says `.atelier-state/` is sufficient to rebuild the local database after checkout, pull, merge, or clone. Committing the SQLite database would invite binary merge conflicts and weaken the export/rebuild invariant.

### Alternatives Considered

- Always ignore `.atelier/state.db`: chosen.
- Permit committed runtime DB for convenience: rejected because it conflicts with Git-mergeable canonical projection.
- Keep DB local but commit deterministic projections only: chosen as the broader policy.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
