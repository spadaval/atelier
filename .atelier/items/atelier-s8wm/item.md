---
schema: atelier.item
schema_version: 3
id: atelier-s8wm
title: Give a plan review whose two reviewers keep disagreeing a way to end
status: dropped
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-18T21:03:38.650364990Z
---

## Goal

A plan merges once both reviewers of a round approve, and nothing ends a
round they split on. `atelier list --disputed` on 2026-11-18 shows 12 split
rounds across two missions: atelier-to83's plan went seven rounds and
atelier-osel's six, with one reviewer approving and the other asking for
changes each time. The planner cannot break the tie, since it wrote the
plan, so the only one who can is the human. Planning is their conversation
with the planner, and the human said on 2026-11-18 that this is a problem
of its own, apart from how a mission runs.

Decide with the human what ends a split, such as the planner bringing the
open findings to them after two split rounds, and say it where a planner
reads it.

## Done when

- [ ] The human has chosen what ends a split plan review.
- [ ] `roles/planner.md` says it, and what `submit` and `status` print to a
  planner after a split round agrees with it.

## Scope

`skills/agent-factory/roles/planner.md` and the lines the binary prints to a
planner. Not how a mission runs once its plan has merged.

## Report

Dropped: Covered by atelier-h6ij in mission atelier-ww7w (agreed with Sumeet on 2026-11-22)
