---
created_at: "2026-10-17T08:19:41.668652659+00:00"
id: "atelier-ytxg"
issue_type: "bug"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-ytxg
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
  - kind: "review"
    id: "atelier-6trx"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T09:11:15.692262634+00:00"
status: "done"
title: "Name the flag that shows the rest on every truncated list"
updated_at: "2026-10-17T09:11:15.692262634+00:00"
---

## Goal

`issue show --full`'s Impact list (limit 8) and the unassigned groups in `work missions` and `status` still end in `... N more` with no command that shows the rest, unlike the lists atelier-ksx6 fixed. Found while implementing atelier-ksx6.

## Done when

- Every `... N more` line names a command or flag that shows the rest, or the list is not cut.
- A rendering test covers each.

## Close Reason

Every truncated list now names the command that shows the rest, or is not cut: unassigned groups in status and work missions name `atelier work missions --all`, status's hidden-label count names `atelier work missions`, work missions --all also lifts the 20-mission and 5-item limits (folds in atelier-6trx), and issue show --full's Impact list is no longer cut. Rendering tests cover each.
