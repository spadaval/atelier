---
created_at: "2026-06-19T03:58:25.483033697+00:00"
id: "atelier-5d7i"
issue_type: "feature"
labels:
- "review"
- "schema"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add structured review issue field and inheritance rules"
updated_at: "2026-06-19T03:58:25.483033697+00:00"
---

## Description

Add the structured issue review link used by room and provider review modes.
This issue is limited to canonical issue record parsing, rendering, validation,
and parent/child inheritance rules.

## Outcome

- Issue Markdown accepts `review.kind: room` with a room ID such as
  `atelier-r123`.
- Issue Markdown accepts `review.kind: pull_request` with provider and
  provider-local ID fields.
- Child issues inherit the nearest parent epic's review field, and defining a
  child-local review field is invalid.
- Human issue views show the effective review link and explain inherited review
  state without mutating child records.

## Evidence

- Focused tests cover valid room links, valid provider links, malformed review
  fields, inherited review fields, and rejected child-local fields.
- `atelier issue show <child-id>` fixture output demonstrates inherited review
  display.
- `atelier lint atelier-5d7i` and focused review-field tests pass.
