---
acceptance: []
created_at: "2026-06-12T02:09:59.661915974+00:00"
evidence_required: []
id: "atelier-cv3p"
issue_type: "task"
labels:
- "cli"
- "mission"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Implement mission lifecycle statuses"
updated_at: "2026-06-12T02:09:59.661915974+00:00"
---

Implement the mission lifecycle model once the contract is specified. Scope: support draft, ready, active, and closed mission statuses; make mission start transition a ready mission to active; enforce at most one active mission; remove or migrate use of data.active as lifecycle state; update mission list/status/show rendering and filters; preserve closeout validation for closed; add focused CLI integration coverage. Acceptance: tests prove default creation state, ready-to-active start, one-active enforcement, list/status behavior, legacy open/active migration or compatibility handling, and closeout behavior.
