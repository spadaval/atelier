---
created_at: "2026-06-17T18:00:03.988050823+00:00"
id: "atelier-x1fn"
issue_type: "task"
labels:
- "fields"
- "forgejo"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Define forge_pr typed field for one active Forgejo PR"
updated_at: "2026-06-17T18:00:03.988050823+00:00"
---

## Description

Define the first concrete typed field, `forge_pr`, for the one active Forgejo
pull request attached to an epic or standalone issue.

## Outcome

- `forge_pr` requires provider, host, owner, repo, number, url, source branch,
  and target branch.
- Child issues inherit PR visibility from the nearest parent epic rather than
  duplicating the field.
- Invalid or mismatched `forge_pr` values produce actionable lint or validator
  failures.

## Evidence

- Manual check of workflow policy documentation or config fixture file content
  shows the `forge_pr` field definition.
- Focused tests cover valid, missing-key, and child-inheritance PR field
  behavior.
