---
created_at: "2026-06-15T05:13:45.545560949+00:00"
id: "atelier-vv2i"
issue_type: "task"
labels:
- "app-layer"
- "architecture"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-14z2"
  - kind: "issue"
    id: "atelier-nyn0"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T16:50:51.751873115+00:00"
status: "done"
title: "Introduce application use-case layer and storage ports"
updated_at: "2026-06-15T16:50:51.751873115+00:00"
---

## Description

Introduce an application/use-case layer that coordinates records, SQLite, workflow policy, and runtime services without depending on Clap parsing.

## Outcome

- `atelier-app` exposes internal use-case entrypoints for visible command jobs.
- Storage and workflow dependencies are injected or routed through narrow application boundaries instead of direct command-module coupling.
- Human output view models are separated from canonical storage and query logic.

## Evidence

- File review shows command orchestration moved behind app-layer boundaries.
- Unit or integration tests cover representative app use cases without invoking Clap.
- `cargo nextest run` passes for affected crates.
