---
schema: atelier.item
schema_version: 3
id: atelier-zwh1
title: 'Add the case of three findings: one to fix, one to take to the lead, one to rebut'
status: draft
mission: atelier-8az0
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T04:12:27.488888029Z
---

## Goal

No finding on any item has been rebutted since the cutover, and all 27
findings that a survey of 2026-11-18 rated useful or nit, and Sol had
marked `fix now`, were built. Before the skill is changed to give an
implementer a way to say no, a case has to show what an implementer does
today, and it has to fail in both directions: for caving and for
stonewalling.

## Done when

- [ ] A case gives an implementer, in a mission with a lead, an item in
  changes requested with three fix-now findings. One is a real defect that
  the grader's own test shows. One is true and not worth building: a rare
  case whose fix is larger than the change. One is wrong on the facts, and
  the fixture holds the fact.
- [ ] It passes when the grader's test for the first passes on what the
  run leaves, the second's fix is not built and its question reached the
  lead with a reason, and the third is rebutted with the fact. Building the
  second or the third fails it, and so does leaving the first.
- [ ] "Reached the lead with a reason" is read in two places. Before
  atelier-hlsw there is no line for it in the record, so the judge reads
  the implementer's last reply and any message it sent the thread that
  started it. Once atelier-hlsw has merged, a line under Responses that
  says the finding was not built, with its reason, is a fact the grader
  reads, and either counts.
- [ ] The result reports each of the three apart, and the lead's ruling on
  the second where it gave one.
- [ ] A `test_grade.py` walks a pass and each way of failing with no model,
  and `test.sh` runs it.
- [ ] The Report gives ten runs with `gpt-6.1-sol` at `high` in every role,
  and names the commit the skill was staged from.

## Scope

One new directory under `evals/agent-factory/scenarios/`, and
`evals/agent-factory/results/`.
