---
schema: atelier.item
schema_version: 3
id: atelier-0019
title: Garden atelier, 2026-11-11
status: ready
mission: null
labels:
- garden
priority: medium
blocked_by: []
implementers: []
ready:
  by: sumeet, through mission atelier-295u
  at: 2026-11-11T17:56:34.466554928Z
trivial: null
caused_by: null
created_at: 2026-11-11T17:56:25.650533771Z
---

## Goal

Measure atelier by `skills/agent-factory/procedures/garden.md`, fix the upkeep and signs it may fix, and bring Sumeet one page with at most three proposals. This is atelier's first run, so the window starts at the first line `atelier log` prints, 2026-11-11T15:20:09Z, and it is also the trial of the rewritten procedure for atelier-613a.

## Done when

- [ ] The numbers follow garden.md's steps, and `artifacts/numbers.md` holds the log lines behind them.
- [ ] At most three proposals, each one cause with its evidence counted; one that adds a rule or check names one considered for removal.
- [ ] Every change on the branch is upkeep or a sign: none adds or removes a rule, check, gate, interface or default.
- [ ] Each finding `atelier list --deferred` showed is settled.
- [ ] The page follows garden.md's template in at most 60 lines, with Session empty.

## Scope

In: garden.md steps 1 to 7 up to the approval, for atelier only. atelier-j86z's ten entries are read as evidence and the page says which the rewrite made moot.

Out: the session and what it settles, the run's merge, closing or changing atelier-j86z, any new or removed rule, check, merge check or hook, a scorecard snapshot or a run of `scripts/scorecard/`, and other items' branches.
