---
schema: atelier.review
schema_version: 3
round: 5
commit: 16e81ecae54f9b410eaf0099b44a82b379fab04c
reviewers:
- 01a1d3ab-09f4-7023-a5fd-d32fed1c5c0f
model: grok-4.6
verdict: changes
created_at: 2026-11-13T06:49:37.919186982Z
---

## Summary

Checked the plan at 16e81ec: `mission.md`, `design.md`, and all eight items; the round-4 amendment against each carried finding; current `docs/product.md` (mission lifecycle at 105-120, drop on the base at 150-153) and `docs/design.md` (act table drop row at 347, winner rule at 658-670); and `crates/atelier-app/src/plan.rs` `in_plan`, which still treats every path under the mission directory other than `reviews/` as the plan. Did not run two-clone start or drop races, time a record-only CI path, or read another round-5 review.

- Complexity: high. Seven commands leave the base, `merge` and the hook change, the mission format changes, and the reader plus the Desk grow a multi-ref view that five repositories then share.
- Value: high. Direct pushes are why the tool cannot sit on a protected base, why drafts live on `master`, and why the Desk is stale.
- Risk of merging it: high. A wrong `merge` or hook stops work in five repositories at once. The round-4 answers give started-item drop an act, put the prefix on every kind of branch, hide verdicts until a round is gathered, and test that a dropped plan is not listed as proposed. Outcome and the mission merge act still treat that dropped plan as proposed, and a `review/` branch still satisfies the records-only merge rule.

Could not check: how GitHub records `done` on a merge commit it creates (left to atelier-ogpg); whether a record-only pull request is under two minutes in the four product repositories; whether listing 200 named remote branches stays under two seconds without a cache.

## Finding 1: A started item's drop never lands on the base, and has no act
Recommendation: fix now
Was: 4-to95.1
Now: fixed

`design.md` Decisions:56 and atelier-ob9u:38 now give the act: any thread, from any checkout, on an item that is started and not merged; `drop` writes the item's branch through its ref, refuses while a checkout of that branch has uncommitted changes to the item's directory, then lands the base copy on a `backlog/` branch by the record-only path. The start-after-fetch test is there: after the item's branch is deleted, a second clone that has fetched still sees the item dropped on the base and refuses `start`.

## Finding 2: atelier-ogpg still documents three kinds of branch
Recommendation: fix now
Was: 4-to95.2
Now: fixed

atelier-ogpg:29 now asks `docs/product.md` for what an item's, a mission's and a `backlog/` branch each need before they merge, and that a `review/` branch never merges on its own but is brought onto the mission's branch by the planner's `submit` or `merge`. The prefix applies to all four kinds. atelier-ob9u:30 names the four prefixed forms, `x/<item-id>`, `x/<mission-id>`, `x/backlog/<slug>` and `x/review/<mission-id>/<round>-<suffix>`, read back by one function. `design.md` Decisions:53 says a `review/` branch never merges on its own.

## Finding 3: show of a proposed plan reads the other reviewer's verdict
Recommendation: fix now
Was: 4-to95.4
Now: fixed

`design.md` Decisions:53 has `show` and the Desk count a round's verdicts and print neither a verdict nor a finding until the round has been brought onto the mission's branch. atelier-dciw:31 is the same rule, with a test that a published verdict in a second clone prints the count and no verdict. atelier-v958:32 shows the count on the Desk and the verdicts once the round is on the mission's branch.

## Finding 4: a dropped proposed plan still matches the definition of proposed
Recommendation: fix now
Was: 4-to95.5
Now: kept

atelier-dciw:32 and atelier-v958:32 now test that a mission dropped on its branch is not listed as proposed, and `design.md` Decisions:55 says the reader gives it as dropped. The definition of proposed was not aligned with that test.

Outcome:58-59 still says a mission is proposed while its plan is on its branch, with no test of stored status. atelier-2vdq:37 still says a mission read from a branch that has not merged is shown as proposed, with no exception for `dropped`. atelier-dciw:33 still defines proposed as missions and record changes on branches not yet merged. atelier-ogpg:29 still asks `docs/product.md` to describe a mission as proposed on its branch and agreed once merged.

The merge act is `from proposed` (atelier-2vdq:32). Drop of a proposed plan commits `dropped` on that branch, pushes, and closes the pull request (atelier-2vdq:39). The branch is still there, and the plan sections the two approvals cover are unchanged. Following 2vdq:37 that mission is still proposed, so `merge` of it still runs. Closing the pull request stops the forge; with no forge, `merge` moves the base locally (atelier-t5am:31, `protected-base`). Align Outcome:58-59, atelier-2vdq:37, atelier-dciw:33 and atelier-ogpg:29 with Decisions:51: proposed is an open mission that has not merged, and `merge` of a dropped plan is refused.

## Finding 5: merge of a review branch is the records-only path
Recommendation: fix now

`design.md` Decisions:53 and atelier-ogpg:29 say a `review/` branch never merges on its own. The merge items do not refuse it.

atelier-ob9u:35 defines records-only by paths: `merge` merges a branch once `atelier check` passes, with no review file, when every path it changes against the base is under `.atelier/`. A `review/<mission-id>/<round>-<suffix>` branch holds one review file under `.atelier/missions/`. That diff does not add a mission and does not change the plan: reviews are excluded (atelier-2vdq:35). atelier-2vdq therefore does not take the branch, and atelier-ob9u:36's act rows are for `backlog/` only.

`submit` on that checkout opens a pull request (atelier-ob9u:35), and `merge` lands the review file on the base. Give `merge` (and `submit`) a refuse for a `review/` ref, with a test, in atelier-ob9u or atelier-2vdq.

## Finding 6: Gathering one verdict of a round unblinds the other reviewer
Recommendation: fix now

atelier-dciw:31 says a reviewer who runs `show` learns nothing of another's verdict in the round it is reviewing, because verdicts print only once the round is on the mission's branch. `design.md` Decisions:53 and atelier-2vdq:34 have the planner's next `submit` or `merge` bring every published review of the round onto that branch.

`merge` needs two approvals (atelier-2vdq:35), so it gathers both after both reviewers have finished. `submit` does not wait: it brings whatever is already published (atelier-2vdq:33-34). After one verdict the planner's `submit` copies that review onto the mission's branch. `show` in the other reviewer's clone then prints it, which is the round they are still reviewing.

Either `submit` and `merge` copy a round only when it has two verdicts or the round is closed, or `show` omits same-round review files while this checkout is reviewing that round, even after they sit on the mission's branch. Keep the test that one published verdict prints a count and no verdict after a `submit` that collected it.
