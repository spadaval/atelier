---
schema: atelier.review
schema_version: 3
round: 2
commit: 8fbc727f1fc0ddd448b7063227b7b8c1729039d1
reviewers:
- thread:delegated-task:command%3Amcp%3A9da2310f-484c-4ef4-9547-00086bbf3b23%3Adelegate-task%3A9223dff5-25b9-4bb0-b864-5f86c709f2ee
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-14T00:50:14.371060847Z
---


## Summary

Complexity: high. The change adds the plan approval contract across mission
records, branch submission and merge, reviewer identity, and published stops.
Value: high. Two independent plan approvals replace the human go-ahead and
make the mission branch usable as the source of the agreed plan.
Risk of merging: high. A missed gate could silently agree an unreviewed plan
or let work proceed after a published stop.
The value warrants this scope; a smaller change would leave part of the
mission lifecycle under the old contract.

Reviewed the full diff from master to 8fbc727f1 and the changes since round 1,
with particular attention to every Done when line and the three carried
findings. Read the plan comparison and approval counting, records and item
merge paths, mission submission and take-over, review landing commands,
legacy status handling, and stop publication and views.

Ran the planning and branches integration suites, act-table and record
suites, CLI views/hooks/log suites, app library/views suites, and all Rust
Desk tests: 196 tests passed; one host-dependent performance test was skipped.
`cargo fmt -- --check` and `git diff --check master...HEAD` passed. Four
additional scratch-repository probes confirmed that removing, reassigning,
or adding an initial item invalidates both plan approvals, and that editing
only a mission review file preserves them and allows the plan to merge.

The three previous findings are fixed. No new problem found. I did not run
the Desk browser suite, exercise the real GitHub forge, or repeat the author's
checks against the five live repositories; compatibility was checked with
the committed old-format fixtures and the cross-clone behavior with the
integration suites' local forge.

## Finding 1: Apply the plan approval gate to item branches too
Recommendation: fix now
Was: 1-lxxo.1
Response: fixed in 6ba389903
Now: fixed

The item merge path now calls `plans_changed` and refuses a mission addition
or plan amendment, directing it through the plan review path. This runs for
both ordinary and trivial item merges. The new item-branch regression test
passed and confirms that one approving item review cannot admit the changed
mission Outcome.

## Finding 2: Require plan reviews when an old base draft becomes agreed
Recommendation: fix now
Was: 1-lxxo.2
Response: fixed in 6ba389903
Now: fixed

`plans_changed` now includes the first agreement of an old base-resident
draft even when the plan sections are unchanged. The regression test passed:
submit writes it open, zero and one approvals each refuse merge, and two
independent approvals allow it.

## Finding 3: Show a fetched published stop as published
Recommendation: fix now
Was: 1-lxxo.3
Response: fixed in 6ba389903
Now: fixed

`show` now applies `published_stops` to the mission it displays, and
`unpublished_stop` consults the fetched forge base. The extended cross-clone
regression test passed with the local base still behind: show reports
dropped without the unpublished warning, list removes its ready item, and
start and merge honor the stop.
