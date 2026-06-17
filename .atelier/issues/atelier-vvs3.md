---
created_at: "2026-06-17T18:00:20.778411303+00:00"
id: "atelier-vvs3"
issue_type: "task"
labels:
- "sessions"
- "status"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Show active sessions in status, man, and history surfaces"
updated_at: "2026-06-17T18:00:20.778411303+00:00"
---

## Description

Show active session context in normal operator surfaces without making sessions
the current-work source of truth.

## Outcome

- `atelier status` can show active session identity and target when present.
- `atelier man` mentions session commands where relevant without reviving
  removed legacy session/current-work guidance.
- `atelier history` can show session lifecycle activity when scoped to related
  work.

## Evidence

- Help/status/history transcript proves active sessions are visible and current
  work remains derived from issue workflow status.
- Command transcript shows focused status/man/history tests pass.
