---
schema: atelier.review
schema_version: 3
round: 2
commit: 86eb55dadf662c3d3fce1fedf00e7e9653d5f9a4
reviewers:
- thread:delegated-task:command%3Amcp%3Aef5c4a2f-47c0-4601-9465-2ae13c1b1a3c%3Adelegate-task%3Ad588ab1a-51d2-4f4d-99e4-a60e7c61e815
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-19T03:02:29.978039966Z
---

## Summary

Complexity: medium. Two cases share fixture construction and fact extraction, with separate rubrics and grader walks.
Value: high. Both a false finding and a real defect are needed to measure whether a lead's rulings preserve the mission's outcome.
Risk of merging: low. The change affects evals and their documentation, and is straightforward to revert.
The value justifies the size. The shared setup keeps the two cases aligned without duplicating their workflow.

I reviewed the change against master at 86eb55dad and the changes since round 1, including both fixtures, prompts, rubrics, graders, tests, the harness change and the README. Both cases exercise the required kept-finding state. The wrong-finding case requires completion on main and a recorded ruling with a reason; the right-finding case independently tests the acceptance examples on main, including when another thread approved the defect. The obsolete scenario name remains only in the README's historical map and dated eval results.

I read all four saved gpt-6.1-sol/Codex runs in 2026-11-18-1tkg-sol-2, their final replies, recorded rulings, judge answers and main histories. Running today's graders on their kept work directories reproduced every saved fact exactly. The reported three passes and one failure hold, including the no-skill lead's correct ruling and branch fix followed by its request to start an independent reviewer. The report discloses that the saved fixtures used the installed binary before the setup fix. I did not run additional models. GitHub reported no checks on the current branch when inspected, so I could not confirm CI's result.

`evals/agent-factory/test.sh` passed all thirteen suites, including the two new binary-selection regression tests and both scenarios' passing and failing grader walks. `git diff --check master...86eb55dad` passed. I found no new issue requiring a change.

## Finding 1: Select the copied test binary before constructing either fixture
Recommendation: fix now
Was: 1-ogm7.1
Response: fixed in d2800a2b0
Now: fixed

The shared setup now exports `$work/bin` first on PATH after loop's setup copies EVAL_ATELIER, before its first Atelier command. Both cases add a regression test that runs setup with directories containing an installed atelier excluded from the caller's PATH. This corrects binary selection at its cause and covers both callers of the shared setup.
