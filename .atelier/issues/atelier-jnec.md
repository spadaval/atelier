---
created_at: "2026-10-17T19:45:46.135278410+00:00"
id: "atelier-jnec"
issue_type: "bug"
labels:
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "review"
    id: "atelier-2tbo"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T23:25:05.165529848+00:00"
status: "done"
title: "Keep the debrief's history when HEAD is a work branch that merged the base"
updated_at: "2026-11-04T23:25:05.165529848+00:00"
---

## Goal

`issue_histories` in crates/atelier-app/src/record_history.rs walks HEAD's first-parent line, so on a work branch that merged master in, every master commit since the branch point collapses into that merge: the debrief moves the mission's start, and loses scope added after the start and reopened items, with no warning. Found by review atelier-2tbo on atelier-rx22.

## Done when

A debrief run from a work branch that merged the base reports the same start and deviations as one run from the base (walk the base's first-parent line, then the branch's own), shown by a CLI fixture.

## Notes

- 2026-11-04 [decision] Take the base from the workflow policy's base_branch and walk its first-parent line only up to the merge base with HEAD, then HEAD's own line (fork..HEAD); with no such branch, or on the base, walk HEAD alone as before
  Reason: The debrief describes HEAD, so base commits HEAD lacks stay out; the policy's base is what every debrief shares, and a missing ref keeps fixtures and detached checkouts working
- 2026-11-04 [decision] Keep the branch's own commits from before its merge of the base in the walk, after the base line
  Reason: They hold the branch's own record changes, which the merge's first-parent diff does not repeat; dropping them would lose those changes, and a stale state only misleads when the base and the branch both changed one record before the merge

## Close Reason

`issue_histories` now walks the workflow policy's base branch's first-parent line up to the merge base with HEAD, then HEAD's own first-parent line (`fork..HEAD`), with positions continuing across both. On the base, or when the base branch is missing, it walks HEAD alone as before. The new CLI fixture `a_debrief_from_a_branch_that_merged_the_base_keeps_the_base_history` forks a branch before the mission starts, starts the mission on main, adds scope and reopens an item there, then merges main into the branch. The branch's debrief now shows the same start line, deviations and quiet counts as main's. The fixture failed before the fix: the added scope and the reopened item were missing.

### Deviations
None.

### Decisions
Two decision notes on this item: the base comes from `branch_policy.base_branch` and is walked only up to the merge base, and the branch's own commits from before its merge of the base stay in the walk.

### Surprises
None.

### Open items
A branch commit made before the branch merged the base reads records as the branch had them. If the base and the branch both changed one record before that merge, the branch's older state follows the base's newer state in the walk. That could read as a reopened item or move the mission's close. This needs both lines to edit the same record before the merge, and no test covers it.
