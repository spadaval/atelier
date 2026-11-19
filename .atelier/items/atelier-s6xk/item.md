---
schema: atelier.item
schema_version: 3
id: atelier-s6xk
title: 'Add five review cases of correct work: three overbuilt, each with its lean reference, and two controls'
status: building
mission: atelier-8az0
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A3a9fab39-7f29-4732-8284-d6c07f2deac6%3Adelegate-task%3Afb974a82-9298-47f8-9874-db72c7c647a0
ready:
  by: 'the human, by a side chat on 2026-11-18: tell the main thread about all of this'
  at: 2026-11-19T04:37:19.810241548Z
trivial: null
caused_by: null
created_at: 2026-11-19T04:12:27.665498032Z
---

## Goal

The human, on 2026-11-18: "I want us to show that Sol can recognize
overbuilt (but correct) solutions." Not one of 143 item reviews since the
cutover asks for less code. `costly-hardening` cannot show whether a
reviewer can: its change has planted defects, its cue is low value, and it
has no control.

## Done when

- [ ] Five review cases are added, each a fixture, facts and a rubric, with
  one reviewer each. In every one the change is correct and its tests pass.
  Three are overbuilt: one by generality nothing asks for, one by a layer
  nothing needs, one by robustness against a case that cannot happen. Two
  are controls: one large because its requirement is large, and one lean.
- [ ] Each overbuilt case ships with its lean reference: the smaller change
  that meets the same Goal and Done when, with the same tests passing. The
  reference stays outside the fixture.
- [ ] A run of an overbuilt case passes when the judge finds that a finding
  names the unneeded part. A run of a control passes when no finding asks
  for less. Whether the finding blocks is reported and changes no grade.
- [ ] `test.sh` builds each fixture, runs each lean reference's tests, and
  grades a recorded review, with no model.
- [ ] The results give all five on `gpt-6.1-sol` at `high`, with no skill
  and with the skill, ten runs a cell.

## Scope

Five new directories under `evals/agent-factory/scenarios/`, and
`evals/agent-factory/results/`.
