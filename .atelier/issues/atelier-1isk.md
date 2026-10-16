---
created_at: "2026-10-16T19:07:52.053881845+00:00"
id: "atelier-1isk"
issue_type: "feature"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-1isk
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Merging the base into an approved branch makes the approval stale"
updated_at: "2026-10-16T19:45:08.711475769+00:00"
---

## Goal

`merge_strategy: fast_forward_only` means a work branch must contain the base before `atelier merge`. When several owners are in flight, each merge to the base forces every other approved branch to merge the base in, and that merge commit (bringing in base code) makes the approval stale. Every integration then costs a fresh independent review of each other open branch, even when the merge is clean and the base commits were themselves reviewed and merged through the gates. Records-only base commits merged in do not trigger this; base code does.

## Done when

A merge commit whose non-record changes against its first parent are exactly the base's own reviewed changes (no conflict resolution in code, the second parent is a base commit that passed the gates) keeps the approval current, or has a lightweight recorded exemption distinct from `trivial`; a conflicted merge still needs review. Documented in reviews-and-merge.

## Context

Found 2026-10-16 orchestrating the astromech backlog with seven owners in flight. Worked around by dispatching a short independent review of each base merge (e.g. astromech-tpdb merge 40521f7, reviewer confirmed each side byte-identical to its parent's own changes). Related: approval freshness in docs/product/reviews-and-merge.md.
