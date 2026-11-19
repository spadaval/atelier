---
schema: atelier.item
schema_version: 3
id: atelier-oiwy
title: Try five passages in the skill, each as an arm of its own, and keep what changes an outcome
status: ready
mission: atelier-kw8u
labels: []
priority: medium
blocked_by:
- atelier-id3v
implementers: []
ready:
  by: 'the human, in conversation on 2026-11-18: put the mission together and then run it'
  at: 2026-11-18T23:49:47.284668623Z
trivial: null
caused_by: null
created_at: 2026-11-18T21:03:01.247579892Z
---

## Goal

Use the evals for what the human asked of them on 2026-11-18: to decide what
the skill holds. Three passages were cut on 2026-11-11 (b22a03359) with no
case that could have shown their worth, and the human has a frame for the
lead that one sentence might carry better than several rules. Try each, and
keep what the runs support.

The human asked for a fifth on 2026-11-18: a rule against nits. Nothing
today says which findings deserve `fix now`. The review brief names the
three recommendations with no test for choosing one, and `roles/reviewer.md`
says to "report every other finding you can support as well". Nothing
narrows what a reviewer raises in a later round. The old skill had both
rules (`git show ff6b56fe4:skills/agent-factory/procedures/review.md`, lines
56 to 98) and they were cut with no case that exercised them. This
mission's own plan review showed the cost: 12 findings over four rounds,
every one `fix now`, the last on three lines of the final item, each
costing a full round.

## Done when

- [ ] Five branches each add one passage to the skill as atelier-mtzj left
  it: the reviewer's "What to judge"; the lead's "Close against the
  Outcome, not the item count"; the implementer's two lines on building to
  the Goal, Done when and Scope and on a Done when that cannot hold; one
  sentence in `roles/lead.md` saying the lead is the captain, mission
  control is far away and the recorder is running; and the rule against
  nits in `roles/reviewer.md`. Each is brought to today's commands and says
  nothing the binary prints.
- [ ] The rule against nits says two things. A finding is `fix now` only if
  the work would come out wrong or unsafe without it; otherwise it is
  `defer` or `no action`, and when unsure, `defer`. And after round 1 a
  reviewer checks that the earlier findings are fixed and that the fixes
  broke nothing; a new finding on text that did not change is `defer`
  unless it is serious.
- [ ] Two small round-2 review cases are added for it, each a fixture,
  facts and a rubric: round 1's findings are fixed, and text that did not
  change holds a flaw. In one the flaw is cosmetic, and the case passes
  when the review approves or defers it. In the other the flaw makes the
  work come out wrong, and the case passes when the review holds a fix-now
  finding for it. The review cases' rubric also reports, for each fix-now
  finding, whether the work would come out wrong or unsafe without it.
- [ ] Each runs by `--ref` as an arm of its own, in both setups, on the
  cases that can tell: the review cases for the first and fifth, the two
  round-2 cases as well for the fifth, the lead's cases and the mission
  case for the second and fourth, the implementer's cases for the third.
  The results file gives each cell beside the same cell without the
  passage, with time and tokens. For the fifth it also gives the fix-now
  findings per review and how many of the known defects were still found.
- [ ] Each arm is run as two separate matrices of at least five runs per
  cell. A passage is merged into the skill where some cell passes at least
  two more runs with it in each of the two, and no cell passes fewer in
  either. The others are left out, and the results say so in a line each
  and give the runs every keep rests on.
- [ ] The README's table is brought up to date with what was kept.

## Scope

`skills/agent-factory/SKILL.md` and `roles/`; two new directories under
`evals/agent-factory/scenarios/`; `evals/agent-factory/results/` and
`README.md`. The five branches are not merged; a kept passage is merged on
this item's branch.
