---
created_at: "2026-10-16T19:32:40.068953229+00:00"
id: "atelier-p2f3"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-p2f3
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-f1vy"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:21:46.082360757+00:00"
status: "done"
title: "Cover the other-branch row and the pruned not-done case in the merged label tests"
updated_at: "2026-10-16T20:21:46.082360757+00:00"
---

## Goal

Test the cases review found untested in atelier-lwdu's merged-label fallback.

## Done when

A test checks the other-branch row (branch_lines_for) for a kept branch whose owner was pruned from the base, and a test where the base has no copy of the record, the branch tip is in the base and the tip's record is not done shows not merged.

## Context

Deferred finding atelier-f1vy from the atelier-lwdu review (approval atelier-w9gp). crates/atelier-cli/src/commands/git.rs unmerged_done_label.

## Close Reason

New test pruned_owner_labels_cover_other_branch_rows_and_an_uncommitted_status checks the other-branch row in status for a kept branch whose owner was pruned from the base (merged), and that an uncommitted status the tip does not have shows not merged though the tip is in the base and the base has no copy. Removing the tip-status condition or the fallback each fails it. The not-done case uses superseded, since the label is computed only for done-category owners.
