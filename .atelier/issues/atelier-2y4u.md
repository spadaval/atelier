---
created_at: "2026-06-17T21:36:08.136514195+00:00"
id: "atelier-2y4u"
issue_type: "bug"
labels:
- "cli"
- "output"
- "readiness"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-17T21:50:48.800737514+00:00"
status: "done"
title: "Repair ready issue list so documented ready queue matches status"
updated_at: "2026-06-17T21:50:48.800737514+00:00"
---

## Description

The documented operator loop uses `atelier status` to find whether ready work
exists and `atelier issue list --ready` to choose executable work. During
mission `atelier-0v3f` readiness inspection, `atelier status` reported 17 ready
issues and `atelier mission status atelier-0v3f` listed selectable ready work,
but bare `atelier issue list --ready` printed `No issues found.`

This issue owns the ready-list command behavior only. It should not reshape the
mission graph, dependency semantics, workflow policy, or parent/child output
hierarchy unless those are the direct cause of the mismatch.

## Outcome

- `atelier issue list --ready` returns startable todo-category work whenever
  `atelier status` reports ready work in the same checkout.
- Ready-list output remains blocker-aware and does not present blocked parent
  epics as selectable work.
- If no work is startable, the command explains that result consistently with
  status/mission readiness rather than hiding real ready children.

## Evidence

- Failing-before/passing-after transcript or focused CLI test covers the
  `atelier-0v3f` shape where ready child tasks exist under open epics.
- Transcript compares `atelier status`, `atelier mission status atelier-0v3f`,
  and `atelier issue list --ready` and shows consistent ready counts or
  selectable ready IDs.
- Focused ready-list tests and `atelier lint` pass.
