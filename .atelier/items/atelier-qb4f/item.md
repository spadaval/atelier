---
schema: atelier.item
schema_version: 3
id: atelier-qb4f
title: 'Garden atelier: structure, 2026-11-13'
status: building
mission: null
labels:
- garden
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A3a128a06-6a57-49de-b3ea-7ef651a4c883%3Adelegate-task%3A5c4d50b4-aed9-49b9-86e1-036314a09ad4
ready:
  by: sumeet
  at: 2026-11-13T08:41:35.493423857Z
trivial: null
caused_by: null
created_at: 2026-11-13T08:41:28.221257700Z
---

## Goal

Garden atelier with structure and design drift as the focus, because Sumeet named it: the modules, what each is for, and the docs against the code. Bring him one page with at most three proposals, and fix only upkeep and signs on this branch. The window starts where atelier-0019's page ends, 2026-11-11T17:56:43Z.

## Done when

- [ ] The numbers follow garden.md's steps, and `artifacts/numbers.md` holds the log lines behind them.
- [ ] The focus area's threads and code were read as garden.md says, and nothing private reaches the page.
- [ ] At most three proposals, each one cause with its evidence counted; one that adds a rule or check names one considered for removal.
- [ ] Every change on the branch is upkeep or a sign: none adds or removes a rule, check, gate, interface or default.
- [ ] Each finding `atelier list --deferred` showed is settled.
- [ ] The page follows garden.md's template in at most 60 lines, with Session empty.

## Scope

In: `skills/agent-factory/procedures/garden.md` steps 1 to 7 up to the approval, for atelier only.

Out: the session and what it settles, the run's merge, any new or removed rule, check, gate, interface or default, a run of `scripts/scorecard/`, and other items' branches.
