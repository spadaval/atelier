---
schema: atelier.review
schema_version: 3
round: 7
commit: 73dfb488da0b9470cc126aca85ca405441d524a3
reviewers:
- 01a1d3fe-c706-78d3-bb16-9c5d70b5d36f
model: grok-4.6
verdict: changes
created_at: 2026-11-13T08:21:06.317624245Z
---

## Summary

Checked the plan at 73dfb48: `mission.md`, `design.md`, and all nine items; the round-6 amendment against each carried finding; and the current `docs/product.md` mission lifecycle (105-120) and `docs/design.md` act table (328-355). Did not run two-clone start or drop races, time a record-only CI path, or read another round-7 review.

- Complexity: high. Seven commands leave the base, `merge` and the hook change, the mission format changes, reviewers publish with Git, escalate and decide go away, and the reader plus the Desk grow a multi-ref view that five repositories then share.
- Value: high. Direct pushes are why the tool cannot sit on a protected base, why drafts live on `master`, and why the Desk is stale.
- Risk of merging it: high. A wrong `merge` or hook stops work in five repositories at once. The round-6 answers use one wording of proposed, and a dropped plan is not listed as proposed. Reviewers now push onto the mission's branch, so the first verdict of a round is on the ref `show` prints, and `design.md` still binds planners with `new --mission`.

Could not check: how GitHub records `done` on a merge commit it creates (left to atelier-ogpg); whether a record-only pull request is under two minutes in the four product repositories; whether listing 200 named remote branches stays under two seconds without a cache.

## Finding 1: a dropped proposed plan still matches the definition of proposed
Recommendation: fix now
Was: 6-sxbi.1
Now: fixed

atelier-2vdq:37 now shows an `open` mission on an unmerged branch as proposed and one dropped there as dropped. atelier-dciw:32-33 defines proposed as status `open` on a branch that has not merged, and `list` of proposed is missions whose status is `open`, plus record changes, on those branches. Outcome:58-61, atelier-ogpg:29 and `design.md` Decisions:51 already used that wording. The two sentences this finding named now match.

## Finding 2: Gathering one verdict of a round unblinds the other reviewer
Recommendation: fix now
Was: 6-sxbi.3
Now: kept

Gathering is gone. `design.md` Decisions:53 and atelier-2vdq:34 have each reviewer commit the file and push it to the mission's branch. Blindness is the checkout at the submitted commit, so a verdict pushed earlier is not in that checkout's files. atelier-dciw:31 no longer hides a round: `show <mission>` on a proposed mission gives each round's verdicts from the mission's branch.

The first verdict of the round is therefore on the branch `show` reads, as soon as that reviewer pushes, with no second verdict and with the round still open. atelier-dciw:29 reads every named branch, local and remote-tracking, not only the files in this checkout. A second reviewer who runs `show`, or whose clone shares those refs, gets the other's verdict in the round it is still reviewing. The test that one published verdict prints a count and no verdict is gone.

The checkout pin stops the other file being in the tree at the start. It does not stop `show`. Have `show` omit that round's files while this checkout is still reviewing it, and keep a test that one published verdict on the mission's branch still prints a count and no verdict in the other reviewer's clone.

## Finding 3: design.md still binds planners with `new --mission`
Recommendation: fix now

`design.md` Decisions:52 says `new --mission` binds the calling thread as a planner, as `start` binds an implementer. atelier-2vdq:32 and atelier-ob9u:34 bind that on `start --mission`, which creates the mission's record on a branch named for it. Outcome:46-49 and the Git-line decision (Decisions:50) make `start` the command that makes that branch; `new` only writes a file (atelier-ob9u:33).

atelier-ogpg copies this `design.md` into `docs/design.md` before the other items land. An implementer of that row records the planner on `new --mission`. Align Decisions:52 with 2vdq:32: `start --mission` records the planner, and a later `submit` by a thread that is not one takes the plan over.
