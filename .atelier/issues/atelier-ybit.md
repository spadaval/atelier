---
created_at: "2026-06-14T16:30:36.976645140+00:00"
id: "atelier-ybit"
issue_type: "epic"
labels:
- "postmortem"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-jxs8"
  children:
  - kind: "issue"
    id: "atelier-c4uz"
  - kind: "issue"
    id: "atelier-c9eo"
  - kind: "issue"
    id: "atelier-mllk"
  - kind: "issue"
    id: "atelier-wbed"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T17:27:28.181310139+00:00"
status: "done"
title: "Epic: Make closeout workflow-driven instead of heuristic proof matching"
updated_at: "2026-06-14T17:27:28.181310139+00:00"
---

## Description

Replace heuristic text matching and mission contract-audit close gates with workflow-owned proof approval and aggregate mission closeout.

## Outcome

- Issue and mission closeout no longer use token-overlap or fuzzy matching
  against free-form Outcome, Evidence, or Validation prose as a hard gate.
- Proof sufficiency is owned by workflow transitions, validation/review issues,
  and attached evidence records.
- Mission closeout is aggregate: linked work done, blockers clear, and
  configured health gates pass.

## Evidence

- Product/workflow docs describe the new closeout model and do not present
  mission contract-audit matching as a required close gate.
- Focused tests or transcripts show a closeout path where explicit validation
  workflow approval is sufficient and text-token matching is not consulted.
- `git diff --check`, `atelier lint`, and relevant focused workflow/mission
  closeout tests pass.
