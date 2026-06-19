---
created_at: "2026-06-19T20:14:25.097863580+00:00"
id: "atelier-xiqw"
issue_type: "feature"
labels:
- "implementation"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-d8bt"
  - kind: "issue"
    id: "atelier-vt42"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Execute transition effects with issue status changes"
updated_at: "2026-06-19T21:03:40.921699602+00:00"
---

## Description

Execute declared transition effects as part of successful issue transition
commands, using the planner and product contract to sequence validators,
preflight, effects, canonical state changes, and activity records.

## Outcome

- Transition execution reports validator failure before effects run.
- Effect execution is ordered, idempotent where the contract requires it, and
  records applied or skipped effects in issue activity.
- Local canonical state is not silently advanced when required effects fail.
- Recovery output names the failed effect and the command or state inspection
  needed before retry.

## Evidence

- Focused tests cover success, validator failure, preflight failure, effect
  failure, idempotent retry, and activity evidence.
- Command transcript shows representative successful and failed transition
  output.
- `atelier lint atelier-xiqw` passes.
