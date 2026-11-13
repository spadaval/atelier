---
schema: atelier.review
schema_version: 3
round: 2
commit: a8103131eb338501a20e07e8b9ce154b14f91236
reviewers:
- 01a1d396-3f44-7df2-95a7-b4204e2d5a15
model: grok-4.6
verdict: changes
created_at: 2026-11-13T06:27:02.302031506Z
---

## Summary

Checked the plan at a810313: `mission.md`, `design.md`, and all eight items; the blocker graph; current `docs/product.md` (lifecycle, Plan, records layout) and `docs/design.md` (act table, where records are read, start, plan review, merge, terminal-status-wins); `crates/atelier-core/src/mission.rs` (`MissionStatus`, no planners field); `crates/atelier-app/src/acts.rs` (`Submit` / `Merge` / `Drop` / `CloseMission` / `Lead`); `roles/planner.md`; and atelier-lkf5 as cited. Did not run commands against a protected remote, time a record-only CI path, or read another round-2 reviewer's file.

Complexity: high. Seven commands move off the base, merge finishes through the forge, the mission record changes shape, and the Desk's reader becomes the live picture.
Value: high. A protected base, no draft commits on master, no cherry-picked reviews, no second ask of Sumeet, and work in motion visible from any clone.
Risk of merging it: high. A mistake in `merge` or the pre-push rule stops five repositories at once.

The round-1 holes that named the plan surface, the dropped-mission guard, the remote claim, record-only path refusal, submit/merge act rows, review refs instead of one shared branch, the gate/criterion mismatch, planners, and the `open` status are written into the mission, the design, and the items. What remains is that the gate still does not cover the items a plan review is asked to read, Result cannot be told from the rest of `mission.md`, the review refs are unnamed and unread, stop is not a complete act, and the remote claim leaves a losing clone looking like a holder. Value is worth this mission once those are specified. A smaller change does not get the protected-base outcome.

## Finding 1: Define the complete plan content covered by the two-review gate
Recommendation: fix now
Was: 1-aooo.1
Now: fixed
The Outcome, `reviews-are-the-gate`, `design.md` Shape, and atelier-2vdq:33-35 now include Success criteria and supporting documents, put lead, notes, Result, status and mid-mission items on the record-only path, test criteria and `design.md` after approval, and say review-file and front-matter commits do not stale an approval.

## Finding 2: Preserve cancellation when removing the mission readiness check
Recommendation: fix now
Was: 1-aooo.2
Now: fixed
`start` and take-over keep the dropped-mission guard; an item's `merge` refuses after the drop merges; a proposed drop commits on the mission branch and closes its pull request; tests cover a proposed plan and an open mission with a ready item and a started item.

## Finding 3: Assign implementation and proof of the remote branch claim
Recommendation: fix now
Was: 1-aooo.3
Now: fixed
atelier-ob9u:37 and the design's claim decision assign the remote push to `start`, name the loser and the no-remote / unreachable cases, and require the two-clone test.

## Finding 4: Record-only merge never refuses a branch that also changes code
Recommendation: fix now
Was: 1-yj2r.1
Now: fixed
Authority, `design.md` Shape, and atelier-ob9u:35 name `.atelier/` as the records path set and require a test that a `backlog/` branch carrying `crates/` is refused.

## Finding 5: No act names who may submit or merge a mission or backlog branch
Recommendation: fix now
Was: 1-yj2r.2
Now: fixed
atelier-2vdq:31 names planner, checkout on the mission branch, from proposed; atelier-ob9u:36 names any thread, checkout on the backlog branch, no from-status; both say these are not the item implementer acts.

## Finding 6: Two plan reviewers sharing the mission branch cannot stay blind or both push
Recommendation: fix now
Was: 1-yj2r.3
Now: fixed
Each `verdict` publishes a ref of its own; neither `review` nor `verdict` reads another reviewer's ref of the same round; the planner's next `submit` or `merge` copies both onto the mission branch. The naming of those refs is finding 12.

## Finding 7: The plan-review gate's surface disagrees with the criterion and with lead
Recommendation: fix now
Was: 1-yj2r.4
Now: fixed
`reviews-are-the-gate`, `design.md` Shape, and atelier-2vdq now use one surface: a new mission or a changed plan needs two reviews; lead, notes, Result, status and an item added to an open mission take `atelier check`.

## Finding 8: Merge cannot tell a planner from a reviewer on a mission
Recommendation: fix now
Was: 1-yj2r.5
Now: fixed
`new --mission` records the calling thread as a planner; a later `submit` takes the plan over; `review` refuses a planner; `merge` requires two reviews from threads that are not its planners.

## Finding 9: An agreed open mission has no status for close, lead, show or the Desk
Recommendation: fix now
Was: 1-yj2r.6
Now: fixed
Stored status is `open`, `done` or `dropped`; unmerged `open` is shown as proposed; `lead` and `close` run from `open`; old `status: ready` reads as `open` and old `status: draft` on the base reads as proposed.

## Finding 10: Items on a proposed plan sit outside the two-review gate
Recommendation: fix now
`docs/product.md:69` defines a plan as the mission's directory plus its items at one commit, and `docs/design.md:643-646` already compares item directories when it reports what changed since a plan review. The review brief for this round still includes the items. The new plan surface does not.

atelier-to83 Outcome:47-53 and `design.md` Shape:31-33 define the plan as Why, Outcome, Success criteria, Scope, Authority and the files beside `mission.md` other than `notes.md` and the reviews. atelier-2vdq:33-35 gates exactly that directory, and its "added item" test is the record-only path for an open mission. Item files live under `.atelier/items/`, so a commit that rewrites every Done when on the mission's branch after two approvals neither changes a gated path nor matches "adds an item to an open mission".

