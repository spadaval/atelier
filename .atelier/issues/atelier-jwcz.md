---
created_at: "2026-06-13T17:29:11.074323503+00:00"
id: "atelier-jwcz"
issue_type: "feature"
labels:
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9t3z"
  - kind: "issue"
    id: "atelier-fyms"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Rework ready queues and orientation surfaces for workflows"
updated_at: "2026-06-13T17:36:57.374944280+00:00"
---

## Description

Update agent orientation so configurable workflow state is visible and useful. Ready work should be based on startable configured transitions rather than hardcoded open status, and normal views should show both status category and exact workflow status.

## Outcome

- atelier status, issue show, issue list, and mission summaries use workflow categories and exact status IDs instead of hardcoded open and closed semantics.
- atelier issue list --ready returns issues with at least one currently passing transition into an in-progress-category status.
- atelier issue list --status accepts exact status IDs, category names todo/in_progress/done, and all.
- Missing workflow policy and unmigrated records keep read commands usable while making the next repair command obvious.

## Evidence

- Focused transcripts prove status, issue list, issue show, mission status, ready queue, missing-policy, and unmigrated-record outputs.
- Tests cover category filtering, exact-status filtering, ready calculation, blocked start transitions, and status output wording.
- atelier lint and atelier export --check pass.
