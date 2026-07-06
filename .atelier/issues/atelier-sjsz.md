---
created_at: "2026-07-06T17:20:25.693972086+00:00"
id: "atelier-sjsz"
issue_type: "task"
labels:
- "cli"
- "docs"
- "mission-dashboard"
- "tests"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Align Mission Overview guidance and regression coverage"
updated_at: "2026-07-06T17:20:25.693972086+00:00"
---

## Description

Align `work missions` help, root examples, role guidance, product docs, and focused tests with the finished overview. Cover default hidden done missions, explicit done inclusion, linked epics, collapsed subtasks, direct and unassigned work, ordering, limits, empty state, quiet mode, TTY color, `NO_COLOR`, and drill-down guidance.

## Outcome

- Help and durable guidance call the surface Mission Overview, distinguish plural `work missions` from scoped `work mission <id>`, and do not route hierarchy back through `issue list` or the legacy broad queue.
- Focused coverage fails if done missions leak into the default, epics lose mission context, leaf subtasks expand by default, color policy regresses, or colorless structure becomes incomplete.

## Evidence

Evidence was not specified in the bundle.
