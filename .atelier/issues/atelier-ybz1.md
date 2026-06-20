---
created_at: "2026-06-20T16:43:31.075624955+00:00"
id: "atelier-ybz1"
issue_type: "task"
labels:
- "command-surface"
- "cutting-pass"
- "mission-collapse"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Sequence mission-specific command collapse work"
updated_at: "2026-06-20T16:48:31.291804899+00:00"
---

## Description

Coordinate the mission-specific command collapse after the target command
contract is documented. This issue owns sequencing, audit updates, and ensuring
dependent implementation issues are ordered rather than duplicating their
implementation details.

## Outcome

- The mission-collapse work is split into small implementation issues with
  explicit dependencies.
- Command-audit and product docs identify which mission commands are replaced,
  which replacement surfaces own the behavior, and which commands remain only
  temporarily.
- Final command removal waits for observable replacement behavior instead of
  preserving mission commands because the generic issue view is incomplete.

## Evidence

- `target/debug/atelier issue show atelier-4h62` shows small child issues with
  dependency blockers for the mission-collapse work.
- Command-audit docs name the replacement surface for each removed mission
  command before implementation removes it.
