---
created_at: "2026-10-17T08:19:41.002938778+00:00"
id: "atelier-6wyv"
issue_type: "bug"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-6wyv
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T08:49:39.474812809+00:00"
status: "done"
title: "Offer start only for work mission rows that pass the readiness gates"
updated_at: "2026-10-17T08:49:39.474812809+00:00"
---

## Goal

`work mission`'s Ready Work skips the readiness gates `work ready` applies, so the `atelier start <top ready>` step atelier-ksx6 added can name an item `start` then refuses. Found while implementing atelier-ksx6.

## Done when

- `work mission` and `work parent` classify Ready Work with the same gates as `work ready`, or offer start only for an item that passes them.
- A test covers an item with an unmet readiness gate.

## Close Reason

Not reproducible as filed: work mission and work parent already classify Ready Work through dependency_closure, the same transitive check as start's blockers.transitive_none_open, and it agrees with work ready's dependency_readiness_batch gate on every acyclic graph (cycles are rejected at load). Added a session test covering an item whose closed blocker waits on open work across work mission, work parent, work ready and start, shown to fail when classification checks only direct blockers, and stated the transitive rule in docs/product/work-view-ordering.md.
