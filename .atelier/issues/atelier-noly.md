---
created_at: "2026-06-14T21:45:07.965981249+00:00"
id: "atelier-noly"
issue_type: "task"
labels:
- "branch"
- "cli"
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
status: "todo"
title: "Implement epic branch lifecycle commands"
updated_at: "2026-06-14T21:45:07.965981249+00:00"
---

## Description

Add epic branch helpers inside the mission workspace. Outcome: operators can create, switch to, inspect, and merge an epic branch that represents the reviewable changeset. Evidence: focused CLI tests prove branch naming, switching, status, merge failure handling, and clean branch/worktree checks.

## Outcome

- CLI can create, switch to, inspect, and merge an epic branch inside the mission workspace.
- Epic branch naming is stable and tied to the epic review boundary.
- Merge/status errors explain dirty worktree, missing branch, or wrong workspace problems.

## Evidence

- Focused CLI tests prove epic branch creation, switch, status, and merge behavior.
- Failure-path transcript shows corrective errors for dirty workspace or missing branch.
- `git branch` and `git status --short --branch` transcript confirms branch state after the command flow.
