---
created_at: "2026-10-22T19:58:07.645851921+00:00"
id: "atelier-c3kh"
issue_type: "feature"
labels:
- "merge"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-c3kh
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-2yl7"
  - kind: "issue"
    id: "atelier-isyf"
  - kind: "issue"
    id: "atelier-p189"
  - kind: "issue"
    id: "atelier-rcnk"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T20:56:13.355577560+00:00"
status: "done"
title: "Merge a moved base without hand work"
updated_at: "2026-10-22T20:56:13.355577560+00:00"
---

## Goal

A branch whose base moved lands without a hand-built catch-up. Merge proceeds when the base moved only by records-only commits. `atelier merge --sync` brings the base in, re-proves and reports whether the approval survived. Merged-tree checks build in their own target directory and leave no checkouts behind. New repositories with a test check start on `merge_commit`, if the human decides so.

## Done when

- Each child's Done when holds, shown by merge tests, with independent review.
- Delivers the merge-default part of atelier-k8h8 S3 (atelier-2yl7) and the merge-housekeeping part of S11 (atelier-isyf, atelier-rcnk), and removes the catch-up macro behind S6 (atelier-p189, atelier-s2ei).

## Background

The children change `crates/atelier-cli/src/commands/merge.rs`, `init` and `.atelier/workflow.yaml`. atelier-p189, atelier-isyf and atelier-rcnk can start at once. atelier-2yl7 waits on its decision record. atelier-s2ei reports approval survival, so it waits on atelier-n642's rule.

## Close Reason

### Outcome
`atelier merge` handles a moved base without hand work. Under merge_commit, a base that moves only by records-only commits during the checks gets its gates rerun on the new base (atelier-p189). Merge checks build in the merged checkout's own target directory (atelier-isyf). Leftover merged-tree checkouts from an interrupted merge are swept at the next merge (atelier-rcnk). A policy without merge_strategy defaults to merge_commit when its merge_checks include a shell.check, and to fast_forward_only otherwise; `atelier init` follows the same rule (atelier-2yl7, per decision atelier-6mgg). Delivers the merge-default part of atelier-k8h8 S3 and the merge-housekeeping part of S11.

### Deviations
atelier-s2ei (`merge --sync`) was taken out of this increment on 2026-10-22. It reports whether an approval survived, so it waits on atelier-n642's approval-currency rule. Keeping it here would have held this reviewed work off the base, so it will run on its own branch once n642 lands. Under fast_forward_only no retry can land; merge_diverged names whether the base gained only records or names the code commit (atelier-p189's decision note).

### Decisions
See the decision notes on atelier-p189, atelier-isyf, atelier-rcnk and atelier-2yl7.

### Surprises
An omitted merge_strategy used to parse as squash, which merge refuses. With --json, the first retry design printed two JSON documents; the review caught it (atelier-grmb).

### Open items
The vendored skill reference change (.agents/skills/agent-factory/references/atelier.md) must be synced to the upstream skill repository. A binary older than 6d983eb5 reads a new repository's omitted strategy as squash: reinstall after merge.
