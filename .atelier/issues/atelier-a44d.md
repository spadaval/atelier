---
created_at: "2026-06-15T21:31:24.649072855+00:00"
id: "atelier-a44d"
issue_type: "feature"
labels:
- "mission"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9p3t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Move mission terminal checks into shared policy without mission workflow graphs"
updated_at: "2026-06-16T19:47:21.597466065+00:00"
---

## Description

Replace the hard-coded mission closeout subsystem with shared terminal-check policy while keeping mission lifecycle simple and built in.

## Outcome

- Mission lifecycle remains `draft`, `ready`, `active`, `closed` with start/focus semantics unchanged.
- Mission terminal checks are configured or declared through the shared validator policy used by workflow evaluation, not a separate closeout-specific status type.
- The implementation does not add arbitrary mission workflow graphs, mission review states, or a second parent acceptance-review issue type.
- Hard-coded closeout naming such as `MissionCloseoutStatus` and default mission closeout validators is removed or renamed to lifecycle-neutral terms.

## Evidence

- Focused tests show mission close or complete blocks on open linked work, open blockers, stale projection, blocking lint, dirty worktree, and incomplete explicit validation work.
- Focused tests show mission start/focus behavior still moves only between `ready` and `active`.
- `rg` over mission command internals shows no closeout-specific type or function names in live code.
- `atelier lint` and relevant cargo tests pass.
