---
created_at: "2026-06-17T18:00:15.473104611+00:00"
id: "atelier-o97w"
issue_type: "task"
labels:
- "cli"
- "sessions"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add session begin, show, list, and end commands"
updated_at: "2026-06-17T18:00:15.473104611+00:00"
---

## Description

Add explicit session management commands for operators who need to start,
inspect, list, or end durable sessions.

## Outcome

- `atelier session begin` accepts role, optional issue or mission target, and
  optional subskill.
- `atelier session show`, `atelier session list --active`, and `atelier
  session end --reason` render concise operator output.
- Unknown roles or invalid targets fail with actionable messages.

## Evidence

- Focused CLI tests prove session command help, valid begin/show/list/end
  behavior, and invalid-role/invalid-target failures.
- Command transcript shows the targeted CLI session tests pass.
