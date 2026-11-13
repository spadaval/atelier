---
schema: atelier.review
schema_version: 3
round: 6
commit: 9be44009a6faba3f4caa8b36a22d15d1f92bce90
reviewers:
- thread:delegated-task:command%3Amcp%3Adbe6537b-1cdf-48e8-acc9-b67a0a17e994%3Adelegate-task%3A87860255-adb4-4bea-87a3-678a879b8cca
model: gpt-6-astra
verdict: approve
created_at: 2026-11-13T06:56:48.684267654Z
---

## Summary

Reviewed the plan at 9be44009a6faba3f4caa8b36a22d15d1f92bce90: the mission,
its design document and all eight items. Checked the three carried findings
against their revised acceptance criteria, traced the item dependencies and
mission criteria, and compared the proposed read/write and merge model with
the current corresponding sections of `docs/design.md`.

- Complexity: high. This changes record writes, branch discovery, mission
  review, merge completion and the Desk across five repositories.
- Value: high. Protected-base support and visibility of published work both
  require changing the current model of writing directly to the base.
- Risk of merging it: high. An incorrect merge gate or record precedence rule
  could silently accept stale approval or miss a stop across clones.

The value warrants the scope. The documentation-first item, explicit tests
with two clones, compatibility reads and staged rollout make the work
reviewable in pieces. I found no additional fix-now problem in the plan.
The three carried findings are fixed by explicit rules and acceptance tests.

This was a plan review, not implementation validation. I did not run the
protected-remote script, concurrency tests, live Desk comparisons or CI
and reader timing measurements; those remain deliverables of the items.
The exact record precedence and forge completion design are assigned to
atelier-ogpg before the implementation items begin.

## Finding 1: a dropped proposed plan still matches the definition of proposed
Recommendation: fix now
Was: 5-xtac.4
Now: fixed

The mission's Outcome and design now define proposed as status `open` on an
unmerged branch. atelier-dciw applies that definition to every view and
excludes dropped plans. atelier-2vdq explicitly refuses both `submit` and
`merge` for a dropped mission and requires the two-approval dropped-plan
merge test with and without a forge. The more general lifecycle sentence
in that item does not override these explicit conditions.

## Finding 2: merge of a review branch is the records-only path
Recommendation: fix now
Was: 5-xtac.5
Now: fixed

atelier-2vdq explicitly refuses `submit` and `merge` on a `review/` branch,
with a merge-refusal test. The design repeats that rule and says such a
branch has no pull request. These exclusions close the records-only path
identified by the earlier finding.

## Finding 3: Gathering one verdict of a round unblinds the other reviewer
Recommendation: fix now
Was: 5-xtac.6
Now: fixed

Gathering reviews now closes their round. atelier-2vdq and the design both
require a late verdict for that round to be refused, and the item tests the
case where one verdict is published, the planner submits, and the second
reviewer then tries to publish. A collected verdict can become visible,
but cannot influence another accepted verdict in that same round. This
implements the earlier finding's alternative of closing the round.
