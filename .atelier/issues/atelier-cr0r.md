---
created_at: "2026-10-16T20:47:51.689801276+00:00"
id: "atelier-cr0r"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-cr0r
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-641q"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-vfgr"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:52:11.606748054+00:00"
status: "done"
title: "Use the nearest parseable earlier copy in the base done-claim check"
updated_at: "2026-10-16T20:52:11.606748054+00:00"
---

## Goal

When an earlier copy of a record does not parse, base_done_claim falls back to the pushed copy. If the item was done upstream, set back locally, broken and restored as done, the pushed copy wrongly says it was already done.

## Done when

The check walks back to the nearest parseable unpushed copy before the pushed one, and tests cover that sequence and the case the fallback exists for (done upstream, broken locally, restored unchanged lands).

## Context

Deferred findings atelier-vfgr and atelier-641q from the atelier-hyew review (approval atelier-6dro). crates/atelier-cli/src/commands/git.rs base_done_claim.

## Close Reason

When an earlier copy does not parse, base_done_claim walks back through unpushed commits touching the record to the nearest parseable copy, then the pushed copy. Test base_done_claim_check_reads_the_nearest_parseable_earlier_copy: restored unchanged lands; set back, broken and restored as done is refused (and passes wrongly when the walk is removed).
