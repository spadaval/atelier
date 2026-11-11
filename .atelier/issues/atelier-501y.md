---
created_at: "2026-10-17T01:10:57.967000630+00:00"
id: "atelier-501y"
issue_type: "feature"
labels:
- "cli-output"
- "start"
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
title: "Show the item's title, Done when and mission when start succeeds"
updated_at: "2026-11-07T21:03:26.730432682+00:00"
---

## Goal

A successful `atelier start` prints the item's title, status and unticked Done when lines, but not the mission the item belongs to (checked 2026-11-11 with the new binary in a converted scratch clone).

## Done when

- [ ] Success output names the item's mission, `Mission: atelier-1w8c "…"`, when it has one.
- [ ] CLI tests cover an item in a mission and one in none.

## Context

Dropped in decision atelier-mzpn: a warning when the recorded base differs from the policy base, since atelier-md0v reads the base from the policy everywhere and `start --base` is hidden and ignored.

Rewritten for the new tool in the cutover sort (atelier-5zre): the new `start` already prints the title and Done when; `-q` went, since the new tool has no quiet form and the branch is named for the item, and the child-of-a-parent case went with parents.
