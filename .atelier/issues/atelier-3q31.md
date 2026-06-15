---
created_at: "2026-06-14T21:45:06.223542241+00:00"
id: "atelier-3q31"
issue_type: "task"
labels:
- "cli"
- "worktree"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ccja"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T22:29:19.348796986+00:00"
status: "done"
title: "Implement mission worktree lifecycle commands"
updated_at: "2026-06-14T22:29:19.348796986+00:00"
---

## Description

Add mission-oriented worktree commands that create or locate one workspace for a mission and prepare local runtime state there. Outcome: mission worktree path and branch naming are stable, status shows mission workspace ownership, and setup does not create per-issue worktrees. Evidence: focused CLI tests and transcript prove create/locate/status behavior and clean error messages.

## Outcome

- CLI can create or locate one mission worktree for a mission.
- Mission worktree setup prepares local runtime state without creating per-issue worktrees.
- Mission worktree status reports mission ownership, branch, path, dirty state, and useful cleanup guidance.

## Evidence

- Focused CLI tests prove mission worktree create-or-locate behavior and idempotency.
- Command transcript proves a new mission worktree rebuilds local runtime state.
- Command transcript or test proves issue work does not create `.atelier-worktrees/<issue-id>` by default.
