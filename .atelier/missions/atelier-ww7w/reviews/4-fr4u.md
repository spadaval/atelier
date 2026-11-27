---
schema: atelier.review
schema_version: 3
round: 4
commit: 96227cc29de0a2d40e3dd14769bf58cc012595bc
reviewers:
- 01a201fd-aa7c-74f2-9969-6f228b0891d2
model: grok-4.6
verdict: changes
created_at: 2026-11-22T06:42:31.500590605Z
---

## Summary

Checked the plan as 96227cc holds it, focused on `f89b57b94..96227cc29` (mission, design, and the five items that commit changed; `atelier-1n49` unchanged). Read the six ready items and verified each of the six carried findings against the code those items name: `start_backlog` ([commands.rs:357](crates/atelier-cli/src/commands.rs:357)), `take_over` switching before it pushes ([commands.rs:258](crates/atelier-cli/src/commands.rs:258)), `delete_branch` as `update-ref -d` ([git.rs:140](crates/atelier-cli/src/git.rs:140)), `plan_diff` / `in_plan` ([plan.rs:135](crates/atelier-core/src/plan.rs:135), [plan.rs:460](crates/atelier-core/src/plan.rs:460)), and `git for-each-ref --contains HEAD` with and without `refs/heads refs/remotes refs/tags`.

Complexity: high. Six items still share `output.rs` / `say.rs` and now also lease-checked deletion, detached-HEAD guards, and a Desk PR fallback. Value: high. Record-only CI, split plan reviews, started-item drop, garden merge, and M5 are paths in current use. Risk of merging it: medium. The six carried holes are specified with tests. One procedure hole remains: Step 4 recovery from the last Report puts a proposal back after a partial answer had removed it.

Could not check: the 2026-11-22 conversation; a real forge merge; `jy8k`'s scheduler; the other round-4 reviewer's file. Did not rebuild or rerun clippy.

The six carried findings are fixed. Round 3's deferred notes on the `"Garden proposals"` title, overlapping `git reset --keep`, and the 24h `created_at` filter are in this delta too. One new fix-now finding.

## Finding 1: removing the garden session leaves its four jobs with nobody
Recommendation: fix now
Was: 3-asf8.2
Now: fixed
The Report lists `Bugs to file`, and Step 4 recovers those as well as `Proposals`. A planner closes `"Garden proposals"` only when every proposal is answered, and otherwise removes only the answered ones.

## Finding 2: detaching after a merge lets the next record commit vanish
Recommendation: fix now
Was: 3-asf8.5
Now: fixed
The guard is `git for-each-ref --contains HEAD refs/heads refs/remotes refs/tags`, and the test plants a `refs/t3/` checkpoint. This clone has three such refs containing HEAD; the unscoped command lists them, the scoped one does not.

## Finding 3: Dated garden-proposal branches cannot update the one open item
Recommendation: fix now
Was: 3-asf8.9
Now: fixed
The slug stays `garden-proposals`. If that branch exists locally or on the remote, merged or not, the gardener switches to it (in the worktree that holds it), brings `base` in, and updates the item. `start --backlog` runs only when neither ref exists. `start_backlog` still refuses a slug that exists ([commands.rs:374](crates/atelier-cli/src/commands.rs:374)).

## Finding 4: an amendment's delta hides the item edits the amendment made, and calls the plan unchanged
Recommendation: fix now
Was: 3-asf8.10
Now: fixed
An amendment keeps item paths the branch itself changed relative to `base` (`git diff <base>...<head>`), less reviews, intersected with `prev`..`head`. Replayed on atelier-osel from the initial plan merge: rounds 4 to 5 keep the six item files, 5 to 6 the two, 6 to 7 `atelier-diru`'s `item.md`. The criterion and the test name both directions.

## Finding 5: the take-over race rule deletes the previous implementer's unmerged work
Recommendation: fix now
Was: 3-asf8.11
Now: fixed
A losing take-over deletes neither ref, refuses, names the branch, and leaves a valid HEAD. A losing fresh claim deletes only the local branch and does not switch. Authority still limits remote deletion to `atelier merge`. The take-over-vs-drop test is named. `take_over` still switches before it pushes ([commands.rs:258](crates/atelier-cli/src/commands.rs:258)).

## Finding 6: The stable garden-proposals slug cannot be started after the first merge while the ref remains
Recommendation: fix now
Was: 3-g7hi.10
Now: fixed
Same rule as Finding 3: reuse the branch whether merged or unmerged, and do not assume `atelier-gqtw` deleted it. `atelier-7637` stays parallelizable with no `blocked_by`.

## Finding 7: Step 4 recovery puts answered proposals back in the Inbox
Recommendation: fix now
The two garden rules added in this round disagree.

`atelier-7637` has the planner remove an answered proposal from `"Garden proposals"` when only some are answered ([item.md:39](.atelier/items/atelier-7637/item.md)). The next run's Step 4 always reads the last merged run's Report and files any proposal that no existing item already covers ([item.md:37](.atelier/items/atelier-7637/item.md); [design.md:65](.atelier/missions/atelier-ww7w/design.md)).

Daily runs (`atelier-jy8k`) with a partial answer are the Why's own case. After Run 1 files A, B, C and the human answers only A, the open item holds B and C. Run 2's Step 4 still sees A in Run 1's Report. A is not in any open or closed `"Garden proposals"` item. The planner's follow-up item for A may still be on an unmerged backlog branch, and Step 4 does not say to search those. Recovery files A again.

Keep answered proposals in the item, marked answered, until it closes, or have recovery ignore a proposal that already has a recorded decision. Scanning the last Report only when `backlog/garden-proposals` was never updated for that run also avoids it.
