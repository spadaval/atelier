---
created_at: "2026-06-12T05:12:05.838632139+00:00"
id: "atelier-8o34"
issue_type: "task"
labels:
- "reliability"
- "validators"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-diom"
  - kind: "issue"
    id: "atelier-pvuz"
  - kind: "issue"
    id: "atelier-w8e3"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Make mission and issue validators target-aware"
updated_at: "2026-06-12T05:12:05.838632139+00:00"
---

## Description

Replace generic validator defaults with target-aware validation policy. A
mission, issue, evidence record, and tracker health check should not all default
to the same durable-state validator.

## Outcome

- Validator selection depends on target kind and transition.
- Mission closeout validation checks linked work, blockers, evidence, issue
  structure, lint, export freshness, and worktree cleanliness where relevant.
- Issue start/close validation checks issue structure, blockers, and required
  evidence where relevant.
- Raw validator names are implementation details behind domain commands.

## Evidence

- Unit tests cover validator selection for issue start, issue close, mission
  closeout, evidence attachment, and tracker health targets.

- CLI transcripts prove mission and issue validation report concrete blocker
  classes such as malformed issue sections, open linked work, missing evidence,
  unresolved blockers, stale projection, and dirty worktree state where
  applicable.

- Run focused validator tests.
