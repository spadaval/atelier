---
schema: atelier.review
schema_version: 3
round: 4
commit: 96720f4605d9787558c2d924728d052c98de425d
reviewers:
- thread:delegated-task:command%3Amcp%3Adbe6537b-1cdf-48e8-acc9-b67a0a17e994%3Adelegate-task%3A1ea3810f-6278-4be9-8e01-a70a711d8590
model: gpt-6-astra
verdict: approve
created_at: 2026-11-13T06:41:37.383216221Z
---

## Summary

Reviewed the plan at `96720f4605d9787558c2d924728d052c98de425d`: the mission,
its design, all eight items and their dependencies, and the amendment against
the three carried findings. Compared the drop contract with the existing act
table and record-writing rules in `docs/design.md`. `atelier check` passes
for all 153 items, nine missions and the configuration.

- Complexity: high. This changes command writes, cross-clone reads and claims,
  plan gates, forge merges, and the Desk across five repositories.
- Value: high. The branch model enables protected bases and makes published
  work visible without the current planning cherry-picks and go-ahead wait.
- Risk of merging it: high. Incorrect merge or record precedence rules can
  allow stopped work to proceed or block all five repositories.

The value warrants the scope. The documentation-first item settles the
remaining implementation choices before code, and the rollout starts with
Atelier before the other four repositories. I found no smaller change that
meets the protected-base, visibility and stop requirements together.

This is a plan review, not verification of the future implementation. I did
not execute the proposed cross-clone races, forge contract tests, Desk tests,
or timing checks. Record precedence, the forge merge race and the chosen
representation of done still require review in the items that own them.

## Finding 1: A started item's drop never lands on the base, and has no act
Recommendation: fix now
Was: 3-ja6u.8
Now: fixed

`design.md:56` and atelier-ob9u's Done when now require the same `drop` to
publish the item's branch and land the base copy through a `backlog/` branch.
The item specifies status 3, resumption, branch-deletion coverage and both
orders of drop versus merge from two clones. This removes the unnamed
follow-up that could leave a ready record on the base. The existing act table
already assigns an item's drop to the planner (`docs/design.md:347`);
atelier-ogpg requires the destination of every act to be documented, and
atelier-ob9u requires the act table and help to match the new behavior.
Those requirements cover adapting the existing act rather than requiring a
new actor to be invented here.

## Finding 2: atelier-ogpg still documents three kinds of branch
Recommendation: fix now
Was: 3-ja6u.9
Now: fixed

`atelier-ogpg/item.md:29` now enumerates all four branch kinds, including the
reviewer's ref, and the optional prefix. Read together with `design.md:49`
and `:53`, the prefix covers the convention as a whole, and review refs are
published by verdict and brought onto the mission branch by its planner.
The item also requires every Decisions row to reach the durable documents.
The lifecycle of a review ref is therefore specified even though the first
checklist line uses the general phrase about what each branch needs before
merging. The reader item explicitly includes review refs.

## Finding 3: Outcome still treats notes.md as both plan and records
Recommendation: fix now
Was: 3-ja6u.10
Now: fixed

`mission.md:53-55` now explicitly excludes `notes.md` and the reviews from
the plan. This agrees with the design's Shape section and atelier-2vdq's
section-based approval gate and notes-only test.
