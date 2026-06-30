---
created_at: "2026-06-30T16:09:49.593046637+00:00"
id: "atelier-ms7i"
issue_type: "epic"
labels:
- "git"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-fs79"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-sszj"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Persist branch base context for workflow branches"
updated_at: "2026-06-30T16:09:49.593046637+00:00"
---

## Description

Make branch base context durable and use it consistently across transition planning and execution.

## Outcome

- Branch-owning work records or transition activity durably record owner issue id, work branch, branch base, and resolved review or integration target.
- Transition planning reads recorded branch base where available instead of recalculating every target from global `branch_policy.base_branch`.
- Branch context is visible in issue transition options and recovery guidance.
- The state model remains committed and reviewable; no hidden runtime branch ownership state is required for correctness.

## Evidence

- Tests prove branch base survives process restart or projection rebuild.
- Transition output shows resolved source and target branches for mission and epic workflows.
- `target/debug/atelier check atelier-ms7i` passes.
