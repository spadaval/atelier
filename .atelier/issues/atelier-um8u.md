---
created_at: "2026-06-19T18:25:54.134784979+00:00"
id: "atelier-um8u"
issue_type: "epic"
labels:
- "implementation"
- "review"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-qsmn"
  children:
  - kind: "issue"
    id: "atelier-vt42"
  - kind: "issue"
    id: "atelier-xiqw"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Transition effect execution engine"
updated_at: "2026-06-19T20:14:27.732533480+00:00"
---

## Description

Implement the runtime that executes declared transition effects as part of an
explicit issue transition. This epic owns mutation sequencing, idempotency,
review effect execution, activity records, and recovery behavior.

## Outcome

- Successful issue transitions evaluate validators, preflight declared effects,
  execute supported effects, update canonical issue state, and record activity
  in a sequence that is auditable and recoverable.
- Local canonical changes are failure-aware: a failed effect does not silently
  report a completed transition, and recovery output names the local or external
  state that needs attention.
- The configured review-open effect creates or reuses the active review artifact
  through the review use-case boundary and links it to the branch-owning issue or
  epic.
- Review merge, approvals, comments, and provider-specific actions remain review
  artifact commands and never transition issues implicitly.

## Evidence

- Focused tests cover successful effect execution, failed preflight, failed
  effect recovery text, idempotent retry, activity records, and canonical issue
  state after success and failure.
- Review integration tests cover both native room mode and configured provider
  mode where available or explicitly document unsupported modes.
- `atelier lint atelier-um8u` passes after child work lands.
