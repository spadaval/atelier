---
created_at: "2026-06-13T02:36:03.729038692+00:00"
id: "atelier-yj4c"
issue_type: "feature"
labels:
- "cli"
- "mission"
- "ux"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ezvf"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Fix mission next actions by state"
updated_at: "2026-06-13T04:23:53.939748864+00:00"
---

## Description

Make mission next actions context-aware so closed, draft, ready, active, blocked, and close-ready missions never suggest stale or nonsensical commands.

## Outcome

- Closed missions do not suggest returning to ready as a normal next action.
- Draft missions suggest shaping or moving to ready only when gates permit it.
- Active and blocked missions surface the next issue, blocker, missing evidence, or validation action.
- Close-ready missions suggest the closeout command and required final checks.

## Evidence

- Transcript or snapshot tests cover next actions for every mission state.
- Regression test covers the closed-mission stale-ready suggestion.
- `atelier lint`, `atelier export --check`, and relevant CLI tests pass.
