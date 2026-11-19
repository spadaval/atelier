---
schema: atelier.item
schema_version: 3
id: atelier-1z52
title: Replay three changes whose defect was found after merge as review cases, and report what one review costs
status: building
mission: atelier-kw8u
labels: []
priority: medium
blocked_by:
- atelier-b9o6
- atelier-8e8e
implementers:
- thread:delegated-task:command%3Amcp%3Adfff1aea-5166-4f31-a652-65819a65bb75%3Adelegate-task%3Ab5915a39-b98f-42ba-891e-26f39f0866dc
ready:
  by: 'the human, in conversation on 2026-11-18: put the mission together and then run it'
  at: 2026-11-18T23:49:47.284668623Z
trivial: null
caused_by: null
created_at: 2026-11-18T20:05:52.155883252Z
---

## Goal

The reviewer is the role the control pilot of 2026-11-10 found to matter
most, and it has two invented cases. This repository holds real ones:
`docs/architecture/quality/scorecard-baseline/defects.md` traces 15 defects
in atelier that were found after the merge that caused them, each to its
causing item and commits. Replay three as review cases, and learn what one
such review costs before building more.

## Done when

- [ ] A shared setup builds a fixture from a case file: this repository's
  tree at the change's base, tracked by today's binary with new records, and
  the change on an item's branch in review with the Goal and Done when it
  had, under a new item ID. The old records are not carried, the fixture's
  history ends at the change, it has no remote, and the case file stays
  outside it.
- [ ] A replay run is kept from the later fix. The repository is public,
  this host holds other checkouts of it, the case files name the fix, and
  today a run has the network and `$HOME`. So in a replay run: no process
  of the run can read this host's other checkouts of this repository or
  the case files; the agent's own web search and fetch tools are off; and
  its commands cannot fetch this repository from GitHub. The model's API
  stays reachable.
- [ ] A test, run where the confinement is, shows each: a command in a
  replay run's environment that reads another checkout, reads a case file,
  or fetches this repository fails.
- [ ] As a backstop, a run whose transcript shows it reached the later fix
  some other way is invalid and not graded, and the Report gives those
  runs.
- [ ] Three case files, each naming its row in `defects.md`, its base and
  change commits, the defect in a sentence and the commit that fixed it.
  They are chosen for a defect a reviewer could find from the change, and a
  diff one reviewer can hold.
- [ ] The rubric asks whether a fix-now finding names the known defect, and,
  for each other fix-now finding, whether the code supports it. The judge is
  given the defect and the fix's diff. The reviewer is told nothing.
- [ ] `test.sh` builds each fixture and grades a recorded review, with no
  model and no `cargo` build.
- [ ] The Report gives one review of each case by the reviewer of `sol` and
  by the reviewer of `house`: found or not, unsupported findings, minutes
  and tokens. It says what five runs of ten such cases would cost in each
  setup. Reviews that build with `cargo` run one at a time, each in its own
  target directory.
- [ ] The Report says what its author saw that would help or block leaving
  a replayed item ready for an implementer, or several ready under an open
  mission, in place of in review. It is an observation: no code is written
  for either.

## Scope

`evals/agent-factory/scenarios/` and `evals/agent-factory/harness/`; CI's
`evals` job where the setup needs this repository's history.
