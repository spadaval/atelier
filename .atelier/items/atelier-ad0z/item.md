---
schema: atelier.item
schema_version: 3
id: atelier-ad0z
title: Replay changes that were built and later removed as unneeded, as review cases
status: building
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

- [ ] At least five rows of Tried and removed whose building was one
  change a reviewer can hold are replayed as review cases, each with a case
  file naming its row, its base and change commits, the part later removed
  and the commit that removed it. A row that does not replay has a line
  saying why.
- [ ] They use the shared replay setup and its confinement with no change
  to either. A case that needs one is left out.
- [ ] A run passes when the judge finds that a finding questions the need
  for the part later removed, or asks for less of it. Whether the finding
  blocks is reported. The judge is given the removal's reason from
  `docs/design.md`; the reviewer is told nothing.
- [ ] `test.sh` builds each fixture and grades a recorded review, with no
  model and no `cargo` build.
- [ ] The results give each case on `gpt-6.1-sol` at `high`, with no skill
  and with the skill, five runs a cell, beside atelier-s6xk's numbers.

## Scope

Case files and recorded reviews under `evals/agent-factory/scenarios/`,
their README, and `evals/agent-factory/results/`. No change to the harness
or the shared replay setup.
