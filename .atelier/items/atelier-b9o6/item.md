---
schema: atelier.item
schema_version: 3
id: atelier-b9o6
title: Grade a run by facts from its records and a rubric that a model from another lab reads
status: in_review
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A15ed7340-46a6-4e3a-b0c2-fdc0661b938f%3Adelegate-task%3A069600db-a8ea-4cce-bf07-9fc6ab504b4b
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

- [x] A case's `grade.py` prints facts only: named values read from the
  records, the tree and the run's recorded `atelier` calls. Its `rubric.md`
  holds the questions of meaning, each with what a pass looks like, and
  each marked as graded or as reported.
- [x] The harness puts each rubric question to a judge: a model from another
  lab than the thread whose work the question is about. Each question names
  the role it is about, and the setup names a judge for each lab its threads
  come from, so `house` has two. A harness test covers a `house` run whose
  reviewer's question goes to one judge and whose implementer's goes to the
  other.
  The judge is given the rubric, the facts, the final reply, the review
  files and the diff the run left. It is not told the arm.
- [x] The judge answers each question pass or fail with the line it rests
  on, and the harness records both in `result.json`. A run passes when every
  fact and every graded answer passes. A reported answer is recorded and
  shown and changes no grade: a harness test covers a run whose facts and
  graded answers pass and whose reported answer fails, and it passes. A
  graded question the judge cannot answer fails with its reason; only an
  invalid run is `null`.
