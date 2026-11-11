---
schema: atelier.item
schema_version: 3
id: atelier-f6dv
title: Fix what atelier's first gardener run showed wrong or unclear in `garden.md` and `readiness.md`
status: in_review
mission: atelier-295u
labels:
- redesign
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3A932c2504-ac02-42e4-95a0-bbe6ecd5863b
ready:
  by: sumeet, relayed by the task that started the lead
  at: 2026-11-11T18:14:51.208602129Z
trivial: null
caused_by: null
created_at: 2026-11-11T18:14:33.439768852Z
---

## Goal

Make the gardener's procedure say what atelier's first run, atelier-0019, had to guess or was told only by its lead. Four more first runs follow it in other repositories, and each would guess again. One of the gaps is harmful: step 1 does not say to start the run in its own worktree, and `atelier start` in a shared base checkout switches that checkout to the run's branch.

## Done when

Each line is a change of a sentence or less in `skills/agent-factory/procedures/garden.md`, unless it names `readiness.md`.

- [x] Step 1 says the run joins no mission, gives its Scope in a line, and says to run `atelier start <id>` in a worktree made for the run, not in the checkout that has the base.
- [x] `artifacts/` is named once in full, as `.atelier/items/<id>/artifacts/`.
- [x] Step 2 says the worst item is found by comparing seconds, and step 4 that a finding a later round carries is counted once.
- [x] Step 4 says a thread is read by its ID exactly as the log prints it.
- [x] Steps 4 and 5 no longer contradict each other on filing: an item made with `atelier new --from` on the run's branch gets its labels and `--caused-by` on the base after the run merges, and Fixed and Settled says which items wait for that. A finding the run itself fixed is settled with `atelier drop --reason "fixed in <commit>"`.
- [x] The template has a line for which numbers are not yet meaningful and why, and says a carried measure's value goes in the This run column with its date. `### Responses`, which `atelier submit` needs in the Report after a review, goes above Session and is not counted in the 60 lines.
- [x] Step 7 says `atelier status` offers `atelier merge` once the run is approved, and that the gardener does not run it before the session.
- [x] `readiness.md` line 4 says every install and build command in `AGENTS.md` and the files it links is run, for each thing the repository builds. The first run missed the Desk server's build, which named a deleted directory, and its reviewer caught it.
- [x] `garden.md` grows by at most 12 lines (it is 176), and the Report gives the count.

## Scope

In: `skills/agent-factory/procedures/garden.md` and line 4 of `skills/agent-factory/procedures/readiness.md`.

Out, and a review finding that asks for one of these is escalated, not fixed:

- Any change under `crates/`. That `atelier status` offers `merge` on an approved `garden` item is the tool's behaviour, and the specifications name no other.
- A change to what any number means: time waiting to start still includes time blocked, and a re-review after a catch-up still counts as a round. The page explains such a number; the procedure does not redefine it.
- The other things the run reported that needed no guess a sentence would save: a definition of a tool call for the navigation sample, a place to compare that sample with the baseline, and readiness lines 5, 7 and 9.
- The run atelier-0019 and its page.

## Context

atelier-0019's branch holds the page (`.atelier/items/atelier-0019/item.md`, pull request 148). Its gardener's full list is in the result of atelier-613a once that closes; the lines above are the ones this item takes. Mission: `.atelier/missions/atelier-295u/`, and atelier-613a's Done when asks for this item before the other four runs start.

## Report

`garden.md` went from 176 to 188 lines, the 12 allowed; `readiness.md` line 4 grew by one line. Commit 4f4788304.

- Step 1: the run joins no mission, its Scope goes in one line, and `atelier start` runs in a worktree of the base made for the run, because in the base checkout it would switch that checkout to the run's branch.
- `artifacts/` is named in full once, as `.atelier/items/<id>/artifacts/numbers.md` in step 2.
- Step 2 names the worst item by comparing seconds, not minutes. Step 4 counts once a finding a later round carries, and reads a thread by its ID exactly as the log prints it.
- Filing: step 4 now files a bug the way step 5 files a finding, instead of saying `atelier set` on the base. Step 5 says an item made on the branch gets its labels and `--caused-by` on the base after the run merges, and that a finding the run fixed is dropped with `"fixed in <commit>"`. The Fixed and Settled template line names the items waiting for the merge.
- Template: a carried measure's value and date go in the This run column, a "What means nothing yet" line follows the table, and `### Responses` sits above Session. Step 6 says Responses is not counted in the 60 lines.
- Step 7: `atelier status` offers `atelier merge` once approved, and that merge waits for the session.
- `readiness.md` line 4 runs every install and build command that `AGENTS.md` and the files it links name, for each thing the repository builds.

To stay within 12 lines, the phrase "which the run's reviewer checks" before the Done when block was cut, and three paragraphs were reflowed.
