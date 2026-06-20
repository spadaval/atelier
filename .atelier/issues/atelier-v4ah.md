---
created_at: "2026-06-19T22:42:56.468829940+00:00"
id: "atelier-v4ah"
issue_type: "task"
labels:
- "review"
- "validators"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T01:07:59.273579968+00:00"
status: "done"
title: "Make review.complete validate actual review completion"
updated_at: "2026-06-20T01:07:59.273579968+00:00"
---

## Description

Replace the current status-category based review-complete check with a read-only check of the configured review artifact. Room mode should require merged room state. Provider mode should require the provider review artifact state required by policy.

## Outcome

- `review.complete` validates the configured review artifact state instead of inferring completion from issue status or category.
- Room-backed and provider-backed review modes each use their configured completion signal.

## Evidence

- Tests fail when an issue merely reaches a review or validation status without a complete review artifact.
- Tests pass when the configured review artifact is complete.
- Recovery text points to review status/show commands.
