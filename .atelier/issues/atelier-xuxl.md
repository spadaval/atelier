---
created_at: "2026-06-19T03:58:30.973607248+00:00"
id: "atelier-xuxl"
issue_type: "task"
labels:
- "migration"
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Migrate legacy pull_request fields and reject old shape"
updated_at: "2026-06-19T03:58:30.973607248+00:00"
---

## Description

Migrate existing provider review links from the legacy `pull_request` issue
field to the new structured `review` field and make the old shape invalid after
migration.

## Outcome

- Existing `pull_request: <number>` fields become `review.kind: pull_request`
  records with provider and ID information.
- Migration preserves provider-local review numbers and issue/epic ownership.
- New or remaining legacy `pull_request` fields fail lint/rebuild after the
  migration path has run.
- Error messages point users to the structured `review` field and
  `atelier review ...` commands.

## Evidence

- Migration fixture proves old records are rewritten to the structured review
  shape.
- Negative test fixture proves old `pull_request` fields are rejected after
  migration.
- Command output from `rg -n 'pull_request:' .atelier/issues docs` shows no
  active legacy field usage remains outside migration fixtures.
