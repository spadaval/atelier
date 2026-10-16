---
created_at: "2026-10-16T18:47:46.535528279+00:00"
id: "atelier-v9ve"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-v9ve
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:55:20.936569915+00:00"
status: "done"
title: "Warn that rebase recovery drops commits made after the rebase"
updated_at: "2026-10-16T19:55:20.936569915+00:00"
---

## Goal

The 'pins a commit outside the work branch' recovery (reset to the pre-rebase tip, then merge the base) loses commits made after the rebase, such as a fix and new proof. Say to cherry-pick them back after the reset.

## Done when

The error in crates/atelier-app/src/review_gate.rs and docs/product/reviews-and-merge.md say to cherry-pick commits made after the rebase onto the restored tip; the rebase recovery test covers a post-rebase commit.

## Context

Deferred finding atelier-eix5 from the atelier-j48s review (approval atelier-0c5b).

## Close Reason

The 'pins a commit outside the work branch' error and reviews-and-merge.md say to cherry-pick commits made after the rebase back onto the restored tip and to record review events made since the rebase again, since they pin rewritten commits. The rebase recovery test now makes a proof commit after the rebase, follows the advice, merges, and finds that proof on the base.
