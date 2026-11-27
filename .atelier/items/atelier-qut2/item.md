---
schema: atelier.item
schema_version: 3
id: atelier-qut2
title: Rewrite the skill for plans as outcomes and lighter later rounds, from what this mission's own plan review showed
status: open
mission: atelier-oqsz
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T07:19:32.527359035Z
---

## Goal

Plan reviews spiral. `atelier-ww7w`'s plan of 2,400 words drew 45 new
findings in five rounds (20, 12, 7, 4, 2), most about how a mechanism
would behave. The skill gives no reason for them not to:
`roles/planner.md` says nothing about what level a plan is written at,
`roles/reviewer.md` is written for a code change and tells a plan's
reviewer to "report every other finding you can support", and a new
mission's plan is six empty headings.

Code reviews have a smaller fault. In `hub-v2-bg14` round one found a
blocking fault in eight of nine merged items; round two approved all
eight and added five findings, none blocking, at the cost of a full
review each.

This mission's plan was written as outcomes and reviewed under the old
text, and `notes.md` holds what that review showed. Change the skill by
what it showed. If the review converged in one or two rounds on outcome
findings, the level a plan is written at was the cause. If it still drew
mechanism findings or ran long, the reviewer's instructions and the
missing stop are.

## Done when

- [ ] The Report says which of the two the plan review showed, from
  `notes.md`, and each change to the skill names the number it follows
  from.
- [ ] The skill says what level a plan is written at and what belongs in
  an item instead.
- [ ] A plan's reviewer is told what a plan review is for, where a finding
  about a mechanism goes, and when plan review stops.
- [ ] A later code-review round is lighter than the first, or the Report
  says why the evidence does not support that.
- [ ] `roles/lead.md` has the lead give the human the pace numbers when it
  reports on a mission.
- [ ] `atelier-ww7w`'s plan as its round 1 read it, commit `6a2c26e`, is
  reviewed again under the new text by two fresh reviewers on the models
  of that round, `claude-opus-5-5` and `grok-4.6`. The Report gives the
  verdicts and the outcome and mechanism findings beside those of the two
  round-1 reviews there (`1-48x6`, `1-i2iw`: 20 findings). No eval matrix
  is run for this item.
- [ ] `evals/agent-factory/test.sh` passes.

## Scope

`skills/agent-factory/` and the documents that repeat it. While
`atelier-oiwy` is open, change neither `roles/reviewer.md` nor the parts
of `SKILL.md` before Starting Another Thread, which that item is testing;
put text for a plan's reviewer where a plan's reviewer will read it
without them, and file what can only go in those two files as an item
blocked by `atelier-oiwy`. `atelier-b0vb`, the per-part question on
plans, waits on the human: read it and leave it. No new role.
