---
schema: atelier.review
schema_version: 3
round: 1
commit: 7c28a3dbd330faa452b65d89d06988b0bb002fa0
reviewers:
- thread:delegated-task:command%3Amcp%3Adbe6537b-1cdf-48e8-acc9-b67a0a17e994%3Adelegate-task%3Ad695cff4-0a8e-481b-9ea8-ab2bf18d496e
model: gpt-6-astra
verdict: changes
created_at: 2026-11-13T06:16:48.378320247Z
---

## Summary

Complexity: high. The plan changes branch ownership, record discovery, review gates, forge merges and the Desk across five repositories.
Value: high. Protected-base support and visibility of work on other clones address concrete limitations of the current tool.
Risk of merging it: high. An incomplete review gate or cancellation rule can silently authorize work that should have stopped.

The value warrants the scope, provided the contracts below are settled before implementation. I read the mission, its design and all eight items at 7c28a3d, compared their success criteria and dependencies, and checked the current product/design documents and the mission-status guards in atelier-app. This is a plan review; I did not run implementation tests, inspect the four other repositories, or verify live GitHub/Desk configuration.

## Finding 1: Define the complete plan content covered by the two-review gate
Recommendation: fix now

`.atelier/items/atelier-2vdq/item.md:34` gates additions of a mission and changes to only Why, Outcome, Scope and Authority. It omits Success criteria and the mission's supporting documents. For example, a backlog branch can weaken an agreed mission's Success criteria or replace its design without changing any listed section, and satisfy this item's gate with only `atelier check`. That contradicts the mission's reviews-are-the-gate criterion and the design's rule that a changed mission needs two plan reviews. The current plan-change comparison already includes the mission directory except reviews and its items (`docs/design.md:643-649`), so this is a narrower definition than the existing advisory check.

Specify the content that triggers a plan review and the content whose changes invalidate an approval, including Success criteria and supporting plan documents. Explicitly distinguish routine attribution, Result, status and lead-note changes that should use the record-only path. Also state how the intended mid-mission item additions fit that boundary. Require tests that change criteria or a supporting design after approval, and tests that routine record changes remain quick. Review-file commits themselves must not invalidate the plan they review.

## Finding 2: Preserve cancellation when removing the mission readiness check
Recommendation: fix now

`.atelier/items/atelier-2vdq/item.md:35` requires `start` to have no check on a mission's status. But a dropped mission currently prevents its items from starting or being taken over (`crates/atelier-app/src/acts.rs`, the Start and TakeOver guards; `docs/design.md:638-639`). Removing all status checks permits an already-ready item to start after its mission is dropped. The Desk item promises a stop action using exactly that drop record (`.atelier/items/atelier-v958/item.md:35`), and the mission promises a veto at any time.

Remove the readiness/go-ahead requirement specifically, retaining the dropped-mission guard. Define what stop does for proposed plans and already-running items, including what may still submit or merge and when a stop awaiting its own PR becomes effective. Test a stopped mission with a ready item and an active item, and stopping a proposed plan before it merges. These cases need an explicit cancellation contract rather than a button that merely writes a status.

## Finding 3: Assign implementation and proof of the remote branch claim
Recommendation: fix now

`.atelier/missions/atelier-to83/design.md:17-20` makes creation of the remote named branch the atomic claim. No item requires changing `start` to acquire that claim. atelier-ob9u covers branch naming and record-writing commands; atelier-dciw only reads fetched branches and explicitly excludes writes. The current start algorithm creates a local ref atomically (`docs/design.md:697-713`), which excludes contenders in one repository but not two clones. Following the listed acceptance criteria can therefore leave two hosts both successfully starting the same item, and the Desk cannot see either until someone later publishes a branch.

Assign the remote claim to an implementation item and specify when it is acquired, what a losing claimant does, and behavior without a remote or when the remote is unavailable. Require a two-clone concurrent-start test in which exactly one claimant succeeds and the other reports the actual holder, plus a no-remote test. Keep the reader's no-fetch rule separate from publication of a claim. If remote exclusion is intentionally deferred, remove the claim guarantee from this mission and explicitly describe the remaining limitation.
