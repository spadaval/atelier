---
created_at: "2026-10-17T01:10:57.964262389+00:00"
id: "atelier-5qep"
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
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Accept several values and --open in issue list filters"
updated_at: "2026-10-17T01:10:57.964262389+00:00"
---

## Goal

`atelier list` filters take a single value each: `list --label cli-output --label redesign` fails with a clap usage error (`the argument '--label <LABEL>' cannot be used multiple times`), and `list --label cli-output,redesign` prints `No open work matches.` although each label has open work (checked 2026-11-11 with the new binary in a converted scratch clone). The empty result does not name the filters given.

## Done when

- [ ] `--label` accepts repeats or a comma list. Values within one flag are ORed; different flags are ANDed.
- [ ] The empty result repeats the active filters, e.g. `No open work with label a or b.`
- [ ] CLI tests cover repeats, a comma list and the empty message.

## Context

Rewritten for the new tool in the cutover sort (atelier-5zre): `issue list` became `list`. The new `list` shows only open work by default and has no `--status`, `--category` or `--issue-type`, so `--open` and the status and category errors went with them.
