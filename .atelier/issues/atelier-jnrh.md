---
created_at: "2026-11-07T05:01:00.003526374+00:00"
id: "atelier-jnrh"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-jnrh
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
status: "in_progress"
title: "Do work in one order: tick Done when, review, and let merge mark it done"
updated_at: "2026-11-07T06:18:35.610701336+00:00"
---

## Goal

Give the lifecycle one order that no source contradicts: start, implement, tick every Done when line and write the implementer report, review, fix and re-review, approve, merge. Merge marks the work done, so "done" means merged everywhere and there is no close step to put in the wrong place. Today the skill says close before review, the policy once had a review status, and `merge --check` says close after approval; every November orchestrator closed after approval.

## Done when

- [ ] `atelier review request` refuses while any Done when line of the open work items it covers is unticked, or their `Report` section is empty, and lists what is missing. A line carrying the old After merge marker is not required to be ticked; the refusal's message names `atelier migrate after-merge` for such lines (that command arrives with atelier-8bh4).
- [ ] `atelier merge` always creates a merge commit, and that commit sets every open item in the `work` workflow on the branch (the owner and its non-superseded children) to the workflow's first done status, keeping the `Report` the reviewer read. Merge refuses while one of them has an open blocker. Tests cover a standalone issue and a parent with children, and `land` passes on the base after each merge.
- [ ] `merge_strategy: fast_forward_only`, `squash` and a missing `merge_strategy` are all read as `merge_commit`, and `merge_commit` no longer needs a shell merge check (`merge_unproven` is removed). The starter policy and atelier's own say `merge_commit` or nothing.
- [ ] The `work` workflow has no `close` transition; `supersede` remains and writes the `Report` section. The required report parts are Agent Factory guidance, not policy: Atelier requires only a non-empty `Report`. No command output, policy description, hint or doc tells an agent to close a work item. An item already done on an unmerged branch (closed by an older binary) merges as it is.
- [ ] Docs and the mission close hint say a mission closes when all its work items are merged and its validation, if it has one, is done. This is described, not a new refusal.
- [ ] A change to a Done when line's text after start stays reported, as atelier-r9hb does today, including after approval and including a change merged in from the base (finding atelier-l5fb), in the review request brief and `merge --check`, without refusing; a change that only ticks boxes is no longer reported. Tests for each.
- [ ] PRODUCT_INTENT.md and ADR 0028 describe the mechanics as built: decision 2's open paragraph is replaced by them, including the merge-strategy tradeoff below and how today's "Atelier never creates commits" principle now reads.
- [ ] Removed, and listed in the implementer report: the `close` transition for work items, the policy's report `required_fields`, `fast_forward_only`, `merge_unproven`, the rule that a parent closes after its children, and the rules and messages about close order.

## Scope

Workflow policy (starter and atelier's own), transitions, `review request`, `merge`, `land`'s view of merge commits, mission close hints, `status`/`work` hints, docs/product, PRODUCT_INTENT.md, ADR 0028. Must not change: merge refuses without a current independent approval; a fix after review is re-reviewed. Other repositories keep working with the shared binary: old policies (`close` transitions with `required_fields`, `fast_forward_only`) and old records (`Close Reason`) still parse; updating their policies is atelier-ml3r's job.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 2 and its open mechanics), docs/strategies/simpler-lifecycle.md (Shape: One order). The `loose_end` type and the after-merge migration are atelier-8bh4's.

## Design

- One section, `Report`, says what happened, for every transition that requires the `close_reason` policy field: the implementer report on a work item (written by the author before requesting review), a supersede reason, a mission's close report, a decision's `decide`, and later a loose end's result (atelier-8bh4). The policy key `close_reason` keeps its name and now means that section. The old `Close Reason` heading is still read.
- Merge marks done by writing the status change into the merge commit it creates; nothing is committed on the branch.
- Merge strategy tradeoff, for Sumeet's after-merge read: always merging with a merge commit gives up linear history and the `merge_diverged` refusal in astromech, demiurge and cosplay-studio, and a repository without merge checks or CI on the merged tree now lands a merged tree nobody ran. In exchange no branch needs a catch-up merge before landing (astromech hit `merge_diverged` about 56 times since 2026-10-15). Rejected: committing the done state on top of the branch head and fast-forwarding, which keeps two strategies and adds a second kind of commit Atelier writes. atelier-ml3r tells those repositories.
