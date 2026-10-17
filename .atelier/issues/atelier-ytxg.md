---
created_at: "2026-10-17T08:19:41.668652659+00:00"
id: "atelier-ytxg"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Name the flag that shows the rest on every truncated list"
updated_at: "2026-10-17T09:06:10.579920690+00:00"
---

## Goal

`issue show --full`'s Impact list (limit 8) and the unassigned groups in `work missions` and `status` still end in `... N more` with no command that shows the rest, unlike the lists atelier-ksx6 fixed. Found while implementing atelier-ksx6.

## Done when

- Every `... N more` line names a command or flag that shows the rest, or the list is not cut.
- A rendering test covers each.
