---
created_at: "2026-06-12T05:12:35.030572941+00:00"
id: "atelier-0u2k"
issue_type: "task"
labels:
- "cli"
- "reliability"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T23:11:41.000674030+00:00"
status: "done"
title: "Make CLI next actions context-aware and testable"
updated_at: "2026-06-12T23:11:41.000674030+00:00"
---

## Description

Make CLI next actions reliable, context-aware, and testable. Bad next actions
send agents into obsolete commands or irrelevant validators.
- Next actions include intent labels and current-state reasons.
- Next actions do not recommend removed, hidden, or internal commands.
- Next actions vary correctly for active work, blocked work, missing evidence,
  malformed issues, dirty worktrees, and ready work.
- Shared next-action helpers format actions but command code owns the context.
- Transcript tests cover next actions for no active work, active work, blocked
  work, malformed issues, missing evidence, dirty worktree, ready work, and
  closeout-ready work.

- Tests fail if removed commands appear in normal next-action output.

- Run focused CLI output tests that assert intent labels and current-state
  reasons are present.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
