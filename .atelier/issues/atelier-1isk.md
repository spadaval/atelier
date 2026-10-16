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
closed_at: "2026-10-16T19:50:48.880974470+00:00"
status: "done"
title: "Merging the base into an approved branch makes the approval stale"
updated_at: "2026-10-16T19:50:48.880974470+00:00"
---

## Goal

`merge_strategy: fast_forward_only` means a work branch must contain the base before `atelier merge`. When several owners are in flight, each merge to the base forces every other approved branch to merge the base in, and that merge commit (bringing in base code) makes the approval stale. Every integration then costs a fresh independent review of each other open branch, even when the merge is clean and the base commits were themselves reviewed and merged through the gates. Records-only base commits merged in do not trigger this; base code does.

## Done when

A merge commit whose non-record changes against its first parent are exactly the base's own reviewed changes (no conflict resolution in code, the second parent is a base commit that passed the gates) keeps the approval current, or has a lightweight recorded exemption distinct from `trivial`; a conflicted merge still needs review. Documented in reviews-and-merge.

## Context

Found 2026-10-16 orchestrating the astromech backlog with seven owners in flight. Worked around by dispatching a short independent review of each base merge (e.g. astromech-tpdb merge 40521f7, reviewer confirmed each side byte-identical to its parent's own changes). Related: approval freshness in docs/product/reviews-and-merge.md.

## Close Reason

A base merge keeps an approval when git merge-tree conflicts only in records, the merge matches merge-tree's result outside those conflicts, and every conflicted issue record is a structural three-way merge of its sides (fields take the side that changed them, relationship and label lists keep both sides' additions and removals, a field both changed differently needs review). In-scope records must change only in status in any base merge, including clean ones (closing an n7g1 gap). Tests: base_merge_record_resolutions_keep_approval_only_when_they_lose_nothing (union keeps; dropped side, edit while merging, clean base edit to the owner stale) and base_merge_resolutions_must_keep_the_blocker_and_child_the_base_added (correct resolution keeps; dropped blocker and dropped child stale). The astromech case (merge 40521f7) is the union shape. Documented in reviews-and-merge.md.
