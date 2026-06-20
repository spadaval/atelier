---
created_at: "2026-06-19T22:42:56.478429537+00:00"
id: "atelier-0d5k"
issue_type: "task"
labels:
- "actions"
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Migrate review artifact behavior to review actions"
updated_at: "2026-06-20T00:19:58.241129890+00:00"
---

## Description

Migrate review artifact open/link behavior to namespaced review actions such as `review.open` and `review.link`. Preserve idempotent retry and provider/room mode behavior.

## Outcome

- Review artifact open/link behavior is driven by namespaced workflow actions rather than legacy review artifact effect identifiers.
- Re-running a review transition remains idempotent for both provider-backed and room-backed review modes.

## Evidence

- Transition tests prove `review.open` opens or reuses the configured review artifact.
- Docs and workflow files contain no `review_artifact_open` or `review_artifact_link` identifiers.
