---
created_at: "2026-06-17T20:03:31.616201608+00:00"
id: "atelier-aqqc"
issue_type: "validation"
labels:
- "docs"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-17T23:03:53.009622351+00:00"
status: "done"
title: "Update docs and tests after plan and milestone removal"
updated_at: "2026-06-17T23:03:53.009622351+00:00"
---

## Description

Validate the documentation, tests, and tracker health after first-class plan and
milestone removal. This is independent validation of the removal epic, not a
place to implement more deletion work.

## Outcome

- Docs, help, man pages, tests, fixtures, and canonical layout references agree
  that plans and milestones are not first-class records.
- Former plan/milestone behavior is either gone or explicitly marked historical
  in migration/audit notes.
- The mission graph has no open references requiring first-class plans or
  milestones for closeout.

## Evidence

- Evidence record maps removal claims to command transcripts and source search.
- Focused tests, `atelier lint`, `atelier export --check`, `atelier doctor`, and
  `git diff --check` pass.
