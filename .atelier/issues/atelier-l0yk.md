---
created_at: "2026-06-12T05:12:30.096190618+00:00"
id: "atelier-l0yk"
issue_type: "task"
labels:
- "assignee:root"
- "reliability"
- "tests"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-g18z"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-eoaq"
    role: "validates"
  - kind: "evidence"
    id: "atelier-mc6x"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Make ignored and stale tests visible blockers"
updated_at: "2026-06-12T21:54:00.991501933+00:00"
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

- Add a test inventory or lint check that lists ignored tests, reasons, linked
  owners, and whether each ignored test covers product behavior.

- Run the inventory against the current suite and create follow-up issues for

unresolved ignored tests.

- Attach evidence to the reliability mission showing unresolved ignored or stale
  tests are either linked to owners or no longer block product closeout.
