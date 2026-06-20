---
created_at: "2026-06-19T20:14:16.728481801+00:00"
id: "atelier-8cjb"
issue_type: "task"
labels:
- "artifact-update"
- "review"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-68sm"
  - kind: "issue"
    id: "atelier-wxj5"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T20:41:00.927811792+00:00"
status: "done"
title: "Define transition effects and review boundary contract"
updated_at: "2026-06-19T20:41:00.927811792+00:00"
---

## Description

Define the transition-effect contract and review boundary before implementation
work begins. This is an artifact-update task and should settle terminology,
schema intent, v1 built-ins, failure semantics, and the relationship to ADR
0011 review modes.

## Outcome

- Product and architecture docs define `transition effect` as configured work
  run by explicit issue transitions after validators pass.
- The v1 effect set is documented, including a review artifact open/link effect
  and explicit non-goals for review merge, approval, comments, hidden issue
  close, PR aliases, and broad automation hooks.
- Failure semantics cover preflight failure, local write failure, external
  provider failure, idempotent retry, and recovery text.
- The docs state which later implementation issues are blocked by the contract.

## Evidence

- Documentation diff covers `SPEC.md`, `CONTEXT.md`, product workflow/review
  docs, and any needed ADR update.
- Search transcript proves review commands are not described as workflow
  transition authority.
- `atelier lint atelier-8cjb` passes.
