---
created_at: "2026-10-17T19:00:36.348421427+00:00"
id: "atelier-mvzi"
issue_type: "feature"
labels:
- "land"
- "orientation"
- "retro-2026-10-17"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Warn before pushing base commits made by another owner"
updated_at: "2026-10-22T20:55:59.294418773+00:00"
---

## Goal

k8h8's push to master twice published o3lm's unpushed records commits without either thread knowing (4eacb80e, 4d687d2b).

## Done when

When the base branch is ahead of its remote with commits that are not the current owner's, `land` (and `land --push`) names them before pushing. A test covers it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).

## Notes

- 2026-10-22 [decision] Another owner's base commit = a commit in <base>@{upstream>..<base> whose message names neither the current owner nor any issue in its scope (owner plus descendants); with no owner every such commit counts. It warns and does not refuse.
  Reason: Git authorship is the same for every agent session, and records carry no per-commit session identity; atelier's own start/close/merge/plan commits name the issue they concern, so the message is the one signal both records and history provide. Refusing would block a deliberate base push.
  Rejected: author/committer comparison (identical across sessions); touched-paths attribution (misattributes shared records edits); refusing
