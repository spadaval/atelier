---
created_at: "2026-10-17T01:36:20.625620010+00:00"
id: "atelier-ydv0"
issue_type: "feature"
labels:
- "merge"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
  - kind: "issue"
    id: "atelier-tciw"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Add a merge strategy that merges a clean branch into the base and checks the merged tree"
updated_at: "2026-10-17T01:36:20.625620010+00:00"
---

## Goal

`branch_policy.merge_strategy: fast_forward_only` means a work branch can land only when it already contains the base. With several branches waiting, every landing forces the others to merge the base in again, re-run proof, and get any conflict resolution re-reviewed (feature/astromech-cmhv: 16 catch-up merges, 2026-10-16). GitHub handles this case differently when "require branches to be up to date" is off. It merges a branch that is behind if it has no conflicts and its CI ran on a test merge of the branch into the base. Atelier can do the same locally. Like GitHub, it would write a commit only when merging.

## Done when

- A base-policy merge strategy (for example `merge`) makes `atelier merge <id>` do the following:
  - build the merge of the work branch into the current base with `git merge-tree`;
  - refuse on any conflict, naming the files;
  - run the base policy's `merge_checks` on the merged tree, not on the branch head;
  - create the merge commit on the base only when the checks pass;
  - refuse if the base moves during the checks (`land_base_changed`).
- A clean merge of an approved branch keeps its approval, for the same reason as atelier-n7g1: it combines two reviewed changes, and nobody edited the result.
- The merged tree's proof is explicit. The `merge_checks` run on the merged tree is recorded, or the gate says what proves it, because evidence at the branch head does not.
- `fast_forward_only` stays available, and stays the default until the human decides otherwise.
- docs/product/reviews-and-merge.md stops saying "Atelier creates no commit". It says instead that merge creates the merge commit and nothing else.
- Tests cover three cases:
  - a clean merge lands;
  - a conflicting merge is refused;
  - a merged tree that fails `merge_checks` is refused. The motivating example is a Cargo.lock that git merges cleanly but `cargo metadata --locked` rejects.

## Context

Lockfile changes still need re-review and re-validation. This item adds no path exemptions (Sumeet, 2026-10-16), and the merged-tree checks are what catch a lockfile that merges cleanly but is broken. This is the second step after the merge-readiness view in status, which needs no change to the merge model. From the 2026-10-16 transcript retrospective.
