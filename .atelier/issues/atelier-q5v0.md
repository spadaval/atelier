---
created_at: "2026-06-14T04:03:19.642443618+00:00"
id: "atelier-q5v0"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-4yrt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T07:41:06.683459409+00:00"
status: "done"
title: "Define diagnostics JSON boundary for operator workflows"
updated_at: "2026-06-14T07:41:06.683459409+00:00"
---

## Description

The CLI exposes diagnostics slow as stable JSON, while Agent Factory and product
guidance say normal planning and validation should not depend on command-result
JSON. Define and document the boundary: diagnostics JSON is allowed for local
performance and telemetry analysis, but ordinary operator workflow, mission
selection, validation, evidence coverage, blocker decisions, and closeout
should route through human-first status, lint, doctor, mission status,
transition, and evidence surfaces.

## Outcome

- Docs/help guidance makes diagnostics JSON clearly advanced and local-only.
- Normal command recipes do not ask agents to parse diagnostics JSON for
  ready-work, blocker, validation, evidence coverage, or closeout decisions.

## Evidence

- Docs/help diff or command transcript shows the diagnostics JSON boundary.
- Search transcript shows normal workflow docs do not route planning or
  validation through diagnostics JSON.
- `git diff --check` and `atelier lint` pass.
