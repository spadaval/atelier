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
closed_at: "2026-10-22T21:02:01.840976410+00:00"
status: "done"
title: "Let atelier start take an explicit base from any checkout"
updated_at: "2026-10-22T21:02:01.840976410+00:00"
---

## Goal

To record master as base, an orchestrator ran `git switch --detach master` and `start` refused with start_detached_head (o3lm 05:23Z). Separately, `start` records whatever branch is checked out as the base, so agents must create `<type>/<id>` from master first (memory rule 'start from a fresh type branch').

## Done when

`atelier start <id> --base master` works from a detached HEAD or any branch, creating the owner branch from the named base and recording it. Tests cover detached HEAD and a different current branch.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). Related: atelier-e151 (start creates the owner worktree).

## Notes

- 2026-10-22 [decision] The base named by start's base option must equal the policy's base branch (and the recorded base if one exists), is resolved as a local branch, and the owner branch is created from its pinned commit with records read from that commit
  Reason: land fails with land_base_mismatch unless branch_base is the trusted policy base, and kops requires owned branches not to trust unchecked local base commits; start's existing base-commit check and land's upstream trust point still apply to the local base
  Rejected: accepting any ref as base (records an untrusted base); remote-tracking refs (not the policy base name)

## Close Reason

`atelier start <id> --base <branch>` creates the owner branch from the named local branch and records it, from a detached HEAD or any current branch; tests cover both and the refusals.

### Deviations
`--base` is restricted to the policy's base branch (`start_base_mismatch` otherwise), because land requires the recorded base to equal it; a general base would record one land refuses. Plain `start` from a detached HEAD still refuses, now pointing at `--base`.

### Decisions
See the item's decision note. The base's records, not the checkout's, are read and checked, and the existing unreviewed-base-commit check still runs.

### Surprises
Default issue type here is task, so fixtures use task/<id>; the code is type-agnostic.

### Open items
Without `--base`, start still records any current branch as the base (only the wrong-prefix case is refused); the land_base_mismatch comes later for other names. Proposed follow-up: refuse a current-branch base that is not the policy base.
