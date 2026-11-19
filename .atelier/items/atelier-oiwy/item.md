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
says to "report every other finding you can support as well". The old skill
had such a rule (`git show ff6b56fe4:skills/agent-factory/procedures/review.md`,
lines 56 to 98) and it was cut with no case that exercised it.

Later that day they replaced the rule's wording with a scale of their own,
after a survey of every Sol review of an item since the cutover: 143
reviews, 136 findings, each classified by Opus agents. All 136 were right
on the facts. Of 117 marked `fix now`, 27 were useful or nit (23%), and all
27 were built. Sol never marked a critical or important finding as less.
The files are in `~/.cache/af-evals/review-survey-2026-11-18/`. What a
later round is for is no longer this passage's: it goes to the brief the
binary prints, in a mission that follows this one.

## Done when

- [ ] Five branches each add one passage to the skill as atelier-mtzj left
  it: the reviewer's "What to judge"; the lead's "Close against the
  Outcome, not the item count"; the implementer's two lines on building to
  the Goal, Done when and Scope and on a Done when that cannot hold; one
  sentence in `roles/lead.md` saying the lead is the captain, mission
  control is far away and the recorder is running; and the rule against
  nits in `roles/reviewer.md`. Each is brought to today's commands and says
  nothing the binary prints.
- [ ] The rule against nits is the human's scale of four levels. Critical:
  a Done when line does not hold, or the change does not do what its Goal
  says; the finding names the line. Important: the change does its job, but
  gives a wrong result, loses something or misleads, in a case that will
  happen in normal use, or its proof does not show what it claims; the
  finding names the case. Useful: real, and the change is still right to
  merge without it, such as a rare case, a problem that was there before,
  or a better way to write code that works. Nit: style, naming, or wording
  that changes no meaning; it is not filed.
- [ ] It gives three rules. Severity is what breaks, not how big the fix
  is. A problem the change did not cause or worsen is useful at most. What
  a tool already reports is not a finding.
- [ ] It needs no change to a record: a finding's title opens `Critical:`,
  `Important:` or `Useful:`, and the Recommendation follows it. Critical
  and important are `fix now`, and useful is `defer`. `Shrink:` and
  `Reject:` stay blocking, outside the scale. It holds no guard against a
  reviewer rating everything high; the human asked for none ahead of
  seeing it happen.
- [ ] The review cases' rubric reports, for each finding, which of the four
  levels the judge gives it, beside the reviewer's own.
- [ ] Each runs by `--ref` as an arm of its own, in both setups, on the
  cases that can tell: the review cases for the first and fifth, the lead's
  cases and the mission case for the second and fourth, the implementer's
  cases for the third. The results file gives each cell beside the same
  cell without the passage, with time and tokens.
- [ ] The fifth is not kept on passes, since it aims at a number no case
  passes or fails on. The results give, with and without it and in each of
  the two matrices, the fix-now findings per review, how many of them the
  judge rates useful or nit, and how many of the known defects were found.
  It is merged where the second number falls in both matrices and the
  third falls in neither.
- [ ] Each arm is run as two separate matrices of at least five runs per
  cell. A passage is merged into the skill where some cell passes at least
  two more runs with it in each of the two, and no cell passes fewer in
  either. The others are left out, and the results say so in a line each
  and give the runs every keep rests on.
- [ ] The README's table is brought up to date with what was kept.

## Scope

`skills/agent-factory/SKILL.md` and `roles/`; the review cases' rubrics;
`evals/agent-factory/results/` and `README.md`. The five branches are not merged; a kept passage is merged on
this item's branch.
