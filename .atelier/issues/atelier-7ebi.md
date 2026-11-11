---
created_at: "2026-10-17T01:10:57.965183217+00:00"
id: "atelier-7ebi"
issue_type: "feature"
labels:
- "cli-output"
- "issue-commands"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Confirm issue update, note and link with the resulting values"
updated_at: "2026-10-17T01:10:57.965183217+00:00"
---

## Goal

`atelier set` confirms what was run but not what the record now says, so agents read the record or run `show` straight afterwards. `atelier set --priority high <id>` prints `Set the fields of item <id>.` and the file it wrote, and never the new values (checked 2026-11-11 with the new binary in a scratch clone). Under the old tool, in astromech, 178 of 572 mutating issue commands were followed within two commands by `issue show`; in atelier, 48 of 347.

## Done when

- [ ] `set` prints only the changed fields, with old and new values: `priority: medium → critical`, `title: "A" → "B"`, `labels: +size-s −foo`, `blocked_by: +atelier-xmtr`. A field given its current value is not reported as a change.
- [ ] CLI tests cover each confirmation, including a field given its current value.

## Context

Rewritten for the new tool in the cutover sort (atelier-5zre): `issue update` became `set`; the `issue note` and `issue link` confirmations left with those commands, which the new tool does not have.
