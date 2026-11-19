---
schema: atelier.item
schema_version: 3
id: atelier-ad0z
title: Replay changes that were built and later removed as unneeded, as review cases
status: in_review
mission: atelier-8az0
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Af63a076b-801f-4619-9867-64e0090d639a%3Adelegate-task%3Add0281b0-8064-44ea-9b0d-bdc765676394
ready:
  by: 'the lead of atelier-8az0: atelier-s6xk''s cases are at their ceiling, so they cannot test the passage atelier-ufbm tries'
  at: 2026-11-19T06:37:15.609718336Z
trivial: null
caused_by: null
created_at: 2026-11-19T06:37:15.525693133Z
---

## Goal

atelier-s6xk showed that Sol can tell correct but overbuilt work when the
excess is plain: it named the unneeded part in 29 of 30 reviews with no
skill and 30 of 30 with it, and asked for less in none of 40 control
reviews. Yet in real use not one of 143 item reviews since the cutover
asks for less. The five cases are at their ceiling and cannot say why, and
they leave atelier-ufbm's passage nothing to improve.

This repository holds real cases. `docs/design.md` lists, under Tried and
removed, what was built, reviewed, merged and later taken out as unneeded:
a SQLite cache of records, evidence gates, sessions as records, a
configurable workflow file and more. Each was correct work that a reviewer
approved. Replay some as review cases, with the setup and confinement the
replayed defects use, and see whether a reviewer questions the need.

## Done when

- [x] At least five rows of Tried and removed whose building was one
  change a reviewer can hold are replayed as review cases, each with a case
  file naming its row, its base and change commits, the part later removed
  and the commit that removed it. A row that does not replay has a line
  saying why.
- [x] They use the shared replay setup and its confinement with no change
  to either. A case that needs one is left out.
- [x] A run passes when the judge finds that a finding questions the need
  for the part later removed, or asks for less of it. Whether the finding
  blocks is reported. The judge is given the removal's reason from
  `docs/design.md`; the reviewer is told nothing.
- [x] `test.sh` builds each fixture and grades a recorded review, with no
  model and no `cargo` build.
- [x] The results give each case on `gpt-6.1-sol` at `high`, with no skill
  and with the skill, five runs a cell, beside atelier-s6xk's numbers.

## Scope

Case files and recorded reviews under `evals/agent-factory/scenarios/`,
their README, and `evals/agent-factory/results/`. No change to the harness
or the shared replay setup.

## Report

Sol does not question work this repository later removed as unneeded. In
50 reviews of five such changes, on `gpt-6.1-sol` at `high` with five runs
a cell, no finding questioned the need for the removed part or asked for
less of it, with the skill or without. On atelier-s6xk's planted cases the
same reviewer named the unneeded part in 59 of 60 runs. Results:
`evals/agent-factory/results/2026-11-19-removed-replays.md`.

- Why the gap: in s6xk's cases the excess is code beyond what the item
  asks for. In each replayed change, the part later removed is what the
  item asks for: every Done when line names it. With the skill, each
  reviewer rated the value high or medium from the item's own Goal
  ("high, the outcome needs it"), so `roles/reviewer.md`'s `Reject:` never
  came up. Each skill-arm review that can still be read, 20 of 25, says
  so in its `Value:` line. Even `merge --wait`, whose own Unknowns names
  the 10-minute limit it was later removed for, drew no such finding.
- The reviews were not idle. They asked for 119 fix-now fixes, and the
  judge found none unsupported (unknown in three runs). Many are real
  defects in the old code, such as `merge --sync` merging the base by a
  short name that a same-named tag can shadow, found in all ten runs.
- Five cases, one change each: `removed-merge-sync` (3ae56002),
  `removed-merge-wait` (bc495ebb), `removed-base-exemption` (cb30c86b),
  `removed-debrief` (43eefb4b) and `removed-decision-notes` (7a371ee7).
  All five were removed in 819024c75, the rewrite's deletion of the old
  workspace. Only `removed-merge-wait`'s row is from Tried and removed's
  first table. The other four are from its table of what the redesign
  removed from 0.4.0, since the first table's other rows did not replay.
  The README gives a line for each row that does not replay: too large
  (over 1,900 lines, against 937 for the largest defect replay), inherited
  with the fork, built with no item, or held under `.atelier/`, which the
  shared setup drops.
- The cases use `harness/replay.py` and its confinement unchanged. The
  removal goes in the fields replay.py names for a defect (`defect`,
  `fix`, `defect_item`). The shared rubric tells the judge that "The known
  defect" holds the part and design.md's reason, and that it need not read
  "The later fix", the whole workspace's deletion. The reviewer is told
  nothing.
- Grading: `questions-need` is graded and passes on a finding of any
  recommendation; `blocks`, `others-supported` and `reasons` are reported.
  I checked the judge's answer against every finding title, and against
  the text of each review whose work directory survived. It agrees
  everywhere.
- `test.sh` runs each case's `test_grade.py`, which builds the fixture and
  grades recorded reviews with no model and no cargo build. The whole of
  `test.sh` passed.
- Outside the Scope: one new file in the harness, `removed_tests.py`. It
  holds the cases' shared tests, replay_tests.py's with those about a
  defect swapped. It changes no existing harness file.

Fixture costs of the shared setup: it drops `.agents/`, so the
decision-notes change lacks the skill guidance its last Done when line
asks for. All ten of its reviews asked for that guidance as fix now. The
base-exemption change lacks one sentence of the vendored skill reference.

The matrix filled the host's disk once, with my `--keep` and on a disk
with about 30 GB free. The harness keeps a failing replay run's work
directory, cargo `target/` and all, about 2.7 GB a run, and nearly every
run of these cases fails. I deleted my runs' work directories, and made
the 17 `null` runs again without `--keep`, with a loop deleting `target/`
from each graded run. A harness that removes `target/` from every replay
run it keeps would prevent this. That is outside this item, so I have
not built it.

For atelier-ufbm: a passage that asks which requirement each new part
serves cannot help here, since every part serves a Done when line. These
cases are at their floor, not their ceiling, so they can show a passage
that makes a reviewer weigh the requirement itself.
