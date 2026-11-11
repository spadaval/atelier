---
schema: atelier.item
schema_version: 3
id: atelier-f6dv
title: Fix what atelier's first gardener run showed wrong or unclear in `garden.md` and `readiness.md`
status: draft
mission: atelier-295u
labels:
- redesign
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T18:14:33.439768852Z
---

## Goal

Make the gardener's procedure say what atelier's first run, atelier-0019, had to guess or was told only by its lead. Four more first runs follow it in other repositories, and each would guess again. One of the gaps is harmful: step 1 does not say to start the run in its own worktree, and `atelier start` in a shared base checkout switches that checkout to the run's branch.

## Done when

Each line is a change of a sentence or less in `skills/agent-factory/procedures/garden.md`, unless it names `readiness.md`.

- [ ] Step 1 says the run joins no mission, gives its Scope in a line, and says to run `atelier start <id>` in a worktree made for the run, not in the checkout that has the base.
- [ ] `artifacts/` is named once in full, as `.atelier/items/<id>/artifacts/`.
- [ ] Step 2 says the worst item is found by comparing seconds, and step 4 that a finding a later round carries is counted once.
- [ ] Step 4 says a thread is read by its ID exactly as the log prints it.
- [ ] Steps 4 and 5 no longer contradict each other on filing: an item made with `atelier new --from` on the run's branch gets its labels and `--caused-by` on the base after the run merges, and Fixed and Settled says which items wait for that. A finding the run itself fixed is settled with `atelier drop --reason "fixed in <commit>"`.
- [ ] The template has a line for which numbers are not yet meaningful and why, and says a carried measure's value goes in the This run column with its date. `### Responses`, which `atelier submit` needs in the Report after a review, goes above Session and is not counted in the 60 lines.
- [ ] Step 7 says `atelier status` offers `atelier merge` once the run is approved, and that the gardener does not run it before the session.
- [ ] `readiness.md` line 4 says every install and build command in `AGENTS.md` and the files it links is run, for each thing the repository builds. The first run missed the Desk server's build, which named a deleted directory, and its reviewer caught it.
- [ ] `garden.md` grows by at most 12 lines (it is 176), and the Report gives the count.

## Scope

In: `skills/agent-factory/procedures/garden.md` and line 4 of `skills/agent-factory/procedures/readiness.md`.

Out, and a review finding that asks for one of these is escalated, not fixed:

- Any change under `crates/`. That `atelier status` offers `merge` on an approved `garden` item is the tool's behaviour, and the specifications name no other.
- A change to what any number means: time waiting to start still includes time blocked, and a re-review after a catch-up still counts as a round. The page explains such a number; the procedure does not redefine it.
- The other things the run reported that needed no guess a sentence would save: a definition of a tool call for the navigation sample, a place to compare that sample with the baseline, and readiness lines 5, 7 and 9.
- The run atelier-0019 and its page.

## Context

atelier-0019's branch holds the page (`.atelier/items/atelier-0019/item.md`, pull request 148). Its gardener's full list is in the result of atelier-613a once that closes; the lines above are the ones this item takes. Mission: `.atelier/missions/atelier-295u/`, and atelier-613a's Done when asks for this item before the other four runs start.
