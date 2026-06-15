---
created_at: "2026-06-15T05:13:47.369776164+00:00"
id: "atelier-14z2"
issue_type: "task"
labels:
- "app-layer"
- "cli"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-zwna"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T07:56:17.125519130+00:00"
status: "done"
title: "Move command handlers and view models into atelier-app"
updated_at: "2026-06-15T07:56:17.125519130+00:00"
---

## Description

Move existing command handlers and human output models into `atelier-app` modules organized by operator job.

## Outcome

- Oversized command handlers are split into cohesive app modules for issue, mission, evidence, graph, workflow, status, and maintenance jobs.
- Human output remains stable in intent and follows existing output grammar.
- Application code no longer depends on raw Clap argument structs except through explicit request types.

## Evidence

- Representative CLI integration tests pass for moved command surfaces.
- `wc -l` or module inventory transcript shows large handlers were split by responsibility.
- Docs/help parity checks still pass for visible commands.
