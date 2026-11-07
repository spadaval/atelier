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
status: "backlog"
title: "Track post-merge work as loose ends, outside any mission"
updated_at: "2026-11-07T05:01:00.329959594+00:00"
---

## Goal

Replace after-merge checks (a marker on a Done when line, ADR 0027) with a `loose_end` issue type: work that changes no code and can only happen after a merge or a release, such as pushing a tag, a play-check on a device, or an eval run. A loose end waits on what it follows through `blocked_by`, shows up as ready when that merges, and is never part of a mission, so a mission closes when its code merges. The v0.3.1 tag was lost because it lived as prose in a Close Reason; astromech-cxgq (a play-check) shows the linked-issue form already works.

## Done when

- [ ] The starter policy and atelier's own define `loose_end` with a workflow that has no start, branch or review: `todo` to `done` on the base, with a required one-line result. `atelier start` and `atelier merge` refuse a loose end.
- [ ] A loose end `blocked_by` an issue appears in `work ready` once that issue merges, and in `work human` when labelled `needs-human`.
- [ ] A loose end cannot advance a mission or be a mission's child.
- [ ] Removed: the After merge marker and stand-in parsing, `carried to`, awaiting-validation rows and views, receipts held at close, and the mission-close warning. Old records with the marker still parse, and `atelier migrate` turns each open marker line into a loose end.
- [ ] The `validation` type is done on the base without its own branch or review round.

## Unknowns

- Whether `validation` and `loose_end` share one workflow.
- The shape of the migration command.
