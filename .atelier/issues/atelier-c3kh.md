---
created_at: "2026-10-22T19:58:07.645851921+00:00"
id: "atelier-c3kh"
issue_type: "feature"
labels:
- "merge"
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
  - kind: "issue"
    id: "atelier-s2ei"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Merge a moved base without hand work"
updated_at: "2026-10-22T19:58:17.133950054+00:00"
---

## Goal

A branch whose base moved lands without a hand-built catch-up. Merge proceeds when the base moved only by records-only commits. `atelier merge --sync` brings the base in, re-proves and reports whether the approval survived. Merged-tree checks build in their own target directory and leave no checkouts behind. New repositories with a test check start on `merge_commit`, if the human decides so.

## Done when

- Each child's Done when holds, shown by merge tests, with independent review.
- Delivers the merge-default part of atelier-k8h8 S3 (atelier-2yl7) and the merge-housekeeping part of S11 (atelier-isyf, atelier-rcnk), and removes the catch-up macro behind S6 (atelier-p189, atelier-s2ei).

## Background

The children change `crates/atelier-cli/src/commands/merge.rs`, `init` and `.atelier/workflow.yaml`. atelier-p189, atelier-isyf and atelier-rcnk can start at once. atelier-2yl7 waits on its decision record. atelier-s2ei reports approval survival, so it waits on atelier-n642's rule.
