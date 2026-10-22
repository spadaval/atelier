---
created_at: "2026-10-16T20:22:22.446783913+00:00"
id: "atelier-wgil"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-wgil
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-6dwg"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:33:32.352209145+00:00"
status: "done"
title: "Reject entries no side wrote in list resolutions of base merges"
updated_at: "2026-10-16T20:33:32.352209145+00:00"
---

## Goal

The three-way list merge in review_gate.rs checks only that a resolved list has the expected length and contains every expected entry, so with a duplicate already on one side a resolution can replace the duplicate with an entry nobody wrote (reproduced with hand-edited labels [dup, dup]).

## Done when

The list check also requires every resolved entry to be expected (or compares as sets), with a test for the duplicate case.

## Context

Deferred finding atelier-6dwg from the atelier-1isk review (approval atelier-c1ws). The CLI de-duplicates labels and links, so only hand-edited records can trigger it, and in-scope records stay status-only.

## Close Reason

The three-way list merge in review_gate.rs now also requires every resolved entry to be expected. Unit test list_resolutions_keep_both_sides_and_add_nothing_even_with_duplicates covers the [dup, dup] case (correct union accepted, a smuggled entry and a dropped side refused) and fails without the new condition.
