---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-0008"
issue_type: "task"
labels:
- "feature"
- "lint"
- "spec"
- "workflow"
priority: "P3"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0007"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T21:18:12.462311269+00:00"
status: "done"
title: "Implement lint severities, waivers, and action-aware guidance"
updated_at: "2026-06-11T21:18:12.462311269+00:00"
---

## Description

Support mechanical rules, lint rules, advisory guidance, severities (`info`, `warn`, `error`, `policy`), and explicit waivers surfaced close to the relevant action.
Lint output includes severity and stable JSON; waivers require explicit reason and are visible in projections; action-aware guidance is scoped to the command being run; tests cover warning, error, policy, and waiver behavior.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
