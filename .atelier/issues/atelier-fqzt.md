---
created_at: "2026-07-06T20:37:49.349657810+00:00"
id: "atelier-fqzt"
issue_type: "feature"
labels:
- "mission-review"
- "subskill-implement"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3v2d"
  - kind: "issue"
    id: "atelier-72k4"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-16T19:40:32.641369071+00:00"
status: "done"
title: "Enforce independent review in the mission lifecycle"
updated_at: "2026-07-16T19:40:32.641369071+00:00"
---

## Description

Assigned subskill: implement. Implement the configured draft, plan-review, ready, and start behavior. The planner can request review but cannot provide the independent approval. Ready and start transitions require a current approval and no unresolved blocking findings. Direct status edits or alternate transition paths cannot bypass the same policy.

## Outcome

- An unreviewed, self-approved, change-requested, unresolved, or stale-approved mission cannot become ready or start execution.
- A different reviewer can approve the exact current revision, after which the configured transition can make the mission ready.
- Every supported lifecycle mutation path applies the same review authority and freshness rules.

## Evidence

- Public transition tests reject readiness and start for review not requested, approval missing, author/material-editor self-approval, unresolved blocking findings, change requested, and approval made stale by a material graph edit.
- A transcript shows a distinct reviewer approving the exact current graph revision, after which the configured ready and start paths succeed when all other validators pass.
- Focused tests exercise every supported lifecycle mutation entry point, including direct record/status mutation recovery and alternate transition attempts, and show none can create executable state without the same authority, freshness, and finding checks.
