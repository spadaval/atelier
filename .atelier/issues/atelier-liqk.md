---
created_at: "2026-06-14T02:52:30.599229887+00:00"
id: "atelier-liqk"
issue_type: "task"
labels:
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Suggest supported replacements for removed commands"
updated_at: "2026-06-14T02:52:30.599229887+00:00"
---

## Description

Improve errors or help for common removed/likely command names observed in logs, without reintroducing compatibility behavior.

## Outcome

Commands such as workflow check, finish, current-work, issue new, archive, session, timer, and work start point to the supported workflow or help surface.

## Evidence

- Focused CLI tests or transcripts assert suggestions for representative
  removed command names including at least `workflow check`, `finish`,
  `current-work`, `issue new`, and `work start`.
- The suggestions name supported commands or help surfaces and do not execute
  compatibility aliases or old-command shims.
- `git diff --check`, `atelier lint`, and the focused command-suggestion tests
  pass.
