---
created_at: "2026-06-12T01:54:59.376705533+00:00"
id: "atelier-c2oj"
issue_type: "task"
labels:
- "cli"
- "docs"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T03:37:44.576687346+00:00"
status: "done"
title: "Document file-first Markdown issue editing workflows"
updated_at: "2026-06-12T03:37:44.576687346+00:00"
---

## Description

Document the preferred file-first issue authoring workflow wherever agents are
likely to look. The workflow is: create a valid issue shell with
`atelier issue create`, edit the generated canonical Markdown file directly,
then run focused lint and any relevant workflow validation.
- `AGENTS.md` describes file-first issue editing for this repository.
- `AGENTFACTORY.md` describes when to use `issue create` plus Markdown editing
  instead of shell-quoted long descriptions.
- The agent-factory skill planning/tracker guidance is updated so coordinated
  agents can discover the workflow outside this repository.
- Product docs link the direct-edit contract from an operator-facing guide, not
  only the storage spec.
- Examples cover creating issues, parented issues, epics, dependencies, and
  validation after direct edits.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
