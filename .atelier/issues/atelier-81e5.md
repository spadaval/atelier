---
created_at: "2026-10-17T19:00:36.355034861+00:00"
id: "atelier-81e5"
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
status: "in_progress"
title: "Warn when another live session holds the item being started"
updated_at: "2026-10-22T21:12:58.938483589+00:00"
---

## Goal

Two sessions worked the same items: another session ran `start atelier-92mz` in the main checkout (10-16 18:10), and a resumed session edited the 37uq worktree, forcing a handoff (8 cross-session messages).

## Done when

`atelier start` and `atelier status` report when another live session or worktree holds the item (by a runtime lease under `.atelier/runtime`), and `start` refuses without `--take-over`. Tests cover a live and a stale lease.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).

## Notes

- 2026-10-22 [decision] Lease liveness: a lease is a JSON file under the primary checkout's .atelier/runtime/leases/<owner>.json (shared by all worktrees via the git common dir), live while its worktree exists and its heartbeat is under 4 hours old; start sets it and releases the worktree's other leases, status renews it from the holder; no PID
  Reason: start is short-lived so a PID would be the agent harness's, which Atelier cannot know and which PIDs reuse; a worktree path is checkable, a deleted worktree or a new owner there expires the lease at once, and a crashed session ages out. The branch is not consulted because git already forbids a second checkout of a branch, so a lease that required the branch checked out could never be taken over
  Rejected: PID liveness; worktree lock files; per-worktree .atelier/runtime (invisible to other worktrees)
