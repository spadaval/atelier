---
schema: atelier.item
schema_version: 3
id: atelier-id3v
title: Run both setups and both arms, write the results, and rewrite the README around the three questions
status: building
mission: atelier-kw8u
labels: []
priority: medium
blocked_by:
- atelier-1tkg
- atelier-s63j
- atelier-6ve6
- atelier-69oh
implementers:
- thread:delegated-task:command%3Amcp%3A3a9fab39-7f29-4732-8284-d6c07f2deac6%3Adelegate-task%3Ad9507e75-71e6-45a7-8d94-f948bd45b50c
ready:
  by: 'the human, in conversation on 2026-11-18: put the mission together and then run it'
  at: 2026-11-18T23:49:47.284668623Z
trivial: null
caused_by: null
created_at: 2026-11-18T20:05:52.226322940Z
---

## Goal

Run what the mission built, and leave a README that says what the evals
measure today. The README is 320 lines, most of them about hand reads and
harness history.

## Done when

- [ ] One matrix in `sol` and one in `house`, each with both arms and every
  case, is recorded in `evals/agent-factory/results/`. Each table gives
  passes, median seconds and tokens per cell, and the file says how many
  runs each difference it names rests on. A cell has five runs, and a
  replayed review three, since those are read across the twelve cases. Only
  one matrix runs on this host at a time.
- [ ] In `sol`, the cases whose first agent starts threads are also run
  with a third arm: a skill that holds only `SKILL.md`'s Starting Another
  Thread section, from a branch that is not merged. Codex adds a rule of its
  own that a thread starts no subagent unless the user, an `AGENTS.md` or a
  skill asks (atelier-69oh), and T3 Code's Codex threads keep it. This arm
  tells what the skill adds beyond that one sentence, and the file gives it
  beside the other two.
- [ ] The lead's cases and the mission case are also run with the skill as
  it was before atelier-mtzj, by `--ref`, in both setups. For each cell the
  file gives the runs where a planted defect reached main, the runs that
  stopped with nothing raised, the departures with no recorded reason, and
  each recorded departure with the judge's reading of its reason. Nothing is
  reverted on a count: the human reads it.
- [ ] Thirty runs, drawn across cases and both verdicts, are read by the
  thread that writes the results. The file lists every run where its reading
  differs from the judge's, and there are at most two.
- [ ] The README is rewritten around the three questions. Its table says,
  for each case, which question it answers and its last result in each
  setup. A case that passed every run of every cell is cut, or the table
  says what it guards. It says the planner, the gardener, the backlog
  procedure and the readiness checklist are unmeasured.
- [ ] The mission's Result gives the line counts of the graders and the
  README before and after, as an indication and not a target.

## Scope

`evals/agent-factory/results/` and `README.md`, and the scenarios where a
case is cut.
