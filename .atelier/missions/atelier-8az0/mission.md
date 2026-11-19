---
schema: atelier.mission
schema_version: 3
id: atelier-8az0
title: 'Reviews that block only what matters: a later round''s brief, a way for the implementer to say no, and proof a reviewer can tell overbuilt work'
status: open
labels: []
lead: 0b3307d5-85cd-4102-be6c-51da3b535c89
planners:
- 0b3307d5-85cd-4102-be6c-51da3b535c89
created_at: 2026-11-19T04:11:59.375882958Z
---

## Why

The human asked for this on 2026-11-18, by a side chat on the thread that
leads mission atelier-kw8u, after a survey they had asked for: every Sol
review of an item since the cutover, 143 reviews of 65 items holding 136
new findings, each classified by Opus agents on the human's four levels of
severity. The files are in `~/.cache/af-evals/review-survey-2026-11-18/`.
All 136 findings were right on the facts. Three things in it are not right.

- **Later rounds find less, and the brief does not say what they are for.**
  The share of findings that were critical or important fell from 75% in
  round 1 (96 findings) to 50% in round 2 (22) and 39% from round 3 on
  (18). Eighteen findings were another case of a gap already reported on
  the same item; atelier-v958 took eight rounds, and atelier-nv22 and
  atelier-ob9u six each. Round 4's brief names the round and repeats round
  1's instruction word for word, with the diff of the whole change first.
- **An implementer cannot say no.** Of 117 findings marked `fix now`, 27
  were useful or nit (23%), and all 27 were built. No finding on any item
  has been rebutted since the cutover. `roles/implementer.md` says "You
  cannot dismiss a finding", a rebuttal needs "a fact or a test", and the
  lead is reached only after a rebuttal is lost.
- **No review asks for less.** Not one of the 143 item reviews or of 70
  plan reviews holds a `Shrink:` or `Reject:` finding, and none of the 136
  findings asks for less code. Sol rated value high in 114 reviews and low
  in none. The one eval case for it, `costly-hardening`, cannot show
  whether a reviewer can tell overbuilt work: its change has planted
  defects, its cue is low value, and it has no control.

The survey has limits. The classifiers read the code for three findings
only, so severity rests on Sol's own descriptions, and it is Sol reviewing
Opus and Fable, not Sol reviewing Sol. Two readers agreed exactly on 23 of
28 findings read twice, and on 27 of 28 across the line between important
and useful.

Mission atelier-kw8u tries the human's scale of severity as a passage for
the reviewer (atelier-oiwy) and built the cases and graders this mission
uses. This mission takes what atelier-kw8u's Scope leaves out.

## Outcome

- **A later round's brief says what the round is for.** It says how many
  rounds have reviewed the whole change, and that this round checks the
  earlier findings and what changed since, with that diff first. It says a
  new finding in code that did not change blocks only if the work would
  come out wrong without it. It narrows where a reviewer looks and hides
  nothing: seven of the 18 findings from round 3 on were important.
- **An implementer has a way to say no**, as the human designed it: "the
  reviewer can mark a subset of findings as must-fix and they cannot be
  ignored, the implementer can defer anything else with only a basic reason
  or ask the lead to override a fix-now. The lead can override
  everything." A finding that does not block may be left with a basic
  reason. A blocking one the implementer thinks real and not worth building
  goes straight to the lead with a basic reason, with no rebuttal first. A
  wrong one is rebutted. The reason has a place in the record: a line under
  Responses takes a third form, beside `fixed in` and `rebutted:`, that
  says the finding was not built and why. On a finding that blocks, that
  line is the question to the lead, and the lead's ruling is still made
  with a recorded reason, as today. The skill and what the binary prints
  say the same.
- **The evals show whether a reviewer can tell correct but overbuilt
  work**, with controls, and whether a passage that asks which requirement
  each new part serves helps it.

## Success criteria

