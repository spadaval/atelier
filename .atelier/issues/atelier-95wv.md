---
created_at: "2026-06-17T18:00:18.307217828+00:00"
id: "atelier-95wv"
issue_type: "task"
labels:
- "sessions"
- "start"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Integrate start with session auto-creation, no-session, and explicit reuse"
updated_at: "2026-06-17T18:00:18.307217828+00:00"
---

## Description

Integrate `atelier start` with optional session auto-creation and explicit
session conflict handling.

## Outcome

- `atelier start <id>` creates a mutating worker session by default.
- `atelier start <id> --no-session` suppresses session creation.
- `atelier start <id> --reuse-session <session-id>` is required to reuse an
  active mutating session.
- Starting with a different active mutating session in the worktree fails by
  default with recovery guidance.

## Evidence

- Focused CLI tests prove auto-create, no-session, explicit reuse, and conflict
  failure output.
- Command transcript shows targeted `start` session tests pass.
