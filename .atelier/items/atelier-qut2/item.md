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
what it showed. A review that converged in one or two rounds on outcome
findings points at the level a plan is written at. One that still drew
mechanism findings or ran long points at the reviewer's instructions and
the missing stop. It is one review, and this plan differs from
`atelier-ww7w`'s in subject, size and reviewers, so it is evidence and
not proof.

On 2026-11-27 Sumeet steered this item, in thread
`cf983ef7-3317-404a-9c82-373f732b9e07`. Two faults the plan did not name
are added. A planner writes each later item's checklist before anything
is built, and its lead then holds the work to it. And an item's reviewer
checks the change against the item's own Done when and never asks whether
that line was worth building: of 50 reviews of five changes this
repository built and later removed, none questioned one
(`atelier-8az0`). He also set the second review of `atelier-ww7w`'s plan
aside: this plan's own review is signal enough, and Atelier's work on
itself stops when this mission closes.

## Done when

- [ ] The Report says what the plan review's counts in `notes.md` suggest
  and what they leave open, and each change to the skill names what it
  follows from: a count there, or the human's direction of 2026-11-27.
- [ ] The skill says what level a plan is written at, and tells a planner
  not to write the checklist of how a later item is built before the work
  ahead of it has landed.
- [ ] `roles/lead.md` says a lead may split, shrink or rewrite an item's
  Done when as earlier items land, inside the mission's Outcome and
  Authority, with its reason on the record.
- [ ] A plan's reviewer is told what a plan review is for, where a finding
  about a mechanism goes, and when plan review stops.
- [ ] An item's reviewer is told that a Done when line asking for more
  than the mission's Outcome needs is a `Shrink:` finding against the
  item, not a box to tick.
- [ ] A later code-review round is lighter than the first, or the Report
  says why the evidence does not support that.
- [ ] `roles/lead.md` has the lead give the human the pace numbers when it
  reports on a mission.
- [ ] `roles/reviewer.md` and `SKILL.md` together hold no more words when
  this item merges than when it started. The Report gives both counts.
- [ ] `evals/agent-factory/test.sh` passes. No eval matrix is run, and no
  plan is reviewed again.

## Scope

`skills/agent-factory/`, the documents that repeat it, and the brief
`atelier review <mission>` prints, in `crates/` and its tests. It starts
once `atelier-oiwy` is merged or dropped, and builds on what that item
left in `roles/reviewer.md`. The skill costs a reviewer real defects
already (`atelier-7h71`: 4 of 26 found with it, 15 of 26 without), so
what this item adds for a reviewer displaces something or goes in the
brief. `atelier-b0vb`, the per-part question on plans, waits on the
human: read it and leave it. No new role.
