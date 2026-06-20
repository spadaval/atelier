---
created_at: "2026-06-20T04:17:09.573358857+00:00"
id: "atelier-4i5f"
issue_type: "task"
labels:
- "regression"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-l7i6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add transition lost-update regression coverage"
updated_at: "2026-06-20T04:17:09.573358857+00:00"
---

## Description

Imported bundle issue.

## Outcome

- Tracker-mutating pre-actions cannot lose fields when the transition writes status.
- Tests cover the reload-after-action behavior that prevents stale whole-record writes.
- The implementation remains a pragmatic reload policy, not a new issue ORM/session layer.

## Evidence

- Regression test demonstrates a review.open-style action writes `fields.review`, the transition changes status, and the final canonical issue preserves both changes.
