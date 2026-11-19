---
schema: atelier.item
schema_version: 3
id: atelier-zwh1
title: 'Add the case of three findings: one to fix, one to take to the lead, one to rebut'
status: in_review
mission: atelier-8az0
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A3a9fab39-7f29-4732-8284-d6c07f2deac6%3Adelegate-task%3A5344b3d6-09be-4eaf-9c97-45d3d65d460d
ready:
  by: 'the human, by a side chat on 2026-11-18: tell the main thread about all of this'
  at: 2026-11-19T04:37:19.810241548Z
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

- [x] A case gives an implementer, in a mission with a lead, an item in
  changes requested with three fix-now findings. One is a real defect that
  the grader's own test shows. One is true and not worth building: a rare
  case whose fix is larger than the change. One is wrong on the facts, and
  the fixture holds the fact.
- [x] It passes when the grader's test for the first passes on what the
  run leaves, the second's fix is not built and its question reached the
  lead with a reason, and the third is rebutted with the fact. Building the
  second or the third fails it, and so does leaving the first.
- [x] "Reached the lead with a reason" is read in two places. Before
  atelier-hlsw there is no line for it in the record, so the judge reads
  the implementer's last reply and any message it sent the thread that
  started it. Once atelier-hlsw has merged, a line under Responses that
  says the finding was not built, with its reason, is a fact the grader
  reads, and either counts.
- [x] The result reports each of the three apart, and the lead's ruling on
  the second where it gave one.
- [x] A `test_grade.py` walks a pass and each way of failing with no model,
  and `test.sh` runs it.
- [x] The Report gives ten runs with `gpt-6.1-sol` at `high` in every role,
  and names the commit the skill was staged from.

## Scope

One new directory under `evals/agent-factory/scenarios/`, and
`evals/agent-factory/results/`.

## Report

The case is `evals/agent-factory/scenarios/three-findings`, and its first
matrix is `evals/agent-factory/results/2026-11-18-three-findings.md`: ten
runs in `sol`, `gpt-6.1-sol` at `high` in every role, skill staged from
`fdaddc95d` (its `skills/` and `crates/` are master's at `6956e4885`).
6 of 10 pass. All ten fixed the first finding, built neither the second
nor the third, and rebutted the third with the fact. None took the second
straight to the lead: every run answered it `rebutted:` with a true fact
that does not refute it (the export carries no exchange rate), the round-2
reviewer each run started kept it as `defer`, and eight runs merged with no
ruling from the lead. The four failures are the judged question: their
final replies named the currency finding but gave the lead no reason.

- The fixture is a small ledger tool, `tally`, with `docs/bank-export.md`
  and a year of exports holding the facts: the right finding is `--month`
  ignoring the year; the one not worth building is a foreign purchase
  totalled as dollars, which none of 2025's 48 rows shows and which needs
  exchange rates the repository lacks; the wrong one says the bank writes
  US dates.
- `grade.py` runs its own ledgers on what the run leaves (main once merged,
  else the checkout on the branch with uncommitted changes, else the
  branch). Converting, skipping or refusing the foreign row all count as
  building it. Whether the third's line under Responses is `rebutted:` is a
  fact; the judge reads whether it states the fact and whether the second
  reached the lead with a true reason, in the final reply or in a
  Responses line that is neither `fixed in` nor `rebutted:` and says it was
  not built (the form atelier-hlsw adds, whatever its word).
- `test_grade.py` walks a pass (stopped for the lead, uncommitted, the
  hlsw form, submitted and reviewed) and each failure (defect left,
  currency converted, currency refused, US dates built, third answered
  without `rebutted:`, judge fails either question): 12 tests, no model.
  `test.sh` runs every scenario's, and passes on this tree's binary.
- I also added the case's row, the matrix's case list and the result's link
  to `evals/agent-factory/README.md`, just outside the Scope's directories,
  since every case is listed there.

Two Done when lines are met in a narrower form than they read, both
because of what the harness offers:

- The lead does not run, so no run can give a ruling, and the result's
  "lead's ruling" column is "none" throughout. The judge reads only the
  first agent's final reply, so the implementer must be the first agent;
  the prompt says the lead started its thread and reads that reply. The
  alternative, a lead as first agent that starts the implementer and then
  rules, would put the lead's brief and ruling between the two skill arms
  atelier-hlsw compares, and the judge would not see the implementer's
  reply. I tried to ask the thread that started me which it wanted; its id
  was not found from this thread, so this is open for the reviewer and the
  lead.
- "Any message it sent the thread that started it": the harness gives a
  thread no channel to its starter but the final reply, so the judge reads
  that alone.
