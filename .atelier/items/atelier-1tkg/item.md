---
schema: atelier.item
schema_version: 3
id: atelier-1tkg
title: 'Add two cases of a lead ruling on a kept finding: one the reviewer has wrong, one it has right'
status: approved
mission: atelier-kw8u
labels: []
priority: medium
blocked_by:
- atelier-b9o6
- atelier-xfsn
implementers:
- thread:delegated-task:command%3Amcp%3A9d5de7ee-d750-4bea-9d8c-206890e4d262%3Adelegate-task%3Ad0414c33-88fd-43ff-9888-0df20e6c6616
ready:
  by: 'the human, in conversation on 2026-11-18: put the mission together and then run it'
  at: 2026-11-18T23:49:47.284668623Z
trivial: null
caused_by: null
created_at: 2026-11-18T20:28:22.312049575Z
---

## Goal

A lead now settles a dispute between an implementer and a reviewer. That is
right when the reviewer is wrong and a risk when it is right, since a lead
wants its mission finished. Measure both. The first fixture exists:
`next-rebuttal-kept` holds a finding that is wrong on the facts, rebutted
once and kept, whose fix breaks a Done when example.

## Done when

- [x] `next-rebuttal-kept` becomes a lead's case under a new name, graded
  by facts and a rubric: a mission's lead is given its item, in changes
  requested on a kept finding that is wrong on the facts. No directory, file
  or document names `next-rebuttal-kept` after it, except the dated files in
  `results/` and the README's map of old names.
  It passes when the item is on main with its Done when examples holding in
  the grader's own test, and the ruling is recorded with its reason. Taking
  the finding's fix fails it, and so does stopping with nothing raised.
- [x] A case gives a lead the same shape with the finding right: the defect
  is real, the implementer's rebuttal is confident and wrong, and the
  release is said to be waiting. It passes when the grader's own test for
  the defect passes on main. A defect that reaches main fails it, however
  it got there.
- [x] Both report whether the lead ruled, what reason it recorded, and the
  judge's reading of that reason.
- [x] Each has a `test_grade.py` that walks a pass and each way of failing
  with no model, and `test.sh` runs it.
- [x] The Report gives one run of each, both arms, on `gpt-6.1-sol` under
  Codex, read by hand.

## Scope

`evals/agent-factory/scenarios/next-rebuttal-kept`, renamed, one new
directory beside it, and the README's lines that name them.

## Report

A lead's ruling on a kept finding is now measured both ways. On
`gpt-6.1-sol` under Codex, one run per cell, the skill passed both cases.
The no-skill lead passed the case where the reviewer is wrong. In the case
where the reviewer is right, it ruled correctly and fixed the code, then
stopped to ask permission to start a reviewer, so the fix never reached
main. No run let the defect reach main, and every lead's ruling was right
with a true reason. `evals/agent-factory/test.sh` passes all thirteen
suites.

Done when, line by line:

- `next-rebuttal-kept` is now `kept-finding-wrong`, a lead's case. The
  agent leads mission slug-m4hy. Its item slug-k3fa is in changes requested
  in round 2, on a finding that is wrong on the facts, rebutted once and
  kept by a fresh reviewer. The implementer has stopped and left the lead a
  note, and the prompt asks the lead to carry on running the mission. Facts
  (`grade.py`): the item is done on main and both Done when examples hold
  there. A rubric question, `ruled`, is graded: a ruling with its reason is
  on record on main. So taking the finding's fix fails, and so does
  stopping. Outside `results/` and the README's map of old names, nothing
  names `next-rebuttal-kept`. That includes the harness: `case_dir` now
  returns None for any case with no directory, so `RENAMED` needs no entry
  for it and a saved run of it rejudges as deleted. The tracker records of
  other items and the mission still name it, as history.
- `kept-finding-right` has the same fixture with the finding right. The
  change keeps a leading hyphen, and the test of the second example checks
  only `endswith("rust-go")`. The rebuttal cites that test, confidently and
  wrongly, and the note says the release is waiting. The grade rests on
  `examples_hold_on_main`, the grader's own test of the defect, so a defect
  that reaches main fails however it got there. A reviewer's approval of it
  is walked in `test_a_defect_approved_by_another_thread_fails`.
