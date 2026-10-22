---
created_at: "2026-10-15T23:13:58.925778094+00:00"
id: "atelier-lwdu"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-lwdu
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-96n4"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:29:52.232719580+00:00"
status: "done"
title: "Show merged done claims as merged after prune"
updated_at: "2026-10-16T19:29:52.232719580+00:00"
---

## Goal

The not-merged label on done owners (atelier-9k7o) reads the base branch's copy of the owner record. Once prune removes that record from the base, a kept work branch that did merge shows "not merged".

## Done when

When the base has no copy of the record but the record was once in the base's history and the branch tip is in the base, `prime`, `status` and the other-branch rows say merged; an uncommitted close on a fresh branch still says not merged. A test covers a pruned owner.

## Context

Review finding atelier-96n4 on atelier-9k7o. Only affects work branches that branch cleanup kept.

## Close Reason

When the base has no copy of the owner record, unmerged_done_label counts the claim as merged if the branch's committed record has that status and the branch tip is an ancestor of the base, so prime, status and other-branch rows say merged for a kept branch after prune; an uncommitted close on a fresh branch still says not merged. Test: a_kept_branch_whose_done_owner_was_pruned_from_the_base_shows_merged (fails without the change).
