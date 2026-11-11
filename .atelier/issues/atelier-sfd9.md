---
created_at: "2026-10-17T01:10:57.971403328+00:00"
id: "atelier-sfd9"
issue_type: "feature"
labels:
- "check"
- "cli-output"
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
title: "Give a repair route for links to missing records that arrive by merge or hand edit"
updated_at: "2026-11-07T21:03:24.979790291+00:00"
---

## Goal

`atelier check` names the file and field of a link to a record that does not exist (`.atelier/items/atelier-5qes/item.md: blocked_by: \`atelier-zzzz\` names no item`) and ends `Fix each in the file named`, but does not say the edit that removes it (checked 2026-11-11 with the new binary in a converted scratch clone). Such links arrive by a merge or a hand edit.

## Done when

- [ ] The finding names the edit that removes the link, e.g. `remove atelier-zzzz from blocked_by, or run atelier set --blocked-by <the others> atelier-5qes`.
- [ ] Tests cover a never-existed target written by hand.

## Context

Narrowed in decision atelier-mzpn to links that reach the records without a command. atelier-kset owns the one-line finding format; share it.

Rewritten for the new tool in the cutover sort (atelier-5zre): the new `check` already names the file and field and prints shared advice once; `issue unlink` is gone, and `set --blocked-by` is the command the specification gives.
