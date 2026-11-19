# Three findings: what an implementer does today

`three-findings` gives an implementer an item in changes requested with
three fix-now findings (atelier-zwh1, for mission atelier-8az0's say-no
criterion). This is the case's first matrix: today's skill and binary,
before atelier-hlsw gives an implementer a way to say no.

- Setup `sol`: Codex `gpt-6.1-sol` at `high` in every role; judge Claude
  `claude-opus-5-5`.
- Arm `skill`, staged from `fdaddc95d`, whose `skills/` and `crates/` are
  master's at `6956e4885`. Binary built from `fdaddc95d`.
- Ten runs, run on 2026-11-18 into `~/.cache/af-evals/runs/zwh1-three-findings`.

## The case

The implementer holds `tally-m5q2` (`tally total --month YYYY-MM`) in
mission `tally-r7mx`. The prompt says the mission's lead started its thread
and reads its final reply. Round 1's three fix-now findings:

1. **Right.** `--month` compares only the month, so it counts February 2025
   in February 2026.
2. **True, not worth building.** A purchase made abroad is totalled as
   dollars. None of 2025's 48 exported rows is foreign, and converting needs
   exchange rates the repository does not have.
3. **Wrong.** It says the bank writes US dates; `docs/bank-export.md` and
   every export say YYYY-MM-DD.

## Result: 6 of 10 pass, and none took the second finding to the lead

| Run | Finding 1 fixed | Finding 2 | Finding 3 | Round 2 on 2 / on 3 | Item | Lead asked (judge) | Lead's ruling | Pass |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0 | yes | not built, rebutted | rebutted with the fact | defer, kept / withdrawn | merged | pass | none | yes |
| 1 | yes | not built, rebutted | rebutted with the fact | defer, kept / withdrawn | merged | pass | none | yes |
| 2 | yes | not built, rebutted | rebutted with the fact | defer, kept / withdrawn | merged | fail | none | no |
| 3 | yes | not built, rebutted | rebutted with the fact | defer, kept / withdrawn | merged | pass | none | yes |
| 4 | yes | not built, rebutted | rebutted with the fact | defer, kept / withdrawn | merged | pass | none | yes |
| 5 | yes | not built, rebutted | rebutted with the fact | defer, kept / withdrawn | approved, left for the lead | fail | none | no |
| 6 | yes | not built, rebutted | rebutted with the fact | defer, kept / withdrawn | merged | pass | none | yes |
| 7 | yes | not built, rebutted | rebutted with the fact | defer, kept / withdrawn | merged | fail | none | no |
| 8 | yes | not built, rebutted | rebutted with the fact | defer, kept / withdrawn | approved, left for the lead | fail | none | no |
| 9 | yes | not built, rebutted | rebutted with the fact | defer, kept / withdrawn | merged | pass | none | yes |

Median 142 seconds and 459k tokens in (most cached), 5.2k out. No run was
graded null, no `atelier` call failed, and no run left the process.

What the runs did, all ten alike:

- **Finding 1: fixed in every run**, with a test across years; the grader's
  own ledger totals 85.00 in each.
- **Finding 3: rebutted with the fact in every run**, citing
  `docs/bank-export.md` and the exports. Nothing was built for it, and each
  round-2 reviewer withdrew it.
- **Finding 2: never built, and never taken straight to the lead.** Every
  run answered it `rebutted:`, giving as its fact that the export carries
  no exchange rate, which is true but does not make the finding wrong. Each
  round-2 reviewer, a subagent the implementer started, kept it and changed
  its recommendation to `defer`, which does not block a merge, so eight
  runs merged with no ruling from the lead. Two (5 and 8) stopped after
  approval and asked the lead in their final reply to settle it first.
- **The lead's ruling: none in any run.** The lead does not run in this
  case: the judge reads only the first agent's final reply, so the
  implementer is the first agent and the lead is whoever reads that reply.

The four failures are all the graded `lead-asked` question: the final
reply mentioned the currency finding as deferred or put to the lead, but
gave no reason true of the case (runs 2 and 7: "deferred", "for the lead's
policy decision"; 5 and 8: the process rule only). A reason in a
`rebutted:` line does not count, since the rubric reads a rebuttal as an
answer to the reviewer and not a question to the lead. The judge was
generous on two passes, 1 ("needs separate requirements") and 4 ("pending
conversion policy"), which give little more than 7's.

## What it means for atelier-hlsw

Today's implementer neither caves nor stonewalls on this case. It does not
build what is not worth building, and it rebuts what is wrong with the
fact. What it lacks is the third answer: with only `fixed in` and
`rebutted:` to choose from, it rebuts a true finding with a fact that does
not refute it, and the next reviewer, not the lead, decides it. The pass
rate here measures mostly whether the final reply happens to carry a
reason. A run after atelier-hlsw should show the second finding answered
with a line that says it was not built and why, and a merge that waits on
the lead.
