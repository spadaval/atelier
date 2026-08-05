---
created_at: "2026-07-28T16:39:31.744381815+00:00"
id: "atelier-js22"
issue_type: "spike"
labels:
- "agent-factory"
- "audit"
- "planning"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-iznq"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Audit Atelier against adaptive mission scenarios"
updated_at: "2026-08-05T16:43:27.127727601+00:00"
---

## Description

Inspect current mission records, issue bodies, status and work views, bundle creation, dependency edits, supersession behavior, strategy references, documentation, and the repository Agent Factory binding. Do not design a heavyweight planning subsystem.

## Outcome

- A durable gap matrix maps each evolved Agent Factory behavior to current Atelier support, friction, or missing behavior.
- Each gap names an observable operator scenario and identifies whether prose, CLI presentation, workflow behavior, or persistence actually needs to change.
- The audit identifies existing capabilities that should be reused and proposed machinery that would violate Atelier's anti-red-tape boundary.

## Evidence

- `docs/architecture/quality/adaptive-mission-gap-analysis.md` records the scenario matrix, reused capabilities, smallest required changes, and rejected machinery.
