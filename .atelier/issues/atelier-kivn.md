---
created_at: "2026-06-21T16:37:30.764931023+00:00"
id: "atelier-kivn"
issue_type: "task"
labels:
- "cleanup"
- "mission-rework"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-76j0"
  - kind: "issue"
    id: "atelier-f9ci"
  - kind: "issue"
    id: "atelier-y3fj"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove mission-specific lifecycle and focus code"
updated_at: "2026-06-21T16:37:30.764931023+00:00"
---

## Description

Delete mission-specific lifecycle, active focus, and terminal-state code once the configured workflow path owns the behavior.

## Outcome

- There is no command path or runtime state that sets active mission focus.
- Mission/objective close uses `atelier issue transition <id> close --reason ...` through normal workflow machinery.
- Root status derives objective context from current work and relationships, not a hidden pointer.

## Evidence

- Code search transcript shows removed mission focus and direct mission close paths.
- Regression tests cover status with zero, one, and multiple current missions/objectives.
