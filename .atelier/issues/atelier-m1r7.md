---
created_at: "2026-06-16T16:18:26.360127183+00:00"
id: "atelier-m1r7"
issue_type: "validation"
labels:
- "cli"
- "commands"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T17:57:20.074566792+00:00"
status: "done"
title: "Validate cleaned command surface and recovery paths"
updated_at: "2026-06-16T17:57:20.074566792+00:00"
---

## Description

Independently validate that the cleaned command surface works as a product workflow and does not leave agents with hidden repair rituals. Start from the epic outcomes and command transcripts, not from the implementation summary.

## Outcome

- Normal worker, reviewer, manager, and admin help surfaces point to product commands and do not teach export/rebuild routines as ordinary workflow.
- Stale local projection state is repaired through `doctor` or `doctor --fix` without editing tracked canonical Markdown.
- Invalid canonical Markdown is reported as a tracked-file problem, not as a cache refresh problem.
- Any retained export/rebuild capability is hidden or admin-framed and has tests or transcripts showing its guardrails.
- Adjacent low-level commands have documented keep/change/defer classifications with follow-up issues for unresolved cleanup.

## Evidence

- Validation command transcript captures root help, role help, `doctor`, `doctor --fix`, and representative stale-state recovery output.
- Validation command transcript captures retained or removed export/rebuild behavior and proves tracked canonical records are not rewritten during ordinary repair.
- Search command transcript over product docs, Agent Factory guidance, and command audits classifies all remaining export/rebuild guidance.
- Evidence record maps each epic Outcome bullet to child proof and lists follow-up issue IDs for failed, blocked, or deferred claims.
- `atelier lint atelier-m1r7`, `atelier lint atelier-c4b8`, `atelier doctor`, focused command-surface tests, and `git diff --check` pass.

## Notes

The validator should not fix command behavior while validating unless a separate implementation issue is assigned.
