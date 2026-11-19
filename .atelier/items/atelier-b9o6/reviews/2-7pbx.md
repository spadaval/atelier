---
schema: atelier.review
schema_version: 3
round: 2
commit: 863d7305a075e8c2b17e6b02a45bf3255d81be98
reviewers:
- thread:delegated-task:command%3Amcp%3A8d837d81-71dd-4c23-805b-4f0a4561ef59%3Adelegate-task%3Aec6e3b1c-dd59-4494-8b6f-e4c1d8333309
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T00:48:42.815517236Z
---

## Summary

Complexity: high. This introduces a model-judging subsystem, a rubric contract,
shared fact readers and saved-run regrading across ten suites.
Value: high. Replacing wording heuristics is necessary to make these evaluations
measure the work rather than the phrasing.
Risk of merging: high. A leaked arm can silently bias grades, and a malformed
judge reply can discard a run and stop a matrix.

The value justifies the change, but two correctness problems remain. I reviewed
the full harness and grader changes against the Done when lines and checked the
round-2 diff against all four carried findings. All ten suites pass via
`evals/agent-factory/test.sh "$(command -v atelier)"` (161 tests), including
cross-lab routing, reported answers, the original malformed-reply reproductions,
unsupported quotations and external-checkout changes. `git diff --check` passes.

I checked the 42 kept result/rejudge pairs against the Report's totals and the
one difference from the old grader, verified that all 76 saved answers quote
supplied material, and inspected the costly-hardening decisions. I did not run
new paid judges or independently recreate the author's entire hand count.
Model-free end-to-end harness reproductions below show a remaining malformed
reply crash and an arm leak through ordinary final-reply file links.

## Finding 1: Malformed judge replies abort grading instead of failing their questions
Recommendation: fix now
Was: 1-0nu9.1
Response: fixed in d8f2cf036
Now: kept

The two original examples are fixed, but response validation still lets malformed
judge output abort the run. In `evals/agent-factory/harness/judge.py:281`, the
answer index uses `a.get("id")` as a dictionary key without checking its type.
Both `{"answers":[{"id":[],"answer":"pass","line":"ok"}]}` and the same reply
with `"id":{}` raise `TypeError` instead of producing unknown answers.

I supplied each reply through `JudgeTest`'s fake judge and called `run_house()`.
Both exceptions propagated out of `run_one`; neither run wrote `result.json`.
This is the same failure the carried finding addresses: a model's malformed
reply loses the grade and can stop the matrix, rather than failing its graded
questions with a reason. Validate answer IDs before indexing and cover malformed
entries through the harness, preserving the result and the unanswered reason.

## Finding 2: A graded pass is accepted without a supporting line
Recommendation: fix now
Was: 1-0nu9.2
Response: fixed in d8f2cf036
Now: fixed

`parse_answers` now changes pass/fail to unknown when the supporting quotation is
missing or absent from the supplied material. The added tests cover both cases
and verify that unknown fails a graded question while leaving a reported
question non-grading. The house fixture now quotes actual supplied material.

## Finding 3: External review checkout paths reveal the arm to the judge
Recommendation: fix now
Was: 1-0nu9.3
Response: fixed in d8f2cf036
Now: fixed

`facts.reviews` now uses the checkout directory's name instead of its absolute
path. The external-checkout prompt test under a `-skill-house-0` directory passes
and removes the specific facts leak. Finding 5 covers a separate remaining
source of arm disclosure.

## Finding 4: Code edits in an external review checkout escape the file-change fact
Recommendation: fix now
Was: 1-0nu9.4
Response: fixed in d8f2cf036
Now: fixed

`touched` and `body_changed` now scan all worktrees. The external-checkout test
confirms that unstaged code, a staged file and an untracked file all appear in
`other_files_changed` and fail the grade.

## Finding 5: Final-reply file links disclose the arm to the judge
Recommendation: fix now

In `evals/agent-factory/harness/judge.py:205`, `material` passes the final reply
through unchanged. `run_one` names the run's work directory with its arm, so
normal absolute Markdown file links in that reply reveal `-skill-` or `-none-`
to the judge. Making the judge's own directory anonymous does not blind this
input.

This occurs in 11 of the 38 saved judged runs used by the Report. For example,
`20261117-164924-next-vacuous-test-review-skill-codex-gpt-6.2-sol-0-7b61` supplies:

```
[Round 1 review](/var/tmp/af-evals/20261117-164924-next-vacuous-test-review-skill-codex-gpt-6.2-sol-0-7b61/repo/.atelier/items/price-r4nd/reviews/1-c7x6.md)
```

I also changed only the fake agent in `JudgeTest`'s temporary fixture to emit a
review link rooted at `Path.cwd()`. A complete `run_house()` put the arm-bearing
run directory in both judges' captured prompts. The existing blinding tests use
`ok` as the final reply, so they miss this ordinary output.

Normalize run-directory paths embedded in judge material to arm-free identifiers
before building the prompt, and apply quotation verification to that same
material. Add a full harness test whose final reply contains an absolute review
link. The saved comparisons cannot support the claim of blind judging until
this input is blinded too.
