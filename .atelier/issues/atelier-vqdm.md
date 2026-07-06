---
created_at: "2026-06-29T20:15:29.393741030+00:00"
id: "atelier-vqdm"
issue_type: "task"
labels:
- "cli"
- "complexity"
- "diagnostics"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Give hidden diagnostics explicit ownership decisions"
updated_at: "2026-06-29T20:15:29.393741030+00:00"
---

## Description

Hidden diagnostics such as workflow rebuild, export/import, bead migration, and low-level diagnostic commands should each have a clear owner decision instead of lingering as undocumented escape hatches.

## Outcome

Each hidden diagnostic or migration command has a Keep, Hide, Fold, or Remove decision recorded in the command audit. Surviving commands are reachable from explicit diagnostics, migration, or recovery guidance only, and stale references are removed from normal operator docs.
