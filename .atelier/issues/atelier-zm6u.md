---
created_at: "2026-07-28T16:39:31.739752508+00:00"
id: "atelier-zm6u"
issue_type: "epic"
labels:
- "mission-control"
- "planning"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "draft"
title: "Support routine replanning and just-in-time expansion"
updated_at: "2026-08-05T16:43:27.127727601+00:00"
---

## Description

Make routine implementation replanning safe and obvious while preserving durable history and dependencies.

Known constraints: Managers may change the implementation graph within strategy; completed work and failed evidence remain intact; strategic changes route back to strategy rather than hiding in task edits.

Current unknowns: whether current issue mutations and lifecycle states are sufficient for superseding and reconciling speculative work.

Expansion condition: expand after the adaptive contract defines the required scenarios and the current-surface audit establishes the actual behavioral gaps.

## Outcome

- A Manager can add, split, resequence, block, defer, or supersede implementation work as evidence changes.
- Ready and blocked views immediately reflect the reconciled graph without a whole-plan resubmission or completeness check.
- Obsolete speculative work remains historically legible and no longer appears as current execution.
