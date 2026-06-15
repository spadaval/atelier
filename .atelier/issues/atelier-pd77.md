---
created_at: "2026-06-14T21:43:35.039004104+00:00"
id: "atelier-pd77"
issue_type: "epic"
labels:
- "product"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-11gp"
  - kind: "issue"
    id: "atelier-9sni"
  - kind: "issue"
    id: "atelier-l8r9"
  children:
  - kind: "issue"
    id: "atelier-kybc"
  - kind: "issue"
    id: "atelier-p2ph"
  - kind: "issue"
    id: "atelier-xzco"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Epic: Define mission workspace and epic review model"
updated_at: "2026-06-15T04:17:25.845678033+00:00"
---

## Description

Update Atelier's product contract so missions are worktree/workspace boundaries, epics are branch and review boundaries, and issues are implementation/accountability slices. Outcome: docs and domain language clearly describe the new hierarchy and remove the old per-issue worktree/per-issue review default. Evidence: docs diff plus review evidence maps the policy to mission, epic, issue, branch, worktree, review, validation, and cleanup behavior.

## Outcome

- Product docs, domain language, and ADRs define the new hierarchy: mission worktree, epic branch/review boundary, issue implementation slice.
- Command target state and migration behavior are specified before implementation work starts.
- Obsolete default guidance for per-issue worktrees and per-issue independent review is removed or classified as exceptional.

## Evidence

- Child issue proof from atelier-kybc, atelier-xzco, and atelier-p2ph maps to each epic outcome.
- Review artifact confirms product docs, ADR, and command target state are mutually consistent.
- `atelier mission status atelier-rxpr` shows this epic's children complete before this epic is closed.
