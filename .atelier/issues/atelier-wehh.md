---
created_at: "2026-06-20T04:17:09.560361660+00:00"
id: "atelier-wehh"
issue_type: "task"
labels:
- "provider"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-l7i6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T05:11:16.173224031+00:00"
status: "done"
title: "Implement provider-owned terminal transition actions"
updated_at: "2026-06-20T05:11:16.173224031+00:00"
---

## Description

Imported bundle issue.

## Outcome

- Provider-backed workflows use provider PR merge as the integration authority.
- Local squash/merge behavior is not planned or executed for provider-backed terminal transitions.
- Terminal provider actions are explicit and ordered in workflow policy.

## Evidence

- Configured provider workflow plans terminal actions as tracker.commit, branch.push, review.merge, and base sync without local branch_integrate; tests cover action planning and recovery output.
