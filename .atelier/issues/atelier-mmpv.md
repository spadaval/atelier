---
created_at: "2026-11-07T05:00:04.971587750+00:00"
id: "atelier-mmpv"
issue_type: "mission"
labels: []
strategy:
  path: docs/strategies/simpler-lifecycle.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Take one issue from start to merge in one obvious order, with nothing but code undoing a review"
updated_at: "2026-11-07T05:00:04.971587750+00:00"
---

## Outcome

An agent takes one issue from start to merge in one obvious order: start, implement, tick every Done when line, review, fix and re-review, approve, merge. An approval goes stale only when code changes, merge marks the issue done, post-merge work is a loose end outside the mission, records are checked when written and correctable by their author until they reach the base, and only the records and gates review needs remain. Independent review before merge, and re-review of fixes, are unchanged. The strategy, docs/strategies/simpler-lifecycle.md, owns the outcome, success criteria, shape and authority.
