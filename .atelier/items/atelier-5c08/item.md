---
schema: atelier.item
schema_version: 3
id: atelier-5c08
title: Date a mission's start in draw_samples.py from a line missions still write
status: open
mission: null
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T18:33:36.893548198Z
---

## Goal

`closed_missions` in `scripts/scorecard/draw_samples.py` takes a mission's
start from the first `atelier log` line whose act is `ready`. No mission
has written that line since 2026-11-13, when a mission's plan began to
merge on two approving plan reviews, and no item writes one since
atelier-v06d. So every mission closed after that date gets `start=None`,
and whatever the scorecard computes from a mission's length reads nothing
for them. Found by atelier-v06d's implementer (its Report, "Left, for the
lead"); atelier-c1z3 readies the scorecard for its rerun and may want this
first.

## Done when

- [ ] A mission's start comes from a line missions still write, such as
  the merge of its plan or its `lead` line, and a test holds it for a
  mission closed after 2026-11-13.
- [ ] A mission with no such line is reported, not sampled with an empty
  start.

## Scope

`scripts/scorecard/draw_samples.py` and its tests.
