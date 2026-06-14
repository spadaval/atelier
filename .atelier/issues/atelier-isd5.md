---
created_at: "2026-06-14T02:50:18.340424688+00:00"
id: "atelier-isd5"
issue_type: "task"
labels:
- "docs"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add transcript-derived operator command map"
updated_at: "2026-06-14T02:50:18.340424688+00:00"
---

## Description

Create or update product docs with the supported normal command paths for
mission, issue, worktree, evidence, health, recovery, setup, and cross-record
graph flows after the linked CLI and proof-gate contracts are stable. Include
removed or intentionally absent commands only as corrective notes when the logs
show repeated confusion.

## Outcome

A future agent can choose the supported operator command without guessing
removed names such as workflow check/init, finish, archive, session, timer,
current-work, issue new, top-level dep, generic link, import-beads, export,
rebuild, or integrations.

## Evidence

- File diff shows the docs update linked from the product or docs index.
- Examples are checked against current CLI help after the relevant command
  surface tasks land.
- Search transcript shows the docs do not recommend removed or intentionally
  absent command names except as corrective "use this instead" notes.
- `git diff --check` and `atelier lint` pass.
