---
schema: atelier.item
schema_version: 3
id: atelier-z8rg
title: Replay every after-merge defect that fits as a review case, now that one review costs two minutes
status: ready
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: the lead of atelier-kw8u, under its Authority to extend the review cases once atelier-1z52 reported what one costs
  at: 2026-11-19T01:56:05.793049132Z
trivial: null
caused_by: null
created_at: 2026-11-19T01:56:05.723046822Z
---

## Goal

atelier-1z52 replayed three changes as review cases and reported the cost:
one review takes 1 to 3.5 minutes, and five runs of ten cases take about
1.5 hours in `sol` and 2.3 in `house`. Three cases are too few to tell two
arms apart: both setups found the known defect in one of three. The plan
lets the lead extend the cases once that cost was known, and three of the
five passages atelier-oiwy tries are judged on them.
`docs/architecture/quality/scorecard-baseline/defects.md` traces 12 more
defects in atelier found after the merge that caused them.

## Done when

- [ ] Each of the 12 remaining after-merge rows for atelier in `defects.md`
  has a case file, or a line in the README of the replayed cases saying why
  it does not replay: its change is several commits that do not stand
  alone, its defect cannot be seen from the change, or its diff is more
  than one reviewer can hold.
- [ ] Each new case builds with the shared setup and runs in the same
  confinement as the first three, with no change to either.
- [ ] `test.sh` builds each new fixture and grades a recorded review, with
  no model and no `cargo` build.
- [ ] The Report gives one review of each new case by the reviewer of
  `sol`: found or not, unsupported findings, minutes and tokens.

## Scope

Case files and recorded reviews beside the first three, under
`evals/agent-factory/scenarios/`, and their README. No change to the
harness or the shared setup; a case that needs one is left out.
