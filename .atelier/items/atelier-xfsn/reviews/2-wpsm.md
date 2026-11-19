---
schema: atelier.review
schema_version: 3
round: 2
commit: d402f8d50b8fce5799385a52eca6df6e052d5403
reviewers:
- thread:delegated-task:command%3Amcp%3Afc8b250b-03b1-4ffb-b409-646a901b77dd%3Adelegate-task%3A0767bce0-67ea-482c-a22f-5605f83c9d4a
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T01:40:23.615093490Z
---

## Summary

Changes requested: two unrelated actions can still share one recorded
exception, making an unrecorded departure pass. Findings 1 and 3 are fixed;
finding 2 remains open, and finding 4 covers the remote association.

- Complexity: high. The shared departure detector replays commit history
  and attributes exceptions across commits and remotes, forming a grading
  contract for seven cases.
- Value: high. The next evaluation matrix needs the retained cases to
  distinguish outcomes from wording and recorded departures from silent ones.
- Risk of merging: high. A false pass silently changes the evaluation's
  conclusions, even though reverting the code is easy.

The value warrants the change, provided the attribution defects below are
fixed. I found no smaller change that delivers the required grading contract.

I read the whole change against the Done when lines, including each grader
and rubric, the scenario renames and deletion, the limited rebuttal-case
updates, the shared facts, call logger, delegation log and saved-run lookup.
I built this checkout's binary with `cargo build -p atelier-cli -j 4`, then
ran all nine suites with `TMPDIR=$HOME/.cache/atelier-tmp` and
`~/.cargo/bin` removed from PATH, explicitly verifying that no installed
`atelier` was available. All 161 tests passed. `git diff --check` passed.
PR #226's `evals` job also passed. I ran two additional fixture reproductions
against this binary, described below. No model evaluation or live judge
call was run; rubric answers in the fixture reproductions were recorded
answers, as in the checked-in tests.

## Finding 1: Put the fixture binary on PATH for every Git command after installing hooks
Recommendation: fix now
Was: 1-l2fa.1
Response: fixed in fc7f27dcf
Now: fixed

`Fixture.sh()` and `sh_out()` now use the fixture's environment, which puts
its logged binary first on PATH and clears inherited identities and skips.
The full test run with no installed `atelier` on PATH passes, including the
hook-installed status fixture. CI's `evals` job passes too.

## Finding 2: Associate a recorded exception with the action it actually covers
Recommendation: fix now
Was: 1-l2fa.2
Response: fixed in fc7f27dcf
Now: kept

The read-only `status` reproduction from round 1 is fixed, but the
pre-commit association still lets an unrelated action hide a departure.
`evals/agent-factory/harness/facts.py:427-441` assigns an unused skipped hook
to a commit by time. Despite the comment saying it covers the first commit,
the hook is consumed only when `covered()` is called for a violation:
lines 475-479 omit code-only commits and short-circuit valid record commits.
A skipped hook for an ordinary commit on an item branch therefore remains
available to cover a later direct commit on main, even by another thread.

I reproduced this with `loop.GradeTest`: call `setUp()` and `done()`, switch
to `slug-k3fa`, run `atelier setup`, add and commit `notes.txt` as thread
`other` with `ATELIER_SKIP="only this ordinary commit"`, switch to `main`,
append a harmless comment to `slug.py`, and immediately commit it with
`--no-verify` and no skip. `graded()` returns `pass: true`, `failed: []`,
and `departures_unrecorded: {"value": [], "pass": true}`. The sole recorded
reason belongs to the earlier item-branch commit. Nothing records a reason
for the direct main commit.

Tie a hook exception to the commit that actually ran that hook, including
ordinary commits that require no exception. Another thread's later
violation must not inherit it. Add this regression alongside the existing
two-invalid-status-commits test, which does not exercise a successful
ordinary commit consuming its own hook.

## Finding 3: Resolve saved scenario names before rejudging kept runs
Recommendation: fix now
Was: 1-l2fa.3
Response: fixed in 2ffc32071
Now: fixed

`judge.rejudge()` now calls `run.case_dir()`, whose map resolves all seven
renamed cases, preserves `next-rebuttal-kept`, and explicitly reports the
deleted latency case. The harness tests exercise rejudging a result saved
under its old name and check that each mapped case still has a grader.
They pass in the full run.

## Finding 4: Restrict a push exception to the remote that received it
Recommendation: fix now

`evals/agent-factory/harness/facts.py:445-457` checks a skipped pre-push
hook's destination ref and commit range but never its remote or repository.
Consequently, a recorded exception to push a commit to a different remote's
`main` also excuses pushing it around the hook to origin's `main`. The log
already records the remote name and URL in the hook's argv, but this code
ignores them.

I reproduced a full false pass with `refused-push.GradeTest`: after
`setUp()`, clone `work/origin.git` as a second bare `work/backup.git`, add
remote `backup` pointing to it, run `git push backup main` with
`ATELIER_SKIP="only send this to the backup"`, and then run
`git push --no-verify origin main` with no skip. `graded()` returns
`pass: true`, `failed: []`, and an empty `departures_unrecorded`. The only
recorded departure is `check --hook pre-push backup ../backup.git`; there
is no recorded reason for the bypassed origin push.

Match the logged hook's repository and remote destination to the origin
being graded before accepting its pushed range, and add a two-remote
regression. Recording why a commit was sent elsewhere must not excuse
sending it to origin without a reason.
