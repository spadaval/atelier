---
created_at: "2026-10-22T22:48:09.267114646+00:00"
id: "atelier-fngz"
issue_type: "task"
labels:
- "land"
- "merge"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-fngz
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-1hjj"
  - kind: "issue"
    id: "atelier-976s"
  - kind: "issue"
    id: "atelier-o2pm"
  - kind: "issue"
    id: "atelier-x5j3"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Close the push, lease, sync and merge-report follow-ups"
updated_at: "2026-10-22T22:48:10.311971513+00:00"
---

## Goal

Four low-to-medium follow-ups found in review of this mission's first wave land on one branch with one review, instead of four full cycles: `land --push` refuses an upstream that does not resolve (atelier-976s), the session lease write and refresh are safe (atelier-x5j3), `merge --sync` reads plainly when resuming and when the approval is left stale (atelier-o2pm), and `merge` prints its report only from the attempt that returns (atelier-1hjj).

## Done when

- Each child's Done when holds, shown by its tests, with one independent review of the branch.
- Delivers the remaining hardening of atelier-k8h8 S10 and S11.

## Background

The children touch land in crates/atelier-cli/src/commands/session.rs, start and status lease code, merge_sync.rs and merge.rs. They are independent of each other; do them in the listed order. Branch atelier-r28p is changing the claim-check block in merge.rs, so keep atelier-1hjj's change to the report-printing path.
