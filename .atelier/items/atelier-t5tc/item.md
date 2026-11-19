---
schema: atelier.item
schema_version: 3
id: atelier-t5tc
title: Show whether the later round's brief hides an important defect in code the fix changed
status: building
mission: atelier-8az0
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Af63a076b-801f-4619-9867-64e0090d639a%3Adelegate-task%3A3bd376d8-91d7-4033-a04c-b16bdb0a00a2
ready:
  by: 'the lead of atelier-8az0: the mission''s Outcome says the brief hides nothing, and atelier-d73k''s runs cannot show it for code the fix changed'
  at: 2026-11-19T07:58:15.274648583Z
trivial: null
caused_by: null
created_at: 2026-11-19T07:58:15.226567467Z
---

## Goal

atelier-d73k changed the brief of a round after the first, and its two
cases put a flaw in code the fix did not change. They show the brief does
not hide a real defect there. They cannot show what it does to a defect in
code the fix did change, and one unplanted crash in such a line suggests
it may hide those: with the old brief 12 of 40 reviews reported it, 4 as
blocking, and with the new brief none of 40 did. That crash hardly
matters. A survey of 2026-11-18 found seven of the 18 findings from round
3 on to be important, and a later round exists to catch what a fix broke.
The mission's Outcome says the brief hides nothing.

## Done when

- [ ] A round-2 review case is added, a fixture, facts and a rubric, beside
  the two of atelier-d73k: round 1's findings are fixed, and the fix itself
  introduces a defect that makes the work come out wrong in a case that
  will happen in normal use. It passes when the review blocks on it.
  `test.sh` runs it with no model.
- [ ] The results give it in `sol`, with the binary before atelier-d73k's
  change and after, as two separate matrices of ten runs a cell.
- [ ] If the defect is found less often after in both matrices,
  atelier-d73k's change to the brief is reverted on this item's branch,
  `docs/design.md` says what was tried and why it went, and nothing else is
  tried in this item. Otherwise the brief stays and the Report says so.
- [ ] Until this item has merged, the shared `atelier` binary is not
  reinstalled from a master that holds atelier-d73k's brief. The Report
  says which binary the host was left with.

## Scope

One new directory under `evals/agent-factory/scenarios/`;
`evals/agent-factory/results/`; the review brief in `crates/` and
`docs/design.md`, only to revert.
