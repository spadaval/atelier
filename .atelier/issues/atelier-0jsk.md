---
created_at: "2026-06-19T03:58:57.561360406+00:00"
id: "atelier-0jsk"
issue_type: "validation"
labels:
- "review"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Validate native room review behavior end to end"
updated_at: "2026-06-19T03:58:57.561360406+00:00"
---

## Description

Independently validate native room-mode behavior against the mission contract.
This issue should not implement fixes; defects become follow-up issues or
blockers.

## Outcome

- A room-mode project opens a room for an epic or standalone issue and persists
  the structured issue `review` link.
- Comments, file anchors, findings, approval, changes requested, resolve, and
  status/show output are exercised through `atelier review`.
- New commits after approval invalidate approval.
- Open blocking findings prevent `review merge`; a resolved finding plus current
  approval allows merge.
- Successful `review merge` does not transition ordinary issue workflow.

## Evidence

- Scenario transcript or integration test output covers each room-mode outcome.
- The generated `.atelier/reviews/<id>.yaml` file is inspected for deterministic
  metadata and event ordering.
- Evidence record classifies any failures as blocking, non-blocking, deferred,
  or not-applicable.
