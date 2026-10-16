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
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Cover the other-branch row and the pruned not-done case in the merged label tests"
updated_at: "2026-10-16T20:19:53.166917955+00:00"
---

## Goal

Test the cases review found untested in atelier-lwdu's merged-label fallback.

## Done when

A test checks the other-branch row (branch_lines_for) for a kept branch whose owner was pruned from the base, and a test where the base has no copy of the record, the branch tip is in the base and the tip's record is not done shows not merged.

## Context

Deferred finding atelier-f1vy from the atelier-lwdu review (approval atelier-w9gp). crates/atelier-cli/src/commands/git.rs unmerged_done_label.
