---
created_at: "2026-06-19T20:14:30.439012825+00:00"
id: "atelier-s8n2"
issue_type: "feature"
labels:
- "cli"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-d8bt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Render transition effect plans in CLI output"
updated_at: "2026-06-19T20:14:30.439012825+00:00"
---

## Description

Render transition effects clearly in the issue transition command family so
operators can understand transition readiness without reading workflow YAML or
parsing JSON.

## Outcome

- `atelier issue transition <id> --options` separates status movement,
  validators, required fields, planned effects, and next commands.
- Blocked transition output names effect preflight blockers separately from
  validator failures.
- Successful transition output names each applied, skipped, or deferred effect
  and any created or reused review artifact.
- Quiet output remains bounded to stable IDs and status tokens.

## Evidence

- CLI integration tests or snapshot/golden tests cover no-effect transitions,
  review-effect transitions, blocked effects, and successful effects.
- Help transcript shows transition effects are discoverable from transition
  commands.
- `atelier lint atelier-s8n2` passes.
