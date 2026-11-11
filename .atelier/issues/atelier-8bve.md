---
created_at: "2026-10-17T01:10:57.962045771+00:00"
id: "atelier-8bve"
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
title: "Make issue create confirm the record and suggest the next step"
updated_at: "2026-10-17T01:10:57.962045771+00:00"
---

## Goal

`atelier new "Title"` confirms `Created draft item atelier-4d3x, with empty Goal, Done when and Scope sections to write.`, the file it wrote and the next steps, but never the title or priority it wrote (checked 2026-11-11 with the new binary in a converted scratch clone).

## Done when

- [ ] The confirmation names the item's title and priority with its ID, e.g. `Created draft item atelier-xxxx, medium: "Title"`.
- [ ] An item created with no mission gets a next step that puts it in one, `atelier set --mission <mission-id> <id>`, with the real mission ID when exactly one mission is open.
- [ ] CLI tests cover an item created with `--in <mission>` and one with no mission.

## Context

Rewritten for the new tool in the cutover sort (atelier-5zre): `issue create` became `new`. The new `new` already names the status and the `ready` and commit steps, so those parts left, and its missing step toward a mission stays, as `set --mission`; `--parent`, the type and the unpersisted-ID error went with parents, types and the old command.
