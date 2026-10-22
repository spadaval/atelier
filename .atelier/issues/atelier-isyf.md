---
created_at: "2026-10-17T15:51:59.572033649+00:00"
id: "atelier-isyf"
issue_type: "task"
labels:
- "adoption"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T20:42:18.006540322+00:00"
status: "done"
title: "Pin merge checks to the merged checkout's own target directory"
updated_at: "2026-10-22T20:42:18.006540322+00:00"
---

## Goal

`shell.check` passes the caller environment through, so an exported `CARGO_TARGET_DIR` makes the merge-check builds share a target outside the merged checkout. Shared targets have run another branch's binary in tests here. Found in the atelier-qymu review (atelier-oi1z).

## Done when

- The nextest merge checks in `.atelier/workflow.yaml` set `CARGO_TARGET_DIR=target`, and the comment says why `.atelier` is excluded from the whitespace check.

## Notes

- 2026-10-22 [decision] Set CARGO_TARGET_DIR=target on the two nextest merge checks only, relative to the merged checkout
  Reason: The checkout is a fresh worktree, so target resolves inside it and is removed with it; an exported CARGO_TARGET_DIR would otherwise point every merge at one shared directory. The fmt and metadata checks build nothing.
  Rejected: Unsetting CARGO_TARGET_DIR: leaves a user-level CARGO_BUILD_TARGET_DIR style override in force; an explicit value is the same on every host

## Close Reason

The two nextest merge checks in `.atelier/workflow.yaml` now set `CARGO_TARGET_DIR=target`, and the comment says why `.atelier` is excluded from the whitespace check. Running all five policy commands in a detached checkout with a foreign `CARGO_TARGET_DIR` exported passed (816 and 820 tests), built in the checkout's `target/`, and never created the foreign directory.

### Deviations
None.

### Decisions
Scope of the variable: see the decision note on this item.

### Surprises
None. The comment's claim about `.atelier` comes from the evidence records, which end with blank lines that `git diff --check` flags.

### Open items
None. The merged checkout builds cold each run; atelier-rcnk decides cache reuse.
