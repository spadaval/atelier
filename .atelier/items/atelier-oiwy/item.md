---
schema: atelier.item
schema_version: 3
id: atelier-oiwy
title: Find which part of the skill costs a reviewer a defect, and try two passages for the reviewer
status: changes_requested
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

- [x] Which part of `SKILL.md` costs the defect is found. `touched-claims`
  runs in `sol`, ten runs a cell, with four arms from branches that are not
  merged, each the skill without one of: the opening lines before Your
  Role, Your Role, Who Works With the Human, and Independent Review. The
  results name the part or parts without which the defect comes back, or
  say that no one part does it.
- [x] Two branches each add one passage to `roles/reviewer.md`: "What to
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
- [x] The skill, and the skill with each passage, run in `sol` as two separate
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
- [x] "What to judge" is merged into the skill where some case passes at
  least two more runs with it in each of the two matrices and no case
  passes fewer in either.
- [x] The scale is merged into `roles/reviewer.md` by the lead's ruling of
  2026-11-27, with its reason in the Report, since the item's rule (fewer
  fix-now findings the judge rates useful or nit in both matrices, and
  in neither fewer known defects found) cannot be met on runs that were
  not made. The results give the rule's counts on the cases that ran.
  By the human's ruling, no `house` run follows it.
- [x] Nothing else in the skill is changed. If a part of `SKILL.md` is
  found to cost the defect, the Report says which, what the runs show and
  what the lead and the human might do, and changes nothing.
- [x] The README's table and its list of results are brought up to date,
  with atelier-7h71's results among them, and it says the three passages
  left out were not tried and why.

## Scope

`skills/agent-factory/roles/reviewer.md`, for a kept passage; the review
cases' rubrics; `evals/agent-factory/results/` and `README.md`. The
branches for the arms are not merged.

## Report

No one part of `SKILL.md` costs the `touched-claims` defect, and the
human's scale is merged into `roles/reviewer.md` by the lead's ruling.
"What to Judge" is left out. Results:
`evals/agent-factory/results/2026-11-19-reviewer-passages.md`.

- **The cuts.** In `sol`, ten runs each, the skill without its opening
  lines, without Your Role, without Who Works With the Human and without
  Independent Review found the defect 0, 0, 1 and 0 times: 1 of 40,
  against 3 of 10 with no skill and with the threads-only skill (p =
  0.004). Whatever costs the defect lies in the four parts together, or in
  them and the role file together; no single-cut arm can say which. I
  changed nothing in `SKILL.md`. The lead and the human might try one arm
  that keeps only Your Role's pointer to the role file, or accept the
  trade 7h71 measured: the skill wins `costly-hardening` and the overbuilt
  cases and costs this one replay.
- **"What to Judge".** It passed no case more often and `costly-hardening`
  once less (4 of 5 in matrix 1), so the item's rule leaves it out. It
  found `touched-claims` 0 of 10 times, so comparing the change with its
  Goal in these words does not send a reviewer to test the walk's
  boundary. It made reviews slower and added fix-now findings (176
  against 165 on the 14 cases that finished).
- **The scale.** On the 14 cases that finished in both matrices, the
  skill raised 9 fix-now findings the judge rated useful or nit (4 and 5),
  and the skill with the scale none. Known defects found: 9 against 10 in
  matrix 1, 10 against 10 in matrix 2. The lead's counts below hold
  against the runs. One thing the totals hide: all 9 useful findings came
  on `quiet-tokens`, where the scale found the defect 2 of 10 times
  against the skill's 5 of 10 (p = 0.35), offset by `base-done-claims` (8
  against 6) and `priority-contract` (9 against 7). With the scale,
  reviewers labelled every finding, and the judge agreed with the label
  on 165 of 176; no reviewer rated everything high.
- **The rubric.** The ten review rubrics gain a reported question,
  `levels`, in which the judge gives each finding's Recommendation, its
  title's label and the judge's own level. It changes no grade.

### The lead's rulings

- 2026-11-19: run here, not on boat, replays at `--cargo-jobs 3` and the
  rest at `--jobs 8`, dropping to 2 if the load stays above 10; three runs
  a cell for the replays no review has found, topped up to five only
  where an arm finds the defect, which is less than the item's "five runs
  a cell" for those cases. The cost is a small bias against a passage that
  would find a floor case only in runs four and five. Order: the cut arms,
  the cases that build no Rust, the replays found before, the floor
  replays.
- 2026-11-27, after the human ruled that the item closes on what it has:
  "What to judge" is left out, by the item's rule. The scale is merged by
  the lead's ruling, not the item's rule, which the unrun cases leave
  unmet: "On the 14 review cases whose runs finished in both matrices,
  reviews with the skill raised 9 fix-now findings the judge rated useful
  or nit, and reviews with the scale none; the known defects found were 9
  against 10 and 10 against 10. The replays no review has found and the
  run in `house` were not run, by the human's ruling that the item closes
  on what it has. The passage is the human's own design, it answers a
  measured cost (27 of 117 blocking findings in real reviews were useful
  or nit, and all were built), and one revert undoes it. Unmeasured: its
  effect on the cases that hold most fix-now findings, and on Sol at
  `xhigh`."

### Runs

- **Made:** 40 cut-arm runs; 300 runs of the ten cases that build no Rust
  (two matrices, three arms, five a cell); 120 of the four replays found
  before; 48 floor replays (matrix 1's skill and "What to Judge" arms,
  three a cell). Two smoke runs checked the `levels` answer first.
- **Made again:** 28 replays in matrix 2 were invalid when the Claude
  judge hit its weekly limit (from 10:42 on 2026-11-19; it reset at
  14:00). All 28 were remade into the same outputs. Earlier I told the
  lead 60; that was wrong. One `overbuilt-robustness` run was judged
  again, not remade, after the judge quoted a line not in its material.
- **Not made:** the floor replays of matrix 1's scale arm and of all of
  matrix 2 (96 runs), any top-up, and the `house` run. The ChatGPT Pro
  allowance, which every `sol` reviewer runs on, ran out at 15:44 on
  2026-11-19, until 2026-11-23, with 4 of matrix 1's floor scale runs
  invalid; the watchdog stopped the matrix there.
- **Both allowances spent.** The matrix used up Claude's weekly limit and
  then the ChatGPT Pro allowance in one day, and that allowance is shared
  with every other Sol and Luna task on the account.
- **Load.** I did not follow the lead's load rule. The one-minute load
  passed 10 in 58 of 278 minutes logged, for up to 9 minutes in a row,
  mostly in the floor replays (peak 38.7 at 15:22), and I left the
  replays at `--cargo-jobs 3` instead of dropping to 2. Free memory stayed
  above 6.8 GB and free disk above 147 GB.
- **Measured on the skill at 6cda5c821.** Master's `SKILL.md`,
  `roles/lead.md` and `roles/implementer.md` changed before this merged;
  `roles/reviewer.md` did not.
- The six eval branches (`eval/skill-no-opening`, `-no-your-role`,
  `-no-human`, `-no-review`, `-what-to-judge`, `-severity-scale`) are
  pushed and not to be merged. The runs are in
  `~/.cache/af-evals/runs/oiwy/`. `evals/agent-factory/test.sh` passes on
  the merged branch.
