---
created_at: "2026-06-12T05:12:30.096190618+00:00"
id: "atelier-l0yk"
issue_type: "task"
labels:
- "reliability"
- "tests"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-g18z"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Make ignored and stale tests visible blockers"
updated_at: "2026-06-12T05:12:30.096190618+00:00"
---

## Description

Make ignored and stale tests visible blockers instead of allowing them to hide
unfinished product behavior.

## Outcome

- Ignored tests must include a reason and owner or linked issue.
- Ignored tests for product behavior appear in closeout summaries.
- Stale tests that preserve obsolete behavior are either updated, deleted, or
  tied to an explicit migration issue.
- Validation agents inspect ignored tests before mission closeout.

## Evidence

- Add a test inventory or lint check for ignored tests.

- Run the inventory against the current suite and create follow-up issues for

unresolved ignored tests.

- Attach evidence to the reliability mission.
