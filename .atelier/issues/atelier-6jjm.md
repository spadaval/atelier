---
created_at: "2026-06-20T04:17:09.563653841+00:00"
id: "atelier-6jjm"
issue_type: "task"
labels:
- "review-room"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-l7i6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T05:11:48.852329077+00:00"
status: "done"
title: "Keep local branch integration as room-only workflow action"
updated_at: "2026-06-20T05:11:48.852329077+00:00"
---

## Description

Imported bundle issue.

## Outcome

- Local branch integration remains available for local review-room workflows.
- `branch_integrate` is an explicit opt-in workflow action, not generic terminal-close behavior.
- Room-mode status and transition output make the local integration authority clear.

## Evidence

- Room-mode workflow still plans and executes an explicit local branch integration action; provider-mode tests prove the same action is absent.
