---
created_at: "2026-10-15T23:49:16.351350337+00:00"
id: "atelier-fj1t"
issue_type: "bug"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-fj1t
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T18:35:17.362482031+00:00"
status: "done"
title: "Refuse done claims for branchless work on the base"
updated_at: "2026-10-16T18:35:17.362482031+00:00"
---

## Goal

`atelier issue transition <id> start` and then `close` on the base branch takes a work item to done without an owner branch, so it never meets the review and merge gates. Now that record-only commits may land on the base (atelier-3mjv), that commit passes `land`, which is the astromech scenario in atelier-kaor item 6.

## Done when

`land` on the base (or the close transition) refuses a new done claim for work that has no recorded owner branch, with guidance to use `atelier start`; superseding work and closing decision records (atelier-ox63) on the base stay allowed. Tests cover all three.

## Context

Found 2026-10-15 in review finding atelier-e91j on atelier-kaor.

## Close Reason

land (and start) on the base refuse an unpushed commit outside review that takes a work item to done, with land_base_done_claim and guidance to move it to a work branch and atelier start it; closing missions, deciding decision records and superseding work stay allowed. For merges only a status that differs from every parent counts. Test: base_refuses_done_claims_outside_review_but_allows_supersede_and_decisions. Documented in reviews-and-merge.md.