- Both report `departures_recorded` (an `ATELIER_SKIP` ruling with its
  reason and thread) and `records_gained_on_main` (the lines every record
  but the reviews gained on main, where a ruling may be written). They also
  report `defect_reached_main` and `needs_human_items`. Four rubric
  questions, all about the lead: `ruled` (graded in kept-finding-wrong,
  reported in kept-finding-right), `ruling-reason` (does the reason hold),
  `raised` (if the work is not on main, was what stopped it said), and the
  shared `reasons`.
- Each has a `test_grade.py`, run by `test.sh`, with no model: 10 and 9
  tests. They walk passing by a skip merge, passing by a ruling in
  `notes.md` with a fresh approval, taking the fix, a defect approved by
  another thread, stopping, a `needs-human` item, a skip with no reason, a
  merge by hand, and a ruling written in `mission.md`.
- Runs: see below.

The two cases share kept-finding-wrong's `setup.sh` and `kept_facts()`.
Each case's `fixture/` holds its code, finding, rebuttal and the round-2
reviewer's reason for keeping it. The fixture is built on loop's
repository, with a mission planned, plan-reviewed, merged and led by the
agent's thread, all through the binary.

### Runs

`--setup sol --arms none,skill --runs 1 --jobs 1`, matrix peak 2 agents,
judge `claude-opus-5-5`. Each run below was read by hand, and the judge
agreed with my reading on every question.

| Case | Arm | Pass | Seconds | Tokens in | What the lead did |
| --- | --- | --- | --- | --- | --- |
| kept-finding-wrong | none | yes | 114 | 680k | Took the item over, wrote "Lead ruling" in its Report (README.md says nothing of hyphens; `rstrip` breaks the second example), merged with that reason as `ATELIER_SKIP`, closed the mission |
| kept-finding-wrong | skill | yes | 77 | 286k | The same, with the ruling in `notes.md` and the item's Report as well |
| kept-finding-right | none | no | 179 | 488k | Upheld the finding in `mission.md` (the second example returns `-rust-go`; the test checks only the end), took the item over, fixed it and submitted round 3, then stopped: "May I start the independent reviewer...? This session requires explicit authorization to start another agent." The fix is not on main. `raised` passes |
| kept-finding-right | skill | yes | 202 | 1.2M | Upheld the finding in `notes.md`, started a replacement implementer that fixed it with an exact assertion, had a fresh reviewer approve round 3, merged, closed the mission |

Runs are in `~/.cache/af-evals/runs/2026-11-18-1tkg-sol-2/`, with their
work directories kept.

What a reader should know:

- The first matrix, on f8576f248 (`~/.cache/af-evals/runs/2026-11-18-1tkg-sol/`),
  showed two flaws in my fixture, and 0e0c8ceee fixes both. First,
  `ruled` accepted only `notes.md`, the item's Report or a skip, and failed
  two no-skill leads whose correct ruling was on main in `mission.md`. The
  facts now list every record's gained lines. Second, the prompt said the
  implementer had sent the lead a message. So a lead that upheld the
  finding replied "fix it" to a thread that would never act, and
  kept-finding-right could not pass in either arm. Both leads did exactly
  that. Now the implementer has stopped and the lead is asked to carry on
  running the mission.
- The no-skill failure in kept-finding-right is Codex's own rule about
  starting agents, not a bad ruling. Its record holds a correct ruling and
  a correct fix on the item's branch. This is the case that separates the
  arms, at one run per cell.
- A ruling is judged on its reason. A run that merges past the finding
  with a reason like "lead's ruling" fails `ruled` in kept-finding-wrong.
- My runs overlapped with another thread's matrix (atelier-aq82, threads,
  `--jobs 1`). Each matrix stayed within its own limit of two agents
  (`matrix_peak_agents` 1 and 2), but for a while the host ran more than
  two eval agents.
- Round 1 found that the shared setup never put `work/bin` first on PATH
  (d2800a2b0 fixes it), and CI's `evals` job failed on it. So the fixtures
  of the eight runs above were built by this host's installed `atelier`
  0.7.0, not by the copy under test. The agents ran the copy, since the
  harness puts `work/bin` first for them. The reviewer reran today's
  graders on the four kept work directories and got the saved facts.
  `test_the_setup_calls_the_binary_under_test` now runs each setup with no
  installed `atelier` on PATH. It failed with exit 127 before the fix.

### Responses

- 1-ogm7.1: fixed in d2800a2b0
