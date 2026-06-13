---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-0006"
issue_type: "task"
labels:
- "config"
- "feature"
- "json"
- "mission-control"
- "spec"
- "validator"
priority: "P3"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0003"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T21:18:41.584015646+00:00"
status: "done"
title: "Implement atelier mission status CLI"
updated_at: "2026-06-11T21:18:41.584015646+00:00"
---

## Description

Add `atelier mission status [<mission-id>]` as the v1 mission-control surface.
With a mission ID, show mission health, ready/blocked/done/backlog work, open
blockers, evidence gaps, workflow validator failures, stale tracker/projection
state, active work/worktree context when available, and concrete action options.
Without a mission ID, use the active mission when one exists; otherwise show a
compact dashboard for open missions.

Status output should not prescribe a single "correct" next step. It should
present a bounded set of work options with context: what each ready issue/epic
would advance, what it would unblock, which validation/evidence gaps it helps
close, and why blocked or closeout options cannot proceed. Epic detail/status
surfaces should use the same option vocabulary where available so orchestrators
can choose work from either mission or epic context.

Quiet mode returns only essential IDs/counts suitable for shell composition. Do
not add command-result JSON.
CLI integration tests cover open mission status, active-mission
defaulting, option-oriented work selection context, evidence gaps, validator
failure or warning state, all-work-done-but-missing-closeout state, no-ID
dashboard mode, and quiet output.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
