---
schema: atelier.item
schema_version: 3
id: atelier-hota
title: Print atelier log newest first with a default limit
status: building
mission: atelier-oqsz
labels: []
priority: low
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A97bcfb9a-33e0-42f5-882a-004f5d9c6644%3Adelegate-task%3Aoqsz-impl-hota
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T07:19:32.618377817Z
---

## Goal

`atelier log` prints the whole history, oldest first. On the Hub's `v2` it
is 170 KB in 675 lines, so a reader with a cut view sees 2026-11-25 and
not the last hour. One mission's log is 30 KB, and a line reaches 2,456
characters where a skip's reason is quoted in full.

## Done when

- [ ] With no option, `atelier log`, `atelier log <item>` and `atelier log
  <mission>` print the newest action first and stop at a default limit,
  and say how many older actions there are and how to see them.
- [ ] The whole history is one flag away, in an order a script can rely
  on.
- [ ] No line is over 300 characters by default; a cut reason says where
  to read it whole.
- [ ] On the Hub's history the default output is under 10 KB; the Report
  gives the size.
- [ ] Everything in this repository that reads `atelier log`, the garden
  procedure's counter and the scorecard scripts among them, gives the same
  answer as before, shown by its tests or by a run on this repository.
- [ ] `docs/design.md` ("The log") and the command's help say the order
  and the limit.

## Scope

`atelier log`'s output, in `crates/` and their tests, its readers in
`skills/` and `scripts/`, and `docs/design.md`. Not what `log` records,
and not `atelier_core::log`, which the Desk reads. `atelier-3sym`,
`atelier-tuln` and `atelier-itqc` fix lines `log` gets wrong; leave them
to those items.
