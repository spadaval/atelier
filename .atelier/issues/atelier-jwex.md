---
created_at: "2026-08-05T18:46:52.084817342+00:00"
id: "atelier-jwex"
issue_type: "task"
labels:
- "review-finding"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-08-05T19:12:10.681413437+00:00"
status: "done"
title: "Unify blocked readiness across work views"
updated_at: "2026-08-05T19:12:10.681413437+00:00"
---

## Description

Repair the blocking PR 77 finding that normal views disagree about blocker-free `blocked` work. Put the classification behind one domain rule shared by status, global work views, and focused mission/epic views.

## Outcome

- Status, global work views, mission views, and epic views classify explicit blocked states consistently.
- A workflow transition out of `blocked` does not make the record ready until its status changes.
- Cross-surface CLI regressions cover the shared rule.

## Evidence

- Fresh global, mission, and epic view scenarios agree on blocker-free blocked classification.
