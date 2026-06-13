---
created_at: "2026-06-13T02:33:43.603242530+00:00"
id: "atelier-2wbz"
issue_type: "epic"
labels:
- "cli"
- "mission"
- "ux"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-6cte"
  - kind: "issue"
    id: "atelier-7xqy"
  - kind: "issue"
    id: "atelier-b7wl"
  - kind: "issue"
    id: "atelier-ezvf"
  - kind: "issue"
    id: "atelier-yj4c"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T16:09:09.597585550+00:00"
status: "done"
title: "Collapse mission operator CLI into contextual status"
updated_at: "2026-06-13T16:09:09.597585550+00:00"
---

## Description

Make mission and closeout commands smaller in number and stronger in behavior. Normal operators should ask one contextual status surface for state, blockers, missing proof, next action, and closeout readiness instead of stitching together status, audit, workflow diagnostics, and history output.

## Outcome

- `atelier mission status <id>` is the primary operator surface for mission state, blockers, missing evidence, validation freshness, and next action.
- Mission audit behavior is folded into contextual status, closeout, or verbose drill-down instead of being a competing normal command.
- Closed, draft, ready, and active missions show state-appropriate next actions.
- Default output is compact and failure/gap oriented; verbose output remains available for full line-by-line audits.
- Help text and docs agree on the reduced command model.

## Evidence

- Command contract docs or help snapshots define the reduced mission/operator surface.
- Transcript tests cover draft, ready, active, blocked, close-ready, and closed mission states.
- Negative transcript proves stale or nonsensical next actions no longer appear.
