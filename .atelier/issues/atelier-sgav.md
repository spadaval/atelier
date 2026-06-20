---
created_at: "2026-06-20T04:17:09.553582228+00:00"
id: "atelier-sgav"
issue_type: "task"
labels:
- "artifact"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2sma"
  - kind: "issue"
    id: "atelier-4i5f"
  - kind: "issue"
    id: "atelier-6jjm"
  - kind: "issue"
    id: "atelier-eomn"
  - kind: "issue"
    id: "atelier-o5a9"
  - kind: "issue"
    id: "atelier-wehh"
  - kind: "issue"
    id: "atelier-y8cs"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T04:49:34.961430459+00:00"
status: "done"
title: "Define transition action contract and merge authority"
updated_at: "2026-06-20T04:49:34.961430459+00:00"
---

## Description

Imported bundle issue.

## Outcome

- Durable product/architecture guidance defines validators as read-only gates, issue mutation as transition-owned state change, and actions as explicit side effects.
- Action failures are specified as stopped, durable, and resumable instead of implicit rollback.
- Provider workflows and room workflows have distinct integration authority rules.
- Tracker-mutating actions rely on reload-after-action policy for now; no ORM/session layer is introduced.

## Evidence

- Documentation or architecture artifact names the transition execution order, action failure semantics, explicit tracker.commit behavior, provider-vs-room merge authority, and current reload policy for tracker-mutating actions.