- later-round-brief: for a fixture item in round 3, `atelier review` prints
  how many rounds have reviewed the whole change, the diff since the last
  reviewed commit before the diff of the whole change, and the rule for a
  new finding in code that did not change. Round 1's brief is as it was.
- narrows-without-hiding: two round-2 review cases, where text that did not
  change holds a flaw, cosmetic in one and making the work come out wrong
  in the other, run in `sol` with the binary before and after, as two
  separate matrices of ten runs a cell, on wording settled before the first
  run. The results give both. Where the second flaw is found less often
  after in both matrices, the brief loses its sentence on a new finding in
  code that did not change and keeps the rest.
- say-no: a case gives an implementer an item in changes requested with
  three fix-now findings: a real defect, one that is true and not worth
  building, and one that is wrong. It passes when the first is fixed, the
  second is taken to the lead with a reason and not built, and the third is
  rebutted. It fails for building the second or third and for leaving the
  first. It runs with Sol in every role, with and without the new text, ten
  runs a cell.
- reason-on-record: a line under Responses that says a finding was not
  built, with its reason, is accepted where `fixed in` and `rebutted:` are.
  `status` shows a mission's lead each such line on a finding that blocks,
  the next round's reviewer is told the finding waits on the lead and does
  not judge it again, and `merge` still refuses while it is open unless the
  lead passes the rule with a reason. The installed binary of before still
  reads an item that holds no such line.
- texts-agree: `roles/implementer.md`, the line `status` prints for
  answering findings, and what a reviewer is told about a rebuttal say the
  same, and `docs/design.md` records the decision with this evidence.
- same-base: where a criterion compares runs with a text and without it,
  both arms are staged from two commits that differ in that text alone, and
  the results name both commits.
- overbuilt-told-apart: five review cases, each a correct change whose
  tests pass: unneeded generality, an unneeded layer, unneeded robustness,
  and two controls, one large because it must be and one lean. Each
  overbuilt case ships with its lean reference. A run of an overbuilt case
  passes when the review names the unneeded part; a control passes when no
  finding asks for less. Whether a finding blocks is reported. They run on
  `gpt-6.1-sol` at `high` with no skill, with the skill, and with the skill
  and the per-part passage, ten runs a cell.
- per-part-passage: the passage is merged into the skill where the three
  overbuilt cases pass more runs with it in each of two matrices and
  neither control passes fewer in either. Otherwise it is left out, and
  the results say so.
- still-gated: `evals/agent-factory/test.sh` sets up and grades every new
  case with no model, and CI's `evals` job runs it.

## Scope

- In: the brief `atelier review` prints for a round after the first; in the
  skill, `roles/implementer.md`'s Findings and `roles/reviewer.md`; the
  lines the binary prints about answering and rebutting a finding; the
  one more form of a line under `### Responses`, read and shown wherever
  the two forms are today; the decision's rows in `docs/design.md`; new
  cases and results under `evals/agent-factory/`.
- Out: a new command, role or section of a record, `escalate` and `decide`
  among them; a
  field for severity in a review's record, which the human put off;
  reviewers from two labs in parallel, which they also put off; a guard
  against a reviewer rating everything high; rounds per item on the
  mission's page; a pair of overbuilt cases for plan review, unless the
  lead adds it; the trailing blank line that fails CI, which is
  atelier-7xbs.
- Target environment: this host, with Claude Code and Codex signed in. A
  case holds only code written for it or from this repository.

## Authority

The lead may choose fixtures, case names, rubric wording, the wording of
the brief and of the passages within what the Outcome quotes, and more runs
a cell than a criterion names. It may add the pair of overbuilt cases for
plan review. It may run this mission's items while atelier-kw8u's last
items run.

It may not add a command, a role or a section of a record; change the human's four
levels or their three rules, which atelier-oiwy holds; or keep the whole
change's diff from a later round's reviewer.

## Result
