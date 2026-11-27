---
schema: atelier.item
schema_version: 3
id: atelier-cpfd
title: Show what a later review round cost in the pace line, from one verdict to the next, not only the reviewer's time
status: building
mission: atelier-oqsz
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Aad94841c-cb71-4522-99ce-e39e71555f54%3Adelegate-task%3Aoqsz-impl-cpfd
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T08:59:58.479123042Z
---

## Goal

`atelier-f0sn` gave a lead's `status` a line of review pace. Its time is
the reviewer's alone, from a review's `created_at` to its verdict. On
`hub-v2-bg14` that reads "a round took 15m at the median, 31m at most",
while a second round cost the mission 33 to 86 minutes from the first
verdict to the second, and found nothing to fix on any item. The human
asked for the line because "your last five reviews took over an hour each
and caught nothing" would have changed behaviour. With the reviewer's time
alone that message would not have appeared on the night that prompted it.
The lead ruled on 2026-11-27 that the line counts what a later round cost.

## Done when

- [ ] For rounds after the first, the pace line gives the time from the
  previous round's verdict to this round's verdict. It still fits on one
  line and says plainly what each time measures.
- [ ] The Report gives the line for `hub-v2-bg14` beside the 33 to 86
  minutes counted by hand in `atelier-f0sn`'s Report.
- [ ] The two size tests of `atelier-f0sn` still pass, and
  `docs/product.md` shows the new line.

## Scope

`atelier_core::pace`, where `status` prints the line, their tests and the
line in `docs/product.md`. Nothing else in `status`.
