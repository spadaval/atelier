---
created_at: "2026-06-21T16:37:30.761160625+00:00"
id: "atelier-7nrk"
issue_type: "task"
labels:
- "docs"
- "mission-rework"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-62po"
  - kind: "issue"
    id: "atelier-e7t1"
  - kind: "issue"
    id: "atelier-iv2x"
  - kind: "issue"
    id: "atelier-nbhp"
  - kind: "issue"
    id: "atelier-ncq9"
  - kind: "issue"
    id: "atelier-s43l"
  - kind: "issue"
    id: "atelier-vays"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Document mission-as-declared-policy target"
updated_at: "2026-06-21T18:32:09.515511386+00:00"
---

## Description

Update the durable product and domain contract for mission simplification. This is an artifact-update task and must be closed before implementation depends on the new model.

## Outcome

- Docs define the retained mission domain fields, relationship roles, lifecycle/status ownership, and status/readiness questions.
- Docs explicitly reject hidden built-in mission workflows, active mission focus pointers, mission command aliases, and fallback compatibility paths.
- Docs explain the migration path from current first-class mission records and command handlers to the target shape.

## Evidence

- `rg` transcript shows no contradictory target guidance for `atelier mission ...`, mission focus, or built-in mission workflow.
- `atelier lint` passes.
