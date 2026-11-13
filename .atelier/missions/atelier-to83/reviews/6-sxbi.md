---
schema: atelier.review
schema_version: 3
round: 6
commit: 9be44009a6faba3f4caa8b36a22d15d1f92bce90
reviewers:
- 01a1d3b1-8020-7681-9485-c90e27c4fbfc
model: grok-4.6
verdict: changes
created_at: 2026-11-13T06:56:41.792363086Z
---

## Summary

Checked the plan at 9be4400: `mission.md`, `design.md`, and all eight items; the round-5 amendment against each carried finding; current `docs/product.md` (mission lifecycle at 105-120, drop on the base at 150-153) and `docs/design.md` (act table drop row at 347, winner rule at 658-670); and `crates/atelier-app/src/plan.rs` `in_plan`, which still treats every path under the mission directory other than `reviews/` as the plan. Did not run two-clone start or drop races, time a record-only CI path, or read another round-6 review.

- Complexity: high. Seven commands leave the base, `merge` and the hook change, the mission format changes, and the reader plus the Desk grow a multi-ref view that five repositories then share.
- Value: high. Direct pushes are why the tool cannot sit on a protected base, why drafts live on `master`, and why the Desk is stale.
- Risk of merging it: high. A wrong `merge` or hook stops work in five repositories at once. The round-5 answers refuse `merge` of a dropped plan, refuse `submit` and `merge` on a `review/` branch, and close a round when its reviews are gathered. Two definition sentences still call a dropped unmerged mission proposed, and a planner `submit` after one verdict still puts that verdict on the mission branch where the other reviewer can `show` it.

Could not check: how GitHub records `done` on a merge commit it creates (left to atelier-ogpg); whether a record-only pull request is under two minutes in the four product repositories; whether listing 200 named remote branches stays under two seconds without a cache.

## Finding 1: a dropped proposed plan still matches the definition of proposed
Recommendation: fix now
Was: 5-xtac.4
Now: kept

Outcome:58-61, atelier-2vdq:32, atelier-dciw:32, atelier-ogpg:29 and `design.md` Decisions:51 now say proposed is status `open` on a branch that has not merged. A dropped plan is dropped, not proposed. `submit` and `merge` of a mission's branch refuse it, with a test that a dropped plan holding two approvals does not merge, with a forge and without.

The two sentences the finding named for `show` and `list` are unchanged. atelier-2vdq:37 still says a mission read from a branch that has not merged is shown as proposed, with no exception for `dropped`. atelier-dciw:33 still lists proposed as missions and record changes on branches not yet merged. A dropped plan stays on that branch (atelier-2vdq:39). An implementer of those two bullets still presents it as proposed.

Align atelier-2vdq:37 and atelier-dciw:33 with Decisions:51 and with 2vdq:32: proposed is an open mission that has not merged.

## Finding 2: merge of a review branch is the records-only path
Recommendation: fix now
Was: 5-xtac.5
Now: fixed

`design.md` Decisions:53 and atelier-2vdq:34 refuse `submit` and `merge` on a `review/` branch, with a test that `merge` on one is refused. atelier-ogpg:29 still says that branch never merges on its own. The records-only path in atelier-ob9u:35 stays path-based; 2vdq takes the `review/` ref before that path can land it.

## Finding 3: Gathering one verdict of a round unblinds the other reviewer
Recommendation: fix now
Was: 5-xtac.6
Now: kept

`design.md` Decisions:53 and atelier-2vdq:34 now gather a round only on `merge` and on the `submit` that opens the next round, and that gather closes the round. atelier-dciw:31 still prints a count and no verdict for a round that has not been brought onto the mission's branch.

Closing the round is how a partial gather becomes visible. atelier-2vdq:34's test is: after one verdict and the planner's `submit`, the second reviewer's `verdict` for the old round is refused. That `submit` is not required to change the plan, though 2vdq:33 opens the next round only after changes. atelier-ogpg:29 still has the planner's `submit` or `merge` bring reviews onto the mission's branch, and says gathering closes the round.

After that `submit` the one verdict sits on the mission's branch. atelier-dciw:31 then gives it to `show` in the other reviewer's clone. Refusing their `verdict` stops them finishing the round; it does not keep the file from them. The test that one published verdict still prints a count and no verdict after a `submit` that collected it is not in dciw:31.

Gather a round onto the mission's branch only when it has two verdicts, or have `show` omit that round's files while this checkout is still reviewing it. Keep the hide-after-collect test. If a `submit` after changes must close a partial round, say that in the test, and keep a no-change `submit` from gathering.
