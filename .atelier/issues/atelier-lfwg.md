---
created_at: "2026-06-14T21:45:37.067812348+00:00"
id: "atelier-lfwg"
issue_type: "task"
labels:
- "cleanup"
- "migration"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove clean contained issue worktrees and stale registrations"
updated_at: "2026-06-14T21:45:37.067812348+00:00"
---

## Description

After classification and after the replacement mission-worktree/epic-branch commands exist, remove worktrees whose branch commits are already contained and whose checkout is clean, and prune stale registrations such as missing `.locks-cache` paths. Outcome: disk usage drops and final worktree status no longer includes clean contained issue worktrees or stale paths. Evidence: before/after worktree, disk-usage, and branch-containment transcripts.

## Outcome

- Clean contained issue worktrees are removed after classification.
- Stale Git worktree registrations are pruned or repaired.
- Final worktree status no longer includes removable issue worktrees or missing-path registrations.

## Evidence

- Before/after `git worktree list --porcelain` transcript shows removed and remaining entries.
- Before/after `du -sh /root/atelier/.atelier-worktrees` transcript shows disk impact.
- Branch containment transcript proves removed worktrees did not hold unique clean work.
