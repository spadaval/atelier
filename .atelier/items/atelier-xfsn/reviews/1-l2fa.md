---
schema: atelier.review
schema_version: 3
round: 1
commit: ebb79b2ec4a6e0ee291a42d71b5be495d37d237d
reviewers:
- thread:delegated-task:command%3Amcp%3Afc8b250b-03b1-4ffb-b409-646a901b77dd%3Adelegate-task%3A747eb743-5efc-436c-89df-7d5d958ba24d
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T01:30:30.374330111Z
---

## Summary

Complexity: high. The shared departure classifier adds a grading contract across every tracked case, including Git-history and hook replay.
Value: high. The mission needs comparable facts and rubric grades for the retained cases.
Risk of merging it: high. A false pass silently changes the evaluation's conclusions.
The value justifies the conversion, but the classifier and rename compatibility need the fixes below.

Reviewed ebb79b2ec4a6e0ee291a42d71b5be495d37d237d against master, including the Done when lines, scenario renames, facts, rubrics, harness changes and retained rebuttal case. Built this checkout's binary with `cargo build -p atelier-cli -j 4`, then ran `evals/agent-factory/test.sh target/debug/atelier`, with `TMPDIR=$HOME/.cache/atelier-tmp`: all 156 tests across the harness and eight cases passed locally. `git diff --check` passed. Ran additional reproductions against the real binary for findings 1 and 2, and called the saved-run rejudge path for finding 3. No model evaluation or live rubric judge was run.

The PR's evals job does not pass: run 44437349508, job 135269644943 failed at the new hook test. Its failure is reproducible independently of this host's installed binary, as below. The Report's claim of ten passing suites also miscounts the nine suites now present.

## Finding 1: Put the fixture binary on PATH for every Git command after installing hooks
Recommendation: fix now

`evals/agent-factory/scenarios/loop/test_grade.py:291-293` installs the Git hooks before `hand_status()` calls `build()`. That reaches `Fixture.sh()` at line 118 for `git switch` and `git commit`, but `sh()` inherits the host's PATH instead of putting `work/bin` first, as `atelier()` does. The hooks execute `atelier` by name. On CI there is no installed binary on PATH, so `git switch` fails in post-checkout before the test reaches the status change. On this host the test silently uses the installed binary for these hooks, outside the fixture's call log, which explains the local pass.

CI already fails at exactly this point: https://github.com/spadaval/atelier/actions/runs/44437349508/job/135269644943. I also reproduced the cause by setting up this fixture, running its `setup` and `start`, then running `git switch -q slug-k3fa` with PATH `/usr/bin:/bin`: exit 1, `.git/hooks/post-checkout: 2: exec: atelier: not found`. Make all fixture Git subprocesses use the fixture's logged binary and intended thread environment. Do not rely on a globally installed `atelier` to get the test green.

## Finding 2: Associate a recorded exception with the action it actually covers
Recommendation: fix now

`evals/agent-factory/harness/facts.py:417-420` considers any call with ATELIER_SKIP within two seconds of a commit sufficient to excuse that commit. It does not check the command, thread, result, rule or commit involved. This lets an unrelated read-only call erase an unrecorded departure, so the graded fact no longer establishes the third Done when line. The origin path at lines 477-482 has the same association problem over a larger interval: any skipped pre-push anywhere in the run excuses every origin commit it examines.

I reproduced a full false pass using `loop.GradeTest`: complete the normal approved merge with `done()`, switch to main, run `atelier status` with `ATELIER_SKIP="only checking status"`, append a comment to slug.py and immediately commit directly on main without any skip or reason. `graded()` returns `pass: true`, `failed: []`, and `departures_unrecorded: []`; the only recorded departure is the status call. Without that unrelated call, the direct main commit is correctly detected. This is more than the Report's acknowledged rebasing limitation: ordinary, unrelated calls can hide a different thread's departure. Record enough action-specific evidence to tie the reason to the particular commit, merge or pushed ref update, and add a regression where a logged skip and a separate unrecorded departure coexist.

## Finding 3: Resolve saved scenario names before rejudging kept runs
Recommendation: fix now

The scenario directory renames, documented in `evals/agent-factory/README.md:23-31`, leave the saved-run consumer unchanged. `harness/judge.py:363` still resolves `result["scenario"]` directly under scenarios. Every retained run saved under one of the renamed names therefore loses its grader, even though the README still promises that judge.py grades kept runs with today's graders and rubrics.

Calling `judging.rejudge` with an existing work/repo and a saved result containing `scenario: next-loop`, `setup: sol`, `pass: true` now returns `pass: null` and `can't open file .../scenarios/next-loop/grade.py: [Errno 2] No such file or directory`. The same lookup fails for all seven renamed cases. The README's name map cannot repair the machine lookup. Resolve the historical names to their current case directories when loading saved results and test that path, preserving the intentional deletion of the latency case and the retained rebuttal name.
