---
created_at: "2026-10-17T06:11:21.436442713+00:00"
id: "atelier-rcnk"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
  - kind: "review"
    id: "atelier-gle3"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Clean up merged-tree checkouts after an interrupted merge"
updated_at: "2026-10-22T19:58:19.211837586+00:00"
---

## Goal

Under `merge_strategy: merge_commit`, `atelier merge` runs `merge_checks` in a temporary detached worktree under `<git-common-dir>/atelier-merge-*/tree`, removed by `Drop`. If merge is interrupted (Ctrl-C, kill during long checks), the worktree stays registered and its directory stays, so `git worktree prune` does not remove it. Each run also starts with no build cache, so a cargo check rebuilds from scratch. Found in review of atelier-ydv0 (finding atelier-gle3).

## Done when

- A later merge, `atelier prune`, or `doctor` finds and removes leftover merged-tree checkouts, shown by a test that leaves one behind.
- Whether merged-tree checks may reuse a build cache is decided and documented.
