---
schema: atelier.item
schema_version: 3
id: atelier-oiwy
title: Find which part of the skill costs a reviewer a defect, and try two passages for the reviewer
status: building
mission: atelier-kw8u
labels: []
priority: medium
blocked_by:
- atelier-id3v
- atelier-scly
- atelier-7h71
implementers:
- thread:delegated-task:command%3Amcp%3A901dd2a0-b01e-4401-8056-a16ce644a3f9%3Adelegate-task%3A8eeb1931-a263-48c9-99af-02fdad3a8f21
ready:
  by: 'the human, in conversation on 2026-11-18: put the mission together and then run it'
  at: 2026-11-18T23:49:47.284668623Z
trivial: null
caused_by: null
created_at: 2026-11-18T21:03:01.247579892Z
---

## Goal

Use the evals for what the human asked of them on 2026-11-18: to decide
what the skill holds. The matrix of atelier-id3v and atelier-7h71 changed
what there is to decide.

- **The lead's and the implementer's cases cannot tell passages apart.**
  Every one passes five runs of five with the skill, in both setups. A
  passage is kept only where some cell passes more runs with it, so three
  of the five passages this item was to try have no room: the lead's
  "Close against the Outcome, not the item count", the implementer's two
  lines, and the sentence that makes the lead a captain. They are left out
  untried, and the lead of the mission took them out of this item.
- **The skill costs a reviewer a real defect, and its reviewer's file is
  not why.** On `touched-claims` reviews with no skill found the known
  defect in 15 of 26 and reviews with it in 4 of 26. Without
  `roles/reviewer.md`, or without its Size and Verdict, the skill still
  found it in 0 of 10. A skill cut to Starting Another Thread found it in
  3 of 10, as no skill does. In `sol`, arms that kept the first four parts
  of `SKILL.md` found it in 0 of 33, and arms without them in 8 of 23. The
  skill's reviewers tested as much and in the wrong direction: whether the
  change misses a claim, and not whether it takes in one it should leave.
- **Two passages for the reviewer remain to try**: "What to judge", cut on
  2026-11-11 (b22a03359), which tells a reviewer to compare the change
  with its Goal, Done when and Scope for an addition nobody asked for, and
  the human's scale of severity.

## Done when

- [ ] Which part of `SKILL.md` costs the defect is found. `touched-claims`
  runs in `sol`, ten runs a cell, with four arms from branches that are not
  merged, each the skill without one of: the opening lines before Your
  Role, Your Role, Who Works With the Human, and Independent Review. The
  results name the part or parts without which the defect comes back, or
  say that no one part does it.
- [ ] Two branches each add one passage to `roles/reviewer.md`: "What to
  judge", brought to today's commands and saying nothing the binary prints,
  and the human's scale. The scale is four levels. Critical: a Done when
  line does not hold, or the change does not do what its Goal says; the
  finding names the line. Important: the change does its job, but gives a
  wrong result, loses something or misleads, in a case that will happen in
  normal use, or its proof does not show what it claims; the finding names
  the case. Useful: real, and the change is still right to merge without
  it. Nit: style, naming, or wording that changes no meaning; it is not
  filed. It gives three rules: severity is what breaks, not how big the fix
  is; a problem the change did not cause or worsen is useful at most; what
  a tool already reports is not a finding. A finding's title opens
  `Critical:`, `Important:` or `Useful:`; critical and important are `fix
  now` and useful is `defer`; `Shrink:` and `Reject:` stay blocking,
  outside the scale. It holds no guard against rating everything high.
- [ ] The skill, and the skill with each passage, run in `sol` as two separate
  matrices on the review cases: `vacuous-test-review`, `costly-hardening`,
  the five cases of atelier-s6xk, the three later-round cases and the four
  replays some review has found, five runs a cell; and the eight replays
  no review has found, three runs a cell, as far as matrix 1's skill and
  "What to judge" arms. The rest of the floor replays and their top-ups
  are not run: both allowances ran out on 2026-11-19, and the human ruled
  on 2026-11-27 that the item closes on what it has. Each arm is staged
  from a commit that differs from the skill's in its passage alone, and
  the results name the commits. The rubric reports, for each finding,
  which of the four levels the judge gives it beside the reviewer's own.
- [ ] "What to judge" is merged into the skill where some case passes at
  least two more runs with it in each of the two matrices and no case
  passes fewer in either.
- [ ] The scale is merged into `roles/reviewer.md` by the lead's ruling of
  2026-11-27, with its reason in the Report, since the item's rule (fewer
  fix-now findings the judge rates useful or nit in both matrices, and
  in neither fewer known defects found) cannot be met on runs that were
  not made. The results give the rule's counts on the cases that ran.
  By the human's ruling, no `house` run follows it.
- [ ] Nothing else in the skill is changed. If a part of `SKILL.md` is
  found to cost the defect, the Report says which, what the runs show and
  what the lead and the human might do, and changes nothing.
- [ ] The README's table and its list of results are brought up to date,
  with atelier-7h71's results among them, and it says the three passages
  left out were not tried and why.

## Scope

`skills/agent-factory/roles/reviewer.md`, for a kept passage; the review
cases' rubrics; `evals/agent-factory/results/` and `README.md`. The
branches for the arms are not merged.
