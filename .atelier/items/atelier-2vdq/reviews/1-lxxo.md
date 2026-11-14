---
schema: atelier.review
schema_version: 3
round: 1
commit: d248fb729d5b95356d95bac07965e343ee83b585
reviewers:
- thread:delegated-task:command%3Amcp%3A9da2310f-484c-4ef4-9547-00086bbf3b23%3Adelegate-task%3A483075ab-79f9-4d2b-af7b-776a7bcd190d
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-14T00:43:57.515626496Z
---

## Summary

Complexity: high. This changes the mission lifecycle, independent review contract, branch merge gates and interpretation of stops across clones.

Value: high. Enforcing two independent plan approvals is necessary once a mission has no human go-ahead.

Risk of merging: high. Two reproducible paths silently admit a mission plan without its required approvals.

The value warrants the implementation, but the approval gate must cover every merge path and the old draft transition before it ships.

Reviewed the diff from master through d248fb729d5b95356d95bac07965e343ee83b585 against every Done when line and the Report. Read the mission model, act rules, plan comparison and approval counting, submit/review/merge commands, stop propagation and views, plus the relevant integration tests. Ran `cargo nextest run -p atelier-cli -p atelier-app -p atelier-records` (302 passed, one skipped), `cargo nextest run -p atelier-core -p atelier-desk` (43 passed), `cargo fmt -- --check` and `git diff --check` (both passed). Used the freshly built debug binary in separate scratch repositories to reproduce all three findings. I did not exercise a live GitHub forge, rerun the five private/live repository checks, or run browser e2e tests; the compatibility fixtures and simulated-forge tests passed in the Rust suite.

## Finding 1: Apply the plan approval gate to item branches too
Recommendation: fix now

In `crates/atelier-cli/src/merge.rs:555`, the new `plans_changed`/`approvals` gate runs only inside `read_records`. `read` dispatches item branches to its other path at line 398, which never performs that check. An item's single code review therefore also admits any mission plan change carried on the branch. This contradicts the Done when requirement that a branch adding a mission or changing a plan needs two plan approvals, and the product rule that a branch merges on its contents whatever its kind.

Reproduced with an open mission on main and an approved item on its own branch. The item's reviewed implementation changed the mission's Why and added a code file. With one approving item review and no mission reviews at all, `atelier merge` exited 0, marked the item done and put the changed Why onto main. The same path can add a new mission without plan reviews. Run the plan gate for item merges as well, including trivial merges, or refuse plan changes there and direct them through the plan review path. Add an integration test with an otherwise mergeable item carrying a mission plan change.

## Finding 2: Require plan reviews when an old base draft becomes agreed
Recommendation: fix now

In `crates/atelier-cli/src/merge.rs:615`, an empty `plan_diff` skips a mission even when `on_base` is false because the base holds an old `draft`. `submit` explicitly changes that draft to `open` in `crates/atelier-cli/src/commands.rs:700`, but front matter is excluded from `plan_diff`. The first agreement of the old plan can consequently merge without either review.

Reproduced with an unchanged `status: draft` mission on main, a checkout on its named mission branch and its recorded planner as caller. `atelier submit` wrote `open` and printed that round 1 needed two reviewers. Immediately running `atelier merge`, with no review files, exited 0 and reported that records alone need no review. Main then held `status: open`, making that previously proposed mission eligible to be led and its ready items started. Treat the transition from a base-resident old draft to an agreed mission as an initial plan agreement even if its sections and items are unchanged, and test that zero or one approval refuses it.

## Finding 3: Show a fetched published stop as published
Recommendation: fix now

In `crates/atelier-cli/src/views.rs:96`, `unpublished_stop` decides publication solely from the local base and branches not merged into that base. It never checks the fetched forge base. `show` also renders `view.dir` at line 28 without applying `Repo::published_stops`, although `list` and the item gates use that newer stop.

Reproduced with two clones and a local bare forge remote. Clone A published an open mission's stop by merging backlog/stop into main and pushing both branches. Clone B fetched origin while keeping its local main at the earlier open mission, the same state covered by the new cross-clone item-stop test. In B, `atelier show demo-m1ss` printed both `Stop not published ... the stop stops nothing yet` and `Status: agreed`. The stop is already published and the new item gates honor it. Apply the fetched published stop consistently to the displayed mission and suppress the unpublished warning once that stop is on the fetched forge base. Cover `show` in the cross-clone stop test.
