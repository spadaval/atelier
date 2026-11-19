---
schema: atelier.item
schema_version: 3
id: atelier-id3v
title: Run both setups and both arms, write the results, and rewrite the README around the three questions
status: in_review
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

- [x] One matrix in `sol` and one in `house`, each with both arms and every
  case, is recorded in `evals/agent-factory/results/`. Each table gives
  passes, median seconds and tokens per cell, and the file says how many
  runs each difference it names rests on. A cell has five runs, and a
  replayed review three, since those are read across the twelve cases. Runs
  go side by side within the caps atelier-u708 sets.
- [x] In `sol`, the cases whose first agent starts threads are also run
  with a third arm: a skill that holds only `SKILL.md`'s Starting Another
  Thread section, from a branch that is not merged. Codex adds a rule of its
  own that a thread starts no subagent unless the user, an `AGENTS.md` or a
  skill asks (atelier-69oh), and T3 Code's Codex threads keep it. This arm
  tells what the skill adds beyond that one sentence, and the file gives it
  beside the other two.
- [x] The lead's cases and the mission case are also run with the skill as
  it was before atelier-mtzj, by `--ref`, in both setups. For each cell the
  file gives the runs where a planted defect reached main, the runs that
  stopped with nothing raised, the departures with no recorded reason, and
  each recorded departure with the judge's reading of its reason. Nothing is
  reverted on a count: the human reads it.
- [x] Thirty runs, drawn across cases and both verdicts, are read by the
  thread that writes the results. The file lists every run where its reading
  differs from the judge's, and there are at most two.
- [x] The README is rewritten around the three questions. Its table says,
  for each case, which question it answers and its last result in each
  setup. A case that passed every run of every cell is cut, or the table
  says what it guards. It says the planner, the gardener, the backlog
  procedure and the readiness checklist are unmeasured.
- [x] The mission's Result gives the line counts of the graders and the
  README before and after, as an indication and not a target.

## Scope

`evals/agent-factory/results/` and `README.md`, and the scenarios where a
case is cut.

## Report

The matrix ran in both setups with both arms on every case: 469 graded
runs in `evals/agent-factory/results/2026-11-19-both-setups.md`.
In `sol` the skill took the lead's cases from 4 of 25 to 25 of 25, and the
skill cut to its Starting Another Thread section did the same, so in `sol`
the skill's effect on passes is lifting Codex's own subagent rule. In
`house` every lead run passed in both arms, and the skill cut a lead's time
and tokens in all five cases. On reviews the skill took `costly-hardening`
from 0 of 10 to 10 of 10, and did not help on the replays: 17 of 72 known
defects found without it, 12 of 72 with it, with `touched-claims` at 5 of 6
against 0 of 6. The skill before atelier-mtzj passed every lead run too, at
more cost. No planted defect reached main in a lead's case, and the judge
held on all 30 runs read by hand.

- **The lead changed a Done when line.** On 2026-11-18 (03:44 UTC on
  2026-11-19), after the human lifted the limit of two eval agents, the
  lead replaced "Only one matrix runs on this host at a time" with "Runs go
  side by side within the caps atelier-u708 sets" and asked for it to be
  made on this branch (a54b69b12). Master, atelier-u708 included, was
  merged in, not rebased (91adbdf18).
- **Reused and remade.** The first 40 `sol` runs went one at a time before
  atelier-u708 landed; all 40 were kept by the rerun on the same setup,
  binary and skill commit. None of the 469 ended on a rate limit or a spent
  allowance, so none was rerun for one. Twelve runs were made again: three
  `costly-hardening` runs with no skill that OpenAI ended as "flagged for
  possible cybersecurity risk" (one in `sol`, two attempts at one `house`
  run), three cut short when the old matrix was stopped, and six `house`
  replays that ran out of disk.
- **`refused-push` ran on its own.** Its cells ran in an invocation of
  their own at `--jobs 1` in each setup, beside the rest, since its
  fixture's server binds port 8080.
- **The disk filled.** At 04:08 on 2026-11-19 this host's disk was full:
  my replay runs' work directories held 132G, since the harness keeps a
  failing run's work directory and a replay's holds about 2.5G of cargo
  output. I deleted the `target/` of my finished runs (125G), set aside the
  three graded replays that had seen "No space left on device"
  (`house-replays/discarded-enospc.jsonl` beside the runs), and made them
  again with a cleaner deleting finished replays' `target/`. Filed as
  atelier-scly in this mission, merged as a draft (PR #257).
- **Which cases got the third arm.** "Cases whose first agent starts
  threads" is read as the seven whose pass needs another thread: `loop`,
  `loop-changes` and the five lead cases. `refused-push`,
  `done-when-conflict` and `non-atelier-repo` run as threads but pass with
  none started. The arm's skill is commit 3f9dfda5b on
  `eval/skill-threads-only`, pushed and never to be merged. "The lead's
  cases and the mission case" is read as the five lead cases.
- **No case is cut.** `loop-changes`, `done-when-conflict`,
  `non-atelier-repo` and `vacuous-test-review` passed all 20 of their runs.
  The README says what each guards, in a list beside its table. Eight
  replays were found by no review; the README keeps them, as defects review
  missed.
- **Stopped with nothing raised.** The results count a stopped run as
  raised where its final reply asks the person who started it. All 21
  stopped lead runs (`sol`, no skill) asked leave to start a subagent; I
  read each reply.
- **The README** is 429 lines, from 649 on master and 234 when the plan
  merged. The harness detail it dropped is in `harness/run.py`'s docstring;
  the old `next-` names are one sentence. Master gained 18 cases from
  mission atelier-8az0 while the matrix ran (`three-findings`, the sized
  reviews, the removal replays and the later rounds). They were not in this
  matrix; the README gives each the results its own item recorded, marks
  the setups and arms it has not run in, and keeps the ones that passed
  every run for what they guard.

Checked: `git diff --check`, every relative link in the README and the
results file resolves. No code changed, so no tests were run.

### Responses

- 1-nmv8.1: fixed in 5d860dd1d. The summary script now takes a count of unsupported findings only where the judge answered `others-supported` fail, counts pass as none, and keeps unknown apart. Of the 144 answers, 126 were pass, 18 unknown and none fail, so every Unsupported cell is 0, and the Result and the replay section say no fix-now finding was found unsupported and that the judge could not tell in 18 reviews.
- 1-nmv8.2: fixed in 5d860dd1d. The Setup says every run used the pinned copy of 4f36b1fa6 and its skill, and that atelier-hlsw and atelier-d73k changed master's crates, and atelier-hlsw its skill, during the matrix.
