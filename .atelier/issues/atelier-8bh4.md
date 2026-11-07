---
created_at: "2026-11-07T05:01:00.329959594+00:00"
id: "atelier-8bh4"
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
status: "todo"
title: "Track post-merge work as loose ends, outside any mission"
updated_at: "2026-11-07T05:55:08.640075964+00:00"
---

## Goal

Replace after-merge checks (a marker on a Done when line, ADR 0027) with a `loose_end` issue type: work that changes no code and can only happen after a merge or a release, such as pushing a tag, a play-check on a device, or an eval run. A loose end waits on what it follows through `blocked_by`, shows up as ready when that merges, and is never part of a mission, so a mission closes when its code merges. The v0.3.1 tag was lost because it lived as prose in a Close Reason; astromech-cxgq (a play-check) shows the linked-issue form already works.

## Done when

- [ ] The starter policy and atelier's own define `loose_end`, and `loose_end` and `validation` share one workflow with no start, branch or review: `todo` to `done` on the base with a required one-line result, plus `supersede`. `atelier start` and `atelier merge` refuse both types, naming the transition to use instead.
- [ ] A loose end `blocked_by` an issue appears in `work ready` once that issue is merged, and in `work human` when labelled `needs-human` (tests).
- [ ] A loose end cannot advance a mission or be a mission's child (refused when the link is written), and mission close ignores loose ends.
- [ ] Removed: the After merge marker and stand-in parsing in new writes, `carried to`, the awaiting-validation rows and views, receipts held at close, the mission-close warning, and the after-merge part of the review request brief. Records that still carry the marker parse and show as ordinary Done when lines.
- [ ] `atelier migrate after-merge` (dry run unless `--apply`) turns each unticked After merge line into a loose end `blocked_by` its item, with the line's check as its Goal and Done when, and removes the line; it is idempotent and covered by a test.
- [ ] The agent-factory guidance is left to atelier-ml3r; docs/product, CONTEXT.md and ADR 0027's Status match the change.

## Scope

Workflow policy (starter and atelier's own), issue types, `start`/`merge` refusals, link validation, work views, review request brief, mission close, `migrate`, docs. Must not change: how ordinary work items merge. Other repositories keep working with the shared binary: policies without `loose_end` and records with the marker still parse; migrating them is atelier-ml3r's job.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 3), docs/adr/0027-proof-has-a-time.md (the machinery being removed), docs/strategies/simpler-lifecycle.md (Shape: Loose ends). What a loose end waits on after a change with no issue is atelier-4nsl's question, not this item's.
