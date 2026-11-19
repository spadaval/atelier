---
schema: atelier.review
schema_version: 3
round: 1
commit: 643588fcb709c15ba765e7e45cd30f5d24410e0f
reviewers:
- thread:delegated-task:command%3Amcp%3Aef5c4a2f-47c0-4601-9465-2ae13c1b1a3c%3Adelegate-task%3Ac50f545a-20e1-4219-8fa3-8862575aa31e
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T02:57:37.691093648Z
---

## Summary

Complexity: medium. Two related scenarios share fixture construction and fact extraction, with separate rubrics and grader walks.
Value: high. Testing a lead against both a false finding and a real defect is needed to measure whether its rulings preserve the mission's outcome.
Risk of merging: low. The changes are confined to evals and their documentation, and are straightforward to revert.
The value justifies the size; the shared setup avoids duplicating the main moving parts. Fix the setup's binary selection before merging.

I read the complete change against master at 643588fcb, including both fixtures, prompts, rubrics, graders, tests, harness changes and README. I checked the Done when lines and Report against the four saved gpt-6.1-sol/Codex runs, their recorded rulings, final replies and repository histories. Running today's graders on all four kept work directories reproduced their saved facts exactly. The reported three passes and one failure, including the no-skill lead stopping before independent review, match the evidence. I did not run additional models.

`evals/agent-factory/test.sh` passed all thirteen suites with this host's normal PATH, including the new nine- and ten-test suites. `git diff --check master...643588fcb` passed. I also ran both new setup scripts with EVAL_ATELIER pointing to this checkout's binary and PATH restricted to `/usr/bin:/bin`; both failed as described below. CI's evals job was still pending when inspected.

## Finding 1: Select the copied test binary before constructing either fixture
Recommendation: fix now

`evals/agent-factory/scenarios/kept-finding-wrong/setup.sh:18-22` calls loop's setup in a child shell and sources `loop/lib.sh`, but never prepends `$work/bin` to PATH. The library's `as` function invokes bare `atelier`, so the first `as $planner set` and all subsequent setup commands use the host's installed CLI instead of the EVAL_ATELIER binary copied into `$work/bin/atelier`. The removed setup explicitly exported that PATH.

I reproduced this for both `kept-finding-wrong/setup.sh` and `kept-finding-right/setup.sh` with PATH set to `/usr/bin:/bin`, an absolute EVAL_ATELIER pointing at this checkout's executable, and T3CODE_THREAD_ID set. Both exit 127 with `loop/lib.sh: line 28: atelier: command not found`, despite the copied binary existing. The right case delegates to the same setup, so both are affected. CI builds target/debug/atelier and calls test.sh without installing atelier, so neither new scenario can construct its fixture there. A host with an installed CLI masks the failure and can construct the fixture using a different version from the binary under test.

Export `$work/bin` first on PATH before the first Atelier call. Protect this with a setup check that supplies EVAL_ATELIER while excluding any installed atelier from the caller's PATH, so the local installation cannot hide this dependency again.
