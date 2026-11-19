---
schema: atelier.item
schema_version: 3
id: atelier-t5tc
title: Show whether the later round's brief hides an important defect in code the fix changed
status: approved
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

- [x] A round-2 review case is added, a fixture, facts and a rubric, beside
  the two of atelier-d73k: round 1's findings are fixed, and the fix itself
  introduces a defect that makes the work come out wrong in a case that
  will happen in normal use. It passes when the review blocks on it.
  `test.sh` runs it with no model.
- [x] The results give it in `sol`, with the binary before atelier-d73k's
  change and after, as two separate matrices of ten runs a cell.
- [x] If the defect is found less often after in both matrices,
  atelier-d73k's change to the brief is reverted on this item's branch,
  `docs/design.md` says what was tried and why it went, and nothing else is
  tried in this item. Otherwise the brief stays and the Report says so.
- [x] Until this item has merged, the shared `atelier` binary is not
  reinstalled from a master that holds atelier-d73k's brief. The Report
  says which binary the host was left with.

## Scope

One new directory under `evals/agent-factory/scenarios/`;
`evals/agent-factory/results/`; the review brief in `crates/` and
`docs/design.md`, only to revert.

## Report

**atelier-d73k's brief does not hide a defect the fix introduced, so it
stays.** In the new case the fix of round 1's two findings adds a
`parse_day` that refuses every real date on the 30th or 31st of a month.
Sol blocked on it in 40 of 40 reviews: 10 of 10 in each cell, with the
binary before atelier-d73k's change and after, in both matrices. It is
not found less often after in either, so nothing was reverted and
`docs/design.md` is unchanged. Results:
`evals/agent-factory/results/2026-11-19-later-round-fix.md`.

The year 9999 overflow in the fix, which 12 of 40 old-brief reviews
reported in atelier-d73k's cases, was reported here by 1 of 20 old-brief
reviews (as `defer`) and none of 20 new-brief ones. With a real defect in
the same commit, the old brief rarely got to it either, so this case says
little more about that difference.

What changed:

- `evals/agent-factory/scenarios/later-round-fix-wrong`: the same
  `hours-q7tn` item in round 2, with a correct `week_lines` and a
  `fixture/fix.patch` that narrows `parse_day`'s pattern to
  `[0-9]{4}-[01][0-9]-[0-2][0-9]`. No test has a date on the 30th or
  31st. It passes when a fix-now finding names the refused dates.
- `evals/agent-factory/harness/later_round.sh` applies a case's
  `fixture/fix.patch` over the shared fix, if it has one.
  `later_round_tests.py` takes `in_fix = True` for this case: it checks the
  fix commit refuses 2026-11-30 and 2026-12-31 with `bad date` and exit 2,
  accepts 2026-11-29, and that the flaw is in the fix's diff only in this
  case. `test.sh`'s run of all three later-round cases passes with both
  binaries, 15 tests each, with no model.
- `evals/agent-factory/README.md`: the scenario table, the Later rounds
  section and the Running list name the case.

The runs used the same two binaries as atelier-d73k: before is
`/var/tmp/af-evals/bin/atelier-221d5c146` (master at 221d5c146), and after
is `/var/tmp/af-evals/bin/atelier-698a306fa`, atelier-d73k's commit of the
brief. The case and skill were staged from 4c1890c19.

The host's shared binary was not touched. It is `~/.cargo/bin/atelier`,
0.7.0, installed at 2026-11-19 02:53, before atelier-d73k merged at 02:57.
It does not hold that item's brief: its strings lack "reviewed the whole
change before this one".

### Responses

- 1-usll.1: fixed in 9ed8ea4f7
