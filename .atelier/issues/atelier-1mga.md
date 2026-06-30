---
created_at: "2026-06-29T17:37:11.545961188+00:00"
id: "atelier-1mga"
issue_type: "mission"
labels:
- "agent-factory"
- "process"
- "validation"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2uim"
    type: "validates"
  - kind: "issue"
    id: "atelier-ef94"
    type: "advances"
  - kind: "issue"
    id: "atelier-h9vl"
    type: "advances"
  - kind: "issue"
    id: "atelier-kzu2"
    type: "advances"
  - kind: "issue"
    id: "atelier-pc7s"
    type: "advances"
  - kind: "issue"
    id: "atelier-qdp8"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "ready"
title: "Mission: Simplify mission planning and workflow behavior"
updated_at: "2026-06-29T17:37:11.545961188+00:00"
---

## Description

Agents can plan and execute missions from a clear target state without planner-authored proof paperwork. Mission lifecycle behavior is controlled by .atelier/workflow.yaml, mission scope is expressed by direct `advances` links to root work and the child issues under those roots, validation is independent worker judgment from the Outcome, and evidence records remain receipts for checks that actually ran.

## Outcome

Mission planning and lifecycle behavior is simpler and worker-usable. A mission record states the target state. Mission scope comes from direct `advances` links to root work plus the descendants of those roots. Mission lifecycle behavior comes from `.atelier/workflow.yaml`. Validators decide how to prove the outcome after implementation. Mission closeout does not require planner-authored validation prose or direct mission evidence records.
