---
created_at: "2026-06-14T16:30:39.794539356+00:00"
id: "atelier-xzsm"
issue_type: "epic"
labels:
- "postmortem"
- "tracker"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-tqjn"
  - kind: "issue"
    id: "atelier-ux3k"
  - kind: "issue"
    id: "atelier-zrmo"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Prevent tracker workflow commands from self-blocking closeout"
updated_at: "2026-06-14T16:31:28.200345043+00:00"
---

## Description

Remove tracker-generated dirty-tree retry loops by making inspection read-only and scoping clean-worktree checks to non-tracker work.

## Outcome

- Read-only tracker inspection commands do not write activity records.
- Clean-worktree validators do not block solely because tracker workflow
  commands updated canonical `.atelier/` records.
- Closeout no longer enters retry loops caused by the tracker dirtying its own
  committed state during transition inspection or closeout.

## Evidence

- Focused tests or transcripts show `issue transition <id> --options` leaves
  the worktree unchanged.
- Focused tests or transcripts show sequential closeout is not blocked by
  canonical tracker records alone, while non-tracker dirty files still block.
- `git diff --check`, `atelier lint`, and relevant workflow tests pass.
