---
schema: atelier.item
schema_version: 3
id: atelier-7h71
title: Find why reviews with the skill miss a defect that reviews without it find
status: in_review
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A2ec81be6-36b2-4b12-a37f-0052f9f2be6c%3Adelegate-task%3A7475e1ef-65b3-43de-bfd9-7c2f5a5d8c6b
ready:
  by: 'the lead of atelier-kw8u: the matrix found the skill costing a reviewer a defect, and atelier-oiwy should not add passages before it is understood'
  at: 2026-11-19T11:37:11.739669970Z
trivial: null
caused_by: null
created_at: 2026-11-19T11:37:11.695173322Z
---

## Goal

The matrix of atelier-id3v found a case where the skill makes a reviewer
worse. On the replayed review `touched-claims`, five of six reviews with
no skill found the known defect, and none of six with the skill did: all
six approved, five with no finding at all. Across the twelve replayed
reviews the skill found the known defect in 12 of 72 reviews against 17 of
72 without it, and helped in no case in both setups. The same skill takes
`costly-hardening` from 0 of 10 to 10 of 10, by `roles/reviewer.md`'s Size
and Verdict, the one passage of the reviewer's file that the evals of
2026-11-11 kept.

Six reviews are few, so first confirm it, then find which text does it.
atelier-oiwy tries passages for the reviewer next, and should know what
it is adding them to.

## Done when

- [x] `touched-claims` is run again with no skill and with the skill, ten
  runs a cell in `sol` and in `house`. The Report says whether the gap
  holds.
- [x] The Report reads the kept reviews of both arms and says what the
  skill's reviews did differently: what they checked, what they wrote under
  Size and Verdict, and where the reviews with no skill found the defect.
- [x] If the gap holds, the case is run with two more arms, each from a
  branch that is not merged, ten runs a cell in `sol`: the skill without
  `roles/reviewer.md`'s Size and Verdict, and the skill with no
  `roles/reviewer.md` at all. `costly-hardening` and the five overbuilt
  cases of atelier-s6xk are run with the same two arms, five runs a cell,
  to show what each costs there. The Report gives all of it in one table.
- [x] Nothing in the skill is changed. The Report says what the runs point
  to, for the human and for atelier-oiwy.

## Scope

`evals/agent-factory/results/`. No change to the skill, the cases or the
harness.

## Report

The gap holds, and cutting `roles/reviewer.md` does not close it. Over ten runs a
cell, `touched-claims` was found by 10 of 20 reviews with no skill and 4 of
20 with it (`sol` 3 to 0, `house` 7 to 4). With id3v's twelve reviews the
totals are 15 of 26 against 4 of 26. In `sol`, the skill without Size and
Verdict found it 0 of 10 times, and the skill with no reviewer file 0 of 10
times. Results:
`evals/agent-factory/results/2026-11-19-why-touched-claims-missed.md`.

- **What the skill's reviews did differently.** They looked just as much,
  but the other way. In each arm, 19 of 26 reviews went past the test suite.
  With no skill, 15 of those 19 asked whether the new `advances` walk keeps
  the Goal's promise that `land` checks only what changed. They built two
  sibling tasks, saw `land` refuse the untouched one, and found the defect.
  With the skill, 15 of 19 tested only whether the walk misses a claim
  (one-sided links, reopened child work), and none of them found it. Under
  Size and Verdict, all 22 failing skill reviews gave the change's own aim
  as its Value. Three of them also stated the Goal's boundary ("unrelated
  historical claims remain trusted") and never tested it; the reviews that
  found the defect are the ones that built a case for it. Eleven named the
  defect's class under Risk
  ("an overly broad traversal produces a visible refusal and is cheap to
  undo") and rated it low or medium. These lines record where a reviewer
  already looks: without Size and Verdict, the reviews wrote no scales and
  still checked only for missing claims.
- **Where the cause is.** As a control outside the Done when, I ran
  id3v's `eval/skill-threads-only` arm (the prompt line, and a `SKILL.md`
  holding only Starting Another Thread). It found the defect 3 of 10 times,
  the same as no skill. That points to `SKILL.md`'s first four sections as
  the next suspect: Run `atelier status` first, Your Role, Who Works With
  the Human, and Independent Review. Every `sol` arm that kept them found
  the defect 0 of 33 times, and every arm without them 8 of 23 (p < 0.001).
  Cuts that stay at zero cannot show that the reviewer file adds nothing,
  and the runs cannot tell which section matters. `house` had no cut arms.
- **What each cut costs.** Without Size and Verdict, `costly-hardening`
  falls to 0 of 5, and with no reviewer file to 1 of 5, against 10 of 10
  with the whole skill. On the overbuilt cases both cut arms still blocked
  in 15 of 15 runs. I ran no skill again today (10 of 15 blocked), and that
  shows the blocking s6xk credited to Size and Verdict comes from elsewhere
  in the skill. No control run asked for less in any arm.
- **What it points to.** For atelier-oiwy: removing reviewer text did not
  fix this miss in `sol`, and cutting Size and Verdict loses
  `costly-hardening`. Whether a new reviewer passage could counter it, such
  as one asking a reviewer to test the change against the Goal, is
  untested and is atelier-oiwy's to try. One cut arm for each of
  `SKILL.md`'s first four sections would narrow the cause. For the human:
  with the skill, reviewers block more on size, and on this replay they
  test the change against its own claim instead of the Goal. That misses a
  check that does too much. The Risk scale then calls such a failure low
  because it shows. Whether a visible refusal of valid work should rate
  low is a question of the skill's intent.

Two notes on scope. I read "the five overbuilt cases of atelier-s6xk" as
its five sized cases, the three overbuilt and two controls. Beyond the Done
when, I ran the threads-only control (10 replays) and no skill on the six
sized cases (30 runs), so every cut-arm cell has a baseline from the same
day. Nothing in the skill, the cases or the harness changed. The two arm
branches, `eval/skill-no-size-verdict` and `eval/skill-no-reviewer`, are
pushed and never to be merged. The README's results list does not link the
new file, since the item's scope is `results/`; the next item that edits
the README can add it.

### Responses

- 1-t0j9.1: fixed in fae6eddc1
- 1-t0j9.2: fixed in fae6eddc1
- 2-ziqq.2: fixed in 03e41eef4
