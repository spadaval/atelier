---
created_at: "2026-06-15T05:22:37.177805855+00:00"
id: "atelier-nrwh"
issue_type: "task"
labels:
- "cleanup"
- "cli"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-t35w"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T06:09:53.446011720+00:00"
status: "done"
title: "Remove active-pointer cleanup command surfaces"
updated_at: "2026-06-15T06:09:53.446011720+00:00"
---

## Description

Remove command surfaces whose only product job is cleaning or steering a local active pointer. The status-derived current-work contract owns whether an operator still needs a durable pause or worktree repair workflow; this issue implements only the command-surface cleanup that follows from that contract.

## Outcome

- Root `atelier repair` is removed when its only job is stale active-pointer cleanup.
- Root `atelier abandon` is removed or replaced only if the status-derived contract defines a durable workflow/status operation for pausing work.
- `atelier worktree repair <id>` is removed or re-scoped if it only repairs stale runtime work_association rows instead of repo-owned worktree state.
- Help, man pages, and command lists route operators to status, issue transitions, notes, evidence, and worktree-owned recovery surfaces rather than active-pointer cleanup.

## Evidence

- Help transcripts prove removed root active-pointer cleanup commands are absent or rejected.
- File review shows any remaining worktree repair behavior is scoped to repo-owned worktree state rather than runtime work_association rows.
- Focused CLI tests or transcripts cover removed command rejection and any replacement pause/recovery behavior named by the contract.
- `atelier lint atelier-nrwh` and `atelier export --check` pass.