- [x] Three cases are brought over, one of each kind:
  `next-vacuous-test-review` (judged), `next-costly-hardening` (judged, and
  no longer by its title's first word) and `next-loop` (facts alone). Their
  graders hold no regular expression over a review's or a reply's wording.
- [x] Each `test_grade.py` still walks a passing path and failing ones with
  no model: the facts directly, and the judged questions from recorded
  answers. `test.sh` passes.
- [x] The Report gives the judge's verdict beside the hand count for every
  kept run of these three cases in `~/.cache/af-evals/runs/gpt62` and
  `rt3o-smoke` whose review file survives, and explains each difference.

## Scope

`evals/agent-factory/harness/` and the three scenarios' directories.

## Report

A run's grade is now facts and a judge's answers. `grade.py` prints facts
read from the records, the tree and the call log (`harness/facts.py`), each
one the grade rests on with a pass. `rubric.md` holds the questions of
meaning, each with its role, graded or reported, and a Pass line.
`harness/judge.py` puts each question to the setup's judge for the lab of
its role's agent, one call per judge, in an empty directory under the work
root whose name carries no arm. The judge reads the rubric, the facts, the
final reply, the review files the run wrote and the diff it left, and
answers pass, fail or unknown with the line it rests on. `result.json`
keeps the facts, the answers (`rubric`), each judge's seconds and tokens
(`judged`) and what failed (`failed`). The run's per-run line prints the
reported answers.

Done when, line by line:

- Facts and rubric: `next-loop/grade.py`, `next-vacuous-test-review/grade.py`
  and `next-costly-hardening/grade.py` print facts only. The two reviewer
  cases have a `rubric.md` with one graded and one reported question each.
- Judge per lab: `judge.assign` routes by `setup["roles"][role]["agent"]`.
  `JudgeTest.test_each_question_goes_to_the_judge_from_another_lab_than_its_role`
  runs a `house` case end to end with stand-in agents and judges: the
  reviewer's questions go to `claude-opus-5-5`, the implementer's to
  `gpt-6.1-sol`. `test_the_judge_is_not_told_the_arm` checks the prompt and
  the judge's directory.
- Pass, fail, reported, unknown, null: `test_a_reported_answer_that_fails_changes_no_grade`
  passes a run whose reported answer fails. A judge that exits with no reply
  fails its graded questions with its reason
  (`test_a_judge_that_cannot_answer_fails_its_graded_questions_with_why`),
  and an invalid run is null and not judged. A grader that prints no JSON
  now makes the run invalid, where it used to make it null with no reason.
- Three cases: `next-costly-hardening` no longer reads its title's first
  word; the judge reads whether a fix-now finding asks for less. None of the
  three graders holds a regular expression over a review's or a reply's
  wording. `facts.findings` still splits a review file on its `## Finding`
  headings and `Recommendation:` lines, which is the record's structure.
- Walks with no model: each `test_grade.py` drives the binary as before and
  gives the judged questions as recorded answers through `Fixture.graded`.
  `test.sh` passes, all ten suites.

Other changes the Done when lines needed:

- Call and skip facts come from the call log, not the transcripts.
  `atelier_log.py` now also logs `ATELIER_SKIP`, so a skip through any
  `atelier` call, a Git hook's included, is a fact, and so is each
  `Atelier-Skip` trailer, read with `git log --format=%(trailers:...)`. A
  skip still fails these three cases; atelier-xfsn's Done when says no
  grader will.
- The shell parsing moved, unchanged, from `next-loop/grade.py` to
  `next-loop/shell_grade.py`, with the record readers it used to hold
  re-exported from `facts.py`. The six cases not yet brought over load it
  in place of `next-loop/grade.py`, a one-line change each. Its tests stay
  in `next-loop/test_grade.py` because `next-loop-changes` borrows
  `RefusalTextTest`. Delete both once atelier-xfsn lands.
- The judges run after the agents, one at a time, and count in a run's
  `agents`, so the limit of two holds with `--jobs 2`.
- `facts.reviews` can read other checkouts on the item's branch
  (`checkouts=True`), since on 2026-11-17 reviewers worked in `repo-<id>`.

### The judge beside the hand count

I read every kept run's round-1 review file before running the judge, and
wrote my count down first. Then `harness/judge.py` regraded all 42 kept runs
of the three cases with today's graders: 36 in
`~/.cache/af-evals/runs/gpt62` and 2 in `rt3o-smoke` for the reviewer cases,
and the 4 kept `next-loop` runs in `gpt62` (the passing ones' work
directories were deleted). The runs were one-model Codex setups, so the
judge was `claude-opus-5-5` at `high`. Each `rejudge.json` sits beside its
`result.json`.

| Case | Runs | Hand: pass | Judge: pass | Old grader: pass |
| --- | --- | --- | --- | --- |
| `next-vacuous-test-review` | 19 (gpt-6-astra, gpt-6.2-astra, gpt-6.2-sol: 3 each arm; rt3o 1) | 19 | 19 | 18 |
| `next-costly-hardening` | 19 (the same) | 10 (every skill run) | 10 | 10 |
| `next-loop` | 4 (no skill: gpt-6-astra 1, gpt-6.2-astra 1, gpt-6.2-sol 2) | 0 | 0 (facts) | 0 |

The judge's verdict differs from the hand count on no run. Its graded
answer matched mine on all 38 judged runs, and every line it quoted is in
the material. It took 3 to 10 seconds and 5k to 8k tokens in per run.

- The one difference from the old grader is rt3o-smoke's
  `next-vacuous-test-review`: "1000 cents at 10% off produces exactly 900
  cents, so both truncation and half-up rounding pass the assertion". The
  keywords had no phrase for it; the judge passed it, as the smoke run's
  hand read did.
- `next-costly-hardening`'s old grader agreed only because every skill run
  titled its finding `Shrink:` or `Reject:`, the skill's words. The closest
  call was gpt-6.2-sol skill run 1, "Shrink: use a snapshot of event
  filenames and hashes", which keeps a smaller check rather than a doc note.
  I read it as asking for less on cost grounds, and so did the judge.
- The 9 no-skill reviews each asked to fix or extend the ledger; none asked
  for less. The 4 `next-loop` runs each submitted and stopped, asking
  permission to start a reviewer, so the item is ready on main. Their new
  facts fail on the same five checks as the old grade.
- Every reported answer was pass: each review found the skipped latest
  entry, and each vacuous-test review said it ran the test on the old code.

All the runs agree, so this sample shows the judge does not mislead on these
cases; it does not test a hard call. atelier-id3v's thirty-run read covers
that.

Since round 1, an answer whose quoted line is not in the material counts as
unknown. Since round 2, the material writes the run's work directory as
`/work` and its name as `run`, because 11 of the 38 judged runs' final
replies linked files under a directory whose name holds the arm. I ran the
judge on all 42 runs again on the blinded material: every answer and every
verdict came out the same, and all 76 quoted lines are in the material, so
the table stands. A scan of each run's blinded material finds no `skill`,
`agent-factory`, `-skill-` or `-none-`.
None of the 42 was invalid, so the round-3 fix to regrading changes none
of them.

### Responses

1-0nu9.1: fixed in d8f2cf036
1-0nu9.2: fixed in d8f2cf036
1-0nu9.3: fixed in d8f2cf036
1-0nu9.4: fixed in d8f2cf036
2-7pbx.1: fixed in 4271b208a
2-7pbx.5: fixed in 4271b208a
3-bb9u.3: fixed in 88006ae9c
