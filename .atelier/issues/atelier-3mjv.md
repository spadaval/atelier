---
created_at: "2026-10-15T22:08:58.949610245+00:00"
id: "atelier-3mjv"
issue_type: "task"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-3mjv
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ox63"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-15T23:10:57.126172791+00:00"
status: "done"
title: "Decide how planning-only changes reach the base branch"
updated_at: "2026-10-15T23:10:57.126172791+00:00"
---

## Goal

Records live on branches, so filing, grooming, re-prioritising and strategy edits are commits too. Nothing says how they land. In practice the atelier repo commits them straight to master (148f3f8c, 547c761a), which `atelier merge` can't gate and the docs don't sanction. Re-prioritising on a work branch instead conflicts with other branches.

## Done when

The docs name the path for record-only changes: either "record-only commits may land on the base branch directly" with a `check` that the commit touches only `.atelier/` and strategies, or a lightweight gated path (for example `atelier plan commit`). `land` on the base branch accepts it.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Related: atelier-40i6, and the two ways to start work in atelier-kaor.

## Notes

- 2026-10-15 [human] Sumeet chose: record-only commits (records and mission strategies) may land on the base branch directly, with a check that they touch only those paths. Design note: the check runs in atelier land on the base branch (which the Stop hook runs) over unpushed base commits, not in atelier check. Commits covered by a recorded approval or trivial exemption count as reviewed; the commit that installs the workflow policy is exempt. Recorded in docs/product/reviews-and-merge.md (Planning changes on the base).

## Close Reason

Decided by Sumeet 2026-10-15: record-only commits (records and mission strategies) may land on the base branch directly. atelier land on the base checks unpushed base commits: each must be covered by a recorded approval or trivial exemption, or touch only records and strategies (land_base_unreviewed otherwise). Documented in docs/product/reviews-and-merge.md.
