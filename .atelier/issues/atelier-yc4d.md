---
created_at: "2026-10-17T19:00:36.355573336+00:00"
id: "atelier-yc4d"
issue_type: "feature"
labels:
- "retro-2026-10-17"
- "start"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Let atelier start take an explicit base from any checkout"
updated_at: "2026-10-17T19:00:36.355573336+00:00"
---

## Goal

To record master as base, an orchestrator ran `git switch --detach master` and `start` refused with start_detached_head (o3lm 05:23Z). Separately, `start` records whatever branch is checked out as the base, so agents must create `<type>/<id>` from master first (memory rule 'start from a fresh type branch').

## Done when

`atelier start <id> --base master` works from a detached HEAD or any branch, creating the owner branch from the named base and recording it. Tests cover detached HEAD and a different current branch.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). Related: atelier-e151 (start creates the owner worktree).
