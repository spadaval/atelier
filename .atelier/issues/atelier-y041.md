---
created_at: "2026-06-13T17:29:11.074101350+00:00"
id: "atelier-y041"
issue_type: "feature"
labels:
- "assignee:root"
- "workflow"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3z35"
  - kind: "issue"
    id: "atelier-9t3z"
  - kind: "issue"
    id: "atelier-ewpk"
  - kind: "issue"
    id: "atelier-fyms"
  - kind: "issue"
    id: "atelier-jwcz"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Replace issue transition engine"
updated_at: "2026-06-13T19:19:47.480806698+00:00"
---

## Description

Replace hardcoded issue transition readiness with configured workflow transitions. The normal operator surface should be atelier issue transition, with explicit execution by transition name and a rich options view for planning and blockers.

## Outcome

- atelier issue transition <id> <transition> executes an allowed configured transition and updates canonical issue status atomically.
- atelier issue transition <id> --options lists every transition from the current state with allowed or blocked status, validator results, guidance, and exact next commands.
- Configured built-in validators replace hardcoded start and close checks for issue sections, blockers, evidence, clean worktree, durable state, and open child work.
- Successful and blocked transition attempts create concise issue activity entries using workflow-specific event types.

## Evidence

- CLI tests cover successful transition, blocked transition, unknown transition, missing workflow policy, unmigrated status, validator failure, and guidance rendering.
- Activity sidecar proof shows transition_applied and transition_blocked summaries without creating evidence records.
- atelier lint and atelier export --check pass after transition tests.
