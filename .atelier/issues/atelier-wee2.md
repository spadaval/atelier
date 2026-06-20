---
created_at: "2026-06-20T13:13:20.928125033+00:00"
id: "atelier-wee2"
issue_type: "task"
labels:
- "review"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Make review.complete gate provider approvals before terminal merge"
updated_at: "2026-06-20T13:13:20.928125033+00:00"
---

## Description

Provider-backed workflow currently treats `review.complete` as "the linked
Forgejo PR is already merged." That forces work to merge before the close
transition, even though close then runs `tracker.commit`, `branch.push`,
`review.merge`, and `base.sync`. The result is a late close tracker commit that
can land after provider merge and miss the base branch.

Change provider `review.complete` to gate on review approval/readiness before
terminal merge. Approval and request-changes remain explicit review commands;
close remains the merge boundary.

## Outcome

- Provider `review.complete` accepts an open linked Forgejo PR with matching
  source/target branches and at least one approving review.
- Provider `review.complete` rejects PRs with no approval or an active
  request-changes review.
- Terminal provider close still owns `tracker.commit`, `branch.push`,
  `review.merge`, and `base.sync`.
- Reviewer/operator guidance shows `atelier review approve` and
  `atelier review request-changes` as the review decision loop.
- Docs describe `review.complete` as approval/readiness for provider workflows,
  not pre-merged state.

## Evidence

Record focused test and documentation validation evidence before closing.
