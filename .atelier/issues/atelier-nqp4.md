---
created_at: "2026-06-13T20:37:07.010119181+00:00"
id: "atelier-nqp4"
issue_type: "task"
labels:
- "cleanup"
- "data-model"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove issue body and description duplication"
updated_at: "2026-06-13T20:37:07.010119181+00:00"
---

## Description

Issue records have sectioned Markdown bodies, while the runtime model still carries description/body concepts and update paths that can imply large-field editing through command flags. Simplify ownership so issue Description, Outcome, Evidence, and Notes have one canonical source.

## Outcome

- Issue body sections are the source of truth for rich issue text.
- Projection/search stores only the metadata or derived index it needs, not a competing full-body field.
- issue update no longer exposes confusing description/body mutation paths unless the data-model contract explicitly keeps them.

## Evidence

- Focused tests prove direct Markdown edits, `atelier issue show`, `atelier search`, `atelier lint`, and `atelier rebuild` agree on the same section text.
- `rg` command output residue search for `Issue.description`, `records.body`, and description update paths is classified or removed.
- Help transcript and documentation diff prove command-line editing for large Markdown sections is no longer taught.
