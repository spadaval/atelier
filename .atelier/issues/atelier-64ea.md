---
created_at: "2026-06-12T20:29:11.166739249+00:00"
id: "atelier-64ea"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
- "command-surface"
- "implementation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-auqt"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-py2d"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T21:15:58.939395642+00:00"
status: "done"
title: "Fold issue helper flows into lifecycle commands"
updated_at: "2026-06-12T21:15:58.939395642+00:00"
---

## Description

Fold retained issue-local helper flows into lifecycle commands. The target is a
small normal `atelier issue` surface where common helpers are options on
`create`, `update`, or `list` instead of separate primary subcommands.

## Outcome

- Parented issue creation is available through `issue create --parent <id>`.
- Create-and-start behavior is available through `issue create --work` or the
  current approved replacement.
- Reopen, label, unlabel, and blocked-list behavior use lifecycle commands or
  their approved homes.
- Folded helper commands are absent from primary help or clearly marked
  non-normal according to the command-surface contract.
- Agent Factory and repository docs use the folded lifecycle forms.

## Evidence

- CLI transcript tests for parented create, create-and-start, reopen,
  label/unlabel, and blocked-list replacement behavior.
- Help transcript proving folded helpers are not presented as primary issue
  lifecycle commands.
- Docs or Agent Factory guidance diff using the replacement commands.
