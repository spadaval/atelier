---
created_at: "2026-06-14T21:44:38.633749909+00:00"
id: "atelier-xzco"
issue_type: "task"
labels:
- "adr"
- "architecture"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-p2ph"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Record ADR for workspace, branch, and review boundaries"
updated_at: "2026-06-14T21:44:38.633749909+00:00"
---

## Description

Add an ADR that captures the policy choice: one worktree per mission, one branch per epic, issue review removed by default, and PR-equivalent review at epic scale. Outcome: future agents can distinguish workspace isolation, branch segmentation, and review scope without private context. Evidence: ADR diff and review note mapping alternatives and consequences.

## Outcome

- A new ADR records the workspace, branch, and review-boundary policy.
- The ADR explains alternatives considered, selected behavior, consequences, and cleanup implications.
- Future agents can identify when to use a mission worktree, epic branch, issue branch, or no extra workspace.

## Evidence

- ADR file diff contains the accepted policy and rejected alternatives.
- Review artifact or issue note maps the ADR to the mission validation criteria.
- `atelier lint atelier-xzco` passes.
