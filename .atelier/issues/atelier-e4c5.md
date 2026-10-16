---
created_at: "2026-10-15T21:00:45.353293225+00:00"
id: "atelier-e4c5"
issue_type: "bug"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-e4c5
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-9vq3"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T00:48:25.376757528+00:00"
status: "done"
title: "Apply an epic's open blockers to its children in ready and start"
updated_at: "2026-10-16T00:48:25.376757528+00:00"
---

## Goal

When an epic is blocked by another epic, its children still show in `atelier work ready`, and `issue transition <child>` shows `start` as allowed. Agents picking from the ready list start work whose prerequisites don't exist yet. The only workaround is to repeat each blocker on every child.

## Done when

Tests show that a child of an epic with an open blocker is classified as blocked (naming the epic's blocker) in `work ready` and `work mission`, and that `start` refuses it.

## Context

astromech: "Find a module near me" was ready while its epic waited on the remote-data epic; the fix duplicated links onto six children. `crates/atelier-app/src/work_classification.rs` (`classify_issue` only reads the issue's own blockers). Related: atelier-28mz (children of draft epics).

Found 2026-10-15 while moving astromech's BACKLOG.md (about 135 items with research, references and decisions) into Atelier records. Sumeet's principle: Atelier records should hold the whole backlog; needing a side document means Atelier needs work.

## Close Reason

Readiness and the start validator use effective blockers: an issue's own plus its parent's. A child of a parent with an open blocker is blocked (naming that blocker) in work ready, work blocked, mission and parent views, and start refuses it. Test: test_children_inherit_their_parents_open_blockers. Documented in docs/product/work-model.md.
