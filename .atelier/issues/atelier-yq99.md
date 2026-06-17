---
created_at: "2026-06-17T20:04:13.438105158+00:00"
id: "atelier-yq99"
issue_type: "task"
labels:
- "artifact-update"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-7uug"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Document workflow crate ownership contract"
updated_at: "2026-06-17T22:18:30.697709586+00:00"
---

## Description

Document the intended ownership boundary for workflow policy and readiness
semantics. The contract should make `atelier-workflow` the source of parsing,
status categories, transition lookup, and validator semantics, while app and CLI
layers consume those APIs.

## Outcome

- Architecture docs identify workflow responsibilities by crate.
- The docs explain which behavior is domain policy, which behavior is app
  orchestration, and which behavior is terminal rendering.
- Follow-on implementation issues have clear acceptance criteria.

## Evidence

- File-change review of workflow architecture docs maps current duplicated
  behavior to the target owner.
- `atelier lint` and `git diff --check` pass.
