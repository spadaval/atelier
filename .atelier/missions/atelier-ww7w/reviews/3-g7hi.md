---
schema: atelier.review
schema_version: 3
round: 3
commit: f89b57b9447f3d65e7ce3df06c763a99f02dad76
reviewers:
- 01a201ec-9132-7f80-abff-3c47511c9dd8
model: grok-4.6
verdict: changes
created_at: 2026-11-22T06:23:50.494662076Z
---

## Summary

Checked the plan as f89b57b holds it, focused on `6d1acd1be..f89b57b94` (mission, design, and the five items that commit changed; `atelier-1n49` unchanged). Read the six ready items and verified each of the nine carried findings against the code those items name: `plan_diff` / `in_plan` ([plan.rs:113](crates/atelier-core/src/plan.rs:113), [plan.rs:460](crates/atelier-core/src/plan.rs:460)), `start_backlog` ([commands.rs:357](crates/atelier-cli/src/commands.rs:357)), `misplaced` ([plan.rs:244](crates/atelier-core/src/plan.rs:244)), Desk `pull_request` keyed by branch ([server.rs:597](crates/atelier-desk/src/server.rs:597), [fetch.rs:175](crates/atelier-desk/src/fetch.rs:175)), `catch_up` ([merge.rs:399](crates/atelier-cli/src/merge.rs:399)), `garden.md` steps 4 and 7, and the product rule that `notes.md` is not part of the plan ([product.md:80](docs/product.md:80), [product.md:394](docs/product.md:394)).

Complexity: high. Six items still share `output.rs` / `say.rs` and now also lease-checked deletion, detached-HEAD guards, and a Desk PR fallback. Value: high. Record-only CI, split plan reviews, started-item drop, garden merge, and M5 are paths in current use. Risk of merging it: medium. The eight deletion, delta, and split holes from round 2 are specified with tests. One procedure hole remains: the stable `garden-proposals` slug still cannot be started after the first merge while that ref exists.

Could not check: the 2026-11-22 conversation; a real forge merge; `jy8k`'s scheduler; the other round-3 reviewer's file. Did not rebuild or rerun clippy.

The nine carried findings are fixed. One new fix-now finding.

## Finding 1: the plan-review delta is mostly the last round's reviews, not the plan
Recommendation: fix now
Was: 2-jo28.7
Now: fixed
`atelier-h6ij` now uses a dedicated helper matching `plan_diff`'s boundary ([plan.rs:460](crates/atelier-core/src/plan.rs:460)): no `reviews/`, no `notes.md`, and on an amendment no `.atelier/items/*`. Tests require an amendment whose base merged an item between rounds to exclude that item's Report and reviews.

## Finding 2: removing the garden session leaves its four jobs with nobody
Recommendation: fix now
Was: 2-jo28.10
Now: fixed
An update keeps every unanswered proposal until the human answers. Step 4 also reads the last merged run's Report and files any proposal no `needs-human,garden` item already lists. A planner still applies answers and closes the item.

## Finding 3: deleting the merged branch lets a stale clone start finished work again
Recommendation: fix now
Was: 2-jo28.15
Now: fixed
`start` (fresh and take-over) fetches `<remote>/<base>` first, refuses `done`/`dropped` there, and re-checks after the claim push. `submit` does the same. `merge` deletes neither ref until `base_tip` contains `<head>`. Each has a test. `docs/design.md`'s `start` row is in Scope.

## Finding 4: the split count never resets, so atelier-kw8u's next amendment is refused before any review
Recommendation: fix now
Was: 2-jo28.16
Now: fixed
Trailing splits count only among rounds present at `head` and absent from `base_tip`. A test requires an amendment of a mission whose plan merged after two splits to open its first round.

## Finding 5: detaching after a merge lets the next record commit vanish
Recommendation: fix now
Was: 2-jo28.17
Now: fixed
Planning commands refuse a detached `HEAD`. `status` names unattached commits and `start` refuses to switch away without `ATELIER_SKIP`. `merge` prints the `start` command. A test covers merge → `new` refused → manual commit → `status` reports it and `start --backlog next` refuses.

## Finding 6: deleting the branch removes done items' pull request links from the Desk and the extension
Recommendation: fix now
Was: 2-jo28.18
Now: fixed
`atelier-desk` is in `atelier-gqtw`'s Scope. A `done`/`dropped` item with `row.branch` `None` looks up `pull_request` by `branch_of(id)`. A test covers a done item with neither ref.

## Finding 7: gqtw cannot pass the evals it must pass, because `evals/` is outside its Scope
Recommendation: fix now
Was: 2-jo28.19
Now: fixed
`evals/agent-factory/` is in `atelier-gqtw`'s Scope and Done when, including the `loop` and `settle-within-authority` fixtures that assumed the merged local branch stayed.

## Finding 8: An empty plan-path list prints `git diff` with no pathspec
Recommendation: fix now
Was: 2-nld2.15
Now: fixed
An empty path list prints that the plan is unchanged and never prints `git diff` with an empty pathspec; a test covers that case. The same Done when still excludes `notes.md`. That matches `plan_diff` and the product rule that `notes.md` is the lead's notes, not part of the plan ([product.md:80](docs/product.md:80), [plan.rs:483](crates/atelier-core/src/plan.rs:483)). A lead ruling that changes the plan is a change to `mission.md` or `design.md`, which the delta still includes.

## Finding 9: Dated garden-proposal branches cannot update the one open item
Recommendation: fix now
Was: 2-nld2.16
Now: fixed
The slug is the stable `garden-proposals`, never dated. An unmerged `backlog/garden-proposals` is reused; an open `needs-human,garden` item is updated in place. Finding 10 is the leftover: after that branch has merged, `start --backlog` still refuses while the ref exists.

## Finding 10: The stable garden-proposals slug cannot be started after the first merge while the ref remains
Recommendation: fix now
`atelier-7637` tells the gardener: if `backlog/garden-proposals` exists and is unmerged, switch to it; otherwise `atelier start --backlog garden-proposals`, "which stays free after each merge once `atelier-gqtw` deletes merged backlog branches" ([item.md](.atelier/items/atelier-7637/item.md)). `start_backlog` refuses whenever that branch exists locally or on the remote ([commands.rs:374](crates/atelier-cli/src/commands.rs:374)), and the refusal says "Pick another slug, or work on that branch" ([output.rs:407](crates/atelier-cli/src/output.rs:407)).

After the first successful merge of that slug, the "unmerged" arm is false. The `start --backlog` arm then fails in every case the ref is still there:

- `atelier-7637` can land first: the design keeps zero `blocked_by` edges and lists `atelier-7637` as parallelizable, with the lead free to run it beside the CLI items ([design.md](.atelier/missions/atelier-ww7w/design.md)). Until `atelier-gqtw` deletes merged backlog branches, the second daily run (`atelier-jy8k`) cannot start the slug.
- After `atelier-gqtw` lands, `merge` deletes neither ref when `catch_up` fails, which is the guard Finding 3 added. The next `start --backlog garden-proposals` still refuses.
- If uncommitted dirt was skipped, `merge` leaves the local branch attached, and `start --backlog` refuses in any other worktree of the same clone.

A gardener who follows the printed refusal and picks another slug recreates Finding 9's two-branch collision on one `needs-human,garden` item ([product.md:464](docs/product.md:464)).

Say: if `backlog/garden-proposals` exists (merged or not), switch to it, bring `base` in, and update the item there. Call `start --backlog garden-proposals` only when neither local nor remote has that branch. Do not assume `atelier-gqtw` has deleted it, and do not pick a second slug.
