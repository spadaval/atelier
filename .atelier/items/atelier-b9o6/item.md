---
schema: atelier.item
schema_version: 3
id: atelier-b9o6
title: Grade a run by facts from its records and a rubric that a model from another lab reads
status: ready
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: 'the human, in conversation on 2026-11-18: put the mission together and then run it'
  at: 2026-11-18T23:49:47.284668623Z
trivial: null
caused_by: null
created_at: 2026-11-18T20:05:51.943289464Z
---

## Goal

The graders decide what a review or a reply means by matching words, and
what a thread wrote by parsing its shell commands. That is where the wrong
and null grades come from: three of nine in the smoke run of 2026-11-17,
every one a pass by hand. Split a grade into facts that code reads from the
records and the tree, and questions of meaning that a model reads against a
rubric.

## Done when

- [ ] A case's `grade.py` prints facts only: named values read from the
  records, the tree and the run's recorded `atelier` calls. Its `rubric.md`
  holds the questions of meaning, each with what a pass looks like, and
  each marked as graded or as reported.
- [ ] The harness puts each rubric question to a judge: a model from another
  lab than the thread whose work the question is about. Each question names
  the role it is about, and the setup names a judge for each lab its threads
  come from, so `house` has two. A harness test covers a `house` run whose
  reviewer's question goes to one judge and whose implementer's goes to the
  other.
  The judge is given the rubric, the facts, the final reply, the review
  files and the diff the run left. It is not told the arm.
- [ ] The judge answers each question pass or fail with the line it rests
  on, and the harness records both in `result.json`. A run passes when every
  fact and every graded answer passes. A reported answer is recorded and
  shown and changes no grade: a harness test covers a run whose facts and
  graded answers pass and whose reported answer fails, and it passes. A
  graded question the judge cannot answer fails with its reason; only an
  invalid run is `null`.
- [ ] Three cases are brought over, one of each kind:
  `next-vacuous-test-review` (judged), `next-costly-hardening` (judged, and
  no longer by its title's first word) and `next-loop` (facts alone). Their
  graders hold no regular expression over a review's or a reply's wording.
- [ ] Each `test_grade.py` still walks a passing path and failing ones with
  no model: the facts directly, and the judged questions from recorded
  answers. `test.sh` passes.
- [ ] The Report gives the judge's verdict beside the hand count for every
  kept run of these three cases in `~/.cache/af-evals/runs/gpt62` and
  `rt3o-smoke` whose review file survives, and explains each difference.

## Scope

`evals/agent-factory/harness/` and the three scenarios' directories.
