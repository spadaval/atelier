---
created_at: "2026-10-17T19:00:36.339073169+00:00"
id: "atelier-n642"
issue_type: "feature"
labels:
- "merge"
- "retro-2026-10-17"
- "review"
priority: "P1"
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
status: "todo"
title: "Keep an approval current across a close commit that only touches the item's record"
updated_at: "2026-10-22T19:58:18.810454767+00:00"
---

## Goal

The required order is close, then approve, because `land` on master refuses a done claim made after the last approval (land_base_done_claim). That forced a fourth review round on sdb0 just to approve the close commit, and the memory rule 'approve after close'. A commit that only changes the item's own record (status, Close Reason) adds nothing a reviewer can judge.

## Done when

An approval pinned to commit A stays current for a later commit B when A..B changes only the item's own issue record, by status and Close Reason fields. `land` and `merge` agree on this. Tests cover the close-only commit and a close commit that also edits code (still stale).

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). Related: atelier-uw4x (merge refuses what land refuses).

## Unknowns

Whether any other record-only change (notes, links) should be covered too.
