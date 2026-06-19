---
created_at: "2026-06-19T20:14:33.233248283+00:00"
id: "atelier-wxj5"
issue_type: "task"
labels:
- "docs"
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
status: "todo"
title: "Align review and transition docs with explicit effects"
updated_at: "2026-06-19T20:14:33.233248283+00:00"
---

## Description

Align product docs, role guides, and help text with the explicit transition
effect model and the ADR 0011 review boundary.

## Outcome

- Docs and help teach `atelier issue transition` as the only workflow movement
  surface for issue status changes.
- Docs and help teach `atelier review` as review artifact discussion, approval,
  request-change, resolve, and merge tooling.
- Any obsolete `pr` or review-command lifecycle guidance is removed rather than
  kept as compatibility guidance.
- The CLI surface document describes transition effects and the review-open
  effect without promising unsupported generic automation.

## Evidence

- Search transcript over docs, role guides, help text, and issue records proves
  no conflicting review-transition guidance remains.
- Docs/help parity check or focused CLI help transcript covers the affected
  surfaces.
- `atelier lint atelier-wxj5` passes.
