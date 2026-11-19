---
schema: atelier.review
schema_version: 3
round: 3
commit: 51dc489b7be1c245e02a193e61d240cf0534e4fd
reviewers:
- thread:delegated-task:command%3Amcp%3A8d837d81-71dd-4c23-805b-4f0a4561ef59%3Adelegate-task%3A1756533f-05e7-4f48-9ad3-67a605461f36
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T00:55:52.034664540Z
---

## Summary

Complexity: high. This adds a model-judging subsystem, a rubric contract,
evidence collection across branches and checkouts, and shared fact readers.
Value: high. The evaluations need to measure the work rather than match the
reviewer's wording.
Risk of merging: high. Incorrect evidence or lost validity checks can silently
produce misleading grades.

The value warrants the change, but regrading must preserve a run's invalidity.
I reviewed the full harness and grader changes against the Done when lines,
the changes since round 2, both carried findings, the rubrics and the legacy
helper migration. All ten suites pass through
`evals/agent-factory/test.sh "$(command -v atelier)"`, with 162 tests, and
`git diff --check` passes. The harness tests cover cross-lab routing,
reported answers, malformed replies and final-reply link blinding.

I checked all 42 kept result/rejudge pairs against the Report's totals and
its one difference from the old grader. All 76 answer quotations occur in
the reconstructed blinded material, which contains none of the arm markers
listed in the Report. I also read the costly-hardening sizing answers,
including the smaller filename/hash snapshot alternative. I did not run
fresh paid judges or independently recreate the author's historical hand
count. A model-free end-to-end probe below exposes an invalid run becoming
a pass through the new regrading entry point.

## Finding 1: Malformed judge replies abort grading instead of failing their questions
Recommendation: fix now
Was: 2-7pbx.1
Response: fixed in 4271b208a
Now: fixed

`parse_answers` now indexes only entries with string IDs. The full harness
test covers both list and object IDs alongside truncated JSON and a wrong
answers type. Each malformed reply fails the graded questions with a reason
and preserves `result.json`.

## Finding 2: Final-reply file links disclose the arm to the judge
Recommendation: fix now
Was: 2-7pbx.5
Response: fixed in 4271b208a
Now: fixed

`material` now normalizes the work directory and its name across all four
inputs before building the prompt or checking quotations. The new full
harness test supplies an absolute review link, verifies its blinded form in
both judges' prompts, and verifies that the quotation remains accepted.
The saved-run material and quotation checks above confirm the reported
regrading used arm-free material.

## Finding 3: Regrading turns invalid runs into valid grades
Recommendation: fix now

In `evals/agent-factory/harness/judge.py:357-370`, `rejudge` reads the saved
result but does not preserve its `invalid` reason or pass that reason to
`run.judged`. The normal run path explicitly skips judging invalid runs;
the regrading path instead judges them and writes an ordinary pass or fail
without the reason that disqualifies the experiment.

I used `JudgeTest`'s stand-in agents and judges, set `codex-effort` to `low`,
and called `run_house()`. It correctly returned `pass: null`, no judge calls,
and `invalid: thread main (reviewer): it ran gpt-6.1-sol at low, and its role
is gpt-6.1-sol at xhigh`. Calling `rejudge` on that same saved run returned
`pass: true`, no invalid reason, and two judge calls. No model was used.

Invalid runs are retained by default, so a batch regrade can silently count
work from the wrong model, wrong effort or contaminated instructions as a
valid evaluation. Preserve the recorded invalid reason and null grade, skip
the judges for such runs, and cover this through the regrading entry point.
