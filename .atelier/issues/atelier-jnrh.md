---
created_at: "2026-11-07T05:01:00.003526374+00:00"
id: "atelier-jnrh"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4yqd"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Do work in one order: tick Done when, review, and let merge mark it done"
updated_at: "2026-11-07T05:54:44.769901755+00:00"
---

## Goal

Give the lifecycle one order that no source contradicts: start, implement, tick every Done when line and write the implementer report, review, fix and re-review, approve, merge. Merge marks the work done, so "done" means merged everywhere and there is no close step to put in the wrong place. Today the skill says close before review, the policy once had a review status, and `merge --check` says close after approval; every November orchestrator closed after approval.

## Done when

- [ ] `atelier review request` refuses while any Done when line of the items it covers is unticked, or their implementer report is missing, and lists what is missing.
- [ ] `atelier merge` always creates a merge commit, and that commit marks every open work item on the branch (the owner and its children) done on the base, keeping the implementer report the reviewer read. A test covers a standalone issue and a parent with children.
- [ ] `merge_strategy: fast_forward_only` is retired: a policy that names it is read as `merge_commit`, and `merge_commit` no longer needs a shell merge check (`merge_unproven` is removed). The starter policy and atelier's own say `merge_commit` or nothing.
- [ ] The work workflow has no close transition (supersede remains). No command output, policy description, hint or doc tells an agent to close a work item. An item already done on an unmerged branch (closed by an older binary) merges as it is.
- [ ] A mission closes when all its work items are merged and its validation, if it has one, is done; no loose end holds it.
- [ ] A change to a Done when line's text after start stays reported, as atelier-r9hb does today, including after approval and including a change merged in from the base (finding atelier-l5fb), in the review request brief and `merge --check`, without refusing; a change that only ticks boxes is no longer reported. Tests for each.
- [ ] PRODUCT_INTENT.md and ADR 0028 describe the mechanics as built, including how "Atelier creates no commits except the merge commit" now reads.
- [ ] Removed, and listed in the implementer report: the close transition for work items, `fast_forward_only`, `merge_unproven`, the rule that a parent closes after its children, and the rules and messages about close order.

## Scope

Workflow policy (starter and atelier's own), transitions, `review request`, `merge`, mission close, `status`/`work` hints, docs/product, PRODUCT_INTENT.md, ADR 0028. Must not change: merge refuses without a current independent approval; a fix after review is re-reviewed. Other repositories keep working with the shared binary: old policies (`close` transitions, `fast_forward_only`) and old records still parse; updating their policies is atelier-ml3r's job.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 2 and its open mechanics), docs/strategies/simpler-lifecycle.md (Shape: One order). The loose-end type is atelier-8bh4's; leave validation and loose-end workflows to it.

## Design

- The implementer report keeps its place in the item record, written by the author before requesting review. The section is named `Report`; the old `Close Reason` heading is still read and is what `supersede` writes. Its required parts follow the policy, as today.
- Merge marks done by writing the status change into the merge commit it creates; nothing is committed on the branch.
