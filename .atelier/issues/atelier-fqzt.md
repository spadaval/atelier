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
status: "todo"
title: "Enforce independent review in the mission lifecycle"
updated_at: "2026-07-06T20:37:49.349657810+00:00"
---

## Description

Assigned subskill: implement. Implement the configured draft, plan-review, ready, and start behavior. The planner can request review but cannot provide the independent approval. Ready and start transitions require a current approval and no unresolved blocking findings. Direct status edits or alternate transition paths cannot bypass the same policy.

## Outcome

- An unreviewed, self-approved, change-requested, unresolved, or stale-approved mission cannot become ready or start execution.
- A different reviewer can approve the exact current revision, after which the configured transition can make the mission ready.
- Every supported lifecycle mutation path applies the same review authority and freshness rules.

## Evidence

Evidence was not specified in the bundle.
