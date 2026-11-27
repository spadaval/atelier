---
schema: atelier.review
schema_version: 3
round: 2
commit: be0a9a2ffb526e86ff18e7e7319b7dc27e6cc644
reviewers:
- thread:delegated-task:command%3Amcp%3A68acf51e-761f-40a4-a250-1f1bc25078c8%3Adelegate-task%3Ac0ab7cf5-c3ad-4920-9afc-a5b3fb5f658a
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-27T08:49:04.311699579Z
---

## Summary

Complexity: medium. The whole change adds review-history aggregation and
reorders several status sections; this round changes only the output
ordering and the backlog's needs-human filter.

Value: high. Pending work must remain visible within the harness's output
limit, and the review pace makes a mission's review cost visible.

Risk of merging: low. The output is read-only and the changes are easy to
revert; the regression tests cover the ordering that previously hid work.

The value is worth the size of the change. I reviewed the diff since
2194f67e5, checked both earlier findings against the code and regression
test, and checked the Done when lines and Report against the current
implementation. Drop notices now follow the lead's handoff, its missions
and the backlog. The needs-human filter removes only items already shown
under led missions, and leaves the heading out when none remain.

`cargo nextest run -p atelier-cli -p atelier-core` passes all 405 tests
(one skipped), including the twelve-item and three-mission size limits,
the review pace, the commands for omitted details and the new forty-drop
regression. Formatting, Clippy with warnings denied and whitespace checks
pass. `evals/agent-factory/test.sh` also passes: all eval fixtures and
harness tests pass. I found no new issue in this round's changes.

I did not repeat the previous round's independent hand count of the live
Hub records; the pace code and those records are unchanged in this round.
At the reviewed head, the hosted checks had no failures and two jobs were
still pending; I did not wait for CI to finish.

## Finding 1: Critical: Unrelated completed drops still precede the lead's pending work
Recommendation: fix now
Was: 1-1tto.1
Response: fixed in ad31d97d2
Now: fixed

`output::overview` now appends both dropped-on-base and unreached-drop
notices after the backlog, so unrelated notices cannot precede a lead's
pending work. The new regression builds the reported forty-drop case and
checks that the handoff is within the first 2 KB and before both its
mission and the notices. It passes. The earlier deferred needs-human
duplication is also fixed and covered by the same test.
