---
created_at: "2026-06-19T20:14:27.731722829+00:00"
id: "atelier-vt42"
issue_type: "feature"
labels:
- "review"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-d8bt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T21:11:53.526596047+00:00"
status: "done"
title: "Add explicit review-open transition effect"
updated_at: "2026-06-19T21:11:53.526596047+00:00"
---

## Description

Implement the v1 explicit review effect that opens or reuses the configured
review artifact from an issue transition. This issue should reuse review
use-case boundaries instead of duplicating provider or room logic in the
workflow engine.

## Outcome

- A configured transition can open or reuse a review artifact and link it to the
  branch-owning issue or epic through the canonical review field.
- Native room mode and provider mode follow the review-mode contract; unsupported
  or misconfigured modes fail in preflight with actionable text.
- The effect respects branch owner, base/source branch, existing review link,
  and one-active-review expectations.
- `atelier review merge`, approvals, comments, and provider-specific commands
  still do not transition issue workflow.

## Evidence

- Focused tests prove review artifact creation or reuse, canonical review-field
  persistence, branch mismatch rejection, duplicate active review handling, and
  no implicit workflow transition from review commands.
- Command transcript shows transition output naming the review artifact opened
  or reused.
- `atelier lint atelier-vt42` passes.
