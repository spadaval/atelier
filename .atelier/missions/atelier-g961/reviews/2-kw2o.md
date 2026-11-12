---
schema: atelier.review
schema_version: 3
round: 2
commit: 31a5d5d85de3ae45e4a87fb2dc9c65983fcf67d3
reviewers:
- 01a1cd51-5d38-78d1-85dd-5ae1ad2f978f
model: grok-4.6
verdict: changes
created_at: 2026-11-12T01:14:00.743777254Z
---

## Summary

Reviewed the plan as commit 31a5d5d holds it: the mission, the round-1 amendment, all three draft items, and the files they rewrite (`garden.md`, `readiness.md`, `groom.md`, `planner.md`, `SKILL.md`) plus the named lines in `docs/product.md`, `docs/product/desk.md` and `docs/architecture/desk.md`. Checked the Why against atelier-613a's Report, atelier-h201 (dropped into wb05), atelier-8i86, atelier-0019 via `atelier show` (page on the branch, Window ending 2026-11-11T17:56:43Z), atelier-j86z (also labelled `garden`), atelier-jy8k, and the 295u gardener design. Counted today's line budgets with `wc -l` (garden 188 + readiness 110 = 298; planner 11 + groom 85 = 96). Did not start a gardener, read another reviewer's file, or replay the 2026-11-11 sweep item by item.

- Complexity: medium. Two procedure rewrites and one trial; no new command or record kind.
- Value: high. 613a's nine proposals and 8i86's 21-entry digest show the two human-facing jobs failing in the way the Why states.
- Risk of merging it: medium. A drop on the base is visible and reversible, and a bad garden page is caught in session; a window or thread-cap rule two gardeners apply differently will make the trial fail or will skip the evidence the rewrite is for.

The split is still worth doing. Round 1's seven contract gaps are closed in the items and in the mission criteria. Two holes remain before the mission starts: the trial's window cannot be found from `garden.md`'s rule as written, and the thread cap still does not say what it counts.

## Finding 1: A missing code reference does not prove the work is stale
Recommendation: fix now
Was: 1-pxw5.1
Now: fixed
`.atelier/items/atelier-s47q/item.md:31-33` now requires proof against the Goal and unticked Done when lines (need gone by a named commit, behaviour already there, or a named item covering all that remains), and says a search that finds nothing is not proof.

## Finding 2: Define the test for an item Sumeet directed or parked
Recommendation: fix now
Was: 1-veho.1
Now: fixed
Mission Authority and `atelier-s47q/item.md:34` give a records test: `ready.by` names him, or the body says he asked or gives his pick-up condition. "Parked" is gone from the calls line. `start --new` items are started, and started items are left to their owners.

## Finding 3: Gone names cannot both prove a drop and justify a rewrite
Recommendation: fix now
Was: 1-veho.2
Now: fixed
Drops on proof and Gone names are not proof are now separate Done when lines. A miss in the code is a trace, a rewrite, a keep, or a call, never a drop by itself.

## Finding 4: Reading every escalation and skip fights the lead's cap on threads
Recommendation: fix now
Was: 1-veho.3
Now: fixed
`atelier-wb05/item.md:33` drops "every", states an order, and caps at a number the procedure gives. Finding 9 is the remaining hole in what that number counts.

## Finding 5: Say how a run picks a focus when no area has been looked at
Recommendation: fix now
Was: 1-veho.4
Now: fixed
`atelier-wb05/item.md:31` now picks Sumeet's name, else the first listed area no earlier page recorded, else the one recorded longest ago. A page with no focus line counts as recording none.

## Finding 6: The trial gardener will not take 0019's window
Recommendation: fix now
Was: 1-veho.5
Now: fixed
Self-selection is closed: `atelier-wb05/item.md:32` takes the newest other garden item that has a page, and `atelier-s0j2/item.md:31` checks 0019 by that rule. Finding 8 is the remaining hole in how that page is read.

## Finding 7: Thread quotes on the page go to public Git
Recommendation: fix now
Was: 1-veho.6
Now: fixed
The trial criterion, `atelier-s0j2/item.md:32-33` and `atelier-wb05/item.md:34` require a thread ID and a trip in the gardener's words, and forbid transcript text, titles, home paths and personal details on the page and in `artifacts/`.

## Finding 8: Say how a waiting run's page is read
Recommendation: fix now
`.atelier/items/atelier-wb05/item.md:32` starts the window at "the end given on the page of the newest garden item other than this run that has a page, merged or waiting for its session". That does not say where the page is, or what counts as one.

On this repository `atelier list --label garden` has two items: atelier-0019 and atelier-j86z. 0019's base `item.md` is still `ready` and has no Report. `atelier show atelier-0019` reads the branch and prints `Window: 2026-11-11T15:20:09Z to 2026-11-11T17:56:43Z`. j86z is the old digest, also labelled `garden`, status `ready`, with a Context window of 2026-11-01 to 2026-11-08 and no garden page; atelier-613a's Design already said it is not a run.

`.atelier/items/atelier-s0j2/item.md:22-23` starts the gardener with "Garden this repository; focus on structure" and nothing else. Line 31 requires the window to start at 0019's page end, found by `garden.md`'s rule. A gardener that only reads base files will treat this as a first run (first `atelier log` line, 2026-11-11T15:20:09Z) or take j86z's Context. Only `atelier show` on 0019 yields 17:56:43Z.

Say in `garden.md` that the previous run is the newest other item labelled `garden` whose Report has the template's `Window:` line, read the way `atelier show` reads a started item, and that a digest sharing the label is not a run. s0j2 can keep 0019 as the check.

## Finding 9: Say what the thread cap counts
Recommendation: fix now
`.atelier/items/atelier-wb05/item.md:33` has the run "choose threads by the numbers in a stated order (the item longest in each status, then the items with the most rounds, then those with escalations, then those with skips) up to a number the procedure gives, and read them in full, implementer and reviewer both."

The order picks items. The cap is named as threads. "Implementer and reviewer both" then expands each pick, and "the reviewer" has no bound when an item has several rounds (atelier-0019 had two; atelier-6w8d had three). Two gardeners will count the number as items or as transcripts, and will read the latest reviewer or every round. That is the unbounded slog Finding 4 closed for escalations and skips, and it makes `transcripts-first` (`mission.md:59-61`) uncheckable until the procedure is written.

State in the item what the number counts (transcripts, or items with a stated expansion), and that "the reviewer" is the thread of the latest round unless the numbers picked a skip or escalation on an earlier one.
