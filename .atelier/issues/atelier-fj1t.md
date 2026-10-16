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
status: "in_progress"
title: "Refuse done claims for branchless work on the base"
updated_at: "2026-10-16T18:31:17.626533827+00:00"
---

## Goal

`atelier issue transition <id> start` and then `close` on the base branch takes a work item to done without an owner branch, so it never meets the review and merge gates. Now that record-only commits may land on the base (atelier-3mjv), that commit passes `land`, which is the astromech scenario in atelier-kaor item 6.

## Done when

`land` on the base (or the close transition) refuses a new done claim for work that has no recorded owner branch, with guidance to use `atelier start`; superseding work and closing decision records (atelier-ox63) on the base stay allowed. Tests cover all three.

## Context

Found 2026-10-15 in review finding atelier-e91j on atelier-kaor.
