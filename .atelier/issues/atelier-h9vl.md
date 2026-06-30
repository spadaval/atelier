---
created_at: "2026-06-29T17:37:22.449578117+00:00"
id: "atelier-h9vl"
issue_type: "epic"
labels:
- "agent-factory"
- "planning"
- "process"
review:
  kind: pull_request
  number: 32
  provider: forgejo
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2uim"
  children:
  - kind: "issue"
    id: "atelier-4d3f"
  - kind: "issue"
    id: "atelier-pi3o"
  - kind: "issue"
    id: "atelier-ql9k"
  - kind: "issue"
    id: "atelier-r6a2"
  - kind: "issue"
    id: "atelier-siu5"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1mga"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-30T15:16:12.212071333+00:00"
status: "done"
title: "Epic: Simplify mission planning to target state"
updated_at: "2026-06-30T15:16:12.212071333+00:00"
---

## Description

Mission planners write the target state clearly enough for implementation and independent validation. They do not pre-author evidence sections, validation checklists, or architecture-sounding abstractions in place of concrete outcomes.

## Outcome

Planner-facing guidance and examples make the planner's job small and concrete: write the target state clearly enough that a worker knows what to change and a validator can independently decide how to test it. The guidance stops teaching generic validation/evidence boilerplate as mission-planning work.
