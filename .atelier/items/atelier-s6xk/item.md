---
schema: atelier.item
schema_version: 3
id: atelier-s6xk
title: 'Add five review cases of correct work: three overbuilt, each with its lean reference, and two controls'
status: changes_requested
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

- [x] Five review cases are added, each a fixture, facts and a rubric, with
  one reviewer each. In every one the change is correct and its tests pass.
  Three are overbuilt: one by generality nothing asks for, one by a layer
  nothing needs, one by robustness against a case that cannot happen. Two
  are controls: one large because its requirement is large, and one lean.
- [x] Each overbuilt case ships with its lean reference: the smaller change
  that meets the same Goal and Done when, with the same tests passing. The
  reference stays outside the fixture.
- [x] A run of an overbuilt case passes when the judge finds that a finding
  names the unneeded part. A run of a control passes when no finding asks
  for less. Whether the finding blocks is reported and changes no grade.
- [x] `test.sh` builds each fixture, runs each lean reference's tests, and
  grades a recorded review, with no model.
- [x] The results give all five on `gpt-6.1-sol` at `high`, with no skill
  and with the skill, ten runs a cell.

## Scope

Five new directories under `evals/agent-factory/scenarios/`, and
`evals/agent-factory/results/`.

## Report

Sol can tell correct but overbuilt work, and the skill makes it block. With
no skill, `gpt-6.1-sol` at `high` named the unneeded part in 29 of 30
overbuilt runs but as fix now in only 20; with the skill, 30 of 30, all as
fix-now `Shrink:` findings. No run of either control, 40 in all, asked for
less. Results:
`evals/agent-factory/results/2026-11-19-overbuilt-told-apart.md`.

- Five cases: `overbuilt-generality` (`--csv` asked, an exporter registry
  built), `overbuilt-layer` (`tag` asked, a repository and service layer
  built), `overbuilt-robustness` (`--low` asked, guards against items
  `load` never returns), `control-large` (a 275-line two-bank import, each
  part named by a Done when line) and `control-lean` (a 9-line `done`
  command). One reviewer each. Every item's Scope allows new modules, so
  the extra parts cannot be flagged as out of scope instead of unneeded.
- Each overbuilt case's lean reference is in its `lean/`, outside the
  fixture. Its test files are the Done when's tests. `sized_tests.py`
  checks that they pass on the reference and on the change and fail on
  main, and that no file of the reference's code is in the fixture.
- Grading: an overbuilt run passes when the judge reads a finding, any
  recommendation, as naming the unneeded part (`names-unneeded`); a
  control passes when none asks for less (`nothing-less`). `blocks` is
  reported and changes no grade. I checked the judge's answer against each
  review; it agreed every time.
- `test.sh` runs each case's `test_grade.py`, which builds the fixture,
  runs the change's suite, runs the lean reference's tests, and grades
  recorded reviews, with no model. The whole of `test.sh` passed.
- The setup, grader and tests are shared, in `harness/sized.sh`,
  `harness/sized.py` and `harness/sized_tests.py`, as the replay cases
  share `harness/replay.py`. That puts three files outside the Scope's
  five directories.

The first matrix found real defects in three changes meant to be correct.
In `overbuilt-layer` the defect crowded out the sizing finding: with no
skill it passed 1 of 10 until I fixed it, then 10 of 10. I fixed those,
reran the three cases, and report each cell from the commit that last
changed its fixture. `control-large`'s reviews still raise narrower
points, such as stray quotes Python's `csv` accepts and the physical line
of a multi-line record's error. I stopped there: each round found
narrower ones, and none bears on its graded question, which all 60 of its
runs passed. The results list what is left.

For atelier-ufbm: Sol already sees the overbuilt part without the skill,
so a per-part passage can only show its worth in how often the finding
blocks, or on a model that misses it. The skill arm is already at 30 of
30 on these cases, so a third arm cannot pass more runs here.