A branch that still "adds a mission" needs two reviews of "the plan its head holds". If that plan excludes items, those reviews stay valid while the work they were asked to read changes. State that items on an unmerged mission branch are part of the plan, that a change to them after approval is refused, and that adding an item is record-only only once the mission is on the base. Require a test that an item's Done when changed on the proposed branch after approval is refused, beside the existing open-mission add-item test.

## Finding 11: Result cannot be told from the gated sections of mission.md
Recommendation: fix now
Result is a section of `mission.md` (`docs/product.md:294`, `docs/design.md:179-180`), not a file of its own. atelier-2vdq:35 and `reviews-are-the-gate` put a Result-only change on the record-only path. atelier-2vdq:34 treats Why, Outcome, Success criteria, Scope and Authority, which are the other sections of the same file, as the plan, and "any other file in its directory" as the plan as well. The only tests are a Success-criteria change (refused) and a note or added item (allowed). There is no test that a Result-only edit merges on `atelier check`.

An implementer who diffs `mission.md` as one file will demand two plan reviews for writing Result, or will treat any body edit as records. Spell the comparison as sections of `mission.md`: the five plan headings are the plan; Result, like `lead` and status, is records. Require a test that a Result-only change on an open mission merges without a plan review, and that a Why change on that mission is refused.

## Finding 12: Plan-review refs are unnamed and the reader never looks at them
Recommendation: fix now
atelier-2vdq:33 says each `verdict` publishes "on a ref of its own, named by a rule in the design". `.atelier/missions/atelier-to83/design.md:50` says only that the ref is "of its own". atelier-ogpg's Done when copies this file into `docs/design.md` and never adds a naming rule. `merge` has to find those refs to copy them; two clones in the required test have to write and read the same names.

atelier-dciw:29 reads "every branch the convention names": item id, mission id, `backlog/<slug>`. Review refs are none of those, and they must not be copied onto the mission branch until both verdicts exist, or the second reviewer who fetches that branch sees the first's file (the blindness atelier-2vdq:33 is there to keep). Until that copy, `list`, `show` and the Desk have no record of the reviews. atelier-v958:32 still asks the proposed-work page to show how many approving plan reviews a plan has, and its Out says any read dciw does not give is filed against dciw.

Name the refs (thread, round, mission), that `verdict` writes only that ref and not the mission branch, that `merge` / `submit` read them, and that the named-branch reader, or a rule beside it, finds them so `show` and the Desk can count a proposed plan's approvals without breaking blindness. Put that rule in this mission's `design.md` so atelier-ogpg has something to copy.

## Finding 13: Stop is not an act that publishes, and the Desk is not required to finish it
Recommendation: fix now
The veto is the replacement for the go-ahead (Outcome:64, design decision "Stopping a mission is dropping it"). atelier-2vdq:39 says `drop` on a proposed mission commits on its branch and closes its pull request, and that an open-mission stop is effective from its merge. atelier-v958:35-36 says the Desk's "stop" writes the same record as `atelier drop`, takes effect when that branch merges, and shows when it has. Neither item gives `drop` an act-table row: who may run it, from proposed versus from `open`, in which checkout. atelier-ob9u:36 and atelier-2vdq:31 spelled those rows only for `submit` and `merge`.

Today `Act::Drop` is a planner act from `draft` / `ready` / a started item (`crates/atelier-app/src/acts.rs:195-207`). Sumeet on the Desk is not that planner. Drop now commits on a branch (`atelier-ob9u:32`) and needs `submit` then `merge` before the base changes. The Desk item requires neither. A stop can remain a local commit, or an unmerged backlog pull request, while `start` still succeeds, which is the "button that merely writes a status" the last round asked to replace.

Give `drop` of a proposed plan and of an open mission act-table rows, including who the Desk acts as. Require the Desk stop, or the command it runs, to push and merge that branch, or name who else must and how soon. Test that after Desk stop of an open mission, and after the path that makes it effective, `start` on a ready item is refused.

## Finding 14: The losing start claimant keeps a local branch that still looks started
Recommendation: fix now
atelier-ob9u:37 pushes the new branch before `start` reports success and refuses when the remote already has it. The current algorithm still creates the local ref first (`docs/design.md:697-707`). Two clones can each create `refs/heads/<id>`, then one push wins. The loser is told the holder, and is left with an unmerged local branch whose record names them.

atelier-dciw:29 reads every named local branch, and today's winner rule is the unmerged branch's record (`docs/design.md:658-660`). In the losing clone, `list`, `show`, `status` and the Desk's read of that clone still show a started item held by the loser. The two-clone test as written checks only that one `start` succeeds.

Say what `start` does to the losing clone's local ref (delete it, or refuse while it exists and tell the operator to delete it), that a later `start` in that clone is a take-over of the remote holder rather than a second start, and that the reader prefers the remote-tracking claim over a leftover local ref. Extend the two-clone test to `list` in the losing clone after the refused `start`.

## Finding 15: Dropping a started item is no longer effective until some other branch merges
Recommendation: fix now
Today `drop` writes the item on the base and every later command reads that record first, so `submit`, `merge` and take-over are refused at once (`docs/design.md:631-637`, `666-670`). atelier-ob9u:32 moves `drop` onto `backlog/<slug>`. Nothing says an in-flight item `merge` waits for that backlog branch, or is refused while it exists. Two pull requests then race: the item can merge as `done` after `drop` has already been committed on the other branch.

atelier-2vdq spelled the delayed-effectiveness contract for a mission stop and not for an item. Either keep item cancellation immediate on the base (which this mission forbids), or say that `merge` of a started item is refused while an unmerged drop of it exists, that the drop's merge wins over the item's, and test both orders.
