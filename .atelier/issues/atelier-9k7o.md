---
created_at: "2026-10-15T22:08:59.002443828+00:00"
id: "atelier-9k7o"
issue_type: "task"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-9k7o
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Decide whether the done claim should come after review"
updated_at: "2026-10-15T23:06:48.990194150+00:00"
---

## Goal

The close transition is committed on the work branch before review. Until the merge, that branch says done for work the base branch doesn't have, which misleads anyone reading another branch's records and turns a rejected review into a done → in_progress reversal.

## Done when

Sumeet decides between keeping the order (and making status output show "done on branch X, not merged"), or closing at merge time with review recording the claim. Recorded in the product docs.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Documented in the close transition's description, but easy to misread.

## Notes

- 2026-10-15 [human] Sumeet chose to keep the order: the done claim stays on the work branch before review. Status output now marks it: prime, status and other-branch rows show 'done on branch X, not merged into <base>' (or 'done, merged into <base>'). Recorded in docs/product/work-model.md (Completion) and sessions-and-hooks.md.
