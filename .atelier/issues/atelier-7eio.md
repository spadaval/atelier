---
created_at: "2026-06-20T16:07:30.693708413+00:00"
id: "atelier-7eio"
issue_type: "task"
labels:
- "cutting-pass"
- "removal"
- "worktree"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Strip visible worktree feature pending redesign"
updated_at: "2026-06-20T16:48:41.105213519+00:00"
---

## Description

Remove the visible `atelier worktree` command family pending a future redesign.
The current implementation exposes buggy workspace setup, stale association
repair, and mission-worktree ownership behavior that duplicates issue/mission
status and branch workflow surfaces.

## Outcome

- `atelier worktree ...` is no longer a root command, helper, or documented
  normal workflow path.
- Issue and mission status surfaces remain responsible for current-work
  orientation.
- Epic branch helpers operate from the current checkout instead of requiring a
  mission-worktree owner marker.
- Product docs and command audit classify worktree management as deferred.

## Evidence

- `target/debug/atelier --help` does not list `worktree`.
- `target/debug/atelier worktree --help` fails as an unknown subcommand.
- Focused CLI tests cover removed worktree surface and current-checkout branch
  helper behavior.
