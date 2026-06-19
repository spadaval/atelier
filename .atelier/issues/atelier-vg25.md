---
created_at: "2026-06-17T18:00:40.993898812+00:00"
id: "atelier-vg25"
issue_type: "task"
labels:
- "fields"
- "pr"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T01:01:40.505948257+00:00"
status: "done"
title: "Persist active PR metadata in the owning issue forge_pr field"
updated_at: "2026-06-18T01:01:40.505948257+00:00"
---

## Description

Persist active Forgejo PR metadata into the owning epic or standalone issue's
`forge_pr` typed field.

## Outcome

- `pr open` writes or confirms the owning issue's active `forge_pr` field.
- Child issue PR lookup inherits the nearest parent epic PR without duplicating
  the field.
- Branch, repo, and provider mismatches are visible in status or validator
  output.

## Evidence

- Focused CLI or RecordStore tests prove PR field write, child inheritance, and
  mismatch diagnostics.
- Command transcript shows targeted PR persistence tests pass.
