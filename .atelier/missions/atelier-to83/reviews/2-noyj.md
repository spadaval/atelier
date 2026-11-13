---
schema: atelier.review
schema_version: 3
round: 2
commit: a8103131eb338501a20e07e8b9ce154b14f91236
reviewers:
- thread:delegated-task:command%3Amcp%3Adbe6537b-1cdf-48e8-acc9-b67a0a17e994%3Adelegate-task%3A37d6e6ed-9d17-4db2-8c1c-1e0e0dc75af9
model: gpt-6-astra
verdict: changes
created_at: 2026-11-13T06:26:48.461736793Z
---

## Summary

Complexity: high. The plan changes record writes, cross-clone claims, review publication, forge merges and the Desk across five repositories.
Value: high. Protected-base support and a live view of work address current limitations.
Risk of merging it: high. Incorrect approval invalidation can silently accept a plan that nobody reviewed.

The value warrants the scope, with the remaining approval gap below fixed first. I read the mission, design and all eight items at a810313, checked the nine carried findings against the revised requirements, and compared the existing plan-change code and the record rules in docs/design.md. I did not inspect another reviewer of this round. This was a plan review: I did not run implementation tests, exercise GitHub, inspect the other four repositories or measure the live Desk/CI. The smaller choices explicitly assigned to atelier-ogpg can be settled there.

## Finding 1: Define the complete plan content covered by the two-review gate
Recommendation: fix now
Was: 1-aooo.1
Now: fixed

The gate now includes Success criteria and supporting documents; routine records and review-file commits are distinguished explicitly in atelier-2vdq:35-36. The separate remaining issue with initial item content is Finding 10.

## Finding 2: Preserve cancellation when removing the mission readiness check
Recommendation: fix now
Was: 1-aooo.2
Now: fixed

atelier-2vdq:38-39 retains start/take-over guards, adds the item-merge guard and specifies proposed-plan cancellation. The design states when cancellation of an open mission becomes effective and requires tests with ready and active items.

## Finding 3: Assign implementation and proof of the remote branch claim
Recommendation: fix now
Was: 1-aooo.3
Now: fixed

atelier-ob9u assigns publication at start, refusal with the existing holder, unreachable-remote behavior, local-only behavior and the two-clone concurrent-start test.

## Finding 4: Record-only merge never refuses a branch that also changes code
Recommendation: fix now
Was: 1-yj2r.1
Now: fixed

atelier-ob9u explicitly limits the record-only path to .atelier/ and requires a refusal test for a backlog branch changing crates/.

## Finding 5: No act names who may submit or merge a mission or backlog branch
Recommendation: fix now
Was: 1-yj2r.2
Now: fixed

atelier-ob9u and atelier-2vdq now specify actors, checkout and state for backlog and proposed-mission submit/merge, separately from item acts.

## Finding 6: Two plan reviewers sharing the mission branch cannot stay blind or both push
Recommendation: fix now
Was: 1-yj2r.3
Now: fixed

atelier-2vdq requires separate reviewer refs, publication at verdict, no reading the other reviewer in the round and automatic collection, tested with two clones.

## Finding 7: The plan-review gate's surface disagrees with the criterion and with lead
Recommendation: fix now
Was: 1-yj2r.4
Now: fixed

The mission criterion, design and atelier-2vdq agree on the mission sections, supporting documents and routine-record exceptions.

## Finding 8: Merge cannot tell a planner from a reviewer on a mission
Recommendation: fix now
Was: 1-yj2r.5
Now: fixed

atelier-2vdq records planners on creation and takeover and excludes them both at review and merge.

## Finding 9: An agreed open mission has no status for close, lead, show or the Desk
Recommendation: fix now
Was: 1-yj2r.6
Now: fixed

atelier-2vdq defines open/done/dropped, lead and close from open, and compatibility mappings for both old draft and ready records.


## Finding 10: Keep the proposed mission's items inside approval invalidation
Recommendation: fix now

`.atelier/items/atelier-2vdq/item.md:35` defines the content an approval covers as mission sections and files in the mission directory. Item records live outside that directory. The exception on line 36 explicitly allows adding items to an already-open mission, but nothing protects the items of a still-proposed plan. After two approvals, a planner can change an initial item's Goal, Done when, Scope or blockers without changing the defined plan content, and the old approvals still satisfy this gate. For example, removing the concurrent-start test from atelier-ob9u after review would not stale either approval.

The existing comparison includes item directories belonging to the mission at either the reviewed commit or the new tip (`crates/atelier-app/src/plan.rs:130-176`). Review still examines those items, as this review's command explicitly required. Narrowing the comparison loses protection for a substantial part of what was reviewed.

Specify that, before the initial plan merges, changing, adding, removing or reassigning its items invalidates the plan approvals. Keep the intended record-only additions after agreement. Require a test that edits an initial item's acceptance criteria after both approvals and refuses merge until fresh approvals, plus the already-planned test that adding an item to an agreed mission needs no plan review.

## Finding 11: Show the correct stop timing for a proposed plan
Recommendation: defer

`.atelier/items/atelier-v958/item.md:35` says the stop page tells the user cancellation takes effect when its branch merges. For a proposed plan, atelier-2vdq:39 instead closes that branch's pull request. There will be no merge to wait for. During the Desk item, distinguish the proposed-plan case from an open mission: show the proposed plan stopped when its cancellation is published and its PR closed; show an open mission's stop pending until the record branch merges. Include both in the UI test. This wording mismatch need not delay the mission's start.
