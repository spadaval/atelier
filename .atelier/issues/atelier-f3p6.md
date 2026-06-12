---
created_at: "2026-06-12T05:06:28.483421954+00:00"
id: "atelier-f3p6"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
- "rework"
- "work"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-trr2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Explode work command group into domain start finish and status surfaces"
updated_at: "2026-06-12T21:03:14.689647967+00:00"
---

## Description

Explode the current `atelier work` command group so ordinary operators do not
need to understand an abstract work/session namespace before starting,
finishing, or inspecting work.

## Outcome

- Root `atelier start <issue-id>` is the normal entrypoint for beginning work.
- A root or domain-specific finish command exists, or the mission records a
  deliberate decision to keep finish under a clearly named domain surface.
- Root `atelier status` and mission/issue status surfaces expose the common
  current-work answer, so `atelier work status` is no longer required for normal
  orientation.
- `atelier work start`, `atelier work finish`, and `atelier work status` are
  removed, hidden, or marked as advanced/internal compatibility according to the
  final CLI-surface policy.
- Worktree commands remain explicit advanced/orchestrator tools and are not
  required for the default start path.
- Help output and Agent Factory guidance stop listing the `work` group as a
  normal workflow.
- Tests prove old work commands are absent from primary help or clearly marked
  non-normal.

## Evidence

- CLI transcript tests prove root `atelier start <issue-id>` begins normal work
  and reports the active work state through the chosen status/current-work
  surface.

- Transcript tests prove finish or equivalent completion behavior is reachable
  without requiring normal users to discover `atelier work`.

- Help transcript tests prove `atelier --help` no longer recommends

`atelier work start`, `atelier work finish`, or `atelier work status` as

common commands.

- Docs and Agent Factory guidance name the new normal start, status, and
  finish/current-work surfaces and classify worktree commands as advanced.

- Run focused work/start/status CLI tests plus `atelier lint`.

## Notes

This does not remove the local runtime concept of active work. It removes the

confusing public command-group shape from the normal workflow.
