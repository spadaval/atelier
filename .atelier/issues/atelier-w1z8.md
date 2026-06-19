---
created_at: "2026-06-19T19:39:44.792922880+00:00"
id: "atelier-w1z8"
issue_type: "feature"
labels:
- "git"
- "prune"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-iq7f"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Prune stale branches and removable worktrees"
updated_at: "2026-06-19T19:39:44.792922880+00:00"
---

## Description

Implement pruning for stale Git branches and removable worktrees according to
the documented branch/worktree safety contract.

## Outcome

- Dry-run and apply paths identify branches and worktrees owned by closed or
  archived work and explain protected branches/worktrees.
- Apply refuses dirty worktrees, protected base branches, current branch,
  unmerged branches, and branches linked to open or active work unless the
  contract defines an explicit override.
- Successful cleanup reports removed worktree paths and deleted branch refs.

## Evidence

- Focused tests or scripted repository scenarios prove protected branch,
  current branch, dirty worktree, unmerged branch, and eligible closed-work
  cleanup behavior.
- Command transcript captures dry-run and apply output plus `git diff --check`.
