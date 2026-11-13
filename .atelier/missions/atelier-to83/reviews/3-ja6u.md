---
schema: atelier.review
schema_version: 3
round: 3
commit: ddb78efe1c459bc6096a449919c43b2a4983ae40
reviewers:
- 01a1d39d-ed90-7272-9e56-6c2a9595c1d7
model: grok-4.6
verdict: changes
created_at: 2026-11-13T06:35:20.113804463Z
---

## Summary

Checked the plan at ddb78ef: `mission.md`, `design.md`, and all eight items; the round-2 amendment against each carried finding; current `docs/product.md` (Plan at line 69, `mission.md` sections at 294) and `docs/design.md` (winner rule at 658-670, start at 697-707); and `crates/atelier-app/src/plan.rs` `in_plan`, which already treats item directories as part of the plan. Did not run two-clone start or drop races, time a record-only CI path, or read another round-3 review.

- Complexity: high. Seven commands leave the base, `merge` and the hook change, the mission format changes, and the reader plus the Desk grow a multi-ref view that five repositories then share.
- Value: high. Direct pushes are why the tool cannot sit on a protected base, why drafts live on `master`, and why the Desk is stale.
- Risk of merging it: high. A wrong `merge` or hook stops work in five repositories at once. The round-2 answers close the carried holes. The remaining ones still let a drop fail to land and let the first item document the wrong branch set.

Could not check: how GitHub records `done` on a merge commit it creates (left to atelier-ogpg); whether a record-only pull request is under two minutes in the four product repositories; whether listing 200 named remote branches stays under two seconds without a cache.

## Finding 1: Items on a proposed plan sit outside the two-review gate
Recommendation: fix now
Was: 2-knm6.10
Now: fixed
Outcome:52-55, `design.md` Shape:32-34, and atelier-2vdq:35 put a proposed mission's items inside the plan and make a change, add, remove or reassignment stale the approvals. atelier-2vdq:36 requires a test that an initial item's Done when changed after approval is refused, beside the open-mission add-item path.

## Finding 2: Result cannot be told from the gated sections of mission.md
Recommendation: fix now
Was: 2-knm6.11
Now: fixed
The comparison is by section of `mission.md` (`design.md` Shape:36, atelier-2vdq:35). Result and front matter are records. atelier-2vdq:36 requires a Result-only change on an open mission to merge without a plan review, and a Why change on that mission to be refused.

## Finding 3: Plan-review refs are unnamed and the reader never looks at them
Recommendation: fix now
Was: 2-knm6.12
Now: fixed
The name is `review/<mission-id>/<round>-<suffix>` (Outcome:49, `design.md` Decisions:49 and :53). `verdict` writes only that branch and never the mission's. atelier-dciw:31 has the reader find those branches so `show` can count verdicts before they are copied onto the mission branch. Finding 9 is the leftover: atelier-ogpg still describes three kinds of branch.

## Finding 4: Stop is not an act that publishes, and the Desk is not required to finish it
Recommendation: fix now
Was: 2-knm6.13
Now: fixed
`design.md` Decisions:55 and atelier-2vdq:39 give `drop` of a mission act-table rows (planner, or the Desk as its configured caller), push and close a proposed plan, and have an open-mission drop submit and merge itself. atelier-v958:35-36 has the Desk drive that stop to its end and show pending until the record-only branch has merged. `stop-is-effective` tests the merged result from a second clone.

## Finding 5: The losing start claimant keeps a local branch that still looks started
Recommendation: fix now
Was: 2-knm6.14
Now: fixed
`design.md` Decisions:54 and atelier-ob9u:37 have `start` delete the local branch it made, treat a later `start` in that clone as a take-over, and test that `atelier list` in the losing clone after a fetch shows the winner.

## Finding 6: Dropping a started item is no longer effective until some other branch merges
Recommendation: fix now
Was: 2-knm6.15
Now: fixed
The drop now commits on the item's own branch, so it is not a second pull request racing the item's merge (`design.md` Decisions:56, atelier-ob9u:38). `submit` and `merge` of that item are refused from then, and tests run both orders from two clones. Finding 8 is the leftover: the base copy and the act are still unnamed.

## Finding 7: Keep the proposed mission's items inside approval invalidation
Recommendation: fix now
Was: 2-noyj.10
Now: fixed
Same contract as Finding 1: proposed items are part of the plan, a change after approval is refused, and adding an item is record-only only once the mission is on the base.

## Finding 8: A started item's drop never lands on the base, and has no act
Recommendation: fix now
`design.md` Decisions:56 and atelier-ob9u:38 have `drop` of a started item commit on that item's branch and push, then refuse the item's `submit` and `merge`. "The base's copy follows by the record-only path." Nothing names who writes that copy, on which branch, or that `drop` itself submits and merges it.

An open-mission drop does name that: it commits on `backlog/`, and itself submits and merges, with act-table rows (atelier-2vdq:39, `design.md` Decisions:55). A started item's drop has no such row: who may run it, and from which checkout, including the other host in `stop-is-effective`.

Today a drop is effective because it writes the base and every later command reads that record first (`docs/design.md:631-637`, `:666-670`). After this change the item branch cannot merge, so the dropped record stays off the base until the unnamed follow-up. The base still holds `ready`. If the remote branch is deleted, or a clone never fetches it, `start` sees a ready item again. Say that `drop` of a started item itself lands the base copy, the way an open-mission drop does, give it act-table rows, and test that after that path a second clone that has fetched shows the item dropped on the base and refuses `start`.

## Finding 9: atelier-ogpg still documents three kinds of branch
Recommendation: fix now
Outcome:47-49 and `design.md` Decisions:49 now name four refs: `<item-id>`, `<mission-id>`, `backlog/<slug>`, and `review/<mission-id>/<round>-<suffix>`. atelier-dciw:31 and atelier-2vdq:33 are written against the fourth.

atelier-ogpg:29 still asks `docs/product.md` to describe "the three kinds of branch and the optional prefix; what each kind of branch needs before it merges". Review branches are not a kind that merges to the base; they are a kind the reader must find. The first item is the one that copies this mission's `design.md` into the documents later items implement against. Update that Done when so product.md names the review refs and says they are published by `verdict` and read, not merged on their own. Say how the optional prefix applies to `review/…`, because atelier-ob9u:30 only gives the item example `x/<id>` and atelier-dciw:31 looks for the literal `review/<mission-id>/...`.

## Finding 10: Outcome still treats notes.md as both plan and records
Recommendation: fix now
Outcome:52-55 defines the plan as the five sections of `mission.md`, "the documents beside it", and, until merge, its items. The next sentence says "its notes" are records. `notes.md` is a document beside `mission.md`.

`design.md` Shape:30-35 and atelier-2vdq:35 exclude `notes.md` and the reviews from the plan. `reviews-are-the-gate` (mission.md:93-94) lets a notes-only branch merge on `atelier check`. The Outcome is the gated definition of the plan. Spell it the same way as Shape: the documents beside `mission.md` other than `notes.md` and the reviews.
