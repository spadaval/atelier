---
created_at: "2026-06-14T21:45:34.212318406+00:00"
id: "atelier-zbd4"
issue_type: "task"
labels:
- "audit"
- "migration"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-hrmj"
  - kind: "issue"
    id: "atelier-lfwg"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Classify existing issue worktrees and branches"
updated_at: "2026-06-14T21:45:34.212318406+00:00"
---

## Description

Inventory current .atelier-worktrees entries, codex/atelier-* branches, dirty state, contained/uncontained commits, duplicate branch heads, and stale Git registrations. Outcome: a durable classification table identifies delete, preserve, fold into epic branch, or investigate actions for every existing worktree and branch. Evidence: command transcript and evidence record attached to this issue.

## Outcome

- Every current `.atelier-worktrees` entry and `codex/atelier-*` branch is classified as delete, preserve, fold into epic branch, or investigate.
- Dirty state, contained/uncontained commits, duplicate heads, disk usage, and stale Git registrations are recorded.
- Cleanup tasks have enough evidence to remove only safe worktrees and preserve useful work.

## Evidence

- Command transcript captures `git worktree list --porcelain`, branch containment checks, dirty counts, duplicate heads, and `du -sh`.
- First-class evidence record attaches the classification table to this issue.
- `atelier lint atelier-zbd4` passes.
