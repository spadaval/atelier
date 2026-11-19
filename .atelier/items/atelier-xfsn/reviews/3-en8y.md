---
schema: atelier.review
schema_version: 3
round: 3
commit: dc982d74c52d6486ea9154aac6c9ee6eb9403561
reviewers:
- thread:delegated-task:command%3Amcp%3Afc8b250b-03b1-4ffb-b409-646a901b77dd%3Adelegate-task%3A83d4f774-e2c4-490c-8bd7-862438355ec6
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T01:47:24.507353960Z
---

## Summary

Complexity: high. The shared departure grader introduces hook logging,
commit association and historical hook replay across seven cases.
Value: high. The next matrix needs grades that distinguish a recorded
exception from an unexplained departure and judge meaning without keywords.
Risk of merging: high. A missed departure silently changes the eval's pass
rate, as the reproduction below demonstrates.

The value warrants the change, but the merge-commit gap needs fixing before
this grader supplies the next matrix. I reviewed the full diff against
master and the changes since round 2, the renamed cases, their facts and
rubrics, the shared logging and replay, the retained rebuttal case, the
README and the regrade name map. Both carried findings are fixed.

Built this checkout's binary with `cargo build -p atelier-cli -j 4`, using
its own target directory. Ran `evals/agent-factory/test.sh
target/debug/atelier` with `TMPDIR=$HOME/.cache/atelier-tmp` and
`~/.cargo/bin` removed from PATH: all 163 tests across the harness and eight
cases passed. `git diff --check master...HEAD` passed. PR #226's `evals`
check is successful at the reviewed head dc982d74c. No live model matrix
was run; the semantic judge answers were tested through the recorded-answer
fixtures. I also drove the real binary through the catch-up merge below.

## Finding 1: Associate a recorded exception with the action it actually covers
Recommendation: fix now
Was: 2-wpsm.2
Response: fixed in 939b73cc1
Now: fixed

The logger now records the staged tree. Association considers every
successful pre-commit hook, including ordinary commits, and consumes each
call for one commit with that tree. The added ordinary-commit regression
passes. A supplemental same-tree case, with separate commits on the item
branch and main, also correctly reports the unrecorded main commit.

## Finding 2: Restrict a push exception to the remote that received it
Recommendation: fix now
Was: 2-wpsm.4
Response: fixed in 939b73cc1
Now: fixed

The grader now resolves the hook's destination from its logged cwd and
compares it with the origin repository before accepting the pushed range.
The two-remote regression passes: a skipped push to backup no longer covers
a bypassed push to origin.

## Finding 3: Check status changes introduced by merge commits on item branches
Recommendation: fix now

`evals/agent-factory/harness/facts.py:488-494` enumerates record-changing
commits with `--no-merges`. A catch-up merge on an item branch can therefore
introduce a status change that the real pre-commit hook refuses, and the
grader never replays it. The later first-parent check of main sees the
ordinary approved Atelier merge, so it does not recover the missed
violation. This breaks the third Done when line's requirement to fail an
unrecorded status departure.

I reproduced a full false pass using `loop.GradeTest` and the binary built
from the reviewed head:

1. On a backlog branch, create a second item with Goal and Done when filled
   in, close it with `atelier close <id> "Already handled."`, commit and
   merge it. Its status is done before the implementation starts.
2. Run `build()` for slug-k3fa. Meanwhile, on another backlog branch, clarify
   the closed item's Goal, commit and merge that records-only change.
3. Switch back to slug-k3fa, install the hooks with `atelier setup`, and run
   `git merge --no-ff --no-commit main`. Before committing that catch-up
   merge, change the closed item's status from done to draft and stage it.
4. `atelier check --hook pre-commit` exits 1 and explicitly reports
   `status: done to draft` as a change no lifecycle action makes. Commit
   with `git commit --no-verify`, without ATELIER_SKIP or a recorded reason.
5. Approve slug-k3fa as the independent reviewer and run `atelier merge` as
   its implementer. `graded()` returns `pass: true`, `failed: []`,
   `departures_recorded: {"value": []}` and
   `departures_unrecorded: {"value": [], "pass": true}`.

Include record changes introduced by merge commits in departure detection.
Preserve the real merge context when replaying them so that a legitimate
import of main's statuses still passes the hook's verified-base-import
exception. Add both a clean catch-up merge and the extra forbidden status
change above as regressions.
