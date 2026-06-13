---
created_at: "2026-06-12T20:29:41.674600364+00:00"
id: "atelier-u4nx"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
- "history"
- "prime"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-trr2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T22:34:58.063491602+00:00"
status: "done"
title: "Repair history and prime signpost surfaces"
updated_at: "2026-06-12T22:34:58.063491602+00:00"
---

## Description

Repair history and prime surfaces according to the closed signpost contracts in
`atelier-hggl` and `atelier-bzts`. The product decisions already exist; this
work should implement or repair them rather than reopen the question.

## Outcome

- Repo-wide and scoped history views support the event sources, ordering,
  filters, bounded output, empty states, and drill-down behavior from
  `atelier-hggl`.
- Issue and mission show surfaces point to scoped history for full activity
  rather than printing unbounded activity.
- `atelier prime` provides recovery and onboarding guidance distinct from
  `atelier status`, following `atelier-bzts`.
- Prime output avoids generic filler and includes only commands with concrete
  reasons.
- Docs/help/Agent Factory guidance agree on when to use history, prime, and
  status.

## Evidence

- CLI transcript tests for repo-wide history, mission-scoped history,
  issue-scoped history, empty filters, bounded output, and drill-down commands.
- Prime transcript tests for empty checkout, active mission, and active work
  states.
- Docs/help parity check against `atelier-hggl` and `atelier-bzts`.
