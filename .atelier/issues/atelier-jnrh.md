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
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Do work in one order: tick Done when, review, and let merge mark it done"
updated_at: "2026-11-07T05:01:00.003526374+00:00"
---

## Goal

Give the lifecycle one order that no source contradicts: start, implement, tick every Done when line, review, fix and re-review, approve, merge. Merge marks the issue done, so "done" means merged everywhere and there is no close step to put in the wrong place. Today the skill says close before review, the policy once had a review status, and `merge --check` says close after approval; every November orchestrator closed after approval.

## Done when

- [ ] `atelier review request` refuses while any Done when line is unticked and lists those lines.
- [ ] After `atelier merge`, the merged issue is done on the base in the merge commit, carrying the implementer report the reviewer read.
- [ ] The work workflow has no close transition (supersede remains), and no command output, policy description, hint or doc tells an agent to close a work item.
- [ ] A mission closes when all its work items are merged; nothing about post-merge work holds it.
- [ ] PRODUCT_INTENT.md and ADR 0028 describe the mechanics as built.
- [ ] Removed, and listed in the close report: the close transition for work items and the rules and messages about close order.

## Unknowns

- Where the implementer report lives until merge: a section the author writes before requesting review, or input to `merge`.
- How a parent and its children are marked done when one merge integrates them.
- What happens to items already closed on unmerged branches when this ships (old-form reads, since one binary serves every repository).
