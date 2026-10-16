---
created_at: "2026-10-16T01:35:12.440876380+00:00"
id: "atelier-n7g1"
issue_type: "task"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-n7g1
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T18:24:38.989537800+00:00"
status: "done"
title: "Keep an approval current when the base is merged into a reviewed branch"
updated_at: "2026-10-16T18:24:38.989537800+00:00"
---

## Goal

Merge gates fast-forward only, so once another branch lands, every approved branch must merge the base in before it can merge. That merge brings in code the reviewer never saw (it was reviewed on its own branch), and the approval goes stale, so every branch reviewed in parallel needs a second approval round even when its own changes are untouched. Rebasing is worse (atelier-j48s).

## Done when

Approval freshness treats a merge commit whose second parent is the base, and whose merge result changes nothing beyond the two sides, as not touching the approved work, so the approval stays current; or the docs describe the second-approval cost and how reviewers check such merges. A test covers merging the base into an approved branch.

## Context

Found 2026-10-15 while landing the adoption fixes: by8u, 4n7v and others needed a re-approval after merging master.

## Close Reason

A merge commit whose other parents are on the base and whose tree equals git merge-tree's result of its parents keeps an approval current; the base commits it brings in are left out of the freshness walk, and an approval pinned before the merge still counts. Conflicted, hand-edited or non-base merges still stale it, and land on the base treats clean merges as introducing nothing. Tests: clean_merge_of_the_base_keeps_approval_but_resolved_or_edited_merges_do_not, approval_from_before_a_clean_base_merge_lands_on_both_branches. Proof still has to be rerun after merging. Documented in reviews-and-merge.md and ADR 0019.
